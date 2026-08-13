# `ship-v0-0-2-inc1` round 3: TRIAGE

Independent triager. I did not write this change, did not review rounds 1, 2 or 3, and did not triage the earlier rounds. Every figure below is my own measurement. Where I reach the same number as a reviewer I say so, and where I reach a different one I say that too.

## Artifact

- Worktree `.claude/worktrees/tri-r3`, detached at `c45c501`.
- Artifact ruled on: `git diff main..HEAD`, 12 files, 1694 insertions, 148 deletions.
- Specification: `docs/plans/agent-scaffold.steps/ship-v0-0-2.md`.
- Settled and not reopened: `v002-r1-triage.md`, `v002-r2-triage.md`.
- Round 3 findings adjudicated: `v002-r3-reviewer-boundary.md` (1 finding) and `v002-r3-reviewer-text.md` (7 findings).

The two reviewers worked at `53c3a27`. My `c45c501` differs from it only by the two round 3 findings files and one metrics line (`git diff c45c501 53c3a27` touches those three files and nothing else), so every source line number a reviewer cites resolves here unchanged. I verified that before starting.

THE COMMIT CHAIN, established rather than assumed, because the whole re-seeding question turns on it:

| commit | role |
| --- | --- |
| `9364293` | `main`, `Cargo.toml` version `0.0.1`, what the last release ships |
| `d639a4b` | original implementation, `F4` (`dest`), creates `src/safe_path.rs` |
| `35028fd` | original implementation, `F1` (render escaping) |
| `b58f770` | `chore: release 0.0.2` |
| `4639d93`, `1da790d` | original implementation, `F4b` (`source`), then the shared read site |
| `fda1412` | ROUND 1 FIX PASS (the resolved rule) |
| `c45c501` | ROUND 2 FIX PASS (`read_optional`), the commit round 3 reviewed |

`fda1412` and `c45c501` are the same trees the reviewers call `f2308d6`/`PRE` and `53c3a27`/`HEAD`.

## Method

THREE release binaries, one `CARGO_TARGET_DIR` each, built from three separate `git archive | tar -x` extracts, verified distinct by `md5sum`:

```
6321b895a5906c8c54b71dcbc3d9aee9  tgt-head/release/agent-scaffold   (c45c501, HEAD)      0.0.2
93fdb21dd0c1c929256647ee7fd99679  tgt-pre/release/agent-scaffold    (fda1412, PRE)       0.0.2
71f2e1ac83a0140c7b1e9236a01a68c9  tgt-main/release/agent-scaffold   (9364293, MAIN)      0.0.1
```

Two further trees carried one source mutation each, in their own target directories:

```
8dc92cd0057c2e33dfb7e2b38cd9be3f  tgt-probe/release/agent-scaffold    (HEAD + pack_principles swallows)
                                  tgt-probe2                          (HEAD + a NEW swallowing caller)
                                  tgt-premut, tgt-premut2             (PRE + two absence over-tightenings)
```

Every fixture, symbolic-link target, escape target and cargo target directory is under my own scratch subdirectory. No tracked file was modified in this worktree or in the main repository, except this triage file. I used invalid UTF-8 and a symbolic-link loop in place of `chmod` for every unreadable case, so no measurement here depends on a permission bit.

GATES I RAN MYSELF AT HEAD:

| gate | result |
| --- | --- |
| `cargo test` | 467 passed, 0 failed, over 11 result lines (409+5+1+1+9+2+12+3+20+1+4) |
| `cargo clippy --all-targets -- -D warnings` | clean, exit 0 |
| `validate --source ... --metrics ...` | `333 records, valid`, `99 steps, 75 questions, valid`, exit 0 |
| `validate --source ... --workflow` | `workflow invariants hold`, exit 0 |
| `render --check --strict` | `up to date`, exit 0 |
| `validate --plan ... --metrics ...` | EXACTLY ONE problem, the `Q-43` `superseded by` one criterion 4 excludes, exit 1 |
| ASCII check on all 12 changed files | `0` on every file |

`cargo package --list` reports 373 files. It includes `CHANGELOG.md`, `README.md`, all 37 `src/` files, all 10 `tests/` files, and the 8 review documents. There is NO `[lib]` section and no `src/lib.rs`, so the crate is binary-only and docs.rs publishes no rustdoc for it. That measurement decides the user-facing boundary below.

## Verdicts

Eight findings reported, eight valid, none invalid, none duplicate, none out of scope. I lowered no severity and raised none, so every rating below is the reviewer's rating confirmed on my own measurement. NO `high` AND NO `medium` WAS RAISED THIS ROUND, and since I lowered nothing, this triage creates no dismissed-or-downgraded-high re-check obligation for the orchestrator.

`C1` is the boundary reviewer's only finding. `X1` to `X7` are the text reviewer's 1 to 7, renumbered here so they cannot be confused with the spec's `F1`/`F4`/`F4b` or with the text reviewer's own assertion tags. `Z1` is mine, raised in answer to the question about the other CHANGELOG bullets.

| id | verdict | severity | class | site | one line |
| --- | --- | --- | --- | --- | --- |
| `C1` | valid | low | USER-FACING | `src/main.rs:2133` message | A `--template` naming a plain file, a symlink loop or anything that is not a directory is reported as a failure to read the pack's `principles.toml`. |
| `X5` | valid | low | USER-FACING | `CHANGELOG.md:37` | The new `Fixed` bullet's account of the previous release is wrong twice, and one of the two is the inverse of the truth. |
| `X6` | valid | low | USER-FACING | `README.md:325` | "This is the same rule ... that the metrics and ledger boundary above takes" is false. The pack rule is that rule plus a lexical rule. |
| `Z1` | valid | low | USER-FACING | `CHANGELOG.md:38` | A third `Fixed` bullet documents a message wording that no released version ever printed. Raised by me, by no reviewer. |
| `X1` | valid | low | INTERNAL | `src/safe_path.rs:4-7` | The module doc names `plan::source` and `manifest` as the joiners. Neither joins what the sentence says it joins. |
| `X2` | valid | low | INTERNAL | `tests/pack_source_stays_inside_the_pack.rs:384-386` | A test comment claims `pack.toml` is the first read a run makes. It is not, and the fixture avoids the case. |
| `X3` | valid | low | INTERNAL | `src/manifest.rs:1319-1323` | A test comment says nothing held the absence contract "at any level before". A test at `main` held it for `principles.toml`. |
| `X4` | valid | low | INTERNAL | `src/manifest.rs:524-528` | `read_optional`'s doc says a swallowing caller "must write an explicit arm". One does not, as the same paragraph concedes. |
| `X7` | valid | low | INTERNAL | `src/main.rs:229-231` | `PrinciplesError::Read`'s doc asserts "The file is present". The variant fires when it is not. |

