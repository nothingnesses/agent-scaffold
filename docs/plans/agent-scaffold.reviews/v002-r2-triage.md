# `ship-v0-0-2-inc1` round 2: TRIAGE

Independent of both round 2 reviewers, of round 1's triager, of the implementer, and of the orchestrator. Every figure below is my own measurement. Where it differs from a reviewer's or from round 1's, the difference is called out in place.

## Artifact and commits

`git diff main..HEAD` in the detached worktree `.claude/worktrees/tri-r2` at `0a6d479`. Six commits, 12 files, 1367 insertions, 137 deletions:

1. `4080be5` fix: refuse a pack dest that leaves the output directory (`F4`)
2. `ead4de9` fix: keep every interpolated free-text value on one generated line (`F1`)
3. `d52b0cf` chore: release 0.0.2
4. `19f50f8` fix: refuse a pack asset source that leaves the pack directory (`F4b`, initial)
5. `f10ac96` fix: contain every pack-controlled path at the shared read site (`F4b`, final; the round 1 tree)
6. `0a6d479` fix: contain a pack path by where it resolves, not by its string alone (the round 1 fix pass)

NOTE ON THE BASELINE. Both reviewers report `main` at `c68f541`. In my worktree `main` is `8c7175f`, three docs commits later. `git diff c68f541 8c7175f -- src/ Cargo.toml CHANGELOG.md README.md` is EMPTY, so the baseline for everything the artifact touches is identical and nothing below turns on the difference.

Specification: `docs/plans/agent-scaffold.steps/ship-v0-0-2.md`, read in full. Round 1's adjudication: `docs/plans/agent-scaffold.reviews/v002-r1-triage.md`, read in full and treated as settled.

Round 2 findings adjudicated: `v002-r2-reviewer-attack.md` (`B1` high, `B2` medium) and `v002-r2-reviewer-claims.md` (five low, 62 assertions checked).

## Method

THREE release binaries, one `CARGO_TARGET_DIR` each, from three separate `git archive | tar -x` extracts, verified distinct by `md5sum`:

```
f23f245e905be8706c52eea573c791fc  tgt-head/release/agent-scaffold   (HEAD, 0a6d479, the fix pass)
e06aaff80b82ffb72a2279ba894444b2  tgt-pre/release/agent-scaffold    (f10ac96, the round 1 tree, PRE-fix)
a2ab4d93e51bab0525d772e604dee902  tgt-main/release/agent-scaffold   (main)
```

Below, `$HEAD`, `$PRE` and `$MAIN` are those three. `PRE` isolates what the round 1 fix pass changed; `MAIN` isolates what the whole increment changed. Both matter this round, because the question the orchestrator asked is which pass wrote which finding.

Fixtures live under my own scratch subdirectory. Every escape target and every symlink target is inside it. No tracked file was modified anywhere except this triage file.

Gates I ran myself at HEAD, all green:

| Gate | Result |
| --- | --- |
| `cargo test` | 461 passed, 0 failed, across 11 result lines |
| `cargo clippy --all-targets -- -D warnings` | clean |
| `validate --source ... --metrics ...` | `330 records, valid`; `99 steps, 75 questions, valid`, exit 0 |
| `validate --source ... --workflow` | `workflow invariants hold`, exit 0 |
| `render --check --strict` | `up to date`, exit 0 |
| ASCII check on all 12 changed files | `0` on every file |
| `cargo publish --dry-run` | packaged, `aborting upload due to dry run`, exit 0 |

`validate --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl` reports EXACTLY ONE problem, the pre-existing `` Open Questions item `Q-43` has an unknown status `superseded by `Q-44`` `` the spec excludes. `F1` criteria 3 and 4 hold at HEAD, as they did at round 1.

Neither finding this round is a gate failure. Both pass all seven.

## Verdict table

Severity is ruled by me and is absolute impact if left unfixed. It rates the finding, not the exploit that exposed it.

| id | verdict | reviewer severity | MY severity | one line |
| --- | --- | --- | --- | --- |
| `B1` | VALID (surviving id of the merged pair) | high | **high** | Two `PackSource::read` callers discard every error, so a containment refusal on `principles.toml` or `instrument.md` becomes "the pack ships none": a wrong `AGENTS.md` at exit 0 with no diagnostic. |
| claims 1 | DUPLICATE of `B1`, merged | low | (merged) | The same cause seen as two false claim sites. Its doc half is a REQUIRED part of `B1`'s remedy; see the merge ruling. |
| `B2` | VALID | medium | **medium** | A pack directory assembled from links into a shared store stops working, and the only public description of the change enumerates the two link shapes that survive and names none that stops. |
| claims 2 | VALID | low | **low** | The `ReadError` and `LoadError` docs still describe the one-rule world, including "The path never reached the filesystem", which the same file now contradicts 60 lines later. |
| claims 3 | VALID | low | **low** | `safe_path`'s module doc says a `[step.provenance].findings` ref is joined onto the plan directory and read. It is shape-checked only and never joined. |
| claims 4 | VALID | low | **low** | The 0.0.2 CHANGELOG section omits the `agent-flow` rename notice the same release adds to `README.md`. |
| claims 5 | VALID | low | **low** | "the fallback is unreachable" holds on Unix and fails on Windows, where a `Prefix`-bearing `dest` reaches it. |

SIX valid findings after merging one duplicate pair from seven reported: one `high`, one `medium`, four `low`. No `critical`. None invalid, none out of scope.

I lowered no `high` and no `medium`. `B1` and `B2` are confirmed at the severity their reviewer gave them, on my own measurements. So this triage creates NO dismissed-or-downgraded-high re-check obligation for the orchestrator to discharge.

## `B1` (high): a containment refusal is silently converted into an absent file

MERGED: `B1` survives; the claims lens's finding 1 is folded into it. The merge ruling is below, and it carries a condition the orchestrator must not lose.

### My measurements

`PackSource::read` (`src/manifest.rs:450`) has five production callers. I enumerated them myself rather than accepting the reviewer's list:

```
src/manifest.rs:489   self.read("pack.toml")         -> .map_err(io::Error::from)?   exit 2, no field label
src/manifest.rs:611   source.read(guidance)          -> LoadError::UnsafeModuleGuidance, exit 2
src/manifest.rs:735   source.read(&spec.source)      -> LoadError::UnsafeAssetSource, exit 2
src/main.rs:229-233   source.read("principles.toml") -> Err(_) => Ok(Vec::new())
src/main.rs:259       source.read("instrument.md")   -> .unwrap_or_default()
```

