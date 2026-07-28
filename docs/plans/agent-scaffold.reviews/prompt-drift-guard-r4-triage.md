# Step 92 `prompt-drift-guard`: work review round 4, triage

Artifact: `src/agents_md_drift.rs` at `90b1527`. Diffs adjudicated: `git diff 0517838..90b1527` (round 3's deletion-only fix pass) and `git diff 149d415..90b1527` (the whole step).
Worktree: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage4-pdg`, detached at `90b1527`. I am independent of both the implementer and the orchestrator, and I edited no source, plan, or pack file as a deliverable. I ran no `nix fmt` and no `just fmt`.

## ROUND OUTCOME, STATED FIRST SO IT CANNOT BE MISREAD

**Round 4 is NEW_VALID.** One finding, `RD4-1`, is VALID at severity `low` and REQUIRES A FIX. It is not an accepted residual. The consecutive-clean streak stays at 0.

I considered "valid but accept residual" directly, because it was the convenient outcome and I was told it was available. I rejected it on the evidence and on this step's own settled precedent, set out under question 3 below. The short version: this step's ledger already contains two triager rulings (`FN-1` in round 1, `A2-1` in round 2) on findings of the identical species, and both were ruled VALID and fix-required rather than accepted as residuals. The only thing that has changed since is the round counter, and that is not a reason.

## Inputs

- `prompt-drift-guard-r4-reviewer-verification.md`: ZERO findings. Under `AGENTS.md:22` a reviewer reporting zero findings gives me nothing to adjudicate, so I raise no verdict against it. I did spot-check its two load-bearing conclusions (the deletion-only proof and the `:312` upholding); both hold, and both are recorded below because they bear on the fix I specify.
- `prompt-drift-guard-r4-reviewer-reader.md`: ONE finding, `RD4-1`, rated `low`.

## `RD4-1`: the GUARDED SET's self-extension sentence states a sufficient condition the code does not honour

VERDICT: **VALID. Severity `low` (the reviewer's rating CONFIRMED). Doc-only fix REQUIRED. Not an accepted residual.**

SITE: `src/agents_md_drift.rs:50-53`.

    Check 3 is a filter over a rendered set, not a directory listing
    and not a hand-written list, which is what makes it self-extending: an `[[asset]]` row
    added to `pack/pack.toml` whose `dest` falls under the prefix is guarded with no edit
    here.

### The evidence reproduced. YES, in full, on a clean tree

1. BASELINE. `cargo test` at `90b1527` with an unmodified tree: 367 + 5 + 1 + 3 + 1 + 2 = 379 passed, 0 failed.
2. THE DEMONSTRATION. I wrote a maximally drifted file (sharing no sentence with `pack/prompts/checks-reviewer.md`) to `.agents/prompts/checks-reviewer.md`, a path that is under `PROMPT_DEST_PREFIX` and whose `[[asset]]` row exists in `pack/pack.toml` with a `dest` under the prefix. Result: `cargo test --bin agent-scaffold` -> 367 passed, 0 failed, run three times. Nothing in `agents_md_drift` fires, and nothing anywhere else in the suite fires either. Removed by path afterwards.
3. THE ROW AND ITS TAG. `pack/pack.toml:219-223` carries `source = "prompts/checks-reviewer.md"`, `dest = ".agents/prompts/checks-reviewer.md"`, `ownership = "reference"`, `module = "checks"`. The `dest` is under the prefix; the row is module-tagged.
4. THE MECHANISM FACT, PROVED WITHOUT A PROBE. `self_scaffold_assets` passes `&[]` for modules (`src/agents_md_drift.rs:140`), so the pinned render emits no module-tagged asset. This is proved by the baseline itself rather than by inspection: `committed_asset` (`:159-167`) PANICS on a missing committed copy, and the repo commits no `.agents/prompts/checks-reviewer.md` (`git ls-files .agents/prompts/` returns seven files, none of them that one), yet check 3 passes. If the pinned render emitted the asset, check 3 would panic. It does not, so it does not emit it.
5. THE SIDE-BY-SIDE THAT MAKES THE OVERSTATEMENT CONCRETE. I inserted `TRIAGE4MUTATION.` into `.agents/prompts/triager.md` (a NON-module-tagged path under the same prefix) with the Edit tool: `the_committed_role_prompts_match_a_fresh_render ... FAILED`, 4 passed 1 failed. Reverted with the Edit tool. So the identical class of edit is caught at a non-module-tagged path under the prefix and is silent at a module-tagged one. The sentence names conditions that do not distinguish the two.
6. REACHABILITY. `.agents/checks.toml` is tracked (`git ls-files .agents/`) and its row is module-tagged (`pack/pack.toml:196-201`, `module = "checks"`), so the repo ALREADY commits a file at a module-gated destination that the pinned render never emits. One `agent-scaffold scaffold --module checks --write --force` run puts `.agents/prompts/checks-reviewer.md` into the tree by the same route, and its `ownership = "reference"` makes it tool-owned rather than create-if-absent, so it lands unconditionally.

### Why VALID

The sentence names two conditions on a row (it is an `[[asset]]` row in `pack/pack.toml`, and its `dest` falls under the prefix) and asserts a conclusion (it is guarded). Those two conditions are not sufficient. The true sufficient condition is that the PINNED render emits the row, which module-tagging defeats.

The defence available to the producer is that the immediately preceding clause, "Check 3 is a filter over a rendered set", supplies the missing condition, and that `R1` at `:74-75` states the fact outright twenty-four lines below. I weighed that and it does not carry, for three reasons.

- THE GLOSS SUBSTITUTES A BROADER ANTECEDENT THAN THE PREMISE IT CLAIMS TO ELABORATE. The premise's subject is a "rendered asset"; the gloss's subject is "an `[[asset]]` row added to `pack/pack.toml`". Those denote different sets. This is not a terse restatement that inherits a qualifier, it is a restatement over a wider domain. The GUARDED SET's numbered item 3 at `:45-46` gets this right ("For each RENDERED asset whose `dest` starts with `PROMPT_DEST_PREFIX`"); the gloss then loses it.
- THE READING BEHAVIOUR THE DEFENCE ASSUMES HAS BEEN FALSIFIED TWICE BY DIRECT MEASUREMENT. Round 3's cold reader recorded `B7` from this text with no module qualifier (`prompt-drift-guard-r3-reviewer-reader.md:18`) and marked it HELD after testing only the non-module-tagged path. Round 4's cold reader recorded `B7` the same way (`prompt-drift-guard-r4-reviewer-reader.md:18`). This is stronger than it first looks: BOTH readers held the module-gating fact elsewhere in their own belief lists in the same sitting (round 3's `B5` carries "non-module-tagged" explicitly, and its `B12` reasons about `checks-reviewer.md` being module-gated; round 4's `B13` records "Its `pack/pack.toml` row IS module-gated"). So neither reader lacked the concept. They had the fact, read this sentence, and still wrote the general rule without it. Dismissing this finding would require me to assert a propagation behaviour that two independent measurements say does not occur.
- `R1` DOES NOT CORRECT THE BELIEF. `R1`'s frame is a committed file that the render does not emit becoming invisible, "reached by ... module-tagging one". That reads as a way to LOSE coverage a row already had. It is fully compatible with the false belief that a newly added module-tagged row is guarded in the first place. So the file does not in fact self-correct on the path a reader takes.

Against all of this, `:38` is the block's own charter: "Write a coverage claim here or not at all." The GUARDED SET is the one place the file designates as authoritative, and `:101` tells the reader everything past it merely cites. A reader is entitled to stop at the GUARDED SET, and a reader who does is misinformed.

### Severity: `low`, CONFIRMED

Judged on the `AGENTS.md:21` scale as the absolute impact if left unfixed, not relative to the round.

For `low`: nothing misbehaves; the mechanism is correct and unmoved; no drift is masked in the tree today (no committed copy of a module-gated prompt exists, verified); the fix is comment-only; and the file elsewhere contains the material a reader could reconstruct the truth from.

I considered `medium` and decline it. The argument for `medium` is that this instance trips NOTHING anywhere in the suite, whereas three of `R1`'s four routes trip `manifest::tests::builtin_manifest_lists_the_expected_assets`, so a maintainer who acts on the wrong belief gets no compensating signal from any direction. That is a real aggravator and it is why I rate this at the top of `low` rather than the bottom. It does not reach `medium`, because the harm remains contingent on a module-gated prompt copy ever being committed here, and nothing is wrong in the tree today.

On the reviewer's stated comparator: `RD-2` is an apt precedent, but it is not the closest one. The closest are `FN-1` (round 1) and `A2-1` (round 2), both of which are the same species (prose asserts coverage over a class the accepted residual excludes), both ruled VALID, both rated `low`, and both fix-required. `A2-1`'s severity reasoning (`prompt-drift-guard-r2-triage.md:121`) is almost word for word the reasoning that applies here. So `low` is right, and it is right on three concordant precedents rather than one.

### The minimal fix, and it IS available by deletion

REQUIRED. At `src/agents_md_drift.rs:50-53`, delete the clause after the colon and terminate the sentence. The result:

    Check 3 is a filter over a rendered set, not a directory listing
    and not a hand-written list, which is what makes it self-extending. Checks 1 and 2 embed
    their committed side with `include_str!` and check 3 cannot, ...

Authored content: ONE character, `:` changed to `.`. Zero new words of prose. This is the same class of edit round 3 executed twice (`:` -> `.` at `:76`, and a `.` added at `:391`), and round 3's pass manufactured no new defect.

NO INFORMATION IS LOST. The surviving clause still states the property and its reason. The immediately following sentence at `:53-56` ("That is what the self-extension costs: less hermetic than a compile-time snapshot ...") presupposes the property rather than the deleted gloss, and reads correctly without it. The operational definition remains at `:45-46`.

PERMITTED ALTERNATIVE, NOT PREFERRED. Insert `non-module-tagged` before "`[[asset]]` row". Two words of new prose. I permit it because it preserves the actionable "with no edit here", and I do not prefer it because every authoring pass in this step has manufactured a defect and every deletion pass has not.

CONSTRAINTS ON THE FIX, so it cannot manufacture a fifth defect:

- ONE SITE ONLY. I checked: `grep -n '\[\[asset\]\]' src/agents_md_drift.rs` returns exactly one hit, line 51. No other comment in the module uses a `pack.toml` row as the antecedent for guardedness. `:45-46`, `:123-126`, `:133-134`, and `:157-158` all correctly key on the RENDER.
- DO NOT author a new explanation of module gating. `R1` at `:74-75` already carries it.
- DO NOT touch `:45-46`, `R1`, or `R2`; do not add an exclusion, a test, a constant, or any mechanism change. Comment-only, and preferably deletion-only.
- DO NOT touch `:302`, `:311-316`, `:344-346`, `:423-426`, or `:255-256`. Each is settled or ruled below.

## Answers to the four questions put to me

### 1. Re-raise of `R1`, or a distinct claim? DISTINCT. Not barred.

I apply the test round 2's triager set and that has never been overturned (`prompt-drift-guard-r2-triage.md:115`): "whether the finding contests the settled verdict or depends on it. ... A finding whose entire content is 'the prose denies an accepted residual' cannot be a re-raise of that residual, because it presupposes it."

`RD4-1` passes that test exactly. `R1`'s settled content is a MECHANISM concession: check 3 is one-way in set membership, accepted, not to be fixed. `RD4-1` asks for no mechanism change, no exclusion, no new test, and no reopening of `R1`; it would be FALSE if `R1` did not hold. Its content is that a positive coverage claim elsewhere in the same block contradicts `R1`. That is a different proposition about a different sentence.

The precedent is not merely analogous, it is this step's own: round 2's `A2-1` was the identical structure (the module header claimed coverage over "every deployed role prompt under `.agents/prompts/`", which `R1`'s module-gating case falsifies) and was ruled VALID and NOT BARRED on this exact test; round 1's `FN-1` was ruled VALID under an explicit heading "WHY THIS IS A DOC DEFECT AND NOT A MECHANISM DEFECT". Round 2's triager wrote the governing sentence: "accepting a mechanism limitation does not license prose that denies the limitation." `RD4-1` is that rule applied at a third site.

One further point that the reviewer did not make and that I established rather than assumed. The sentence is not inherited text and it is not text this step's earlier rounds ever certified. `git log -S "which is what makes it self-extending" -- src/agents_md_drift.rs` returns exactly `0517838`, the round-2 fix pass. And round 2's triage PRESCRIBED the qualifier: its recommended wording was "every asset of the MODULE-FREE self-scaffold render whose `dest` starts with `PROMPT_DEST_PREFIX`" (`prompt-drift-guard-r2-triage.md:176`). The consolidation honoured that in numbered item 3 and then dropped it in the gloss it added alongside. So this is the defect the round-2 fix pass manufactured, found two rounds later, not a relitigation of anything.

On round 3's reader having written "THE GUARDED SET ... [is] accurate as stated" (`prompt-drift-guard-r3-reviewer-reader.md:185`): that is a reviewer's not-raised note, not a triager verdict, so it is not in the settled set at all. And its supporting evidence is four content-direction mutations, none of which touches the module-tagged class. It certified the sentence on evidence that cannot reach the counterexample. Round 4 brings the demonstration round 3 never ran plus a second independent reader measurement, which is new evidence by any reading of `triager.md:7`.

### 2. Severity and the right comparator

`low` CONFIRMED, at the top of the band. Full reasoning above. The reviewer's comparator (`RD-2`) is apt but secondary; `A2-1` and `FN-1` are the closer precedents and they agree. The "trips nothing at all" point the brief raises is real and I have weighed it: it moves the finding to the top of `low`, not into `medium`, because the harm stays contingent and nothing in the tree is wrong today.

### 3. Fix versus residual: FIX. This is the question the round turns on and I answer it directly.

The case for accepting the residual is: the mechanism is sound, nothing is masked today, the truth is in the same file, and every authoring pass in this step has manufactured a defect.

I reject it on four grounds.

- AN ACCEPTED RESIDUAL AND A FALSE CLAIM ARE DIFFERENT OBJECTS. In this file, `R1` and `R2` are accepted MECHANISM GAPS whose descriptions are ACCURATE. That is what makes accepting them coherent: the reader is told the truth and the project consciously declines to close the gap. `RD4-1` would have me knowingly leave a FALSE statement inside the block whose entire charter (`:38`) is "Write a coverage claim here or not at all", in the block that exists because coverage claims kept being false. Accepting a false coverage claim inside the block built to end false coverage claims defeats the block.
- THE STEP'S OWN LEDGER HAS ALREADY DECIDED THIS SPECIES TWICE, BOTH TIMES FOR THE FIX. `FN-1` and `A2-1` were both `low`, both doc-only, both mechanism-sound, and both fix-required. Nothing distinguishes `RD4-1` from them on the merits. What distinguishes it is that it arrived at round 4 instead of round 1, which is not a merit.
- THE CHURN ARGUMENT IS MATERIALLY WEAKER THAN IT WAS. "Every fix pass manufactures a defect" was true of rounds 1 and 2, which AUTHORED prose. Round 3 constrained itself to deletion and manufactured nothing, and round 4's verification reviewer proved that mechanically. I verified the proof myself from the diff: `0517838..90b1527` is four hunks, every changed line a comment line, two clause deletions and two token corrections, no code line touched. The fix I require is a single-site deletion plus one terminator character, in exactly that proven-safe class, at a site I have confirmed is the only one of its kind in the file. The residual risk of the fix is about as low as an edit gets.
- THE COUNTERFACTUAL TEST. If this finding had arrived in round 1, no triager would have accepted it as a residual, and I am confident of that because two of them did not, on the same species. The only new input is the counter. `AGENTS.md:22` says in terms that the orchestrator owns convergence and cost and is therefore biased toward dismissing findings, which is why the triager is separate. Ruling for the residual here would be that bias operating through me.

For completeness on the arithmetic, offered as an observation for the orchestrator and the human and NOT as part of my verdict: with round 4 at `new_valid` the streak is 0, the artifact needs 2 consecutive clean rounds, and the cap is 5. A clean round 5 reaches streak 1, which does not converge, and round 5 also reaches the cap. So convergence within the cap is now arithmetically unreachable and an escalation is certain regardless of round 5's outcome. Whether to spend round 5 before escalating, or to apply the fix and escalate at once with the fix in hand, is the orchestrator's call under the convergence rule and the human-input contract, not mine.

### 4. The reviewer's restraint on `:423-426` past the `:101` marker: CORRECT. Spot-checked.

The text ("Two-way in CONTENT ... One-way in SET MEMBERSHIP, which is residual `R1`") is byte-identical to `0517838`; it does not appear in the `0517838..90b1527` diff. Round 3's reader reached the identical negative result on the identical text (`prompt-drift-guard-r3-reviewer-reader.md:183`, where the same passage is cited as `:418-427`; the one-line shift is the deletion at `:388-391`). Round 4's reader states it has no new evidence against it.

I checked the substance rather than the procedure. The content half is TRUE and I verified one direction myself: the `.agents/prompts/triager.md` hand edit above failed check 3, which is the copy-edited direction, and the pack-edited direction is verified by round 4's `B5` and round 3's mutations. The set-membership half CITES `R1` by name rather than restating it. Nothing there quantifies over the coverage set.

So this is the correct side of the line, and the line is the right one. The distinction from `RD4-1` is precisely the `triager.md:7` test: `RD4-1` carries new evidence that a prior negative result was wrong (a demonstration on a class no prior round tested, plus a second independent reader measurement), and `:423-426` carries none against a statement that is true. Re-raising `:423-426` would be relitigation, and any rewrite of it would be the authoring that produced a defect in each of rounds 1 and 2. Restraint upheld.

## Backstop status

NOT TRIGGERED, and nothing is pending on it. The backstop (`AGENTS.md:59`) covers a DISMISSED finding at `high` or above. I dismissed nothing at any severity: the round's one finding is ruled VALID. No second-triager re-check is required before this round's outcome is recorded.

## Things the reviewers got wrong or missed, recorded so they are not carried forward

- REACHABILITY MIS-TRANSCRIPTION IN `RD4-1`'S EVIDENCE. The finding says "the repo already commits `.agents/checks.toml` and `.agents/hooks/pre-commit` at module-gated destinations". `.agents/hooks/pre-commit` is NOT committed; the directory `.agents/hooks/` does not exist in the tree and `git ls-files .agents/hooks/` is empty. Round 2's triage, which the reviewer cites, said only that `pack/pack.toml:232-236` is a second such ROW, which is correct; the reviewer turned a row into a committed file. The reachability conclusion is unaffected: `.agents/checks.toml` alone establishes it, and I verified that directly. Recorded so the wrong fact is not propagated into a brief.
- "UNLIKE THE `pack.toml` ROUTES IN `R1`, WHICH TRIP `builtin_manifest_lists_the_expected_assets`" IS THREE OF FOUR, NOT ALL. `R1`'s hand-placed-file route trips nothing either, as round 4's own `B8` records. Minor imprecision in the finding's prose; it does not affect the ruling, since the demonstrated silence is what matters.
- THE `checks::tests` FLAKE IS STILL LIVE AND NOW HITS A DIFFERENT PAIR. My first `cargo test` in this worktree failed `checks::tests::a_stdin_reading_check_does_not_hang` and `checks::tests::a_failing_check_reports_and_leaves_no_worktree`; eight subsequent runs, with and without my probe file in place, were green (367/367 and 379 total). Round 2's triage diagnosed this and routed it out of scope (`prompt-drift-guard-r2-triage.md`, "Out of scope" item 1: the runner worktree path is `{temp}/{RUNNER_PREFIX}{pid}-{nanos}` and cargo runs tests as threads of one process). It saw a DIFFERENT pair of tests fail. That the failing pair varies run to run corroborates the diagnosis that any two concurrent `run()` calls can collide, rather than it being specific to two tests. NOT a finding against step 92, whose diff is comment-only in `src/agents_md_drift.rs`, and it is already an active separate step; recorded only so the orchestrator knows a green-suite claim on this repo needs more than one run to be worth anything.
- THE STEP BRIEF'S ROUTED CORRECTION IS STILL OUTSTANDING AND THERE IS ONLY THE ONE. `docs/plans/agent-scaffold.steps/prompt-drift-guard.md:21` still cites `src/manifest.rs` for the `checks-reviewer` gating, the same mis-citation `RD-3` fixed in the comment. Already routed to the planner (`prompt-drift-guard-r3-triage.md:140`) and re-noted by round 4's verification reviewer. I checked whether the brief also carries the unqualified self-extension claim that `RD4-1` is about: `:19` says "That makes adding a prompt to the pack automatically guarded", but `:21` immediately states the `checks-reviewer` exclusion in the same brief, so the brief self-corrects and needs no change beyond the routed mis-citation.
- THE `:312` UPHOLDING IS CORRECT AND I CONFIRMED THE LOAD-BEARING HALF WITHOUT A PROBE. Every member of the enumerated list at `:292-296` is caught by the per-line check, provable by reading `:217-221`: a nested or continuation-indented item and 4-space indented code have leading whitespace, and a two-space run (inside an inline code span or not) is an internal whitespace run, so in each case `line != line.split_whitespace().join(" ")`. The one construct that passes is the raw HTML block, which is exactly what the NEXT sentence at `:313` names as `R2`. Fixing `:312` would have manufactured a defect; leaving it was right, and the implementer should not be sent there.

## Settled items, confirmed untouched

`R1` and `R2` remain ACCEPTED RESIDUALS with no mechanism change, no exclusion, and no new test added (I confirmed the non-comment diff `0517838..90b1527` is empty and `grep -n "checks-reviewer\|exclude\|skip" src/agents_md_drift.rs` finds no exclusion). The `checks-reviewer` implicit exclusion stays SOUND and is not contested. The render-config duplication is not raised. `RD-4` (the thematic-break claim at `:255-256`) stays VALID BUT OUT OF SCOPE and backlogged, and is byte-identical. `R3-CQ-1` stays deferred to step 85. The restored `only when` at `:302` stays a RECORDED EXPECTED EXEMPTION under the amended invariant, not a regression, and my fix instructions exclude it. No finding here concerns line length or formatter reflow.

## Tree state

`git status --porcelain` shows only the three untracked findings files (the two round-4 reviewer files copied in, and this one). `git diff` is empty. HEAD is `90b1527`.

Mutations made and reverted:

1. `.agents/prompts/checks-reviewer.md` created for the `RD4-1` demonstration, then removed by path (it was untracked; removal by path is its revert).
2. `TRIAGE4MUTATION.` inserted into `.agents/prompts/triager.md` with the Edit tool for the side-by-side, then removed with the Edit tool.

I used no `git checkout` and no `git restore`. Final `cargo test` on the reverted tree: 367 + 5 + 1 + 3 + 1 + 2 = 379 passed, 0 failed.
