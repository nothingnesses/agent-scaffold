# `ship-v0-0-2-inc1` round 4: REVIEWER (the text half of the deletion pass)

Independent reviewer. I did not write this change and did not review rounds 1, 2 or 3. Every figure below is my own measurement, made in the detached worktree `.claude/worktrees/r4-text` at `406c278`. No tracked file was modified in this worktree or in the main repository except this findings file.

## Artifact and commit

- The commit under review is `406c278` ("docs: delete the nine unmeasured claims round 3 found, and label root failures"), 6 files, 53 insertions, 24 deletions. Its parent is `dbbf937`, the round 2 fix pass round 3 reviewed.
- Scope: the TEXT half of that commit. `CHANGELOG.md` (the three `Fixed` bullets it edited), `README.md:325`, the doc and test comments in `src/manifest.rs`, `src/safe_path.rs` and `tests/pack_source_stays_inside_the_pack.rs`, the comment and the new user-facing message in `src/main.rs`, and the 0.0.2 section as a whole. Another reviewer holds the code half.
- Read in full before starting: `v002-r3-triage.md`, `v002-r3-reviewer-text.md`, `v002-r3-reviewer-boundary.md`.

## Method

TWO release binaries, one `CARGO_TARGET_DIR` each, from two separate `git archive | tar -x` extracts, verified distinct by `md5sum`:

```
c5da98aca079b20f62a68ef08ae5d8d5  tgt-head/release/agent-scaffold   (406c278, HEAD)   0.0.2
fb91924fbe4529460c54bbacf28c6aaa  tgt-main/release/agent-scaffold   (main, f91de6a)   0.0.1
```

`MAIN` is what the 0.0.2 section is measured against. `HEAD` is what it describes. I did not build the intermediate commits: every comparative claim in the edited text is stated against 0.0.1, so 0.0.1 is what falsifies it.

THREE further trees carried one source mutation each, in their own target directories, for the one API claim this pass added:

```
probe1   HEAD + pack_principles reverted to `.unwrap_or_default()`   (the caller that reads principles.toml)
probe2   HEAD + build_assets reverted to `.unwrap_or_default()`      (the caller that reads instrument.md)
probe3   HEAD + a NEW swallowing caller (`banner.md`), one expression
```

Every fixture, symbolic-link target, escape target and cargo target directory is under my own scratch subdirectory. I used invalid UTF-8 wherever it would do. Two measurements need a permission bit and nothing else can produce them (an unreadable file, and a directory the process cannot traverse); both used `chmod` on my own fixtures and both were restored, which the run output records.

GATES I RAN MYSELF AT HEAD:

| gate | result |
| --- | --- |
| `cargo test --no-fail-fast` | 468 passed, 0 failed, over 11 result lines (409+5+1+1+9+2+13+3+20+1+4). Round 3 measured 467; the added test is the difference |
| `cargo clippy --all-targets -- -D warnings` | clean, exit 0 |
| `validate --source ... --metrics ...` | `334 records, valid`, `99 steps, 75 questions, valid`, exit 0 |
| `validate --source ... --workflow` | `workflow invariants hold`, exit 0 |
| `render --check --strict` | `up to date`, exit 0 |
| `validate --plan ... --metrics ...` | EXACTLY ONE problem, the pre-existing `Q-43` `superseded by` one criterion 4 excludes, exit 1 |
| ASCII check on all six changed files | `0` non-ASCII lines on every file |

## The headline

Assertions checked: 50. Confirmed: 45. Falsified: 5, grouped into 4 findings, all `low` (two of the five share one root cause and one remedy).

The deletion worked on the six claims round 3 named: every one of them is gone from the tree and nothing false was left where it stood. It did NOT stop the pass writing new falsifiable text. Five new comparative claims about 0.0.1 were added in `CHANGELOG.md:37`, four hold in every half I could run and one is false of one of the three files it enumerates. One new non-comparative claim was added to `read_optional`'s doc, naming a command, and that command refutes it for both of the callers that exist.

The most important result is not a claim in the tree. Round 3's `X7` is recorded by this commit as closed without an edit, on the ground that the new `--template` root check makes `PrinciplesError::Read`'s "the file is present" true as written. It does not: `is_dir` answers TRUE for a directory the process cannot traverse, so the root failure still reaches `principles.toml` and still names a file that is not there.

