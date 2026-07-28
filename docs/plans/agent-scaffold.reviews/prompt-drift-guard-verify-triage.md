# Triage: step 92 `prompt-drift-guard`, authorised-fix verification round

Triager worktree: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage-verify-pdg`, detached at `3e4fb6c`. Read-only with respect to `src/`, the plan, the ledger, and the metrics log; the only file written is this one. Every line number below was re-read at that line in this worktree at `3e4fb6c`.

Findings judged: the 2 raised by the cold-reader lens (`prompt-drift-guard-verify-reviewer-reader.md`). The fix-verification lens (`prompt-drift-guard-verify-reviewer-verification.md`) raised zero, so it has nothing to adjudicate; I spot-checked its two load-bearing mechanical claims against the commit and they hold (`git show 3e4fb6c` is one file, one hunk `@@ -48,9 +48,8 @@`, `2 insertions 3 deletions`, comment lines only).

Environment: `cargo test` passes in full at `3e4fb6c` in this worktree, 367 + 5 + 1 + 3 + 1 + 2 = 379 tests, 0 failed. The known step-93 `checks::tests` worktree-name flake did not fire on my run.

## Summary

| Id | Verdict | Reviewer severity | Final severity | Introduced by `3e4fb6c`? |
| --- | --- | --- | --- | --- |
| RD-V1 | VALID, fix required | medium | **low** (downgraded) | No, predates it (`28f5702`) |
| RD-V2 | VALID BUT ACCEPT RESIDUAL | low | low (confirmed) | No, predates it (`28f5702`) |

Neither finding is at or above the high/critical backstop, so no second-triager re-check is triggered by either verdict.

---

## RD-V1: the COMPLEMENT paragraph's "the Markdown copies" sentence is false of `docs/plans/TEMPLATE.md`

**Verdict: VALID, fix required. Severity: low (downgraded from the reviewer's medium).**

### Evidence reproduced

All three sub-claims reproduce independently, and so does the reviewer's own demonstration.

(a) Precondition. I copied the body of `assert_no_unprotected_construct` verbatim out of `src/agents_md_drift.rs:197-222` into a standalone program in my scratchpad (nothing in the repo was mutated) and ran it over the 17 Markdown files the COMPLEMENT paragraph names. Result: **checked 17, rejected 1**, byte-for-byte the reviewer's message:

```
REJECTED docs/plans/TEMPLATE.md
    docs/plans/TEMPLATE.md line 45 is not in canonical whitespace form. The line is
    "| `example-step` | not started |  |"; its canonical form is "| `example-step` | not started | |".
```

The double space at `docs/plans/TEMPLATE.md:45` is real (`grep -n 'example-step' docs/plans/TEMPLATE.md | cat -A` prints `45:| \`example-step\` | not started |  |$`), and the fence exemption cannot save it: `grep -n '```\|~~~' docs/plans/TEMPLATE.md` returns nothing at all, so the file has no fence and line 45 is inspected.

Not a stale-copy artefact. I ran `cargo run -- scaffold --output-dir <tmp> --write --force --principles default --instrument`; `diff <tmp>/docs/plans/TEMPLATE.md docs/plans/TEMPLATE.md` is empty, and the freshly rendered file is rejected at the same line 45. The property holds of what the scaffold EMITS as well as of what the repo COMMITS, which is exactly the pair the COMPLEMENT paragraph quantifies over (`:57-58`).

(b) Prettier settings. Confirmed at `flake.nix:53`, inside `settings.global.excludes` opened at `:50`, with the reason given at `:47-49`. The exclude list holds exactly three patterns (`src/plan/testdata/render-fixture*`, `docs/plans/agent-scaffold.md`, `docs/plans/TEMPLATE.md`); none of the other 16 Markdown files matches any of them. So `docs/plans/TEMPLATE.md` is the one file in the set that is under NO prettier settings. Minor citation imprecision, not a defect: the reviewer cites `flake.nix:47-53` but its quote block runs to the closing `];` at line 54. The substantive line is 53 and is exact.

