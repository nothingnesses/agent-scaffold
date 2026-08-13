# `ship-v0-0-2-inc1` round 3: REVIEWER, the read-boundary blast radius

Independent reviewer. I did not write this change and did not review rounds 1 or 2. This is the SCOPED behaviour half of round 3, covering the blast radius `v002-r2-triage.md` enumerated. Another reviewer holds the text sites. Every figure below is my own measurement, built from the source rather than from the implementer's report.

## Artifact and commits

Worktree `.claude/worktrees/r3-boundary`, detached at `53c3a27`. The commit under review is the single fix pass:

- `53c3a27` fix: report a refused pack literal instead of calling it absent (`B1` both halves, `B2`, claims 2 to 5).

Its parent, `f2308d6`, is the round 2 tree. `main` is `9446608`. `git diff f2308d6..53c3a27` touches six files: `CHANGELOG.md`, `README.md`, `src/main.rs`, `src/manifest.rs`, `src/safe_path.rs`, `tests/pack_source_stays_inside_the_pack.rs` (354 insertions, 38 deletions).

Read in full before starting: `docs/plans/agent-scaffold.reviews/v002-r2-triage.md` and `docs/plans/agent-scaffold.steps/ship-v0-0-2.md`.

## Method

THREE release binaries, one `CARGO_TARGET_DIR` each, from three `git archive | tar -x` extracts, verified distinct by `md5sum`:

```
dae9a78cf1b249a6c796240413389134  tgt-head/release/agent-scaffold   (HEAD, 53c3a27, the fix pass)
d0ecc77d0097ebef8bf50fa86585435e  tgt-pre/release/agent-scaffold    (f2308d6, the round 2 tree, PRE-fix)
0f1394eef0f68328f3cdfa329ffc3980  tgt-main/release/agent-scaffold   (main, 9446608)
```

Below, `HEAD`, `PRE` and `MAIN` are those three. `PRE` isolates what this fix pass changed. `MAIN` isolates what the whole increment changed.

A FOURTH tree, a copy of the HEAD source, carried three source mutations and one pack mutation for the non-vacuity work. Every fixture, escape target, symlink target and cargo target directory lives under my own scratch subdirectory. No tracked file was modified anywhere except this findings file.

Gates I ran myself at HEAD:

| Gate | Result |
| --- | --- |
| `cargo test` | 467 passed, 0 failed, across 11 result lines |
| `cargo clippy --all-targets -- -D warnings` | clean |
| `validate --source ... --metrics ...` | `332 records, valid`; `99 steps, 75 questions, valid`, exit 0 |
| `validate --source ... --workflow` | `workflow invariants hold`, exit 0 |
| `render --check --strict` | `up to date`, exit 0 |
| ASCII check on all six changed files | `0` on every file |
| `validate --plan ... --metrics ...` | EXACTLY ONE problem, the pre-existing `Q-43` `superseded by` one the spec excludes, exit 1 |

The one finding below is not a gate failure.

## My own consumer enumeration

Built from `grep -rn '\.read(\|read_optional' src/ tests/` and then read at each site, not taken from the implementer's report or from round 2's triage. Test-only callers are excluded and listed under the table.

| # | site | primitive | maps a refusal to | absence handled as | exit |
| --- | --- | --- | --- | --- | --- |
| 1 | `src/manifest.rs:548` `PackSource::manifest`, `read("pack.toml")` | `read` | `io::Error::from` -> `LoadError::Io`, message carries the whole refusal text | absence is an error: `pack.toml` is required | 2 |
| 2 | `src/manifest.rs:670` `module_guidance`, `read(guidance)` | `read` | `LoadError::UnsafeModuleGuidance { module, guidance }` | absence is an error: a declared `guidance` is required | 2 |
| 3 | `src/manifest.rs:794` `load`, `read(&spec.source)` | `read` | `LoadError::UnsafeAssetSource { source }` | absence is an error: a declared `[[asset]]` source is required | 2 |
| 4 | `src/main.rs:264` `pack_principles`, `read_optional("principles.toml")` | `read_optional` | `PrinciplesError::Read` -> `error: could not read the pack's principles.toml: ...` | `Ok(None)` -> empty principle set, silent | 2 |
| 5 | `src/main.rs:298` `build_assets`, `read_optional("instrument.md")` | `read_optional` | `LoadError::UnreadablePackFile { rel, problem }` -> `error: could not read the pack's \`instrument.md\`: ...` | `Ok(None)` -> empty block, silent | 2 |

