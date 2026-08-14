# `ship-v0-0-2-inc1` round 6 (verification, beyond the cap): REVIEWER, the LOSS lens

Independent reviewer. I did not write this change and I did not review rounds 1 to 5. Every figure below is my own measurement, taken in this session. Where a round 5 result matters to a ruling I re-measured it rather than citing it.

My question is not whether the surviving text is true. It is whether the cut removed something a reader needs.

## Artifact and commits

- Worktree `.claude/worktrees/r6-loss`, detached at `ba466c2` ("docs: cut the release notes to what is true of the published v0.0.1"). `git status --short` is empty apart from this file.
- Ruled on: `git diff HEAD~2..HEAD`, two commits and two files.
  - `6050f93` "docs: re-render the plan view after the Q-76 rebase" (`docs/plans/agent-scaffold.md`, 10 lines).
  - `ba466c2` "docs: cut the release notes to what is true of the published v0.0.1" (`CHANGELOG.md`, 12 lines).
- Read first, as instructed: `docs/plans/agent-scaffold.reviews/v002-r5-triage.md`.
- Out of scope and not reopened: everything settled in rounds 1 to 5, the recorded residuals (including the missing bullet for the `--template` root message), the containment mechanism, `A2`, `A3`, `A5`, the audit's `F2` and `F3`, the `superseded by` projection defect, the rename, ANSI escapes in a `dest`, and the plan-side sidecar symlink hole.

### The size of the cut, measured

```
0.0.2 section, characters   pre-cut 26112   post-cut 14145   delta -11967
0.0.2 section, bullets      pre-cut    21   post-cut    15
bullet lines deleted        9   (7 outright deletions + the 2 rewritten bullets' old form)
bullet lines added          3   (1 new bullet + the 2 rewritten bullets' new form)
```

ONE CORRECTION TO MY OWN BRIEF, and it changes nothing but the arithmetic. The brief says six bullets were deleted outright. **Seven were.** The brief's own enumeration lists seven: five ruled vacuous (`:27`, `:28`, `:29`, `:39`, `:40`), plus `metrics_absent_reason` (`:15`), plus the `harness` field (`:21`). 21 - 7 + 1 = 15, which is the count I measure. I reviewed all seven.

## Method

TWO release binaries, one `CARGO_TARGET_DIR` each, from two separate `git archive | tar -x` extracts, verified distinct by `md5sum`:

```
3d4559c930b39338f6d34d997864bdd3  tgt-001/release/agent-scaffold    tag v0.0.1   --version: agent-scaffold 0.0.1
3daaebcace16a305beee35c4d104a4f5  tgt-head/release/agent-scaffold   ba466c2      --version: agent-scaffold 0.0.2
```

The tag `v0.0.1` is an annotated tag object `0f33878`, resolving to commit `2bbce2e`. `git rev-list --count v0.0.1..main` reports **966** commits (`main` tip `8993642`), and `v0.0.1..HEAD` reports 977.

**Every 0.0.1 claim in this document was produced by `tgt-001`, the binary built from the tag.** Where I give a HEAD result it came from `tgt-head`. I never used a `main` build and I never inferred a 0.0.1 result from source reading alone, except where I say so explicitly (the literal-read grep, which I then confirmed by running the binary).

All fixtures, packs, symbolic-link targets, output directories and target directories sit under my own scratch subdirectory. I used symbolic links and invalid UTF-8 rather than permission bits, so I SET NO `chmod` anywhere and there is none to restore.

### The facts about the tag that decide most of the table

Measured with `tgt-001`, not read from a previous round:

