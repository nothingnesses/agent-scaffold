# `workflow-enforcement-tier-inc4`, round 4, cross-artifact consistency lens

Reviewer: opus. Worktree `review/wet-inc4-r4-b` at `7ab5d48`. Lens: does every artifact that states a fact state the SAME fact? This lens is pointed BETWEEN artifacts, not at any one of them.

## Result

FOUR findings: `R4B-1` (medium), `R4B-2` (medium), `R4B-3` (low), `R4B-4` (low). No `high` and no `critical`: I found no restatement that would make a user or an agent take a wrong action, and the shipped human output strings, the JSON token vocabulary, the tier-boundary sentence and every count in the waiver notes all agree across every site that states them.

I mapped 71 facts stated in more than one place and opened both (or all) sites of each. 67 agree. 4 disagree, and they are the findings below.

## Method, so the denominator means something

For each fact I opened every site rather than remembering one, and where the sites disagreed I established which is true against the tree by running the binary or by opening the cited range. Every reproduction below was run in this worktree against `target/debug/agent-scaffold` built from `7ab5d48`. Fixtures were built only under `<scratch>/rev-inc4-r4-b/`.

Two mechanical gates were run first, because they collapse a whole class of candidate findings:

```
$ cargo run -q -- render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date
exit=0
$ cargo run -q -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 296 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

`render --check` up to date means `docs/plans/agent-scaffold.md` is byte-derived from `agent-scaffold.plan.toml` and the sidecars, so the generated view CANNOT disagree with either source. Every candidate of the form "the rendered view says X and the sidecar says Y" reduces to a disagreement between the sidecar and some other artifact, and I checked it there. I say this explicitly because the round 1 triager's `R1C-5` treated a rendered-view self-contradiction as its own fact; it was a self-contradiction between two SOURCES that both render, not a projection defect, and that is still the only shape available.

---

## `R4B-1` (medium): the two reason definitions `Q-55-reasondefs` closed in the sidecar are still standing, verbatim, in `src/next.rs`, which is the file the sidecar was the spec FOR

The round 3 fix at `7ab5d48` (`Q-55-reasondefs`, "Close them now") deleted the words "of the plan this surface reads" from the sidecar's two reason definitions, on the measured ground that both reasons fire where the surface reads NO plan and the root comes from the anchors. The identical clause survives in the enum doc comments those definitions specify, in the same words, untouched.

FIXED SITE, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:217`:

> - `log-not-this-project`: the resolved path is not under the root, so the tool cannot vouch that its records belong to that plan. This is the `Q-55-refusalscope` case.

SURVIVING TWIN, `src/next.rs:103-107`:

```
    /// No file at the resolved metrics path.
    LogAbsent,
    /// The resolved path is not under the root of the plan this surface reads, so the
    /// tool cannot vouch that its records belong to that plan.
    LogNotThisProject,
```

(the leading hard tabs of the source are shown as spaces here, so this findings file stays ASCII-printable; nothing else is altered)

FIXED SITE, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:229`:

> - `ledger-not-this-project`: the resolved ledger is not under the root, which is either an explicit `--ledger-fragment` outside it or, on `next`, a DEFAULT ledger anchored on a `--source` that itself lies outside it.

SURVIVING TWIN, `src/next.rs:140-143`:

```
    /// The resolved ledger is not under the root of the plan this surface reads: either
    /// an explicit `--ledger-fragment` outside it, or a default ledger anchored on a
    /// `--source` that itself lies outside it.
    LedgerNotThisProject,
