# `ship-v0-0-2-inc1` round 5: REVIEWER (truth of the surviving sentences)

Independent reviewer. I did not write this change and I did not review rounds 1 to 4. Every figure below is my own measurement, made in this session. Where I reach the same result as an earlier round I say so and give my own numbers.

## Artifact

- Worktree `.claude/worktrees/r5-a`, detached at `e3a466e` ("docs: delete the three claims round 4 measured false").
- Reviewed: `git diff HEAD~1..HEAD`. Three files, four lines added, five removed.
- Lens assigned: are the sentences that SURVIVE in the three edited passages true? A second reviewer covers whether the deletions removed something that needed to stay.
- Read first: `docs/plans/agent-scaffold.reviews/v002-r4-triage.md`, for what each deletion had to achieve.

## Method

THREE release binaries, one `CARGO_TARGET_DIR` each, from three separate `git archive | tar -x` extracts, verified distinct by `md5sum`:

```
c3dfebfcc96a6badd137f08a3266e306  tgt-head/release/agent-scaffold  (e3a466e, HEAD)    0.0.2
a9a1f89a8440d2f068b915f8c9aa5196  tgt-main/release/agent-scaffold  (main)             0.0.1
7fb089e85bf74394cd794082a3adfd9d  tgt-001/release/agent-scaffold   (tag v0.0.1)       0.0.1
```

`--version` confirms 0.0.2, 0.0.1 and 0.0.1.

I built TWO 0.0.1 binaries on purpose. Rounds 1 to 4 built one, from branch `main`, and `v002-r3-triage.md:19` records that choice as "`main`, `Cargo.toml` version `0.0.1`, what the last release ships". The published release is the tag `v0.0.1`, which the step specification puts "937 commits ago" (`docs/plans/agent-scaffold.steps/ship-v0-0-2.md:3`). The two trees are not the same code, and on one of the three files in scope they do not behave the same way. Out-of-scope observation 1 sets out the measurement and why I did not raise it as a finding.

Two further trees carried one source mutation each, in their own target directories, for the `read_optional` probes. I confirmed each mutation against HEAD by diff before building.

Every fixture, symbolic-link target, escape target and cargo target directory sits under my own scratch subdirectory. I built every fixture from scratch and reused no shape from the implementer or from an earlier round. The `[[principle]]` fixture carries `tags`, `default_selected` and `default_order`, and the `[[var]]` fixture uses `name`, not `key`: my first `[[var]]` fixture was wrong and the tool rejected it with `missing field 'name'`, which I fixed before drawing any conclusion from it.

TWO fixtures needed mode `000`, because nothing else produces `EACCES`. Both were restored, and the restoration is in the run output: `stat -c '%a %n'` prints `644` for the file and `755` for the directory, the contents read back, and `find <scratch> -perm 000` returns nothing.

GATES I RAN MYSELF AT HEAD:

| gate | result |
| --- | --- |
| `cargo test --no-fail-fast` | 468 passed, 0 failed, over 11 result lines, exit 0 |
| `cargo clippy --all-targets -- -D warnings` | clean, exit 0 |

`git status --short` is empty in this worktree and in the main repository, apart from this file.

## Every surviving sentence in the three edited passages

The `CHANGELOG.md:37` passage is one bullet of five sentences. Only its second sentence was edited, so the other four survive inside the edited passage and I tested them too.

