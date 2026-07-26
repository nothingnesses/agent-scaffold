# Step 88 review: reviewer-reproducible-evidence rule soundness

Reviewer lens: RULE SOUNDNESS. Does the folded reviewer-reproducible-evidence
rule say the right thing, with no failure mode that suppresses valid findings or
mandates contrived tests? Artifact: commit `5ef42db` (diff `0252d85..5ef42db`),
7 files. Spec: `docs/plans/agent-scaffold.steps/reviewer-reproducible-evidence.md`
and the decided rule (Q-66).

## Verdict

- Critical: zero findings.
- High: zero findings.
- Medium: zero findings.
- Low: one finding (LOW-1), borderline and likely acceptable; see below.

The rule is faithfully and soundly folded. Every soundness probe below came up
clean with the quoted evidence. The one low item is a naming nicety that mirrors
existing project convention, not a suppression or contrived-test risk.

## LOW-1: new text hardcodes "high/critical" for the configurable backstop constant

Severity: low.

Files/lines:
- `pack/AGENTS.md:22` (and identical `AGENTS.md:22`, `.agents/AGENTS.reference.md:22`):
  "that dismissal ground composes with the high/critical backstop (see the
  Convergence rule below) rather than bypassing it."
- `pack/prompts/triager.md:5` (and `.agents/prompts/triager.md:5`): "it composes
  with the high/critical backstop below rather than bypassing it."

Evidence. The backstop severity is a configurable control constant, not fixed at
high. `src/workflow_spec.rs:55` sets the default `backstop_severity: Severity::High`,
and `src/workflow_spec.rs:253-259` is a test where a project sets
`[backstop] severity = "critical"`. The authoritative prose keeps it configurable:
`pack/AGENTS.md:59` says the re-check covers "a dismissed finding at or above the
backstop severity set by the control constants above," and the generated
`{{workflow_control}}` block renders the actual value (`src/workflow_spec.rs:118`,
"whose severity is {severity} or above"). The new step-88 sentences instead write
the literal "high/critical backstop."

Why this is low, not higher, and likely acceptable:
- It is not a soundness defect under the review lens: it neither suppresses a
  valid finding nor mandates a contrived test. It is a precision nit about a
  constant's name.
- It is accurate for the shipped default (`Severity::High`), where "high or above"
  on the four-level scale is exactly {high, critical}.
- It mirrors pervasive pre-existing convention rather than introducing a new
  inconsistency: the unchanged triager sentence immediately after it already says
  "When you dismiss a finding of high or critical severity ... re-checked by a
  second triager" (`pack/prompts/triager.md:5`), and `pack/AGENTS.md:33` and
  `pack/AGENTS.md:47` already say "the high/critical dismissal re-check." Step 88
  follows the house style.
- The new AGENTS.md sentence cross-references the authoritative definition
  ("see the Convergence rule below"), so a reader who reconfigures the constant is
  routed to `pack/AGENTS.md:59` for the precise, configurable threshold.

Possible fix (optional): drop the literal severity from the new sentences, e.g.
"composes with the backstop (see the Convergence rule below) rather than bypassing
it." But because this is a project-wide convention predating this change, changing
only the two new sentences would leave lines 33 and 47 inconsistent; a uniform
rewording is a separate style decision, out of this step's scope. Recommendation:
leave as-is (accept the convention), since it is accurate by default and
cross-referenced.

## Soundness probes that came up clean (grounded, no finding)

Fidelity of the two-tier rule (reviewer + AGENTS bullet). Both carry the
"proportional" qualifier and the "not a contrived test" escape hatch verbatim.
`pack/prompts/reviewer.md:11`: "Make each finding's evidence reproducible and
proportional to its claim ... the reproducible evidence is an exact command
(a grep, a diff, or build or validator output) or a `file:line` citation, not a
contrived test: the evidence scales to the claim, so do not manufacture a test
where a command or a citation already settles the point." `pack/AGENTS.md:21`
reviewer bullet: "never a contrived test where a command or a citation already
settles the point." Both halves match the decided rule; nothing dropped.

Triager half present and correct. `pack/prompts/triager.md:5`: "Reproduce the
evidence a finding carries before you rule on it ... and dismiss any testable claim
whose demonstration does not reproduce." `pack/AGENTS.md:22` triager bullet: "It
reproduces the evidence a finding carries and dismisses any testable claim whose
demonstration does not reproduce." The triager half is stated in both prompt and
guidance, consistent with each other and with the reviewer half (same two tiers,
same labels "behavioural or correctness" / "doc, design, or style").

