# `workflow-enforcement-tier` plan review, round 4, reviewer: internal consistency

Artifact: the `workflow-enforcement-tier` fold at commit `e34c2c9` (`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, `docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md`, `docs/plans/agent-scaffold.steps/status-resume-ignores-json.md`, and the `[[step]]` / `[[question]]` entries in `docs/plans/agent-scaffold.plan.toml`).

Lens: the document read against itself. Every finding below is a pair of claims inside the fold where one falsifies the other. No finding here rests on a claim being wrong about the code alone; where I used the code it was to decide which side of a contradiction is true, and those checks are recorded in the enumeration.

## Verdict

SEVEN VALID FINDINGS, all `low`. The fold is sound on its decisions, its mechanism, its increment division and its acceptance checks; nothing here re-litigates a decided item and nothing here is a mechanism defect. What it carries is seven numeral-or-absolute-claim inconsistencies, five of which admit a one-word or one-clause fix and two of which (R4B-3, R4B-4) are absolute claims that need a qualifier or a deletion.

TWO OF THE SEVEN ARE RESIDUE OF EARLIER ROUNDS' OWN FIXES, which is this artifact's recorded failure shape rather than new damage. R4B-1 is round 3's `INC2-6` numeral fix applied at two of five sites (the round 3 reviewer's own sweep grepped two literal strings and could not reach the other three spellings). R4B-3 is the third site of the exclusivity claim that round 1 fixed at `:272` (`EX-5`) and round 3 fixed at `:290` (`R3B-1`), in a third wording that neither grep could match.

I did NOT raise `INC2-7` or `F-5`, and I raise no objection to the enforcement tier, the one-step/three-increment shape, anchor-plus-refusal, the conventionless fallback, omit-and-exit-0, the serialised reason, either accepted cost, or the nearest-wins judgement. R4B-4 is about the wording of an end-property sentence and is emphatically NOT a request to change accepted cost (ii).

| id | severity | one-line summary |
| --- | --- | --- |
| `R4B-1` | low | The JSON-reason change falsifies FOUR doc comments per `:198` and `:354`, but THREE per `:275`, `:298` and the plan TOML at `:1704`. |
| `R4B-2` | low | Candidate (d) is "rejected on five measured grounds" at `:242`, and "rejected on the four grounds above, not on its cost" in the fifth bullet at `:248`. |
| `R4B-3` | low | `:280` says the predicate is the only part of the mechanism that changes what a currently-succeeding invocation reports; `:274`, `:296` and checks 3 to 7 say inc1 does too. Third site of round 1's `EX-5`. |
| `R4B-4` | low | `:111` requires a run from the plan's own project root to be unchanged; accepted cost (ii) at `:258` and check 19 make exactly such a run exit 1. |
| `R4B-5` | low | `:343` says the ledger default after inc1 is beside the plan source, full stop; `:274` keeps `docs/plans/<task>.ledger.md` when there is no source, and that case is reachable. |
| `R4B-6` | low | The predicate has "two responses" at `:168` and `:284` and "three responses" at `:180`, `:282` and `:298`, and the two "three"s enumerate different triples. |
| `R4B-7` | low | `:226` says the three `resume_state` causes "are already distinguished IN THE CODE", one sentence after saying the third arrives with inc2; the cited lines hold two branches. |

---

## `R4B-1`. `low`. The doc-comment count is FOUR in two places and THREE in three others, one of which is the plan TOML

SIDE A, FOUR, stated twice and enumerated once. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:198`:

> THIS IS A DOCUMENTED-CONTRACT CHANGE AND IS TREATED AS ONE. Four doc claims are falsified or made incomplete by it, found by sweeping `src/next.rs` and the `status` projection for exhaustiveness claims rather than by patching the one already known.

The list it heads has exactly four bullets, at `:200` (`src/next.rs:114-115`), `:201` (`src/next.rs:95-97`), `:202` (`src/main.rs:561-564`) and `:203` (`src/next.rs:111-112`). The documentation-impact list repeats the four at `workflow-enforcement-tier.md:354`:

> THE FOUR DOC COMMENTS `Q-55-jsonreason` FALSIFIES OR LEAVES INCOMPLETE, all four in the same change because two of them are tied by a cross-reference: `src/next.rs:114-115` [...], `src/next.rs:95-97` [...], `src/main.rs:561-564` [...], and `src/next.rs:111-112` [...]

SIDE B, THREE, in three places. `workflow-enforcement-tier.md:275`, the inc2 increment description:

> Plus the serialised reasons `Q-55-jsonreason` settled, so the omission explains itself on `--json` as well as in the human text, with the three falsified doc comments corrected in the same change.

`workflow-enforcement-tier.md:298`, the inc2 risk argument:

> `Q-55-jsonreason` adds a second: the increment now also changes a DOCUMENTED JSON CONTRACT on two commands, falsifying three doc comments and breaking a byte-compare golden [...]

`docs/plans/agent-scaffold.plan.toml:1704`, the `Q-55` question text:

> [...] three doc comments that claim the JSON contract is exhaustive or enumerate the causes of an absent part are falsified and corrected in the same change, `README.md` and `CHANGELOG.md` carry the addition [...]

WHICH SIDE IS WRONG: side B. The four-item list is the correct one and was established by round 2's `INC2-6` (the missing `src/next.rs:111-112` bullet) and applied in round 3; I re-read all four cited comments and each is a real claim that the change falsifies or leaves short by one (`src/next.rs:95-97` "a missing plan or log", `src/main.rs:561-564` "a missing plan or metrics file", `src/next.rs:111-112` "the ledger is absent or carries no such section", `src/next.rs:114-115` "the JSON contract is exactly the fields above"). The TOML's own description of the set ("claim the JSON contract is exhaustive OR enumerate the causes of an absent part") covers one plus three, so "three" is wrong even against its own predicate. Note also that "falsified" is not a narrower reading that rescues side B: strictly falsified is ONE (`:200` "BECOMES FALSE"), the other three are incomplete (`:201` "BECOMES INCOMPLETE", `:202` "HAS THE SAME DEFECT", `:203` "IS SHORT BY ONE"), so "three" matches neither sense.

WHY THE ROUND 3 FIX DID NOT REACH THESE SITES, which is the finding's real value. The round 3 residue reviewer recorded its own sweep at `docs/plans/agent-scaffold.reviews/workflow-enforcement-tier-planreview-r3-reviewer-residue.md:133`: "`Three doc claims` / `THE THREE DOC COMMENTS`: 0. `Four doc claims` / `THE FOUR DOC COMMENTS`: 1 each." Both greps are literal-string matches on the two sentences the fix touched. The three sites above spell the same numeral as "the three falsified doc comments", "falsifying three doc comments" and "three doc comments that claim", none of which either grep can match. This is the same scoping blind spot the round 3 triage named as the `RES-1` shape.

MINIMAL FIX, number-edit class, three sites plus a re-render: "three" -> "four" at `workflow-enforcement-tier.md:275` and `:298`, and at `docs/plans/agent-scaffold.plan.toml:1704`. `docs/plans/agent-scaffold.md` carries the same text at `:1670`, `:1693` and `:168` and is a generated projection, so re-render rather than hand-edit.

EVIDENCE COMMANDS:

```
$ grep -rn "three doc comment\|three falsified doc\|falsifying three doc" docs/plans/agent-scaffold.steps/ docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md | wc -l
6
```

(three source sites plus their three projections in `docs/plans/agent-scaffold.md`.)

---

## `R4B-2`. `low`. Candidate (d) is rejected on five grounds in the section opener and on four in the section's own fifth bullet

SIDE A, `workflow-enforcement-tier.md:242`:

> The first planner pass named a `[meta]`-declared log path as "where the cleaner long-term architecture points". Explorer B BUILT it, including the sibling and ledger extensions, and argued against it; explorer C reached the same vulnerability independently. It is rejected on five measured grounds, recorded here so the direction is closed rather than rediscovered.

SIDE B, the closing sentence of the fifth bullet under that opener, `workflow-enforcement-tier.md:248`:

> This bullet is relevant only as the record of why the first pass's cost accounting should not be reused; (d) is rejected on the four grounds above, not on its cost.

The list has five bullets (`:244`, `:245`, `:246`, `:247`, `:248`), and the fifth one disclaims being a ground for the rejection. So the count in the opener does not match the list it heads, on the list's own account of itself.

WHICH SIDE IS WRONG: the opener. The fifth bullet is precise and self-consistent (it is bullet 5 pointing at bullets 1 to 4, and it is headed "CORRECTION TO THE FIRST PASS'S COST LIST FOR (d)", which is a correction to the earlier pass rather than a reason to reject). The plan TOML's own summary of the rejection agrees with FOUR and states no count, `docs/plans/agent-scaffold.plan.toml:1700`: "its builder measured that it contributes nothing on shipping day, that a declared path reconstructs the same false pass in a worse form [...], that no validator can refuse it [...], and that it cannot cover the Markdown `--plan` substrate at all". That is four, matching bullets 1 to 4 exactly.

WHY IT MATTERS RATHER THAN BEING A SLIP: `:242`'s stated purpose is that the direction is "closed rather than rediscovered", so the paragraph is written for a later reader who wants to know what would have to be beaten to reopen (d). A reader who counts five grounds and finds that one of them is a cost correction the document itself says not to reuse has to work out which four are load-bearing.

MINIMAL FIX, number-edit class, one site plus a re-render: at `:242`, "five measured grounds" -> "four measured grounds". The fifth bullet already explains its own status and needs no new prose. `docs/plans/agent-scaffold.md:1637` carries the projection.

---

## `R4B-3`. `low`. `:280`'s exclusivity claim for the predicate is falsified by inc1's own description and by four of its acceptance checks. Third site of round 1's `EX-5`

SIDE A, `workflow-enforcement-tier.md:280`:

> WHY THE PREDICATE IS ITS OWN INCREMENT. It is the only part of the mechanism that changes what a currently-succeeding invocation REPORTS, whether by failing (the validator) or by withholding (the projections); it carries a known false positive (accepted cost (ii)); and it deliberately uses a DIFFERENT resolution from the default, so its review must check the lexical/canonical SPLIT rather than one rule.

"The mechanism" here is the one named at `:150` ("The mechanism, decided rather than chosen here"), the anchor PLUS the refusal, whose parts are inc1's derivation and inc2's predicate. So the claim is that inc1's half changes nothing a currently-succeeding invocation reports.

SIDE B, four falsifiers, all inside the same file.

`workflow-enforcement-tier.md:274`, the inc1 description, which is itself the text round 1's `EX-5` fix wrote:

> NO new REFUSAL mechanism: any new non-zero exit comes from the pre-existing W3 check finally running against the right project, which is check 4's whole point.

A new non-zero exit on a run that exits 0 today is precisely a currently-succeeding invocation whose report changes by failing.

`workflow-enforcement-tier.md:311`, check 4:

> 4. AFTER INC1, the false pass is dead: rerun the borrowed-slug demonstration (fixture step `complete` with slug `triager-runs-only-on-findings`) from the agent-scaffold root. Before the fix it exits 0 with `workflow invariants hold`. After, no green. Give the fixture a log of its OWN with no evidence for that slug and expect the correct RED instead of the absence of a green.

`workflow-enforcement-tier.md:310`, check 3, which is the withholding half on the validator, again after inc1 and not inc2:

> 3. AFTER INC1, defect B's original reproduction is dead: from the agent-scaffold root, `agent-scaffold validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --workflow` does NOT read agent-scaffold's own log and does NOT print `workflow invariants hold`.

`workflow-enforcement-tier.md:296`, inc1's own risk argument:

> It changes WHICH FILE the tool reads on every invocation of THREE commands that do not pass `--metrics`, plus which LEDGER two of them read, and the failure mode of a wrong anchor is not a crash but a confident wrong answer.

Checks 5, 6 and 7 (`:312`, `:313`, `:314`) are three more: each takes an invocation that exits 0 today with a particular output (`state: converged`, agent-scaffold's record count, this repository's `## RESUME STATE`) and requires a different output after inc1.

WHICH SIDE IS WRONG: side A. Both halves of `:280`'s parenthetical are wrong about inc1: after inc1 the validator both fails (check 4) and withholds the green (check 3), and the projections change what they report (checks 5 to 7). This is also the third site of one claim family. Round 1's `EX-5` struck it at `:272` ("NO new failure mode: every invocation that exited 0 before still exits 0"), round 3's `R3B-1` struck it at `:290` ("the tier policy goes LAST because it is the only increment that makes a previously-green run fail"), and both fixes were scoped by literal-string greps (`"still exits 0"` and `"only increment that makes a previously-green"`) that this third wording does not match. Confirmed at commit level: `git log -S "only part of the mechanism that changes what a currently-succeeding"` returns `8db3f83`, the refusal-scope fold, so the sentence predates both fixes and is not fix-induced.

TENSE: I applied the post-increment tense throughout, which is the tense that makes this a finding. Against today's tree the sentence has no truth value (neither increment has landed); against the tree inc1 produces, it is false.

MINIMAL FIX, deletion class, one site (`grep -c "only part of the mechanism"` returns 1 in the primary sidecar and 0 in the other two sidecars and the TOML): strike the first clause, leaving "WHY THE PREDICATE IS ITS OWN INCREMENT. It carries a known false positive (accepted cost (ii)); and it deliberately uses a DIFFERENT resolution from the default, so its review must check the lexical/canonical SPLIT rather than one rule." The paragraph's remaining two reasons plus the negative-correctness-property sentence carry the argument on their own. I recommend authoring no replacement clause: this project's calibration data on prose-authoring fixes is what round 1 and round 3 both cited, and the two positive replacements available here (a new-mechanism framing, a not-a-pre-existing-check framing) were both examined by the round 3 triage and one of them was found false on the facts.

---

## `R4B-4`. `low`. The defect B end property requires a run from the plan's own project root to be unchanged, and accepted cost (ii) makes exactly such a run exit 1

SIDE A, `workflow-enforcement-tier.md:111`:

> THE REQUIRED END PROPERTY, which is what "done" means for this half regardless of the mechanism: `validate --workflow` must never pair a plan source with a metrics log belonging to a different project and report success. Where the tool cannot establish that the two belong together, it must say so and exit non-zero rather than proceed. A run made from the plan's own project root, which is the normal invocation and the only one the scaffolded guidance documents, must be unchanged (Safe on existing projects).

SIDE B, `workflow-enforcement-tier.md:258`:

> (ii) A SYMLINKED `docs/plans` DIRECTORY BECOMES A FALSE POSITIVE ON THE PREDICATE. Where `<root>/docs/plans` is a symlink to `<root>/elsewhere`, the lexical default and the canonical guard disagree about which project the plan belongs to, and the guard wins: A measured this layout going from reading its 37-record log to `exit=1 REFUSED`. This is a genuine new failure for a layout that works today, and it is the main false-positive risk in the mechanism.

The refused run is a run made from the plan's own project root: it is the run that reads that project's own 37-record log today, which is what the anchored default resolves to from anywhere and what the CWD-relative default resolves to from the root. Check 19 (`:333`) pins it: "a layout where `<root>/docs/plans` is a SYMLINK to a sibling directory is REFUSED under `validate --workflow` after inc2, with the refusal message and a non-zero exit". Both sides sit inside the defect B half that `:111` scopes itself to ("what 'done' means for this half"), since the containment refusal is that half's second mechanism (`:164`).

The contradiction is also internal to `:111` itself: its second sentence ("where the tool cannot establish that the two belong together, it must say so and exit non-zero") and its third sentence ("a run made from the plan's own project root [...] must be unchanged") give opposite verdicts on the symlinked layout, and the document resolves that conflict 147 lines later in favour of the second ("the guard wins").

A SECOND FALSIFIER UNDER THE TENSE RULE, if `:111` is read as a property of the finished step rather than of the defect B half. `:300` says inc3 "is INTENDED to flip a currently-passing gate to failing for every non-instrumented project", and check 15 (`:329`) runs it from inside the fixture, that is, from the plan's own project root: "from inside the fixture, `agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow` exits NON-ZERO". Against today's tree `:111` is unobjectionable; against the tree the three increments produce it is false twice, and I flag the difference because the two tenses disagree here.

BOTH SIDES ARE DEFENSIBLE AND THE DEFECT IS THE ABSOLUTE FORM. I am NOT raising accepted cost (ii): it is decided, human-accepted, and `:254` correctly tells a reviewer not to raise it. What is defective is that the end property is written as an unqualified "must", so a reader who holds `:111` as the acceptance bar for the defect B half will read check 19 as a violation of the step's own stated requirement rather than as its accepted cost.

MINIMAL FIX, one clause at one site (`grep -rn "must be unchanged"` over the three sidecars and the TOML returns exactly this one hit): qualify the third sentence, for example "must be unchanged, except for the symlinked-`docs/plans` layout recorded as accepted cost (ii) below". Nothing else changes.

---

## `R4B-5`. `low`. The inc1 documentation-impact bullet states the new ledger default unconditionally; the inc1 description keeps the old default for the no-source case

SIDE A, `workflow-enforcement-tier.md:343`, the INC1 documentation-impact list:

> - `src/main.rs:461` (`StatusArgs::resume`), `:464-466` (`StatusArgs::ledger_fragment`) and `:482-484` (`NextArgs::ledger_fragment`), all of which say the default is `docs/plans/<task>.ledger.md`; after inc1 it is `<task>.ledger.md` BESIDE the plan source.

SIDE B, `workflow-enforcement-tier.md:274`, the inc1 description:

> With BOTH a `--source` and a `--plan`, the anchor follows the source-then-plan order `derive_task` already uses (`src/next.rs:997-999`); with NEITHER, the ledger keeps today's `docs/plans/<task>.ledger.md`, as the metrics rule keeps its CWD-relative path for the same case.

The same rule is stated for the metrics half at `:158`: "With neither a `--source` nor a `--plan` there is nothing to anchor to, so the historical CWD-relative path stands unchanged."

WHICH SIDE IS WRONG: side A, and the case it omits is reachable rather than theoretical. `derive_task` (`src/next.rs:993-1003`) falls back to the literal string `"task"` when neither `--source` nor `--plan` is given (`.map_or_else(|| "task".to_string(), task_from_filename)`), and `run_resume` (`src/main.rs:1152-1154`) then calls `default_ledger_path(&task)`, which builds `docs/plans/task.ledger.md` (`src/main.rs:1136-1138`). Neither `--source` nor `--plan` is required by `StatusArgs` or `NextArgs`, so `agent-scaffold status --resume` with no source is a valid invocation today and keeps its CWD-relative ledger path after inc1 by `:274`'s own rule.

WHY IT MATTERS: `:343` is the instruction for what three user-facing help strings must say after inc1. An implementer who follows it literally writes a help string that is false for the no-source invocation, in the same release whose whole subject is a tool that names the wrong file.

MINIMAL FIX, one clause at one site (`grep -rn "BESIDE the plan source"` over the three sidecars and the TOML returns exactly this one hit): "after inc1 it is `<task>.ledger.md` BESIDE the plan source, or today's `docs/plans/<task>.ledger.md` when there is no source to anchor to".

---

## `R4B-6`. `low`. The predicate drives "two responses" in two places and "three responses" in three others, and the two three-counts enumerate different triples

SIDE A, TWO. The section heading itself, `workflow-enforcement-tier.md:168`:

> ## One predicate, two responses: the validator refuses, the projections omit (`Q-55-refusalscope`)

and the argument that places the serialised reasons in inc2, `workflow-enforcement-tier.md:284`:

> That framing is wrong and the right one is stronger: the JSON reason is not a third response, it is the SECOND RENDERING of one response. The predicate yields two responses (refuse, omit), and the omit has two renderings (human text, JSON).

SIDE B, THREE, in three places. `workflow-enforcement-tier.md:180`:

> The trigger in all three cases is the SAME containment predicate the validator's refusal uses (the canonically-derived plan root, and whether the resolved artifact lives under it). One predicate, three consumers, three responses.

`workflow-enforcement-tier.md:282`, two paragraphs above side A's second quote:

> SECOND, the three responses are only reviewable AGAINST EACH OTHER.

`workflow-enforcement-tier.md:298`:

> `Q-55-refusalscope` ADDS A FACTOR RATHER THAN LEAVING THE CLASS UNCHANGED: one predicate now drives THREE responses, two of which must NOT fail [...]

THE TWO THREE-COUNTS ARE NOT THE SAME THREE. At `:180` the antecedent of "all three cases" is the bullet list immediately below it, which is `status` (`:182`), `status --resume` (`:183`) and `next` (`:184`), so the three consumers there EXCLUDE the validator, and all three of them give the same response kind, an omission. At `:298` the three must include the validator, because "two of which must NOT fail" identifies the other two as the projections. So the fold contains three different answers to "how many responses does the predicate drive", and two different membership lists for the same numeral.

WHICH SIDE IS WRONG: the numeral at `:180`, `:282` and `:298`, on the ground that the section heading and the inc2-placement argument both need the two-kinds sense and are the load-bearing uses. `:284`'s argument ("the JSON reason is not a third response, it is the SECOND RENDERING of one response") only parses if the predicate's responses are the two kinds refuse and omit; under `:180`'s or `:298`'s sense the JSON reason would be a fourth thing, and the framing `:284` rebuts does not arise. The argument's conclusion survives either sense (a rendering is not a response either way), so this is a term-and-numeral defect rather than a broken argument, which is why it is `low`.

MINIMAL FIX, word-substitution class, three sites: at `:180` "One predicate, three consumers, three responses." -> "One predicate, three projection consumers."; at `:282` "the three responses" -> "the three consumers' answers"; at `:298` "one predicate now drives THREE responses" -> "one predicate now drives THREE consumers". No new prose, and the heading at `:168` and the argument at `:284` stay exactly as they are.

---

## `R4B-7`. `low`. The three `resume_state` causes are said to be "already distinguished IN THE CODE" one sentence after the third is said to arrive with inc2

BOTH SIDES ARE THE SAME PARAGRAPH, `workflow-enforcement-tier.md:226`:

> `resume_state_absent_reason`, on `NextProjection`, beside `resume_state`. `Some` exactly when `resume_state` is `None`. This one is included rather than skipped because without it the same defect lands on a third field: after inc2, `next --json` can omit the resume block for a THIRD reason and report the same bare `null` it reports for the other two. The three causes are already distinguished IN THE CODE at `src/main.rs:1208-1212`, where `ledger_path.exists()` being false and `extract_resume_state` returning `None` are separate branches that both collapse to `None`, so naming them costs nothing beyond the naming.

Sentence three says the third cause arrives with inc2 ("after inc2, `next --json` can omit the resume block for a THIRD reason"). Sentence four says all three are "already distinguished IN THE CODE" and cites lines that hold exactly two branches, which the sentence itself then enumerates as two. The third variant, `ledger-not-this-project` (`:230`, "an explicit `--ledger-fragment` resolves outside the plan's project root"), depends on the containment predicate, which inc2 introduces and which does not exist at the cited lines.

CODE CHECK, used only to decide which side is true. `src/main.rs:1207-1212` reads:

```rust
let ledger_path = args.ledger_fragment.clone().unwrap_or_else(|| default_ledger_path(&task));
let resume_state = if ledger_path.exists() {
	next::extract_resume_state(&fs::read_to_string(&ledger_path)?)
} else {
	None
};
```

Two branches, two distinguishable causes. The citation itself is correct (round 2's inc2 reviewer verified it), and the argument the sentence makes ("naming them costs nothing beyond the naming") is right for the two pre-existing causes and harmless for the third, which inc2 computes anyway. Only the numeral is wrong.

TENSE: this one is a claim about TODAY's tree ("already", plus a citation to today's line numbers), and it is false today. Against the tree inc2 produces, all three causes are distinguished, so the sentence would be true with "already" removed; that is the ambiguity, and it is why the fix has two equally small forms.

MINIMAL FIX, number-edit class, one site: "The three causes are already distinguished" -> "Two of the three causes are already distinguished". (Dropping "already" instead would also resolve it but reads as a claim about the post-inc2 tree in a sentence that cites pre-inc2 line numbers.)

---

# CLAIM INVENTORY

The load-bearing claims I extracted and then tested against each other. Each row is the claim, its site, and the verdict of the cross-check.

## What each increment does

| claim | site | cross-check |
| --- | --- | --- |
| inc1 = the resolution rule and all four of its call sites (`--metrics` to `Option<PathBuf>` on three arg structs, lexical derivation, `validate`/`status`/`next`, plus `default_ledger_path` and both its call sites) | `:274` | consistent with the doc-impact list `:342-346` and with checks 3 to 10; `R4B-5` is the one clause of `:343` that does not match |
| inc1 adds NO new refusal mechanism; any new non-zero exit is the pre-existing W3 check | `:274` | consistent with `:311` check 4; FALSIFIES `:280` (`R4B-3`) |
| inc1 keeps today's CWD-relative metrics path and today's `docs/plans/<task>.ledger.md` when there is neither a `--source` nor a `--plan` | `:274`, `:158` | consistent with `:317` check 10; FALSIFIED BY `:343` (`R4B-5`) |
| inc1's conventionless fallback is the source's own directory | `:160` | consistent with `:315` check 8 and with the TOML at `:1706` |
| inc2 = the canonical root derivation, the predicate, its three consumers' responses, the serialised reasons, and their documentation | `:275` | consistent with `:168-238` and with checks 11 to 14h, except the doc-comment numeral (`R4B-1`) |
| inc2's refusal is the validator's alone; `status` and `next` never exit non-zero | `:172`, `:321`, `:374` | consistent across all three sites and with the TOML at `:1702` |
| inc3 = the `_` catch-all becomes a problem, plain `validate` untouched, plus the `SE-3` documentation half and the two regenerated deployed copies | `:276`, `:360-366` | consistent with `:55`, `:148`, checks 15, 16, 20 |
| the red case per increment: check 4 (inc1), checks 11 / 14b / 14e (inc2), check 15 (inc3) | `:304` | all four checks exist and each states a pre-fix red |

## What the step does not do

`:370-381`'s ten scope bullets were each matched against the body: no round-logging core (`:57`), no project identity (`:266`), no `[meta].metrics` or `[meta].ledger` (`:240-250`), no containment refusal on the projections (`:172`), no exit-code policy on the projections (`:374`, `:321`), no bare-absence `--json` (`:192-234`), no reason beside `status`'s `plan` and no malformed-log variant (`:238`), no `status --resume` JSON surface (`:236`), neither accepted cost fixed (`:252-258`), no `src/workflow.rs` change (`:264`), no TMPDIR fix (`:306`), nearest-wins left unevidenced (`:162`). All twelve claims have a matching body passage and none contradicts one. The `status --resume` JSON bullet also matches the sibling step (`status-resume-ignores-json.md:97`, `:124`).

## Exclusivity and absolute claims tested

| claim | site | verdict |
| --- | --- | --- |
| the predicate is the ONLY part of the mechanism that changes what a currently-succeeding invocation reports | `:280` | FALSE, `R4B-3` |
| a run from the plan's own project root MUST BE UNCHANGED | `:111` | FALSE after inc2 and after inc3, `R4B-4` |
| candidate (d) is rejected on FIVE grounds | `:242` | FALSE by the fifth bullet, `R4B-2` |
| inc2 is the only place where two different resolutions run against each other by design | `:298` | consistent: the canonical guard is inc2's alone (`:164`, `:166`), inc1 is lexical only (`:158`) |
| the refusal (exit non-zero) remains the validator's alone | `:172` | consistent with `:275`, `:321`, `:374`, TOML `:1702` |
| `#[serde(skip)]` appears exactly ONCE in the whole of `src/` | `:207` | TRUE, `grep -c` over every file in `src/` and `src/plan/` returns 1, in `src/next.rs` |
| no other doc comment in `src/next.rs` or `src/main.rs` claims its JSON output is exhaustive | `:207` | TRUE, `grep -n "///.*JSON"` returns 9 hits and only `src/next.rs:114` makes an exhaustiveness claim |
| the only two places `pack/AGENTS.md` mentions the round log outside the instrumentation section are `:61` and `:63` | `:146` | TRUE, `grep -c` returns 2 in the whole file, both in "When instrumentation is on" clauses |
| `status --json` has NO golden and no test on its serialisation | `:209` | consistent with `:284` and `:325`, and with the acceptance check carrying that half |
| `status --resume` has NO JSON surface and none is owed | `:236`, `:327` | consistent with `status-resume-ignores-json.md:3`, `:22`, `:124` |
| the ledger needs no root derivation at all, the source's own directory is the whole rule | `:136` | consistent with `:274`, `:278`, `:343`'s first half |
| `next` MUST NOT emit an action or a `summary` line from a log it cannot vouch for | `:186` | consistent with `:322` check 14b's field-by-field list and with TOML `:1702` |
| UNSAFE IS NOT ABSENT | `:188` | consistent with `:232` precedence, `:324` check 14d, `:326`'s fourth run, TOML `:1702` |

## Counts and enumerations tested

| stated count | site | list | verdict |
| --- | --- | --- | --- |
| four defects, one family | `:3` | A, B, C, D at `:5-8` | consistent |
| SIX decision receipts | `:10` | six bullets `:12-17`; six `q_id`s in the TOML question | consistent |
| three exploration records, 1514 lines | `:19` | 521 + 483 + 510 = 1514 | consistent |
| three of the first pass's factual claims superseded | `:21` | exactly three "CORRECTION TO THE FIRST PASS" headers at `:117`, `:136`, `:248`, and `:117` says "the most consequential of the three" | consistent (the `Q-55`-wording correction at `:51` and the second pass's self-correction at `:175` have different subjects and are correctly uncounted) |
| four-arm `match` at `src/main.rs:958-1004` | `:53`, `:164` | the code has exactly four arms, catch-all at 999-1003 | consistent |
| seven `StepPhase` variants | `:205` | `src/next.rs:388-396` has 7 | consistent |
| four doc claims falsified or incomplete | `:198`, `:354` | four bullets | consistent with each other, CONTRADICTED at three other sites (`R4B-1`) |
| five measured grounds | `:242` | five bullets, the fifth disclaiming ground status | CONTRADICTED (`R4B-2`) |
| two responses / three responses | `:168`, `:284` vs `:180`, `:282`, `:298` | two senses, two membership lists | CONTRADICTED (`R4B-6`) |
| three `resume_state` causes already distinguished in the code | `:226` | two branches at `src/main.rs:1208-1212` | CONTRADICTED (`R4B-7`) |
| three inc2 red cases | `:304` | checks 11, 14b, 14e | consistent |
| two `metrics_absent_reason` variants | `:217-218` | `log-absent`, `log-not-this-project`; check 14f exercises both plus the precedence case | consistent |
| three `no_active_loop_reason` variants, the third string collapsed | `:220-224` | two existing plus one new; `:220`'s "collapses it to two answers" is about the pre-existing strings | consistent |
| three `resume_state_absent_reason` variants | `:228-230` | check 14g exercises all three plus precedence; `status-resume-ignores-json.md:97` names the same three | consistent |
| two accepted costs | `:252-258` | (i) and (ii); the "THIRD BEHAVIOUR CHANGE" at `:260` is explicitly not a cost | consistent with TOML `:1706` |
| three `--metrics` help strings | `:274`, `:342` | `src/main.rs:429-431`, `:455-457`, `:479-481` | consistent, and all three resolve |
| three suite tests that need a non-repo `TMPDIR` | `:306` | three named tests, the same three at `test-tmpdir-repo-assumption.md:13-15` | consistent |
| 235 records | `:72`, `:75`, `:93`, `:109`, `:122`, `:264` | one value throughout, with the 233/235 drift explained at `:72` | consistent |

