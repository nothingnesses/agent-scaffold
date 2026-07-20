# Triage: task-entry-regrounding-inc2 (Part B, per-step provenance)

Triager pass, independent of reviewers/implementer/orchestrator. Diff `main` (e48cb75)
.. `impl/ter-inc2` (fd42f21). Verdicts: VALID (fix now), VALID-BUT-DEFER, INVALID.

## Verdict table

| id    | verdict          | actionable this round? | one-line fix |
|-------|------------------|------------------------|--------------|
| I2-1  | VALID            | yes                    | Correct the false "binds to the wrong table" ordering rationale (source.rs:126-132, :151-158) and the test docstring (source.rs:1273-1277) to state the true reason (grouping/readability; no round-trip ordering constraint applies), keep the round-trip test as a genuine round-trip check. |
| I2s-1 | VALID            | yes                    | Remove `commits = ["1e1d26f"]` from the `task-entry-regrounding` exemplar (agent-scaffold.plan.toml:720) and re-render agent-scaffold.md; decisions-only (`Q-53`) is the honest justification. |
| I2-2  | VALID-BUT-DEFER  | no                     | Inherited `is_safe_sidecar_ref("")` gap; fixing it correctly belongs in the shared rule, not a provenance-only divergence; defer to a follow-up. |

## I2-1 (medium, schema reviewer) -- VALID, actionable this round

Finding: the comments (source.rs:126-132 Step doc, :151-158 provenance field) assert
`provenance` MUST precede `increment`/`waiver` because a `[step.provenance]` sub-table
emitted after `[[step.increment]]` "would bind to the wrong table" and break the
round-trip, and the test `a_populated_provenance_parses_and_round_trips_alongside_increments_and_waivers`
(source.rs:1271-1303) docstring claims it "pins the ordering constraint".

Reproduction (independent, toml 0.8.23 per Cargo.lock, scratch project OUTSIDE the
worktree, no tracked files touched):

- Built a `Step`-shaped struct with `provenance` declared AFTER `increment` and `waiver`
  (the "wrong" order), serialized a step carrying decisions + findings + commits + an
  increment + a waiver, and re-parsed. Result: serializer emits `[step.provenance]` AFTER
  the `[[step.increment]]` / `[[step.waiver]]` blocks, and it re-parses to an EQUAL value
  (`reparse OK; equal=true`). The "correct" order also round-trips `equal=true`. CONFIRMED:
  the reviewer's `equal=true` under both orders is correct. `[step.provenance]` is a
  fully-qualified header that binds to the last `[[step]]` regardless of position; the
  "binds to the wrong table" mechanism does not occur.

- Went further and tested the reviewer's fallback hazard (a SCALAR field declared after an
  array-of-tables). toml 0.8.23 HOISTS the scalar above the `[[step.increment]]` header in
  its output (`note = "hello"` emitted before the increment block) and round-trips
  `equal=true`. So in this toml version declaration order is round-trip-neutral even for a
  scalar-after-table; the serializer reorders to emit values before table headers itself.
  Consequence: the reviewer's suggested alternative ("add a test that fails on a scalar
  declared after a table") would NOT fail either, so that option is not viable. The only
  correct fix here is honesty, not a new ordering guard.

Ruling: the ordering guard the comments/test claim does not exist for toml 0.8.23; the
test passes under both field orders and does not pin what it says. No runtime/SSOT risk
(the field IS in a valid position), but the emphatic false invariant and the overclaiming
test docstring mislead a future maintainer, which Principle 11 (test honesty) treats as a
defect. The increment newly introduced this false rationale (main's Step comment was the
modest two-line "scalar keys come before array-of-table keys ... stays valid"; this
increment expanded it into the incorrect sub-table-binding claim), so it is in-scope to
correct, and the fix is cheap.

Smallest correct fix:
1. source.rs:126-132 (Step doc) and :151-158 (provenance field): drop the "would bind to
   the wrong table" claim. State the real reason for the placement: it groups provenance
   with the value-ish fields for readability, and there is no round-trip ordering
   constraint on it (the toml serializer emits values and sub-tables in a valid order
   regardless of declaration order). Do NOT keep language asserting a correctness-bearing
   ordering invariant that does not hold.
