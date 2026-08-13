# `ship-v0-0-2-inc1` round 4: TRIAGE

Independent triager. I did not write this change, I did not review it in any round, and I did not triage rounds 1, 2 or 3. Every figure below is my own measurement, made in this session. Where I reproduce a reviewer's result I say so and give my own numbers rather than citing theirs.

## Artifact

- Worktree `.claude/worktrees/tri-r4`, detached at `fde1d60` ("docs: delete the nine unmeasured claims round 3 found, and label root failures").
- Ruled on: `git diff main..HEAD`, and for the last pass `git diff HEAD~1..HEAD` (6 files, 53 insertions, 24 deletions).
- Specification: `docs/plans/agent-scaffold.steps/ship-v0-0-2.md`. Risk class `low_risk` (`Q-74`), so ONE clean round converges the loop.
- Findings adjudicated: `v002-r4-reviewer-code.md` (`F1`, 1 `low`) and `v002-r4-reviewer-text.md` (findings 1 to 4, 4 `low`).
- Settled and not reopened: the round 1, 2 and 3 triages, `A2`, `A3`, `A5`, the audit's `F2` and `F3`, the `superseded by` projection defect, the rename, ANSI escapes in a `dest`, the plan-side sidecar symlink hole, the containment mechanism, and the empty-directory gap.

## Method

THREE release binaries, one `CARGO_TARGET_DIR` each, from three separate `git archive | tar -x` extracts, verified distinct by `md5sum`:

```
e2fb17926f5774293f45db6bd9b69186  tgt-head/release/agent-scaffold  (fde1d60, HEAD)   0.0.2
cd9577c5a29b3d8353dcc102f712d100  tgt-pre/release/agent-scaffold   (62b6571, PRE)    0.0.2
6f74853eccd2f098a6e70efc7134adaa  tgt-main/release/agent-scaffold  (main, 0.0.1)     0.0.1
```

`--version` confirms 0.0.2, 0.0.2 and 0.0.1. Three further trees carried one source mutation each, in their own target directories, for the clippy probes.

Every fixture, symbolic-link target, escape target and cargo target directory sits under my own scratch subdirectory. THREE fixtures needed a permission bit, because nothing else produces `EACCES` on a directory. All three were restored to `755` and the restoration is recorded in the run output (`stat -c '%a %n'` at the end of the script prints `755` for each). `git status --short` is empty in this worktree and in the main repository, apart from this file.

GATES I RAN MYSELF AT HEAD:

| gate | result |
| --- | --- |
| `cargo test --no-fail-fast` | 468 passed, 0 failed, over 11 result lines (409+5+1+1+9+2+13+3+20+1+4) |
| `cargo clippy --all-targets -- -D warnings` | clean, exit 0 |
| `validate --source ... --metrics ...` | `334 records, valid`, `99 steps, 75 questions, valid`, exit 0 |
| `validate --source ... --workflow` | `workflow invariants hold`, exit 0 |
| `render --check --strict` | `up to date`, exit 0 |
| `validate --plan ... --metrics ...` | EXACTLY ONE problem, the pre-existing `Q-43` `superseded by` one the spec's criterion 4 excludes |
| ASCII check on all six changed files | `0` non-ASCII lines on every file |

I did not run `cargo publish --dry-run`, which needs the network. It is the one release gate I leave unverified.

## Verdicts

FIVE findings reported, FOUR after merging. All four valid. None invalid, none out of scope. I raised no severity and lowered none: every rating is `low` on my own measurement, so this triage creates no dismissed-or-downgraded-high re-check obligation.

`P1` is the code reviewer's `F1` merged with the text reviewer's finding 1. `P2`, `P3` and `P4` are the text reviewer's findings 2, 3 and 4.

| id | verdict | severity | class | site | can be a residual? | one line |
| --- | --- | --- | --- | --- | --- | --- |
| `P1` | valid, MERGED from two reviewers | low | half A USER-FACING, half B INTERNAL doc plus a USER-FACING message | `src/main.rs:2117-2119` and `src/main.rs:229-231` | YES, with the record corrected | `is_dir` cannot tell "not a directory" from "cannot tell", so a real directory under an unreadable parent is refused as "must name a directory", and an unreadable root still reports against a `principles.toml` that is not there. `X7` is NOT discharged. |
| `P2` | valid | low | USER-FACING | `CHANGELOG.md:37` | NO | "In 0.0.1 each was read through a symbolic link out of the pack and its contents inlined" is false of `pack.toml`. The outside manifest was OBEYED, and nothing of it was inlined. |
| `P3` | valid | low | INTERNAL | `src/manifest.rs:524-528` | YES | `read_optional`'s doc says a swallowing caller "passes `cargo clippy --all-targets -- -D warnings`". Both callers that exist fail it at exit 101. |
| `P4` | valid | low | USER-FACING | the 0.0.2 section | YES | The new `--template` root message is user-visible behaviour this commit added, and no bullet describes it. |