# ENUMERATION: what I swept, including the negatives

## Pairs checked and found CONSISTENT

These are the cross-checks that came back clean. They are recorded so a later round knows where not to look again.

1. Defect A's remedy (`:55`) against inc3's description (`:276`), check 15 (`:329`), check 16 (`:330`) and the TOML decision (`:1684` onward). Plain `validate` unaffected is stated identically in all four.
2. Defect A's "present but EMPTY log is not part of this" (`:61`) against the control (`:95-107`) and check 17 (`:331`). Same case, same expected red, same W3 message.
3. Defect B's mechanism (`:109`) against the code: `#[arg(long, default_value = ...)]` at `src/main.rs:429-431`, `metrics_path.exists()` at `:823`, `fs::read_to_string` at `:824`, the stderr note at `:845`. All resolve.
4. The two stderr announcements claim (`:51`) against `src/main.rs:845` and `:1001-1003`. Both are `eprintln!`, both in the cited places, and the only stdout line is the ok summary.
5. The sibling-arm precedent cited at `:59` (`src/main.rs:992-998`) and at `:300` (`:995-998`). Different ranges for the same arm, but `:59` quotes the arm's comment (lines 992-994) and `:300` cites the arm body alone, so both are correct for what they claim. NOT a finding.
6. Defect C's `next` reproduction (`:119-130`) against check 5 (`:312`), check 14b (`:322`) and check 14d (`:324`). The `in-progress` fixture status is required consistently in all of them (this was round 2's `INC2-4`, and the fix holds).
7. Defect C's `status --resume` leak (`:132`) against check 7 (`:314`) and against the inc1 ledger rule (`:274`). Same reproduction, same fixture rename, same expected output.
8. Defect D (`:140-148`) against inc3's documentation impact (`:363-364`) and check 20 (`:334`). The pack sentence, its two deployed copies, the `--instrument` render command and the "do not run `just scaffold-self`" instruction all agree; `justfile:46-48` is the render plus `nix fmt`, as claimed.
9. `Q-55-refusalscope`'s three per-surface behaviours (`:182-184`) against inc2's description (`:275`), checks 14b, 14c, 14g, and the TOML at `:1702`. The `status --resume` reason-in-place rule (round 2's `INC2-5`) is stated identically at `:183`, `:184`, `:275` and `:323`.
10. The JSON vocabulary (`:213-234`) against checks 14e, 14f, 14g, 14h. Every specified variant is exercised, the precedence rule has its own run (round 2's `INC2-3`), and the correlation rule's shared `not-this-project` token matches the variant names.
11. `metrics_absent_reason` on BOTH projections (`:215`) against check 14e's two commands (`:325`) and against `:209`'s note that the `status` half is unguarded by the suite. Consistent, and the acceptance check covers exactly the unguarded half it names.
12. The caller-computes-the-reasons rule (`:215`, round 2's `INC2-10` fix) against the `NextInputs` claim and against check 14h's golden expectation. No conflict.
13. The `active_loop` pre-existing mismatch (`:205`) against the documentation-impact bullet (`:354`) and against `:220`'s collapse of the third string. All three say reconcile the comment, do not add a variant.
14. The increment-division rule (`:272`) against the four call sites paragraph (`:278`), the omit placement (`:282`), the serialised-reason placement (`:284`) and the documentation placement (`:288`). Each argues from the same rule and none contradicts another.
15. The ordering argument (`:290`) against the risk section (`:296-300`) and the cost-of-placement paragraph (`:286`). After round 3's deletion the ordering claim is the escape-hatch argument alone, which is true and is not contradicted anywhere.
16. Risk classes: three `risky` increments in the sidecar (`:296`, `:298`, `:300`) against `docs/plans/agent-scaffold.plan.toml:1309-1319`; `test-tmpdir-repo-assumption-inc1` `low_risk` at its `:60` against TOML `:1330-1331`; `status-resume-ignores-json-inc1` `low_risk` at its `:105` against TOML `:1343-1344`. All match.
17. Cross-referenced backlog steps and their orders: `sidecar-ref-empty-string` 63, `sidecar-ref-symlink` 64 (`deferred`), `reviewer-reproducible-evidence` 88 (`Q-66`), `checks-runner-worktree-name-collision` 93 (`risky`), `workflow-enforcement-tier` 94, `test-tmpdir-repo-assumption` 95, `status-resume-ignores-json` 96. Every order and every risk class cited in the three sidecars matches the TOML.
18. `blocked_by = []` on all three steps against `test-tmpdir-repo-assumption.md:7` and `status-resume-ignores-json.md:5` and `:101`. The empty-`blocked_by` reasoning is stated in prose in the sidecars and matches the TOML.
19. `status-resume-ignores-json.md` against the primary on every shared claim: the resume slice returning before serialisation (`:3`, `:22` vs primary `:236`), the fork (B) ordering constraint and the vocabulary reuse (`:97`, `:124` vs primary `:377`), the `--ledger-fragment` `requires` precedent (`:74` vs primary `:236`), and the explicit refusal to borrow this step's urgency (`:66-70`). No contradiction, including on the `src/main.rs` citations (`:1062-1069`, `:1152-1165`, `:464-466`, `:556-558`), all of which resolve.
20. `test-tmpdir-repo-assumption.md` against the primary: the three test names and their file:line citations (`:13-15`), the "not a dependency of `workflow-enforcement-tier`" claim (`:7`) against the primary's `:306` and `:380`, and the scratch-`TMPDIR` discipline. No contradiction.
21. The plan TOML's `Q-55` question text against the primary on every decision: the tier decision, the "silently passes" correction, the two scope additions, the mechanism, the refusal scope, the machine surface, the conventionless case and the two accepted costs. The only divergence found is the doc-comment numeral (`R4B-1`).
22. Every `file:line` citation I relied on for a finding, opened and confirmed: `src/main.rs:429-431`, `:438-440`, `:455-457`, `:461`, `:464-466`, `:479-481`, `:482-484`, `:561-564`, `:823-847`, `:845`, `:958-1004`, `:995-998`, `:999-1003`, `:1104`, `:1133-1138`, `:1147-1151`, `:1150-1151`, `:1200-1205`, `:1208-1212`; `src/next.rs:95-97`, `:108-109`, `:111-112`, `:114-115`, `:116`, `:388-396`, `:993-1003`; `pack/AGENTS.md:61`, `:63`, `:93`, `:116`; `README.md:210`, `:212-224`, `:226`, `:228-237`; `justfile:46-48`. No misnumbering found in this set.

## Shapes checked that produced NOTHING

- PROHIBITIONS versus PERMISSIONS beyond `R4B-4`. I checked every "MUST NOT" and "DO NOT" in the fold against what the document elsewhere requires: `:186` (`next` must emit no action), `:188` (do not treat unsafe as absent), `:205` (do not add a blocked-steps variant), `:213` (closed enums, not free strings), `:236` (do not invent a JSON reason for `status --resume`), `:250` (`[meta].ledger` not built), `:254` (do not "fix" the accepted costs), `:364` (do not run `just scaffold-self`), `:366` (not `pack/instrument.md`, not the role prompts), `:375` (do not widen or rename the variants). Each has a matching permission or requirement elsewhere and none conflicts.
- THE TERM "SURFACE", which is used for a command (`:258`, `:333`) and for a rendering (`:275`, `:304`, `:325`). Every load-bearing use is disambiguated in its own sentence (check 19 names `validate` and `status`/`next` explicitly; check 14e names `--json`), so no claim is true under one sense and false under the other. Not raised.
- THE TERM "THE MECHANISM", which means the anchor plus the refusal at `:150`, `:164`, `:298` and `:280`. The sense is stable; only `:280`'s claim about it is false (`R4B-3`).
- "IN TWO CASES RE-RUN FIRST-HAND BY THE ORCHESTRATOR" (`:3`). The body attributes a first-hand orchestrator run to the `next` fabrication at `:119` and `:186` (commit `3170e3f`) and attributes the `status --resume` leak to explorer B at `:132`. The claim is a count without an enumeration rather than a count against a list, so it is unsupported rather than contradicted. The plan TOML at `:1698` makes the same attribution without a count ("re-verified first-hand by the orchestrator on main"), so it neither supports nor contradicts the numeral (`grep -c "in two cases"` over the TOML returns 0). NOT raised as a finding; recorded here so the next round does not spend time on it.
- THE ACCEPTANCE CHECK NUMBERING (1 to 20 with 14b to 14h). Every internal reference resolves: `:304` cites 4, 11, 14b, 14e, 15; `:312` cites 14b; `:324` cites 14b; `:325` cites 14b, 14c, 14d; `:326` cites 14f's own runs; `:330` cites 10; `:333` cites both inc2 manifestations. No dangling check reference.
- ROUND 2's `INC2-7` (no precedence for an over-determined `no_active_loop_reason`) and ROUND 1's `F-5` (the dangling `validation-constraints` reference). Both are accepted residuals; I confirmed both are still present and did NOT raise either.

## Method, in the order I ran it

1. Read the primary sidecar in full, then both other sidecars in full, then the `[[step]]` and `[[question]]` entries in the plan TOML.
2. Built the claim inventory above: per-increment scope, per-defect end properties, every exclusivity claim, every stated count, every prohibition, and every acceptance-check assertion.
3. Cross-multiplied the inventory: each scope claim against each acceptance check, each exclusivity claim against the whole file, each count against the list it heads and against any twin statement of the same count elsewhere in the fold.
4. Ran the prior rounds' own fix-scope greps to find twin sites their literal-string sweeps could not reach, which is what produced `R4B-1` and `R4B-3`.
5. Opened the code only to decide which side of a contradiction is true (`R4B-1`, `R4B-5`, `R4B-7`) or to confirm a negative (`#[serde(skip)]`, the JSON exhaustiveness sweep, the pack mentions, the `StepPhase` variants, the four match arms).
6. Applied the post-increment tense throughout, and said so explicitly in `R4B-3`, `R4B-4` and `R4B-7`, which are the three findings where the two tenses give different answers.
