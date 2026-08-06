# `workflow-enforcement-tier-inc3` work review, round 3, CLOSURE-AND-COMMENT-TRUTH lens

Reviewed on branch `review/inc3-r3-closure` at `ce820fb`, the tip of the branch under
review, in worktree `.claude/worktrees/rev-inc3-r3-closure`. Governing specification:
`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`. Round 1's triage
(`workflow-enforcement-tier-inc3-workreview-r1-triage.md`) and round 2's triage
(`workflow-enforcement-tier-inc3-workreview-r2-triage.md`) were both read in full before
any verdict below, and every ruling in them is treated as settled.

METHOD. `which cargo` resolves to `/nix/store/76jaab43a2l7n7fiifxjngp68kk167vm-rust-mixed/bin/cargo`
(1.98.0-nightly) after `direnv allow && eval "$(direnv export bash)"`, confirmed before
every build-dependent claim below; every toolchain command in this review ran with that
exact prefix. `TMPDIR` was pointed at
`/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/r3-closure/tmpdir`,
outside any git repository, for every `cargo test`. All fixtures live under
`/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/r3-closure/`,
a directory of this reviewer's own naming; every directory chmodded to 600 or 000 during
a reproduction was chmodded back to 755 immediately after, and a closing
`find <scratch>/r3-closure -type d ! -perm -u+rwx` returns nothing. One binary was built,
`target/debug/agent-scaffold` at `ce820fb` in this worktree; the reviewed worktree itself
was never edited, and `git status --short` is empty here throughout and at the time of
writing.

Nothing below is adjudicated by reading. Every verdict rests on a command run against
the built binary, or against `cargo test` itself (once normally, once under
`unshare -Ur` as namespace root), with the fixture and its output shown.

---

## Job A: closure table

All ten prior findings were reproduced against the current tip using the original
demonstration (or, where the triage gave an exact fixture, that fixture rebuilt by hand
from the schema the test file itself uses). All ten are CLOSED. None of the closures
broke another check: the full suite is 422 passed / 0 failed as the ordinary user and
422 passed / 0 failed under `unshare -Ur` as namespace root, `cargo clippy --all-targets
-- -D warnings` is clean in the flake toolchain, and `render --check` reports up to date.

