### `sidecar-status-opening-drift`: correct the 21 step sidecars whose opening status token contradicts the step's declared `status` (`Q-78-statusdrift`)

ONE CLASS OF EDIT, ENUMERATED BY A COMMAND RATHER THAN BY A READING. A step's `status` lives in `docs/plans/agent-scaffold.plan.toml` and is the single source of truth for it. Forty-five of the 101 step sidecars ALSO open by naming a state in prose, and that second copy was never maintained: 21 of the 45 now name a state their step is not in. This step makes each of those 21 openings state the step's actual state. It changes nothing else about any step.

THE HUMAN'S DECISION THIS IMPLEMENTS. `Q-78-statusdrift` (human, 2026-08-18) chose to fix this now as its own step, over folding it into the `Q-78` design pass and over flipping the affected statuses to `not-started` instead. Its `type:"decision"` receipt is in `docs/metrics/workflow.jsonl` with `q_id:"Q-78-statusdrift"`, carried by commit `e8b2992`, which adds exactly two lines to that file and touches nothing else.

WHY THAT RECEIPT IS NOT IN `[step.provenance].decisions`, RECORDED SO THE ABSENCE IS A CHOICE. `decisions` resolves fail-closed to a registered `[[question]]` id of `Q-<n>` shape: the provenance arm of `validate_source` (`src/plan/source.rs`) rejects a decision that `question_id_index` (`src/metrics.rs:427`) cannot parse, and then rejects one that names no `[[question]]`. `Q-78-statusdrift` is a sub-decision id with a receipt and no registered `[[question]]`, the same convention `Q-55-<suffix>`, `Q-65-<suffix>` and `Q-77-changelogfix` already follow, so putting it there would fail `validate --source` on both arms. The step's provenance therefore names commit `e8b2992`, which is where the receipt lives, and this paragraph is the pointer to the id. Registering a `Q-<n>` question for it instead was considered and rejected: W4 joins a decided item to a receipt on `d.q_id == question.id` (`w4_problems`, `src/workflow.rs:309`), so a new `Q-79` would demand a receipt with `q_id:"Q-79"` that does not exist, and writing one is not this step's to do.

### THE DRIFT, MEASURED

THE METHOD, so the number is reproducible rather than asserted. For each of the 101 Roadmap steps, take the first line of `docs/plans/agent-scaffold.steps/<slug>.md` that is neither blank nor a Markdown heading, and read its leading word or phrase. When that leading token is one of the seven Roadmap status labels (`not started`, `in progress`, `complete`, `skipped`, `next`, `optional`, `deferred`), compare it against the step's declared `status`. Report every pair that disagrees. The 101st step is this one, whose opening carries no such token.

TWO PAIRS DISAGREE ON THE TOKEN AND AGREE ON THE FACT, so they are exempt and the exemption is stated rather than silent. `optional` and `deferred` say nothing about whether work has started, so an opening of "Not started" alongside either adds information without contradicting it. Four sidecars are exempt on that ground: `git-url-fetch` (`deferred`, and its opening names both, "Not started (deferred)."), and `greenfield-flake`, `later-enhancements` and `tui-authoring` (all `optional`). No other pair is exempt: `complete` and `in progress` both assert that work happened, so an opening of "Not started", "Next" or "Deferred" on either contradicts the declared status outright.

THE COUNT IS 21 OF 101, MEASURED ON THE TREE THIS STEP IS AUTHORED INTO. Ten `complete` steps open "Not started": `optional-modules`, `instrument-flag`, `reviewer-harness-field`, `reviewer-diversity`, `review-mode`, `doc-redundancy-cleanup`, `doc-currency-guidance`, `human-input-gate-reinforce`, `planner-folds-decisions` and `prompt-drift-guard`. Seven `complete` steps open "Next": `session-preflight`, `round-log-core`, `workflow-invariants`, `decision-receipt`, `structured-skeleton`, `reviewer-reproducible-evidence` and `decision-folder-currency`. Two `complete` steps open "Deferred": `state-schema` and `checks-runner-worktree-name-collision`. Two of the three `in-progress` steps open "Not started": `workflow-calibration` and `workflow-driver`.

