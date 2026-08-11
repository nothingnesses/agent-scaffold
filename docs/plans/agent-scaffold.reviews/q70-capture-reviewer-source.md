# Round 1 review: `q70-capture`, source-tree and test-suite lens

Artifact: `git diff main..HEAD` on `review/q70-source` (the `Q-70` `[[question]]` entry, the `[meta].orphan_tasks` line, the empty sidecar, and the regenerated Markdown).

Lens: the code and the tests, not the document. The two known validation paths (W5's lexical check in `src/workflow.rs`, the structural membership check in `src/plan/source.rs`) are treated as already recorded and are not re-reported as findings.

Baseline state, measured before anything else: `just test` is green at HEAD (377 lib tests plus the integration binaries, 0 failures), and the artifact itself validates and renders clean (`validate --source ... --workflow` reports `workflow invariants hold`; `render ... --check` reports `up to date`).

Seven findings: three `high`, three `medium`, one `low`. No `critical`.

## R1C-1 (high): the declared-increment namespace covers under half of the increment identities the checks operate on, and two of them cannot be declared at all

Claim. `Q-70` frames one of the two directions as "a narrow lookup of the waived increment against the step's declared `[[step.increment]]` set". Measured against the live data, that set is not a model of the plan's increments: it holds 43 of the 94 increment identities the round log actually uses, and `round-log-core-incA` / `round-log-core-incB` cannot be added to it, because increment ids are lowercase-only. The pass would weigh that direction against a wrong picture of what it covers.

Evidence. Reproducible measurement, run from the worktree root:

```
nix shell nixpkgs#python3 --command python3 - <<'PY'
import tomllib, json
d=tomllib.load(open('docs/plans/agent-scaffold.plan.toml','rb'))
slugs={s['slug'] for s in d['step']}
declared={i['id'] for s in d['step'] for i in s.get('increment',[])}
orph=set(d['meta'].get('orphan_tasks',[]))
ids=set()
for line in open('docs/metrics/workflow.jsonl'):
    line=line.strip()
    if not line: continue
    o=json.loads(line)
    if o.get('type')=='round': ids.add(o.get('increment') or o.get('task'))
und=[i for i in ids if i not in declared]
print(len(ids), len(ids&declared), len(und))
print(sum(1 for i in und if i in slugs), sum(1 for i in und if i in orph))
print(sorted(i for i in und if i not in slugs and i not in orph))
PY
```

Output: 94 distinct round increment identities, 43 declared, 51 undeclared. Of the 51, 24 are exactly a step slug, 14 are `[meta].orphan_tasks` tokens, and 13 are neither: `decision-folder-currency-fold`, `optional-modules-inc1`, `optional-modules-inc2a`, `optional-modules-inc2b`, `optional-modules-inc2ci`, `optional-modules-inc3`, `round-log-core-incA`, `round-log-core-incB`, `state-schema-inc1`, `state-schema-inc2`, `state-schema-inc3`, `workflow-enforcement-tier-endproperty-fold`, `workflow-enforcement-tier-fold`.

The undeclarable pair is settled by citation, so no test is manufactured for it. `is_kebab_case_token` rejects any uppercase byte (`src/plan/source.rs:475-477`), increment ids are held to it (`src/plan/source.rs:552-557`), and the behaviour is already pinned by `an_uppercase_increment_id_is_flagged` (`src/plan/source.rs:1220-1228`, which asserts `increment id 'a-incA' is not a well-formed kebab-case id`). The round log's `-incA` / `-incB` form is not accidental: `src/workflow.rs:64-68` documents it as the reason the strip accepts an alphanumeric run rather than digits.

Two further measured shapes the item does not record. `round-log-core` is `complete` and declares ZERO increments while its rounds carry `round-log-core-incA` and `-incB`. `optional-modules` declares exactly one increment, `optional-modules-inc2cii`, which is exactly the one it waives, while five more of its increment identities appear only in the log. The declared-increment set is, on the live data, a by-product of `src/plan/source.rs:807`'s membership rule rather than an independent declaration of the plan's increments, and that is the set the "narrow lookup" direction would key on.

Impact if left unfixed. The pass is invited to compare a direction whose coverage on real data is 43/94, with two identities structurally excluded, against an alternative, without that number in front of it.

## R1C-2 (high): W5 cannot perform the declared-increment lookup at all without widening a shared, serialised, cross-substrate type

Claim. The direction `Q-70` calls narrow is not a change inside `w5_problems`. W5 is handed a step view that has no increment concept, and on one of the two substrates no increment declarations exist anywhere to populate it from.

Evidence, all by citation, which settles a structural claim.

- `w5_problems(waivers: &[Waiver], steps: &[Step], escalations: &[Escalation])`, `src/workflow.rs:544-548`.
- That `Step` is `plan::Step`, `src/plan.rs:54-60`, whose entire content is `slug: String` and `status: String`.
- `PlanToml::step_views()` builds it and DROPS the increments: `src/plan/source.rs:422-430` maps only `slug` and `status`.
- The Markdown substrate cannot supply them at all. `grep -c increment src/plan.rs` returns `0`: the Markdown plan model has no increment concept. `src/next.rs:520` and `src/next.rs:551` both state it in the code's own words, "the Markdown substrate (which declares no increments)".
- `plan::Step` is `Serialize` and is the `status --json` payload (`PlanProjection.steps: Vec<plan::Step>`, `src/main.rs:583-585`), so widening it changes a machine output contract.
- `src/next.rs:517-523` records that the declared `[[step.increment]].risk_class` is deliberately NOT carried into the projection, precisely so "the Markdown substrate ... produces an identical projection to the TOML one (the parity property)". Making W5 depend on declared increments puts a second, opposite pressure on that same property, and the item does not mention it.

Runnable confirmation that the Markdown substrate is live and hits the same refusal with no declared-increment set in existence:

```
printf '## Roadmap\n\n| Step | Status | Notes |\n| ---- | ------ | ----- |\n| `alpha` | complete | n |\n\n## Open Questions\n\n(none)\n' > $S/md/docs/plans/t.md
printf '{"type":"round","task":"alpha-stage0a","artifact":"a","phase":"review","changed_since_prev":true,"outcome":"new_valid","valid_findings":1,"severities":["low"],"consecutive_clean":0,"risk_class":"risky","step":"alpha"}\n{"type":"waiver","task":"alpha","unit":"increment","step":"alpha","increment":"alpha-stage0a","reason":"review-skipped","evidence_tier":"self-declared"}\n' > $S/md/docs/metrics/workflow.jsonl
just run validate --plan $S/md/docs/plans/t.md --metrics $S/md/docs/metrics/workflow.jsonl --workflow
```

Reports `round log line 2: increment waiver names step 'alpha' but increment 'alpha-stage0a' belongs to step 'alpha-stage0a'`, exit 1. `check_workflow` is reached from `src/main.rs:1030`, so this path is not dead.

Impact if left unfixed. `Q-70` asks each explorer to "state the edit surface its direction implies", then supplies a framing that makes one direction sound like a one-line swap at `src/workflow.rs:564`. The real surface is the shared `plan::Step` type, the `run_checks` funnel (`src/workflow.rs:206-221`), a JSON output contract, and a ruling on what the Markdown substrate does when it has no declared increments.

## R1C-3 (high): the item imposes a binary choice on the pass, and the code admits a third direction it does not name

Claim. The coupling paragraph tells each proposal it "must state which it is choosing" between "a narrow lookup of the waived increment against the step's declared `[[step.increment]]` set, or a rework of how a waiver names its unit". The code admits at least one further direction that is neither, and that is narrower than both.

Evidence. The identity W3 actually exempts against is `round_increment_id`, "the structured `increment` id when the record carries one, else its `task` verbatim" (`src/workflow.rs:127-129`), and W3 matches a covering waiver on exactly that value plus the step (`src/workflow.rs:498-502`). `run_checks` already holds `rounds` and already passes them to `w3_problems`; `w5_problems` is simply not given them (`src/workflow.rs:206-221`). An ownership rule stated against the round log rather than against the plan therefore needs no new data source, no type change, and no substrate fork: `check_workflow` and `check_workflow_toml` both feed `metrics::parse_rounds(log_contents)` into the same funnel (`src/workflow.rs:158-167` and `src/workflow.rs:185-194`).

I am not recommending it, and it may well be wrong for other reasons the pass should weigh. The finding is that the item forecloses it by instruction rather than by argument, in a document whose own opening says it "carries NO options and NO recommendation, deliberately".

Impact if left unfixed. Explorers are constrained to two directions, one of which R1C-1 and R1C-2 show is more costly than the item implies, while a candidate the code plainly permits is out of bounds before the pass starts.

## R1C-4 (medium): a third live waiver-validation path exists, and the item refers to it only in the past tense

Claim. `Q-70` says the `src/plan/source.rs` rules were "moved from the round log's `check_record` waiver arm". That arm was not retired. It is live, reachable, and enforces a third waiver schema, in which `step` IS independently authorable, which is the exact thing escape route 2 says is impossible.

Evidence, measured. `src/metrics.rs:539-601` is the `waiver` arm of `check_record`. Run against a log-only fixture:

```
printf '{"type":"waiver","task":"t","unit":"increment","step":"anything-at-all","reason":"review-skipped","evidence_tier":"self-declared"}\n' > $S/logonly/w.jsonl
just run validate --metrics $S/logonly/w.jsonl
```

reports `w.jsonl:1: missing field 'increment'`, exit 1, so the arm runs. And:

```
printf '{"type":"waiver","task":"t","unit":"increment","step":"totally-made-up-step","increment":"some-other-step-inc1","reason":"review-skipped","evidence_tier":"self-declared"}\n' > $S/logonly/w2.jsonl
just run validate --metrics $S/logonly/w2.jsonl
```

reports `1 records, valid`, exit 0: a waiver whose `step` names no step at all and whose `increment` belongs to a different step passes this path, which performs no ownership check.

Two things follow that the item does not record. First, escape route 2's strengthened claim is substrate-scoped, correctly so for the TOML flow, but the JSONL `type:"waiver"` record still carries a freely-authorable `step` (`src/metrics.rs:556-559`, `step` required and non-empty, nothing more), and that record shape is what `AGENTS.md:147` and `pack/instrument.md:11` still document as THE way to write a waiver. It is inert here only because `check_workflow_toml` reads `waivers_from_toml(plan)` alone (`src/workflow.rs:180-195`), so under `[meta].primary = "toml"` a JSONL waiver record grants nothing and is reported by nothing. Neither half of that is in the item. Second, the "rework how a waiver names its unit" direction therefore has three schemas to change, not one: `src/metrics.rs`'s `check_record` arm, `src/plan/source.rs`'s typed struct, and the `waivers_from_toml` flattening at `src/workflow.rs:237-267` that reconciles them.

Impact if left unfixed. An explorer costing the second direction counts one schema and one file, and misses two.

## R1C-5 (medium): the fix's edit surface includes drift-guarded generated files, and the item points at neither them nor the pack

Claim. `Q-70` requires each proposal to say "whether any generated const or drift-guarded file is involved", and names only `src/workflow.rs` and `src/plan/source.rs` anywhere in the item. W5's ownership rule is stated verbatim in a pack source and in two generated files under a whole-file drift guard, so any fix that changes the rule must change all three together or the test suite fails.

Evidence. `LC_ALL=C grep -n "must own its .increment." AGENTS.md .agents/AGENTS.reference.md pack/instrument.md` hits all three, each carrying the clause "an `increment`-unit waiver's `step` must own its `increment` (the increment's leading slug equals the step)" (`AGENTS.md:147`, `.agents/AGENTS.reference.md:147`, `pack/instrument.md:11`).

Mutation demonstration. In `pack/instrument.md` only, that clause was changed to "(the increment is one of the step's declared increments)", which is the doc edit a structural-lookup fix requires. `just test` then reports:

```
test agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render ... FAILED
test result: FAILED. 377 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

The edit was reverted and `git status --porcelain` is empty. The guarded set is stated at `src/agents_md_drift.rs:41-55`: the committed root `AGENTS.md` and `.agents/AGENTS.reference.md` against a fresh render of the pack under the pinned `just scaffold-self` config. Regenerating them is `just scaffold-self`, whose second line is `nix fmt` over the whole tree.

The same applies to the second direction: `pack/plan-template.plan.toml:39-44` carries the commented `[[step.waiver]]` example, and `src/plan/render.rs:513-529` writes each waiver into the generated `<task>.md` as `waived: increment '<id>' ...`, which `render --check` pins.

Impact if left unfixed. A proposal that answers the item's own edit-surface question with "`src/workflow.rs`" is accepted as complete, and the step authored from it fails `just test` on a guard nobody costed.

## R1C-6 (medium): a third instance of the blocker is already latent on declared increments, and is not in the item

Claim. `Q-70` presents the blocker as arising from two orphan fold tokens whose `task` does not end `-inc<alnum>`. The live plan already declares three `[[step.increment]]` ids in the same shape, under a step that has not yet gone `complete`, so a third case is waiting behind exactly the latency the item describes.

Evidence. `docs/plans/agent-scaffold.plan.toml`, step `workflow-driver` (status `in-progress`) declares `workflow-driver-stage0a`, `workflow-driver-stage0b` and `workflow-driver-stage1`. None carries an `-inc<alnum>` suffix, so `leading_slug` returns each unchanged (`src/workflow.rs:88-96`) and it can never equal the step slug `workflow-driver`.

Measured, not read. A copy of `docs/` was taken under the scratch fixture directory, and a single self-declared increment waiver was injected on the step's own declared increment:

```
[[step.waiver]]
id = "workflow-driver-w1"
unit = "increment"
increment = "workflow-driver-stage0a"
reason = "review-skipped"
evidence_tier = "self-declared"
```

`just run validate --source <copy>/docs/plans/agent-scaffold.plan.toml --metrics <copy>/docs/metrics/workflow.jsonl --workflow` reports exactly one problem, at exit 1:

```
TOML waiver `workflow-driver-w1`: increment waiver names step `workflow-driver` but increment `workflow-driver-stage0a` belongs to step `workflow-driver-stage0a`
```

No problem is reported from the `src/plan/source.rs` path, because membership holds. So this is escape route 4's declared case, on pre-existing plan data rather than an injected fold token: a properly declared increment of its own step, refused by W5 and accepted by the source path. W3 skips it today only because the step is not `complete` (`src/workflow.rs:445-447`).

Impact if left unfixed. The pass is scoped from a two-case sample of orphan fold tokens, when the third case shows the blocker is a property of the plan's increment-naming convention, which `[[step.increment]]` accepts freely and W5's lexical rule contradicts. That changes what "a rework of how a waiver names its unit" would have to cover: keeping the lexical rule would mean renaming three already-declared increments, not just two orphan tokens.

## R1C-7 (low): the routed writer item resolves on the code, and the resolution is a type distinction the item never names

Claim. The planner reported escape route 4 and the later paragraph as stating route 4's evidence at two different scopes, named two closures, and declined to choose. On the code they are not in tension, and route 4's evidence is true but weaker than the source supports.

Evidence. There are TWO types called `Step`. `plan::Step` (`src/plan.rs:55-60`) holds `slug` and `status` and nothing else, and is what `w5_problems` is handed (`src/workflow.rs:546`). `plan::source::Step` (`src/plan/source.rs:135-165`) holds `increments: Vec<Increment>` and is what `validate_source` walks (`src/plan/source.rs:791-793`). Route 4's phrasing, "`w5_problems` derives only a slug set from the steps ... and never reads `step.increments`", is accurate but reads as a behavioural choice. The stronger and simpler statement is that W5's `Step` has no `increments` field to read, which is also precisely why a structural lookup can exist in the other path without contradicting route 4: the two paths hold different types. Stating it at the type level closes both closures at once with no preference to express.

Impact if left unfixed. A reader of route 4 can reasonably infer W5 could read the declared increments and simply does not, which is the inference R1C-2 shows is wrong.

## Source-code observations, NOT findings against this artifact

Routed separately so the orchestrator can place them. Neither is a finding against `Q-70`.

1. `AGENTS.md:147` and its pack source `pack/instrument.md:11` document `type: "waiver"` as a JSONL record and say "Two checks read waivers", with no mention that under `[meta].primary = "toml"` the workflow checks read `[[step.waiver]]` entries only (`src/workflow.rs:180-195`) and ignore every JSONL waiver record. This repo's own log carries zero `type:"waiver"` records (`grep -c '"type":"waiver"' docs/metrics/workflow.jsonl` returns 0) while the plan carries 25 TOML waivers, so an agent following the generated operating doc would write a waiver into a substrate that grants nothing here. The adjacent `baseline` bullet at `AGENTS.md:146` DOES carry the TOML-supersession note, so the omission looks like an oversight in the waiver bullet rather than a decision.

2. `check_record`'s waiver arm accepts a `type:"waiver"` record naming a step that does not exist and an increment belonging to another step (demonstrated in R1C-4). That is by design as a schema check, and W5 catches both when the record reaches it, but on a TOML-primary repo W5 never reads that record, so the combination leaves an unreachable-but-valid authoring surface. Whether that matters is a scoping call, not a defect I am asserting.

## What was settled by running something, and what by reading

Run: the `just test` baseline; the `agents_md_drift` mutation (R1C-5); the injected `workflow-driver-stage0a` waiver against a copy of the live plan (R1C-6); the two `validate --metrics` fixtures showing `check_record`'s waiver arm is live and performs no ownership check (R1C-4); the Markdown-substrate `validate --workflow` fixture (R1C-2); the `tomllib` plus JSONL measurement of declared versus logged increment identities (R1C-1); the artifact's own `validate` and `render --check`.

Read: the type-level claims in R1C-2 and R1C-7 (`plan::Step`, `plan::source::Step`, `step_views`, `grep -c increment src/plan.rs`); the third-direction claim in R1C-3 (what `run_checks` already holds); the uppercase-id exclusion in R1C-1, which an existing test and a citation already settle; the drift-guard coverage statement in R1C-5, whose consequence was then measured.

All fixtures were built under the session scratchpad directory only. No file outside it was created or deleted. The `pack/instrument.md` mutation was reverted and the worktree is clean.