(c) Reachability by check 3's filter. `grep -c 'dest = "docs/plans/TEMPLATE.md"' pack/pack.toml` prints `0`, reproduced. Stronger: `grep -rn 'TEMPLATE\.md' src/ --include=*.rs` returns nothing at all, so no code path synthesises that `dest` into an asset either. The file is produced at `src/main.rs:1666-1671`, after the assets land, by stripping `.plan.toml` off an asset `dest` and rendering; the comment at `:1658-1665` says so in terms ("The generated view is NOT a manifest asset"). Check 3 filters `self_scaffold_assets()` (`:426-429`), so no widening of a `dest` prefix can ever reach a file that is not in that set.

### Does the conclusion follow?

Yes, and I tested the one reading that would rescue the sentence.

The paragraph lists four illustrations (`:59-61`): the `.agents/user-prompts/` copies, `.agents/LEDGER.template.md`, the `.toml` copies under `.agents/`, and the `docs/plans/TEMPLATE` family. It then says "Leaving **them** uncovered is a scope call whose cost is uneven", and splits **them** into "the Markdown copies" (cheap) and "the TOML copies" (expensive).

The rescuing reading is that "copies" is a term of art meaning "committed copy of an `[[asset]]` row", which `docs/plans/TEMPLATE.md` is not. I reject it, because it does not survive the split. `docs/plans/TEMPLATE.plan.toml` is not a "`.toml` copy under `.agents/`" either, so under the strict reading the fourth illustration falls into NEITHER half, and a sentence whose entire job is to say the cost of covering "them" is uneven would leave a quarter of its own list unaccounted for. The only coherent reading is Markdown-versus-TOML across all four groups, and under it `docs/plans/TEMPLATE.md` is squarely in "the Markdown copies" and falsifies all three sub-claims. It is also the member of that family that carries the family's bare name, so a cold reader has no signal it is meant to be excluded.

### Severity: downgraded to low

Against `AGENTS.md`'s definition (an absolute rating of impact if left unfixed, four-level scale):

- Nothing about what IS guarded is misstated. The GUARDED SET, the complement rule, R1 and R2 are all untouched by this. There is no coverage hole, and the reviewer agrees.
- The false belief is about the effort of an optional, explicitly-out-of-scope future widening, and it is self-correcting on first contact: an author who tries it finds no asset row at all, immediately.
- The file is guarded elsewhere by `render --check` (`flake.nix:41-49`), which the reviewer correctly notes it is not claiming otherwise.

The reviewer's stated ground for medium, that this is "the block's stated justification for a scope call", overstates the sentence's role. The justification for leaving the complement uncovered is the rule at `:57-59` ("an inventory carries an obligation to stay complete that prose reliably fails"). The cost sentence is a supplementary aside about relative difficulty; deleting it entirely would cost the block no argument. A false aside is low, not medium.

### Why VALID and not accept-as-residual

Accept-as-residual is for an understood structural limitation whose closure costs more than it is worth, which is what R1 and R2 are. This is not that: it is a sentence that is simply false, and the minimal fix is one word at one site. Declining a one-word fix for a verified-false statement inside the block that declares itself the sole coverage statement would be shading toward convergence, which the round brief forbids and which I agree would be wrong here.

### Minimal fix, verified single-site

At `src/agents_md_drift.rs:62`, change `the Markdown copies` to `the Markdown asset copies`. **One word, one line, one site, no reflow, no restructure.**

Single-site: `grep -n 'TEMPLATE' src/agents_md_drift.rs` returns exactly one line (`:61`), and `the Markdown copies` occurs once. Nothing else in the module carries the construct.

It uses no new vocabulary: "asset" is already established in-block at `:45` ("each rendered asset") and `:73` ("deleting an asset row from `pack/pack.toml`").