```
agent-scaffold <sub> --help, tag binary, exit codes:
  scaffold 2   validate 2   status 2   next 2   checks 2   render 2   audit 2
  (each prints "error: unexpected argument '<sub>' found")

tag usage line:   Usage: agent-scaffold [OPTIONS]
tag flags:        --output-dir --force --vcs --write --dry-run --principles
                  --principle-detail --list-principles --template --var
tag flags absent: --module 2   --instrument 2   --json 2   --resume 2   --workflow 2

tag src/ :        main.rs manifest.rs pack.rs tui.rs   (four files, no plan/ module)
literal reads at the tag, grepped then confirmed by running:
  src/manifest.rs:197  self.read("pack.toml")
  src/main.rs:181      source.read("principles.toml")
  occurrences of "instrument" in the whole of tag src/ :  0

*.plan.toml anywhere in the tag tree:  0 files
tag scaffolded tree (11 files):  AGENTS.md, .agents/{AGENTS.reference.md,
  principles.toml, prompts/*7}, docs/plans/TEMPLATE.md
```

### Gates I ran myself at HEAD

| gate | result |
| --- | --- |
| `render --check --strict docs/plans/agent-scaffold.plan.toml` | `up to date`, **exit 0**. Round 5's `T1` is fixed by `6050f93`. |
| `validate --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl` | EXACTLY ONE problem, the pre-existing `Q-43` `superseded by` one the specification's criterion 4 excludes |
| ASCII check on both changed files | 0 non-ASCII lines on each |

## Every deletion, what it carried, and where that fact now lives

The test in the last column is the one I was given: does its absence mislead or deprive a 0.0.1 user upgrading to 0.0.2. Not "was it true".

### The seven bullets deleted outright

| old | subject | what it carried | reachable by a 0.0.1 user? | where the fact lives now |
| --- | --- | --- | --- | --- |
| `:15` | `metrics_absent_reason` and the absent-reason token vocabulary on `status`/`next` `--json` | A closed kebab-case enum for a machine consumer of two JSON projections | **NO.** `status` and `next` both exit 2 at the tag, and `--json` exits 2. No 0.0.1 user has a projection to consume. | Nowhere in the section. `status` and `next` are named as new in the BREAKING bullet; `--help` and the README carry the detail. NOT NEEDED. |
| `:21` | optional `harness` on `reviewers[]` in the `round` record | A schema addition to the instrumentation round record, and that existing records stay valid | **NO.** No instrumentation exists at the tag: `--instrument` exits 2, `instrument.md` is never read, and there are no round records to keep valid. | Nowhere. NOT NEEDED. The bullet's own reassurance ("so existing round records stay valid") has no population at 0.0.1. |
| `:27` | `validate --workflow` now FAILS with no round log | A false-green CI break, and the population it breaks | **NO.** `validate` exits 2 at the tag. Its explicit "THE POPULATION THIS BREAKS" contains no 0.0.1 user. | Nowhere. NOT NEEDED as a delta from 0.0.1. |
| `:28` | `validate --workflow` REFUSES an unattributable log; `status`/`next` withhold | Two named breaks, both about `--metrics`/`--plan` pairings | **NO.** All three commands exit 2 at the tag. | Nowhere. NOT NEEDED. |
| `:29` | metrics log and ledger resolved from the PLAN SOURCE | A default-path change, "previously both defaults were relative to the current directory" | **NO.** No `--metrics`, no `--ledger-fragment`, no such defaults at the tag. | Nowhere. NOT NEEDED. See out-of-scope observation `O2` for what this bullet's removal exposes rather than causes. |
| `:39` | `render` no longer lets free TEXT control STRUCTURE (`F1`) | The four interpolation sites, the fabricated queue entry, and the visible effect on an existing plan | **NO.** `render` exits 2 at the tag, the tag's `src/` has no `plan/` module, and the tag ships zero `*.plan.toml`. No artifact a 0.0.1 user owns was produced by the defective renderer. | The FIX is nowhere. `render` itself IS named in the section (see `R1` below). CORRECT TO OMIT, ruled at `R1`. |
| `:40` | W5's waiver-ownership rule now decided from the ROUND LOG | A narrowing of two populations under a check added in the same cycle | **NO.** W5 was added and fixed inside the unreleased cycle; no W5 ever shipped. | Nowhere. NOT NEEDED, and consistent: the W5 ADD bullet survives at `:16` while its cycle-internal fix is cut. |

**Not one of the seven carried a fact a 0.0.1 user could act on.** Every one of them describes machinery that returns exit 2 on the binary I built from the tag.

