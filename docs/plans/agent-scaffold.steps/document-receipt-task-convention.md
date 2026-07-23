### `document-receipt-task-convention`: document (or rule non-normative) the decision-receipt `task` convention

Nine decision receipts in `docs/metrics/workflow.jsonl` set their `task` field to the question's `folded_into` slug, but nothing joins on a decision record's `task`: the W4 receipt check keys on `q_id`, not `task`. So the `task` value on a `type:"decision"` record is currently descriptive convention, not a load-bearing join key.

Decide one of: document the convention in the instrumentation docs (state that a decision receipt's `task` should carry the `folded_into` slug), or declare it non-normative (the `task` field on a decision record is free-form and nothing joins on it). Low priority; no behaviour change either way, this is a documentation-currency cleanup.