I measured that it makes the sentence true of everything it then covers. The Markdown asset copies are the 7 at `.agents/user-prompts/*.md` plus `.agents/LEDGER.template.md`, and the 9 Markdown rows under `docs/plans/TEMPLATE.` (`grep -c 'dest = "docs/plans/TEMPLATE\..*\.md"' pack/pack.toml` -> `9`), so 16 files:

- All 16 are accepted by the precondition (the 17-file run above, minus the one rejection).
- None of the 16 matches any prettier exclude, so all are under the same prettier settings.
- All 16 have exactly one `[[asset]]` row each, so a widened `dest` filter over `self_scaffold_assets()` does reach them.

Rejected alternative: deleting `and the docs/plans/TEMPLATE family` from `:61`. Also single-site, but it needs an `and` inserted before `the .toml copies` to stay grammatical, so it is not cheaper, and it discards an accurate illustration of the complement rule.

Do NOT author any explanatory prose about `docs/plans/TEMPLATE.md`, its prettier exclusion, or its generation path. This step's record is that every prose-authoring fix pass manufactured the next round's finding, and the defect needs one word.

### Provenance: predates `3e4fb6c`

`git blame -L 57,68` puts the entire COMPLEMENT paragraph at `28f5702` ("docs: consolidate the drift guard's coverage prose into one COVERAGE block"), two commits before the fix. `git show 3e4fb6c` touches only `:48-53`. **The fix neither introduced nor worsened this. It is a pre-existing defect at a different site, missed by the four prior rounds.**

---

## RD-V2: "Comments past this point cite it and do not restate it" is false of the file's own comments

