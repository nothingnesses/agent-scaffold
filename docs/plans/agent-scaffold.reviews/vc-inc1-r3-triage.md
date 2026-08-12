# `validation-constraints-inc1`, round 3: triage

Triager worktree: `.claude/worktrees/tri-inc1-r3`, branch `triage/inc1-r3`, at `0202df6`.
Artifact: `git diff main..HEAD` in that worktree, three commits (`2435067` the implementation, `e0406bc` the round 1 fix pass, `0202df6` the round 2 fix pass, which is the shortening), touching `.agents/AGENTS.reference.md`, `AGENTS.md`, `CHANGELOG.md`, `pack/instrument.md`, `src/plan/source.rs`, `src/workflow.rs`. The reviewers name the same three commits by their own rebased hashes (`0110828`, `86e00ed`, `651ff63`); the trees are identical.
Specification: `docs/plans/agent-scaffold.steps/validation-constraints.md`, increment 1 and the Acceptance section.
Findings adjudicated: `vc-inc1-r3-reviewer-claims.md` (`W3A-*`, two) and `vc-inc1-r3-reviewer-mutate.md` (`W3B-*`, two).
Settled and read in full before starting: `vc-inc1-r1-triage.md` (five valid, one duplicate, one dismissed) and `vc-inc1-r2-triage.md` (nine valid, two duplicates, none dismissed).

EVERY MEASUREMENT BELOW WAS TAKEN BY ME, IN MY OWN TREES AND FIXTURES. Nothing is carried from a reviewer's report on trust. Where my figure or my reading differs from a reviewer's, I say so.

## Verdict table

| id | verdict | severity | ground |
| --- | --- | --- | --- |
| `W3B-1` | VALID | `medium` | Rebuilt and reproduced. The declined "report only when the log is non-empty" build passes 386 of 386 tests, clippy and the live plan, while returning `workflow invariants hold` at exit 0 on three trees the shipped binary refuses, one of which the PRE-FIX binary also refuses. Round 1's own remedy caused it, by SUBSTITUTING the only fixture that held the empty-log axis. TEST-COVERAGE GAP over correct code. |
| `W3B-2` | VALID | `low` | Both mutations rebuilt and reproduced: `N8` and `N10` each pass 386 of 386 while dropping the second problem from a two-problem report. Verdict-neutral, measured on my own fixtures. TEST-COVERAGE GAP over correct code. |
| `W3A-1` | VALID | `low` | Reproduced: `plan-fold` is an increment id that does not end `-inc<alnum>` whose `leading_slug` output EQUALS the step slug, and the pre-fix binary accepts the waiver at exit 0, so the test comment's "it can never equal the step slug" is false by that class. In-code comment only. |
| `W3A-2` | VALID | `low` | Reproduced: the shipped rule text justifies the derived mark with "such a step is one the join computed rather than one the log carries", and on my `g9` fixture the marked step `gamma` is carried verbatim by the log as a structured `step` id. Written BY the shortening, replacing a hedged claim that was true. |

FOUR VALID FINDINGS, NO DUPLICATE, NONE DISMISSED. Ceiling `medium`, carried by `W3B-1` alone. NO `high` AND NO `critical` FINDING WAS RAISED OR FOUND, and I dismissed nothing, so NO BACKSTOP RE-CHECK IS OWED on this round.

NOT ONE OF THE FOUR IS A DEFECT IN WHAT THE TOOL DOES. Two are test-coverage gaps over correct code and two are claim defects. That is the third round to reach that result, by a fifth and sixth independent lens.

## Trees, binaries and fixtures

All under `<scratch>/tri-r3`, a subdirectory of my own naming. `TMPDIR` pointed at `<scratch>/tri-r3/tmp`, outside every git repository, per the Acceptance preamble. `src/` was mutated ONLY in scratch copies extracted with `git archive`; my worktree carries this file and nothing else (`git status --porcelain` is empty apart from it).

ONE `CARGO_TARGET_DIR` PER BINARY, nine binaries, all nine verified distinct (`md5sum ... | awk '{print $1}' | sort | uniq -d` returns nothing):

```
82fa3e0c2aa4380751f6cbaa4fb54842  target-head/debug/agent-scaffold       (git archive HEAD, 0202df6)
bf195003d95a96b98f63f9e42d736246  target-prefix/debug/agent-scaffold     (git archive main)
236f02c5945ea273db312439cb70f967  target-m-rh4rep/debug/agent-scaffold   (report only when the log is non-empty)
09a872e567a478f22a765926aed52e82  target-m-guard/debug/agent-scaffold    (HEAD plus the proposed assertion)
30c45d4408586fba2ec30890368a5edc  target-m-rh4guard/debug/agent-scaffold (both)
411996d50f48ba6d1aaa9e361e80a4ae  target-m-n8/debug/agent-scaffold       (evidence join skipped when an earlier problem exists)
17566a8ac84077737a9a0e4dfda6abf2  target-m-n10/debug/agent-scaffold      (ownership arm skipped when the step is not a Roadmap step)
f1dbbec1e73b6df57663b89209c1400e  target-m-b5/debug/agent-scaffold       (the `escalation.task != evidence` clause dropped)
3a144a04d1a895c7ade205f19da2b9b0  target-m-c4/debug/agent-scaffold       (W3's step-level exemption drops its `unit == Step` check)
```

Each mutation was applied with the file editor against the exact quoted line and then re-diffed against the pristine `head` tree, so a stale anchor could not silently rebuild the pristine source. The four production mutations are one line each and are quoted at their findings below.

Suite results, `cargo test --bins`:

```
head       386 passed; 0 failed
prefix     378 passed; 0 failed
m-rh4rep   386 passed; 0 failed
m-guard    386 passed; 0 failed
m-rh4guard 385 passed; 1 failed  (w5_flags_an_increment_waiver_whose_increment_has_no_round_records)
m-n8       386 passed; 0 failed
m-n10      386 passed; 0 failed
m-b5       386 passed; 0 failed
m-c4       386 passed; 0 failed
```

