# `ship-v0-0-2-inc1` round 3: REVIEWER (text half of the scoped round)

Independent reviewer. I did not write this change and did not review rounds 1 or 2. Every figure below is my own measurement, made in the detached worktree `.claude/worktrees/r3-text` at `53c3a27`. No tracked file was modified in this worktree or in the main repository except this findings file.

## Artifact reviewed

The text half of round 2's enumerated blast radius. The round 2 fix pass is ONE commit, `53c3a27` ("fix: report a refused pack literal instead of calling it absent"), 6 files, 354 insertions, 38 deletions. I read it in full, then read `git diff main..HEAD` for the files it touches.

Read in full first: `docs/plans/agent-scaffold.reviews/v002-r2-triage.md` and `docs/plans/agent-scaffold.reviews/v002-r2-reviewer-claims.md`.

### Method

THREE release binaries, one `CARGO_TARGET_DIR` each, from three separate `git archive | tar -x` extracts, verified distinct by `md5sum`:

```
8643b440815d5eb3dac56d98ab68855e  tgt-head/release/agent-scaffold   (HEAD, 53c3a27, the round 2 fix pass)
19da1197f2ef6af3120cb582df26dc71  tgt-pre/release/agent-scaffold    (f2308d6, the round 1 fix pass, PRE)
7c5bd336ff852eb74e208537c583495a  tgt-main/release/agent-scaffold   (main, what 0.0.1 ships)
```

`PRE` isolates what this pass changed. `MAIN` is what the CHANGELOG's 0.0.2 section is measured against, and it turned out to matter (finding 5).

- Fixtures under my own scratch subdirectory only. Every escape target and every symlink target is inside it.
- `strace -f -y -e trace=openat,open,stat,lstat,newfstatat,readlink,readlinkat,statx,read` for the reached-the-filesystem and never-read-the-contents claims.
- Mutation testing for the six tests added this pass: eight mutations, each applied alone in a `git archive` copy, run with `--no-fail-fast` (without it `cargo test` stops at the first failing target and hides the rest of the kill set). Two of the eight were applied to the PRE tree, to answer what pre-existing coverage held before this pass.
- A compile probe for one API claim: a swallowing caller added to a copy of the tree, built and run.
- A standalone `rustc` program reproducing both `safe_path` predicates, to test the Unix half of the Windows claim over 22 path shapes.
- Real GNU stow (`nix shell nixpkgs#stow`) to produce the deployment shape the CHANGELOG names, rather than hand-building an approximation of it.

Gates I ran myself at HEAD, all green: `cargo test` 467 passed 0 failed across 11 result lines; `cargo clippy --all-targets -- -D warnings` clean; `validate --source ... --metrics ...` `332 records, valid` / `99 steps, 75 questions, valid` exit 0; `validate --workflow` `workflow invariants hold` exit 0; `render --check --strict` `up to date` exit 0.

### The headline

The two claims round 2 falsified are now TRUE, and I verified each by constructing and running the case that would falsify it, not by reading. `src/manifest.rs:454-458` no longer says the literals can never escape, and the literal that escapes is refused. `src/manifest.rs:543-546` no longer says the refusal cannot fire at `manifest()`, and I fired it. Both remedies hold.

Assertions checked: 82. Confirmed outright: 67. Falsified: 12, grouped into 7 findings, all `low`. Three more (`R6`, `C2`, `T3`) are imperfect but defensible and are recorded as observations rather than raised. Separately, the six tests added this pass were each checked for non-vacuity by mutating the behaviour it names, and all six died as required.

## Assertion table

Every assertion I checked, the case I constructed to falsify it, and the measured result. `MF` is `src/manifest.rs`, `SP` is `src/safe_path.rs`, `MN` is `src/main.rs`, `CL` is `CHANGELOG.md`, `RM` is `README.md`, `PT` is `tests/pack_source_stays_inside_the_pack.rs`. Line numbers are at HEAD.

### The two claims round 2 falsified (`B1`'s doc half)

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| R1 | `MF:454-458`: the fixed literals "are subject to the same two rules" | Made each of `pack.toml`, `principles.toml` and `instrument.md` a symbolic link to a file outside the pack, one at a time and all together | All three refused at exit 2, output directory empty after every run. CONFIRMED |
| R2 | `MF:456-457`: "a literal names nothing that escapes" | Checked the three literal strings against both rules | All three are relative, carry no `..`, and pass the lexical rule; only the resolved rule can refuse them. CONFIRMED |
| R3 | `MF:457-458`: "the file it names can still BE a link out, and then it is refused like anything else" | The same three fixtures, compared against a refused `[[asset]].source` | Same predicate, same cause phrase, same exit code, nothing written. CONFIRMED |
| R4 | `MF:543-545`: "a `pack.toml` that is itself a link out of the pack is refused here" | A pack whose `pack.toml` is a symbolic link to a manifest outside it, with no `principles.toml` present | `` error: `pack.toml` is not a contained pack path (it resolves outside the pack directory, through a symbolic link) `` , exit 2, output directory empty. CONFIRMED |
| R5 | `MF:545-546`: "the refusal is mapped to an `io::Error` rather than special-cased, so this stays a plain read" | Read `MF:547-550`; mutated the map to drop the name | One line, `.map_err(io::Error::from)?`, no special case. The mutation killed `a_linked_pack_manifest_is_refused_with_a_message_naming_it`. CONFIRMED |
| R6 | `MF:452-453`: "Every path a pack author writes reaches the filesystem through here" | Looked for a pack-author path reaching the filesystem elsewhere | `[[asset]].dest` does, at `MN:88` and `MN:120`. Round 1 ruled the sentence can stand on its parenthetical's scope, and round 2 recorded it as observation 2 rather than a finding. Not raised; see observation 2 |

