# `ship-v0-0-2-inc1` round 5: REVIEWER, deletion loss

Independent reviewer. I did not write this change and did not review rounds 1 to 4. My lens is the complementary one to the truth question: did a deletion remove something that needed to survive, and does the 0.0.2 section still describe what the increment does. Every figure below is my own measurement, made in this session. Where I reach the same result as an earlier round I say so and give my own numbers.

## Artifact

- Worktree `.claude/worktrees/r5-b`, detached at `e3a466e` ("docs: delete the three claims round 4 measured false").
- Scope ruled on: `git diff HEAD~1..HEAD` (3 files, 4 insertions, 5 deletions), plus the whole 0.0.2 section against `git diff main..HEAD`.
- Read first: `v002-r4-triage.md`, then `v002-r3-triage.md`.
- Not reopened: the recorded residuals (the `--template` "must name a directory" message for a real directory under an unreadable parent, and the missing bullet for that message), `A2`, `A3`, `A5`, the audit's `F2` and `F3`, the `superseded by` projection defect, the rename, ANSI escapes in a `dest`, the plan-side sidecar symlink hole, the empty-directory gap, and the containment mechanism.

## Method

TWO release binaries, one `CARGO_TARGET_DIR` each, from two separate `git archive | tar -x` extracts, verified distinct by `md5sum`:

```
54505959c281ed50385a6d1c6c626fae  tgt-head/release/agent-scaffold   (e3a466e, HEAD)  0.0.2
ec5e1145d437161569373d129274a954  tgt-main/release/agent-scaffold   (7e8e26c, MAIN)  0.0.1
```

`--version` confirms 0.0.2 and 0.0.1. A third tree carried one source mutation (a NEW swallowing caller of `read_optional`) in its own target directory.

Every fixture, symbolic-link target, escape target and cargo target directory sits under my own scratch subdirectory. I used symbolic links and invalid UTF-8 rather than permission bits, so I SET NO `chmod` ANYWHERE and there is none to restore. `git status --short` is empty in this worktree and in the main repository, apart from this file.

GATES I RAN MYSELF AT HEAD:

| gate | result |
| --- | --- |
| `cargo test --no-fail-fast` | 468 passed, 0 failed, over 11 result lines (409+5+1+1+9+2+13+3+20+1+4) |
| `cargo clippy --all-targets -- -D warnings` | clean, exit 0 |

I did not re-run the plan-side gates, which round 4 ran; the commit under review touches no plan, no metrics record and no code path.

ONE MEASUREMENT NOTE THAT MATTERS FOR EVERY "0.0.1" CLAIM, mine included. `main` is NOT the 0.0.1 release tree. `v0.0.1` is `2bbce2e` (2026-07-10), and `git diff --stat v0.0.1..main -- src/ Cargo.toml pack/` reports 18589 insertions across 73 files. `main` carries `version = "0.0.1"` and is what rounds 3 and 4 measured as MAIN, so I used it too for continuity, and every claim I make about "0.0.1 behaviour" below is a claim about `main`'s tip. The published 0.0.1 binary is not what any round has run. I record this rather than act on it, because the orchestrator's standard for this round names `main..HEAD`, and because the read paths in question predate the tag on inspection.

## The three deletions: what each carried, and where that fact now lives

| deletion | site | what the deleted words carried | where that fact lives now | lost? |
| --- | --- | --- | --- | --- |
| "and its contents inlined" (5 words) | `CHANGELOG.md:37` | that the outside file's CONTENTS were placed into the scaffolded output, not merely read | carried by reference in the surviving clause "the same leak as the two fields above on three more paths", whose referent (`CHANGELOG.md:36`) spells out "copied the outside file into the scaffolded project" and "spliced the outside file into the scaffolded `{{modules}}` slot"; and by `README.md:327`, "like `principles.toml`, that fragment is read directly and inlined, not dropped as its own asset" | NO |
| "and a caller that does passes `cargo clippy --all-targets -- -D warnings`" | `src/manifest.rs:524-527` | that the project's own gate does not catch a swallowing caller | carried by the surviving "It does not make swallowing impossible, and nothing in Rust can: a caller may still write `.unwrap_or_default()` on this too" and "The invariant is held by review, not by the compiler" | NO |
| "The file is present and could not be read: ... or an unreadable file" | `src/main.rs:229-230` | (a) an existence assertion, measured FALSE in rounds 3 and 4; (b) the contrast with absence | (a) correctly gone; (b) carried by the enum-level doc two lines above, `src/main.rs:224-226`, "ABSENCE is not in here: a pack that ships no `principles.toml` has no principles to select" | NO |