FOUR user-facing, five internal. Of the eight reviewer findings, three are user-facing and five are internal.

### The user-facing versus internal boundary, and why I drew it there

The test is: can a person who INSTALLS the crate encounter it. Three routes exist, and I measured each rather than reasoning about it.

- THE CRATES.IO PAGE renders `README.md`, which `Cargo.toml` names as `readme`. `X6` is on that page.
- THE BINARY prints error messages. `C1` is one of them, reachable by typing `--template ./pack.toml` instead of `--template ./pack/`.
- THE RELEASE NOTES are `CHANGELOG.md`, which `cargo package --list` confirms ships inside the `.crate` tarball and which this project's own release mechanics (`ship-v0-0-2.md`, "Release mechanics" step 2) treat as the release artefact. `X5` and `Z1` are in it.

The strict part of the boundary is the part I want on the record, because the tarball makes it non-obvious. `cargo package --list` also ships all 37 `src/` files and all 10 `tests/` files. So `X1`, `X2`, `X3`, `X4` and `X7` are all, literally, inside the artefact a user downloads. I still class them INTERNAL, on two measured grounds:

1. THERE IS NO LIBRARY TARGET. No `[lib]` section, no `src/lib.rs`. docs.rs builds no documentation for a binary-only crate, so `PackSource::read_optional`'s `pub` doc comment (`X4`) and `safe_path`'s module doc (`X1`) are never rendered anywhere a user looks, and no downstream crate can depend on `PackSource` to read them in an IDE. `pub` here means "visible across modules of one binary", not "published API". Had a lib target existed, `X1` and `X4` would move to user-facing and my recommendation below would change.
2. REACHING THE OTHER THREE REQUIRES OPENING THE SOURCE. Someone reading `tests/pack_source_stays_inside_the_pack.rs:385` or a private enum's doc comment in `main.rs` has stopped being an installer and become a maintainer or an auditor. That is a real reader with a real claim on accuracy, and it is why these findings are valid rather than dismissed. It is not the reader the phrase "a person who installs the crate can encounter it" names.

I argue the boundary this way rather than by artefact type because artefact type alone would have put `X4` on the wrong side: a `pub fn`'s doc comment is user-facing in most crates and is not user-facing in this one, and only the absent `[lib]` section says so.

## Finding by finding

Each section states what I measured myself, not what the reviewer reported.

### `C1` (low, USER-FACING): a `--template` root failure is reported against `principles.toml`

CONFIRMED, and reproducible in more shapes than the reviewer gave.

```
--template names a PLAIN FILE:
  HEAD  exit=2  error: could not read the pack's principles.toml: Not a directory (os error 20)
  PRE   exit=2  error: Not a directory (os error 20)
  MAIN  exit=2  error: Not a directory (os error 20)

--template names a SYMBOLIC LINK LOOP:
  HEAD  exit=2  error: could not read the pack's principles.toml: Too many levels of symbolic links (os error 40)
  PRE   exit=2  error: Too many levels of symbolic links (os error 40)
  MAIN  exit=2  error: Too many levels of symbolic links (os error 40)

CONTROLS, --template names an empty directory, and a path that does not exist:
  HEAD / PRE / MAIN, both cases  exit=2  error: No such file or directory (os error 2)
```

In every case the output directory stays empty at all three binaries. The mechanism is as the reviewer states: `resolved_within` canonicalises the root first (`src/safe_path.rs:104`), a non-directory root fails with `ENOTDIR` and a loop with `ELOOP`, neither is `NotFound`, so `read_optional` correctly does not fold it and `pack_principles` correctly reports it. The defect is only which file the message names. I found the second shape (the loop) myself and it matters for the remedy: this is not one odd `ENOTDIR` case, it is the whole class of root failures whose kind is not `NotFound`.

The reviewer's severity argument holds and I confirm it. Both the old and the new behaviour exit 2 and write nothing, so no outcome changes. `low`.

Why it is nonetheless real, and I put it more strongly than the reviewer did: the message sends a user to a file that in the plain-file case CANNOT EXIST, because their `--template` is not a directory at all. The project set its own standard for this class in the same commit (`src/main.rs:229-231` and `:2135-2137`, both arguing that naming the wrong step is the defect), and this fails it one level up.

### `X1` (low, INTERNAL): `safe_path`'s module doc names the wrong joiners

CONFIRMED on my own greps.

```
grep -n "join(\|read_to_string\|fs::" src/plan/source.rs   ->  NO OUTPUT (exit 1)
```

`plan::source` contains no join, no read and no filesystem access of any kind. The joins are `src/plan/render.rs:167` and `:169` (`load(&base.join(reference))`). The second clause fails the same way: `src/manifest.rs:743` is `is_contained_relative(&spec.dest)`, a check on the string, and the `dest` joins are `src/main.rs:88` and `:120`. Only the third clause holds (`src/manifest.rs:493-509` really does join and read `source` and `guidance`).

The reviewer's argument for why it matters is sound and I verified its supporting facts. `src/safe_path.rs:31-34` says the plan-side boundary "has only the lexical rule available, and is lexical for that reason rather than by preference", which is TRUE precisely because `plan::source` never touches disk. Line 5 says it already joins and reads. The module doc contradicts itself about the one fact that decides whether `resolved_within` may be applied there.

ONE THING THE REVIEWER DID NOT MEASURE, and it sharpens the finding rather than softening it. The round 2 fix pass REWROTE this exact sentence. `git diff fda1412..c45c501 -- src/safe_path.rs` shows the whole clause deleted and re-flowed, and the new paragraph "ONE CALLER USES THE LEXICAL RULE WITHOUT JOINING ANYTHING" added directly beneath it. That new paragraph asserts the findings ref is the ONE caller that does not join, which implies the sidecar refs in `plan::source` DO join. So the pass did not merely inherit the falsehood, it re-authored it and then added a sentence that depends on it.

### `X2` (low, INTERNAL): the test comment's read order

CONFIRMED on my own fixture. A pack whose `pack.toml` AND `principles.toml` are both symbolic links out, run at HEAD:

```
error: could not read the pack's principles.toml: `principles.toml` is not a contained pack path ...
exit=2
  names principles.toml: 1 occurrence     names pack.toml: 0 occurrences
```

`pack_principles` (`src/main.rs:2133`) runs before `build_assets` (`src/main.rs:2197`), which is the only path to `manifest()`. The comment at `tests/pack_source_stays_inside_the_pack.rs:384-386` says the `pack.toml` read "is the first read a run makes, so on a linked pack it is the first thing a user sees". Both halves are false, and the test's own fixture (`:387-396`) ships no `principles.toml`, so the body never exercises the claim.

The reviewer's distinction from `CHANGELOG.md:36` is correct and I checked it: the CHANGELOG says only that A refusal comes first, which is true. The comment says THE `pack.toml` refusal is what you hit, which is not.

