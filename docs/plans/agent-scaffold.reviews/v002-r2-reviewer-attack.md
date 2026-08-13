# `ship-v0-0-2-inc1` round 2: reviewer, adversarial-construction lens

## Artifact reviewed

`git diff main..HEAD` at `f92e6df` in the detached worktree `.claude/worktrees/r2-attack`, `main` at `c68f541`. Five commits, 12 files, 1367 insertions and 137 deletions:

- `1a74512` fix: keep every interpolated free-text value on one generated line (`F1`)
- `dcdb037` chore: release 0.0.2
- `07f985d` fix: refuse a pack asset source that leaves the pack directory (`F4b`, initial)
- `e571f10` fix: contain every pack-controlled path at the shared read site (`F4b`, final; the round 1 tree)
- `f92e6df` fix: contain a pack path by where it resolves, not by its string alone (the round 1 fix pass)

Read in full before any attack: `docs/plans/agent-scaffold.steps/ship-v0-0-2.md`, then `docs/plans/agent-scaffold.reviews/v002-r1-triage.md`.

I did not write this change, did not review round 1, and fixed nothing. No repository file was modified anywhere except this findings file.

## Method

The lens is construction. Every claim below was built and run; nothing is inferred from reading alone.

THREE release binaries, one `CARGO_TARGET_DIR` each, from three separate `git archive | tar -x` extracts, verified distinct by `md5sum` so no stale fingerprint could make two revisions look alike:

```
a2225d9b7e505efe2531361c47a551b9  $SB/tgt-head/release/agent-scaffold   (HEAD, f92e6df, the fix pass)
f173cdc73180f22be4bb29f9f2e55641  $SB/tgt-pre/release/agent-scaffold    (e571f10, the round 1 tree, PRE-fix)
697469dc53023e50b50696360cc558ce  $SB/tgt-main/release/agent-scaffold   (main, c68f541)
```

Below, `$HEAD`, `$PRE` and `$MAIN` are those three binaries. `PRE` is the important comparison for this round: it isolates what the fix pass changed, rather than what the whole increment changed.

A FOURTH tree, `$SB/src-mut` with its own `$SB/tgt-mut`, carries one deliberate mutation used to test whether an axis is pinned. It is described where it is used.

All fixtures live under `$SB` = `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/r2-attack/`. Every escape target is inside that directory. No file outside it was created, changed or deleted, and no fixture was left with modified permissions.

Gates run at HEAD, all green:

| Gate | Result |
| --- | --- |
| `cargo test` | 461 passed, 0 failed (407 in the binary plus 54 across 10 integration binaries) |
| `cargo clippy --all-targets -- -D warnings` | clean |
| `validate --source ... --metrics ...` | `330 records, valid`; `99 steps, 75 questions, valid` |
| `render --check --strict docs/plans/agent-scaffold.plan.toml` | `up to date` |

The two findings below are NOT gate failures. Both pass every gate this repository runs, which is part of what makes them worth reporting.

## Verdict table

| id | severity | one line |
| --- | --- | --- |
| `B1` | high | Two of the five `PackSource::read` callers swallow every error, so the new containment refusal is silently converted into "the pack ships no such file": the tool's own pack, deployed with `instrument.md` as a symlink, scaffolds an `AGENTS.md` missing 9556 bytes of the instrumentation contract at exit 0 with no message. |
| `B2` | medium | The resolved rule refuses a pack whose files are symlinks into a shared location (the stow, home-manager and nix deployment shape), which worked one commit earlier; the CHANGELOG bullet enumerates the two link shapes that keep working and names no shape that stops. |

Two findings: one `high`, one `medium`. No `low`, no `critical`.

`B1` and `B2` share a precondition (a pack file that is a symlink out of the pack directory) and have DIFFERENT remedies. `B2` is the loud half and is arguably the trade the fix exists to take, so its remedy is disclosure. `B1` is the silent half and is not a trade anyone chose: the same cause produces exit 2 at three call sites and exit 0 with a wrong artifact at the other two. If the human decides the resolved rule must not refuse the symlinked-file shape at all, both collapse into that one decision; if it must, both stand as written.