### The `ReadError` and `LoadError` doc block (round 2's claims 2)

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| E1 | `MF:400-401`: "the path was refused by a containment rule, or the read itself failed" | Enumerated the variants and every reachable producer | Two variants, `Escapes` and `Io`, and no third outcome. CONFIRMED |
| E2 | `MF:403-404`: "The file's CONTENTS were never read" | `strace` of a resolved refusal on an `[[asset]].source` naming a link out | No `open`, no `openat`, no `read` of the target anywhere in the trace. CONFIRMED |
| E3 | `MF:406-407`: "Deciding where a path lands does stat and follow links, so a refusal is not free of filesystem access; it is free of the read it refuses" | Same trace | `readlink(".../pack/leak.md", ".../secret.md") = 132` then `readlink(".../secret.md") = -1 EINVAL`, both before the refusal. The path reaches the filesystem and the contents are not read, which is exactly what the sentence now says. CONFIRMED, and it replaces the sentence round 2 falsified |
| E4 | `MF:412-414`: `Escapes` fires when the path "is absolute, it carries a `..` component, or it lands outside the pack once symbolic links are followed" | Fired all three shapes at the read site | All three produce `Escapes`; the cause phrase in the message matches the input in each. CONFIRMED, and the third cause is the one round 2 found missing |
| E5 | `MF:414-415`: "Refused BEFORE the file is opened, so the outside file's contents are never read" | `strace` of a lexical refusal (`../secret.md`) and of a resolved one | Lexical: zero syscalls touch the target at all. Resolved: two `readlink`, no open. CONFIRMED |
| E6 | `MF:191-193`: `UnsafeAssetSource`'s three causes | Ran an absolute, a `..`-bearing and a linked `source` | All three produce the variant. CONFIRMED |
| E7 | `MF:201-203`: `UnsafeModuleGuidance`'s three causes | Same three through `[[module]].guidance` | All three produce the variant, and the message names the module and the guidance path, never "asset source". CONFIRMED |
| E8 | `MF:226-228`: `UnsafeAssetDest`'s enumeration is STILL only "absolute, or ... `..`", deliberately left alone | Constructed the third cause on the write side: a `dest` of `linkdir/escaped.md` where `linkdir` inside `--output-dir` is a symbolic link out of it | The run wrote the file OUTSIDE the output directory at exit 0, reporting `create linkdir/escaped.md` / "Wrote to <output-dir>". So the write side really is lexical only and there is no third cause to enumerate. CONFIRMED; leaving this variant alone was correct. See observation 1 for what the same measurement says about round 1's `A2` |
| E9 | The claim "the relative path was refused because it leaves the pack" is gone | `grep -n "leaves the pack" src/manifest.rs` | Zero hits in the file. CONFIRMED |
| E10 | The claim "The path never reached the filesystem" is gone | `grep -n "never reached the filesystem" src/` | Zero hits in the tree. CONFIRMED |

### `LoadError::UnreadablePackFile` (new this pass)

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| U1 | `MF:214-218`: "An ABSENT file is not this error" | A pack shipping no `instrument.md`, run with `--instrument`; and a DANGLING pack-internal link (`principles.toml -> ./nowhere.toml`) | Both exit 0 with an empty block. The dangling link canonicalises to `NotFound`, so it lands on the absence path rather than the refusal path. CONFIRMED |
| U2 | `MF:220-222`: `rel` exists "so the message names the file even when the inner error does not (an unreadable file reports only the I/O cause)" | `chmod 000` on `instrument.md`, then `--instrument` | `` error: could not read the pack's `instrument.md`: Permission denied (os error 13) ``. The inner error names nothing; the outer `rel` is what names the file. CONFIRMED. Nothing pins it: mutation D, which drops `rel` from the message, killed no test. See observation 4 |
| U3 | `MF:320-323`: the Display form | Ran both reachable causes through it | `` could not read the pack's `instrument.md`: <cause> `` for a refusal and for a permissions failure. CONFIRMED |

### `PackSource::read_optional` (new this pass)

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| O1 | `MF:513-515`: "`Ok(None)` means the pack ships no such file ... Every other outcome, a containment refusal included, is an `Err`" | Refusal, permissions failure, invalid UTF-8, symlink loop, and genuine absence, all through both literal callers | Absence alone is `Ok(None)`. Refusal, `Permission denied (os error 13)`, `stream did not contain valid UTF-8` and `Too many levels of symbolic links (os error 40)` are all `Err` reported at exit 2. CONFIRMED |
| O2 | `MF:535-537`: "`Embedded` reports a missing file as `NotFound` too, so the built-in pack keeps answering 'ships none' the same way" | The unit test's `builtin()` assertions, plus a default scaffold | `builtin().read_optional("principles.toml")` is `Some`, `builtin().read_optional("no-such-file.md")` is `None`, and the default scaffold still renders its principles. CONFIRMED |
| O3 | `MF:524-525`: "a caller may still write `.unwrap_or_default()` on this too" | Wrote one | It compiles. CONFIRMED, and it is what falsifies O4 |
| O4 | `MF:527-528`: "a caller who still discards a refusal must write an explicit arm to do it, which is visible in review and findable with one grep" | Replaced `.map_err(PrinciplesError::Read)?` with `.unwrap_or_default()` in `pack_principles`, changing nothing else, and built it | Builds clean. The probe binary prints an empty principle list at exit 0 on the pack HEAD refuses. No arm, no `ReadError` named anywhere. FALSIFIED. FINDING 4 |
| O5 | The absence path is not widened by accident: only `NotFound` is folded | Mutation B, mapping `PermissionDenied` instead of `NotFound` to `Ok(None)` | Killed 15 tests including the new absence pin. The boundary is exactly `NotFound`. CONFIRMED |

### `PrinciplesError` (new this pass, `src/main.rs`)

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| N1 | `MN:224-226`: "ABSENCE is not in here: a pack that ships no `principles.toml` ... is an empty set rather than a failure" | A pack shipping no `principles.toml`, at `--list-principles` and at `--write` | Empty set, exit 0, `AGENTS.md` written with an empty principles block. CONFIRMED |
| N2 | `MN:229-230`: "The file is present and could not be read: a containment refusal, or an unreadable file" | Made the PACK DIRECTORY unreadable (`chmod 000`) with no `principles.toml` in it at all | `error: could not read the pack's principles.toml: Permission denied (os error 13)`, exit 2. The file is not present, the unreadable thing is the directory, and the message names a file that does not exist. FALSIFIED. FINDING 7 |
| N3 | `MN:230-231`: "Distinct from `Parse` because the file never became text, so telling the user it did not parse would name the wrong step" | A refused `principles.toml`, a malformed one, and one that is both | Refused reports "could not read", malformed reports "could not parse", and the linked-and-malformed one reports "could not read" because the refusal precedes the parse. CONFIRMED |
| N4 | `MN:2135-2137`: "Printing 'could not parse' for a containment refusal would name a step that never ran" | Mutation C, which restores the parse wording for a read failure | Killed `a_linked_principles_file_is_reported_not_silently_dropped`, whose body asserts `!stderr.contains("could not parse")`. CONFIRMED and pinned |

