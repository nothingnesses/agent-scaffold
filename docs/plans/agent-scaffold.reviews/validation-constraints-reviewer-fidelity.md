# `validation-constraints` plan review, fidelity lens

Reviewer lens: FIDELITY TO WHAT WAS ACTUALLY DECIDED AND MEASURED. The artifact is `git diff main..HEAD` on `review/vcstep-fidelity`: the new `[[step]]` `validation-constraints` with six `[[step.increment]]` entries in `docs/plans/agent-scaffold.plan.toml`, its prose sidecar `docs/plans/agent-scaffold.steps/validation-constraints.md`, the `Q-70` fold to `decided`, and the regenerated `docs/plans/agent-scaffold.md`.

Sources checked: the `Q-70` `[[question]]` entry, all three files in `docs/plans/validation-constraints.explorations/` read in full, every `type:"decision"` receipt whose `q_id` starts `Q-70` or `Q-55` in `docs/metrics/workflow.jsonl`, and `docs/plans/agent-scaffold.ledger.md` located by line-start anchored greps only. Every `file:line`, property and command in the step was re-measured rather than read.

SIX FINDINGS. One `high`, one `medium`, four `low`. No `critical`.

The step's factual spine is otherwise sound. I could not falsify any source citation, any measured property, or any decided-direction claim in it; the list of confirmations is at the end so a triager can see what was checked and passed.

---

## PR-A-1 (`high`): inc5's `low_risk` classification rests on a claim that is false, namely that nothing reads `blocked_by`

CLAIM. The sidecar states three times that `blocked_by` is unread, and makes that the sole ground for classifying `validation-constraints-inc5` as `low_risk` (one clean round). The field is read at four production sites today, and populating it changes both the committed rendered plan view and the step `agent-scaffold next` recommends. The classification's stated ground is therefore false, and its written escape hatch is scoped to a condition that cannot arise as stated.

THE THREE CLAIM SITES, in `docs/plans/agent-scaffold.steps/validation-constraints.md`:

- `:69` "`validation-constraints-inc5`, THE UNUSED `blocked_by` FIELD".
- `:104` "Populating `blocked_by` changes no product behaviour today, because nothing reads the field, so a wrong value is inert and reversible in one revert."
- `:146` "`blocked_by`'s own doc comment in `src/plan/source.rs`, which describes a typed field that nothing currently reads."

THE READERS, by `file:line`:

- `src/plan/render.rs:480`, `for blocker in &step.blocked_by { notes.push(format!("{ROADMAP_BLOCKED_PREFIX}`{blocker}`")); }`, which writes the Roadmap notes cell of the generated `<task>.md`.
- `src/next.rs:722`, `.filter(|step| step.phase.is_pending() && blockers_met(step, steps))`, which selects the pending step `next` recommends, and `src/next.rs:730-731`, which returns a `LoopState::Blocked` loop when no pending step has its blockers met.
- `src/next.rs:742` and `:750` (`blockers_met`, `unmet_blockers`), and `src/next.rs:1012`, which fills the `blocked_by` context slot of the emitted instruction.
- `src/plan/source.rs:601-608`, the `validate_source` cross-reference that flags a self-reference or a dangling slug, pinned by `a_dangling_blocked_by_is_flagged` (`src/plan/source.rs:949`).

RUNNABLE DEMONSTRATION 1, the rendered view changes. Against a scratch copy of `docs/` taken from this branch, adding one blocker to the step this very diff introduces:

```
D=<scratch>/bb2; cp -r docs "$D/"; cp -r docs "$D/docs-orig"
P="$D/docs/plans/agent-scaffold.plan.toml"
awk 'NR==1391{print "blocked_by = [\"workflow-enforcement-tier\"]"; next} {print}' "$P" > "$P.new" && mv "$P.new" "$P"
./target/debug/agent-scaffold render "$P" --check
```

Output:

```
warning: .../agent-scaffold.md differs from a fresh render (a hand-edit, or a stale render after a source edit)
(first difference at line 350: expected "| `validation-constraints` | not started | blocked on `workflow-enforcement-tier...",
committed "| `validation-constraints` | not started | why: decisions Q-55, Q-70 |")
```

So one populated `blocked_by` value rewrites a row of the committed plan view. A wrong value is not inert.

RUNNABLE DEMONSTRATION 2, `next`'s answer follows the field. Using the repository's own fixture `src/plan/testdata/skeleton.plan.toml`, with `alpha` and `beta` both set `not-started` so the pending-selection rule at `src/next.rs:722` is the one that decides:

```
# c.plan.toml: alpha (order 1) has blocked_by = ["beta"]; beta (order 2) has blocked_by = []
./target/debug/agent-scaffold next --source c.plan.toml --json   ->  "step": "beta"
# d.plan.toml: identical except alpha's blocked_by is emptied
./target/debug/agent-scaffold next --source d.plan.toml --json   ->  "step": "alpha"
```

The recommended step flips on the field alone.

WHY THE ESCAPE HATCH DOES NOT COVER IT. `:104` says "RE-CLASSIFY IT TO RISKY BEFORE IT CONVERGES if the chosen treatment also teaches `next` to honour the field or retires the field from the schema". `next` already honours it, so the trigger as written never fires for the treatment the record actually asks for. The ledger's own routing of this member (find it with `grep -n '^A FOURTH `agent-scaffold next` DEFECT' docs/plans/agent-scaffold.ledger.md`) says the field is unused in the sense of UNPOPULATED, not unread, and states the consequence in terms of what `next` advises: "`next --source docs/plans/agent-scaffold.plan.toml` therefore advises ... which is not the next action". The step converted "populated on none" into "read by nothing" and priced the increment on the converted claim.

The step's own acceptance check 12 (`:125`) contradicts `:104` in the same file: it asks the implementer to "show that the answer `next` gives about which step is next follows from the structured source", which only makes sense if `next` reads the field.

IMPACT IF LEFT UNFIXED. An implementer builds inc5 from this brief, populates `blocked_by` across the Roadmap, and it converges on one clean round on the recorded ground that a wrong value is inert. A wrong value is not inert: it changes what `next` tells the next agent to do, which is the same failure mode the step itself classifies inc4 as `risky` for (`:102`, "`next` emits an INSTRUCTION that an agent acts on, so a wrong answer is not a wrong report but a wrong action").

SEVERITY: `high`. The defect is in a recorded risk classification, which AGENTS.md's convergence rule makes a property of the artifact that later rounds inherit rather than re-judge, and its stated ground is falsifiable in one command.

---

## PR-A-2 (`medium`): two members the ledger explicitly routes to this step are absent, and the step closes the set with a count of "TWO"

CLAIM. The sidecar at `:15` and `:67` states that inc3 carries "THE TWO PRE-EXISTING `validate` DEFECTS routed here from `workflow-enforcement-tier-inc3`". The ledger routes four things in that family to this step, not two. The two the step omits appear nowhere in the diff.

THE FOUR, each located by a line-start anchored grep over `docs/plans/agent-scaffold.ledger.md`:

1. `grep -n '^ONE ITEM IS QUEUED BY THIS DECISION RATHER THAN FIXED'` -> the plain-`validate` mode-000 FILE versus unsearchable DIRECTORY inconsistency. CARRIED by the step.
2. `grep -n '^THE PRE-EXISTING CONTAINMENT TOCTOU IS CONFIRMED PRE-EXISTING AND IS ROUTED'` -> the FIFO-widened mid-run symlink swap. CARRIED by the step. That same paragraph says the TOCTOU "routes to the validation-constraints step beside `R2A-4` and `R3A-3`", which names the other two.
3. `grep -n '^`R2A-4` WAS ACCEPTED AS A RESIDUAL'` -> "the pre-existing `no metrics log at <path>` note still prints one line above the corrected sentence ... Routed into the queued validation-constraints item alongside the pre-existing plain-`validate` inconsistency." NOT CARRIED.
4. `grep -n '^`R3A-3` IS OUT OF SCOPE FOR THIS INCREMENT AND IS ROUTED'` -> a mode-600 `docs/plans` yielding "no source plan at X" with a remedy aimed at someone who already passed one; "IT GOES TO THE VALIDATION-CONSTRAINTS STEP, beside `R2A-4` and the queued plain-`validate` inconsistency. Same family, NOT the same defect: that one is about the LOG input's exit codes, this one about the PLAN-SOURCE input's message." NOT CARRIED.

Each anchored grep returns exactly 1 hit, so the handles resolve unambiguously.

MEASURED ABSENCE from the diff:

```
S=docs/plans/agent-scaffold.steps/validation-constraints.md
grep -cF 'R2A-4' $S                 -> 0
grep -cF 'R3A-3' $S                 -> 0
grep -cF 'no source plan' $S        -> 0
grep -cF 'mode-600' $S              -> 0
grep -cF 'R2A-4' docs/plans/agent-scaffold.plan.toml   -> 0
grep -cF 'R3A-3' docs/plans/agent-scaffold.plan.toml   -> 0
```

WHY THE COUNT MAKES IT WORSE. The step opens its members section with "NO COUNT IS STATED HERE AND NONE MUST BE ADDED ... a maintained count of a moving set is this project's most repeated defect", then states a closed count for this sub-population two bullets later. `R3A-3` is explicitly a distinct defect in the same family, so the population is not closed at two. inc3's stated review question (`:67`) is "does the tool report the same thing about a log it cannot read, whichever way it cannot read it, and can the containment guard still be defeated between the check and the use", which does not reach the PLAN-SOURCE message defect at all, so an inc3 reviewer working from this brief has no prompt to look for it.

IMPACT IF LEFT UNFIXED. The findings files are commit-deleted and the ledger is deleted at task close, so this step is the durable home for these two. They are lost when the ledger goes, and the affirmative "TWO" tells a re-deriver the set is already complete.

SEVERITY: `medium`. Two recorded, reproduced, human-routed defects drop out of the plan, and the loss is masked by a count rather than visible as an omission.

---

## PR-A-3 (`low`): explorer B's negative result, routed to this step by name, is not carried

CLAIM. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` routes a further input to this step that the step does not record.

