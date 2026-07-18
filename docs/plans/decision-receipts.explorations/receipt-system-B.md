# Decision-receipt design exploration (lens: Architect / Steelman)

Explorer B. Companion to `receipt-system-A.md` (separate lens). Prepared as input to the orchestrator's synthesis for Q-42.

## 1. The question and what is fixed

Q-42 asks: should the workflow keep a checkable receipt that the human-input contract was presented at every human-decision point, and if so, in what form?

The human-input contract (AGENTS.md, "Human-input contract" section) already REQUIRES that every decision point present the viable options, their trade-offs, a recommendation, and Principle-judged reasoning before the human decides. The question is whether the workflow should GUARANTEE this by keeping durable, machine-checkable evidence that it happened, the same "rule, then make it checkable" pattern W3 applied to convergence.

What is fixed and not re-opened: receipts do not live in the ledger. The ledger is per-task and deleted at task close. Decisions are permanent. Most decisions happen outside any review loop, where no ledger exists. The ledger may narrate a mid-loop decision as a pointer to the durable receipt, exactly as it already does with escalation records. Permanent homes only: the Open Questions queue and/or `docs/metrics/workflow.jsonl`.

What this exploration does: designs the strongest coherent receipt system, judges whether it is worth the ceremony it adds, and gives a genuine verdict including the null option as a first-class comparator.

## 2. Design space

### Option 0 (null): existing machinery, build nothing new

The existing artifacts already record decisions partially. The Open Questions queue holds what was decided and which step it was folded into (AGENTS.md, "Tracking progress" and "Preventing relitigation" sections). `type:"escalation"` records (src/metrics.rs line 311-315) record that a cap was hit and the human's binary decision (resume vs. decision). `type:"intake"` records (src/metrics.rs line 322-331) record that an interrupt was classified. AskUserQuestion, where the harness supports it, produces a structured interaction record. Git history shows before/after state. The rule obliges the contract; the queue records the outcome.

The null option says: this machinery is sufficient. The contract is a matter of discipline enforced by the rule in AGENTS.md, not by a separate enforcement mechanism. Build nothing.

### Option 1 (light): enhanced in-queue standard with prose completeness check

Every `decided -> folded into <slug>` queue item must embed the presented options and the human's selection in the item body (either in the one-line ask or in the step detail it folds into). The `validate --plan` check is strengthened to require their presence, enforced as a structural prose requirement.

The feasibility problem: `plan::parse_questions` (src/plan.rs line 274-309) projects only `id`, `status`, and `ask` from a queue item. The `ask` is a one-liner. Structured options and selection in a one-liner are awkward to machine-parse. The decision body and reasoning live in the step detail, not the queue item, so a check of the queue item text alone cannot verify the full contract was followed. A check of the step detail would require parsing narrative prose, which the plan parser deliberately does not do (src/plan.rs line 11-14). This option's completeness check degrades to a presence check ("does the step detail contain the word 'options'?"), which is weaker than it appears.

### Option 2 (full): a permanent `type:"decision"` JSONL record plus a W4 check

A new record type in `docs/metrics/workflow.jsonl` captures the process: the question id, the options presented, the recommendation, and the human's selection. A new `validate --workflow` check (W4) asserts that every `decided -> folded into <slug>` Open Questions item has a matching receipt in the log. This follows the W3 pattern precisely: W3 reads `complete` Roadmap steps against round records; W4 reads `decided` queue items against decision records.

The record shape, following the schema conventions in src/metrics.rs lines 34-56:

```
{"type":"decision","task":"agent-scaffold","q_id":"Q-42","options":["a: type:decision JSONL receipt + W4 check","b: light in-queue standard","c: null option"],"recommendation":"a","chosen":"a","ts":"2026-07-18T12:00:00Z"}
```

Required fields:

- `type`: the string `"decision"` (the record type discriminant)
- `task`: the task slug, same as every other record type
- `q_id`: the Open Questions item id (e.g. `"Q-42"`)
- `options`: a non-empty array of strings naming the presented options
- `recommendation`: a string naming the recommended option
- `chosen`: a string naming the human's selected option
- `ts`: optional ISO 8601 timestamp (present but optional, same as every other record type)