2. source.rs:1273-1277 (test docstring): remove the "pins the ordering constraint" /
   "must land before ... for the round-trip to hold" wording. Reframe as what it actually
   is: a genuine round-trip test that a populated provenance survives serialize -> re-parse
   alongside increments and waivers. The test body is fine and worth keeping; only the
   docstring overclaims.

No field move is needed (placement is a valid readability choice); no new test is needed
(an ordering-sensitive guard is not achievable in this toml version).

## I2s-1 (low, render reviewer) -- VALID, actionable this round

Finding: `agent-scaffold.plan.toml:720` gives the `task-entry-regrounding` exemplar
`commits = ["1e1d26f"]`, but `1e1d26f` is inc1's own deliverable ("docs: add task-entry
re-grounding discipline to pack guidance (Q-53 part a)"; touches pack/AGENTS.md and the
orchestrator prompt), i.e. what the step PRODUCED. The `commits` field is documented as
"commit hashes that justify this step" (source.rs:256-259) and renders under "why: ...",
so the exemplar models output-as-justification.

Verified: `git log -1 1e1d26f` subject is the inc1 deliverable; `git merge-base
--is-ancestor 1e1d26f main` returns 0 (real, main-reachable); it touches exactly the inc1
Part-A files. So the commit is genuine and step-related; the defect is purely the
output-vs-justification semantic, low severity.

Ruling: VALID, low. The exemplar exists specifically to be a template for future authors
(D2's "prove it renders end-to-end"), so seeding "cite the step's output commit" under a
"why this step exists" field is the kind of small wrong-pattern worth not shipping. The
step's real justification is `Q-53`, already present in `decisions`. No prior commit
cleanly justifies the step's existence (the step is a new capability decided by Q-53), so
re-pointing is not available; removal is the honest fix.

Smallest correct fix: delete the `commits = ["1e1d26f"]` line at
agent-scaffold.plan.toml:720 (leaving `decisions = ["Q-53"]`), then re-run `render` and
commit the regenerated agent-scaffold.md so `render --check` stays green.

Coverage note (no loss): the render fixture already exercises all three lists
(decisions + findings + commits) in the pinned golden, so removing the live `commits`
entry costs no test coverage. The live plan never carried a `findings` entry on any step,
so it never actually demonstrated all-three on live data regardless; the fixture is the
real all-three proof.

Trade-off: this is a borderline fix-now vs defer. It is cheap (one line + a re-render) and
about template correctness, so fix-now is reasonable; if the round is being kept strictly
minimal it could defer without any correctness impact, since the entry is real and inert.
Recommendation: fix now.

## I2-2 (low, schema reviewer) -- VALID-BUT-DEFER, not actionable this round

Finding: `findings = [""]` validates clean. `is_safe_sidecar_ref("")` returns `true`
(empty path is not absolute and has zero components, so `components().all(...)` is
vacuously true), and the D5 all-empty-block check (source.rs:634-642) does not fire because
`findings` has one element. `decisions`/`commits` both reject empty (`question_id_index("")`
is None; `is_commit_shaped("")` fails the 7..=40 length), so `findings` is the only list
admitting an empty string.

Reproduction (independent): reimplemented `is_safe_sidecar_ref` verbatim in a scratch
binary: `is_safe_sidecar_ref("") = true`, empty-path component count = 0. CONFIRMED.

Ruling: VALID but DEFER. The gap is INHERITED from the existing `is_safe_sidecar_ref`
precedent (the same rule admits an empty `[meta].sidecars` ref; not new to this increment),
and the build plan deliberately scoped findings validation to "reuse `is_safe_sidecar_ref`"
(build plan Part B, findings rule). An empty findings ref is inert (render splices an empty
path, does not escape the directory). A provenance-only `!finding.is_empty()` guard would
make provenance-findings stricter than the shared rule's other caller (`[meta].sidecars`),
introducing a NEW asymmetry between two users of the same function; the correct fix is to
decide the empty-ref rule inside `is_safe_sidecar_ref` uniformly, which also changes sidecar
behavior and is out of scope for this increment (scope discipline: do not re-cut an
inherited shared precedent inside a feature increment). Defer to a follow-up that fixes the
shared rule for both callers.

Not actionable this round.