```

WHICH IS TRUE, MEASURED. The sidecar is right and `src/next.rs` is wrong. A Markdown-primary `--source` that resolves neither a TOML source nor a readable `--plan` makes the surface read no plan at all, and both reasons fire anyway.

Fixture: `<scratch>/rev-inc4-r4-b/projA` is a project whose `docs/plans/p.plan.toml` declares `[meta].primary = "markdown"`, with one step and its own empty `docs/metrics/workflow.jsonl`. Run from this worktree's root, with an explicit `--metrics` naming THIS repository's log and NO `--plan` at all:

```
$ ./target/debug/agent-scaffold status --source "$S/projA/docs/plans/p.plan.toml" \
    --metrics "$W/docs/metrics/workflow.jsonl" --json
{
  "plan": null,
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project"
}
exit=0
```

`"plan": null` is the tool reporting that it read no plan, and `log-not-this-project` is the reason firing anyway. The ledger half, same fixture, with a `--ledger-fragment` outside the plan's root:

```
$ ./target/debug/agent-scaffold next --source "$S/projA/docs/plans/p.plan.toml" \
    --ledger-fragment "$S/foreign.ledger.md" --json
{
  "task": "p",
  "source": "no plan source",
  ...
  "resume_state_absent_reason": "ledger-not-this-project",
  "no_active_loop_reason": "no-plan-steps"
}
exit=0
```

`"source": "no plan source"`, and `ledger-not-this-project` fires.

The code itself contradicts these two comments three files-worth of lines away. `src/main.rs:1409-1411`, `containment_roots`' doc comment: "Where NO plan is read, `checked_plan_root` has nothing to derive from, so the rule SUPPLIES a root from the anchors instead". `src/main.rs:1556-1557`, `resume_roots`: "The project roots the surfaces that read NO PLAN test their artifacts against: `status --resume` always, and `status` and `next` through `containment_roots` whenever no plan resolves."

NEVER TOUCHED AND NEVER RAISED, so this is not a re-raise:

```
$ git log --oneline -S 'not under the root of the plan this surface reads' -- src/next.rs
8beb1c2 feat: refuse and omit on a round log or ledger the plan cannot vouch for
$ grep -rn 'LogNotThisProject\|LedgerNotThisProject\|next.rs:10[0-9]\|next.rs:1[34][0-9]' docs/plans/agent-scaffold.reviews/
(no output)
```

Zero mentions across all twelve inc4 findings and triage files. Round 3's `R3B-1` reached the same class in the SIDECAR at `:163` and `:179` and quoted `src/main.rs` as the code that gets it right; nobody opened the enum definitions in `src/next.rs`, which get it wrong.

WHY IT IS IN SCOPE, on the condition-3 test this task has applied three times (`R1C-3`, `R1C-4`, `R2B-1`, `R3B-3`) and the human ruled on once (`Q-55-twinsites`). The comments were TRUE when written at `8beb1c2` (2026-08-03) and were falsified by `269d075` (2026-08-03, "fix: supply a root to the surfaces that read no plan"), an inc2 fix-round commit, which is this step's own increment. That is exactly `R3B-3`'s provenance shape, and `R3B-3` was ruled IN SCOPE and fixed. `src/main.rs` is already in the INC4 impact list at `:386`; `src/next.rs` is not, which is the reason these two survived, and is the same reason `R3B-3` gave for its own two twins ("the sidecar site that OWED this change is `:367` ... which names `run_resume`'s doc comment and only that").

SEVERITY medium. No behaviour is wrong and no user-visible string is wrong. What earns medium: these two comments are the ONLY definitions of two serialised contract tokens anywhere in the source, `Q-55-jsonreason` exists so a machine consumer can tell the causes apart, and a consumer reading the definition would conclude that a projection reporting `"plan": null` cannot also report `log-not-this-project` when it demonstrably can. It is also the fourth recorded twin-site defect in this task, at the site the sidecar's own spec was written for, one round after a human decision closed the same claim two sites away.

MINIMAL REMEDY: DELETION at both sites, the same four words the sidecar fix deleted. `src/next.rs:105` becomes "The resolved path is not under the root, so the tool cannot vouch that its records belong to that plan." `src/next.rs:140` becomes "The resolved ledger is not under the root: either an explicit `--ledger-fragment` outside it, or a default ledger anchored on a `--source` that itself lies outside it." Nothing is authored; every word is drawn from the already-corrected twin. If the fix pass touches `src/next.rs`, the INC4 impact list needs a bullet for it (see `R4B-2`).

---

## `R4B-2` (medium): the INC4 documentation-impact list, which `Q-55-impactlist` decided must be exhaustive, omits three of the sites this increment edited, and one of them contradicts acceptance check 21 in the same file

`Q-55-impactlist` (2026-08-08, "Add the missing bullet") was decided on exactly this ground, recorded at `docs/plans/agent-scaffold.ledger.md:561`:

> THE ITEM: the `INC4:` documentation-impact list ENUMERATES ITS EXCLUSIONS ("NOT `README.md`, NOT `pack/AGENTS.md`, NOT `CHANGELOG.md`"), so it reads as exhaustive, while OMITTING `tests/unsafe_pairings_are_refused_and_omitted.rs`, which this increment edited under `Q-55-twinsites`.

WHAT THE INCREMENT ACTUALLY EDITED, measured over the whole inc4 range:

```
$ git diff --stat 42ba172^ HEAD -- . ':(exclude)docs/plans/agent-scaffold.ledger.md' \
    ':(exclude)docs/plans/agent-scaffold.reviews' ':(exclude)docs/metrics/workflow.jsonl'
 docs/plans/agent-scaffold.md                       | 175 ++++++------
 docs/plans/agent-scaffold.plan.toml                |  14 +-
 .../checks-runner-worktree-name-collision.md       |   2 +-
 .../instrument-magic-filename.md                   |   2 +-
 .../status-resume-ignores-json.md                  |   4 +-
 .../test-tmpdir-repo-assumption.md                 |   6 +-
 .../workflow-enforcement-tier.md                   | 147 +++++-----
 src/main.rs                                        |   8 +-
 tests/unsafe_pairings_are_refused_and_omitted.rs   |   5 +-