THE THIRD `in-progress` STEP IS CLEAN AND IS NAMED SO THE READER KNOWS IT WAS CHECKED. `code-value-audit-static` opens "Build the Tier-0 slice of the Q-52 code-value audit", which is a statement of what the step does and not a claim about its state, so it contradicts nothing. Of the 80 sidecars this step does not touch, 56 open with no status token at all (this one included), 20 open with a token that matches the declared status, and 4 are the exempt pairs named above. 56 plus 20 plus 4 plus 21 is 101.

THE OPENINGS FOLLOW NO SHIPPED CONVENTION, which is why correcting them breaks nothing downstream. The scaffolded template for a step sidecar, `pack/plan-template.steps/example-step.md` and its rendered copy `docs/plans/TEMPLATE.steps/example-step.md`, is one heading plus one instruction: "What this step does and how; once done, the outcome and the evidence." It prescribes the `### <slug>: <title>` heading and says nothing about opening with a state. So the status token in these 21 openings is a local habit, not an instruction any project inherits.

### WHY THE STATUS IS RIGHT AND THE PROSE IS STALE, FOR THE TWO IN-PROGRESS CASES

This step corrects the PROSE, so the direction has to be established rather than assumed. It is established independently for each of the two, and neither derivation depends on the other.

`workflow-calibration` CONTRADICTS ITSELF INSIDE ONE FILE, which is the strongest form the evidence takes here and needs no external comparison. It opens "Not started (deferred)." and four paragraphs later states "WHERE THIS STEP'S AUDIT RECORDS LIVE (`Q-73`, human, 2026-08-13, already executed at `572e331`)", describing the 2026-08-13 audit as this step's delivered work. Outside the file: `docs/plans/workflow-calibration.explorations/` holds 12 files, and `docs/metrics/workflow.jsonl` carries two `type:"decision"` receipts whose `task` is `workflow-calibration`, `Q-73` (2026-08-13) and `Q-76-causation` (2026-08-14). Work has happened, so `in-progress` is right.

`workflow-driver` IS STALE ON TWO INDEPENDENTLY CHECKABLE COUNTS IN ONE SENTENCE. It opens "Not started; the build-start is a separate pending human decision, so this umbrella step stays `not-started` with no increments declared yet". First, the step declares THREE increments in the plan TOML, `workflow-driver-stage0a`, `workflow-driver-stage0b` and `workflow-driver-stage1`, so "no increments declared yet" is false against the same document the sentence sits beside. Second, `docs/metrics/workflow.jsonl` carries nine `type:"round"` records whose `task` is `workflow-driver`, spread across those three increments and dated 2026-07-19 and 07-20, plus three `type:"decision"` receipts (`Q-51` once, `Q-58` twice), and `docs/plans/workflow-driver-stage1.build-plan.md` and `docs/plans/mealy-workflow-driver.explorations/` both exist. Work has happened, so `in-progress` is right.

THE OTHER 19 NEED NO SUCH DERIVATION, because they are `complete` steps and a `complete` step is by definition not "Not started", "Next" or "Deferred". One of them is close to self-correcting already and is called out so the implementer does not read it as a rewrite: `structured-skeleton` opens "Next; the planning pass is DONE (2026-07-18) and this umbrella step is built in six reviewed increments", so the same sentence that carries the stale token already says the step is built. It still gets the token fix, because a reader scanning openings reads the token.

### WHAT THE FIX IS, AND WHAT IT IS NOT

THE FIX IS THE TOKEN AND THE CLAUSE THAT DEPENDS ON IT. Replace the stale opening state claim with the step's actual state, which for 19 of the 21 is `complete` and for 2 is `in progress`. Where the stale token carries a dependent clause that is false with it, that clause goes too: `workflow-driver`'s "with no increments declared yet" is the one measured instance, and `instrument-flag`'s "The design is decided below; the build is deferred" is the second, since the build is not deferred on a `complete` step. Where the rest of the opening sentence stays true once the token is corrected, leave it exactly as it is.

THE WORDING IS THE IMPLEMENTER'S, subject to one constraint: the opening must not become a second hand-maintained copy of a field that will drift again in a different way. Naming the state plainly ("Complete.") is what the two sidecars that already do this use, `task-entry-regrounding` ("Complete (2026-07-20), built in two increments") and `waiver-model` ("Complete (merged into main by fast-forward `7be5c2a`, 2026-07-18; RISKY, five rounds)"), so there is an in-repo form to follow rather than one to invent.