## Assertion table

Every assertion I checked, the case I constructed to falsify it, and the measured result. `CL` is `CHANGELOG.md`, `RM` is `README.md`, `MF` is `src/manifest.rs`, `SP` is `src/safe_path.rs`, `MN` is `src/main.rs`, `PT` is `tests/pack_source_stays_inside_the_pack.rs`. Line numbers are at HEAD.

### `CL:36`, the one sentence this commit ADDED to the first `Fixed` bullet

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| A1 | "Each refusal message names the value" | Fired all EIGHT pack containment refusals: `source` absolute / `..` / link out, `guidance` absolute / `..` / link out, `dest` absolute / `..`, plus the three literals linked out | Every message quotes the offending value in backticks. CONFIRMED |
| A2 | "then the specific cause in parentheses (an absolute path, a `..` component, or a resolution outside the directory through a link)" | The same eleven runs, checking the cause phrase against the input each time | `(it is an absolute path)`, `(it carries a `..` component)`, `(it resolves outside the pack directory, through a symbolic link)`. Each matches its input; no fourth cause is reachable on Unix. CONFIRMED |
| A3 | "then the rule" | The same runs | Every message ends with the rule: "a source/guidance path/pack path must be relative, carry no `..` component, and resolve to a location inside the pack directory", and the `dest` pair with the string half. CONFIRMED |
| A4 | The sentence is exhaustive over the refusals its bullet covers | Enumerated every containment refusal the pack path can produce and ran each | Six read-side (three fields x three causes collapses to `source` x3, `guidance` x3, three literals x link) and two write-side. All eight have the stated shape. CONFIRMED. The plan-side `[meta].sidecars` refusal has a different shape; it is outside this bullet's subject, see observation 1 |

### `CL:37`, the bullet this commit REPLACED

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| B1 | "The three files the tool reads by literal name, `pack.toml`, `principles.toml` and `instrument.md`" | `grep -rn 'read("\|read_optional("' src/` for a fourth literal | Exactly three in production code: `MF:548`, `MN:264`, `MN:298`. CONFIRMED, and the enumeration is exhaustive |
| B2 | "are contained too" | Each of the three made a symbolic link to a file outside the pack, one fixture per literal | All three refused at exit 2, the file named, output directory empty. CONFIRMED |
| B3 | "In 0.0.1 each was read through a symbolic link out of the pack" | The same three fixtures against `MAIN` | All three read the outside file: the leaked principle renders, the leaked fragment inlines, the outside manifest is obeyed. Exit 0 in every case. CONFIRMED |
| B4 | "and its contents inlined" | The same three fixtures at `MAIN`, then grep of the whole output tree for the outside file's text | TRUE for `principles.toml` (`P:1. LEAKED PRINCIPLE - from outside the pack`) and `instrument.md` (`I:LEAKED-INSTRUMENT-FRAGMENT`). FALSE for `pack.toml`: no output file contains one byte of it. FALSIFIED. FINDING 2 |
| B5 | "the same leak as the two fields above on three more paths" | Compared the mechanism per literal at `MAIN` | Same leak at the class level (an outside file is read). For `pack.toml` the outside file DIRECTS the run instead of being inlined, which B4 covers. CONFIRMED at the class level |
| B6 | "They are now refused by the same rule." | Compared each literal's refusal against a refused `[[asset]].source` | Same predicate, same cause phrase, same rule sentence, same exit code, nothing written. CONFIRMED |
| B7 | "A file the tool cannot read (invalid UTF-8 ...) produced an empty principle set or an empty instrumentation block at exit 0 with empty stderr in 0.0.1" | `principles.toml` and `instrument.md` each filled with an invalid UTF-8 byte sequence, as real files inside the pack, against `MAIN` | Both exit 0, stderr EXACTLY 0 bytes, `AGENTS.md` is `P:\nI:\n`. CONFIRMED |
| B8 | "(or one it lacks permission to read)" | `principles.toml` at mode 000 against `MAIN` (restored to 644 afterwards) | Exit 0, stderr 0 bytes, empty principles block. CONFIRMED |
| B9 | "indistinguishable from a pack shipping neither" | Compared the unreadable run against the absence run at `MAIN`, byte for byte | `AGENTS.md` md5 `5a28f5e12a01946aaad53f844b4db5fe` in both, stdout identical after path normalisation, stderr empty in both, exit 0 in both. CONFIRMED |
| B10 | "it now exits 2 naming the file" | All four unreadable cases (two files x invalid UTF-8 / permission) at HEAD | Exit 2 every time, the file named every time, nothing written every time. CONFIRMED |
| B11 | "which matches a malformed `principles.toml`, already loud in 0.0.1" | A schema-invalid `principles.toml` against `MAIN` and HEAD | Both exit 2 with the same 150-byte `could not parse the pack's principles.toml: TOML parse error ...`. CONFIRMED |
| B12 | "ABSENCE IS UNCHANGED ... byte for byte what 0.0.1 produced" | A pack shipping neither literal, with and without `--instrument`, at `MAIN` and HEAD | `AGENTS.md` byte-identical (same md5), stdout identical after path normalisation, exit 0, stderr empty, in all four runs. CONFIRMED |