```

WHAT THE LIST SAYS, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:384-388`, quoted whole:

> - THIS FILE, which is the increment's main artifact and is covered by acceptance check 21.
> - The three sidecars this step's line-number movement broke, covered by check 21b: `docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md`, `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md` and `docs/plans/agent-scaffold.steps/instrument-magic-filename.md`. ...
> - `src/main.rs:Projection`'s `plan` field doc comment, covered by check 22.
> - `tests/unsafe_pairings_are_refused_and_omitted.rs`, the two comment corrections `Q-55-twinsites` authorised; no acceptance check states them.
> - NOT `README.md`, NOT `pack/AGENTS.md` and NOT the deployed `.agents/` copies: inc4 changes no behaviour, so no shipped prose goes stale and no drift guard is touched. NOT `CHANGELOG.md`, for the same reason: inc4 corrects one user-visible string, `src/main.rs:StatusArgs::resume`'s `--help` text, and a corrected help string is a documentation fix rather than a change to read about.

THREE EDITED SITES ARE ABSENT FROM IT.

(a) `docs/plans/agent-scaffold.steps/status-resume-ignores-json.md`, a FOURTH sidecar, edited at `7a2e776`. The commit that edited it says so in its own message: "`status-resume-ignores-json.md` quotes the corrected comment verbatim and is updated with it, so this fix does not falsify a live sidecar." That SAME commit amended the exclusions bullet to record the `--help` change and did not add a bullet for the sidecar it had just edited. It is not covered by check 21 (which is scoped to "THIS FILE" and to two regions of the plan TOML), and it is not covered by check 21b (which names three sidecars "AND ONLY THOSE"). The list's own precedent for exactly this case is the `tests/...` bullet: name it, and say no acceptance check states it.

(b) `docs/plans/agent-scaffold.plan.toml`, edited at `3c7b04e`, `f607680` and `8a05468` under `Q-55-receiptcurrency` and `Q-55-w1figure`. THIS ONE CONTRADICTS AN ACCEPTANCE CHECK IN THE SAME FILE. Check 21 at `:345`:

> THE PLAN SOURCES ARE TWO REGIONS OF `docs/plans/agent-scaffold.plan.toml`, a file this increment edited: the `Q-55` question record and the three `workflow-enforcement-tier-w*` waiver notes.

The check says the increment edited it; the impact list, which enumerates its exclusions and so reads as exhaustive, does not list it. `R3B-2` was confirmed at medium on precisely this shape: "It is a document whose definition of done contradicts its own acceptance criterion."

(c) `src/main.rs:run_status`'s comment at `:1192-1195`, the third of inc4's three `src/main.rs` edits, corrected under `R3B-3` at `7a2e776`. The list names `Projection`'s `plan` doc comment and the `StatusArgs::resume` help string, and stops there.

SEVERITY medium. No behaviour is wrong. What earns medium is the same ground `Q-55-impactlist` was decided on, plus two aggravations: the step goes `complete` after this loop, so the impact list becomes the permanent record of what this increment touched, and one omission is a direct contradiction with an acceptance check three sections above it in the same file. A reader auditing "did inc4 touch anything it did not declare" gets `No` from the list and `Yes` from `git diff` and from check 21.

MINIMAL REMEDY: two bullets and one clause, all recording facts already settled and recorded elsewhere, nothing new asserted. One bullet for `status-resume-ignores-json.md` in the `tests/...` bullet's exact form. One bullet (or a clause on the `THIS FILE` bullet) for the plan TOML regions, drawn verbatim from check 21's own sentence. And "and `run_status`'s comment" appended to the `src/main.rs` line. If `R4B-1` is fixed, `src/next.rs` needs a bullet too and the count becomes four.

---

## `R4B-3` (low): `README.md:236` and `CHANGELOG.md:24` state the accepted-cost-(ii) symlink population differently, and the README one is false on the layout `R2B-4` measured