REPRODUCTION 1, the repository's own pack with `instrument.md` deployed as a link out. Nothing in this shape is adversarial.

```
cp -r $SB/src-head/pack $SB/b1/pack
mv  $SB/b1/pack/instrument.md $SB/b1/shared/instrument.md
ln -s $SB/b1/shared/instrument.md $SB/b1/pack/instrument.md

$PRE  scaffold --template $SB/b1/pack --output-dir .../out-pre  --vcs none --instrument --write
$HEAD scaffold --template $SB/b1/pack --output-dir .../out-head --vcs none --instrument --write
```

Measured:

```
PRE:   Wrote to .../out-pre  (30 changed, 0 left untouched).   exit=0
HEAD:  Wrote to .../out-head (30 changed, 0 left untouched).   exit=0    stderr = 0 bytes

AGENTS.md bytes:  PRE 58989   HEAD 49433   (9556 missing)   MAIN 58989
occurrences of `## Instrumentation (metrics logging)`:  PRE 1   HEAD 0
occurrences of `dismissal_recheck`:                      PRE 1   HEAD 0

diff -rq out-pre out-head
Files .../out-pre/.agents/AGENTS.reference.md and .../out-head/.agents/AGENTS.reference.md differ
Files .../out-pre/AGENTS.md            and .../out-head/AGENTS.md            differ
```

The user passed `--instrument`, the run reported "30 changed" at exit 0 with its ordinary plan lines, stderr was empty, and the whole round-record schema is missing from BOTH generated guidance files. `--dry-run` prints the same plan lines at the same exit 0.

The refused state is indistinguishable from the legitimate absent state. Against the same pack with `instrument.md` DELETED rather than linked:

```
cmp out-absent/AGENTS.md out-head/AGENTS.md   ->  BYTE-IDENTICAL
```

The same drop happens under `--module checks --with-precommit-hook --instrument`: exit 0, zero occurrences of the instrumentation heading.

REPRODUCTION 2, `principles.toml` deployed as a link out, on a pack that does not also declare it as an `[[asset]]`. `README.md:360` documents `principles.toml` as a property of the pack, not as an asset, so nothing requires a third-party pack to declare it. The repository's own pack does declare it (`pack/pack.toml:140`), which is the only reason the repo pack fails loudly on this one file.

```
$PRE  scaffold --template $SB/b2/pack --list-principles  ->  "1. My rule - One sentence."   exit=0
$MAIN scaffold --template $SB/b2/pack --list-principles  ->  "1. My rule - One sentence."   exit=0
$HEAD scaffold --template $SB/b2/pack --list-principles  ->  ""                             exit=0

$PRE  ... --instrument --write   ->  exit 0, stderr 0 bytes, AGENTS.md:
    PRINCIPLES:
    1. My rule - One sentence.
    INSTRUMENT:
    INSTRUMENT FRAGMENT

$HEAD ... --instrument --write   ->  exit 0, stderr 0 bytes, AGENTS.md:
    PRINCIPLES:

    INSTRUMENT:

$HEAD ... --principles my-rule --write  ->  error: unknown principle id: my-rule    exit=2
```

The pack's whole principle set is dropped, `AGENTS.md` is generated with an empty principles block at exit 0, and the only user-visible symptom is that naming one of the pack's own principles reports it as unknown.

ONE MEASUREMENT NEITHER REVIEWER MADE, and it sharpens the finding. While building the fixture I shipped a `principles.toml` that was malformed as well as linked. `PRE` reported it loudly:

```
$PRE: error: could not parse the pack's principles.toml: TOML parse error at line 1, column 1
      missing field `rationale`                                                    exit=2
$HEAD: (empty)                                                                     exit=0
```

So at HEAD a linked `principles.toml` is not merely unread: it cannot even be reported as malformed, because the swallow happens before the parse. A pack author debugging their own principles file gets silence where the previous commit gave them the parse error and the line number.

CONTROLS, so the boundary of the defect is exact. A `principles.toml` and an `instrument.md` that are links landing INSIDE the pack both work correctly at HEAD (measured: `1. R - S.` and `FRAG` both render). The default embedded scaffold is unaffected (one occurrence of the instrumentation heading). The repository's shipped `pack/` carries 0 symlinks across 38 files. So the population is: a `--template <directory>` pack whose optional literal files are links out of the pack.

### The mechanism, and why nobody caught it

Before the fix pass, `ReadError::Escapes` was UNREACHABLE for those two callers, because both pass a fixed literal and a literal is relative and carries no `..`. Discarding every error was therefore correct there: the only reachable errors were I/O errors, and "the pack ships no `principles.toml`" is the documented meaning of that. The code says so at `src/main.rs:231`, the README says so at `:360`, and `src/manifest.rs:95-97` states the design intent in terms, contrasting a declared `guidance` file with "the tool-computed `instrument.md`, which is silently optional".

The resolved rule made `Escapes` reachable for a fixed literal for the first time. A REFUSAL now arrives at two callers written when the only question was PRESENCE, and both answer it as absence. `src/main.rs` was touched by the whole increment at exactly one line, `+mod safe_path;`, so neither caller was revisited by anyone.

The read site's own doc still states the premise under which leaving them alone was correct (`src/manifest.rs:429-430`): "the fixed `pack.toml`, `principles.toml` and `instrument.md` literals pass through too and can never escape". And `src/manifest.rs:485-487` says of the `pack.toml` caller that "the containment refusal in `read` cannot fire here". I fired it: a pack whose `pack.toml` is a link out reports `` error: `pack.toml` is not a contained pack path ... `` at exit 2.

### Severity: `high`, ruled

A live wrong behaviour, which this project's calibration puts at `high` without further argument. The run writes a materially wrong artifact, at exit 0, with empty stderr, on a pack that produced the right artifact at the immediately preceding commit and at `main`.

Three facts sit at the top of the band and the orchestrator should have them. The degraded artifact is `AGENTS.md` and its reference copy, the tool's central output. The content dropped is the instrumentation contract, so the failure is invisible until someone later asks why a project has no round records, and this project's own convergence arithmetic is computed from those records. And the identical cause is a loud exit 2 at the other three call sites, so the boundary is inconsistent with itself rather than uniformly weak.

I considered whether the narrower population (a directory pack, not the embedded default) argues for `medium`. It does not, and ruling otherwise would be inconsistent with this loop's own precedent: round 1's `A1` had the SAME reachability requirement, a directory pack carrying a link, and round 1 ruled it `high`. The two findings are reachable by exactly the same user with exactly the same pack.

### The merge ruling: one finding, two remedies, and a condition

`B1` and the claims lens's finding 1 are the same underlying defect. One cause, the resolved rule reaching a fixed literal, produces two consequences: a behaviour (the silent swallow) and two false sentences (`src/manifest.rs:429-430` and `:485-487`). Both reviewers saw the same thing through their own lens, and the attack reviewer explicitly says the claims version should merge rather than count twice. I merge them, on the round 1 precedent for `A1` and Contract Finding 2.

THE CONDITION, and it is the reason the merge needs stating rather than just doing. Merging must not let either half close the other. Fixing the CODE does not make `:429-430` true: after the remedy a literal can still escape, it is merely reported instead of swallowed, so "can never escape" stays false. And `:485-487` is false for `pack.toml` regardless of anything done in `src/main.rs`. So `B1` is closed only when BOTH the code half and the two doc sites are done, and a fix pass that lands one and calls the finding closed has not closed it. I have written the remedy below as two required parts for that reason.

## `B2` (medium): a legitimate deployment shape stops working and nothing says so

VALID. Reproduced, `PRE` versus `HEAD`, three shapes:

```
-- pack.toml as a link to a shared manifest:
PRE:   create a.md / Wrote to .../out (1 changed).                                  exit=0
HEAD:  error: `pack.toml` is not a contained pack path (it resolves outside the pack
       directory, through a symbolic link); ...                                     exit=2

