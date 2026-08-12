# Q-70, the architecture lens: one authoritative notion of waiver ownership

Explorer proposal for `Q-70`. Lens: the cleanest long-term architecture. Two other explorers ran in parallel with different lenses and I have not seen their work.

Everything below is either MEASURED (I ran a command, and the command is given in the appendix) or REASONED (I argue it from source I cite). Every claim is labelled. I re-verified every figure I rely on from `Q-70`, because this item's own record says every figure in this project has a history of being wrong, and I found one of `Q-70`'s verification commands now self-falsifies (recorded under "corrections to the item" below).

---

## 1. The question, restated

`Q-70` opens with "how to fix W5's waiver-ownership check". That is the symptom. The question underneath it, and the one this document answers, is:

> **The tool answers "does this waiver own this increment" in more than one place, by more than one method, with a measured disagreement, and the answer differs by substrate. What is the one correct answer, where should it live, and what falls out of it?**

The mandate is wider than the W5 fix: `Q-55-entryroute` chose "Design pass, validator cluster only" over "Split out the W5 fix first", so the W5 fix plus all three detection mechanisms are in scope. I rule on all of it, and on every lettered duty, in section 12.

---

## 2. How many places answer the ownership question, and what each does

MEASURED by reading the source and by running the fixtures in the appendix. There are **four** representations, not the two or three the item names.