### The five claims cut from `:36` (now `:32`) and `:37` (now `:33`)

| # | site | the cut claim | what it carried | verdict |
| --- | --- | --- | --- | --- |
| 1 | `:36` | "TWO pack-controlled fields reach that join, and all three shapes leaked through both" plus the whole `[[module]]`'s `guidance` clause | That a module guidance path also leaked, silently, with no plan line naming it | **NOT NEEDED, and it was false of 0.0.1.** Measured: at the tag a `[[module]]` section is ignored outright (`AGENTS.md` = `M:{{modules}}`, exit 0, nothing spliced) and `--module` exits 2. The fact survives for a 0.0.2 pack author in `README.md`, added by this same increment, which names "an `[[asset]]`'s `source`, a `[[module]]`'s `guidance`, and the `pack.toml`, `principles.toml` and `instrument.md` the tool reads by name". |
| 2 | `:36` | "so an escaping `guidance` reports as a module guidance problem and never as an asset `source` one" | That the refusal is labelled by field | **REDUNDANT with the surviving clause, which I measured true.** HEAD prints ``error: module `leaky` guidance file `g.md` is not a contained pack path ...`` and ``error: asset source `link.md` is not a contained pack path ...``. The surviving "Each caller labels the refusal with its own field" carries it. But the cut left a dangling "neither": see finding `L2`. |
| 3 | `:37` | "and `instrument.md`" from the literal-name list, and "the same leak as the two fields above on three more paths" | That `instrument.md` is a third literal-name read and is contained | **THE 0.0.1 HALF WAS FALSE and is correctly gone. THE 0.0.2 HALF IS TRUE AND IS NOW LOST.** Measured at the tag: a symlinked, an invalid-UTF-8 and a plain pack-internal `instrument.md` all give exit 0 and `I:{{instrument}}`, so 0.0.1 never reads it. Measured at HEAD with `--instrument`: ``error: could not read the pack's `instrument.md`: `instrument.md` is not a contained pack path ...``, exit 2. See finding `L1`. |
| 4 | `:37` | "or an empty instrumentation block" (two occurrences) and ", indistinguishable from a pack shipping neither" | That an unreadable file produced an empty instrumentation block, and why that was bad | **CORRECT TO CUT.** 0.0.1 produces no instrumentation block at all: it leaves `{{instrument}}` unsubstituted. The surviving principle-set half is measured true: invalid-UTF-8 `principles.toml` gives tag exit 0 with EXACTLY 0 stderr bytes and `P:`, HEAD exit 2 with "could not read the pack's principles.toml: stream did not contain valid UTF-8". The "indistinguishable" clause was rationale, not an action. |
| 5 | `:37` | the whole final sentence, "ABSENCE IS UNCHANGED and stays silent: a pack shipping neither file still yields no principles and an empty instrumentation block, byte for byte what 0.0.1 produced" | Two facts: that absence stays silent at exit 0, and that the bytes are identical | **ONE HALF FALSE, ONE HALF TRUE AND ACTIONABLE.** Measured, pack shipping neither file: tag exit 0, 0 stderr bytes, `P:\nI:{{instrument}}\n`; HEAD exit 0, 0 stderr bytes, `P:\nI:\n`. So absence IS unchanged in exit code and silence, and is NOT byte-identical for a pack carrying the placeholder. The true half is the disambiguator the surviving sentence now needs: see finding `L3`. |

## Verdicts

THREE valid findings, all `low`. No `critical`, no `high`, no `medium`.

| id | verdict | severity | class | site | one line |
| --- | --- | --- | --- | --- | --- |
| `L1` | valid | low | USER-FACING | `CHANGELOG.md:33` | The literal-name list reads as exhaustive and is now under-inclusive for 0.0.2: `instrument.md` is a third literal-name read and IS contained, measured. The increment's own `README.md` names all three. |
| `L2` | valid | low | TEXT | `CHANGELOG.md:32` | "and neither reports as a failed read" is left with no antecedent. The cut removed the pair that "neither" and "Each caller" referred to, leaving one caller in the bullet. |
| `L3` | valid | low | USER-FACING | `CHANGELOG.md:33` | Deleting the whole "ABSENCE IS UNCHANGED" sentence removed the only thing separating "an unreadable `principles.toml`" (now exit 2) from "no `principles.toml`" (still silent at exit 0). The surviving description of the old behaviour fits both. |

