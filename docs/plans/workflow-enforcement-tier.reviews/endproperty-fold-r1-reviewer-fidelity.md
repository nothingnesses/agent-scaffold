# Fidelity and internal-consistency review: the Q-55-endproperty fold (commit c131292)

Reviewer: independent fidelity/internal-consistency lens, round 1. Scope: the plan amendment at commit `c131292` on `plan/q55-endproperty`, touching `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` and its projection `docs/plans/agent-scaffold.md`. Lens: (A) does every claim about current code behaviour match the code and the running binary; (B) does the amendment's new text contradict unchanged text elsewhere in the same sidecar, the plan projection, `docs/metrics/workflow.jsonl`, or `docs/plans/agent-scaffold.plan.toml`.

Working directory used for all commands and fixtures: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-ep-fidelity`. Scratch fixtures under `/tmp/claude-1000/rev-ep-fidelity-scratch/` (cleaned up at the end; see the report).

## Summary

One finding, FI-1, severity MEDIUM: two unchanged rationale paragraphs still describe the residual gap that survives inc1's anchoring as being "exactly"/"precisely" the explicit-`--metrics`-naming-a-foreign-log case. The amendment's own new material (`Q-55-endproperty`, the updated `next` bullet, and acceptance checks 13b/14g) establishes a second, independent way the same gap survives anchoring, using only default resolution with no explicit override at all. I reproduced this directly against the built binary: both the metrics false-pass and the ledger leak occur with `--source`/`--plan` alone, no `--metrics` or `--ledger-fragment` involved.

Everything else checked out. Every fidelity-to-code claim I was asked to verify is true of the current tree, including one I expected might not hold (the `run_status`/`run_next` plan-selection claim) and one I verified experimentally rather than by inspection (canonicalize on a nonexistent path). All counts and enumerations I re-counted matched the text. The vocabulary bullets are updated correctly and consistently with how each surface is now described.

## Part A: fidelity to the code

### A-1. `run_validate`'s `--workflow` match arms

Claim (sidecar line 171, `agent-scaffold.md` line 1673): "`run_validate`'s `--workflow` match reads the TOML source in its `(Some(source), _, Some(metrics_text))` arm and the Markdown `--plan` in its `(None, Some(plan_text), Some(metrics_text))` arm."

Verified by reading `src/main.rs:run_validate` directly. The match is:

```
match (toml_primary, &plan_contents, &metrics_contents) {
    (Some(source), _, Some(metrics_text)) => { ... check_workflow_toml(...) ... }
    (None, Some(plan_text), Some(metrics_text)) => { ... check_workflow(...) ... }
    (None, None, _) => problems.push("--workflow requested but no plan source resolved: ...".to_string()),
    _ => eprintln!("--workflow has a plan source but the metrics log is missing; skipping the workflow check"),
}
```

Both arms match the claim exactly. TRUE.

### A-2. `resolve_metrics_path` anchors source-first

Claim: the metrics default anchors on `--source` first, `--plan` second.

`src/main.rs:resolve_metrics_path`:

```
source.as_ref().or(plan.as_ref()).map_or_else(
    || PathBuf::from(METRICS_RELATIVE),
    |anchor| project_root_of_source(anchor).join(METRICS_RELATIVE),
)
```

`source.as_ref().or(plan.as_ref())` is source-first, unconditional (does not check whether the source is TOML-primary). TRUE.

### A-3. `default_ledger_path` anchors source-first; `run_next` selects its plan differently

`src/main.rs:default_ledger_path`:

```
source.as_ref().or(plan.as_ref()).map_or_else(
    || PathBuf::from(format!("docs/plans/{task}.ledger.md")),
    |anchor| anchor.parent().unwrap_or_else(|| Path::new("")).join(format!("{task}.ledger.md")),
)
```

Same unconditional source-then-plan anchor as `resolve_metrics_path`. Meanwhile `src/main.rs:run_next` selects its STEPS via:

```
let (steps, principles, source) = if let Some(source_plan) = toml_source(&args.source)? {
    ... next::steps_from_toml(&source_plan) ...
} else {
    match &args.plan { Some(path) if path.exists() => { ... next::steps_from_markdown(...) ... } _ => (...) }
};
```

`toml_source` returns `None` whenever `--source` is absent, unreadable, unparseable, OR parses but is Markdown-primary (`src/main.rs:toml_source`'s own doc comment and body confirm this). So a Markdown-primary `--source` makes `run_next` select steps from `--plan`, while the SAME call's ledger path is still anchored on `--source` (via `default_ledger_path`, called at the `ledger_fragment ... unwrap_or_else(|| default_ledger_path(...))` site a few lines below). Two different selections sharing one input pair. TRUE, and confirmed to matter empirically (see A-6 below).

### A-4. Which surface reads which plan: `run_status`, `run_next`, `run_resume`

Claim: `run_status` and `run_next` both project from `toml_source(&args.source)` when TOML-primary, else `--plan`; `status --resume` reads no plan at all and falls back to the source-then-plan anchor.

`src/main.rs:run_status`:

```
let plan = if let Some(source) = toml_source(&args.source)? {
    Some(PlanProjection { steps: source.step_views(), open_questions: source.question_views() })
} else {
    match &args.plan { Some(path) if path.exists() => { ... } _ => None }
};
```

Same shape as `run_next`'s selection (A-3). Both confirmed TRUE.

`src/main.rs:run_resume`:

```
fn run_resume(args: &StatusArgs) -> io::Result<()> {
    let task = next::derive_task(&args.source, &args.plan);
    let ledger_path = args.ledger_fragment.clone().unwrap_or_else(|| default_ledger_path(&task, &args.source, &args.plan));
    if !ledger_path.exists() { println!("no ledger at {}; nothing to resume", ledger_path.display()); return Ok(()); }
    let contents = fs::read_to_string(&ledger_path)?;
    match next::extract_resume_state(&contents) { ... }
}
```

No plan file (TOML or Markdown) is read anywhere in this function; `derive_task` only reads the filename (`source.as_ref().or(plan.as_ref()).and_then(Path::file_name)...`), never the contents. The claim that `status --resume` "reads NO plan" and "its root falls back to the source-then-plan anchor `default_ledger_path` already uses" is TRUE, and I went in expecting to find a counterexample (some code path in `run_resume` that peeks at plan content) and did not.

### A-5. A nonexistent `--source` yields no canonical root

Claim: "a path that does not exist yields no canonical root," which is why the rejected second-condition alternative would not have covered the typo'd-`--source` case.

Verified experimentally rather than by inspection, since this is a claim about `std::path::Path::canonicalize`'s behaviour rather than about this crate's code:

```
$ cat > check.rs <<'EOF'
fn main() {
    let p = std::path::Path::new("/this/path/almost/certainly/does/not/exist/typo.plan.toml");
    match p.canonicalize() {
        Ok(c) => println!("OK: {}", c.display()),
        Err(e) => println!("ERR: {}", e),
    }
}
EOF
$ rustc check.rs -o check && ./check
ERR: No such file or directory (os error 2)
```

Confirms `canonicalize()` on a nonexistent path returns `Err`, so there is no canonical root to compare against, so the rejected "second condition on two roots" alternative would indeed have nothing to compare on a typo'd `--source`. TRUE.

### A-6. Direct reproduction of the divergent `--source`/`--plan` pairing against the live binary

This is the amendment's central factual claim about CURRENT (pre-inc2) behaviour, so I built it and ran it rather than trusting the prose.

Fixture A: `agent-scaffold scaffold --output-dir A --write --force --principles default --instrument`, then edited `A/docs/plans/TEMPLATE.plan.toml`'s `[meta].primary` from `"toml"` to `"markdown"`, then copied this repository's own `docs/metrics/workflow.jsonl` (250 records, including real convergent rounds for step `triager-runs-only-on-findings`) into `A/docs/metrics/workflow.jsonl`.

Fixture B: `agent-scaffold scaffold --output-dir B --write --force --principles default --instrument`, then renamed B's single step's slug to `triager-runs-only-on-findings` and its status to `complete` in `B/docs/plans/TEMPLATE.plan.toml`, fixed up the matching Step Detail heading/sidecar filename, and re-rendered `B/docs/plans/TEMPLATE.md` from it.

```
$ agent-scaffold validate --source A/docs/plans/TEMPLATE.plan.toml --plan B/docs/plans/TEMPLATE.md --workflow
A/docs/metrics/workflow.jsonl: 250 records, valid
A/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
B/docs/plans/TEMPLATE.md: 1 steps, 0 open-questions items, valid
B/docs/plans/TEMPLATE.md vs A/docs/metrics/workflow.jsonl: workflow invariants hold
$ echo $?
0
```

No `--metrics` flag was passed anywhere in this invocation; the metrics path was resolved purely by the DEFAULT (`resolve_metrics_path`, anchored on `--source`). This reproduces exactly what the amendment describes at `Q-55-endproperty` and pins at check 13b: a Markdown-primary `--source` in project A paired with a `--plan` in project B is checked against A's own real log and reports a false `workflow invariants hold` at exit 0, for a step (B's) with no round record of its own. TRUE, and it is the load-bearing empirical claim the whole amendment rests on.

Then, for the ledger side (the `next` bullet's new "SECOND WAY TO BE UNSAFE" text), I added `A/docs/plans/TEMPLATE.ledger.md` with a `## RESUME STATE` block naming itself as project A's own internal state, and ran:

```
$ agent-scaffold next --source A/docs/plans/TEMPLATE.plan.toml --plan B/docs/plans/TEMPLATE.md
task: TEMPLATE
source: B/docs/plans/TEMPLATE.md
metrics: 250 records

no active review loop (all steps complete)

RESUME STATE (verbatim from the ledger):
## RESUME STATE (compaction checkpoint, read this first)

THIS IS PROJECT A'S OWN INTERNAL RESUME STATE. If you see this printed while projecting project B's plan, that is the leak the amendment describes.
```

Again, no `--ledger-fragment` was passed. `source:` correctly echoes B (steps are projected from B, confirming the `toml_source(&args.source)`-falls-back-to-`--plan` selection), while the ledger is resolved from A (via `default_ledger_path`'s unconditional source-first anchor) and its `## RESUME STATE` block is echoed verbatim under B's plan. This reproduces exactly what the `next` bullet's new sentence describes ("resolves the ledger in the FIRST project while projecting the SECOND project's steps, and echoes one project's `## RESUME STATE` under another's plan on the DEFAULT ledger path") and what check 14g's fourth run pins. TRUE.

Both reproductions used ONLY default resolution (no `--metrics`, no `--ledger-fragment`), which is the fact that drives FI-1 below.

### A-7. Build and test baseline

`cargo build --release` succeeds. `cargo test --release` (with `TMPDIR` outside any repository): 373 + 5 + 1 + 1 + 9 + 3 + 1 + 2 = 395 tests, 0 failed. `render docs/plans/agent-scaffold.plan.toml --check` reports "up to date," confirming the projected `agent-scaffold.md` is a faithful regeneration of the sidecar plus the other sidecars, not hand-drifted. `grep -nP '[^\x00-\x7F]'` over both changed files returns nothing (0 non-ASCII characters in the diff).

## Part B: internal consistency

### FI-1 (MEDIUM): two unchanged rationale paragraphs claim the residual gap is exhaustively the explicit-`--metrics` case, which the amendment's own new material falsifies

**Where:** `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` line 183 (paragraph "A CORRECTION THE SECOND PASS OWES ON ITS OWN TEXT...") and line 289 (paragraph "WHY THE OMIT BEHAVIOUR LANDS IN INC2 AND NOT IN INC1..."); mirrored at `docs/plans/agent-scaffold.md` lines 1578 and 1684 respectively. Neither paragraph is touched by the diff (confirmed: `git diff main HEAD` shows no `+`/`-` lines inside either paragraph; both are used only as unmodified hunk-header context for nearby changes).

**The claims:**

- Line 183: "Anchoring changes where the DEFAULT resolves; it does nothing to an EXPLICIT `--metrics` naming a foreign log, and THE EXPLICIT CASE IS PRECISELY WHAT SURVIVES IT."
- Line 289: "the omit is TRIGGERED BY THE CONTAINMENT PREDICATE and by nothing cheaper. ITS SCOPE IS EXACTLY THE CASE THAT SURVIVES ANCHORING, AN EXPLICIT `--metrics` NAMING A FOREIGN LOG, and no lexical test separates that from an explicit `--metrics` naming the plan's own log spelled differently."

Both are affirmative exhaustiveness claims ("precisely," "exactly the case") that the residual gap surviving inc1's anchoring is confined to the EXPLICIT-`--metrics`-naming-a-foreign-log scenario.

**Why they are now false.** The amendment's own new paragraph ("WHY THE ROOT COMES FROM THE CHECKED PLAN AND NOT FROM THE ANCHOR," sidecar line 167) and the updated `next` bullet (sidecar line 191) establish a SECOND, independent way the gap survives anchoring: a Markdown-primary `--source` in one project paired with a `--plan` in another, resolved with NO explicit `--metrics` or `--ledger-fragment` at all, using pure default resolution. Check 13b and check 14g's "fourth run" (both added by this amendment) exist specifically to pin this second case, and both explicitly attribute it to `Q-55-endproperty`. I reproduced both halves directly against the built binary in A-6 above: the metrics false pass and the ledger leak both occur with `--source`/`--plan` alone, no explicit override anywhere in the command line. That is a direct, reproduced counterexample to "the explicit case is precisely what survives it" and to "its scope is exactly the case ... an explicit `--metrics` naming a foreign log": there is at least one more thing that survives inc1's anchoring, and it needs no explicit flag to trigger.

**Why this matters.** Both paragraphs are rationale/"why" prose explaining why `Q-55-refusalscope`'s omit behaviour had to land in inc2 rather than inc1 (line 289) and why the omit decision was necessary at all (line 183). The operative behavioural specification elsewhere in the SAME document (the `next` bullet's "SECOND WAY TO BE UNSAFE" sentence, checks 13b and 14g) is correct and complete, so an implementer following the acceptance checks literally would not miss anything. The risk is to a reader relying on either paragraph to understand the SCOPE of what the omit/refusal must cover: read in isolation, either paragraph would lead a reviewer to conclude that covering the explicit-`--metrics` case is sufficient, an inaccurate model of the decision that the rest of the document (correctly) does not share. This is exactly the "narrows one member of a pair, leaves the other" pattern this project has repeatedly found productive: the amendment updated the MECHANISM and BEHAVIOUR text to reflect the broader scope but left two "WHY" paragraphs asserting the narrower, now-superseded scope.