### `CL:38`, the sentence this commit MOVED into the `dest` bullet

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| C1 | "The `dest` message states only the string half of the rule" | Both `dest` refusals at HEAD | "a dest must be relative and carry no `..` component". The resolved half is absent, correctly. CONFIRMED |
| C2 | "because the write side applies only that half" | A `dest` of `linkdir/escaped.md`, where `linkdir` inside `--output-dir` is a symbolic link out of it | The file was written OUTSIDE the output directory at exit 0, reporting `create linkdir/escaped.md` and "Wrote to `<output-dir>`". The write side really is lexical only. CONFIRMED |
| C3 | The sentence survived the deletion of its old bullet unchanged | `git diff --word-diff` of the commit | Moved verbatim, no word altered. CONFIRMED |

### The 0.0.2 section as a whole

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| S1 | No `Fixed` bullet still describes a regression that existed only between unreleased commits | Ran each `Fixed` bullet's described prior behaviour against `MAIN` | Bullet 1 (read containment) TRUE at `MAIN`; bullet 2 (the three literals) TRUE at `MAIN` (B3); bullet 3 (`dest`) TRUE at `MAIN`, measured myself: `dest = "../escaped.md"` writes the file outside at exit 0 while reporting "Wrote to `<output-dir>`". Bullets 4 and 5 are untouched by this commit and were confirmed genuine in round 3. The two bullets round 3 falsified are gone. CONFIRMED |
| S2 | The disclosure round 3's triager said must not be lost is present | Read the replacement bullet for the statement that 0.0.1 leaked through the three literals | Present, as `CL:37`'s second sentence. It is the sentence B4 falsifies in part, so the disclosure exists and mis-describes one of its three subjects. CONFIRMED that it was not lost |
| S3 | Nothing else true was lost with the two deleted bullets | Listed every factual claim in the old `CL:37` and old `CL:38` and looked for it at HEAD | The `dest`-message sentence moved verbatim; the message-form sentence became `CL:36`'s new last sentence; absence-stays-silent and unreadable-becomes-loud both survive in `CL:37`. Everything else deleted was either false (round 3's `X5`, `Z1`) or an internal API detail (`read_optional` separating the two facts by type). CONFIRMED |
| S4 | The section describes everything user-visible in `git diff main..HEAD` | Enumerated what this commit changes for a user and looked for a bullet | The `--template` root message is new user-visible behaviour and no bullet mentions it. Three invocations change what they print relative to 0.0.1. FALSIFIED. FINDING 4 |
| S5 | The section describes nothing that is NOT user-visible | Read the three edited bullets against the code | Every claim in them is about behaviour a user can reach. CONFIRMED |