| ruling | subject | outcome |
| --- | --- | --- |
| `R1` | the undisclosed `render` fix (`F1`) | **CORRECT, and not a loss.** The implementer's premise is also wrong on the facts: the section does mention `render`. |

**ANSWER TO THE QUESTION I WAS ASKED: no deletion removed a fact a 0.0.1 user could act on.** All three findings concern a 0.0.2 pack author, a population that did not exist at the tag. I lowered no reviewer severity because I am the only reviewer on this lens.

## `R1` (ruling): the undisclosed `render` fix

The implementer flagged this against themselves and asked for a ruling. I rule the omission **correct**, and I correct the premise it was raised on.

### The premise is wrong on the facts

"The section now contains NO mention of `render`" is false. `render` appears three times in `CHANGELOG.md:7-35`:

```
:20  "It renders setup pointers into `AGENTS.md`'s `{{modules}}` block"
:21  "A `{{modules}}` render slot concatenates each enabled module's guidance"
:25  "The other subcommands, `validate`, `status`, `next`, `checks`, `render` and
      `audit`, are all new in this release; none of them exists in 0.0.1."
```

`:25` is the new BREAKING bullet, written by this same commit. So `render` is introduced to a 0.0.1 user by name, as new. What is undisclosed is `F1`'s FIX, not the command.

### `F1`'s population among 0.0.1 upgraders is empty, measured three ways

```
tag binary, `render --help`                        exit 2, "unexpected argument 'render' found"
tag src/                                           four files; no plan/ module, so no renderer at all
*.plan.toml in the tag tree                        0
tag scaffolded plan artefact                       docs/plans/TEMPLATE.md   (Markdown, no TOML source)
```

No artefact a 0.0.1 user owns was produced by the defective renderer, because 0.0.1 has no renderer and no structured source for one to project. A user who upgrades and scaffolds renders with the FIXED binary and never meets the defect. The reachable population is exactly zero.

### And listing it would have been the error, not the omission

Under Keep a Changelog, which `CHANGELOG.md:5` names as the format, closing the section makes every bullet a claim about the delta from 0.0.1. A `### Fixed` entry for `render` asserts that something a 0.0.1 user had was broken and is now repaired. `render` is new in 0.0.2. The entry would be false in its implication and would contradict `:25`, which says `render` does not exist in 0.0.1.

The section already applies this reasoning twice, and the cut applies it CONSISTENTLY rather than as a one-off:

- `:19` states it outright: `trivial` and `grandfathered` were added and "RETIRED in the same unreleased cycle", with the net effect on a 0.0.1 user stated as nothing.
- W5 is the exact parallel to `render`. Its ADD bullet survives at `:16`; its cycle-internal FIX bullet (`:40`) was cut. `render` gets the same treatment: named as new, its cycle-internal fix not described.

Had the cut kept `F1` while dropping `:40`, that inconsistency would be my finding.

### What I would change in the record

Record it as a **correct classification**, not as an accepted loss. "`F1`'s fix is cycle-internal, so it is not a delta from 0.0.1 and does not belong in the section" is the true statement. "We deliberately accepted losing `F1`" overstates it and would invite a future round to put it back.

One qualification I want in the record, because it is the only sense in which anything about `render` was lost. `:39` was the section's only description of what `render` does at all, so `render` is now announced as new with no description of its behaviour. That is a gap in what the section says about a NEW command, not a fact taken from a 0.0.1 user, and its larger form predates this cut. It is `O2` below.

## `L1` (low, USER-FACING): the literal-name list is presented as exhaustive and is now short by one

### The text

`CHANGELOG.md:33` opens:

> The files the tool reads by literal name, `pack.toml`, `principles.toml`, are contained too

The pre-cut form named three files and said "three". The cut removed `instrument.md` and the count, leaving a construction that still presents itself as the definitive list of the files the tool reads by literal name.

