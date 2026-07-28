# Step 92 `prompt-drift-guard`: triage verdicts (work review, round 1)

Artifact: `git diff 852a8c4..8012e05`, a single-file change to `src/agents_md_drift.rs` (+121 / -18).
Brief: `docs/plans/agent-scaffold.steps/prompt-drift-guard.md`.
Findings triaged: `prompt-drift-guard-reviewer-falseneg.md` (`FN-1`, `FN-2`, `FN-3`) and `prompt-drift-guard-reviewer-contract.md` (`CT-1`). All four were rated `low` by their reviewers.
Worktree: `.claude/worktrees/triage-pdg`, detached at `8012e05`. Every mutation below was reverted with the Edit tool (never `git checkout`); `git status --short` and `git diff` are clean at the end, with only this findings file untracked.

## Summary

| Finding | Reviewer severity | My verdict | My severity |
| --- | --- | --- | --- |
| `FN-1` | low | VALID, doc-only fix | low |
| `FN-2` | low | VALID, doc-only fix; mechanism ACCEPTED AS RESIDUAL | low |
| `FN-3` | low | VALID BUT ACCEPT RESIDUAL (no implementer work) | low |
| `CT-1` | low | VALID, doc-only fix | low |

Every finding's demonstration reproduced. None was dismissed. Three findings require an implementer fix, all comment-only in `src/agents_md_drift.rs`, with no behaviour change and no new test.

BACKSTOP: NOT triggered. No finding is rated high or critical by its reviewer or by me, and I dismissed nothing at any severity, so no second independent triager is required.

## Baseline and spot-checks I ran before ruling

Baseline at `8012e05`: `cargo test` 367 + 5 + 1 + 3 + 1 + 2 = 379 passed, 0 failed. `cargo clippy --all-targets -- -D warnings` clean. Both match the contract reviewer's report.

The contract reviewer's MET verdicts are load-bearing for eventual convergence, so I re-ran them myself rather than accepting them. All three reproduce:

1. PACK EDIT FAILS, NAMING THE FILE. Edited `pack/prompts/triager.md:3` ("You adjudicate review findings." -> "You adjudicate the review findings.") with the deployed copy left stale. `cargo test --bin agent-scaffold the_committed_role_prompts_match_a_fresh_render` FAILED at `src/agents_md_drift.rs:417` with "`.agents/prompts/triager.md` has drifted from a fresh render of the pack's prompts ... Edit the pack source, not the copy, then run `just scaffold-self`". The message names the file, both causes, and the fix. Reverted.
2. HAND EDIT OF A COMMITTED COPY ALSO FAILS. Edited `.agents/prompts/open-questions-gate.md:9` ("the next steps." -> "the next step.") with the pack left alone. FAILED, naming `.agents/prompts/open-questions-gate.md`. The two-way correspondence is real in both directions. Reverted.
3. A PRETTIER REFLOW DOES NOT FAIL. Replaced the same line with a soft-wrapped two-line form (`git diff --stat` = 2 insertions / 1 deletion, no content change). PASSED. Reverted.

Independent config check: rendered the real justfile config to a scratch directory (`cargo run -- scaffold --output-dir <tmp> --write --force --principles default --instrument`) and diffed every emitted file against its committed copy. 31 emitted files (excluding the `.git` the scaffold initialises in the output dir), 31 SAME, 0 DIFFERS. This confirms both the contract reviewer's evidence 4 and the falseneg reviewer's avenue 3.

CHANGELOG-convention claim: reproduces. `git log --format='%h %s' -- CHANGELOG.md` returns 14 commits; none has a `test:` prefix. Omitting a CHANGELOG line for this test-only change is the convention.

## `FN-1`: the module doc says "a prompt added to the pack is guarded"; only a prompt added to the pack MANIFEST is

