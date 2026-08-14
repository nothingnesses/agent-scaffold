# `ship-v0-0-2-inc1` round 5: TRIAGE (the cap round)

Independent triager. I did not write this change, I did not review it in any round, and I did not triage rounds 1 to 4. Every figure below is my own measurement, made in this session. Where I reproduce a reviewer's result I say so and give my own numbers rather than citing theirs.

This is the fifth round of a cap of five. The loop has reached its cap without converging, so this ruling is the evidence the human decides from. I adjudicate and I do not take the decision.

## Artifact

- Worktree `.claude/worktrees/tri-r5`, detached at `973f4e0` ("docs: delete the three claims round 4 measured false").
- Ruled on: `git diff main..HEAD` (12 files, 1723 insertions, 149 deletions), and for the deletion pass `git diff HEAD~1..HEAD` (3 files, 4 insertions, 5 deletions).
- Specification: `docs/plans/agent-scaffold.steps/ship-v0-0-2.md`. Risk class `low_risk` (`Q-74`), so ONE clean round converges the loop.
- Findings adjudicated: `v002-r5-reviewer-truth.md` (zero findings) and `v002-r5-reviewer-loss.md` (`L1`, `low`).
- Settled and not reopened: the round 1, 2, 3 and 4 triages, the round 4 recorded residuals (the `--template` "must name a directory" message under an unreadable parent, and the missing bullet for it), `A2`, `A3`, `A5`, the audit's `F2` and `F3`, the `superseded by` projection defect, the rename, ANSI escapes in a `dest`, the plan-side sidecar symlink hole, the empty-directory gap, and the containment mechanism.

ONE DIFFERENCE FROM WHAT THE REVIEWERS SAW, and it matters for one of my findings. Both round 5 reviewers worked at `e3a466e`, whose commit message is identical to `973f4e0`'s. The commit was rebased onto a newer `main` between their round and mine. `git merge-base --is-ancestor main HEAD` reports `main` as an ancestor of `973f4e0`, and `main`'s tip `bb7a937` carries `6c85fdc` (2026-08-14), which edited `docs/plans/agent-scaffold.plan.toml`. That rebase is what produces `T1` below. At `e3a466e` the gate in question is green and at `973f4e0` it is red, both measured by me, so neither reviewer could have seen it.

## Method

THREE release binaries, one `CARGO_TARGET_DIR` each, from three separate `git archive | tar -x` extracts, verified distinct by `md5sum`:

```
0474635842eb360933f61729944d412f  tgt-head/release/agent-scaffold  (973f4e0, HEAD)     0.0.2
89885fff7efac8f37415f7d958a9a1ce  tgt-main/release/agent-scaffold  (main, bb7a937)     0.0.1
35e3eabc38d8bddb6b8baf8388df6b0e  tgt-001/release/agent-scaffold   (tag v0.0.1)        0.0.1
```

`--version` confirms 0.0.2, 0.0.1 and 0.0.1. The tag `v0.0.1` resolves to commit `2bbce2e` (2026-07-10). `git rev-list --count v0.0.1..main` reports 965 commits, and `git diff --shortstat v0.0.1..main -- src/ Cargo.toml pack/` reports 73 files changed, 18589 insertions, 455 deletions.

I built the PUBLISHED 0.0.1 because the orchestrator asked for the measurement nobody has taken. Rounds 1 to 4 built one 0.0.1 binary, from branch `main`, and `v002-r3-triage.md:19` records that choice in writing as "`main`, `Cargo.toml` version `0.0.1`, what the last release ships". It is not what the last release ships. Round 5's truth reviewer built the tag as well and reported the discrepancy as an out-of-scope observation. This document takes the measurement to its end.

Every fixture, symbolic-link target, escape target and cargo target directory sits under my own scratch subdirectory. I used symbolic links and invalid UTF-8 rather than permission bits, so I SET NO `chmod` ANYWHERE and there is none to restore. `git status --short` is empty in this worktree and in the main repository, apart from this file.

GATES I RAN MYSELF AT HEAD:

| gate | result |
| --- | --- |
| `cargo test --no-fail-fast` | 468 passed, 0 failed, over 11 result lines (409+5+1+1+9+2+13+3+20+1+4), exit 0 |
| `cargo clippy --all-targets -- -D warnings` | clean, exit 0 |
| `validate --source ... --metrics ...` | `337 records, valid`, `99 steps, 76 questions, valid`, exit 0 |
| `validate --source ... --workflow` | `workflow invariants hold`, exit 0 |
| `render --check --strict` | **exit 1, RED.** See `T1` |
| `validate --plan ... --metrics ...` | EXACTLY ONE problem, the pre-existing `Q-43` `superseded by` one the spec's criterion 4 excludes |
| ASCII check on all 12 changed files | `0` non-ASCII lines on every file |

I did not run `cargo publish --dry-run`, which needs the network. It stays the one release gate unverified, as it was in rounds 4 and 5.

## Verdicts

FOUR valid findings. One reported by a reviewer and confirmed, three raised by me from my own required measurements. None invalid, none out of scope, none duplicate.