### `safe_path`'s module doc (round 2's claims 3)

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| S1 | `SP:12-14`: a `[step.provenance].findings` ref "is shape-checked with `is_contained_relative` and is never joined onto a directory and never read" | Traced every use of a findings ref; set one to `nowhere/absent-findings.md` and ran `validate --source` and `render` under `strace` | `src/plan/source.rs:643-644` shape-checks it via `is_safe_sidecar_ref`, which is a pure delegate to `is_contained_relative` (`:480-482`). `validate --source` accepts at exit 0, `render` succeeds at exit 0, and ZERO syscalls name the path. CONFIRMED |
| S2 | `SP:14-15`: "`render` puts it on the Roadmap Notes line as text" | Same render, then grepped the output | `` | `alpha` | complete | ... why: decisions Q-2; findings nowhere/absent-findings.md; commits abc1234 | `` at `render-fixture.md:44`. CONFIRMED |
| S3 | `SP:16-19`: `resolved_within` must not be applied, because it requires the path to exist while `plan::source` deliberately does not existence-check one | Read `src/plan/source.rs:245-247`; ran the absent-path case through `resolved_within`'s own unit test | The contract is stated in the code in those terms, and `resolved_within` on an absent path is `Err`. CONFIRMED |
| S4 | `SP:4-5`: "`plan::source` joins a `[meta].sidecars` front/tail ref onto the plan directory to READ it" | Searched `src/plan/source.rs` for any join, any read, any filesystem access | `grep -n "join(\|read_to_string\|fs::" src/plan/source.rs` returns NOTHING. The join and the read are at `src/plan/render.rs:167` and `:169`. The file says so itself twice, at `:474` and `:737-738`. FALSIFIED. FINDING 1 |
| S5 | `SP:5-6`: "`manifest` joins an `[[asset]].dest` onto the `--output-dir` to WRITE it" | Searched for the join | `src/manifest.rs:743` is a string check only. The joins are `src/main.rs:88` (`root.join(&asset.dest).exists()`) and `:120` (`let dest = root.join(&asset.dest)`). FALSIFIED. FINDING 1 |
| S6 | `SP:6-7`: "`manifest` joins ... an `[[asset]].source` or a `[[module]].guidance` onto the pack root to READ it" | Read `MF:493-509` | `root.join(rel)` and `fs::read_to_string` are both there. CONFIRMED, and it is the one clause of the three that holds |
| S7 | `SP:12`: "ONE CALLER USES THE LEXICAL RULE WITHOUT JOINING ANYTHING" | Enumerated every `is_contained_relative` caller | Four: the read site (joins), the `dest` check (its value is joined downstream), the sidecar refs (joined in `render`), and the findings ref (never joined). Exactly one. CONFIRMED |

### The Windows scoping (round 2's claims 5)

I cannot execute Windows: only `x86_64-unknown-linux-gnu` std is installed and there is no Windows target in this toolchain. W2 and W3 below rest on documented `std::path` behaviour, and I say so rather than implying I ran them. W1 and W4 I did run.

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| W1 | `MF:310-311`: "every refusal here has a lexical cause ON UNIX, where the two predicates agree and the fallback does not fire" | A standalone `rustc` program reproducing both predicates over 22 shapes, including `C:foo.md`, `c:temp`, `C:`, `\\?\C:\foo`, `\\server\share\f`, `\temp`, `a\b`, `//net/share`, `///triple`, `""`, `.`, `..`, `a//b`, a NUL-bearing name and a bidi-override name | The complement (`is_contained_relative` false AND `lexical_failure` `None`) is EMPTY on Unix. Every Windows-flavoured string is a single `Normal` component here and is accepted. CONFIRMED by running it |
| W2 | `MF:312-314`: "a `Prefix`-bearing path such as `C:foo.md` fails `is_contained_relative` while being neither absolute nor `..`-bearing" | Reasoned from documented `std::path` behaviour; could not execute | On Windows `Path::new("C:foo.md").components()` yields `Prefix(Disk('C'))` then `Normal("foo.md")`. `is_contained_relative` requires every component to be `Normal` or `CurDir`, so `Prefix` makes it false. `Path::is_absolute`'s documentation states that on Windows a path is absolute only with a prefix AND a root, so `c:temp` is not absolute; and there is no `ParentDir`, so `lexical_failure` is `None`. CONFIRMED from documented behaviour, NOT executed |
| W3 | `MF:314`: "so on Windows the fallback is reached" | Same | `UnsafeAssetDest` fires exactly when `is_contained_relative` is false (`MF:743`), and its Display calls `lexical_failure(dest).unwrap_or(...)` (`MF:317-318`). W2 gives false and `None`, so the `unwrap_or` arm runs. CONFIRMED from documented behaviour, NOT executed |
| W4 | `MF:310`: "The write side applies the lexical rule only" | The symlinked output subdirectory of E8, plus reading `apply_asset` | `MF:743` is `is_contained_relative` alone and `MN:120` is a bare `root.join`. A `dest` that lands outside through a link is accepted and written. CONFIRMED |
| W5 | The fallback is still a real phrase and not a panic | Read `MF:317-318`; ran a Unix `dest` of `C:foo.md` | `.unwrap_or("it is not accepted as an output path")` is intact. On Unix `C:foo.md` is accepted and writes a file of that name inside the output directory, so the fallback is not reached here. CONFIRMED; round 2's predicted failure mode was avoided |