`cargo clippy --all-targets -- -D warnings` on `m-rh4rep`: exit 0. So the declined build passes the lint as well as the suite.

Live plan and log, unmodified, in my worktree (`validate --source docs/plans/agent-scaffold.plan.toml --workflow`): `workflow invariants hold`, exit 0, on `head`, on `prefix`, on `m-rh4rep` and on `m-b5`. Acceptance item 2 holds and, as the item itself says, demonstrates nothing on its own: three of those four binaries differ in behaviour.

Acceptance item 7b, re-run in my worktree:

```
grep -c -F "the increment's leading slug equals the step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md          -> 0, 0, 0
grep -c -F "some `round` record must join that increment to that step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md -> 1, 1, 1
```

The `type: "waiver"` bullet is byte-identical across the three copies (`grep '^- `type: "waiver"`' <file> | md5sum` returns `e74cc93448b5baecdf7de5d057e93af9` for all three).

Eleven fixtures, each its own project root with `docs/plans/t.md` (a Roadmap table plus the Step Detail headings the plan-structure check requires, so a fixture's exit code reports the workflow verdict and nothing else) and `docs/metrics/workflow.jsonl`. Command in every case: `agent-scaffold validate --plan <root>/docs/plans/t.md --workflow`, paths elided to `<root>` below. Every round record is schema-complete and every increment's streak is consistent, so the only problems any fixture reports are the ones it was built for.

```
L0       no `type:"round"` record at all; increment waiver `alpha-inc1` on step `alpha`, record-backed and scoped
L6       the same, plus one round record for a DIFFERENT increment
L0fold   the same as L0 for `alpha-fold`, an id `leading_slug` leaves unstripped
g9       one record declares `gamma` for ANOTHER increment; one increment-only record whose `task` is `gamma-inc1`
fderiv   one pre-migration record whose `task` is `alpha-fold`, so the owner is derived and occurs nowhere else
g12      increment id EQUALS the step slug (`plan-fold`), records join it there
g12b     an unstripped id that is NOT the step slug (`plan-fold` on step `plan`)
b5       step-unit record-backed waiver citing evidence `no-such-pointer`, with a scoped `decision` escalation present
n8       increment waiver mis-scoped AND citing a pointer no escalation carries (two problems)
n10      increment waiver naming a ghost step AND mis-scoped (two problems)
c4       `complete` step with no records of its own, plus an increment-unit waiver naming it
```

## `W3B-1`: the declined form now passes the whole suite, because round 1's remedy substituted the fixture that caught it

VALID, `medium`. The reviewer's rating is confirmed and the reasoning for it is re-derived below rather than inherited. THIS IS THE ROUND'S REAL RESULT AND THE FIRST MATTER I WAS ASKED TO SETTLE.

THE DECISION. Receipt `Q-70-emptycase` presented three options and the human chose the first:

```
jq -r 'select(.type=="decision" and .q_id=="Q-70-emptycase") | [(.options|join(" | ")), .chosen] | @tsv' docs/metrics/workflow.jsonl
Report it | Stay silent on it | Report it, but only when the log is non-empty     ->  Report it
```

THE MUTATION, `src/workflow.rs:625`, one added clause, applied to a scratch copy of HEAD:

```
-				if !rounds.iter().any(|round| waiver_covers_round(waiver, round)) {
+				if !rounds.is_empty() && !rounds.iter().any(|round| waiver_covers_round(waiver, round)) {
```

That build implements the third option verbatim: it reports the unobserved case when the log carries some other increment's records and stays silent when the log carries no readable `type:"round"` record at all.

THE SUITE, THE LINT AND THE LIVE PLAN ALL STAY GREEN, measured by me:

```
cargo test --bins                          -> 386 passed; 0 failed
cargo clippy --all-targets -- -D warnings  -> exit 0
validate --source docs/plans/agent-scaffold.plan.toml --workflow  -> workflow invariants hold, exit 0
```

FALSE GREEN AT THE COMMAND LEVEL, on my own fixtures:

```
L0      head      exit=1  round log line 2: increment waiver names increment `alpha-inc1`, which no `type:"round"` record resolves to (by its structured `increment` id, else its `task`; a record the schema check rejected is not read), so the round log joins it to no step
        m-rh4rep  exit=0  workflow invariants hold
L0fold  head      exit=1  ... increment waiver names increment `alpha-fold`, which no `type:"round"` record resolves to ...
        prefix    exit=1  round log line 2: increment waiver names step `alpha` but increment `alpha-fold` belongs to step `alpha-fold`
        m-rh4rep  exit=0  workflow invariants hold
L6      head      exit=1  (the same refusal, log non-empty)
        m-rh4rep  exit=1  (the same refusal, so the mutant is exactly the declined option and not a broader break)
```

`L0fold` IS THE STRONGEST TREE AND THE REVIEWER DID NOT RUN IT. There the mutant accepts a tree that BOTH the shipped binary AND the pre-fix binary refuse, so this is a regression against the shipped tool and not only against the fix. That is the same ground round 1 gave for rating `W1A-1` `medium`, and I re-establish it here rather than borrowing it.

THE CAUSE IS ROUND 1'S OWN REMEDY, AND I VERIFIED IT IN THE HISTORY RATHER THAN ACCEPTING THE NARRATIVE:

```
git show 2435067:src/workflow.rs | grep -A 22 "fn w5_flags_an_increment_waiver_whose_increment_has_no_round_records"
   1570:  let problems = w5_problems(&waivers(&waiver), &steps, &[], &escalations);

git show e0406bc:src/workflow.rs | grep -A 30 "fn w5_flags_an_increment_waiver_whose_increment_has_no_round_records"
   1648:  let other = rounds(&owning_round_line("alpha", "alpha-other"));
   1649:  let problems = w5_problems(&waivers(&waiver), &steps, &other, &escalations);
```

At the implementation commit the fixture was an EMPTY rounds slice, and round 1's mutation battery recorded `m6`, "report only when the log is non-empty (the option the human DECLINED)", as CAUGHT by exactly that test (`vc-inc1-reviewer-behaviour.md:128`). Round 1's triage remedy then read: "give the FIRST assertion a non-empty log that lacks the waived increment" (`vc-inc1-r1-triage.md:109`). That is a SUBSTITUTION, not an addition, and it was correct for the increment axis it aimed at. Nothing kept the empty case beside it, and the empty slice was the only fixture that could distinguish the decided form from the third option.

NO OTHER TEST COVERS THE AXIS, measured rather than assumed. Eight `w5_problems` call sites still pass an empty rounds slice (`:1368`, `:1427`, `:1436`, `:1444`, `:1534`, `:1547`, `:1976`, `:1989`; my figure is eight where the reviewer says nine, because one of the lines they counted carries its `&[]` in the escalations argument). Every one of the eight carries a STEP-unit waiver, which never reaches the ownership arm, and the mutant's own green suite proves the point independently: an increment-unit waiver with an empty rounds slice anywhere in the suite would have reddened it.

CODE DEFECT OR TEST GAP: TEST GAP, STATED PLAINLY. `w5_problems` as shipped implements the decided reporting form on every tree I ran, `L0`, `L0fold` and `L6` included. No line of `src/` needs to change.

IS THE REMEDY ONE ASSERTION, AS CLAIMED? YES, AND I BUILT BOTH HALVES RATHER THAN ASSERTING IT. I added one assertion to `w5_flags_an_increment_waiver_whose_increment_has_no_round_records`, passing `&[]` and asserting the same message:

```
m-guard     (HEAD + the assertion)     -> 386 passed; 0 failed
m-rh4guard  (m-rh4rep + the assertion) -> 385 passed; 1 failed
   workflow::tests::w5_flags_an_increment_waiver_whose_increment_has_no_round_records
   panicked at src/workflow.rs:1648:9: assertion `left == right` failed: []  left: 0  right: 1
```

So the fix costs nothing on a correct build and reddens the declined build at the new assertion. One assertion is enough.

WHERE THE ASSERTION SHOULD LIVE, WHICH IS A JUDGEMENT THE REVIEWER LEFT OPEN AND I RULE. PREFER A SIBLING TEST. The existing test's comment ends "THE LOG IS NON-EMPTY AND SIMPLY LACKS THIS INCREMENT, which is what makes the case the INCREMENT axis rather than an absent log" (`:1627-1631`). An empty-log assertion inside that test contradicts its own comment unless the comment is reworked in the same pass, and a comment that tells a maintainer the other case does not exist is precisely round 2's `W2A-4` defect, which this loop has already paid for once. A sibling test named for the empty log keeps each comment true of its own body. Either form is acceptable if the comment moves with the assertion; only the silent form is not.

WHY `medium` AND NOT `low`. Severity is absolute impact if left unfixed, and the four-level scale rates the finding rather than the mutation used to expose it. Left unfixed the shipped tool is right on every tree I ran; the exposure is that a later edit implementing a form the human explicitly declined ships green with `workflow invariants hold` at exit 0 over a waiver nothing evidences. This project's own calibration draws the `low`/`medium` line at whether the unpinned thing is VERDICT-bearing: round 2 rated `W2A-2` `low` on the measured ground that "a defect that cannot change a verdict does not reach `medium`" (`vc-inc1-r2-triage.md:422`), and round 1 rated `W1A-1` `medium` where the verdict moved. This one moves the verdict on three of my trees, so it sits on the `medium` side of that line, and on `L0fold` it moves it below the pre-fix tool.

WHY NOT `high`. No live verdict moves, the shipped tool is correct today, and the affected population is a repository whose log carries no readable `type:"round"` record at all. It is narrower than `W1A-1`'s, which is why I do not rate it above `medium`.

DOES IT LEAVE ACCEPTANCE ITEM 4 UNMET? PARTLY, AND THE PRECISE ANSWER MATTERS BECAUSE THE ITEM'S OWN WORDING AND THE RECEIPT DISAGREE ON HOW MANY FORMS THERE ARE. Item 4 (`validation-constraints.md:122`) closes "A test that does not distinguish the two forms has not pinned the decision", and its two forms are the step's own fork, REPORT versus SILENT. The suite still distinguishes those two: `RH4sil`, the silent build, is red at that test, which I take from the reviewer's table and did not need to rebuild, since the surviving mutation is the other one. So the item's literal clause is MET. What is not met is the item's stated purpose, "THE UNOBSERVED CASE IS PINNED AS A DECISION", over the population the item itself defines ("an increment-unit waiver whose increment has NO round records anywhere"): the decision the item points at is now `Q-70-emptycase`, whose receipt presented THREE options, and the suite distinguishes the chosen form from only one of the two declined ones. HALF-MET is a fair label; the half that is missing is the half the receipt added after the item was written.

THE GENERAL LESSON, RECORDED BECAUSE THE BRIEF ASKS FOR IT AND BECAUSE THIS IS THE SECOND ROUND IN A ROW WHERE A FIX PASS WROTE THE NEXT ROUND'S FINDING. A remedy that SUBSTITUTES an existing fixture retires whatever that fixture was the only pinner of, and neither the triager who wrote round 1's remedy nor the implementer who applied it asked what the old fixture pinned. The rule this loop should adopt, and which I have applied to my own remedies below: A REMEDY THAT CHANGES AN EXISTING FIXTURE MUST SAY WHAT THAT FIXTURE CURRENTLY PINS AND WHICH OTHER TEST PINS IT AFTERWARDS; where nothing else does, the remedy ADDS a case rather than replacing one. Round 1's diagnosis was right ("the only empty-case test passes an EMPTY rounds slice, so it cannot distinguish 'no records for this increment' from 'no records at all'"), and the correct conclusion from that same sentence was that the test then needed BOTH fixtures, not the other one.

REMEDY, SCOPED TO THE CLASS (the empty-log axis of the decided reporting form is pinned on neither consumer):

- `src/workflow.rs:1632-1650`, `w5_flags_an_increment_waiver_whose_increment_has_no_round_records`: ADD an assertion, or preferably a sibling test beside it, passing an EMPTY rounds slice and asserting the same empty-owners message. If it lands inside the existing test, its comment at `:1627-1631` must gain a clause saying the empty log is asserted too, so the comment stays true of its own body. Verified above: one assertion, `m-guard` green, `m-rh4guard` red.
- `src/workflow.rs:1635`, the non-empty `other` fixture round 1 added: NO EDIT, AND DO NOT REVERT IT. It pins the increment axis and is round 1's own remedy; the two fixtures pin different axes and the test needs both.
- `src/workflow.rs:625`, the `!rounds.iter().any(...)` guard: NO EDIT. Correct as shipped on `L0`, `L0fold` and `L6`.
- `src/workflow.rs:583-586`, the `w5_problems` bullet's empty-case sentence: NO EDIT. It states the reporting form unconditionally, which is what the code does.
- The other eight empty-rounds call sites (`:1368`, `:1427`, `:1436`, `:1444`, `:1534`, `:1547`, `:1976`, `:1989`): NO EDIT. All eight carry step-unit waivers, which never reach the ownership arm, so an empty slice is the right fixture for each.
- FOR THE POST-MERGE PLANNER, not a finding against this artifact: acceptance item 4's "two forms" wording predates `Q-70-emptycase`'s three-option receipt, so an item that pins the decision needs to name the third option too. The sidecar is known stale and I do not raise it; this is the fourth fact recorded for that pass.

## `W3B-2`: the new ownership arm is never exercised beside another W5 check on the same waiver

VALID, `low`. Both mutations rebuilt.

W5 runs four per-waiver checks in one loop: the Roadmap-step check, the NEW round-log ownership check, the record-backed evidence join, and the reason/tier pairing. NO TEST ASSERTS MORE THAN ONE PROBLEM ALONGSIDE THE OWNERSHIP MESSAGE, which I established directly rather than from the mutation alone: `grep -n "problems.len(), [2-9]" src/workflow.rs` returns nothing, and every assertion carrying an ownership message reads `problems[0]` (`:1607`, `:1640`, `:1695`, `:1726`, `:1746`, `:1774`, `:1809`, `:2167`). The one test that does exercise two W5 checks on one waiver, `w5_flags_each_inconsistent_reason_tier_pairing`, uses a STEP-unit waiver, so the ownership arm never runs in it.

MUTATION `N8`, the evidence join suppressed when an earlier problem exists:

```
-			if waiver.evidence_tier == EvidenceTier::RecordBacked {
+			if waiver.evidence_tier == EvidenceTier::RecordBacked && problems.is_empty() {
```

386 passed, 0 failed. My fixture `n8`:

```
head  exit=1  round log line 2: increment waiver names step `alpha` but the round log joins increment `alpha-inc1` to step `beta`
              round log line 2: `record-backed` waiver cites evidence `no-such-pointer` but no `type:"escalation"` record with `human_decision` `decision` is scoped to this waiver's unit
m-n8  exit=1  round log line 2: increment waiver names step `alpha` but the round log joins increment `alpha-inc1` to step `beta`
```

MUTATION `N10`, the ownership arm suppressed by a failed Roadmap-step check:

```
-		if waiver.unit == WaiverUnit::Increment {
+		if waiver.unit == WaiverUnit::Increment && slugs.contains(waiver.step.as_str()) {
```

386 passed, 0 failed. My fixture `n10`:

```
head   exit=1  round log line 2: `type:"waiver"` names step `ghost`, which is not a Roadmap step
               round log line 2: increment waiver names step `ghost` but the round log joins increment `alpha-inc1` to step `beta`
m-n10  exit=1  round log line 2: `type:"waiver"` names step `ghost`, which is not a Roadmap step
```

BOTH ARE VERDICT-NEUTRAL, MEASURED. The tree is refused at exit 1 under `head` and under both mutants; what the mutants lose is the SECOND reason, so an author fixes one fault, re-runs, and meets the next one instead of both at once.

IN SCOPE THOUGH THE MUTATED LINES ARE CONTEXT. Neither `if waiver.evidence_tier == EvidenceTier::RecordBacked {` nor `if waiver.unit == WaiverUnit::Increment {` is a `+` line, so out-of-scope condition 2 holds for them. Condition 3 FAILS, and it is the one that does the work: the behaviour the mutations remove is the NEW arm's own contribution to the report in `N10`, and in `N8` it is the new arm's interaction with its sibling; the missing fixture is one that exercises the new arm. The subject is not independent of inc1's review question, so the finding is in scope. That is the same reasoning round 2 applied to nine findings and I reach it the same way.

WHY `low`. No verdict moves anywhere, and neither mutant makes a shipped message FALSE. That is exactly where round 2 set its `low` bar (`vc-inc1-r2-triage.md:422`), and rating this `medium` would contradict a calibration this loop set two rounds ago on weaker facts.

REMEDY, SCOPED TO THE CLASS (nothing pins the new arm's independence from its three siblings):

- `src/workflow.rs`, beside `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment` (`:1581-1611`): ADD ONE fixture in which a single increment-unit waiver names a step the Roadmap does not carry AND whose increment the log joins elsewhere AND whose evidence joins to nothing, asserting all three problems. One fixture reddens `N8` and `N10` together. My `n8` and `n10` roots are worked instances of the two halves.
- `src/workflow.rs:603-704`, `w5_problems`: NO EDIT. Correct as shipped on both fixtures.
- `src/workflow.rs:1418-1450`, `w5_flags_each_inconsistent_reason_tier_pairing`: NO EDIT. It pins the evidence join beside the pairing on a step-unit waiver and keeps its subject; the new case is an addition, not a change to it.
- The existing single-problem ownership assertions (`:1604`, `:1637`, `:1693`, `:1724`, `:1744`, `:1772`, `:1807`, `:2165`): NO EDIT. `problems.len() == 1` is the right assertion for a fixture built to raise one problem, and weakening them would lose what they pin.

## `W3A-1`: the unblocking test's comment infers a property of the retired rule that is false by one class

VALID, `low`.

THE CLAIM, `src/workflow.rs:1653-1658`, added by `2435067` and untouched by both fix passes (`git log --oneline -S"it can never equal the step slug" -- src/workflow.rs` returns exactly `2435067`, and the line is a `+` line in `git diff main..HEAD`):

```
// THE UNBLOCKING (Q-70). This is the shape the retired lexical rule made
// unwritable: an increment id that does not end `-inc<alnum>`, so
// `leading_slug` returns it unchanged and it can never equal the step slug, while
// the round log joins it to that step.
```

REPRODUCED, my fixture `g12` (Roadmap step `plan-fold`; one record joining increment `plan-fold` to step `plan-fold`; waiver `step = plan-fold`, `increment = plan-fold`):

```
prefix  exit=0  workflow invariants hold
head    exit=0  workflow invariants hold
```

`plan-fold` does not end `-inc<alnum>`, `leading_slug` returns it unchanged, that value EQUALS the waiver's `step`, and the PRE-FIX binary accepted the waiver. So the shape the comment calls unwritable has at least one writable member. The control, `g12b`, is the same id against a step it does not equal:

```
prefix  exit=1  round log line 2: increment waiver names step `plan` but increment `plan-fold` belongs to step `plan-fold`
head    exit=0  workflow invariants hold
```

which is the unblocking the comment means, correctly.

I REJECT THE CHARITABLE READING, on the reviewer's own ground and one more. "so" introduces both conjuncts, so both are offered as consequences of the id's shape; and the test's own assertion message pins only the other half ("the fixture must use an id the shim leaves unstripped"), so nothing in the test holds the clause that is false.

NOT A RE-RAISE, AND I CORRECT THE REVIEWER'S ACCOUNT OF WHY. Round 2's `W2B-3` was the same defect class at `CHANGELOG.md:32`, and its remedy named that clause alone. The reviewer says the round 2 triage's "class framing" made this site owed; it did not, since `W2B-3`'s remedy carries no sweep of `src/` and no in-place verdict on this comment. The correct ground is simpler: round 2 gave this site no verdict at all, so nothing about it is settled, and a `+` line in the diff carrying a false inference is raisable now. The `CHANGELOG` half was fixed and I re-verified it (`C3` in the claims file; the entry now states the retired rule itself and the degenerate class follows).

WHY `low`. An in-code test comment. No verdict, no shipped text, no emitted message. Its cost is that a maintainer reading it predicts a refusal the pre-fix binary did not make, in a comment whose whole job is to explain what the change unblocked.

REMEDY, SCOPED TO THE CLASS (every site in the diff that states which ids the retired rule blocked):

- `src/workflow.rs:1654-1658`, the whole comment sentence: state the retired rule itself, as the `CHANGELOG` now does, and let the blocked class follow. For example: an increment id whose leading slug is not the waiver's `step`, which for an id that does not end `-inc<alnum>` means any id that is not the step slug itself. The wording is the implementer's; what is not acceptable is a form that keeps offering "can never equal the step slug" as a consequence of the id's shape.
- `CHANGELOG.md:32`: NO EDIT. Already correct, re-verified on `g12`, and it is the model for the comment above.
- `src/workflow.rs:1619-1621`, the empty-case test's comment: NO EDIT. It already carries the qualifier "whenever the id happened to strip to the step slug".
- `src/workflow.rs:1659-1669`, the fixture and its assertions: NO EDIT. `beta-fold` on step `beta` is a correct instance of the unblocking, and `workflow-enforcement-tier-fold` is correctly named as the live one.
- `src/workflow.rs:411-425` and `:574-586`, the two doc comments that state the rule: NO EDIT. Neither makes a claim about which ids the retired rule blocked.

## `W3A-2`: the shipped rule text justifies the derived mark with a categorical claim about the log that a fixture falsifies

VALID, `low`. This is the second matter I was asked to settle, and the severity ruling is at the end of this section.

THE CLAIM, in `pack/instrument.md:11`, `AGENTS.md:147` and `.agents/AGENTS.reference.md:147`, byte-identical:

```
... an increment no `round` record resolves to is reported, and an owning step that no record of that increment
declares in a structured `step` id is reported as derived, SINCE SUCH A STEP IS ONE THE JOIN COMPUTED RATHER
THAN ONE THE LOG CARRIES ...
```

The rule half is correctly scoped to the increment's own records. THE JUSTIFICATION HALF DROPS THAT SCOPE and contrasts the marked step with what "the log" carries.

WRITTEN BY THE SHORTENING, VERIFIED IN THE HISTORY. `git show e0406bc:pack/instrument.md` carries, in the same position, "and a step reached through the `leading_slug` fallback is reported as derived, BECAUSE SUCH A STEP NEED NOT APPEAR IN THE ROADMAP OR ANYWHERE IN THE LOG". That is a possibility claim, and round 2's messages lens measured it true. `0202df6` replaced it with the categorical form. So the shortening turned a hedged true claim into an unhedged false one, in the copy that ships.

REPRODUCED, my fixture `g9`. The log carries one record declaring `gamma` in a structured `step` id for a DIFFERENT increment, and one increment-only record for `alpha-fold` whose `task` is `gamma-inc1`:

```
{"type":"round","task":"x","artifact":"a",...,"step":"gamma","increment":"other-inc1"}
{"type":"round","task":"gamma-inc1","artifact":"a",...,"increment":"alpha-fold"}
{"type":"waiver","task":"t","unit":"increment","step":"beta","increment":"alpha-fold","reason":"predates-logging","evidence_tier":"self-declared"}

head  exit=1  round log line 3: increment waiver names step `beta` but the round log joins increment `alpha-fold` to step `gamma` (derived from a record's `task`)

grep -c '"step":"gamma"' <root>/docs/metrics/workflow.jsonl  -> 1
```

`gamma` is marked derived, correctly, because no record OF `alpha-fold` declares it. AND THE LOG CARRIES `gamma` VERBATIM AS A STRUCTURED `step` ID. A reader of a scaffolded project's `AGENTS.md` is told a marked step is not one the log carries, and can falsify that with one grep of the log the refusal just printed.

THE REPLACED WORDING WAS TRUE, WHICH IS WHY RESTORING IT IS A REMEDY AND NOT A RETREAT. My fixture `fderiv` carries one pre-migration record whose `task` is `alpha-fold` and an increment waiver naming step `beta`:

```
head    exit=1  round log line 2: increment waiver names step `beta` but the round log joins increment `alpha-fold` to step `alpha-fold` (derived from a record's `task`)
prefix  exit=1  round log line 2: increment waiver names step `beta` but increment `alpha-fold` belongs to step `alpha-fold`

grep -c '"step":"alpha-fold"' <root>/docs/metrics/workflow.jsonl  -> 0
```

`alpha-fold` is not a Roadmap step and appears in the log only as a `task` value, so the pre-shortening "need not appear in the Roadmap or anywhere in the log" holds as a possibility claim, on this tree.

THE CHARITABLE READING, STATED AND REJECTED. If "the log carries" inherits the preceding clause's scope, meaning the records of that increment, the sentence is exactly true, since an owner is marked iff no record of that increment declares it. I reject that reading for two reasons. The justification carries no scoping word where the clause immediately before it does, which shows the author scopes when they mean to; and the sentence it replaced ranged explicitly over the whole log ("anywhere in the log"), so a reader who knew the previous text reads the replacement in the same frame. The fix is three words, which is the other reason not to argue the reading.

NOT A RE-RAISE OF `W2A-1` OR `W2B-4`. `W2A-1` was the mark's trigger stated per RECORD where the code applies it per OWNER; that clause is gone and its replacement is correct, which I checked on `g9` (the mark is per owner and per increment). `W2B-4`'s scope half was raised against three in-code comments, all now correctly scoped. This is a new sentence, in a different file, written after both verdicts.

IS `low` RIGHT GIVEN THE TEXT SHIPS INTO EVERY SCAFFOLDED PROJECT? YES, AND THE REASON IS THE HARM PER READER RATHER THAN THE NUMBER OF READERS. The blast radius is wide and the cost per instance is close to zero: no verdict moves, the emitted message is true, the mark itself is correct, and the action the refusal asks for is the same under either reading of the justification (make a record join that increment to that step). What a misled reader loses is a prediction about what a grep of their own log will show, not a decision. Round 2 settled this exact question for this exact text in Ruling 3 (`vc-inc1-r2-triage.md:424-432`), rating shipped rule text `low` because the direction of the error cost nothing, and round 1 rated `W1B-4` `low` on the same footing. `medium` in this project's calibration has meant an unmet, explicitly stated disclosure obligation about a breaking change (`W2B-2`) or an unpinned VERDICT-bearing axis (`W1A-1`). This is neither. I would rate it `medium` if the false clause could send a reader to a wrong action, and it cannot.

THE BLAST RADIUS DOES CHANGE THE REMEDY'S HANDLING, which I state rather than leave implicit, and which I verified: the `type: "waiver"` bullet is byte-identical across the three copies, so all three must move in one commit or the drift guard fails.

REMEDY, SCOPED TO THE CLASS (every site that justifies the derived mark with a claim about what the log does or does not carry):

- `pack/instrument.md:11`, the `type: "waiver"` bullet's W5 clause, over the JUSTIFICATION only: either carry the per-increment scope into it ("since no record of that increment states it") or restore a possibility claim ("since such a step need not appear in the Roadmap or anywhere in the log"). I prefer the second: it is the same length, it is measured true, and it restores the Roadmap half of the fact, which the shortening removed from every production site and which now survives only in one test comment and its assertion (`src/workflow.rs:1706`, `:1734`).
- `AGENTS.md:147` and `.agents/AGENTS.reference.md:147`, the same clause. THE THREE MUST MOVE IN ONE COMMIT.
- Acceptance item 7b's fixed-string command must be RE-RUN after the edit, since its replacement wording sits at the start of the parenthetical being changed. It must still report 0, 0, 0 and 1, 1, 1.
- CARRY THE REGENERATION HAZARD the step records at `validation-constraints.md:142`: do not run `just scaffold-self` naively, because its second line runs `nix fmt` over a tree that is not formatter-clean at HEAD. Run the render half alone, or regenerate and commit only the three rule files.
- `src/workflow.rs:543-547` and `:626-630`: NO EDIT. Both are correctly scoped to the increment's own records, both were fixed by this same commit, and they are the text the shipped clause must be made to agree with.
- `src/workflow.rs:579-582`, the `w5_problems` bullet's "may be one `round_step_slug` computed rather than one a record carries": NO EDIT. It is hedged with "may be", so it is a possibility claim, and my `fderiv` fixture satisfies it. THE FIX PASS MUST NOT IMPORT THE SHIPPED CLAUSE'S UNHEDGED FORM HERE.
- `src/workflow.rs:631-640`, the owners map, and `:647-653`, the message: NO EDIT. Both are correct on `g9`, which round 2's triage already ruled ("THE MESSAGE STAYS TRUE ... and the per-increment scope is the right scope").
- `CHANGELOG.md:32`: NO EDIT. Its version of the sentence reads "an owning step that no record of the increment declares is reported as derived" and carries no justification clause, which I confirmed by reading the whole `### Fixed` paragraph.

## Ruling: the out-of-scope survivors, verified, plus one the reviewer's own accounting drops

I CHECKED THE RULING RATHER THAN INHERITING IT, and I AGREE with it on all four the reviewer settled. Condition 2 I measured directly: `git diff main..HEAD -- src/workflow.rs` contains NO `+` and NO `-` line matching any of the anchors (`escalation.task != evidence`, `human_decision != HumanDecision`, `escalation_increment_id`, `increments.entry`, `fn leading_slug`, `rfind`, `INCREMENT_MARKER`, `fn round_log_consistency_problems`), and the enclosing function bodies are byte-identical between `main` and `HEAD`:

```
git show main:src/workflow.rs | grep -A 12 "fn leading_slug"                    md5 89d5105ca7f22c90df6ec8dd703acde8
git show HEAD:src/workflow.rs | grep -A 12 "fn leading_slug"                    md5 89d5105ca7f22c90df6ec8dd703acde8
   the same identity holds for `fn escalation_increment_id`, `fn escalation_step_slug`
   and `fn round_log_consistency_problems`
```

- `B5`, the `escalation.task != evidence` clause: OUT OF SCOPE, AND THE STRONGEST OF THE FIVE. Condition 1 holds (the evidence join predates the base commit). Condition 2 holds (no `+`/`-` line; the `w5_problems` hunk that widened the signature stops before the evidence arm). Condition 3 holds, and it is the load-bearing one: the subject is whether a record-backed waiver's POINTER names the escalation it claims, which is a different arm over a different input (escalations, not rounds) and is not what inc1's review question asks. Condition 4 holds: the remedy is a new test that shares nothing with the ownership predicate's tests. REPRODUCED ANYWAY, because a routing recommendation with no evidence is worth nothing: `m-b5` passes 386 of 386, and on my fixture `b5` `head` exits 1 with "`record-backed` waiver cites evidence `no-such-pointer` but no `type:\"escalation\"` record with `human_decision` `decision` is scoped to this waiver's unit" while `m-b5` exits 0 with `workflow invariants hold`. A FALSE GREEN IN THE ENFORCEMENT TIER, unpinned. Routing is below.
- `B2`, `escalation_increment_id` ignoring the structured id: OUT OF SCOPE. All four conditions hold on the same grounds; the subject is the escalation join's increment axis, a pre-existing Inc 2 identity question.
- `C2` and `C5`, the two grouping sites keying on `round_increment_id`: OUT OF SCOPE. All four conditions hold. The subject is how an increment is IDENTIFIED, which predates the ownership rule; `C5`'s site is not touched by the diff at all, and `C2`'s grouping line is context inside a function the diff changes elsewhere, which is condition 2 satisfied at the line rather than at the function.
- `A7`, `leading_slug` taking the first `-inc` marker instead of the last: OUT OF SCOPE. `fn leading_slug` is byte-identical across the range and the class needs a `task` carrying two markers.

`C4` IS A FIFTH SURVIVOR AND THE REVIEWER'S REPORT SETTLES IT NOWHERE. Its table row reads "NO, 386 passed | none. SURVIVOR, see below", and no "below" follows: the survivors section counts eight where the tables mark ten, and accounts for nine of them (two equivalent, two the `W3B-2` pair, five pre-existing). `C4` is the one that falls out. I settle it myself rather than leave it hanging.

I BUILT IT. Mutation: `.any(|waiver| waiver.unit == WaiverUnit::Step && waiver.step == step.slug)` becomes `.any(|waiver| waiver.step == step.slug)`, so any waiver naming the step exempts a `complete` step with no records of its own. 386 passed, 0 failed. My fixture `c4` (Roadmap `alpha` `complete` and `beta` `in progress`; one record joining `alpha-inc1` to `beta`; an increment-unit waiver naming step `alpha`):

```
head   exit=1  Roadmap step `alpha` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"` for it ...
               round log line 2: increment waiver names step `alpha` but the round log joins increment `alpha-inc1` to step `beta`
m-c4   exit=1  round log line 2: increment waiver names step `alpha` but the round log joins increment `alpha-inc1` to step `beta`
```

VERDICT-NEUTRAL AND NOT A FALSE GREEN, and that is structural rather than lucky: W3's no-records branch fires only when no record joins the step, and W5's ownership arm then necessarily refuses the same waiver, so the two cannot both fall silent. OUT OF SCOPE by the same four conditions (the mutated line carries no `+`/`-` in the diff, and the subject is W3's step-level exemption UNIT axis, which predates inc1 and is not what its review asks). NOT A FINDING, recorded here so the survivor list is complete and so a later round does not re-derive it.

## Ruling: where `B5` should be routed

IT IS GENUINELY OUT OF THIS INCREMENT'S SCOPE, per the four conditions above, and it must not be folded into inc1's fix pass: doing so would widen inc1's review question after three rounds, which is the scope expansion Principle 8 forbids, and inc1's remaining fix surface is otherwise four small edits.

IT SHOULD GO TO THE PLANNER, as a new member of the `validation-constraints` step, which is the validator cluster's own home and already carries every other W5 and `validate` defect this loop has routed. Two placements are available and the choice is the planner's, not mine:

- FOLD IT INTO `validation-constraints-inc2`, which is the only other increment that touches `w5_problems` and whose review question is already "is the rule's reachability now described truthfully, on both substrates". A test pinning that the evidence pointer is load-bearing sits close to that question, and inc2 has not started, so nothing is disturbed. The cost is that inc2's review question has to widen by a clause, and the step's own division rule is that an increment asks ONE question.
- DECLARE IT AS ITS OWN INCREMENT. Cleaner against the one-question rule and more ceremony for what is one test.

I RECOMMEND THE FIRST, on the ground the step itself uses for inc2 ("the cheapest moment to state the reachability of one rule truthfully is while its sibling is fresh"), with the second recorded as authorised and not taken so that splitting later needs no new decision. That is the same shape the round 2 fix pass used for its own routed fork.

WHAT THE PLANNER SHOULD RECORD WITH IT, so the member is re-derivable: the mutation, the fixture shape (a step-unit record-backed waiver citing a pointer no escalation carries, with a scoped `decision` escalation present), the measured result (386 green, `head` exit 1, mutant exit 0 with `workflow invariants hold`), and the reason the three tests that look like they cover it do not (`w5_flags_a_record_backed_waiver_citing_an_unrelated_escalation`, `w5_flags_a_record_backed_waiver_citing_an_escalation_for_another_step` and `w5_flags_a_record_backed_waiver_with_no_matching_escalation` all fail their join on the UNIT SCOPE or on an empty escalation slice, never on the pointer equality). HAD IT BEEN IN SCOPE it would have rated `medium`, by the same verdict-bearing test that puts `W3B-1` there; that figure is recorded for the planner's sequencing and is not a verdict against this artifact.

## Overall assessment

THE ROUND'S REAL RESULT: NOT CLEAN. Four valid findings, no duplicate, none dismissed, ceiling `medium` carried by `W3B-1` alone. No `high` and no `critical` was raised or found, and I dismissed nothing, so no backstop re-check is owed. The round outcome is `new_valid` and the consecutive-clean streak stays at 0 against the two a `risky` artifact needs.

THE SHIPPED BEHAVIOUR IS CORRECT, AND I MEASURED IT RATHER THAN ACCEPTING IT. Eleven fixture trees of my own construction across nine binaries, plus the unmodified live plan and log: `head` returned the verdict I computed by hand from the documented accessors on every one. `prefix` was run on eight of the eleven and differed from `head` in VERDICT on three (`L0` and `L6`, where the pre-fix binary accepts at exit 0 what `head` reports, the `Q-70-emptycase` narrowing; and `g12b`, where the pre-fix binary refuses at exit 1 what `head` accepts, the unblocking) and in MESSAGE on three more (`L0fold`, `g9`, `fderiv`, each naming a step derived from the id where `head` names what the records join). Every difference is a documented narrowing or the documented unblocking, and there is no tree on which `head` answers wrongly. The live plan is green at exit 0, `cargo test --bins` is 386 green, clippy is clean, acceptance item 7b reports 0, 0, 0 and 1, 1, 1, and the three prose copies are byte-identical. THREE ROUNDS AND SIX INDEPENDENT LENSES HAVE NOW LOOKED FOR A WRONG VERDICT AND NONE HAS FOUND ONE.

EVERY VALID FINDING IS IN WHAT THE CHANGE SAYS OR IN WHAT PINS IT. Two are test-coverage gaps over correct code (`W3B-1`, `W3B-2`), one is a false clause in an in-code test comment (`W3A-1`), and one is a false clause in the shipped rule text (`W3A-2`). Not one is a defect in what the tool does. That is the same result rounds 1 and 2 reached, now three times by three disjoint sets of reviewers.

SAFE TO MERGE ONCE THE REMEDIES LAND? YES. No remedy changes a verdict; none touches `waiver_covers_round`, `round_step_slug`, `round_increment_id`, the owners map, or any check's logic. The whole fix surface is one added test assertion or sibling test, one added fixture, one comment sentence, and one clause in three drift-guarded prose copies moved in a single commit with acceptance item 7b re-run afterwards. There is no judgement inside the set beyond where the empty-log assertion lives, which I ruled above.

IS THE LOOP CONVERGING? YES ON THE EVIDENCE, AND I GIVE THE NUMBERS RATHER THAN THE IMPRESSION. Valid findings by round: 5, then 9, then 4. Severity ceiling: `medium`, `medium`, `medium`, with no `high` or `critical` ever raised or found across three rounds. Findings written BY the previous fix pass: round 2 had four of nine, round 3 has one of four (`W3A-2`). Reviewer yield is falling too: this round's two lenses raised four RAW findings between them and all four survived triage, where round 1 raised seven raw for five valid and round 2 eleven raw for nine. The shortening the human chose did attenuate the mechanism round 2 diagnosed: it closed the four claim findings it aimed at, every pointer it introduced resolves to a target that says what the pointer claims, and it wrote exactly one new defect where the previous pass wrote four.

WHAT IT HAS NOT DONE IS REDUCE THE NUMBER OF SURFACES. The ownership relation is still stated at about ten places, and the claims lens measured production comment lines at 279 on `main`, 331 before the shortening and 320 after, so the change still adds 41 comment lines over `main`. Each restatement stays a place to be wrong. My honest judgement is that a further round would find prose defects at a low rate and would not find a wrong verdict, which is what the last three rounds measured and what the behaviour surface predicts: one predicate over two accessors, now pinned on both consumers by mutations spanning 16 to 20 tests.

WHERE THE REMAINING RISK ACTUALLY SITS IS THE SUITE AND NOT THE PROSE, and that is this round's one genuinely new fact. Two of the four findings are coverage gaps, one of them caused by a previous remedy retiring a red half. Both fix passes so far have been judged on whether they closed the named findings and not on what their edits stopped pinning. THE FIX PASS FOR THIS ROUND MUST BE THE ONE THAT CHANGES: every remedy above is an ADDITION except the two comment edits, and the rule recorded under `W3B-1` (say what a changed fixture currently pins, and add rather than replace when nothing else pins it) applies to whatever the implementer writes next.

THE ARITHMETIC, STATED AS A FACT AND NOT AS A CONVERGENCE RECOMMENDATION, WHICH IS NOT MINE TO MAKE. This is round 3 of a cap of 5 on a `risky` artifact needing two consecutive clean rounds, with the streak at 0. Rounds 4 and 5 must BOTH be clean for the loop to converge under the cap; one valid finding in either sends the artifact to the human at round 5. There is no margin left, so the fix pass's own cost matters as much as its correctness.

TWO THINGS THE ORCHESTRATOR STILL OWES, NEITHER A FINDING AGAINST THIS ARTIFACT, both unchanged from rounds 1 and 2. Acceptance item 3, the plan-side unblocking (the two `[[step.increment]]` declarations, the two owed waivers, the `workflow-enforcement-tier` status flip), is still absent from `git diff main..HEAD`, and the step assigns those edits to the orchestrator and the planner. And the post-merge planner pass owes the sidecar the facts rounds 1 and 2 recorded, to which this round adds two: acceptance item 4's "two forms" wording predates `Q-70-emptycase`'s three-option receipt and cannot pin the decision as written, and item 7b should name the whole parenthetical rather than one fixed string inside a sentence that has now moved twice.