The `options` array is the evidence: the human's `chosen` value must be one of the strings in `options`, which `validate` enforces, so a record claiming the human chose an option that was never presented is a validation error. This is the genuineness property (discussed below).

The W4 check: for each `Question` from `plan::parse_questions` (src/plan.rs lines 274-309) whose `status` starts with `"decided -> folded into "` (the `QUEUE_FOLD_PREFIX` constant at src/plan.rs line 87), look for a `type:"decision"` record in `docs/metrics/workflow.jsonl` whose `q_id` matches the question id. A `decided` item with no matching record is a W4 violation, the decision-process analog of W3's `pause.md` catch (a step marked complete with no round records).

The W4 check also accepts two existing partial-receipt types as meeting the requirement for their specific contexts, to avoid requiring a redundant second record at those points: a `type:"escalation"` record covers the cap-escalation decision (where the contract was already followed), and a `type:"intake"` record covers an interrupt intake (which is itself a contract-shaped exchange). These two existing types predate the full receipt system; acknowledging them avoids making them redundant. The `q_id` field would need to be added to `type:"escalation"` and `type:"intake"` records to enable this cross-reference, which is a small schema extension.

Historical items (Q-1 through Q-41) predate this mechanism and will have no matching records. The pragmatic exemption follows the W3 precedent: W4 fires only for items decided after the `decision-receipts` Roadmap step is complete. In practice, W4 is forward-looking and does not audit history. Items decided before this step was introduced do not need retroactive records. This is the honest answer: the check covers future decisions, not the archive.

## 3. Trade-offs against the numbered Project Principles

### Option 0 (null)

Principle 2 (Minimal by default): the null option wins on ceremony; it adds nothing.

Principle 6 (Ground decisions in evidence): fails this principle for the decision-process itself. The rule in AGENTS.md is trusted; there is no way to distinguish "contract was presented and human decided" from "human was asked for a quick answer and obliged." The distinction matters most when auditing an unexpected decision or onboarding a new contributor who wants to verify the workflow was followed.

Principle 9 (Durable notes that survive context loss): the queue survives. What the queue does NOT survive is the evidence that the OPTIONS were explicitly enumerated and a SELECTION made from among them. A `decided` item records the outcome but not the presentation.

Principle 16 (One source of truth): the queue is already the authoritative record of decisions. The null option preserves single-source for decision CONTENT. The cost is that process fidelity has no machine-verifiable home.

Verdict on Option 0: sufficient for the current workflow, insufficient for the verification guarantee the human asked for. It is a genuine option if the team decides the rule alone is adequate discipline, but it explicitly under-delivers on the ask.

### Option 1 (light)

Principle 2: adds minor ceremony (the orchestrator must embed options in the item body) without meaningful mechanical verification.

Principle 6: weaker than it appears. The plan parser does not parse item bodies, so any check would be nominal. A reviewer could manually inspect items, but that is the same as trusting the rule.

