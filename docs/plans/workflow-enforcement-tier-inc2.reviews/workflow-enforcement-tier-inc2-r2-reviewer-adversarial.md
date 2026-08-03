# Review findings: `workflow-enforcement-tier-inc2`, ROUND 2, ADVERSARIAL CONSTRUCTION lens

LENS. Adversarial construction. Nothing below was concluded by reading the diff. Every claim was produced by building a project layout on disk, running a built binary against it, and recording the streams and the exit code. Both findings are reduced to a self-contained script that builds its own fixture from scratch.

ARTIFACT. Worktree `.claude/worktrees/r2-adversarial`, HEAD `6bf5280` ("fix: supply a root to the surfaces that read no plan, and pin six unguarded clauses"). The fix pass under attack is `git diff HEAD~1..HEAD`; the whole increment is `git diff main..HEAD`. Binaries: `cargo build --release` at HEAD, and a second build of the tree exported from HEAD~1 (`git archive HEAD~1 | tar -x -C <scratch>/r2adv-prev`, then `cargo build --release` there) so every behaviour change could be measured old against new rather than asserted.

TARGET. The new root-supply policy: `containment_roots` (`src/main.rs:1332-1339`), `checked_plan_root(...).map_or_else(|| resume_roots(source, plan), |root| vec![root])`, consumed at `src/main.rs:1149` (`run_status`), `:1551` and `:1571` (`run_next`), each root tested with `is_outside_root`.

SUITE STATE. `cargo test --release` is green at this HEAD, including all of the increment's own new acceptance tests (`a_surface_that_reads_no_plan_is_supplied_a_root` and the rest of `tests/unsafe_pairings_are_refused_and_omitted.rs`). R2A-1 below reproduces against that green suite.

## THE TWO QUESTIONS THE BRIEF ASKED DIRECTLY

CAN AN EMPTY ROOT VECTOR MEAN "CONTAINED IN EVERY ROOT" VACUOUSLY? YES, AND IT DOES. That is R2A-1. It is reachable whenever no anchor canonicalises, and the classic empty-quantifier failure is exactly what happens: `[].iter().find(...)` is `None`, both filters go vacuous, and another project's round log and `## RESUME STATE` block are read and printed at exit 0.

DOES THE `validate --workflow` ASYMMETRY CLAIM HOLD? YES, in every configuration I could build. See ATTACKS THAT FAILED, V-series. One accuracy correction to the claim's wording, which is not a finding: `validate` does READ and schema-check the named `--metrics` log before the `--workflow` guard runs, in every configuration including the rootless ones. What it never does is JOIN it to a plan without a root, because every rootless configuration lands in the `(None, None, _)` arm at `src/main.rs:1033` and exits 1. The substantive claim is sound; "reads a log with no root" is literally true and harmless, "joins a log with no root" is what matters and I could not construct it.

## R2A-1: an empty root vector is vacuously contained, so an anchor that does not exist disables containment entirely on `next`, `status` and `status --resume`

SEVERITY: high.

CLAIM. `containment_roots` returns an EMPTY vector whenever neither `--source` nor `--plan` names a file that canonicalises, and `.iter().find(...)` over an empty vector is `None`, so both of `next`'s filters and `status`'s one filter go vacuous exactly as they did before the fix: another project's round log is counted and another project's `## RESUME STATE` block is echoed verbatim at exit 0, with `"metrics_absent_reason": null` and `"resume_state_absent_reason": null` on the machine surface positively asserting that both are this plan's. One character changed in the `--source` path turns the increment's refusal into the increment's original defect.

WHY IT IS REACHABLE. `resume_roots` builds its roots through `canonical_project_root`, which is `fs::canonicalize(plan).ok().map(...)`: an anchor that does not exist canonicalises to `None` and is silently dropped by the `filter_map`. When BOTH anchors drop (or are absent), the vector is empty. The fix's own doc comment names the reachable configuration as "a Markdown-primary `--source` and no `--plan`", and it closed that one; it did not consider that `checked_plan_root` returns `None` for a MISSING plan as well as for an ABSENT one, and that the fallback it delegates to fails the same way.

