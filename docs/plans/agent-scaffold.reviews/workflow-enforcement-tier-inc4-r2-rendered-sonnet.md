# `workflow-enforcement-tier-inc4`, round 2, rendered-view reader

Reviewed at `a534d69` on branch `review/wet-inc4-r2-c`, worktree
`.claude/worktrees/rev-inc4-r2-c`. Lens: read `docs/plans/agent-scaffold.md` as its actual
reader, top to bottom, and only consult the TOML/sidecar sources to attribute a defect once
found in the output. Every command below was run in this worktree; the one fixture I built
lives at `<scratchpad>/rev-inc4-r2-c/f3` only.

## What I read

- The Status line and its counts (`docs/plans/agent-scaffold.md:5`), cross-checked against
  `docs/plans/agent-scaffold.plan.toml` by grep/count for step statuses, question statuses,
  and waiver reasons.
- The full Roadmap row for `workflow-enforcement-tier` (`:324`), including its three waiver
  notes, cross-checked verbatim against the TOML waiver `note` fields.
- The complete `Q-55` question record as rendered (`:152` through `:170`), cross-checked
  against `docs/plans/agent-scaffold.plan.toml:1713-1736` (the `ask` field; `docs/plans/
  agent-scaffold.questions/Q-55.md` is an empty placeholder, so the ask lives in the TOML).
- The complete Step Detail for `workflow-enforcement-tier` (`:1396` through `:1799`),
  including the four defects, the mechanism, the refusal/omit split, the JSON reason
  section, the four accepted costs, the increment list, the risk classifications, the full
  numbered acceptance check (1 through 23), the Documentation impact section, and the
  "Scope: what this step does not do" section.
- Sampled citation resolution in the three sidecars acceptance check 21b names
  (`test-tmpdir-repo-assumption.md`, `checks-runner-worktree-name-collision.md`,
  `instrument-magic-filename.md`): I opened every `src/main.rs:` and `src/checks.rs:` range
  I found cited in those files and confirmed each holds its named subject.
- `pack/AGENTS.md:88-96` and the deployed `AGENTS.md:88-96` (the Worktree lifecycle
  paragraph Defect D concerns), and `README.md:210-260`, read directly against the plan's
  claims about them.
- The round-1 triage file (`workflow-enforcement-tier-inc4-r1-triage.md`) and the round-1
  fix commits (`8945e76`, `2eb06f5`, `218c8c3`, `5b529eb`, `a534d69`), to identify which
  round-1 remedies this round needed to verify landed in full.