| id | source | verdict | severity | class | site | one line |
| --- | --- | --- | --- | --- | --- | --- |
| `T1` | triager | valid | **medium** | RELEASE GATE | `docs/plans/agent-scaffold.md` | `render --check --strict` exits 1 on the release commit's own tree. The spec's release criterion 4 requires it green. Fixed by one `render` and a commit. |
| `T3` | triager | valid | **medium** | USER-FACING | the 0.0.2 section as a whole | The CLI became subcommand-based since 0.0.1, so EVERY documented 0.0.1 invocation now exits 2, and no bullet says so. |
| `L1` | reviewer | valid | low | USER-FACING | `CHANGELOG.md:39` | The render fix makes a committed `<task>.md` stale, so `render --check --strict` exits 1 on an unchanged project. No bullet says so. |
| `T2` | triager | valid | low | USER-FACING | `CHANGELOG.md:36` and `:37` | Five claims about what 0.0.1 did are measured FALSE against the published `v0.0.1`. |

NO `critical`, NO `high`.

**I RAISE TWO FINDINGS TO `medium`, AND I STATE IT EXPLICITLY BECAUSE IT CARRIES OBLIGATIONS.** Neither is a reviewer severity I lifted: both are findings no reviewer reported, so no dismissed-or-downgraded-high re-check obligation arises. What the `medium` ratings do carry is that this round cannot be closed by accepting everything as a residual, and that the escalation the human receives is not a choice between shipping and polishing.

I lowered no severity. `L1` is confirmed at the reviewer's own `low`, on my own reproduction and on sharper reasoning about its population than the reviewer had.

## `L1` (low, USER-FACING): the section does not say an existing plan view must be re-rendered

### My reproduction, built from scratch

`git archive main docs` into a scratch tree, which gives this project's own plan exactly as `main` commits it, with nothing hand-edited. I verified the extracted `docs/plans/agent-scaffold.md` is byte-identical to `git show main:docs/plans/agent-scaffold.md` before running anything.

```
render --check docs/plans/agent-scaffold.plan.toml
  MAIN  exit=0  "docs/plans/agent-scaffold.plan.toml: up to date"
  HEAD  exit=0  "warning: docs/plans/agent-scaffold.md differs from a fresh render
                 (a hand-edit, or a stale render after a source edit)
                 (first difference at line 145: ...)
                 ; re-render with `agent-scaffold render docs/plans/agent-scaffold.plan.toml`"

render --check --strict docs/plans/agent-scaffold.plan.toml
  MAIN  exit=0  "up to date"
  HEAD  exit=1  "error: docs/plans/agent-scaffold.md differs from a fresh render
                 (a hand-edit, or a stale render after a source edit)
                 (first difference at line 145: ...)"
```

Confirmed. Same source, same committed view, same command, exit 0 becomes exit 1 on an untouched tree.

ONE DETAIL THE REVIEWER DID NOT REPORT, AND IT CUTS BOTH WAYS. The WARNING form ends with "re-render with `agent-scaffold render docs/plans/agent-scaffold.plan.toml`". The `--strict` ERROR form does not: I grepped both, and "re-render with" appears once in the warning and zero times in the error. So the user who meets this in CI, which is the whole population that cares, gets the two false causes and no remedy pointer, while the user who meets it interactively gets the remedy. That sharpens the finding slightly.

### Severity: `low`, confirmed, and my ground is not the reviewer's

The reviewer rated `low` while believing the population is "every project following the scaffolded workflow whose committed `<task>.md` carries a multi-line free-text value at one of the four sites". Measured against the published `v0.0.1`, that population is smaller than they thought, and the correction supports `low` rather than undermining it:

```
v0.0.1 binary, `render --help`                 exit 2, no such subcommand
v0.0.1 pack contents                           AGENTS.md pack.toml plan-template.md principles.toml prompts
v0.0.1 scaffolded plan artefact                docs/plans/TEMPLATE.md   (Markdown, no TOML source)
`*.plan.toml` anywhere in the v0.0.1 pack      0 files
```

A user of the PUBLISHED 0.0.1 has no `render` command and no `<task>.plan.toml`, so they cannot hit `L1` on their own plan at all. A user who scaffolds fresh with 0.0.2 renders with the fixed binary and never sees a mismatch. The affected population is exactly those who ran a PRE-FIX UNRELEASED build's `render`: people installing from git, and this project itself.

That is not an empty population, and `T1` below is this project meeting it. But it is not an upgrade consequence for anyone upgrading from the released 0.0.1, which is what round 2's `B2` `medium` bar requires.

WHICH PRECEDENT GOVERNS. The orchestrator asked me to rule this against two of them, and the correct comparison is with a third.

- Round 2's `B2` (`medium`) is an UNDISCLOSED BEHAVIOUR CHANGE that breaks a legitimate deployment for a user upgrading from the last release, whose only public description positively misleads them, with a heavy recourse (`v002-r2-triage.md:229`). `L1` fails two of those four tests: no released-version user is affected, and the recourse is one command.
- Round 4's `P4` (`low`) is an OMISSION of a bullet, where nothing false is published (`v002-r4-triage.md:243`). `L1` is the same class: an omission, in the same file, about the same kind of fact.
- The orchestrator's framing asks whether "a breaking change to a documented CI gate is the same category as a missing bullet". My answer is that it would not be, if the break reached a released-version user. It does not. What `L1` describes is a break for users of unreleased builds, and for that population the section is not a release-notes contract at all.

`low`, on `P4`'s precedent. And `L1` is NOT the finding on this mechanism that matters most, because the same mechanism has already produced a red gate on this project's own release commit, which is `T1`.

## `T1` (medium, RELEASE GATE): `render --check --strict` is red on the artifact

### What I measured