-- an [[asset]] source as a link into a store directory:
PRE:   create AGENTS.md / Wrote to .../out (1 changed).                             exit=0
HEAD:  error: asset source `AGENTS.md` is not a contained pack path (...)           exit=2

-- a [[module]] guidance as a link into a store directory:
PRE:   exit 0, body.md contains "MODULES:\nSECRET GUIDANCE"
HEAD:  error: module `m` guidance file `g.md` is not a contained pack path (...)    exit=2
```

The refusal is loud, nothing is written, and the message names the cause accurately. That is the trade the fix exists to take and I am not arguing it is wrong.

ONE MEASUREMENT THAT CORRECTS THE REVIEWER, and it makes the finding stronger rather than weaker. The reviewer records that "a workaround exists and works: point `--template` at the directory the links resolve to". That holds only when every pack file resolves into ONE real directory. It does not hold for the home-manager and nix-profile shape, where each file links to its own store path:

```
pack/pack.toml  -> store-a/pack.toml
pack/AGENTS.md  -> store-b/AGENTS.md

PRE:                            exit 0, writes AGENTS.md
HEAD:                           error: `pack.toml` is not a contained pack path ...   exit=2
HEAD --template store-a:        error: No such file or directory (os error 2)         exit=2
```

There is no directory to point at. The affected user's only recourse is to materialise the pack (`cp -rL`, or a clone that does not preserve links), and the disclosure has to say that rather than implying a flag change is enough.

### Why this is a disclosure obligation and not an opinion

The repository already took the identical trade on the plan-side boundary and stated it, in the user-facing document. I read `README.md:242` myself:

> A layout where `docs/plans` or `docs/metrics` is a symlink pointing somewhere the other one is not under will now be refused by `validate --workflow` and left out by the projections, even though it worked before; the trade taken is that a loud refusal beats silently reading the wrong file.

That sentence names the layout, says it worked before, and names the trade. Round 1's triage cited exactly this precedent as the reason the pack boundary should be resolved rather than lexical (`v002-r1-triage.md`, the CODE ruling). The disclosure half of the precedent came with it and was not carried over.

The pack side says none of it. `CHANGELOG.md:32` is the only public description, it is bound for crates.io, and its one sentence about which link shapes survive says the opposite-facing half:

> A pack-internal link, and a `--template` naming a link to the pack directory itself, both keep working: the rule is about where a path lands, not about whether a link was involved.

I verified both named shapes at HEAD and both are true. That is what makes the sentence a problem rather than an error: it is an accurate, reassuring enumeration that omits the one shape a deployment tool actually produces. `README.md` documents `--template` and pack authoring at lines 287 to 360 and states no containment rule for a pack at all, so there is no second place a user could learn this. The README diff for this release adds only the rename section.

### Severity: `medium`, ruled

An unmet disclosure obligation, which is where this project's calibration puts `medium`. Not `high`: the refusal is loud, the message names the cause, nothing is written, and no artifact is wrong. Not `low`: 0.0.2 is about to be published, a CHANGELOG bullet is a durable public claim, and the claim as written enumerates surviving link shapes in a way that positively misleads the affected reader, whose recourse I measured to be heavier than the reviewer thought.

## claims 2 (low): the error-type docs still describe the one-rule world

VALID. Three separate falsifications, all measured.

`src/manifest.rs:381-382` says "The path never reached the filesystem, so reporting it as a read error would describe an attempt that never happened". I traced a resolved refusal with `strace -f -e trace=openat,open,stat,lstat,newfstatat,readlink,readlinkat,statx`:

```
readlink(".../pack/link.md", "../secret.md", 1023) = 12
readlink(".../secret.md", 0x..., 1023)             = -1 EINVAL (Invalid argument)
```

Two `readlink` calls on the path, before the refusal, and no `openat` of the target anywhere in the trace. The path reached the filesystem. The narrower claim the fix pass took care to preserve, that the CONTENTS are never read, holds. `PackSource::read`'s own doc says the opposite of `:381-382` sixty lines later (`:442-443`, "this site stats and follows links before it refuses"), so the file contradicts itself.

`src/manifest.rs:378-379` says "the relative path was refused because it leaves the pack". This is the exact assertion round 1's `A4` found untrue and the fix pass removed from all four MESSAGES. It survives here. `a/../b.md` is refused and leaves nothing.

`src/manifest.rs:389-390`, `:191-192` and `:200-201` all enumerate the cause set as "it is absolute, or it carries a `..` component". `link.md` is neither and produces all three variants, which I fired at all three sites. Round 1's triage checked the `ReadError::Escapes` doc, ruled it TRUE, and recorded the ground: "the variant is only produced for those shapes as shipped". The fix pass removed that ground and did not revisit the sentence. Round 1's ruling was correct when it was made.

`low`: no outcome changes, and the only reader misled is a maintainer. Confirmed.

## claims 3 (low): `safe_path` says a findings ref is read, and it is not

VALID. `src/safe_path.rs:4-6` says `plan::source` joins a `[meta].sidecars` front/tail ref "(and a `[step.provenance].findings` ref) onto the plan directory to READ it".

The sidecar half is true: `src/plan/render.rs:167` and `:169` are `base.join(reference)` followed by a load. The parenthetical is false. The only use of a findings ref in `render.rs` is `format!("findings {}", provenance.findings.join(", "))` at `:511`, which puts it on the Roadmap Notes line as text. It is never joined onto anything.

Measured. The render fixture ships NO `render-fixture.findings/` directory at all, so its declared ref already names an absent path. I replaced it with `nowhere/absent-findings.md`:

```
validate --source  ->  7 steps, 5 questions, valid                exit=0
render             ->  rendered .../render-fixture.md             exit=0
grep absent-findings render-fixture.md
| `alpha` | complete | ... why: decisions Q-2; findings nowhere/absent-findings.md; commits abc1234 |
```

`low`, and the reason it matters is specific rather than general. `safe_path` now offers `resolved_within` in the same file, and round 1's remedy anticipates the plan-side boundary adopting it in a follow-up. A maintainer who believes a findings ref is read may apply the resolved rule to it. That rule requires the path to exist, and `src/plan/source.rs:245-247` records the deliberate opposite contract: a findings file is committed then deleted at task close, so a valid historical pointer may name an absent path.

## claims 4 (low): the 0.0.2 section omits the release's own rename notice

VALID. `d52b0cf` adds `README.md:7-11`, announcing that the project is being renamed to `agent-flow`, that the crate name is reserved, that releases move there once the rename lands, and that `agent-scaffold` is then reclaimable. `grep -n "agent-flow\|rename" CHANGELOG.md` returns nothing.

I enumerated the user-visible changes in `main..HEAD` and matched each to a bullet, as the reviewer did, and reached the same result: `F1`, `F4`, `F4b`/`A1`, `A4` and the version bump are all covered, and the rename section maps to nothing.

`low`, and I am ruling it BELOW the `medium` I gave `B2` on a distinction the orchestrator should see, because both are disclosure gaps and they are not the same kind. `B2` is an undisclosed BEHAVIOUR CHANGE whose only public description positively misleads the affected reader. Here the fact IS disclosed, in the same published artifact, in the README's second section, which crates.io renders on the crate page; `cargo publish --dry-run` packages that README and round 1 verified its contents. No reader is misled. What is imperfect is only the changelog's own coverage claim at `CHANGELOG.md:3`. That cannot change a verdict or an outcome, so the calibration keeps it at `low`.

## claims 5 (low): "the fallback is unreachable" is false on Windows

VALID. `src/manifest.rs:296-298` says "The write side applies the lexical rule only, so every refusal here has a lexical cause and the fallback is unreachable."

`UnsafeAssetDest` fires exactly when `is_contained_relative(dest)` is false (`src/manifest.rs:684`), and the cause phrase is `lexical_failure(dest).unwrap_or("it is not accepted as an output path")` (`:299-300`). The two disagree on a path carrying a `Prefix` component. `is_contained_relative` requires every component to be `Normal` or `CurDir`, so a `Prefix` makes it false. `lexical_failure` returns `Some` only for `is_absolute()` or a `ParentDir`, and on Windows a path is absolute only with a prefix AND a root, so `C:foo.md` is not absolute and carries no `..`. It returns `None` and the fallback fires.

I could not run this: I am on ext4 and the toolchain here has no Windows target. It rests on documented `std::path` semantics, which is the weakest evidence in this round's set, and I am ruling it valid on that basis while saying so. The platform is not hypothetical: the crate carries `#[cfg(not(unix))]` arms deliberately. On Unix the complement is empty and the sentence is true, which I did verify: `is_contained_relative("C:foo.md")` is true on Unix, because the string is a single `Normal` component there, and a dest of that name correctly writes a file so named inside the output directory.