| # | Site | Method | Data it consults | Verdict on the declared case |
| --- | --- | --- | --- | --- |
| 1 | `src/workflow.rs:498-502` (W3's covering-waiver match) | Structural pair match | The round log: `round_step_slug(r)` and `round_increment_id(r)` | Owns it (the exemption applies) |
| 2 | `src/workflow.rs:564` (W5's ownership rule) | Lexical substring strip | The waiver's own `increment` string, nothing else | Does NOT own it (refused) |
| 3 | `src/plan/source.rs:807-811` (per-step waiver loop) | Structural set membership | The step's hand-declared `[[step.increment]]` ids | Owns it (silent) |
| 4 | `src/metrics.rs:539-601` (the JSONL `check_record` waiver arm) | No ownership check at all | Presence rules only | No opinion |

**The architectural finding, and it is the whole of my proposal: site 1 is already the correct answer, and it is already single-sourced, derived, and substrate-uniform. Site 2 is a second, weaker restatement of the same relation that disagrees with it.** W3 does not ask "does `leading_slug(increment)` equal the step". It asks whether the waiver's `(step, increment)` pair is a pair the round log actually recorded. That is a fact about the world. W5's rule is a fact about a string.

`Q-70` records the disagreement as a two-path problem between sites 2 and 3. That framing is accurate but it points at the wrong pair. Sites 2 and 3 disagree, but site 1, which nobody has been comparing them against, already holds the answer both of them are approximating, and site 1 is the one the tool actually acts on when it decides whether an exemption applies.

The code says so about itself twice. `leading_slug`'s own doc closes at `src/workflow.rs:83-87` with "This shim remains only for pre-migration records that omit the structured id", and thirty lines below the ownership rule the record-backed evidence join at `:594-596` is already structured, under a comment that says it prefers "the escalation's structured ids (Inc 2) over the `leading_slug`/`task` shim". One function, two checks, one migrated and one not.

MEASURED: `leading_slug` has exactly three production call sites today (`round_step_slug`, `escalation_step_slug`, and W5's ownership rule at `:564`). Under my recommendation it has two, and both are the documented pre-migration shim applied to RECORD data, which is the only scope its doc comment claims. Today the doc comment is contradicted by `:564`, which applies the shim to a plan-authored id that is not a `task` at all.

---

## 3. What I measured

All commands are in the appendix. Fixtures live under my own scratch subdirectory and touch nothing in the repository.

### 3.1 The affected population is six, not two

```
decision-folder-currency        decision-folder-currency-fold
workflow-driver                 workflow-driver-stage0a
workflow-driver                 workflow-driver-stage0b
workflow-driver                 workflow-driver-stage1
workflow-enforcement-tier       workflow-enforcement-tier-endproperty-fold
workflow-enforcement-tier       workflow-enforcement-tier-fold
```

MEASURED with the item's own reproduction command. `Q-70` names two blocking members and the `workflow-driver` trio, and correctly declines to state a count. The sixth, `decision-folder-currency-fold`, is the one worth noticing: **its step is already `complete`**, it converged (peak `consecutive_clean` 1 against a `low_risk` requirement of 1), so W3 is satisfied and nothing fires. Had that loop escalated instead of converging, it would have been a second blocked step, in a step that has already shipped. The defect is not confined to in-flight work; it is latent across the finished plan and only invisible because those loops happened to converge.

### 3.2 The hand-declared increment set does not model the identities the checks operate on

MEASURED:

- 95 distinct increment identities appear in the round log.
- 45 `[[step.increment]]` ids are declared in the plan; 43 of them appear in the log, and 2 are declared with no round records at all (`status-resume-ignores-json-inc1`, `test-tmpdir-repo-assumption-inc1`, both on `not-started` steps).
- 52 log identities are undeclared. 15 of those are `[meta].orphan_tasks` tokens (tasks that own no step). **37 belong to a step and are simply not declared.**
- 2 of those 37, `round-log-core-incA` and `round-log-core-incB`, **cannot be declared at all**: `is_kebab_case_token` (`src/plan/source.rs:475-477`) forbids an uppercase byte in an increment id, pinned by the test `an_uppercase_increment_id_is_flagged`, and the round log uses the `-incA` / `-incB` form that `src/workflow.rs:64-68` documents the strip as existing for.

This is the fact that decides the design space. The declared set is hand-maintained, is derived from nothing, and describes 43 of 95 identities. Any design that makes it the authority for enforcement makes a hand-maintained partial list load-bearing.

### 3.3 The two paths disagree, and the disagreement reproduces exactly as the item records

MEASURED, in a scratch copy of `docs/`, against the binary built from this tree.

| Fixture | Shape | `src/plan/source.rs` path | W5 | Exit |
| --- | --- | --- | --- | --- |
| `fx-base` | Live plan, untouched | silent | silent | 0 |
| `fx-A` | `-w5` waiver added, fold token UNDECLARED | fires | fires | 1 |
| `fx-B` | `fx-A` plus the fold token declared as `[[step.increment]]` | silent | fires | 1 |
| `fx-C` | Waiver naming `totally-not-a-step-inc1` | fires | fires, naming a step that does not exist | 1 |
| `fx-D` | Both fold tokens declared, both owed waivers written, step flipped `complete` | silent | fires twice | 1 |

`fx-D` is the decisive one: **with the declarations, both waivers, and the step complete, the only thing left failing in the entire cross-reference is W5's ownership rule, twice.** Everything else about the step is already green. `Q-70` records this and it holds.

### 3.4 The prototype

MEASURED. I copied the tree to scratch, replaced W5's lexical rule with a lookup into the observed `(step, increment)` relation derived from the round log, and ran the whole matrix again.

| Fixture | Shipped binary | Prototype |
| --- | --- | --- |
| `fx-base` (all 25 live waivers) | exit 0 | **exit 0** |
| `fx-A` (undeclared) | both paths fire, exit 1 | source path only, exit 1 |
| `fx-B` (declared) | W5 fires, exit 1 | **exit 0, the two paths now agree** |
| `fx-C` (phantom increment) | both fire, W5 invents a step | both fire, W5 says "has no round records" |
| `fx-D` (full unblocking) | W5 fires twice, exit 1 | **exit 0, "workflow invariants hold"** |
| Markdown+JSONL fixture, same shape | W5 fires, exit 1 | **exit 0** |

And the suite:

- `cargo test`: **422 pass, 0 fail** on the prototype, including `the_committed_scaffold_matches_a_fresh_render` (the drift guard) and the render golden fixtures.
- `cargo clippy --all-targets -- -D warnings`: clean.
- The prototype touches **one file**, `src/workflow.rs`. The production change is about 30 lines: a 6-line derived-relation helper, one widened signature, one changed call site, and the rule body. The rest of the diff is the six test fixtures described in section 11.

The `fx-base` result is the migration proof: **every one of the 25 waivers already committed to this plan satisfies the new rule with no edit to any waiver and no edit to the append-only log.**

### 3.5 The Markdown substrate carries the identical defect and takes the identical cure

MEASURED, with a minimal Markdown plan plus a JSONL log carrying a `type:"waiver"` record for increment `beta-fold` under step `beta`:

- Shipped binary: `round log line 5: increment waiver names step 'beta' but increment 'beta-fold' belongs to step 'beta-fold'`, exit 1.
- Prototype: `workflow invariants hold`, exit 0.

This is the single most important measurement in the document and I return to it in section 6.

### 3.6 Every dangling decision receipt is one question's sub-decisions

MEASURED (2026-08-12; this population moves, so re-measure rather than citing the figure):

- 63 distinct `q_id` values across `type:"decision"` records.
- 70 registered `[[question]]` ids.
- 41 receipt ids resolve to no registered question.
- **All 41 are of the form `Q-<n>-<suffix>`. Zero plain `Q-<n>` receipts dangle. All 41 have the same parent, `Q-55`, and `Q-55` is registered.**

That settles letter (e) on evidence rather than on deliberation. See section 12(e).

### 3.7 The waiver-note breakdown convention has four instances and is prose

MEASURED. `grep -onE "[(][0-9]+(, [0-9]+){1,6}[)]"` over the plan returns exactly four sites, all four in the `workflow-enforcement-tier` step's waivers. Thirteen waivers are increment-unit; nine of them carry no breakdown at all. The phrasing is not uniform: three read `<n> valid findings (...)` and one reads `<n> valid findings in scope (...)`.

I wrote a probe that joins each breakdown to its increment's round records **using the observed `(step, increment)` relation** and compares the per-round numbers. All four agree, first try. That is a runnable demonstration that the prospective waiver-note join and the W5 ownership rule share the join, which is the coupling question letter (a) asks. See section 12(a).

### 3.8 Corrections to the item

Small, and offered as evidence rather than as findings.

1. `Q-70` says of the `fx-C` shape that "`grep -c 'slug = "totally-not-a-step"'` over that same plan returns 0". **It returns 1 today**, because `Q-70`'s own prose, quoting that command, now lives in the plan. The anchored form `grep -c '^slug = "totally-not-a-step"$'` returns 0. This is exactly the self-quoting class the item identifies as constraint (3a) on the quotation resolver, turned on the item itself, and it is a live demonstration that (3a) is real.
2. `Q-70`'s claim (3b) holds: a fixed-string grep for either quoted validator problem string finds them only in the plan TOML and its rendered `agent-scaffold.md` projection, in no source file. A naive resolver would report the item's strongest evidence as dangling.
3. I spot-checked five of the fifteen distinct `src/checks.rs` citations in `checks-runner-worktree-name-collision.md` (lines 78, 400, 407, 425, 1438). All five land on unrelated content, consistent with the item's claim that every one of them does. I did not verify all fifteen; the claim is not load-bearing for my proposal.

---

## 4. The design space

Five viable directions. (i) to (iv) are the item's candidates; (v) is outside them. I price each against the plan's numbered Project Principles by name.

### Option A: direction (i), a lookup against the step's declared `[[step.increment]]` set, inside W5

Teach W5 the same structural membership check `src/plan/source.rs:807` already performs.

- **Prefer the cleaner long-term architecture over the smallest diff**: fails. It does not remove the second answer to the ownership question, it creates a duplicate of the third answer in a place that cannot see the data. Today sites 2 and 3 at least ask different questions; under A they ask the same question twice, in two files, over two copies of the input. That is strictly further from one authoritative notion.
- **Structured data first, project for humans**: fails on the substance rather than the letter. It makes a hand-maintained list the enforcement authority. MEASURED: that list models 43 of 95 identities and cannot express 2 of them. It is hand-authored prose data that is also the machine input, which is the shape this principle exists to avoid.
- **Minimal by default**: fails. `PlanToml::step_views()` (`src/plan/source.rs:422-430`) drops increments when it builds the view W5 receives, and `plan::Step` (`src/plan.rs:55-60`) is `Serialize` and is the `status --json` payload (`PlanProjection.steps`, `src/main.rs:582-585`), so widening it changes a machine output contract for the benefit of one check.
- **Fatal defect, MEASURED by reading**: the Markdown substrate declares no increments, ever. `plan::parse_roadmap` yields `Step { slug, status }` and nothing else. So on Markdown the rule either vanishes (a substrate-conditional check, breaking the one-implementation property `src/workflow.rs:196-200` states as Principle 16) or refuses every increment waiver on every Markdown project. Neither is shippable. `src/next.rs:517-523` already records the parity property that deliberately withholds declared-increment data from its own projection so both substrates project identically; option A pushes directly against it.
- **Ground decisions in evidence**: the rule's refusals would be facts about the plan document, which is an improvement on today. That is the one thing it gets right.

### Option B: direction (ii), rework how a waiver names its unit

Reshape the waiver schema so ownership is positional, for example nesting the waiver under the increment (`[[step.increment.waiver]]`) instead of under the step, which collapses `unit`, the `increment` field, and the presence rules that pair them.

- **Make illegal states unrepresentable**: this is the only option that genuinely serves it, and I take that seriously. See section 7, where I rule on it properly.
- **Prefer the cleaner long-term architecture**: this IS the cleaner long-term shape of the waiver schema. It is not the cleaner shape of the ownership relation, which is a different question, and B answers the schema question while leaving the ownership question exactly where it is.
- **Ground decisions in evidence**: fails on sequencing. B encodes ownership against the declared set, and MEASURED, the declared set does not describe reality (section 3.2). B would make "a waiver names a non-declared increment" unrepresentable while leaving "the declared set is 43 of 95" untouched, and would block waiving the 37 undeclared step-owned identities until each is hand-declared, two of which cannot be declared without relaxing `is_kebab_case_token`.
- **Cost, MEASURED**: three representations move together (`src/metrics.rs:539-601`, `src/plan/source.rs:279-300`, `src/workflow.rs:237-267`), plus the commented example at `pack/plan-template.plan.toml:44`, plus `waiver_note` at `src/plan/render.rs:516-530` which is pinned by `render --check`, plus all 25 committed waivers rewritten. It also collides head-on with the recorded `Q-55-mechanism` constraint that the record schema W3, W4 and W5 all read must take one deliberate edit rather than a rider on a path fix.
- MEASURED and worth knowing before pricing B: there are **zero** `type:"waiver"` records in this repository's log. The JSONL representation is live, reachable code with no live data here. B pays to reshape three representations, one of which this project does not use.

### Option C: direction (iii), state the ownership rule against the round log

W5 asks whether the waiver's `(step, increment)` pair is a pair the round log records, keyed on `round_step_slug` and `round_increment_id`, exactly as W3's covering-waiver match at `src/workflow.rs:498-502` already is.

- **Prefer the cleaner long-term architecture over the smallest diff**: this is the only option that reduces the number of answers rather than moving one. Sites 1 and 2 become one predicate over one derived relation. It also happens to be nearly the smallest change, which is the strongest form this lens can take: the cleanest design here is also the cheapest, and that is not a coincidence but a consequence of the right answer already being present in the code.
- **Structured data first, project for humans**: served directly. The ownership relation becomes a projection of the append-only event log rather than a hand-authored list, which is precisely "prefer machine-readable structured formats as the single source for the data the workflow operates on".
- **Ground decisions in evidence**: served in the strongest available sense. Every refusal W5 can emit is backed by a record in the log. Today, MEASURED at `fx-C`, W5 emits "increment `totally-not-a-step-inc1` belongs to step `totally-not-a-step`" about a step that does not exist anywhere. The ownership assertion is a substring, not a fact. Under C it is a fact or it is not emitted.
- **Minimal by default**: served. No new data source, no type change, no schema change, no machine output contract touched. `run_checks` (`src/workflow.rs:206-221`) already holds `rounds` and already hands them to `w3_problems`; it simply does not hand them to `w5_problems`. One argument.
- **Make illegal states unrepresentable**: not served, and I do not claim it is. C makes the check SOUND, which is a different and lesser property than unrepresentability. Section 7 says what unrepresentability would cost and why it is not available yet.
- **Idempotent** and **Reproducible**: unaffected. The relation is a pure function of the log.
- **Cost**: W5's verdict now depends on the log. That is not a new dependency class: `w5_problems` already takes `escalations`, which come from the log, and its record-backed evidence join already crosses from plan to log. C adds a second log input to a check that is already a plan-to-log join.

### Option D: direction (iv), scope the rule to the substrate where its premise holds, or retire it

Keep W5's ownership rule only for JSONL waivers (whose `step` is independently authorable), or delete it and lean on `src/plan/source.rs:807`.

- **Prefer the cleaner long-term architecture**: fails. It writes the current confusion into the design instead of removing it: two substrates, two different notions of ownership, permanently.
- **Minimal by default**: superficially served, genuinely failed. A substrate-conditional rule pushes directly against the one-implementation property at `src/workflow.rs:196-200`, and MEASURED, it would leave the Markdown+JSONL substrate defective in exactly the way section 3.5 demonstrates, since the JSONL waiver record's `step` being authorable does not make `leading_slug(increment) == step` a true test of ownership there either. D fixes the authoring half of the asymmetry and leaves the correctness half.
- **Ground decisions in evidence**: MEASURED, D plus the declarations does completely unblock the two owed waivers, which is what the item records and which I confirmed at `fx-D` (the ownership rule is the only remaining failure, so removing it yields exit 0). D works. It is the option that works for the wrong reason.
- Retiring the rule entirely also deletes the only ownership check that runs on the Markdown substrate at all, since `src/plan/source.rs` never runs there.

### Option E, outside the item's candidates: derive the declared increment set, then encode ownership positionally

Make `[[step.increment]]` a projection of, or validated against, the round log, then do option B on top of a set that is true.

- **Prefer the cleaner long-term architecture** and **Make illegal states unrepresentable**: both served, and this is the genuine end state.
- **Ground decisions in evidence**: fails today on precondition. It requires resolving what a declared increment is FOR (`Q-70` records that `complete` steps exist declaring none while their round records carry increment ids), relaxing `is_kebab_case_token`, and reconciling with `src/next.rs:517-523`'s parity property. None of that is diagnosed.
- **Minimal by default**: fails hard as a next step. This is a multi-increment step in its own right.
- I record E because it is where C leads and because a reviewer should see that C is not a dead end. C is a strict prerequisite for E, not an alternative to it: E needs the derived relation C builds.

---

## 5. Recommendation

**Take option C, direction (iii): state W5's ownership rule against the round log, extract the `(step, increment)` relation W3 already computes into one named predicate, and have both checks consult it.**

Concretely:

1. Add `observed_membership(rounds) -> BTreeSet<(&str, &str)>` in `src/workflow.rs`, built from `round_step_slug` and `round_increment_id`. This is the repository's own established idiom for exactly this class: `peak_consecutive_clean` (`src/workflow.rs:407`) is `pub(crate)` precisely so `next` and W3 run identical arithmetic, and `WaiverReason::required_tier` is single-sourced between W5 and `src/plan/source.rs`. This is the third instance of a pattern the project already trusts.
2. Widen `w5_problems` to take `rounds`. `run_checks` already holds them.
3. Replace `leading_slug(increment) != waiver.step` with a lookup into the relation, emitting one of two messages: the increment is recorded under a different step (mis-scoped, and the message names the real step, a fact), or the increment has no round records at all (dangling).
4. Have `w3_problems` consult the same predicate for its covering-waiver match, so the two cannot drift.
5. Change the one parenthetical in the rule text at `pack/instrument.md:11` from "(the increment's leading slug equals the step)" to a statement of the observed relation, and regenerate.
6. Declare `workflow-enforcement-tier-fold` and `workflow-enforcement-tier-endproperty-fold` as `[[step.increment]]` entries, write the two owed waivers, flip the step to `complete`. MEASURED at `fx-D`: this returns exit 0.

Step 6 is required and I want to be explicit that it is not optional under my recommendation. The `src/plan/source.rs` referential-integrity check still fires on an undeclared increment (MEASURED at `fx-A` under the prototype), and that is correct behaviour: it says the plan is incomplete, which it is. `Q-70`'s escape route 4 concludes that declaring the increment does not help; that is true against the shipped binary and false against the fixed one. Declaring plus fixing is the unblocking; neither alone is.

### Why this and not the others, in one paragraph

The tool already contains the correct notion of ownership, computes it correctly, and acts on it. It is at `src/workflow.rs:498-502` and it has been there since W3 was written. Every other option invents or relocates a notion of ownership; only C deletes one. Under "Prefer the cleaner long-term architecture over the smallest diff", the cleanest design is the one with the fewest answers to the question, and C is the only option that ends with one. That it is also the smallest of the five is what makes it worth recommending rather than merely defensible: this is the case where the two criteria agree, and when they agree the argument for the cleaner design does not have to be paid for.

---

## 6. The substrate asymmetry: my position, and its defence

`Q-70` records, correctly, that on this project's own TOML substrate the state W5's rule exists to report cannot be authored: `waivers_from_toml` sets `step: step.slug.clone()` (`src/workflow.rs:258`) and the TOML `Waiver` struct (`src/plan/source.rs:279-300`) is `deny_unknown_fields` with no `step` field. My brief asks whether cross-substrate parity is worth preserving, or whether it is the thing that produced the defect.

**My position: the asymmetry is not the defect, and demanding that every rule fire on every substrate is what would produce a defect. But the parity that matters is parity of IMPLEMENTATION and parity of INPUT, not parity of reachability, and option C is the only direction that preserves both.**

The defence, in four parts.

**First, an unreachable rule is a success, not a bug.** W5's first rule ("a waiver must name a real Roadmap step") is structurally unreachable on the TOML path because the nesting supplies the step. That is Principle "Make illegal states unrepresentable" working exactly as intended: the substrate made the bad state unrepresentable, so the runtime guard has nothing to catch. The correct response to a vacuous rule on one substrate is to note it, not to weaken the substrate or the rule. The rule remains live and load-bearing on the JSONL substrate, where `step` is a free string, and I MEASURED that path firing in section 3.5.

**Second, the actual defect is in a different rule.** The unreachability of rule 1 and the wrongness of rule 2 are separate facts that the item's paragraph joins. Rule 2 is not unreachable on TOML; it fires, loudly, twice, on the live plan (MEASURED at `fx-D`). It is wrong on BOTH substrates, and I demonstrated that on Markdown+JSONL in section 3.5, where the shipped binary refuses a correctly-scoped waiver whose `step` field was independently and correctly authored. The substrate asymmetry is therefore not the cause: making `step` authorable on TOML would not fix rule 2, because rule 2 does not read `step` against anything real on either substrate.

**Third, option C removes the asymmetry rather than accommodating it, and this is measurable.** The round log is the ONE input that is already identical on both substrates: `check_workflow` (`:158-167`) and `check_workflow_toml` (`:185-194`) both pass `metrics::parse_rounds(log_contents)` into the same `run_checks`. Moving ownership onto that input moves the rule's premise from "the waiver's `step` field is independently authored" (true only on JSONL) to "the `(step, increment)` pair is recorded in the log" (true on both). MEASURED: the same fixture shape goes from exit 1 to exit 0 on the TOML substrate (`fx-D`) and on the Markdown substrate, under the same one-file change. That is the parity commitment honoured, not traded away.

**Fourth, the two directions that break parity are the ones that break it in the input, which is worse than breaking it in the output.** Option A feeds W5 declared-increment data that only one substrate possesses, which is why `src/next.rs:517-523` deliberately withholds exactly that data from its own projection. Option D makes the rule itself substrate-conditional. Both put a fork upstream of `run_checks`, where the one-implementation property lives. Option C puts nothing upstream at all: it consumes an argument `run_checks` already has.

So: preserve parity of implementation and of input. Do not preserve, or expect, parity of reachability. A project that insists every rule must be reachable on every substrate will eventually weaken a schema to keep a runtime guard employed, which is the opposite of what "Make illegal states unrepresentable" asks for.

---

## 7. Can a mis-scoped waiver be made unconstructible?

Yes, partly, and part of it is already built. The honest full answer has three parts.

**Already achieved.** On the TOML substrate a waiver cannot name a step at all, because the step is the nesting. The class "waiver names a step that is not its own" is unrepresentable there today. This is why the TOML `Waiver` struct has no `step` field, and it is a real Principle-5 win that the item records as escape route 2 (correctly, as a constraint on authors) without crediting it as a design success.

**Available, at a stated cost.** The remaining half, "waiver names an increment its step does not own", becomes unrepresentable if the waiver nests under the increment rather than the step: `[[step.increment.waiver]]`. Ownership then has no field to get wrong. `WaiverUnit`, the `increment` field, and the two presence rules that pair them all collapse: a waiver nested on a step is a step waiver, a waiver nested on an increment is an increment waiver. That is option B, and it is genuinely the right shape for the schema.

**Why not now, and this is the part I want to be held to.** Unrepresentability is only as good as the thing you encode against, and MEASURED, the declared increment set is not that thing. It models 43 of 95 identities, is derived from nothing, drifts independently of the log, and cannot express two identities the log actually uses. Nesting the waiver under a declared increment would make one illegal state unrepresentable while making the legitimate state (waiving a real increment identity that the plan has not declared) unauthorable for 37 step-owned identities, two of them permanently. It would trade a false refusal for a false prohibition.

The correct sequencing is E, not B: make the declared set true first, then encode against it. Until then, option C buys the property that is actually available, which is SOUNDNESS: every refusal the check can emit is backed by a record. That is weaker than unrepresentability and I do not dress it up as more. It is also, unlike unrepresentability, obtainable today for about thirty lines in one file.

**Cost of doing B or E now, stated as the item requires:** three waiver representations move together, all 25 committed waivers are rewritten, `pack/plan-template.plan.toml:44` and `src/plan/render.rs:516-530` (pinned by `render --check`) both change, the increment-id character rule and its pinning test are relaxed, 37 identities are hand-declared, and the record-schema edit collides with the `Q-55-mechanism` constraint that the schema must take one deliberate edit. That is a step, not an increment, and it does not unblock anything that C does not unblock sooner.

---

## 8. The three detection mechanisms under the correct design

`Q-70` asks whether they fall out of the same structure or merely look related. MEASURED answer: **one of them shares the structure, and the sharing is in the join, not in the data. The other two share nothing with it and should not be designed here.**

### Mechanism 1, the waiver-note breakdown join

It needs two things: the join from a waiver to its increment's round records, and the `valid_findings` field, which `Round` (`src/metrics.rs:620-651`) does not carry and `parse_rounds` discards even though `check_record` requires it in the raw record (`src/metrics.rs:454`; MEASURED, all 235 round records carry it).

**The join is exactly `observed_membership`.** I demonstrated this rather than argued it: my probe in section 3.7 joins each note breakdown to its rounds through the observed `(step, increment)` pair and gets all four right. Under option C that join already exists as a named predicate, so mechanism 1's join cost is zero.

**The data widening does not couple and must not be smuggled in.** Adding `valid_findings` to `Round` is an edit to the record projection that W3, W4 and W5 all read, and that is the precise thing `Q-55-mechanism`'s recorded human constraint governs: one deliberate edit, not a rider on a path fix. So mechanism 1 splits cleanly in two, and the split runs exactly along the line the human already drew.

### Mechanism 2, dangling decision-receipt detection

Shares nothing. `Q-70` records that its inputs are exactly W4's, and MEASURED, its entire red list is one convention question that section 3.6 settles. Its design collapses to a rule once that ruling is made.

### Mechanism 3, the quotation resolver

Shares nothing. It reads no workflow record, no waiver, no round. Its design space is real but entirely separate: scoping to live passages so a post-mortem quoting a deleted sentence does not resolve it (constraint 3a, which section 3.8 shows firing on `Q-70` itself), and an expected-tool-output escape (constraint 3b, MEASURED). Designing it inside a decision about waiver ownership mixes two unrelated design spaces, against "Minimal by default".

---

## 9. Migration

**Nothing in my recommendation rewrites history, and I checked rather than assumed.**

- The append-only round log is not touched. Zero records rewritten, zero records added, zero records reinterpreted.
- MEASURED: `fx-base`, the live plan unmodified, is green under the prototype at exit 0. All 25 committed waivers satisfy the new rule as written.
- The only plan edit is additive: two `[[step.increment]]` entries and two `[[step.waiver]]` entries for work that has already happened, plus the step status flip. No existing entry changes.
- No generated const, no drift-guarded file and no golden fixture is touched by the code half. MEASURED: the full suite including `the_committed_scaffold_matches_a_fresh_render` passes on the prototype with no regeneration.
- The rule-text half does touch drift-guarded files, and I state that fully in section 11 rather than hiding it.

For contrast, and because the item asks every proposal to price the alternatives: option B requires rewriting all 25 waivers and reshaping three representations. Option A requires no rewrite but leaves the Markdown substrate unimplementable. Option E requires everything B requires plus 37 declarations plus a character-rule relaxation.

**One behaviour widening I am recommending, stated plainly.** The "increment has no round records" branch is a new problem class. MEASURED, no existing waiver trips it. A future waiver authored before its rounds are logged would. The conservative alternative is to make the unobserved case silent in W5 and leave it to `src/plan/source.rs` on the TOML path. I recommend keeping it, because it is the only thing that catches a typo'd increment id on the Markdown and JSONL substrate, where no declared set exists to catch it. If a reviewer disagrees, dropping that branch costs four lines and weakens nothing else in the proposal.

---

## 10. What not to build (the YAGNI boundary)

Explicitly out of scope for the step this pass authorises.

1. **Do not reshape the waiver schema.** No `[[step.increment.waiver]]` nesting, no collapsing of `WaiverUnit`, no change to the JSONL waiver arm, no change to `pack/plan-template.plan.toml`, no change to `src/plan/render.rs`. Section 7 gives the precondition; it does not hold.
2. **Do not widen `plan::Step`.** It is a machine output contract (`status --json`). Nothing in the recommendation needs it.
3. **Do not add `project` to `Round` here.** It is queued as `Q-55-mechanism` with a recorded human constraint that the record schema takes one deliberate edit. Option C composes with it for free: an identity filter applied to `rounds` at the `check_workflow_toml` boundary is inherited by every check that reads `rounds`, which under C now includes W5. Today it would not be, because W5 reads no rounds. So C makes the queued edit strictly cheaper, and that is a reason to keep them separate, not to merge them.
4. **Do not add `valid_findings` to `Round` here.** Same reason. It is the second edit to the same struct and it belongs with the first.
5. **Do not build the waiver-note join here.** Build the predicate it will use; stop there.
6. **Do not design the quotation resolver here.** Bound it, do not build it.
7. **Do not derive the declared increment set here.** That is option E, a step of its own.
8. **Do not renumber any check.** See 12(d).
9. **Do not retire `leading_slug`.** Its two remaining call sites are the documented pre-migration shim on record data and are correct.
10. **Do not delete the `src/plan/source.rs` membership check.** Section 12(b) explains why it survives, reclassified.
11. **Do not touch the `agent-scaffold next` defects, the plain-`validate` mode-000 inconsistency, the containment TOCTOU, or the `project_root_of_source` shared cause.** They are queued to the same step and diagnosed; they are not this design question and mixing them into it is how a step becomes unreviewable.

---

## 11. The edit surface

MEASURED against the prototype except where marked REASONED.

**Code, one file.**

- `src/workflow.rs`: the `observed_membership` helper, the widened `w5_problems` signature, the rule body, the `run_checks` call site, and the W5 doc comment at `:519-543` and `:526` which states the lexical rule verbatim. About 30 production lines.
- Tests, same file: **14 unit-test call sites of `w5_problems` need the new argument. Six of them also need a round-log fixture**, because they construct an increment-unit waiver with no log at all. MEASURED precisely: with the new rule and an empty log substituted at every test call site, 372 tests pass and exactly 6 fail, all six of them W5 unit tests. I then added a nine-line `observed_rounds` test helper and a fixture to each of the six, and the full suite returns 422 passing, 0 failing. The mis-scoped-waiver test keeps its exact assertion text.
- MEASURED: no generated const, no drift-guarded file, no golden fixture, and no integration test is affected by the code change. `cargo clippy --all-targets -- -D warnings` is clean.

**Rule text, three files that must move together.**

MEASURED: the ownership clause is byte-identical in `pack/instrument.md:11`, `AGENTS.md:147` and `.agents/AGENTS.reference.md:147`, and those are the only three prose statements of it in the tree (a fourth statement is the `src/workflow.rs` doc comment). The guard is `src/agents_md_drift.rs`, whose test `the_committed_scaffold_matches_a_fresh_render` fails if the three diverge. `Q-70`'s statement of this is correct and I verified it.

The change is one parenthetical inside one bullet: "(the increment's leading slug equals the step)" becomes a statement of the observed relation. Regeneration is `just scaffold-self`, whose second line runs `nix fmt` over the whole tree; the justfile documents this as intentional. REASONED, and flagged as an operational hazard rather than a design one: this repository is not formatter-clean at HEAD, so a `scaffold-self` run reflows unrelated files. The implementer should regenerate, then commit only the three rule files, leaving unrelated reflow uncommitted, which is what the `reviewer-reproducible-evidence` step's record shows was done before.

**Plan, additive only.** Two `[[step.increment]]` entries, two `[[step.waiver]]` entries, one status flip on `workflow-enforcement-tier`. This is planner work with its own receipt, not implementer work.

**Not touched:** `src/plan/source.rs` except for one comment (12(h)), `src/plan/render.rs`, `src/metrics.rs`, `src/next.rs`, `src/main.rs`, `pack/plan-template.plan.toml`, `src/isolation_policy.rs`, `src/workflow_spec.rs`, and every `.agents/` asset other than `AGENTS.reference.md`.

---

## 12. Rulings on every lettered duty

### (a) The coupling ruling, all three parts

**Part 1, do W5's ownership check and the waiver-note breakdown join share a mechanism? YES, in the join; NO, in the data. MEASURED, not argued.** The join both need is the `(step, increment)` relation derived from the round log. I demonstrated it by writing the note-breakdown probe against that relation and getting all four live breakdowns right on the first attempt (section 3.7). The data the note join additionally needs, `valid_findings` on `Round`, is not shared with W5 at all: W5 needs the pair, not the count.

**Part 2, does the project-identity edit queued by `Q-55-mechanism` share it? It shares the DATA half and not the JOIN half, and under option C it composes for free.** Project identity adds `project` to `Round` and to `[meta]` and filters the join in `check_workflow_toml`. `Round` is the same struct the note join widens, so those two couple to each other and should land as one edit, which is exactly what the recorded human constraint says. The filter half composes with C at zero cost and with a benefit: MEASURED by reading, `check_workflow_toml` computes `metrics::parse_rounds(log_contents)` at one site (`:189`) and hands it to `run_checks`, so a filter applied there is inherited by every check that reads `rounds`. Today W5 reads none, so an identity filter would not protect W5's ownership verdict. Under C it would. **Option C strictly improves the coverage of the queued identity edit rather than competing with it.**

**Part 3, the cross-pricing, which the item calls the deliverable the pass exists to obtain.** What the other mechanism costs under my direction:

- **Under C (recommended)**, the note join costs: one field on `Round`, one line in `parse_rounds`, a note-breakdown parser, and the comparison. The join is free because C built it. Total: one struct, one projection function, one new check. The identity edit costs what it already costs, minus the coverage gap C closes.
- **Under A (declared-set lookup)**, the note join costs everything it costs under C, plus the join, built separately, because a note breakdown must be compared against ROUND records and the declared set contains no rounds. A buys the note join nothing. Worse, A's widened `plan::Step` is a second data path that the note join does not use, so the two mechanisms end up reading different notions of which increments a step has.
- **Under B (schema rework)**, the note join must be rebuilt against the new waiver shape after the shape changes, so B forces an ordering (schema first, then join) and pays the join twice if it is built before the reshape.
- **Under D (substrate-scoped or retired)**, the worst case: the note field is TOML-only, so the note join would live on the TOML side while W5's ownership remnant lives on the JSONL side, maximising divergence. If the rule is retired outright, ownership for the note join has to be re-derived from scratch with no existing predicate to reuse.

A coupling verdict without this pricing would be half the ruling, and the pricing is the reason C wins rather than merely ties.

### (b) The authoritative-path ruling

**W5 owns waiver ownership. The `src/plan/source.rs:807` check survives, reclassified, and it is not a competing answer.**

The two checks are not two answers to one question, and once you see what each is for, the "opposite verdicts" stop being a contradiction and become a coverage report:

- `src/plan/source.rs:807-811` is an **internal cross-reference check**: does this waiver's `increment` name an increment THIS DOCUMENT declares? That is the same class as `Provenance.decisions` resolving to `[[question]]` ids in the same document, which `src/plan/source.rs:239-243` already documents as this file's own in-TOML-versus-out-of-TOML principle. It belongs to `validate --source`, it is TOML-only by nature, and it is correct.
- W5 is a **plan-to-log cross-reference check**: is the ownership this waiver asserts a fact about work that actually happened? That is the workflow invariant, it runs on both substrates, and it is where ownership belongs.

Under my recommendation `fx-A` is red on the source path and silent in W5: "you waived a real increment of this step, but the plan never declared it". That is a coherent and useful message and it is the right one. Today the same shape produces two problems that contradict each other on the declared case, which is the state the item measured.

Both survive; only one of them is the ownership authority; the disagreement disappears because the wrong one stops asserting ownership.

### (c) The direction and its edit surface

**Direction (iii), one of the item's named candidates, extended in two ways: the shared predicate is extracted so W3 and W5 consult one implementation, and the refusal splits into two fact-backed messages instead of one fabricated one.** The extension is what turns (iii) from a bug fix into the single-authority design; (iii) as stated would fix W5 while leaving W3's copy of the relation beside it.

The edit surface is section 11, MEASURED. The short form: one source file for the code, three drift-guarded files for one parenthetical of rule text, an additive plan edit. No generated const beyond the rule text, no golden fixture, no machine output contract.

**What W5 does on the Markdown substrate** (owed by direction (i), answered anyway because it is the strongest evidence for C): exactly what it does on TOML, because the round log is the same input on both. MEASURED in section 3.5: the shipped binary refuses a correctly-scoped Markdown+JSONL waiver at exit 1 and the prototype accepts it at exit 0. Direction (i) cannot give this answer at all, which is the reason it is not viable.

**What W5 does on the JSONL substrate** (owed by direction (iv), answered for the same reason): the ownership rule stays fully live and meaningful there, and rule 1 (the waiver's `step` must name a real Roadmap step) stays live there too, since `step` is a free string in a JSONL waiver record. Nothing is retired on either substrate. MEASURED and worth recording: there are currently **zero** `type:"waiver"` records in this repository's log, so that arm is exercised by tests and by the Markdown path, not by live data here.

### (d) The W6 disambiguation

Wherever this document says "the waiver-note join" or "mechanism 1", it means **the waiver-note breakdown check in `Q-70`'s scope**. I have used the token "W6" nowhere except when quoting the item. The other claimant is `Q-59`'s transition-legality check, MEASURED as the only other `W6` occurrence in the plan (line 1774).

**Recommendation on the collision: name, do not number, until one ships.** Neither check exists. Assigning a number to an unbuilt check is what created the collision, and assigning a different number to an unbuilt check merely moves it. Both should be referred to by name in the record from now on; the first one to actually ship takes `W6` and the other takes the next free letter-number at its own build time. This costs nothing today, resolves the collision by construction, and pre-decides neither check's fate. The three-way `w5`/`w6` overload inside this step's records (two check names plus two waiver ids) is not touched: the waiver-id sequence `-w1` to `-w6` is an established convention and is not the thing to change, as the item says.

### (e) The sub-decision ruling

**The `Q-55-<suffix>` ids are a legitimate sub-decision convention, not dangling receipts. This is answerable from evidence rather than deliberation, and the evidence is decisive.**

MEASURED (2026-08-12; the population moves, so re-measure rather than citing this): 63 distinct receipt `q_id` values, 70 registered questions, 41 receipts resolving to no registered question. **All 41 are suffixed. Zero plain `Q-<n>` receipts dangle. All 41 share the parent `Q-55`, which is registered.**

There is no ambiguous middle. If the convention were an accident, one would expect stray plain ids or suffixed ids with unregistered parents; there are none of either. The convention is uniform and it is one question's.

**What this changes about mechanism 2.** Stage one becomes: a receipt `q_id` resolves if it names a registered `[[question]]`, or if it is `Q-<n>-<suffix>` and `Q-<n>` is registered. Under that rule the detector's red list today is **empty**. So the item's recorded claim that "mechanism (2) is red today on the whole unregistered-receipt set" is true only under the reading this ruling rejects. Mechanism 2 is a regression guard, not a defect finder, and that materially changes its priority. This does not touch the human's decision to keep it in scope; it changes what the mechanism is, which is precisely what the item says this ruling would do.

### (f) The scope of mechanisms 2 and 3

**Both are BOUNDED by this pass, not designed by it.** Stated deliberately, as the item requires, rather than assumed.

- **Mechanism 2 is bounded because its design space is now empty.** The one open question was letter (e), and (e) is settled on measurement. What remains is a rule of two clauses and its tests. There is nothing left for a design pass to weigh, and inventing options for it would be deliberation theatre.
- **Mechanism 3 is bounded because its design space is real but disjoint.** It reads no workflow record, no waiver and no round; it shares no data, no join and no type with the ownership question. Its genuinely open questions are what counts as a live passage (constraint 3a, which section 3.8 shows firing on `Q-70` itself) and what escape expected tool output gets (constraint 3b, MEASURED). Both are interesting; neither is decidable from anything this pass has looked at.

**What would settle whether mechanism 3 should be designed rather than bounded:** a decision on whether the quotation resolver is a validator (runs in `validate`, must be sound, needs both escapes) or an advisory report (runs on request, may be noisy, needs neither). That is a one-question human call, and until it is made, designing the resolver means designing two different tools at once. I recommend putting that question to the human alongside this proposal, not resolving it here.

Bounds for the step, so "bounded" is not a way of saying nothing: mechanism 2 is one check reading W4's existing inputs, no new data source, no schema change. Mechanism 3 is not authorised for build by this pass at all, and it gets its own decision first.

### (g) The YAGNI boundary

Section 10, eleven items.

### (h) The comment-coverage ruling

**A documentation defect, narrowly, with a one-line fix; and the fix is not to widen the comment but to reclassify the check.**

The comment at `src/plan/source.rs:785-790` says the block's rules are moved from the round log's `check_record` waiver arm and the W5 pairing, closing "one behaviour, two data representations". That claim is true of the presence rules and of the pairing. It is not true of the membership check at `:807`, which has no `check_record` ancestor (`check_record` has no access to a step's declared increments) and no W5 counterpart. So the comment's enumeration of its own provenance is incomplete, and a reader who trusts it will conclude the block contains no rule of its own, which is false.

It is a documentation defect rather than a design divergence because the check itself is correct and belongs there, per ruling (b). The fix is one clause naming `:807` for what it is: an internal cross-reference of the same class as `Provenance.decisions`, which this file already documents at `:239-243`. That single-sources the explanation instead of adding a second one.

It should land on the same commit as the W5 change, because the two comments must agree about which check owns ownership. That is the only reason `src/plan/source.rs` appears in my edit surface at all.

---

## 13. What I did not settle

- I did not verify all fifteen `src/checks.rs` citations, only five (section 3.8). The claim is the item's and is not load-bearing for this proposal.
- I did not measure whether the `workflow-driver` case would fire if that step were flipped to `complete`. It is latent by the W3 status skip and my recommendation cures it either way, but a verification pass on the fix should include a fixture that flips it, because it is the one member of the affected population whose increments the plan HAS declared.
- I have taken no position on whether `[[step.increment]]` should exist at all in its current hand-maintained form. Option E depends on that question and I have flagged it rather than answered it. What would settle it: a decision on what a declared increment is FOR, given that MEASURED, `complete` steps exist that declare none while their round records carry increment ids.
- I have not costed the note-breakdown parser itself, only its join. The convention is prose with at least two phrasings across four instances (section 3.7), so the parser's real cost is deciding whether to tighten the convention or to parse loosely, and that belongs with mechanism 1's own build.

---

## Appendix: reproduction

Every measurement above, with the command that produced it. Run from the repository root unless stated. Fixtures were built under a scratch subdirectory and nothing outside it was written or deleted.

**The affected population (section 3.1)**

```
jq -r 'select(.type=="round") | [(.step // (.task|sub("-inc[a-zA-Z0-9]+$";""))), (.increment // .task)] | join(" ")' docs/metrics/workflow.jsonl | sort -u | awk '{lead=$2; sub(/-inc[a-zA-Z0-9]+$/,"",lead); if (lead != $1) print $1, $2}'
```

**Declared set versus log identities (section 3.2)**

```
jq -r 'select(.type=="round") | (.increment // .task)' docs/metrics/workflow.jsonl | sort -u > log_ids.txt
grep -A1 '^\[\[step.increment\]\]' docs/plans/agent-scaffold.plan.toml | grep '^id = ' | sed 's/id = "//; s/"$//' | sort -u > declared_ids.txt
comm -23 log_ids.txt declared_ids.txt | wc -l
comm -13 log_ids.txt declared_ids.txt
```

Subtract the `[meta].orphan_tasks` tokens from the first difference to get the 37 step-owned undeclared identities.

**The fixture matrix (sections 3.3, 3.4)**

Copy `docs/` to a scratch directory, then per fixture:

- `fx-A`: add a `[[step.waiver]]` with `id = "workflow-enforcement-tier-w5"`, `unit = "increment"`, `increment = "workflow-enforcement-tier-fold"`, `reason = "accepted-at-escalation"`, `evidence_tier = "record-backed"`, `evidence = "workflow-enforcement-tier-fold"`, nested under the `workflow-enforcement-tier` step.
- `fx-B`: `fx-A` plus `[[step.increment]] id = "workflow-enforcement-tier-fold"`, `risk_class = "risky"`.
- `fx-C`: base plus a waiver naming `increment = "totally-not-a-step-inc1"`, `reason = "review-skipped"`, `evidence_tier = "self-declared"`.
- `fx-D`: `fx-B` plus the `-endproperty-fold` increment and its `-w6` waiver, plus `status = "complete"` on the step.

Then, for each:

```
cargo build
./target/debug/agent-scaffold validate --source <fixture>/docs/plans/agent-scaffold.plan.toml --workflow ; echo $?
```

**The prototype (section 3.4)**

Copy the tree (excluding `.git` and `docs`) to scratch. In `src/workflow.rs`: add `observed_membership(rounds) -> BTreeSet<(&str, &str)>` mapping each round to `(round_step_slug(round), round_increment_id(round))`; give `w5_problems` a `rounds` parameter and pass `rounds` from `run_checks`; replace the `leading_slug(increment) != waiver.step` branch with `!observed.contains(&(waiver.step.as_str(), increment))`, reporting the observed step when the increment appears elsewhere and "has no round records" when it appears nowhere. Add an `observed_rounds` test helper and a fixture to the six W5 tests that construct an increment waiver with no log. Then:

```
cargo build && cargo test && cargo clippy --all-targets -- -D warnings
```

422 tests pass, clippy clean. Re-run the fixture matrix against `target/debug/agent-scaffold` in the patched tree.

**The Markdown substrate (section 3.5)**

Build a scratch project with `docs/plans/demo.md` (a Roadmap declaring `alpha` and `beta` both `complete`, an Open Questions section, and a Step Detail heading per step) and `docs/metrics/workflow.jsonl` carrying: two `round` records for `task` `beta-fold` with structured `step` `beta` reaching `consecutive_clean` 1 at `risky`, one `escalation` for `beta-fold` with `human_decision` `decision`, and one `type:"waiver"` record with `unit` `increment`, `step` `beta`, `increment` `beta-fold`, `reason` `accepted-at-escalation`, `evidence_tier` `record-backed`, `evidence` `beta-fold`. Then:

```
<shipped>/agent-scaffold validate --plan <fixture>/docs/plans/demo.md --workflow ; echo $?
<patched>/agent-scaffold validate --plan <fixture>/docs/plans/demo.md --workflow ; echo $?
```

Exit 1 then exit 0.

**Dangling receipts (section 3.6)**

```
jq -r 'select(.type=="decision") | .q_id' docs/metrics/workflow.jsonl | sort -u > receipts.txt
grep -A1 '^\[\[question\]\]' docs/plans/agent-scaffold.plan.toml | grep '^id = ' | sed 's/id = "//; s/"$//' | sort -u > questions.txt
comm -23 receipts.txt questions.txt > dangling.txt
grep -cE '^Q-[0-9]+-' dangling.txt      # every one is suffixed
grep -cE '^Q-[0-9]+$' dangling.txt      # zero plain ids dangle
sed 's/^\(Q-[0-9]*\)-.*/\1/' dangling.txt | sort -u | comm -23 - questions.txt   # empty: every parent is registered
```

**The note-breakdown probe (section 3.7)**

```
grep -onE "[(][0-9]+(, [0-9]+){1,6}[)]" docs/plans/agent-scaffold.plan.toml
```

Four sites. The probe script joins each waiver note's `<total> valid findings[ in scope] (<r1>, ...)` breakdown to the round records whose `(round_step_slug, round_increment_id)` pair equals the waiver's `(step, increment)`, and compares the per-round `valid_findings`. All four agree.

**Corrections (section 3.8)**

```
grep -c 'slug = "totally-not-a-step"' docs/plans/agent-scaffold.plan.toml     # 1, the item quoting itself
grep -c '^slug = "totally-not-a-step"$' docs/plans/agent-scaffold.plan.toml   # 0, no such step
grep -rlF 'increment waiver names step `workflow-enforcement-tier` but increment' . --exclude-dir=.git --exclude-dir=target
```

The last returns the plan TOML and its rendered projection only, no source file.

**Rule-text sites (section 11)**

```
grep -rc "must own its" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md README.md
```

One each in the first three, zero in the README. The fourth statement is the `src/workflow.rs` doc comment.

**`leading_slug` call sites (section 2)**

```
awk '/^mod tests/{exit} /leading_slug\(/ {print NR": "$0}' src/workflow.rs
```

Three real call sites before the change; two after.