```
render --check --strict docs/plans/agent-scaffold.plan.toml, HEAD binary, three trees:
  git archive e3a466e   (what the round 5 reviewers reviewed)   exit 0
  git archive 973f4e0   (the artifact I was given)               exit 1
  worktree tri-r5       (973f4e0, git status clean)              exit 1
```

The cause, established rather than inferred. `git log -1 -- docs/plans/agent-scaffold.plan.toml` gives `6c85fdc` (2026-08-14, in `main`), which added `Q-76`. `git log -1 -- docs/plans/agent-scaffold.md` gives `3b4a2dc` (2026-08-13, the `F1` fix commit, in `main..HEAD`). The re-render happened BEFORE `Q-76` was written, and the rebase onto the newer `main` carried `Q-76` in with its pre-fix fragmented form intact. `git show main:docs/plans/agent-scaffold.md | grep -c Q-76` and the same at HEAD both report 5 lines, so HEAD's committed view still holds `Q-76` fragmented across five lines while a fresh HEAD render collapses it to one.

A fresh render at HEAD differs from the committed view by 10 lines, all of them `Q-76`'s paragraphs folding into one queue line. This is `F1`'s own fix meeting a view a pre-fix binary produced.

### Why it is a finding and not housekeeping

The specification's release acceptance criterion 4 (`ship-v0-0-2.md:124`) names seven gates that must be green on the release commit, and `render --check --strict` is one of them. It is red. Round 4's triage recorded it green, correctly, at `fde1d60`.

Two further facts I measured rather than assumed:

- The stale view is not corrupt. `validate --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl` on the stale view reports EXACTLY ONE problem, the pre-existing `Q-43` `superseded by` one criterion 4 excludes. No fabricated queue entry, no lost text.
- The stale view SHIPS. `cargo package --list` reports 380 files, of which 285 are under `docs/`. `docs/plans/agent-scaffold.md` is inside the 0.0.2 crate tarball, so publishing as it stands puts a stale projection on crates.io in the release whose headline fix is stale projections.

### The fix, measured

One command, no authored text:

```
agent-scaffold render docs/plans/agent-scaffold.plan.toml
  then: render --check --strict   exit 0
        validate --plan           EXACTLY ONE problem, the pre-existing Q-43 one
```

### Severity: `medium`, ruled

Not `low`. This loop's own test for staying at `low` is round 4's, at `v002-r4-triage.md:115`: raising above `low` requires the defect to be able to produce a wrong RESULT. A declared release gate returning exit 1 on the commit proposed for release is a wrong result, and it is the only finding in five rounds that changes an exit code on the artifact itself rather than on a fixture.

Not `high`. Nothing is read that should not be, nothing is written outside anywhere, no output byte is wrong, and the whole of it is repaired by one command that this project runs routinely. Round 1's `high` bar is a live wrong behaviour in what the software does (`v002-r1-triage.md:145`), and this is a stale artefact, not a code defect.

Principle 8 (Structured data first, project for humans) is the principle this breaks, and it breaks it in the sharpest available way: the plan's human view is a projection its own projector rejects, in the release that exists to make projections trustworthy.

## `T2` (low, USER-FACING): five claims about 0.0.1 are false against the published `v0.0.1`

This is the enumeration the orchestrator asked for, and it is in the next section, bullet by bullet. The finding is the five measured-false claims it contains, all of them in `CHANGELOG.md:36` and `:37`.

SEVERITY `low`, on round 4's `P2` precedent (`v002-r4-triage.md:190-194`), which rated exactly this class at `low`: a false factual sentence about the previous release's behaviour, in the shipped release notes. I weighed `medium` and rejected it on the direction of the error. `P2`'s falsity pointed a forensic reader at the WRONG PLACE, which is the under-warning direction, and round 4 still held it at `low`. These five all err in the OVER-WARNING direction: they tell a 0.0.1 user to check for leaks through an `instrument.md` and a `[[module]]` `guidance` their version does not have. Nobody is left less careful than they should be.

`T2` CANNOT BE A RECORDED RESIDUAL, on exactly the ground round 4 gave for `P2` (`v002-r4-triage.md:277-285`), and I apply that ruling rather than reinventing it. Principle 6 (Ground decisions in evidence) is the principle that refuses it: the project now holds the measurement, and publishing against a measurement you hold is what that principle names.

## The baseline enumeration, bullet by bullet, against the real `v0.0.1`

The 0.0.2 section is `CHANGELOG.md:7-40`, 21 bullets. The published `v0.0.1` section sits immediately below it at `:42-52`. Under Keep a Changelog, which `CHANGELOG.md:5` names as the format, every bullet in the 0.0.2 section is a claim about the delta from 0.0.1.

FIRST, THE FACT THAT DECIDES MOST OF THE TABLE. At the tag, the tool is a SINGLE COMMAND with no subcommands:

```
v0.0.1  Usage: agent-scaffold [OPTIONS]
        --output-dir --force --vcs --write --dry-run --principles
        --principle-detail --list-principles --template --var

HEAD    Usage: agent-scaffold <COMMAND>
        scaffold validate status next checks render audit help
```

Every one of the seven HEAD subcommands returns exit 2 at the tag. `src/` at the tag holds four files (`main.rs`, `manifest.rs`, `pack.rs`, `tui.rs`), zero occurrences of `instrument`, and a `Manifest` struct whose only sections are `asset` and `var` (`src/manifest.rs:67-71`); its own comment at `:65` says a "`[[module]]` section can be added without breaking older loaders", which is the anticipation, not the implementation.

