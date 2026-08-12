# `validation-constraints-inc1`, round 2: triage

Triager worktree: `.claude/worktrees/tri-inc1-r2`, branch `triage/inc1-r2`, at `0bdb414` (`60ee7d0` before the rebase the reviewers saw).
Artifact: `git diff main..HEAD` in that worktree, two commits (`8d58e7b`/`6ec9f1a` the implementation, `0bdb414`/`60ee7d0` the round 1 fix pass), touching `.agents/AGENTS.reference.md`, `AGENTS.md`, `CHANGELOG.md`, `pack/instrument.md`, `src/plan/source.rs`, `src/workflow.rs`.
Specification: `docs/plans/agent-scaffold.steps/validation-constraints.md`, increment 1 and the Acceptance section.
Findings adjudicated: `vc-inc1-r2-reviewer-residue.md` (`W2A-*`, four) and `vc-inc1-r2-reviewer-messages.md` (`W2B-*`, seven). Round 1's settled verdicts are in `vc-inc1-r1-triage.md`.

EVERY MEASUREMENT BELOW WAS TAKEN BY ME, IN MY OWN TREES AND FIXTURES. Nothing is carried from a reviewer's report on trust. Where my figure or my reading differs from a reviewer's, I say so.

## Verdict table

| id | verdict | severity | ground |
| --- | --- | --- | --- |
| `W2A-1` | VALID | `low` | Reproduced on my own fixture: one record declaring `alpha` and one deriving it through the shim produce an UNMARKED owner, so the shipped clause "a step reached through the `leading_slug` fallback is reported as derived" is false as written. Claim defect in three shipped copies plus the `CHANGELOG`. |
| `W2A-2` | VALID | `low` | All three mutations rebuilt and reproduced: `m9`, `m16` and `m20` each leave 385/385 unit tests green while making the refusal state something false about the log. TEST-COVERAGE GAP over correct code; no verdict moves under any of the three. |
| `W2A-3` | DUPLICATE OF `W2B-4` | (`low`, merged) | Same defect class in the same sentences. `W2B-4` is the better statement: it carries both the enumeration half and the scope half and names two more sites. `W2A-3`'s accessor-block argument is merged into the remedy. |
| `W2A-4` | VALID | `low` | Reproduced: with the W3 sibling test ignored, `m10` still reddens `w5_flags_an_increment_waiver_whose_increment_has_no_round_records`, so "with the whole suite green" is false of this commit. The same commit's own empty-case comment (`:1640-1644`) already says the opposite. |
| `W2B-1` | DUPLICATE OF `W2A-1` | (`low`, merged) | One defect found by two lenses. `W2A-1` is kept as the primary because its second evidence item (the new test at `:1767` asserts the per-owner rule the prose denies) is what makes it a defect in this diff rather than a preference. `W2B-1`'s replacement wording is the better one and is merged into the remedy. |
| `W2B-2` | VALID | `medium` | Reproduced on two fixtures: the pre-fix binary exits 0 with `workflow invariants hold` where the fixed binary refuses, on a population the `CHANGELOG`'s exhaustive "THE POPULATION THIS NARROWS is ..." sentence does not name. The step's own Documentation impact makes naming the population an obligation, and it is not met. |
| `W2B-3` | VALID | `low` | Reproduced: `plan-fold` is an increment id that does not end `-inc<x>` whose records join it to the waiver's step, and the pre-fix binary ACCEPTED it, so "it refused every increment id that does not end `-inc<x>` even when ..." is false by one degenerate class. |
| `W2B-4` | VALID | `low` | Both halves reproduced. Scope: a record of ANOTHER increment declares `alpha-fold` in a structured `step` id and the owner is still marked derived. Enumeration: two records carrying a structured `increment` and no `step` give two derived owners by neither of the doc's two routes. |
| `W2B-5` | VALID | `low` | Confirmed by reading and by a diff audit: the fix pass moved twelve sites from "attributes" to "joins" and left `src/workflow.rs:412`, the doc comment on the predicate that DEFINES the relation, on the retired verb. The weakest finding of the round; the remedy is one word. |
| `W2B-6` | VALID | `low` | Reproduced both orderings: with the derived owner sorting last the trailing parenthetical can be read as qualifying the whole list, so a reader can conclude a recorded step was computed. That is the exact mis-statement the mark was added to prevent. |
| `W2B-7` | VALID | `low` | Reproduced on two fixtures: a record carrying the waived increment id is dropped by `parse_rounds` and the refusal then asserts that no record resolves to it. One of the finding's sub-claims does NOT generalise; see that section. |

NINE VALID FINDINGS, TWO DUPLICATES, NONE DISMISSED. Ceiling `medium`, carried by `W2B-2` alone. NO `high` AND NO `critical` FINDING WAS RAISED OR FOUND, and I dismissed nothing, so NO BACKSTOP RE-CHECK IS OWED on this round.

EVERY ONE OF THE NINE IS IN WHAT THE CHANGE SAYS OR IN WHAT PINS IT. Not one is a defect in what the tool does. That is the same shape round 1 recorded, and I state it here rather than only in the assessment because it is the fact the orchestrator most needs.

## Trees, binaries and fixtures

All under `<scratch>/tri-vc-r2` (a subdirectory of my own naming; the pre-existing `tri2` contents from an earlier session were left untouched). ONE `CARGO_TARGET_DIR` PER BINARY, all eight verified distinct (`md5sum ... | awk '{print $1}' | sort | uniq -d` returns nothing):

```
c7558f05bfc00a806f27e1237eafaa6e  target-prefix/debug/agent-scaffold  (git archive main)
b728af5f1614e2f92d8f1420840ed044  target-mid/debug/agent-scaffold     (6ec9f1a, before the fix pass)
f67cc3ed57611f1005911dd106e6da3e  target-head/debug/agent-scaffold    (HEAD)
661160921bde8a29c095a2b663881346  target-m9/debug/agent-scaffold      (owners scan keys on raw `task`)
fe211c0bcc19a748ce6c545425c53a8e  target-m16/debug/agent-scaffold     (mark reads `round.increment`)
d7ebc9893df2f3bac1b156e0e38ecfb4  target-m20/debug/agent-scaffold     (merge is first-write-wins)
96f9e1a4f6203093941347039ae3afa7  target-m10/debug/agent-scaffold     (increment axis dropped)
b34f9258529e954aaa50445c2b566052  target-m15/debug/agent-scaffold     (m10 plus the W3 sibling test ignored)
```