FIVE production consumers, exactly the five round 2's triage listed at `v002-r2-triage.md:78-84`, with sites 4 and 5 moved from `read` to `read_optional`. MY ENUMERATION MATCHES THE IMPLEMENTER'S AND ROUND 2'S. There is no sixth production consumer and no consumer of `read_optional` outside `src/main.rs`.

Test-only callers, verified `#[cfg(test)]` and excluded: `src/manifest.rs:1160`, `:1168`, `:1175`, `:1310`, `:1328-1336`, `:1355`; `src/main.rs:2384`, `:2399`, `:2409`; `src/agents_md_drift.rs:135` (inside `#[cfg(test)] mod tests`, line 102).

NONE OF THE FIVE ANSWERS A REFUSAL AS ABSENCE. Measured for every one of the five, not read off the source: see the PASSED section.

## Verdict table

Severity is absolute impact if left unfixed, rating the finding rather than the exploit.

| id | severity | one line |
| --- | --- | --- |
| `C1` | **low** | A `--template` that names a plain file instead of a directory is now reported as a failure to read the pack's `principles.toml`, a file the user's mistake never involved; `main` and the round 2 tree both report the plain cause. |

ONE valid finding, `low`. No `critical`, no `high`, no `medium`.

The two defects this round exists to check, `B1`'s code half and the absence-stays-silent contract, are both CLOSED and both PINNED by tests I confirmed non-vacuous. `C1` is a message-attribution wart introduced by the same edit, and it cannot change a verdict or an outcome.

## `C1` (low): a bad `--template` root is reported against `principles.toml`

### What happens

`pack_principles` is the FIRST thing in `run_scaffold` that touches the pack's filesystem (`src/main.rs:2133`, before `build_assets` and so before `pack.toml` is ever read). Its read now propagates every non-`NotFound` error instead of discarding it, which is correct and is the whole point of the change. The side effect is that when the failure belongs to the `--template` ROOT rather than to any file in it, the first read to hit it is `principles.toml`, and that is the file the message names.

`fs::canonicalize` on a `<file>/principles.toml` path returns `ENOTDIR`, whose `io::ErrorKind` is `NotADirectory` rather than `NotFound`, so `read_optional` correctly does not fold it into `Ok(None)` and the caller correctly reports it.

### Reproduction and measured output

```
mkdir -p $SB/p6b/out
printf '[[asset]]\nsource = "a.md"\ndest = "a.md"\nownership = "working"\n' > $SB/p6b/pack.toml
<bin> scaffold --template $SB/p6b/pack.toml --output-dir $SB/p6b/out --vcs none --write
```

Measured, all three binaries:

```
HEAD  exit=2  error: could not read the pack's principles.toml: Not a directory (os error 20)
PRE   exit=2  error: Not a directory (os error 20)
MAIN  exit=2  error: Not a directory (os error 20)
```

CONTROL, the same mistake in its other common form, a `--template` naming a directory that does not exist or that is empty. Here `canonicalize` fails with `NotFound`, `read_optional` folds it to `Ok(None)`, and the error surfaces later from the `pack.toml` read instead:

```
-- --template names an empty existing directory:
HEAD  exit=2  error: No such file or directory (os error 2)
PRE   exit=2  error: No such file or directory (os error 2)
MAIN  exit=2  error: No such file or directory (os error 2)

-- --template names a path that does not exist:
HEAD  exit=2  error: No such file or directory (os error 2)
PRE   exit=2  error: No such file or directory (os error 2)
MAIN  exit=2  error: No such file or directory (os error 2)
```

So the misattribution is specific to the not-a-directory root, and HEAD is the only binary of the three that produces it. Nothing is written in any of these cases, at any of the three binaries.

### Why I am raising it, and the argument against

The argument against, stated first because it is real: the sentence is literally TRUE (the tool could not read that path, and the cause is a not-a-directory error), and it is strictly more specific than the bare `Not a directory` that `main` prints. A reader could call this an improvement.