| # | bullet | verdict against the published `v0.0.1` |
| --- | --- | --- |
| 1 | `:11` Deprecated, the `agent-scaffold` name | NOT COMPARATIVE. Forward-looking. Unaffected. |
| 2 | `:15` serialised reasons on `status`/`next` `--json` | VACUOUS-ADJACENT. Both commands are absent at the tag. Its comparatives ("rather than reading the same bare `null`", "matching the projections' existing no-omitted-field convention", "unchanged for every cause that existed before") all describe `main`. |
| 3 | `:16` `agent-scaffold audit` | TRUE as an addition. Presupposes the unannounced subcommand CLI. |
| 4 | `:17` `type:"waiver"` record and W5 | TRUE as an addition. "W3 becomes convergence-OR-waiver" presupposes W3, absent at the tag. |
| 5 | `:18` `type:"decision"`, `type:"baseline"`, W4 | TRUE as an addition. Presupposes the instrumentation machinery, absent at the tag. |
| 6 | `:19` `validate --workflow` cross-reference | TRUE as an addition. "It reuses the same metrics log as the rest of `validate`" presupposes `validate`, absent at the tag. |
| 7 | `:20` `trivial`/`grandfathered`, RETIRED in the same cycle | TRUE, and the ONE bullet in the section that reasons correctly about the unreleased cycle. Net effect on a 0.0.1 user is nothing, and it says so. |
| 8 | `:21` optional `harness` on `reviewers[]` | TRUE as an addition. "so existing round records stay valid" names records absent at the tag. |
| 9 | `:22` `--module isolation` | TRUE as an addition. |
| 10 | `:23` optional modules and `--module checks` | TRUE as an addition. |
| 11 | `:27` `validate --workflow` now FAILS with no round log | **VACUOUS.** "where it previously announced a skip on stderr and exited 0" describes `main`. At the tag there is no `validate`. Its "THE POPULATION THIS BREAKS" breaks no 0.0.1 user. |
| 12 | `:28` `validate --workflow` now REFUSES an unattributable log | **VACUOUS.** `validate`, `status` and `next` are all absent at the tag. Its "TWO BREAKS TO KNOW ABOUT" break no 0.0.1 user. |
| 13 | `:29` metrics log and ledger resolved from the PLAN SOURCE | **VACUOUS.** "Previously both defaults were relative to the current directory" describes `main`. No such defaults exist at the tag. |
| 14 | `:30` hardened the scaffolded workflow guidance | **TRUE, MEASURED.** `pack/AGENTS.md` exists at the tag. Its line 104 reads "The ledger is transient working state, keep it as scratch notes", against 0.0.2's committed-beside-the-plan ledger; its line 99 reads "a clean round ends the loop", against 0.0.2's consecutive clean rounds. Both changes are real against the published release. |
| 15 | `:31` split review and triage in the README diagram | **TRUE, MEASURED.** The tag's README diagram has ONE node, `preview["Review the plan, then triage<br/>(reviewers, triager)"]`. `main`'s has two, `preview["Review the plan<br/>(reviewers)"]` and `ptriage["Triage the findings<br/>(triager)"]`. |
| 16 | `:32` generalised the diversity guidance | **TRUE, MEASURED.** The tag's `pack/AGENTS.md:28-29` reads "different models / where available, since same-model reviewers share blind spots", which is the quoted "from" text exactly. |
| 17 | `:36` a pack path can no longer read outside the pack | **CONTAINS ONE FALSE CLAIM.** See below. |
| 18 | `:37` the three literal-name files | **CONTAINS FOUR FALSE CLAIMS.** See below. |
| 19 | `:38` a pack `[[asset]]`'s `dest` | **TRUE, MEASURED, IN FULL.** The one Fixed bullet wholly true of the published release. |
| 20 | `:39` `render` and free-text structure | **VACUOUS.** `render` is absent at the tag, as is `validate --source` and `validate --plan`. "THE VISIBLE EFFECT on an existing plan" reaches no plan a 0.0.1 user owns. |
| 21 | `:40` W5's waiver-ownership rule | **VACUOUS.** W5 was both added (bullet 4) and fixed (this bullet) inside the unreleased cycle. The "retired rule" never shipped. |

ROLLUP: 4 bullets measured fully true against the published `v0.0.1` (`:30`, `:31`, `:32`, `:38`); 2 carrying measured-false claims (`:36`, `:37`); 5 vacuous (`:27`, `:28`, `:29`, `:39`, `:40`); 8 true as additions but presupposing machinery the section never introduces; 1 non-comparative; 1 (`:20`) correct about the cycle.

### The five false claims, each with its run

