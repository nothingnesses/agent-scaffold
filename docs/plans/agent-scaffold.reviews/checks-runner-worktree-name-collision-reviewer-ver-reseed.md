# RE-SEED review of commit 596548b (test(checks): pin claim_dir's own error arm and delete two false claim-outcome sentences)

Step: `checks-runner-worktree-name-collision`. Lens: does this escalation-authorised
fix re-seed the next round. Artifact: `596548b`, diff against `bc9908e`, 19
insertions / 9 deletions in `src/checks.rs`, claimed zero production line changes.

Baseline confirmed before any mutation:

- `cargo test`: 386 passed, 0 failed, across all seven binaries (373 + 5 + 1 + 1 +
  3 + 1 + 2 = 386). Matches the number given in the assignment.
- `cargo test --bin agent-scaffold`: 373 passed, 0 failed (this is the pure-binary
  crate's real unit-test count; `cargo test --lib` would silently run nothing).
- `cargo clippy --all-targets` (the project's `just clippy` recipe): silent, no
  warnings, `Finished` with only `Checking` lines.
- `#[test]` count identical at `bc9908e` and `596548b` (`33` in both, via `git show
  <rev>:src/checks.rs | grep -c '#\[test\]'`): confirms the commit message's claim
  that the third outcome was folded into the existing
  `a_directory_claim_is_exclusive` rather than adding a new test function.

## Mutation table

| # | Mutation | Location | Result | Killing test / evidence |
|---|----------|----------|--------|--------------------------|
| 1 | `Err(error) => Err(error)` -> `Err(_) => Ok(false)` | `src/checks.rs:452`, `claim_dir`'s third match arm | RED | `checks::tests::a_directory_claim_is_exclusive`. Full bin: `test result: FAILED. 372 passed; 1 failed`. Isolated: panic `a claim that cannot be made is an error, not a verdict: false` |
| 2 | `Err(error) => Err(error)` -> `Err(_) => Ok(true)` | `src/checks.rs:452`, same arm | RED | Same test. Full bin: `test result: FAILED. 372 passed; 1 failed`. Isolated: panic `a claim that cannot be made is an error, not a verdict: true` |
| 3 | Instrumentation only (`eprintln!("REVIEWER-PROBE kind={:?} msg={error}", error.kind());` inserted before the new `assert_ne!`, no logic change) | `src/checks.rs`, inside the new test code | N/A (probe, not a mutation test) | Confirms the real, uninjected error the fixture produces on this platform: `REVIEWER-PROBE kind=NotADirectory msg=Not a directory (os error 20)`. Test still passed (1 passed, 0 failed). This is the empirical basis for finding R1 below being a non-finding: the `assert_ne!(..., io::ErrorKind::AlreadyExists, ...)` claim is true on this system, not merely assumed. |

All three mutations were reverted individually before the next was applied. Full
revert verification is in the Hygiene section.

## Findings

None survived verification. See NON-FINDINGS below for candidates that were hunted
and ruled out with evidence.

## Attacks that FAILED (evidence the fix holds, not absence of effort)

- **Both directions of the error-arm fold, mutation-tested.** `Err(_) => Ok(false)`
  and `Err(_) => Ok(true)` both go RED, both killed by exactly the new assertion in
  `a_directory_claim_is_exclusive`, both leaving every other test green (372/1 in
  each direction, never a second failure). This directly verifies the two
  behaviours the task asked me to confirm are covered: a permissions-class fault
  folding into false-not-collision, and it folding into true-claim-not-established.
- **Searched for a surviving third instance of "every real claim ... wins".**
  `grep -n "every real claim" -i src/checks.rs` and a repo-wide `grep -rn "every
  real claim\|WINS\b" -i --include="*.rs" .` (excluding `target`) found the phrase
  nowhere in `.rs` files. The only remaining "wins"-adjacent hits in `.rs` files are
  unrelated: `audit.rs:1152` (FFI-marker precedence), `manifest.rs:780` and
  `metrics.rs:1316` (test function names about override/last-one-wins config
  resolution), `pack.rs:318`, `main.rs:1174`, `workflow.rs:270,303,314`
  (last-one-wins baseline resolution), and `checks.rs:1754`
  (`a_claim_that_never_wins_fails_at_the_attempt_bound`, a pre-existing, unrelated
  test name). None is the deleted sentence or a paraphrase of it. Historical
  mentions of the deleted sentence exist only in
  `docs/plans/agent-scaffold.reviews/*.md` and `docs/plans/agent-scaffold.ledger.md`
  as quoted review history, not as live claims about the code.
- **Hunted for truncation damage in both surviving doc comments.** Read both in
  full, final form:
  - `reserve_runner_worktree_with`'s doc (`src/checks.rs:502-505`): "`reserve_runner_worktree`
    (above) with its claim injected. Production passes `claim_dir` and is otherwise
    unchanged; the whole reservation, including the temp-dir creation above the
    loop, lives here so the tests drive the same code the runner does." Every
    remaining clause is true and the paragraph is grammatical; the sentence-fragment
    style ("... with its claim injected.") is unchanged from before the edit, not a
    new defect from truncation.
  - `a_lost_claim_retries_with_a_fresh_name_and_never_returns_the_lost_one`'s doc
    (`src/checks.rs:1722-1724`): "Layer 2's collision handling, driven at the use
    site rather than only at `claim_dir`. The claim is injected: this one records
    every name it is offered and loses the first two, which is the state the loop
    exists for." True, coherent, and does not depend on the deleted clause for
    sense.
- **Hunted for orphaned "see above" references.** No comment elsewhere points at
  either deleted sentence by name or position. The `claim_dir` doc block
  (`src/checks.rs:437-447`, untouched by this diff) says "both outcomes are pinned
  directly by a test (`a_directory_claim_is_exclusive`)", referring to the
  won/lost boolean outcomes for the uniqueness argument, a claim that was true
  before this commit and remains true after it; it does not claim to cover the
  error arm and was not touched by the diff, so it is not something this commit
  could have orphaned.
- **Hunted for a new fixture-name class that could leak on a failing run.** The
  13 added lines inside `a_directory_claim_is_exclusive` create `dir.join("a-regular-file")`
  and claim `file.join("under-a-file")`, both nested under the same `dir` the test
  already creates via `scratch("claim-dir")` (`src/checks.rs:1700`, unchanged).
  `scratch` names its directory `agent-scaffold-checks-test-{pid}-{name}` and
  `remove_dir_all`s any stale one before creating fresh
  (`src/checks.rs:1037-1046`, unchanged), so the new lines add no new top-level
  temp-dir name; they can only ever leak nested inside a name the test already
  owned. This matches the commit message's claim exactly.
- **Verified the platform-observed error kind actually used by the new assertion.**
  Mutation 3 (probe) confirms the real error diverting to `NotADirectory` (os error
  20) on this Linux system when the fixture claims a path under a plain file, so
  `assert_ne!(error.kind(), io::ErrorKind::AlreadyExists, ...)` is checking a claim
  against a genuinely different, non-`AlreadyExists` `ErrorKind`, not a
  vacuously-true comparison against a value it happens never to hit.

## NON-FINDINGS (checked, and settled or not raised on purpose)

- **`RG4` / Invariant B over-claim near `src/checks.rs:49-56`.** Confirmed still
  present, confirmed untouched by this diff (outside the 502-505 / 1701-1717 /
  1721-1724 hunks). Per the assignment this is an accepted residual by human
  decision, known, `low`, deliberately unfixed. Not raised.
- **Whether the new test's third-outcome behaviour is portable across OSes.**
  The new comment and assertion make no cross-platform claim (they assert
  `!= AlreadyExists`, not a specific `ErrorKind`, and the commit message frames it
  as "without needing a permissions fixture," not as an OS-independence claim), so
  there is no false portability claim to raise. This project's toolchain is
  Nix/Linux-only per its flake, and I did not chase this further since no text in
  the diff asserts anything about other platforms.
- **The `claim_dir` doc's "both outcomes are pinned directly by a test" sentence**
  (`src/checks.rs:446-447`, untouched by this diff). Considered as a possible
  stale-coverage claim since a third outcome now exists and is also tested. Ruled
  out: the sentence scopes itself explicitly to the two outcomes that carry "the
  second and decisive layer of `reserve_runner_worktree`'s uniqueness argument" (won
  vs. lost), which the error arm is not part of. The sentence was true before this
  commit and remains true after it, unaffected by the diff.
- **The `a_claim_error_that_is_not_a_collision_propagates_at_once_and_names_the_path`
  test** (`src/checks.rs:1777`, pre-existing, from round 3's `bc9908e`). Checked for
  overlap/duplication with the new test. Not a duplicate: that test exercises the
  *caller's* handling of an injected non-collision error at `reserve_runner_worktree_with`'s
  loop level; the new assertions in `a_directory_claim_is_exclusive` exercise
  `claim_dir` itself, uninjected, via a real filesystem error. Complementary, not
  redundant, and the new test's own comment ("neither assertion above reaches")
  correctly scopes "above" to the two assertions inside the same test function, not
  to this other test.

## Verdict

ZERO findings from this lens. The reduced fix set (two pure deletions plus the
inline assertion addition) does not re-seed: every added sentence checked out true
against reproduced evidence, both deleted sentences are fully gone with no
paraphrase or third instance surviving anywhere in `.rs` sources, no dangling
reference points at the deleted text, and the new coverage claim ("the third
outcome ... which neither assertion above reaches") is accurate and confirmed by
mutation testing in both directions.

## Hygiene

- `git status --short` (worktree, after final revert): empty.
- `git diff HEAD` (worktree, after final revert): empty.
- Byte comparison: `git show 596548b:src/checks.rs` saved to a scratch copy and
  `cmp`'d against the live `src/checks.rs`: `BYTE-IDENTICAL`, no difference.
- All three mutations (two logic mutations, one probe-only instrumentation) were
  reverted individually via `Edit` before applying the next, and the file was
  re-verified clean at the end.
- `/tmp` count: 0 created by this review in bare `/tmp`. `TMPDIR` was exported to
  the scratchpad path (`/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/ver-b`)
  in every Bash call, and the test binary's own scratch fixtures (from `scratch()`
  in `src/checks.rs`) landed there under this session's pids, not in bare `/tmp`,
  confirming the export took effect. Those fixtures and the saved committed-blob
  copy have since been deleted; the scratchpad directory is now empty. A repo-wide
  scan of bare `/tmp` (`find /tmp -iname "*agent-scaffold*" -not -path
  "/tmp/claude-1000/*"`) found 72 pre-existing directories from unrelated, earlier
  pids (consistent with the assignment's warning that a previous reviewer ran
  `cargo test --lib` and other unscoped commands); none carry this session's pids,
  none were created or touched by this review, and none were deleted by me since
  they belong to other sessions' history, not this one.