### What I measured

At the TAG, `instrument.md` is never read, in any shape. `tgt-001`, three packs, each with `AGENTS.md` = `P:{{principles}}\nI:{{instrument}}\n`:

```
instrument.md is a symlink out of the pack   tag  exit 0, stderr 0 bytes, AGENTS.md = "P:\nI:{{instrument}}\n"
instrument.md is invalid UTF-8, in the pack  tag  exit 0, stderr 0 bytes, AGENTS.md = "P:\nI:{{instrument}}\n"
instrument.md is a plain file in the pack    tag  exit 0, stderr 0 bytes, AGENTS.md = "P:\nI:{{instrument}}\n"
```

So deleting the 0.0.1 claim about `instrument.md` is right. But at HEAD it is read by literal name AND contained, which is what the sentence now denies by omission:

```
tgt-head, scaffold --template <pack> --instrument
  instrument.md -> outside      exit 2  "error: could not read the pack's `instrument.md`:
                                         `instrument.md` is not a contained pack path (it resolves
                                         outside the pack directory, through a symbolic link); ..."
  instrument.md invalid UTF-8   exit 2  "error: could not read the pack's `instrument.md`:
                                         stream did not contain valid UTF-8"
  instrument.md plain, in pack  exit 0  AGENTS.md = "P:\nI:INSIDE-PACK-INSTRUMENT\n"
```

`src/main.rs`, changed by this increment, is where it happens: `build_assets` now calls `read_optional("instrument.md")` and maps a failure to `LoadError::UnreadablePackFile`.

### The increment's own README contradicts the bullet

`README.md`, added in the same `git diff main..HEAD`, states the complete rule:

> an `[[asset]]`'s `source`, a `[[module]]`'s `guidance`, and the `pack.toml`, `principles.toml` and `instrument.md` the tool reads by name are each refused if the path is absolute, carries a `..` component, or lands outside the pack once symbolic links are followed

Five paths in the README, three in the CHANGELOG. The CHANGELOG's list is the odd one out, and it is the shorter one.

### A second, smaller symptom at the same site

"`pack.toml`, `principles.toml`, are contained too" is a two-item list joined by a comma with no conjunction, with a comma still sitting before the verb. It reads exactly like a list an item fell out of, which is what happened. A reader who notices it will suspect the sentence is damaged, which is a poor state for the one bullet in the section that enumerates a security boundary.

### Severity: `low`, ruled

Not `medium`. The affected reader is a 0.0.2 pack author who ships an `instrument.md` and runs `--instrument`. That population did not exist at 0.0.1 (`--instrument` exits 2 at the tag, measured), so round 2's `B2` bar, which requires an upgrader from the last release, is not met. The failure they meet is loud, exits 2, names the file and states the rule, and the README states the full boundary correctly.

Not below `low`. The direction of the error is UNDER-warning: the section says fewer files are covered by the boundary than are. Round 5's `T2` held five over-warning claims at `low` on the express ground that "nobody is left less careful than they should be". This one points the other way, so it does not get the benefit of that reasoning, and it stays at `low` only because the recourse is one loud message.

Principle 6 (Ground decisions in evidence) is engaged, mildly: the project now holds the measurement that `instrument.md` is contained, and the section says otherwise by omission.

### The fix

Two forms, both small. The first is what round 5's `E1` offered as its alternative and the pass did not take:

- Scope the second sentence instead of shortening the first. Restore `instrument.md` to the opening list, which is true of 0.0.2, and confine "In 0.0.1 each was read through a symbolic link out of the pack" to the two files that existed. This needs one authored clause.
- Or drop the exhaustive framing with a smaller edit, for example "Every file the tool reads by literal name, `pack.toml` and `principles.toml` among them, is contained too". No new fact is asserted, and the missing conjunction goes with it.

I prefer the second. It is nearer a deletion, and this loop has now gone many pure-deletion sites without one producing a finding.

## `L2` (low, TEXT): "neither" has nothing left to refer to

`CHANGELOG.md:32` now ends:

> Each caller labels the refusal with its own field, and neither reports as a failed read, since nothing was opened.

Pre-cut, "Each caller" and "neither" both pointed at the pair the bullet had just described, an `[[asset]]`'s `source` and a `[[module]]`'s `guidance`. The cut removed the guidance half. The bullet now names exactly one caller, so "Each caller" governs one item and "neither" has no antecedent at all.

The FACT is intact and I measured it: HEAD's refusals are labelled per field (`asset source ...`, ``module `leaky` guidance file ...``) and neither is reported as a failed read, because nothing is opened. Nothing is false. What is left is a sentence that tells the reader two things were being discussed while the bullet mentions one, which is the reader's clearest signal that text was removed here.

Severity `low`, on round 4's `P4` precedent: nothing false is published, and no behaviour is misdescribed. The fix is a pure deletion of the word: "Each caller labels the refusal with its own field, and a refusal never reports as a failed read, since nothing was opened."

## `L3` (low, USER-FACING): the surviving description of the old behaviour fits both the changed case and the unchanged one

### What the bullet now says

> A file the tool cannot read (invalid UTF-8, or one it lacks permission to read) produced an empty principle set at exit 0 with empty stderr in 0.0.1; it now exits 2 naming the file

### Why that is now ambiguous, measured

A pack that ships NO `principles.toml` also produced an empty principle set at exit 0 with empty stderr in 0.0.1, and still does at 0.0.2:

```
pack shipping neither principles.toml nor instrument.md:
  tgt-001   exit 0   stderr 0 bytes   AGENTS.md = "P:\nI:{{instrument}}\n"
  tgt-head  exit 0   stderr 0 bytes   AGENTS.md = "P:\nI:\n"

pack shipping an invalid-UTF-8 principles.toml:
  tgt-001   exit 0   stderr 0 bytes   AGENTS.md = "P:\n..."
  tgt-head  exit 2   "error: could not read the pack's principles.toml:
                      stream did not contain valid UTF-8"
```

So the outcome the bullet uses to identify the OLD behaviour, "an empty principle set at exit 0 with empty stderr", is produced by two different situations at 0.0.1, and only one of them changes at 0.0.2. The deleted sentence, "ABSENCE IS UNCHANGED and stays silent", was the only thing in the section that told them apart. A pack author who recognises their own symptom in the surviving sentence cannot tell from it which population they are in.

The parenthetical "(invalid UTF-8, or one it lacks permission to read)" does narrow it, which is why this is `low` and not higher. It names the trigger even though the outcome description does not distinguish.

### The half that was false is correctly gone

"byte for byte what 0.0.1 produced" is false for a pack carrying the `{{instrument}}` placeholder, measured above: 0.0.1 leaves `{{instrument}}` unsubstituted and HEAD substitutes it to empty. Deleting that half was right. The defect is that the true half went with it.

The increment's own code states the boundary the section dropped. `src/main.rs`, in this diff:

> `/// Why the active pack's principle set could not be resolved. ABSENCE is not in here: a pack that ships no `principles.toml` has no principles to select, which is an empty set rather than a failure.`

The commit is `9d4d6e2`, "fix: report a refused pack literal instead of calling it absent". Distinguishing a refusal from an absence is the entire point of the increment's last code change, and the section no longer states the absence side of that distinction.

### Severity: `low`, ruled

Round 4's `P4` precedent governs: this is an OMISSION, nothing false is published, and the reader can settle it with one command. No 0.0.1 user is deprived of an action, because absence behaves identically on both binaries. It is an honest candidate for a recorded residual if the human prefers to stop cutting.

The fix is one short sentence made only of figures measured here, for example: "A pack that ships no `principles.toml` is unchanged: no principles, exit 0, silent."

## `### Deprecated` and the section structure

Clean. No heading is empty and none is orphaned.

```
## [0.0.2] - 2026-08-13 / ### Deprecated  ->  1 bullet
## [0.0.2] - 2026-08-13 / ### Added       ->  7 bullets
## [0.0.2] - 2026-08-13 / ### Changed     ->  4 bullets
## [0.0.2] - 2026-08-13 / ### Fixed       ->  3 bullets
## [0.0.1] - 2026-07-10 / ### Added       ->  7 bullets
```