I raise it because it fails the project's own stated standard for this exact class, written by this same commit. `src/main.rs:229-231` justifies splitting `PrinciplesError::Read` from `PrinciplesError::Parse` on the ground that "telling the user it did not parse would name the wrong step", and `src/main.rs:2135-2137` repeats it: "Printing 'could not parse' for a containment refusal would name a step that never ran." Naming `principles.toml` for a failure that belongs to the `--template` root is the same shape of misattribution one level up. The user's mistake is that `--template` names a file; the message sends them to look at a `principles.toml` they do not have and never needed.

The inconsistency inside the change makes it sharper: for one form of a bad `--template` the message is unlabelled, and for another form it is labelled with a file that is not the problem, and which of the two you get turns on an `io::ErrorKind` the user cannot see.

### Severity: `low`, ruled

Both the old and the new behaviour exit 2 and write nothing, so no verdict and no outcome can change. Under this project's calibration that is `low` and cannot reach `medium`. The impact is one misdirected user on one plausible mistake (`--template ./pack.toml` for `--template ./pack/`).

### Remedy direction, not a prescription

Two shapes exist and I am not choosing between them.

The cheaper one is to check that the `--template` path is a directory once, where the `PackSource::Directory` is constructed (`src/main.rs:2111-2113`), and refuse with a message naming `--template` and the path. That also fixes the unlabelled empty-directory and missing-directory cases in the same place, and it is the only site that knows the failure is about the root rather than about a file.

The narrower one is to leave the root unchecked and have `pack_principles` report the root cause when the error is not attributable to the file. That is harder to write honestly, because at the point of failure the caller cannot tell the two apart without re-stating the check.

MUST NOT BE EDITED, whichever is taken:

- `read_optional`'s `NotFound`-only mapping (`src/manifest.rs:538`). Widening it to swallow `NotADirectory` would reopen `B1`'s class on a new error kind, and my M1 mutation below shows exactly which tests that breaks.
- `PrinciplesError`'s `Read`/`Parse` split (`src/main.rs:228-235`) and the two comments quoted above. They are correct and they are the standard this finding is judged against.
- The four new tests in `tests/pack_source_stays_inside_the_pack.rs` and the two in `manifest::tests`. All six are non-vacuous, measured below, and none of them pins the `--template`-root message.

## Checks that PASSED

Listed so the triager knows what this round covers. Each was run, not reasoned about.

### The refusal path, all three literals, both flags

A pack whose `pack.toml`, `principles.toml` or `instrument.md` is a symbolic link OUT of the pack directory, under `--write` and under `--dry-run`, against all three binaries. Twenty-four runs. At HEAD every one of the six literal-and-flag combinations gives exit 2, a message naming the file, empty stdout (no plan preview), and zero files written:

```
pack.toml        --write / --dry-run
  HEAD  exit=2  files=0  error: `pack.toml` is not a contained pack path (it resolves outside the pack
                         directory, through a symbolic link); a pack path must be relative, carry no
                         `..` component, and resolve to a location inside the pack directory
  PRE   exit=2  files=0  (same message)
  MAIN  exit=0  files=1  create AGENTS.md / Wrote to ...

principles.toml  --write / --dry-run
  HEAD  exit=2  files=0  error: could not read the pack's principles.toml: `principles.toml` is not a
                         contained pack path (...)
  PRE   exit=0  files=1  (silent, the B1 defect)
  MAIN  exit=0  files=1

instrument.md    --write / --dry-run
  HEAD  exit=2  files=0  error: could not read the pack's `instrument.md`: `instrument.md` is not a
                         contained pack path (...)
  PRE   exit=0  files=1  (silent, the B1 defect)
  MAIN  exit=0  files=1
```

`B1`'s own reproduction, the REPOSITORY'S OWN pack copied to a directory with `instrument.md` moved out and linked back, under `--instrument --write`:

```
HEAD  exit=2  nothing written    error: could not read the pack's `instrument.md`: ... (names the file)
PRE   exit=0  AGENTS.md 49433 bytes, `## Instrumentation (metrics logging)` x0, `dismissal_recheck` x0
MAIN  exit=0  AGENTS.md 58989 bytes, `## Instrumentation (metrics logging)` x1, `dismissal_recheck` x1
```

The 9556-byte silent drop round 2 measured is closed. The same pack under `--module checks --with-precommit-hook --instrument --vcs git`, both `--write` and `--dry-run`: HEAD exits 2, writes nothing, and installs no hook.

NOTHING IS CREATED AT ALL, not merely nothing written. With a `--output-dir` that does not yet exist and `--vcs git`, a refused literal at HEAD leaves the output directory ITSELF uncreated and runs no `git init`; `PRE` and `MAIN` both create the directory and initialise the repository before proceeding. The refusal is upstream of `init_plan` (`src/main.rs:2213`), which is Principle 3 (Safe on existing projects) held at the strongest point available.

### Consumers 1, 2 and 3, the `read` half

Neither `module_guidance` nor `load` was touched by this commit, so this is a regression check rather than a new-behaviour one. A pack declaring a `[[module]]` with a `guidance` and an `[[asset]]` with a `source`, each in turn a link out, missing, and legitimate:

```
module guidance link OUT   --write / --dry-run:  HEAD exit=2 files=0, `module `m` guidance file `g.md`
                                                 is not a contained pack path (...)`; PRE identical;
                                                 MAIN exit=0 and splices SECRET GUIDANCE into AGENTS.md
module guidance MISSING    --write:              HEAD/PRE/MAIN all exit=2, `... could not be read: No
                                                 such file or directory`
module guidance legitimate --write:              HEAD/PRE/MAIN all exit=0, identical output
asset source link OUT      --write / --dry-run:  HEAD exit=2 files=0, `asset source `a.md` is not a
                                                 contained pack path (...)`; PRE identical; MAIN exit=0
asset source MISSING       --write:              HEAD/PRE/MAIN all exit=2
```

Each refusal carries its own field label, none reports another field's, and none is answered as absence.

### The absence-stays-silent contract

A pack shipping NEITHER `principles.toml` NOR `instrument.md`, under `--instrument --write`:

```
HEAD  exit=0  stderr empty  AGENTS.md = "P:\nI:\n"
PRE   exit=0  stderr empty  AGENTS.md = "P:\nI:\n"
MAIN  exit=0  stderr empty  AGENTS.md = "P:\nI:\n"
```

Byte-identical across all three. `README.md:362` ("A pack that ships no `principles.toml` simply has no principles to select"; it was `:360` before this commit added two README lines) and `src/manifest.rs:95-97` both survive. The remedy did not over-tighten.

### The new tests are non-vacuous

Four source mutations in a copy of the HEAD tree, each built and run with `cargo test --no-fail-fast`. The baseline is 467 passed, 0 failed.

M1, `read_optional` folds EVERY error into `Ok(None)` (the pre-fix behaviour restored at the primitive): `Err(error) => Err(error)` replaced with `Err(_) => Ok(None)`. THREE tests fail, and they are the three that name the behaviour:

```
manifest::tests::read_optional_reports_a_refused_literal_rather_than_calling_it_absent  FAILED
a_linked_principles_file_is_reported_not_silently_dropped                               FAILED
a_linked_instrument_fragment_is_reported_not_silently_dropped                           FAILED
```

The two absence tests stay green under M1, which is correct: M1 does not touch absence.

M2, `read_optional` never returns `Ok(None)`, so absence becomes loud: the `NotFound` guard made unreachable. The absence pins fail first, which is what makes them real:

```
manifest::tests::read_optional_answers_absence_with_none_and_never_with_an_error  FAILED
tests::a_pack_without_principles_has_an_empty_set (in src/main.rs)               FAILED
a_pack_shipping_neither_optional_literal_still_scaffolds                         FAILED
```

plus 12 cascading failures in the two containment integration files, whose fixture packs ship no `principles.toml`. The over-tightening the round 2 triage warned about would be caught at three independent levels.

M3, `read_optional` left correct but both CALLERS reverted to swallowing (`Err(_) => Ok(Vec::new())` and `.unwrap_or_default()`): the two integration tests fail and the unit tests do not.

```
a_linked_principles_file_is_reported_not_silently_dropped     FAILED
a_linked_instrument_fragment_is_reported_not_silently_dropped FAILED
(409 lib tests still pass)
```

This is the important one for the triager. It shows that the caller-level behaviour is pinned INDEPENDENTLY of the primitive, so a future edit that keeps `read_optional` honest and re-breaks a call site is caught. That is the exact regression shape round 2 found, and it is now held.

`a_linked_pack_manifest_is_refused_with_a_message_naming_it` needed no mutation: the refusal it pins is accepted at `MAIN` (exit 0, one file written, measured above), so the test discriminates HEAD from `main` directly.

### The embedded pack's `NotFound` behaviour

The `Embedded` arm (`src/manifest.rs:483-492`) is `dir.get_file(rel).and_then(contents_utf8).ok_or_else(|| ReadError::Io(io::Error::new(io::ErrorKind::NotFound, ...)))`. A missing embedded file therefore reports as `NotFound` and `read_optional` folds it to `Ok(None)`, so the built-in pack answers "ships none" the same way a directory pack does. Pinned by `builtin().read_optional("no-such-file.md") == None` and `builtin().read_optional("principles.toml").is_some()` in the new unit test, both of which I ran.

The one conflation in that arm is that `and_then(contents_utf8)` maps a file that EXISTS but is not valid UTF-8 onto the same `NotFound`, which would make it read as an absence. I CHECKED WHETHER THAT IS REACHABLE AND IT IS NOT: appending a `0xff` byte to `pack/instrument.md` in a copy of the tree fails the build, not a test.

```
error: `../pack/instrument.md` wasn't a utf-8 file
error: could not compile `agent-scaffold` (bin "agent-scaffold" test) due to 1 previous error
```

A non-UTF-8 file in `pack/` cannot ship, so the conflation cannot be reached by any binary that exists. Not a finding, and recorded so nobody has to re-derive it.

### The built-in pack is unchanged, HEAD versus MAIN

Twenty invocations of the embedded pack covering every module and flag combination the CLI admits: no module, `checks`, `isolation`, both; `--instrument` on and off; `--with-precommit-hook`; `--principles default|all|none`; `--principle-detail full|name|summary`; `--force`; `--write`, `--dry-run`, `--list-principles`. For each, HEAD and MAIN output trees compared with `diff -r` and stdout compared after path normalisation.

ALL TWENTY: identical exit code, identical file count (31 core, 36 with `checks`), `diff -r` clean, stdout identical. `PRE` matches on exit code throughout. The `{{instrument}}` slot is confirmed live on the embedded path, not merely absent-and-equal: `--write` gives a 49433-byte `AGENTS.md` with zero `## Instrumentation (metrics logging)` headings and `--write --instrument` gives 58989 bytes with one.