### `RM:325`, the README edit

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| D1 | "This is the same trade the metrics and ledger boundary above takes" | Compared against `RM:242` | `RM:242`: "the trade taken is that a loud refusal beats silently reading the wrong file", for the metrics and ledger boundary. Same trade in the repository's own words. CONFIRMED |
| D2 | The false half round 3 found ("the same rule") is gone | `grep -n "the same rule" README.md` | Zero hits in that sentence; the surviving clause claims only the trade. CONFIRMED |
| D3 | The rest of the edited sentence is still true | Ran all six read-side refusals and counted the output directory afterwards | Exit 2, ZERO entries written, the file named, in every case. "The refusal is loud, names the file, and writes nothing" holds. CONFIRMED |
| D4 | The deletion did not damage the sentence before it, which states the rule in full | Read the paragraph | Unchanged and still correct; it is the sentence that carries the rule a pack author acts on. CONFIRMED |

### `MF:513-528`, `read_optional`'s doc

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| E1 | "a caller may still write `.unwrap_or_default()` on this too" | Wrote it, three ways | All three trees compile. CONFIRMED |
| E2 | "and a caller that does passes `cargo clippy --all-targets -- -D warnings`" | probe1 (`pack_principles`), probe2 (`build_assets`), probe3 (a NEW caller), each `cargo clippy --all-targets -- -D warnings` | probe1 EXIT 101, `error: variant `Read` is never constructed`. probe2 EXIT 101, `error: variant `UnreadablePackFile` is never constructed`. probe3 exit 0. False for both callers that exist. FALSIFIED. FINDING 3 |
| E3 | "The invariant is held by review, not by the compiler." | probe3, the case the doc exists to guard | A new swallowing caller passes clippy at exit 0. CONFIRMED |
| E4 | The clause round 3 falsified ("must write an explicit arm") is gone | `grep -n "explicit arm" src/` | Zero hits in the tree. CONFIRMED |

### `MF:1319-1320`, the test comment this commit shortened

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| F1 | "Absence stays silent, held at the primitive level." | Read the test it introduces | A unit test over `read_optional` itself, which is the primitive level. CONFIRMED |
| F2 | "A pack that ships neither optional file is legitimate (`README.md`)" | Checked whether the README covers BOTH files, which is where round 3's `X3` remedy pointed | `RM:362` ("A pack that ships no `principles.toml` simply has no principles to select") and `RM:327` ("the pack's optional `instrument.md` render fragment ... empty otherwise"). Both covered. CONFIRMED |
| F3 | The claim round 3 falsified ("nothing held at any level before") is gone | `grep -n "at any level" src/` | Zero hits. CONFIRMED |

### `SP:1-31`, the module doc

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| G1 | "The strings are external (a `.plan.toml` or a `--template` pack ...)" | Enumerated every caller of the two predicates and traced where its string comes from | Four callers: the pack read site, the `dest` check, the sidecar refs, the findings ref. Every string comes from a `.plan.toml` or a `pack.toml`. CONFIRMED |
| G2 | "the rules are authored here once rather than copied per caller" | `grep -rn "is_absolute()\|ParentDir" src/` outside `safe_path.rs`, looking for a re-implementation | Three hits, all unrelated (absolute-path joins in `main.rs`). `is_safe_sidecar_ref` is a pure delegate (`src/plan/source.rs:480-482`). Nothing copies the rule. CONFIRMED |
| G3 | The false joiner enumeration round 3 found is gone | `grep -n "plan::source joins\|joins an" src/safe_path.rs` | Deleted; the paragraph names no joiner at all. CONFIRMED |
| G4 | The paragraph beneath it no longer rests on a false premise | Read `SP:9-16` after the deletion | "ONE CALLER USES THE LEXICAL RULE WITHOUT JOINING ANYTHING" still holds (four callers, one never joined) and now depends on nothing deleted. CONFIRMED. See observation 2 for the header sentence |

### `PT:383-385` and the new test at `PT:445`

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| H1 | The kept sentence: "The `pack.toml` literal goes through `io::Error::from` and carries no field label, so nothing pinned its wording." | Read `MF:548`; read the measured message | `.map_err(io::Error::from)?`, and the message carries no field label, unlike `source` and `guidance`. CONFIRMED |
| H2 | The false ordering sentence is gone | `grep -n "first read a run makes" tests/` | Zero hits. CONFIRMED |
| H3 | "A failure of the `--template` root must name the flag and the path, not the first file inside the pack a run happens to read" | Ran the case at HEAD | `error: --template `<path>` must name a directory`. Names the flag and the path, does not name `principles.toml`. CONFIRMED |
| H4 | "Nothing pinned this message before, so nothing is replaced." | `git grep "must name a directory" HEAD~1`, and read every `--template` argument in the pre-commit test suite | Zero hits at `HEAD~1`, and all six `--template` call sites there pass a pack DIRECTORY, so no test exercised a non-directory root. CONFIRMED |

