# Triage verdicts: uniform-agent-isolation (Q-61), diff `1ce9de3..600e6a3`

Triager run isolated in its own worktree (`triage/ui`), independent of the writer and the orchestrator. Each finding was re-verified against the artifact at commit `600e6a3`; the reviewers' write-ups were not trusted on their own. Severity is the four-level `low`/`medium`/`high`/`critical` scale from `AGENTS.md:21` (an absolute rating of impact if left unfixed).

## Finding 1 (Reviewer A High-1): shipped pack file `pack/isolation-guidance.md:37` still asserts the retired read-only carve-out

- Verdict: VALID.
- Severity: high (uphold Reviewer A's rating).
- In scope: yes, in scope for THIS change; a required fix before convergence.
- Verification:
  - Confirmed verbatim at `pack/isolation-guidance.md:37`: "Read-only agents (reviewers, the triager, explorers) need no isolation and run without a container or a worktree of their own." The writer-only framing at line 30 ("prefer the `--git` sandbox for writer agents") and line 37 ("whether a writer will run in a container, a worktree, or under the file-safety fallback") is also present.
  - Confirmed it ships: `pack/pack.toml:22-25` declares the `isolation` module with `guidance = "isolation-guidance.md"`; per the manifest header a module's `guidance` partial concatenates into the `{{modules}}` render slot when `--module isolation` is selected. So this file is emitted product (conditionally, on module opt-in), not internal scaffolding.
  - Confirmed untouched: `git diff 1ce9de3..600e6a3 --stat` does not list `pack/isolation-guidance.md`; it is empty in the diff.
- Reasoning: Q-61's explicit success condition is that "The tool's emitted instructions state this" (`docs/plans/agent-scaffold.md:900`). When `--module isolation` is selected the user gets, in the SAME scaffold drop, `AGENTS.md:83,89,91` saying every spawned agent isolates and `isolation-guidance.md:37` saying read-only agents need none, a flat contradiction on the single most on-topic file a reader consults for isolation. The file even frames itself as NOT restating the rule ("It does not restate that rule ... it supplies only the setup", line 3), yet line 37 restates a who-isolates claim that contradicts the rule, overstepping its own stated scope. This is a live faithfulness gap in the product the change exists to deliver, so it is in scope and high. The module-gating (only emitted under `--module isolation`) lowers reach and is a mild mitigant, but does not lower the tier: where it is emitted, the contradiction is stark and lands on the exact topic. Reviewer B's faithfulness sweep (F-verification, "the human's requirement that the tool instructions state it is met") checked `AGENTS.md` and the three prompts but MISSED this fourth emitted file; that conclusion is therefore incomplete. No new finding is created, since Reviewer A already caught it.

## Finding 2 (Reviewer A High-2 == Reviewer B F1, deduplicated): explorer classified as both writer and read-only; Q-61 receipt claims a resolution the shipped text does not carry

- Verdict: VALID (single finding; A and B raised the same issue).
- Severity: high. I uphold Reviewer A's HIGH over Reviewer B's MEDIUM.
- In scope: yes; this change introduced the contradicting lines (`AGENTS.md:83,89,91`) and wrote the receipt. A required fix before convergence.
- Verification:
  - `AGENTS.md:65` (unchanged by the diff): "Explorers are writer agents, so the file-safety and writer-isolation rules below apply to them." Same paragraph gives an exploration a review pass ("a single light review pass (one reviewer, one triager ...)").
  - `AGENTS.md:83` (added): "the writers and the read-only reviewers, triager, and explorers alike".
  - `AGENTS.md:89` (added): "a read-only agent (a reviewer, the triager, or an explorer) is read-only with respect to that product".
  - `AGENTS.md:91` (added): "the read-only reviewers, triager, and explorers alike".
  - The roles anchor at `AGENTS.md:25` (per grep) is silent on explorers, so the definitional anchor does not resolve the conflict.
  - Q-61 receipt (`docs/plans/agent-scaffold.plan.toml` Q-61 `ask`, mirrored in the plan Open-Questions entry): "RESOLVES the explorer-classification inconsistency (explorers are writers that isolate ... so they are no longer listed among read-only agents needing no worktree)." Step detail `docs/plans/agent-scaffold.md:901`: "explorers were already called writer agents that isolate ... the uniform rule removes that contradiction." The shipped text does the reverse: it keeps line 65 AND newly lists explorers among the read-only agents.
  - The same contradiction exists in the pack source (`pack/AGENTS.md:65` vs its edited paragraphs) and is inherited by `src/isolation_policy.rs` via line 91's wording, so it is not a render artifact.
- Reasoning for upholding HIGH over MEDIUM: Reviewer B's medium rests on "isolation behaviour is unaffected (explorers isolate either way)," which is correct as far as it goes, an explorer isolates under either label, so there is no illegal state or behavioural divergence, which is why this is below critical. But this is not merely a stale label. Two things push it past a medium labeling nit. (1) `AGENTS.md`, the normative governing document, now asserts an explorer is both "a writer agent" (`:65`) and "a read-only agent" (`:89`) in one shipped document, and the terms-defining roles anchor (`:25`) is silent on explorers, so a reader cannot determine which classification governs even at the definitional anchor. A self-contradiction in the governing doc erodes its standing as the single normative source. (2) The Q-61 decision receipt, a durable W4-enforced record, asserts a resolution that is the reverse of what shipped; a false decision receipt is a durable-record integrity defect, exactly what the receipt discipline exists to prevent, and it touches the product-authority reframe, one of the five load-bearing parts of the human's decision (Reviewer B lists it as such). The combination of a self-contradictory governing document plus a false durable receipt on a load-bearing decision exceeds medium. Reviewer A's line 65 counterpoint (an exploration IS reviewed by a reviewer and triager and IS synthesised into the decision, which undercuts the "output never reviewed" rationale line 89 gives for read-only) is corroborated by the text at `AGENTS.md:65` and reinforces that the contradiction is substantive, not cosmetic.
- Fix direction: DEFERRED to orchestrator/human, not decided here. Two coherent directions exist and this triage does not pick between them:
  - (a) Make explorers read-only everywhere, consistent with the product-authority reframe (an explorer authors advisory design notes, never accepted as product, like reviewer findings): update `AGENTS.md:65` (and `pack/AGENTS.md`), the optional review-pass sentence, and the Q-61 receipt/step-detail text to match.
  - (b) Keep explorers as writers (matching line 65, the receipt, and the review-pass mechanics) and drop "explorers" from the read-only groupings at `AGENTS.md:83,89,91` and the isolation fragment, stating instead that explorers isolate as writers.
  Both A and B lean toward direction (a) on the merits, but they lean in opposite ways on which side is "already correct," so this is a genuine open decision for the human/orchestrator. The triage verdict is validity + severity only.

## Finding 3 (Reviewer A Low): historical/record prose still spells the retired carve-out

- Verdict: VALID as an observation (the lines do state the retired carve-out), but OUT OF SCOPE as a required fix; not a convergence blocker.
- Severity: low (uphold Reviewer A).
- In scope: no. These are frozen historical records of a completed step and a kept design document, not the live normative rule.
- Verification:
  - `docs/plans/agent-scaffold.md:880`: "spawn read-only REVIEWERS that need no isolation, contradicting `pack/AGENTS.md` and `orchestrator.md`." This sits inside the `driver-output-generation` step's "Decided scope" narrative, a record of that completed step's reasoning.
  - `docs/plans/agent-scaffold.steps/driver-output-generation.md:8`: identical sidecar text.
  - `docs/plans/driver-output-generation.design.md:69,86,87`: "read-only agents need none" / "read-only ... need no isolation", inside the kept design document that reasons about the driver-output-generation step.
  - All three confirmed untouched: `git diff 1ce9de3..600e6a3 --stat` lists none of them.
- Reasoning: these describe the rationale as it stood at the driver-output-generation step, which is done. The live normative rule is in `AGENTS.md`, and it WAS updated. Rewriting these records to the new rule would falsify the history of what that step decided, so they should NOT be edited. Reviewer A's own suggested fix agrees (leave the design record as-is; at most add an optional "(superseded by Q-61)" marker on the live step-detail bullet). That optional marker is discretionary, not required for convergence. Correctly ruled low and out of scope.

## Reviewer-coverage note (not a new finding)

Reviewer B concluded the "tool instructions state it" requirement is met after checking `AGENTS.md` and the three prompts, but did not examine `pack/isolation-guidance.md`, which is also emitted (under `--module isolation`) and contradicts the rule. Reviewer A caught it (Finding 1). No new finding is raised; this only records that B's faithfulness sweep was incomplete on that one file.

## Round outcome

NEW-VALID. Two findings must be fixed before convergence:

- Finding 1 (high, in scope): rewrite `pack/isolation-guidance.md:37` (and soften the writer-only framing at 30/37) to the uniform rule.
- Finding 2 (high, in scope): resolve the explorer writer-vs-read-only contradiction across `AGENTS.md` / `pack/AGENTS.md` / the isolation fragment and the Q-61 receipt + step-detail, in ONE direction. The direction (a: read-only everywhere, or b: writer everywhere) is a DEFERRED orchestrator/human decision, not settled by this triage.

Finding 3 (low) is valid-as-observation but out of scope and NOT a convergence blocker; leave the historical/design records unedited (an optional "superseded by Q-61" marker on the live step bullet is discretionary).
