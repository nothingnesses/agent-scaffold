# `workflow-enforcement-tier-inc4`, round 2, fix-induced-residue lens

Reviews exactly what the round 1 fix pass changed: the diff `218c8c3..a534d69` (`5b529eb` "docs: apply the inc4 round 1 remedies" and `a534d69` "docs: correct the w1 waiver figure to 13 (3, 4, 6)"), four files, 40 changed lines. Worktree `.claude/worktrees/rev-inc4-r2-a` at `a534d69`. Every fixture was built under `<scratchpad>/rev-inc4-r2-a/` only. The one mode change is shown restored below.

The lens is the failure modes of a DELETION-AND-TENSE remedy class, which this project has not measured before: a deletion that leaves a dangling reference, a re-tense that produces a claim false even in the past, a re-tense that strips a qualifier or mixes tense inside one argument, and the single place authored prose was licensed.

## Summary

FIVE findings: 0 critical, 0 high, 3 medium, 2 low.

| id | severity | site | class |
| --- | --- | --- | --- |
| `R2A-1` | medium | `docs/plans/agent-scaffold.plan.toml:1732` | re-tense that does not repair the twin, because the falsity is a line number |
| `R2A-2` | medium | `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:346` | authored prose defines a class the tree does not support and the increment does not meet |
| `R2A-3` | medium | `docs/plans/agent-scaffold.plan.toml:1728` | partial re-tense leaves a false present-tense tail |
| `R2A-4` | low | `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:195` | premises re-tensed, dependent conclusion orphaned in the present tense |
| `R2A-5` | low | `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:304` | deletion leaves a residual implication |

NOTHING was found in three of the four target classes I expected to be productive: every deletion in the diff was checked for a dangling reference and only one (`R2A-5`) has one; every re-tensed token was checked against the tree at the commit that wrote the sentence and only `R2A-1` fails; the waiver figure and both its siblings reconcile exactly.

## `R2A-1` (medium): the `README.md:228` twin was re-tensed, but its falsity is a LINE NUMBER, so the tense change did not repair it and the generated view still contradicts itself

`Q-55-receiptcurrency` chose in-place re-tensing over appending on the stated benefit that "the rendered view stops contradicting itself, and no reader meets a false claim at all" (triage `R1C-5`, option 2). On one of the eight sites it still does.

THE EDIT, from the diff:

```
$ git diff --word-diff=porcelain 218c8c3..a534d69 -- docs/plans/agent-scaffold.plan.toml
 ... THE GROUND, which resolves the contract objection rather than overriding it: `README.md:228`
-says
+said
 "Unlike `validate` it never fails on a missing or malformed file ..."
```

The triager's own `FACT 3` row 2 named the falsity precisely, and it is not a tense: "`README.md:228` is now a comment line inside a code fence (`# --workflow would join ...`); the sentence is at `:238`. The pass corrected this same citation to `:238` at sidecar `:173`."

THE CITATION DOES NOT RESOLVE TODAY:

```
$ sed -n '228p' README.md
# --workflow would join /elsewhere/docs/plans/their-task.plan.toml against

$ grep -n 'Unlike `validate` it never fails' README.md
238:`status` prints a best-effort projection of that state: ...
```

IT DOES NOT RESOLVE ON THE DATE THE RECORD STAMPS EITHER, which is the check this lens exists to run. The paragraph is stamped "(human, 2026-07-31, receipt `q_id:"Q-55-refusalscope"`)" and was committed that day at `e019b83`. On that tree the sentence was at line 226, and the ask ITSELF said `226`:

```
$ git show e019b83:README.md | grep -n 'Unlike `validate` it never fails'
226:`status` prints a best-effort projection of that state: ...

$ git show e019b83:docs/plans/agent-scaffold.plan.toml | grep -o 'README.md:2[0-9][0-9]'
README.md:226
```

The full line-number history of that sentence, over every revision of `README.md` that carries it:

```
$ for c in $(git log --format='%H' -- README.md); do n=$(git show $c:README.md | grep -n 'Unlike `validate` it never fails' | head -1 | cut -d: -f1); [ -n "$n" ] && echo "$(git log -1 --format='%h %ad' --date=short $c) line=$n"; done
0d39eaf 2026-08-06 line=238
4ee50fb 2026-08-05 line=238
36e19f0 2026-08-05 line=238
734746f 2026-08-05 line=238
6081df8 2026-08-04 line=238
269d075 2026-08-03 line=238
b236b10 2026-08-03 line=238
609ddcf 2026-08-01 line=228
b821b0a 2026-07-26 line=226
...
```

So `README.md:228` held that sentence for exactly one window, 2026-08-01 to 2026-08-02, which is not the window the paragraph describes. "said" makes the sentence uncheckable against the current tree without making it true of the tree it names. That is the precise mode this lens was convened for: a tense-only remedy that stops the claim being checkable and does not make it correct.

THE SELF-CONTRADICTION IN THE GENERATED VIEW SURVIVES. `docs/plans/agent-scaffold.md` now attributes the SAME quotation to two different lines of `README.md`:

```
$ grep -n 'README.md:2[0-9][0-9]' docs/plans/agent-scaffold.md | cut -c1-140
166:... THE GROUND, which resolves the contract objection rather than overriding it: `README.md:228` said "Unlike `validate` it never fails ...
1568:... `README.md:238` does not merely promise the projections never fail; it says "Unlike `validate` it never fails ...
```

`:166` renders the plan TOML `ask` at `:1732`; `:1568` renders sidecar `:173`, which the planner pass already corrected. Two renderings of one quotation, two different line numbers, one generated document.

WHY THIS IS THE SAME DEFECT THE ORCHESTRATOR ALREADY RECORDED, APPLIED TO ONLY ONE SITE. Ledger `:551` records orchestrator defect (17) and its standing cure: the `Q-55-receiptcurrency` brief said "change only the TENSE of each of the eight claims", and "the EIGHTH site has NO TENSE TO CHANGE. Its falsity is a NUMERAL that was never true". The cure recorded there is "WHEN A BRIEF STATES THE FORM OF AN AUTHORISED EDIT, CHECK THAT FORM AGAINST EVERY SITE THE DECISION COVERS BEFORE PUTTING IT TO THE HUMAN, because a remedy class that fits seven sites can be inapplicable to the eighth". The check was run once, found the waiver numeral, and stopped. Twin 2's falsity is also a numeral (a line number), the authorised form is also inapplicable to it, and the same cure reaches it.

SEVERITY medium. No behaviour, nothing computes on it, and the correcting sibling is in the same file. It is medium because this is the FIFTH occurrence of the recorded twin-site failure mode in this task, on a decision the human took specifically to end it, and because the paragraph is a human decision receipt, which is the artifact class the append convention exists to protect from exactly this kind of half-applied edit.

MINIMAL REMEDY: RE-POINT, one token, `README.md:228` to `README.md:238`, matching sidecar `:173` which already carries it. Acceptance check 21's own rule prescribes this action for this input ("A citation whose subject moved is RE-POINTED at the subject") and it authors nothing. Whether the tense then stays "said" or returns to "says" does not matter to the defect; "said" plus `:238` is coherent and is the smaller change. NOTE FOR THE ORCHESTRATOR: `Q-55-receiptcurrency` authorised a TENSE change on these eight, so a re-point may be a fresh human call, on the same reasoning the ledger already records for the waiver numeral (a citation re-point revises no reasoning, so the append convention is not engaged). It is the same question, on the same decision, with the same answer available.

## `R2A-2` (medium): the 21 authored words narrowing acceptance check 21b define a class the tree does not support, and the increment does not meet the check as narrowed

THE AUTHORED SENTENCE, at sidecar `:346`, rendered at `docs/plans/agent-scaffold.md:1741`:

> THE EXCLUSION IS THE REPLACED-SUBJECT CLASS ONLY: a `src/checks.rs` citation whose subject MOVED and still exists is re-pointed with the rest.

Check 21b is an acceptance criterion ("AFTER INC4, THE CITATIONS THIS STEP'S OWN LINE-NUMBER MOVEMENT BROKE RESOLVE AGAIN ... AND ONLY THOSE"), so the sentence states a required post-condition: every `src/checks.rs` citation in `checks-runner-worktree-name-collision.md` whose named subject moved and still exists must be re-pointed. The pass re-pointed exactly ONE (`src/checks.rs:862-871` to `:1037-1046`, `fn scratch`). AT LEAST FOUR MORE QUALIFY AND WERE NOT RE-POINTED.

Each cited range opened in the current tree, beside the current location of the named subject:

| sidecar line | cited | named subject | what the cited range actually holds | subject now at |
| --- | --- | --- | --- | --- |
| `:14` | `src/checks.rs:78` | `RUNNER_PREFIX`, "the constant `agent-scaffold-checks-run-`" | `         PathBuf,` (an import) | `:98` |
| `:14` | `src/checks.rs:848-852` | `nanos()` | the body of a `Command::new("sh")` builder | `:1023` |
| `:53`, `:61` | `src/checks.rs:400-405` | `owning_pid` | a `git_command().arg("-C")` helper | `:561` |
| `:67` | `src/checks.rs:1438-1442` | `dead_pid()` | a `scratch("paths-skip")` test body | `:1613` |

```
$ sed -n '78p;400,405p;848,852p;1438,1442p' src/checks.rs
            PathBuf,
    args: &[&str],
) -> Result<std::process::Output, RunError> {
    git_command().arg("-C").arg(repo).args(args).output().map_err(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            RunError::GitUnavailable
        .arg(command)
        .current_dir(worktree)
        .stdin(Stdio::null())
        .output()
        .map_err(|error| {
        let dir = scratch("paths-skip");
        init_repo(&dir);
        fs::write(dir.join("main.rs"), "fn main() {}\n").unwrap();
        write_config(
            &dir,

$ grep -n 'const RUNNER_PREFIX\|^fn nanos\|^fn owning_pid\| fn dead_pid' src/checks.rs
98:const RUNNER_PREFIX: &str = "agent-scaffold-checks-run-";
561:fn owning_pid(dir_name: &str) -> Option<u32> {
1023:fn nanos() -> u128 {
1613:   fn dead_pid() -> u32 {
```