NO `critical`, NO `high`, NO `medium`. I state that explicitly because the orchestrator asked: nothing this round carries the obligations a `high` or a `medium` would.

### Why `P1` merges rather than duplicating

The two reviewers worked independently, on separate binaries, and reached the same predicate, the same two artefacts, the same two shapes and the same remedy. That is corroboration, not duplication, and I record it as ONE finding with TWO halves rather than dismissing either as a duplicate of the other.

The round 3 triage ruled that `C1` and `X7` are NOT duplicates because they name two different artefacts. That ruling stands and is untouched by this one. The merge here is between REVIEWERS, not between artefacts: `P1` still holds two artefacts (a message and a doc sentence), with two different remedies, because one change does not close both.

## `P1` (low): the permission class, and why `X7` is not discharged

Both reviewers measured this. I built the fixtures myself and ran all three binaries.

### Half A: the new message is false of an input it refuses. INTRODUCED by `fde1d60`

Three runs of the SAME path with the SAME HEAD binary, changing only the parent directory's mode:

```
control A, parent mode 755:
  test -d <path>  ->  yes
  HEAD  exit=0  31 files written, "Wrote to <output-dir> (30 changed, 0 left untouched)."

control B, parent mode 000, same path, same binaries:
  MAIN  exit=2  error: Permission denied (os error 13)
  PRE   exit=2  error: could not read the pack's principles.toml: Permission denied (os error 13)
  HEAD  exit=2  error: --template `<path>` must name a directory

control C, parent mode 755 again, same path, same binary:
  HEAD  exit=0  31 files written
```

Controls A and C establish that the path names a directory and that HEAD scaffolds 31 files from it. In control B the same binary tells the user that the same path "must name a directory". The statement is false of that path. The cause is a permission failure on the parent, so the message names the wrong problem and a user checks the wrong thing.

This breaches a standard the tree states in its own words, at `src/safe_path.rs:66-68`: a refusal must state the rule and the specific cause "rather than asserting a property of the input", with `a/../b.md` given as the example of getting it wrong. `--template <path> must name a directory` asserts a property of the input, and on this shape the property is false.

`MAIN` names no file and the right cause. `PRE` names the wrong FILE and the right cause. `HEAD` names the right FLAG and the wrong cause. No version is correct on this shape, and HEAD does not make it worse in any outcome, which is why the half is `low`.

### Half B: a root failure is still reported against `principles.toml`

Round 3's own `X7` reproduction, rebuilt from scratch:

```
fixture: a pack directory holding a.md and pack.toml, and NO principles.toml
  ls -A <root>                     ->  a.md  pack.toml
  test -e <root>/principles.toml   ->  NO
  chmod 000 <root>

MAIN  exit=2  error: Permission denied (os error 13)
PRE   exit=2  error: could not read the pack's principles.toml: Permission denied (os error 13)
HEAD  exit=2  error: could not read the pack's principles.toml: Permission denied (os error 13)

(mode restored to 755; the directory still holds a.md and pack.toml)
```

An EMPTY directory at mode `000` gives the identical message at all three binaries, verified empty before the chmod and after the restore.

The file is not present. The unreadable thing is the root. `is_dir()` answers TRUE for the directory, because `stat` on a directory needs search permission on its PARENT and not on the directory itself, so the new root check passes it straight through to the read.

ONE CORRECTION TO THE RECORD that neither reviewer stated in this form. On this shape the misattribution is a REGRESSION AGAINST 0.0.1, not a pre-existing defect: `MAIN` prints an unlabelled `Permission denied`, and `PRE` and `HEAD` print it against a `principles.toml` that does not exist. Naming the wrong file is worse than naming no file. `git log -S` attributes the change to `62b6571`, the round 2 fix pass, so it is this increment's own regression rather than something inherited.

### Severity: `low`, confirmed, and I did not raise it

