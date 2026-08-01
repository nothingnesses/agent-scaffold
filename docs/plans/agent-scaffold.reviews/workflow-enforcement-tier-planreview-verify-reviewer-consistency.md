# `workflow-enforcement-tier` plan review, VERIFICATION ROUND, reviewer: internal consistency

Reviewer model: Claude Opus 5 (1M context). Exact model id `claude-opus-5[1m]`.
Worktree: `.claude/worktrees/verify-q55-b`, branch `verify/q55-b` at commit `61fc8b2`, the commit under review. `TMPDIR` for scratch work was `/tmp/verify-b-scratch`, outside any git repository.
Artifact: the `workflow-enforcement-tier` fold (`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, `docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md`, `docs/plans/agent-scaffold.steps/status-resume-ignores-json.md`, and the `[[step]]` / `[[question]]` entries in `docs/plans/agent-scaffold.plan.toml`).
Lens: is the document now internally consistent, especially about counts, after a fix pass four of whose seven edits were count edits.

## Verdict

CLEAN. ZERO FINDINGS.

Every count claim in the fold was re-derived from the thing being counted rather than taken from any stated numeral, including the numerals the fix just wrote. Twenty-eight count claims re-derived, every one agreeing with the list or artifact it counts and with every other statement of the same count in the fold. Fourteen exclusivity or absolute claims tested, all true or correctly qualified. All ten deletion and edit sites checked for orphaned referents, none orphaned. Cross-sidecar and sidecar-to-TOML consistency checked on every claim the fix touched and on every shared claim, no divergence.

The seven round 4 findings are all closed at every site, not only at the sites the reviewers cited: `three doc comments` and its two other spellings return zero across all three sidecars and the plan TOML, `five measured grounds` returns zero, `only part of the mechanism` returns zero, and `responses` now carries a numeral at exactly the two sites the round 4 triage ruled correct. The three edits that author words (`:111`, `:226`, `:343`) introduce no new count, no new exclusivity claim and no new vocabulary.

Four candidate findings were examined and NOT raised. Each is recorded in full in the re-derivation section with the reasoning, so the record shows what this clean result was based on rather than only that it was clean. The closest call is `:298`'s surviving partitive "two of which must NOT fail"; my analysis of why it is not a finding is at the head of the near-miss section.

I did NOT raise `INC2-7` or `F-5`, both of which I confirmed still present and unchanged, and I did not raise the plan TOML's unprojected step `title`.

---

# RE-DERIVATION

Everything below was computed by running a command against the tree at `61fc8b2`, not by reading a stated number. Negative results are included, because a clean result merges the artifact and the record should show its basis.

## What the fix actually changed, established before checking anything

```
$ git diff c63a1e8 61fc8b2 --stat
 docs/plans/agent-scaffold.md                         | 20 ++++++++++----------
 docs/plans/agent-scaffold.plan.toml                  |  2 +-
 .../workflow-enforcement-tier.md                     | 18 +++++++++---------
 3 files changed, 20 insertions(+), 20 deletions(-)
```

Nine changed lines in the primary sidecar plus one in the plan TOML, carrying eleven edits (`workflow-enforcement-tier.md:298` carries two independent ones). That is exactly the triage's prescribed edit set at `-r4-triage.md:291`, with no collateral change. `docs/plans/agent-scaffold.md` changed on exactly ten lines, one per edited source line, and is a generated projection.

Nothing outside `docs/plans/` was touched, so no code claim in the fold can have gone stale through the fix itself.

## Projection and structural gates

```
$ cargo run -- render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date          exit=0
$ cargo run -- validate --source docs/plans/agent-scaffold.plan.toml
docs/metrics/workflow.jsonl: 244 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid   exit=0
```

The re-render was done rather than hand-edited, so the human projection carries the fixed text at every one of the ten mirror sites the triage listed.

## Counts re-derived, with the number I got

Each row is the thing counted, the number I counted, and every site in the fold that states that count. A row is consistent only if every site agrees with my number AND with every other site.

| what I counted | my number | sites stating it | verdict |
| --- | --- | --- | --- |
| defects in the family | 4 (`:5` A, `:6` B, `:7` C, `:8` D) | `:3` "Four defects", `:304` "all four defects" | consistent |
| decision receipts for the step | 6 distinct `q_id`s in `docs/metrics/workflow.jsonl` (`Q-55`, `-scope`, `-mechanism`, `-noconvention`, `-refusalscope`, `-jsonreason`) | `:10` "SIX decision receipts", six bullets `:12-17`, six lettered paragraphs in TOML `Q-55` | consistent |
| exploration record lines | 521 + 483 + 510 = 1514 | `:19` "1514 lines" | consistent, arithmetic re-done |
| corrections to the FIRST planner pass | 3 (`:117`, `:136`, `:248`; the `:51` correction is to `Q-55`'s own wording and `:176` is the second pass correcting itself, both correctly uncounted) | `:21` "three of its factual claims", `:117` "the most consequential of the three" | consistent |
| human scope additions | 2 (defects C and D) | `:21` "Two human scope additions", TOML "TWO SCOPE ADDITIONS" | consistent |
| scaffolded fixture files | 30 in prose, 30 in the quoted tool output | `:31` twice | consistent |
| arms in the `--workflow` match | 4, catch-all at `src/main.rs:999-1003` | `:53`, `:164` "four-arm" | consistent |
| agent-scaffold log records in the reproductions | one value, 235, at 6 sites (`:75`, `:81`, `:122`, `:164`, `:264`, TOML `:1694`); the 233 at `:72` is explicitly the earlier reproduction | `:72` explains the drift | consistent |
| stdout lines on the correct case | 3 (`:75-77`) | `:166` "two of the three printed lines", `:316` "its three stdout lines" | consistent |
| places `pack/AGENTS.md` mentions the round log | `grep -c` returns 2, at `:61` and `:63`, both inside "When instrumentation is on" | `:146` "The only two places" | consistent, and the exclusivity is TRUE |
| `StepPhase` variants | 7 (`src/next.rs:388-396`) | `:205` "the seven `StepPhase` variants" | consistent |
| `#[serde(skip)]` in `src/` | `grep -rc` returns 1, in `src/next.rs` | `:207` "exactly ONCE" | consistent |
| `skip_serializing_if` in `src/next.rs` and `src/main.rs` | 0 and 0 | `:207` "No `skip_serializing_if` appears in either" | consistent |
| doc claims falsified or left incomplete | 4 (`:200` `src/next.rs:114-115`, `:201` `:95-97`, `:202` `src/main.rs:561-564`, `:203` `src/next.rs:111-112`); the `:205` item is excluded by its own pre-existing ruling | `:198`, `:275`, `:298`, `:354`, TOML `:1704` | consistent at ALL FIVE sites; this is the `R4B-1` fix and it converged |
| grounds candidate (d) is rejected on | 4 substantive bullets (`:244`, `:245`, `:246`, `:247`), the fifth (`:248`) disclaiming ground status | `:248` "the four grounds above"; `:242` now states no count | consistent; `:248`'s "above" refers to the four bullets literally above it and is unharmed by the deletion |
| responses the predicate yields (kinds sense) | 2 (refuse, omit) | `:168` heading "two responses", `:284` "two responses (refuse, omit)" | consistent; no site now states a competing count |
| per-surface answers `Q-55-refusalscope` settles | 4 enumerated at `:275`, 3 bullets at `:182-184` | neither carries a numeral | consistent by construction; the fix removed all three numerals from this sense |
| `metrics_absent_reason` variants | 2 (`:217`, `:218`) | check 14f exercises both plus precedence | consistent |
| `no_active_loop_reason` variants | 3 (`:222`, `:223`, `:224`) | `:220`'s "collapses it to two answers" is about the pre-existing strings, not the variant set | consistent |
| `resume_state_absent_reason` variants | 3 (`:228`, `:229`, `:230`) | `:226` "the three causes", `:377` "the same three causes", `status-resume-ignores-json.md:97` "the same three causes" | consistent at all four sites |
| `resume_state` causes distinguished in TODAY's code | 2 branches at `src/main.rs:1207-1212` | `:226` "Two of the three causes are already distinguished" | consistent; this is the `R4B-7` fix and it keeps "three" for the variant list while making "already" true |
| accepted costs | 2 ((i) `:256`, (ii) `:258`); the `:260` item is explicitly "NOT a cost" | `:252`, `:254`, `:351`, TOML "TWO ACCEPTED COSTS" | consistent |
| increments | 3 (`:274`, `:275`, `:276`) | `:268` "The three increments", TOML `:1309-1319` three `[[step.increment]]`, TOML `Q-55` "re-derived the set as THREE" | consistent |
| call sites in inc1 | 4 (`validate`, `status`, `next`, `default_ledger_path`) | `:278` "THE FOUR CALL SITES" | consistent with `:274`'s enumeration |
| commands whose metrics file changes | 3; ledger read by 2 of them | `:296` "THREE commands ... plus which LEDGER two of them read" | consistent with `:274` |
| `--metrics` help strings | 3 (`src/main.rs:429-431`, `:455-457`, `:479-481`) | `:274` three arg structs, `:342` "the three `--metrics` help strings" | consistent |
| inc2 red cases | 3 (checks 11, 14b, 14e, all three present in the list) | `:304` "for inc2 there are THREE" | consistent |
| deployed pack copies | 2 (root `AGENTS.md`, `.agents/AGENTS.reference.md`) | `:276`, `:300`, `:364`, check 20 | consistent |
| workarounds closed by earlier increments | 2 (standing elsewhere; explicit `--metrics`) | `:290` "the two workarounds" | consistent |
| suite tests needing a non-repo `TMPDIR` | 3 named tests | `:306`, `:380`, `test-tmpdir-repo-assumption.md:3`, `:9`, `:38`, TOML step title "3 tests" | consistent, and the three names match byte for byte across both sidecars |
| clap constraint attributes in `src/main.rs` | 5 (`:396`, `:442`, `:465`, `:525`, `:557`) | `status-resume-ignores-json.md:92`, `:120` | consistent |

TWENTY-EIGHT ROWS, TWENTY-EIGHT CONSISTENT. No count in the fold disagrees with the thing it counts or with another statement of itself.

## Exclusivity and absolute claims tested

Swept with `grep -noE '.{50}\bonly\b.{80}'` over the primary sidecar and by reading both siblings in full, then each claim tested against the rest of the fold.

| claim | site | verdict |
| --- | --- | --- |
| the only thing on stdout is the ok summary | `:51` | TRUE against the observed block at `:45-49` (one stdout line, two stderr) and against `:53`'s account of the catch-all arm |
| a project-root run must be unchanged, EXCEPT accepted cost (ii) | `:111` | CORRECTLY QUALIFIED after the fix. See the dedicated check below |
| the only two places `pack/AGENTS.md` mentions the round log | `:146` | TRUE, `grep -c` returns 2, both at the cited lines |
| the refusal remains the validator's alone | `:172` | TRUE, consistent with `:182-184`, `:275`, check 14 at `:321`, `:374`, TOML `Q-55-refusalscope` |
| `next` must emit no ACTION or `summary` from a log it cannot vouch for | `:186` | consistent with check 14b's field-by-field list at `:322` |
| `active_loop` is `None` ONLY when there are no steps or every step is terminal | `:205` | TRUE in the tense the sentence chooses. Examined at length in the near-miss section |
| `#[serde(skip)]` appears exactly ONCE in `src/` | `:207` | TRUE, verified by `grep -rc` |
| `status --json` has no golden and no test on its serialisation | `:209` | consistent with `:284`, `:298`, check 14e |
| `status`'s `plan` field has exactly ONE cause | `:238` | consistent with `:376`'s scope bullet |
| `status --resume` has NO JSON surface at all | `:236` | consistent with `:327`, `:377`, `status-resume-ignores-json.md:3`, `:22`, `:124` |
| inc2 is the only place where two different resolutions run against each other | `:298` | TRUE: inc1 is lexical only (`:158`), the canonical guard is inc2's (`:164`, `:166`), inc3 introduces no resolution rule (`:276`) |
| the responses are only reviewable against each other | `:282` | a design claim, unchanged in substance by the numeral deletion, and consistent with `:284` |
| identity is the ONLY mechanism separating projects sharing one merged log | `:266` | consistent with `:264`'s "Neither the anchor nor the refusal touches this" |
| the one artifact species with calibration data | `:300` | external to the fold, unchanged, uncontradicted inside it |

THE FAMILY THAT PRODUCED A FINDING IN THREE SEPARATE ROUNDS IS NOW EMPTY. The literal sweeps for every wording rounds 1, 3 and 4 each fixed return zero:

```
$ grep -n "currently-succeeding\|only part of the mechanism\|previously-green\|exited 0 before" \
    docs/plans/agent-scaffold.steps/*.md docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md
(no output)
$ grep -n "still exits 0" docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md
321:14. AFTER INC2, the REFUSAL is correctly scoped to the validator: it does not fire without `--workflow` ...
```

The single surviving `still exits 0` is check 14's correct statement that the refusal does not fire without `--workflow`, which rounds 1 and 3 both cleared and which I re-read and confirm is correct.

## Deletion and edit sites checked for orphaned referents

The artifact has already suffered a deletion orphaning a referent three clauses later, so every one of the ten edited lines was checked for a dangling pronoun, demonstrative, partitive or back-reference.

| site | what changed | orphan check | verdict |
| --- | --- | --- | --- |
| `:111` | clause appended | "recorded below as accepted cost (ii)" needs (ii) to exist below: it does, at `:258`, and the phrase "accepted cost (ii)" is the document's own, used at `:258`, `:280`, `:298`, `:333` | no orphan |
| `:180` | sentence "One predicate, three consumers, three responses." deleted | the same sentence's surviving "all three cases" is a FORWARD reference to the three bullets at `:182-184`, unaffected by the deletion. `grep -rn consumers` over the fold returns only `:304` ("several consumers", no numeral), so no later text refers back to "three consumers" | no orphan |
| `:226` | "The three" -> "Two of the three" | "so naming them costs nothing" now takes "the two" as its antecedent, which is what makes the sentence true; the three-bullet list at `:228-230` still matches the surviving "three" | no orphan |
| `:242` | "five" deleted | THE HIGHEST-RISK SITE, because `:248` says "the four grounds above". `:248` counts the bullets literally above it (`:244`, `:245`, `:246`, `:247` = four), not the deleted numeral, so the deletion cannot orphan it. `grep -rn grounds` over all three sidecars and the TOML returns exactly two hits, `:242` and `:248` | no orphan |
| `:275` | "three" -> "four" | numeral only, no referent attached | no orphan |
| `:280` | first clause and its semicolon deleted | the surviving "It carries a known false positive ..." takes "the predicate" from the paragraph's own heading. Nothing later cites the deleted claim (`grep` for its wording returns 0 fold-wide). `:282` and `:284` argue from the division rule at `:272`, not from `:280` | no orphan |
| `:282` | "three" deleted from "the three responses" | "the responses" keeps a definite antecedent from `:275` ("the responses `Q-55-refusalscope` settled") and is disambiguated by the very next sentence ("one predicate yields DIFFERENT answers on the validator and on the projections") | no orphan |
| `:298` (edit 1) | "THREE" deleted from "THREE responses" | the partitive "two of which" survives. Examined in full in the near-miss section; not raised | no orphan raised |
| `:298` (edit 2) | "three" -> "four" doc comments | numeral only. Both edits landed; neither was collapsed into the other, which the triage warned about at `-r4-triage.md:31` | no orphan |
| `:343` | "when there is one" appended | narrows the absolute to match `:274`; `:344`'s bullet makes no post-inc1 default claim and needed no matching change | no orphan |

## The `:111` qualification, checked as an exhaustiveness claim in the forward tense

TENSE APPLIED: FORWARD, scoped as the sentence scopes itself. `:111` opens "which is what 'done' means for THIS HALF", and it sits inside `## Defect B, cross-project contamination`, so the tense that matters is the tree inc1 and inc2 produce.

An "except for X" clause asserts that X is the only exception, so the fix converted an unqualified absolute into an exhaustiveness claim, which is the class this artifact has been bitten by. I enumerated every other change to a run made from the plan's own project root that inc1 or inc2 produces:

- Check 9 (`:316`): the normal correct case is BYTE-IDENTICAL after inc1. Not an exception.
- Accepted cost (i) (`:256`): the run is from inside `<root>/docs/plans`, not from the project root, so it is outside `:111`'s subject. Not an exception.
- Accepted cost (ii) (`:258`): a project-root run on a symlinked `docs/plans` layout goes from reading its own 37-record log to `exit=1 REFUSED`. THE ONE EXCEPTION, and the one the clause names.
- `:260`'s third behaviour change (an explicit `--metrics` pointed outside the root now exits 1): `:111`'s subject is qualified as "the normal invocation and the only one the scaffolded guidance documents", which an explicit foreign `--metrics` is not; and `:111`'s own second sentence REQUIRES that exit non-zero, so it is a consequence of the requirement, not an exception to it. Not an exception.

One exception exists in scope and the clause names it. The inc3 case (check 15 flips a project-root run on a non-instrumented project to non-zero) is outside `:111`'s stated half, which is the scoping the round 4 triage applied at `-r4-triage.md:152` when it declined to rest `R4B-4` on that falsifier. I reach the same reading independently: the sentence says "this half", and inc3 belongs to defect A.

## Cross-sidecar and sidecar-to-TOML consistency

The fix touched the plan TOML, so the sidecar-to-TOML direction was re-checked on every shared claim rather than only on the edited one.

- DOC-COMMENT COUNT, the edited claim. `docs/plans/agent-scaffold.plan.toml:1704` now reads "four doc comments that claim the JSON contract is exhaustive or enumerate the causes of an absent part". That agrees with `:198`, `:275`, `:298` and `:354`, and with the four-bullet list, and it agrees with the TOML's own predicate for the set (one exhaustiveness claim plus three cause enumerations). The regex sweep for a numeral against any doc-claim noun over all three sidecars and the TOML returns five counted sites, all reading four, and `three doc` returns zero everywhere.
- INCREMENT COUNT AND RISK CLASSES. Three `[[step.increment]]` entries, all `risk_class = "risky"` (TOML `:1309-1319`), matching `:296`, `:298`, `:300`. `test-tmpdir-repo-assumption-inc1` `low_risk` (TOML `:1330-1331`) matches that sidecar's `:60`. `status-resume-ignores-json-inc1` `low_risk` (TOML `:1343-1344`) matches that sidecar's `:105`.
- ORDERS. 94, 95, 96 in the TOML, matching every cross-reference in all three sidecars (`:236`, `:306`, `:377`, `:380`, `test-tmpdir-repo-assumption.md:7`, `status-resume-ignores-json.md:5`, `:115`).
- THE THREE `resume_state_absent_reason` VARIANTS. `:228-230` names them, `:377` calls them "the same three causes", `status-resume-ignores-json.md:97` names the same three by their kebab-case tokens. All agree, and `:226`'s new "Two of the three" does not disturb them because it counts what today's code distinguishes, not the variant set. This was the pair the triage said decided the `R4B-7` fix shape, and the fix preserved it.
- THE THREE TMPDIR TESTS. `:306` and `test-tmpdir-repo-assumption.md:13-15` name the identical three test paths; the TOML step title says "3 tests". Agree.
- `blocked_by = []` on all three steps, matching the prose reasoning at `test-tmpdir-repo-assumption.md:7` and `status-resume-ignores-json.md:5` and `:101`.
- THE (B)-FORK ORDERING. `status-resume-ignores-json.md:97` ("IT MUST NOT BE BUILT BEFORE `workflow-enforcement-tier` INC2 LANDS ... If (B) is ever chosen, it REUSES that enum") against primary `:377` ("If that step ever takes the other fork, it REUSES the `resume_state_absent_reason` vocabulary specified here rather than minting a second one for the same three causes"). Agree, including on the direction of the dependency.
- Neither sibling sidecar contains any of the strings the fix edited. `grep` for `doc comment` counts, `grounds`, `responses` and `only part of the mechanism` over both returns zero except `status-resume-ignores-json.md:111`'s bare "(doc comments, `README.md`, `CHANGELOG.md`)", which carries no count.

## Acceptance-check numbering, re-verified because a deletion can break a reference

The list runs 1 to 20 with 14b to 14h inserted (27 items, `grep -noE '^[0-9]+[a-h]?\.'`). Every internal reference resolves: `:274` cites check 4; `:304` cites 4, 11, 14b, 14e, 15; `:312` cites 14b; `:324` cites 14b; `:325` cites 14b and 14c; `:330` cites 10; `:334` cites 15. No dangling reference, and the fix touched no check line.

## Four candidates examined and NOT raised, with the reasoning

These are recorded because a clean result merges the artifact, and the record should show what was looked at and rejected rather than only what was not found.

### 1. `:298`'s surviving partitive "two of which must NOT fail". THE CLOSEST CALL.

The fix deleted "THREE" from "one predicate now drives THREE responses, two of which must NOT fail", leaving `workflow-enforcement-tier.md:298`:

> `Q-55-refusalscope` ADDS A FACTOR RATHER THAN LEAVING THE CLASS UNCHANGED: one predicate now drives responses, two of which must NOT fail, so the failure this increment can ship is not only "refuses the wrong thing" but "refuses on a surface that must never refuse" or "omits on a surface that should have refused"

Read against `workflow-enforcement-tier.md:284`:

> The predicate yields two responses (refuse, omit), and the omit has two renderings (human text, JSON).

Under the kinds sense that `:168` and `:284` now uniquely count, the set has two members and one of them is the refusal, which `:172` requires to exit non-zero, so "two of which must NOT fail" would range over both and be false of the refusal.

WHY I DO NOT RAISE IT. Three reasons, and I weighed all three rather than taking the first.

FIRST, "two of which" attaches to "responses" and is a partitive, not a count of the set, so no numeral in the document disagrees with any other numeral. The defect class this loop has been paying for is two numerals for one set; that class is now empty.

SECOND, the phrase is PRE-EXISTING and its set was never pinned by any enumeration even before the fix. Round 4's own analysis (`-r4-reviewer-consistency.md`, `R4B-6`) is that the deleted "THREE" and `:180`'s deleted "three" named DIFFERENT triples, neither of which the document enumerated as "the responses". So the deletion removed a false numeral and left a partitive that was already unpinned; it did not orphan a referent that previously resolved.

THIRD, the referent is recoverable in the same sentence, which is the reading the round 4 triage gave at `-r4-triage.md:227` and which I tested rather than accepted: the clause immediately following names the validator (which may refuse) and the projections (which must not), so "two of which must NOT fail" reads as the two projection surfaces. Any fix would have to author replacement words, and `:363` records five retrospective and one prospective confirmation on this project that an authoring fix pass manufactures the next round's finding while a deletion pass does not. Raising a `low` on this would trade a recoverable reading for a new authored clause in the exact document where that trade has repeatedly gone badly.

### 2. `:205`'s "`active_loop` is `None` ONLY when there are no steps or when every step is terminal".

`workflow-enforcement-tier.md:205`:

> `active_loop` is `None` ONLY when there are no steps or when every step is terminal, since `is_pending` (`:415-417`) and `is_terminal` (`:421-426`) partition the seven `StepPhase` variants (`:388-396`) exhaustively and every non-terminal phase reaches a `Some` arm.

Against `workflow-enforcement-tier.md:224`:

> `metrics-not-this-project`, the NEW case: the round log resolved for this plan is not the plan's own, so no loop state can be derived from it.

TENSE, AND THE TWO TENSES DIFFER, WHICH IS WHY I EXAMINED IT. Under the forward tense the increments produce, inc2 adds a third circumstance in which `active_loop` is `None`, so the absolute would be false. Under the present tense it is true, and I verified the seven `StepPhase` variants in the tree.

WHY I DO NOT RAISE IT. The sentence chooses the present tense itself, and does so explicitly: its paragraph opens "A FIFTH ITEM FOUND BY THE SWEEP IS PRE-EXISTING AND IS NOT A CONSEQUENCE OF THIS CHANGE", and the "since" clause derives the claim wholly from today's `select_active_loop`. It is a DIAGNOSIS of why the existing doc comment at `src/next.rs:108-109` misdescribes today's code, not a requirement on the finished increment. That is the distinction the round 4 triage drew between `R4B-7` (a present-tense claim that was false in the present tense, so a finding) and `R4B-4` (a requirement on finished work, so evaluated forward). This sentence is present-tense and true. What remains is that the document never states what the reconciled comment should say after inc2, which is an under-specification of the same size as the accepted `INC2-7`, not a contradiction. The claim is also untouched by the fix and was inspected by round 4 (`-r4-reviewer-consistency.md`, consistent pair 13).

### 3. `:170`'s "all four surfaces" beside the quoted option "Wide: refusal on all three".

`workflow-enforcement-tier.md:170` says the mechanism text "admitted two readings: the refusal on the validator only, or the refusal on all four surfaces", and in the same sentence quotes the receipt's option list including "Wide: refusal on all three". `docs/plans/agent-scaffold.plan.toml` uses "over refusal on all three" for the same option.

WHY I DO NOT RAISE IT. I read the receipt itself:

```
$ grep -o '"q_id":"Q-55-refusalscope"[^}]*' docs/metrics/workflow.jsonl
"q_id":"Q-55-refusalscope","options":["Omit the unsafe part, exit 0","Narrow: refusal on validate only","Wide: refusal on all three"],...
```

The "three" is a verbatim reproduction of the decision receipt, which the document is obliged to quote accurately, and the "four" counts the four items the same sentence has just quoted from `Q-55-mechanism` ("covering `validate`, `next`, `status` and the ledger path"). Different referents, each self-disclosing in its own clause: three commands, four covered artifacts. Not a contradiction, pre-existing, and untouched by the fix.

### 4. `status-resume-ignores-json.md:125`'s "the three cases they cover" beside `:92`'s five attributes.

`:92` states five constraint attributes at five verified line numbers; `:125` says "Five constraint attributes already exist and the three cases they cover are the ones previously found". The three are the silently-ignored-flag cases the same sidecar enumerates at `:82` (`ledger_fragment`, `--workflow-spec`, `render --strict`); the other two attributes (`:396`, `:557`) are a different relation. The sentence is loose about "they", but no other numeral in the fold states a different number of cases, so there is no pair to contradict. Pre-existing, untouched by the fix, and in the sibling sidecar rather than the edited one.

## Accepted residuals confirmed present and not reopened

`INC2-7` (no precedence rule for an over-determined `no_active_loop_reason`) is still open: `:232`'s precedence rule covers the two PATH fields only, and `:234`'s correlation rule qualifies `no_active_loop_reason` with "WHEN the loop's absence is metrics-derived rather than step-derived" without saying which wins when both apply. `F-5` (the dangling `validation-constraints` reference) is still present at `:152`, `:194`, `:270` and `:371`. Both confirmed, neither raised. I raise no objection to the enforcement tier, the one-step three-increment shape, anchor-plus-refusal, the conventionless fallback, omit-and-exit-0, the serialised reason, either accepted cost, or the nearest-wins judgement, and I did not raise the plan TOML's unprojected step `title`.

## Method, in the order I ran it

1. Read the fix diff first only to know which lines to treat as HIGHEST RISK, then set it aside and re-derived every count from the thing counted rather than from the diff.
2. Read the primary sidecar in full, both siblings in full, and the `[[step]]` and `[[question]]` entries in the plan TOML in full.
3. Swept every spelled numeral in the primary sidecar with a regex rather than with literals, then every definite-article numeral (`the two/three/four/five/six/seven ...`) across all three sidecars, and re-derived each hit against its list, its code artifact or its twin sites.
4. Ran the round 4 fix-scope greps in their exact form to confirm every fixed site converged and to find twin spellings the fix's own scope could not reach: zero residue on all seven.
5. Took each of round 4's twenty-two CONSISTENT pairs and re-tested the ones the fix touched, on the reasoning that a pair consistent before an edit is where a new inconsistency lands.
6. Checked every one of the ten edited lines for an orphaned pronoun, demonstrative, partitive or back-reference, and separately checked the acceptance-check numbering for a broken cross-reference.
7. Opened the code only to settle a count or confirm a negative (`#[serde(skip)]`, `skip_serializing_if`, the `StepPhase` variants, the `pack/AGENTS.md` mentions, the five clap constraint attributes, the two `run_resume` branches) and the round log only to count the decision receipts.
8. Ran `render --check` and `validate` to confirm the human projection carries the fixed text and the structured source is well-formed.
9. Applied the forward tense by default, and said which tense I applied wherever the two differ (`:111`, `:205`).