### The CHANGELOG's 0.0.2 section

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| C1 | `CL:11`: the `Deprecated` bullet's four facts against `RM:7-11` | Compared clause by clause | Renamed to `agent-flow`, releases move once the rename lands, every published version stays installable, the name becomes reclaimable, and the README carries the contact route. All four match, and the README section title quoted in the bullet is exact. CONFIRMED |
| C2 | `CL:11`: "this entry points at it rather than restating it so the two cannot drift" | Compared the bullet against the README section | The bullet restates four of the five facts and points at the README only for the contact route, so the two CAN drift. Round 2's own remedy asked for both, which is how this arose. Not raised as a finding; see observation 3 |
| C3 | `CL:36`: "THE SHAPE THAT STOPS WORKING ... is a pack DIRECTORY whose files are symbolic links to targets OUTSIDE it" | Built it two ways: by hand, and with real GNU stow | Refused at HEAD in both. CONFIRMED |
| C4 | `CL:36`: "which is what GNU stow, home-manager, a nix profile and a symlinked dotfiles tree produce" | Ran `stow -d <stowdir> -t <target> mypack` and inspected what it made | Four symbolic links in the target, each `../stowdir/mypack/<file>`, exactly the shape. HEAD refuses that directory. The home-manager and nix-profile forms are the same shape into `/nix/store` and I did not build a generation to prove it. CONFIRMED for stow by running it; the other three rest on documented tool behaviour |
| C5 | `CL:36`: "That worked before this release" | The stow-produced pack against `MAIN` | `create AGENTS.md` / "Wrote to ... (1 changed, 0 left untouched)", exit 0, correct contents from the store. CONFIRMED |
| C6 | `CL:36`: "and is now refused, `pack.toml` included" | A pack whose `pack.toml` is a link out and which ships no other literal | Refused at exit 2 naming `pack.toml`. CONFIRMED |
| C7 | `CL:36`: "so on such a pack the refusal is the first thing you hit" | The fully-linked pack, at `--write` and at `--dry-run`, with and without `--instrument` | A refusal is the first and only output in every case, exit 2, nothing written. CONFIRMED as written. Note that the refusal you actually hit first names `principles.toml`, not `pack.toml`, because `pack_principles` (`MN:2133`) runs before `build_assets` (`MN:2197`). The sentence claims only that the refusal comes first, which it does; the test comment that makes the stronger claim is FINDING 2 |
| C8 | `CL:36`: "The trade is the one this project already took on the plan-side boundary and states in `README.md`" | Read `RM:242` | "the trade taken is that a loud refusal beats silently reading the wrong file", stated for the metrics and ledger boundary. CONFIRMED |
| C9 | `CL:36`: "Where every pack file resolves into ONE real directory, point `--template` at that directory" | Pointed `--template` at the store directory the four links resolve into | Exit 0, `AGENTS.md` with the correct principles and instrument content. CONFIRMED |
| C10 | `CL:36`: where files resolve into different targets "there is no single directory to point at and the pack must be materialised ... for example with `cp -rL`, or by cloning" | Built a four-store pack, one target directory per file; tried `--template` at one store; then `cp -rL` | `--template <store>` gives `error: No such file or directory (os error 2)`; `cp -rL` then `--template` gives exit 0 and correct output. `cp -rL` also fixes the single-store shape. CONFIRMED, both halves |
| C11 | `CL:36`: the two surviving-shape clauses are kept, not replaced | Ran both shapes at HEAD | A pack-internal link (relative target and absolute-inside target) and a `--template` naming a link to the pack directory both scaffold at exit 0 with the linked contents, including both together. CONFIRMED |
| C12 | `CL:36`: "Each caller labels the refusal with its own field ... and neither reports as a failed read, since nothing was opened" | Grepped all six read-side refusal messages for "could not be read" | Absent from the `source` and `guidance` messages, which the bullet's own two-field scope covers. CONFIRMED on that scope. See observation 5 for what the literals now say |
| C13 | `CL:37`: "the two call sites that read them discarded every error because the only outcome that had ever been reachable was 'the pack ships no such file'" | Made `principles.toml` unreadable (`chmod 000`) at `MAIN` and at `PRE` | Both exit 0 with an empty principles block. So an unreadable file was reachable, and silent, long before the containment rule. The same bullet's last sentence says so. FALSIFIED. FINDING 5 |
| C14 | `CL:37`: "a pack whose `instrument.md` was a link out ... scaffolded an `AGENTS.md` with the whole instrumentation contract missing, at exit 0 ... byte-identical to what a pack shipping no fragment at all produces" | The same pack against `MAIN`, `PRE` and `HEAD` | True of `PRE` only. At `MAIN` the linked fragment is READ and inlined: `AGENTS.md` contains `I:INSTRUMENT FRAGMENT`, and with real stow `I:FRAG`. `PRE` is an unreleased intermediate commit. FALSIFIED as an account of what the release changes. FINDING 5 |
| C15 | `CL:37`: "a pack whose `principles.toml` was a link out generated an empty principles block at exit 0" | Same three binaries | True of `PRE` (`P:` empty, exit 0). At `MAIN` the linked file is read and the principle renders (`P:1. My rule - One sentence.`). Same finding as C14. FINDING 5 |
| C16 | `CL:37`: "with a linked-and-malformed file losing even the parse error it used to report" | A `principles.toml` that is both a link out and invalid TOML | `MAIN` exit 2 `could not parse ... TOML parse error at line 1, column 7`; `PRE` exit 0, silent; `HEAD` exit 2, refusal. The parse error existed at the last release and was lost at `PRE`. CONFIRMED |
| C17 | `CL:37`: "everything else, a containment refusal included, is an `Err` the caller reports at exit 2 naming the file" | Refusal, permissions and invalid UTF-8 through both literals, at `--write` and `--dry-run` | Exit 2 every time, the file named every time, output directory empty every time, dry run byte-identical to write. CONFIRMED |
| C18 | `CL:37`: "ABSENCE IS UNCHANGED and stays silent: a pack that ships neither file still yields no principles and an empty instrumentation block" | A pack shipping neither, with `--instrument` | Exit 0, `AGENTS.md` is exactly `P:\nI:\n`. CONFIRMED |
| C19 | `CL:37`: "An UNREADABLE file (permissions, or invalid UTF-8) becomes loud where it was silent" | `chmod 000` and a non-UTF-8 byte sequence, both against `MAIN`, `PRE` and `HEAD` | `MAIN` and `PRE` exit 0 silently; `HEAD` exits 2 with `Permission denied (os error 13)` and with `stream did not contain valid UTF-8`. CONFIRMED, and this is the one part of the bullet that IS a genuine change from 0.0.1 |
| C20 | `CL:37`: "which matches a malformed one, already loud" | A malformed `principles.toml` against all three | Exit 2 with the same parse message at all three. CONFIRMED |
| C21 | The section describes everything user-visible in `git diff main..HEAD` | Enumerated the diff and matched each item to a bullet | Version bump to the heading; the `agent-flow` README section to the new `Deprecated` bullet; `F1`, `F4`, `F4b`/`A1`, `A4` to their `Fixed` bullets; the W5 waiver-ownership change to its own bullet; `src/plan/source.rs` is a pure extraction of an identical predicate with NO behaviour change, so it needs none. Spot-ran six `Added`/`Changed` claims: `audit --help`, `checks --help`, `status --json` carrying `metrics_absent_reason`, `next --json` carrying `resume_state_absent_reason` and `no_active_loop_reason`, `--module isolation` and `--module checks` both scaffolding, and `validate --workflow` exiting non-zero with no round log. All present. CONFIRMED |
| C22 | The section describes nothing that is NOT user-visible in `main..HEAD` | Ran each `Fixed` bullet's described prior behaviour against `MAIN` | The second `Fixed` bullet's central narrative describes `PRE`, an unreleased intermediate commit, and is false of `MAIN`. FALSIFIED. FINDING 5 |

