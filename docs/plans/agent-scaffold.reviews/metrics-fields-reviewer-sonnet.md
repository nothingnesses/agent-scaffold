# Review: metrics-fields-reviewer-sonnet

Commit f57c87f adds two optional `round` record fields (`risk_class` and `reviewers`) with a validator, tests, and prose updates. The change is non-breaking and mechanically sound. Two medium findings concern calibration data quality (an undocumented deduplication relationship and an LLM ergonomics problem that will produce noisy data). Four low findings are a schema gap (empty array allowed), a missing test case, a dead code branch, and a subtle drift-guard coverage gap.

---

## Finding 1 [MEDIUM]: Undocumented deduplication between per-reviewer and round-level valid_findings

**Location:** `pack/instrument.md` line 5; `AGENTS.md` Instrumentation section (the parallel text updated in this commit).

When two reviewers each surface the same bug, both will record `valid_findings: 1` in their reviewer entry; the triager deduplicates and the round gets `valid_findings: 1`. So `sum(reviewer[i].valid_findings)` can exceed (and routinely will exceed) `round.valid_findings`. Neither the prose in `instrument.md` nor `AGENTS.md` explains this relationship. A downstream user summing per-reviewer counts to compute "total valid findings this round" will over-count, and a user trying to compute reviewer overlap from those two numbers will get incorrect results. The whole point of the `reviewers` array is calibration, so an undocumented systematic skew in the central metric is a real data quality defect, not a nit.

---

## Finding 2 [MEDIUM]: Per-reviewer valid_findings is difficult for an LLM orchestrator to populate accurately

**Location:** `AGENTS.md` Instrumentation section; `pack/instrument.md` line 5.

To populate `reviewer.valid_findings`, the orchestrator must attribute each triager-confirmed valid finding back to the specific reviewer who raised it. The triager's output is a verdict per finding, not per reviewer; when two reviewers raised the same finding, it is ambiguous which one gets the `valid_findings` credit. The schema provides no guidance on how to handle overlap (e.g., credit both, credit the one with the higher-resolution report, count once). In practice, an LLM orchestrator is likely to estimate or guess this number, making it inconsistent across tasks and reducing the calibration value of the per-reviewer breakdown. The prose should either document a tie-breaking rule or acknowledge that this field counts the reviewer's own judgement (pre-triage), not post-triage attribution, which would change its interpretation entirely.

---

## Finding 3 [LOW]: Empty reviewers array is syntactically valid but semantically wrong

**Location:** `src/metrics.rs` lines 212-225 (`require_reviewers`).

`"reviewers": []` passes the validator. The for-loop exits immediately and returns `Ok(())`. An empty array is semantically impossible (if the field is present, at least one reviewer examined the artifact). The validator could require `array.len() >= 1` when the field is present. No existing test exercises this case, so the acceptance is untested. Low severity because the validator is a post-hoc consistency check, not a runtime enforcement point, but it creates a hole where a malformed log line passes silently.

---

## Finding 4 [LOW]: Missing test for a non-object element in the reviewers array

**Location:** `src/metrics.rs` tests section; the four new reviewer tests at lines 445-484.

There is no test for `"reviewers": [42]` or `"reviewers": ["string_element"]` (a reviewers array whose element is not a JSON object). The code handles it correctly at line 213-215 (`element.as_object().ok_or_else(...)`), but no test verifies the error message format for this path. The four added tests cover: missing field, wrong array type (string instead of array), and bad count. The non-object element case is the one combination not covered.

---

## Finding 5 [LOW]: Dead branch in require_reviewers - the missing-field error path can never fire

**Location:** `src/metrics.rs` lines 208-210.

```rust
let value = obj.get(name).ok_or_else(|| format!("missing field `{name}`"))?;
```

The only caller (line 263-265) is guarded by `if obj.contains_key("reviewers")`, so `obj.get(name)` is guaranteed to return `Some`. The `ok_or_else` on the `None` branch is dead code. This does not affect correctness; if the guard were ever removed or bypassed by a future refactor, `require_reviewers` would silently panic rather than returning the error (since `?` returns from the function, and the outer caller expects the error to surface there). The established pattern for optional-but-validated fields in this module (compare `require_severities`, which is always called on a required field) would benefit from a comment explaining why the guard is separate.

---

## Finding 6 [LOW]: Drift guard does not distinctly protect the per-reviewer valid_findings documentation

**Location:** `src/metrics.rs` test `instrument_prose_documents_every_accepted_schema_value`, lines 520-540 (the field list).

The drift guard checks `prose.contains("`valid_findings`")`. Because `valid_findings` was already documented at the round level before this commit, removing only the per-reviewer mention of `valid_findings` from `instrument.md` would not fail this test (the round-level mention satisfies the substring check). The guard is effectively intact for `raw_findings` (a new field name exclusive to the reviewer context), but `valid_findings` in the reviewer context is not distinctly guarded. The risk is low in practice because any edit that removes the reviewers description would also remove `raw_findings`, which would catch it. But the guard is weaker than it appears for this one field name.