**Severity:** MEDIUM. Not HIGH because the actual behavioural specification and acceptance checks are correct and would not be missed by an implementer working the checklist; not LOW because it is a directly-falsifiable, reproduced exhaustiveness claim of exactly the class this project's own retrospectives call out as costly, sitting in prose that explains WHY inc2 is scoped the way it is.

**Prescribed fix (deletion-shaped, per this project's standing remedy for exhaustiveness claims):**

- Line 183: delete the exhaustiveness clause rather than rewrite it. Change "...it does nothing to an EXPLICIT `--metrics` naming a foreign log, and the explicit case is precisely what survives it (explorer A's second false pass..." to "...it does nothing to an EXPLICIT `--metrics` naming a foreign log (explorer A's second false pass...", i.e. delete "and the explicit case is precisely what survives it". The sentence's remaining content (anchoring doesn't fix the explicit-metrics case) stays true and needs no replacement claim.
- Line 289: delete the totalizing clause the same way. Change "the omit is TRIGGERED BY THE CONTAINMENT PREDICATE and by nothing cheaper. Its scope is exactly the case that survives anchoring, an explicit `--metrics` naming a foreign log, and no lexical test separates that from an explicit `--metrics` naming the plan's own log spelled differently" to "the omit is TRIGGERED BY THE CONTAINMENT PREDICATE and by nothing cheaper: no lexical test separates an explicit `--metrics` naming a foreign log from an explicit `--metrics` naming the plan's own log spelled differently", i.e. delete "Its scope is exactly the case that survives anchoring, " and let the "no lexical test separates..." clause stand on its own as an example rather than an exhaustive scope claim. The paragraph's actual argument (why a cheap lexical test cannot substitute for the canonical predicate on the spelling-ambiguity sub-case) is unaffected by the deletion.