`low`. Nothing changes for any user. The impact is that a maintainer trusting "unreachable" replaces the fallback with `unreachable!()` or `expect`, turning a tautological message into a panic on a supported platform.

## The three rulings this round turns on

### Ruling 1: where the remedy for the reachable-error class belongs

The round 1 fix closed a real hole and, in doing so, made a previously unreachable error reachable at two call sites that discard errors. The orchestrator asked whether the remedy belongs at the call sites, at `read`'s signature, or at the type level.

AT `read`'s SIGNATURE, and that IS the type-level answer available here. Not at the two call sites alone.

The two call sites are not wrong about what they want. `pack_principles` genuinely wants "no principles" when the pack ships none, and `README.md:360` promises that. The instrument block genuinely wants "" when the pack ships no fragment, and `src/manifest.rs:95-97` states that as the design. What is wrong is that the API gives them only one way to say it, and that way is to discard a `Result` whose error type now carries two incompatible meanings. Patching the two arms leaves `read` public with the same shape and the same trap for the sixth caller, which is precisely what the orchestrator's question anticipates and what Principle 5 (Make illegal states unrepresentable) exists to stop: "Work out the valid inputs and outcomes first and encode them, rather than admitting bad states and guarding against them at runtime."

The remedy that follows from that principle is to move the ABSENT outcome out of the error type and into the success type, by adding a second function beside `read`:

```rust
/// Read an OPTIONAL pack file. `Ok(None)` means the pack ships no such file, which
/// is the only outcome a caller may treat as an absence. Every other outcome,
/// including a containment refusal, is an `Err`.
pub fn read_optional(&self, rel: &str) -> Result<Option<String>, ReadError>
```

mapping only `io::ErrorKind::NotFound` to `Ok(None)`, and having the two callers use it and propagate the `Err`. After that, "the pack ships no such file" and "the pack shipped a file I refuse to read" are different values of different types, and `Err(_) => Ok(Vec::new())` stops being the way anyone spells the first one.

I will not overclaim what this buys. No Rust signature can stop a future caller writing `.unwrap_or_default()`; `Result<Option<String>, ReadError>` still has a `Default`. What the split buys is that the CORRECT optional-read primitive exists and is the obvious one to reach for, that the wrong one no longer produces a plausible-looking empty string, and that a caller who still wants to swallow must write an explicit arm discarding a `ReadError`, which is visible in review and findable with one grep. That is the strongest form available, and it is a real Principle 5 improvement over two patched arms rather than a rhetorical one.

