# Inc 1 (structured-skeleton) reviewer findings: soundness and correctness

Reviewer lens: soundness/correctness. Change under review: commit `aa42412` on base `a780541`. Files: `src/plan/source.rs` (new), `src/plan.rs`, `src/main.rs`, `src/metrics.rs`, `src/workflow.rs`, `src/plan/testdata/skeleton.plan.toml`.

Verification run from the worktree (`.claude/worktrees/structured-skeleton-inc1`), treated read-only:

- `cargo test --all-targets`: 229 passed (225 + 1 + 3), 0 failed. Matches the claim.
- `cargo clippy --all-targets -- -D warnings`: clean (Finished, no warnings).
- Live path `validate --metrics docs/metrics/workflow.jsonl --plan docs/plans/agent-scaffold.md --workflow`: exit 0, `116 records, valid`, `workflow invariants hold`. Undisturbed.

Severity tally: critical 0, high 0, medium 1 (O1), low 5 (O2-O6).

## Cleared (checked and ruled out)

- CRITICAL serde concern (does adding `Serialize`/`Deserialize` to the `enum_field!` enums change how existing metrics records (de)serialize?): NO. The JSONL path never (de)serializes these enums via serde. `check_record` reads a `serde_json::Value` and validates enum fields through `require_enum` + the hand-written `Xxx::parse` (`src/metrics.rs:254-266,393-537`); every projection (`parse_rounds`, `parse_decisions`, `parse_baseline`, `parse_waivers`, `parse_escalations`) reads `Value` strings through `Xxx::parse` as well. The metrics projection structs (`Round`, `Decision`, `Baseline`, `Waiver`, `Escalation`) derive only `Debug, Clone, PartialEq, Eq` (`src/metrics.rs:555,631,676,730,827`), NOT `Serialize`/`Deserialize`, so nothing in the JSONL path routes through the new derives. The derives are consumed ONLY by the new `src/plan/source.rs` TOML structs. Each `#[serde(rename = $text)]` spelling equals the `VARIANTS`/`label()` on-disk token (`src/metrics.rs:35-40,142-179`), so the TOML tokens (`low_risk`, `predates-logging`, `record-backed`, etc.) match the JSONL tokens. Empirically the live 116-record log still validates. No round-trip of any existing record can change.
- W5 byte-faithfulness (`src/workflow.rs:411-415` vs old inline match): faithful. Old logic mapped `PredatesLogging|ReviewSkipped -> require SelfDeclared`, `AcceptedAtEscalation -> require RecordBacked`. `WaiverReason::required_tier` (`src/metrics.rs:102-108`) encodes exactly that mapping, and `required_tier() == waiver.evidence_tier` is equivalent to the old per-arm `waiver.evidence_tier == <tier>` because `EvidenceTier` has exactly two variants (so `x == A` iff `A == x` and `!= A` iff `== B`). Every (reason, tier) pair, including the two rejection cases, is preserved.
- Exit-code wiring (`src/main.rs` run_validate): source problems are pushed into the shared `problems` vec and `!problems.is_empty()` triggers `std::process::exit(1)`; a clean source adds a summary only. Correct.
- Tests are non-vacuous (Principle 11): the round-trip test asserts full struct equality including nested `[[step.increment]]`/`[[step.waiver]]` (`src/plan/source.rs:623-670`); the negative tests use `assert_flags` which asserts a real problem string is produced (`src/plan/source.rs:679-761`). Each of the ACCEPTANCE cases in the plan (bad status, dangling blocked_by, decided-with-no-folded_into, record-backed-missing-evidence, self-declared-dressed-as-record-backed, increment-waiver-naming-absent-increment) has a dedicated test.

All findings below were reproduced by running `validate --source` on a crafted `.plan.toml` from the worktree; each returned `valid` with exit 0.

---

## O1 (medium): a dangling `folded_into` on a non-decided question is never resolved

`src/plan/source.rs:491-505`. `folded_into` is resolved ONLY inside the `QuestionStatus::Decided` arm; the `Open | Exploring | Superseded` arm is `{}` (no checks). So a question that carries `folded_into` while NOT `decided` has that reference neither resolved nor rejected. `superseded_by` in the same function is checked UNCONDITIONALLY (`src/plan/source.rs:506-513`), so the two sibling cross-references are treated asymmetrically.

Reproduced: an `open` question with `folded_into = "ghost"` (no such step) validates clean, exit 0.

Why it matters: the Inc 1 contract (plan line 684) lists `folded_into` among the cross-references that must "resolve", and the module doc (`src/plan/source.rs:423-426`) claims `validate_source` catches the dangling cases. A dangling `folded_into` on a non-decided question is a dangling reference that slips through. Impact today is low (nothing consumes `folded_into` on a non-decided question, and this module is unconsulted pre-cutover), which is why this is medium rather than high; but it is a genuine gap in the validator's stated contract, and it is exactly the class of authoring error `validate_source` exists to catch.

Suggested direction: resolve `folded_into` whenever it is `Some`, independent of status (so a dangling target is always flagged), and separately decide whether `folded_into` present on a non-`decided` question is itself an error (illegal-states-unrepresentable would say yes). Mirror the unconditional treatment already given to `superseded_by`.

## O2 (low): `superseded` status does not imply a resolving `superseded_by`