| id | sentence | the falsifying case I ran | measured result |
| --- | --- | --- | --- |
| `C1` | "The three files the tool reads by literal name, `pack.toml`, `principles.toml` and `instrument.md`, are contained too" | Look for a FOURTH literal that reaches a pack path, and for one of the three that is not contained | Exactly three literal reads on a `PackSource` in production code: `src/manifest.rs:547` (`pack.toml`), `src/main.rs:264` (`principles.toml`), `src/main.rs:298` (`instrument.md`). `workflow.toml` is read from the project's `.agents/`, never from a pack. All three refused at HEAD. TRUE |
| `C2` | "In 0.0.1 each was read through a symbolic link out of the pack, the same leak as the two fields above on three more paths" | Three packs, one per literal, each deploying that literal as a link to a file outside the pack, run on BOTH 0.0.1 binaries | Against `main`, the baseline rounds 1 to 4 used: all three read the outside file, exit 0. TRUE. Against the published tag `v0.0.1`: `principles.toml` and `pack.toml` read the outside file, `instrument.md` is never read at all. Baseline-dependent; see observation 1 |
| `C3` | "They are now refused by the same rule." | Run all three link fixtures at HEAD | All three exit 2, write 0 files, and each message names the value, then the cause, then the same rule text. TRUE |
| `C4` | "A file the tool cannot read ... produced an empty principle set ... at exit 0 with empty stderr in 0.0.1 ... it now exits 2 naming the file, which matches a malformed `principles.toml`, already loud in 0.0.1" | Invalid UTF-8 in `principles.toml`, then a malformed `principles.toml`, at all three binaries | Published 0.0.1: exit 0, EXACTLY 0 stderr bytes, empty principle set. HEAD: exit 2, `could not read the pack's principles.toml: stream did not contain valid UTF-8`. Malformed at the published 0.0.1: exit 2, same parse message form as HEAD. TRUE at both baselines. The "empty instrumentation block" half is baseline-dependent, as `C2` |
| `C5` | "ABSENCE IS UNCHANGED ... byte for byte what 0.0.1 produced." | A pack shipping neither literal, at all three binaries, md5 of the rendered `AGENTS.md` | `d38c0b5760f21fb257dbe46ef146e7b5` at all three, exit 0 at all three. TRUE, and true against the published 0.0.1 as well as against `main` |
| `M1` | "It does not make swallowing impossible, and nothing in Rust can: a caller may still write `.unwrap_or_default()` on this too." | Write the swallow, at an existing caller and at a new one, and try to compile | Both compile. probe1 (`pack_principles` swallows) `cargo build --release` exits 0 and produces a working binary. TRUE |
| `M2` | "What it buys is that the correct optional-read primitive exists and is the obvious one to reach for." | Check the primitive exists and that the callers reach for it | `read_optional` exists and both production callers use it rather than `read`. No measurable claim beyond that. TRUE |
| `M3` | "The invariant is held by review, not by the compiler." | Find a compiler rule that stops a swallow. Two probes: swallow at an existing caller, and swallow at a new caller | probe1 builds at exit 0 (dead-variant WARNING only). probe2, a new swallowing caller, passes `cargo clippy --all-targets -- -D warnings` at exit 0. Nothing in the type system rejects a swallow. TRUE |
| `P1` | "The read did not produce text: a containment refusal, or an unreadable path." | Enumerate every construction of `PrinciplesError::Read` and drive each reachable error kind | ONE construction site (`src/main.rs:264`). Six reachable shapes, listed below. No text was produced in any of them. TRUE of every one |
| `P2` | "Distinct from `Parse` because the file never became text, so telling the user it did not parse would name the wrong step." | Try to reach `Parse` without text, and to reach `Read` with text | `Parse` is constructed only from `pack::parse_principles(&toml)`, which needs the text. None of the six `Read` shapes printed a parse message, and the malformed file printed only the parse message. TRUE |
| `P3` | "The file was read and is not a valid `principles.toml`." (untouched, beside the new sentence) | The malformed fixture | Exit 2, `could not parse the pack's principles.toml: TOML parse error at line 1, column 6`. TRUE |
| `P4` | The enum's lead doc: "ABSENCE is not in here ... an empty set rather than a failure." (untouched, beside the new sentence) | A pack shipping no `principles.toml`, and a pack shipping a DANGLING link | Both exit 0 with an empty principle set and empty stderr. TRUE |

### `P1`: every way `PrinciplesError::Read` can be constructed

One construction site, `src/main.rs:264`, `.map_err(PrinciplesError::Read)` over `read_optional("principles.toml")`. `read_optional` folds only `NotFound` into `Ok(None)`, so every other `ReadError` reaches the variant. Each row is a run of the HEAD binary against its own fixture pack:

```
shape                              error                                    text produced?
containment refusal (link out)     Escapes                                   NO (refused before open)
unreadable FILE (mode 000)         Io(Permission denied, os error 13)        NO
principles.toml is a DIRECTORY     Io(Is a directory, os error 21)           NO
invalid UTF-8 contents             Io(stream did not contain valid UTF-8)    NO
symbolic-link loop                 Io(Too many levels of symbolic links)     NO
unreadable pack ROOT (mode 000)    Io(Permission denied, os error 13)        NO
```

Every shape exits 2 and writes 0 files. "The read did not produce text" is true of all six, which is the whole of the first clause.

