# Q-70 capture, round 3, reviewer: quotation and handle integrity

Lens: every string `Q-70` presents as another document's words, every find-by-quoted-text handle, every `file:line` citation and range, every command the item tells a reader to run, and any quotation that is accurate but misleading in its new context.

Artifact: `git diff main..HEAD` on `review/q70r3-quotes`, HEAD `61b1d35`. The change is the `[[question]] Q-70` entry in `docs/plans/agent-scaffold.plan.toml`, the `q70-capture` token in `[meta].orphan_tasks`, the empty sidecar `docs/plans/agent-scaffold.questions/Q-70.md`, and the regenerated `docs/plans/agent-scaffold.md`.

LEDGER STATE PINNED. Every resolution below is against `docs/plans/agent-scaffold.ledger.md` AS IT IS IN THIS WORKTREE at `61b1d35`. `git diff main..HEAD -- docs/plans/agent-scaffold.ledger.md` is empty, so the ledger here is identical to main's at the time this worktree was cut. The ledger is a moving target and these counts move with it.

Binary: `target/debug/agent-scaffold` built from this worktree at HEAD. Fixture root: `<scratch>/r3quotes/`. Nothing outside that directory was written or deleted. All commands were run under `bash` (`R2C-1` settled that the project's configured shell is `bash`, so `nu` failures are not a defect and are not re-raised).

RESULT: TWO FINDINGS, one `medium` and one `low`. NO finding at `high` or `critical`; I looked for them and did not find any.

THE MECHANISM MOSTLY HOLDS. Of 44 resolvable quotations and 42 citations enumerated below, 43 quotations resolve and all 42 citations resolve and say what is claimed. Every range was checked against its block and NONE overshoots, which matters because two range-overshoot defects are already live in this loop. Both reproduction fixtures reproduce byte-for-byte. All six commands run and their output supports the sentence around them. The one broken item is a uniqueness ASSERTION about a handle, not a dangling quotation.

---

## Findings

### R3A-1. The handle `Q-70` picks precisely because it "resolves uniquely" resolves to two paragraphs, and the first hit is the wrong one

SEVERITY: `medium`.

THE CLAIM IN THE ITEM, at `docs/plans/agent-scaffold.plan.toml:1895` (`THE COUPLING HYPOTHESIS`):

> Find that paragraph by the quoted text "THE MEMBERS KNOWN AT THIS WRITING", which resolves uniquely, rather than by a line number or by the fix quotation itself, which now resolves to two paragraphs because the ledger's own round records quote it.

THE MEASUREMENT, in this worktree:

```
$ grep -cF "THE MEMBERS KNOWN AT THIS WRITING" docs/plans/agent-scaffold.ledger.md
2
$ grep -cF "teaching W5 the structured step association W3 already uses" docs/plans/agent-scaffold.ledger.md
2
$ grep -nF "THE MEMBERS KNOWN AT THIS WRITING" docs/plans/agent-scaffold.ledger.md | cut -d: -f1
567
587
```

Both handles resolve to 2. The distinction the sentence draws between them, the whole reason the item tells a reader to prefer one over the other, does not exist.

THE FIRST HIT IS NOT THE TARGET. Line 587 is the `validation-constraints` routing paragraph, the intended target and the one that carries the authoritative member set. Line 567 is `ORCHESTRATOR DEFECTS (19) AND (20)`, which quotes the handle inside defect (20)'s own text: `as the round 2 fix pass did for "THE MEMBERS KNOWN AT THIS WRITING", which resolves to 1`. A reader following the item's instruction lands on 567 first, reads the ledger asserting the handle resolves to 1 while their own `grep -cF` returned 2, and has to work out which paragraph was meant.

THIS BREAKS BOTH USE SITES. The same handle is the finding instruction at `:1899` too (`DEFERRED INPUTS`): "The authoritative set is the ledger's `validation-constraints` routing paragraph, found by the quoted text "THE MEMBERS KNOWN AT THIS WRITING"". At that site the first hit does not contain the member set at all, so a reader who stops at hit one gets no members.

THE CLAIM WAS ALREADY FALSE WHEN WRITTEN, so this is a `Q-70` defect and not a mid-loop ledger move by the orchestrator:

```
$ for c in 4351e6c 198556e 61b1d35; do
    echo "$c ledger=$(git show $c:docs/plans/agent-scaffold.ledger.md | grep -cF 'THE MEMBERS KNOWN AT THIS WRITING') \
    q70claim=$(git show $c:docs/plans/agent-scaffold.plan.toml | grep -cF 'which resolves uniquely')"
  done
4351e6c ledger=2 q70claim=0
198556e ledger=2 q70claim=0
61b1d35 ledger=2 q70claim=1
```

`4351e6c` ("docs: record the round 2 fix pass and defect (21)") took the ledger from 1 to 2, and it is an ancestor of `61b1d35`, the round 2 fix pass that wrote "which resolves uniquely". The fix pass asserted a measured property of a source it could see, without measuring it, in the paragraph that instructs its reader to reference by quoted text rather than by position.

WHY `medium` AND NOT `low`. The handle still resolves, so nothing is lost outright. But this item deliberately makes quoted text its whole referencing mechanism, it is the artifact the design pass reads to find the authoritative input set, and this is the second time in two rounds that a uniqueness property of a handle into this exact ledger has gone stale unnoticed (orchestrator defect (21) was the first). An instruction that names a wrong first hit is worse than no instruction, because it discourages the reader from checking. It is not `high` because both hits are near each other, both concern the same subject, and a reader who reads both recovers.

WHY IT IS NOT AN INSTANCE OF A SETTLED FINDING. `R2B-5` (VALID, `low`) was about the item using the WRONG LABEL for this paragraph ("current next-action paragraph" naming the superseded one) and was fixed by relabelling it the routing paragraph. This is a different defect in a different clause: the label is now correct, and what fails is the uniqueness assertion attached to the handle. No settled verdict covers it.

TWO MEASURED REMEDIES, either of which resolves to exactly 1:

```
$ grep -cF "THE MEMBERS KNOWN AT THIS WRITING. (a)" docs/plans/agent-scaffold.ledger.md
1
$ grep -c '^THE IMMEDIATE NEXT ACTION IS THE `validation-constraints` STEP' docs/plans/agent-scaffold.ledger.md
1
```

The second is the form the item ALREADY uses successfully twice ("the paragraph beginning ..."), and it is the more durable one: a paragraph-opening handle read with `grep -c '^...'` cannot be decayed by the ledger quoting itself mid-paragraph, which is exactly how this one decayed. See the source-document note below, where the same property is measured for the item's two other paragraph-beginning handles.

The narrower fix is to drop the words "which resolves uniquely" and keep the handle. That removes the false claim but leaves the reader on the wrong first hit, so it is the weaker of the two.

### R3A-2. `src/agents_md_drift.rs:41-55` starts one line inside the block it names

SEVERITY: `low`.

At `docs/plans/agent-scaffold.plan.toml:1901`: "the two drift-guarded generated files `AGENTS.md:147` and `.agents/AGENTS.reference.md:147` (the guard is `src/agents_md_drift.rs:41-55`)".

The block being cited begins at line 40, not 41:

```
40	//! GUARDED SET. Three comparisons make up the drift coverage; the other tests in this
41	//! file exercise the helpers and add none.
```

A reader who opens `41-55` reads "file exercise the helpers and add none." as the first line, a sentence fragment with the label `GUARDED SET` and the three-comparisons statement cut off. `40-55` is the block.

THIS IS AN UNDER-START, NOT AN OVERSHOOT, and the surrounding claim holds either way: lines 43 to 46 list the three comparisons, checks 1 and 2 are `AGENTS.md` and `.agents/AGENTS.reference.md` against a fresh pack render, and the test `the_committed_scaffold_matches_a_fresh_render` (`src/agents_md_drift.rs:375`) asserts both, so a change to the W5 clause that moves only one of the three sites does fail it. I raise it because the brief asks every range to be checked and this one is off, not because the sentence it supports is wrong. The triager may reasonably rate it below the bar.

---

## Table A. Quotations into `docs/plans/agent-scaffold.ledger.md`

Resolved with `grep -cF` against the ledger in this worktree. "Lines" is the `grep -cF` count; the ledger is not hard-wrapped, so one line is one paragraph.

| # | quoted text | lines | target | verdict |
| --- | --- | --- | --- | --- |
| A1 | `TWO WAIVERS ARE OWED AND CANNOT YET BE WRITTEN` | 1 | 591 | RESOLVES, unique. Paragraph carries the drafted notes for `-w5` and `-w6` as claimed. |
| A2 | `THE MEMBERS KNOWN AT THIS WRITING` | 2 | 587 (target), 567 (decoy) | RESOLVES but NOT unique. See `R3A-1`. |
| A3 | `teaching W5 the structured step association W3 already uses` | 2 | 587, 555 | RESOLVES. The item states 2 and 2 is measured. |
| A4 | `A FOURTH \`agent-scaffold next\` DEFECT` | 4 | 997 | RESOLVES. Used as a PARAGRAPH-BEGINNING handle: `grep -c '^A FOURTH ...'` returns 1. Unique in the form the item uses. |
| A5 | `belongs to the validation-constraints step with the other three` | 3 | 997 | RESOLVES. Line 997 states it verbatim, dated 2026-08-01 as claimed. |
| A6 | `THREE DEFECTS IN \`agent-scaffold next\`` | 3 | 1363 | RESOLVES. Paragraph-beginning form `grep -c '^THREE DEFECTS ...'` returns 1. Unique as used. |
| A7 | `\`Q-55-resumecost\` DECIDED` | 1 | 879 | RESOLVES, unique. Paragraph names `src/main.rs:project_root_of_source` and the queued shared cause as claimed. |
| A8 | `NO OWNER anywhere in the plan` | 1 | 909 | RESOLVES, unique. |
| A9 | `THE BACKSTOP CORRECTED BOTH EARLIER AGENTS ON OWNERSHIP` | 1 | 909 | RESOLVES, unique. Same paragraph as A8, which is the item's claim. |
| A10 | `29 of 51` | 3 | 539, 587, 651 | RESOLVES. The item says the durable record "carried" it; it does. |
| A11 | `about eleven` | 1 | 629 | RESOLVES, unique. Line 629 is the `Q-55-check21b` record; the item quotes the right one of the two live phrasings (`roughly eleven` also exists, at 559, 565 and 651, and the item does not claim it). |
| A12 | `four` (the ledger says four) | n/a | 997 | RESOLVES. Property claim, not a count claim. Verified at 997. |
| A13 | `three` (the ledger says three in several places) | 6 by the item's own grep | 533, 569, 1071, 1275, 1277, 1353 | RESOLVES. Both places the item names by hand are present: 533 is the `Q-55-entryroute` record, 1353 is the human decision of 2026-07-30. |
| A14 | `an optional \`project\` field on \`Round\` and on the plan's \`[meta]\`, filtering the join in \`check_workflow_toml\`` (relayed, not quote-marked) | 1 | 1089 | RESOLVES verbatim. |
| A15 | orchestrator defect (12), the standing cure on moving counts | 1 | 885 | RESOLVES. Paragraph states the cure as characterised. |

## Table B. Quotations into `docs/metrics/workflow.jsonl` and `docs/plans/agent-scaffold.plan.toml`

| # | quoted text | claimed source | verdict |
| --- | --- | --- | --- |
| B1 | `Design pass, validator cluster only` | `Q-55-entryroute` `chosen` | RESOLVES. `jq` confirms `chosen` and `recommendation` both equal it. |
| B2 | `Split out the W5 fix first` | one of the three alternatives it beat | RESOLVES. `options` has 4 entries, the chosen plus 3, and this is one of the 3. |
| B3 | `must be made with W6 in view` | the record's stated human ground | RESOLVES in the ledger's `Q-55-entryroute` paragraph (533). |
| B4 | `Prefer the cleaner long-term architecture over the smallest diff` | Project Principle 1 | RESOLVES twice: `plan.toml:1907` under `n = 1`, and attached to the W6-in-view clause at ledger 533. |
| B5 | `Ground decisions in evidence` | Project Principle 6 | RESOLVES: `plan.toml:1932` under `n = 6`, and attached at ledger 533 to the never-measured coupling claim, exactly as the item relays. |
| B6 | `Minimal by default` | Project Principle 2 | RESOLVES: `plan.toml:1912` under `n = 2`, and attached at ledger 533 to the no-open-design-space clause. The ledger's clause covers "the two inc3 defects plus the three `next` defects", which is the item's (a) and (b) and nothing after, so the item's "THAT CLAUSE SCOPES TO (a) AND (b) ONLY" is exact. |
| B7 | `Anchor plus refusal, identity queued` | `Q-55-mechanism` `chosen` | RESOLVES. |
| B8 | `Anchor, refusal, and identity fields now` | its declined wider option | RESOLVES. In `options`; the ledger at 1097 confirms the human declined the wider and the narrower. |
| B9 | `Accept as (iv), queue the shared cause` | `Q-55-resumecost` `chosen` | RESOLVES. |
| B10 | `session_state enum + a W6 transition-legality check, no commands` | `Q-59`'s `ask` | RESOLVES. `grep -cF` returns 2 in the plan TOML: `Q-59` at 1774 and `Q-70`'s own quotation of it. |
| B11 | the `Q-55-impactclaim` ruling on a list that enumerates its own exclusions | ledger 667 | RESOLVES. Source states "A DOCUMENTATION-IMPACT LIST THAT ENUMERATES ITS OWN EXCLUSIONS IS A COMPLETENESS CLAIM, AND A COMPLETENESS CLAIM ... CANNOT BE KEPT TRUE BY ANY AMOUNT OF DILIGENCE". The item's paraphrase, including "any amount of diligence", is faithful. |
| B12 | `Q-55-check21b`'s ruling that the citations are deliberately left stale | ledger 629 plus the receipt | RESOLVES. Receipt `chosen` is "Revert the one re-point"; the ledger states the choice deliberately restores a false citation so all of them belong to the owning step together. |
| B13 | the discipline `Q-69` records for its own case (no options, no recommendation while `exploring`) | `plan.toml:1864` | RESOLVES. `Q-69`'s `ask` states the option set is WITHDRAWN and the directions carry no recommendation. |
| B14 | the shape `Q-68` and `Q-69` use for a closing NOT-DECIDED paragraph | `plan.toml:1855-1859`, `:1878` | RESOLVES as a shape claim, not a quotation. `Q-68` carries "A DESIGN PASS IS OWED (human-directed, 2026-07-26) ... No receipt and no steps yet"; `Q-69` carries the `NOT DECIDED, and NO STEP YET, deliberately` paragraph. |

## Table C. Quotations of source text and of program output

| # | quoted text | claimed source | verdict |
| --- | --- | --- | --- |
| C1 | `leading_slug(increment) != waiver.step` | `src/workflow.rs:564` | RESOLVES byte-exact at 564. |
| C2 | `step: step.slug.clone()` | `src/workflow.rs:258` | RESOLVES byte-exact at 258. |
| C3 | `one behaviour, two data representations` | the comment at `src/plan/source.rs:785-790` | RESOLVES, `grep -cF` returns 1, at line 790. |
| C4 | `waiver \`workflow-enforcement-tier-w5\` on step \`workflow-enforcement-tier\` names increment \`workflow-enforcement-tier-fold\`, which is not one of the step's increments` | `validate --workflow` on the injected fixture, source path | REPRODUCED byte-exact. See the fixture below. |
| C5 | `TOML waiver \`workflow-enforcement-tier-w5\`: increment waiver names step \`workflow-enforcement-tier\` but increment \`workflow-enforcement-tier-fold\` belongs to step \`workflow-enforcement-tier-fold\`` | same run, W5 path | REPRODUCED byte-exact. |
| C6 | `no plan source` | `agent-scaffold next` with `--source` omitted | RESOLVES. `src/main.rs:1712` holds the literal; running `next` with no `--source` prints `source: no plan source` and exits 0, so "the calm exit" is exact. |
| C7 | the question, the design space, per-option trade-offs against the numbered Project Principles, a recommendation with its reasoning, and an explicit `what not to build` | the Design explorations rule in `pack/AGENTS.md` | RESOLVES at `pack/AGENTS.md:65`, all five components present in that order. The naming rule `<q-id>.md` / `<q-id>-<disambiguator>.md` is there too. |
| C8 | W5's ownership clause, "an `increment`-unit waiver's `step` must own its `increment` (the increment's leading slug equals the step)" | `pack/instrument.md:11`, `AGENTS.md:147`, `.agents/AGENTS.reference.md:147` | RESOLVES at all three, and the three lines are byte-identical to one another. |
| C9 | acceptance check 21 instructs opening each `file:line` and showing the named subject is there | the check the quotation resolver would automate | RESOLVES at `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:345`. |

## Table D. Find-by-quoted-text handles, and how many places each resolves to

The item's own convention is that a handle must FIND its target. Multiplicity is only a defect when the item asserts uniqueness, or when the reader cannot tell which hit is meant.

| handle | hits | which is the target | can a reader tell? |
| --- | --- | --- | --- |
| `TWO WAIVERS ARE OWED AND CANNOT YET BE WRITTEN` | 1 (591) | 591 | Yes, unique. |
| `THE MEMBERS KNOWN AT THIS WRITING` | 2 (567, 587) | 587 | NO. First hit is the decoy, and the item asserts uniqueness. `R3A-1`. |
| `A FOURTH \`agent-scaffold next\` DEFECT` | 4 raw (539, 567, 587, 997), 1 anchored | 997 | Yes. The item says "the paragraph BEGINNING", and `grep -c '^...'` returns 1. |
| `THREE DEFECTS IN \`agent-scaffold next\`` | 3 raw (567, 587, 1363), 1 anchored | 1363 | Yes, same paragraph-beginning form. Note the target is the LAST raw hit, not the first, so the anchoring is doing real work here. |
| `\`Q-55-resumecost\` DECIDED` | 1 (879) | 879 | Yes, unique. |
| `THE BACKSTOP CORRECTED BOTH EARLIER AGENTS ON OWNERSHIP` | 1 (909) | 909 | Yes, unique. |
| `NO OWNER anywhere in the plan` | 1 (909) | 909 | Yes, unique, and it is the same paragraph the handle above reaches, which is what the item claims. |
| `teaching W5 the structured step association W3 already uses` | 2 (555, 587) | 587 | Yes. The item explicitly says this one resolves to two and tells the reader to use the other handle instead. Its stated reason ("the ledger's own round records quote it") is accurate: 555 is the round 1 lens-calibration record. |

## Table E. Every `file:line` citation, resolution and range check

Verified by opening each range and comparing against the sentence that cites it. "Range" reports whether the cited range matches the block it names.

| citation | claimed subject | resolves | range |
| --- | --- | --- | --- |
| `src/workflow.rs:64-68` | the doc for the `-inc<alnum>` strip and the `-incA` / `-incB` form | Yes | Exact, the doc paragraph. |
| `src/workflow.rs:88` | `leading_slug` | Yes, `fn leading_slug` is at 88 | n/a |
| `src/workflow.rs:119` | `round_step_slug`, W3 prefers the structured `step` | Yes, and 120 is `round.step.as_deref().unwrap_or_else(...)` | n/a |
| `src/workflow.rs:127` | `round_increment_id` | Yes | n/a |
| `src/workflow.rs:141` | `escalation_increment_id`, the `task` fallback | Yes, and its doc says W5's increment-unit scope check keys off it | n/a |
| `src/workflow.rs:206-221` | `run_checks` holds `rounds`, hands them to `w3_problems` and not to `w5_problems` | Yes, 217 vs 219 | Exact, `fn` to closing brace. |
| `src/workflow.rs:237-267` | `waivers_from_toml` | Yes | Exact, `fn` to closing brace. |
| `src/workflow.rs:258` | `step: step.slug.clone()` | Yes | n/a |
| `src/workflow.rs:321` | W4 skips a non-decided-fold status | Yes, `starts_with(QUEUE_FOLD_PREFIX)` guard | n/a |
| `src/workflow.rs:445-447` | W3 skips a step that is not `complete` | Yes | Exact, the three lines of the guard. |
| `src/workflow.rs:450` | a step-unit waiver is consulted only when `matching.is_empty()` | Yes | n/a |
| `src/workflow.rs:498-502` | W3's own covering-waiver match keyed on increment plus step | Yes | Exact, the closure. |
| `src/workflow.rs:549` | W5 derives a slug set from the steps | Yes | n/a |
| `src/workflow.rs:553` | W5's real-Roadmap-step rule | Yes | n/a |
| `src/workflow.rs:564` | the lexical ownership comparison | Yes | n/a |
| `src/plan.rs:55-60` | `plan::Step` is `slug` and `status` only, and is `Serialize` | Yes, derive at 54 | Exact, struct open to close. |
| `src/plan/source.rs:279-300` | the TOML typed `Waiver` | Yes, and it carries NO `step` field, which is what makes escape route 2 structural | Exact, struct open to close. |
| `src/plan/source.rs:422-430` | `step_views()` drops the increments | Yes | Exact, `fn` to closing brace. |
| `src/plan/source.rs:475-477` | `is_kebab_case_token` forbids an uppercase byte | Yes | Exact, `fn` to closing brace. |
| `src/plan/source.rs:785-790` | the comment introducing the waiver-integrity block | Yes | Exact, the six comment lines. |
| `src/plan/source.rs:791-856` | the whole per-step waiver loop | Yes | Exact. 791 opens `for step in &plan.steps`, 855 closes the inner loop, 856 closes the outer. Round 1's `R1A-5` under-cite is properly fixed. |
| `src/plan/source.rs:792-793` | building the declared-increment set | Yes | Exact, the two-line `let`. |
| `src/plan/source.rs:807-811` | the membership check | Yes | Exact, the match arm. |
| `src/plan/source.rs:807` (bare) | same | Yes | n/a |
| `src/metrics.rs:539-601` | the JSONL `type:"waiver"` arm of `check_record` | Yes | Exact. 539 is `"waiver" => {`, 601 closes it. |
| `src/metrics.rs:620-651` | `Round` carries no `project` | Yes. Fields are exactly `line`, `task`, `artifact`, `outcome`, `consecutive_clean`, `risk_class`, `step`, `increment`, as the item lists them | Exact, struct open to close. |
| `src/main.rs:582-585` | `PlanProjection.steps` is `Vec<plan::Step>` and is `Serialize` | Yes | Precise rather than short: 582 is the derive, 585 is the `steps` field the citation names. |
| `src/main.rs:project_root_of_source` | the fallback to the plan's own parent | Yes, `fn` at 1311, fallback `parent.to_path_buf()` at 1327 | n/a |
| `src/next.rs:517-523` | the parity property withholding declared-increment data | Yes | Exact, the doc paragraph. |
| `src/next.rs:520` | "the Markdown substrate (which declares no increments)" | Yes | n/a |
| `src/next.rs:551` | "The Markdown substrate declares no increments." | Yes | n/a |
| `src/agents_md_drift.rs:41-55` | the drift guard's coverage | Yes | UNDER-STARTS BY ONE LINE. Block opens at 40. `R3A-2`. |
| `pack/instrument.md:11` | the W5 ownership clause in the pack source | Yes | n/a |
| `AGENTS.md:147` | the same clause, generated | Yes | n/a |
| `.agents/AGENTS.reference.md:147` | the same clause, generated | Yes | n/a |
| `pack/AGENTS.md` (Design explorations rule, no line) | the rule | Yes, line 65. Dropping the line number is settled DISMISSED as `R1B-3` and is not re-raised | n/a |
| `pack/plan-template.plan.toml` | a commented `[[step.waiver]]` example | Yes, line 44 | n/a |
| `src/plan/render.rs` `waiver_note` | writes each waiver into the generated `<task>.md`, pinned by `render --check` | Yes, `fn waiver_note` at 516, called per waiver at 484 | n/a |
| test `an_uppercase_increment_id_is_flagged` | pins the uppercase refusal | Yes, `src/plan/source.rs:1221`, asserts on `a-incA` | n/a |
| test `the_committed_scaffold_matches_a_fresh_render` | fails if the three W5-clause sites drift apart | Yes, `src/agents_md_drift.rs:375`, asserts both generated files against a fresh pack render | n/a |
| `just scaffold-self`, second line runs `nix fmt` | regeneration | Yes, `justfile:47` is the render, `justfile:48` is `nix fmt` | n/a |
| `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md` | the `src/checks.rs` citation site | Yes, file exists and carries 15 distinct citations | n/a |

## Table F. Every command the item tells a reader to run

Run under `bash` in this worktree.

| command | runs | output supports the sentence |
| --- | --- | --- |
| the `jq ... \| sort -u \| awk` affected-population pipeline (`:1885`) | Yes, exit 0 | Yes. Returns 6 members including both fold tokens the item names as blocking, and all three `workflow-driver-stage*` ids the item names as the live declared case. |
| `grep -onE "[(][0-9]+(, [0-9]+){1,6}[)]" docs/plans/agent-scaffold.plan.toml` (`:1893`) | Yes, exit 0 | Yes. Four hits, at `plan.toml:1331`, `:1340`, `:1349`, `:1358`, and each one IS a `note` field on a `[[step.waiver]]`, which is the convention the sentence claims. |
| `jq -r 'select(.type=="decision") \| .q_id' ... \| sort -u` plus the set difference (`:1893`) | Yes | Yes. 62 distinct receipt ids against 70 registered questions gives 40 dangling. Consistent with the item's "materially different" from the recorded "29 of 51". All 40 are `Q-55-<suffix>`, so "dominated by" holds (it is in fact all of them, which the item's phrasing does not contradict and which is the safer wording for a moving set). |
| `grep -oE "src/checks[.]rs:[0-9]+(-[0-9]+)?" ... \| sort -u` (`:1893`) | Yes | Yes. 15 distinct citations. See the universal check below. |
| `grep -niE "three .?(agent-scaffold )?next.? defects" docs/plans/agent-scaffold.ledger.md` (`:1899`) | Yes | Yes. 6 hits, and BOTH passages the item names by hand are among them: 533 is the `Q-55-entryroute` record it cites as its own authority, 1353 is the human decision of 2026-07-30 it names as the routing authority. |
| `jq -r 'select(.type=="round") \| (.increment // .task)' ... \| sort -u` (`:1895`) | Yes | Yes. Returns the identity set the direction (i) comparison needs. |

## The two universal claims, checked exhaustively rather than sampled

EVERY `src/checks.rs` CITATION IS STALE: CONFIRMED, all 15. Each cited range was opened and compared against the subject the step file names for it.

| citation | subject the step file names | what is actually there | stale |
| --- | --- | --- | --- |
| `:78` | the `RUNNER_PREFIX` constant | `PathBuf,` in an import list. The constant is at 98 | Yes |
| `:329-342` | the `WorktreeGuard` cleanup that unregisters and `remove_dir_all`s | `impl From<io::Error> for RunError` plus the guard's doc comment. `impl Drop for WorktreeGuard` is at 352 | Yes |
| `:388-392` | "no libc pulled in just for a `kill(pid, 0)`" | `fn git_command`. The quoted text is at 413 | Yes |
| `:400-405` | `owning_pid` reading the first `-` segment | `git_output`'s args. `fn owning_pid` is at 561 | Yes |
| `:407-461` | the pid-liveness gate ON the startup prune | contains the helper `pid_is_alive` (416-421) but not the prune, which is `fn prune_orphan_worktrees` at 588. The rest of the range is `claim_dir`, `NEXT_RUNNER_SEQ` and `RUNNER_RESERVE_ATTEMPTS` | Yes, the weakest member: the range does contain the liveness helper, so a resolver that matched on the helper alone could disagree |
| `:425-428` | the prune's benign pid-reuse edge | the `NEXT_RUNNER_SEQ` doc about separating two THREADS | Yes |
| `:791-792` | the inline `format!` at a worktree-NAME construction site | a `format!` building a `git ls-files` ERROR message | Yes, wrong `format!` |
| `:795-800` | `RunError::WorktreeSetup` in `run()` | the closing parens and `let files = ...`. `RunError::WorktreeSetup` is at 792 | Yes |
| `:845-847` | a doc comment stating "the process id in the path already provides per-process uniqueness" | a function signature and the start of a `Command::new("sh")` builder | Yes |
| `:848-852` | `nanos()` | more of the same builder. `fn nanos` is at 1023 | Yes |
| `:862-871` | scratch helpers discriminating by a per-test literal name | stdout/stderr combining in `run_one_check` | Yes |
| `:1438-1442` | `dead_pid()`, the `u32::MAX` constant | `scratch("paths-skip")` and `init_repo`. `fn dead_pid` is at 1613 | Yes |
| `:1462` | a fixture building a `{RUNNER_PREFIX}{pid}-{nanos}` name by hand | `init_repo(&dir);` | Yes |
| `:1491` | the one fixture using the live pid | `init_repo(&dir);` | Yes |
| `:1492` | a constant-pid fixture | `write_config(&dir, ...)` | Yes |

The item's sentence, "EVERY distinct `src/checks.rs` citation ... resolves to unrelated content in the current source, so the resolver's opening red-list is all of them rather than a subset", HOLDS under the reading that a citation resolves when the range contains the construct the sentence names at it. I flag `:407-461` as the one a differently built resolver might score green, because it does contain `pid_is_alive`; it does not contain the prune the sentence attributes the gate to.

THE TWO-PATH MEASUREMENT: REPRODUCED EXACTLY, both cases. Fixture at `<scratch>/r3quotes/fix/`, a copy of `docs/` with the owed waiver injected under the `workflow-enforcement-tier` step exactly as the item specifies (`unit = "increment"`, `increment = "workflow-enforcement-tier-fold"`, `reason = "accepted-at-escalation"`, `evidence_tier = "record-backed"`, `evidence = "workflow-enforcement-tier-fold"`).

UNDECLARED: exit 1, exactly two problems, both quoted strings byte-exact (C4 and C5 above). DECLARED, the same fixture with `workflow-enforcement-tier-fold` added as a `[[step.increment]]` with `risk_class = "risky"`: exit 1, exactly ONE problem, the W5 one; the `src/plan/source.rs` problem disappears. That is the item's double-lock claim, its "escape route 4 is CONFIRMED BY MEASUREMENT" claim, and its "the two paths return opposite verdicts on the same waiver" claim, all three reproduced.

The declared run also independently confirms a claim the item makes elsewhere: "ONLY THE OWNERSHIP CHECK BLOCKS THEM". With the waiver declared, the record-backed evidence join did NOT fire, so the `type:"escalation"` records do resolve through the `task` fallback and pass, exactly as `WHAT IS BLOCKED ON IT` states.

## Other measured claims checked in passing, all of which hold

- FIVE `type:"round"` records for each fold token, every one carrying structured `step` `workflow-enforcement-tier` and NO structured `increment`; peak `consecutive_clean` 1 for the plan fold and 0 for the endproperty fold. All confirmed by `jq`.
- Each fold token has a `type:"escalation"` record whose `task` equals it, `human_decision` is `decision`, and which carries no structured `increment`. Confirmed.
- `blocked_by = []` on all 95 steps and zero populated. Confirmed: 95 `[[step]]` blocks; `grep -c 'blocked_by = \[\]'` returns 96, of which one is `Q-70`'s own prose quoting the pattern, leaving 95; `grep -cE 'blocked_by = \[".+'` returns 0.
- The token `W6` occurs exactly once in `docs/plans/agent-scaffold.plan.toml` outside `Q-70`, and that occurrence is `Q-59`'s. Confirmed: `W6` appears at 1774 (`Q-59`) and then only at 1883, 1893, 1895, 1897 and 1901, all inside `Q-70` (1880-1903).
- The `-w1` to `-w4` waiver-id sequence is already carried by the `workflow-enforcement-tier` step, so the two owed ids continue it. Confirmed, all four ids present.
- `[meta].orphan_tasks` occurs nowhere in `src/workflow.rs` and is consumed only by `src/plan/source.rs` for duplicate and slug-collision validation. Confirmed by `grep -rn`, and the consuming loop is at `src/plan/source.rs:767-783`.
- The JSONL `type:"waiver"` arm performs no ownership check of its own: zero occurrences of `leading_slug` or a belongs-to-step message in `src/metrics.rs:539-601`. Confirmed. And `check_workflow_toml` passes `&waivers_from_toml(plan)` alone into `run_checks`, at `src/workflow.rs:192`.
- `complete` steps declare no increments while their round records carry increment ids. Confirmed, and it holds under the strong reading too: `state-schema` and `round-log-core` are both `complete`, declare zero `[[step.increment]]`, and carry round identities genuinely distinct from the step slug (`state-schema-inc1/2/3`, `round-log-core-incA/B`). The `-incA` / `-incB` pair is the same pair the item says cannot be declared, and `an_uppercase_increment_id_is_flagged` pins that.
- W4 requires a receipt STRICTLY after the cutoff. Confirmed: `if index <= cutoff { continue; }` at `src/workflow.rs:333`.
- The three detection mechanisms are given "in the buildability order round 3 of inc4 recorded". Confirmed: the routing paragraph at ledger 587 says "The three detection mechanisms round 3 of inc4 produced, in buildability order", and ledger 651 records the same three in the same order.

## Accurate-but-misleading sweep

I looked specifically for a quotation that is correctly copied, resolves fine, and is used to support something its source does not say. I found NONE. The three places where the risk was highest, and why each is clean:

1. `must be made with W6 in view` (`:1901`). The ledger's sentence scopes that ground to a choice between TWO directions, a declared-increment lookup and a waiver-unit-naming rework, while `Q-70` names THREE candidates. The item does not attribute the third direction to the record, though: it says explicitly of direction (iii) "THIS IS RECORDED, NOT RECOMMENDED", and separately attributes it to the routing paragraph's own wording. The widening was `R1C-3`'s remedy and it is done without misattributing it.
2. `Minimal by default` and the no-open-design-space clause (`:1899`). The ledger's clause covers the two inc3 defects plus the `next` defects and nothing more. The item states this limit itself: "THAT CLAUSE SCOPES TO (a) AND (b) ONLY and must not be stretched over the entries after them". The quotation is used inside the boundary its source draws.
3. `the record schema W3, W4 and W5 all read must take ONE DELIBERATE EDIT RATHER THAN A RIDER ON A PATH FIX` (`:1895`). The ledger frames this as the reasoning that carried the human's decision, itself citing an earlier calibration close. The item calls it "THE CONSTRAINT THE HUMAN'S RECORDED REASONING ATTACHES", which is what the source says.

One compression worth noting without raising it: "under `[meta].primary = "toml"` a JSONL waiver record grants nothing and is reported by nothing" (`:1895`). Read absolutely, "reported by nothing" is false, since `check_record` still schema-checks the record and the same sentence says so one clause earlier ("The JSONL arm is LIVE and reachable"). The clause is grounded on `check_workflow_toml` reading `waivers_from_toml(plan)` alone, which scopes it to the workflow checks, so the sentence is internally consistent. I raise no finding.

## Projection and mechanical guards

- `render --check --strict` on `docs/plans/agent-scaffold.plan.toml`: "up to date", exit 0.
- `validate --workflow --source docs/plans/agent-scaffold.plan.toml`: 310 records valid, 95 steps, 70 questions, "workflow invariants hold", exit 0.
- The projected view reproduces the `ask` BYTE-FOR-BYTE: `docs/plans/agent-scaffold.md:227-247` diffed against `plan.toml:1883-1903` with only the TOML `ask = """` delimiters and the `- \`Q-70\` (exploring) ` list prefix removed is empty. So no quotation is mangled in projection, and a reader greping either substrate for a handle gets the same text.
- ASCII: `LC_ALL=C grep -cnP '[^\t\x20-\x7e]'` returns 0 for the plan TOML, the projected view, and the sidecar.
- `[meta].orphan_tasks` now carries 17 tokens including `q70-capture`, matching the ledger's "seventeen declared orphan tasks".

---

## Defects in SOURCE documents, not in `Q-70`'s use of them, for the orchestrator to route

NOT findings against this artifact. Recorded because they are the upstream cause of `R3A-1` and because one of them is a false claim in the ledger.

S1. THE LEDGER'S DEFECT (20) PARAGRAPH FALSIFIED ITS OWN EXAMPLE, at `docs/plans/agent-scaffold.ledger.md:567`. It ends "measure a handle with `grep -cF` when it matters, as the round 2 fix pass did for "THE MEMBERS KNOWN AT THIS WRITING", which resolves to 1." That sentence IS the second occurrence, so the handle resolved to 2 the moment the sentence was written. `grep -cF` returns 2 today. The paragraph that records handle decay decayed a handle by recording it.

S2. THE SAME PARAGRAPH'S OTHER TWO MEASUREMENTS ARE NOW STALE, AND ITS STATED PROPERTY IS FALSE. It says: 'Measured on 2026-08-11, "THREE DEFECTS IN `agent-scaffold next`" resolves to 2 paragraphs and "A FOURTH `agent-scaffold next` DEFECT" to 3 ... Both still resolve and the first hit is the real paragraph in each case, so the handles work'. Measured now: 3 and 4. And the first hit is NOT the real paragraph in either case: `THREE DEFECTS` hits 567, 587, 1363 with the target at 1363 (LAST), and `A FOURTH` hits 539, 567, 587, 997 with the target at 997 (LAST). The recorded property "the first hit is the real paragraph" is false for both examples it was derived from. `Q-70` is unaffected, because it uses the paragraph-BEGINNING form for both, which `grep -c '^...'` resolves to 1; that is the property worth promoting into the convention.

S3. EVERY LIVE "THREE `agent-scaffold next` DEFECTS" IS STILL OWED A CORRECTION, at ledger 533, 1071, 1275, 1277 and 1353. `Q-70` states this and correctly says it is not the item's to make. Recorded so it is not lost.

No defect in `src/` was found by this lens. Every source citation the item makes resolves and the code says what is claimed, so I have nothing to route there.