TWO ALTERNATIVES I CONSIDERED AND REJECT.

Putting the distinction at the read SITE, so `read` itself decides what is optional, fails because the site cannot know: the same literal is required in one caller's world and optional in another's. The distinction belongs to the caller, so it belongs in which function the caller picks.

Relaxing the resolved rule so an optional literal that links out is allowed would make `B1` and `B2` both disappear, and the attack reviewer correctly notes that if the human takes that route both collapse into one decision. I rule against it and the human should know why before considering it. It would mean a linked `pack.toml` loading a manifest from outside the pack, which is strictly worse than what `A1` described, since the manifest controls every subsequent read. And it would make the boundary field-dependent, which is the incoherence `Q-75` already rejected in terms when it admitted `F4b`: containment on `dest` and not on `source` is "an incoherent boundary rather than a smaller one", and containment on `source` and not on `principles.toml` is the same sentence one level down.

### Ruling 2: `B2`'s disclosure gap is real, and what discharges it

REAL, on the ground the reviewer gave and on one they did not have. The reviewer's ground is the `README.md:242` precedent, which I read and confirmed word for word: the repository took the identical trade on the plan-side boundary, named the layout, said it worked before, and named the trade. Round 1's triage used that precedent to argue FOR the resolved rule and did not carry its disclosure half across. My additional ground is the measurement above: the workaround the reviewer found does not exist for the multi-store shape, so the affected reader needs more than a flag change and the disclosure has to say so.

WHAT THE CHANGELOG MUST SAY to discharge it. Five things, in `CHANGELOG.md:32`, the 0.0.2 Fixed bullet on the pack read escape:

1. Name the shape that stops working: a pack directory whose files are symbolic links to targets OUTSIDE that directory, which is what GNU stow, home-manager, a nix profile and a dotfiles tree produce. Name that a linked `pack.toml` is included, so the failure can be the first thing a user hits.
2. Say plainly that it worked before.
3. Name the trade in the repository's own established words, so the pack side and the plan side read as one policy: a loud refusal beats silently reading a file the pack did not ship.
4. Name the recourse accurately. Pointing `--template` at the directory the links resolve to works ONLY when every pack file resolves into one real directory; where the files resolve into different store paths there is no such directory and the pack must be materialised.
5. Keep the two surviving shapes, because both are true, but stop letting them stand alone as the account of which link shapes work. They must sit beside the shape that stops, in the same sentence or the next one.

One thing more, which is not strictly the CHANGELOG's job and is where a pack author will actually look: `README.md`'s pack-authoring section (287 to 360) states no containment rule for a pack at all, while `README.md:242` states the plan side's in full. One sentence there closes an asymmetry that is otherwise a second edition of this same finding.

### Ruling 3: round 1's triage was wrong on one sentence

YES, and I say it plainly because it is data about the method rather than about a colleague.

The sentence is round 1's summary of its prototype measurement: "no measured behaviour change on any legitimate input", supported by a table with two rows, a pack-internal link and a symlinked pack root. Both rows are correct and both still hold at HEAD; I re-measured them. What is wrong is the scope of the summary. Two shapes were tested and a third was not: a link INSIDE the pack whose target is OUTSIDE it. On that third shape behaviour changes at HEAD against both `PRE` and `MAIN`, loudly for three fields and silently for two, as measured throughout this document.

The reviewer's framing of the third shape as "legitimate" needs one qualification, and it does not rescue round 1's sentence. That shape is simultaneously the attack vector `A1` exists to close and the artefact of ordinary deployment tooling. The tool cannot tell them apart, which is exactly why the trade is a trade. So the correct finding is not that round 1 should have kept the shape working. It is that round 1's evidence sentence was broader than its evidence, and that the trade it recommended therefore reached the human without the disclosure obligation that the very precedent it cited came with.

THE LARGER MISS is `B1`, and it is worth naming precisely because it was not a wrong ruling. Round 1's triager built the candidate fix and ran it (Principle 6, Ground decisions in evidence, well applied, and the reason the code question was settled on a measurement rather than an estimate). But the measurement set was "does everything I already know about still work": 450 tests, clippy, the two legitimate link shapes, the shipped pack, a runtime comparison, a `diff -r` of the output trees. Nothing in that set asks "what is now reachable that was not". `read`'s callers were never enumerated. `src/main.rs:229` and `:259` appear in neither round 1's remedy nor its must-not-edit list. A prototype answers the first question well and cannot answer the second at all.

Round 1 also got the central thing right, and the record should carry that too. The resolved rule was the correct call: this round's attack reviewer spent the larger part of a review trying to defeat it and could not, and neither could I. Its instruction to author `resolved_within` in `safe_path` rather than inline is what makes `B1`'s remedy cheap now. And its must-not-edit list was respected: I verified that all four original integration cases in `tests/pack_source_stays_inside_the_pack.rs` survive with four ADDED beside them, and that both `tests/pack_dest_stays_inside_the_output_dir.rs` cases are intact.

## Remedies

Scoped to the class rather than the instance. Each names what must NOT be edited and why. Where a fixture is the only thing pinning an axis, the remedy ADDS rather than replaces.

### `B1` remedy, PART 1 of 2 (code)

CLASS: an outcome a caller is entitled to treat as ordinary must be encoded in the success type, so that a refusal cannot be spelled the same way as an absence.

CHANGE: add `read_optional` beside `PackSource::read` in `src/manifest.rs` as described in ruling 1, mapping only `io::ErrorKind::NotFound` to `Ok(None)`. Change `pack_principles` (`src/main.rs:228-234`) and the instrument block (`src/main.rs:258-259`) to call it and propagate the `Err`. Both must report at exit 2 with a message naming the file, matching the three call sites that already do.

MUST NOT BE EDITED:

- `is_contained_relative` and `resolved_within` (`src/safe_path.rs:49` and `:92`) and their four tests. `B1` is not a defect in the rule. Relaxing the rule so the literals pass would reopen `A1` for `pack.toml`, which is the worst field to reopen it on. Round 1's instruction not to touch `is_contained_relative` stands for its original reason as well: `is_safe_sidecar_ref` (`src/plan/source.rs:481`) depends on its no-filesystem-access form.
- The `Embedded` arm and the `assert!(builtin().read("pack.toml").is_ok())` line inside `the_read_site_contains_every_pack_controlled_path` (`src/manifest.rs:1116`). It is the only pin on the embedded exemption, and `read_optional` must keep the Embedded arm's NotFound behaviour or the default scaffold loses its principles.
- `a_missing_pack_file_still_reports_as_missing_not_as_an_escape` (`src/manifest.rs:1243`). It is the only pin on the missing-versus-escape distinction, which this remedy depends on to keep absence silent.
- `src/manifest.rs:95-97`, "the tool-computed `instrument.md`, which is silently optional". This sentence stays TRUE and is the boundary of the remedy: absence stays silent, only refusal stops being silent. A remedy that makes a MISSING `instrument.md` or `principles.toml` loud has broken `README.md:360` and gone too far.
- `src/manifest.rs:680-683` ("A `source` reaches the filesystem only through `PackSource::read`, which contains it there for every caller"). I checked it and it is TRUE as written, because it is scoped to `source`, whose caller propagates. It must not be swept into this edit.

DISCLOSE IN THE OUTCOME: mapping only `NotFound` to `Ok(None)` also makes an UNREADABLE `principles.toml` (permissions, non-UTF8) loud where it is silent today. That is the right direction and worth stating rather than leaving to be found: a MALFORMED `principles.toml` is already loud (`error: could not parse the pack's principles.toml`, exit 2, which I measured at `PRE`), so an unreadable one being silent is an inconsistency this closes rather than one it creates.

TESTS, ALL ADDITIONS:

- Unit, in `manifest::tests`: a pack whose `principles.toml` is a link out gives `Err(Escapes)` from `read_optional`, and one whose `instrument.md` is gives the same. ADD beside the existing read-site cases; do not fold them into `the_read_site_contains_every_pack_controlled_path`, whose four pins the claims lens verified all survive the last pass and which should not grow a fifth responsibility.
- Unit: a pack that ships NEITHER file still yields the empty set and the empty string, at no error. THIS IS THE IMPORTANT ADDITION and nothing currently pins it at any level: the shipped pack ships both files, so criterion 4's "a normal scaffold run drops the same set of files" cannot catch an over-tightening that makes absence loud.
- Integration, in `tests/pack_source_stays_inside_the_pack.rs`: `--instrument` against a directory pack whose `instrument.md` is a link out exits non-zero, the message names `instrument.md`, and no `AGENTS.md` is written. ADD; do not replace any of the eight cases now in that file.
- Integration, cheap and worth it: a linked `pack.toml` refuses with a message that names `pack.toml`. That path goes through `io::Error::from` and carries no field label, and nothing pins its wording.

### `B1` remedy, PART 2 of 2 (claims), REQUIRED, not optional

CLASS: a doc that asserts a code path is unreachable, where the reachability was a property of the rule rather than of the path.

Two sites, and the code fix does not discharge either:

- `src/manifest.rs:429-430`, "the fixed `pack.toml`, `principles.toml` and `instrument.md` literals pass through too and can never escape". After the code fix a literal can still escape; it is reported instead of swallowed. The sentence must say that the literals are subject to the same two rules as any other pack path, which is what makes the site genuinely THE ONE boundary.
- `src/manifest.rs:485-487`, "The path is a fixed literal, so the containment refusal in `read` cannot fire here; it is mapped rather than special-cased so this stays a plain read." The refusal fires. The mapping is still the right construction and the second clause survives; the first must go.

Nothing pins doc text, so no fixture is at risk. That absence is the same one round 1 recorded, and it is why both rounds have produced claim findings: these sentences are held by review and by nothing else.

### `B2` remedy

CLASS: a release that removes a working input shape must name the shape, not only the shapes that survive.

SITE: `CHANGELOG.md:32`, the five points in ruling 2. Plus one sentence in `README.md`'s pack-authoring section stating the pack containment rule, which currently exists only for the plan side at `README.md:242`.

MUST NOT BE EDITED: the two surviving-shape clauses in the same bullet. I measured both and both are true. ADD beside them; do not replace them, or the release loses a correct statement to gain a correct statement.

No fixture is at risk. Note that this remedy is required whichever way the human decides the ending, and it is required even if `B1`'s code half is deferred, since it describes what the code already does.

### claims 2 remedy

CLASS: an error-type doc that enumerates the cause set of a variant whose cause set grew.

SITES: `src/manifest.rs:378-379`, `:381-382`, `:389-390`, `:191-192`, `:200-201`. Correct the enumeration to include the resolved cause, and delete "The path never reached the filesystem", which the fix pass's own `:442-443` already states correctly.

MUST NOT BE EDITED, and this is the whole risk in this remedy:

- `src/manifest.rs:212-213`, `UnsafeAssetDest`'s IDENTICAL enumeration. It is still correct. The write side applies the lexical rule only: `src/manifest.rs:684` is `is_contained_relative` alone and `apply_asset` (`src/main.rs:120`) is a bare `root.join`, both of which I read. A search-and-replace across the four sibling enumerations would make this one wrong and turn a `low` claims fix into a new claims defect.
- `src/manifest.rs:442-445`. It is true and it is the model the other sentences must be brought to, not the reverse.

### claims 3 remedy

CLASS: a module doc that asserts a caller relationship that does not exist.

SITE: the `(and a `[step.provenance].findings` ref)` parenthetical at `src/safe_path.rs:5`. Delete it, or state the real relation, which differs in kind: a findings ref uses the same predicate as a shape check and is deliberately never joined and never read.

MUST NOT BE EDITED: `src/plan/source.rs:235-237` and `:245-247`. They are the correct statements and they are what falsify the module doc. Their not-existence-checked contract must not be "fixed" by applying `resolved_within` to a findings ref, which would require the path to exist and break a valid historical pointer to a findings file deleted at task close.

### claims 4 remedy

SITE: a `### Deprecated` bullet in `CHANGELOG.md`'s 0.0.2 section naming the move to `agent-flow`, that every published `agent-scaffold` version stays installable, and that the name becomes reclaimable.

MUST NOT BE EDITED: `README.md:7-11`. It is the authoritative text, it is correct, and the bullet should point at it rather than restate it, so the two cannot drift.

### claims 5 remedy

SITE: `src/manifest.rs:296-298`. Scope the unreachability claim to Unix, or drop it and keep only the justification for the fallback's wording.

MUST NOT BE EDITED: the fallback itself (`:300`). On Windows it is the only thing between a `Prefix`-bearing `dest` and a panic, and the refusal it reports is correct. Do not replace it with `unreachable!()` or `expect`, which is the exact failure this finding predicts.