ON THE ENUMERATION AFTER THE COLON. Five of the six sit under "a containment refusal, or an unreadable path" without argument. The sixth, invalid UTF-8, is the one where "unreadable" is loosest: the bytes were read and the path is readable, and what failed is that the bytes are not text. I do not raise it, for two reasons I checked rather than assumed. The tree's own vocabulary already puts that case under "cannot read": `CHANGELOG.md:37` reads "A file the tool cannot read (invalid UTF-8, or one it lacks permission to read)", and round 4 confirmed that sentence true. And the gloss is inherited rather than introduced: the retired sentence said "an unreadable file" and had to cover the same case. The edit widened the noun from "file" to "path", which is exactly what the round 4 triage asked for at `v002-r4-triage.md:159`, and it changed nothing about how the UTF-8 case is glossed.

ON WHETHER THE NEW SENTENCE IS NOW MISLEADINGLY WEAK. It is weaker: it no longer asserts that the file is present. That is the point of the edit, and reality is on the weaker side of it. Of the six shapes, five involve a file that exists, and the unreadable-root shape can fire when no `principles.toml` exists at all. "The read did not produce text" is true in both cases, where "The file is present" was false in the second. The weakening removed a false claim and added none.

## Verdicts

ZERO findings. Nothing to report at any severity.

| id | verdict | severity |
| --- | --- | --- |
| - | no finding | - |

TWELVE sentences checked across the three passages. TWELVE survive. Nothing I ran falsifies any of them under the baseline convention rounds 1 to 4 used and recorded.

THE ONE RESULT THAT IS NOT UNCONDITIONAL is `C2`, and the condition is which binary the words "In 0.0.1" name. It is not a defect this round's deletion introduced, it is not specific to the sentence I was asked to check, and resolving it is a scope decision rather than a truth verdict on three deletions. It is written up in full below, as my brief directs for anything real but out of scope, and I recommend the human read it before treating this round as clean.

FROM MY LENS, THIS ROUND IS CLEAN, with that observation on the record.

### Check 4: did any deletion leave a sentence whose meaning changed once its neighbour went

Three deletions, three answers, each measured rather than reasoned.

`CHANGELOG.md:37`. Removing "and its contents inlined" leaves "the same leak as the two fields above" carrying the comparison alone. The two fields above are described as copying and splicing an outside file into the scaffolded project, so the comparison could over-claim for `pack.toml`, whose own text round 4 measured as not inlined. It does not over-claim. I ran a pack whose `pack.toml` is a link to an outside manifest that declares a `[[var]]`:

```
outside manifest:  [[var]] name = "brand"  default = "VALUE-AUTHORED-INSIDE-THE-OUTSIDE-MANIFEST"
                   [[asset]] source = "../outside/body.md"  dest = "planted.md"

published 0.0.1  exit=0  2 files written
   AGENTS.md   | BRAND:VALUE-AUTHORED-INSIDE-THE-OUTSIDE-MANIFEST
   planted.md  | OUTSIDE-ASSET-BODY
main             exit=0  identical
HEAD             exit=2  refused, 0 files
```

So a linked `pack.toml` reads a file outside the pack, lands text authored inside that outside file into the scaffolded `AGENTS.md`, and copies a further outside file into the project. That is the same leak as the two fields above, at the containment level and at the outcome level. The surviving comparison is fair under both a narrow reading (the read escaped the pack) and a broad one (outside content reached the project).

`src/manifest.rs`. The deleted clause was the paragraph's only claim about the project's gate. Its neighbours do not inherit anything false. `M3` remains true, and the direction of the change is worth stating plainly: with the clause gone, the paragraph now says less than the tree currently enforces, because at the two existing call sites the dead-variant lint does stop a swallow under `-D warnings` (probe1, exit 101). Understating a guard is the safe direction and states nothing false, so I raise nothing. probe2 shows why the understatement is also correct as a general claim: a third caller swallows and passes every gate at exit 0.

`src/main.rs:229`. The replacement sentence and the untouched sentence after it now say a similar thing twice ("The read did not produce text", then "the file never became text"). That is redundancy, not contradiction, and the second sentence was ruled true and not-to-be-touched at `v002-r4-triage.md:161`. The second sentence still says "the file" where the first deliberately says "the read", so on the unreadable-root shape with no `principles.toml` present it refers to a file that is not there. That shape is a RECORDED RESIDUAL and I do not raise it.

## Out-of-scope observations

None of these is a finding. I report them because they are real and measured, not to affect this round's outcome.