THE MECHANISM ITSELF HELD. `resolved_within` is correctly constructed against everything I could throw at it directly: it canonicalises both ends, it returns the canonical path and the caller reads THAT path rather than re-joining, and `Path::starts_with` is component-wise. I could not defeat it. The full list of what I tried and failed to break is in the failed-attacks section, which is longer than the findings section. Both findings are about what the change does to callers and readers AROUND the new mechanism, not about the mechanism.

## `B1` (high): a containment refusal is silently swallowed at the two optional-literal read sites

### What is wrong

`PackSource::read` has FIVE callers. Three label or propagate a refusal; two discard every error:

```
src/manifest.rs:489   self.read("pack.toml")        -> propagates (exit 2)
src/manifest.rs:611   source.read(guidance)         -> LoadError::UnsafeModuleGuidance (exit 2)
src/manifest.rs:735   source.read(&spec.source)     -> LoadError::UnsafeAssetSource (exit 2)
src/main.rs:229-233   source.read("principles.toml")-> Err(_) => Ok(Vec::new())
src/main.rs:259       source.read("instrument.md")  -> .unwrap_or_default()
```

Before this fix pass, `ReadError::Escapes` was UNREACHABLE for those two, because both pass a fixed literal and a literal cannot fail the lexical rule. Discarding every error was therefore exactly right there: the only reachable errors were I/O errors, and "the pack ships no `principles.toml`" is the documented meaning of that (`src/main.rs:231`, and `README.md:360`, "A pack that ships no `principles.toml` simply has no principles to select").

The resolved rule makes `Escapes` reachable for a literal for the first time. A containment REFUSAL now arrives at two callers that were written when the only question was PRESENCE, and both answer it as absence. `src/main.rs` was touched by this change at exactly one line, `+mod safe_path;`, so neither caller was revisited.

The read site's own doc still states the premise under which leaving them alone was correct (`src/manifest.rs:430`): "the fixed `pack.toml`, `principles.toml` and `instrument.md` literals pass through too and can never escape". After this change they can, and two of the three do so silently.

### Reproduction 1: the tool's own pack, `instrument.md` deployed as a symlink

This is the shape a pack gets when it is deployed by GNU stow, by home-manager, or by any `ln -s` into a shared location. Nothing in it is adversarial.

```
cp -r $SB/src-head/pack $SB/c3/pack
mv $SB/c3/pack/instrument.md $SB/c3/shared/instrument.md
ln -s $SB/c3/shared/instrument.md $SB/c3/pack/instrument.md

$PRE  scaffold --template $SB/c3/pack --output-dir $SB/c3/out-pre  --vcs none --instrument --write
$HEAD scaffold --template $SB/c3/pack --output-dir $SB/c3/out-head --vcs none --instrument --write
```

Measured:

```
-- PRE:   Wrote to .../c3/out-pre  (30 changed, 0 left untouched).   exit=0
-- HEAD:  Wrote to .../c3/out-head (30 changed, 0 left untouched).   exit=0

AGENTS.md bytes: PRE=58989  HEAD=49433

diff out-pre/AGENTS.md out-head/AGENTS.md
136,151d135
<
< ## Instrumentation (metrics logging)
<
< Instrumentation is enabled for this project (it was scaffolded with `--instrument`). ...
  [the whole record schema: round, escalation, dismissal_recheck, intake, decision,
   baseline, waiver]
<
< This section is present only because instrumentation was enabled; a scaffold without
  `--instrument` omits it entirely.

diff -rq out-pre out-head
Files .../out-pre/.agents/AGENTS.reference.md and .../out-head/.agents/AGENTS.reference.md differ
Files .../out-pre/AGENTS.md            and .../out-head/AGENTS.md            differ
```