Others in the same file with the same property, not tabulated individually: `:72-77` (`RUNNER_PREFIX`'s comment), `:329-342` (`WorktreeGuard`, now at `:345`), `:388-392` (the dependency-discipline comment, now `git_command`'s), `:400-402` (`owning_pid`'s comment), `:407-461` and `:425-428` (the pid-liveness gate; `pid_is_alive` is now at `:416`, `prune_orphan_worktrees` at `:588`), `:845-847` (`nanos()`'s doc comment), `:1462`, `:1491`, `:1492` (the hand-built fixture names).

THE AUTHORED DISTINCTION IS ITSELF FALSE OF THE TREE. The triage built the two-class split on one worked example each: `fn scratch` as moved-and-exists, and `owning_pid` as replaced ("The sibling `src/checks.rs:400-405` was correctly left alone; it now resolves to `fn git`, its `owning_pid` subject having been replaced", triage `R1C-6`). `owning_pid` was not replaced. It exists at `src/checks.rs:561` with its documentation rewritten around `reserve_runner_worktree`. So the one citation the sentence was written to keep in the exclusion is in the class the sentence excludes from the exclusion, and the sentence commits the increment to re-pointing it.

THE SENTENCE ALSO REOPENS THE SCOPE ITS OWN PARAGRAPH CLOSES. The two sentences immediately before it say the `src/checks.rs` citations are "the owning step's closure work, not this one's; pulling it in would widen a scope the human closed (`Q-55-currencyscope`)". The narrowed exclusion pulls in at least four of them.

THIS IS THE INCREMENT'S NAMED FAILURE MODE LANDING AGAIN, at sidecar `:308`: "a pass that re-tenses a false claim can write a NEW false claim in its place". `R1C-6` was rated `low` and its stated cost was that check 21b's "AND ONLY THOSE" was untrue of one citation. The remedy authored a clause that makes the same check untrue of at least four, in the direction that widens a closed scope rather than narrowing a disclosure.

SEVERITY medium. Nothing behavioural, and a reader is not misled about the tool. It is medium because acceptance check 21b is the criterion a round uses to settle whether inc4 is done, and as narrowed the increment fails it; and because the clause was authored specifically to make a scope boundary honest and instead moves the boundary.

MINIMAL REMEDY, TWO SHAPES, and I do not choose between them because the second is a scope call the orchestrator owns.

- DELETION-CLASS, and it restores the state `R1C-6` actually asked for: delete the authored sentence and instead name the single exception in the existing disclosure, for example "except `:1037-1046`, re-pointed because `test-tmpdir-repo-assumption.md:35` already cites that exact range for that exact helper". That is the true and complete statement of what the pass did, it authors about a dozen words in place of the twenty-one it removes, and it defines no class. I verified the ground: `test-tmpdir-repo-assumption.md:35` reads "- `src/checks.rs:1037-1046`, `fn scratch(name)`, builds `std::env::temp_dir().join(format!("agent-scaffold-checks-test-{pid}-{name}"))`", and that range is `fn scratch(name)` today.
- REVERT, which was `R1C-6`'s other option and which the orchestrator declined on the ground that reverting "would restore a citation that is stale for another reason and make the file worse for a reader". That ground is unchanged, and this finding does not reopen it; it only shows that the alternative taken was not costed correctly.

## `R2A-3` (medium): the `Q-55-scope` twin was half re-tensed, and the two verbs left behind are the two that are now false

RULING ON CARRIED JUDGEMENT CALL 3: LEAVING THE PROSPECTIVE CLAUSE ALONE WAS NOT CORRECT.

THE SENTENCE, at plan TOML `:1728`, rendered at `docs/plans/agent-scaffold.md:162`, with the pass's two changes marked:

> (1) The DOCUMENTATION half of SE-3 is IN SCOPE: the two-tier split [was] undocumented in the scaffolded AGENTS.md, so a non-instrumented user [read] an unconditional promise of the `validate --workflow` backstop (`pack/AGENTS.md:93`) and, after the tier policy lands, meets a hard failure from a check the guidance still promises them.

Four verbs describe one causal chain. The pass changed the first two ("is" to "was", "reads" to "read") and left "lands", "meets" and "still promises". The tail is not a neutral prospective clause: "the guidance still promises them" is a present-tense assertion about the shipped pack, and it is FALSE.

THE CITATION THE SENTENCE ITSELF SUPPLIES SETTLES IT IN ONE STEP. `pack/AGENTS.md:93` is the same paragraph now that it was on 2026-07-31 (I checked, so the citation is not the problem here), and it now reads:

```
$ sed -n '93p' pack/AGENTS.md | grep -o 'when instrumentation is on.*rather than passing'
when instrumentation is on, the deterministic `validate --workflow` check is the backstop that the
required reviewed rounds happened before a step is marked complete, and on a project with no round
log yet, which every project scaffolded without `--instrument` remains, that check exits non-zero
reporting that it could not run rather than passing
```

versus the same line at the commit that wrote the decision:

```
$ git show e019b83:pack/AGENTS.md | sed -n '93p' | grep -o 'the deterministic `validate --workflow` check, once built, is the backstop[^;]*'
the deterministic `validate --workflow` check, once built, is the backstop that the required reviewed
rounds happened before a step is marked complete
```

The unconditional promise is gone. Acceptance check 20 pins its removal ("rebuild the fixture WITHOUT `--instrument` and grep its `AGENTS.md` for the backstop sentence. It must now carry the instrumentation qualifier"), and defect D is what this step closed. So the decision record now asserts, in the present tense, that a defect this step closed is open.

THE CONTRAST THAT SHOWS THE HALF-MEASURE IS THE PROBLEM AND NOT THE PROSPECTIVE MOOD. The sidecar states the same idea CONDITIONALLY at `:294` and stays true: "a tier policy without its qualifier leaves a non-instrumented user hitting a hard error from a check the guidance still promises them unconditionally". That sentence is a counterfactual about a policy shipped without its qualifier, so nothing falsifies it. The `Q-55` version is not conditional. Once its two premises are re-tensed, its tail is left as the only unhedged present-tense claim in the sentence, and it is the wrong one to leave.

The writer's stated ground was that the clause "was a prediction written before the policy landed". It was, and predictions are why the fix pass left it. But the prediction did not come true: the documentation half shipped WITH the tier policy, by design, which is exactly what sidecar `:294` argues for. A prediction that was averted by the work the record schedules is not preserved by leaving it in the present tense; it is converted into a false statement about the shipped tree.

SEVERITY medium, on the same ground the triage rated `R1A-3`/`R1C-2` medium: a present-tense claim about the tree, in a paragraph whose neighbouring verbs the pass re-tensed, which a reader can only reconcile by supplying a tense the sentence does not have.

MINIMAL REMEDY: FINISH THE RE-TENSE, token-level, the same class the pass already applied twice in this sentence. "after the tier policy lands, meets" becomes "after the tier policy landed, would have met", and "the guidance still promises them" becomes "the guidance still promised them". Nothing is authored, no fact is added, and the sentence then reads as one consistent historical clause.

TWIN-SITE WARNING, so a fix does not manufacture the sixth instance. The four-defect bullet list at the head of the sidecar (`:5` to `:9`, rendered `docs/plans/agent-scaffold.md` in the same block) states all four defects in the unqualified present tense, including defect D ("The scaffolded `AGENTS.md` promises the `validate --workflow` backstop without qualification"). The planner pass re-tensed the defect-reproduction SECTIONS from `:44` onward and did not open `:1` to `:43`:

```
$ git diff --unified=0 363ac06 218c8c3 -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md | grep '^@@' | head -1
@@ -44 +44 @@
```

I am NOT raising that as my finding: it is text the fix pass never touched, so it belongs to this round's cold-read lens, and it may be defensible as a dated reproduction record. I raise it here only because a remedy for `R2A-3` that ignores it leaves the plan asserting defect D open in one place and closed in another, which is the pattern this task has already recorded four times.

## `R2A-4` (low): at sidecar `:195` two premises were re-tensed and the conclusion they feed was left in the present tense, where it is now false

THE SENTENCE, sidecar `:195`, rendered at `docs/plans/agent-scaffold.md:1590`, with the pass's two changes marked:

> `no_active_loop_reason` [WAS] `#[serde(skip)]` (`src/next.rs:NextProjection::no_active_loop_reason`) and `status`'s `Projection` [HAD] no reason field at all, so under `--json` an omitted part serialises as a bare `null` with nothing distinguishing why.

The two premises are true of the past, which I checked rather than assumed:

```
$ git show 75c962d:src/next.rs | grep -n -B3 'no_active_loop_reason' | head -4
114-    /// Why there is no active loop, for the human renderer. Not serialised (the JSON
115-    /// contract is exactly the fields above); recomputed each call, never stored.
116-    #[serde(skip)]
117:    pub(crate) no_active_loop_reason: Option<String>,

$ git show 75c962d:src/main.rs | sed -n '/^struct Projection/,/^}/p'
struct Projection {
    /// The plan projection, present only when a readable `--plan` was given.
    plan: Option<PlanProjection>,
    /// The metrics summary, present only when the metrics log exists.
    metrics: Option<MetricsProjection>,
}
```

The conclusion is false of the present, measured on the binary rather than read:

```
$ agent-scaffold next --json --source "$SB/f16/docs/plans/TEMPLATE.plan.toml" --metrics docs/metrics/workflow.jsonl
{
  "task": "TEMPLATE",
  "source": ".../f16/docs/plans/TEMPLATE.plan.toml",
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project",
  "active_loop": null,
  "resume_state": null,
  "resume_state_absent_reason": "ledger-absent",
  "no_active_loop_reason": "metrics-not-this-project"
}
exit: 0
```