VERDICT: VALID. Severity `low` (reviewer's rating CONFIRMED). DOC defect, not a mechanism defect. Doc-only fix required.

EVIDENCE REPRODUCED: yes. Created `pack/prompts/experimental.md` with no `[[asset]]` row in `pack/pack.toml`, then ran `cargo test`: 367 + 5 + 1 + 3 + 1 + 2 passed, 0 failed. The whole suite is green with an unregistered prompt sitting in the pack directory. File removed; tree clean.

The mechanism claim is also correct as stated: `pack/pack.toml` carries seven `[[asset]]` rows with a `dest` under `.agents/prompts/` (plus the module-gated `checks-reviewer`), and the derived set comes from the render of those rows. `include_dir!` at `src/manifest.rs:29` embeds the whole `pack/` directory but asserts no registration, so nothing in the repo notices an unregistered file.

WHY THIS IS A DOC DEFECT AND NOT A MECHANISM DEFECT. The set of files this guard can meaningfully guard is exactly the set of DEPLOYED prompts, and the derived set equals that set exactly (verified above: the render emits exactly the seven committed prompts, byte-identical). An unregistered pack file is never rendered and never deployed, so no committed copy exists for it to drift from and there is nothing for a correspondence check to catch. The enumerated `include_str!` form the doc contrasts against would not catch it either, so the contrast the sentence draws is unaffected. Extending the guard to assert that every file under `pack/prompts/` has a manifest row would be a different property (registration completeness, not drift) and would be scope expansion the brief forbids without a human call (`prompt-drift-guard.md:21`; AGENTS.md Principle 8, "No silent scope expansion").

WHAT THE FIX MUST ACHIEVE. `src/agents_md_drift.rs:36-38` must not claim more than the mechanism sentence immediately preceding it delivers. Narrow "a prompt added to the pack is guarded without editing this file" to name the manifest, for example "a prompt added to the pack MANIFEST is guarded without editing this file". One clause; no behaviour change.

## `FN-2`: a raw HTML block is a fourth unprotected construct the precondition does not catch

VERDICT: VALID. Severity `low` (reviewer's rating CONFIRMED, and I decline to escalate; reasoning below). The MECHANISM gap is ACCEPTED AS A RESIDUAL for this step and is explicitly OUT OF SCOPE for the implementer. A doc fix recording the residual IS required in this round.

EVIDENCE REPRODUCED: yes, both halves.

MASKING. I added the reviewer's probe verbatim as a temporary test and ran it (`cargo test --bin agent-scaffold temp_probe_raw_html_block_masking -- --nocapture`):

    precondition rejects multi: false
    precondition rejects single: false
    normalized multi:  "# T\n\n<pre> line one line two </pre>"
    normalized single: "# T\n\n<pre> line one line two </pre>"

Exact reproduction. Every line of the block form is in canonical whitespace form, so `assert_no_unprotected_construct` accepts it; none of its lines is a hard start per `is_hard_start` (`src/agents_md_drift.rs:187-215`), so `normalize_wrapping` joins them; the two forms normalize to the identical string. Probe reverted with the Edit tool.

REACHABILITY. Confirmed independently with prettier 3.6.2 and the repo's own `.prettierrc.json` (`proseWrap: never`), in a scratch directory outside the repo. Both a `<pre>` block and a `<div>` block came back byte-identical, multi-line. So the block form IS a stable fixed point of `nix fmt`, which is exactly what distinguishes this from the two masking avenues the reviewer correctly dismissed (avenue 6, a heading absorbing the following line, which prettier normalises away by inserting a blank line; avenue 7, lazy continuations, which prettier joins). Reachable in principle.

LATENCY. Confirmed: a scan of `.agents/prompts/` and `pack/prompts/` for HTML tags returns nothing once the guidance's own `<task>` / `<step>` / `<role>` placeholder metasyntax is excluded. Nothing is masked today.

THE GAP IS REAL. `assert_no_unprotected_construct` is a PER-LINE predicate. `normalize_wrapping` performs two collapsing operations: an intra-line whitespace collapse and a cross-line JOIN of soft lines onto their logical line. The precondition covers the first and has no check at all on the second. The three constructs the module doc enumerates at `src/agents_md_drift.rs:269-279` are all intra-line, so the enumeration is presented as complete when it is not. That is genuine, and the reviewer identified it correctly.

WHY `low` STANDS, and I do not escalate on severity:

- Reachability today is zero (verified above).
- The masked class is narrow even when reachable. The transform still compares the full ordered non-whitespace token stream, so any actual text change inside an HTML block is still caught. Only the POSITION of a newline inside the block is masked.
- A newline position is only semantically real inside whitespace-significant HTML such as `<pre>`. For `<div>` or `<details>` the join is semantically inert, since HTML collapses those newlines anyway.
- The guarded files are prose role prompts and generated agent guidance. Raw HTML in them is unlikely; `<pre>` in them is very unlikely.
- The worst outcome is a deployed prompt differing from its pack source only in line breaks inside an HTML block. That is close to the least harmful drift this guard could miss.

WHY THE MECHANISM IS NOT FIXED IN THIS STEP. Three independent reasons, the second of which is decisive and is my own finding rather than the reviewer's.

1. THE `F1` PRECEDENT DOES NOT TRANSFER, and I checked its provenance rather than taking the analogy. `F1` was raised in a review round of step 80, `agents-md-drift-guard`, THE STEP THAT BUILT the precondition (`docs/plans/agent-scaffold.ledger.md:363` records that increment converging over four rounds, "r1/r2 new_valid on real false-negatives the adversarial lens caught", and names the precondition fail-safe as part of that step's delivered realisation). Tightening the predicate was inside that step's own deliverable. Step 92 only REUSES the precondition, and its brief states the single trigger for hardening it (`prompt-drift-guard.md:17`): "if a prompt ALREADY contains such a construct, the precondition will trip immediately, and the answer is to harden `normalize_wrapping` (or exempt that construct deliberately), NOT to drop the assertion." That trigger did not fire. The brief anticipated this exact question and answered it: harden when it trips, not speculatively.

2. THE REMEDY THE REVIEWER PROPOSES IS UNSOUND AS STATED, and adopting it would make the artifact worse. The reviewer offers as "the cheap fix": "reject a non-hard-start, non-blank line that directly follows a non-blank line". I implemented that exact predicate as a temporary helper and measured it:

       proposed rejects committed AGENTS.md: false
       proposed rejects committed .agents/AGENTS.reference.md: false
       proposed rejects committed .agents/prompts/orchestrator.md: false
       ... (all 7 prompts: false)
       proposed rejects a reflowed paragraph: true
       current precondition rejects that reflow: false
       proposed rejects the html block: true

   It accepts all nine currently guarded files, so it would not break the tree today. But it REJECTS a soft-wrapped paragraph, which the current precondition accepts and which contract requirement 3 (`prompt-drift-guard.md:11`, "an incidental prettier reflow of a committed copy MUST NOT fail") mandates must pass. My spot-check 3 above is precisely that case: it passes today and would fail under the proposal. The module's own design statement (`src/agents_md_drift.rs:25-30`) says the normalization exists so the guard "keeps passing on incidental reflow if a future pack edit introduces wrapped prose, rather than turning a formatter reflow into a false failure"; the proposed predicate would convert exactly that into a loud failure. So this cannot be handed to the implementer as a one-liner. A correct fix needs design work (candidates: treat raw HTML blocks verbatim the way fences already are, or reject only a line that OPENS a raw HTML block), each with its own trade-offs against reflow tolerance.

3. SCOPE. `normalize_wrapping` and `assert_no_unprotected_construct` are shared machinery that also governs `AGENTS.md` and `.agents/AGENTS.reference.md`. The reviewer says so itself ("this construct is inherited from step 80 and applies to `AGENTS.md` and `.agents/AGENTS.reference.md` as well"). Changing them here would widen a single-file, coverage-extension step into a change to the guard's core transform, against AGENTS.md Principle 4 ("Keep changes small and reviewable") and Principle 8 ("No silent scope expansion").

WHAT THE FIX MUST ACHIEVE (doc only, required this round). The convergence rule permits resolving a valid finding by "consciously accepting its residual risk and recording that". The recording is the required part, and the module doc is the durable place for it (AGENTS.md Principle 9, "Leave durable notes that survive context loss"; the ledger is deleted at task close, the module doc is not). Add the cross-line join case to the UNPROTECTED CONSTRUCTS enumeration at `src/agents_md_drift.rs:269-279` as a fourth entry, stating that: the precondition is a PER-LINE predicate and does not constrain the JOIN; a raw HTML block is the construct that reaches it, because prettier keeps such blocks verbatim so the multi-line form survives `nix fmt`; the masked class is the position of newlines inside such a block, which is only significant for whitespace-significant HTML such as `<pre>`; and no guarded file contains raw HTML today, so this is latent. The three-item restatement the new prompt loop adds at `src/agents_md_drift.rs:410-413` should either point at that enumeration or not read as exhaustive. Comment-only; no behaviour change, no new test.

## `FN-3`: `H4-3`'s recorded description names one of three mechanisms that reach the same residual

VERDICT: VALID BUT ACCEPT RESIDUAL. Severity `low` (reviewer's rating CONFIRMED). This is (a) a more accurate description of an already-accepted residual, NOT something materially wider. NO implementer work; does not block convergence. One orchestrator-owned wording update recommended below.

EVIDENCE REPRODUCED: yes, both mechanisms, and I ran the second one the reviewer described but did not demonstrate.

MECHANISM 1, module-tagging an existing row. Added `module = "checks"` to the `dest = ".agents/prompts/reviewer.md"` row in `pack/pack.toml`:

    cargo test --bin agent-scaffold the_committed_role_prompts_match_a_fresh_render
    test agents_md_drift::tests::the_committed_role_prompts_match_a_fresh_render ... ok

    cargo test
    test manifest::tests::builtin_manifest_lists_the_expected_assets ... FAILED
    test manifest::tests::builtin_checks_module_adds_its_five_assets ... FAILED
    test result: FAILED. 365 passed; 2 failed

The guard passed over six files instead of seven and said nothing. The non-empty assertion at `src/agents_md_drift.rs:401-404` cannot see a set that shrinks from 7 to 6. Reverted.

MECHANISM 2, changing the `dest`. Changed that row's `dest` to `.agents/roles/reviewer.md`:

    cargo test --bin agent-scaffold the_committed_role_prompts_match_a_fresh_render
    test agents_md_drift::tests::the_committed_role_prompts_match_a_fresh_render ... ok

    cargo test
    test manifest::tests::builtin_manifest_lists_the_expected_assets ... FAILED
    test result: FAILED. 366 passed; 1 failed

Reverted. One correction to the reviewer's description, in the direction of slightly worse: a `dest` change out of the prefix leaves BOTH an orphaned unguarded copy at the old path AND a newly deployed file at the new path that is also unguarded, since the new `dest` is outside `PROMPT_DEST_PREFIX`. The reviewer described only the orphan.

WHY THIS IS THE SAME RESIDUAL AND NOT A NEW FINDING. `H4-3` as recorded (`docs/plans/agent-scaffold.ledger.md:353`) is: "the derived-from-manifest guard would silently drop a prompt REMOVED from the pack, where the enumerated form panics; an unstated limitation of a choice judged correct, costing one orphaned prose file." All three mechanisms (remove the row, module-tag it, re-dest it) share the identical CAUSE (the `dest` leaves the module-free render, so the derived filter cannot see it), the identical CONSEQUENCE (a committed copy that is neither compared nor deleted), and the identical COST (one orphaned prose file). The acceptance reasoning that was recorded is therefore unchanged by the extra mechanisms; only the recorded description's breadth is imprecise. My role prompt forbids reopening a settled finding absent new evidence that its VERDICT was wrong, and this is not that: the reviewer explicitly offers it as an accuracy note and does not reopen. So the residual stands as accepted.

FOR THE ORCHESTRATOR (not the implementer). The ledger is the orchestrator's to maintain, and `H4-3`'s carried-forward description at `docs/plans/agent-scaffold.ledger.md:353` is the imprecise text. Recommended wording: "a prompt that LEAVES THE MODULE-FREE RENDER (removed, module-tagged, or re-destined)" rather than "REMOVED from the pack", and record that `manifest::tests::builtin_manifest_lists_the_expected_assets` (`src/manifest.rs:584-621`), an exact-list assertion the guard neither owns nor mentions, is what makes any of these edits a deliberate update rather than an accident. I do NOT require the implementer to add a pointer to that test in the guard's comments; it would be a reasonable nicety if the implementer is already editing comments in that file for `FN-2` and `CT-1`, but it is not a condition of this round.

## `CT-1`: the residual-gap note presents an incomplete list as the complete complement

VERDICT: VALID. Severity `low` (reviewer's rating CONFIRMED). Doc-only fix required. This is the most clear-cut of the four: a factual misstatement in shipped content, with a trivial fix and no counter-argument that survives checking.

EVIDENCE REPRODUCED: yes, every item.

1. The comment is at `src/agents_md_drift.rs:71-77` as quoted.
2. Twelve `[[asset]]` rows have a `dest` under `docs/plans/TEMPLATE` (`pack/pack.toml:38-96`), all `ownership = "working"`, none carrying `render = true`, so all copied verbatim.
3. All twelve are committed: `git ls-files docs/plans/ | grep TEMPLATE` returns them, plus the generated `docs/plans/TEMPLATE.md` view.
4. All are byte-identical to the render. My own 31-file sweep reports 31 SAME and 0 DIFFERS, including all twelve.
5. All are unguarded. `grep -rn "TEMPLATE\." --include=*.rs src/` (excluding testdata and the render fixture) returns only the manifest's expected-dest list at `src/manifest.rs:591-602`, and the complete `include_str!` inventory across `src/` contains no `docs/plans/TEMPLATE.*` embed and no comparison against a render.

THE COUNTER-ARGUMENT DOES NOT SURVIVE. The reviewer disclosed it fully and honestly: the twelve are `ownership = "working"` while the four the comment names are `ownership = "reference"`, so the comment might be read as scoped to tool-owned assets. I checked the reviewer's rebuttal directly and it holds: `AGENTS.md` is itself `ownership = "working"` (`pack/pack.toml:27-31`) and IS guarded, by the very test in the same module. Ownership therefore demonstrably does not define the guarded class in this repo, and the tool-owned reading does not rescue the list. The comment also carries no ownership qualifier; it says "copied assets".

ONE POINT NEITHER REVIEWER QUANTIFIED, WHICH IS WHAT LIFTS THIS ABOVE A NITPICK. Of the 31 emitted files, 9 are guarded (`AGENTS.md`, `.agents/AGENTS.reference.md`, and the 7 prompts) and 22 are not. The comment names 9 of those 22 (`.agents/user-prompts/*` = 6, plus `LEDGER.template.md`, `principles.toml`, `workflow.toml`) and omits 13 (the 12 TEMPLATE assets plus the generated `docs/plans/TEMPLATE.md`). The omitted group is LARGER than the named one. Since the note exists precisely so a human can judge whether widening is worth it (`prompt-drift-guard.md:21`: do not widen "without a human call ... note it in the step's report so the human can decide"), understating the residual by more than half is a decision-quality harm, not a cosmetic one. It stays `low` because no code misbehaves and the fix is a comment, but it is the finding I would least want left unfixed.

WHAT THE FIX MUST ACHIEVE. `src/agents_md_drift.rs:71-77` must stop asserting a completeness it does not have. Either name `docs/plans/TEMPLATE.*` in the parenthetical (preferred, since the note's purpose is to inform a human decision and this is the largest omitted group), or reword so the list is explicitly non-exhaustive. Comment-only; no behaviour change.

## Out of scope, for the orchestrator to route to a human

1. THE BRIEF CARRIES THE SAME OMISSION AS `CT-1`. `docs/plans/agent-scaffold.steps/prompt-drift-guard.md:21` lists the same four asset groups and omits the twelve `docs/plans/TEMPLATE` assets. The implementer reproduced its source faithfully, so this is not an implementer defect. The brief is PLAN content and only a planner may change it, so this is an out-of-scope item for the orchestrator to route to the planner, not a fix for the implementer. It also means the step's report to the human should carry the corrected, complete list of unguarded copied assets (22 of 31 emitted files), since that report is what the human's widening decision rests on.

2. A BACKLOG STEP FOR THE `FN-2` MECHANISM. The cross-line join has no fail-safe, and the obvious cheap predicate regresses contract requirement 3 (measured above). If the human wants this closed rather than carried, it needs a small design pass, not a one-line predicate: the question is how to fail loudly on a raw HTML block without failing on a legitimately reflowed paragraph. Framing for the backlog item: "give `assert_no_unprotected_construct` a cross-line fail-safe that does not regress the guard's reflow tolerance." I recommend deferring rather than doing it now: reachability is zero today, the residual is documented by the `FN-2` fix above, and the fail-safe would fire on the first real occurrence only if the design is right, which is worth taking time over.

3. `FN-1`'s ADJACENT GAP, offered as low value so the human is not burdened. No test asserts that every file under `pack/prompts/` has a `[[asset]]` row, so an unregistered pack file is invisible to the whole suite. This is a registration-completeness property, not a drift property, and its failure mode (a role prompt that silently never ships) is loud at first use. I do NOT recommend building it; it is recorded only so the decision is deliberate.

4. Ledger wording for `H4-3`, per the `FN-3` verdict above. Orchestrator-owned, not a human decision.

## Anything both reviewers missed

- The unsoundness of `FN-2`'s own proposed remedy. The falseneg reviewer offered "reject a non-hard-start, non-blank line that directly follows a non-blank line" as "the cheap fix in the same spirit as the existing precondition" and verified only that no guarded file contains that shape today. It did not test the predicate against contract requirement 3, which it violates. Had this been handed to the implementer as written, the fix round would have regressed a stated acceptance requirement that this very review confirmed as MET.
- The `F1` precedent's provenance. The falseneg reviewer invoked it and handed the escalation question to triage without checking where `F1` was raised. It was raised inside the step that BUILT the precondition (`ledger:363`), which is why hardening was in scope there and is not here.
- `CT-1`'s ratio. The omitted group (13 files) is larger than the group the comment names (9), out of 22 unguarded emitted files. The contract reviewer called TEMPLATE "the largest omitted group" but did not establish that the omission exceeds the disclosure, which is what makes the note actively misleading to a human deciding on widening rather than merely incomplete.
- `FN-3`'s `dest`-change mechanism is marginally worse than described: it orphans the old committed copy AND deploys a new file that is also outside the guarded prefix. Same accepted-residual class, recorded here for accuracy.
- Nothing else surfaced. I attacked the two remaining shapes myself and found no further issue: an EXTRA committed file under `.agents/prompts/` that the render does not emit is invisible to the loop, but that is exactly the `H4-3` orphan already accepted; and the working-tree-versus-index read is disclosed at `src/agents_md_drift.rs:38-42` and matches the pre-existing `include_str!` sides, as the falseneg reviewer's avenue 13 concluded.

## Round outcome

THREE valid findings require an implementer fix: `FN-1`, `FN-2` (doc portion only), and `CT-1`. All three are comment-only edits to `src/agents_md_drift.rs`, none changes behaviour, and none requires a new or changed test.

ACCEPTED RESIDUALS (do not block convergence): the `FN-2` MECHANISM gap (the precondition has no cross-line fail-safe), documented rather than fixed; and `FN-3`, which is `H4-3` restated more accurately and remains accepted on its original reasoning.

This round is NEW VALID FINDINGS, so the consecutive-clean streak is 0. The artifact is classified `risky` and needs 2 consecutive clean rounds, so at least two further rounds are required after the fixes land.

Backstop: NOT triggered. Nothing was dismissed, and nothing is rated high or critical.

Tree state: clean. `git status --short` shows only this untracked findings file; `git diff` is empty; HEAD is `8012e05`. Post-triage re-verification: `cargo test` 379 passed, 0 failed; `cargo clippy --all-targets -- -D warnings` clean.