The user passed `--instrument`, the run reported "30 changed" at exit 0 with its ordinary plan lines, and 9556 bytes of the instrumentation contract are missing from both generated guidance files. Nothing on stdout or stderr says so. `--dry-run` on the same pack prints the same plan lines and the same exit 0.

The refused state is BYTE-IDENTICAL to the legitimate absent state, so no downstream check can tell them apart:

```
# same repo pack with instrument.md DELETED rather than symlinked
cmp -s $SB/c4/out-absent/AGENTS.md $SB/c4/out-link/AGENTS.md   ->  BYTE-IDENTICAL
```

The content that vanishes is the metrics-logging schema, so a project scaffolded this way runs the workflow with agents that were never told to log rounds. This project's own convergence checks are computed from those records.

The same silent drop happens under `--module checks --with-precommit-hook --instrument`: exit 0, 0 occurrences of `## Instrumentation (metrics logging)` in the generated `AGENTS.md`.

### Reproduction 2: `principles.toml` deployed as a symlink

Reached whenever a pack ships a `principles.toml` without also declaring it as an `[[asset]]`. The repository's own pack happens to declare it as an asset (`pack/pack.toml:140`), which is what makes the repo pack fail LOUDLY on this file rather than silently; nothing requires a third-party pack to do that, and `README.md:360` documents `principles.toml` as a property of the pack rather than as an asset.

```
# pack ships principles.toml (one principle, valid TOML) as a symlink to a sibling dir
$PRE  scaffold --template $SB/c1/pack --list-principles   ->  "1. My rule - One sentence."   exit=0
$HEAD scaffold --template $SB/c1/pack --list-principles   ->  ""                             exit=0

$PRE  scaffold ... --instrument --write   ->  exit 0, AGENTS.md:
PRINCIPLES:
1. My rule - One sentence.
INSTRUMENT:
INSTRUMENT FRAGMENT

$HEAD scaffold ... --instrument --write   ->  exit 0, AGENTS.md:
PRINCIPLES:

INSTRUMENT:

$HEAD scaffold ... --principles my-rule --write
error: unknown principle id: my-rule            exit=2
```

The pack's whole principle set is silently dropped, `AGENTS.md` is generated with an empty principles block at exit 0, and the only user-visible symptom is that naming one of their own principles reports it as unknown.

### Severity: `high`

A live wrong behaviour, which this project's calibration puts at `high` without further argument. The run writes a materially wrong artifact, at exit 0, with no diagnostic, on a pack that produced the right artifact at the immediately preceding commit. It is the same silent-at-exit-0 shape as `F4b` itself, inverted: `F4b` read a file it should not have, and this reads nothing where it should have read something, both while reporting success.

Two facts sit at the top of the band. The degraded artifact is `AGENTS.md`, the tool's central output, and the content dropped is the instrumentation contract, so the failure is invisible until someone later asks why a project has no round records. And the identical cause is a loud exit 2 at the other three call sites, so the boundary is inconsistent with itself rather than uniformly weak.

### What I did NOT find, so the triager does not have to re-derive it

The swallow itself is pre-existing and, for absence, deliberate and correct. What is new is the error class flowing into it, and the regression that follows. `load` (`src/manifest.rs:735`) and `module_guidance` (`src/manifest.rs:611`) both match on the variant correctly and neither reports a missing file as an escape; I checked both by construction (see the failed-attacks section).

## `B2` (medium): a legitimate pack shape stops working, and the artifact does not say so

### What is wrong

The resolved rule refuses any pack file that is a symbolic link whose target is outside the pack directory. That is what the rule is for and I am not arguing it is wrong. It also refuses the ordinary deployment shape in which the pack directory is assembled from links into a shared store: GNU stow, home-manager, nix profile links, or a dotfiles repository. Measured, `PRE` versus `HEAD`, three shapes:

