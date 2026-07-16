# Review findings: round-log-core increment B (reviewer: sonnet)

Artifact: ledger template, AGENTS.md convergence prose, orchestrator prompt (diff `eba3c99..HEAD`, branch `impl/round-log-core`).
Files reviewed: `pack/LEDGER.template.md`, `pack/AGENTS.md`, `pack/prompts/orchestrator.md`, `.agents/LEDGER.template.md`, `.agents/AGENTS.reference.md`, `.agents/prompts/orchestrator.md`.

---

## Finding 1 (high): RESUME STATE template comment directly contradicts the de-dup rule

**Location:** `pack/LEDGER.template.md` line 13, `.agents/LEDGER.template.md` line 13.

The RESUME STATE comment still reads:

> "State where the work is: the current step and its status, what is complete, what is next, any open questions awaiting the human, and any workflow rules being applied that are not obvious from the code."

This tells the orchestrator to write exactly what the de-dup rule forbids. The Tracking progress paragraph in `pack/AGENTS.md` says "its RESUME STATE is a pointer into that durable state plus the non-plan-derivable transient in-flight state, not a restatement of the Roadmap." The final paragraph in `pack/prompts/orchestrator.md` says the same. The template comment says the opposite: restate step status, what is complete, what is next, and open questions - all plan-owned content.

This is the implementer's flagged inconsistency, and it is real. An orchestrator reading the template to know what to write in RESUME STATE will write Roadmap content in the ledger, defeating the de-dup rule. The two authoritative prose sources (AGENTS.md, orchestrator.md) are consistent with each other and with the rule; the template - the place the orchestrator actually fills in - contradicts both.

The comment needs to describe RESUME STATE as: a pointer into the plan's current state (naming the active step and which artifact is in review) plus non-plan-derivable transient in-flight state (the current round number, the consecutive-clean streak, any locked context about the in-progress round). It should say not to restate the Roadmap or the Open Questions queue, since those are read directly from the plan. The existing "Flush and commit this before a compaction" sentence is correct and should be kept.

---

## Finding 2 (medium): "Preventing relitigation" paragraph gives an incomplete description of the required narrative fields

**Location:** `pack/AGENTS.md` and `AGENTS.md`, "Preventing relitigation" paragraph (the sentence beginning "recording each round in the round-records narrative...").

The paragraph summarises what goes in the narrative as: "what was reviewed, the reviewers and separate triager that ran with their findings-file paths, the verdicts, and each round's outcome (new valid findings, or clean)."

This omits three fields that the orchestrator prompt's step 1 and/or the template explicitly require:

- The artifact's risk classification (in both the template and step 1).
- Whether the artifact changed since the previous round (in step 1 but not in the template - see Finding 3).
- The running consecutive-clean streak (explicitly named in step 1; implied but not named in the template via "the convergence decision").

This paragraph is in AGENTS.md, the primary reference document. A reader who relies on it to understand what the narrative must contain will produce incomplete entries that omit the streak, which is the field most critical for recomputation after a compaction. The Tracking progress paragraph does say "counts the consecutive-clean streak and the total-round count from that narrative," which implies the streak must be there, but the Preventing relitigation paragraph's enumeration of required content is where an orchestrator will look for the concrete spec, and it is incomplete.

---

## Finding 3 (medium): Template omits "whether the artifact changed since the previous round" from the per-round narrative requirements

**Location:** `pack/LEDGER.template.md` and `.agents/LEDGER.template.md`, line 9 (Round records comment).

Orchestrator step 1 explicitly requires "whether the artifact changed since the previous round" in the per-round narrative. The template's Round records comment does not include this field. The template lists: what was reviewed; risk classification; reviewers and triager with findings paths; verdicts; round outcome; convergence decision. The "changed since prev" field is absent.

This field was previously the "Changed since prev" column in the removed round-summary table and serves as evidence that reviewers looked at fresh output rather than re-reviewing unchanged content. While it is not needed to recompute the streak or total-round count, it is part of the audit trail and was intentionally preserved in the orchestrator prompt. The template and the orchestrator prompt are the two canonical sources for what to write per round; having the template omit a field the orchestrator prompt requires creates a gap where an orchestrator following only the template will not record it.

---

## Finding 4 (low): Risk classification tokens in the template use a different form than the prose in AGENTS.md and the orchestrator prompt

**Location:** `pack/LEDGER.template.md` line 9 (and `.agents/` copy).

The template says: "the artifact's risk classification (`low_risk` needs one clean round to converge, `risky` / high-blast-radius needs two)."

The backtick-quoted `low_risk` and `risky` suggest these are the tokens to write in prose. But everywhere else in the pack (AGENTS.md, orchestrator.md) the same classes are described as "low-risk" and "risky or high-blast-radius" in natural language. The JSONL `risk_class` field does use `low_risk` and `risky` as code values, so the template appears to be trying to align prose notation with the structured field, but this creates an inconsistency: different orchestrators following different sources will write the classification in different forms in their ledger narratives (some "low_risk", some "low-risk", some "LOW-risk" as in the existing ledger). For a re-spawned orchestrator reading a ledger written by a different session, mixed forms are harmless but represent a spec gap.

---

## Finding 5 (low): Stale "round-summary" references in the plan and live ledger (step C scope, noted for awareness)

**Location:** `docs/plans/agent-scaffold.md` lines 407, 411, 413, 560, 566, 603; `docs/plans/agent-scaffold.ledger.md` line 25.

After increment B removes the round-summary and findings-index tables, multiple references to "round-summary" and "round-summary lines/table/state" remain in the plan's step narratives and in the live ledger. These are explicitly assigned to increment C ("C, orchestrator-owned: slim the live ledger and this plan"), so they are out of B's scope and not a defect in this increment. Flagging because a resuming orchestrator reading the plan now will encounter stale references before C runs, which may cause confusion. Increment C should clear all of them.

Additionally, `src/plan.rs` line 13 (unchanged by this PR) says "the ledger's round-summary narrative is out of scope for this increment." After B removes the round-summary table, the phrase "round-summary narrative" no longer refers to anything that exists; the round-records narrative is a different thing. This is a pre-existing code comment that increment B has made slightly more ambiguous. Not a defect introduced by B, but worth noting for cleanup.

---

## No findings

- Scope: no scope creep detected. Exactly the seven files specified in increment B's plan description were changed; `--instrument` is correctly left as opt-in throughout, with consistent "When instrumentation is on..." framing in all new text. The narrative is the always-present counting source; the JSONL is the opt-in superset. No accidental always-on framing found.
- The convergence accounting is fully recomputable from the narrative, given the required fields (what was reviewed, outcome per round, artifact scope, risk classification, and the explicit streak in the orchestrator step 1). The total-round count is derivable by counting narrative entries, which is adequate.
- The de-dup rule is stated consistently and without contradiction in AGENTS.md and orchestrator.md. The inconsistency is only in the template comment (Finding 1).
- No dangling "see the round-summary table" references in the changed files. README has no references to the removed tables.
- The backstop re-check mechanism and the "changed since prev" amend instruction are correctly updated from "round-summary outcome" to "round outcome in the narrative."
- No critical findings.
