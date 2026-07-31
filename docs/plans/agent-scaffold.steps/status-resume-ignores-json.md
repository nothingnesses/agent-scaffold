### `status-resume-ignores-json`: `status --resume --json` silently ignores `--json` and prints human text

A small consistency defect on the CLI surface, found while specifying the JSON reasons for `workflow-enforcement-tier` (`Q-55-jsonreason`) and deliberately not folded into that step. `run_status` dispatches to the resume slice before any serialisation happens, so `--json` passed alongside `--resume` has no effect at all: no JSON, no warning, no non-zero exit.

Scheduled as backlog behind the release gates by human decision (2026-07-31), against the standing backlog, with no `[[question]]` registered and no decision receipt owed, on the same precedent as `test-tmpdir-repo-assumption` (order 95). It blocks nothing and nothing blocks it; it is deliberately NOT a dependency of `rename-to-agent-flow`, of `workflow-enforcement-tier`, or of `test-tmpdir-repo-assumption`.

## The mechanism

`run_status` (`src/main.rs:1062-1069`) opens with the resume branch and returns from it:

```rust
fn run_status(args: StatusArgs) -> io::Result<()> {
	// The thin `status --resume` slice: print the ledger's `## RESUME STATE` block
	// verbatim (reusing the same extractor `next` uses) instead of the state projection.
	// A missing ledger or absent section is a note and exit 0, not a failure (`status` is
	// best-effort).
	if args.resume {
		return run_resume(&args);
	}
```

Every serialisation path in `status` is BELOW that return: the projection is assembled afterwards and `serde_json::to_string_pretty` is reached at `src/main.rs:1104`. `run_resume` (`src/main.rs:1152-1165`) only ever `println!`s. So `args.json` is read nowhere on the resume path, and clap accepts the pair without complaint.

## The observed behaviour, run rather than reasoned

Four cases, all against a build of this worktree, with a two-line probe ledger outside the repository.

A, the resume block present. `--json` has no effect; the raw Markdown block is printed:

```
$ agent-scaffold status --source docs/plans/agent-scaffold.plan.toml --resume --json --ledger-fragment <probe>.ledger.md
## RESUME STATE

probe ledger body.
exit: 0
```

B, the ledger absent:

```
no ledger at <path>; nothing to resume
exit: 0
```

C, a ledger with no `## RESUME STATE` section:

```
<path>: no `## RESUME STATE` block found
exit: 0
```

D, THE CONTROL, which is what makes this a dispatch problem rather than a broken flag. Without `--resume`, the same `--json` works exactly as documented:

```
$ agent-scaffold status --source docs/plans/agent-scaffold.plan.toml --json
{
  "plan": {
    "steps": [
...
```

So the flag is fine and is simply never reached. In all three resume cases the output is not JSON and the exit code is 0.

## Severity, stated honestly so the queue is not distorted

THIS IS A FALSE SILENCE, NOT A FALSE ASSERTION, and it is milder than anything `workflow-enforcement-tier` fixes. It must not borrow that step's urgency just because it was found next to it.

The caller asked for JSON and got output that is visibly not JSON, so any consumer that parses it fails on the first character, immediately and locally. Nothing wrong is propagated: there is no wrong value, no fabricated instruction, and no green that should have been red. Compare the defects that step exists to fix, where the tool emits a CONFIDENT WRONG ANSWER that a human or an agent then acts on. Here the tool emits a right answer in the wrong format, and the format mismatch is self-announcing.

What the defect actually costs is DIAGNOSIS, not correctness. The failure surfaces to the user as "the tool produced garbage" rather than as "you passed a flag that does nothing here", so the user debugs their parser before they suspect their command line. That is a real cost and a small one.

## The in-repo precedent, which makes this an inconsistency rather than a gap

This project has already ruled on silently-ignored flags, in this file, with the reasoning written down. `StatusArgs` itself carries one, at `src/main.rs:464-466`, on the sibling flag of the very branch in question:

```
/// ... Requires --resume (the flag is meaningless without it, and would otherwise be silently ignored on an exit-0 run).
#[arg(long, requires = "resume")]
ledger_fragment: Option<PathBuf>,
```

"the flag is meaningless without it, and would otherwise be silently ignored on an exit-0 run" is exactly the condition `--json` is in under `--resume`, on the same struct, one field away. The same sentence appears again on `--workflow-spec` (`src/main.rs:441-443`, "the flag is meaningless without it, and would otherwise leave a malformed spec unparsed and exit 0"), and a third, shorter form on `render --strict` (`src/main.rs:524-526`, "Meaningless without --check").

There is also a precedent for the RELATION this case needs, which is not the same relation as those three. `audit` resolves a clash between two mutually exclusive OUTPUT MODES at `src/main.rs:556-558`:

```
/// Override the report output path. ... Conflicts with --json, which writes no file.
#[arg(long, conflicts_with = "json")]
out: Option<PathBuf>,
```

So a fix here FOLLOWS AN ESTABLISHED CONVENTION rather than inventing one, and that is the proposition to state in the commit. Five constraint attributes already exist in `src/main.rs` (`:396`, `:442`, `:465`, `:525`, `:557`), so the mechanism is routine in this codebase.

## The fix fork, not pre-decided

- (A) REJECT THE COMBINATION WITH A CLAP CONSTRAINT. `--resume` and `--json` are two mutually exclusive OUTPUT MODES, so the relation is `conflicts_with`, matching the `audit --out` precedent, NOT `requires`, which is the relation the other three precedents use. Getting that backwards matters: a `requires` in either direction would break `status --json` on its own, which is a working, documented surface with a README example. Cost is one attribute, one help-text sentence on each flag, and a test per direction. Promises nothing new, closes the defect exactly as stated, and is a one-attribute revert if the project later wants (B).
- (B) GIVE `--resume` A REAL JSON SURFACE. Serialise the resume block and its absence reason instead of rejecting the pair. Materially larger, and it answers a DIFFERENT question, namely whether `status --resume` should have a machine surface at all, which nobody has asked for. IT MUST NOT BE BUILT BEFORE `workflow-enforcement-tier` INC2 LANDS. That increment introduces `resume_state_absent_reason` on `NextProjection` with the closed vocabulary `ledger-absent` / `no-resume-section` / `ledger-not-this-project`, and those are the same three causes a resume JSON surface would have to report. Building (B) first would mint a second vocabulary for one set of causes, which is the "an under-specified vocabulary reproduces the defect in a new place" hazard in a new place, and it would then have to be reconciled. If (B) is ever chosen, it REUSES that enum.

RECOMMENDATION: (A). It closes the defect at the size of the defect (Minimal by default), it follows two in-file conventions rather than inventing one, and it forecloses nothing, since (A) and (B) are mutually exclusive in the long run and (A) is trivially revertible if appetite for (B) appears. (B) is a capability question wearing a defect's clothes.

NOTE ON WHY THE (B) ORDERING IS NOT ENCODED IN `blocked_by`. It is conditional on a fork nobody has chosen, and under the recommended (A) this step is not blocked by anything. Encoding a dependency for a branch that may never be taken would make the step falsely blocked and would drag it into `workflow-enforcement-tier`'s wake, which is the opposite of what scheduling it as independent backlog was for. The constraint is stated here in prose instead; an empty `blocked_by` is deliberate, not an oversight.

## Risk classification

`status-resume-ignores-json-inc1` is `low_risk` (one clean review round), AS SCOPED TO FORK (A).

The change is one clap attribute plus help text and tests. It touches no logic, changes no output for any invocation that is not currently meaningless, ships nothing to a scaffolded project, and reverts in one line. It DOES turn a currently-accepted invocation into a usage error (exit 2), which is a CLI contract change, and this plan classifies `workflow-enforcement-tier-inc3` as `risky` partly on that basis; the two are not comparable. Inc3 flips a currently-PASSING gate to failing for an entire population, on an invocation people run in CI, and its boundary (which cases convert to problems and which stay skips) is easy to get subtly wrong. Here the rejected combination has never done what its flags advertise, so nothing can be depending on the behaviour being removed; the break is loud, immediate, names both flags, and is fixed by deleting one of them.

The one real way to get this wrong is naming the WRONG RELATION or putting the attribute on the wrong field, which would break `status --json` standing alone, a documented surface with a README example. That is a single-attribute mistake with a single-test detection, which is what one clean round with a mandated control case is for; a second round would re-read one attribute.

IF FORK (B) IS CHOSEN INSTEAD, THIS CLASSIFICATION DOES NOT HOLD and the step must be RE-CLASSIFIED BEFORE IT IS BUILT rather than waived afterwards. (B) adds a serialised contract, must reuse another step's vocabulary, and would carry the same documented-contract obligations `Q-55-jsonreason` carries (doc comments, `README.md`, `CHANGELOG.md`).

## Acceptance check

1. `cargo test` and `cargo clippy --all-targets -- -D warnings` clean. Point `TMPDIR` outside any git repository first, per `test-tmpdir-repo-assumption` (order 95).
2. THE DEFECT IS CLOSED: `agent-scaffold status --source <plan> --resume --json` exits NON-ZERO with a usage error naming both flags, rather than exit 0 with human text. Red against the pre-fix build, where it exits 0 printing the `## RESUME STATE` block.
3. THE CONTROL, which is the mistake this step can actually ship and must be pinned in BOTH directions: `status --source <plan> --json` alone still emits the projection as JSON and exits 0, and `status --source <plan> --resume` alone still prints the block and exits 0. A fix that spells the relation backwards passes check 2 and fails one of these.
4. The two remaining resume paths are unchanged when `--json` is absent: a missing ledger prints `no ledger at <path>; nothing to resume` at exit 0, and a ledger with no section prints `<path>: no ``## RESUME STATE`` block found` at exit 0.
5. `--ledger-fragment` still requires `--resume` and is unaffected, so the new constraint did not disturb the existing one on the same struct.
6. `--help` for `status` reads correctly with the new constraint, and the help text on each flag states the exclusion in the style the five existing constraint attributes use.

## Scope

- It does not give `status --resume` a JSON surface. That is fork (B), it is not recommended, and it must not be built before `workflow-enforcement-tier` inc2 in any case.
- It does not audit the other subcommands for silently-ignored flag combinations. Five constraint attributes already exist and the three cases they cover are the ones previously found; a sweep for further cases is a different piece of work and is not smuggled in here (Minimal by default). If the implementer notices one while working, RAISE it rather than folding it in.
- It does not change `run_resume`'s output, its exit code, or its best-effort contract.