```
1. `:36` "TWO pack-controlled fields reach that join, and all three shapes leaked
   through both ... and a `[[module]]`'s `guidance`"
   FALSE. At the tag ONE such field exists.
     pack declaring [[module]] name="leaky" guidance="g.md" (g.md -> outside):
       v0.0.1  exit 0, AGENTS.md = "M:{{modules}}"   nothing spliced, module ignored
       main    exit 0, AGENTS.md = "M:OUTSIDE-ASSET-BODY"   spliced
       HEAD    exit 2, "module `leaky` guidance file `g.md` is not a contained pack path"
     v0.0.1 has no --module flag (exit 2) and no `module` section in its Manifest.

2. `:37` "The three files the tool reads by literal name"
   FALSE as a statement of 0.0.1. At the tag there are TWO literal reads:
     src/manifest.rs:197  self.read("pack.toml")
     src/main.rs:181      source.read("principles.toml")
     zero occurrences of "instrument" in the whole of src/.

3. `:37` "In 0.0.1 each was read through a symbolic link out of the pack"
   FALSE of `instrument.md`, TRUE of the other two.
     principles.toml -> outside   v0.0.1 exit 0, "P:1. LEAKED-PRINCIPLE - OUTSIDE-PRINCIPLE-SUMMARY"  LEAKED
     pack.toml       -> outside   v0.0.1 exit 0, the outside manifest is OBEYED, its asset planted
     instrument.md   -> outside   v0.0.1 exit 0, "I:{{instrument}}"   NOT READ
   And not merely for the linked case: a PACK-INTERNAL instrument.md is not read
   either, and `--instrument` does not exist (exit 2).
     pack-internal instrument.md  v0.0.1 "I:{{instrument}}"   HEAD "I:INSIDE-PACK-INSTRUMENT"

4. `:37` "produced an empty principle set or an empty instrumentation block at exit 0
   with empty stderr in 0.0.1"
   The principle-set half is TRUE, measured:
     invalid-UTF-8 principles.toml  v0.0.1 exit 0, stderr EXACTLY 0 bytes, "P:"
                                    HEAD   exit 2, "could not read the pack's principles.toml"
   The instrumentation half is FALSE: 0.0.1 produces no instrumentation block.

5. `:37` "an empty instrumentation block, byte for byte what 0.0.1 produced"
   MIXED, and false in the reading the sentence invites.
     0.0.1-era pack (no {{instrument}} placeholder), neither literal present:
       v0.0.1 / main / HEAD  all d38c0b5760f21fb257dbe46ef146e7b5   TRUE
     pack carrying {{instrument}}:
       v0.0.1  3d3b03e33db2eef7d20c85da75db1fea  ("P:\nI:{{instrument}}\n")
       main    5a28f5e12a01946aaad53f844b4db5fe  ("P:\nI:\n")
       HEAD    5a28f5e12a01946aaad53f844b4db5fe
   Byte identity holds only for a pack that could actually have been authored for
   0.0.1. A pack using the placeholder gets different bytes, because 0.0.1 leaves it
   unsubstituted.
```

`:37`'s remaining claim, "which matches a malformed `principles.toml`, already loud in 0.0.1", is TRUE and I measured it: the tag, `main` and HEAD all exit 2 with the same `TOML parse error at line 1, column 6`.

I also confirmed `:36`'s and `:38`'s surviving claims against the tag, so the record is not one-sided:

```
[[asset]] source = "../outside/secret.md"   v0.0.1 exit 0, leaked.md carries TOP-SECRET-OUTSIDE-THE-PACK
[[asset]] source = "link.md" -> outside     v0.0.1 exit 0, same
[[asset]] dest = "../elsewhere/planted.md"  v0.0.1 exit 0, planted OUTSIDE the output dir
   HEAD refuses all three at exit 2, each message naming the value and the cause.
```

So the read escape and the write escape both really did exist in the version the notes name. The defect is in the ENUMERATION around them, not in the disclosure itself.

## `T3` (medium, USER-FACING): the hard break nobody has disclosed

The enumeration above is one direction. The other direction is what a 0.0.1 user gains and is not told about, and there the section has a hole no reader can work around.

```
The documented 0.0.1 invocation, run at HEAD:
  agent-scaffold --output-dir <dir> --vcs none --write
    v0.0.1  exit 0, 11 files written
    HEAD    exit 2, "error: unexpected argument '--output-dir' found"
                    "Usage: agent-scaffold <COMMAND>"
  agent-scaffold --list-principles     v0.0.1 exit 0   HEAD exit 2
  agent-scaffold --dry-run             v0.0.1 exit 0   HEAD exit 2
```

Every flag the `## [0.0.1]` section itself advertises at `CHANGELOG.md:47-51` (`--principles`, `--principle-detail`, `--list-principles`, `--write`, `--dry-run`, `--force`, `--template`, `--var`, `--vcs`) is now a top-level parse error. The remedy is to insert `scaffold`, and no bullet in the 0.0.2 section says so. I grepped the section: "agent-scaffold scaffold" 0, "Usage" 0, "top-level" 0.

The section spends a 171-word bullet on the `metrics_absent_reason` JSON field and does not mention that the command line is now subcommand-based.

Nor does it introduce, as additions, the commands a 0.0.1 user gains: `validate`, `render`, `status`, `next` and `checks` all appear in the section only as things whose behaviour has CHANGED, presupposing an existence a 0.0.1 user has no basis for. Only `audit` and the module system are introduced as new.

### Severity: `medium`, ruled

This is round 2's `B2` shape, and `B2` was rated `medium` (`v002-r2-triage.md:227-229`): a legitimate usage stops working, the only public description of the change does not name it, and the affected reader is an upgrader from the last release. `T3` is strictly larger than `B2`. `B2` broke pack directories assembled from symbolic links, a real but narrow deployment. `T3` breaks every invocation every 0.0.1 user knows.

Not `high`. The failure is loud, exits 2, writes nothing, and prints `Usage: agent-scaffold <COMMAND>`, so the recourse is one `--help` away. Nothing is silently wrong.

## The ruling on classification

The orchestrator asked whether the baseline problem is a finding against this artifact, a finding against settled text, or a scope question. Both round 5 reviewers placed it out of scope on the ground that the deletions did not create it and that it reaches settled text in bullets nobody is reviewing. **I rule that both were wrong on the first ground and half right on the second**, and the measurement that settles it is which text is inside `git diff main..HEAD`.

