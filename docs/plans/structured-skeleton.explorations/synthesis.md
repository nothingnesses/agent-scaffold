# Structured-skeleton synthesis (Q-44 / Q-45 option B)

Synthesis of designs A (minimal-schema), B (rich-schema), and C (migration-safety) against the clean-slate target (`target-arch-B-cleanslate.md`) and the grounded repo state. This feeds a planning pass and a human decision batch; it does not re-derive the designs. One divergence is load-bearing (section 2) and is the human's to decide; the rest are a crisp decision batch (section 3).

Grounding checked against the code and artifacts (2026-07-18):

- `docs/metrics/workflow.jsonl` holds 113 records: 93 `round`, 16 `waiver` (11 `predates-logging`, 3 `review-skipped`, 2 `accepted-at-escalation`), 2 `escalation`, 1 `decision`, 1 `baseline` (`questions_through = Q-44`). Only Q-45 is a decided item strictly after the baseline, so exactly one receipt is required by W4 today.
- `src/plan.rs` `ROADMAP_STATUSES` has already RETIRED `trivial` and `grandfathered` (tests pin their rejection); `QUEUE_FOLD_PREFIX = "decided -> folded into "` is still a parametric status string; `parse_roadmap` / `parse_questions` still parse Markdown regions.
- `src/workflow.rs` implements W3 (convergence-OR-waiver, incl. the pause.md catch), W4 (receipt vs declared baseline cutoff), W5 (waiver integrity), and the round-log consistency check; `leading_slug` still does the lexical `-inc<x>` strip.
- `pack/instrument.md` documents the shipped `waiver` / `decision` / `baseline` record schemas.
- The live plan `docs/plans/agent-scaffold.md` is 750 lines: 51 Roadmap rows, 45 `Q-<n>` queue items.

The shared, load-bearing finding: the enforcement substrate (receipts, unified waivers, baselines, W3/W4/W5) already shipped in pilots 1 and 2 and lives in the append-only JSONL. So this initiative is NOT "add structure"; it is skeleton relocation (the Roadmap + queue + plan prose move from parsed Markdown to a TOML skeleton plus opaque Markdown sidecars) plus a new `render` engine. The 113 JSONL records and the enforcement DATA do not move under the recommended path, so the migration cannot break an enforcement check mid-flight.

---

## 1. The agreed core (settled baseline)

All three designs (or a clear majority) converge here. Treat these as decided unless a human objects.

### Substrate and boundary

- Two substrates, disjoint roles. `docs/plans/<task>.plan.toml` is the LIVE, mutable skeleton (steps, queue, meta). `docs/metrics/workflow.jsonl` is the APPEND-ONLY event log (rounds, escalations, decisions, intakes, dismissals, and under the recommended path also waivers and the baseline). The TOML subsumes no JSONL record type; the overlap is one-directional (the plan asserts a state; the log carries the evidence W3/W4/W5 check it against). This is the same two-homes-one-check shape that exists today; only the plan side moves from parsed Markdown to typed TOML.
- The 113 JSONL records are never rewritten. Append-only is preserved throughout.

### The TOML skeleton (fields all three agree on)

- `[meta]`: `task` and `title` (the rendered `# <title> plan` heading). The Status line is DERIVED by render, never a stored field.
- `[[step]]`: `slug` (kebab, stable, unique), `title` (one line), `status` (from the cleaned `ROADMAP_STATUSES` set: `not started`, `in progress`, `complete`, `skipped`, `next`, `optional`, `deferred`). "blocked" stops being a status value and becomes a structured `blocked_by` slug field that render displays as `blocked on <slug>`.
- `[[question]]`: `id` (`Q-<n>`), `ask` (one line only; the multi-KB body moves to a sidecar, closing SE-15), `status` as a clean enum (`open`, `exploring`, `decided`, `superseded`), and `folded_into` as a SEPARATE slug field required exactly when `status == "decided"`. This retires the parametric `decided -> folded into <slug>` prefix (SE-11) with no parsing convention.
- Typed cross-references replace parametric status strings everywhere: `folded_into` (not the prefix hack) and `blocked_by` (not `blocked on <slug>`). `validate` checks each resolves to a real slug.

