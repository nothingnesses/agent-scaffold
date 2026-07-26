# Step 88 review round 2 (reviewer-reproducible-evidence, Q-66) - fresh holistic sweep

Reviewer: fresh, independent, adversarial, read-only. Worktree `step88-review-r2`
at `557fa46` (diff `e8f458c..557fa46`, the single commit 557fa46). Artifact: the
reviewer-reproducible-evidence rule folded into `pack/prompts/reviewer.md`,
`pack/prompts/triager.md`, `pack/AGENTS.md`, plus the regenerated deployed copies
`AGENTS.md`, `.agents/AGENTS.reference.md`, `.agents/prompts/reviewer.md`,
`.agents/prompts/triager.md`. This is round 2 of a RISKY increment; round 1 raised
one low (LOW-1, accepted as residual) and the artifact is unchanged since.

## Verdict

ZERO findings. Clean round (no low, no medium, no high, no critical).

Every probe below was checked independently with a re-runnable command or an exact
`file:line` citation and came up clean. LOW-1 is not re-raised: no new evidence its
accept-residual verdict was wrong (its default-accurate, pervasive-convention,
cross-referenced reasoning still holds; `grep -n "high/critical" pack/AGENTS.md`
still shows the two added lines sit among the pre-existing lines 33 and 47).

## Guard / currency results (all green)

- `cargo test`: exit 0, all suites pass. Drift guard specifically:
  `cargo test --bin agent-scaffold agents_md_drift` ->
  `the_committed_scaffold_matches_a_fresh_render ... ok` (4 passed, 0 failed). So
  the deployed copies ARE a fresh render of the edited pack, no drift.
- `cargo clippy --all-targets -- -D warnings`: exit 0, `Finished` with no warnings.
- `validate --source docs/plans/agent-scaffold.plan.toml --workflow`: exit 0,
  "workflow invariants hold" (206 records valid; 89 steps, 67 questions valid).
- `render docs/plans/agent-scaffold.plan.toml --check --strict`: exit 0,
  "up to date".
- Pack vs deployed same added text:
  `git diff e8f458c 557fa46 -- pack/prompts/reviewer.md .agents/prompts/reviewer.md`
  shows the identical `+` line (index `accdde3..9acf7a8` on both paths);
  `git diff ... pack/prompts/triager.md .agents/prompts/triager.md` shows the
  identical `+` line (index `bee731d..4950d11` on both). Live
  `diff pack/prompts/reviewer.md .agents/prompts/reviewer.md` and the triager pair
  are both byte-identical (empty diff).

## Probes that came up clean (grounded)

### 1. Fidelity to the decided rule (Q-66) - PASS

Spec `docs/plans/agent-scaffold.steps/reviewer-reproducible-evidence.md:5` states:
behavioural/correctness claim -> runnable demonstration, strongest form a mutation;
doc/design/style claim -> exact command or `file:line`, not a contrived test;
proportional evidence; triager reproduces and dismisses any testable claim whose
demonstration does not reproduce; a high/critical non-reproduction dismissal still
passes the second-triager backstop. Every element is present in the added text:

- `pack/prompts/reviewer.md:11`: "For a behavioural or correctness claim ... provide
  a runnable demonstration a second party can re-run; the strongest form is a
  mutation: to prove 'test T does not really cover C', break C and show T still
  passes. For a doc, design, or style claim ... an exact command ... or a `file:line`
  citation, not a contrived test ... do not manufacture a test where a command or a
  citation already settles the point."
- `pack/prompts/triager.md:5`: "Reproduce the evidence a finding carries before you
  rule on it ... and dismiss any testable claim whose demonstration does not
  reproduce. A dismissal on non-reproduction is an ordinary dismissal, so it
  composes with the high/critical backstop below rather than bypassing it."

No element of the decided rule is dropped, softened, or added to.

### 2. No reading suppresses a valid non-testable finding - PASS