MUST NOT BE EDITED: `the_lexical_rule_names_the_component_that_failed_it` (`src/safe_path.rs:127`). It pins the five Unix return values and is the only pin on the phrase set. If a case is added it must be `#[cfg(windows)]`; a Unix assertion about `C:foo.md` would pin the wrong thing, since on Unix that string is a single `Normal` component and is correctly accepted.

## The re-seeding measurement

Round 1's fix pass is `0a6d479`. I attributed every finding site with `git log -L <line>,<line>:<file>` and every behaviour with the `PRE` and `MAIN` binaries.

| finding | site or behaviour | authored or introduced by | fix pass caused it? |
| --- | --- | --- | --- |
| `B1` behaviour | silent drop of a refused literal | `0a6d479` (absent at `PRE` and `MAIN`, both measured at 58989 bytes) | INTRODUCED |
| `B1` claims half | `src/manifest.rs:429-430`, `:485-487` | `f10ac96` | INVALIDATED |
| `B2` behaviour | refusal of a link out of the pack | `0a6d479` | INTRODUCED |
| `B2` claims half | the two-surviving-shapes sentence, `CHANGELOG.md:32` | `0a6d479` | INTRODUCED |
| claims 2 | `src/manifest.rs:378-382`, `:389-390`, `:200-201` | `f10ac96`; `:191-192` from `19f50f8` | INVALIDATED |
| claims 3 | `src/safe_path.rs:5` | `4080be5`, and false at `4080be5` too | NEITHER |
| claims 4 | the missing `Deprecated` bullet | `d52b0cf` | NEITHER |
| claims 5 | `src/manifest.rs:296-298` | `0a6d479` | INTRODUCED |

THE RATE. Of the 6 valid findings, 3 are against text or behaviour the round 1 fix pass INTRODUCED: `B1`, `B2` and claims 5. That is 3 of 6, 50 percent strict. Adding the findings whose subject the fix pass did not write but did make false gives 4 of 6, 67 percent broad. The 2026-08-13 audit measured this project's fix-pass re-seeding at 49 percent strict and 61 percent broad, so this loop sits marginally above both figures and well within the range the audit describes. Weighted by severity the picture is sharper than the count: the fix pass owns the round's ONLY `high` and its ONLY `medium`, and the two findings it does not own are both `low`.

THE MECHANISM IS NOT THE ONE THE AUDIT NAMED, and the difference changes what to do about it.

The claims lens checked the fix pass's four rewritten claim sites and found all four true. I re-ran three of them myself against the shapes above: `AssetSpec.source`'s "on both counts" claim holds (a linked source is refused, exit 2, nothing written); `ModuleSpec.guidance`'s "cannot splice a file from outside the pack into `{{modules}}`" holds (`PRE` splices `SECRET GUIDANCE` into `body.md` at exit 0, `HEAD` refuses at exit 2 and writes nothing); and the read site's two-rules claim holds against everything I threw at it. Exactly ONE new false statement was authored by the fix pass, claims 5, which is `low` and false only on a platform this environment cannot run.

So the fix pass did not re-seed by writing wrong things about what it did. It re-seeded by widening the domain of a value and not re-checking what consumes and produces that value. Two forms, and they account for all four fix-pass findings:

- DOMAIN WIDENING WITHOUT RE-CONSUMING. `ReadError::Escapes` went from producible only for an absolute or `..`-bearing string to producible for ANY string, including a fixed literal. The pass revisited every site that DESCRIBES the rule and no site that CONSUMES the error. `B1` and claims 2 are both this, and they are the round's `high` and one of its `low`s.
- INPUT-SET WIDENING WITHOUT RE-ENUMERATING. The refusal's input set grew from strings a pack author must write deliberately to any file a deployment tool happens to link. The pass disclosed the two shapes it tested and enumerated none it did not, both on the filesystem axis (`B2`) and on the platform axis (claims 5).

Both are the same failure at different levels: the pass measured the fix against the cases it already knew and not against the cases the fix newly created. Round 1's triage used the same method and reached the same blind spot, which is why the triage did not catch it either. That is the honest reading of why this round happened, and it points at a corrective the audit's framing does not: a claims lens over a fix pass's own new text would not have found `B1`, because the fix pass's own new text is true. What finds `B1` is enumerating the consumers of any value whose domain the change widens, and running the pre-change and post-change binaries against shapes outside the existing test set. Both reviewers this round did exactly that, which is why both findings landed.

## Round outcome

ROUND OUTCOME: `new_valid`.

- Valid findings: 6 (after merging one duplicate pair from 7 reported).
- Severity list: `high` x1 (`B1`), `medium` x1 (`B2`), `low` x4 (claims 2, 3, 4, 5).
- No `critical`, none invalid, none out of scope.
- No `high` or `medium` was lowered or dismissed, so this triage creates no re-check obligation.

THIS ROUND DOES NOT CONVERGE THE INCREMENT. `ship-v0-0-2-inc1` is `low_risk` by `Q-74`, so one CLEAN round converges it, and this is the second consecutive `new_valid` round. Two rounds have run against a five-round cap, so a third is within the cap and does not itself escalate.

WHAT WOULD CONVERGE IT: one round producing zero valid findings, run after the remedies land. Unlike round 1, I can say something concrete about whether that is realistic, because the blast radius of the remaining work is enumerable in advance rather than open. `B1`'s code half touches one new function and two call sites, and the set of things it can break is `read`'s five callers, the embedded arm, and the absence-stays-silent contract, all named above. The other five remedies are text in four files with no behaviour at all. That is a bounded question for a third round rather than an open one, and it is what makes another round affordable here.

## Merge and publish

THE CHANGE IS NOT SAFE TO MERGE AND PUBLISH AS IT STANDS.

Two blocking reasons, and they are different in kind.

`B1` is the harder one. The tool's central output is silently and materially wrong, at exit 0 with empty stderr, on an input that worked at the immediately preceding commit and at `main`, and the wrong output is byte-identical to a legitimate one so no downstream check can tell them apart. The content dropped is the instrumentation contract, so a project scaffolded this way runs the workflow with agents that were never told to log rounds, and this project's own convergence arithmetic is computed from those records. Publishing that is worse than publishing the hole it replaced, because a leak is at least discoverable in the scaffolded tree while this is discoverable only much later and by inference.