### The README addition

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| D1 | `RM:325`: "Every file a pack reads must live inside the pack directory, and that is enforced rather than assumed" | Tried to make each of the five named entry points read an outside file | None succeeded. CONFIRMED |
| D2 | `RM:325`: the five entry points "are each refused if the path is absolute, carries a `..` component, or lands outside the pack once symbolic links are followed" | All three causes through `source` and `guidance`; the link cause through all three literals (the other two are not constructible for a fixed literal) | Every constructible pair refused, cause phrase matching the input. CONFIRMED |
| D3 | `RM:325`: "The refusal is loud, names the file, and writes nothing" | Counted the entries in the output directory after each of the six refusals | Exit 2 and ZERO entries written in every case, and every message names the offending file. CONFIRMED |
| D4 | `RM:325`: "This is the same rule ... that the metrics and ledger boundary above takes" | Gave `validate --workflow` an ABSOLUTE `--metrics` under the plan's root, and a `..`-bearing one that lands under it | Both accepted at exit 0 (`workflow invariants hold`). The pack rule refuses both shapes outright regardless of where they land. The two rules share the resolved half and differ on the lexical half. FALSIFIED. FINDING 6 |
| D5 | `RM:325`: "and the same trade" | Compared against `RM:242` | Same trade, stated in the repository's own words. CONFIRMED |
| D6 | `RM:325`: "A link INSIDE the pack is fine, and so is pointing `--template` at a link to the pack directory itself" | Four shapes: internal link with a relative target, internal link with an absolute inside target, a linked pack root, and a linked pack root containing an internal link | All four exit 0 with the linked contents. CONFIRMED |
| D7 | `RM:325`: "what is refused is a link whose target is outside" | The complement of D6 | Refused in every case. CONFIRMED |
| D8 | `RM:325`: the stow / home-manager / nix-profile recourse | Same measurements as C9 and C10 | CONFIRMED |

### The tests added this pass

Six tests were added. Each was checked by mutating the behaviour it names and confirming it dies. Eight mutations, each applied alone in a copy of the tree; two of them applied to the PRE tree to establish what pre-existing coverage held.

| # | Test or comment | Mutation applied | Result |
| --- | --- | --- | --- |
| T1 | `read_optional_answers_absence_with_none_and_never_with_an_error` (`MF:1318`) | B: only `PermissionDenied` folds to `Ok(None)`, so a genuine absence becomes an `Err` | FAILED as required, alongside 14 others. NON-VACUOUS |
| T2 | Its comment `MF:1319-1320`: "THE pin on the absence-stays-silent contract, which nothing held at any level before" | Applied the equivalent over-tightening to the PRE tree, separately for each literal | For `principles.toml`, 11 pre-existing tests died, including `a_pack_without_principles_has_an_empty_set` (`MN:2406`, present at `main`, added by `991a1d4`). For `instrument.md`, NOTHING died. So half the claim is false. FALSIFIED. FINDING 3 |
| T3 | Its comment: "`README.md` and `ModuleSpec.guidance`'s doc both promise that a pack shipping neither file is legitimate" | Read both | `RM:362` ("A pack that ships no `principles.toml` simply has no principles to select") and `RM:327` cover both files. `MF:96-97` covers only `instrument.md` ("the tool-computed `instrument.md`, which is silently optional") and says nothing about `principles.toml`. Loose rather than false, since the README does promise both. Not raised |
| T4 | `read_optional_reports_a_refused_literal_rather_than_calling_it_absent` (`MF:1342`) | A: `read_optional` swallows every error again | FAILED as required. Also FAILED under F (read site reverted to lexical only). NON-VACUOUS |
| T5 | `a_linked_instrument_fragment_is_reported_not_silently_dropped` (`PT:331`) | A | FAILED as required. NON-VACUOUS |
| T6 | `a_linked_principles_file_is_reported_not_silently_dropped` (`PT:356`) | A, and C (a read failure reported as a parse failure) | FAILED under both, so both its assertions bite. NON-VACUOUS |
| T7 | `a_linked_pack_manifest_is_refused_with_a_message_naming_it` (`PT:383`) | E: the `pack.toml` refusal loses the name of the file it refused | FAILED as required. NON-VACUOUS |
| T8 | Its comment `PT:385-386`: "It is the first read a run makes, so on a linked pack it is the first thing a user sees" | A pack whose `pack.toml` AND `principles.toml` are both links out, the shape the comment describes | The message names `principles.toml`. `pack_principles` (`MN:2133`) reads first; `manifest()` is reached only from `build_assets` (`MN:2197`). The test's own fixture ships no `principles.toml`, so its body never exercises the claim. FALSIFIED. FINDING 2 |
| T9 | `a_pack_shipping_neither_optional_literal_still_scaffolds` (`PT:410`) | B | FAILED as required. NON-VACUOUS |
| T10 | The added file header `PT:299-306`: "ADDED beside the eight cases above; none of those is replaced" | Counted the test functions at `f2308d6` and at HEAD | Eight before, the same eight present at HEAD, four added. CONFIRMED |
| T11 | The same header, `PT:302-303`: "their call sites once discarded every error because the only reachable one meant 'the pack ships none'" | Same case as C13 | An unreadable file was reachable and silent. The same false rationale appears at `CL:37`, in this header, and in the commit message; only the CHANGELOG copy is user-facing. FALSIFIED, folded into FINDING 5 |
| T12 | No existing assertion was weakened to make a new case pass | Diffed the two integration matchers and the eight pre-existing cases | `assert_refused` and `assert_guidance_refused` are unchanged and still require the message to name the offending value and its field. The four unit-test pins round 2 verified are unchanged. CONFIRMED |