What I did not do: a fresh citation sweep of the whole step detail (round 1 already ran three
such sweeps); re-derive the round-count arithmetic in the waiver notes (already verified
exact by `R1B-1`'s triage and confirmed unchanged here by grep, see below); or read the
Roadmap rows for steps other than `workflow-enforcement-tier` beyond the aggregate count
check.

## Summary

Three findings, all medium, all in the same family: the round-1 fix pass retensed the
`Q-55` record's twin claims in place (git commit `5b529eb`), but did the retensing
incompletely on two of the seven twins, and missed an eighth, structurally identical,
present-tense-"today" claim sitting in the same step's own risk-classification prose. All
three are reproducible by reading the rendered file alongside itself: each is a direct
contradiction between two passages describing the same fact, one correctly retensed and one
not, both still present after the round-1 fix pass whose entire job was this retensing sweep.

No high or critical findings. No findings on the Status line, the Roadmap row, the waiver
note arithmetic, the increment list, or the acceptance-check numbering: all of those were
internally consistent and matched their TOML sources on inspection.

## `R2C-1` (medium): `README.md:228` citation in the `Q-55` record is wrong and contradicts the correct citation elsewhere in the same document

REPRODUCIBLE. Rendered `docs/plans/agent-scaffold.md:166` (source: `docs/plans/
agent-scaffold.plan.toml:1732`, the `Q-55-refusalscope` paragraph of the `Q-55` `ask`):

> THE GROUND, which resolves the contract objection rather than overriding it: `README.md:228` said "Unlike `validate` it never fails on a missing or malformed file (a missing part is simply left out of the projection)", and `run_resume`'s doc comment (`src/main.rs`) matches it...

`README.md:228` today:

```
$ sed -n '228p' README.md
# `--source` and `--plan` pair
```

That is a comment line inside the fenced shell example in the `validate` section, not the
quoted sentence. The quoted sentence is at `README.md:238`:

```
$ grep -n "Unlike \`validate\` it never fails" README.md
238:`status` prints a best-effort projection of that state: ... Unlike `validate` it never fails on a missing or malformed file (a missing part is simply left out of the projection), and `--json` emits the projection as JSON ...
```

The SAME document states the correct line number four times elsewhere, describing the SAME
fact for the SAME decision:

- `docs/plans/agent-scaffold.md:1568` (source: sidecar `workflow-enforcement-tier.md`, the
  "One predicate, two responses" section): "`README.md:238` does not merely promise the
  projections never fail; it says 'Unlike `validate` it never fails on a missing or
  malformed file...'"
- `:1754`, `:1761`, `:1764` (source: same sidecar, "Documentation impact" section), all
  three citing `README.md:238` for this paragraph.

So the rendered document contradicts itself on where this sentence lives: `:166` says `228`,
`:1568`/`:1754`/`:1761`/`:1764` say `238`, and the tree confirms `238` is right.

ATTRIBUTION. The false claim is in `docs/plans/agent-scaffold.plan.toml:1732`, inside the
`ask` field of `[[question]] id = "Q-55"`. It is not covered by acceptance check 21 or 21b:
both are scoped to "THIS FILE" (the step's own sidecar, `workflow-enforcement-tier.md`) and
to the three named sidecars respectively; the `Q-55` question's `ask` field is neither, so no
declared acceptance check exercises this citation at all.

WHY THIS SURVIVED THE ROUND-1 FIX. Round-1 triage (`R1C-5`, row 2 of its seven-twin table)
found this exact defect and recorded that "the pass corrected this same citation to `:238`
at sidecar `:173`" (the sidecar copy), leaving the plan-TOML copy as one of the twins the
human had to decide about. Commit `5b529eb` retensed the surrounding sentence's verb
("says" to "said") but left the citation number itself untouched, so the fix addressed the
tense half of the twin and missed the citation half, which was the actual falsity `R1C-5`
identified.

SEVERITY medium. A reader who follows this citation from the `Q-55` record lands in the
middle of a shell comment inside a code fence, ten lines from the sentence being quoted, and
has no cue that they are in the wrong place other than the quoted text not being adjacent. It
is the same class of defect this whole increment exists to remove (a citation that no longer
resolves), sitting in a document that four lines later, in the same document, gets it right.

MINIMAL REMEDY: token substitution, `228` to `238`, at `docs/plans/agent-scaffold.plan.toml:1732`, then re-render. No prose is authored.

## `R2C-2` (medium): the `Q-55` record claims the scaffolded guidance "still promises" an unconditional backstop, which the same document elsewhere says was corrected

REPRODUCIBLE. Rendered `docs/plans/agent-scaffold.md:162` (source: `docs/plans/
agent-scaffold.plan.toml:1728`, the "TWO SCOPE ADDITIONS" paragraph of the `Q-55` `ask`):

> (1) The DOCUMENTATION half of SE-3 is IN SCOPE: the two-tier split was undocumented in the scaffolded AGENTS.md, so a non-instrumented user read an unconditional promise of the `validate --workflow` backstop (`pack/AGENTS.md:93`) and, after the tier policy lands, meets a hard failure from a check the guidance still promises them.

The first clause ("read") is correctly past-tensed. The second clause ("meets ... the
guidance still promises them") is present tense and asserts, as an ongoing fact, that the
scaffolded guidance currently, unconditionally promises the `--workflow` backstop. That is
false of the tree today. `pack/AGENTS.md:93` (and the deployed `AGENTS.md:93`) read:

```
$ sed -n '93p' pack/AGENTS.md
Correctness against the plan is established by the review loop and the acceptance review, not by the merge; when instrumentation is on, the deterministic `validate --workflow` check is the backstop that the required reviewed rounds happened before a step is marked complete, and on a project with no round log yet, which every project scaffolded without `--instrument` remains, that check exits non-zero reporting that it could not run rather than passing.
```

That is the QUALIFIED form inc3 shipped, not the unconditional one. The same rendered
document says so correctly at `docs/plans/agent-scaffold.md:1534` (source: same sidecar,
Defect D section):

> That sentence WAS rendered from `pack/AGENTS.md:93`, in the "Worktree lifecycle and merge-back" paragraph, and it WAS UNCONDITIONAL.

`:1534` is consistently past-tensed throughout ("WAS rendered", "WAS UNCONDITIONAL", "READ a
promise", "GOT no `docs/metrics/` directory", "GOT exit 0"). `:162`'s "still promises them"
directly contradicts it: one passage says the unconditional promise WAS the state (now
fixed), the other says the guidance STILL promises it (an ongoing, current state). A reader
of the `Q-55` record alone, on reaching "a check the guidance still promises them", forms the
belief that the scaffolded `AGENTS.md` currently contains an unconditional promise; it does
not, and the same document says so 1,372 lines later.

ATTRIBUTION. `docs/plans/agent-scaffold.plan.toml:1728`, inside `Q-55`'s `ask` field. This is
row 6 of `R1C-5`'s seven-twin table in the round-1 triage; commit `5b529eb` retensed the same
paragraph's neighbouring clause ("reads" to "read", "carry" to "carried") but left this
clause's "meets ... still promises" untouched.

SEVERITY medium, on the same reasoning as `R2C-1`: it is a direct contradiction between two
passages of the same rendered document about the same fact, not merely an awkward phrasing,
and it sits in the exact record this increment's own text (`docs/plans/agent-scaffold.md:1703`)
names as the failure mode to guard against ("a pass that re-tenses a false claim can write a
NEW false claim in its place... a reviewer must check what was written and not only what was
removed"); here nothing new was written, but a sibling clause in the same sentence that WAS
retensed makes the untouched one conspicuous rather than incidental.

MINIMAL REMEDY: token-level re-tense, for example "meets" to "would have met" and "still
promises" to "then promised" (or fold the whole clause into past tense to match "read"). No
new fact is introduced.

## `R2C-3` (medium): the inc2 risk-classification paragraph says validator and projection invocations "succeed today" / "answer today", which inc2 (already landed) made false, while the sibling accepted-costs paragraphs describing the identical change were correctly retensed

REPRODUCIBLE. Rendered `docs/plans/agent-scaffold.md:1699` (source: sidecar
`workflow-enforcement-tier.md:304`, the inc2 risk-classification paragraph):

> `workflow-enforcement-tier-inc2` is RISKY (two consecutive clean rounds), and for reasons that do not overlap inc1's. It INTRODUCES a non-zero exit on validator invocations that succeed today AND withholds output from projection invocations that answer today, and it does so with a MEASURED FALSE POSITIVE already in hand (accepted cost (ii), the symlinked `docs/plans` directory)...

"Succeed today" and "answer today" assert, in the present tense, that these invocations
currently succeed/answer. inc2 has already shipped (inc3 and inc4 are what remain), and its
entire effect on this named example (accepted cost (ii), the symlinked `docs/plans` layout)
is that such invocations no longer succeed. Demonstrated on a scratch fixture outside any
repository:

```
$ cargo run --quiet -- scaffold --output-dir "$SCRATCH/elsewhere" --write --force --principles default --instrument
$ mkdir -p "$SCRATCH/elsewhere/docs/metrics" && : > "$SCRATCH/elsewhere/docs/metrics/workflow.jsonl"
$ mkdir -p "$SCRATCH/root/docs" && ln -s "$SCRATCH/elsewhere/docs/plans" "$SCRATCH/root/docs/plans"
$ cargo run --quiet -- validate --source "$SCRATCH/root/docs/plans/TEMPLATE.plan.toml" --workflow
--workflow would join .../root/docs/plans/TEMPLATE.plan.toml against .../root/docs/metrics/workflow.jsonl, which is not under the plan's project root .../elsewhere; pass a `--metrics` under that root, run against the plan's own log, or correct the `--source` and `--plan` pair
EXIT: 1
```

Exit 1, refused, exactly as the document's own accepted-cost (ii) paragraph
(`docs/plans/agent-scaffold.md:1652`) says: "A measured this layout going from reading its
37-record log to `exit=1 REFUSED`."

The SAME document, describing the SAME accepted costs of the SAME inc2 change, correctly
retensed the identical claim twice, in the immediately following section:

- `:1652`: "This is a genuine new failure for a layout that **worked before inc2**" (not
  "works today").
- `:1654`: "`--source <root>/docs/plans/x.plan.toml --plan <root>/notes/p.md` **greened
  before inc2**" (not "greens today").

Both of those were the exact remedy `R1C-4`(b) and (c) prescribed in round 1 ("a layout
that works today" to "a layout that worked before inc2"; "greens today" to "greened before
inc2"), and both landed correctly. The identical pattern in the risk-classification
paragraph, describing the identical inc2 behaviour change, was not swept.

ATTRIBUTION. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:304`. Not covered
by acceptance check 21 in the sense of being run as a literal quotation search (it is not a
quoted fragment of source, test, README or pack text), but it is exactly the class of
present-tense-"today" claim `R1C-4` found and fixed three sibling instances of in this same
file.

SEVERITY medium: consistent with `R1C-4`'s own severity for the same claim shape in the same
file ("its capitalised headline claim is INVERTED... a reader sent to verify an accepted cost
and finding the opposite behaviour is the failure this section exists to prevent"). Here the
claim is not capitalised or as emphatic, but it is the risk-classification paragraph a
reviewer reads first to calibrate how hard to look at inc2, and it currently tells that
reader the invocations it is talking about currently succeed, when the whole point of
reading the next section (four paragraphs later) is that they no longer do.

MINIMAL REMEDY: token-level re-tense, "succeed today" to "succeeded before inc2", "answer
today" to "answered before inc2", matching the phrasing already used at `:1652` and `:1654`
for the same fact. No new prose, no restructuring.

## Checks that came back clean

- Status line counts (`:5`): step-status tally (4 not-started, 4 in-progress, 63 complete, 4
  skipped, 3 optional, 17 deferred = 95), open-question tally (2 `open` + 3 `exploring` = 5),
  and waiver tally (11 `predates-logging` + 4 `review-skipped` + 9 `accepted-at-escalation`
  = 24) all matched an independent count of `docs/plans/agent-scaffold.plan.toml` exactly.
- The `workflow-enforcement-tier` Roadmap row (`:324`): the step has exactly 4 increments and
  3 waivers in the TOML (inc4 has none, correctly, since it has not converged), and all three
  waiver `note` fields render byte-for-byte into the Roadmap row (checked programmatically,
  not by eye).
- The inc1 waiver-note figure ("13 valid findings (3, 4, 6)") that `R1B-1` found wrong at 20
  in round 1: now consistent at both rendered sites, `:324` (from the TOML waiver note) and
  `:1703` (from the sidecar's risk-classification paragraph), and both match the round log
  (`3 + 4 + 6 = 13`, confirmed via `jq` against `docs/metrics/workflow.jsonl`).
- Citations sampled from the three sidecars acceptance check 21b names
  (`test-tmpdir-repo-assumption.md`, `checks-runner-worktree-name-collision.md`,
  `instrument-magic-filename.md`): every `src/main.rs:` and `src/checks.rs:` range I opened
  held its named subject.
- `checks-runner-worktree-name-collision.md:55`, the subject of both `R1A-1` and `R1C-6` in
  round 1: now reads as a conclusion with no enumerated list (`R1A-1`'s deletion remedy) and
  cites `src/checks.rs:1037-1046` for the moved-subject helper (matching check 21b's narrowed
  exclusion clause, `R1C-6`'s remedy). Consistent.
- The four accepted-costs paragraphs' OTHER retensed claims (cost (i)'s "remained a silent
  miss" / "became a HARD FAILURE" at `:1650` and `:170`): correctly past-tensed and agree
  with acceptance check 18's phrasing.
- Step count of in-progress steps (4): `workflow-calibration`, `workflow-driver`,
  `code-value-audit-static`, `workflow-enforcement-tier`, matching the Status line.

## Fixture note

The one fixture built for `R2C-3` lived entirely under
`<scratchpad>/rev-inc4-r2-c/f3/` (a scaffold copy plus one symlink), outside any git
repository and outside this worktree. No chmod was used. Nothing outside that subdirectory
was touched.