`B2` is round 1's blocking reason recurring in the opposite direction. Round 1 blocked because the artifact claimed a property the code did not have. Now the code has the property and the artifact does not disclose what the property cost, in a CHANGELOG bound for crates.io, while the same repository disclosed the identical trade on the plan side in terms.

MINIMUM SET THAT MAKES IT SAFE TO PUBLISH:

1. `B1` part 1, the code half: `read_optional` and the two callers, with the three test additions.
2. `B1` part 2, the two doc sites. Required, and not discharged by part 1.
3. `B2`'s CHANGELOG disclosure, all five points, plus the one README sentence.

That is the blocking set. Claims 2, 3, 4 and 5 do NOT block publication: none can change a verdict or an outcome, and none contradicts anything a user is told. I would still land all four in the same pass, because they are text in files the blocking set already opens, claims 2 fixes a file that currently contradicts itself sixty lines apart, and deferring them creates exactly the process-generated follow-up work the 2026-08-13 audit measured at 54.2 percent.

## How this increment should end

This is a genuine decision and I am not taking it. The pressures are real in both directions and the human should choose against the measurements rather than the framing. The step exists to end a 34-day delivery drought and declares its scope closed; two rounds have now run and both were `new_valid`.

Judged against the plan's Project Principles by name (`docs/plans/agent-scaffold.md`, `## Project Principles`).

### Option A: one fix pass, then a third round

COSTS. One implementer pass and one review round of delay. A third round is not guaranteed clean, and the record shows why: the last fix pass wrote this round's `high`.

GAINS. The only path that both closes the `high` and converges the increment the way `Q-74` defines convergence, on a clean round rather than on a waiver.

PRINCIPLES. Principle 6 (Ground decisions in evidence) is the strongest argument for it: the convergence bar exists because rounds keep finding things, and two consecutive `new_valid` rounds are evidence for the bar rather than against it. Principle 1 (Prefer the cleaner long-term architecture over the smallest diff) supports the `read_optional` shape, which is a genuine improvement rather than a patch.

### Option B: one fix pass, then merge without a further round

COSTS. The increment converges on a waiver rather than a clean round. That is a legitimate recorded exit and this project just built the mechanism for it: a `type:"waiver"` with `unit:"increment"`, `reason:"review-skipped"`, and `evidence_tier:"record-backed"` if a decision receipt backs it, which W3 and W5 then check. The real cost is specific rather than procedural: an unreviewed fix pass is exactly the input that produced this round's `high`, on the same defect class, one round ago.

GAINS. Ships roughly a round sooner, with the blocking defects closed.

PRINCIPLES. Principle 2 (Minimal by default) and the step's own closed-scope sentence support stopping. Principle 6 cuts against skipping the check on the one class that has already produced a fix-pass regression.

### Option C: revert the resolved-path fix, ship the lexical rule with honest wording

COSTS. I measured what this ships. `PRE` on a pack carrying `link.md -> ../secret.md` prints `create leaked.md`, reports "Wrote to .../out (1 changed, 0 left untouched)" at exit 0, and the scaffolded project contains `SECRET`. That is round 1's `A1`, a `high` the human decided to fix one round ago under `Q-71-r1close`, published with a note. It also discards a mechanism that this round's attack reviewer spent most of a review failing to defeat and that I could not defeat either.

GAINS. `B1` and `B2` both disappear, since both are consequences of the resolved rule, and the remaining work is text.

PRINCIPLES. Fails Principle 5 (Make illegal states unrepresentable) and Principle 1 on the same argument `Q-75` already accepted when it admitted `F4b`. It also reverses a human decision on evidence that has not changed: nothing found this round weakens the case for the resolved rule, and the reviewer who attacked it hardest reports that it held.

### Option D: split the release

CONCRETELY: ship 0.0.2 with `F1` and `F4` and the release mechanics, hold `F4b` for 0.0.3.

COSTS. `F1` and `F4` are both closed, both gated green, and neither was challenged by either reviewer in either round, so this ships only uncontested work. But `Q-75` admitted `F4b` precisely because leaving `AssetSpec` enforcing containment on `dest` and not on `source` is "an incoherent boundary rather than a smaller one", and splitting publishes that incoherence. It also means removing bullets from a CHANGELOG that is otherwise accurate, and the increment's review record then covers code that is not in the release, which is a worse audit trail than either A or B.

GAINS. The drought ends now, on a release whose every claim is uncontested.

PRINCIPLES. Principle 2 supports it. Principle 1 and `Q-75`'s own recorded reasoning cut against it.

### Recommendation and reasoning

I RECOMMEND OPTION A, with round 3 scoped rather than open.

- PRINCIPLE 6 (Ground decisions in evidence) is decisive and it is the same principle round 1 invoked. Round 1 measured the fix and shipped it unreviewed into a round; that round found a `high` in it. Option B proposes the same input again on the same defect class. The measurement that should settle this is in this document: 3 of this round's 6 valid findings, including both non-`low` ones, are against what the last unreviewed fix pass introduced.
- PRINCIPLE 5 (Make illegal states unrepresentable) is what makes A worth the round rather than merely safe. The remedy is not a patch: it moves the absent outcome into the success type, so the next caller of a public API cannot express a refusal as an absence without saying so explicitly. Option B would probably land the same code; the difference is whether anyone checks that it did not widen something else, which is the failure mode this whole loop is now evidence for.
- PRINCIPLE 1 (Prefer the cleaner long-term architecture over the smallest diff) rules out C and weighs against D, on the argument `Q-75` already accepted and which applies unchanged.
- PRINCIPLE 2 (Minimal by default) and the drought are the honest counterweight, and they are why I would SCOPE round 3 rather than repeat a full review. The blast radius of the remaining work is enumerable in advance, and I have enumerated it: `read`'s five callers, the embedded arm's NotFound behaviour, the absence-stays-silent contract at `README.md:360` and `src/manifest.rs:95-97`, and six text sites with no behaviour. Give round 3 that list as its scope, plus the standing instruction to enumerate the consumers and producers of anything the fix pass widens. That is a bounded round, not another open one, and it is the difference between finishing this increment and looping on it.

If the human instead prefers to ship a round sooner, Option B is defensible and the waiver mechanism exists for it. Two conditions would make it much less risky, and I would put them alongside the option rather than let it be taken bare: the full blocking set lands, including both halves of `B1`; and the fix pass is required to state, in its outcome, the enumeration of `read`'s callers and what each now does with a refusal, so the thing that was missed twice is written down even if no round checks it.