NOT IN SCOPE, EACH CONSIDERED AND EXCLUDED SO THE ABSENCE IS A CHOICE.

- WHAT ANY OF THESE 21 STEPS SHOULD SAY. This step corrects a stale claim about state. It does not review, re-scope, re-title or rewrite a single step, and it changes no step's `status`, which is the orchestrator's.
- THE FRONT MATTER. `docs/plans/agent-scaffold._status-narrative.md` lists `workflow-calibration` under "Still optional/deferred", which is the same contradiction in the plan's most-read paragraph. It is left alone deliberately: that file is a long historical accretion whose currency is a separate and much larger question, and correcting one clause inside it invites the rewrite this step exists to avoid. It is recorded here so it is a named exclusion rather than a miss.
- ANY SIDECAR SENTENCE THAT IS NOT THE OPENING, with the single exception specified as item `B` below. No sweep was run for stale non-opening claims across the other 100 sidecars, so nothing here says there are none.
- THE RULE THAT WOULD PREVENT RECURRENCE. See the next paragraph.

THE RECURRENCE QUESTION IS ROUTED AND NOT DECIDED HERE. Correcting 21 openings restores 21 hand-maintained copies of a field the TOML owns, which is the mechanism that produced the drift in the first place. The alternative, that a sidecar never restates a TOML field and these openings lose their state token entirely, is a design question about which facts belong in the schema and which in the prose, and that is exactly what `Q-78` commissions a design pass to settle; it is registered there as design-space item (g). This step deliberately takes the human's chosen direction, state the actual state, and leaves the rule to the pass. THE COST OF THAT ORDER, stated so it is weighed: if the pass later rules that the token goes, these 21 openings are edited a second time. That second edit is a deletion of one token per file against 21 files that are open anyway, so the cost of doing it twice is small, and the alternative is leaving 21 false statements standing for as long as the pass takes.

### `B`: the one non-opening sentence, in `workflow-calibration.md`

`workflow-calibration.md` also states "The directory currently holds `2026-08-13-audit-when-the-loop-turned.md` (the verdict, its evidence, and ten ordered recommendations), `2026-08-13-audit-measurement-methods.md` (the methods reference), and `calibration-analysis.md` (the 2026-07-31 prior analysis, which the audit re-tests with 12 more days of data and corrects in one place)." That reads as an exhaustive list and it names 3 of the 12 files now in `docs/plans/workflow-calibration.explorations/`. The nine it does not name are `2026-08-14-causation-investigation.md`, `finding-provenance-a.tsv`, `finding-provenance-b.tsv`, `finding-provenance-extract-a.md`, `finding-provenance-extract-b.md`, `fix-pass-shape-a.tsv`, `fix-pass-shape-b.tsv`, `fix-pass-shape-mechanical-a.md` and `fix-pass-shape-mechanical-b.md`.

WHY IT IS IN SCOPE, AGAINST THE HABIT THAT ARGUES OTHERWISE. The 2026-08-13 audit measured steps generated by the process itself rising from 8.3% to 54.2%, so "while you are in the file anyway" is the exact habit that produced that number, and it is the reason this is argued rather than assumed. It survives the test on three grounds: it is the same defect class this step is defined by, a stale statement about this step's own state; it is in the same file and two paragraphs from the opening the implementer is already editing; and it is one sentence. It does not open the door to a general currency sweep, which the exclusion above rules out explicitly.

THE IMPLEMENTER PICKS THE FORM. Either name all 12, or drop the enumeration and say what the directory is for. Naming 12 files re-creates a list that goes stale on the next write, so dropping the enumeration is the form that does not need maintaining, and the choice is left open because either satisfies the criterion.

### ACCEPTANCE, each executable

1. THE ENUMERATION IS EMPTY. Run this from the repository root and it prints nothing. It is the same command that produced the 21 above, it drives off the generated Roadmap table so it needs no JSON tool, and it encodes the `optional`/`deferred` exemption stated above.

