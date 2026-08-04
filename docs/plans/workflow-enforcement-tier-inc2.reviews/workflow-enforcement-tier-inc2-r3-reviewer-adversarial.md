# Review findings: `workflow-enforcement-tier-inc2`, ROUND 3, ADVERSARIAL CONSTRUCTION lens

LENS. Adversarial construction. Nothing below was concluded by reading the diff. Every claim was produced by building a project layout on disk, running a built binary against it, and recording the streams and the exit code. Every finding carries a differential against a second binary so that "this commit changed it" is a measurement rather than an inference.

ARTIFACT. Worktree `.claude/worktrees/r3-adversarial`, HEAD `a7e05c3` ("fix: root containment on an anchor that does not exist"), `main` at `b4c0688`. The round 2 fix alone is `git diff HEAD~1..HEAD` (`HEAD~1` is `b957d19`); the whole increment is `git diff main..HEAD`. Specification: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`.

BINARIES. `cargo build --release` at HEAD, and a second build of the tree exported from `HEAD~1` (`git archive HEAD~1 | tar -x -C <scratch>/r3adv-prev`, then `cargo build --release` there). Both are used below; where a transcript says OLD it is the `HEAD~1` binary and NEW is HEAD.

SUITE STATE AT HEAD. `cargo test --release`: 416 passed, 0 failed, across 9 binaries (378 + 5 + 1 + 1 + 9 + 3 + 16 + 1 + 2). Every finding below reproduces against that green suite.

WHAT I ATTACKED. The third rewrite of the root-supply policy, `resume_roots` (`src/main.rs:1509`), reached by `run_status` and `run_next` through `containment_roots` (`src/main.rs:1379`) and by `run_resume` directly (`src/main.rs:1531`); the partial resolution `resolve_for_containment` (`src/main.rs:1397`) now applied to an ANCHOR rather than only to an artifact; and the stderr note `note_missing_anchors` (`src/main.rs:1110`). Four attack directions were run: pathological anchors, the two-root intersection in both directions, the three surfaces' verdicts diffed against each other on identical inputs, and the note.

THE HEADLINE. The round 2 fix closed the leak it was written for; I confirmed that by running round 2's own reproduction (section "attacks that failed", A-series). It is a STRICT TIGHTENING by construction, so it cannot introduce a new leak, and I could not build one. What it introduced instead is on the other side of the predicate: a `--plan` PATH THAT DOES NOT EXIST now manufactures a containment root that OVERRIDES the root derived from a `--source` that does exist, so a single project's own log and its own ledger are withheld from `next` and `status` with machine reasons that positively assert they belong to a different project, and `status --resume` refuses a ledger that `next` and `validate --workflow` accept on the same command line.

---

## R3A-1: a `--plan` that does not exist supplies a containment root that vetoes the real one, so a project's own log and ledger are withheld with `log-not-this-project` and `ledger-not-this-project`

SEVERITY: medium.

CLAIM. `resume_roots` now derives a root from EVERY supplied anchor including one with nothing behind it, and containment requires the artifact to be under EVERY root, so naming a `--plan` that is not on disk is strictly worse than naming no `--plan` at all: it converts a correct single-project run into a full omission at exit 0, with `metrics_absent_reason: "log-not-this-project"` and `resume_state_absent_reason: "ledger-not-this-project"` asserting that the project's OWN artifacts are not its own. The same mechanism makes `status --resume` disagree with `next` and with `validate --workflow` on identical inputs.

WHY IT IS REACHABLE. `containment_roots` falls through to `resume_roots` whenever `checked_plan_root` is `None`, which now includes "a `--plan` was supplied and is not there". Before `a7e05c3` such an anchor was dropped by `resume_roots`'s `filter_map` and contributed nothing; after it, `project_root_of_source(&resolve_for_containment(anchor))` returns a root for it unconditionally. Where the missing `--plan` sits in another project's `docs/plans`, the derived root is that other project's root by the ordinary convention walk, not by `project_root_of_source`'s parent-directory fallback, so this is NOT the root cause the specification queues at line 271 and NOT any of the four accepted costs (all four are same-project layouts; costs (iii) and (iv) additionally require a `--plan` that EXISTS and is READ, which is what makes it "the checked plan").

REPRODUCTION. Self-contained; builds its own two-project fixture, no `scaffold` required. Save as `repro.sh` and run `bash repro.sh <path-to-agent-scaffold> <a scratch dir outside any repo>`.

```sh
set -eu
BIN="$1"; R="$2"
rm -rf "$R"; mkdir -p "$R/alpha/docs/plans" "$R/alpha/docs/metrics" "$R/beta/docs/plans"