EVIDENCE. `grep -c "B's negative result belongs to the same queued step" docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` returns 1. The sentence reads: "B's negative result belongs to the same queued step and should be carried into it: declaring the log's LOCATION does not establish that the log BELONGS to the plan, and building a path field first would make the identity work harder, because the identity check would then have to reconcile itself against a declared path that may disagree with it."

`grep -ciF 'negative result'`, `grep -ciF 'path field'` and `grep -ciF 'declared path'` over `docs/plans/agent-scaffold.steps/validation-constraints.md` all return 0.

The step inherits the three project-identity limitations from the same paragraph correctly (verified below), so the omission is specific to this one sentence rather than to the paragraph.

IMPACT IF LEFT UNFIXED. inc6 is the increment this constrains, and it is an ordering constraint ("do not build a path field first"). An implementer of inc6 with the ledger gone could take the declared-path route the record already measured as counterproductive.

SEVERITY: `low`. It is a design hint on a late increment, not a blocker, and inc6 carries an entry gate to a human anyway.

---

## PR-A-4 (`low`): the TOML `title` and the sidecar heading disagree about the step's scope, and only the narrower one reaches the rendered view

CLAIM. The two hand-authored one-line descriptions of this step in the same commit describe different scopes, and the one a reader of `docs/plans/agent-scaffold.md` sees is the narrower.

EVIDENCE. `docs/plans/agent-scaffold.plan.toml:388` (the `title` at plan line 1388) reads:

`state W5's waiver-ownership rule against the round log so the two owed waivers become writable (`Q-70`, direction (iii)), then treat the validator-cluster defects, the `agent-scaffold next` defects, project identity and the detection mechanisms the record routes here`

`docs/plans/agent-scaffold.steps/validation-constraints.md:1` reads:

`### `validation-constraints`: state W5's waiver-ownership rule against the round log so the two owed waivers become writable (`Q-70`), then treat the validator-cluster defects the record routes here`

The sidecar heading drops three of the four named member classes: the `agent-scaffold next` defects, project identity, and the detection mechanisms. The generated view carries the sidecar heading verbatim and reduces the TOML title to nothing: the Roadmap row rendered by this diff is `| `validation-constraints` | not started | why: decisions Q-55, Q-70 |`, with no title. This is the unguarded drift surface the ledger already records (find it with `grep -n '^ACCEPTED RESIDUALS AND OUT-OF-SCOPE ITEMS CARRIED FORWARD' docs/plans/agent-scaffold.ledger.md`, item (3): "THE PLAN TOML'S STEP `title` IS NOT PROJECTED INTO THE RENDERED VIEW AT ALL ... the TOML title and the sidecar heading ALREADY DIVERGE with nothing keeping them in step"), instantiated here with a real divergence.

The pre-existing tool behaviour is out of scope and I am not raising it. The authored divergence is in this diff.

IMPACT IF LEFT UNFIXED. The recorded orchestrator defect (19) was precisely an under-description of this step's scope. The heading a human reads reproduces that shape, though the body below it is complete.

SEVERITY: `low`. The members section three paragraphs down is correct and explicit, so a reader who continues is not misled.

---

## PR-A-5 (`low`): a ground held by one explorer is attributed to two

CLAIM. The sidecar at `:57` says of the open narrowing point: "Two REPORT it, on the ground that it is the only thing that catches a typo'd increment id on the Markdown and JSONL substrates, where no declared set exists to catch it."

EVIDENCE. The report-versus-silent split is correct: `Q-70-architecture.md:203` and `:286` report the unobserved case, `Q-70-evidence.md:208-213`'s `any` predicate reports it, and `Q-70-minimal.md:90-91` leaves it silent. But only the architecture lens gives the stated ground; `Q-70-architecture.md:286` reads "because it is the only thing that catches a typo'd increment id on the Markdown and JSONL substrate, where no declared set exists to catch it". The evidence lens gives a different ground at `Q-70-evidence.md:165`: "a waiver that covers an increment with no round records grants nothing in W3 anyway ... and reporting it turns a dead waiver into a visible one, which is Make illegal states unrepresentable applied to a waiver."

IMPACT IF LEFT UNFIXED. The step presents one argument as independently reached by two lenses, which overstates its corroboration. The second, independent argument is also lost.

SEVERITY: `low`. Both grounds support the same default the step takes, and the step leaves the point explicitly open for the implementer.

---

## PR-A-6 (`low`): inc6's recorded limitations are stated from the pre-inc1 world, and the one interaction inc1 changes is not recorded

CLAIM. The step lists inc6's three inherited limitations faithfully but omits a measured consequence of inc1 on inc6 that the architecture exploration established.

EVIDENCE. `docs/plans/agent-scaffold.steps/validation-constraints.md:70` records the three limitations. `Q-70-architecture.md:336` measures a fourth, related fact: "`check_workflow_toml` computes `metrics::parse_rounds(log_contents)` at one site (`:189`) and hands it to `run_checks`, so a filter applied there is inherited by every check that reads `rounds`. Today W5 reads none, so an identity filter would not protect W5's ownership verdict. Under C it would." The same file states it again at `:296`: "C makes the queued edit strictly cheaper". Confirmed at the source: `src/workflow.rs:189` is the single `parse_rounds` call site feeding `run_checks`, and `src/workflow.rs:219` today passes `waivers, steps, escalations` to `w5_problems` with no `rounds`.

The step's inc6 review question (`:70`) is "does an identity that is declared actually separate two projects sharing one log, and does an identity that is absent change nothing", and acceptance check 13 asks the round report to state "which surfaces the filter placement does and does not cover, `next` included". Neither names W5, whose coverage inc1 changes.

IMPACT IF LEFT UNFIXED. An inc6 reviewer would measure the filter's coverage against a mental model in which W5 reads no rounds, which inc1 will already have falsified.