Mutations were applied by `<scratch>/tri-vc-r2/apply.py`, an exact-string replacement with an occurrence-count assertion, so a stale anchor fails loudly rather than silently rebuilding the pristine tree. `TMPDIR` pointed at `<scratch>/tri-vc-r2/tmp`, outside every git repository, per the Acceptance preamble. `src/` was mutated only in scratch copies; my worktree carries this file and nothing else.

Control, HEAD unmutated:

```
cargo test --bins                          -> 385 passed; 0 failed
cargo test (9 binaries)                    -> 385 + 5 + 1 + 1 + 9 + 3 + 20 + 1 + 4 = 429 passed; 0 failed
cargo clippy --all-targets -- -D warnings  -> exit 0
```

Fifteen fixtures, each its own project root with `docs/plans/t.md` and `docs/metrics/workflow.jsonl`, built by `<scratch>/tri-vc-r2/mkfx.py`. Command in every case: `agent-scaffold validate --plan <root>/docs/plans/t.md --workflow`, with paths elided to `PLAN` and `LOG` below. Every fixture's escalation is scoped to its waiver's increment and every round record is schema-complete, so the ONLY problem any fixture reports is the W5 ownership one.

VERDICTS ACROSS `prefix`, `mid` AND `head` ON ALL FIFTEEN (exit code):

```
f-derived              prefix=1 mid=1 head=1
f-ormerge              prefix=1 mid=1 head=1
f-ormerge-rev          prefix=1 mid=1 head=1
f-taskdiffers          prefix=1 mid=1 head=1
f-twoderived           prefix=1 mid=1 head=1
g12-retired-accepts    prefix=0 mid=0 head=0
g4-mixed-last          prefix=1 mid=1 head=1
g5-mixed-first         prefix=1 mid=1 head=1
g9-cross-increment     prefix=1 mid=1 head=1
h1-rehomed-declared    prefix=0 mid=1 head=1
h2-rehomed-derived     prefix=0 mid=1 head=1
h3-malformed           prefix=1 mid=1 head=1
h3-minimal             prefix=1 mid=1 head=1
h4-malformed-repaired  prefix=0 mid=1 head=1
w3-sibling             prefix=1 mid=1 head=1
```

TWO THINGS THIS SETTLES INDEPENDENTLY OF THE RESIDUE REVIEWER'S 36-TREE MATRIX, which I did not re-run. `mid` and `head` agree on every one of the fifteen, so THE FIX PASS CHANGED NO VERDICT. And `head` matched the verdict I computed by hand from the documented accessors on every one of the fifteen, so I found no tree on which the shipped build answers wrongly.

Live plan and log, unmodified, with the `head` binary in my worktree:

```
validate --source docs/plans/agent-scaffold.plan.toml --workflow
  -> 319 records valid, 96 steps, 70 questions, workflow invariants hold, exit 0   (acceptance item 2)
grep -c -F "the increment's leading slug equals the step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md   -> 0, 0, 0   (item 7b)
grep -c -F "some `round` record must join that increment to that step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md   -> 1, 1, 1
```

DRIFT GUARD RE-DEMONSTRATED, because three of my remedies move the shipped clause. In a scratch copy of HEAD with `AGENTS.md` alone reverted to `main`, `agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render` fails with "root AGENTS.md has drifted from a fresh pack render"; with all three copies in step it passes. The three copies cannot drift apart silently, so a fix pass must move all three together.

## The out-of-scope precedent: checked, and it applies to none of the nine

I checked all four conditions once, against the whole set, because the answer is the same for every finding.

1. PROVENANCE PREDATES THE BASE COMMIT: holds only for two mechanisms, `round_step_slug`'s per-axis fallback and `parse_rounds`'s best-effort drop. It holds for NO claim at issue.
2. NO COMMIT IN RANGE MODIFIES THE CLAIM'S LINES: FAILS for all nine. I checked each cited line against `git diff main..HEAD` and each is a `+` line. `git show <commit>:<file> | grep -c -F` also dates each prose claim: "reported as derived" and "marks it as derived" enter at the fix pass (0 at `main` and `6ec9f1a`, 1 at `60ee7d0`); "THE POPULATION THIS NARROWS" and the retired-rule clause enter at the implementation commit (1 at `6ec9f1a`).
3. THE SUBJECT IS INDEPENDENT: FAILS for all nine, and this is the condition that does the real work. In every case the subject is whether W5's refusal, or the text that describes it, states a fact the round log records. That is inc1's own review question, stated at `validation-constraints.md:68`.
4. NO SHARED FIX: FAILS. Every remedy is to text this diff adds.

I DO NOT REOPEN DIRECTION (iii) OR `Q-70-emptycase`, and I have no evidence beating either. `head` refuses the empty case unconditionally on `g6`-shaped input, and every acceptance the fix adds is a tree where a round record joins the waived increment to the waiver's step. Both are the receipted decisions, implemented as receipted.

## `W2A-1` (with `W2B-1` merged): the shipped mark clause is stated per record and computed per owner

VALID, `low`. Both reviewers rated it `low` and both offered the argument for `medium`; I uphold `low` and give my reasoning under the third ruling below.

THE CODE. `src/workflow.rs:644-653` builds the owners map with `*seen |= declared` where `declared = round.step.is_some()`, so the mark is a property of the OWNER after an OR across all of that increment's records, not of the record that produced it.

THE CLAIMS. `pack/instrument.md`, `AGENTS.md:147` and `.agents/AGENTS.reference.md:147`, byte-identical and all rewritten by the fix pass, say "a step reached through the `leading_slug` fallback is reported as derived". `CHANGELOG.md:32` says "a refusal naming such a step marks it as derived".

REPRODUCED, fixture `f-ormerge` (Roadmap `alpha` and `beta`, one record `{"step":"alpha","increment":"alpha-inc1"}` and one pre-migration record `{"task":"alpha-inc1"}`, an increment waiver naming `beta`):

