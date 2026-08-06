# `workflow-enforcement-tier-inc3` work review, round 2, FIX-INDUCED-RESIDUE lens

Branch `review/inc3-r2-residue` at `141cf1c`, the tip of the branch under review. Target: the fix pass alone, `git diff 18176fa..HEAD`, which touches `src/main.rs`, both test files, `CHANGELOG.md`, `README.md`, `pack/AGENTS.md` and its two deployed copies, and `docs/plans/agent-scaffold.ledger.md`. Every claim below was checked by running a command (a built binary, a fresh fixture, or a real git clone); commands and output are quoted in full.

Three findings, all `low`. No `medium`, `high`, or `critical`. All three remedies are deletions or number-only substitutions; none adds prose.

---

## `R2B-1` (low): the new test's doc comment cites a commit hash that a real clone of this repository does not have

### The text

`tests/validate_workflow_toml_source_needs_no_plan.rs:279-282`, the doc comment on `a_round_log_that_cannot_be_checked_is_not_reported_as_missing`:

> RED against `1799f8b`, the round 1 tip: that build panicked the `no round log at` assertion with the false sentence in its stderr, and is green here.

### Why it is false or incomplete

`1799f8b` is not reachable from any branch or tag in this repository. It survives only as a dangling object kept alive by a reflog entry on `impl/wet-inc3` from a rebase (`rebase (finish): refs/heads/impl/wet-inc3 onto 18176fa...`). A real clone of this repository, the way any contributor or CI system actually obtains the code, transfers only objects reachable from refs, so it never receives this commit at all. The claim "RED against `1799f8b`" is therefore already unverifiable outside this exact working copy, and it will stop being resolvable even here once the reflog entry expires and the object is pruned (git's defaults: 90-day reflog expiry for unreachable entries, then pruning).

### Evidence

```
$ git branch -a --contains 1799f8b   # (from the worktree under review)
(no output)
$ git tag --contains 1799f8b
(no output)
$ git reflog --all | grep 1799f8b
1799f8b refs/heads/impl/wet-inc3@{4}: rebase (finish): refs/heads/impl/wet-inc3 onto 18176fa3a47f2be9cce1175006186b4646efeabe
1799f8b worktrees/impl-wet-inc3/HEAD@{10}: rebase (finish): returning to refs/heads/impl/wet-inc3
1799f8b worktrees/impl-wet-inc3/HEAD@{11}: rebase (pick): docs: qualify the SE-3 backstop promise as the instrumented tier

$ git clone --quiet --no-local /home/jessea/Documents/projects/agent-scaffold /tmp/.../freshclone2
$ cd /tmp/.../freshclone2 && git cat-file -t 1799f8b
fatal: Not a valid object name 1799f8b
```

`--no-local` forces the network-style object-negotiation path (a same-filesystem `git clone` without it copies the whole object store, including dangling objects, and would have hidden this). The commit this citation actually needs is the branch tip immediately before `4801898` in the FINAL history, which is `7ce4443`; `1799f8b` was that tip only in a pre-rebase version of the branch that no longer exists in any ref.

The same pattern recurs in `4801898`'s own commit message ("The new test is RED against 1799f8b, the branch tip before this commit"), and the ledger's `dd947a7` paragraph cites three more now-unreachable hashes (`af850b5`, `16531c5`, `691f88f`) for the same reason (that paragraph was written after a later rebase moved it earlier in the graph). I am not raising those as separate findings: a commit message is not shipped, editable documentation, and a ledger entry is a dated journal record of a point in time, not a claim meant to stay independently checkable forever. The test file's doc comment is different in kind: it ships with the code indefinitely and is the one place a future maintainer would actually go looking to verify the red-then-green story.

### Smallest remedy

Delete the specific hash and its qualifier, which needs no replacement content: "RED against `1799f8b`, the round 1 tip: that build panicked" becomes "RED before this commit: the prior build panicked". Net change: two words shorter, and the claim no longer depends on an object that a real clone does not carry.

---

## `R2B-2` (low): the corrected tier-boundary sentence still opens on the flag it was rescoped away from, and a freshly `--instrument`ed project fails identically to one without it

### The text

`pack/AGENTS.md:93` and its two byte-identical deployed copies (`AGENTS.md:93`, `.agents/AGENTS.reference.md:93`):

> ... when instrumentation is on, the deterministic `validate --workflow` check is the backstop that the required reviewed rounds happened before a step is marked complete, and on a project with no round log yet, **which every project scaffolded without `--instrument` remains**, that check exits non-zero reporting that it could not run rather than passing.

`README.md:210`: "... and `--workflow` fails on a project with no round log yet, **which every project scaffolded without `--instrument` remains**; plain `validate` ..."

`CHANGELOG.md:23`: "THE POPULATION THIS BREAKS is every project with no round log at the resolved path, **which every project scaffolded without `--instrument` remains**: such a project has the guidance tier of the workflow ..."

### Why it is false or incomplete

This is round 1's `T-2` remedy, applied verbatim (the round-1 triage's own recommended text for this clause is exactly "which every project scaffolded without `--instrument` remains"). The rescoping from flag to log is correct and I am not re-raising `T-2`. What is new: the sentence still opens with "when instrumentation is on ... is the backstop" and still singles out the non-`--instrument` population by name, which invites the same inference `T-2` fixed one layer up: that turning the flag on gets you the working backstop. It does not, immediately after scaffolding, and I checked this by running the check on both populations rather than assuming the fixed clause was safe.

