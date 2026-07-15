# Reviewer findings: state-schema increment 2, round 4 (LOW-2 confirming review)

Artifact: the metrics drift-guard test in `src/metrics.rs`, after the LOW-2 fix. Commit reviewed: `4188105`. Classification: LOW-risk artifact (one consecutive clean round to converge). Scale: four-level (`low` / `medium` / `high` / `critical`).

---

## Verdict summary

All three loops in `instrument_prose_documents_every_accepted_schema_value` are now anchored. Coverage mirrors `check_record` exactly. The guard catches drift of each kind. This is a CLEAN round.

No critical, high, medium, or low findings.

---

## Anchoring verification (Principle 11)

`src/metrics.rs:417-421` - record-type loop:

```rust
for record_type in ["round", "escalation", "dismissal_recheck", "intake"] {
    assert!(
        prose.contains(&format!("\"{record_type}\"")),
        ...
    );
}
```

Uses the quoted form `"\"..."\"`. Prior to LOW-2 this was `prose.contains(record_type)` (bare substring). The fix is in place.

`src/metrics.rs:445-448` - field loop: uses ``prose.contains(&format!("`{field}`"))`` (backtick form). Fixed in LOW-1.

`src/metrics.rs:464-467` - enum-value loop: uses ``prose.contains(&format!("`{variant}`"))`` (backtick form). Fixed in LOW-1.

All three loops are anchored. No unanchored loop remains.

---

## Completeness: test coverage vs. `check_record` (Principle 16)

Cross-referenced the test field list against every `require_str`, `require_bool`, `require_count`, `require_enum`, and `require_severities` call in `check_record` (`src/metrics.rs:191-235`).

**Record types** - `check_record`'s `match record_type` accepts four arms: `round`, `escalation`, `dismissal_recheck`, `intake`. The test list (`src/metrics.rs:417`) is `["round", "escalation", "dismissal_recheck", "intake"]`. Exact match.

**Fields** - unique fields that `check_record` requires or conditionally checks:

| Field | `check_record` call | In test? |
| --- | --- | --- |
| `type` | `require_str` (line 196) | Yes |
| `task` | `require_str` (line 197) | Yes |
| `ts` | optional `require_str` (lines 198-200) | Yes |
| `artifact` | `require_str` in `round`, `escalation`, `dismissal_recheck` (lines 204, 215, 221) | Yes |
| `phase` | `require_enum` Phase (line 205) | Yes |
| `changed_since_prev` | `require_bool` (line 206) | Yes |
| `outcome` | `require_enum` RoundOutcome (lines 207-209) | Yes |
| `valid_findings` | `require_count` (line 210) | Yes |
| `severities` | `require_severities` (line 211) | Yes |
| `consecutive_clean` | `require_count` (line 212) | Yes |
| `human_decision` | `require_enum` HumanDecision (lines 216-218) | Yes |
| `result` | `require_enum` RecheckResult (lines 222-224) | Yes |
| `classification` | `require_enum` Classification (lines 227-229) | Yes |
| `replanned` | `require_bool` (line 230) | Yes |

14 unique fields in `check_record`, 14 entries in the test list. No field the validator checks is absent from the test; no phantom field is in the test.

**Enum types** - all six `enum_field!` types appear in the enum loop (`src/metrics.rs:453-459`): `Phase`, `RoundOutcome`, `HumanDecision`, `RecheckResult`, `Classification`, `Severity`. Each is driven from its own `VARIANTS` slice (self-tracking). Coverage is complete and correct.

No coverage gap found.

---

## Soundness proof: induced-drift experiments (Principle 6)

Each experiment temporarily edited `pack/instrument.md`, ran only the drift-guard test, then reverted before the next experiment.

**Experiment 1 (record type):** Changed `` `type: "round"` `` to `` `type: "round_DRIFT"` `` on line 5 of `pack/instrument.md`. Test failed:

```
record type `round` accepted by the validator is not documented in pack/instrument.md
```

at `src/metrics.rs:418`. Reverted.

**Experiment 2 (field):** Changed `` `consecutive_clean` `` to `` `consecutive_clean_DRIFT` `` on line 5 of `pack/instrument.md`. Test failed:

```
field `consecutive_clean` checked by the validator is not documented in pack/instrument.md
```

at `src/metrics.rs:445`. Reverted.

**Experiment 3 (enum value):** Changed `` `plan_review` `` to `` `plan_review_DRIFT` `` on line 5 of `pack/instrument.md`. Test failed:

```
enum `Phase` value `plan_review` accepted by the validator is not documented in pack/instrument.md
```

at `src/metrics.rs:464`. Reverted.

After all three reverts: `direnv exec . just test` returned 64 passed, 0 failed. `git status --short` was empty (only this findings file present).

---

## Tool checks

- `direnv exec . just test`: 64 passed, 0 failed.
- `direnv exec . cargo clippy --all-targets -- -D warnings`: clean (no warnings).

---

## Findings

### No critical findings.

### No high findings.

### No medium findings.

### No low findings.

The guard is complete (coverage mirrors the validator exactly) and sound (catches drift of all three kinds). Not raised, per scope: the absence of status / plan-parsing (increment 3), the deferred duplicate-keys item (L1), and line length / wrapping.