### `X3` (low, INTERNAL): what nothing held before

CONFIRMED, and I ran both halves on the PRE tree myself rather than taking the reviewer's figures.

```
PRE baseline:                                          461 passed, 0 failed
PRE + an absent principles.toml becomes loud:          FAILED, 11 tests died, including
                                                       tests::a_pack_without_principles_has_an_empty_set
PRE + an absent instrument.md becomes loud:            461 passed, 0 failed. NOTHING died.
```

`a_pack_without_principles_has_an_empty_set` is at `src/main.rs:2406` at HEAD and `src/main.rs:2355` at `main`, and `git log -S` attributes it to `991a1d4` (2026-07-10), the commit that made principles pack-owned. Its body is the contract exactly: a directory pack with no `principles.toml`, `pack_principles` on it, `assert!(principles.is_empty())`.

So the comment at `src/manifest.rs:1319-1323` is FALSE for `principles.toml` and TRUE for `instrument.md`. My numbers match the reviewer's on both halves.

I record one thing the reviewer noticed and understated. The false sentence is a near-verbatim carry of round 2's own triage remedy text at `v002-r2-triage.md:363`: "THIS IS THE IMPORTANT ADDITION and nothing currently pins it at any level". The implementer copied an assertion the triage made and the triage had not run. That is a loop mechanism, not an implementer error, and I return to it under the re-seeding measurement.

### `X4` (low, INTERNAL): the "explicit arm" claim

CONFIRMED, and my measurement differs from the reviewer's in one direction and extends it in another. Both differences matter to the remedy, so I give them in full.

PROBE 1, the reviewer's own: replace `.map_err(PrinciplesError::Read)?` with `.unwrap_or_default()` in `pack_principles`, changing nothing else.

```
cargo build --release                            exit 0, one dead-code WARNING
probe against a pack whose principles.toml is a link out, --write:
  PROBE  exit=0  AGENTS.md = "P:|"      (the refusal swallowed, a degraded file written)
  HEAD   exit=2  error: could not read the pack's principles.toml: ...
```

So it compiles and it swallows, as reported. BUT it does NOT pass this project's own gate:

```
cargo clippy --all-targets -- -D warnings        exit 101
error: variant `Read` is never constructed  ...  = note: `-D dead-code` implied by `-D warnings`
```

Removing the only construction site of `PrinciplesError::Read` makes the variant dead, and the gate rejects it. The reviewer did not run clippy on its probe and so did not find this. On the reviewer's example alone, the doc's claim would be defensible.

PROBE 2, which is the case the doc's own stated purpose names ("a future reviewer ... not checking a NEW caller of `read_optional`"). I added a genuinely new optional literal to `build_assets`, one expression, no explicit arm:

```rust
let banner = source.read_optional("banner.md").unwrap_or_default().unwrap_or_default();
```

```
cargo clippy --all-targets -- -D warnings        exit 0, CLEAN
probe2 against a pack whose banner.md is a link out, --instrument --write:
  exit=0   AGENTS.md = "I:|"    the refused file silently dropped
```

A new swallowing caller passes every gate the project has. And the second half of the claim fails too: `grep -rn unwrap_or_default src/` returns six hits across five files, one of which is the doc sentence itself and four of which are unrelated legitimate uses. The only precise grep is `read_optional`, which is the thing the split already gave you.

So the claim fails on both clauses, for the case it exists to guard.

### `X5` (low, USER-FACING): the CHANGELOG bullet describes a state no released version had

CONFIRMED, both halves, on my own three binaries. This is ruled on in full below under "The two rulings", because it is the one that ships.

### `X6` (low, USER-FACING): "the same rule" as the metrics boundary

CONFIRMED on my own runs against this repository's own plan:

```
validate --source docs/plans/agent-scaffold.plan.toml --metrics <ABSOLUTE path under the root> --workflow
  ->  workflow invariants hold      exit=0
validate --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/../metrics/workflow.jsonl --workflow
  ->  workflow invariants hold      exit=0
```

Both shapes accepted. The pack rule refuses both outright, before asking where they land, and I measured that too: an absolute `source` gives "(it is an absolute path)" at exit 2, and `../secret.md` gives "(it carries a `..` component)" at exit 2.

`README.md:242` describes the metrics boundary as RESOLVED containment only ("resolving both through their real on-disk locations so a symlink cannot disguise one as the other"). The pack rule is that rule PLUS a lexical rule. "The same trade" is true. "The same rule" is false, and the mis-inference cuts in both directions, as the reviewer says.

The reviewer's mitigation is correct and I confirm it: the sentence immediately before states the pack rule correctly and in full, so a reader has the right rule in the same paragraph.

### `X7` (low, INTERNAL): `PrinciplesError::Read`'s "the file is present"

CONFIRMED, and I reproduced it WITHOUT `chmod`. The symlink-loop `--template` above fires `PrinciplesError::Read` for a pack directory that does not resolve at all, so no `principles.toml` exists anywhere:

```
--template <a symbolic link to itself>
  HEAD  exit=2  error: could not read the pack's principles.toml: Too many levels of symbolic links (os error 40)
```

The file is not present. The unreadable thing is the root. The message names a file that does not exist.

RELATIONSHIP TO `C1`, which I rule on explicitly because the two reviewers were independent and neither could see it. `X7` and `C1` are NOT duplicates: they name two different artefacts (a doc sentence and an error message) found by two reviewers on two different reproductions, and each is independently valid. But they share ONE root cause and admit ONE remedy. If the `--template` root is checked where the `PackSource::Directory` is constructed, then for any root that canonicalises, every non-`NotFound` failure of `root.join("principles.toml")` does involve a `principles.toml` that exists, and the doc sentence becomes true without being edited. I say so in the remedies so the fix pass does not do the work twice or, worse, weaken the doc sentence and leave the message wrong.

### `Z1` (low, USER-FACING): a third `Fixed` bullet with the same defect, raised by me

The orchestrator asked whether any other bullet in the 0.0.2 section has `X5`'s defect. One does, and no round 3 reviewer raised it. The text reviewer's `C22` ran each `Fixed` bullet's described prior behaviour against `MAIN` and reported only the second bullet, so this is a gap in round 3's coverage rather than a disagreement with it.

QUOTED, `CHANGELOG.md:38`:

> Every containment refusal now states the RULE and names the cause, instead of asserting something about the input that can be false of it ... `UnsafeAssetSource`, `UnsafeModuleGuidance`, `UnsafeAssetDest` and the internal `ReadError::Escapes` all opened with "leaves the ... directory" ...

MEASURED. None of those four types exists at the last release:

