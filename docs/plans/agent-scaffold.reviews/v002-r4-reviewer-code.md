# `ship-v0-0-2-inc1` round 4: REVIEWER (code half)

Independent reviewer. I did not write this change and did not review rounds 1 to 3. Every figure below is my own measurement, built in this session.

## Artifact

- Worktree `.claude/worktrees/r4-code`, detached at `406c278`.
- Artifact ruled on: the CODE half of `git diff HEAD~1..HEAD`, which is `src/main.rs:2111-2122` (the new `--template` root check) and one added integration test at `tests/pack_source_stays_inside_the_pack.rs:445-469`.
- Round 3 remedy this answers: `docs/plans/agent-scaffold.reviews/v002-r3-triage.md`, finding `C1` and its stated discharge of `X7`.
- Out of scope and not reopened: all prose, the containment mechanism, the read boundary, and everything settled in rounds 1 to 3.

## Method

TWO release binaries, one `CARGO_TARGET_DIR` each, built from two separate `git archive | tar -x` extracts, verified distinct by `md5sum`:

```
5a89f571e6212c896ca47c63250332e4  tgt-head/release/agent-scaffold   (406c278, HEAD)
3588c6d5747137387a17ab7a9dd7c89f  tgt-pre/release/agent-scaffold    (dbbf937, PRE = HEAD~1)
```

A third tree carried one source mutation at a time in its own target directory (`tgt-mut`), for the five mutations in the test section.

Every fixture, symbolic-link target, cargo target directory and test `TMPDIR` sits under my own scratch subdirectory. No tracked file in this worktree or in the main repository was modified, except this findings file: `git status --short` is empty at the end of the session. Two shapes required a permission bit, because no other construct produces `EACCES` on a directory. Both chmods are restored to `755`, and the restoration is recorded in the run output.

GATES I RAN MYSELF:

| gate | revision | result |
| --- | --- | --- |
| `cargo test` | HEAD | 468 passed, 0 failed |
| `cargo test` | PRE | 467 passed, 0 failed |
| `cargo clippy --all-targets -- -D warnings` | HEAD | clean, exit 0 |

The test count differs by exactly one, which matches one added test and no removed test.

## Enumeration of every input shape

Each row is one `--template` argument, run at both binaries with a fresh output directory and `--vcs none --write` unless the row says otherwise. `files` counts everything under the output directory after the run.