The third deletion also WIDENS "an unreadable file" to "an unreadable path", which is coverage gained rather than lost: the unreadable thing can be the pack root directory, which is the shape that falsified the old sentence.

## Verdicts

ONE finding. It is not about the three deletions; all three are clean on my lens. It is on question 4, the section's completeness against `main..HEAD`, in the "something happened and is not described" direction.

| id | verdict | severity | class | site | one line |
| --- | --- | --- | --- | --- | --- |
| `L1` | valid | low | USER-FACING | the 0.0.2 section, `CHANGELOG.md:39` | The render fix makes an existing committed `<task>.md` stale, so `render --check` warns and `render --check --strict` exits 1 on an unchanged project until it is re-rendered. No sentence in the section says so, and the tool's own mismatch message names two causes that are both false of this case. |

NO `critical`, NO `high`, NO `medium`.

## `L1` (low, USER-FACING): the section does not say an existing plan view must be re-rendered

### What I measured

Same source, same committed view, same command, two binaries. The tree is a copy of `main`'s `docs/` (this project's own plan, unmodified) in my scratch directory:

```
render --check docs/plans/agent-scaffold.plan.toml
  MAIN  exit=0  "docs/plans/agent-scaffold.plan.toml: up to date"
  HEAD  exit=0  "warning: docs/plans/agent-scaffold.md differs from a fresh render
                 (a hand-edit, or a stale render after a source edit)
                 (first difference at line 145: ...)"

render --check --strict docs/plans/agent-scaffold.plan.toml
  MAIN  exit=0  "up to date"
  HEAD  exit=1  "error: docs/plans/agent-scaffold.md differs from a fresh render ..."
```

The mismatch is the render fix, not a hand-edit. A fresh HEAD render of that same unmodified source differs from the committed view at 112 lines, across 6 hunks, all of them multi-paragraph `[[question]].ask` items (`Q-52`, `Q-55`, `Q-58`, `Q-64`, `Q-65`, `Q-68`, `Q-69`, `Q-70`, `Q-76`) folding from a fragmented list into one queue line. I verified no text is lost: the `DESIGN PASS DONE (2026-07-24)` paragraph appears once in the 0.0.1-rendered file and once in the fresh HEAD render.

The increment itself had to do exactly what a user must do. `git diff --stat main..HEAD -- docs/plans/agent-scaffold.md` reports 151 changed lines, and `git log main..HEAD -- docs/plans/agent-scaffold.md` attributes it to `ac9f150`, "fix: keep every interpolated free-text value on one generated line". The re-render is part of the fix commit.

### Why no earlier round saw it

Every round ran `render --check --strict` inside the increment's own tree, where the re-render had already landed, so the gate reported "up to date" (`v002-r1-triage.md:43`, `v002-r2-reviewer-attack.md:42`, `v002-r3-reviewer-boundary.md:37`, and round 4's gate table). The effect appears only when the HEAD binary meets a `<task>.md` that a pre-fix binary rendered. I grepped all twelve round 1 to 4 documents: no finding, observation or residual concerns a stale committed view, and `re-render` appears nowhere in any of them.

### What the section says, and what it does not

`CHANGELOG.md:39` ends: "THE VISIBLE EFFECT on an existing plan is that a multi-paragraph `ask` now renders as one queue line instead of a fragmented list with prose loose between the items; no text is lost, and the queue's items are again exactly the `[[question]]` entries the source declares."

Both halves of that are true and I measured both. What the section never states, in any bullet:

```
occurrences in the 0.0.2 section (CHANGELOG.md:7-41):
  "render --check"   0
  "re-render"        0
  "stale"            0
  "regenerat"        0
```