```
occurrences in src/manifest.rs:            main   HEAD
  UnsafeAssetSource                          0      9
  UnsafeModuleGuidance                       0      8
  UnsafeAssetDest                            0      9
  ReadError                                  0     24
```

And the wording it says was fixed lived entirely inside this unreleased cycle:

```
"leaves the ... directory" in src/manifest.rs, per commit:
  9364293 (main)  0     d639a4b  1     35028fd  1     b58f770  1
  4639d93  2      1da790d  4      fda1412  0     c45c501  0
```

The phrase first appears at `d639a4b`, the increment's own first commit, peaks at `1da790d`, and is gone by `fda1412`, the round 1 fix pass. A 0.0.1 user never saw any of those four messages, because 0.0.1 performs no containment at all.

So this is a `Fixed` entry in a Keep a Changelog 0.0.2 section for something that never shipped broken. Same class as `X5`. `low`, and one degree milder than `X5` for a reason I want stated: it makes no false claim about 0.0.1 BEHAVIOUR that a user could act on, because a 0.0.1 user encounters none of these messages. `X5` states the inverse of what 0.0.1 does.

FOR COMPLETENESS, the other five bullets in the 0.0.2 section, each run against `MAIN`:

| bullet | prior-behaviour claim | measured at `MAIN` |
| --- | --- | --- |
| `CL:36` pack path read containment | `..`, absolute and linked `source` and `guidance` all leak | TRUE. `..` source leaks `TOP SECRET`, absolute leaks, linked leaks, and a linked `guidance` splices `SECRET GUIDANCE` into `AGENTS.md` at exit 0 with no plan line. Genuine 0.0.1-relative fix |
| `CL:37` silent drop | see `X5` | FALSE |
| `CL:38` refusal wording | see `Z1` | FALSE (describes unreleased churn) |
| `CL:39` `dest` write escape | writes outside `--output-dir` at exit 0 while reporting "Wrote to" | TRUE. The file lands at the escaped path, exit 0. Genuine |
| `CL:40` `render` free-text structure | a two-line `ask` fabricates `Q-42` | TRUE. `MAIN` reports `Open Questions item `Q-42` has an unknown status `undecided``; HEAD reports only the pre-existing `superseded by` problem. Genuine |
| `CL:41` W5 waiver ownership | round-log ownership rule | `src/workflow.rs` is not in `main..HEAD` and `main` already carries `round_increment_id`, so this landed before this increment and is genuinely unreleased since 0.0.1. Genuine |

Two bullets of seven describe unreleased churn. Five are sound.

## The two rulings

### Ruling 1: is `low` right for `X4`

YES, `low` is right, and the question the severity answers is not the question the orchestrator is actually asking. I will answer both.

ON SEVERITY. This project rates absolute impact if left unfixed. `X4` is a doc comment on a function whose code is correct, in a binary-only crate with no rustdoc, and no exit code, no output byte and no gate turns on it. Rounds 1 and 2 put every comparable doc-claim finding at `low` (`claims 2`, `claims 3`, `claims 5`), and round 2 rated `B1`'s doc half `high` only because its CODE half was `high`. Here there is no code half: `read_optional` is correct, its `NotFound`-only fold is exactly right, and I confirmed the caller-level pin myself. Raising `X4` to `medium` would require the sentence to be able to produce a wrong outcome, and a comment cannot. `low` holds.

ON WHAT THE ORCHESTRATOR IS ASKING, which is whether a self-contradiction inside one paragraph, in the doc whose purpose is to stop `B1` recurring, deserves more weight than its rating suggests. It does, and severity is the wrong axis for it. Three properties make this the cheapest and most certain fix of the eight, and none of them is severity:

- IT NEEDS NO MEASUREMENT TO FIX. The sentence is refuted by the sentence two lines above it. A fix pass cannot get this wrong by mis-measuring, because there is nothing to measure.
- THE FIX IS A DELETION. Removing "and that a caller who still discards a refusal must write an explicit arm to do it, which is visible in review and findable with one grep" leaves a paragraph that is entirely true and that keeps the Principle 5 argument intact.
- IT IS THE ONE DOC IN THE TREE THAT EXISTS SPECIFICALLY TO PREVENT `B1`. `B1` was a `high` finding. A guard doc that overstates what the guard buys is the exact way the next author concludes the guard is stronger than it is. My probe 2 shows precisely that path: a new caller, one expression, clippy clean, refusal silently dropped.

So: `low` on severity, first on the fix order. I say both plainly rather than inflating the rating to carry the priority, because inflating it would corrupt the round record.

### Ruling 2: what the CHANGELOG bullet must say to be true of 0.0.1

MEASURED FIRST. A pack in the shape `CHANGELOG.md:37` names, produced with symbolic links out of the pack directory (the GNU stow shape), run with `--instrument --write`:

```
                          exit   AGENTS.md
MAIN (0.0.1)                0    P:1. My rule - One sentence.      I:INSTRUMENT FRAGMENT
PRE  (fda1412)              2    (nothing written; the pack.toml refusal)
HEAD (c45c501)              2    (nothing written; the principles.toml refusal)
```

And the bullet's own single-file example, only `instrument.md` linked out:

```
MAIN  exit=0  AGENTS.md = "P:|I:INSTRUMENT FRAGMENT|"     the OUTSIDE file read and inlined
PRE   exit=0  AGENTS.md = "P:|I:|"                        the silent drop the bullet describes
HEAD  exit=2  error: could not read the pack's `instrument.md`: ...
CONTROL, a pack shipping NO instrument.md at all:
MAIN  exit=0  AGENTS.md = "P:|I:|"
```

So at the only released version the linked fragment is READ AND INLINED, and the result is NOT byte-identical to a pack shipping no fragment, which is the bullet's own falsifiable claim. The described behaviour belongs to `fda1412`, an unreleased intermediate commit that is this change's own parent.

Second falsification, measured with invalid UTF-8 rather than a permission bit:

```
principles.toml containing invalid UTF-8, inside the pack, a real file:
MAIN  exit=0  create AGENTS.md, empty principles block
PRE   exit=0  create AGENTS.md, empty principles block
HEAD  exit=2  error: could not read the pack's principles.toml: stream did not contain valid UTF-8
```

An unreadable file was reachable and silent long before the containment rule existed. "the only outcome that had ever been reachable was 'the pack ships no such file'" is false, and the bullet's own last sentence says so.

WHAT THE BULLET MUST SAY. For a 0.0.1 user, exactly two facts in this bullet are true and are not already carried by `CL:36`:

1. A pack's `principles.toml` or `instrument.md` that the tool CANNOT READ (invalid UTF-8, or a file it has no permission to read) produced an empty principle set or an empty instrumentation block at exit 0 with empty stderr, indistinguishable from a pack that ships neither. It now exits 2 naming the file, which matches a malformed `principles.toml`, already loud at 0.0.1 (I measured the parse message identical at all three binaries).
2. ABSENCE IS UNCHANGED and stays silent. A pack shipping neither file still yields `P:\nI:\n` at exit 0, byte-identical at all three binaries.