REPRODUCTION. Self-contained; builds its own two-project fixture, no `scaffold` required. Save as `repro.sh` and run `bash repro.sh <path-to-agent-scaffold> <a scratch dir outside any repo>`.

```sh
set -eu
BIN="$1"; R="$2"
rm -rf "$R"; mkdir -p "$R/alpha/docs/plans" "$R/beta/docs/plans" "$R/beta/docs/metrics"

# Project ALPHA: a MARKDOWN-primary <task>.plan.toml. No --plan is given, so no plan is READ.
cat > "$R/alpha/docs/plans/p.plan.toml" <<'TOML'
[meta]
title = "alpha"
primary = "markdown"
TOML

# Project BETA: a disjoint sibling project with its own private ledger and round log.
cat > "$R/beta/docs/plans/b.ledger.md" <<'LED'
# beta ledger

## RESUME STATE

BETA PRIVATE RESUME STATE.

## NEXT
LED
cat > "$R/beta/docs/metrics/workflow.jsonl" <<'JSONL'
{"type":"round","task":"b","artifact":"b1","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk"}
{"type":"round","task":"b","artifact":"b2","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":2,"risk_class":"low_risk"}
{"type":"round","task":"b","artifact":"b3","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":3,"risk_class":"low_risk"}
JSONL

echo "=== CONTROL: the anchor EXISTS. Both artifacts are refused (the fix working). ==="
"$BIN" next --source "$R/alpha/docs/plans/p.plan.toml" \
            --metrics "$R/beta/docs/metrics/workflow.jsonl" \
            --ledger-fragment "$R/beta/docs/plans/b.ledger.md" || true
echo "exit=$?"

echo "=== ATTACK: ONE CHARACTER changed in the anchor (p -> q, a file that does not exist). ==="
"$BIN" next --source "$R/alpha/docs/plans/q.plan.toml" \
            --metrics "$R/beta/docs/metrics/workflow.jsonl" \
            --ledger-fragment "$R/beta/docs/plans/b.ledger.md" || true
echo "exit=$?"

echo "=== ATTACK, machine surface. ==="
"$BIN" next --json --source "$R/alpha/docs/plans/q.plan.toml" \
            --metrics "$R/beta/docs/metrics/workflow.jsonl" \
            --ledger-fragment "$R/beta/docs/plans/b.ledger.md" || true
echo "exit=$?"

echo "=== ATTACK, status --resume on the same anchors. ==="
"$BIN" status --resume --source "$R/alpha/docs/plans/q.plan.toml" \
              --ledger-fragment "$R/beta/docs/plans/b.ledger.md" || true
echo "exit=$?"

echo "=== ATTACK, status --json on the same anchors. ==="
"$BIN" status --json --source "$R/alpha/docs/plans/q.plan.toml" \
              --metrics "$R/beta/docs/metrics/workflow.jsonl" || true
echo "exit=$?"

echo "=== ATTACK, a NON-EXISTENT --plan instead of a non-existent --source. ==="
"$BIN" next --plan "$R/alpha/docs/plans/q.md" \
            --metrics "$R/beta/docs/metrics/workflow.jsonl" \
            --ledger-fragment "$R/beta/docs/plans/b.ledger.md" || true
echo "exit=$?"

echo "=== ATTACK, an anchor that EXISTS as a symlink loop (canonicalize fails). ==="
ln -sfn "$R/alpha/docs/plans/loop.plan.toml" "$R/alpha/docs/plans/loop.plan.toml"
"$BIN" next --source "$R/alpha/docs/plans/loop.plan.toml" \
            --metrics "$R/beta/docs/metrics/workflow.jsonl" \
            --ledger-fragment "$R/beta/docs/plans/b.ledger.md" || true
echo "exit=$?"
```