The reviewer rule gives every finding type a tier, and tier two accepts a bare
`file:line` citation, the minimum a reviewer already supplies
(`pack/prompts/reviewer.md:9`, unchanged, already requires "cite the file and
line"). A completeness/design/naming claim ("this design is over-built", "the plan
misses a step") is a doc/design claim -> `file:line`, never forced into a runnable
test. There is no orphan finding category pushed into the runnable tier, so no
valid non-testable finding is suppressed. The "not a contrived test" clause is an
imperative ("do not manufacture a test where a command or a citation already
settles the point"), not advisory, so no reading mandates a contrived test.

### 3. Backstop composition is a compose, not a bypass - PASS

`pack/prompts/triager.md:5` makes a non-reproduction dismissal an ORDINARY
dismissal ("A dismissal on non-reproduction is an ordinary dismissal, so it
composes with the high/critical backstop below rather than bypassing it"). The
backstop at `pack/AGENTS.md:59` fires "before a dismissed finding at or above the
backstop severity set by the control constants above ... counts towards a clean
round", so a high/critical non-reproduction dismissal still gets the second,
independent triager. The one new-looking hole this change could open (a triager
silently voiding a genuine high/critical demonstration by claiming it did not
reproduce) is exactly the hole the compose clause closes. The forward references
resolve: `grep -n -i "convergence\|backstop" pack/AGENTS.md` puts the Convergence
header at line 49 and the backstop paragraph at line 59, both below the triager
bullet at line 22; in `pack/prompts/triager.md` the "backstop below" points to the
later clause in the same line-5 paragraph ("When you dismiss a finding of high or
critical severity ... re-checked by a second triager").

New angle checked (medium/low tail): a non-reproduction dismissal of a
medium or low finding has no backstop, but that is the pre-existing accepted design
(`pack/AGENTS.md:59`, the backstop "guards the dangerous tail ... without doubling
the cost of ordinary triage"; a triager could already dismiss a medium as invalid
with no re-check). The change does not lower the backstop threshold nor add a new
unguarded tail; it adds one specific ground that rides the existing structure. Not
a regression.

### 4. One-rule consistency across the seven files - PASS

`grep -cn "reproducible\|does not reproduce\|contrived test\|composes with the
high/critical backstop"` returns 2 for each of `pack/AGENTS.md`, `AGENTS.md`,
`.agents/AGENTS.reference.md` (reviewer bullet + triager bullet) and 1 for each of
the four prompt files (single-paragraph line). The three AGENTS variants carry the
same reviewer-bullet and triager-bullet text; each pack prompt equals its `.agents`
copy byte-for-byte (section 0 above). The AGENTS bullet is a faithful compression
of the prompt: `pack/AGENTS.md:21` "a runnable demonstration (the strongest form a
mutation that breaks the code and shows the test still passes) ... never a contrived
test where a command or a citation already settles the point" vs the fuller
`reviewer.md:11`. Same claim, no contradiction, no divergent scope (AGENTS is the
role summary; the prompt is operational, exactly what the spec asks at
`reviewer-reproducible-evidence.md:11`, "so the guidance and the prompts state one
rule").

### 5. Second-order interactions - PASS

- Zero-findings / clean-round machinery: unaffected. The triager dismisses a
  demonstration that FAILS to reproduce, not a finding shipped without one; "Zero
  findings" is still read objectively from the committed reviewer files
  (`pack/AGENTS.md:22`, unchanged clause). A finding lacking required evidence is
  judged weak under the pre-existing "judge each finding on its evidence"
  (`triager.md:5`), not auto-suppressed by the new clause.
- Acceptance phase and review-entry mode: both reuse the same `reviewer.md` and
  `triager.md`, so their reviewers now ground findings too, which is coherent, the
  reviewer prompt already anticipates a no-plan run ("when a review run has no plan,
  the criteria you were given", `reviewer.md:5`), and the acceptance doc-currency
  check (`reviewer.md:17`) yields doc claims that carry a `file:line`, fitting tier
  two. The acceptance/review "high/critical dismissal re-check" (`pack/AGENTS.md:33`,
  `:47`) is the same backstop the new compose clause rides, so it holds in those
  passes.
- Explorer / design-notes flow: explorers author advisory notes, not review
  findings (`pack/AGENTS.md:65`), so the reviewer/triager evidence rule does not
  bind exploration authoring; when an exploration is given its optional light review
  pass, findings about it are doc/design claims carrying a `file:line`, consistent
  with the rule. No conflict.
- Worktree isolation: `reviewer.md:11` "Run any demonstration in your own worktree,
  which you get like every spawned agent" is consistent with `pack/AGENTS.md:89,93`
  (every spawned agent, reviewers included, isolates and writes only its findings
  file), so a mutation demonstration is a throwaway that never corrupts the branch
  under review.

### 6. Scope and house style - PASS

`git show --stat --format="" 557fa46` = exactly the 7 owned files, 12 insertions /
8 deletions; `... | grep -E "src/|\.plan\.toml|docs/metrics/|docs/plans/agent-
scaffold\.md"` returns nothing (no code, plan, or metrics touched). The added text
sits in the placeholder-free "Roles and their prompts" list, not inside any
`{{...}}` render token. ASCII only:
`git diff e8f458c 557fa46 | grep '^+' | grep -P '[^\x00-\x7F]'` and the em/en-dash /
double-hyphen scan both return no matches (exit 1). British "behavioural" matches
the surrounding house spelling (`reviewer.md:7`, unchanged, "Check behaviour").

## Not reviewed (per task scope)

Line length / prose wrapping; incidental formatter reflow; the Q-66 decision
itself; LOW-1 re-litigation (no new evidence).
