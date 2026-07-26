# Step 89 review: consistency / documentation-currency / scope / house-style

Reviewer lens: documentation currency, pack-to-deployed consistency, scope, house style.
Target: step 89 (`planner-folds-decisions`, Q-67), combined diff `f1abf65..aa771ce`.
Worktree: `.claude/worktrees/step89-review-b` at `aa771ce`.
All evidence below is reproducible from that worktree.

## Verdict

One Low finding (an out-of-scope currency observation). No high, critical, or
medium findings. The drift guard passes, the deployed copies faithfully match the
pack edit, the added prose is byte-identical across all three files, the
"isolation-policy fragment below" cross-reference resolves correctly, the commit
scope is exactly what the step calls for, and no house-style or unicode violation
appears in the added lines or the commit messages.

## Checks performed and results (all pass)

### Drift guard passes (deployed copies faithful)

Command: `cargo test` (via the toolchain prefix). Relevant tests all `ok`:

    test agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render ... ok
    test agents_md_drift::tests::precondition_rejects_non_space_whitespace_and_round_one_cases ... ok
    test agents_md_drift::tests::precondition_exempts_fenced_indented_lines_but_not_bare_ones ... ok
    test agents_md_drift::tests::normalization_tolerates_wrapping_but_not_content_change ... ok
    test tests::isolation_policy_slot_renders_the_generated_fragment ... ok
    test tests::recommendation_rule_slot_renders_the_generated_fragment ... ok
    test tests::workflow_control_slot_renders_the_generated_fragment ... ok

No test failed across the whole run.

### Added text is byte-identical across pack and both deployed copies

Command: grep each added sentence out of all three files
(`pack/AGENTS.md`, `AGENTS.md`, `.agents/AGENTS.reference.md`). All three return
identical strings for each of the three added sentences (human-input-contract,
Socratic, checkpoint). The expected `{{recommendation_rule}}` placeholder in
`pack/AGENTS.md:41` expands to the settled recommendation-rule prose in both
deployed copies (drift test above confirms the expansion is consistent), and the
new sentence sits after the expansion identically in pack and deployed.

### The "isolation-policy fragment below" cross-reference resolves and points BELOW

Command: `grep -n` for the checkpoint text and the fragment in `AGENTS.md`.

- `AGENTS.md:71` (checkpoint) states "the generated isolation-policy fragment
  below lists the orchestrator's closed set of direct-on-main edits".
- `AGENTS.md:91` is the `{{isolation_policy}}` slot rendered; it reads "The only
  edits made directly on main are the orchestrator's own integration-level ones,
  which author no reviewed product content ... : flipping a step's status,
  declaring an increment, recording a round record, and moving the ledger's
  resume anchor."

Line 91 > line 71, so "below" is accurate; the fragment does present a closed
set ("The only edits ... are ...") and it excludes authoring `[[question]]` /
`[[step]]`, so the claim in the checkpoint sentence is grounded, not just
asserted. In `pack/AGENTS.md` the same ordering holds: checkpoint at line 71,
`{{isolation_policy}}` placeholder at line 91.

### One-rule / no-duplication holds

The human-input-contract sentence (`AGENTS.md:41`) is the single authoritative
statement of the planner-routing-of-non-trivial-folds rule; it references the
request-interrupt path ("as on the request-interrupt path above") rather than
restating that path's mechanism. The Socratic sentence
(`AGENTS.md:43`) and the checkpoint sentence (`AGENTS.md:71`) both defer with
"routed as above" / "as above" instead of duplicating the rule. No paragraph
restates a rule already stated elsewhere.

### Scope is exactly the step's scope; no reflows leaked

- `git show --stat aa771ce`: exactly `{.agents/AGENTS.reference.md, AGENTS.md,
  pack/AGENTS.md}`, 9 insertions / 9 deletions (3 changed lines x 3 files).
- `git show --stat c0e880a`: exactly `docs/plans/agent-scaffold.plan.toml`,
  1 insertion / 1 deletion (title-only change).
- No `src/`, prompt, metrics-log, or `{{isolation_policy}}`-region change is in
  either commit; the diff at `pack/AGENTS.md:91` (the `{{isolation_policy}}`
  slot) is untouched. None of the uncommitted `nix fmt` reflows leaked in;
  `git status --porcelain` in this worktree is clean.

### House style clean

- `git diff f1abf65 aa771ce | grep '^+' | grep -cP '[^\x00-\x7F]'` returns `0`
  (no non-ASCII bytes in added lines).
- Added lines contain no em-dash, en-dash, ` -- ` dash-substitute, or
  intra-word `--` (grep for those patterns returns no match).
- Commit messages (`f1abf65..aa771ce`) contain no non-ASCII and no dash
  substitutes, and both use a conventional `docs:` prefix. No characteristic-AI
  filler appears in the added prose.

## Findings

### F1 (Low, out of scope): orchestrator prompt does not yet name planner-routing on the decision-fold path

- File: `pack/prompts/orchestrator.md:33`.
- Evidence: line 33 reads "The ledger is separate from the plan: do not put
  individual findings in the plan's Open Questions section; only durable
  decisions, the ones that change the plan, fold into it." This uses the passive
  "fold into it" and never names who authors the fold. Meanwhile the just-added
  `AGENTS.md:41` now states that a non-trivial decided-decision fold (authoring a
  `[[question]]` or `[[step]]`) "routes it to the planner to author ... rather
  than editing the plan directly." The orchestrator prompt already routes the
  *request-interrupt* path to the planner (`pack/prompts/orchestrator.md:29`,
  "route anything non-trivial to the planner to fold into the plan"), but it has
  no equivalent instruction for the *decision-fold* path that AGENTS.md now
  covers.
- Assessment: this is a gap, not a contradiction. Line 33 is silent on the
  actor, so it does not assert the orchestrator folds decisions directly; it is
  simply less specific than AGENTS.md now is. Grep of the sibling prompts
  (`pack/prompts/{orchestrator,planner,implementer}.md`) for decision-folding
  language finds nothing that contradicts the new AGENTS.md text.
- Scope note: step 89's title was deliberately narrowed in `c0e880a` to
  "... in `pack/AGENTS.md`" only, dropping the broader clause. Bringing the
  orchestrator prompt into line is therefore out of scope for this step. Recorded
  so the orchestrator can decide whether a follow-up step should make
  `pack/prompts/orchestrator.md` name the planner on the decision-fold path (line
  29's request path plus a decision-fold sentence), keeping the prompt current
  with AGENTS.md.
- Fix (for a future step, not this one): add a clause at
  `pack/prompts/orchestrator.md:33` (or near line 29) stating that a non-trivial
  decided-decision fold routes to the planner to author, referencing the
  human-input contract in AGENTS.md rather than restating it.