| # | input shape | HEAD | PRE | admitted? |
| --- | --- | --- | --- | --- |
| 1 | plain file | exit 2, 0 files, ``error: --template `<path>` must name a directory`` | exit 2, `error: could not read the pack's principles.toml: Not a directory (os error 20)` | rejected |
| 2 | symbolic link to a directory (valid pack) | exit 0, 31 files | exit 0, 31 files, tree byte-identical | ADMITTED, works |
| 3 | symbolic link to a plain file | exit 2, new message | exit 2, `principles.toml: Not a directory (os error 20)` | rejected |
| 4 | symbolic link self-loop (`a -> a`) | exit 2, new message | exit 2, `principles.toml: Too many levels of symbolic links (os error 40)` | rejected |
| 5 | symbolic link 2-cycle (`b -> c -> b`) | exit 2, new message | exit 2, `principles.toml: Too many levels of symbolic links (os error 40)` | rejected |
| 6 | dangling symbolic link | exit 2, new message | exit 2, bare `error: No such file or directory (os error 2)` | rejected |
| 7 | missing path | exit 2, new message | exit 2, bare `error: No such file or directory (os error 2)` | rejected |
| 8 | empty string | exit 2, `error: a value is required for '--template <TEMPLATE>' but none was supplied` | identical | rejected by clap, never reaches the check |
| 9 | trailing slash on a directory | exit 0, 31 files | exit 0, identical tree | ADMITTED, works |
| 10 | trailing slash on a plain file | exit 2, new message, path printed with the slash | exit 2, `principles.toml: Not a directory (os error 20)` | rejected |
| 11 | FIFO (`mkfifo`) | exit 2, new message | exit 2, `principles.toml: Not a directory (os error 20)` | rejected |
| 12 | character device (`/dev/null`) | exit 2, new message | exit 2, `principles.toml: Not a directory (os error 20)` | rejected |
| 13 | EMPTY directory | exit 2, bare `error: No such file or directory (os error 2)` | identical | ADMITTED, unlabelled failure downstream |
| 14 | valid pack, absolute path | exit 0, 31 files | exit 0, identical tree | ADMITTED, works |
| 15 | pack with a `pack.toml` and no `principles.toml` | exit 0, 1 file | exit 0, identical tree | ADMITTED, works |
| 16 | path with a `..` component resolving to the pack | exit 0, 31 files | exit 0, identical tree | ADMITTED, works |
| 17 | path with a `.` component | exit 0, 31 files | exit 0, identical tree | ADMITTED, works |
| 18 | symbolic link to a symbolic link to a directory | exit 0, 31 files | exit 0, identical tree | ADMITTED, works |
| 19 | relative path (`--template goodpack`) | exit 0, 31 files | exit 0, identical tree | ADMITTED, works |
| 20 | relative path with `./` and a trailing slash | exit 0, 31 files | exit 0, identical tree | ADMITTED, works |
| 21 | no `--template` at all (embedded pack) | exit 0, 31 files | exit 0, identical tree | check not reached |
| 22 | UNSEARCHABLE root, mode `000`, holding a `principles.toml` | exit 2, `error: could not read the pack's principles.toml: Permission denied (os error 13)` | identical | ADMITTED, see `F1` |
| 23 | UNSEARCHABLE root, mode `000`, EMPTY | exit 2, `error: could not read the pack's principles.toml: Permission denied (os error 13)` | identical | ADMITTED, see `F1` |
| 24 | valid pack under an UNSEARCHABLE PARENT | exit 2, ``error: --template `<path>` must name a directory`` | exit 2, `error: could not read the pack's principles.toml: Permission denied (os error 13)` | rejected, see `F1` |
| 25 | directory holding only a `principles.toml` | exit 2, bare `error: No such file or directory (os error 2)` | identical | ADMITTED, unlabelled failure downstream |
| 26 | directory of unrelated files | exit 2, bare `error: No such file or directory (os error 2)` | identical | ADMITTED, unlabelled failure downstream |
| 27 | this repository's own root | exit 2, bare `error: No such file or directory (os error 2)` | identical | ADMITTED, unlabelled failure downstream |
| 28 | empty directory, `--list-principles` | exit 0, empty output | identical | ADMITTED, exits 0 |
| 29 | `principles.toml`-only directory, `--list-principles` | exit 0, prints the principle | identical | ADMITTED, exits 0 |
| 30 | plain file, `--dry-run` | exit 2, new message | exit 2, `principles.toml: Not a directory` | rejected |
| 31 | plain file, `--list-principles` | exit 2, new message | exit 2, `principles.toml: Not a directory` | rejected |
| 32 | plain file, `--vcs git` | exit 2, 0 files, no repository created | exit 2, `principles.toml: Not a directory` | rejected |
| 33 | plain file, `--force` | exit 2, new message | exit 2, `principles.toml: Not a directory` | rejected |
| 34 | plain file plus a malformed `--var` | exit 2, the TEMPLATE message | exit 2, the `--var` message | rejected, order changed |
| 35 | plain file plus an incoherent `--with-precommit-hook` | exit 2, the HOOK message | identical | the hook check still runs first |

WHAT THE PREDICATE ADMITS: any path whose `stat`, after following links, reports a directory. That is rows 2, 9, 13 to 23, and 25 to 29.

WHAT IT REJECTS: any path whose `stat` reports a non-directory (rows 1, 3, 10, 11, 12) and any path whose `stat` fails for any reason at all, which covers `ELOOP` (rows 4, 5), `ENOENT` (rows 6, 7) and `EACCES` (row 24).

The code comment at `src/main.rs:2113-2116` claims the predicate covers "a plain file, a link loop and a path that is not there". All three claims are TRUE and I measured each one. The comment claims nothing about the permission shapes, so the comment is not falsified by `F1`.