- `### Deprecated` keeps its single rename bullet, untouched by the cut.
- `### Added` lost two of nine and keeps seven. `### Changed` lost three of six, gained the BREAKING bullet, and keeps four. `### Fixed` lost two of five and keeps three, two of them rewritten.
- No heading was left with zero bullets, so no `### Removed` or `### Security` style orphan was created, and none existed before.
- Every heading is preceded and followed by a blank line, so the Markdown still parses as Keep a Changelog: `## [version] - date` then `### <change type>` then a bullet list.
- The file has no reference-style link definitions at the bottom and never had any, so the cut could not have orphaned one. Nothing in `tests/`, `src/` or the `justfile` reads `CHANGELOG.md`, so no drift guard is affected.
- The heading ORDER is Deprecated, Added, Changed, Fixed, rather than Keep a Changelog's own listing order. That is unchanged by this cut and predates it, and Keep a Changelog does not mandate an order. Not a finding.

## Does the section still describe everything user-visible in `git diff main..HEAD`?

`git diff main..HEAD` is 12 files, 1724 insertions, 164 deletions, over eleven commits. Both directions checked.

### Something described that did not happen: nothing found

The new BREAKING bullet is the only text this commit added, so I verified every claim in it against both binaries:

```
"each of those now exits 2 with Usage: agent-scaffold <COMMAND>"
  tag  `--output-dir <dir> --vcs none --write`  exit 0, 11 files written
  HEAD same invocation                          exit 2, "unexpected argument '--output-dir'"

"every option 0.0.1 documented is still accepted there"
  the nine flags the ## [0.0.1] section advertises, plus --output-dir, all appear in
  HEAD's `scaffold --help`: --principles --principle-detail --list-principles --write
  --dry-run --force --template --var --vcs --output-dir     10 of 10

"validate, status, next, checks, render and audit are all new ... none exists in 0.0.1"
  HEAD  all six `--help` exit 0        tag  all six exit 2
```

All true. I also re-measured the three surviving `### Fixed` bullets' subjects against the tag and each holds:

```
[[asset]] source = "../outside/secret.md"   tag exit 0, leaked.md = TOP-SECRET-OUTSIDE-THE-PACK
[[asset]] source = "link.md" -> outside     tag exit 0, same
  HEAD refuses both at exit 2, each message naming the value and the specific cause
```

### Something that happened and is not described

Four candidates in `main..HEAD`, three of them covered:

| change in `main..HEAD` | user-visible? | described? |
| --- | --- | --- |
| `dba2caa` `dest` containment | yes | yes, `:34` |
| `e04f096`, `65b8c0d`, `007886d` pack source containment | yes | yes, `:32` |
| `9d4d6e2` a refused pack literal is reported, not called absent | yes | yes for `principles.toml` at `:33`; NOT for `instrument.md`, which is `L1` |
| `1814647` `F1`, `src/plan/render.rs` +187 | reachable only from 0.0.2 | no, and correctly so, ruled at `R1` |
| `--template` root message, `src/main.rs` | yes | RECORDED RESIDUAL, not raised |
| `src/plan/source.rs` sidecar predicate moved to `safe_path::is_contained_relative` | no behaviour change | see below |

On the sidecar predicate: I compared the old and new bodies and they are the same rule. The old `is_safe_sidecar_ref` was `!path.is_absolute() && all components are Normal | CurDir`; `safe_path::is_contained_relative` at `src/safe_path.rs:54-60` is character-for-character that predicate. So the surviving `:34` claim that this is "the same rule `validate --source` already applied to a plan's `[meta].sidecars` refs" is true, and no undescribed behaviour change hides in that move.

Nothing else in `main..HEAD` reaches a user undescribed.

## Out-of-scope observations

Real, reproduced, and NOT findings. None was created by the cut under review, and each falls outside what I was asked to rule on.

### `O1`: a 0.0.1 user's hand-edited `docs/plans/TEMPLATE.md` is destroyed by the first 0.0.2 `scaffold`, silently, at exit 0