Suppression risk handled: the design/naming escape hatch is unambiguous.
`pack/prompts/reviewer.md:11` routes "a naming-collision risk" and any "doc,
design, or style claim" to a command or a `file:line` citation, explicitly "not a
contrived test." Every finding type (correctness, design, naming, scope,
plan-completeness) maps to a tier, and tier two always accepts a bare `file:line`
citation, so a valid non-testable finding is never forced to carry a runnable
test. No orphan category is pushed into the runnable tier.

Contrived-test risk handled: the prohibition is binding. "do not manufacture a
test where a command or a citation already settles the point"
(`pack/prompts/reviewer.md:11`) and "never a contrived test where a command or a
citation already settles the point" (`pack/AGENTS.md:21`) are imperative, not
advisory.

Backstop composition is correct, not a bypass. The triager text makes a
non-reproduction dismissal an ORDINARY dismissal: "A dismissal on non-reproduction
is an ordinary dismissal, so it composes with the high/critical backstop below
rather than bypassing it" (`pack/prompts/triager.md:5`). The backstop at
`pack/AGENTS.md:59` fires "before a dismissed finding at or above the backstop
severity ... counts towards a clean round," so an ordinary high/critical
non-reproduction dismissal still passes the second-triager re-check. This is
exactly the mitigation for a triager who fails to reproduce a genuinely valid
demonstration (`pack/AGENTS.md:59`: "A backstop guards the loop against a
stochastic reviewer or triager"). The sidecar's cited anchor is correct: `sed -n
'59p' pack/AGENTS.md` is the backstop paragraph. No contradiction between the
bullet and the Convergence rule.

Cross-file consistency (one rule). `grep -n "reproducible\|does not reproduce\|
contrived test"` across `pack/AGENTS.md`, `pack/prompts/reviewer.md`,
`pack/prompts/triager.md` shows the same rule at appropriate granularity:
reviewer authors proportional reproducible evidence, triager reproduces and
dismisses non-reproducing testable claims, AGENTS states both. AGENTS gives fewer
inline examples than the prompts (role summary vs operational prompt), which the
spec asks for (`reviewer-reproducible-evidence.md:11`, "so the guidance and the
prompts state one rule"). No divergent scope.

Mutation-demonstration correctness. `pack/prompts/reviewer.md:11`: "the strongest
form is a mutation: to prove 'test T does not really cover C', break C and show T
still passes." That is genuinely how you show a test does not exercise code
(pseudo-tested / mutation logic). It correctly localizes to isolation: "Run any
demonstration in your own worktree, which you get like every spawned agent." A
mutation must run in the reviewer's own worktree so it never corrupts the branch
under review; `pack/AGENTS.md:93` confirms reviewers write only in their own
worktree and the orchestrator merges only their findings file, so the mutation is
a throwaway experiment. Faithful to the spec's "run in the reviewer's own isolated
worktree" (`reviewer-reproducible-evidence.md:9`).

Clean-round / zero-findings machinery unaffected. The change adds an evidence
requirement per finding and a new (ordinary) triager dismissal ground; it does not
alter what counts as a finding or a clean round. "Zero findings" is still read
objectively from the committed reviewer files (`pack/AGENTS.md:22`, unchanged
clause), and a non-reproduction dismissal feeds the existing all-dismissed =>
clean logic through the backstop (`pack/AGENTS.md:56,59`). The triager dismisses
only a demonstration that FAILS to reproduce, not a finding that ships without a
demonstration, so the new rule does not mechanically auto-suppress an
un-demonstrated finding at triage.

Deployed copies are a fresh render (no drift). `diff pack/prompts/reviewer.md
.agents/prompts/reviewer.md` and `diff pack/prompts/triager.md
.agents/prompts/triager.md` are both IDENTICAL. The drift guard passes:
`cargo test the_committed_scaffold_matches_a_fresh_render` =>
"test agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render ... ok".
`git show --stat 5ef42db` = 7 files (no `nix fmt` reflow of unrelated files in the
commit), as the task stated.

Not reviewed (per task scope): line length / prose wrapping; the 28 unrelated
`nix fmt` reflows correctly kept out of the commit; the Q-66 decision itself.
