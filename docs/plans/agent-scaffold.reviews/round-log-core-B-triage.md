# Triage: `round-log-core` increment B

Triager: independent (separate from producer, the two reviewers, and the orchestrator). Branch `impl/round-log-core`, HEAD `a9e08b0`, diff `eba3c99..HEAD`. Artifact: increment B, the RISKY core reshape (removed the ledger template's round-summary/findings tables + Artifact-classification line; rewrote convergence accounting to count from the ledger Round-records NARRATIVE; folded in the plan/ledger de-dup rule; opt-in `--instrument` preserved).

Inputs adjudicated:
- `round-log-core-B-reviewer-opus.md` (correctness): M1.
- `round-log-core-B-reviewer-sonnet.md` (clarity): F1 (high), F2/F3 (medium), F4/F5 (low).
- The implementer's own flagged RESUME STATE tension.

Judged against `pack/AGENTS.md` Principles, in particular Principle 16 "One source of truth" (keep one authoritative source per piece of data and derive the rest). Line length is never a finding and none was raised. Scope, opt-in preservation, Q-1 semantics, mechanicals: confirmed by opus and re-checked here (plan line 599 authorizes exactly B's template + prose edits; `--instrument`/`{{instrument}}`/`instrument.md` untouched; the JSONL `risk_class` code values live in `instrument.md`, unchanged).

Verdict summary: 4 VALID (fix in B), 1 VALID-but-DEFERRED-to-C, 0 DISMISSED. The RESUME-STATE finding is deduplicated across opus + sonnet + implementer into a single finding (T1).

---

## T1 (VALID, fix in B, severity HIGH) - RESUME STATE authoring comment contradicts the de-dup rule this increment adds

Sources deduplicated: opus M1 (medium) + sonnet F1 (high) + implementer's flagged tension. One finding.

Location: `pack/LEDGER.template.md:13` (and the regenerated `.agents/LEDGER.template.md:13`), the `## RESUME STATE` section authoring comment.

The increment introduces the de-dup rule in three places it rewrote: `pack/AGENTS.md:55` (Tracking progress), `pack/prompts/orchestrator.md:31`, and the template intro comment at line 3, all stating "its RESUME STATE is a pointer into that durable state plus the non-plan-derivable transient in-flight state, not a restatement of the Roadmap." The template's own `## RESUME STATE` authoring comment (line 13, not touched by the increment) still instructs the opposite: "State where the work is: the current step and its status, what is complete, what is next, any open questions awaiting the human, and any workflow rules being applied that are not obvious from the code." Three of those four items (current step and status; what is complete / what is next; open questions) are exactly the Roadmap / Open-Questions restatement the new rule forbids; only the last (non-obvious workflow rules being applied) is the transient the rule keeps.

This is an internal contradiction inside B's own deliverable and a direct Principle 16 violation: the template is the operative artifact the orchestrator copies and fills in, so a contradictory authoring comment defeats the de-dup discipline the increment exists to establish. It does not break the convergence-counting mechanism (the counts recompute from the separate Round-records narrative), which is why it is not critical.

Severity reconciliation (opus medium vs sonnet high): I side with sonnet, HIGH. The two prose sources being consistent does not rescue it, because the orchestrator authors RESUME STATE from the template, not from the prose; the one wrong source is the operative one. Left unfixed, the contradiction ships to every downstream task and the discipline fails in practice, not once. It is a one-file edit in a file B already rewrites, so there is no cost argument to defer. Opus's medium is defensible on the "no mechanism breakage" axis, but the impact-if-unfixed is a permanent, shipped self-contradiction that negates a stated increment goal, which reads as high.

Owner: isolated implementer (worktree/pack fix).

Recommended action: rewrite the `## RESUME STATE` authoring comment (line 13) to describe RESUME STATE as: the first thing to read on resume; a pointer into the plan's current durable state (the active step and which artifact is in review) plus the non-plan-derivable transient in-flight state (the current round number, the consecutive-clean streak, any locked context about the in-progress round); explicitly NOT a restatement of the Roadmap or the Open Questions queue, which are read directly from the plan. Keep the existing "Flush and commit this before a compaction (see the checkpoint procedure in `AGENTS.md`)" sentence. Then run `just scaffold-self` so `.agents/LEDGER.template.md` matches, and confirm `git status --short` is clean.

Scope boundary: keep this fix to the template authoring comment only. The full RESUME-STATE slimming and the repointing of `resume.md` / `compaction-prep.md` are deferred to `state-queries` (plan), and the live-ledger RESUME STATE slim is increment C; neither is a B fix.

---

## T2 (VALID, fix in B, severity LOW; reviewer rated medium) - "Preventing relitigation" paragraph enumerates the narrative fields incompletely

Source: sonnet F2 (medium).

Location: `pack/AGENTS.md:57` (and regenerated `AGENTS.md` / `.agents/AGENTS.reference.md`), "Preventing relitigation" paragraph.

The paragraph enumerates the per-round narrative as "what was reviewed, the reviewers and separate triager that ran with their findings-file paths, the verdicts, and each round's outcome (new valid findings, or clean)." It omits the running consecutive-clean streak, the risk classification, and changed-since-prev, all of which the authoritative sources require (`pack/AGENTS.md:55` Tracking progress names the streak; `pack/prompts/orchestrator.md:17` step 1 names all three; the template Round-records comment names classification and streak).

Severity correction (medium -> LOW): the paragraph explicitly delegates the field list to the template ("recording each round in the round-records narrative the scaffolded template pins (see below)"), so it is a summary, not the authoritative field spec; the operative sources the orchestrator follows (step 1, the template, the adjacent Tracking-progress paragraph) are complete and name the streak. Impact if unfixed is a mildly incomplete summary in the reference doc, not data loss or a broken recomputation. I considered DISMISSING it on the "it is a summary that defers to the template" reading, but it lists a partial field set (implying a spec) while omitting the streak, which is the recomputation-critical field named one line above in the same document; that mismatch is worth removing, and the fix is cheap in a paragraph B already rewrote.

Owner: isolated implementer (pack fix; `AGENTS.md` and `.agents/AGENTS.reference.md` regenerated).

Recommended action: extend the enumeration to name the running consecutive-clean streak (mandatory; it is the recomputation-critical field) and the artifact's risk classification, so the summary is consistent with the adjacent Tracking-progress paragraph and step 1. Minimal edit, for example: "... the verdicts, the artifact's risk classification, each round's outcome (new valid findings, or clean), and the running consecutive-clean streak." changed-since-prev is handled in the template (T3) and need not be added to this summary. Regenerate and confirm clean.

---

## T3 (VALID, fix in B, severity LOW; reviewer rated medium) - Template Round-records comment omits "changed since previous round"

Source: sonnet F3 (medium).

Location: `pack/LEDGER.template.md:9` (and regenerated `.agents/LEDGER.template.md:9`), the Round-records comment.

Orchestrator step 1 (`pack/prompts/orchestrator.md:17`) explicitly requires "whether the artifact changed since the previous round" in the per-round narrative, and the JSONL schema carries `changed_since_prev` (`pack/instrument.md:5`). The template's Round-records comment lists what-was-reviewed, risk classification, reviewers/triager + paths, verdicts, outcome, and the convergence decision, but omits changed-since-prev. This field was previously the "Changed since prev" column in the removed round-summary table; dropping the table without re-adding it to the narrative comment is a genuine audit-trail regression, and the template and step 1 are the two canonical per-round-authoring sources, so the divergence is a real spec gap in B's own deliverable.

Severity correction (medium -> LOW): as sonnet concedes, this field is not needed to recompute the streak or the total-round count; it is audit-trail evidence that reviewers looked at fresh output. The operative instruction (step 1) still requires it, so an orchestrator following the full prompt records it regardless. The value of fixing is template/step-1 parity, which is a low-severity consistency fix, not a defect that risks convergence or data loss.

Owner: isolated implementer (pack fix; template regenerated).

Recommended action: add "whether the artifact changed since the previous round" to the template Round-records comment, near what-was-reviewed / risk classification, matching step 1. Regenerate `.agents/LEDGER.template.md` and confirm clean.

---

## T4 (VALID, fix in B, severity LOW) - Risk-classification tokens in the template use code form in prose

Source: sonnet F4 (low).

Location: `pack/LEDGER.template.md:9` (and `.agents/` copy).

The template writes "the artifact's risk classification (`low_risk` needs one clean round to converge, `risky` / high-blast-radius needs two)." The backticked `low_risk` matches the JSONL `risk_class` code value in `instrument.md`, but the Round-records comment governs the NARRATIVE prose, not the JSONL. Everywhere else in the pack the classes are written in natural language: `pack/AGENTS.md` and `pack/prompts/orchestrator.md:19` use "low-risk" and "risky or high-blast-radius." Putting the code tokens in backticks in a prose-authoring comment invites orchestrators to write the code form in the narrative, forking the notation (the existing live ledger already has a third form, "LOW-risk").

Judgment call, ruled FIX. The template is being rewritten in B and the inconsistency is with B's own sibling files; the JSONL keeps `low_risk`/`risky` as code values in `instrument.md` (correct there, untouched). Accepting it as-is would leave a self-inconsistent notation in the new template; fixing is cheap and removes the fork.

Owner: isolated implementer (pack fix; template regenerated).

Recommended action: in the template Round-records comment, drop the backticks and use the natural-language forms to match the prose sources, for example "the artifact's risk classification (low-risk needs one clean round to converge, risky or high-blast-radius needs two)." Leave the `instrument.md` `risk_class` code values unchanged. Regenerate and confirm clean.

---

## T5 (VALID but DEFERRED-to-C, severity LOW) - Stale round-summary references in the plan and the live ledger

Source: sonnet F5 (low). Opus also flagged the live ledger's `## Round summaries` / `## Findings` tables as correctly out of B scope.

Locations: `docs/plans/agent-scaffold.md` lines 113, 289, 407, 411, 413, 560, 566, 603 (references to the removed round-summary table / lines / schema); `docs/plans/agent-scaffold.ledger.md:25` (`## Round summaries`) and `:112` (`## Findings`).

These are real stale references after B removes the tables, but they are explicitly assigned to increment C in the plan (line 599: "(C, orchestrator-owned) slim the live `docs/plans/agent-scaffold.ledger.md` (remove the stale tables, slim RESUME STATE) and this plan"). They are NOT in the B worktree's deliverable (B rewrites `pack/` + regenerated `.agents/`/`AGENTS.md`; the plan and live ledger are orchestrator-owned). So this is valid-but-deferred, not a B fix.

Owner: orchestrator (increment C).

Recommended action (C): when slimming the live ledger and the plan, clear every listed stale round-summary reference. Note that several plan hits (lines 113, 289, 407, 411, 413, 560, 566) sit inside historical / decided-outcome narrative describing prior module scopes; C should judge which are live guidance that must be corrected versus frozen historical record, rather than blanket-rewriting.

Related cleanup (DEFERRED, severity LOW, owner orchestrator to route): `src/plan.rs:13` comment says "the ledger's round-summary narrative is out of scope for this increment." After B removes the round-summary table, "round-summary narrative" no longer names anything (the Round-records narrative is a distinct thing). This is a pre-existing code comment unchanged by B (src/plan.rs is not in B's diff), so it is not a B defect; sonnet flagged it as slightly more ambiguous post-B. Route it as a low cleanup note (C, or a follow-up nit) to reword to "the ledger's round-records narrative" or drop the sentence.

---

## Confirmations (no finding)

- Q-1 convergence semantics preserved (source moved table -> narrative only): confirmed by opus and re-checked. Two-way clean / new-valid partition, per-artifact consecutive-clean streak, reset on new artifact/step, dismissal-recheck amendment, total-round cap (default five) with the "convergence check applies first" rule, all intact across `pack/AGENTS.md:47-55`, orchestrator steps 2-3, and the template.
- Opt-in preserved: `--instrument`, `{{instrument}}`, `instrument.md` untouched (not in diff); every JSONL mention is conditionally gated ("When instrumentation is on, ... ALSO append"); the core counting reads the narrative, stated consistently in AGENTS.md, orchestrator steps 1-3, and the template.
- No leftover table references in the changed files (opus's grep, re-checked). The de-dup rule aligns with, and does not contradict, the Roadmap-single-source line, the Open-Questions queue, and the checkpoint/resume anchor at `pack/AGENTS.md:87-90`.
- Mechanicals (opus): `just test` 96 passed; `just clippy` clean; `validate --plan --metrics` exit 0. After the T1-T4 pack edits, the implementer must re-run `just scaffold-self` and confirm `git status --short` is clean so the regenerated `.agents/`/`AGENTS.md` stay in sync.