| Finding | Closed | Command | Observed |
| --- | --- | --- | --- |
| `T-1` (round log falsely reported absent behind an unsearchable ancestor) | YES | `chmod 600 docs/metrics` (readable, not searchable) over a real one-record log at `docs/metrics/workflow.jsonl`, then `validate --source p.plan.toml --workflow` | `--workflow requested but the round log at docs/metrics/workflow.jsonl could not be checked (Permission denied (os error 13)): ... pass a --metrics naming this project's log`, exit 1. No longer asserts absence or prescribes recording rounds already recorded. `chmod 755` control: `workflow invariants hold`, exit 0. |
| `T-2` (tier boundary written at `--instrument`, not the log) | YES | Scaffolded `t2-noinst` (no `--instrument`) and `t2-inst` (with `--instrument`); `diff -rq` shows only `AGENTS.md`/`AGENTS.reference.md` differ; ran `validate --source TEMPLATE.plan.toml --workflow` in each | Byte-identical stdout/stderr/exit(1) in both. `pack/AGENTS.md:93`, `README.md:210` and the new `CHANGELOG.md` bullet all now read "on a project with no round log yet, which every project scaffolded without `--instrument` remains" (log-scoped, not flag-scoped). `once built` also removed (see `T-4`). |
| `T-3` (`PLAN_MD` fixture claimed schema-valid, was hyphenated `not-started`) | YES | Wrote the current `PLAN_MD` fixture verbatim (`| \`only-step\` | not started |`, space) to a file and ran `validate --plan plan.md` (no `--workflow`) | `plan.md: 1 steps, 0 open-questions items, valid`, exit 0, genuinely schema-valid now, matching the doc comment's claim. `PLAN_TOML`'s hyphenated `not-started` deliberately left untouched (correct for the TOML schema). |
| `T-4` (`once built` retained on a check that runs today) | YES | `grep -n "once built" pack/AGENTS.md AGENTS.md .agents/AGENTS.reference.md README.md CHANGELOG.md` (no hits); `validate --source docs/plans/agent-scaffold.plan.toml --workflow` on this repo's own plan | Clause gone from all three sites (all three copies byte-identical at line 93, drift guard holds). The check runs today and reports `workflow invariants hold`, exit 0. |
| `T-5` (README says bare-filename-from-`docs/plans` "reports that it found no log") | YES | `cd docs/plans && validate --source TEMPLATE.plan.toml --workflow` on a fresh scaffold | `no round log at docs/metrics/workflow.jsonl: the workflow check could not run ...`, exit 1. `README.md:234` now reads "fails, naming the log it looked for", matching. |
| `T-6` (CHANGELOG `Added` bullet says `--workflow` requires `--plan`, contradicted by the same section's `Changed` bullet) | YES | `grep -n "workflow: bool" -A3 src/main.rs` (no `requires`); `CHANGELOG.md` `Added` bullet text; `validate --source ... --workflow` with no `--plan` | `workflow: bool` carries no `requires`; the check runs and passes with no `--plan`; CHANGELOG now reads "It reuses the same metrics log as the rest of `validate`," the three deleted words gone. |
| `V-1` (new test fails outright as namespace root, message describes the opposite of what happened) | YES | `cargo test --test validate_workflow_toml_source_needs_no_plan` both normally and under `unshare -Ur env PATH="$PATH" TMPDIR="$TMPDIR" HOME="$HOME" cargo test ...` | Both: `4 passed; 0 failed`. As the ordinary user `opaque` is still true (mode 600 still bites), so every assertion inside `if opaque` still runs and is checked exactly as before; as namespace root `opaque` is false and only the closing control runs, which passes in both environments. The move did not weaken what is pinned for the ordinary user. |
| `V-2` (arm re-stats the log, two disagreement directions constructible with a FIFO) | YES | FIFO plan source, two race cells: Cell 1 (log created mid-run after a truthful absent gate answer) and Cell 2 (dir made unsearchable mid-run after a truthful absent gate answer, log never exists) | Cell 1: still prints the absent sentence (inherited TOCTOU, not a regression; the single probe was captured before the window and cannot see the later write, and no implementation that stats once can do better). Cell 2: now prints `no round log at docs/metrics/workflow.jsonl ...` (the TRUE sentence), NOT the false `could not be checked`. The single-probe rebinding (`metrics_probe` computed once at `src/main.rs:845`, reused via `&metrics_probe` at `:1067`) closed the false-direction regression. |
| `V-3` (`Err` arm fired on every errno, losing the `--metrics` remedy on ENOTDIR/ELOOP/ENAMETOOLONG) | YES | Built ENOTDIR (`--metrics` inside a file), ELOOP (self-referential symlink), ENAMETOOLONG (300-char leaf), and the static mode-600 EACCES fixture; control: dangling symlink (ENOENT via link-follow) | All four errno cases: `... could not be checked (<errno>): ... ; pass a \`--metrics\` naming this project's log`. Control (dangling symlink, genuinely absent): `no round log at ...: ... ; pass a \`--metrics\` naming this project's log, or record the project's review rounds there`, the split between "cannot tell" and "confirmed absent" still holds correctly. |
| `V-4` (test doc comment cited an unreachable commit hash) | YES | `grep -n "RED before\|RED against" tests/validate_workflow_toml_source_needs_no_plan.rs` | Reads "RED before this commit: the prior build printed ..."; no commit hash anywhere in the file. |

---

## Job B: the three flagged items, ruled

**1. The DECLINED instruction to delete the `Ok` asserts absence clause.** RULED: the
implementer's refusal was correct; the clause is TRUE as the code now stands, and I
verified this by construction rather than by trusting the commit message. At
`src/main.rs:845-846`, `metrics_probe` is computed exactly once
(`metrics_path.try_exists()`) and reused via `&metrics_probe` at the match arm
(`:1067`); there is no second call. Enumerating every tuple the enclosing
`match (toml_primary, &plan_contents, &metrics_contents)` can produce, the only tuples
that reach the `_` catch-all (`:1067`) have `metrics_contents == None`, and
`metrics_contents` is `None` if and only if `metrics_probe != Ok(true)` (when it is
`Ok(true)`, `fs::read_to_string` either succeeds, producing `Some(contents)`, or fails
and propagates via `?`, returning early and never reaching the match at all). So
`metrics_probe` can only be `Ok(false)` or `Err(_)` at the point the `Ok(_) =>` arm
matches, and `Ok(false)` from `try_exists()` is by definition a confirmed absence (any
other stat failure surfaces as `Err`). Reproduced empirically with the same FIFO
technique `V-2` used (see the closure table): every path that reaches the `Ok(_)` arm in
practice really is a confirmed-absent log. The clause is true; deleting it would have
lost information.

**2. The removed gate-line comment.** RULED: not a real hazard, no remedy needed. The
gate at `src/main.rs:845` reads `let metrics_probe = metrics_path.try_exists();`
immediately followed on the next line by `matches!(metrics_probe, Ok(true))`, so a
reader who reaches the `try_exists()` call at all sees, on the very next line, that the
result is pattern-matched down to the old `exists()` boolean rather than propagated with
`?`. That is the tell that distinguishes this from `Q-55-existsgate`'s declined
`try_exists()?` change (which would propagate the error and alter plain `validate`), and
it sits inside the same two-line span, not pages away. The comment 210 lines below
(`:1064-1066`, "ARM-SCOPED BY `Q-55-existsgate`: the gate above keeps that predicate, so
plain `validate` is untouched") states the resolution explicitly for anyone who reads
that far, and the commit message (`7f2e3c3`) states it a third time. I looked for a
concrete way a reader would be misled in practice; grepping for `try_exists` (a
plausible move for anyone auditing this) surfaces only one call in `run_validate`, at
the gate, with its disambiguating `matches!` on the same line pair, and did not find
one. Given the no-new-sentence constraint and this project's preference for deletion
over addition, adding a comment here to guard against a misreading that the adjacent
code already forecloses would not earn its words.

**3. `V-3`'s clause landing on EACCES too, so the sentence is no longer byte-identical
to what `T-1`'s fix produced.** RULED: true on every errno it can now reach, confirmed
by construction across all four reachable classes (see `V-3` in the closure table:
ENOTDIR, ELOOP, ENAMETOOLONG, EACCES all print "... pass a `--metrics` naming this
project's log", and none of them can exist at the resolved path, so the advice to name a
different `--metrics` is not false for any of them: it does not claim the current path
is wrong for a reason other than being unreachable, only that a different, reachable path
would let the check run). The control (dangling symlink, which resolves to `Ok(false)`
via `try_exists`'s ENOENT-mapping) still gets the fuller sentence with "or record the
project's review rounds there", so the two-way split the round 1 fix introduced is intact;
only the `Err`-side sentence gained the shared clause, and it is true on every errno
observed.

---

## Findings

One finding, discovered while auditing every comment in the changed code rather than
trusting the three flagged items to be the whole list.

### `R3B-1`, low: the comment describing the `Err` arm's message claims it says "only"
two things, and the message has said a third thing since `V-3`'s fix

**The comment**, `src/main.rs:1059-1063` (unchanged since round 2's `7f2e3c3`, before
round 2's later `V-3` fix `ce820fb` touched the code it describes):

> `Ok` asserts absence and prescribes recording rounds, `Err` says only that the
> question could not be answered and names the error, in the vocabulary
> `note_missing_anchors` already uses, because a real log may sit behind that error and
> sending its operator to record rounds that are already recorded is the falsehood
> `Q-55-emptyroot` decided against.

**Why it is stale.** When this comment was written (`7f2e3c3`), the `Err` arm's format
string was:

```
"--workflow requested but the round log at {} could not be checked ({error}): the
workflow check could not run, so it cannot report that the invariants hold"
```

which really did say only two things about the failure: that the question could not be
answered, and the error. `ce820fb` (the `V-3` fix, endorsed by round 2's triage and
confirmed true on every reachable errno above) appended a third thing to the same
string, still live at `src/main.rs:1072-1075`:

```
"--workflow requested but the round log at {} could not be checked ({error}): the
workflow check could not run, so it cannot report that the invariants hold; pass a
`--metrics` naming this project's log"
```

`ce820fb` touched only the format string; it did not touch the comment above it. Read
literally, "says only that the question could not be answered and names the error" is
now an exhaustiveness claim the code no longer satisfies: the message also gives the
same actionable remedy the sibling `Ok(_)` arm gives (minus that arm's "or record the
project's review rounds there" clause, which the rest of the sentence, correctly,
still explains the omission of). A reader who trusted the comment's literal claim rather
than the code, exactly the failure mode this lens exists to catch, would believe the
`Err` arm offers no actionable next step beyond naming the error, when it has offered
one since `ce820fb`.

**Command and output**, confirming the current message contains the clause the comment's
"only" does not account for:

```
$ ln -sf loopy docs/loop/loopy   # ELOOP fixture, reused from the V-3 closure check
$ agent-scaffold validate --source docs/plans/p.plan.toml --workflow --metrics docs/loop/loopy
--workflow requested but the round log at docs/loop/loopy could not be checked (Too many
levels of symbolic links (os error 40)): the workflow check could not run, so it cannot
report that the invariants hold; pass a `--metrics` naming this project's log
exit=1
```

**Severity.** `low`: this is a Rust source comment with no external visibility (no user
or CLI surface reads it), the contrastive point the comment exists to make, that `Err`
deliberately omits "record the project's review rounds there", remains completely
true and is not what "only" is about. The cost of leaving it is the same class `V-4` was
rated for: a future reader relying on the comment rather than the code could waste
effort re-proposing a remedy `V-3` already shipped.

**Smallest remedy**, a one-word deletion, matching this project's stated preference for
deletion over addition: delete "only". "`Err` says only that the question could not be
answered and names the error" becomes "`Err` says that the question could not be
answered and names the error". Nothing else in the sentence needs to change; the
`because` clause that follows still correctly explains why "record the project's review
rounds there" specifically is omitted, which is the actual point being made.

---

## Tally

| Severity | Count | Findings |
| --- | --- | --- |
| critical | 0 | |
| high | 0 | |
| medium | 0 | |
| low | 1 | `R3B-1` |

CLOSURE RESULT: 10 of 10 prior findings (`T-1` through `T-6`, `V-1` through `V-4`)
confirmed CLOSED by reproduction against the current tip, none broken by another
closure. Full suite 422/0 as the ordinary user and 422/0 under `unshare -Ur` as
namespace root; `cargo clippy --all-targets -- -D warnings` clean in the flake
toolchain; `render --check` up to date; containment/unsafe-pairing behaviour
(a foreign `--metrics` outside the plan's root) reproduced unaffected by the
single-probe rebinding.

All three items the implementer flagged for this round were checked and ruled: item 1
(the `Ok` asserts absence clause) is TRUE and correctly kept; item 2 (the removed gate
comment) is not a real hazard and earns no remedy; item 3 (`V-3`'s clause landing on
EACCES) is true on every errno it can now reach.

RELITIGATION CHECK. `R3B-1` is not a re-raise: it targets a comment/code pairing that
only became inconsistent at `ce820fb`, the last commit in the branch, and no round 1 or
round 2 artifact discusses it. Nothing here re-raises the four standing residuals (the
in-root bound, the single-anchor `..` case, the earlier increment's rejected-ledger
context slot, the off-convention `--source` surface), the accepted costs (i) through
(iv), round 1's `T-7`/`ADV-4` or round 2's `R-1`/`R2A-4` (both ACCEPTED AS RESIDUALS and
not re-raised here), round 1's `T-8`/`SC-3` or round 2's `X-1`/`R2B-2` and `X-2`/`R2B-3`
(all INVALID), `Q-55-existsgate`'s DECLINED `try_exists()?` gate change, or the
pre-existing plain-`validate` inconsistency QUEUED to the validation-constraints step. No
line-length, wrapping, or ragged-comment observation appears anywhere in this file. The
reviewed worktree was never edited; `git status --short` is empty both before and after
writing this file, and no temporary source edit was made or needed.

This is the first clean-of-medium-or-higher round on this artifact (round 1: six valid,
ceiling medium; round 2: four valid, ceiling medium; round 3: one valid, ceiling low). At
`low` with a clear, minimal remedy, whether this round counts toward the two-consecutive-
clean streak is the orchestrator's call under the convergence rule.