Every shape exits 2 and writes nothing, at all three binaries. No exit code, no output byte and no written file changes. Raising this above `low` would require the message or the doc sentence to be able to produce a wrong RESULT, and neither can. That is the same standard round 3 applied to `C1` and `X7`, both rated `low`, so the rating is consistent across rounds rather than newly lenient.

## The `X7` ruling

The orchestrator asked me to rule on three things. I rule on each separately.

### 1. Is `X7` genuinely still open? YES

`fde1d60`'s commit message records:

> With that, PrinciplesError::Read's "the file is present" becomes true as written and needs no edit.

`src/main.rs:229-231` still reads "The file is present and could not be read: a containment refusal, or an unreadable file." On the fixture above the file is not present, and the sentence is false of the run that produces the message. `X7` is open. The round record must not carry it as fixed.

### 2. Was the round 3 triage's own premise wrong? YES, and I measured the exact step that fails

`v002-r3-triage.md:412` states:

> For any root that canonicalises, every non-`NotFound` failure of `root.join("principles.toml")` involves a `principles.toml` that exists.

Measured on a mode `000` directory:

```
realpath <root>                  ->  <root>              (SUCCEEDS)
realpath <root>/principles.toml  ->  Permission denied   (FAILS)
```

The root canonicalises. The child fails with a non-`NotFound` kind. No `principles.toml` exists. The premise is false, and it is false for exactly the reason the reviewers give.

The mechanism is visible in the source. `safe_path::resolved_within` (`src/safe_path.rs:101-103`) calls `fs::canonicalize(root)`, which needs only search permission on the root's PARENT and therefore succeeds, then `fs::canonicalize(root.join(rel))`, which needs search permission on the ROOT and therefore fails with `EACCES`. `read_optional` folds only `NotFound` to `Ok(None)` (`src/manifest.rs:538`), so the `EACCES` propagates as `PrinciplesError::Read`.

The round 3 triage rated this class correctly, scoped its remedy correctly, and named the correct fallback. It got one inference wrong, and the inference is the one the fix pass relied on to skip an edit. I record that plainly, and I record that it is the SAME defect class round 3 itself named against round 2: a remedy stated as an assertion becomes a claim in the tree, and the next round falsifies it. Round 3 wrote that its own remedies were built to avoid it. On `X7` it did not.

### 3. The minimum honest wording

Round 3 already named the fallback: weaken `X7` "to what `Read` actually means, that the read did not produce text". That fallback is correct for the FIRST clause and INCOMPLETE for the second, and I measured why.

Current text, `src/main.rs:229-231`:

> The file is present and could not be read: a containment refusal, or an unreadable file. Distinct from `Parse` because the file never became text, so telling the user it did not parse would name the wrong step.

`Read` fires for every read failure other than `NotFound`. That covers a containment refusal, an unreadable FILE, and an unreadable or unenterable pack ROOT. So:

- "The file is present and could not be read" must go. Round 3's replacement, "the read did not produce text", is accurate and is a deletion of three words plus a shorter clause.
- "or an unreadable file" is still incomplete after that edit, because on the measured shape the unreadable thing is the pack DIRECTORY and not a file. Widening it to a path the tool could not read, or naming the root case, closes it.

THE MINIMUM HONEST WORDING is therefore both clauses, not one: state that the read did not produce text, and let the enumeration name a path rather than a file. The third sentence ("Distinct from `Parse` ...") is true as it stands and must not be touched. No new comparative claim is needed for either edit, and no measurement beyond the ones in this document.

## `P2` (low, USER-FACING): what a linked `pack.toml` did at 0.0.1

QUOTED, `CHANGELOG.md:37`:

> The three files the tool reads by literal name, `pack.toml`, `principles.toml` and `instrument.md`, are contained too. In 0.0.1 each was read through a symbolic link out of the pack and its contents inlined, the same leak as the two fields above on three more paths.

I built three fixtures at `MAIN`, one per literal, each with the literal deployed as a symbolic link to a file outside the pack.

```
principles.toml -> outside   MAIN exit=0  AGENTS.md: "P:1. LEAKED PRINCIPLE - from outside the pack - from outside the pack"
instrument.md   -> outside   MAIN exit=0  AGENTS.md: "I:LEAKED-INSTRUMENT-FRAGMENT"
pack.toml       -> outside   MAIN exit=0  NO AGENTS.md at all; the outside manifest was OBEYED
```

For the first two, "inlined" is the exact word, and it is the word `README.md:327` uses for them. For `pack.toml` nothing is inlined. I ran an outside manifest that exercises both escapes at once:

```
outside manifest:  source = "../outside/secret.md"   dest = "../elsewhere/planted.md"
MAIN  exit=0  stdout: "create  ../elsewhere/planted.md" / "Wrote to <output-dir> (1 changed, 0 left untouched)."
      the file lands OUTSIDE the output directory, carrying "TOP SECRET OUTSIDE THE PACK"
      grep of the whole output tree and the planted tree for the manifest's own text: NO FILE
```

So at 0.0.1 a linked `pack.toml` is a DIRECTIVE leak and not a content leak, and it composes with the two escapes the neighbouring bullets describe. The sentence is false of one of the three files it quantifies over, in the UNDERSTATING direction.

All three are refused at HEAD, each naming the file, each at exit 2, each writing nothing. I measured that too.

SEVERITY: `low`, and this was the closest call of the four. I considered `medium` on one ground and rejected it on two.

The ground for `medium`: a reader doing forensics on a 0.0.1 run would, under the sentence as written, look for the outside `pack.toml`'s TEXT inside their scaffolded project, find none, and conclude nothing leaked. The true behaviour tells them to look for files planted outside their output directory. That is a wrong action pointer, not merely an inaccuracy.

The two grounds against: the same section already discloses both halves independently. `CHANGELOG.md:38` states that a `dest` could write outside `--output-dir` at exit 0 while reporting "Wrote to `<output-dir>`", and `CHANGELOG.md:36` states the read-side escape in full. Only the COMPOSITION is mis-described. And the remedy for all three files is the same release, so no upgrade decision turns on it. `low`, and the human should know it is the one I would most readily hear an argument about.

## `P3` (low, INTERNAL): `read_optional`'s doc names a gate that refutes it

QUOTED, `src/manifest.rs:524-528`:

> a caller may still write `.unwrap_or_default()` on this too, and a caller that does passes `cargo clippy --all-targets -- -D warnings`.

THREE probe trees, each one expression different from HEAD, each with its own target directory. I confirmed the textual diff of each against HEAD before building.

```
probe1  pack_principles:  match source.read_optional("principles.toml").unwrap_or_default() {
        cargo clippy --all-targets -- -D warnings   EXIT 101
        error: variant `Read` is never constructed

probe2  build_assets:     source.read_optional("instrument.md").unwrap_or_default().unwrap_or_default()
        cargo clippy --all-targets -- -D warnings   EXIT 101
        error: variant `UnreadablePackFile` is never constructed

probe3  a NEW caller:     builtin.insert("banner".to_string(), source.read_optional("banner.md").unwrap_or_default().unwrap_or_default());
        cargo clippy --all-targets -- -D warnings   EXIT 0
```

There are exactly two callers of `read_optional` in production code (`src/main.rs:264` and `src/main.rs:298`). Each is the only construction site of its error variant, so a swallow written at either makes the variant dead and fails the project's own gate. The claim holds only for a caller that does not exist yet.

The paragraph's first sentence and its last ("The invariant is held by review, not by the compiler") are both true, and the last is the honest one round 3 asked for. The clause between them is wider than any measurement behind it.

SEVERITY: `low`. It is a doc comment on a private-in-practice method of a binary-only crate. `Cargo.toml` has no `[lib]` section and there is no `src/lib.rs`, which I verified, so docs.rs renders nothing and no downstream crate can read it. No exit code and no output byte turns on it.

## `P4` (low, USER-FACING): the 0.0.2 section does not describe the message this commit adds

Measured against 0.0.1, four invocations change what they print:

```
--template <a plain file>          MAIN: error: Not a directory (os error 20)
--template <a symbolic-link loop>  MAIN: error: Too many levels of symbolic links (os error 40)
--template <a path that is absent> MAIN: error: No such file or directory (os error 2)
--template <a dangling link>       MAIN: error: No such file or directory (os error 2)
  HEAD, all four:  error: --template `<path>` must name a directory
```

Exit 2 and an empty output directory in every case, at both binaries.

The 0.0.2 section is 36 lines. `grep -c 'name a directory'` returns 0 for the section and 0 for `README.md`. The three occurrences of `--template` in the section are `CL:36`'s surviving-shape clause, `CL:36`'s recourse paragraph, and `CL:38`'s external-input clause. None is about a root that is not a directory.

The standard I apply is the section's own, which round 3 tested in both directions and which the round 3 triage endorsed: the section describes everything user-visible in `git diff main..HEAD` and nothing that is not. This is the only change to that diff since round 3 measured it, so it is the only new gap.