So the sentence that exists to tell an existing user what changes for them names the cosmetic half of the effect and stops at the half that changes a command's exit status.

### Why it matters, and why I still rate it `low`

The population is every project following the scaffolded workflow whose committed `<task>.md` carries a multi-line free-text value at one of the four sites. That workflow is what the shipped pack instructs: `pack/AGENTS.md:30`, `pack/plan-template.documentation-protocol.md:3` and `pack/prompts/implementer.md:5` all tell the project to run `agent-scaffold render --check` before committing, and `README.md:207` documents `--strict` for CI. This project's own plan is in the population.

The sharp edge is that the tool's own message misdirects on exactly this case. It reports the mismatch as "a hand-edit, or a stale render after a source edit". Neither happened: nobody edited the file and nobody edited the source, only the renderer changed. So a user hitting a red gate is told two causes that are both false of them, and the release notes do not carry the third. That message is pre-existing (unchanged in `main..HEAD`); I am not raising it as a defect, only recording that it is why a reader is not one easy inference away from the answer.

`low`, and I say plainly what keeps it there. The failure is loud, writes nothing, destroys nothing, and is fixed by one `agent-scaffold render` plus a commit. Plain `render --check` still exits 0. And the same bullet names the reader's exact symptom, so a user who does re-read it will connect the two.

### The argument against, stated fairly

A triager could rule this derivable rather than missing: the bullet says the render output for an existing plan changes, `README.md:210` says `render --check` compares the committed file against a fresh render, and the consequence follows from those two premises. I weighed that and still raise it, on the standard round 4 applied to `P4`: the section's own threshold for naming an upgrade consequence sits below this, not above it. Three other bullets in the same section spell out a consequence that is equally derivable from the rule they state, and they spell it out anyway: `CHANGELOG.md:27`'s "THE POPULATION THIS BREAKS", `CHANGELOG.md:28`'s "TWO BREAKS TO KNOW ABOUT", and `CHANGELOG.md:36`'s "THE SHAPE THAT STOPS WORKING" together with "THE RECOURSE". A command that returned 0 and now returns 1 on an unchanged repository is the same kind of fact those three record.

### The remedy, and its shape

One clause on the existing sentence, using words already measured in this document, with no new comparative claim: an existing committed `<task>.md` needs one `render` after the upgrade, and `render --check --strict` fails until it gets one. Nothing else in the bullet changes, no behaviour changes, and no test changes.

## Question 1: the five deleted words in `CHANGELOG.md:37`

NOT A FINDING. The fact survives.

WHAT THE WORDS CARRIED, measured at MAIN on three fixtures, each a minimal pack with `AGENTS.md` holding `P:{{principles}}` and `I:{{instrument}}`, run with `--instrument --write --vcs none`:

```
principles.toml -> ../outside/principles.toml
  MAIN exit=0  AGENTS.md: "P:1. LEAKED PRINCIPLE - OUTSIDE-PRINCIPLE-SUMMARY"  the outside file's content
  HEAD exit=2  "could not read the pack's principles.toml: `principles.toml` is not a contained pack path ..."

instrument.md -> ../outside/instrument.md
  MAIN exit=0  AGENTS.md: "I:OUTSIDE-INSTRUMENT-FRAGMENT"                      the outside file's content
  HEAD exit=2  "could not read the pack's `instrument.md`: ... is not a contained pack path ..."

pack.toml -> ../outside-pack/pack.toml   (source = "../outside/secret.md", dest = "../elsewhere/planted.md")
  MAIN exit=0  stdout "create  ../elsewhere/planted.md" / "Wrote to <output-dir> (1 changed, 0 left untouched)."
               the outside manifest is OBEYED; a file lands OUTSIDE the output directory carrying
               "TOP-SECRET-OUTSIDE-THE-PACK"; NOTHING of the outside pack.toml's own text is inlined
  HEAD exit=2  "`pack.toml` is not a contained pack path ..."; nothing planted
```

So the deleted words stated a content leak that is real for two of the three files and false for the third, which is why round 4 ruled the deletion the minimum honest fix.