### Evidence

Two fixtures, one scaffolded with `--instrument`, one without, using the branch binary:

```
$ agent-scaffold scaffold --instrument --write --output-dir with-instrument
$ agent-scaffold scaffold --write --output-dir without-instrument
$ find with-instrument -iname '*metrics*' -o -iname '*.jsonl'
(no output)
$ find without-instrument -iname '*metrics*' -o -iname '*.jsonl'
(no output)

$ (cd with-instrument && agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow)
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, ...
exit=1

$ (cd without-instrument && agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow)
(byte-identical stdout, stderr, and exit code)
```

`--instrument` renders no `docs/metrics/` and no log (confirmed again here; this is the same fact `T-2` established). So the very first thing a reader who follows the README's advice to scaffold with `--instrument` and then runs the check the sentence just promised them would see is the identical exit-1 failure the sentence attributes, by name, to projects that skipped the flag. The sentence's literal claim ("no round log yet, which every project scaffolded without `--instrument` remains") is defensible read narrowly, since it only asserts something about the non-instrumented population and never claims the instrumented one is exempt; a careful reader who treats "no round log" as the operative rule and the `--instrument` clause as illustrative rather than exhaustive predicts correctly. But the sentence is not written for that careful a reading, and the opening clause it retained ("when instrumentation is on ... is the backstop") sets up exactly the contrast the fixture disproves.

### Smallest remedy

Delete the clause naming the flag; the sentence needs it least of anywhere, since the log (not the flag) is already the thing every other part of the sentence hinges on:

- `pack/AGENTS.md:93` (one edit, one re-render of the two deployed copies): "and on a project with no round log yet, which every project scaffolded without `--instrument` remains, that check exits non-zero" becomes "and on a project with no round log yet, that check exits non-zero".
- `README.md:210`: "and `--workflow` fails on a project with no round log yet, which every project scaffolded without `--instrument` remains; plain `validate`" becomes "and `--workflow` fails on a project with no round log yet; plain `validate`".
- `CHANGELOG.md:23`: "is every project with no round log at the resolved path, which every project scaffolded without `--instrument` remains: such a project" becomes "is every project with no round log at the resolved path: such a project".

All three are pure deletions and none weakens the acceptance-check-20 property: a reader predicts the exit code from "has a round log or does not," which is the true discriminator, without the flag mentioned at all.

---

## `R2B-3` (low): the ledger's own gate re-run understates how many tests the fix pass added

### The text

`docs/plans/agent-scaffold.ledger.md:535`:

> GATES RE-RUN BY THE ORCHESTRATOR: 422 tests passing with 0 failures, **up from 421 by the one new test**, clippy silent at 0 matching lines, `render --check` up to date.

### Why it is false or incomplete

The headline total, 422, is correct (reproduced below). The breakdown is not: the true pre-fix-pass baseline is 420, not 421, and the fix pass adds two new test functions, not one, both in `tests/validate_workflow_toml_source_needs_no_plan.rs`. `6de3a8f` adds `workflow_with_no_metrics_log_hard_errors_instead_of_skipping` (2 -> 3 test functions in that file); `4801898` adds `a_round_log_that_cannot_be_checked_is_not_reported_as_missing` (3 -> 4). 420 + 2 = 422, matching the correct total by a route the sentence does not describe. The "421" figure is the round-1 triage's own count at `74e6426`, a now-unreachable pre-rebase tip that already included the first of the two new tests; the ledger paragraph, itself written after the second test existed (its `222/10` diff figure for the same file already sums the contributions of all five fix-pass commits, confirmed below), carried the triage's stale baseline forward instead of the true one.

### Evidence

```
$ git show 18176fa:tests/validate_workflow_toml_source_needs_no_plan.rs | grep -c '#\[test\]'
2
$ (build from 18176fa) cargo test 2>&1 | grep -oE '[0-9]+ passed' | awk '{s+=$1} END{print s}'
420
$ (build from HEAD) cargo test 2>&1 | grep -oE '[0-9]+ passed' | awk '{s+=$1} END{print s}'
422

$ for c in 18176fa dd947a7 6de3a8f 7ce4443 60679ca 4801898 141cf1c; do
    git show "$c:tests/validate_workflow_toml_source_needs_no_plan.rs" | grep -c '#\[test\]'
  done
2 2 3 3 3 4 4        # 6de3a8f: 2->3; 4801898: 3->4

$ git diff 18176fa..HEAD --numstat -- tests/validate_workflow_toml_source_needs_no_plan.rs
222   10   tests/validate_workflow_toml_source_needs_no_plan.rs   # matches the ledger's own "222/10"
```