VALID, and it is the weakest of the four. The argument against is real: Keep a Changelog asks for notable changes, this is message quality on an invalid invocation, and a reader who never mistypes `--template` never meets it. I still rule it valid, because the section already documents message FORM in two places (`CL:36`'s new last sentence and `CL:38`'s `dest` sentence), so the section's own threshold sits below this and not above it.

SEVERITY: `low`. It is an omission. Nothing false is stated.

## The user-facing versus internal split

I apply round 3's test, which I re-verified rather than assumed: can a person who INSTALLS the crate encounter it. `cargo package --list` reports 403 files including `CHANGELOG.md`, `README.md` and every `src/` and `tests/` file. `Cargo.toml` names `readme = "README.md"` and declares no `[lib]`, and `src/lib.rs` does not exist, so the crate is binary-only and docs.rs publishes no rustdoc for it.

WHAT A PERSON WHO INSTALLS THE CRATE CAN ENCOUNTER:

- `P1` half A. Typing `--template <a pack under a directory you cannot traverse>` prints a false statement about the path. The binary prints it.
- `P1` half B's MESSAGE, though not its doc. `--template <an unreadable pack root>` prints `could not read the pack's principles.toml` for a pack with no such file. This is `C1`'s class surviving in the one shape the new check does not catch, and on this shape it is a regression against 0.0.1's unlabelled message.
- `P2`. `CHANGELOG.md` ships inside the `.crate` tarball and is this project's release-notes artefact by its own release mechanics.
- `P4`. Same file, same route.

WHAT REQUIRES OPENING THE SOURCE:

- `P1` half B's DOC SENTENCE at `src/main.rs:229-231`. A doc comment on a private enum in a binary. Reaching it means being a maintainer or an auditor, not an installer.
- `P3`. A doc comment on `PackSource::read_optional`. `pub` here means visible across the modules of one binary, not published API. With no lib target it is rendered nowhere.

So: THREE user-facing (`P1` half A, `P2`, `P4`), plus the user-facing message face of `P1` half B; TWO internal (`P1` half B's doc, `P3`). Both internal ones ship inside the tarball and neither is published anywhere a user reads.

## The residual question, per finding

The project's rule (`AGENTS.md:57`): "A valid finding may instead be resolved by consciously accepting its residual risk and recording that; an accepted risk does not block convergence."

I read that rule as being about RISK. A residual is an accepted exposure whose cost is judged not worth the fix. It is not a mechanism for accepting a statement that has been measured false and will then be published as a factual account of a previous release. That distinction decides three of my four answers.

### `P1`: YES, both halves, with one obligation attached

Half A can honestly ship. No version of this tool gets the shape right, no outcome changes, the shape needs a specific permission arrangement to reach, and the fix is a new error-kind arm at the one site, which is a new unpinned code surface in a delivery increment. Accepting it means accepting a KNOWN breach of the tree's own stated standard at `src/safe_path.rs:66-68`, and the record must say so in those words rather than calling it a wart.

Half B's doc sentence can honestly ship: it is internal, unpublished, and false only for a maintainer reading it while debugging a permission failure.

THE OBLIGATION IS NOT OPTIONAL AND DOES NOT DEPEND ON THE RESIDUAL DECISION. The round record currently carries `X7` as CLOSED. That is wrong on my own measurement. Whether the human fixes the sentence or accepts it, `X7` must be reclassified from "closed by `C1`" to either "open" or "accepted residual", and the round 3 triage's premise at `v002-r3-triage.md:412` must be marked false. A converged increment whose ledger says a finding was fixed when it was not is a defect in the evidence the workflow exists to produce.

### `P2`: NO

This is the one finding I rule cannot honestly be a recorded residual.

It is a FALSE FACTUAL SENTENCE about the previous release's behaviour, in a file that ships to crates.io as the release notes, in a bullet whose entire reason for existing is disclosure. The round 3 triage required that disclosure to be ADDED, on the ground that `CL:36`'s enumeration excludes the three literals. One third of the added disclosure mis-describes the mechanism, and it mis-describes it in the direction that points a forensic reader at the wrong place.

Accepting it as a residual would mean recording "we measured this sentence false and published it anyway". Principle 6 (Ground decisions in evidence) is the principle that refuses it: the project has the measurement, in two independent findings files and now in this one, and publishing against a measurement you hold is the failure that principle names.

The fix is a five-word DELETION. Removing "and its contents inlined" leaves "In 0.0.1 each was read through a symbolic link out of the pack, the same leak as the two fields above on three more paths", which I measured true of all three files at the class level. No replacement prose, and no new comparative claim, is required.

### `P3`: YES

Internal, unpublished, no rustdoc, no lib target. Nothing a user reads. The wrong action it invites is narrow: a maintainer who reverts a call site to swallow, is stopped by clippy, and concludes the doc is wrong about the guard rather than that the guard is real at that site.

It is also a one-clause deletion that leaves a paragraph entirely true, so accepting it buys almost nothing. I rule it CAN be a residual and note that it is the cheapest of the four to simply close.

### `P4`: YES, and this is the easiest of the four

It is an OMISSION. Nothing false is published. A section that is silent about a message-quality improvement on an invalid invocation is incomplete, not wrong. Under the test that decides `P2` (no measured-false sentence ships), `P4` passes without argument.

## The re-seeding measurement

Attributed with `git log -S` per site and confirmed against the three binaries.

| finding | site or behaviour | authored or introduced by | round 3 fix pass caused it? |
| --- | --- | --- | --- |
| `P1` half A | the `--template` root message and its comment | `fde1d60` | INTRODUCED |
| `P1` half B | `src/main.rs:229-231` doc | `62b6571` (the ROUND 2 fix pass) | no; but its FALSE CLOSURE is `fde1d60`'s |
| `P2` | `CHANGELOG.md:37`, the whole replacement bullet | `fde1d60` | INTRODUCED |
| `P3` | `src/manifest.rs:524-528`, the new clause | `fde1d60` | INTRODUCED |
| `P4` | the gap the new message opens | `fde1d60` | INTRODUCED |

THE RATE, on the same strict definition rounds 2 and 3 used (text or behaviour the fix pass INTRODUCED), counting defect SITES so the numbers compare: 4 of 5. Round 3 measured 7 of 8. Round 2 measured 3 of 6.

```
round 2:  3 of 6   50 percent
round 3:  7 of 8   88 percent
round 4:  4 of 5   80 percent
```

So the RATE is flat within noise and the ABSOLUTE COUNT fell, from 8 findings and 7 re-seeded to 5 findings and 4 re-seeded.

### The text reviewer's claim about the deletion method: CONFIRMED, with one correction

The claim was that deletion cut the crop rather than eliminating it. I checked both halves.

ROUND 3'S FALSIFIED CLAIMS ARE ALL GONE. I grepped each falsified string at HEAD and at `HEAD~1`:

```
                                                     HEAD  HEAD~1