`git diff main..HEAD -- CHANGELOG.md` adds exactly:

```
-## [Unreleased]
+## [0.0.2] - 2026-08-13
+### Deprecated ... (the rename bullet)
+ four Fixed bullets: the pack-path read, the three literals, the dest, and render
```

From that:

1. **A FINDING AGAINST THIS ARTIFACT.** `CHANGELOG.md:36` and `:37`, which carry all five measured-false claims, are this increment's own text. `git log -S` attributes `:36`'s "TWO pack-controlled fields" to `0ff3508` and `:37`'s "In 0.0.1 each was read" to `59d3591`, both inside `main..HEAD`. The round 5 reviewers' "settled text" reasoning does not apply to them. `T2` is in scope without argument.

2. **A FINDING AGAINST THIS ARTIFACT, BY THE ACT OF CLOSING THE SECTION.** The `## [Unreleased]` to `## [0.0.2] - 2026-08-13` change is itself in the diff, made by `8670e6c` ("chore: release 0.0.2"), which is inside `main..HEAD`. That single line is what converts every accumulated cycle-internal bullet into a claim about the delta from 0.0.1. `:27`, `:28`, `:29` and `:40` were true and useful sentences while the section was `## [Unreleased]` and a developer read them as "since the last release we changed this thing we also added". The commit under review is the one that publishes them as a released version's account of its predecessor. So the vacuity of those four bullets is not inherited settled text: it is created by an edit inside the artifact.

3. **A SCOPE QUESTION, IN ONE PART ONLY.** Whether to REWRITE the four inherited bullets so each describes a delta from 0.0.1 is a scope question, and a large one. It is not needed to make the section honest, because a scoping sentence at the head of the section fixes the whole class without touching a bullet.

So: not a scope question, except in its most expensive remedy. It is a finding against this artifact, twice over.

## Can the 0.0.2 section be published as it stands? NO

Three grounds, in order of how hard they are to argue with:

1. `T2`. Five sentences that this project has now measured false, about the behaviour of a version a reader can install and check, in the file that ships as the release notes. This is round 4's `P2` at five times the count, and round 4 ruled that class cannot be a recorded residual.
2. `T3`. A total break of every documented 0.0.1 invocation, undisclosed.
3. `T1`, which is not about the section but blocks the release independently: a named release gate is red.

### The minimum set of edits

I separate them as asked, into this increment's own bullets and bullets settled in earlier rounds.

**To this increment's own bullets (inside `git diff main..HEAD`):**

- E1. `CHANGELOG.md:37`: delete "and `instrument.md`" from the opening list, or scope the "In 0.0.1" sentence to the two files that existed. Pure deletion is available: removing `instrument.md` from the sentence's quantifier leaves a claim I measured true of both survivors.
- E2. `CHANGELOG.md:37`: delete "or an empty instrumentation block" (two occurrences, at the unreadable-file clause and the absence clause). Pure deletion. The surviving principle-set claims are measured true at the tag.
- E3. `CHANGELOG.md:36`: scope or delete the `[[module]]` `guidance` half of the "TWO pack-controlled fields" claim as a statement about 0.0.1. This one cannot be a clean deletion without losing real disclosure for `main` users, so it is the one site where a replacement clause is needed.

**To bullets settled in earlier rounds: none required.** One added sentence at the head of the section covers `:27`, `:28`, `:29`, `:39` and `:40` at once, and it also fixes `T3` and `L1` if it carries the CLI fact:

- E4. One paragraph under `## [0.0.2]`, before `### Deprecated`, stating three measured facts: that 0.0.2 follows 0.0.1 by 965 commits; that the command line is now subcommand-based, so every 0.0.1 invocation gains a leading `scaffold`; and that a project whose `<task>.md` was rendered by an earlier unreleased build needs one `agent-scaffold render` before `render --check --strict` passes. Every figure in it is measured in this document.

**Outside the section:**

- E5. `T1`: run `agent-scaffold render docs/plans/agent-scaffold.plan.toml` and commit. One command, no authored text.

E4 is authored text, and this loop's own evidence says authored text is where defects come from. I state that plainly rather than pretending otherwise, and it is why my recommendation includes a verification round.

## The round outcome

**ROUND 5 IS NOT CLEAN.** Four valid findings: two `medium` (`T1`, `T3`) and two `low` (`L1`, `T2`). The consecutive-clean streak stays at 0.

```
round 1:  5 valid, ceiling high     streak 0
round 2:  6 valid, ceiling high     streak 0    (one high, one medium)
round 3:  8 valid, ceiling low      streak 0
round 4:  4 valid, ceiling low      streak 0
round 5:  4 valid, ceiling MEDIUM   streak 0
```

The ceiling rose this round, for the first time since rounds 1 and 2. That is not a regression in the code. It is the consequence of measuring against the version the notes name instead of against `main`, which no round had done, and of the artifact being rebased between the reviewers' round and mine.

`ship-v0-0-2-inc1` is `low_risk` (`Q-74`), so one clean round would converge it. The loop is at the cap of five without a clean round, so it escalates to the human under `AGENTS.md:57`.

TWO OF THE FOUR CANNOT BE RECORDED RESIDUALS. `T2` cannot, on round 4's `P2` ruling and Principle 6. `T1` cannot, because a residual is an accepted RISK and an unmet written acceptance criterion is not a risk, it is a criterion that has not been met. `T3` and `L1` are both omissions and both CAN honestly be residuals under round 4's `P4` test, though I advise against it for `T3` and think it is reasonable for `L1`.