## Verdicts

ONE finding. No `critical`, no `high`, no `medium`.

| id | severity | class | site | one line |
| --- | --- | --- | --- | --- |
| `F1` | low | USER-FACING message plus one INTERNAL doc | `src/main.rs:2117-2119` and `src/main.rs:229-231` | Neither side of the predicate handles a permission failure: a pack under an unsearchable parent is told it "must name a directory" when it is one, and an unsearchable root still reports its failure against `principles.toml`, so `PrinciplesError::Read`'s "the file is present" is NOT true as written and `X7` is not discharged. |

THE CHANGE IS OTHERWISE SOUND. It does what its comment says for every shape the comment names, it cannot fail any invocation that succeeded at `HEAD~1`, the added test is non-vacuous and is the only pin on the new message, and both gates are green.

## `F1` (low): the permission class, and why `X7` is not discharged

Two shapes, one root cause, one remedy. `is_dir()` collapses "this is not a directory" and "I cannot tell whether this is a directory" into the same `false`, and it answers `true` for a directory that the process cannot enter. The commit made a general claim about the consequence, and the general claim fails on both sides.

### Half A, the message is false for a pack under an unsearchable parent. INTRODUCED by this commit.

Three runs of the SAME path with the SAME HEAD binary, changing only the parent directory's mode:

```
control A, parent mode 755:
  test -d <path>            -> yes
  test -f <path>/pack.toml  -> yes
  HEAD  exit=0  31 files written, empty stderr

control B, parent mode 000, same path, same binary:
  HEAD  exit=2  error: --template `<path>` must name a directory
  PRE   exit=2  error: could not read the pack's principles.toml: Permission denied (os error 13)

control C, parent mode 755 again, same path, same binary:
  HEAD  exit=0  31 files written, empty stderr
```

Control A and control C establish that the path names a directory, and that HEAD scaffolds from it. In control B the same binary tells the user that the same path "must name a directory". The statement is false of that path. The cause is a permission failure on the parent, and the message names the wrong problem, so a user checks the wrong thing.

`PRE` names the wrong FILE and the right CAUSE. `HEAD` names the right FLAG and the wrong CAUSE. Neither is correct, and the trade is a wash on this shape, which is why I rate the half `low` and not higher.

### Half B, a root failure is still reported against `principles.toml`. PRE-EXISTING, and the reason `X7` is not discharged.

An empty directory at mode `000` passes `is_dir()`, because `stat` on the directory itself needs search permission on the PARENT and not on the directory:

```
fixture: an EMPTY directory, verified empty before the chmod and after the restore
  ls -A <root>  ->  (nothing)
  chmod 000 <root>

HEAD  exit=2  error: could not read the pack's principles.toml: Permission denied (os error 13)
PRE   exit=2  error: could not read the pack's principles.toml: Permission denied (os error 13)
```

There is no `principles.toml`. The directory is empty. The message names a file that does not exist, which is exactly the `C1` defect the commit exists to close, on a shape the predicate cannot see.

WHICH CANONICALISATION FAILS, measured with `realpath` on a mode `000` directory:

```
realpath <root>                    ->  <root>            (succeeds)
realpath <root>/principles.toml    ->  Permission denied (fails)
```

So the root DOES canonicalise and the child read fails with a non-`NotFound` kind, and no `principles.toml` exists. That falsifies the premise the round 3 triage used to discharge `X7` (`v002-r3-triage.md:412`): "For any root that canonicalises, every non-`NotFound` failure of `root.join("principles.toml")` involves a `principles.toml` that exists".

CONSEQUENCE, and this is the part the triager must not lose. The commit message states:

> With that, PrinciplesError::Read's "the file is present" becomes true as written and needs no edit.

The doc at `src/main.rs:229-231` still reads "The file is present and could not be read". On the fixture above the file is not present. The sentence is still false, `X7` is NOT discharged, and the round record must not close it as fixed. Round 3's own remedy already names the fallback (`v002-r3-triage.md:412`): if `C1` does not cover the case, then `X7` "must instead be weakened to what `Read` actually means, that the read did not produce text".