DOES THE SURVIVING TEXT STILL CARRY THE CONTENT-PLACEMENT FACT? Yes, by reference, and the reference is explicit. The surviving clause is "the same leak as the two fields above on three more paths". "The two fields above" is `CHANGELOG.md:36`, which describes their leak in these words: "copied the outside file into the scaffolded project" and "spliced the outside file into the scaffolded `{{modules}}` slot". A reader who follows the equation is told that content from outside the pack landed in the scaffolded output, and that is the forensic pointer a 0.0.1 user needs. The word "leak" is itself a content word, not a path word.

A SECOND CARRIER, on the crates.io page. `README.md:327` reads "`{{instrument}}` is filled from the pack's optional `instrument.md` render fragment when `--instrument` is set (empty otherwise); like `principles.toml`, that fragment is read directly and inlined, not dropped as its own asset." That is the mechanism, stated for both files, in the artefact `Cargo.toml` names as `readme`.

AGAINST ROUND 3'S PRECEDENT. `v002-r3-triage.md:349` and `:473` required that the rewrite must not leave the release notes with NO statement that 0.0.1 followed a link out of the pack for the three literals and inlined what it found. The hole round 3 named was a bullet deleted outright. The bullet is still there, it still names all three files, it still says each was read through a link out of the pack in 0.0.1, and it still equates that to the leak `CHANGELOG.md:36` describes as content landing in the scaffolded project. The precedent is satisfied.

WHAT A 0.0.1 USER LOSES BY THE DELETION: one step of explicitness, not a fact. I considered raising it and decided against, because restoring the explicit form for two of three files requires authoring a new sentence that splits the three literals into two classes, which is the exact shape that produced a new defect in rounds 3 and 4, and because the surviving cross-reference plus `README.md:327` leave the reader able to act.

## Question 2: the deleted clause in the `read_optional` doc

NOT A FINDING. Nothing of value went with it, and the surviving paragraph is both a warning and a correct picture.

THE SURVIVING PARAGRAPH, `src/manifest.rs:524-527`:

> It does not make swallowing impossible, and nothing in Rust can: a caller may still write `.unwrap_or_default()` on this too. What it buys is that the correct optional-read primitive exists and is the obvious one to reach for. The invariant is held by review, not by the compiler.

DOES IT STILL WARN THAT SWALLOWING IS POSSIBLE AT ALL: yes, twice. The first sentence states it directly, and the last states where the guard actually lives.

IS THE READER LEFT WITH A CORRECT PICTURE: yes, and I measured the part that could have made it wrong. If the project's gates DID catch a swallow, then "held by review, not by the compiler" would understate the guard. They do not. My probe tree adds ONE new caller to `build_assets` and changes nothing else:

```rust
builtin.insert(
    "banner".to_string(),
    source.read_optional("banner.md").unwrap_or_default().unwrap_or_default(),
);
```

```
cargo clippy --all-targets -- -D warnings   exit 0, CLEAN
```

So a new swallowing caller passes the project's own gate, the surviving sentence is the honest description, and the deleted clause added nothing the survivors do not say. The clause's own defect was that it generalised to "a caller that does", which round 4 measured false of both callers that exist (each is the only construction site of its error variant, so a swallow makes the variant dead and fails `-D warnings`).

## Question 3: the replaced sentence at `src/main.rs:229`

NOT A FINDING.

```
old:  The file is present and could not be read: a containment refusal, or an
      unreadable file.
new:  The read did not produce text: a containment refusal, or an unreadable path.
```

WHAT THE OLD CONVEYED THAT THE NEW DOES NOT:

1. "The file is present". An existence assertion that rounds 3 and 4 both measured false, on a `--template` root that is a symbolic-link loop and on an unreadable pack root holding no `principles.toml`. Losing a false clause loses nothing a maintainer should have.
2. The contrast with absence, implied by "present". This survives, at a better scope: the enum-level doc at `src/main.rs:224-226` states "ABSENCE is not in here: a pack that ships no `principles.toml` has no principles to select, which is an empty set rather than a failure", and `read_optional`'s `NotFound`-only fold is what implements it. A maintainer reading the enum cold reads that sentence first, two lines above the variant.