### 1. "In 0.0.1" names a version that does not behave the way rounds 1 to 4 measured

THE MEASUREMENT, three packs, one literal each deployed as a link to a file outside the pack:

```
                            published v0.0.1        main                   HEAD
principles.toml -> outside  exit 0, LEAKED          exit 0, LEAKED         exit 2, refused
                            principle rendered      principle rendered
instrument.md   -> outside  exit 0, NOTHING READ    exit 0, LEAKED         exit 2, refused
                            "I:{{instrument}}"      fragment inlined
pack.toml       -> outside  exit 0, outside         exit 0, outside        exit 2, refused
                            manifest obeyed         manifest obeyed
```

The published 0.0.1 does not read `instrument.md` because that version has no instrumentation at all. Measured at the tag `v0.0.1`: zero occurrences of the string `instrument` in `src/*.rs`, no `--instrument` flag in `--help`, and a pack directory of `AGENTS.md pack.toml plan-template.md principles.toml prompts` with no `instrument.md`. `pack/instrument.md` was added on 2026-07-15 by `1cd3211`, five days after the 0.0.1 release, and `[[module]]` guidance on 2026-07-17 by `260a222`.

So `C2` is true of three files against `main` and true of two of three against the tag. The same applies to the two other "in 0.0.1" clauses in the same bullet that mention an instrumentation block, and, by inspection rather than measurement, to `CHANGELOG.md:36`'s `[[module]]`'s `guidance` field, which the published 0.0.1 also does not have.

WHY THIS IS NOT A FINDING FROM MY LENS:

- The baseline is a convention rounds 1 to 4 adopted, recorded and applied to every comparative claim in the section (`v002-r3-triage.md:19`). Under the convention in force when this sentence was written and reviewed, the sentence is true of all three files, which I reproduced independently.
- This round's deletion did not create it. The words "In 0.0.1" and the three-file list predate `e3a466e`.
- The remedy is not local. It reaches settled text in neighbouring bullets that my brief places out of scope, and the question of what the 0.0.2 section is written against is a scope decision for the human.

WHY THE HUMAN MUST STILL SEE IT. The evidence against the convention is the project's own. The step specification states one release, `v0.0.1`, 937 commits ago, and its release criterion 3 requires "`agent-scaffold` 0.0.1 is still installable", so 0.0.1 is a real artefact a reader can install and check. `CHANGELOG.md:42-52` is the `## [0.0.1]` section, and its Added list has no instrumentation, no modules and no `validate`. The release mechanics at `ship-v0-0-2.md:112` close a `## [Unreleased]` section as `## [0.0.2]`, and in Keep a Changelog an Unreleased section holds everything since the last release. A reader upgrading from the installed 0.0.1 therefore reads "In 0.0.1 each was read through a symbolic link out of the pack" and looks for a linked `instrument.md` their version never read. The error is in the over-warning direction, unlike round 4's `P2`, which pointed a forensic reader the wrong way, and it changes no exit code and no byte of output.

If the human rejects the `main`-as-0.0.1 convention, `C2` becomes false of one of its three files and several settled sentences fall with it. I did not make that call on my own authority in a round that converges at the cap.

### 2. A dangling `principles.toml` link is folded into absence, including one that points outside the pack

```
principles.toml -> ./nope.toml           HEAD exit 0, empty principle set, empty stderr
principles.toml -> <outside>/nope.toml   HEAD exit 0, empty principle set, empty stderr
```

The second is an escaping link, and it reads as "the pack ships no such file" rather than as a refusal, because `fs::canonicalize` fails with `NotFound` before the containment rule can answer. This is the documented design and `src/manifest.rs:502-504` states it in those terms ("A path that cannot be canonicalised (most often a missing file) is an I/O error, so a missing pack file still reports as missing rather than as an escape"). Nothing is read from outside the pack in either case. The read boundary is settled, so this is a note, not a finding.

### 3. A linked `pack.toml` can inline its own text after all

Round 4's `P2` recorded that a linked `pack.toml` is "a DIRECTIVE leak and not a content leak" and that "nothing of it was inlined". The fixture behind that used a manifest with no `[[var]]`. With one, the manifest's own authored text lands in the scaffolded `AGENTS.md` (observation under check 4 above). This changes nothing that needs fixing: the five deleted words are gone, and the sentence that survives is true either way. It matters only if someone later reasons from the round 4 record about what a linked `pack.toml` could carry.