Four artifacts state which symlink layouts the containment guard refuses. Three agree with the tree. `README.md:236` does not.

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:257`, cost (ii), the definition:

> THE COST IS THE DIVERGENCE AND NOT THE LAYOUT: any symlink that makes the canonicalised checked plan and the canonicalised resolved log fall under different roots produces it, on either side, and `docs/plans` is the placement that was MEASURED rather than the population.

`CHANGELOG.md:24`:

> a layout in which `docs/plans` or `docs/metrics` is a symlink that lands the plan and the log under different real roots is now refused by `validate --workflow` and omitted by the projections

`README.md:236`:

> A layout where `docs/plans` or `docs/metrics` is a symlink pointing somewhere the other one is not under will now be refused by `validate --workflow` and left out by the projections, even though it worked before

The sidecar and the CHANGELOG state a condition on the two REAL locations landing under different roots. The README states a condition on the symlink TARGET not containing the other directory. Those are not the same condition, and the difference is reachable.

MEASURED, both orientations, fixtures under `<scratch>/rev-inc4-r4-b/sym` and `.../sym2`.

Case A, `docs/metrics` a symlink to an IN-ROOT sibling (`<root>/docs/metrics -> ../elsewhere`, plan where it belongs, log at `<root>/elsewhere/workflow.jsonl`):

```
$ cd "$S/sym/root" && agent-scaffold validate --source docs/plans/p.plan.toml --workflow
docs/metrics/workflow.jsonl: 0 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

NOT refused. `docs/plans` is not under `<root>/elsewhere`, so README's condition holds and README predicts a refusal that does not happen. The sidecar's and the CHANGELOG's condition does NOT hold (both files land under `<root>`), so both correctly predict no refusal.

Case B, the mirror, `docs/plans` a symlink to an in-root sibling, which is the layout check 19's FIRST clause pins:

```
$ cd "$S/sym2/root" && agent-scaffold validate --source docs/plans/p.plan.toml --workflow
--workflow would join docs/plans/p.plan.toml against docs/metrics/workflow.jsonl, which is
not under the plan's project root <S>/sym2/root/elsewhere; pass a `--metrics` under that
root, run against the plan's own log, or correct the `--source` and `--plan` pair
exit=1
```

Refused. The two orientations are asymmetric, which is exactly what the sidecar's and the CHANGELOG's phrasing captures and the README's does not.

NEW EVIDENCE AGAINST A SETTLED VERDICT, stated as the reviewer contract requires. Round 2's triage ruled `R2B-4` out of scope and asserted, in the same paragraph, that "`:257`, `README.md:236` and `CHANGELOG.md` all state the general rule correctly" (`docs/plans/agent-scaffold.ledger.md:565`). I am not re-raising `R2B-4`, whose subject was acceptance check 19's second layout and which has since been reworded. I am reporting that the clearing verdict given to `README.md:236` in passing was not measured, and that running the very case `R2B-4` measured against the README sentence falsifies it.

SCOPE, argued rather than asserted, because I expect it to be ruled out. The sentence was written in inc2 and was never true, so no increment of this step falsified it, and condition 1 and condition 2 both hold: `git log -S` puts it inside inc2 and no commit in `42ba172^..HEAD` touches `README.md`. By the recorded precedent that `R2B-4` was ruled on, this is OUT OF SCOPE for a documentation-currency increment, and `:388` excludes `README.md` explicitly. I report it under `R2B-4`'s own guard ("THE FIX IS RECORDED ANYWAY"), so the closing step does not lose it.

SEVERITY low. The failure direction is benign: a user with this layout is told it will be refused and finds that it works. No wrong value, no false green over foreign evidence, and the same paragraph's closing sentence states the true general rule ("The rule is CONTAINMENT, not identity"). What keeps it a finding at all is that two shipped surfaces describe one rule with two different conditions.

MINIMAL REMEDY, if it is ruled in: replace the README's condition with the CHANGELOG's, which is already in the tree and already correct. "a symlink pointing somewhere the other one is not under" becomes "a symlink that lands the plan and the log under different real roots". Nothing authored.

---

## `R4B-4` (low): the `Q-55` record and the step sidecar disagree about the second planner pass's increment set, three against four

`docs/plans/agent-scaffold.plan.toml:1726`, inside the `Q-55` `ask`, rendered at `docs/plans/agent-scaffold.md:160`:

> ... the second planner pass re-derived the set as THREE (the resolution rule and all its call sites; the containment refusal; the tier policy plus the documentation half), applying the same division principle rather than appending to the pair.

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:14`:

> THIS FILE IS THE SECOND PLANNER PASS.

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:275`, opening the increments section:

> The count is now UNDER-SPECIFIED rather than wrong: what the human decided is the DIVISION PRINCIPLE, and the set below is that principle re-applied to the widened scope, not an appendix to the old pair.

and "the set below" is FOUR bullets, `:279` to `:282`. The plan TOML carries four `[[step.increment]]` entries at `:1307-1321`.

So the record that a reader consults for this step's scope says the second planner pass produced three increments, and that pass's own file presents four as its set. Joining them gives two different answers to one question.

WHICH IS TRUE. Both, at different times, which is why neither is a lie and why it is still a disagreement a reader cannot resolve without `git log`:

```
$ git log --oneline -S 're-derived the set as THREE' -- docs/plans/agent-scaffold.plan.toml
7807c6b docs: re-derive the Q-55 increment set after the design pass
$ git log --oneline -S 'workflow-enforcement-tier-inc4' -- docs/plans/agent-scaffold.plan.toml
c43b1c6 docs: open the inc4 currency loop and close its scope
$ git log --oneline -S 'workflow-enforcement-tier-inc4`, THE DOCUMENTATION-CURRENCY PASS' \
    -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md
42ba172 docs: make the step's own claims current and specify inc4
```

The `THREE` clause is the second planner pass's own output. The fourth increment was added by a later pass, at the inc4 loop open. The sidecar records that fact at `:308` ("classified at loop-open") and `w1`'s note foresaw the pass ("A single documentation-currency pass is scheduled at the step close, after inc3"), but neither the `Q-55` record nor `:14` was updated, so `:14`'s self-identification now attributes a four-increment set to a pass that produced three.

`Q-55-receiptcurrency` re-tensed eight claims in this exact record without reaching this one. The round 1 triager's enumeration ("the twin count is SEVEN, across SIX lines") measured `:1722`, `:1724`, `:1728`, `:1732`, `:1734` and `:1736`; `:1726` was not among them.

SEVERITY low. Nobody acts wrongly on it: the increment set is authoritative in `[[step.increment]]`, which is correct, and the check that reads it passes. It is a finding because the step closes on this loop and the `Q-55` record is what a later reader opens to learn what the step was scoped as.

MINIMAL REMEDY, and both options are deletion-class or near it. Either delete the parenthetical count from `:1726` (leaving "the second planner pass re-derived the set by the same division principle rather than appending to the pair"), or leave `:1726` alone as a dated record and delete the four words "THIS FILE IS THE SECOND PLANNER PASS." from sidecar `:14`, whose surrounding sentences already carry everything that clause conveys. I do not choose between them; the first touches a decision receipt and the second touches the step file, and which of those this project prefers is a settled convention I should not re-derive.

---

## What I mapped and confirmed AGREEING, 67 facts

Recorded in full, because a clean result on a consistency lens is worth nothing without its denominator. Each line is a fact stated at two or more sites; I opened every site named.

COUNTS AND CALIBRATION (7), each checked against `docs/metrics/workflow.jsonl` with `jq`, not against another prose site:

1. inc1: three work-review rounds, 13 valid findings (3, 4, 6). Waiver `w1` note (`plan.toml:1330`), sidecar `:308`, rendered `:324`, four `type:"round"` records. AGREE.
2. inc1's streak never left 0 of 2. `w1`, `consecutive_clean` on all three records. AGREE.
3. inc2: four rounds, 24 valid (9, 5, 6, 4), ceilings high, high, medium, high. `w2` (`:1339`), rendered `:324`, records. AGREE, including every ceiling.
4. inc3: five rounds, 14 valid (6, 4, 2, 0, 2), ceilings medium, medium, low, none, medium, peak streak 1. `w3` (`:1348`), rendered `:324`, records. AGREE, including the round 4 `consecutive_clean: 1`.
5. Step 92 (`prompt-drift-guard`): six rounds, fifteen valid findings, all prose. Sidecar `:306` and `:308`, records (4+3+5+1+2+0 = 15, all `low`). AGREE at both sites.
6. Project median of two rounds per artifact, and step 92 joint-third. Sidecar `:306`; computed over 85 artifacts, median 2, ranking 9, 7, 6, 6. AGREE.
7. inc4 is `risky` (two consecutive clean rounds). `plan.toml:1320-1321`, sidecar `:308`. AGREE.

INCREMENT SET AND ORDER (2 agreeing, 1 disagreeing at `R4B-4`):

8. Four `[[step.increment]]` entries, and four bullets at sidecar `:279-282`, and four in the rendered view. AGREE.
9. Order inc1 -> inc2 -> inc3 -> inc4, with inc4 last because the code stops moving. Sidecar `:282`, `:296`, `:352`, `w1`'s closing sentence. AGREE.

BACKLOG STEP ORDERS AND STATUSES (10), sidecar against `plan.toml`:

10-19. `instrument-magic-filename` 60; `sidecar-ref-empty-string` 63; `sidecar-ref-symlink` 64 and `deferred`; `triager-runs-only-on-findings` 78; `reviewer-reproducible-evidence` 88; `prompt-drift-guard` 92; `checks-runner-worktree-name-collision` 93 and `complete`; `workflow-enforcement-tier` 94; `test-tmpdir-repo-assumption` 95; `status-resume-ignores-json` 96. ALL AGREE.

THE TIER BOUNDARY (6):

20. `pack/AGENTS.md:93` is byte-identical in all three deployed locations. `grep -no` on `pack/AGENTS.md`, root `AGENTS.md` and `.agents/AGENTS.reference.md` returns the same string at the same line in each. AGREE.
21. The boundary is drawn at "a project with no round log yet, which every project scaffolded without `--instrument` remains". `pack/AGENTS.md:93`, `README.md:210`, `CHANGELOG.md:23`. AGREE, same clause in all three.
22. Sidecar `:377`'s "WHAT IT MUST SAY" (instrumented tier; no round log without `--instrument`; refuses rather than passing) against the shipped sentence. AGREE, all three requirements met.
23. Sidecar `:139`'s verbatim quotation of the PRE-inc3 sentence, framed "CONTAINED, verbatim", and the current sentence. AGREE: the quoted form is gone from the tree and the frame is past.
24. `pack/AGENTS.md:61` and `:63` are the only `docs/metrics/workflow.jsonl` mentions outside the instrumentation section, both under "When instrumentation is on"; `{{instrument}}` at `:116`; `pack/instrument.md:15` closing line. Sidecar `:139`. AGREE, all four.
25. Plain `validate` unaffected by the tier policy. Sidecar `:48`, checks 10 and 16, `README.md:210`, `CHANGELOG.md:23`. AGREE.

THE JSON REASON VOCABULARY (8):

26. `metrics_absent_reason` tokens `log-absent` / `log-not-this-project`. Sidecar `:216-217`, `src/next.rs:100-108`, `README.md:240`, `src/next.rs:1951-1952`. AGREE on the tokens.
27. `no_active_loop_reason` tokens `no-plan-steps` / `all-steps-terminal` / `metrics-not-this-project`. Sidecar `:221-223`, `src/next.rs:118-130`, `README.md:240`. AGREE.
28. `resume_state_absent_reason` tokens `ledger-absent` / `no-resume-section` / `ledger-not-this-project`. Sidecar `:227-229`, `src/next.rs:133-144`, `README.md:240`, `status-resume-ignores-json.md:97`. AGREE.
29. "`Some` exactly when `<part>` is `None`". Sidecar `:214` and `:225`, `src/next.rs:179` and `:187-188`, `src/main.rs:575`. AGREE.
30. The precedence rule, unsafe beats absent. Sidecar `:231`, checks 14f and 14g, `src/main.rs:1646-1647`, `src/next.rs:1988`. AGREE.
31. The correlation rule and the deliberately shared `not-this-project` token. Sidecar `:233`, `src/next.rs:125-129`, `README.md:240`. AGREE, including the "without a lookup table" reasoning at all three.
32. No `skip_serializing_if` anywhere in `src/next.rs` or `src/main.rs`; the new fields are always present and `null` in the normal case. Sidecar `:206` and `:337`; `grep -c` returns 0 in both files; `GOLDEN_JSON:2083, 2119` carry the nulls. AGREE.
33. The enum is the machine value only and the caller assembles the human message. Sidecar `:212`, `src/next.rs:193-203` (`metrics_absent_note`, `resume_state_absent_note`, both `#[serde(skip)]`), `src/next.rs:1184-1198`. AGREE.