```
head    exit=1  PLAN vs LOG: round log line 3: increment waiver names step `beta` but the round log joins increment `alpha-inc1` to step `alpha`
prefix  exit=1  PLAN vs LOG: round log line 3: increment waiver names step `beta` but increment `alpha-inc1` belongs to step `alpha`
```

`alpha` WAS reached through `leading_slug("alpha-inc1")` by the second record and is NOT reported as derived. The control, `f-derived`, with the declaring record removed:

```
head    exit=1  ... but the round log joins increment `alpha-fold` to step `alpha-fold` (derived from a record's `task`)
```

The three copies, one command:

```
grep -c -F "a step reached through the `leading_slug` fallback is reported as derived" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md   -> 1, 1, 1
```

WHY IT IS A DEFECT AND NOT A PREFERENCE. The same commit adds `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it` (`src/workflow.rs:1767`), whose fixture is `f-ormerge`'s shape and whose assertion is `!problems[0].contains("derived")`. The test states the per-owner rule and the prose states the per-record rule, inside one commit. That is the residue lens's second evidence item and it is why I keep `W2A-1` as the primary statement.

THE OVER-PROMISE IS IN THE SAFE DIRECTION, which is why it is `low` and not more. `declared` is true only for a record whose `step` is `Some`, and `round_step_slug` returns exactly that value, so an UNMARKED owner is always a byte-identical `step` value some record of that increment carries. A reader who sees an unmarked owner and concludes a record declares it is CORRECT. The prose promises more marking than the code does; it never lets a computed value pass as a recorded one.

REMEDY, SCOPED TO THE CLASS (every site that states the derived mark's trigger per RECORD where the code applies it per OWNER, across the increment's own records):

- `pack/instrument.md`, the `type: "waiver"` bullet's W5 clause, over the whole clause: state the trigger as the code applies it. `W2B-1`'s wording is the better one and I adopt it, because it also carries the per-increment scope that `W2B-4`'s scope half needs: a step that NO record OF THAT INCREMENT declares in a structured `step` id is reported as derived.
- `AGENTS.md:147` and `.agents/AGENTS.reference.md:147`, the same clause. THE THREE MUST MOVE TOGETHER or the drift guard fails, which I re-demonstrated above.
- `CHANGELOG.md:32`, the "DERIVATION IS NOT FULLY RETIRED" sentence, over the whole sentence. THIS PARAGRAPH ALSO CARRIES `W2B-2` AND `W2B-3`, so rework the whole `### Fixed` paragraph in one pass rather than three, exactly as round 1 ruled for the same paragraph. The rework must not re-introduce a per-record-kind framing, which is `W2B-4`'s enumeration half landing in the same sentence.
- Acceptance item 7b's fixed-string command must be RE-RUN after the edit, because the replacement wording it greps for ("some `round` record must join that increment to that step") sits inside the sentence being changed. It must still report 0, 0, 0 and 1, 1, 1.
- CARRY THE REGENERATION HAZARD the step records: do not run `just scaffold-self` naively, because its second line runs `nix fmt` over a tree that is not formatter-clean at HEAD.
- `src/workflow.rs:644-653`, the owners map: NO EDIT. Correct as shipped on every tree I ran.
- `src/workflow.rs:1767`, `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it`: NO EDIT for this finding. Its comment states the per-owner rule correctly and it is the site the prose must be made to agree with. (`W2A-2` asks it for one ordering assertion; that is a different obligation.)

## `W2A-2`: three unpinned inputs to the new owners map, each of which makes a refusal false

VALID, `low`. This is the second matter I was asked to settle; the ruling is under "Ruling 2" below and the evidence is here.

ALL THREE MUTATIONS REBUILT AND RE-RUN BY ME. Each leaves the whole unit suite green:

```
head  cargo test --bins  ->  385 passed; 0 failed
m9    cargo test --bins  ->  385 passed; 0 failed
m16   cargo test --bins  ->  385 passed; 0 failed
m20   cargo test --bins  ->  385 passed; 0 failed
```

MUTATION A (`m9`), the scan's increment axis: `round_increment_id(round) == increment` becomes `round.task == increment` in the owners scan only, leaving `waiver_covers_round` untouched. Fixture `f-taskdiffers`, one record `{"task":"zzz-task","step":"alpha","increment":"alpha-inc1"}`, waiver naming `beta` for `alpha-inc1`:

```
head  ... increment waiver names step `beta` but the round log joins increment `alpha-inc1` to step `alpha`
m9    ... increment waiver names increment `alpha-inc1`, which no `type:"round"` record resolves to (a record resolves to its structured `increment` id, or to its `task` when that id is absent), so the round log joins it to no step
```

The `m9` sentence is FALSE OF ITS OWN FIXTURE by the rule the sentence itself states.

MUTATION B (`m16`), the mark's axis: `round.step.is_some()` becomes `round.increment.is_some()`. Fixture `f-twoderived`, two records each carrying a structured `increment` and no `step`, with different `task` values:

```
head  ... joins increment `alpha-fold` to steps `yyy-task` (derived from a record's `task`), `zzz-task` (derived from a record's `task`)
m16   ... joins increment `alpha-fold` to steps `yyy-task`, `zzz-task`
```

`m16` PRESENTS TWO `leading_slug` PRODUCTS AS STEPS THE RECORDS DECLARE. That is, verbatim, the recorded `src/` defect the plan says inc1 closes, reproduced green.

MUTATION C (`m20`), the merge rule: `*seen |= declared` becomes a no-op, so the mark is first-write-wins. Fixture `f-ormerge-rev`, `f-ormerge` with the two records in the opposite file order:

```
head  ... joins increment `alpha-inc1` to step `alpha`
m20   ... joins increment `alpha-inc1` to step `alpha` (derived from a record's `task`)
```

And on `f-ormerge`, with the declaring record first, `m20` and `head` agree. THE ONLY MIXED-CASE FIXTURE IN THE SUITE PUTS THE DECLARING RECORD FIRST (`src/workflow.rs:1782`), so it cannot distinguish a union from first-write-wins.

NO VERDICT MOVES UNDER ANY OF THE THREE, measured rather than argued. Across eleven fixtures, `head`, `m9`, `m16` and `m20` return identical exit codes and exactly one W5 ownership problem each (zero on the accepting fixture). That follows from the code: the owners map is built inside the `!rounds.iter().any(waiver_covers_round)` branch and decides only WHICH of two messages is pushed, never whether one is.