OBSERVED, verbatim, against `target/release/agent-scaffold` at HEAD `6bf5280` (paths abbreviated to `<R>`).

The CONTROL, which is the fix doing its job:

```
task: p
source: no plan source
metrics: unavailable, the round log <R>/beta/docs/metrics/workflow.jsonl is not under the plan's project root <R>/alpha, so its records cannot be paired with this plan

no active review loop (no plan steps found)

the ledger <R>/beta/docs/plans/b.ledger.md is not under the plan's project root <R>/alpha; nothing to resume
exit=0
```

The ATTACK, one character later:

```
task: q
source: no plan source
metrics: 3 records

no active review loop (no plan steps found)

RESUME STATE (verbatim from the ledger):
## RESUME STATE

BETA PRIVATE RESUME STATE.
exit=0
```

The machine surface on the same inputs, which is worse than silent because it is a positive assertion:

```
{
  "task": "q",
  "source": "no plan source",
  "metrics": {
    "records": 3
  },
  "metrics_absent_reason": null,
  "active_loop": null,
  "resume_state": "## RESUME STATE\n\nBETA PRIVATE RESUME STATE.",
  "resume_state_absent_reason": null,
  "no_active_loop_reason": "no-plan-steps"
}
exit=0
```

`status --resume` on the same anchors prints beta's block verbatim at exit 0. `status --json` prints `"metrics": {"records": 3}` with `"metrics_absent_reason": null`. The non-existent `--plan` variant and the symlink-loop-anchor variant both produce the same output as the `--source` attack above (`task: q` / `task: loop`, `metrics: 3 records`, beta's block echoed, exit 0). A fourth route, with NO anchor at all, does the same: `next --metrics <R>/beta/... --ledger-fragment <R>/beta/...` prints `task: task`, `metrics: 3 records`, and beta's block.

THE DISCRIMINATING CONTROL FOR THE IN-ROOT BOUND, which the brief requires before attributing anything to it. `<R>/alpha` and `<R>/beta` are TOP-LEVEL SIBLINGS with no containment relationship in either direction; the ledger that gets echoed is not nested inside alpha's subtree, and no root was derived at all, so the in-root bound is not what is happening here. To make the control explicit I ran the identical fixture in both arrangements: DISJOINT (above) reproduces, and the same command with the anchor spelled correctly (the CONTROL block) refuses. The variable is whether the anchor exists, not whether the artifact is nested. This is not the in-root bound and it is filed.

A SECOND ROUTE WITH NO EXPLICIT FLAGS AT ALL, for how ordinary the trigger is. Two scaffolded projects A and B, `<A>/docs/plans/onlyA.plan.toml` Markdown-primary and existing only in A, `<B>/docs/plans/onlyA.ledger.md` and `<B>/docs/metrics/workflow.jsonl` belonging to B. The SAME command run from the two directories:

```
cd <A> && agent-scaffold next --source docs/plans/onlyA.plan.toml
  -> metrics: 1 records ; "PROJECT-A-ONLYA-LEDGER: do the A thing next."   exit 0
cd <B> && agent-scaffold next --source docs/plans/onlyA.plan.toml
  -> metrics: 3 records ; "PROJECT-B-ONLYA-LEDGER: do the B thing next."   exit 0
```

No `--metrics`, no `--ledger-fragment`, one wrong working directory. I record honestly that this variant is the weaker half of the finding: in the wrong-cwd run the derived defaults are lexically beside the anchor path as typed, so a defender can argue the tool answered about the anchor it was given. The explicit-flag repro above has no such defence, because the caller named alpha and got beta.