Everything else in the bullet must go, and specifically these must go:

- "the only outcome that had ever been reachable was 'the pack ships no such file'". False, and self-contradicted eight lines later.
- "So a pack whose `instrument.md` was a link out of the pack scaffolded an `AGENTS.md` with the whole instrumentation contract missing, at exit 0, with empty stderr and an ordinary '30 changed' report, byte-identical to what a pack shipping no fragment at all produces". At 0.0.1 that pack inlines the outside file.
- "and a pack whose `principles.toml` was a link out generated an empty principles block at exit 0". At 0.0.1 that pack renders the outside file's principle.
- "with a linked-and-malformed file losing even the parse error it used to report". True of `PRE` only; at `MAIN` the parse error is reported.

A HOLE THE FIX MUST NOT LEAVE, and this is the part I most want the fix pass to read. `CL:36` scopes its leak enumeration to "TWO pack-controlled fields", `source` and `guidance`. The three literals are not fields, so they are correctly outside that sentence's scope, and `CL:37` is the only bullet that discusses them. Once `CL:37`'s false narrative is deleted, the 0.0.2 release notes will contain NO statement that 0.0.1 followed a symbolic link out of the pack for `principles.toml`, `instrument.md` and `pack.toml` and inlined what it found. That is the same leak class as `A1`, on three more paths, and it is currently disclosed nowhere and actively contradicted where it is discussed. The rewrite must ADD that disclosure, either by widening `CL:36`'s enumeration past "two fields" or by stating it in the rewritten `CL:37`. A rewrite that only deletes makes the release notes less complete than they are now, on a security-relevant fact.

DOES ANY OTHER BULLET HAVE THE SAME DEFECT: yes, `CL:38`. See `Z1` above for the measurement. Two of seven.

## The re-seeding measurement

The round 2 fix pass is `c45c501`. I attributed every finding site with `git log -S`, `git log -L` and `git diff fda1412..c45c501`, and every behaviour with the PRE and MAIN binaries.

| finding | site or behaviour | authored or introduced by | round 2 fix pass caused it? |
| --- | --- | --- | --- |
| `C1` | the message naming `principles.toml` for a root failure | `c45c501` (PRE prints the bare cause; measured) | INTRODUCED |
| `X1` | `src/safe_path.rs:4-7` | `d639a4b`, false there too; REWRITTEN by `c45c501` | RE-AUTHORED |
| `X2` | `tests/...:384-386` comment | `c45c501` | INTRODUCED |
| `X3` | `src/manifest.rs:1319-1323` comment | `c45c501` | INTRODUCED |
| `X4` | `src/manifest.rs:524-528` doc | `c45c501` | INTRODUCED |
| `X5` | `CHANGELOG.md:37` | `c45c501` | INTRODUCED |
| `X6` | `README.md:325` | `c45c501` | INTRODUCED |
| `X7` | `src/main.rs:229-231` doc | `c45c501` | INTRODUCED |
| `Z1` | `CHANGELOG.md:38` | `fda1412` (round 1 fix pass) | no, round 1's |

THE RATE, on round 2's own two definitions so the numbers compare.

- STRICT (text or behaviour the fix pass INTRODUCED): 7 of 8. Round 2 measured 3 of 6.
- BROAD (strict, plus findings whose subject the pass did not write but did make false): still 7 of 8, because `X1` was already false before the pass touched it. Round 2 measured 4 of 6.
- A THIRD FIGURE, which I give because round 2's definitions do not cover `X1`'s shape: on "text this pass wrote OR rewrote", it is 8 of 8. The pass deleted and re-flowed the whole `X1` sentence and added a paragraph beneath it that depends on the falsehood.

So 88 percent strict against round 2's 50 percent, and against the 2026-08-13 audit's project-wide 49 percent strict and 61 percent broad. The rate roughly doubled. Weighted by severity the picture is the reverse of round 2's: round 2's fix pass owned the round's only `high` and only `medium`, whereas this pass owns seven `low`s and no defect of any severity in the code it changed.

THE MECHANISM IS NOT THE ONE ROUND 2 IDENTIFIED, and the difference is the whole point of this section.

Round 2 named domain widening without re-checking consumers, with input-set widening as its twin. I checked whether that mechanism recurred, and on the strongest available test it did not. I enumerated `read_optional`'s consumers myself and found the same five production sites both reviewers and round 2 found, with no sixth. I ran the consumer-level mutation myself: reverting `pack_principles` to swallow at HEAD kills `a_linked_principles_file_is_reported_not_silently_dropped` and NOTHING ELSE, while `read_optional`'s own unit tests stay green. That is exactly the regression shape round 2 found, and it is now held at the call site independently of the primitive. `C1` is the one finding that is round 2's mechanism recurring, and it is one of eight.