**Verdict: VALID BUT ACCEPT RESIDUAL. Severity: low (reviewer's rating confirmed).**

### Evidence reproduced

Every citation is exact at the cited lines: `:100` is `//! End of COVERAGE. Comments past this point cite it and do not restate it.`; `:4` and `:38` carry the companion claims; the two comments are at `:376-382` and `:422-425` verbatim as quoted. `grep -n "End of COVERAGE" src/agents_md_drift.rs` -> `100`, so both comments are past it.

The substance holds under both readings the reviewer weighed:

- Broad reading (restate = summarise). `:376-379` restates GUARDED SET items 1 and 2 (`:43-44`) plus the render/normalisation sentence (`:48-50`). It opens by citing COVERAGE and then also restates it, which is exactly what `:100` says does not happen.
- Narrow reading (restate = assert coverage the block does not assert). `:379-381` ("This fails on a real content drift, a hand edit, a dropped slot, or a stale pack source that the per-fragment guards do not cover") is a coverage claim absent from COVERAGE. I checked: that failure-mode enumeration lives at `:9-11`, in the motivation paragraph BEFORE the block, and nothing inside `:34-100` states what a comparison fails on. R1's list at `:72-75` is a different list about set membership. `:422-424` ("Two-way in CONTENT", with its supporting argument) is likewise stated only there.

So `:100` is inaccurate either way, and `:38` ("Write a coverage claim here or not at all") is violated by `:379-381`.

### Why accept-as-residual rather than fix

- No reader is currently misled about coverage. Both comments are TRUE of the code, which the reviewer states and which I confirmed against `:376-412` and `:414-456`. The harm is a latent second-site drift risk, not a present falsehood about the guard.
- **The fix is not single-site in either direction, and every direction trades a real loss.** `grep` puts the monopoly construct at three sites: `:4-5`, `:38`, `:100`. Softening or deleting `:100` alone leaves `:4-5` ("it is the one place in this file that states coverage, and the rest of the file cites it rather than restating it") equally false. Fixing the other side means deleting the offending sentences at `:379-381` and `:422-424`, which are accurate and give a reader arriving at a test function its local orientation; the reviewer explicitly declined to recommend that, and I agree. The reviewer's own suggested fix is "soften line 100", which is authoring prose at a minimum of two sites.
- This step has measured the cost of that trade: three of four fix passes authored prose in this block and each manufactured the next round's finding; the one deletion-only pass manufactured nothing. A multi-site prose pass at low severity has negative expected value here.

This is the same shape as R1 and R2 already carry in the block: a known, bounded limitation of the restructure, worth recording rather than closing. Recording it is not optional; it should be written into the review ledger as an accepted residual so a later author does not rediscover it as a fresh finding, and so that the next person to edit `:379-381` or `:422-424` knows those sites exist.

I want to be explicit that this is not a dismissal. The finding is real and its evidence reproduces completely. I am ruling that its cost to close exceeds its impact, not that it is wrong.

Note that this verdict does not, on its own, let the round score clean, because RD-V1 is valid. So there is no convergence pressure sitting behind this accept-residual.

### Provenance: predates `3e4fb6c`

`git blame` puts `:4`, `:38`, `:100`, `:376-382` and `:422-425` all at `28f5702`. `git show 3e4fb6c` touches only `:48-53`, none of which is cited here. **Pre-existing, not introduced or worsened by the fix.**

---

## Citation hygiene

I checked every `file:line` in the reader lens's file, including those in its "checked and found TRUE" list. All resolve to the claimed text. Two trivial imprecisions, neither affecting a finding: the `flake.nix:47-53` quote block runs one line past its cited range to the closing `];` at `:54`, and the `src/main.rs:1658-1665` quote is truncated at `:1662` (the cited range is correct for the full comment). The settled-item citation `pack/pack.toml:219-223` is exact: `dest = ".agents/prompts/checks-reviewer.md"` at `:221` with `module = "checks"` at `:223`.

## Settled items

I re-opened none of the mechanism's correctness, the deliberate no-explicit-exclusion of `.agents/prompts/checks-reviewer.md`, accepted residual R1, or the upholding at `:312`. Neither finding contradicts R1; RD-V1 concerns the COMPLEMENT paragraph and RD-V2 concerns the block's meta-claim about itself. I found no new evidence that any prior verdict was wrong.

Line length, prose wrapping, and incidental formatter reflow: neither reviewer raised one, and I raise none.

## Recommendation on scoring this round

**My recommendation to the orchestrator, for the orchestrator to take to the human. The scoring decision is not mine to make alone.**

The two questions come apart, and I think they should be reported to the human as two answers, not one:

1. **Did the authorised fix land exactly as prescribed, and re-seed nothing? Yes.** The fix-verification lens established this mechanically and I confirmed its load-bearing claims. `3e4fb6c` is one file, one hunk, comment-only, net minus-18 tokens with a single punctuation change and zero new words, byte-identical from the end of the edited paragraph to EOF. It removed the only site contradicting R1 and left the block internally consistent. On the narrow question this round was convened to answer, the answer is clean.

2. **Should the round be SCORED clean? No, NOT CLEAN**, because RD-V1 is a valid finding on the artifact under review. `AGENTS.md`'s convergence rule scores a round by whether the round produced valid findings on the artifact, not by whether the round's findings were caused by the round's change, so a round carrying one valid finding is `new_valid` with `severities: ["low"]`.

The honest framing for the human is therefore: the escalation's fix succeeded and is safe to keep, and the verification round separately turned up ONE pre-existing low-severity false sentence at an untouched site, with a one-word single-site fix. That is a materially different situation from the four rounds that preceded it, where the loop kept re-seeding its own findings; here the fix caused nothing and the round found a genuine miss the earlier rounds walked past.

If the human's intent in authorising "one fix, one verification round, then merge" was to stop the loop rather than to reach a formally clean round, the cheapest path consistent with that intent is to apply the one-word RD-V1 fix as a second authorised deletion-class edit under the same escalation, record RD-V2 as an accepted residual in the ledger, and merge without another full round. I flag that as an option for the human, not a decision I am making: it is a deliberate departure from running another review round, and it is the human's call, not mine and not the orchestrator's.

What I would advise against is scoring this round clean as written. RD-V1 reproduces completely, its conclusion follows, and the block it sits in is the one place in the file that is supposed to state coverage truthfully.