## Re-seeding, measured

The round 4 fix pass is `973f4e0`, three files, 4 insertions, 5 deletions, three edit sites:

| site | what the pass did | findings at this site |
| --- | --- | --- |
| `CHANGELOG.md:37` | DELETE five words | 0 |
| `src/manifest.rs:524-525` | DELETE one clause, re-wrap | 0 |
| `src/main.rs:229-230` | delete a false clause, WRITE a replacement clause, widen one noun | 0 |

STRICT RATE, on the definition rounds 2, 3 and 4 used (text or behaviour the fix pass INTRODUCED): **0 of 4**. None of the four findings is at text this pass wrote. `L1` is at `CHANGELOG.md:39`, attributed by `git log -S` to `8670e6c`. `T2`'s five claims predate the pass (`0ff3508` and `59d3591`). `T3`'s subject predates it. `T1` was introduced by the rebase, not by the pass.

BROAD RATE (findings whose subject the pass did not write but did touch): **1 of 4**. The pass edited `CHANGELOG.md:37` and left four of `T2`'s five false claims standing in the sentence it was editing.

```
round 2:  3 of 6   50 percent
round 3:  7 of 8   88 percent
round 4:  4 of 5   80 percent
round 5:  0 of 4    0 percent   (broad: 1 of 4, 25 percent)
```

### The correction to round 4's record

Round 4's triage states at `v002-r4-triage.md:369`: "EVERY FINDING THIS ROUND IS AT A SITE WHERE THE PASS AUTHORED SOMETHING. EVERY PURE-DELETION SITE PRODUCED NOTHING." The second sentence survives. The first does not, and round 4's own table two lines above refutes it: of its five authored sites, `src/manifest.rs:1319-1320` (delete then write a shorter replacement) and `tests/...:445-469` (write a new test and its comment) each produced 0 findings. So three of five authored sites produced findings, yielding four defects, and the universal claim is false.

Round 5 adds three more sites: two pure deletions and one delete-plus-write, all three producing nothing.

CUMULATIVE, over rounds 4 and 5:

```
pure-deletion sites:   6, producing 0 findings
authored sites:        6, producing findings at 3 of them, 4 defects total
```

The ONE-WAY CLAIM IS WHAT THE DATA SUPPORTS, and it is now stronger than round 4 could show: no pure-deletion site has ever produced a finding, across two passes and six sites. The converse, that authoring produces a finding, is refuted at half the authored sites. The useful instruction is round 4's, unchanged and now better evidenced: prefer deletion, and where a claim cannot be deleted, copy its replacement verbatim from a measurement already in a findings file.

AND ONE THING THE RE-SEEDING FRAME MISSES ENTIRELY. Three of this round's four findings are at text that NO fix pass ever touched, and one is at no text at all. A metric that counts what the fix pass re-seeded cannot see a defect that has been sitting in the section since `8670e6c` closed it, or one that a rebase introduced. Five rounds optimised against re-seeding and the remaining defects are all in the places re-seeding does not look.

## What five rounds bought

The human is entitled to this before deciding whether to spend more, so I give it without dressing it up.

Across five rounds, roughly 27 valid findings. Splitting them by whether they were defects in what the SOFTWARE DOES or in the TEXT that describes it:

**Defects in shipped behaviour: five, all in rounds 1 and 2.**

- `A1` (round 1, `high`): the containment predicate was lexical, so a pack shipping a symbolic link still read outside the pack. This changed the code that ships and it is the single most valuable finding of the loop.
- `B1` (round 2, `high`): the follow-on defect in the same boundary.
- `A2`, `A3`, `A5` (round 1, `low`): a `dest` writing through a pre-existing user symlink; degenerate `dest` values reaching a raw `Debug` `io::Error`; the source and dest checks disagreeing about disabled modules. All three were routed to follow-ups rather than fixed.
- `T1` (round 5, `medium`) is behaviour-adjacent rather than a code defect: a stale committed artefact and a red gate, not a bug in the binary.

**Defects in the text: the remaining 21.**

- Round 1: 1 (`A4`, refusal messages asserting something untrue of the input).
- Round 2: 5 (`B2` plus the four claims findings).
- Round 3: 8, all `low`, all text. Round 3's own triage records at `:375` that the pass "owns seven `low`s and no defect of any severity in the code it changed".
- Round 4: 4, all `low`, three of them text and one a message string.
- Round 5: 3 of 4.

So rounds 1 and 2 found the code defects, and rounds 3, 4 and 5 found fifteen text defects and zero code defects. Every gate has been green throughout except the one that turned red by rebase this round.

**AND THE PART THAT IS UNCOMFORTABLE AND HAS TO BE SAID.** Rounds 3 and 4 were intensive text-truth reviews. Round 4's reviewers checked 50 assertions and confirmed 45 (`v002-r4-triage.md:456`), and round 2's claims reviewer checked 62 (`v002-r2-triage.md:20`). Both later rounds, and both triages, measured "0.0.1 behaviour" against branch `main`, on a convention `v002-r3-triage.md:19` wrote down as "what the last release ships". It is 965 commits and 18589 source insertions from what the last release ships. So a share of the text-verification work of rounds 3 and 4 certified as TRUE a set of sentences that are false of the version they name, and the loop then spent round 4 and round 5 refining sentences whose baseline was wrong the whole time. Round 4 correctly deleted "and its contents inlined" from `CHANGELOG.md:37` on a measurement against `main`; the same sentence is still false against the tag, for a different reason nobody had looked for.