WHAT THE NEW SENTENCE GAINS: "an unreadable file" becomes "an unreadable path", which is the widening round 4's triage required, because the unreadable thing can be the pack directory rather than a file inside it. `PrinciplesError::Read` fires for `ReadError::Escapes` and for any `ReadError::Io` other than `NotFound`, and "a containment refusal, or an unreadable path" covers exactly that set.

The third sentence, "Distinct from `Parse` because the file never became text, so telling the user it did not parse would name the wrong step", is unchanged in substance and only re-wrapped. Round 4's triage asked for it to be left alone, and it was.

## Question 4: the 0.0.2 section as a whole

Both directions, against `git diff main..HEAD` (16 files, 1728 insertions, 248 deletions).

### Direction A: something described that did not happen

I re-measured every prior-behaviour claim in the four bullets this increment added, at MAIN, on my own fixtures. All hold.

| bullet | prior-behaviour claim | measured at MAIN |
| --- | --- | --- |
| `CL:36` | an `[[asset]]`'s `source` leaked through `..`, absolute and link shapes, at exit 0 with an ordinary `create` plan line | TRUE. All three shapes exit 0 and write `leaked.md` carrying `OUTSIDE-ASSET-BYTES`; stdout shows "create leaked.md" and "Wrote to <output-dir> (2 changed, 0 left untouched)." |
| `CL:36` | a `[[module]]`'s `guidance` spliced the outside file into `{{modules}}` at exit 0 with no plan line naming it | TRUE. `AGENTS.md` carries `M:OUTSIDE-GUIDANCE-BYTES`; the only plan line is `create AGENTS.md` |
| `CL:37` | each of the three literals was read through a link out of the pack in 0.0.1 | TRUE of all three (see question 1) |
| `CL:37` | an unreadable file produced an empty principle set or an empty instrumentation block at exit 0 with empty stderr | TRUE. Invalid UTF-8 in `principles.toml`, and in `instrument.md`, each give MAIN exit 0, stderr empty, `AGENTS.md` = `P:\nI:\n`; HEAD exits 2 naming the file |
| `CL:37` | ABSENCE IS UNCHANGED, byte for byte what 0.0.1 produced | TRUE. `md5sum` of `AGENTS.md` is `5a28f5e12a01946aaad53f844b4db5fe` at MAIN and at HEAD, with and without `--instrument` |
| `CL:38` | a `..`-bearing `dest` wrote the file outside the output directory at exit 0 while reporting "Wrote to `<output-dir>`" | TRUE. `planted2.md` lands outside; stdout reports "Wrote to <output-dir> (2 changed, 0 left untouched)." |
| `CL:39` | a multi-line `ask` split the queue and fabricated an entry, so a source `validate --source` accepted rendered a `<task>.md` that `validate --plan` rejected | TRUE. On a mutated render fixture MAIN's `validate --source` exits 0 and its rendered view draws `Open Questions item `Q-42` has an unknown status `undecided``; at HEAD `Q-42` is gone and only the pre-existing `Q-3` `superseded by` problem remains |

Nothing in the four bullets describes churn that no released version had. `CL:40` (W5) is not in `main..HEAD` at all, which round 3 already established and which I confirmed: `src/workflow.rs` does not appear in the diff, and the scaffolded output of both binaries is byte-identical, so the shipped rule text did not change in this increment either.

### Direction B: something that happened and is not described

I compared the two binaries on the ordinary paths first, to bound the search:

```
scaffold (built-in pack, plain)                                  trees IDENTICAL, stdout SAME
scaffold --instrument --module checks --module isolation
         --principles all --principle-detail full                trees IDENTICAL, stdout SAME
validate --source ... --metrics ...                              SAME, exit 0
validate --source ... --metrics ... --workflow                   SAME, exit 0
validate --plan ... --metrics ...                                SAME, exit 1
status --source ...                                              SAME, exit 0
next --source ...                                                SAME, exit 0
render --check --strict ...                                      DIFFERS: main exit 0, head exit 1   -> L1
```