src/safe_path.rs      "plan::source. joins"            0      1     X1
tests/...             "first read a run makes"         0      1     X2
src/manifest.rs       "at any level before"            0      0*    X3
src/manifest.rs       "explicit arm"                   0      1     X4
CHANGELOG.md          "read_optional. now separates"   0      1     X5
README.md             "This is the same rule"          0      1     X6
CHANGELOG.md          "Every containment refusal ..."  0      1     Z1
```

`*` X3's sentence wraps across two comment lines, so my single-line grep misses it at `HEAD~1`. `git diff HEAD~1..HEAD -- src/manifest.rs` shows the deletion directly.

NOTHING INHERITED THEIR FALSITY. I verified each replacement rather than trusting the deletion. `README.md:242` reads "the trade taken is that a loud refusal beats silently reading the wrong file", so `README.md:325`'s surviving "the same trade" is true. `src/safe_path.rs` now names no joiner at all. `src/manifest.rs:1319`'s new comment claims only what the test does, and `README.md:362` and `README.md:327` cover the absence of both optional files as it says. `tests/...:441` keeps the one true sentence.

FOUR OF THE FIVE NEW COMPARATIVE CLAIMS HOLD, and I ran each myself rather than confirming the reviewer:

```
"byte for byte what 0.0.1 produced"            AGENTS.md md5 5a28f5e12a01946aaad53f844b4db5fe at MAIN and at HEAD,
                                                with and without --instrument.  TRUE