THE MECHANISM THIS ROUND IS AN UNMEASURED COMPARATIVE CLAIM. Six of the eight findings (`X1` through `X7`, less `X7` which is `C1`'s doc half) are new prose of the identical grammatical shape: "X now, where before or elsewhere it was Y", with the Y half never run.

| finding | the comparison asserted | the state that was never run |
| --- | --- | --- |
| `X5` | what the previous release did | `MAIN` (the pass compared against its own parent commit) |
| `X3` | what the test suite held before | the PRE suite under the over-tightening |
| `X2` | which read a run makes first | a pack with both literals linked |
| `X6` | what a sibling boundary's rule is | `validate --workflow` with an absolute and a `..`-bearing `--metrics` |
| `X1` | which module joins and reads | `grep` for a join in `plan::source` |
| `X4` | what the type system forces a caller to write | a swallowing caller |

Every one of those is a single command. Not one of them was run. The pass verified what it BUILT (and it verified that well: both reviewers confirmed the behaviour, six new tests are non-vacuous, all seven gates are green) and ASSERTED what it replaced.

A SECOND-ORDER DRIVER, which I think is the most useful thing in this document. THREE of the six trace directly to round 2's own triage remedy text:

- `X3`'s false sentence is a near-verbatim carry of `v002-r2-triage.md:363`, "THIS IS THE IMPORTANT ADDITION and nothing currently pins it at any level". The triage asserted it without running the PRE suite; the implementer copied it into a comment; round 3 falsified it.
- `X5` and `X6` are the two artefacts round 2's `B2` remedy asked for by name, `CHANGELOG.md:32` rewritten and "one sentence in `README.md`'s pack-authoring section stating the pack containment rule, which currently exists only for the plan side at `README.md:242`". The pointer at `:242` is what produced "the same rule ... that the metrics and ledger boundary above takes".
- `X1` is the residue of round 2's `claims 3` remedy, which scoped the fix to the parenthetical ("Delete it, or state the real relation") and did not check the clause the parenthetical sat inside. The implementer did both things the remedy offered and left the sentence around them false.

So the review loop is now a source of its own next round's findings. A remedy stated as an assertion becomes a claim in the tree, and the next round falsifies the claim. Neither round 1 nor round 2 named this, and I am naming it against my own role: this document's remedies are written to avoid it, which is why each one below states what I MEASURED rather than what the fix pass should assert.

## Remedies

Scoped to the class, not the instance. Each names what must NOT be edited and why. Every test change is an ADDITION.

### `C1` and `X7` remedy, ONE change closes both

CLASS: a failure that belongs to a boundary's ROOT must not be reported against the first file inside it that a run happens to touch.

CHANGE: check once, where the `PackSource::Directory` is constructed (`src/main.rs:2111-2113`), that `--template` names a directory, and refuse with a message naming `--template` and the path. That is the only site that knows the failure concerns the root rather than a file in it. It covers the whole class, not the one `ENOTDIR` case: I measured `ENOTDIR` (a plain file) and `ELOOP` (a symlink loop) both landing on the `principles.toml` message, and it also gives the currently unlabelled empty-directory and missing-directory cases a message that names the flag (all three binaries print a bare `No such file or directory` today).

WITH THAT CHANGE, `X7` NEEDS NO EDIT. For any root that canonicalises, every non-`NotFound` failure of `root.join("principles.toml")` involves a `principles.toml` that exists, so `src/main.rs:229-231` becomes true as written. If `C1` is deferred, then `X7` must instead be weakened to what `Read` actually means, that the read did not produce text.

MUST NOT BE EDITED:

- `read_optional`'s `NotFound`-only fold (`src/manifest.rs:538`). Widening it to swallow `NotADirectory` or `ELOOP` reopens `B1`'s class on a new error kind. I verified the pin myself: at HEAD, reverting `pack_principles` to swallow kills `a_linked_principles_file_is_reported_not_silently_dropped` and nothing else, so the call-site behaviour is held independently of the primitive.
- `PrinciplesError`'s `Read`/`Parse` split (`src/main.rs:228-235`) and the comment at `src/main.rs:2135-2137`. They are correct and they are the standard `C1` is judged against.
- `a_missing_pack_file_still_reports_as_missing_not_as_an_escape` and `read_optional_answers_absence_with_none_and_never_with_an_error`. They pin absence staying silent, which this remedy must not touch.

TESTS: ADD one integration case, `--template` naming a plain file exits 2 with a message naming `--template` and the path and NOT naming `principles.toml`. Nothing pins that message today (it is new behaviour introduced by `c45c501` and no test asserts it), so nothing is replaced. Afterwards that case pins it.

### `X1` remedy

CLASS: a module doc that names the wrong module as the joiner, in a file whose later paragraphs turn on which module can touch disk.

CHANGE: `src/safe_path.rs:4-7`. State the measured relation: the strings are CHECKED at their boundaries (`plan::source` for the sidecar refs, `manifest` for `source`, `guidance` and `dest`) and JOINED elsewhere (`src/plan/render.rs:167` and `:169` for the sidecar refs, `src/main.rs:88` and `:120` for a `dest`). Only `manifest` both joins and reads, at `src/manifest.rs:493-509`.

MUST NOT BE EDITED:

- `src/safe_path.rs:31-34`, "has only the lexical rule available, and is lexical for that reason rather than by preference". It is TRUE, and it is the sentence the correction must be made consistent WITH.
- The `ONE CALLER USES THE LEXICAL RULE WITHOUT JOINING ANYTHING` paragraph (`:12-19`). It is correct and it is round 2's `claims 3` remedy discharged. It must survive, and after the correction above it stops resting on a false premise.
- `src/plan/source.rs:474` and `:737-738`. They already state the relation correctly and are what falsify line 5. Do not "fix" them to match the module doc.

No fixture is at risk. Nothing pins doc text, which is why every round has produced findings of this class.

### `X2` remedy

CLASS: a test comment asserting a run-order property the test's own fixture does not exercise.

CHANGE: `tests/pack_source_stays_inside_the_pack.rs:384-386`. KEEP the first sentence, which is true and which justifies the test. Replace or delete the second. If it is replaced, the measured fact is that `pack_principles` (`src/main.rs:2133`) reads first and `manifest()` is reached only through `build_assets` (`src/main.rs:2197`), so the `pack.toml` refusal is what a user sees only when the pack ships no `principles.toml`, which is what this fixture does.

MUST NOT BE EDITED: the test body and its fixture. The body is the ONLY pin on the `pack.toml` refusal's wording (mutation E killed it, and there is no field label to hold it otherwise). If the ordering claim is wanted as a pin rather than as prose, ADD a second case with both literals linked out and assert the message names `principles.toml`. Do not change this fixture to carry both, or the `pack.toml` message loses its only pin.

### `X3` remedy

CLASS: a comment justifying a new test by an unrun claim about pre-existing coverage.

CHANGE: `src/manifest.rs:1319-1323`. Scope the claim to `instrument.md`, for which it is true (I measured: over-tightening its absence path at PRE kills nothing, 461 passed and 0 failed, identical to the PRE baseline). For `principles.toml` it is false: the same over-tightening kills 11 tests including `a_pack_without_principles_has_an_empty_set`. While the sentence is open, correct its third clause too: `src/manifest.rs:96-97` covers only `instrument.md`, whereas `README.md:327` and `:362` cover both, so cite the README rather than `ModuleSpec.guidance`'s doc. That is the text reviewer's `T3` observation, which it chose not to raise, and folding it in costs nothing once the sentence is being edited.

MUST NOT BE EDITED:

- The new test itself. It is non-vacuous and it holds the primitive level.
- `a_pack_without_principles_has_an_empty_set` (`src/main.rs:2406`). It is the pre-existing pin the comment denies, it is present at `main`, and it is now the evidence for the corrected sentence. It must not be deleted as redundant: it holds the unit level for `principles.toml` while the new test holds `read_optional` and the integration case holds the whole run. Three levels, three different failure modes.

### `X4` remedy

CLASS: a doc that claims a language-level guarantee the language does not give, two sentences after conceding it.

CHANGE: `src/manifest.rs:524-528`. Delete "and that a caller who still discards a refusal must write an explicit arm to do it, which is visible in review and findable with one grep". KEEP what is true and measured: the wrong primitive no longer yields a plausible-looking empty `String` but a `None` the caller must convert, and the correct optional-read primitive exists and is the obvious one to reach for. That is the Principle 5 gain and it is real.

MY MEASUREMENT CHANGES WHAT THE REPLACEMENT MAY SAY. Do not replace the deleted clause with a claim that the gates catch a swallow. They do not: a NEW swallowing caller of `read_optional` passes `cargo clippy --all-targets -- -D warnings` at exit 0 and silently drops a refused file at exit 0. The reviewer's own probe fails clippy, but only incidentally, because it renders `PrinciplesError::Read` dead. The honest replacement says the invariant is held by review, not by the compiler.

MUST NOT BE EDITED: `read_optional` itself, its `NotFound`-only fold, the two call sites, and the three tests that pin them.

### `X5` remedy

CLASS: a release note whose account of the previous release describes the working tree's parent instead.

CHANGE: `CHANGELOG.md:37`, reduced to the two facts ruling 2 lists, phrased against 0.0.1. Delete the four false claims ruling 2 names.

MUST NOT BE LOST, and this is the whole risk in this remedy:

- The disclosure that 0.0.1 FOLLOWED a link out of the pack for `principles.toml`, `instrument.md` and `pack.toml` and inlined what it found. It is currently stated nowhere in the 0.0.2 section and contradicted where those files are discussed. `CL:36`'s enumeration is scoped to "TWO pack-controlled fields" and correctly excludes the literals, so this disclosure must be ADDED, either by widening that scope or in the rewritten `CL:37`.
- The bullet's last two sentences. Both are genuinely 0.0.1-relative and I measured both: absence stays silent (`P:\nI:\n`, byte-identical at all three binaries) and an unreadable file becomes loud where it was silent (invalid UTF-8: `MAIN` and `PRE` exit 0 silently, `HEAD` exits 2 naming the file).

MUST NOT BE EDITED: `CL:36`'s two surviving-shape clauses and its recourse paragraph. Round 2 protected them, the text reviewer re-measured them with real GNU stow, and I re-measured the recourse myself (`cp -rL` then `--template` gives exit 0 with the correct principles and instrument content).

### `X6` remedy

CLASS: a cross-reference asserting two boundaries share a rule when they share only a trade.

CHANGE: `README.md:325`. Keep "the same trade". Drop or qualify "the same rule". The measured relation is that the pack rule is the metrics rule PLUS a lexical rule the metrics boundary does not apply.

MUST NOT BE EDITED: the sentence immediately before it, which states the pack rule correctly and in full, and `README.md:242`, which states the metrics rule correctly in its own section. Neither is wrong. The join between them is.

### `Z1` remedy

CLASS: a `Fixed` entry for a defect that never reached a release.

CHANGE: `CHANGELOG.md:38`. Its CONTENT is worth keeping, because it tells a reader what the new refusal messages say, but not under `Fixed` and not as a correction of prior wording no user saw. Either fold it into `CL:36` and `CL:39` as a description of the message form those two bullets introduce, or restate it as a plain description without the "all opened with 'leaves the ... directory'" history.

MUST NOT BE EDITED: its final sentence, "The `dest` message states only the string half of the rule, because the write side applies only that half". That is TRUE, the text reviewer verified it by writing a file outside the output directory through a linked subdirectory at exit 0, and it is the only place the write side's weaker rule is disclosed to a user.

## Round outcome

ROUND OUTCOME: `new_valid`.

- Valid findings: 8 from the two reviewers, plus 1 raised by this triage (`Z1`). Nine in total.
- Severity list: `low` x9. No `critical`, no `high`, no `medium`.
- None invalid, none duplicate, none out of scope.
- No severity lowered and none raised, so this triage creates NO re-check obligation.
- User-facing 4 (`C1`, `X5`, `X6`, `Z1`), internal 5 (`X1`, `X2`, `X3`, `X4`, `X7`).

THIS ROUND DOES NOT CONVERGE THE INCREMENT. `ship-v0-0-2-inc1` is `low_risk` by `Q-74`, so ONE clean round converges it. This is the third consecutive `new_valid` round, `consecutive_clean` stays 0, and three of five rounds are used.

THE SHAPE, stated because the decision below turns on it. Round 1: 5 valid, ceiling `high`. Round 2: 6 valid, ceiling `high`. Round 3: 9 valid, ceiling `low`. The count rose and the ceiling fell, and the two moved together for a reason my measurements show directly: the code question closed and the prose question did not. The behaviour half of this change is now sound and pinned. Every finding this round is a sentence.

## Merge and publish

THE CHANGE IS NOT SAFE TO MERGE AND PUBLISH AS IT STANDS. One reason, and it is narrow.

`CHANGELOG.md:37` tells a 0.0.1 user that their current version silently drops a linked `instrument.md`. Their current version reads the outside file and inlines it into `AGENTS.md`, which I measured at `MAIN`. That is the inverse of the truth on a path that is the same leak class as `A1`, and because that bullet is the only one that discusses the three literals, deleting it without replacement would leave the release notes with no statement that 0.0.1 leaked through them at all. Release notes bound for crates.io that describe the previous release's security-relevant behaviour backwards are not publishable, and the fact that the falsity is `low` in impact does not make the artefact fit to ship.

THE MINIMUM SET THAT MAKES IT SAFE:

1. `X5`. Rewrite `CHANGELOG.md:37` to ruling 2, INCLUDING the leak disclosure the rewrite must not drop.
2. `Z1`. Reframe or fold `CHANGELOG.md:38`, which documents unreleased churn as a user-facing fix.
3. `X6`. Delete or qualify four words in `README.md:325`. The README renders on the crates.io page and the false half is a rule a pack author could act on.

That is three edits in two files, all of them user-facing text, none of them touching a line of code or a test.

NOT IN THE MINIMUM SET, and I say so explicitly so the set is not padded:

- `C1` is a message-quality wart. Both the old and the new behaviour exit 2 and write nothing. It is worth fixing and it discharges `X7` for free, but nothing about it makes the release unsafe.
- `X1`, `X2`, `X3`, `X4`, `X7` are internal. No person who installs the crate encounters any of them, on the measured grounds in the boundary argument above.

WHAT IS ALREADY SAFE, and the record should carry it. The code half of this increment is done. `B1` is closed and held at the call sites independently of the primitive, which I verified with my own mutation. Absence stays silent, pinned at three levels. All seven gates are green at HEAD. `validate --plan` reports exactly the one pre-existing problem criterion 4 names. Every shape either reviewer or I could build behaves as the change says it does, and the two defects round 3 existed to check are both closed. There is no code change in the minimum set because no code defect was found.

## How this increment ends

Not my decision. Here is what each option costs and buys, measured where I could measure it, and my recommendation with its reasoning against the plan's Project Principles by name.

FIRST, THE HONEST FORECAST, because every option turns on it. WILL A ROUND 4 PRODUCE ANOTHER CROP OF `low` TEXT FINDINGS? On this evidence, yes, unless the fix pass is constrained in kind rather than told to be careful. Three grounds:

- The measured re-seeding rate on prose is 7 of 8 strict this round, up from 3 of 6. A fix pass that rewrites nine prose sites is the highest-yield input a reviewer has had yet.
- The mechanism producing the findings is asserting a comparison without running it, and it is not a carelessness that instruction fixes. Three of six instances came from round 2's triage remedy text being carried across, meaning the loop generated them, not the implementer.
- Nothing in the repository pins doc text. Round 1, round 2 and round 3 have each said so. Every prose claim is held by review and by nothing else, so each rewrite is a fresh unpinned surface.

There is a measured precedent in this project's own metrics for what shortening does. On `validation-constraints-inc1`, round 1 gave 5 valid, a fix pass that "reworked every refusal message ... and the shipped rule text" gave 9 valid in round 2, the human chose "Fix by shortening, then one more round" (`Q-70-inc1close`, 2026-08-12), and the round after the shortening pass gave 4 valid. Shortening roughly halved the crop. IT DID NOT PRODUCE A CLEAN ROUND. Any option that requires a clean round should be priced on that.

### Option A: fix all nine, then round 4

COSTS: one fix pass over nine sites plus a full fourth round. It uses the fourth of five rounds, leaving one. On the forecast above a fourth round is more likely than not to be `new_valid` again, which would put the increment at the cap with escalation the only remaining move. The pass rewrites the two CHANGELOG bullets, the README sentence, the module doc, two test comments, a `pub fn` doc and a variant doc, which is the largest prose surface any pass in this increment has touched.

BUYS: it is the only option that can converge the increment by its own rule (`Q-74`, one clean round), and it is the only one that leaves no recorded residual behind.

### Option B: fix the user-facing subset, accept the internal five as recorded residuals

COSTS: five false statements remain in the tree, two of them (`X1`, `X4`) in the maintainer-facing docs from which this increment's own `B1` and `claims 3` came. Establishing an accepted-residual rule is a workflow change and needs its own human decision, which is work that is not delivery. It does not converge the increment unless a round follows.

BUYS: the smallest possible fix surface, three edits in two files, of which two are deletions and one is a rewrite against a measurement I have already made. That is the lowest re-seed exposure of any option. It makes the artefact publishable, and it ends the drought this step exists to end.

### Option C: fix all nine and merge without a further round

COSTS: nine unreviewed prose edits ship. At a 7-of-8 measured prose re-seeding rate the expected number of new false statements in that pass is not small, and by construction no round would find them, so they ship to crates.io. It also sets aside `Q-74`'s convergence rule without a waiver, which the workflow validator will see.

BUYS: everything gets fixed and the release goes out immediately.

### Option D: escalate now

COSTS: two rounds of the cap go unused, and the escalation is raised on a round whose ceiling is `low`, whose code half is sound and whose only unsafe artefact is three text edits away from safe. Escalation is the move for a loop that cannot resolve a question, and this loop has resolved its question.

BUYS: a human decision about the loop mechanism rather than about this increment, which the falling-ceiling and rising-count shape does legitimately raise. The re-seeding section above is the material such a decision would need, and it exists now whether or not the escalation happens.

### My recommendation

FIX THE FOUR USER-FACING FINDINGS BY SHORTENING, TAKE THE FIVE INTERNAL ONES AS DELETIONS IN THE SAME PASS, AND RUN ONE ROUND SCOPED TO THE EDITED TEXT ONLY. That is between option A and option B, and I recommend it over both in their pure forms.

Concretely: `C1` is one root check with one added test. `X5` and `Z1` and `X6` are rewrites against measurements already in this document. `X1`, `X2`, `X3`, `X4` and `X7` are all closed by DELETING a clause, with no replacement prose authored for any of them, except `X3` where one sentence is scoped down. `X7` needs no edit at all if `C1` lands. That is one code change, three rewrites, and five deletions.

THE REASONING, against the plan's Project Principles by name.

PRINCIPLE 2 (Minimal by default) is the one that decides it, and it applies to the prose as much as to the core. The round 2 fix pass wrote 354 lines to change three lines of behaviour. Six of this round's findings are in prose that did not need to exist: a test comment can name what the test pins without asserting what the suite held two commits ago, a `pub fn` doc can state what the type buys without claiming what the compiler enforces, and a `Fixed` entry can state the two facts a 0.0.1 user needs without narrating an intra-release regression. Deleting a comparative clause cannot be falsified by a fourth round. Rewriting it can. That asymmetry, plus the precedent's 9 to 4 fall after a shortening pass, is why I recommend shortening as the METHOD rather than correction, and it is also why I do not recommend option B in its pure form: the internal five cost almost nothing to close as deletions, so accepting them as residuals buys less than it appears to and leaves five false statements in the two docs this increment's own defects came from.

PRINCIPLE 1 (Prefer the cleaner long-term architecture over the smallest diff) argues against option B's pure form for a second reason. `X1` and `X4` are the module map and the guard doc that a later change to this boundary navigates by. `X1` tells a maintainer that `plan::source` already joins and reads, which is the premise under which applying `resolved_within` there looks safe, and `src/plan/source.rs:245-247` records why that would break a valid historical findings pointer. Leaving that as an accepted residual leaves a booby trap in the map, and the deletion that removes it is three lines.

PRINCIPLE 6 (Ground decisions in evidence) argues against option C. Publishing nine unreviewed prose edits at a measured 7-of-8 re-seed rate is forcing an unvalidated approach through, which is the shape Principle 6 exists to refuse. One round after the pass is what grounds it, and it is affordable: four of five rounds used, with the fifth in reserve.

PRINCIPLE 8 (Structured data first, project for humans) is the one that says the CHANGELOG must be fixed rather than deferred. The release notes are a projection for humans of what the release changed, and a projection whose account of the prior state is the inverse of the measured prior state is not a projection. That is the same argument the spec makes for `F1` and it applies here without modification.

AND THE STEP'S OWN PURPOSE, which is not a principle but is written into the spec: this step exists to end a 34-day delivery drought, and "its purpose is delivery, so its scope is closed and every addition to it defeats it". Nine deletions and rewrites of text this increment itself wrote are not an addition to the scope. A fourth round after them is the smallest thing that satisfies `Q-74`. Option D spends the increment's remaining rounds on a decision about the loop, which the re-seeding measurement above lets a human take at any time, including after this converges.

WHAT WOULD CHANGE MY RECOMMENDATION. If the fix pass cannot be constrained to deletions and short factual replacements, option B's pure form becomes the better bet, because a correction pass over nine sites at this re-seeding rate is worse than five recorded residuals. And if the human's tolerance for a fourth `new_valid` round is nil, then the minimum publish set (`X5`, `Z1`, `X6`) plus merge is the only option that guarantees the release, at the cost of `Q-74`.