### The render engine

- `render` is a strict pure function `(plan.toml, sidecars) -> <task>.md`. It parses TOML, splices Markdown sidecars OPAQUELY (never parses their bytes back into structure), and emits a few generated fragments. There is no prose round-trip and nothing to clobber: render writes exactly one file (`<task>.md`), never a sidecar or the TOML.
- Strict failure: a schema violation, an unresolved cross-reference, or a missing sidecar exits non-zero and writes NOTHING (no partial plan an agent could read as authoritative).
- A generated do-not-hand-edit banner heads `<task>.md`, naming the real sources.
- `render --check` is the guard: it re-renders in memory and compares against the committed `<task>.md`, catching both a hand-edit of the generated file and a stale render after a source edit. (Mechanism varies: A does a byte-for-byte compare against the committed golden; B and C store a content hash in `[meta]`. This is an implementation detail; see 3(d) for severity.)
- The Status line is derived from the step-status distribution + open-question count (fixing the drift-prone hand-maintained Status line, D1/A1/SE-13). The status vocabulary is generated from the code constants, not a hand-copied prose list (fixing B3).

### Sidecars

- Opaque Markdown, no frontmatter, fixed-convention paths derived from the task name and the slug/id: `<task>.steps/<slug>.md`, `<task>.questions/<id>.md`, plus front/tail prose sidecars for Motivations / Project Principles / Repository Layout and Success Criteria. Render inlines each verbatim.

### Enforcement preserved as a parser swap, not a rewrite

- W3/W4/W5 and the round-log consistency check read the same `Vec<Step>` / `Vec<Question>` shapes from a new `plan::parse_toml` instead of the Markdown parsers. W4's only logic change is `status.starts_with(QUEUE_FOLD_PREFIX)` becomes `status == "decided"` reading `folded_into`. The pause.md catch is untouched (a `complete` step with no records and no covering waiver still fails).
- The structured step<->increment link retires the lexical `-inc<x>` strip for new data: new `round` (and, under the recommended path, `waiver` / `escalation`) records carry a structured step/increment id; `leading_slug` stays only as a compat shim for pre-migration records. No existing record is rewritten. (B additionally enumerates increments as nested TOML tables; A and C keep increments a JSONL-side concept. See 2 and 3.)

### Migration shape

- Shadow-render-then-cutover: build schema + parser + render additively while the Markdown `.md` stays the live source; generate the TOML skeleton (scriptable from the existing parsers) and split the prose into sidecars; shadow-render and diff against the current hand-authored `.md` to confirm fidelity; then cut over in a single commit (TOML + sidecars + generated `.md` together). The JSONL is untouched throughout, so `validate --workflow` stays green at every stage.

---

## 2. The load-bearing divergence: do waivers and the baseline move into the TOML?

This is the one call that splits the designs and it is the human's to decide.

- Design B: MOVE them. A `[[step.waiver]]` nests on the step it exempts (and `[meta].w4_baseline` holds the cutoff), on the principle that a waiver is a standing DECLARATION / live state and a baseline is one-time CONFIG, not temporal events.
- Designs A and C: KEEP them in the append-only JSONL, where pilot 2 shipped them. The TOML holds only the skeleton; the log is the waiver's built home.

### The argument each way

For MOVING (B):

- Queryability and co-location. "Why is `optional-modules` complete without a converged streak?" becomes a local lookup on the step's `[[step.waiver]]` rather than a scan of 113 log lines. A waiver nested on a step cannot dangle to a nonexistent step (Principle 5, illegal states unrepresentable).
- Revisability. A waiver is a claim that can legitimately be withdrawn (a `predates-logging` step that later gets real rounds should drop its waiver). Append-only cannot express retraction except by convention; mutable TOML state can.
- The append-only-immutability rationale is weaker for waivers than for genuine events: the defensive "a malformed waiver must never silently exempt" property is preserved by the strict/best-effort split moving to `validate --source` (strict reports it; the projection W3 reads drops it).

For KEEPING (A, C):