```
-- pack.toml as a link to a shared manifest:
PRE:   create a.md / Wrote to .../out (1 changed).                                  exit=0
HEAD:  error: `pack.toml` is not a contained pack path (it resolves outside the pack
       directory, through a symbolic link); ...                                     exit=2

-- an [[asset]] source as a link into a store directory:
PRE:   create AGENTS.md / Wrote to .../out (1 changed).                             exit=0
HEAD:  error: asset source `AGENTS.md` is not a contained pack path (...)            exit=2

-- a [[module]] guidance as a link into a store directory:
PRE:   (loads)
HEAD:  error: module `m` guidance file `g.md` is not a contained pack path (...)     exit=2
```

The refusal is loud, the message states the rule, and a workaround exists and works: point `--template` at the directory the links resolve to.

```
$HEAD scaffold --template $SB/b7/store --output-dir $SB/b7/out --vcs none --write
          create  AGENTS.md
Wrote to .../b7/out (1 changed, 0 left untouched).                                   exit=0
```

What is missing is the statement that this shape stopped working. The 0.0.2 CHANGELOG bullet (`CHANGELOG.md:32`) is the only public description of the change, it is bound for crates.io, and its only sentence about which link shapes survive says the opposite-facing half:

> A pack-internal link, and a `--template` naming a link to the pack directory itself, both keep working: the rule is about where a path lands, not about whether a link was involved.

A reader who has a stow-deployed pack reads that sentence as reassurance. Both named shapes were the two the round 1 triage measured; the third shape, a link INTO the pack pointing out, is the one that breaks, and it is not mentioned. `README.md` documents `--template` and pack authoring (lines 287 to 360) and states no containment rule for a pack at all, so there is no second place a user could learn this.

### Why this is a disclosure obligation rather than an opinion

The repository already took the identical trade on the plan-side boundary, and stated it, in the user-facing document (`README.md`, the anchoring section):

> A layout where `docs/plans` or `docs/metrics` is a symlink pointing somewhere the other one is not under will now be refused by `validate --workflow` and left out by the projections, even though it worked before; the trade taken is that a loud refusal beats silently reading the wrong file.

That sentence names the layout, says it worked before, and names the trade. The pack-side change takes the same trade against a more hostile input and says none of it. Round 1's triage cited that precedent as the reason the pack boundary should be resolved rather than lexical; the disclosure half of the precedent came with it and was not carried over.

The round 1 triage also recorded, from its own prototype, "no measured behaviour change on any legitimate pack". That measurement covered two legitimate shapes (a pack-internal link, a symlinked pack root) and both still hold at HEAD. This is a third legitimate shape it did not test, and it changes behaviour. I record that as a correction to the evidence the decision rested on, not as a criticism of the ruling.

### Severity: `medium`

An unmet disclosure obligation, which is where this project's calibration puts `medium`. Not `high`: the refusal is loud, the message names the cause accurately ("it resolves outside the pack directory, through a symbolic link"), nothing is written, and the workaround is one flag away. Not `low`: 0.0.2 is about to be published, a CHANGELOG entry is a durable public claim, and the claim as written enumerates surviving link shapes in a way that positively misleads the affected reader.

## Attacks that FAILED

Ground covered, so the triager knows what does not need re-running. Everything here was built and run.

### The resolved mechanism itself