### `MN:2111-2121`, the comment and the message this commit added

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| J1 | "This is the only site that knows the difference." | `grep -rn "PackSource::Directory" src/` for another production construction site | One production site (`MN:2121`); every other hit is inside `#[cfg(test)]`. CONFIRMED |
| J2 | "`is_dir` follows links" | `--template` naming a symbolic link TO the pack directory, at HEAD | Exit 0, one file written, same as `MAIN`. The surviving shape survives. CONFIRMED |
| J3 | "answers false on any error, so one predicate covers a plain file, a link loop and a path that is not there" | All three shapes at HEAD, plus a dangling link | All four print the new message at exit 2, nothing written. CONFIRMED. (`MAIN` prints `Not a directory`, `Too many levels of symbolic links` and `No such file or directory` for the first three) |
| J4 | The new message is true of the input it refuses | A `--template` naming a REAL directory whose parent the process cannot traverse (mode 000, restored) | `error: --template `<path>` must name a directory`, for a path that does name a directory. `MAIN` prints `Permission denied (os error 13)`, the true cause. FALSIFIED. FINDING 1 |
| J5 | An empty `--template` directory is unaffected | An empty existing directory at `MAIN` and HEAD | Both print `No such file or directory (os error 2)` from the `pack.toml` read. Unchanged, and the comment does not claim otherwise. CONFIRMED |