The `222/10` the ledger cites for this file already reflects the cumulative effect of `6de3a8f` (+133/-10), `60679ca` (+2/-2), and `4801898` (+90/-1) together, which is only possible if the paragraph was written with knowledge of all three; the test-count clause was not updated to match.

### Smallest remedy

Two numbers, no words added: "up from 421 by the one new test" becomes "up from 420 by the two new tests".

---

## What I checked and found true (the surface this round covers)

- **The new arm's inline comment and both error-message branches** (`src/main.rs:1044-1073`). Reproduced both: a genuinely missing log gives `--workflow requested but no round log at <path>: ... pass a --metrics naming this project's log, or record the project's review rounds there` at exit 1; a mode-600 (readable, unsearchable) ancestor directory gives `--workflow requested but the round log at <path> could not be checked (Permission denied (os error 13)): ...` at exit 1, with a mode-755 control on the same fixture reading the log and printing `workflow invariants hold` at exit 0. Both match the comment's "TWO CLAIMS, NOT ONE" description exactly.
- **The claim that this is "the last way `--workflow` could reach exit 0 without checking anything."** Traced every arm of the match plus the containment guard and the `--workflow-spec` parse error; confirmed the `_` catch-all was indeed the sole remaining silent-pass path before this fix.
- **The run_validate doc-comment rewrite** ("`--workflow` IS THE DELIBERATE EXCEPTION ... Both of the check's inputs answer that way: no resolvable plan source, and no round log at the resolved metrics path"). Matches the two problem-pushing arms exactly; the deleted clauses ("which still requires `--plan`", "`--plan` stays clap-required for now") were already false before this pass touched them and nothing else in the touched files depends on either deleted clause (grepped `once built` and `requires` `--plan`` across the whole tree; the only remaining hits are in untouched planning documents describing the sentence historically, out of this fix pass's scope).
- **The bare-filename-from-`docs/plans` case** (`README.md`, `CHANGELOG.md`, and the renamed test in `tests/metrics_and_ledger_anchor_to_the_plan_source.rs`). Reproduced: exit 1 naming the path it looked for, matching all three texts.
- **The three re-scoped tier-boundary sentences, checked against each other and against behaviour.** `pack/AGENTS.md` (+ 2 copies), `README.md`, and `CHANGELOG.md` all draw the boundary at the round log rather than the flag, and all three are consistent with each other's claim (mod `R2B-2` above) and with running `validate --source docs/plans/TEMPLATE.plan.toml --workflow` on fixtures scaffolded both with and without `--instrument` (byte-identical output, exit 1, both times).
- **The `Ok(_)` arm of `metrics_path.try_exists()`.** Matched against a wildcard rather than `Ok(false)`, but the preceding `metrics_path.exists()` gate already returned `false` to reach this arm, so `try_exists()` on the same path can answer `Ok(true)` only via a TOCTOU race with something else creating the file mid-function. Not deterministically reachable; matches the implementer's own reasoning that a third message for this case is prose for nothing. Held, not raised.
- **The test module's "This pins three directions" enumeration against its four `#[test]` functions.** The fourth (`a_round_log_that_cannot_be_checked_is_not_reported_as_missing`) is folded into the third bullet's own text ("A path the tool cannot answer that question for gets the same exit 1 and a different sentence"), matching the same 1+1+2 grouping the `--workflow` help string uses. Nothing is omitted from the description; only the count of bulleted headings is coarser than the count of tests. Held, not raised.
- **The 365-word test's assertions and the 59 words of assertion messages**, all four sub-cases of `workflow_with_no_metrics_log_hard_errors_instead_of_skipping` and both branches of `a_round_log_that_cannot_be_checked_is_not_reported_as_missing`: ran `cargo test` (`TMPDIR` outside the repo) and reproduced every sub-case by hand; all assertion text matches actual stdout/stderr.
- **Full suite and regression check**: `cargo test` at HEAD, 422 passing, 0 failing (matches the ledger's headline number, see `R2B-3` for the breakdown it attaches to that number). `cargo clippy --all-targets -- -D warnings` fails identically on a pre-existing, environment-dependent `dead_code` lint (`enum_field!`'s `VARIANTS` constant) reproduced unchanged on a build from `18176fa`; this predates the fix pass and is not residue from it. The project's own `just clippy` recipe (no `-D warnings`) is unaffected.

---

## Constraints observed

No line-length or wrapping findings. No style preferences raised. Round 1's `ADV-4` (accepted residual), `SC-3` (invalid), and the four human-decided residuals and accepted costs (i)-(iv) in the ledger were not re-raised; none of the three findings above touches any of them. The pre-existing plain-`validate` inconsistency on unreadable logs (mode-000 file vs. unsearchable directory) was not raised; it is queued to the validation-constraints step by prior decision and none of this round's evidence bears on it beyond what round 1 already recorded.