The loop's method was sound and its instrument was miscalibrated. That is the honest account of what five rounds bought, and it is the strongest argument for the fifth round having happened at all, because round 5 is the round that caught it.

## The options for the human at the cap

Not my decision. Here is the position, the options, what each costs and buys, and my recommendation, judged against the plan's own Project Principles by name.

ONE THING IS NOT AN OPTION. `T1` must be fixed under every option below. `render --check --strict` is a written release acceptance criterion (`ship-v0-0-2.md:124`), it is red, and the fix is one command with no authored text and no review risk. I record it as a correction rather than a choice.

### Option A: accept the residuals and merge

COSTS. Publishes five sentences the project has measured false about a version anyone can install and check, and ships a release-notes section that never tells a 0.0.1 user their command line changed. Fails Principle 6 (Ground decisions in evidence) on `T2`, which is the same call round 4 already made against `P2` and which nothing has weakened. Fails the disclosure standard round 2 set at `medium` for `B2`, on a break far larger than `B2`'s.

BUYS. The release ships today. Zero further rounds.

### Option B: fix a minimum set and merge without a further round

COSTS. E4 is authored text in the release notes, and this loop has found a defect in newly authored release-notes text in three of the last three rounds that produced any. Merging it unreviewed discards the loop's own strongest measurement about itself. E3 also needs a replacement clause.

BUYS. Cheapest honest option. One pass, no round. Satisfies Principle 6 and Principle 2 (Minimal by default), since every edit is text and nothing in the binary moves.

### Option C (RECOMMENDED): authorise a specific fix and ONE verification round beyond the cap

The fix set is E1 to E5 above, enumerated and closed, with the same instruction round 4 derived and round 5 confirmed: every edit is a deletion, except E3's replacement clause and E4's paragraph, and both of those may state only figures already measured in this document. The verification round reviews ONLY the edited text plus the re-rendered plan, not the whole section again.

COSTS. One round beyond the cap, which means the human is authorising an exception to the control constant rather than following it. Roughly one round's elapsed time.

BUYS. The one thing five rounds have shown this loop is reliably good at: catching a defect in newly authored release-notes text before it ships. The scope is small enough that a round on it is cheap, unlike the open-ended rounds 3 and 4.

BY PRINCIPLE:

- **Principle 6 (Ground decisions in evidence)** decides it against Option A. The project holds measurements that five claims are false of the version they name, and publishing against a measurement you hold is what this principle forbids.
- **Principle 8 (Structured data first, project for humans)** decides `T1` and makes it urgent rather than tidy. A projection its own projector rejects is not a projection, which is the specification's own wording for `F1` at `ship-v0-0-2.md:28`. Publishing 0.0.2 with a stale `docs/plans/agent-scaffold.md` inside the crate tarball would ship exactly the failure the release exists to fix.
- **Principle 1 (Prefer the cleaner long-term architecture over the smallest diff)** decides HOW THE FIX IS BUILT, in favour of E4's one scoping paragraph over rewriting `:27`, `:28`, `:29` and `:40` individually. One statement of what the section is written against fixes the class; four rewrites patch four instances and leave the next one open.
- **Principle 2 (Minimal by default)** keeps the fix set to five edits and refuses the temptation to rewrite the section. It is also the principle that argues against Option D.
- The specification's own closed-scope sentence (`ship-v0-0-2.md:3`) is respected. No edit in E1 to E5 adds a behaviour, a flag, a test or a message.

### Option D: pause the release and settle the baseline problem

COSTS. The step's stated purpose is delivery, and the audit it answers measured eleven consecutive days producing zero completed steps. Converting a delivery step into a baseline-methodology step is the exact mechanism the audit measured when it put steps generated by the process itself at 54.2 percent, and `ship-v0-0-2.md:3` exists to refuse it. Against Principle 2 (Minimal by default).

BUYS. The review loop stops measuring "the last release" against a branch, which is a real methodological defect with a real cost already paid in rounds 3 and 4. But it can be bought later, and cheaply: the fact fits in E4's paragraph now and the convention fix belongs in `workflow-audit-followups`, which already exists for exactly this class.

### My recommendation

**Option C.** Fix E1 to E5 as an enumerated, closed set, and spend one verification round on the edited text alone.

The reasoning in one paragraph. Two of this round's findings cannot honestly be residuals, so Option A is closed under Principle 6 and under the release's own acceptance criteria. Between B and C, the deciding evidence is this loop's own: the fix requires authoring a new paragraph in the release notes, and newly authored release-notes text is precisely where rounds 3, 4 and 5 each found a defect, while pure deletion has now gone six sites without producing one. Merging authored text unreviewed after five rounds of evidence that authored text is where the defects are would be ignoring the measurement the loop was run to produce. And Option D buys a real methodological fix at the price of the delivery the step exists for, when the same fact can be recorded in one sentence now and the convention repaired in the follow-up step that already exists to hold it.

One limitation of my own recommendation, since the human is deciding whether to spend a sixth round. The reason this loop has not converged is not that the code is bad. The code half has been clean since round 2 and every gate but one is green. It has not converged because the release notes are a 34-line comparative document about a version nobody had run, and each round rewrote a piece of it against the wrong reference. Option C ends that by fixing the reference rather than by rewriting the document again. If a sixth round finds a defect in E4's paragraph, the correct response is to shorten E4, not to run a seventh.