Total: 82 assertions checked. 67 confirmed outright, 12 falsified (grouped into the 7 findings below), 3 imperfect but defensible and not raised (`R6`, `C2`, `T3`).

## Verdict table

Severity is absolute impact if left unfixed.

| id | severity | site | one line |
| --- | --- | --- | --- |
| 1 | low | `SP:4-6` | The module doc says `plan::source` joins a sidecar ref onto the plan directory to READ it and `manifest` joins a `dest` onto the output directory to WRITE it; `plan::source` contains no join, no read and no filesystem access at all, and `manifest` does not join a `dest`. |
| 2 | low | `PT:385-386` | A new test's comment says the `pack.toml` read "is the first read a run makes, so on a linked pack it is the first thing a user sees"; `principles.toml` is read first, and the test's own fixture avoids the case. |
| 3 | low | `MF:1319-1320` | A new test's comment says nothing held the absence-stays-silent contract "at any level before"; a test at `main` held it for `principles.toml`, and dies under the over-tightening. |
| 4 | low | `MF:527-528` | `read_optional`'s doc says a caller who discards a refusal "must write an explicit arm to do it"; a one-token change to `.unwrap_or_default()` compiles and swallows, as the same paragraph concedes two sentences earlier. |
| 5 | low | `CL:37` | The new `Fixed` bullet's account of the prior behaviour is wrong twice: the silent drop it describes was never in a released version (at `main` the linked file is READ), and "the only outcome that had ever been reachable" is contradicted by the bullet's own last sentence. |
| 6 | low | `RM:325` | "This is the same rule ... that the metrics and ledger boundary above takes" is false: that boundary accepts an absolute and a `..`-bearing path that land inside, and the pack rule refuses both outright. |
| 7 | low | `MN:229-230` | `PrinciplesError::Read`'s doc says "The file is present and could not be read"; with an unreadable pack directory the variant fires for a file that does not exist, and the message names it. |

Seven findings, all `low`. No `medium`, no `high`, no `critical`.

Both claims round 2 rated the `high`'s doc half are now true, verified by firing the case each one denies. Round 2's `medium` disclosure obligation is discharged on all five of its points and I could falsify none of them; the CHANGELOG's account of what stops working, what survives, and what the recourse is, is accurate down to the multi-store case, and I reproduced the named deployment shape with real GNU stow. The findings below are in prose the pass wrote fresh or in one sentence it edited without finishing.

## Finding 1 (low): `safe_path`'s module doc names the wrong modules as the joiners

QUOTED, `src/safe_path.rs:4-7`:

> Several callers join such a string onto a directory they own: `plan::source` joins a `[meta].sidecars` front/tail ref onto the plan directory to READ it, and `manifest` joins an `[[asset]].dest` onto the `--output-dir` to WRITE it and an `[[asset]].source` or a `[[module]].guidance` onto the pack root to READ it.

Round 2's claims 3 was the parenthetical inside the first clause, `(and a `[step.provenance].findings` ref)`. The pass removed the parenthetical, which was the right removal, and left the clause it sat in, which is false without it.

EVIDENCE. `plan::source` joins nothing and reads nothing:

```
grep -n "join(\|read_to_string\|fs::" src/plan/source.rs   ->  no output
```

The join and the read are in `plan::render`, at `src/plan/render.rs:167` and `:169`, both `load(&base.join(reference))`. `plan::source` says so itself, twice, in text the pass did not touch:

- `src/plan/source.rs:474`: "The render engine joins these free-string refs straight onto the base directory".
- `src/plan/source.rs:737-738`: "The `[meta].sidecars` front/tail refs are joined onto the plan directory by the render engine ... Reject both at the boundary (Principle 21) rather than at the render read."

The second clause is wrong in the same way. `manifest` does not join a `dest` onto the output directory: `src/manifest.rs:743` is `is_contained_relative(&spec.dest)`, a check on the string. The joins are `src/main.rs:88` (`root.join(&asset.dest).exists()`) and `src/main.rs:120` (`let dest = root.join(&asset.dest)`). Only the third clause holds: `manifest` really does join `source` and `guidance` onto the pack root and read them (`src/manifest.rs:493-509`).

WHY IT MATTERS, and it is the same reason round 2 gave for claims 3 rather than a general tidiness argument. The module doc is the map a later change navigates by, and this module now offers `resolved_within` beside the lexical rule. Further down the same doc, at `src/safe_path.rs:32-34`, the plan-side `validate --source` boundary "has only the lexical rule available, and is lexical for that reason rather than by preference", which is true precisely BECAUSE `plan::source` never touches disk: it is "a pure function over the string" in the file's own words (`src/plan/source.rs:237`, and again at `:613`). Line 5 tells a maintainer that `plan::source` already joins and reads, which is the premise under which applying `resolved_within` there looks safe. The module doc contradicts itself about the one fact that decides whether that change is allowed.

`low`: no outcome changes and the only reader misled is a maintainer, which is where this project put claims 2 and 3 last round.

## Finding 2 (low): a new test's comment claims a read order the run does not have, and the test avoids the case

QUOTED, `tests/pack_source_stays_inside_the_pack.rs:384-386`:

> The `pack.toml` literal goes through `io::Error::from` and carries no field label, so nothing pinned its wording. It is the first read a run makes, so on a linked pack it is the first thing a user sees.

The first sentence is true and the test that follows it is a good one; mutation E killed it. The second sentence is false in both halves.

EVIDENCE. `run_scaffold` reads `principles.toml` first, at `src/main.rs:2133` (`pack_principles(&source)`), and reaches `pack.toml` only through `build_assets` at `src/main.rs:2197`. Measured on the exact shape the comment describes, a pack whose files are ALL links out:

```
$HEAD scaffold --template <stow-produced pack> --output-dir ... --vcs none --instrument --write
error: could not read the pack's principles.toml: `principles.toml` is not a contained pack path
(it resolves outside the pack directory, through a symbolic link); ...
exit=2
```

Same message with `--dry-run` and same message without `--instrument`. The `pack.toml` refusal appears only when the pack ships no `principles.toml`, which is exactly what the test's own fixture does (`PT:387-396` writes `pack.toml` as a link and `a.md`, and nothing else). So the body never exercises the claim the comment makes, which is the class my scope names.

The neighbouring CHANGELOG sentence survives this, and I want to be precise about the difference. `CL:36` says "is now refused, `pack.toml` included, so on such a pack the refusal is the first thing you hit", which claims only that A refusal comes first. That is true and I measured it. The test comment claims that THE `pack.toml` refusal is what you hit, and that is the part that fails.