THE FOUR PLUS ONE DOC COMMENTS `Q-55-jsonreason` FALSIFIED (6):

34. `NextProjection::no_active_loop_reason` no longer `#[serde(skip)]` and no longer claims the contract is exactly the fields above. Sidecar `:199` and `:219`, `plan.toml:1734`, `src/next.rs:189-192`. AGREE.
35. `NextProjection`'s own doc enumerates three causes. Sidecar `:200`, `src/next.rs:162-167`. AGREE.
36. `status`'s `Projection` doc enumerates the same three, preserving the "mirrors" cross-reference. Sidecar `:201`, `src/main.rs:561-567`, `src/next.rs:164`. AGREE, and the two are consistent with each other, which is what `:201` says both must be.
37. `resume_state`'s doc names three causes. Sidecar `:202`, `src/next.rs:184-185`. AGREE.
38. `active_loop`'s doc no longer names "every pending step blocked". Sidecar `:204`, `src/next.rs:181-182`. AGREE, and no blocked-steps variant was added, as `:204` forbids.
39. `Projection.plan` no longer says "present only when a readable `--plan` was given". Check 22, `src/main.rs:570-571`. AGREE.

SHIPPED OUTPUT STRINGS (5), each checked as a literal in the source AND against a run:

40. `metrics: unavailable, <note>`. Sidecar `:181` and `:183`, `README.md:238`, `CHANGELOG.md:24`, `src/main.rs:1278`, `src/next.rs:1156`; observed in case B of `R4B-3`. AGREE.
41. `metrics: no log found`. Sidecar `:321`, `src/main.rs:1279`, `src/next.rs:1157`. AGREE.
42. `no active review loop (<reason>)`. Sidecar `:183`, `src/next.rs:1144`, `:1162`, `:1184`. AGREE.
43. The refusal message names the plan, the log, the root and three remedies. Sidecar `:157`, `README.md:225-231`'s example block, `src/main.rs:1000`; observed verbatim in case B of `R4B-3`. AGREE, including the third remedy `Q-55-endproperty` added.
44. `status --resume`'s three-cause note. Sidecar `:367` (past-framed), `src/main.rs:461` (`--help`), `:1192-1195` (comment), `run_resume`'s doc at `:1632-1636`, `src/next.rs:184-185`, and `status-resume-ignores-json.md:15-16`, which quotes the corrected comment verbatim. AGREE at all six, which is the twin sweep `R3B-3` and the round 3 fix pass got RIGHT.