WHY IT MATTERS. This is the same payload as ROUND 1's ADV-1, at the same severity, reachable in the same surface, and the fix pass exists to close it. The specification's own framing (line 127) is that `next`'s false instruction is consumed by an AGENT that acts on it; here the agent is handed another project's private resume anchor with a `null` reason beside it, meaning "this is yours". The trigger is a typo, a stale path, a renamed plan, or a wrong working directory, which is the population most likely to occur in practice, and it is precisely the population where an operator has ALREADY made a mistake and most needs the tool not to compound it. It is also silent: `next` prints `source: no plan source` but that line was already the normal output for the Markdown-primary configuration this increment supports, so it does not distinguish "no plan was read because none was asked for" from "no plan was read because the one you asked for is not there".

The increment's own acceptance test for this surface, `a_surface_that_reads_no_plan_is_supplied_a_root` (`tests/unsafe_pairings_are_refused_and_omitted.rs:578`), only ever writes an anchor that EXISTS, so nothing in the suite pins the empty-vector case in either direction.

A NOTE ON SCOPE, not a request. `run_resume` has the identical hole through the same `resume_roots`, so this is not a `next`-versus-`status --resume` divergence like ADV-1 was; all three surfaces are vacuous together. That makes it a gap in the increment's guard rather than an inconsistency between its surfaces, and it means a fix belongs in one place. Both obvious shapes (treat an empty vector as "refuse", or supply a root from the lexical anchor when canonicalisation fails) change behaviour for the no-anchor-at-all case that README documents as keeping current-directory-relative paths, so this is a decision, not a mechanical patch.

## R2A-2: a Markdown-primary `--source` outside any `docs/plans` with no `--plan` now omits its own project's explicitly named log and ledger

SEVERITY: low.