An omitted part now serialises as `null` WITH a reason distinguishing why, which is the whole point of `Q-55-jsonreason`. Both doc comments say so in the tree: `src/next.rs:165-167` reads "Each optional part carries a closed reason enum beside it, so `--json` reports WHY a part is absent rather than a bare `null` that reads the same for every cause", and `src/main.rs` carries the mirror on `Projection`.

The triage prescribed exactly two token substitutions here and the writer made exactly two. The brief records that the writer deliberately re-tensed MULTIPLE verbs in single sentences in three other places for precisely this reason. This is the sentence where it did not, and it is the sentence where the dependent clause is a claim about the tree rather than about the decision.

SEVERITY low. The two capitalised past-tense verbs two clauses earlier make the whole sentence read historically, and the paragraph is explicitly framed "THE PROBLEM, in the form that decided it", so a careful reader is not misled. It is a finding rather than nothing because the fix pass's own act of re-tensing the premises is what created the mismatch, and because the claim is checkable in one command.

MINIMAL REMEDY: one token, "serialises" to "serialised". Nothing is authored.

## `R2A-5` (low): the `R1C-3` deletion left "the guarded half" without the clause that grounded it

RULING ON CARRIED JUDGEMENT CALL 1: IT IS A DEFECT, at `low`, and the minimal remedy is a two-token substitution. I also state the residual-acceptance case, because it is close.

THE SENTENCE AFTER THE DELETION, sidecar `:304`, rendered `docs/plans/agent-scaffold.md:1699`:

> `Q-55-jsonreason` adds a second: the increment now also changes a DOCUMENTED JSON CONTRACT on two commands, falsifying four doc comments and breaking a byte-compare golden. None of this changes the class ... but it widens what the two rounds have to cover and a reviewer who checks only `next --json` has checked the guarded half.

"The guarded half" carries a definite article with no antecedent left in the paragraph, and it entails that `status --json` is the unguarded half.

THE ENTAILMENT IS THE CLAIM THE DELETION JUST REMOVED. The triage reproduced six `status --json` serialisation assertions added by inc2, two of them asserting on the serialised text rather than the exit code (`tests/unsafe_pairings_are_refused_and_omitted.rs:1427` and `:1740`). The file's own definition of "unguarded" is conjunctive and the pass has already marked half of it historical, at sidecar `:208`: "`status --json` has NO golden, and HAD no test on its serialisation at all ... so the `status` half of this change WAS UNGUARDED". So `:208` says `status --json` WAS unguarded (past) while `:304` implies it IS the unguarded half (present). That is the same twin-site disagreement `R1C-3` was raised to close, one hop further out.

THE CASE FOR ACCEPTING IT AS A RESIDUAL, which is why this is `low` and not medium. "Guarded" also has a surviving grounding that is still TRUE: sidecar `:290` says "`status --json` has no golden test", and `grep -n 'GOLDEN' src/next.rs` confirms the byte-compare golden is `next`'s alone. Under that grounding "the guarded half" reads as "the half with the golden" and stays correct. A reader who reaches `:290` or `:208` recovers the right fact either way, and the remedy authors words into a closed increment's risk paragraph, which is the class this project measures as re-seeding.

I rule it a defect anyway, because the surviving clause is doing live work (it tells a reviewer that checking one surface is not enough) and it does that work through a term whose plain reading is now wrong, in the exact paragraph a deletion was just applied to for that reason.

MINIMAL REMEDY: two tokens, "has checked the guarded half" to "has checked only one of the two". No new fact, no restructuring, and the sentence's instruction is unchanged.

## The three carried judgement calls, collected

1. `R1C-3`'s residual implication: DEFECT, `low`, see `R2A-5`. Minimal remedy is a two-token substitution; accepting it as a residual is defensible on `:290`'s surviving grounding and I say why.
2. The numeral `13` beside "SIX rounds and FIFTEEN valid findings": A PREFERENCE, NOT A FINDING. The capitalised spelling in this file is an emphasis device, and a numeral cannot carry it, so the change loses emphasis; it asserts nothing false and it matches the sibling waiver notes, which use numerals for the same quantity ("24 valid findings (9, 5, 6, 4)", "14 valid findings (6, 4, 2, 0, 2)"). The file also uses bare numerals for magnitudes elsewhere in the same two paragraphs ("Step 92", "roughly 80 lines"). Trading one spelling for another here costs more than it saves, and I raise no finding.
3. The prospective clause left alone in the `Q-55` record: LEAVING IT WAS NOT CORRECT, see `R2A-3`, `medium`.

## The waiver figure, recomputed first-hand

CLEAN. All three sibling notes reconcile exactly against the round log.