### Round 3's `X7`, which this commit reports as closed without an edit

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| K1 | The root check makes `MN:229-230` ("The file is present and could not be read") true as written | A pack DIRECTORY at mode 000 containing NO `principles.toml` (restored to 755), which is round 3's own `X7` reproduction | `is_dir` is TRUE for it, so the root check passes, and HEAD prints `error: could not read the pack's principles.toml: Permission denied (os error 13)` for a file that does not exist. FALSIFIED. FINDING 1 |
| K2 | The symbolic-link-loop reproduction of `X7` is closed | A `--template` naming a link to itself | Now caught by the root check and reported against `--template`. CONFIRMED, so the fix closes part of the class |

## Verdict table

Severity is absolute impact if left unfixed.

| id | severity | class | site | one line |
| --- | --- | --- | --- | --- |
| 1 | low | USER-FACING message and INTERNAL doc | `MN:2118`, `MN:229-230` | The root predicate cannot tell "not a directory" from "cannot tell": a real directory the process cannot traverse is refused as "must name a directory", and an unreadable pack directory still reports a `principles.toml` that is not there, so round 3's `X7` is not closed. |
| 2 | low | USER-FACING | `CL:37` | "In 0.0.1 each was read through a symbolic link out of the pack and its contents inlined" is false of `pack.toml`: at 0.0.1 the outside manifest is obeyed rather than inlined, which is a larger leak than the sentence describes. |
| 3 | low | INTERNAL | `MF:524-528` | `read_optional`'s doc says a caller that writes `.unwrap_or_default()` "passes `cargo clippy --all-targets -- -D warnings`". Both callers that exist fail it at exit 101. |
| 4 | low | USER-FACING | the 0.0.2 section | The new `--template` root message is user-visible behaviour this commit added, and no bullet describes it. |

Four findings, all `low`. No `medium`, no `high`, no `critical`.

WHAT THE DELETION ACHIEVED, stated plainly because it is most of the result. All six claims round 3 falsified are gone from the tree, and I checked each site for a replacement that inherits the falsity: none does. The two CHANGELOG bullets that described unreleased-only regressions are gone and no surviving bullet has that defect. The disclosure round 3's triager said the rewrite must not drop is present. The README's "same rule" half is gone and the "same trade" half is true. Every one of the four surviving comparative claims about 0.0.1 in the replacement bullet holds in every half I could run, including the two that a rewrite would most easily have got wrong ("indistinguishable from a pack shipping neither" and "byte for byte what 0.0.1 produced"), both of which are byte-exact.

WAS A NEW COMPARATIVE CLAIM INTRODUCED: yes, five of them, all in `CL:37`. Four are true. One (finding 2) is true of two of the three files it quantifies over and false of the third. Beside them the pass added one new non-comparative claim that names a command (finding 3), and that command refutes it. So the deletion method reduced the crop rather than eliminating it: the falsification rate on new text fell from six of six sentences of that shape to one of five, and the one that failed fails on a third of its subject rather than on its whole subject.

## Finding 1 (low): the root check cannot tell "not a directory" from "cannot tell", and `X7` is not closed

TWO ARTEFACTS, ONE PREDICATE, and I report them together because one change closes both and because round 3's triager asked for exactly this pairing to be ruled on rather than duplicated.

QUOTED, `src/main.rs:2113-2118`:

> `is_dir` follows links and answers false on any error, so one predicate covers a plain file, a link loop and a path that is not there.
> ```
> if !path.is_dir() {
> 	eprintln!("error: --template `{}` must name a directory", path.display());
> ```

QUOTED, `src/main.rs:229-230`, which this commit deliberately did NOT edit:

> The file is present and could not be read: a containment refusal, or an unreadable file.

The comment is true. The consequences the commit drew from it are not.

HALF ONE, the new message asserts something that can be false of the input. `is_dir` answering false "on any error" includes the error that means "I cannot tell". Measured on a pack directory whose PARENT the process cannot traverse:

```
--template <a real directory, reached through a mode-000 parent>
  MAIN  exit=2  error: Permission denied (os error 13)
  HEAD  exit=2  error: --template `<path>` must name a directory
(the path is a directory: `test -d` answers yes once the parent is restored)
```

The message tells the user their path is not a directory when it is. That is the class this commit exists to close, one level up: `src/safe_path.rs:66-68` states the standard in the tree's own words, that a refusal must state the rule and the cause "rather than asserting a property of the input", and `a/../b.md` is given there as the example of getting it wrong.

HALF TWO, and the more consequential one, `X7` is not closed. The commit message records it as needing no edit because the root check makes "the file is present" true as written. The triager's ruling carried the same premise, "for any root that canonicalises, every non-`NotFound` failure of `root.join("principles.toml")` does involve a `principles.toml` that exists". A directory at mode 000 canonicalises, and `is_dir` is TRUE for it, because `stat` needs execute permission on the PARENT and not on the directory itself. So the root check passes it through, and round 3's own `X7` reproduction still reproduces:

```
pack/ contains a.md and pack.toml, and NO principles.toml
chmod 000 pack/
  MAIN  exit=2  error: Permission denied (os error 13)
  HEAD  exit=2  error: could not read the pack's principles.toml: Permission denied (os error 13)
(mode restored to 755; the directory still holds a.md and pack.toml)
```

The file is not present. The unreadable thing is the root. The message names a file the user does not have. This is `X7` verbatim, and it is also `C1`'s misattribution surviving in a second shape: the same root failure, reported against a file inside the root, for the one root-failure kind the new check does not catch.

WHAT IS CLOSED, so the record is fair: the plain-file, link-loop, dangling-link and missing-path roots are all now reported against `--template`, and the link-loop shape is the one round 3's triager used to reproduce `X7` without a permission bit. That reproduction is dead. The permission one is not.

`low`: both the old and the new behaviour exit 2 and write nothing, so no outcome changes, which is where rounds 3 put `C1` and `X7`. The wrong action it invites is a maintainer reading `MN:229-230` while debugging this message and looking for a `principles.toml` that is not there, and a user being told a directory is not a directory.

## Finding 2 (low): the CHANGELOG says 0.0.1 inlined the contents of a linked `pack.toml`, and it did something else

QUOTED, `CHANGELOG.md:37`:

> The three files the tool reads by literal name, `pack.toml`, `principles.toml` and `instrument.md`, are contained too (`src/manifest.rs`, `src/main.rs`). In 0.0.1 each was read through a symbolic link out of the pack and its contents inlined, the same leak as the two fields above on three more paths.

"Each was read through a symbolic link out of the pack" is true of all three; I measured it. "And its contents inlined" is true of two and false of the third.

EVIDENCE. Three fixtures at `MAIN`, one per literal, each with the literal deployed as a symbolic link to a file outside the pack:

```
principles.toml -> outside   MAIN exit=0  AGENTS.md: P:1. LEAKED PRINCIPLE - from outside the pack
instrument.md   -> outside   MAIN exit=0  AGENTS.md: I:LEAKED-INSTRUMENT-FRAGMENT
pack.toml       -> outside   MAIN exit=0  no AGENTS.md at all; the outside manifest was OBEYED
                                          grep of the whole output tree for the manifest's text: no file
```

For the first two, "inlined" is the exact word, and it is the word `README.md:327` uses for them. For `pack.toml` nothing is inlined: the outside file becomes the manifest that decides which files are read and where they are written. Measured with an outside manifest that exercises both, at `MAIN`:

```
outside manifest:  source = "../outside/secret.md"   dest = "../elsewhere/planted.md"
MAIN  exit=0  stdout: create ../elsewhere/planted.md / Wrote to <output-dir> (1 changed, 0 left untouched)
      the file lands OUTSIDE the output directory, carrying TOP SECRET OUTSIDE THE PACK
```

So at 0.0.1 a linked `pack.toml` is a directive leak, not a content leak, and it composes with the two escapes the neighbouring bullets describe. The sentence understates it rather than overstating it, which is why this is `low` and not higher: no user is told their exposure was smaller than it was in a way that changes what they do, and the remedy for all three is the same release.

WHY IT IS STILL WORTH FIXING. This sentence exists because round 3's triager required the rewrite to ADD the disclosure that 0.0.1 leaked through the three literals, on the ground that `CL:36`'s enumeration is scoped to two fields and excludes them. The disclosure is the one thing in this bullet that could not be inherited from anywhere else, and one third of it is wrong about the mechanism. The fix is one clause: say the outside file was read and its contents used, or split `pack.toml` out and say the outside manifest was obeyed.

`low`: a false statement about the previous release's behaviour in text bound for crates.io, in the understating direction, with no user action turning on it. That is where round 3 put the same class.

## Finding 3 (low): `read_optional`'s doc names a command that refutes it

QUOTED, `src/manifest.rs:524-528`:

> It does not make swallowing impossible, and nothing in Rust can: a caller may still write `.unwrap_or_default()` on this too, and a caller that does passes `cargo clippy --all-targets -- -D warnings`. What it buys is that the correct optional-read primitive exists and is the obvious one to reach for. The invariant is held by review, not by the compiler.

The first and last sentences are true, and the last is the honest one round 3's triager asked for. The clause between them is a claim about a named gate, and the gate says otherwise for every caller that exists today.

EVIDENCE. Three trees, each one expression different from HEAD, each with its own target directory:

```
probe1  pack_principles:  .read_optional("principles.toml").unwrap_or_default()
        cargo clippy --all-targets -- -D warnings   EXIT 101
        error: variant `Read` is never constructed

probe2  build_assets:     .read_optional("instrument.md").unwrap_or_default().unwrap_or_default()
        cargo clippy --all-targets -- -D warnings   EXIT 101
        error: variant `UnreadablePackFile` is never constructed

probe3  a NEW caller:     source.read_optional("banner.md").unwrap_or_default().unwrap_or_default()
        cargo clippy --all-targets -- -D warnings   EXIT 0
```

There are exactly two callers of `read_optional` in production code. A swallow written at either of them fails the project's own gate, because each is the only construction site of its error variant and removing it makes the variant dead. The claim holds only for a caller that does not exist yet.

WHAT THE PASS WAS TOLD, since this is a re-seeding case and the record should carry it. Round 3's triager measured both probes and wrote the remedy as: do not replace the deleted clause with a claim that the gates catch a swallow, they do not for a NEW caller; the honest replacement says the invariant is held by review, not by the compiler. The pass wrote the honest sentence AND a clippy claim wider than the measurement behind it. The deleted overclaim said the compiler catches more than it does; the replacement says it catches less than it does.

`low`: an internal doc comment on a function whose code is correct, in a binary-only crate with no rustdoc, and no exit code or output byte turns on it. The wrong action it invites is the mirror of round 3's finding 4: a maintainer who reverts a call site to swallow, is stopped by clippy, and concludes the doc is wrong about the guard rather than that the guard is real at that site. Deleting the clause leaves a paragraph that is entirely true and needs nothing added.

## Finding 4 (low): the 0.0.2 section does not describe the one user-visible thing this commit adds

The commit adds a `--template` root check with a new message. Measured against 0.0.1, three invocations change what they print:

```
--template <a plain file>          MAIN: error: Not a directory (os error 20)
                                   HEAD: error: --template `<path>` must name a directory
--template <a symbolic-link loop>  MAIN: error: Too many levels of symbolic links (os error 40)
                                   HEAD: error: --template `<path>` must name a directory
--template <a path that is absent> MAIN: error: No such file or directory (os error 2)
                                   HEAD: error: --template `<path>` must name a directory
```

Exit code 2 and an empty output directory in every case, at both binaries. No bullet in the 0.0.2 section mentions `--template` naming a directory, or the message. I checked the section for a bullet that could carry it: the two mentions of `--template` in the section are `CL:36`'s recourse paragraph and its surviving-shape clause, neither of which is about a root that is not a directory.

The standard I am applying is the section's own, the one round 3 tested in both directions: the section describes everything user-visible in `git diff main..HEAD` and nothing that is not. This commit is the only change to that diff since round 3 measured it, so this is the only new gap.

THE ARGUMENT AGAINST, stated because it is real and a triager may well take it. This is a message-quality improvement on an invalid invocation. Nothing a working setup does changes, no exit code moves, and Keep a Changelog asks for notable changes rather than every string. A reader who never mistypes `--template` never meets it. That makes this the weakest of my four findings, and I would not argue against dismissing it if the triage judges message wording on an invalid flag to be below the section's threshold. What decides it either way is a rule about that threshold, and the section currently documents message form elsewhere (`CL:36`'s new last sentence, `CL:38`'s `dest` sentence), which is why I raise it rather than record it as an observation.

`low`: a release note that is silent about a message a user can meet, in a section that is otherwise complete.

## Out-of-scope observations

Not findings. Recorded so the triage can see they were considered and does not have to re-derive them.

1. `CL:36`'s new sentence begins "Each refusal message", with no scope word. Under the bullet's own subject, pack paths, it is exactly true and I verified all eight messages. Under the widest possible reading it would also cover the plan-side refusals, which have a different shape: `src/plan/source.rs:646` and `:743` produce "meta sidecar front ref `X` must be a task-relative path (no absolute path, no `..` component)", which names the value and the rule with no cause in parentheses. I am not raising it. The sentence sits in a bullet about pack paths, the bullet it replaces named four pack-side types explicitly, and `CL:38` describes the plan-side rule as one `validate --source` "already applied", so no reader is being pointed at it.

2. `SP:1-2`, which the commit kept, still says these are strings "the tool then joins onto a base directory". One class, a `[step.provenance].findings` ref, is never joined, which the same doc states seven lines later in capitals. Self-consistent as a subject sentence plus a stated exception, and round 3 did not raise it against the fuller version. Not raised.

3. The `MF:1319` test comment lost a true clause with the false one: the shipped pack does ship both optional literals (`pack/principles.toml` and `pack/instrument.md` are both present), so no scaffold-parity gate can catch an over-tightening of the absence path. That was the justification for the test existing at the primitive level. The test survives and explains itself; deleting a true justification is Principle 2 rather than a defect. Recorded because a later reader may wonder why the test is there.

4. `LoadError::UnreadablePackFile`'s `rel` field is still unpinned, which round 3 recorded as its observation 4 and nothing in this commit changes.

5. The write side still escapes through a link (round 1's `A2`, listed as settled in my scope). I measured it while confirming `C2`: a `dest` of `linkdir/escaped.md` writes outside `--output-dir` at exit 0 while reporting "Wrote to `<output-dir>`". Raised only to record that the sentence this commit moved into the `dest` bullet is the disclosure of that asymmetry, and it is accurate.

6. `validate --plan` still reports the pre-existing `Q-43` `superseded by` problem the spec excludes. Confirmed incidentally, unchanged.
