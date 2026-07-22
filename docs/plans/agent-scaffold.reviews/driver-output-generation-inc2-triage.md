# Triage: driver-output-generation inc2 (next.rs consumer, RISKY)

Triager: independent of both reviewers, implementer, and orchestrator. READ-ONLY.
Diff: main (17dce12) .. impl/dog-inc2 HEAD (36ed42a). Worktree /home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/dog-inc2.
Findings adjudicated: D2-1 (opus, low informational), D2s-1 (sonnet, low).

## Round result: NEW_VALID

D2s-1 is actionable this round (a trivial two-hunk revert). D2-1 is a valid non-actionable observation. Because D2s-1 requires a fix, the round is NOT clean.

## D2-1 (opus) -- no full golden for a writer-state instruction block

Verdict: VALID-NON-ACTIONABLE
Actionable this round? No

Reasoning:
- The observation is accurate. The golden fixtures (`GOLDEN_TEXT`/`GOLDEN_JSON`, src/next.rs:1632-1633, 1668-1669) pin an AwaitingReviewers (reviewer) state. No golden pins the assembled writer-state block `"Writer isolation (resolved tier: X). <fragment>"` as one byte-exact unit.
- The writer-state output is not un-pinned; it is a concatenation of pieces each independently guarded: the fragment is matched against the WHOLE shared const (`a_writer_state_emits_the_isolation_fragment_for_any_tier`), the tier echo is checked (`a_writer_state_echoes_the_resolved_tier_in_the_isolation_reminder`, src/next.rs:1449), and the unknown-tier resolve note is checked (`an_unknown_tier_adds_the_resolve_note_at_a_writer_state`). The only thing a full golden would add is pinning the exact lead-plus-fragment concatenation and the reminder ordering.
- Match ceremony to stakes: the residual regression a writer-state golden would catch (a reordering or a lead/fragment join defect that the contains-checks pass through) is narrow, and the fragment itself already has a byte-guard against the AGENTS.md slot (src/isolation_policy.rs:66) plus the const-verbatim match. Adding a second full golden now is optional polish, not closing a correctness gap. Reviewer 1 explicitly ruled "no action required." Accept as a valid observation with no code change.

Smallest fix if actionable: n/a.

## D2s-1 (sonnet) -- whitespace-only rustfmt reflows in run_resume committed out of scope

Verdict: VALID
Actionable this round? Yes

Reasoning:
- Confirmed by `git diff main..HEAD -- src/main.rs`: two hunks in `run_resume` collapse a two-line `let ledger_path =` binding (src/main.rs:1093-1094 on main) and a four-line `println!` arm (src/main.rs:1102-1105 on main) into single lines. The intended change in this increment is the principle-threading in `run_next` (the `(steps, principles, source)` tuple, `source_plan.principles.clone()`, the `Vec::new()` degrade paths, and `principles: &principles` into NextInputs, src/main.rs:1116-1172). The two `run_resume` hunks carry zero behaviour: pure whitespace.
- These are NOT in stash@{0} (which captures prettier-on-docs/ledger and rustfmt range-spacing on source.rs); they are committed in 36ed42a. Confirmed against the stash description and the committed diff.
- Verified the reflows are a fmt "fix" of PRE-EXISTING drift, not new formatting the increment must own: `git show main:src/main.rs` shows both `run_resume` regions already multi-line on main. The config `use_small_heuristics = "Max"` (rustfmt.toml) collapses both to single lines, so the increment's nix fmt / scaffold-self pass reformatted lines the edit never touched. Reverting the two hunks restores main's exact `run_resume` form; it introduces NO new drift (main already ships that same form), and it makes the committed src/main.rs diff intent-only (principle-threading in run_next only).
- Precedent: this project consistently EXCLUDES incidental fmt reflow from increments (38 stash entries doing exactly this, including stash@{25}/{26} which reverted incidental rustfmt reflow/import-reordering triggered by formatting src/main.rs in another increment). The prior D1s-2 ruling this session accepted regenerable reflows precisely because they were stashed/reverted OUT of the commit; the distinguishing fact here is that these are COMMITTED. Consistent application of the same discipline requires excluding them.
- The counter-argument (in-scope file, take the formatter's whole-file output, reverting is churn) is outweighed: the project's demonstrated discipline extends to reverting incidental reflow even when triggered by an in-scope edit (stash@{25}), and reverting here restores parity with main rather than fighting the formatter. The fix is one shot and trivial.

Smallest fix if actionable: revert only the two whitespace hunks in `run_resume`, restoring main's form. Leave all `run_next` principle-threading untouched.

Hunk 1 (src/main.rs, run_resume): change
	`	let ledger_path = args.ledger_fragment.clone().unwrap_or_else(|| default_ledger_path(&task));`
back to
	`	let ledger_path =`
	`		args.ledger_fragment.clone().unwrap_or_else(|| default_ledger_path(&task));`

Hunk 2 (src/main.rs, run_resume match arm): change
	`		None => println!("{}: no `+"`"+`## RESUME STATE`+"`"+` block found", ledger_path.display()),`
back to
	`		None => println!(`
	`			"{}: no `+"`"+`## RESUME STATE`+"`"+` block found",`
	`			ledger_path.display()`
	`		),`

Note on fmt: after the revert, src/main.rs carries the same pre-existing fmt drift it has on main; do NOT re-run nix fmt / scaffold-self on the whole file to "fix" it, as that would re-collapse these lines and re-introduce the out-of-scope hunks. This matches how the project carries the other stashed pre-existing drift.