```
$ jq -r 'select(.type=="round") | [.task, (.valid_findings|tostring)] | @tsv' docs/metrics/workflow.jsonl | grep 'workflow-enforcement-tier-inc'
workflow-enforcement-tier-inc1  3
workflow-enforcement-tier-inc1  4
workflow-enforcement-tier-inc1  6
workflow-enforcement-tier-inc2  9
workflow-enforcement-tier-inc2  5
workflow-enforcement-tier-inc2  6
workflow-enforcement-tier-inc2  4
workflow-enforcement-tier-inc3  6
workflow-enforcement-tier-inc3  4
workflow-enforcement-tier-inc3  2
workflow-enforcement-tier-inc3  0
workflow-enforcement-tier-inc3  2
workflow-enforcement-tier-inc4  11
```

| note | text | log | verdict |
| --- | --- | --- | --- |
| `-w1` (`docs/plans/agent-scaffold.plan.toml:1330`) | "Three work-review rounds, 13 valid findings (3, 4, 6)" | 3, 4, 6 = 13 | EXACT |
| `-w2` (`:1339`) | "Four work-review rounds, 24 valid findings (9, 5, 6, 4)" | 9, 5, 6, 4 = 24 | EXACT |
| `-w3` (`:1348`) | "Five work-review rounds, 14 valid findings (6, 4, 2, 0, 2)" | 6, 4, 2, 0, 2 = 14 | EXACT |

THE ORDER IS ALSO RIGHT, which a bare sum does not establish. The log is append-only, so file order is chronological, and the three inc1 records are lines 246, 247, 248 carrying 3, 4 and 6 in that order; the escalation is line 249 and there is no `dismissal_recheck`. The note's own later sentence corroborates the ordering independently: "two of round 3's six in-scope findings were sites an earlier fix had ruled on", and 6 is the third element.