Principle 14 (Parse, don't validate): option 1 validates prose at the wrong boundary. A mechanical check over unstructured prose in a step detail is prone to false negatives (the text says "options were" but the actual enumeration is buried in a sub-paragraph).

Principle 16: embedding structured data in prose risks drift between the queue item and the step detail. Which is authoritative when they disagree?

Verdict on Option 1: the light option has worse properties than either the null or the full option. It adds ceremony without genuine checkability. It is not recommended.

### Option 2 (full)

Principle 1 (Clean long-term architecture): the JSONL log is already the machine-readable event record for the workflow. Adding a `type:"decision"` record is coherent with the existing schema rather than inventing a new artifact class. The pattern (a structured record per event type, validated by `metrics::validate_log`, cross-referenced by `check_workflow`) is established (src/metrics.rs lines 276-331, src/workflow.rs lines 77-86).

Principle 2 (Minimal by default): the cost is one JSONL write per human decision. The project has approximately 42 decisions in its lifetime. The per-decision overhead is a single JSON object written to an append-only log. This is low ceremony. The AGENTS.md contract already requires the orchestrator to enumerate options and make a recommendation; writing the receipt records what the contract already required to be assembled. The incremental work beyond what the contract already requires is: write one JSON line after the human responds.

Principle 6 (Verify, don't trust): this is the strongest match. The `options` array and the `chosen` field are the evidence. W4 checks that each `decided` item has a receipt. The combination replaces "trust that the orchestrator followed the contract" with "verify that options were enumerated and a selection was made." This is the same principle W3 applies to convergence.

Principle 9 (Durable notes that survive context loss): `docs/metrics/workflow.jsonl` is committed and accumulates across tasks (AGENTS.md "Instrumentation" section: "never rewrite past lines"). Decision records are permanent and cross-task, the same durability property that makes round records useful for calibration. An audit after a context loss can read the record and verify the process was followed.

Principle 16 (One source of truth): the tension here is real and needs to be resolved directly. The queue holds decision CONTENT (what was decided, reasoning, which step). The receipt holds process ATTESTATION (that the contract was presented and a choice was made from among enumerated options). These are different things. The analogy to the existing ledger-narrative / JSONL-round split is exact (AGENTS.md "Tracking progress" section): the ledger narrative is the human-authoritative record of what happened; the round records are the machine-readable event log. The plan step detail is the human-authoritative record of WHAT was decided and WHY; the decision receipt is the machine-readable record that the CONTRACT was followed. Neither is a duplicate of the other. The receipt references the queue item by `q_id`; the queue item is authoritative for content; the receipt is authoritative only for the process attestation. If the two disagree (the receipt says "chosen: option A" but the step detail records a different decision), the queue is authoritative for content and the disagreement is a flag for a human reviewer, not a validator error. This is the same resolution the project applies to the round-log / ledger-narrative split: the narrative wins on content, the JSONL wins on the machine-readable event record.

## 4. What makes the proof genuine rather than orchestrator self-attestation

The Q-42 item correctly identifies the central design nuance: "the strongest receipt records the OPTIONS PRESENTED plus the human's SELECTION among them (choosing option Y from {X,Y,Z} inherently proves {X,Y,Z} was shown), rather than a self-certified 'I presented the contract' flag."

A bare boolean `"contract_presented": true` in the receipt is the weakest possible proof: it is pure self-certification, adds one character of overhead, and verifies nothing. Option 0 is effectively this boolean (the rule certifies the contract was followed, but there is no record).

The options array plus the selection is genuinely stronger because:

First, the options themselves are on record. A reviewer or auditor can read `options: ["a", "b", "c"]` and judge whether they represent a real design space or a false choice. A receipt that lists one option and "chose" it is a red flag. A receipt that lists substantive, distinguishable options is meaningful evidence.

Second, the selection must be from the presented set. The validator checks that `chosen` is a member of `options`. This cannot be trivially satisfied by writing "options: ['a'], chosen: 'a'" without it being visible as a single-option choice (also a red flag a reviewer can catch).

Third, when the decision followed an exploration (the `exploring` status in the queue, src/plan.rs line 82), the exploration file at `docs/plans/<task>.explorations/<q-id>-<disambiguator>.md` contains the design space the options came from. A receipt that names options inconsistent with the exploration is a discrepancy a reviewer can catch. This chain (exploration -> options enumerated in receipt -> selection) is the strongest form of genuine proof the workflow can produce without independent verification.

However, there is an honest limitation: the orchestrator writes both the presentation (in its message to the human) and the receipt (after the human responds). There is no independent verification that the orchestrator's message actually contained the options listed. This is the same limitation W3 faces: the orchestrator writes round records that W3 then checks; if the orchestrator fabricated a round record, W3 would not catch it. The check is a deterrent and a structural guard, not an adversarial proof. The genuine-proof property is: the record is evidence a reviewer can inspect, not a cryptographic guarantee. That is the right bar for a workflow tool.

## 5. How the receipt is written uniformly across all decision modes

The human-input contract applies at four entry modes (AGENTS.md "Human-input contract" section):

Escalation at the total-round cap: the orchestrator presents the options (resume, change course) to the human per the contract, the human decides, and the orchestrator writes both the existing `type:"escalation"` record (event record, W3 exemption signal) and a `type:"decision"` record (process attestation, W4 input). The two records coexist. A `q_id` is added to the `type:"escalation"` record if the escalation corresponds to a queue item (often it does not; it is a mid-loop event, not a queue-tracked decision). If it does not correspond to a queue item, no `type:"decision"` record is needed and W4 does not check for one.

Intake (human interrupt): the orchestrator assesses the request, presents options (fold in, route to planner, reject), the human decides, and the orchestrator writes both `type:"intake"` (the existing event record) and a `type:"decision"` record (if the intake produces a queue item). If the intake is trivial and produces no queue item, no `type:"decision"` record is needed.

Socratic (question-driven) mode: the human asks a question, the orchestrator presents options per the contract, the human decides. The resolved answer becomes a durable Open Questions decision (AGENTS.md "Question-driven (Socratic) input" section). At that point the orchestrator writes the `type:"decision"` record. There is no prior event record for this mode, so the receipt is the only JSONL trace.

Exploration-then-decision: after a design pass, the orchestrator synthesizes the explorers' proposals into options per the contract, the human decides. The resolved decision moves the item from `exploring` to `decided -> folded into <slug>`. The orchestrator writes the `type:"decision"` record. The exploration file is the prior evidence of the design space; the receipt records the selection from among its synthesized options.

All four modes produce a `type:"decision"` record at the moment a queue item moves to `decided`. The record is always written by the orchestrator, always after the human's response is known, and always contains the options that were presented and the option the human selected.

## 6. Relation to the Open Questions queue and Principle 16

To resolve this directly:

The queue holds: what was decided (the content), the reasoning (in the folded-into step), and provenance (which step owns the decision). It is the human-readable, human-authoritative source for decision content. It is what a future contributor reads to understand why the workflow allows something.

The receipt holds: that the process was followed (the options were enumerated, a recommendation was made, the human selected one). It is the machine-readable, validator-checkable source for process fidelity. It is what `validate --workflow` reads to verify the contract was not short-circuited.

These are not the same thing. A receipt that says "chosen: option A" is not an authoritative record of what option A entails (the step detail carries that). The queue item that says "decided -> folded into escalation-exempt" is not a record of how many options were presented or which was recommended (the receipt carries that). Neither duplicates the other.

The parallel to the established split is exact. The project already accepts that round records in the JSONL log and the ledger narrative are two homes for different aspects of the same events (AGENTS.md "Tracking progress": "the orchestrator records each round in the ledger's round-records narrative" and "ALSO appends a structured `round` record to the round log"). That split is accepted because the narrative serves human reading and the JSONL serves machine processing and calibration. The decision receipt is the same split applied to decisions: the queue serves human reading, the receipt serves machine verification.

The key guard against drift: the `q_id` in the receipt references the queue item. The queue item is authoritative for content; the receipt is authoritative only for attestation. A disagreement between them (receipt says "chosen: A" but the step detail records something different) is a flag for human review, not a validator error; the queue wins on content. The validator checks structural validity of the receipt (options non-empty, chosen is a member of options, q_id is a non-empty string) but does not cross-reference receipt content against step detail prose, which would require parsing narrative text and is out of scope.

## 7. Enforcement: detection, not prevention

Following the W3 pattern (src/workflow.rs line 1-6): W4 is a read-only check, not a runtime gate. The workflow writes decision records and W4 checks them after the fact. No runtime binary dependency on `agent-scaffold` at workflow run time; validation runs in CI, the checks pre-commit hook, or on demand via `agent-scaffold validate --workflow`.

W4's check:

Input: `plan::parse_questions` (src/plan.rs lines 274-309) over the plan file, producing `Vec<Question>`. For each question whose status starts with `QUEUE_FOLD_PREFIX` (src/plan.rs line 87), look for a `type:"decision"` record in the parsed JSONL whose `q_id` matches the question id.

Violation: a `decided` item with no matching record. Problem message: `"Open Questions item Q-42 is decided but has no matching type:decision receipt in the round log"`.

Forward-looking scope: W4 fires only for items decided after the `decision-receipts` Roadmap step is complete. Items decided before that step are implicitly exempt. This is the honest scope: the check cannot audit history it did not witness. No new queue status vocabulary is needed to express this; it falls out of the log's absence for historical items.

The W4 check lives in `src/workflow.rs` alongside W3 (src/workflow.rs lines 77-86), added to `check_workflow`. It reads `metrics::parse_decisions` (a new projection function parallel to `metrics::parse_rounds`, src/metrics.rs lines 372-415) that projects well-formed `type:"decision"` records from the JSONL log. `check_record` in `src/metrics.rs` gains a `"decision"` arm (after line 329, before the `other` arm) that validates the required fields.

## 8. Recommendation

Build Option 2 (the full `type:"decision"` record plus W4). The strongest version is worth the ceremony.

The reasoning against the null option: the human explicitly asked for a guarantee, not just a rule. The workflow already applies this pattern to convergence (W3) and the audit has proven valuable (it caught the `pause.md` step, which was the motivation for W3 in the first place, Q-27). A decision process that cannot be checked for contract compliance is weaker than the rest of the workflow's verification discipline, which is precisely the argument Principle 6 makes.

The reasoning against the light option: it is worse than null on Principle 14 (parse, don't validate). Prose-check-as-enforcement is the wrong shape for machine verification. If it cannot be checked mechanically, it is not a receipt; it is documentation that the orchestrator can write after the fact to satisfy a reviewer without actually having followed the contract in real time.

The honest verdict on the full option's limitations: the receipt is evidence, not adversarial proof. The orchestrator writes both the presentation and the receipt. W4 checks structural properties (options enumerated, selection is from the set, q_id present) but cannot verify the presentation actually happened in the exchange. This limitation is accepted: it is the same limitation W3 faces with round records, and it is the right bar for a workflow tool. The record is auditable by a human reviewer, and the structural check is a deterrent against casual non-compliance.

The ceremony cost: one JSONL append per human decision. This is low. The AGENTS.md contract already requires the orchestrator to enumerate options and make a recommendation; writing the receipt captures what it was already required to produce. The marginal work is the JSON write, not the option enumeration.

The coupling to `escalation-exempt` (Q-40): the `type:"escalation"` record (the W3 exemption signal) and the `type:"decision"` record (the W4 receipt) serve different checks and coexist at a cap escalation that corresponds to a queue item. They do not overlap: the escalation record signals "human authorized convergence despite short streak"; the decision record signals "human was presented options and selected one." Neither is a substitute for the other. The `escalation-exempt` step can be built using only the existing `type:"escalation"` record (W3 reads it); W4 additionally requires a `type:"decision"` record for the queue item that the escalation corresponds to, if it has one.

## 9. What NOT to build (YAGNI boundary)

Do not add a `q_id` field to existing `type:"escalation"` and `type:"intake"` records to make W4 accept them as partial receipts. The existing records are event records, not process receipts. Conflating the two would make `type:"escalation"` serve double duty (W3 convergence exemption AND W4 receipt), violating Principle 16 (one purpose per record). If an escalation or intake corresponds to a queue item, the orchestrator writes both the event record and a separate `type:"decision"` receipt.

Do not require retroactive decision receipts for Q-1 through Q-41. Historical items predate the mechanism. The check is forward-looking; attempting to reconstruct options from git history or step prose would be expensive and unreliable.

Do not add a `receipt-exempt` queue status or any new queue vocabulary to exempt historical items. The forward-looking scope makes exemption vocabulary unnecessary. W4 simply has no records to check for items decided before the mechanism existed.

Do not parse step detail prose to cross-reference receipt content against decision narrative. The `chosen` field in the receipt is a label, not a full record; the queue and step detail remain authoritative for content. Cross-referencing prose to JSONL would require the plan parser to leave its bounded, structured scope (src/plan.rs line 11-14) and is not worth the complexity.

Do not make `type:"decision"` records mandatory for every human interaction. Only interactions that produce a `decided -> folded into <slug>` queue item require a receipt. Factual questions, clarifications, and interactions that do not produce a queue decision do not. The contract applies only where "the human is asking which way to go" (AGENTS.md "Human-input contract" section); W4 applies only where that interaction produced a recorded decision.