- The waiver model just shipped and is tested. Moving it re-homes pilot-2 code: it makes W3/W5's source swap a risky increment (it rewrites the enforcement backstop's read path) for a benefit (queryability) that a JSONL-projecting `status --json` already delivers as a read-model without relocating state.
- Single-substrate integrity join. W5's `record-backed` join requires a waiver's `evidence` to point at a real `type:"escalation" human_decision:"decision"` record scoped to the waived unit. Today both the waiver and the escalation are JSONL events, so the join is within one substrate. If waivers move but escalations stay events (B keeps escalations in the log, correctly), that integrity join now CROSSES substrates (mutable TOML declaration joined to immutable JSONL evidence). The evidence is still the immutable escalation so the property survives, but the join spans two files with different mutation rules.
- Minimal migration. Keeping waivers where they are means the enforcement DATA never moves, so no migration window can read half-migrated evidence, and B's "17 valid-but-unconsulted historical lines left in the append-only log" ugliness never arises.

### Under the Principles

- P8 (structured single source, derived views) does NOT by itself force relocation: P8 says "a structured file OR A SMALL SET OF FILES," and the JSONL is already structured data. Both a TOML waiver and a JSONL waiver satisfy P8; what P8 forbids is hand-narrating derived views, which both paths avoid. So P8 is roughly neutral here, contrary to B's framing that it favors the TOML.
- P2 (minimal by default; do not complicate the core) favors KEEPING: no new schema surface, no risky read-path swap, no cross-substrate join.
- P6 (evidence-first / proven by dogfooding) leans toward KEEPING: the record-backed evidence join is cleanest when waiver and escalation live in the same event substrate; moving the waiver splits the join across mutation regimes for a mostly-read-model benefit.

### Recommendation

KEEP waivers and the baseline in the JSONL (designs A / C). Reasoning: the enforcement substrate shipped and is tested in pilot 2; P2 and P6 both weigh against churning it; the queryability B wants is a read-model concern that `status` / `status --json` can serve by PROJECTING the JSONL waivers (no relocation needed); and the W5 record-backed join stays single-substrate. P8 does not tip the balance because the JSONL is itself structured. The one genuine B advantage, revisability, is rare and is better served by a small explicit convention (a waiver is rendered moot once real converging rounds exist for its unit, or is superseded by a later record) than by moving mutable state that must then be cross-joined to immutable events. This makes the checks' source-swap increment (section 4, Inc 4) touch only steps/questions, not waivers, shrinking the risky surface.

Flag for the human: if you weight the plan-as-a-single-queryable-object goal and waiver co-location very highly and accept a risky W3/W5 read-path swap plus the cross-substrate join, B is defensible. It is your call.

---

## 3. The other open sub-questions (decision batch)

(a) Prune the 17 historical JSONL waiver/baseline lines, or keep them? KEEP. One-line reasoning: append-only integrity is a load-bearing invariant; a one-time clean-slate exception buys a marginally tidier log at the cost of breaking the property the pilots relied on. (Under the recommended keep-in-JSONL path this is moot: the lines stay consulted.)

(b) Do Success Criteria and Project Principles become structured TOML or stay prose? SPLIT: structure Principles as `[[principle]]` (number + name + text) so render numbers them and single-sources them (closes B8 and makes the SE-1 "Project P5 vs Scaffold P16" citation fix mechanical; cheap, no rot risk). Keep Success Criteria PROSE in a sidecar; defer B's `satisfied_by` step-linking as rich-lens gold-plating that rots on rename and pays off only in a coverage report nobody has asked for yet. One-line reasoning: principles are cheap to structure and have a real drift problem; success-criteria links add schema and maintenance for speculative queries.

(c) Do the queue `options` / `chosen` live in the TOML (with a receipt pointer) or only in the JSONL decision receipt? JSONL RECEIPT ONLY (A, C over B). One-line reasoning: they already have exactly one structured home and W4 enforces it; copying them into the TOML recreates a two-homes-plus-cross-check pattern the audit was shrinking, for no consumer the receipt does not already serve.

(d) `render --check` severity? WARN-LOCAL, FAIL-CI (all three lean this way). One-line reasoning: matches the shipped "warn loudly, fail hard only when asked" stance so a forgotten re-render never blocks an in-flight workflow step, while CI (`--strict`) and an optional pre-commit hook stop a stale plan from landing.