CODE DEFECT, CLAIM DEFECT OR TEST GAP: TEST GAP, STATED PLAINLY, exactly as round 1 ruled `W1A-1`. No line of `w5_problems` is wrong.

REMEDY, SCOPED TO THE CLASS (the owners map's three inputs: which records enter the scan, which structured field decides the mark, and whether the merge is order-independent), TEST-SIDE ONLY:

- `src/workflow.rs:1717`, `w5_marks_an_owner_derived_from_a_pre_migration_records_task`: give it (or a sibling beside it) a record carrying a structured `increment` and NO `step`, whose `task` DIFFERS from the increment id, and assert the refusal names the owner derived from that record's `task` and marks it derived. ONE FIXTURE KILLS `m9` AND `m16` TOGETHER; `f-twoderived` is a worked instance.
- `src/workflow.rs:1766-1795`, `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it`: assert the same expectation with the two log lines in BOTH orders, or reverse the existing order at `:1782`. One line, and it kills `m20`.
- `src/workflow.rs:644-653`, the owners map: NO EDIT.
- `src/workflow.rs:426-433`, `waiver_covers_round`: NO EDIT. Its two axes are pinned; I re-ran `m10` (2 red, spanning W3 and W5) and the residue lens's `m11` is the acceptance item 7 demonstration.
- FOR THE POST-MERGE PLANNER, not a finding against this artifact: acceptance items 4b and 7 pin the shared PREDICATE, and item 4 pins the unobserved case. NOTHING IN THE LIST REACHES THE MESSAGE'S PROVENANCE INPUTS, which is why three mutations of them pass every item. The sidecar is known stale and I do not raise it, but this specific gap should be recorded when it is updated.

## `W2A-3`: duplicate of `W2B-4`

Both findings are that the fix pass's new doc comments describe the derived mark more broadly, or the routes to several owners more narrowly, than the code computes. They overlap exactly on `src/workflow.rs:551-555` (the "SEVERAL OWNERS ARISE TWO WAYS" enumeration) and are adjacent at `:586-589`.

`W2B-4` IS THE BETTER STATEMENT: it carries both halves (the enumeration AND the "any record" scope), and it names two sites `W2A-3` does not (`:545` and `:640`) plus the test comment at `:1688`. `W2A-3` contributes one argument worth keeping, that `:586-588` cites the accessor block at `:98-111` as its authority while enumerating two record kinds the block's per-axis statement does not reduce to. I MODERATE THAT ARGUMENT: the block does not state the OPPOSITE of the new comment, as `W2A-3` puts it. `:586` opens "BOTH AXES STILL DEGRADE PER RECORD, per the accessor block above", which is correct; the defect is that the illustration which follows is a two-kind enumeration where the block documents a per-axis property. That is an incomplete enumeration, not a contradiction.

`W2A-3` also names `CHANGELOG.md:32`'s "a pre-migration record carries no `step` id, so `round_step_slug` still derives its step" clause. That clause is TRUE and is an illustration rather than an enumeration, so it owes no separate edit; it sits inside the sentence `W2A-1`'s remedy reworks, and that rework must not re-introduce a per-record-kind framing. Recorded in `W2B-4`'s remedy.

## `W2A-4`: the new W3 test's comment says the suite would be green without it, and the same commit falsifies that

VALID, `low`.

THE CLAIM. `src/workflow.rs:1029-1031`, added by the fix pass: "Without this case a build that dropped `waiver_covers_round`'s increment comparison would report `workflow invariants hold` at exit 0 over an unconverged `risky` increment, with the whole suite green."

REPRODUCED IN TWO HALVES, and the sentence is a conjunction whose halves now disagree.

First half, TRUE. My fixture `w3-sibling` (Roadmap `stall` `complete`; `stall-incA` `risky` at peak streak 1 of 2; `stall-incB` converged and carrying the only waiver, which names `stall-incB`):

```
head  exit=1  PLAN vs LOG: Roadmap step `stall` increment `stall-incA` reached a consecutive-clean streak of 1 but its `risky` risk class needs 2
m10   exit=0  workflow invariants hold
m15   exit=0  workflow invariants hold
```

Second half, FALSE. `m15` is `m10` plus this test marked `#[ignore]`:

```
m10  test result: FAILED. 383 passed; 2 failed
       an_increment_waiver_does_not_exempt_a_sibling_increment_of_the_same_step
       w5_flags_an_increment_waiver_whose_increment_has_no_round_records
m15  test result: FAILED. 383 passed; 1 failed; 1 ignored
       w5_flags_an_increment_waiver_whose_increment_has_no_round_records
```

The fix pass gave `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` a non-empty log at `src/workflow.rs:1648`, and that fixture catches the same mutation. THE SAME COMMIT SAYS SO IN THE OTHER TEST'S OWN COMMENT (`src/workflow.rs:1640-1644`): "a build that dropped the increment axis from `waiver_covers_round` and compared the step alone would still pass it". So the two comments contradict each other inside one commit, which is the sharpest form of this finding and is why I rate it a defect rather than a stale aside.

WHY `low`. A test comment. Its cost is that a maintainer reading either comment is told the other case does not exist, so weakening either looks safe from the other's text.

REMEDY, SCOPED TO THE CLASS (both comments must state the same two-consumer fact):

- `src/workflow.rs:1029-1031`, the last clause: say that this case pins the increment axis on W3's side and that `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` pins it on W5's, which is also the accurate statement of what round 1's remedy asked for (two additions, one per consumer).
- `src/workflow.rs:1640-1644`, the empty-case test's comment: NO EDIT. It is already the accurate half and is the text the other must be made to agree with.
- The fixture and assertions at `src/workflow.rs:1020-1050`: NO EDIT. `m10` shows the case is a genuine catch.

## `W2B-2`: the `CHANGELOG` names one narrowed population and there are two

VALID, `medium`. THIS IS THE FIRST MATTER I WAS ASKED TO SETTLE, AND I VERIFIED IT RATHER THAN READING IT.

THE CLAIM. `CHANGELOG.md:32`: "THE POPULATION THIS NARROWS is an increment-unit waiver whose `increment` NO round record resolves to". "THE POPULATION ... is X" is a definite description, so it asserts exhaustiveness.

REPRODUCED, TWO FIXTURES, TWO ROUTES INTO THE SAME POPULATION.

`h1-rehomed-declared`, one record `{"step":"beta","increment":"alpha-inc1"}`, waiver `step = alpha`, `increment = alpha-inc1`:

```
prefix  exit=0  workflow invariants hold
head    exit=1  PLAN vs LOG: round log line 2: increment waiver names step `alpha` but the round log joins increment `alpha-inc1` to step `beta`
```

`h2-rehomed-derived`, the same population reached with no mis-declared `step` at all, by the increment-only record the accessor block documents (`{"task":"beta-inc1","increment":"alpha-inc1"}`):

```
prefix  exit=0  workflow invariants hold
head    exit=1  PLAN vs LOG: round log line 2: increment waiver names step `alpha` but the round log joins increment `alpha-inc1` to step `beta` (derived from a record's `task`)
```

In both, a round record DOES resolve to the waived increment, so neither is the population the entry names. The retired rule accepted them because `leading_slug("alpha-inc1") == "alpha"` equals the waiver's `step`. `mid` also refuses both, so THIS NARROWING ENTERS WITH THE IMPLEMENTATION COMMIT AND NOT WITH THE FIX PASS.

I DO NOT ENDORSE THE FINDING'S HEADLINE, "the second is the one a real project hits". I MEASURED THE FREQUENCY CLAIM AND IT IS UNPROVEN. On this repository's own log there are zero records carrying a structured `increment` with no structured `step` whose `task` disagrees with the increment, and the six increments whose resolved step differs from their id's leading slug all run the OTHER way (`decision-folder-currency-fold`, the `workflow-driver` trio, and the two blocking `workflow-enforcement-tier` fold tokens), which is the population the fix UNBLOCKS rather than narrows. The finding does not depend on frequency and I rule it on its other ground.

THE STEP'S REQUIREMENT IS NOT MET. `validation-constraints.md:145` makes it an obligation of this increment: "The narrowing is a behaviour change to `validate --workflow` and the population it affects must be named." The entry names one population and asserts it is the population. There are two, and the unnamed one is the direction in which the PRE-fix tool was silently wrong: it accepted a waiver while the records contradicted it, which is the defect the increment exists to close. A user reading the entry to decide whether upgrading breaks their plan is told the only new refusal is a dead waiver that grants nothing, and can then hit a refusal the entry gave them no way to predict.

WHY `medium` AND NOT `low`. Severity is absolute impact if left unfixed. This is not imprecise wording; it is a false exhaustive claim in the one artifact the step required to carry that claim truthfully, about a deliberate breaking change, omitting the half that actually breaks a working plan. Against `medium`: no verdict is wrong, the `CHANGELOG` does not ship into scaffolded projects, and the tool's own refusal is explicit and correct, so the surprised user is not stranded. Those bound it BELOW `high`, which is where the backstop would engage; they do not reduce an unmet, explicitly stated disclosure obligation to `low`. I rate it `medium` and it is the round's ceiling.

REMEDY, SCOPED TO THE WHOLE SENTENCE:

- `CHANGELOG.md:32`, the "THE POPULATION THIS NARROWS" sentence: name BOTH populations, an increment no round record resolves to, AND an increment the records do resolve but join to a step other than the one the waiver names. The sentence's second half, "the retired rule accepted it silently whenever the id happened to strip to the step slug", is the condition under which BOTH were accepted and needs no separate change. Fold this edit into the single `### Fixed` rework that `W2A-1` and `W2B-3` also land in.
- `CHANGELOG.md:32`, "That case grants nothing under W3 either (W3 builds its increments from the records, so an increment with none never enters the loop)": NO EDIT, and it stays true if the sentence above widens. For the second population the increment's records group under the step they join, not under the step the waiver names, so `waiver_covers_round` fails in that group too and the waiver exempts nothing under W3 either. The parenthetical's own reason is scoped to the empty case, so if the sentence is widened the reason must stay attached to the case it explains.
- `CHANGELOG.md:32`, "No waiver committed to this project's own plan is affected": NO EDIT. Verified on the live plan and log, exit 0.
- `src/workflow.rs`: NO EDIT. The refusal on the second population is correct and is the increment's whole point.

## `W2B-3`: the `CHANGELOG`'s account of the retired rule is false by one class

VALID, `low`.

THE CLAIM. `CHANGELOG.md:32`: "it refused every increment id that does not end `-inc<x>` even when the step's own round records join it to that step".

REPRODUCED, fixture `g12-retired-accepts` (Roadmap step `plan-fold`; one record joining increment `plan-fold` to step `plan-fold`; waiver `step = plan-fold`, `increment = plan-fold`):

```
prefix  exit=0  workflow invariants hold
head    exit=0  workflow invariants hold
```

`plan-fold` does not end `-inc<x>`, its records join it to the waiver's step, and the PRE-FIX binary accepted it, because the retired rule refused iff `leading_slug(increment) != waiver.step` and `leading_slug` returns such an id unchanged. So it refused every such id EXCEPT one identical to the step slug.

THE COUNTER-EXAMPLE CLASS IS DEGENERATE AND I SAY SO. It needs the increment id to equal a Roadmap step slug, which no waiver in this plan does. The finding still holds: this is the entry's account of the old behaviour, and a reader who takes "every" literally predicts a refusal that did not happen. `low` is right, and it costs one clause folded into an edit already owed to that paragraph.

REMEDY:

- `CHANGELOG.md:32`, the clause quoted above: state the retired rule itself (it required the increment id's leading slug to equal the waiver's `step`) and let the refused class follow, which also removes the need for an exception. Fold into the single `### Fixed` rework.
- The rest of the retired-rule sentence, including "its refusal named a step derived from the WAIVED INCREMENT'S id, which need not be a Roadmap step": NO EDIT. Round 1 settled that clause and it is accurate.

## `W2B-4` (with `W2A-3` merged): the new doc comments state the declaration test's scope and its routes wrongly

VALID, `low`.

SCOPE HALF, REPRODUCED. `src/workflow.rs:545` says "Each owner maps to whether ANY record declared it in a structured `step` id"; `:640` says "an owner no record declares was computed by the join"; `:589` says the named step "is READ from a record's `step` id where one exists and is DERIVED otherwise". The loop at `:645-647` is filtered to records that resolve to THIS increment. Fixture `g9-cross-increment` (a pre-migration record for `alpha-fold`, plus a record `{"step":"alpha-fold","increment":"other-inc1"}`):

```
head  ... increment waiver names step `beta` but the round log joins increment `alpha-fold` to step `alpha-fold` (derived from a record's `task`)
```

A record DOES declare `alpha-fold` in a structured `step` id and the owner is still marked derived. THE MESSAGE STAYS TRUE (that owner's attribution IS derived, and the per-increment scope is the right scope), so this is a comment defect and not a message defect. I agree with the reviewer's own framing on that point.

ENUMERATION HALF, REPRODUCED. `:551` says "SEVERAL OWNERS ARISE TWO WAYS AND NEITHER NEEDS A MALFORMED LOG" and lists two structured `step` ids, or one structured record plus one pre-migration record; the test comment at `:1688` calls its case "the first of the two routes". Fixture `f-twoderived`, two records for one increment each carrying a structured `increment` and no `step`, with different `task` values, in a log with no schema problems:

```
head  ... joins increment `alpha-fold` to steps `yyy-task` (derived from a record's `task`), `zzz-task` (derived from a record's `task`)
```

Neither record carries a structured `step`, so route one does not apply; neither is pre-migration in the sense the comment means, so route two does not apply. This is a third route, and it is the shape the accessor block at `:98-111` already documents and pins.

NEITHER REVIEWER CLAIMS NEW EVIDENCE AGAINST ROUND 1's `W1B-3`, and neither needs to. `W1B-3` was VALID and its remedy was executed; this is the same defect class landing on the replacement text, in a sentence that did not exist at round 1. That is not a re-raise and I do not treat it as one.

WHY `low`. In-code documentation only; no behaviour, no verdict, and no emitted message is false. It matters because this file's own recorded recurring defect is an enumeration that bounds a set to its own size (`validation-constraints.md:13`), and "TWO WAYS" is exactly that shape, written into the sentence that replaced the last instance of it.

REMEDY, SCOPED TO THE CLASS (every new comment that states the declaration test's scope, or counts the routes to several owners):

- `src/workflow.rs:543-555`, `step_attribution`'s whole doc comment: scope the declaration test to the increment's OWN records, and state the cause per AXIS rather than per record kind, so the enumeration is a property (a record whose structured `step` id is absent derives its step) rather than a count of record kinds.
- `src/workflow.rs:586-589`, the `w5_problems` bullet's illustration and its "READ from a record's `step` id where one exists" clause: the same restatement, over the whole passage rather than the quoted fragments. Keep the opening "BOTH AXES STILL DEGRADE PER RECORD, per the accessor block above", which is correct.
- `src/workflow.rs:637-643`, the inline comment on the owners map, including "an owner no record declares": the same scope fix.
- `src/workflow.rs:1687-1691`, the test comment's "This is the first of the two routes": moves with the enumeration.
- `src/workflow.rs:98-111`, the four-accessor block: NO EDIT. Pre-existing, accurate, and the source of truth the other comments must be made to agree with. Same ruling as round 1.
- `src/workflow.rs:655-666`, the two emitted messages: NO EDIT for this finding. Their words stay true on `g9-cross-increment`. (`W2B-6` and `W2B-7` ask them for other things.)
- `CHANGELOG.md:32`'s "a pre-migration record carries no `step` id, so `round_step_slug` still derives its step" clause: NO SEPARATE EDIT. It is a true illustration, it sits inside the sentence `W2A-1` reworks, and that rework must not re-introduce a per-record-kind framing.

## `W2B-5`: one doc comment left on the retired verb

VALID, `low`. THE WEAKEST FINDING OF THE ROUND, and I say so rather than dressing it up.

CONFIRMED BY AUDIT. `git diff 6ec9f1a..60ee7d0 -- src/workflow.rs | grep -E "^[-+].*(attribut|joins)"` shows twelve removed lines carrying "attribut" against one added line carrying it, and that one is the helper's own name. At HEAD the only prose occurrence left in the file is `src/workflow.rs:412`:

```
/// Whether `waiver` exempts the increment `round` belongs to, as the ROUND LOG
/// attributes it: ...
```

That line is a `+` line in `git diff main..HEAD`, so the diff both introduced the site and standardised every other one away from it. It is the doc comment on the predicate that DEFINES the relation both checks consult, which is the one place a retired word is most likely to be read as naming a different thing. The project's guidance on technical prose is that one word carries one meaning for the whole document.

REMEDY:

- `src/workflow.rs:411-412`, one word: "attributes" becomes "joins", the vocabulary the rest of the commit and every emitted message use.
- `src/workflow.rs:556` and `:665`, the helper name `step_attribution`: NO EDIT REQUIRED. The reviewer did not press it and neither do I: it is a private helper, the doc at `:543` already says "joins", and a rename is churn a later reader gains nothing from. If `W2B-6`'s remedy restructures the helper anyway, renaming it then is free and is the implementer's call.

## `W2B-6`: the plural refusal's trailing parenthetical does not say which owner it qualifies

VALID, `low`.

REPRODUCED, BOTH ORDERINGS. `g4-mixed-last` (a structured record joining `alpha-fold` to `alpha`, plus a pre-migration record for `alpha-fold`):

```
head  ... increment waiver names step `beta` but the round log joins increment `alpha-fold` to steps `alpha`, `alpha-fold` (derived from a record's `task`)
```

`g5-mixed-first` (an increment-only record with `"task":"aaa"`, plus a record declaring `zzz`):

```
head  ... increment waiver names step `mmm` but the round log joins increment `zeta-inc1` to steps `aaa` (derived from a record's `task`), `zzz`
```

THE SAME CODE READS UNAMBIGUOUSLY IN ONE ORDER AND AMBIGUOUSLY IN THE OTHER, and that asymmetry is the finding. A trailing parenthetical can qualify the list it follows, so a reader of `g4-mixed-last` can conclude `alpha` is derived too. `alpha` is a value a record carries verbatim, so that conclusion is exactly the mis-statement of provenance the mark was added in round 1 to prevent.

WHY `low`. No verdict, and every fact the reader needs is present in the string; what is wrong is that one grammatical reading of it is false.

REMEDY, SCOPED TO THE WHOLE PER-OWNER ARM:

- `src/workflow.rs:556-573`, `step_attribution`: bind the mark to its own slug so the trailing position cannot slide, for example by repeating the slug inside the parenthetical, or by listing the derived owners in a separate trailing phrase. The wording is the implementer's choice; what is not acceptable is a form whose correctness depends on the owner ordering.
- `src/workflow.rs:568-572`, the single-owner form: NO EDIT. Unambiguous.
- The `BTreeMap` iteration order: NO EDIT. It is pinned by the residue lens's `m7` and re-ordering would not fix the ambiguity, only relocate it.
- THE FIX IS ALREADY PINNED and needs no new test: an existing assertion carries the mixed-last string verbatim (added by the fix pass, visible in the `6ec9f1a..60ee7d0` diff), so any rewording reddens it and the implementer must update it deliberately.

## `W2B-7`: the empty-owners refusal asserts a fact about a log it could not fully read

VALID, `low`. ONE SUB-CLAIM DOES NOT GENERALISE AND I CORRECT IT.

THE CORE CLAIM, REPRODUCED. `metrics::parse_rounds` is best effort and drops a record missing a required field, so a record carrying `"increment":"<the waived id>"` can be invisible to the check while visible to the reader who greps the log. Fixture `h3-malformed`, one record `{"step":"beta","increment":"alpha-inc1"}` with `consecutive_clean` removed:

```
head  LOG:1: missing field `consecutive_clean`
      PLAN vs LOG: round log line 2: increment waiver names increment `alpha-inc1`, which no `type:"round"` record resolves to (a record resolves to its structured `increment` id, or to its `task` when that id is absent), so the round log joins it to no step
```

`h4-malformed-repaired`, the identical log with the field supplied:

```
head  PLAN vs LOG: round log line 2: increment waiver names step `alpha` but the round log joins increment `alpha-inc1` to step `beta`
```

By the message's own stated rule the line 1 record DOES resolve to `alpha-inc1`.

THE SUB-CLAIM I CORRECT. The reviewer says "The schema problem printed beside it names `valid_findings`, a different field from the one that caused the drop, so the two lines do not connect for the reader". THAT IS A PROPERTY OF THEIR FIXTURE, NOT OF THE CODE. In my `h3-malformed`, where the record is otherwise schema-complete, the reported field IS the one that caused the drop. In `h3-minimal`, a deliberately sparse record missing several fields, the reported field is `phase`, which is neither. So the disconnect is real for a sparse record and absent for a complete one. The finding does not rest on it, and I rule on the core claim.

WHY `low`. `validate` exits non-zero either way, the malformed line is always reported in the same run, and the verdict is defensible: a record the projection could not read evidences nothing. What is wrong is that the sentence states a universal about the log that a reader can falsify with one grep.

I RULE THE REMEDY THE REVIEWER DECLINED TO RULE. It records two options and leaves the choice open; my brief requires a remedy, so I take the treating option. THIS INCREMENT EXISTS BECAUSE A VALIDATOR ASSERTED WHAT IT COULD NOT VOUCH FOR, and round 1 required W5's non-empty refusal to disclose its provenance for exactly that reason. Leaving the empty-owners refusal asserting a universal it cannot vouch for would apply two different standards to two branches of one message.

REMEDY, SCOPED TO THE WHOLE SENTENCE:

- `src/workflow.rs:654-658`, the empty-owners format string: make the claim true of an unreadable record too, by scoping it to the records the log projection could read. One clause; the wording is the implementer's, and the message is already long, so prefer a tightening (for example qualifying which records the claim ranges over) to an added sentence.
- `src/metrics.rs:660-698`, `parse_rounds`'s best-effort drop: NO EDIT. Pre-existing, deliberate, and outside this increment. The finding is about the sentence, not the drop.
- The schema problem the same run prints: NO EDIT. It is not this increment's to change, and it is the mitigation that keeps this `low`.
- `src/workflow.rs:659-666`, the non-empty refusal: NO EDIT for this finding. It ranges over the owners it found and asserts no universal.

## Ruling 1: the second narrowed population, and whether the step's requirement is met

VERIFIED, and it is real. Two fixtures, two routes, `prefix` accepting at exit 0 where `head` refuses, with `mid` agreeing with `head` so the narrowing is the implementation commit's and not the fix pass's. The `CHANGELOG`'s "THE POPULATION THIS NARROWS is ..." is exhaustive in form and there are two populations, so the sentence is false as written.

THE STEP'S REQUIREMENT IS NOT MET. `validation-constraints.md:145` requires the affected population be named, and the entry names one of two, omitting the one where the pre-fix tool accepted a waiver its own records contradicted. That is the direction that breaks a working plan on upgrade, and it is the direction the increment exists to close.

I DO NOT ENDORSE THE FINDING'S FREQUENCY CLAIM. I measured this repository's own log: zero records produce the shape, and the six increments whose resolved step differs from their id's leading slug all run the other way, which is the population the fix unblocks. The finding stands on the disclosure obligation, not on how often the case occurs, and the remedy is one sentence.

## Ruling 2: are the three surviving mutations acceptable

NO. THE GAP MUST BE CLOSED, AND IT RATES `low`. Those are two separate answers and the finding needs both.

WHY NOT ACCEPTABLE. The orchestrator asked whether a message a validator emits is itself load-bearing here. IT IS, AND MORE THAN USUALLY. This step exists because W5 asserted an ownership fact the plan need not carry, and the whole of round 1's fix pass was spent making the refusal state what the records say. The derived mark IS the mechanism that makes the refusal vouchable. Mutation B reproduces, green, the exact recorded `src/` defect the plan says inc1 closes: it presents two `leading_slug` products as steps the records declare. A build that regresses the increment's headline promise and passes 385 of 385 tests plus clippy has no guard on the thing the increment ships. The remedy is two test edits and I measured that one fixture kills two of the three mutations, so the cost is close to nothing against this project's own thesis that a claim owes a demonstration.

WHY IT STILL RATES `low`. Severity is absolute impact if left unfixed, and the four-level scale rates the finding rather than the mutation used to expose it. Left unfixed, the shipped tool emits a correct message on every tree I ran; the exposure is that a later edit to the owners map ships green. That is strictly a tier below round 1's `W1A-1`, where the unpinned axis was VERDICT-bearing and the same argument earned `medium`. I verified the tier difference rather than assuming it: across eleven fixtures the three mutations return identical exit codes and identical problem counts to `head`, because the owners map is built inside the refusal branch and decides only which message is pushed. A defect that cannot change a verdict does not reach `medium` on this project's own calibration.

## Ruling 3: the shipped rule text ships into every scaffolded project, so does the blast radius make it more than `low`

NO. IT IS `low`, and the reason is the direction of the error rather than the size of the audience.

The clause is wrong only where one record of an increment declares a step in a structured `step` id and another reaches the same value through the shim. In that case the owner is unmarked, and the shipped clause promised a mark. THE UNMARKED VALUE IS ONE A RECORD GENUINELY CARRIES: `declared` is true only for a record whose `step` is `Some`, and `round_step_slug` returns exactly that value, so an unmarked owner is always a byte-identical `step` value on some record of that increment. A reader of the refusal who sees no mark and concludes the log states that step is CORRECT. The rule text over-promises the marking and never lets a computed value pass as a recorded one.

So the blast radius is wide and the harm per instance is close to zero: no verdict moves, no emitted message is false, and the discrepancy is visible only to a reader who builds the two-record case and predicts a mark that does not appear. Compare round 1's `W1B-4`, also shipped rule text in all three copies, also rated `low`, where the omission was likewise safe-direction. `medium` would require the text to mislead in the direction that costs something, and it does not.

THE BLAST RADIUS DOES CHANGE THE REMEDY'S HANDLING, and I say so rather than leaving it implicit: the three copies must move in one commit or the drift guard fails (re-demonstrated above), and acceptance item 7b must be re-run afterwards because its replacement wording sits inside the sentence being changed.

## Overall assessment

THE ROUND'S REAL RESULT: NOT CLEAN. Nine valid findings after deduplication, ceiling `medium`, no `high` and no `critical`, no dismissal and so no backstop re-check owed. The round outcome is `new_valid` and the consecutive-clean streak stays at 0 against the two a `risky` artifact needs.

THE SHIPPED BEHAVIOUR IS CORRECT, AND I MEASURED IT RATHER THAN ACCEPTING IT. Fifteen fixtures across three binaries plus the live plan and log, with the expected verdict computed by hand from the documented accessors: `head` matched my expectation on all fifteen, `mid` and `head` agreed on all fifteen (so the fix pass changed no verdict), and `prefix` differed from `head` on exactly four, every one of them a documented direction (iii) narrowing or unblocking. The live plan is green at exit 0 over 319 records, 96 steps and 70 questions; `cargo test` is 429 green across nine binaries; clippy is clean; acceptance item 7b reports 0, 0, 0 and 1, 1, 1.

EVERY VALID FINDING IS IN WHAT THE CHANGE SAYS OR IN WHAT PINS IT. Eight are claim defects: three shipped prose copies plus three `CHANGELOG` sentences, four in-code doc comments, one message wording, one message universal, and one retired verb. One is a test-coverage gap over correct code. Not one is a defect in what the tool does. That is the same result round 1 reached by a different route, and two independent lenses reaching it twice is stronger evidence than either reaching it once.

SAFE TO MERGE ONCE THE REMEDIES LAND? YES. No remedy changes a verdict, and none touches `waiver_covers_round`, `round_step_slug`, the owners map, or any check's logic. The whole fix surface is: one sentence in three drift-guarded prose copies, one `### Fixed` paragraph reworked in a single pass (which discharges `W2A-1`, `W2B-2` and `W2B-3` together), five in-code comment restatements, two message edits, one word, and two test additions. The only judgement inside the set is `W2B-7`'s treat-or-leave fork, which I ruled rather than passing on, and it sits inside inc1's own review question rather than opening a new design decision.

WOULD FURTHER ROUNDS FIND DEFECTS IN BEHAVIOUR OR ONLY IN PROSE? MY HONEST JUDGEMENT IS ONLY IN PROSE, AND THE ORCHESTRATOR SHOULD HAVE THAT PLAINLY. Two rounds and five independent lenses have now searched for a wrong verdict and none has found one; round 1 measured sixteen trees, the residue lens measured thirty-six with an independently computed expectation, and I measured fifteen more of my own construction. The behaviour surface here is small (one predicate over two accessors) and it is now pinned on both consumers by `m10` and `m11`. What keeps producing findings is the DENSITY OF CLAIMS this change ships: three drift-guarded prose copies, a long `CHANGELOG` paragraph, and roughly a dozen doc comments that each restate the same relation in slightly different words. Every restatement is a new place to be wrong, and each round's fix pass writes more of them, which is visibly what happened between round 1 and round 2 (four of this round's nine findings are against sentences the round 1 fix pass wrote).

THE PRACTICAL CONSEQUENCE, offered as an observation and not a convergence recommendation, which is not mine to make: a fix pass that closes these nine by ADDING further explanatory prose is likely to generate a round 3 of the same shape. A fix pass that closes them by making each restatement SHORTER and pointing at the accessor block as the single source, rather than paraphrasing it a fourth time, attacks the mechanism producing the findings. Principle 16 is the one that bites here, and it bites on the prose as much as on the code.

TWO THINGS THE ORCHESTRATOR STILL OWES, NEITHER A FINDING AGAINST THIS ARTIFACT, both unchanged from round 1. Acceptance item 3, the plan-side unblocking (the two `[[step.increment]]` declarations, the two owed waivers, the `workflow-enforcement-tier` status flip), is still absent from `git diff main..HEAD`, and the step assigns those edits to the orchestrator and the planner. And the post-merge planner pass owes the sidecar three specific facts this round measured: item 5's sentence needs re-scoping to the empty-owners branch (W5 still reports a derived step on the non-empty branch, by design and now marked), item 7b's replacement-wording grep is now "some `round` record must join that increment to that step", and the acceptance list has no item reaching the message's provenance inputs, which is why three mutations of them pass every item on it.