### Why I did not raise the severity

Every shape exits 2 and writes nothing, at both binaries, so no outcome and no verdict changes. That is the same reasoning round 3 applied to `C1` and to `X7`, and both were rated `low`. Raising this to `medium` would need the message or the doc sentence to be able to produce a wrong result, and neither can. I state the priority separately rather than inflating the rating, because inflating it corrupts the round record.

### The remedy, and what must NOT be done

DO NOT strengthen the predicate to `fs::read_dir(path)`. It looks like the natural fix for half B and it introduces a false rejection: `read_dir` needs READ permission on the directory, whereas the pack reads need only SEARCH permission, so a pack at mode `0111` scaffolds today and would start failing. I did not build that mutation, so treat it as an argument and not as a measurement, but the permission bits are documented and the asymmetry is not in doubt.

There is no cheap root-level predicate that detects an unenterable root without also rejecting legitimate roots. The cheapest correct closure is therefore the one round 3 already specified as the fallback: weaken `src/main.rs:229-231` to what `Read` means, which is that the read did not produce text. That is a deletion of three words, and it closes half B as a defect of accuracy without touching behaviour.

For half A, the honest options are to leave it (the message is right about the flag and wrong about the cause, and the previous message was wrong about the file) or to distinguish the `EACCES` case with a second arm. The second costs a `symlink_metadata` or a `metadata` call and an error-kind match at the one site. I recommend leaving the behaviour and NOT adding a second arm, on Principle 2 (minimal by default): the shape is rare, no outcome changes, and every added arm is a new unpinned surface in an increment whose remaining risk is entirely re-seeding.

## Ruling: the empty-directory gap

THE IMPLEMENTER'S REASONING HOLDS, AND LEAVING IT IS ACCEPTABLE. Four grounds, three of them measured.

1. THE SCOPING ARGUMENT IS CORRECT ON ITS OWN TERMS. An empty directory IS a directory, so it satisfies the predicate the site is entitled to apply. A `pack.toml` requirement is a statement about the directory's CONTENTS, and the site cannot make it without becoming a contents check, which is the thing the round 3 remedy told the fix pass to keep out of `read_optional` and its callers.

2. EXTENDING THE CHECK WOULD CHANGE BEHAVIOUR, NOT ONLY WORDING. Measured, at both binaries:

```
--template <a directory holding only principles.toml> --list-principles
  HEAD  exit=0   1. Ask clarifying questions before forging ahead - ...
  PRE   exit=0   identical
--template <an empty directory> --list-principles
  HEAD  exit=0   (empty output)
  PRE   exit=0   identical
```

`--list-principles` never reads `pack.toml`. A `pack.toml` requirement at the root would turn both of those exit 0 runs into exit 2. That is a behaviour change on a path that works today, in a `low_risk` delivery increment, and Principle 2 (minimal by default) refuses it.

3. THE EMPTY CASE IS NOT THE `C1` DEFECT. `C1` is MISATTRIBUTION, a message naming a file that cannot exist. The empty-directory message attributes nothing: `error: No such file or directory (os error 2)` names no flag, no path and no file. It is identical at HEAD and at `HEAD~1`, and round 3 measured it identical at `0.0.1` as well. It is a pre-existing unlabelled-message defect, unchanged by this commit, and not the defect the remedy was scoped to.

4. THE IMPLEMENTER DISCLOSED IT INSTEAD OF HIDING IT, which is the method round 3's own remedy asks for.

ONE CORRECTION FOR THE RECORD, because it bears on whether the remedy is discharged. The round 3 triage remedy (`v002-r3-triage.md:410`) says the change "also gives the currently unlabelled empty-directory and missing-directory cases a message that names the flag". The MISSING-directory half is delivered: row 7 shows the new message. The EMPTY-directory half is NOT delivered and could not have been delivered by the predicate the same paragraph prescribes, because an empty directory passes `is_dir()`. The triage remedy therefore carried an unmeasured claim of exactly the kind round 3 existed to remove. The implementer was right not to follow it and right to say so.