This is the largest undisclosed upgrade consequence I found. It is out of scope because `git diff main..HEAD` does not touch `pack/`, so this increment did not cause it and the cut did not remove any description of it.

```
tag binary scaffolds into an empty dir, then I append a comment to
docs/plans/TEMPLATE.md, then HEAD's `scaffold --output-dir <same> --vcs none --write`:

  before my edit    md5 cc84f74ec07a4d387e91ba7ccc7d22d7
  after my edit     md5 8f7d11411cac0cdbc1410ea10cffa07b
  HEAD run prints   "render  docs/plans/TEMPLATE.md" ... "(29 changed, 1 left untouched)"
  after the run     md5 ed298f0465e8a321825d962b7aaba924
  my comment        GONE
```

At the tag, `docs/plans/TEMPLATE.md` is declared `ownership = "working"` in `pack/pack.toml`, so it is protected. At HEAD it is not a manifest asset at all: HEAD's `pack.toml` says "the generated view is not a manifest asset; it is regenerated by `render`". A hand-edited `AGENTS.md` survives the same run; `docs/plans/TEMPLATE.md` does not. The `## [0.0.1]` section, three lines below, advertises "Write safety ... leaves user working files untouched", which is the promise this breaks for that path.

### `O2`: the section never says the plan is now a TOML source projected into Markdown, and the cut removed its last incidental traces

Term counts I measured in the 0.0.2 section, before and after the cut:

```
"TOML"            pre-cut 3   post-cut 0
"docs/metrics"    pre-cut 2   post-cut 0
"workflow.jsonl"  pre-cut 1   post-cut 0
"--source"        pre-cut 5   post-cut 1   (the survivor is about the sidecar rule)
"plan.toml"       pre-cut 0   post-cut 0
```

Measured, the same upgrade run as `O1` creates twenty files a 0.0.1 user has never seen, including `docs/plans/TEMPLATE.plan.toml`, ten prose sidecars, `.agents/workflow.toml`, `.agents/LEDGER.template.md` and six `.agents/user-prompts/`. The section describes the ledger move in `:26` and nothing else of it.

**This is NOT a finding against the cut.** The pre-cut mentions were all incidental, inside bullets round 5 ruled vacuous, and the section never stated the fact properly in any revision. The cut removed a poor vehicle for a fact that was never carried. It is the residue of round 5's `T3`, which the new BREAKING bullet closes for the command line and not for the artefact model. If the human wants it closed, the cheap form is one clause in the existing BREAKING bullet, not a restored bullet.

### `O3`: `{{instrument}}` renders differently on a pack authored for 0.0.1

Measured above: a pack whose `AGENTS.md` carries `{{instrument}}` gets the literal `{{instrument}}` at the tag and an empty string at HEAD. A 0.0.1 pack author had no reason to write that placeholder, since 0.0.1 does not compute it, so the population is close to empty. Recording it because it is the measurement that makes the deleted "byte for byte" claim false, and a future round may want the figure without re-deriving it.

## The round outcome, from my lens

**NOT CLEAN, at `low` only.** Three valid findings, all `low`, all at the two bullets the pass rewrote rather than deleted. Zero at any of the seven pure deletions.

That result is worth stating plainly, because it is the fifth consecutive round to produce it. Every deletion in this pass was correct: the seven deleted bullets describe machinery that returns exit 2 on the binary I built from the tag, and the deleted claims within `:36` and `:37` were false of 0.0.1 in every case I measured. All three of my findings are at text the pass REWROTE. `L1` and `L3` are both a true fact removed alongside a false one in the same sentence, and `L2` is a pronoun left pointing at deleted text. Round 4's instruction, that pure deletion is safe and authored replacement is where defects come from, holds again, with one refinement this round supplies: cutting PART of a sentence is not a pure deletion, and it behaves like authoring.

None of the three blocks the release on my lens. All three are honest candidates for recorded residuals under round 4's `P4` test, though `L1` is the one I would fix, because its direction is under-warning and its fix is nearly a deletion.