- SIBLING SHARING A NAME PREFIX. `pack` versus `pack-evil`, reached through a link inside the pack. Refused. `Path::starts_with` is component-wise as claimed. The implementer's claim is correct.
- THE INVERSE, A LEGITIMATE PATH WRONGLY REFUSED BY `starts_with`. I could not construct one. A genuine child shares every component of the canonical root by construction, and on this filesystem canonicalisation is byte-exact for both ends, so the comparison cannot fail for a real child. Not testable here, and NOT reproduced, so not reported: a case-insensitive or unicode-normalising filesystem (macOS HFS+/APFS) could in principle canonicalise the two ends to different spellings. Every symlink test in the change is `#[cfg(unix)]` and I ran on ext4.
- HARDLINK. A hardlink inside the pack to a file outside it IS read, and its contents land in the scaffolded project (measured, exit 0). Not reported as a finding: the path is genuinely inside the pack directory, so the claim as worded ("resolve to a location inside the pack directory") is true of it; git cannot carry a hardlink in a fetched pack, so the delivery vector the CHANGELOG describes does not exist for this shape; and anyone able to create the link already had read access to the file and could have copied it.
- CHAINED LINKS. A link to a link to an outside file. `canonicalize` resolves the whole chain; refused.
- DIRECTORY LINK ESCALATION. `up -> $root`, source `up/secret.md`, a string that is relative and carries no `..`. Refused. The round 1 shape 3 is closed.
- ABSOLUTE LINK TARGET INSIDE THE PACK. Allowed and read correctly, which is right: the rule is about landing, not about spelling.
- A MOVING PACK ROOT. `resolved_within` canonicalises the root first and the joined path second, and the joined path is built from the ORIGINAL root string. If the root link is repointed between the two calls, the two ends resolve under different real directories and `starts_with` fails, so the race can only produce a false REFUSAL, never a false accept. Fail-safe by construction in both swap directions.
- TIME OF CHECK TO TIME OF USE. The window exists: `fs::canonicalize` then `fs::read_to_string`. I did not attempt to report it as a finding and did not reproduce a win. The construction is the right one, because the caller reads the CANONICAL path the check returned rather than re-joining the original, so an exploit needs a component of an already-fully-resolved path replaced by a link inside a window of microseconds. The actor this boundary defends against is a pack AUTHOR who ships a directory tree, and who is not present on the machine at run time; an actor who can write into the pack's ancestry DURING the run can rewrite the pack before it and needs no race. Nothing in the program pauses between the check and the open, and no pack-controlled code runs.
- PACK ROOT FORMS. All of `<abs>`, `<abs>/`, `<abs>/.`, `<abs>/./pack`, `<abs>/pack/../pack`, a relative `pack` from the parent directory, and `.` from inside the pack: all exit 0 and write the same file at HEAD. `<abs>/pack/sub/..` where `sub` does not exist fails at both HEAD and PRE, because the kernel resolves components one at a time.
- NONEXISTENT PACK ROOT, and a REGULAR FILE as the pack root: byte-identical messages at HEAD and PRE (`No such file or directory (os error 2)`, `Not a directory (os error 20)`).
- `--template /`. Everything is then contained. This is the user naming their own root and is not an escape.

### The error path, `Escapes` versus `Io`

- AN ESCAPE REPORTED AS A MISSING FILE. Reproduced, and not reported. A link inside the pack pointing at a nonexistent outside file cannot be canonicalised, so it reports `error: No such file or directory (os error 2)` rather than as an escape. The message is byte-identical at PRE, nothing is read, and the file it would have escaped to does not exist. Same for a symlink loop: `Too many levels of symbolic links (os error 40)` at both revisions.
- A MISSING FILE REPORTED AS AN ESCAPE. Not constructible. `Escapes` from the resolved rule requires `canonicalize` to SUCCEED, which requires the path to exist. `load` and `module_guidance` both match on the variant rather than on the error's presence, so an I/O error stays an I/O error and carries its own wording (`could not be read`), which the new unit test asserts negatively.
- THE LEXICAL FAIL-FAST'S UNIQUE CONTRIBUTION. An escaping string naming a file that does not exist still reports as a refusal, not as a missing file: `../nonexistent.md` gives "(it carries a `..` component)" and `/etc/definitely-not-here` gives "(it is an absolute path)". The claim at `src/manifest.rs:466-468` holds.
- IS THE FAIL-FAST PINNED? I removed the lexical check from the read site in `$SB/src-mut` and ran the suite: `an_escaping_source_is_refused_at_load` and `an_escaping_module_guidance_is_refused_before_any_read` FAIL, 405 passed 2 failed. The two-rule ordering is pinned by existing tests. Not a finding.

### The two-rule ordering