(e) RESUME STATE as a structured block or ledger prose? LEDGER PROSE, with the derivable pointer half (active step, open questions, current streak) computed by `status --resume` / `next` from the TOML + JSONL. One-line reasoning: the non-derivable remainder (a pending dismissal recheck, a running debate) is genuinely transient in-flight state that gains nothing from a schema, and structuring it invites another double-write.

(f) Others the three flagged:

- One-line labels (`title` / `ask`): inline in the TOML, or the sidecar's first line? INLINE (A). One-line reasoning: keeps every sidecar a fully opaque blob and render dumb, at the cost of two short strings.
- Orphan JSONL `task` slugs (`consolidate-plan`, `metrics-fields`, `plan-fold`, `plan-maintenance`, `workflow-hardening`): declare them explicitly in `[meta]` (B, C) or leave tolerated (A)? DECLARE in a `[meta]` orphan list. One-line reasoning: closes SE-16 cheaply and lets `validate` flag any NEW orphan instead of silently skipping.
- `[meta] primary` cutover fallback bit (C): ADOPT for the live migration only. One-line reasoning: it decouples "the TOML exists" from "the TOML is the source," so a partial in-progress TOML never accidentally becomes primary; it can be retired after cutover once the `.md` parsers are removed.
- Relocate question bodies to a `## Question Details` section mirroring `## Step Details` (A): ADOPT. One-line reasoning: makes the queue a scannable one-line-per-item list and is the natural consequence of the one-line `ask`; a visible-but-benign shape change a reviewer signs off once.
- Validate the extraction/render tooling on a small SYNTHETIC pilot plan before touching the live 51-step plan (C): ADOPT. One-line reasoning: cheap insurance that the prose-split script and render are correct before the heaviest, least-reversible task.
- Preserve the current ~1,200-word hand-authored Status prose as a sidecar during migration (C's `_status-narrative.md`): PRESERVE during migration, prune later. One-line reasoning: the derived Status line cannot reconstruct the editorial "which clusters completed when / current priority" context; keep it as prose to avoid data loss, retire it once the ledger RESUME STATE carries the resume anchor.

---

## 4. Recommended staged roadmap

One reconciled ordered list. Each increment is independently shippable and reviewable under the role-separated loop. Risk class in parentheses; risky = touches the enforcement read-path or cuts over the live plan.

- Inc 1 (low): TOML schema + `plan::parse_toml -> (Vec<Step>, Vec<Question>, Meta)` + `validate --source` (schema + internal cross-references: unique slugs, `blocked_by` resolves, `decided` implies a resolving `folded_into`, well-formed ids, in-set statuses, `[meta]` orphan list well-formed). No render, no change to existing Markdown plans; nothing reads the TOML in anger yet. Acceptance: `parse_toml` round-trips a fixture; `validate --source` flags an injected bad status, a dangling `blocked_by`, and a `decided` question with no `folded_into`; `validate --plan` on the live repo stays green.
- Inc 2 (low): structured step/increment link on JSONL records. Add an optional structured step (and increment) id to new `round` / `waiver` / `escalation` records; W3/W5 prefer it and fall back to `leading_slug` for pre-migration records. Additive, retires the SE-10/B6 over-strip risk (T3) for new data. Can land in parallel with Inc 1. Acceptance: a record carrying the field joins without the lexical strip; a pre-migration record with no field still joins via the shim; the existing W3/W5 suite passes.
- Inc 3 (risky): the render engine + `render --check` guard. Opaque sidecar splicing, derived Status line, generated status-vocabulary fragment, do-not-edit banner, strict failure on a broken source, and the check (byte/golden or `[meta]` hash). Tested against a fixture `.plan.toml` + sidecars and the synthetic pilot plan (3f). Acceptance: render is deterministic and byte-stable; `render --check` fails on a one-byte hand-edit and on a stale render after a source edit; render exits non-zero and writes nothing on a missing sidecar or an unresolved cross-reference. Risky: new load-bearing derived-artifact write path.
- Inc 4 (risky): point the checks at the TOML source. W3/W4/W5 and `status` read steps/questions from `parse_toml` when `[meta].primary == "toml"` (else the Markdown fallback, per 3f); W4's decided-gate becomes `status == "decided"` reading `folded_into`. Waivers and the baseline stay JSONL-sourced (section 2 recommendation), so W5 and W4's cutoff logic are untouched. Acceptance: the full `workflow.rs` suite passes with TOML-sourced fixtures; the pause.md catch and the `optional-modules` accepted-at-escalation waiver shape behave identically. Risky: rewrites the enforcement checks' plan-side source.
- Inc 5 (risky-but-reversible): migrate this repo. Synthetic-pilot-first (3f). Generate the skeleton (scripted from the existing parsers), split the prose to sidecars (the heavy manual lift, ~51 step bodies + ~45 question bodies + the meta prose; the single place a data-loss slip would hide), keep `[meta].primary = "markdown"` and shadow-render + fidelity-diff against the current `.md` (expected diffs: banner, derived Status line, question-body relocation), then cut over in one atomic commit (set `primary = "toml"`, render, commit TOML + sidecars + generated `.md` together) and wire `render --check` into `checks.toml` + CI. Acceptance: the generated `.md` matches the reviewed plan modulo the documented expected diffs; `validate --source`, `validate --workflow`, and `status` are green reading the TOML; `git revert` on the cutover commit restores the hand-authored `.md` to green. Risky because it cuts over the LIVE plan, but each pre-cutover stage is additive and reversible (delete the TOML/sidecars, tests pass unchanged) and the cutover is a single revertible commit.
- Inc 6 (risky): pack template + scaffold. Replace `pack/plan-template.md` with `pack/plan-template.plan.toml` + starter sidecar directories (`.steps/.gitkeep`, `.questions/.gitkeep`) + the front/tail prose sidecars; `scaffold` drops the TOML template and runs an initial `render`. Migrate or drop `plan_template_documents_every_accepted_status` (render now generates the vocabulary from the constants, so the live-plan drift guard is largely subsumed). Acceptance: a fresh `agent-scaffold` run drops a coherent `plan.toml` + rendered `<task>.md` + starter sidecars, and `validate` passes on it. Risky: changes what every scaffolded project inherits.

Dependency order: Inc 1 -> Inc 3 -> Inc 4 -> Inc 5, with Inc 2 parallel to Inc 1/3 and Inc 6 after Inc 3. Inc 4 needs Inc 1; Inc 5 needs both Inc 3 (shadow render) and Inc 4 (TOML-source validation at cutover). Inc 1 and Inc 2 are the lowest-regret and can go first. C's `[meta] primary` fallback bit is folded into Inc 4/5 as the deliberate one-line cutover switch and the migration-order de-risking (build and validate infra on the synthetic pilot before touching the live plan).

Risky vs low: Inc 1 and Inc 2 are low (pure additions). Inc 3, Inc 4, and Inc 6 are risky (new derived-write path, enforcement read-path swap, scaffold blast radius). Inc 5 is risky-but-reversible (cuts over the live plan, but every stage is additive and the cutover is one revertible commit).

---

## 5. Load-bearing invariants the build must preserve

- W3, W4, W5, and the round-log consistency check keep working, reading the same `Step` / `Question` shapes from `parse_toml` instead of the Markdown parsers; the JSONL evidence is untouched. The pause.md catch survives verbatim: a `complete` step with no matching rounds and no covering waiver still fails W3.
- The pilots' 113 JSONL records stay valid and `validate` stays green at every migration stage; the enforcement DATA never moves (recommended path), so no window reads half-migrated evidence. Append-only is honored (no line rewritten).
- Exemptions stay declared and visible, and the two evidence tiers do not launder: the strict/best-effort discipline is preserved (`validate` reports a malformed or mis-tiered waiver; the projection the checks read drops it), and W5's record-backed join to a decision-scoped escalation is unchanged.
- No prose round-trip or clobber: render writes only the derived `<task>.md`, reads sidecars as opaque byte blobs, and never reconstructs prose from fields, so there is no path to overwrite an author's Markdown.
- The scaffolded default stays coherent: a fresh scaffold drops a TOML skeleton + prose sidecars + a rendered `<task>.md` that are internally consistent and `validate`-green.