`src/plan/source.rs:491-513`. `decided` implies `folded_into` (checked), but `superseded` does NOT imply `superseded_by`. A question with `status = "superseded"` and no `superseded_by` validates clean (reproduced, exit 0). This is the dual of the decided-implies-folded_into rule and is absent.

Why it matters: a `superseded` item that names no successor is an incomplete/illegal state analogous to a decided item with no fold target. The contract does not name this case explicitly, so this may be an intentional omission, but the asymmetry is surprising and lets an under-specified superseded item through.

Suggested direction: add a `superseded`-implies-`superseded_by` presence rule paralleling the decided/folded_into one, or document why superseded is deliberately weaker.

## O3 (low): waiver `id` (and `principle.n`) are neither uniqueness- nor well-formedness-checked

`src/plan/source.rs:544-609`. Step slugs, increment ids, and question ids each get a uniqueness + well-formedness check, but waiver `id` gets neither. Two waivers on the same step with the same `id = "dup"` validate clean (reproduced, exit 0). `principle.n` likewise has no uniqueness check, so two principles numbered `n = 1` pass.

Why it matters: waiver ids are new to this schema and are plausibly referenced by later increments/tooling; a duplicate or blank waiver id is a collision the validator silently accepts. Lower severity than O1 because nothing in Inc 1 joins on the waiver id yet.

Suggested direction: add waiver-id uniqueness (scope: per-step or plan-wide, pick one and state it) and `is_well_formed_token` checks, matching the treatment of the other ids; optionally check `principle.n` uniqueness.

## O4 (low, and the flagged open question): no `deny_unknown_fields` lets a typo in an OPTIONAL key silently drop a cross-reference

`src/plan/source.rs:402-404` (`toml::from_str`, no `deny_unknown_fields`). Required-field typos ARE caught (the field goes missing -> parse error), so a mistyped `status`/`slug`/`title` fails loudly. But a typo in an OPTIONAL key is silently ignored and the field defaults. Reproduced: a step written with `blockd_by = ["ghost"]` (typo for `blocked_by`) validates clean, exit 0; the intended blocking edge is silently dropped AND the dangling `ghost` is never seen. The same hazard applies to `folds`, `orphan_tasks`, `superseded_by`, `receipt`, `note`, `evidence`, `increment`, and the `[meta.sidecars]` keys.

This is the open question the implementer flagged (forward-compat vs typo-catching). Both sides:

- Keep permissive (current): forward-compatible with schema growth; matches the JSONL log's stance. But the JSONL records are validated field-by-field with explicit presence checks, so a typo there still shows up as a missing required field; the TOML's `#[serde(default)]` optionals have no such backstop, so a typo silently loses data in a hand-authored file.
- Add `#[serde(deny_unknown_fields)]`: a mistyped key becomes a loud parse error, which for a hand-authored skeleton is the safer failure direction (a dropped `blocked_by` is a correctness bug, not a compatibility feature). Cost: future additive keys require touching the structs, but this schema is versioned in-repo and Inc 3-6 will edit it anyway, so the forward-compat argument is weak here.

Recommendation: add `deny_unknown_fields`. The skeleton is hand-authored and internally versioned; the value of catching a typo that silently drops a cross-reference outweighs the loss of unknown-key tolerance, and it directly closes the O4 data-loss path. This is a decision for the human/triager, not something I would change unilaterally.

## O5 (low): self-referential and cyclic `blocked_by` / self-`folds` are accepted

`src/plan/source.rs:473-488`. `blocked_by`/`folds` are checked only for existence in the slug set, not for self-reference or cycles. A step with `blocked_by = ["a"]` and `folds = ["a"]` on step `a` itself validates clean (reproduced, exit 0). A two-cycle (`a` blocks `b`, `b` blocks `a`) would likewise pass.

Why it matters: a step that blocks itself can never unblock, and a `blocked_by` cycle is unsatisfiable; both are illegal states any downstream ordering (render/status) would trip on. The contract says "resolve", not "acyclic", so this is arguably out of Inc 1 scope, hence low.

Suggested direction: at minimum reject self-reference (`target == step.slug`) for `blocked_by` and `folds`; consider a cycle check if later increments topologically order steps.

## O6 (low): `orphan_tasks` are not unique and may collide with a real step slug

`src/plan/source.rs:531-536`. Each orphan task is checked for token well-formedness only. There is no uniqueness check and no check that an orphan task does NOT name an existing step slug, which contradicts the field's definition (`src/plan/source.rs:244-247`: "tasks that appear in the round log but own no Roadmap step"). An orphan-task token equal to a declared `slug` passes.

Why it matters: an orphan task that is also a real step is self-contradictory by the field's own contract and could double-count in later tooling. Low because orphan handling is not consumed in Inc 1.

Suggested direction: reject duplicates within `orphan_tasks` and reject any orphan token present in the step-slug set.

---

Nothing rises to high or critical: the two enforcement-adjacent touches (the `metrics.rs` derives and the `workflow.rs` W5 refactor) were checked closely and are both faithful/inert with respect to the live path, and the live pipeline is empirically undisturbed. The findings above are gaps in `validate_source`'s cross-reference coverage (O1-O6), the most substantive being O1 (a dangling reference in the contract's own resolve-list) and the O4 open question.