"indistinguishable from a pack shipping neither" the unreadable run at MAIN gives the SAME md5 as the absence run.  TRUE
"produced ... at exit 0 with empty stderr"      MAIN exit=0, stderr exactly 0 bytes, AGENTS.md "P:\nI:\n".  TRUE
"already loud in 0.0.1" (a malformed file)      MAIN and HEAD both exit 2 with the same parse error.  TRUE
"its contents inlined"                          FALSE of pack.toml.  P2
```

I also ran `CL:36`'s new last sentence against four refusals (absolute `source`, `..` `source`, linked `source`, escaping `dest`). Every message names the value in backticks, then the cause in parentheses, then the rule. TRUE.

### The correction, and what it implies for a fifth round

The reviewer's framing is that the method cut the crop. The sharper measurement is WHERE the surviving crop grows, and I partitioned `fde1d60`'s seven edit sites by what the pass did at each:

| site | what the pass did | findings |
| --- | --- | --- |
| `CHANGELOG.md`, two bullets removed and one sentence moved verbatim | DELETE and MOVE | 0 |
| `README.md:325` | DELETE | 0 |
| `src/safe_path.rs:1-7` | DELETE | 0 |
| `tests/...:383-385` | DELETE | 0 |
| `src/manifest.rs:1319-1320` | delete, then WRITE a shorter replacement | 0 |
| `tests/...:445-469` | WRITE a new test and its comment | 0 |
| `src/manifest.rs:524-528` | delete, then WRITE a replacement clause | `P3` |
| `CHANGELOG.md:37` | delete, then WRITE a replacement bullet | `P2` |
| `src/main.rs:2110-2124` | WRITE new code, a comment and a message | `P1` half A, `P4` |

EVERY FINDING THIS ROUND IS AT A SITE WHERE THE PASS AUTHORED SOMETHING. EVERY PURE-DELETION SITE PRODUCED NOTHING. That is four sites deleted cleanly and zero defects from them, against four defects from the five sites where text or behaviour was written.

SO DELETION IS THE RIGHT INSTRUCTION, and it is not sufficient on its own, because a pass cannot delete its way to a new error message or a new CHANGELOG bullet. What the measurement supports for a fifth round is narrower and stronger than "delete more": **every remaining edit must be a deletion, and where a claim cannot be deleted outright its replacement must be copied verbatim from a measurement already in a findings file.** Each of the four remaining edits happens to satisfy that.

- `P2`: delete five words. The surviving sentence is true, measured above.
- `P3`: delete one clause. The surviving paragraph is true.
- `P1` half B: delete three words and widen one noun. No new claim.
- The `X7` record correction: a ledger edit, no tree text at all.

## The round outcome

ROUND 4 IS NOT CLEAN. Four valid findings, all `low`, so the consecutive-clean streak stays at 0 unless the human accepts them as residuals under the rule at `AGENTS.md:57`.

Rounds 1 to 4, on my reading of the settled record plus this ruling:

```
round 1:  5 valid, ceiling high    streak 0
round 2:  6 valid, ceiling high    streak 0
round 3:  8 valid, ceiling low     streak 0
round 4:  4 valid, ceiling low     streak 0
```

The trend is real and it is worth stating without dressing it up: the severity ceiling fell from `high` to `low` after round 2 and has stayed there, the count is falling, and every remaining defect is a sentence rather than a behaviour. The CODE is in a different condition from the TEXT. Across four rounds, the code half of this round produced one finding whose two artefacts are a message string and a doc comment, no test is vacuous, and no legitimate invocation regressed.

I confirmed the last claim myself rather than accepting it. Eight legitimate `--template` shapes, run at `PRE` and at `HEAD` with fresh output directories:

```
absolute path        pre=0 head=0  31 files  tree IDENTICAL
trailing slash       pre=0 head=0  31 files  tree IDENTICAL
symbolic link to dir pre=0 head=0  31 files  tree IDENTICAL
link to a link       pre=0 head=0  31 files  tree IDENTICAL
`..` resolving back  pre=0 head=0  31 files  tree IDENTICAL
`./` component       pre=0 head=0  31 files  tree IDENTICAL
relative path        pre=0 head=0  31 files  tree IDENTICAL
no --template at all       0     0  31 files  tree IDENTICAL
```

The root check is purely subtractive on the accepted set, which also follows structurally: a read that succeeded needed `fs::canonicalize(root)` to succeed, and a path whose `stat` fails cannot canonicalise.

## Is the change safe to merge and publish as it stands?

NO, on ONE ground only, and the ground is `P2`.

Everything else is safe. The three spec defects (`F1`, `F4`, `F4b`) are fixed and I measured the refusals firing on both boundaries. All six gates I can run are green. No legitimate invocation regresses. The new root check cannot admit anything the previous commit refused. The 468-test suite passes and clippy is clean under `-D warnings`. No non-ASCII line entered any changed file.

What stops it is that publishing means putting a sentence on crates.io that this project has measured false, about how the previous release leaked, in the bullet that exists to disclose that leak.

THE MINIMUM SET THAT MAKES IT SAFE TO MERGE AND PUBLISH:

1. `CHANGELOG.md:37`: delete "and its contents inlined" (five words). Nothing else in the bullet changes.
2. The round record: reclassify `X7` from closed to open or to accepted residual, and mark `v002-r3-triage.md:412`'s premise false. This is a ledger correction, not a tree edit, and it is required whichever way the human decides on the residuals.

That is the whole minimum. `P1`, `P3` and `P4` can all ship as recorded residuals without a false sentence reaching a user.

## Recommendation on how the increment ends

Not my decision. Here is the position, the options and my recommendation, judged against the plan's own Project Principles by name.

WHAT MY RULING CHANGES. The orchestrator framed two options, and one of them is narrower than it looked: accepting ALL of round 4 as residuals is not honestly available, because `P2` cannot be a residual. So the real choice is between accepting three of four and fixing one, or accepting two and fixing two, or fixing all four.

### Option A: fix `P2` only, accept `P1`, `P3` and `P4` as residuals, run round 5

One five-word deletion plus the record correction. Round 5 lands at the cap. If it is clean the increment converges without escalation.

Against: it leaves three known-true findings unfixed when two of them (`P3` and `P1` half B) are also deletions, so it spends a whole round on the smallest possible diff and still carries the residuals into the release.

### Option B (RECOMMENDED): fix `P2`, `P3` and `P1` half B by deletion, accept `P1` half A and `P4` as residuals, correct the `X7` record, run round 5

Four edits, every one a deletion or a noun widened, no new sentence authored anywhere, plus a ledger correction that touches no tree text.

FOR IT, by principle:

- **Principle 6 (Ground decisions in evidence)** is the one that decides it. Three measured-false statements exist in the tree and the project holds the measurements. Two of the three cost a deletion. Publishing against a measurement you hold is exactly what this principle forbids, and it is also the mechanism round 3 named against round 2 and then repeated on `X7`.
- **Principle 2 (Minimal by default)** is what keeps `P1` half A out of the fix. A second error-kind arm is new unpinned code in a delivery increment whose stated purpose is to ship, and no outcome changes without it. Accept the residual and name the standard it breaches.
- **Principle 1 (Prefer the cleaner long-term architecture over the smallest diff)** supports fixing `P1` half B rather than leaving it: the doc sentence is what a future maintainer reasons from, and correcting it costs three words. Option A's argument for leaving it is a smallest-diff argument, which is the argument this principle refuses.
- The spec's own closed-scope sentence is respected. No fix in this set adds a behaviour, a flag, a test or a message. Every one removes text.

### Option C: accept everything including `P2` and converge now

Cheapest, converges without spending round 5, and I advise against it under Principle 6 for the reason in the `P2` section. If the human takes it anyway, the record must state in plain words that a sentence measured false was published knowingly, so the decision is auditable rather than invisible.

### Do I expect a round 5 to be clean?

More likely than any round so far, and I will not pretend it is certain.

THE EVIDENCE FOR: across `fde1d60`, every pure-deletion site produced zero findings and every authored-text site produced a finding. Option B's fix set is four deletions and a ledger edit, which is the shape that has a perfect record across this round. The code is unchanged by it, so the code half of round 5 has nothing new to review. The falsification rate on newly authored comparative claims already fell from 6 of 6 to 1 of 5.

THE EVIDENCE AGAINST: four rounds have each found something, the last two found only text defects, and this increment's text has now been rewritten three times with a new defect each time. A round 5 reviewer given the whole 0.0.2 section can still find a claim nobody has run, because the section is long and its claims are comparative by nature. Round 4's own reviewers checked 50 assertions and confirmed 45, so the assertion surface is large and thinly pinned.

MY HONEST ANSWER: if the round 5 fix pass is held to deletions and forbidden to author a replacement sentence anywhere, I expect round 5 to be clean, and Option B is the way to get there. If the fix pass is allowed to rewrite `CHANGELOG.md:37` into a corrected description of what a linked `pack.toml` did at 0.0.1, I expect round 5 to find something in that new sentence, because that is precisely the shape that failed in rounds 3 and 4. The instruction matters more than the finding count.