cat > "$R/alpha/docs/plans/p.plan.toml" <<'TOML'
[meta]
title = "alpha plan"
primary = "toml"

[[step]]
slug = "step-one"
title = "Alpha step one"
status = "in-progress"
order = 1
TOML
cat > "$R/alpha/docs/plans/m.plan.toml" <<'TOML'
[meta]
title = "alpha markdown-primary"
primary = "markdown"
TOML
cat > "$R/alpha/docs/metrics/workflow.jsonl" <<'JSONL'
{"type":"round","task":"p","artifact":"step-one","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"risky"}
JSONL
cat > "$R/alpha/docs/plans/p.ledger.md" <<'LED'
# alpha ledger

## RESUME STATE

ALPHA PRIVATE RESUME STATE.

## NEXT
LED
cp "$R/alpha/docs/plans/p.ledger.md" "$R/alpha/docs/plans/m.ledger.md"
: > "$R/beta/docs/plans/.keep"

MISSING="$R/beta/docs/plans/NOSUCH.md"

echo "=== A. CONTROL: no --plan at all. ==="
"$BIN" next --source "$R/alpha/docs/plans/m.plan.toml" || true; echo "exit=$?"

echo "=== B. ATTACK: add a --plan naming a file that DOES NOT EXIST. ==="
"$BIN" next --source "$R/alpha/docs/plans/m.plan.toml" --plan "$MISSING" || true; echo "exit=$?"

echo "=== C. The machine surface for B. ==="
"$BIN" next --json --source "$R/alpha/docs/plans/m.plan.toml" --plan "$MISSING" || true; echo "exit=$?"

echo "=== D. Same attack, alpha's OWN artifacts named EXPLICITLY. ==="
"$BIN" next --json --source "$R/alpha/docs/plans/m.plan.toml" --plan "$MISSING" \
    --metrics "$R/alpha/docs/metrics/workflow.jsonl" \
    --ledger-fragment "$R/alpha/docs/plans/p.ledger.md" || true; echo "exit=$?"

echo "=== E. THREE-WAY SPLIT, TOML-primary source, identical inputs. ==="
"$BIN" validate --workflow --source "$R/alpha/docs/plans/p.plan.toml" --plan "$MISSING" || true; echo "exit=$?"
"$BIN" next --source "$R/alpha/docs/plans/p.plan.toml" --plan "$MISSING" 2>/dev/null | tail -4 || true
"$BIN" status --resume --source "$R/alpha/docs/plans/p.plan.toml" --plan "$MISSING" || true; echo "exit=$?"
```

OBSERVED at HEAD `a7e05c3`, verbatim, paths abbreviated to `<R>`.

Block A, the CONTROL, which is the correct answer:

```
task: m
source: no plan source
metrics: 1 records

no active review loop (no plan steps found)

RESUME STATE (verbatim from the ledger):
## RESUME STATE

ALPHA PRIVATE RESUME STATE.
exit=0
```

Block B, the ATTACK. The ONLY difference is a `--plan` naming a path that is not on disk. No file was created, deleted or moved:

```
note: --plan <R>/beta/docs/plans/NOSUCH.md does not exist
task: m
source: no plan source
metrics: unavailable, the round log <R>/alpha/docs/metrics/workflow.jsonl is not under the plan's project root <R>/beta, so its records cannot be paired with this plan

no active review loop (no plan steps found)

the ledger <R>/alpha/docs/plans/m.ledger.md is not under the plan's project root <R>/beta; nothing to resume
exit=0
```

Block C, the machine surface, which is the part that matters most because an agent consumes it:

```
{
  "task": "m",
  "source": "no plan source",
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project",
  "active_loop": null,
  "resume_state": null,
  "resume_state_absent_reason": "ledger-not-this-project",
  "no_active_loop_reason": "no-plan-steps"
}
exit=0
```

Block D is byte-identical to block C, so naming alpha's own log and alpha's own ledger EXPLICITLY does not change the verdict.

Block E, the three-way split, every line of it on the same two anchors:

```
--- validate --workflow
no plan at <R>/beta/docs/plans/NOSUCH.md; nothing to validate
<R>/alpha/docs/metrics/workflow.jsonl: 1 records, valid
<R>/alpha/docs/plans/p.plan.toml: 1 steps, 0 questions, valid
<R>/alpha/docs/plans/p.plan.toml vs <R>/alpha/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
--- next (tail)
RESUME STATE (verbatim from the ledger):
## RESUME STATE

ALPHA PRIVATE RESUME STATE.
--- status --resume
note: --plan <R>/beta/docs/plans/NOSUCH.md does not exist
the ledger <R>/alpha/docs/plans/p.ledger.md is not under the plan's project root <R>/beta; nothing to resume
exit=0
```

`validate --workflow` accepts alpha's log and greens. `next` echoes alpha's ledger with `resume_state_absent_reason: null`. `status --resume` refuses the SAME ledger. Two surfaces say the pairing is safe and the third says it is not.

THE DIFFERENTIAL, which is what establishes that `a7e05c3` caused it. The identical script against the `HEAD~1` binary:

```
=== B. ATTACK ===                        === C. machine surface ===
task: m                                  "metrics": { "records": 1 },
source: no plan source                   "metrics_absent_reason": null,
metrics: 1 records                       "resume_state": "## RESUME STATE\n\nALPHA PRIVATE RESUME STATE.",
RESUME STATE (verbatim from the ledger): "resume_state_absent_reason": null,
## RESUME STATE
ALPHA PRIVATE RESUME STATE.              === E. status --resume ===
exit=0                                   ## RESUME STATE
                                         ALPHA PRIVATE RESUME STATE.
```

At `HEAD~1` all three surfaces read alpha's own artifacts. Block A (the control) is byte-identical between the two binaries, so the change is confined to the attack.

THE SAME-PROJECT VARIANT, for the population. The `--plan` need not be in another project. `--source <R>/alpha/docs/plans/m.plan.toml --plan <R>/alpha/notes/missing.md` (one project, a `notes/` directory that exists, a `--plan` file that does not) produces the same omission with the root `<R>/alpha/notes`, and OLD reads both artifacts. That variant DOES rest on `project_root_of_source`'s parent-directory fallback and a triager may reasonably rule it absorbed by specification line 271; the beta variant above does not, because `<R>/beta/docs/plans/NOSUCH.md` matches the `docs/plans` convention walk and yields `<R>/beta` without the fallback ever running. I give both so the ruling can be made on the discriminating one.

MEASURED SURFACE-AGREEMENT TABLE, nine configurations, HEAD, one fixture. `next-log` is 1 when the record count is printed, `next-ledger` and `resume-ledger` are 1 when a `## RESUME STATE` body is echoed:

| Configuration | next-log | next-ledger | resume-ledger | validate exit |
| --- | --- | --- | --- | --- |
| T1 TOML-primary alpha `--source` alone | 1 | 1 | 1 | 0 |
| T2 T1 plus a NONEXISTENT beta `--plan` | 1 | 1 | 0 | 0 |
| T3 T1 plus an EXISTING beta `--plan` | 1 | 1 | 0 | 0 |
| T4 Markdown-primary alpha `--source` alone | 1 | 1 | 1 | 1 |
| T5 T4 plus a NONEXISTENT beta `--plan` | 0 | 0 | 0 | 1 |
| T6 T4 plus an EXISTING beta `--plan` | 0 | 0 | 0 | 1 |
| T7 NONEXISTENT alpha `--source` plus an EXISTING alpha `--plan` | 1 | 0 | 0 | 1 |
| T8 T1 plus a NONEXISTENT `alpha/notes` `--plan` | 1 | 1 | 0 | 0 |
| T9 T4 plus a NONEXISTENT `alpha/notes` `--plan` | 0 | 0 | 0 | 1 |

T2, T5, T8 and T9 all moved at `a7e05c3` (each was 1/1/1 at `HEAD~1` for the first three columns). T3 and T6 did not move; T3's `next`-versus-`resume` split is present at `HEAD~1` too and is the accepted cost (iv) shape on a wider population, which I do not file (see "attacks that failed", D-series). T7's `next-ledger=0` is a `default_ledger_path` effect, not containment: the ledger name is derived from the missing `--source` (`NOSUCH.ledger.md`), so the reason is `ledger-absent` and it is correct.

WHY IT MATTERS. Three things, in the order I weigh them.

- THE MACHINE REASON IS A POSITIVE FALSE ASSERTION, not a silence. `"metrics_absent_reason": "log-not-this-project"` and `"resume_state_absent_reason": "ledger-not-this-project"` are the vocabulary `Q-55-jsonreason` added so an omission explains itself to an agent; here they explain it wrongly, about a log and a ledger the tool would have read a commit earlier. The specification's whole ground for the omit response (line 240 and following) is that "a log that does not belong to this plan is exactly a part that is not available for this projection". This log DOES belong to this plan.
- THE TRIGGER IS A TYPO OR A STALE PATH, the same population round 2's finding turned on, and it is the population where the operator has already made one mistake. Naming a `--plan` that is not there is now strictly worse than naming none, which inverts what an operator would predict. The `note:` line does tell them the path is missing, but it does not say that the missing path has taken over the containment root, and `note_missing_anchors`'s own doc comment says it "names no derived root deliberately".
- THE HUMAN LINE NAMES A PLAN THAT DOES NOT EXIST AS "THE PLAN". In block B the output says `source: no plan source` and `"plan": null` on the same run whose containment note reads "not under the plan's project root `<R>/beta`". No plan was read, and the file the root is derived from is not on disk.

NOT COVERED BY THE SUITE, checked rather than assumed. `tests/unsafe_pairings_are_refused_and_omitted.rs` exercises a missing anchor only through `--source` (`:680` builds the missing `q.plan.toml`, `:707` asserts the note). Every `--plan` the file passes is WRITTEN first: `beta_plan` at `:262`, `:829` and (by the same helper) `:1034`, and `notes_plan` at `:1151`, which is the accepted cost (iii) and (iv) pin. No test passes a `--plan` that does not exist, so nothing pins either direction of this behaviour, and the accepted-cost pin at `:1144` deliberately uses an EXISTING `notes/p.md`, which is the discrimination R3A-1 turns on.

A NOTE ON SCOPE, not a request. The fix site is one function (`resume_roots`, `src/main.rs:1509`) and the question it raises is a policy one that `Q-55-resumepairing` does not answer: that decision conditions on a `--source` and a `--plan` that BOTH EXIST, and the round 2 fix extended the rule to anchors that do not without extending the decision. Whether a nonexistent anchor should CONTRIBUTE a root (as now), be IGNORED when another anchor yields one, or be ignored only when it is the non-primary anchor, is a human call, not an implementer's.

---

## R3A-2: the missing-anchor note reports "does not exist" for an anchor that does exist but cannot be stat'd

SEVERITY: low.

CLAIM. `note_missing_anchors` (`src/main.rs:1110`) tests `!path.exists()`, and `Path::exists` returns `false` both for "not there" and for "there, but the metadata could not be read". The note therefore prints a statement that is false, on the one line the fix pass added specifically so the operator is told the truth about their anchor. `Path::try_exists` is the standard-library call that separates the two.

REPRODUCTION.

```sh
set -eu
BIN="$1"; F="$2"
rm -rf "$F"; mkdir -p "$F/proj/docs/plans" "$F/proj/docs/metrics"
printf '[meta]\ntitle = "x"\nprimary = "markdown"\n' > "$F/proj/docs/plans/p.plan.toml"
chmod 000 "$F/proj/docs/plans"
"$BIN" next --source "$F/proj/docs/plans/p.plan.toml" || true
chmod 755 "$F/proj/docs/plans"
ls "$F/proj/docs/plans"
```

OBSERVED at HEAD (`<R>` abbreviated), exit 0:

```
note: --source <R>/proj/docs/plans/p.plan.toml does not exist
task: p
source: no plan source
metrics: no log found
```

and then `ls` prints `p.plan.toml`, so the file the note says does not exist is there.

WHY IT MATTERS. It is small, and the projection would degrade in this configuration whatever the note said. It is filed because the note is the entire "Fail loudly" half of the round 2 fix (the triage's recommendation was Option A "with the anchor-does-not-exist condition additionally surfaced ... so Fail loudly is not simply traded away"), and a loud line that says the wrong thing about the filesystem is worth one call site. The fix is `Path::try_exists` with the error arm phrased separately, and it is a `low` because the population is a directory the caller cannot read.

---

## R3A-3: a containment root can be a literal `..` path, which refuses the project's own log and prints a root that is not a directory

SEVERITY: low.

CLAIM. `resolve_for_containment` re-appends a `..` LITERALLY when a directory above it is missing. Its doc comment argues this is sound, and for an ARTIFACT it is (I could not falsify that; see "attacks that failed", C-series). Applied to an ANCHOR, which `a7e05c3` newly does, the argument does not transfer: the anchor is never opened, so the literal `..` survives into `project_root_of_source` and out into the printed root, where it refuses everything canonical including the project's own log and names a path that cannot be a project root.

REPRODUCTION.

```sh
set -eu
BIN="$1"; F="$2"
rm -rf "$F"; mkdir -p "$F/proj/docs/metrics"
echo '{"type":"round","task":"q","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk"}' \
  > "$F/proj/docs/metrics/workflow.jsonl"
echo "--- ATTACK: one missing directory and a .. in the anchor"
"$BIN" next --source "$F/proj/ghost/../q.plan.toml" --metrics "$F/proj/docs/metrics/workflow.jsonl" || true
echo "--- CONTROL: the same anchor with the ghost/.. removed"
"$BIN" next --source "$F/proj/q.plan.toml" --metrics "$F/proj/docs/metrics/workflow.jsonl" || true
```

OBSERVED at HEAD (`<R>` abbreviated), both exit 0:

```
--- ATTACK
note: --source <R>/proj/ghost/../q.plan.toml does not exist
metrics: unavailable, the round log <R>/proj/docs/metrics/workflow.jsonl is not under the plan's project root <R>/proj/ghost/.., so its records cannot be paired with this plan
--- CONTROL
note: --source <R>/proj/q.plan.toml does not exist
metrics: 1 records
```

`<R>/proj/ghost/../q.plan.toml` and `<R>/proj/q.plan.toml` name the same file, and the tool gives them opposite verdicts on the same log.

WHY IT IS ONLY `low`. The direction is safe: at `HEAD~1` this same command printed `metrics: 3 records` off another project's log (I measured it), so the fix moved this case from a leak to a refusal. What is left is a false positive with a root string that is not a path a user can act on, in a population (a `..` through a directory that does not exist) that is small. I file it because the doc comment at `src/main.rs:1397-1404` justifies the literal `..` with an argument about READABILITY that only holds for artifacts, and that comment is now the guarantee for two different uses of the function.

---

## ATTACKS THAT FAILED

Everything below was built and run and did NOT break the fix. This section is the coverage record, and the negative results are scoped to the dimensions the fixtures actually varied.

A-SERIES, THE ROUND 2 DEFECT ITSELF. It is closed. `next --source <alpha>/docs/plans/q.plan.toml --metrics <beta>/docs/metrics/workflow.jsonl --ledger-fragment <beta>/docs/plans/b.ledger.md` with `q.plan.toml` absent now prints the containment note on both halves and echoes nothing of beta, where round 2 measured `metrics: 3 records` and beta's block verbatim. The `--json` surface reports `log-not-this-project` and `ledger-not-this-project` rather than `null`. `status --resume` on the same anchors refuses.

B-SERIES, CAN THE INTERSECTION ADMIT A FOREIGN ARTIFACT? NO, AND THE REASON IS STRUCTURAL. Containment over N roots requires the artifact to be under all of them, so it can only admit what the NARROWEST root already admits; adding a root can never widen the accepted set. `resume_roots` at HEAD is a strict SUPERSET of `resume_roots` at `HEAD~1` for every input (an anchor that exists resolves identically under both spellings, since `resolve_for_containment` of a wholly existing path is `fs::canonicalize` of it; an anchor that does not exist was dropped and is now kept). `containment_roots` delegates to it on the same condition in both builds. So `a7e05c3` cannot have introduced a leak, and I did not find one. I tried anyway with two-root pairings where one root is an ancestor of the other (`--source` in `<R>/alpha/docs/plans`, `--plan` a nonexistent `<R>/NOSUCH.md`, roots `<R>/alpha` and `<R>`): beta is refused, because the intersection is the narrower root.

C-SERIES, PATHOLOGICAL ANCHORS AND ARTIFACTS. Each was run against a two-project disjoint fixture with an explicit foreign `--metrics` and `--ledger-fragment`.

- An empty anchor (`--source ""`): rejected by clap before any code under review runs (`error: a value is required for '--source <SOURCE>'`). Not a route.
- An anchor whose every component is missing but which sits inside a project (`<alpha>/docs/plans/q.plan.toml`): root `<alpha>`, both foreign artifacts refused. This is the A-series case.
- A `/dev/null` anchor: parses as empty and Markdown-primary, root `/dev`, both foreign artifacts refused.
- A FIFO anchor: `fs::read_to_string` blocks forever and the process hangs. IDENTICAL at `HEAD~1`, and identical for a FIFO passed as `--metrics`, so it is a property of reading a named file rather than anything this increment authored. Not filed; recorded so the next reader does not spend the time.
- A dangling-symlink anchor: treated as missing (the note fires), root correctly `<alpha>`, foreign artifacts refused.
- A trailing slash on an existing plan file (`p.plan.toml/`): the note fires (arguable but defensible, `exists()` is false for it), the root still resolves to `<alpha>`, foreign artifacts refused. The only cost is that the TOML source is not read.
- An anchor that is a DIRECTORY: `status --resume` accepts it (it reads no plan) and derives the root from the directory's PARENT, so `--plan <alpha>/docs/plans` yields root `<alpha>/docs` and `--plan .` yields the parent of the working directory. `next` and `status` fail earlier with `IsADirectory` at exit 1. I did not file this: every artifact it then admits is inside the derived root, which is the in-root bound (discriminating control below), and the input is a directory where a file is documented.
- The classic `..` bypass on the ARTIFACT, through a missing intermediate (`<alpha>/ghostdir/../../beta/docs/metrics/workflow.jsonl`): the literal `..` survives, containment passes, and the run reports `metrics: no log found` because the path cannot be opened for the same reason. I attacked the doc comment's soundness argument three further ways and could not falsify it: through a dangling symlink (`no log found`), through a symlink loop (`no log found`), and through a directory with mode `000`. The last is the interesting one and it goes the SAFE way: `fs::canonicalize` resolves `..` lexically after symlink resolution, so it succeeds through a non-traversable directory and canonicalises the whole path, which makes the guard TIGHTER than `open`, not looser. A mode `111` (traversable, unreadable) directory behaves the same. The claim "no readable file hides behind one" holds on everything I could build.
- A sibling project whose name is a string prefix of the root's: refused, `Path::starts_with` is component-wise. Re-confirmed rather than taken from round 2.

D-SERIES, THE THREE SURFACES DIFFED AGAINST EACH OTHER. I enumerated the nine configurations in R3A-1's table and diffed the verdicts. The result is a bounded one and it is worth stating as a property rather than a list: `run_resume` always tests against `resume_roots` (both anchors), while `run_status` and `run_next` test against `containment_roots`, which is `[checked_plan_root]` whenever a plan is read and `resume_roots` otherwise. `[checked_plan_root]` is always a SUBSET of `resume_roots` (the checked plan is one of the two anchors), so `status --resume` is always at least as strict as `next` and `status`. THEREFORE `status --resume` CAN NEVER ACCEPT WHAT `next` REFUSES, and every disagreement is of the form "next projects, resume refuses". That direction cannot leak. Round 1's ADV-1 was the opposite direction and it is structurally unreachable now.

Within that, T3 and T6 (an EXISTING `--plan` in another project beside a `--source` in this one) produce a `next`-accepts / `resume`-refuses split that is present at `HEAD~1` as well, so it is not this commit's. It is the accepted cost (iv) shape ("`status --resume` ... in EITHER `primary` spelling, so its population is WIDER than (iii)'s") on a population the specification describes as same-project. I do not file it: the specification records the divergence and its wider population explicitly, both policies are individually correct per `Q-55-endproperty` and `Q-55-resumepairing`, and the refusing surface is the safe one. T2 and T8, the same split caused by a `--plan` that does NOT exist, ARE new at `a7e05c3` and are filed as part of R3A-1.

E-SERIES, THE NOTE. Beyond R3A-2 I could not break it.

- It cannot contaminate `--json`. `next --json --source <missing>` writes the note to stderr and stdout alone parses: `jq -r '.task'` returns `q` at exit 0. Confirmed with the note firing.
- It cannot fire when nothing is missing. Every control run in this review with both anchors present printed no `note:` line, and the increment's own test asserts the negative at `tests/unsafe_pairings_are_refused_and_omitted.rs:780`.
- It fires once per missing anchor and both lines appear when both are missing, in flag order, which is what the loop reads.
- It fires on `status --resume` as well as on `status` and `next`, because `run_status` calls it before the `--resume` split.
- One inaccuracy I judged too small to file on its own and record here instead: the note's doc comment says "the containment rule roots them on the anchor whether or not it exists", but with a TOML-primary `--source` a missing `--plan` does NOT contribute a root to `next` or `status` (only to `status --resume`), so on those two surfaces the note can fire for an anchor that changed nothing they printed. It is a doc-comment scope claim, the same class as round 1's TRI-1.

F-SERIES, PRECEDENCE AND CORRELATION. All correct, and one of them is correct for a structural reason worth recording.

- Unpairable beats absent on both halves, including in the NEW two-root regime: an explicit `--metrics` that is both outside every root and absent reports `log-not-this-project`, and the ledger reports `ledger-not-this-project`.
- I could not produce `no_active_loop_reason: "metrics-not-this-project"` from the new two-root regime, and it is unreachable rather than merely unbuilt: `containment_roots` reaches `resume_roots` only when no plan was read, and a run with no plan has no steps, so the loop's absence is always step-derived and the reason is always `no-plan-steps`. That matches specification line 233 (the metrics cause is reported only when the loop's absence is metrics-derived). In R3A-1's block C it reads `no-plan-steps` beside `log-not-this-project`, which is right.
- I could not manufacture an `ACTIVE LOOP` block from an unpairable or wrongly-rooted log by any route, for the same reason.

G-SERIES, THE IN-ROOT BOUND, WITH THE DISCRIMINATING CONTROL RUN. Two constructions reproduce a foreign read and I attribute BOTH to the in-root bound, having run the control the brief requires rather than asserting it.

- An anchor directly under the filesystem root (`--source /q.plan.toml`, nothing created on disk) derives the project root `/`, under which every path on the machine is contained, so beta's log is counted and beta's block echoed at exit 0. Round 2 recorded the same class for `/docs/plans/x.plan.toml`; this variant needs no privileges and nothing on disk, which is the only new fact. The derived root CONTAINS the artifact, so it is the in-root bound; a disjoint arrangement does not exist, since nothing is disjoint from `/`.
- An anchor whose `..` climbs out of the project through EXISTING directories (`--source <G>/nested/alpha/docs/plans/../../../q.plan.toml`) resolves to `<G>/nested/q.plan.toml` and derives the root `<G>/nested`, which contains both alpha and beta, so beta leaks. I ran ONE fixture in TWO arrangements, changing only where beta sits:

```
NESTED   (beta a sibling of alpha under the derived root)
  -> metrics: 3 records ; BETA PRIVATE RESUME STATE. ; exit 0
DISJOINT (identical anchor shape, beta outside the derived root)
  -> metrics: unavailable, the round log <G>/disjointbeta/beta/docs/metrics/workflow.jsonl is not
     under the plan's project root <G>/nested2, so its records cannot be paired with this plan
  -> the ledger <G>/disjointbeta/beta/docs/plans/b.ledger.md is not under the plan's project root
     <G>/nested2; nothing to resume ; exit 0
```

  The disjoint case does NOT reproduce, so by the brief's own rule this IS the in-root bound and I do not file it. I record one adjacent fact for the queued work, measured rather than reasoned: in the nested arrangement, the SAME command with no explicit `--metrics` reads alpha's log (`metrics: 1 records`, `metrics_absent_reason: null`), because the LEXICAL derivation used by `resolve_metrics_path` returns `<G>/nested/alpha` (the `..` matches the `docs/plans` it climbs through, as `project_root_of_source`'s own comment describes), while the CANONICAL derivation used by containment returns `<G>/nested` and admits beta's log at 3 records on the same anchor. The guard's root is a strict ancestor of the resolution's root, so the guard does not police the path the same run resolves. I could not turn that divergence into a disjoint reproduction, because an artifact under the broader root and outside the narrower one is by construction nested.

DIMENSIONS THE FIXTURES VARIED. Which anchors are supplied (`--source` only, `--plan` only, both, neither); whether each anchor exists; whether the `--source` is TOML-primary, Markdown-primary, unparseable, a device, a FIFO, a directory, a dangling symlink or a symlink loop; whether the `--plan` is in the same project, in another project, or in a non-`docs/plans` directory of the same project; whether the artifact is defaulted or named explicitly; whether the artifact is inside the derived root, outside it, or reachable only through a `..`; absolute versus relative spellings on both anchors and both artifacts; the working directory (inside the project, inside a sibling project, inside `docs/plans`); directory permissions (`000`, `111`, `755`) on both an anchor's parent and an artifact's intermediate; and all four surfaces (`next`, `next --json`, `status`, `status --json`, `status --resume`, `validate --workflow`).

DIMENSIONS THEY DID NOT VARY, so no negative result above should be read past them. Symlinked layouts beyond a single dangling link and a single loop, in particular a symlinked `docs/plans` directory, which is accepted cost (ii) and which I deliberately did not attack. Nested projects, except in the one discriminating control in G-SERIES; every other fixture is two top-level siblings. Plans with more than one step, and steps in states other than `in-progress`, so nothing here exercises the loop projection's own arithmetic. Round logs longer than three records and logs with malformed lines. Concurrent modification of any file between the resolution and the read. Non-UTF-8 or unusually long path components. Multiple mount points, bind mounts and hard links, none of which the sandbox let me create. `audit`, which shares `derive_task` and the default paths but not the containment predicate. Windows and macOS path semantics; everything here is Linux.

NOT RAISED, per the brief: the in-root bound (with the discriminating control run and reported above), `ADV-2`, `R2A-2`, `R2C-2`, the stale owed-demonstration count, the four accepted costs, the `validate --workflow` asymmetry (I built no new construction against it and confirmed only that it accepts in R3A-1's block E for the ordinary reason that it roots on the TOML source it reads), project identity, line length, prose wrapping, and the specification's own wording.