Both fixes are pure deletions of the falsified totalizing language; neither requires authoring new prose describing the second case, since that is already stated correctly elsewhere (sidecar line 191, checks 13b/14g).

## Counts and enumerations re-counted

- **Decision receipts.** "SEVEN decision receipts... all carrying `task:"workflow-enforcement-tier"`": `grep -c '"task":"workflow-enforcement-tier"' docs/metrics/workflow.jsonl` -> 7. `grep` for `q_id` on those 7 lines lists exactly: `Q-55`, `Q-55-scope`, `Q-55-mechanism`, `Q-55-noconvention`, `Q-55-refusalscope`, `Q-55-jsonreason`, `Q-55-endproperty`. Matches. Cross-checked against `docs/plans/agent-scaffold.ledger.md` line 397 ("five `Q-55-<name>` sub-decision receipts already exist on this step" as of just before this fold, i.e. before `Q-55-endproperty` was added) - 5 named sub-receipts + 1 base `Q-55` = 6 (matching the pre-amendment "SIX"), +1 (`Q-55-endproperty`) = 7. Consistent on both sides.
- **`Q-55-endproperty`'s own receipt.** `grep '"q_id":"Q-55-endproperty"' docs/metrics/workflow.jsonl` shows `"ts":"2026-08-02"` (matches "the last taken on 2026-08-02") and three options ("Root on the plan the check reads" / chosen, "Add a second condition on the two roots", "Parse the triple at the boundary"), matching the "BOTH ALTERNATIVES WERE PUT AND REJECTED" paragraph's two rejected alternatives exactly (second condition on two roots; parsing the whole triple at the boundary).
- **Inc2's red-case count ("FOUR").** Counted the enumeration in the acceptance-check intro paragraph: check 11, check 13b, check 14b, check 14e = 4, matching "FOUR" exactly as a literal count of what is listed. I considered whether this undercounts given check 14g's new "fourth run" (also attributable to `Q-55-endproperty`) is not in the list, but the pre-amendment version already excluded other clearly-red checks (14c, 14f) from its "THREE," so the intro paragraph is evidently a curated illustrative list, not a claim to exhaustively enumerate every red-green pair in the acceptance check. I did not raise this as a finding after re-reading it this way; flagging it here in case a triager weighs it differently.
- **"Four doc comments" (`Q-55-jsonreason` section).** Counted the bulleted list: `no_active_loop_reason`'s doc comment, `NextProjection`'s own doc comment, `status`'s `Projection` doc comment, `resume_state`'s doc comment = 4, matching, with the fifth (pre-existing, `active_loop`'s doc comment) correctly called out as separate and not counted among the four. Unaffected by this amendment (not in the diff); confirmed still internally consistent.
- **README.md:228 quotation.** The sidecar quotes README.md verbatim ("Unlike `validate` it never fails on a missing or malformed file (a missing part is simply left out of the projection)"); `grep -n` confirms this exact string at README.md:228. Unaffected by the amendment but re-verified since it is asserted as present-tense fact.
- **`--metrics` false-pass record counts** ("233... 235... 235-record log," "37-record log"): these are historical measurement citations, unaffected by the amendment (not touched by the diff), not re-verified against a live count since the log has since grown to 250 records and the text already explains growth over time ("the record count grows as the log accumulates").

## What I did NOT find, despite specifically looking

- No unchanged text still describes the containment predicate's root as coming from the plan SOURCE (`--source` literally, as opposed to "the plan the check reads") in a way that contradicts the new rooting, other than the two instances in FI-1 (which are about SCOPE, not about WHICH file the root comes from).
- No stale reference to "the anchor" as the guard's root outside of the two FI-1 paragraphs and the deliberately-unaffected `status --resume` case (which correctly stays anchor-rooted, per the amendment's own "WHICH PLAN EACH SURFACE READS" paragraph, since it reads no plan).
- The vocabulary bullets (`log-not-this-project`, `ledger-not-this-project`) are both updated and consistent with how each surface is now described ("the root of the plan this surface reads"), including the `next`-only default-ledger clause on `ledger-not-this-project`.
- No other sidecar or `docs/plans/agent-scaffold.plan.toml` reference to this step's receipt count, red-case count, or increment scope needed updating; `status-resume-ignores-json.md` and `test-tmpdir-repo-assumption.md` reference this step only by name/order and are unaffected.
- `docs/plans/agent-scaffold.ledger.md` already carries its own `Q-55-endproperty` narrative (line 395) predating this fold, and it is consistent with the fold's content (same three options, same reasoning, same "planner fold is the next action" framing this commit fulfils).