CLAIM. The fix pass extends containment to a configuration that previously had none, and in doing so creates a new false positive: ONE project whose plan source lives outside `docs/plans` (for example in `notes/`), invoked with no `--plan` and an explicit `--metrics` or `--ledger-fragment` naming that same project's own files, now has that half omitted at exit 0. The root derived from the source is the source's own directory (`project_root_of_source`'s fallback), and the project's real `docs/` tree is not under it. HEAD~1 read both files; HEAD omits both.

REPRODUCTION. Self-contained. `bash repro2.sh <path-to-agent-scaffold> <scratch dir>`.

```sh
set -eu
BIN="$1"; R="$2"
rm -rf "$R"; mkdir -p "$R/proj/notes" "$R/proj/docs/metrics" "$R/proj/docs/plans"

# ONE project. Its Markdown-primary plan source lives in `notes/`, not in `docs/plans`.
cat > "$R/proj/notes/n.plan.toml" <<'TOML'
[meta]
title = "n"
primary = "markdown"
TOML

# The project's own round log and its own ledger, both named EXPLICITLY on the command line.
cat > "$R/proj/docs/metrics/workflow.jsonl" <<'JSONL'
{"type":"round","task":"n","artifact":"n1","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk"}
JSONL
cat > "$R/proj/docs/plans/n.ledger.md" <<'LED'
## RESUME STATE

THIS PROJECT'S OWN RESUME STATE.
LED

cd "$R/proj"
"$BIN" next --source notes/n.plan.toml --metrics docs/metrics/workflow.jsonl || true
"$BIN" next --source notes/n.plan.toml --ledger-fragment docs/plans/n.ledger.md || true
"$BIN" status --source notes/n.plan.toml --metrics docs/metrics/workflow.jsonl || true
```

OBSERVED at HEAD~1 (`<R>` abbreviated): `metrics: 1 records`; the ledger echoed as `THIS PROJECT'S OWN RESUME STATE.`; `status` `metrics: 1 records`. All exit 0.

OBSERVED at HEAD `6bf5280`:

```
metrics: unavailable, the round log docs/metrics/workflow.jsonl is not under the plan's project root <R>/proj/notes, so its records cannot be paired with this plan
the ledger docs/plans/n.ledger.md is not under the plan's project root <R>/proj/notes; nothing to resume
plan: not provided
metrics: unavailable, the round log docs/metrics/workflow.jsonl is not under the plan's project root <R>/proj/notes, so its records cannot be paired with this plan
```

All exit 0.

WHY IT MATTERS, AND WHY IT IS LOW. It is a silent omission of correct data for a single legitimate project, which is the failure mode the accepted-cost list exists to bound, and it is a population NONE of the four listed costs covers: cost (iii) is a `--plan` outside `docs/plans` paired with a Markdown-primary `--source` INSIDE one, and here the `--source` itself is outside and there is no `--plan` at all. That configuration had no containment rule of any kind before this commit, so it is not covered by the earlier increment's acceptance record either. Against that, three things pull the severity down and the triager may reasonably close it: the root cause is exactly the one the specification names ("COSTS (iii) AND (iv) SHARE ONE ROOT CAUSE, `src/main.rs:project_root_of_source`'s fallback ... treating it ONCE IS QUEUED TO THE SAME STEP rather than accumulating a fresh accepted cost on every new surface"), which reads as anticipating new surfaces inheriting it; the note names the derived root, so the user is told what happened rather than left guessing; and the behaviour IS derivable from the README's new sentence that in the no-plan-read case "the roots come from the `--source` and `--plan` themselves". I file it because it is a measured behaviour change introduced by THIS commit that neither the four costs, the CHANGELOG, nor the acceptance checks name, and because the parallel documented layout is NOT affected and the difference is not obvious: a source at the project ROOT with no `docs/plans` anywhere (`--source n.plan.toml` beside `docs/metrics/`) still works at HEAD, measured, because the fallback root is then the project root itself.

## THE ATTACKS THAT FAILED

Everything below was built and run and did NOT break the fix. This section is the coverage record.

FALSE NEGATIVES ON THE NEW ROOT-SUPPLY POLICY, all against a two-project disjoint fixture (A and B), all correctly refused or omitted at HEAD.

- A Markdown-primary `--source` in A with no `--plan`, an explicit `--ledger-fragment` and `--metrics` in B: both refused, and `status --resume` on the same inputs prints the same note. This is ROUND 1's ADV-1 and it is genuinely closed.
- A Markdown-primary `--source` in A beside an EXISTING `--plan` in B (the divergent pairing): `next` roots on B, refuses A's default log with `metrics-not-this-project`, prints no `ACTIVE LOOP` block, and `validate --workflow` refuses at exit 1.
- An anchor that EXISTS but is NOT a plan (a plain text file at `<A>/notes.txt`, which produces the `did not parse` stderr note): still supplies the root `<A>`, and B's log and ledger are both refused. An unparseable source is safe; only a MISSING one is not.
- An anchor that is a DIRECTORY: `Error: Os { code: 21, kind: IsADirectory }` at exit 1 on both builds, so it never reaches the guard.
- `..` escaping the root through an EXISTING intermediate (`<A>/docs/../../B/docs/metrics/workflow.jsonl`): canonicalised by `resolve_for_containment` and refused.
- `..` escaping the root through a MISSING intermediate (`<A>/ghost/../../B/docs/metrics/workflow.jsonl`), the classic bypass: the path is re-appended literally and DOES pass containment, but the file is unopenable for the same reason and the run reports `metrics: no log found`. The doc comment's claim that "no readable file hides behind one" holds as written; I could not construct a counterexample, because any path all of whose components exist canonicalises whole.
- A sibling project whose directory name has the root's name as a STRING prefix (`r2adv-A` versus `r2adv-A2`): correctly refused, because `Path::starts_with` is component-wise.
- Two anchors agreeing through DIFFERENT spellings: absolute source with relative plan, relative source with absolute metrics, `..`-detoured spellings on both sides, and one anchor reached through a project-level SYMLINK while the other uses the real path. None produced a false positive on `next`, `status` or `status --resume`; canonicalisation collapses the spellings before the roots are compared.
- Whether `containment_roots` can ever hold TWO roots, which is where a two-anchor intersection false positive would live. It cannot: in the `toml_primary == false` branch `checked_plan_root` IS `canonical_project_root(plan)`, so any `--plan` that yields a root prevents the fallback from being taken at all, and a `--plan` that yields no root contributes nothing to `resume_roots` either. The fallback therefore yields at most the `--source`'s root. Confirmed by construction (the divergent pairing above accepts a ledger in B under the single root B, while `status --resume`, which really does intersect two roots, refuses the same ledger on the same inputs).

THE `validate --workflow` ASYMMETRY, tested as a claim rather than accepted as an argument. Every configuration in which no plan is read exits 1 on the `(None, None, _)` arm and never runs the join:

- `validate --workflow --source <Markdown-primary in A> --metrics <B log>`, no `--plan`: `--workflow requested but no plan source resolved`, exit 1.
- The same with a TYPO'D `--source` (file absent): the `no source plan at ...` note, then the same problem, exit 1. With a MALFORMED foreign log it additionally reports that log's schema errors, which is the `--metrics` surface doing its own job, and still exits 1 without joining.
- `validate --workflow --metrics <B log>` with no anchors at all: exit 1.
- `validate --workflow --plan <nonexistent> --metrics <B log>`: exit 1.
- Control, a well-formed same-project run: `workflow invariants hold`, exit 0. Control, the divergent pairing: the refusal message naming plan, log, root and remedy, exit 1.
- I also tried to construct the one structural gap the claim depends on, a `--plan` for which `Path::exists()` is TRUE but `fs::canonicalize` FAILS (which would give `plan_contents = Some` with `checked_root = None` and slip past the `(None, None, _)` arm). Symlink loops, trailing slashes on a regular file, and paths through missing intermediates all fail BOTH calls, since `exists()` and `canonicalize` follow symlinks alike. I could not build one. The asymmetry is sound on the evidence I could produce.

PRECEDENCE AND CORRELATION, all correct.

- An explicit `--metrics` that is BOTH outside the root AND does not exist reports `"metrics_absent_reason": "log-not-this-project"`, not `log-absent`. Same for the ledger: `ledger-not-this-project`, not `ledger-absent`. Unpairable wins over absent on both halves.
- Where the metrics half is unpairable AND steps exist, `"no_active_loop_reason": "metrics-not-this-project"` with `"active_loop": null`.
- Where the metrics half is unpairable but the steps ALONE already leave no loop (a Markdown-primary source with no `--plan`), the reason is `no-plan-steps`. This is correct, not a correlation break: the specification's rule (line 233) is that the metrics cause is reported "WHEN the loop's absence is metrics-derived rather than step-derived", and `src/next.rs:687`'s `unpairable_log && !steps_leave_no_loop(...)` implements exactly that.
- I could not construct an `ACTIVE LOOP` block derived from an unpairable or unrooted log by any route, including the empty-root hole in R2A-1. Steps only exist when a plan was read, and a plan that was read always yields a root, so the empty-root case always lands on `no plan steps found`. R2A-1's damage is confined to the record count and the resume echo; it cannot manufacture a `mark the step complete` instruction.

ONE DEGENERATE LAYOUT, REPRODUCED BUT NOT FILED. A plan source at `/docs/plans/x.plan.toml` (the FILESYSTEM root) derives the project root `/`, under which every path on the machine is contained, so containment is vacuous and another project's ledger is echoed. I built it (the sandbox permitted `mkdir /docs/plans`; the directory was removed afterwards) and it reproduces. I do not file it: if a project's root really is `/`, then everything really is inside it, and the guard is answering its own question correctly. It is recorded only so the next reader does not spend the time.

NOT RAISED, per the brief: the in-root bound (with the discriminating control run and reported above), ADV-2's `ledger:` context slot, the four accepted costs, project identity, line length, prose wrapping, and the specification's own wording.
