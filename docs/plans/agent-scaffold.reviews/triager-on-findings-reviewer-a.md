# Consistency/completeness review: triager-on-findings (`plan/triager-on-findings`, commit `423bf9b`, base `cfeec01`)

Lens: consistency and completeness of the workflow-rule rewrite that makes the separate triager run only on review rounds with one or more findings, with the never-collapse property preserved for rounds where the triager does run.

## Findings

### F1 (medium): Review-reports paragraph still implies a triager findings file always exists, contradicting the new zero-findings review path

Location: `pack/AGENTS.md` line 69 (mirrored byte-identically in `AGENTS.md` line 69 and `.agents/AGENTS.reference.md` line 69). Quoted: "A review-entry-mode run (the review mode above) terminates in a single committed report at `docs/plans/<task>.review-report.md`, which the orchestrator synthesises from the reviewers' and the triager's findings files".

The same change edited the review-entry-mode paragraph (line 47) to add the carve-out: "a separate triager when the reviewers raise one or more findings; a zero-findings review produces a clean report with no triage step". So the change itself establishes that a zero-findings review has no triager and therefore no triager findings file. The Review-reports paragraph was not updated to match: it describes the report as synthesised from "the reviewers' and the triager's findings files", which asserts a triager findings file as an input to every review-mode report. For a zero-findings review there is no triager findings file, so the input description is false for exactly the case the change introduced. This is a self-contradiction across two paragraphs about the same review entry mode, and it is a leftover always-run implication of the kind this change set out to remove (the triager producing a findings file on every review pass, including clean ones).

Practical impact is limited (on a clean review the report body, the triager-valid findings, the dismissed findings, and the kickoff tasks, is trivially empty, so an orchestrator would write a clean report regardless), which is why this is medium rather than high. But the guidance as written is internally inconsistent: the acceptance/review path is precisely the path this reviewer was asked to trace for a dangling triager dependency on a clean round, and this is one. A minimal fix would qualify the input clause, for example "from the reviewers' findings files and, when a triager ran, its findings file", parallel to the conditional phrasing the change already uses in phases 3, 4, 5, the ledger line, and the orchestrator prompt.

## Locations checked and found consistent (no finding)

Every triager-mention location on the branch was read in full context via `git show 423bf9b:<path>`. The following now correctly reflect the findings-only rule with no leftover always-run phrasing:

- Roles-separation paragraph (`pack/AGENTS.md` line 15): correctly scoped, "on any review round that has one or more findings it is never merged ...; a round where every reviewer reports zero findings is already clean and has no triage step". Never-collapse preserved.
- Triager role bullet (`pack/AGENTS.md` line 22): full new rule present, including the objective "Zero findings" read from the committed reviewer findings files and the never-collapse restatement for when it does run.
- Phase 3 (line 31): triager conditional on findings, with the zero-findings skip stated.
- Phase 4 (line 32): triager conditional on findings.
- Phase 5 (line 33): triager conditional on shortfalls; the documentation-currency-check sentence is intact and the backstop reference ("it keeps the high/critical dismissal re-check (the backstop below)") is intact; the zero-shortfalls-done vs shortfalls-then-triager branch reads cleanly.
- Review-entry-mode paragraph (line 47): conditional triager and zero-findings clean report stated.
- Convergence-decision line (line 53): "After each review round, the orchestrator decides from the round's outcome (the triager's verdicts when the round had findings, or the reviewers' zero findings when it was clean) and the round count". Not over-wordy, not self-contradictory; correctly covers the zero-findings case.
- Backstop paragraph (line 59): unchanged and intact; concerns a dismissed finding, which exists only when there are findings, so it is not weakened.
- Ledger round-record line (`pack/AGENTS.md` line 63) and orchestrator-prompt round-record line (`pack/prompts/orchestrator.md` line 19): agree with each other ("the reviewers that ran and, on a round with findings, the separate triager that ran, with their findings-file paths" vs "which reviewers ran and, on a round with findings, which separate triager ran, with their findings-file paths") and read well.
- Authored-conflict-resolution rule (`pack/AGENTS.md` line 95): now "through a reviewer and, if that review raises one or more findings, a separate triager that is never collapsed into the producer or the orchestrator (the Triager role above)". No longer implies an always-run triager; never-collapse kept. The old "always-separate-triager rule" name was dropped, and `git grep "always-separate-triager"` over `pack .agents AGENTS.md` returns no hits, so no dangling reference was left.
- Orchestrator prompt roles line (`pack/prompts/orchestrator.md` line 5), review-round ordering line (line 11), and acceptance line (line 25): all carry the findings-conditional and the zero-findings skip; never-collapse preserved.
- Findings-files naming rule (`pack/AGENTS.md` line 67) and the Preflight "separate-triager rule" reference (line 104): describe mechanism and name the rule, not a per-round frequency; not weakened by the change.
- `pack/prompts/triager.md`: not touched by the change and carries no "every round" / always-run claim; consistent with the new rule.

Searches run: `git grep "always-separate-triager"` (no hits), `git grep "review-then-triage\|reviewers-then-triage round"` (no hits, the convergence line was correctly renamed), and `git grep "for every review round"` (no hits). No leftover always-run phrasing was found other than F1.

## Never-collapse, backstop, findings-naming

The never-collapse property is preserved everywhere the triager can run (roles paragraph, Triager bullet, worktree-lifecycle line 93, authored-conflict-resolution line 95, orchestrator roles line 5). The backstop re-check (dismissed high/critical) and the findings-files naming rule are unchanged and not weakened; both concern findings that exist only when there are findings, consistent with the carve-out.

## Logic-hole trace

Convergence path: the convergence-decision line, phase 3, and phase 4 now derive the round outcome from the triager's verdicts when there were findings, or from the reviewers' zero findings when clean, so a clean round no longer depends on any triager output. No dangling triager dependency on the convergence path.

Acceptance/review path: phase 5 and the review-entry-mode paragraph both branch on zero shortfalls/findings without a triager. The one dangling triager dependency on a clean run is F1 (the Review-reports paragraph naming the triager's findings file as an input to every review-mode report).

## Source-vs-generated and validation

- Mirroring: the `pack/AGENTS.md` triager edits are byte-identical to the changes in `AGENTS.md` and `.agents/AGENTS.reference.md` (verified by grep line-for-line and by the byte-guard tests); the `pack/prompts/orchestrator.md` edits mirror `.agents/prompts/orchestrator.md`.
- `cargo test`: all pass (342 in the main suite plus the integration suites; 0 failed), including the pack-vs-generated byte-guard drift tests.
- `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow`: exit 0, "workflow invariants hold" (run against the review branch base plan; the review branch is based on `cfeec01`, so only `pack`, `.agents`, and `AGENTS.md` were checked out from `423bf9b` per the validation command).
- `cargo run -- render --check docs/plans/agent-scaffold.plan.toml`: exit 0, "up to date".
- Step-count cross-check: the branch `423bf9b` `plan.toml` has 78 `[[step]]` entries and its prose status reads "78 steps", matching (base had 77); no miscount introduced by the change.

## Prose (added text)

No em-dashes, en-dashes, double-hyphen dashes, unicode, or emoji found in the added/changed prose across `pack/AGENTS.md`, `AGENTS.md`, `.agents/AGENTS.reference.md`, `pack/prompts/orchestrator.md`, `.agents/prompts/orchestrator.md`, the new step sidecar, and the `Q-63` question body. ASCII only; no hard-wrapping; no AI filler phrasings. Line length not raised.