SEVERITY: `low`. It does not affect the ordering decision (inc1 before inc6 is right under this fact, not against it), and check 13 partly reaches it.

---

## Member-by-member enumeration: what the record routes here, against what the step carries

The routing paragraph is found with `grep -n '^THE IMMEDIATE NEXT ACTION IS THE `validation-constraints` STEP' docs/plans/agent-scaffold.ledger.md` (1 hit, anchored). It states no count and names its members (a) to (f). Enumerated independently:

| Routed member | Source | Carried? | Where |
| --- | --- | --- | --- |
| (a) The W5 fix, "teaching W5 the structured step association W3 already uses" | routing paragraph; receipt `Q-55-w5defect` | Yes | `inc1` |
| (b) The three detection mechanisms (waiver-note breakdown join, dangling-receipt detection, quotation resolver) | routing paragraph | Yes | no increment, deliberately, with the pass's rulings recorded |
| (c) The plain-`validate` mode-000 file versus unsearchable directory inconsistency | routing paragraph; anchored ledger paragraph | Yes | `inc3` |
| (c) The containment TOCTOU | routing paragraph; anchored ledger paragraph | Yes | `inc3` |
| (c-adjacent) `R2A-4`, the `no metrics log at <path>` note | anchored ledger paragraph, routed by name | NO | PR-A-2 |
| (c-adjacent) `R3A-3`, the plan-source "no source plan at X" message | anchored ledger paragraph, routed by name | NO | PR-A-2 |
| (d) The `agent-scaffold next` defects (four, per the routing paragraph's own correction) | routing paragraph; human decision 2026-07-30; human decision 2026-08-01 | Yes | `inc4` (three) and `inc5` (`blocked_by`), with the split argued |
| (e) Project identity in the round record | routing paragraph; receipt `Q-55-mechanism` | Yes | `inc6` |
| (f) The shared root cause behind the resume cost (`project_root_of_source`'s fallback) | routing paragraph; receipt `Q-55-resumecost` | Yes | `inc6`, as one treatment with (e), as the receipt requires |
| The two `src/` defects the `Q-70` loop found | `grep -n '^TWO `src/` DEFECTS THE LOOP FOUND'` | Yes | first closed by construction in `inc1`, second is `inc2` |
| The ledger half of the `run_next` false green | `grep -n '^THE BACKSTOP CORRECTED BOTH EARLIER AGENTS ON OWNERSHIP'` | Yes | carried, explicitly not scheduled, ownership left to a human at step entry |
| Explorer B's negative result | `workflow-enforcement-tier` sidecar, routed by name | NO | PR-A-3 |
| `Q-55-fallbacksurface`'s measured new surface of the same fallback | `grep -n '^`Q-55-fallbacksurface`: ABSORBED, FILE NOTHING'` | Partly | subsumed by (f)'s root cause; the specific surface is not named. Not raised as a separate finding. |

The conditional route in the `workflow-enforcement-tier` sidecar ("parsing the whole `(source, plan, metrics)` triple ... if it is ever wanted it belongs in the validation-constraints step") is conditional rather than routed, so its absence is not a finding.

Receipts checked for a `chosen` that routes work here: `Q-55-entryroute`, `Q-55-w5defect`, `Q-55-mechanism`, `Q-55-resumecost`, `Q-70`, `Q-70-resolver`. `Q-55-jsonreason` listed "Defer to the validation-constraints step" as an option but the human chose "Add a serialised reason", so it routes nothing. `Q-55-scope` listed folding into this step and the human declined.

## Pre-decision check

Two content receipts exist, `Q-70` (`chosen` "Round-log join, direction (iii)") and `Q-70-resolver` (`chosen` "Bound it now, design it later"). Two process receipts also exist for this task, `Q-70-planreview` and `Q-70-stoppoint`, which decide how to review and where to stop, not content.

The step treats as settled: direction (iii) (receipted), mechanism 3 bounded and designed later (receipted), the sub-decision ruling on `Q-<n>-<suffix>` receipt ids, mechanism 2's empty red list, and the comment-coverage ruling. The last three are lettered duties (e), (f) and (h) that `Q-70`'s own "WHAT THE PASS OWES BACK" assigns to the PASS to rule on, all three explorers ruled the same way, and the evidence lens explicitly recommended they reach the human as findings rather than choices (`Q-70-evidence.md:249`). So they are pass rulings the step relays, not human decisions it invents.

The step correctly refuses to pre-decide: inc2's treatment (`:66`), inc5's treatment (`:69`), whether mechanism 1 is built (`:78`), whether mechanism 2 is built (`:80`), when mechanism 3's design pass runs (`:82`), and the narrowing's exact form (`:57`, stated as a default with a fork escape). It also puts a fresh gate to the human before inc6 (`:72`) rather than deciding the note-join question itself.

I found nothing the step treats as settled that no receipt and no pass ruling supports.

## What I checked and could not falsify

Recorded so a triager can see the boundary of this review, and because a lens that only contradicts is as biased as one that only agrees.

SOURCE CITATIONS, all re-read at the line: `leading_slug` at `src/workflow.rs:88`; `round_step_slug` at `:119`; `round_increment_id` at `:127`; `escalation_increment_id` at `:141`; the one-implementation comment naming Principle 16 at `:196-200`; `run_checks` at `:206-221` holding `rounds` and handing them to `w3_problems` but not to `w5_problems`; `step: step.slug.clone()` at `:258`; the W3 `complete` skip at `:445-447`; the step-unit waiver rule at `:450`; W3's covering-waiver match at `:498-502`; `w5_problems`'s signature `(waivers, steps, escalations)` at `:544-548`; the real-step rule at `:553`; the lexical strip at `:564`; the `w5_problems` doc comment restating the lexical rule at `:525-527`. In `src/plan/source.rs`: the `Waiver` struct at `:279-300`; `step_views()` at `:422-430`; `is_kebab_case_token` at `:475-477` with `an_uppercase_increment_id_is_flagged` at `:1221`; the waiver-loop comment at `:785-790`; the declared-increment set at `:792-793`; the membership check at `:807-811` with its format string at `:809`. In `src/metrics.rs`: `Round` at `:620-651` carrying exactly `line, task, artifact, outcome, consecutive_clean, risk_class, step, increment` with no `project` and no `valid_findings`; `parse_rounds` at `:660-711`. `src/plan/render.rs:527` as the sole reader of `waiver.note`. `src/next.rs:517-523` stating the parity property. `src/main.rs:1311` `project_root_of_source`. `peak_consecutive_clean` `pub(crate)` at `src/workflow.rs:407` and `WaiverReason::required_tier` single-sourced between `src/workflow.rs:610` and `src/plan/source.rs:846`, which are the two idiom precedents the step cites.

MEASURED PROPERTIES:

- The affected population reproduces at six pairs with the step's own command, including `decision-folder-currency-fold` under a `complete` step, which is the third latency mode the step says to carry.
- `grep -c '^\[\[step\]\]$'` and `grep -c '^blocked_by = \[\]$'` over the plan TOML both return 96 and `grep -cE '^blocked_by = \[.+\]$'` returns 0, so the field is empty on every step and populated on none, as `:69` claims.
- All four waiver-note breakdowns agree with the round records: `(3, 4, 6)`, `(9, 5, 6, 4)`, `(6, 4, 2, 0, 2)`, `(11, 9, 6, 4, 5)` against `workflow-enforcement-tier-inc1` to `-inc4` respectively.
- The two sub-decision properties both hold: every unresolved receipt id is `Q-<n>-<suffix>` shaped (0 bare ids dangle), and every parent is registered. The parent set is now `{Q-55, Q-70}`, so the pass's single-parent enumeration has indeed moved while the structural property survived, exactly as `:80` says.
- `W6` occurs 14 times in the plan TOML, exactly once outside the `Q-70` item, at the `Q-59` entry, and `Q-59`'s own text confirms the human's 2026-07-23 decision deferred the transition-legality check behind an evidence gate rather than dropping it.
- The `totally-not-a-step` control: unanchored returns 1 and the hit is the `Q-70` item's own corrected sentence; line-start anchored returns 0. The item's self-quoting correction in this diff is itself the evidence it claims to be.
- `workflow-enforcement-tier-w1` to `-w4` exist, so `-w5` and `-w6` continue an established sequence.
- The rule text is stated in exactly `pack/instrument.md:11`, `AGENTS.md:147` and `.agents/AGENTS.reference.md:147`; `grep -c "must own its" README.md` returns 0; `justfile:48` is `nix fmt` as the second line of `scaffold-self`; `CHANGELOG.md`'s `[Unreleased]` has `Added` and `Changed` and no `Fixed`; `pack/plan-template.plan.toml:44` carries the commented waiver example.
- The three project-identity limitations at `:70` match the `workflow-enforcement-tier` sidecar's "the queued step inherits all three" verbatim, including the third (`decisions` and `escalations` read unfiltered), which is recorded in `docs/plans/workflow-enforcement-tier.explorations/metrics-path-independent-map.md:446-450`.
- The `TMPDIR` preamble at `:112` matches `docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md`, which names the three tests.
- Two of inc4's three defects reproduce live: `next` with no `--source` prints "no plan source" and "no plan steps found" at exit 0, and `next --source docs/plans/agent-scaffold.plan.toml` emits 587835 bytes. The step correctly states no size figure.
- Every ledger handle the step uses resolves to exactly 1 hit under a line-start anchored grep, and each one lands on its intended paragraph. No ledger line number is cited anywhere in the diff.
- `receipt record 308` for `Q-55-entryroute` is correct and stable, the log being append-only.
- `order = 97` is unique; `grep '^order = ' | sort | uniq -d` returns nothing.

DECIDED-DIRECTION FIDELITY. The `Q-70` receipt's `chosen` is "Round-log join, direction (iii)" over "Retire W5 ownership rule, direction (iv)" and "Put both to a build, decide on measurement"; the step's PROVENANCE paragraph states exactly that. The `Q-70-resolver` receipt's `chosen` is "Bound it now, design it later" over "Design and build it in this step" and "Drop it from the step entirely"; the step states exactly that. `Q-55-entryroute`'s four options give three alternatives, one of which is "Split out the W5 fix first", which the step reports correctly.

SEQUENCING. The claim that inc1 ships first and alone survives testing. All three explorers built direction (iii) rather than proposing it. The no-coupling half is measured, not asserted: direction (iii) adds no field to any struct (`Q-70-evidence.md:32`, `Q-70-architecture.md:99`, `Q-70-minimal.md:114-116`), while the note join needs `valid_findings` and project identity needs `project` on the same `Round`, so the recorded one-deliberate-edit constraint binds that pair and not inc1. The blocks-a-step-today half is confirmed by the ledger. The one measured interaction between inc1 and inc6 runs the beneficial way and is the subject of PR-A-6, not a counterexample. inc2's "same function, fresh" edge and inc6's last-among-declared edge are both argued from the record; inc3 to inc5 are explicitly stated as a non-load-bearing default, which is the honest form.

GATES, run on this branch with `TMPDIR` outside any repository: `cargo test` all suites pass; `cargo clippy --all-targets -- -D warnings` clean; `render --check --strict` reports up to date; `validate --source docs/plans/agent-scaffold.plan.toml --workflow` exits 0 with "316 records, valid", "96 steps, 70 questions, valid" and "workflow invariants hold". `LC_ALL=C grep -cP '[^\t\x20-\x7e]'` returns 0 on all three changed files.

## `src/` defects found, for the orchestrator to route

NONE NEW. Everything I touched in `src/` behaved as the step and the explorations describe it. The two `src/` defects the `Q-70` loop already recorded (W5's message asserting a step that need not exist, and the structural unreachability of `src/workflow.rs:553` on the TOML path) both reproduce and are already scheduled by this step as inc1-by-construction and inc2 respectively.