Every pack-shape difference I could produce is described:

```
source "../outside/secret.md"      MAIN exit 0 leaks   HEAD exit 2   CL:36
source "<absolute>/secret.md"      MAIN exit 0 leaks   HEAD exit 2   CL:36
source "link.md" -> outside        MAIN exit 0 leaks   HEAD exit 2   CL:36
guidance "g.md" -> outside         MAIN exit 0 splices HEAD exit 2   CL:36
dest "../elsewhere/planted2.md"    MAIN exit 0 plants  HEAD exit 2   CL:38
pack-internal link "alias.md"      MAIN exit 0         HEAD exit 0   CL:36 ("both keep working"), unchanged
principles.toml / instrument.md / pack.toml linked out or unreadable          CL:37
```

So `L1` is the one undescribed user-visible difference I found, beside the recorded residual I am not raising.

## Considered in scope and NOT raised

Recorded so the triager can see these were checked rather than missed.

- A `source` that is ABSOLUTE but points INSIDE the pack, and one that carries a `..` which cancels back inside (`sub/../inside.md`), both worked at MAIN (exit 0, the asset is copied) and are refused at HEAD (exit 2). `CHANGELOG.md:36`'s "THE SHAPE THAT STOPS WORKING ... is a pack DIRECTORY whose files are symbolic links to targets OUTSIDE it" does not name either. I did not raise it: that sentence is scoped by "beside those two that survive" to the LINK shapes it has just been discussing, the same bullet states the lexical rule in full two sentences earlier ("the path string must be relative and carry no `..` component"), so both refusals follow from the stated rule, and each refusal names the value and the specific cause. A pack naming an absolute path inside its own pack was also outside the documented contract before this release (`AssetSpec.source` is documented as "Path of the source file within the pack").
- The two refusal messages for the optional literals spell the filename differently: "could not read the pack's principles.toml: ..." with no backticks, "could not read the pack's `instrument.md`: ..." with them. Cosmetic, no fact is wrong, and it is outside both the three deletions and the section question.

## Out-of-scope observations

Real, reproduced, and NOT findings. Reported here because the record should carry them.

1. `src/main.rs:224-226`, the enum-level doc, reads "ABSENCE is not in here: a pack that ships no `principles.toml` has no principles to select, which is an empty set rather than a failure." Round 4 measured a shape where a pack ships no `principles.toml` and the run fails anyway: an unreadable pack ROOT, where `PrinciplesError::Read` fires and the message names a file that is not there. Read as a statement about the enum's design (no absence variant, absence returns `Ok(Vec::new())`) the sentence is true; read as a universal claim about packs it is falsified by that shape. I do not raise it: the shape is the recorded permission-class residual, the sentence was not touched by this commit, the artefact is a doc comment on a private enum in a binary-only crate with no lib target and so no rendered rustdoc, and no exit code or output byte turns on it.

2. `main` is not the 0.0.1 release tree (see Method). Rounds 3, 4 and this one have all measured "0.0.1 behaviour" at `main`'s tip, which is 18589 source insertions ahead of the `v0.0.1` tag. Nothing I measured changes if the tag is used instead, as far as the read paths go, but the record should say which tree the phrase "0.0.1" names in these documents, because the release notes make claims about the PUBLISHED 0.0.1 and no round has run that binary.

3. `cargo publish --dry-run` needs the network and I did not run it. It stays the one release gate unverified by this round, as it was in round 4.

## Round outcome, from my lens

NOT CLEAN, on one finding, `low`, and the finding is an omission rather than a false statement.

THE THREE DELETIONS ARE CLEAN. Each removed a statement that was measured false, each left the fact that mattered stated somewhere a reader reaches, and one of the three widened a noun in the direction round 4's triage required. Nothing needed to survive that did not. That is four rounds of evidence and one more data point for the pattern round 4 measured: the pure-deletion sites in this increment have now produced zero findings across two passes.

`L1` is not a defect of the deletion method. It is older than the deletion passes: the bullet it concerns was authored when the render fix landed, and four rounds missed it because every round ran the gate inside the tree where the re-render had already happened.