THE SIDECAR TWIN AGREES. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:308` now reads "THREE rounds and 13 valid findings", rendered at `docs/plans/agent-scaffold.md:1703`, so the twin-site disagreement `a534d69` was written to close is closed. `grep -rn 'TWENTY' docs/plans/*.md docs/plans/*.toml docs/plans/agent-scaffold.steps/` finds no surviving "twenty valid findings" claim in the plan or its sidecars.

RECORDED, NOT RAISED: `docs/plans/agent-scaffold.ledger.md:535` still says "Inc1 of this very step spent THREE rounds and TWENTY valid findings". The triage put the ledger sites outside inc4's closed scope, and the ledger applies its own recorded append convention ten lines later at `:545`: "The third is `R1B-1`, the risk paragraph's 'TWENTY valid findings' for inc1, which measures THIRTEEN". Correction present, convention followed, no finding.

## The deletions, each checked for a dangling reference

Four deletions in the diff. Only one leaves a dangling reference (`R2A-5`).

- SIDECAR `:206`, the `#[serde(skip)] appears exactly ONCE` negative result. CLEAN. Nothing in the file depends on it: `grep -n 'serde(skip)\|silently-dropped\|skip_serializing_if'` over the sidecar returns `:195`, `:206`, `:219` and `:337`, and `:219`'s "NO LONGER `#[serde(skip)]`" takes its antecedent from `:195`, which the pass re-tensed rather than deleted. The surviving half of the sentence is TRUE: `grep -rn 'skip_serializing_if' src/` hits only `src/plan/source.rs`, never `src/next.rs` or `src/main.rs`.
- SIDECAR `:304`, the `status --json` test clause. See `R2A-5`.
- SIDECAR `:345`, check 21's "and the check is mechanical rather than a reading". CLEAN. `grep -n 'mechanical'` over the sidecar, the plan TOML and the ledger finds no other claim that depends on it; the inc4 risk paragraph's "ITS EVIDENCE IS MECHANICAL AND CHEAP TO PRODUCE" is a different claim, about the evidence, and it survives on its own terms. The instruction the deleted sentence restated is still carried by the imperative that follows it.
- `checks-runner-worktree-name-collision.md:55`, the `{pid}-{nanos}` enumeration. CLEAN, AND THE DELETION FIXED THE PARTITION THE TRIAGE WARNED ABOUT. The surviving sentence is true over all seven sites, which I checked rather than inferred:

```
$ grep -rn 'as_nanos' tests/ | wc -l
7
$ grep -rn -B6 'as_nanos' tests/ | grep -o '"agent-scaffold-[a-z-]*' | sort -u
"agent-scaffold-anchor-
"agent-scaffold-containment-
"agent-scaffold-validate-projection-
"agent-scaffold-validate-toml-only-
"agent-scaffold-validate-workflow-no-log-
"agent-scaffold-validate-workflow-no-source-
"agent-scaffold-validate-workflow-opaque-log-
```

Seven sites, seven distinct literal prefixes, so "each carries a distinct literal prefix, so they cannot collide today" holds. The two helpers that use a literal name AND the clock (`agent-scaffold-anchor-`, `agent-scaffold-containment-`) now fall under the second sentence, where the claim is true of them, instead of being forced into the first list whose "rather than by the clock" they would falsify. Every surviving citation in that sentence resolves at its cited range: `src/checks.rs:1037-1046` is `fn scratch(name)`, `src/main.rs:2280-2285` is the `agent-scaffold-poc-` helper, `src/manifest.rs:552-558` is `fn scratch`, `src/plan/render.rs:638` is `fn scratch`, `tests/audit_command.rs:20` is `fn scratch`, `tests/scaffold_precommit_hook.rs:14` is `fn scratch`, `tests/checks_staged_hook_env.rs:50` is the `agent-scaffold-hookenv-` name.

## The writer's own reported call on `R1A-4`, verified

The writer re-tensed rather than deleted the two bullets because deleting them would falsify two neighbouring sentences that COUNT them. The call was correct and both counts survive.

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:197` reads "Four doc claims are falsified or made incomplete by it", and exactly four bullets follow at `:199` to `:202`. `:204` opens "A FIFTH ITEM FOUND BY THE SWEEP IS PRE-EXISTING". The pass changed only the verbs inside bullets three and four ("HAS" to "HAD", "IS SHORT BY ONE" to "WAS SHORT BY ONE"), leaving four bullets and the fifth item intact.

Both re-tensed bullets are TRUE OF THE PAST, checked against the pre-inc2 tree rather than assumed. The literal-string pickaxe finds nothing for either quotation only because both doc comments are line-wrapped in the source:

```
$ git show 8beb1c2~1:src/main.rs | grep -n -B4 '^struct Projection'
561-/// A derived, best-effort projection of the workflow state, serialised by
562-/// `status`. Every part is optional so a missing plan or metrics file yields a
563-/// partial projection rather than a failure; nothing here is a source of truth
564-/// (it is regenerable from the plan and the metrics log).

$ git show 8beb1c2~1:src/next.rs | grep -n -B2 'pub(crate) resume_state'
111-    /// The ledger's `## RESUME STATE` block, extracted verbatim, or `None` when the
112-    /// ledger is absent or carries no such section.
113:    pub(crate) resume_state: Option<String>,
```

CONSIDERED AND NOT RAISED on the same bullets: bullets one and two keep prospective verbs ("BECOMES FALSE", "BECOMES INCOMPLETE") while three and four are now past, so "HAD THE SAME DEFECT" back-references a defect stated in the prospective mood. The triage explicitly ruled the prospective verbs correct as a specification of owed work, and the two kinds of bullet were always different in kind (one states the effect of the change, the other the state at spec time), so the mix is not incoherent. Not a finding.

## Every re-tensed token checked against the tree it describes

The other twenty-one re-tensed tokens are TRUE of the past. Each was checked with `git show <commit>:<path>` against the commit that wrote the sentence (`7807c6b`, `e019b83`, `75c962d`, all 2026-07-31) or against the pre-increment parent, not inferred from the sentence.

| claim, after re-tensing | check | verdict |
| --- | --- | --- |
| the skip WAS announced twice in `src/main.rs:run_validate`, both WENT to stderr, stdout CARRIED only the ok summary | `git show 6b1c847~1:src/main.rs` has `eprintln!("no metrics log at ...")` at `:858` and the `skipping the workflow check` note at `:1040`; the message existed from `88356ad` (2026-07-17) to `6b1c847` (2026-08-05), spanning the 2026-07-31 decision | TRUE |
| the metrics-log path RESOLVED against the CWD (`src/main.rs:ValidateArgs::metrics`) | `git show 609ddcf~1:src/main.rs` shows `#[arg(long, default_value = "docs/metrics/workflow.jsonl")] metrics: PathBuf`, a CWD-relative default on exactly the cited symbol | TRUE, and the symbol citation still names the right site |
| the two-tier split WAS undocumented, a user READ an unconditional promise at `pack/AGENTS.md:93` | `git show e019b83:pack/AGENTS.md` line 93 carries "the deterministic `validate --workflow` check, once built, is the backstop", unqualified, and `:93` is still the same paragraph today | TRUE (the sentence's TAIL is not; see `R2A-3`) |
| `status`, `next` and the derived ledger path CARRIED the identical CWD-relative defect | same parent commit; the anchoring landed at `609ddcf` | TRUE |
| `no_active_loop_reason` WAS `#[serde(skip)]`; `status`'s projection HAD no reason field | `git show 75c962d:src/next.rs:116` is `#[serde(skip)]` on that field; `git show 75c962d:src/main.rs` `struct Projection` has two fields and no reason | TRUE, both in the plan TOML and the sidecar |
| a bare filename from inside `docs/plans` WAS a silent miss; after the policy LANDED it BECAME a hard failure | the mechanism clauses around it are present tense and still hold; the outcome clause changed with the tier policy at `3d00341`, and check 18 states the same pair | TRUE and internally coherent |
| a layout that WORKED BEFORE INC2; the `--source`/`--plan` pairing GREENED BEFORE INC2 | inc2 is `8beb1c2` (2026-08-03) and the containment guard rooted on the checked plan is what refuses both; the file's own measurement ("A measured this layout going from reading its 37-record log to `exit=1 REFUSED`") corroborates the pre-inc2 state | TRUE on the code history; NOT re-measured against a rebuilt pre-inc2 binary, see the limits below |
| `README.md:228` SAID "Unlike `validate` it never fails ..." | see `R2A-1` | NOT TRUE of the date the record stamps |

## The one authored edit outside 21b, re-run rather than read

Acceptance check 16's quoted command line gained four words (`--metrics <the path as given>`), which is `R1A-6`'s remedy. IT IS CORRECT ON BOTH SPELLINGS. Measured at uid 1000 on a purpose-built fixture outside any repository, scaffolded with `agent-scaffold scaffold --output-dir <scratch>/f16 --write --force --principles default` and given a real one-record `docs/metrics/workflow.jsonl`:

```
$ agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl/
no metrics log at docs/metrics/workflow.jsonl/; nothing to validate
docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
exit=0

$ agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl/ --workflow
--workflow requested but the round log at docs/metrics/workflow.jsonl/ could not be checked (Not a directory (os error 20)): ...
exit=1

$ chmod 600 docs/metrics
$ agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
exit=0

$ agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl --workflow
--workflow requested but the round log at docs/metrics/workflow.jsonl could not be checked (Permission denied (os error 13)): ...
exit=1

$ chmod 755 docs/metrics && ls -ld docs/metrics
drwxr-xr-x 2 jessea users 4096 docs/metrics        # restored
```

Both quoted exit codes and both quoted `could not be checked` fragments reproduce with the corrected command line, on both spellings, which is what `R1A-6` said the old wording could not do. The plan file name differs from the check's `p.plan.toml` and nothing turns on it. NOTE, not a finding: `<the path as given>` now appears twice in one sentence, once as the argument to pass and once as the path the tool prints back, which is slightly circular; the check's own preceding sentence supplies both concrete spellings, so a reader is not stuck.

## Mechanical gates, run first-hand in this worktree

```
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s)

$ agent-scaffold render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date                                   exit 0

$ agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 289 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold   exit 0
```

The rendered view is a faithful render of a source that disagrees with itself in the two places `R2A-1` names, so `render --check` cannot catch either; that is the same property the triage recorded for `R1C-5`.

## What this lens varied, and what it held fixed

VARIED. Commit axis: every re-tensed token checked against `git show <commit>:<path>` at the commit that wrote its sentence (`7807c6b`, `e019b83`, `75c962d`) and at the pre-increment parents (`609ddcf~1`, `8beb1c2~1`, `6b1c847~1`). Line-number axis: the `README.md` sentence tracked across all nineteen revisions of `README.md` that carry it, and the `Q-55` `ask`'s own citation tracked across every revision of the plan TOML since 2026-07-30. Citation axis: every `src/checks.rs` citation in `checks-runner-worktree-name-collision.md` opened at its cited range in the current tree and its named subject located separately, plus every surviving citation in the sentence the pass edited there, plus the `src/checks.rs` citations in `test-tmpdir-repo-assumption.md` (both resolve). Behavioural axis: check 16 re-run on a purpose-built fixture, two probe failure classes (EACCES from a mode-600 ancestor, ENOTDIR from a trailing slash), each with and without `--workflow`; `next --json` on an unsafe pairing. Arithmetic axis: all three waiver notes recomputed from the round log, with the record ORDER checked as well as the sum. Deletion axis: all four deletions checked for a dangling reference by grepping the whole sidecar for the terms the deleted text supplied.

HELD FIXED, so a defect here survives this review. One platform (Linux, local filesystem), one build profile (debug), one binary (built at `a534d69`), uid 1000 only: I did NOT run any cell under `unshare -Ur`, so a uid-dependent difference in check 16's root cell is untested here. I did NOT rebuild a historical binary, so the recorded measurements embedded in the re-tensed sentences (`next` emitting `state: converged` and `rounds: 2/5`, `status --resume` echoing the RESUME STATE, the 37-record symlink measurement, and "greened before inc2") were checked for consistency against the code history and against the file's own cross-references, NOT re-measured against a pre-inc1 or pre-inc2 build. I ran no concurrency and no TOCTOU case.

SCOPE HELD. I reviewed the diff `218c8c3..a534d69` and what it entails. A false claim in text the fix pass did not open survives this lens by construction; the head of the sidecar (`:1` to `:43`) is the clearest example and I name it under `R2A-3` without claiming it. I re-raised none of inc2's four or inc3's four recorded residuals, and none of round 1's four dismissals (`R1A-5`, `R1A-7`, `R1A-8`, `R1B-3`): I checked each of my five against that list before writing it. I raised nothing on `run_validate`'s `--plan` clap-required claims, on `src/next.rs:162` or `:181-183`, on the Status narrative at `docs/plans/agent-scaffold.md:7`, on the plain-`validate` mode-000 inconsistency or the containment TOCTOU that check 16 pins, or on line length and prose wrapping.