BREADTH OF THE UNLABELLED CASE, so the orchestrator can price a later fix. Every "a directory that is not a pack" shape gives the same bare message at both binaries: an empty directory, a directory holding only `principles.toml`, a directory of unrelated files, and this repository's own root (a plausible user error, since the pack lives at `pack/`). Rows 13, 25, 26 and 27. If that message is ever worth labelling, the site is the `manifest()` read and not the root check, and it is a separate change.

## Checks that PASSED

Recorded so the triager knows what is covered.

### The predicate covers every shape its comment names

Rows 1, 4, 5, 6 and 7. A plain file, a self-loop, a 2-cycle, a dangling link and a missing path are all rejected with the new message. `is_dir` follows links, confirmed in both directions by row 2 (link to a directory is admitted and works) and row 3 (link to a file is rejected).

### The check cannot fire for the embedded pack

STRUCTURAL: the check sits inside the `Some(path)` arm of `match &args.template`, and the `None` arm reaches `manifest::builtin()` without passing it. MEASURED: row 21, a run with no `--template` writes 31 files at exit 0 at both binaries, byte-identical.

### No legitimate invocation that worked at `HEAD~1` fails now

PROVED, then measured. Success at `HEAD~1` requires reading `<root>/pack.toml`, which requires `<root>` to be a directory after links are followed. So the set of paths that succeeded at `HEAD~1` is a subset of the set the predicate admits, and the check can only remove paths that were already failing. The check is purely subtractive on the accepted set.

MEASURED: rows 2, 9, 14 to 21 are the ten legitimate shapes. All exit 0 at both binaries and every output tree is byte-identical under `diff -r`. Stdout is identical too, apart from the output directory path each run names.

```
02-symlink-to-dir: IDENTICAL (31 files)      16-dotdot-to-good: IDENTICAL (31 files)
09-trailing-slash-dir: IDENTICAL (31 files)  17-dot-slash-good: IDENTICAL (31 files)
14-good-pack-abs: IDENTICAL (31 files)       18-link-to-link-dir: IDENTICAL (31 files)
15-no-principles: IDENTICAL (1 files)        19-no-template: IDENTICAL (31 files)
rel: IDENTICAL (31 files)                    rel2: IDENTICAL (31 files)
```

### The check runs before anything is written, on every flag path

Rows 30 to 33. `--dry-run`, `--list-principles`, `--vcs git` and `--force` all exit 2 with the new message and write nothing. The `--vcs git` run creates no repository. The `--with-precommit-hook` coherence check still runs FIRST, at both binaries (row 35), so the pre-existing ordering of that usage error is unchanged.

### The added test is non-vacuous, and it is the only pin

Five mutations, each built and run in its own tree. The baseline is 13 passed in that test file and 468 passed across the suite.

| mutation | test result | note |
| --- | --- | --- |
| M1, the whole check deleted | KILLED at `:458` | "the message must name the flag: error: could not read the pack's principles.toml: Not a directory (os error 20)" |
| M2, message drops `--template` | KILLED at `:458` | "the message must name the flag" |
| M3, message drops the path | KILLED at `:459` | "the message must name the path" |
| M4, `exit(2)` becomes `exit(0)` | KILLED at `:457` | `assertion left != right failed ... left: Some(0)` |
| M5, `!path.is_dir()` becomes `path.is_file()` | SURVIVED | the whole suite still passes, 468 of 468 |

M1 is the mutation that matters and the test kills it, which also confirms the test would have failed at `HEAD~1`. I ran M1 over the WHOLE suite with `--no-fail-fast`, because "only pin" is a claim about every test target and not about one file:

```
M1, cargo test --no-fail-fast:  467 passed, 1 failed
  the single failure is a_template_that_is_not_a_directory_is_reported_against_the_flag
  every other target is green (409, 5, 1, 1, 9, 2, 3, 20, 1, 4)
```

So the added test is the ONLY pin on the new message and on the `C1` behaviour, across the whole suite.