- AN INPUT THE LEXICAL RULE REFUSES THAT THE RESOLVED RULE WOULD ALLOW. `a/../b.md` with a real `a/` and `b.md` inside the pack: refused lexically, would resolve inside. Also an ABSOLUTE path naming a file inside the pack: refused lexically, would resolve inside. Both are deliberate, both predate this pass, both are pinned by `src/safe_path.rs`'s own tests, and the new message names the actual cause correctly ("it carries a `..` component" / "it is an absolute path"). Round 1's `A4` is genuinely fixed. Not a false refusal that breaks a legitimate pack: a pack author has no reason to write either form, and neither is a shape a deployment tool produces.

### Regression hunting

- THE SHIPPED PACK, EVERY FLAG COMBINATION. `--template pack` (the `Directory` arm, not the embedded one) with each of: no modules, `--module checks`, `--module isolation`, `--module checks --module isolation --instrument`, and `--module checks --with-precommit-hook --instrument`. All exit 0 at `MAIN`, `PRE` and `HEAD`, and `diff -r` between the `MAIN` and `HEAD` output trees reports NO differences for all five. `F4`/`F4b` criterion 4 holds.
- DEGENERATE `source` VALUES. `""`, `"."`, `"./"`, `"sub/"`, and a `source` naming a directory: all give `Is a directory (os error 21)` at both PRE and HEAD, identical.
- THE `F1` HALF, HUNTING A FIFTH INTERPOLATION SITE. The `one_line` doc claims every free-text value on a generated line passes through it. I tried to falsify that with a `[[step]].slug` carrying a newline and a table row (refused by `validate --source` as "not a well-formed kebab-case id"), a `[[question]].receipt` carrying a fabricated queue item (refused as "not a `Q-<n>` id"), and by reading every remaining interpolation: `question.id`, `folded_into` and `superseded_by` are validated id references, and `waiver.note` and the provenance lists are free text but reach the page only through `escape_cell`, which calls `one_line`. I found no fifth site. The four-site claim holds.
- THE `A2` WRITE SIDE. Confirmed STILL OPEN at HEAD, as round 1 adjudicated: with `$SB/e1/out/docs` a pre-existing symlink to a sibling directory, `dest = "docs/dropped.md"` writes through it at exit 0 while the run reports "Wrote to .../e1/out". This is context, not a finding. The fix did not half-close it: the write side is untouched, `apply_asset` still does a bare `root.join`, and the `dest` check is still the lexical rule alone. The `UnsafeAssetDest` message is worded to match that ("a dest must be relative and carry no `..` component", with a comment at `src/manifest.rs:290-293` recording that the write side makes no resolution claim), so the two boundaries are now documented as different strengths rather than silently assumed equal. That is a coherent state, not an inconsistent one.

## Out-of-scope observations

Not findings. Recorded so the triager can see they were checked rather than missed.

- ROUND 1'S `A2`, `A3` AND `A5` are all still open at HEAD, as the `Q-71-r1close` decision intended. I confirmed `A2` by construction (above) and did not re-test `A3` or `A5`.
- The `src/manifest.rs:430` claim that the three fixed literals "can never escape" is a CLAIM defect as well as the cause of `B1`. I raise it inside `B1` because the code remedy is what makes it true again; the other reviewer holds the claims lens and may raise it separately, in which case it should merge into `B1` rather than count twice.
- An asset `source` that is missing reports as a bare `error: No such file or directory (os error 2)` with no mention of the asset or the pack. Identical at `MAIN`, `PRE` and `HEAD`, so it is pre-existing and out of this change.

## Round outcome, from this reviewer only

Two valid findings, one `high` and one `medium`. The triager rules; this is my input, not a verdict.

The new mechanism is soundly built. I attacked it directly for the larger part of this review and did not defeat it once: the containment holds against links, chained links, directory links, prefix-sibling roots, moving roots and every degenerate path form I could construct, and the round 1 `high` is genuinely closed at both consumer fields, in `--write` and `--dry-run` alike. Both findings are about the change's edges rather than its centre: what two untouched callers now do with an error they could not previously receive, and what the published bullet tells a reader about a shape it does not mention.
