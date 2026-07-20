# Build plan: `task-entry-regrounding` (Q-53)

Read-only design pass. This document is the build plan for the decided step `task-entry-regrounding`. It is a plan, not an implementation. The single write of this pass is this file.

## Scope lock (the SSOT for WHAT)

From `docs/plans/agent-scaffold.steps/task-entry-regrounding.md:1-3` and the `Q-53` block at `docs/plans/agent-scaffold.plan.toml:1047-1051`, the step has two parts:

- Part A (prose, LOW risk): a LIGHT, stakes-scaled TASK-ENTRY RE-GROUNDING and human alignment check, run before starting a step and on resume. The orchestrator writes a short brief (what the task is, why it exists, the cited evidence INCLUDING the decision receipt with the human's recorded choice, and what it is about to do) and pushes it for a go/no-go via the human-input contract, scaled to stakes. Sourced ONLY from durable artifacts (plan Step Detail by slug, ledger RESUME STATE, decision receipts by `q_id`, findings by path, code by `file:line`/commit), NEVER conversation history.
- Part B (structured, RISKY): optional PER-STEP PROVENANCE links (a step -> the decisions/findings/commits that justify it), mirroring questions' `folded_into`/`receipt`, which `render` presents as "why this step exists".

Chosen over no-regrounding and over out-of-VCS history pointers. Complements `workflow-driver` (`Q-51`), which its own design pass found should complement rather than subsume this step.

Grounding note: the live plan is already TOML-primary (`docs/plans/agent-scaffold.plan.toml:4` `primary = "toml"`), so a schema change in Part B is load-bearing for the live repo's own `render`/`validate --source`/`status`/`next` path, not a dormant schema.

## Part A design (prose discipline, low_risk)

### Where it goes

Part A adds ONE new discipline to the pack guidance and ONE pointer in the orchestrator prompt. It must REFERENCE, not restate, the existing machinery (Principle 8, one source of truth), exactly as `Q-54` did (`git show 99c5c84`: it rewrote a single orchestrator-prompt paragraph to reference the AGENTS.md human-input contract, touching no other file's copy of the rule).

Recommended home: a new short subsection in `pack/AGENTS.md` titled "Task-entry re-grounding", placed immediately AFTER the "Preflight (establish adherence before starting or resuming)" section (`pack/AGENTS.md:102`).

Reasoning:

- The nearest kin is Preflight: both are "establish adherence before proceeding" gates that confirm with the human via the human-input contract. Preflight is per-SESSION (before the workflow starts, and on resume; `pack/AGENTS.md:102`). Task-entry re-grounding is finer-grained, per-STEP-ENTRY (before starting a step, and on resume), so it reads as the step-scoped sibling of the session-scoped Preflight, not a replacement for it. Placing it next to Preflight lets it reference Preflight and the "Checkpoint and resuming after context loss" section (`pack/AGENTS.md:95-100`) for the durable-artifact reconstruction rather than re-describing it.
- The durable-artifact list it draws from is already enumerated in "Checkpoint and resuming" (plan, ledger RESUME STATE, plan Status line) and in the round-log receipt spec (`type:"decision"` with `q_id`/`options`/`chosen`, `pack/AGENTS.md:141`). The new subsection cites those, so the "NEVER conversation history" rule and the receipt-with-recorded-choice requirement are single-sourced.

### Exact prose approach (what the subsection says, referencing not restating)

The subsection states, in a few sentences:

1. Trigger: before the orchestrator starts a Roadmap step (the phase-4 step-entry, `pack/AGENTS.md:31` phase 4), and on resume after reconstructing state.
2. The brief: what the task/step is, why it exists (its provenance), the cited evidence, and what the orchestrator is about to do. Every cited fact comes from a durable artifact named by a stable handle: the plan Step Detail by slug, the ledger RESUME STATE, a decision receipt by `q_id` (the `type:"decision"` receipt with the human's `chosen`, `pack/AGENTS.md:141`), a finding by path, code by `file:line` or commit. A fact available only in conversation history is not citable; it is a durability bug to flush into an artifact first (state this once, referencing the "Checkpoint and resuming" section which already establishes that working context is not durable, `pack/AGENTS.md:95`).
3. The gate: push the brief for a go/no-go "per the human-input contract" (link, do not restate; `pack/AGENTS.md:41`), scaled to stakes exactly as the human-input contract already prescribes ("the full structure for a real decision, a one-line recommendation and reason for a trivial confirmation", `pack/AGENTS.md:41`). This is the match-ceremony-to-stakes discipline the step itself embodies, so the subsection must NOT build its own separate ceremony (a low-stakes step entry is a one-line brief and an implicit go; a high-stakes step entry is the full brief and an explicit go/no-go).
4. Relationship to the existing checkpoint queue-push: a step boundary is already a checkpoint where the orchestrator pushes open items (`pack/AGENTS.md:71`, and the orchestrator prompt at `pack/prompts/orchestrator.md:25`). Re-grounding is the ENTRY-side counterpart (before starting the next step) to that EXIT-side push; the subsection says so and references it rather than duplicating the queue-push rule.

### Orchestrator-prompt pointer

Yes, add a short per-step-entry pointer, mirroring `Q-54`'s single-paragraph edit. Home: the "Implement step by step" paragraph (`pack/prompts/orchestrator.md:23`) or the checkpoint paragraph (`pack/prompts/orchestrator.md:25`). The pointer says: before starting each step, run the task-entry re-grounding in `AGENTS.md` (brief from durable artifacts + go/no-go per the human-input contract, scaled to stakes), referencing the AGENTS.md subsection rather than restating it. Keep it to one or two sentences, as `Q-54` did (the diff was a single reworded paragraph).

### Part A files

- `pack/AGENTS.md` (authored source): add the "Task-entry re-grounding" subsection after Preflight.
- `pack/prompts/orchestrator.md` (authored source): add the per-step-entry pointer.
- Dogfood instances, regenerated/synced from the pack, NOT hand-authored twice: the root `AGENTS.md` is GENERATED from `pack/AGENTS.md` (main.rs:9 "`AGENTS.md` is generated by rendering the selected principles into the" pack template; `{{principles}}`/`{{instrument}}`/`{{modules}}` placeholders at `pack/AGENTS.md:110-114`), and `.agents/prompts/orchestrator.md` is the synced dogfood copy of `pack/prompts/orchestrator.md` (the justfile keeps `AGENTS.md`, `.agents/`, and the plan template in sync with the pack, `justfile:37-39`; `Q-54` committed both `pack/prompts/orchestrator.md` and `.agents/prompts/orchestrator.md`). The implementer edits the pack source, then runs the justfile dogfood-sync recipe to regenerate `AGENTS.md` and `.agents/prompts/orchestrator.md`, and commits all of them together. Do NOT hand-edit the generated `AGENTS.md`.

## Part B design (structured provenance: schema / validation / render)

### Field shape on `Step`

Recommended: a single optional `provenance` sub-struct on `Step`, holding three optional string lists.

```
// on Step, declared AFTER folds and BEFORE increments/waivers (see ordering note):
#[serde(default, skip_serializing_if = "Option::is_none")]
pub(crate) provenance: Option<Provenance>,

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Provenance {
    /// The Open-Questions decisions (`Q-<n>`) that justify this step.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) decisions: Vec<String>,
    /// The findings artifacts (task-relative paths) that justify this step.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) findings: Vec<String>,
    /// The commits (hashes) that justify this step.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) commits: Vec<String>,
}
```

Reasoning, judged against the conventions already in `source.rs`:

- One grouped sub-struct mirrors the `Increment`/`Waiver`/`Sidecars` pattern (nested struct with `#[serde(deny_unknown_fields)]`, at `source.rs:88-97`, `:223-230`, `:235-258`). `deny_unknown_fields` on `Provenance` makes a mistyped inner key (for example `decisons`) a loud parse error, matching the plan.toml's deliberately strict stance (`source.rs:355-358`, `source.rs:1122-1134`). Three loose top-level `decisions`/`findings`/`commits` fields on `Step` would work too (they would serialize as inline arrays like `blocked_by`/`folds` and avoid the ordering note below), but they clutter the `Step` namespace and lose the "this is provenance" grouping; the sub-struct is the cleaner mirror of the existing nested-struct precedent.
- Optional and back-compatible: `Option<Provenance>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` mirrors `Waiver.increment`/`evidence`/`note` (`source.rs:244`, `:253`, `:256`) and `Meta.tail`/`w4_baseline`. A step with no `provenance` deserializes to `None`, serializes to nothing, and renders byte-identically to today. That is the back-compat guarantee: every existing step still validates and renders unchanged.
- Typed vs freeform: the three lists are `Vec<String>` (freeform strings), because a `q_id` is validated by resolution not by type, a commit hash and a findings path are external references that cannot be an enum. This matches how `blocked_by`/`folds`/`receipt` are `String`/`Vec<String>` resolved or shape-checked in `validate_source`, not typed at parse.

Ordering note (round-trip correctness): `toml::to_string` emits struct fields in declaration order, and the round-trip test re-parses the output (`source.rs:804-808`). The `Step` doc comment already warns that "Scalar and value keys come before the array-of-table keys (`increment`/`waiver`)" (`source.rs:126-128`). `provenance` serializes as a `[step.provenance]` sub-table (its inner lists are inline arrays, not arrays-of-tables), so it MUST be declared after the inline-value fields (`blocked_by`, `folds`) and BEFORE the arrays-of-tables (`increments`, `waivers`); otherwise a `[step.provenance]` emitted after `[[step.increment]]` would bind to the wrong table and the round-trip would break. Declare the field between `folds` (`source.rs:145-146`) and `increments` (`source.rs:148-149`). The round-trip test must exercise a step whose provenance sub-table sits alongside increments and waivers to pin this.

### Validation strictness (the main design decision)

Recommended split, mirroring exactly how `validate_source` already treats in-TOML vs out-of-TOML references:

- `decisions` (q_ids): RESOLVE each to a real `[[question]]` id (fail-closed). A `decision` naming no question in the plan is a build error, the same class as `superseded_by` resolving to a real question (`source.rs:582-589`) and `folded_into` resolving to a real step (`source.rs:573-580`). The q_id target lives in the SAME TOML, so it can and should be resolved (Principle 14 parse-don't-validate, Principle 12 fail-loud: a dangling justification link is exactly the kind of correctness-bearing cross-reference the plan.toml is strict about, `source.rs:355-358`). Also require each `decisions` entry to be `Q-<n>`-shaped via `question_id_index` before resolution, so the error distinguishes a malformed id from a well-formed-but-absent one (mirror `source.rs:511-513`).
- `commits` (hashes): FREEFORM, shape-checked only, NOT resolved. A commit hash is a git object external to the TOML; `validate_source` is a pure function over the TOML string (`source.rs:474-478`) and never shells out, so it cannot and must not resolve a hash against a repo. Shape check: non-empty, lowercase hex, length in a sane range (for example 7 to 40 chars). This is the same posture as `receipt`, which only shape-checks `Q-<n>` and does NOT confirm the JSONL receipt exists (`source.rs:624-631`), precisely because the receipt target is out-of-TOML.
- `findings` (paths): FREEFORM, shape-checked only, NOT existence-checked. Reuse the `is_safe_sidecar_ref` rule (`source.rs:447-453`): a task-relative path, no absolute root, no `..` component (Principle 18 least authority, Principle 21 validate at the boundary). Do NOT check the file exists: a findings file is committed then deleted as a committed deletion at task close (`pack/AGENTS.md` findings-file lifecycle; `pack/prompts/orchestrator.md:7`), so a valid historical pointer may name a path not present on the current disk. Shape-check the reference, do not stat it.

This split is the same one the codebase already draws: in-TOML targets (`folded_into`, `superseded_by`, `blocked_by`, `folds`) resolve; out-of-TOML targets (`receipt`, sidecar refs) shape-check. Provenance decisions are in-TOML (resolve); commits and findings are out-of-TOML (shape-check).

Additional consistency check (recommended, low cost): if `provenance` is present, at least one of the three lists must be non-empty, so an empty `[step.provenance]` (a meaningless block) is flagged rather than silently accepted. Optional; flag if the reviewer wants illegal-states-unrepresentable (Principle 13) taken to its conclusion.

### Render presentation ("why this step exists")

Mirror how questions render `receipt`. A question renders its `receipt` inline on the one-line queue entry (`render.rs:431-435`: ` Receipt: \`Q-2\`.`), and `folded_into`/`superseded_by` inline in the status display (`render.rs:444-453`). The step's scannable one-line analog is the Roadmap table row's Notes cell, which ALREADY renders `blocked_by` and waivers from the TOML (`render.rs:475-484` `notes_cell`, joined with `; `, escaped by `escape_cell` at `:511-513`).

Recommended: append a provenance fragment to the Notes cell, LAST (after the `blocked on <slug>` markers and the waiver descriptors), of the form:

```
why: decisions Q-53; findings docs/plans/x.md; commits abc1234
```

with only the sub-lists that are present shown, decisions then findings then commits, each in source order (already deterministic, a `Vec`). Use a sub-separator that reads cleanly inside the `; `-joined cell (for example the label form above), and rely on the existing `escape_cell` to neutralise any `|`/newline in a path or note. This is the faithful mirror: provenance is TOML-derived, and the Notes cell is the render's home for TOML-derived per-step facts (blocked_by, waivers), exactly as the queue line is the home for the question's TOML-derived facts.

Determinism and `render --check`: the Notes cell is already deterministic and byte-compared by `render --check` (`render.rs:256-279`), so the provenance fragment is pinned automatically once the golden includes a provenanced step. No new mechanism.

Alternative (flag, do not build unless the human prefers it): a dedicated generated "why this step exists" line under each step in `## Step Details`. Rejected as the default because `## Step Details` is opaque sidecar prose spliced verbatim (`render.rs:519-530`), not generated-from-TOML content; mixing a generated line into it breaks the clean split (sidecars are authored, the table is generated). The Notes cell keeps generated content in the generated region.

### W-check involvement: NO

Provenance is source-only, like `blocked_by`/`folds`. The enforcement checks read `step_views` (`source.rs:381-389`), which projects ONLY `slug` and `status`; `blocked_by`/`folds` are never projected and never read by W3/W4/W5 (confirmed: `src/workflow.rs:187` consumes `step_views`; `src/main.rs:1009` `steps: source.step_views()`). The `next` projection reads `slug`/`order`/`status`/`blocked_by` only (`src/next.rs:372-382` `steps_from_toml`), and ignores any new `Step` field. Adding `provenance` touches no W-check and no `next` behaviour. This is a stated fact, not a design decision for the human.

## Staging: two increments

Recommended: TWO increments.

- `task-entry-regrounding-inc1`: Part A prose discipline (pack/AGENTS.md subsection + orchestrator-prompt pointer + dogfood sync). `risk_class = "low_risk"`.
- `task-entry-regrounding-inc2`: Part B schema + validation + render (source.rs field + validate rules, render.rs Notes fragment, fixtures/goldens, exemplar back-population). `risk_class = "risky"`.

Reasoning (judged against match-ceremony-to-stakes, the discipline the step itself embodies): the two parts touch disjoint substrates (Markdown guidance vs Rust parse/validate/render with round-trip and golden implications) and carry different risk. Splitting lets each converge under its own risk-class clean-round bar (the `Increment.risk_class` sets the required streak, `source.rs:225-230`), so the low-risk doc change is not held to the risky code change's bar and vice versa. This mirrors the existing multi-increment steps (for example `structured-skeleton-inc1..inc6`). The increment ids follow the kebab-case `<slug>-inc<n>` convention validated at `source.rs:434-436`.

## Dogfood / back-population decision

Recommended: ship the CAPABILITY (schema + validation + render) as the deliverable, and populate a COUPLE of exemplar live steps to prove it renders end-to-end; do NOT mass-migrate the ~56 existing steps.

- Exemplars: populate `task-entry-regrounding` itself with `decisions = ["Q-53"]` and `human-input-gate-reinforce` with `decisions = ["Q-54"]` (both already carry `folded_into`/`receipt` on their questions at `docs/plans/agent-scaffold.plan.toml:1047-1058`, so the decision q_ids resolve). Optionally add a `commits`/`findings` entry on one exemplar to exercise all three lists on the live plan.
- Because the live plan is TOML-primary (`docs/plans/agent-scaffold.plan.toml:4`), populating exemplars means inc2 must re-run `render` and commit the regenerated `docs/plans/agent-scaffold.md` so `render --check` stays green. This is the real-data proof that the Notes fragment renders.
- The render GOLDEN test itself uses the render fixture (`src/plan/testdata/render-fixture.*`), not the live plan; the live exemplars are the dogfood proof, the fixture is the pinned unit test.

Flag to the human: the SCOPE of back-population (two exemplars now vs a broader sweep). Mass-migration of 56 steps is out of scope for this step (YAGNI); it can be a separate follow-up if the human wants full coverage.

## Test plan

All mirror existing tests in `source.rs`/`render.rs`.

Schema (source.rs tests):

- Round-trip incl provenance: a step with a populated `[step.provenance]` (all three lists) parses, and serialize -> re-parse yields an equal document, with the provenance sub-table declared alongside increments/waivers so the ordering constraint is pinned (mirror `source.rs:761-809`).
- `deny_unknown_fields` on `Provenance`: a mistyped inner key (for example `decisons`) fails to parse and surfaces as `malformed` (mirror `source.rs:1122-1134`).
- Back-compat: a step WITHOUT provenance parses, validates clean, and serializes without any `provenance` key (mirror `source.rs:1136-1160` unexercised-variants round-trip; assert the absence of `provenance` in the re-serialized output).

Validation (source.rs tests):

- Dangling decision: a `decisions = ["Q-99"]` naming no question is flagged (mirror `a_dangling_superseded_by_is_flagged`, `source.rs:912-920`).
- Malformed decision id: `decisions = ["nope"]` flagged as not a `Q-<n>` id (mirror `source.rs:511-513`).
- Bad commit shape: a non-hex or over-length `commits` entry flagged.
- Unsafe findings path: a `findings = ["../escape.md"]` and an absolute `findings = ["/etc/x"]` flagged, reusing `is_safe_sidecar_ref` (mirror `source.rs:923-947`).
- Clean case: a fully-populated, well-formed provenance validates clean (mirror `the_fixture_skeleton_validates_clean`, `source.rs:811-814`).
- (If the non-empty rule is adopted) an empty `[step.provenance]` flagged.

Render (render.rs tests):

- Golden for a provenanced step: add `provenance` to one render-fixture step (for example `alpha` at `src/plan/testdata/render-fixture.plan.toml:19`), regenerate `src/plan/testdata/render-fixture.md`, and assert the `why: ...` fragment appears in that step's Notes cell (extend `the_rendered_document_carries_every_generated_fragment`, `render.rs:661-715`).
- Determinism/`render --check` stability: the existing `render_is_deterministic_and_matches_the_golden` (`render.rs:643-651`) and `the_committed_golden_passes_check` (`render.rs:653-659`) cover it once the fixture carries provenance; no new mechanism.
- Unchanged-without-provenance: the fixture steps that carry NO provenance must render byte-identically (the golden diff for those rows is empty), proving the back-compat guarantee at the render layer.

Toolchain: `just test` (the justfile handles the Nix env; `just` recipes per the global preference). Also run `just` dogfood-sync + `render --check` after inc1 (AGENTS.md regen) and inc2 (live exemplar re-render) so the generated `AGENTS.md` and `docs/plans/agent-scaffold.md` stay in sync.

## Files to add / change

Part A (inc1):

- `pack/AGENTS.md`: add "Task-entry re-grounding" subsection after Preflight (`:102`).
- `pack/prompts/orchestrator.md`: add the per-step-entry pointer (near `:23`/`:25`).
- Regenerated/synced (not hand-authored): root `AGENTS.md`, `.agents/prompts/orchestrator.md` (via the justfile dogfood recipe).

Part B (inc2):

- `src/plan/source.rs`: add `Provenance` struct + `Step.provenance` field; add the validation arm (resolve decisions, shape-check commits/findings); add schema + validation unit tests.
- `src/plan/render.rs`: extend `notes_cell` (`:475-484`) with the provenance fragment; add/extend the render unit test.
- `src/plan/testdata/render-fixture.plan.toml` + `src/plan/testdata/render-fixture.md`: add provenance to one fixture step and regenerate the golden.
- `docs/plans/agent-scaffold.plan.toml`: add exemplar `provenance` to `task-entry-regrounding` and `human-input-gate-reinforce`; declare the two `[[step.increment]]` entries with their risk classes.
- `docs/plans/agent-scaffold.md`: regenerated via `render` (committed with the source).
- `docs/plans/agent-scaffold.steps/task-entry-regrounding.md`: no schema change needed; the step body may note the shipped capability, but it is opaque sidecar prose and not required for the feature.

## YAGNI boundary (what NOT to build)

- NO new W-check. Provenance is source-only; W3/W4/W5 and `next` never read it.
- NO history-sourced anything. The brief and the provenance cite only durable artifacts by stable handle; conversation-history pointers were rejected in the decision (`docs/plans/agent-scaffold.steps/task-entry-regrounding.md:3`).
- NO git resolution of commit hashes and NO filesystem existence check of findings paths. `validate_source` stays a pure function over the TOML string; commits and findings are shape-checked only.
- NO mass back-population of the ~56 existing steps. Two exemplars prove the render; broader coverage is a separate optional follow-up.
- NO heavy re-grounding ceremony. The discipline is one short subsection that references the human-input contract and the checkpoint/preflight machinery; it must not restate them (Principle 8) or invent its own escalation/gating apparatus. Ceremony scales to stakes.
- NO new struct typing for commit/findings/decision beyond `Vec<String>`; NO `render_sha256` reconciliation (out of scope, still vestigial per `render.rs:24-28`).

## Design decisions for the human (relay these through the human-input contract)

Each is presented with options, trade-offs, a recommendation, and Principle-judged reasoning, so the orchestrator can relay them.

### D1. Validation strictness of provenance references

- Options: (a) decisions RESOLVE to real questions, commits + findings shape-checked only [recommended]; (b) all three shape-checked only (no resolution); (c) all three fully resolved (would require git + filesystem access in `validate_source`).
- Trade-offs: (a) catches a dangling decision link at build time while keeping `validate_source` pure; (b) simplest, but lets a decision typo pass silently; (c) strongest, but couples validate to a git repo and the working tree, which it never does today.
- Recommendation: (a).
- Reasoning: mirrors the split the code already draws, in-TOML targets resolve (`folded_into`/`superseded_by`, `source.rs:573-589`), out-of-TOML targets shape-check (`receipt`, `source.rs:624-631`); Principle 14 (parse, don't validate) and 12 (fail-loud) for the in-TOML decision link; Principle 18/21 (least authority, validate at the boundary) for keeping validate pure and shape-checking the external commit/findings refs.

### D2. Back-population scope

- Options: (a) ship the capability + populate 2 exemplar live steps [recommended]; (b) ship the capability only, no live population; (c) capability + full sweep of all ~56 steps.
- Trade-offs: (a) proves end-to-end render on the live TOML-primary plan at near-zero cost; (b) leaves the feature unexercised on real data; (c) large, mostly-mechanical, and much of the history predates the artifacts to cite.
- Recommendation: (a).
- Reasoning: Principle 6 (evidence over assertion), a rendered exemplar is proof; Principle 11 (test the real path). Full sweep is YAGNI at this stage and can be a later step if wanted.

### D3. Render placement of "why this step exists"

- Options: (a) append to the Roadmap Notes cell, mirroring `blocked_by`/waivers and the question queue line [recommended]; (b) a dedicated generated line under `## Step Details`.
- Trade-offs: (a) keeps generated content in the generated region and is the faithful mirror of how questions render `receipt`; can lengthen a Notes cell; (b) more room, but injects generated text into the opaque authored sidecar region, breaking the generated-vs-authored split.
- Recommendation: (a).
- Reasoning: Principle 16 (one source of truth / one home for generated content); the Notes cell is already the render's home for per-step TOML-derived facts (`render.rs:475-484`), and the scope says "mirror how questions render `folded_into`/`receipt`", which is inline on the scannable line.

### D4. Field shape (grouped sub-struct vs flat lists)

- Options: (a) one `provenance` sub-struct with three optional lists [recommended]; (b) three flat optional `Step` fields `decisions`/`findings`/`commits`.
- Trade-offs: (a) groups related fields and gets `deny_unknown_fields` typo-catching, matching the `Increment`/`Waiver`/`Sidecars` precedent, but requires the sub-table to be declared before the arrays-of-tables for round-trip validity; (b) serializes as inline arrays like `blocked_by`/`folds` and sidesteps the ordering note, but clutters the `Step` namespace and loses the grouping.
- Recommendation: (a).
- Reasoning: Principle 13 (illegal states unrepresentable) via `deny_unknown_fields`; Principle 16 (one consistent structural convention). This is the lowest-stakes of the four and could be delegated to the implementer/reviewer rather than the human if the orchestrator prefers to keep the human's queue short.

### D5 (optional). Reject an empty `[step.provenance]` block

- Options: (a) flag an all-empty provenance as invalid [recommended if taking Principle 13 to its conclusion]; (b) accept it silently.
- Recommendation: (a), low cost.
- Reasoning: Principle 13 (illegal states unrepresentable), a present-but-empty provenance carries no meaning. Minor; the orchestrator may fold this into D4 rather than raising it separately.