ACCEPTED COSTS (5 agreeing, 1 disagreeing at `R4B-3`):

45. Cost (i), the bare filename from inside `docs/plans`: a silent miss before inc3, a hard failure after. Sidecar `:255`, check 18, `README.md:234`, `CHANGELOG.md:23`, `plan.toml:1736`. AGREE.
46. Costs (iii) and (iv) and their receipts. Sidecar `:259` and `:261`, check 19b, `Q-55-conventionlesscost` and `Q-55-resumecost` receipts present in the log. AGREE.
47. Four costs, attributed to `Q-55-noconvention` (two of them), `Q-55-conventionlesscost` and `Q-55-resumecost`. Sidecar `:253`, `plan.toml:1736` ("TWO ACCEPTED COSTS come with this choice"). AGREE.
48. The deliberate `--metrics`-outside-root break must be in the CHANGELOG. Sidecar `:263`, `CHANGELOG.md:24`. AGREE, it is there.
49. Cost (ii) has two manifestations, loud on `validate` and quiet on the projections. Sidecar `:257`, `:288`, check 19, `README.md:236`, `CHANGELOG.md:24`. AGREE on the two manifestations (only the POPULATION diverges, at `R4B-3`).

THE SIBLING SIDECARS (5):

50. The three ambient-`TMPDIR` test names. Sidecar `:314`, `test-tmpdir-repo-assumption.md:13-16`. AGREE, all three identical.
51. `test-tmpdir-repo-assumption.md`'s re-pointed `src/main.rs:2289-2305` and `:2878-2889`. Both opened: `init_plan_defaults_to_git_and_skips_inside_a_repo` and `install_precommit_hook_skips_a_non_repo` are there. AGREE.
52. `test-tmpdir-repo-assumption.md:35`'s `src/main.rs:2279-2287` and `checks-runner-worktree-name-collision.md:55`'s `src/main.rs:2280-2285`, two different ranges for the same `fn scratch`. Both opened; both hold the named subject; the offsets are consistent with the pre-inc4 pair. AGREE.
53. `instrument-magic-filename.md`'s re-pointed `src/main.rs:257-258`. Opened: `source.read("instrument.md").unwrap_or_default()` for the `{{instrument}}` slot. AGREE.
54. `status-resume-ignores-json.md:97` and `:124` on inc2's vocabulary and the (B) ordering. Checked and NOT raised: "IT MUST NOT BE BUILT BEFORE ... INC2 LANDS" is a constraint whose precondition is now met rather than a false statement, and the vocabulary it names matches. AGREE.

SHARED CODE CITATIONS, each cited from two or more places or load-bearing for two claims (8):

55. `src/workflow.rs:180-195`, `check_workflow_toml`. Sidecar `:102`. Opened, correct.
56. `src/workflow.rs:448-449`, W3's bare-slug join. Sidecar `:102` and `:267`, plus `src/main.rs:1490-1491`. Opened, correct, and the two prose sites say the same thing.
57. `src/plan/source.rs:480-495`, `is_safe_sidecar_ref`. Sidecar `:245` and `:302`. Opened, correct at both.
58. `src/plan/source.rs:102`, `#[serde(deny_unknown_fields)]` on `Meta`. Sidecar `:247`. Opened, correct.
59. `src/plan/render.rs:296` (`meta.title`) and `:167-169` (`meta.sidecars`). Sidecar `:247`. Opened, both correct.
60. `src/findings_naming.rs:52-55`. Sidecar `:185` and `:404`. Opened, `join_dir` builds from the task name, which is what both sites claim.
61. `justfile:46-48`, the render-then-`nix fmt` recipe. Sidecar `:378`. Opened, correct.
62. `tests/validate_workflow_toml_source_needs_no_plan.rs:127-171` and the `(None, None, _)` arm's comment quoted at sidecar `:52`. Both opened; the test spans exactly `127-171`; the comment matches at `src/main.rs:1039-1041` modulo `//` wrapping.

THE `run_validate` MATCH (3):