### Legitimate packs that must keep working

| shape | HEAD | PRE | MAIN |
| --- | --- | --- | --- |
| plain directory pack, both literals real | exit 0, `P:1. My rule - One sentence.` / `I:INSTRUMENT FRAGMENT` | same | same |
| all three literals are links INSIDE the pack (`pack.toml`, `principles.toml`, `instrument.md` to `sub/`) | exit 0, full content | same | same |
| `--template` names a symbolic link TO the pack directory | exit 0, 1 file | same | same |
| pack ships neither optional literal | exit 0, empty blocks | same | same |
| malformed `principles.toml` | exit 2, `could not parse the pack's principles.toml: TOML parse error ... missing field \`name\`` | same | same |

No legitimate pack that worked at `main` fails at HEAD on any shape I could construct, and none that failed now passes. The one population that changes is the one `B2`'s CHANGELOG bullet already discloses.

### The implementer's own widenings

`pack_principles`' error type went from `toml::de::Error` to a new private `PrinciplesError`. Its only production consumer is `src/main.rs:2133`; the other three callers are `#[cfg(test)]`. The `Parse` message is preserved verbatim, which I verified by measurement rather than by reading: a malformed `principles.toml` prints the same `error: could not parse the pack's principles.toml: TOML parse error at line 1, column 1 ... missing field \`name\`` at HEAD, PRE and MAIN. No other site in the repository pins the old string (`grep -rn "could not parse the pack"` finds only `src/main.rs:246` and two round 2 review documents).

`LoadError::UnreadablePackFile` is a new variant with one construction site (`src/main.rs:299`) and one display arm (`src/manifest.rs:320`). No exhaustive match elsewhere had to change, and clippy is clean at `-D warnings`.

The widening the round 2 triage told the fix pass to disclose is disclosed, and disclosed in the right place. An unreadable optional literal is now loud where it was silent:

```
principles.toml with invalid UTF-8 contents:
  HEAD  exit=2  error: could not read the pack's principles.toml: stream did not contain valid UTF-8
  PRE   exit=0  scaffolds silently        MAIN  exit=0  scaffolds silently

instrument.md with invalid UTF-8 contents, under --instrument:
  HEAD  exit=2  error: could not read the pack's `instrument.md`: stream did not contain valid UTF-8
  PRE   exit=0  scaffolds silently        MAIN  exit=0  scaffolds silently

principles.toml is a DIRECTORY:
  HEAD  exit=2  error: could not read the pack's principles.toml: Is a directory (os error 21)
  PRE   exit=0  scaffolds silently        MAIN  exit=0  scaffolds silently
```

`CHANGELOG.md`'s Fixed section states it in terms: "An UNREADABLE file (permissions, or invalid UTF-8) becomes loud where it was silent, which matches a malformed one, already loud." It also states "ABSENCE IS UNCHANGED and stays silent", which I confirmed above. Both public claims are true as measured.

WITHOUT `--instrument`, an unreadable or refused `instrument.md` is not read and not reported, and the run succeeds. That is correct: nothing consumes the fragment, so there is no fact to report.

The read ORDER is `principles.toml`, then `instrument.md`, then `pack.toml`. A pack with more than one refused literal reports the first of those the run reaches, and the run still exits 2 naming a real cause, so the order costs nothing.

### Refusal beats a stale claim

Round 2's sharpest measurement was that a `principles.toml` that is both linked out AND malformed lost even its parse error at `PRE`. At HEAD it reports the refusal, not the parse:

```
HEAD  exit=2  error: could not read the pack's principles.toml: `principles.toml` is not a contained
              pack path (it resolves outside the pack directory, through a symbolic link); ...
PRE   exit=0  silent
MAIN  exit=2  error: could not parse the pack's principles.toml: TOML parse error ...
```

`MAIN` names the parse because it follows the link and reads the outside file, which is the hole. `HEAD` names the step that actually failed. The new integration test asserts `!stderr.contains("could not parse")` for exactly this, and it holds.

## Out-of-scope observations

Reported separately, not as findings, as instructed.

A DANGLING SYMBOLIC LINK out of the pack is treated as an absence at all three binaries. `resolved_within` canonicalises, canonicalisation of a dangling link fails with `NotFound`, and `read_optional` folds it to `Ok(None)`:

```
instrument.md -> <a target that does not exist>, under --instrument --write:
  HEAD exit=0, empty instrument block   PRE exit=0   MAIN exit=0
principles.toml -> <a target that does not exist>:
  HEAD exit=0, empty principle set      PRE exit=0   MAIN exit=0
```

I am not raising this. It is not a regression (`main` does the same), the pack ships no readable file either way so no artifact is wrong relative to what the pack actually contains, it matches what `open` does with a dangling link, and the round 2 triage's must-not-edit list pins `a_missing_pack_file_still_reports_as_missing_not_as_an_escape` for precisely this contract. It is recorded because a future change to the `NotFound` mapping would want to know that a stale store link lands here rather than in the refusal path.

A MISSING `[[asset]]` `source` reports `error: No such file or directory (os error 2)` with no field label, so it does not name the asset the way a refused one does. Identical at HEAD, `PRE` and `MAIN`, so it is pre-existing and outside this round.

`LoadError::UnreadablePackFile`'s doc (`src/manifest.rs:213-218`) describes the class as "a pack file the tool reads DIRECTLY, rather than through an `[[asset]]`", which is two files, but only `instrument.md` uses the variant; `principles.toml` uses `PrinciplesError::Read` in `src/main.rs` instead. No behaviour turns on it, and both messages name their file. Flagged for the text reviewer rather than ruled on here.

The two refusal messages differ cosmetically in whether the file name is backticked (`the pack's principles.toml` versus ``the pack's `instrument.md` ``). Cosmetic, no impact, noted for the text reviewer.

## Summary

One valid finding, `low` (`C1`). My consumer enumeration matched the implementer's and round 2's exactly: five production consumers, three on `read` and two on `read_optional`, with no sixth.

The result that matters: `B1` is closed on the behaviour side and the closure is held by tests that fail when the behaviour is removed, including one mutation that keeps `read_optional` honest and re-breaks only the call sites. Absence stays silent, the built-in pack is byte-identical to `main` across twenty module and flag combinations, all three literals are refused at exit 2 under both `--write` and `--dry-run` with nothing created, and no legitimate pack shape I could build changed its answer.