M5 IS A MEASURED SURVIVOR AND I RULE IT NOT A FINDING, so the triager does not have to guess why. Under M5 the plain-file shape still refuses, and the link-loop and missing-path shapes regress to the old messages, which I confirmed by running the mutated binary:

```
M5 binary, --template <self-loop>   ->  error: could not read the pack's principles.toml: Too many levels of symbolic links (os error 40)
M5 binary, --template <missing>     ->  error: No such file or directory (os error 2)
```

So the test pins one shape of a three-shape claim. I do not raise it, for three reasons. The behaviour is correct and I measured all three shapes at HEAD, so nothing false is being asserted. The round 3 remedy asked for exactly one integration case and named the plain-file shape, and the implementer delivered that. And one-shape-per-name is the convention throughout this suite, so raising it would apply a standard to this test that no neighbouring test meets. If the orchestrator wants the class pinned rather than the instance, two more cases in the same test are an ADDITION and cost nothing, but they are not owed.

### The test pins what its name claims, at the level the name claims it

`a_template_that_is_not_a_directory_is_reported_against_the_flag` asserts a non-zero exit, that stderr names `--template`, that stderr names the path, that stderr does NOT name `principles.toml`, and that the output directory stays empty. The negative assertion is the `C1` regression pin and it is the one that fails at `HEAD~1`. The comment above it claims only that nothing pinned this message before, which is true: the message did not exist before this commit.

### The check is in the right place

There is exactly ONE production construction site for `PackSource::Directory`, and it is the one behind the check.

```
src/main.rs:2121                     the guarded site
src/main.rs:2409, :2419              inside #[cfg(test)] (module starts at :2334)
src/manifest.rs, 40 sites            inside #[cfg(test)] mod tests (module starts at :811)
src/manifest.rs:493                  a match PATTERN, not a construction
```

`Cargo.toml` has no `[lib]` section and there is no `src/lib.rs`, so the crate is binary-only and no external caller can construct the variant at all. A future in-crate caller CAN construct it and bypass the check, because the variant stays publicly constructible, but that is unchanged from `HEAD~1` and the unit tests depend on it: they construct `PackSource::Directory` over fixtures on purpose, to exercise the read boundary without the CLI. Moving the check into a constructor would also move the `exit(2)` and the flag-named message out of the one layer that knows the flag exists. The placement follows the round 3 remedy and I found no reason to disagree with it.

### The check is not a containment boundary, and does not weaken the one that is

`is_dir()` follows links and answers before the reads happen, so a swap between the check and the read is possible in principle. It does not matter: containment is enforced at `PackSource::read` for every path, which this commit does not touch, and the check can only reject paths, never admit a path that `HEAD~1` refused. No shape in the enumeration writes a file that `HEAD~1` did not write.

## Out-of-scope observations

Real, measured, and not raised as findings.

1. WHICH USAGE ERROR IS REPORTED FIRST CHANGED, for an invocation that carries two of them. With a non-directory `--template` AND a malformed `--var`, `PRE` reports the `--var` error and `HEAD` reports the `--template` error (row 34). Both exit 2 and write nothing, and with a valid `--template` the `--var` message is unchanged at both binaries. This follows unavoidably from putting the check where the round 3 remedy said to put it, which is before the `--var` loop. No test pins either order. I record it only because it is a behaviour difference in the commit that is not mentioned in the commit message.

2. THE MISSING-PATH MESSAGE LOSES THE ERRNO. At `HEAD~1` a typo in `--template` produced "No such file or directory (os error 2)", and at HEAD it produces "must name a directory" (rows 6, 7). The user gains the flag and the path and loses the fact that the path is absent. Nothing false is stated, and the previous message attached its true cause to the wrong file, so I do not call this a regression. It is the one place where a second error-kind arm would pay for itself, and it is the same arm half A of `F1` would need.

3. `--list-principles` SUCCEEDS ON A DIRECTORY THAT IS NOT A PACK (rows 28, 29), at both binaries. This is consistent with the rule that a pack shipping no `principles.toml` has no principles, and it is unchanged by this commit. It is also the measured reason not to put a `pack.toml` requirement in the root check.