63. A four-arm match over `(toml_primary, &plan_contents, &metrics_contents)`. Sidecar `:46`, `src/main.rs:1005`. AGREE.
64. Arms `(Some(source), _, Some(metrics_text))` and `(None, Some(plan_text), Some(metrics_text))` are the two plan selections. Sidecar `:163`, `src/main.rs:1010` and `:1024`. AGREE.
65. The `_` catch-all still exists and now pushes a problem. Sidecar `:46`, `:281`, `:306`, `src/main.rs:1067`. AGREE at all three prose sites.

THE GOLDENS (2):

66. `GOLDEN_JSON`, `golden_json`, `GOLDEN_HUMAN`, `golden_human_text` all exist. Sidecar `:208` and `:337`, `src/next.rs:2052, 2077, 2132, 2137`. AGREE.
67. `awaiting-reviewers` is on the wire in the golden (sidecar `:212`, `GOLDEN_JSON:2088`), and `"resume_state": null` appears in it (sidecar `:337`, `GOLDEN_JSON:2117`). AGREE.

Plus, counted inside the numbers above rather than separately: the eight `Q-55-receiptcurrency` twins at `plan.toml:1722, 1724, 1728 (twice), 1732, 1734, 1736` and the `w1` figure, each opened and each correctly re-tensed; `CHANGELOG.md`'s `[Unreleased]` carrying `Added` and `Changed` and no `Fixed`, stated at sidecar `:360` and `:370` and true of the file; `README.md` documenting no `next` section, stated at sidecar `:366` and `:369` and true; and "no malformed-log variant is distinguished today", stated at sidecar `:237` and `:398` and true of `count_records` and `parse_rounds`.

## Checked and deliberately NOT raised

- The `Q-55` `ask`'s opening paragraph (`plan.toml:1718`, rendered `:152`) still says `validate --workflow` "silently passes" and that the split is "undocumented in the scaffolded AGENTS.md". Both are false of the tree. NOT A FINDING: the record appends its own correction at `:1722` ("CORRECTION TO THIS ITEM'S OWN WORDING") and at `:1728` ("the two-tier split was undocumented"), and sidecar `:44` QUOTES the uncorrected wording in a live present-tense frame ("`Q-55` says `validate --workflow` 'silently passes'"), which resolves only while `:1718` is left as written. The two artifacts are consistent with each other by design; re-tensing `:1718` would falsify sidecar `:44`.
- Sidecar `:182`'s "a `--source` and a `--plan` that both exist must resolve to the SAME root or the block is omitted". The shipped rule is "the artifact must be under EVERY DECIDING anchor's root", which is not equivalent in a nested layout. NOT A FINDING for this lens: `resume_roots`' own doc comment at `src/main.rs:1559-1561` uses the SAME "must resolve to the SAME root" phrasing and reconciles it in place, so the two artifacts agree. The residual that `:182` states the anchor-supplied root only for `status --resume` is an ACCEPTED residual, recorded by the round 3 triager in advance.
- Sidecar `:288` and `:304` name cost (ii) by its measured placement ("the symlinked `docs/plans` directory") rather than by the population `:257` defines. NOT A FINDING: unlike `:104`, which `R3B-2` fixed, neither is an exception clause on a requirement, and both are true statements about the layout that was measured.
- The `src/checks.rs` citations in `checks-runner-worktree-name-collision.md`, one deliberately reverted to a stale value by `Q-55-check21b`. Out of scope by the brief.

## Not reached

- I did not execute the 23 acceptance checks; another reviewer this round owns them. Where a check's TEXT restates a fact stated elsewhere (checks 16, 18, 19, 19b, 20, 21, 21b, 22) I compared the text against its twin, but I did not run checks 3 to 15 or 17.
- I did not run `cargo test` or `cargo clippy`. My changes are confined to this file.
- I did not verify `w1`'s "51 adversarial attacks" or "claim inventories of 81 and 118 claims" against a primary source. The inc1 findings files were cleaned up at `a932e47`, the ledger is the only remaining site, and the brief directs that the ledger is evidence of what was decided rather than an artifact that must be current. I found no restatement of those figures anywhere that CONTRADICTS `w1`, which is all this lens can establish.
- I did not sweep the `pack/prompts/` or `.agents/prompts/` role prompts. Sidecar `:380` states that no prompt says where the log is resolved from; I did not independently confirm it, because no second site restates it.

## Fixture hygiene

All fixtures were created under `<scratch>/rev-inc4-r4-b/` and nothing outside it was created, modified or deleted. No `chmod` was used, so none needs restoring. Nothing in the main repository or in any other worktree was touched; every command named this worktree by absolute path or ran with its root as the working directory.

## ASCII check

```
$ LC_ALL=C grep -n '[^ -~]' docs/plans/agent-scaffold.reviews/workflow-enforcement-tier-inc4-r4-crossartifact-opus.md
(no output, exit 1)
```