```
sed -n 's/^| `\([a-z0-9-]*\)` | \([a-z ]*\) |.*/\1\t\2/p' docs/plans/agent-scaffold.md |
while IFS=$(printf '\t') read -r slug status; do
  line=$(sed -e '/^#/d' -e '/^[[:space:]]*$/d' "docs/plans/agent-scaffold.steps/$slug.md" | head -1)
  token=$(printf '%s' "$line" | sed -n 's/^\(Not started\|In progress\|Complete\|Skipped\|Next\|Optional\|Deferred\)\([.;,: ].*\)*$/\1/p' | tr 'A-Z' 'a-z')
  case "$status:$token" in
    "$status:"|"$status:$status"|"optional:not started"|"deferred:not started") ;;
    *) printf '%s\t%s\t%s\n' "$slug" "$status" "$token" ;;
  esac
done
```

The empty output is the oracle, not the pipeline's exit status. MEASURED before the fix, this command parses all 101 Roadmap rows and prints exactly the 21 rows listed above, so it is known to detect the condition rather than merely to stay silent.

2. RED THEN GREEN, demonstrated rather than asserted. Revert one corrected opening to its stale token, show criterion 1 prints that one row, restore it, show criterion 1 prints nothing again. The red output lands as evidence in the outcome.

3. EXACTLY 21 SIDECARS CHANGE, plus `docs/plans/agent-scaffold.md`. `git diff --name-only` over the step's change lists 22 paths: the 21 files under `docs/plans/agent-scaffold.steps/` named above, and the regenerated `docs/plans/agent-scaffold.md`. No file under `src/`, `pack/` or `tests/` appears, `docs/plans/agent-scaffold.plan.toml` does not appear (no `status` is flipped, and the step's own entry is authored by this planning pass rather than by the implementation), and `docs/metrics/workflow.jsonl` does not appear.

4. THE CHANGE IS CONFINED TO THE OPENING, WITH ONE NAMED EXCEPTION. For each of the 21 files, `git diff --numstat` reports at most 2 added and 2 removed lines, since each opening is one line in an unwrapped file. The exception is `docs/plans/agent-scaffold.steps/workflow-calibration.md`, which carries item `B` as well and so reports at most 4 and 4.

5. ITEM `B` MAKES NO FALSE CLAIM. In `workflow-calibration.md`, every `.md` or `.tsv` filename the audit-records paragraph names exists in `docs/plans/workflow-calibration.explorations/`, and the paragraph either names all 12 entries of that directory or makes no exhaustive claim about its contents (no "the directory currently holds X, Y and Z" form).

6. THE PROJECTION IS REGENERATED AND MATCHES. `./target/debug/agent-flow render docs/plans/agent-scaffold.plan.toml` then `./target/debug/agent-flow render --check --strict docs/plans/agent-scaffold.plan.toml` exits 0, so the committed `docs/plans/agent-scaffold.md` carries the corrected Step Details. `docs/plans/agent-scaffold.md` is never hand-edited.

7. THE VALIDATORS AND THE SUITE STAY GREEN. `./target/debug/agent-flow validate --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/workflow.jsonl` and `./target/debug/agent-flow validate --source docs/plans/agent-scaffold.plan.toml --workflow` both exit 0, and `cargo test` passes. None of the three reads sidecar prose, so they are the no-regression check rather than the oracle for this step. Criterion 1 is the oracle.

8. ASCII ONLY. `LC_ALL=C grep -cP '[^\t\x20-\x7e]' <file>` prints 0 for every changed file. Use that pattern rather than `[^ -~]`, which matches every hard tab. Note `grep -c` exits 1 when the count is 0, so it breaks an `&&` chain.

### DOCUMENTATION IMPACT

None outside `docs/plans/`. `grep -rn 'sidecar' pack/ README.md` returns matches, and they were read rather than counted: they describe the skeleton-plus-sidecar structure, the render path, and the `<task>.steps/<slug>.md` naming convention. The one that comes closest to prescribing a sidecar's content is `pack/plan-template.steps/example-step.md`, quoted above, and it asks for what the step does and how, not for a state. So no shipped guidance, prompt or template goes stale, and the pack is untouched.

The one document this change makes newly relevant is `Q-78`, which carries this step's measurement as design-space evidence and points back here. If that pass rules that a sidecar never restates a TOML field, the template above is where the resulting rule would be written, and that is the pass's to schedule rather than this step's.