`low`: a maintainer is misled about the read order, which could send someone to `manifest()` looking for the earliest boundary in the run.

## Finding 3 (low): a new test's comment overstates what nothing held before

QUOTED, `src/manifest.rs:1319-1323`:

> THE pin on the absence-stays-silent contract, which nothing held at any level before: the shipped pack ships both optional files, so no scaffold-parity gate can catch an over-tightening that makes a MISSING file loud.

The test is a good addition and mutation B kills it. The justification is half false, and the false half is the one a maintainer would act on.

EVIDENCE. I applied the over-tightening the comment describes to the PRE tree, once per literal, and ran the PRE suite with `--no-fail-fast`:

```
PRE + "an absent principles.toml is loud":  11 tests died, including
    tests::a_pack_without_principles_has_an_empty_set
PRE + "an absent instrument.md is loud":     NOTHING died
```

`a_pack_without_principles_has_an_empty_set` is at `src/main.rs:2406`, was added by `991a1d4`, and is present at `main`. Its body is exactly the contract: a directory pack shipping no `principles.toml`, `pack_principles` on it, `assert!(principles.is_empty())`. So the contract WAS held at unit level for `principles.toml`, and by ten integration cases besides.

The claim is true for `instrument.md`, which nothing held at any level. The reason the sentence overstates is traceable: round 2's remedy text asserted the same thing ("THIS IS THE IMPORTANT ADDITION and nothing currently pins it at any level"), and the pass carried it across without checking it. That is worth recording, because it is the second time this round a remedy's own wording became a false claim in the tree (finding 5's C13 is the same pattern).

`low`: nothing behaves differently. The wrong action it invites is a maintainer trusting the sentence and not looking for the coverage that exists, or reading the older test as redundant.

## Finding 4 (low): `read_optional`'s doc claims a swallow needs an explicit arm, and it does not

QUOTED, `src/manifest.rs:524-528`:

> It does not make swallowing impossible, and nothing in Rust can: a caller may still write `.unwrap_or_default()` on this too. What it buys is that the correct optional-read primitive exists and is the obvious one to reach for, and that a caller who still discards a refusal must write an explicit arm to do it, which is visible in review and findable with one grep.

The first sentence and the second contradict each other, and the measurement is on the side of the first.

EVIDENCE. I changed `pack_principles` in a copy of the tree by exactly one expression, leaving the `match` and everything else alone:

```rust
-	match source.read_optional("principles.toml").map_err(PrinciplesError::Read)? {
+	match source.read_optional("principles.toml").unwrap_or_default() {
```

It builds clean (`cargo build --release`, exit 0, no warnings on that line). Run against the same pack HEAD refuses:

```
$PROBE scaffold --template <pack with a linked principles.toml> --list-principles
(empty)
exit=0
```

versus HEAD, which reports the refusal at exit 2. So a caller discards a refusal with no arm, no `ReadError` named anywhere, and nothing to grep for beyond `read_optional` itself. `Result::unwrap_or_default` applies because `Option<String>: Default`, which the same paragraph already says.

What the split DOES buy survives and is worth keeping in whatever replaces the sentence: the wrong primitive no longer yields a plausible-looking empty `String` (it yields `None`, which the caller must then convert), and the correct primitive exists and is the obvious one. That is the Principle 5 gain, and it is real. The overclaim is the "must write an explicit arm" clause.

`low`: an internal API doc, and the code it describes is correct. The wrong action is a future reviewer trusting the doc and not checking a new caller of `read_optional`, which is the precise failure this whole finding class exists to prevent.

## Finding 5 (low): the new CHANGELOG bullet describes a state no released version had

QUOTED, `CHANGELOG.md:37`:

> A pack file the tool reads directly can no longer be dropped silently when it is refused ... the two call sites that read them discarded every error because the only outcome that had ever been reachable was "the pack ships no such file". So a pack whose `instrument.md` was a link out of the pack scaffolded an `AGENTS.md` with the whole instrumentation contract missing, at exit 0, with empty stderr and an ordinary "30 changed" report, byte-identical to what a pack shipping no fragment at all produces

Two separate falsifications, and one sentence of the bullet that should survive any rewrite.

FIRST. The silent drop is true of `PRE` (`f2308d6`) and false of `MAIN`. At the last released version a pack whose `instrument.md` is a link out READS the outside file and inlines it:

```
pack/{pack.toml,body.md,principles.toml,instrument.md} all links out (real GNU stow)

MAIN:  create AGENTS.md / Wrote to ... (1 changed, 0 left untouched).  exit=0
       AGENTS.md:  P:1. My rule - One sentence.
                   I:FRAG
PRE:   exit=0, AGENTS.md:  P:
                           I:
HEAD:  error: could not read the pack's principles.toml: ... exit=2
```

So relative to 0.0.1 the transition for that pack is leak to loud refusal, which the FIRST `Fixed` bullet already describes in full. The second bullet's narrative describes a regression that existed only between two commits inside this unreleased cycle. A 0.0.1 reader is told their current version silently drops a linked `instrument.md`; it does not, it inlines the outside file.

The bullet's causal framing ("The containment rule above made a refusal reachable") does let a careful reader reconstruct that this is an intra-release state, which is why this is `low` and not higher. But a `Fixed` entry in a Keep a Changelog 0.0.2 section is read against 0.0.1, `CHANGELOG.md:3` says so in terms, and the concrete claims here ("at exit 0", "byte-identical") are false of 0.0.1.

SECOND, and independent of the first. "the only outcome that had ever been reachable was 'the pack ships no such file'" is false. An unreadable file was reachable and silent at `MAIN` and at `PRE`:

```
principles.toml, mode 000:
MAIN  exit=0  create AGENTS.md          (empty principles block)
PRE   exit=0  create AGENTS.md          (empty principles block)
HEAD  exit=2  error: could not read the pack's principles.toml: Permission denied (os error 13)

principles.toml, invalid UTF-8:
MAIN  exit=0   PRE  exit=0   HEAD exit=2  ... stream did not contain valid UTF-8
```

The same bullet's last sentence says exactly this: "An UNREADABLE file (permissions, or invalid UTF-8) becomes loud where it was silent". The two cannot both be true. The same false rationale is repeated in the added test-file header (`PT:302-303`) and in the commit message, but only the CHANGELOG copy is user-facing.

WHAT MUST NOT BE LOST. The bullet's last two sentences are the part of it that IS a genuine 0.0.1-relative change and needs to stay disclosed: absence stays silent (measured, `AGENTS.md` is exactly `P:\nI:\n`), and an unreadable or non-UTF-8 file becomes loud where it was silent (measured above, at `MAIN` as well as `PRE`). A rewrite that deletes the bullet outright would drop a real behaviour change from the release notes.

`low`: no user action turns on it and no security property is misstated, which is what separates it from round 1's `high`. What is wrong is an account of the previous release's behaviour in text bound for crates.io.

## Finding 6 (low): the README calls the pack rule the same rule as the metrics boundary, and it is stronger

QUOTED, `README.md:325`:

> The refusal is loud, names the file, and writes nothing. This is the same rule, and the same trade, that the metrics and ledger boundary above takes: a loud refusal beats silently reading a file the pack did not ship.

The trade half is true and is what round 2's remedy asked for. The rule half is false.

EVIDENCE. The metrics and ledger boundary applies resolved containment only: it asks whether the artifact lands under the plan's project root and does not care what the string looks like. Measured against the repository's own plan:

```
validate --source docs/plans/agent-scaffold.plan.toml --metrics /abs/.../docs/metrics/workflow.jsonl --workflow
  ->  workflow invariants hold        exit=0
validate --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/../metrics/workflow.jsonl --workflow
  ->  workflow invariants hold        exit=0
```

Both shapes are accepted. The pack rule refuses both outright, before asking where they land, and I measured that too: an absolute `source` gives "(it is an absolute path)" and `../secret.md` gives "(it carries a `..` component)", both exit 2. So the pack rule is the metrics rule plus a lexical rule the metrics boundary does not apply.

The mis-inference cuts both ways. A pack author could read across and conclude a `..`-bearing `source` that lands inside is fine, which it is not. A reader could conclude an absolute `--metrics` under the root is refused, which it is not, and `README.md:242` correctly says the opposite in its own section.

`low`, and lower than it would otherwise be for one reason: the sentence immediately before it states the pack rule correctly and in full, so the reader has the exact rule in the same paragraph. The fix is to keep "the same trade" and drop or qualify "the same rule".

## Finding 7 (low): `PrinciplesError::Read`'s doc asserts the file is present, and it need not be

QUOTED, `src/main.rs:229-231`:

> The file is present and could not be read: a containment refusal, or an unreadable file. Distinct from `Parse` because the file never became text, so telling the user it did not parse would name the wrong step.

The second sentence is true and well pinned (mutation C kills the test that holds it). The first is false in a reachable case.

EVIDENCE. A pack directory the process cannot read, containing NO `principles.toml`:

```
ls <pack>            ->  body.md  pack.toml       (no principles.toml)
chmod 000 <pack>
$HEAD scaffold --template <pack> --output-dir ... --vcs none --write
error: could not read the pack's principles.toml: Permission denied (os error 13)
exit=2
```

`resolved_within` canonicalises the pack root first (`src/safe_path.rs:104`), that fails with `EACCES` rather than `NotFound`, so `read_optional` does not fold it to `Ok(None)` and the variant fires. The file is not present, the unreadable thing is the directory, and the user-facing message names a file that does not exist while the real problem is one level up.

The `Ok(None)` boundary itself is right and I am not arguing for widening it: folding `EACCES` into absence would put back a silent swallow, which is the defect this pass exists to close. What is wrong is only the doc sentence, and the message it justifies.

`low`, and the weakest of the seven. The cause reported (`Permission denied`) is correct and a user will find the problem; the file it names is not the one at fault.

## Out-of-scope observations

Not findings. Recorded so the triage can see they were considered.

1. THE WRITE SIDE STILL ESCAPES THROUGH A LINK, and I measured it while confirming that `UnsafeAssetDest`'s enumeration was correctly left alone. A `dest` of `linkdir/escaped.md`, where `linkdir` inside `--output-dir` is a symbolic link out of it, writes the file outside the output directory at exit 0 while reporting "Wrote to <output-dir>". This is round 1's `A2`, which my scope lists as settled, and I raise it only to record that the same measurement which confirms `MF:226-228` is still true also confirms `A2` is still open. The asymmetry is now documented rather than hidden: `CL:38` says "The `dest` message states only the string half of the rule, because the write side applies only that half", which is accurate.
2. `MF:452-453`'s "Every path a pack author writes reaches the filesystem through here" still excludes `[[asset]].dest`. Round 1 ruled the sentence can stand on its parenthetical's scope and round 2 recorded it as its observation 2. I reached the same place and am not reopening it.
3. `CL:11`'s closing clause, "this entry points at it rather than restating it so the two cannot drift", is not quite true of itself: the bullet restates four of the five facts and points at the README only for the contact route, so the two can drift. I am not raising it, because round 2's remedy asked for both things at once (name the three facts, AND point at the README rather than restate it), and the implementer resolved that instruction reasonably. If the triage wants it closed, deleting the final clause is the whole fix.
4. `LoadError::UnreadablePackFile`'s `rel` field is UNPINNED. Mutation D, which changes the message to "could not read a pack file: {problem}" and drops the name, killed no test, because in the refusal case the inner `ReadError::Escapes` names the file anyway. The field earns its place in the permissions case (measured at U2), and that case has no test. If the triage wants one addition from this round, that is the cheapest.
5. `CL:36`'s "Each caller labels the refusal with its own field" is still true on the bullet's own two-field scope, which is how round 2 ruled it. It is now slightly more exposed than it was, because the same bullet explicitly discusses `pack.toml`, and the three literal callers label their refusals with a FILE rather than a field. Nothing a reader is told is wrong, since all three now name the file loudly, so I am not raising it.
6. The `--dry-run` and `--write` paths are byte-identical for every new refusal I constructed, so a preview never promises a run the write would reject. That is not claimed anywhere in the new text; I checked it because the new failure sites are new opportunities for the two to diverge.
7. Out of scope and not raised: `A2`, `A3`, `A5`, the audit's `F2` and `F3`, the `superseded by` projection defect, the rename itself, ANSI escapes in a `dest`, and the plan-side sidecar symlink hole. `validate --plan` still reports the pre-existing `superseded by` problem the spec excludes, which I confirmed incidentally and which is unchanged.
