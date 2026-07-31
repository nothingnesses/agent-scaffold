# Metrics-path independent map (Explorer C)

Model: Claude Sonnet 5 (claude-sonnet-5). Date: 2026-07-31. Worktree:
`.claude/worktrees/explore-metricspath-c`, branch `explore/metrics-path-c`.

TMPDIR discipline: `/tmp` held 106 directories before this session's work and 107 after,
a net increase of 1. The one new top-level entry, `agent-scaffold-explore-a`, carries
today's timestamp and a name matching a sibling exploration's worktree (`explore-a`,
presumably building candidate (a) in parallel), not any command run here; every
temp-file-creating command in this record had `TMPDIR` explicitly redirected, first to
`.scratch` inside this worktree, later to the session scratchpad
(`/tmp/claude-1000/.../scratchpad/metricspath-c-test-tmp`) once `.scratch` was found to
break unrelated tests (see Part 2). I cannot prove the `explore-a` entry is not mine
beyond that correlation, since `/tmp` is shared across concurrently running agents on
this machine, but no command below ran without a TMPDIR override.

A note on independence before Part 1: the Read tool returned the sidecar file whole, so
the "CANDIDATE MECHANISMS" section (which I was asked to skip until after brainstorming)
was in front of me before I wrote anything down. I read the reproduction and code
sections first and did the brainstorm below in good faith before re-consulting that
section, but I cannot claim the isolation the task intended. Where my list overlaps the
sidecar's, treat that as weaker evidence of independent convergence than it would
otherwise be. The Part 3 attacks and the Part 2 build are unaffected by this; only the
Part 1 comparison should be read with this caveat in mind.

## Reproducing the defect first-hand

Built the worktree (`cargo build`, clean). Scaffolded a throwaway fixture exactly per the
sidecar's recipe:

```
./target/debug/agent-scaffold scaffold --output-dir "$SCRATCH" --write --force --principles default
```

`ls "$SCRATCH/docs"` printed only `plans`, confirming no `docs/metrics/` exists for an
uninstrumented project.

Defect B, cross-project contamination, run from this worktree's root against the
fixture's plan:

```
$ ./target/debug/agent-scaffold validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --workflow
docs/metrics/workflow.jsonl: 235 records, valid
.../fixture-repro/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
.../fixture-repro/docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
EXIT:0
```

This worktree's own 235-record log was read (via the CWD-relative `--metrics` default)
and joined against a foreign one-step plan. Confirmed the sharper demonstration too: gave
the fixture's step the slug `agents-md-drift-guard` (which has real records in this
worktree's log) and marked it `complete`:

```
$ ./target/debug/agent-scaffold validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --workflow
docs/metrics/workflow.jsonl: 235 records, valid
.../TEMPLATE.plan.toml: 1 steps, 0 questions, valid
.../TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
EXIT:0
```

A fixture with zero review evidence of its own, declared complete, gets a green because
its borrowed slug matches unrelated rounds in a file it never intended to read. The
control, from inside the fixture with an empty log of its own:

```
$ cd "$SCRATCH" && mkdir -p docs/metrics && : > docs/metrics/workflow.jsonl
$ agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow
.../TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `agents-md-drift-guard` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"` for it if it predates logging or its review was skipped
EXIT:1
```

The check itself is sound; it was handed the wrong file. Both reproductions and the
control match the sidecar's own account exactly.

Code read to understand the mechanism (not just the sidecar's line numbers, verified
directly): `ValidateArgs::metrics` (`src/main.rs:429-431`) is a `PathBuf` with a relative
`default_value`; `run_validate` (`src/main.rs:823-847`) resolves it against
`fs::read_to_string` with no reconciliation against `--source`. `check_workflow_toml`
(`src/workflow.rs:180-195`) hands the parsed plan and log straight to `run_checks`. W3
(`src/workflow.rs:437-461`) joins `rounds.iter().filter(|round| round_step_slug(round) ==
step.slug)`, purely lexical, with no project field anywhere in `Round`
(`src/metrics.rs:620-651`) or `Meta` (`src/plan/source.rs:99-124`) before my change.

## Part 1: independent map, written before re-reading the candidate list

Generated from the defect and the code alone (with the caveat above). Organised by the
suggested directions, plus two I did not see suggested.

- Anchor the metrics default to a root derived from `--source`'s location (the
  `docs/plans/<task>.plan.toml` convention, stripping three path components). Cheapest
  fix that matches the scaffolded layout; undefined when `--source` is not laid out that
  way.
- Anchor via `git rev-parse --show-toplevel` run against `--source`'s directory rather
  than the CWD. More general than the convention-strip (works for any `--source` location
  inside a git repo), but has no answer for a non-git project and needs to handle nested
  repositories and worktrees explicitly.
- Anchor via a marker-file walk: walk up from `--source`'s directory for a sentinel (a
  `docs/plans/` sibling, or a dedicated marker like `.agent-scaffold`) rather than relying
  on git or a fixed path shape. Works without git; needs the marker to actually exist,
  which an unscaffolded or partially-migrated project may not have.
- Detect-and-refuse: keep the CWD default, but compare the resolved metrics path and
  `--source` for a shared root and hard-error the `--workflow` join when they diverge.
  Needs the same root derivation as the anchor options, so it is not a cheaper
  alternative, only a different failure mode (refuse vs. silently redirect).
- Require `--metrics` explicitly whenever `--source` is not reachable from the CWD (or
  unconditionally, dropping the default entirely). Simplest to reason about; pushes cost
  onto the user and does not fix two relative paths that are both wrong.
- Fold `--source` and `--metrics` into a single derived value structurally, removing
  `--metrics` as an independently-defaultable flag (only an override), so the two paths
  cannot drift apart because there is only one knob. A step further than the anchor
  option: instead of "apply a default derived from source," make "derived from source"
  the only path that exists absent an explicit override, closing the `value_source`
  ambiguity the sidecar names for option (a) by removing the two-flags-two-defaults shape
  that creates it.
- Plan-declared log path: a `[meta]` field in the TOML naming the metrics path relative to
  the source. Data the plan owns rather than a CLI convention reproduced by the user.
- Mutual declaration: the metrics log ALSO carries a declared identity (a first record or
  a per-record field) that must match what the plan expects, so the pairing needs
  agreement from both sides, not just the plan naming a path.
- Project identity in the round record itself (and in the plan's `[meta]`): the round,
  decision, and escalation records carry a project id, and the join in `src/workflow.rs`
  requires it to match the plan's declared id (when the plan declares one), independent of
  which file was opened. This is the one the mandate calls out as fixing the DATA rather
  than the PATH; I flag it here because it is the one place a shared or merged log
  ("two projects that legitimately share one log," named in Part 3's own suggestions)
  cannot be solved by any path-only mechanism, since a shared file necessarily contains
  both projects' records under whichever anchor gets it opened.
- Content-fingerprint join: instead of a human-chosen project name, hash something
  content-derived (the plan's own steps, or a nonce minted once at `scaffold` time) and
  require the round record to carry the same hash. A stricter, harder-to-spoof variant of
  the identity idea above; more machinery for the same property.
- Refuse invocation unless the CWD is inside `--source`'s own tree, treating "running from
  elsewhere" itself as the usage error rather than deriving anything. A more aggressive
  cousin of detect-and-refuse: it does not even try to compute the right pairing, it just
  forbids the invocation shape that produced the bug, with an escape hatch flag for the
  legitimately different case.

## Comparison against the sidecar's four

The sidecar's (a) Anchor-the-default is the same idea as my anchor-via-convention bullet,
essentially verbatim, including the exact ambiguity I called out (what happens when
`--source` is not under `docs/plans/`). (d) Plan-declared log matches my plan-declared
bullet exactly. (b) Detect-and-refuse matches mine. (c) Require `--metrics` explicitly
matches mine, including the "does nothing for two relative paths" weakness the sidecar
itself names.

Given the caveat about reading order stated above, I do not treat this four-for-four
overlap as strong evidence the space is small independently discovered; it is at least
partly convergence-by-construction, since I had already seen the section once. What I
can say with more confidence is what is NOT in the sidecar's four: the git-root-discovery
and marker-file variants (both named as directions in the mandate, but the sidecar's (a)
resolves the ambiguity by convention-stripping only, not by git or a marker), the
single-flag / no-independent-default restructuring, the mutual-declaration and
content-fingerprint identity variants, and the CWD-must-be-inside-source refusal. Most
load-bearing: DATA IDENTITY (project id on the round record itself) is explicitly named
in my mandate as a direction the sidecar's four do not take ("all of (a), (b), (c) leave
the round record itself carrying no project identity... (d) above is [that data model
question's] natural home," i.e. named as a follow-up, not built). That is the one I chose
to build, precisely because it is the one direction genuinely absent from what the other
two explorers are constructing.

## Part 2: building the data-identity mechanism

Chose project identity in the round record over my own list (not the sidecar's four)
because it is the only direction that fixes the shared-log case, which Part 3's own
suggested attacks name explicitly ("two projects that legitimately share one log"), and
because it is orthogonal to whichever anchoring or declaration mechanism ships: this
could layer under (a) or (d) as a second line of defence rather than compete with them.

Implementation (all in `src/`, uncommitted, throwaway):

- `src/metrics.rs`: added `project: Option<String>` to `Round`, parsed from an optional
  `"project"` JSON field in `parse_rounds`, following the exact pattern already used for
  `step`/`increment` (empty string treated as absent).
- `src/plan/source.rs`: added `project: Option<String>` to `Meta`, `#[serde(default,
  skip_serializing_if = "Option::is_none")]`, so an undeclared plan reserialises
  byte-identical to today.
- `src/workflow.rs`: `check_workflow_toml` now pre-filters the parsed `rounds` before
  calling `run_checks`: when `plan.meta.project` is `Some(id)`, only rounds whose own
  `project` field equals `id` survive; a round with no `project` field does NOT count once
  the plan opts in (treated as "not this project," never as a wildcard, which is what
  keeps this a strict tightening and not a new loophole). When the plan declares no
  project id, every round passes through unfiltered, i.e. today's slug-only join,
  unchanged.

This was deliberately the minimal-diff version of the idea. I did not thread a project
parameter through `w3_problems` itself (which would have required updating roughly 25
existing test call sites in `src/workflow.rs` plus `src/next.rs:1339`, which calls
`w3_problems` directly for `next`'s own forward convergence verdict). The consequence,
found only by tracing the call graph, not assumed: `next`'s own `w3_clean` check
(`src/next.rs:1339`) is NOT protected by this filter, since it calls `w3_problems`
directly rather than through `check_workflow_toml`. `next` is a best-effort projection
rather than a validator (per the sidecar's own scope note on the family), so the
severity is lower, but a production version of this mechanism would need the filter
either inside `w3_problems` itself or duplicated at every call site, not bolted onto one
caller. Reporting this as a limitation rather than silently leaving it.

Build, test, and lint:

```
$ cargo build   # clean
$ cargo test    # (see below on TMPDIR)
$ cargo clippy --all-targets -- -D warnings   # clean after cargo clean; see below
```

TMPDIR interaction discovered while testing: running `cargo test` with `TMPDIR` pointed
at `.scratch` inside this worktree (as instructed) produced 3 failures unrelated to my
change: `checks::tests::a_non_repo_target_with_runnable_checks_errors`,
`tests::init_plan_defaults_to_git_and_skips_inside_a_repo`,
`tests::install_precommit_hook_skips_a_non_repo`. All three build a scratch directory via
`std::env::temp_dir()` and assert it is NOT inside a git repository; `.scratch` inside a
worktree IS inside a git repository (a linked worktree of this one), so pointing TMPDIR
there breaks exactly these three pre-existing tests, independent of any code change.
Confirmed by re-running with `TMPDIR` pointed at the session scratchpad instead (outside
any repo): all 373 unit tests plus the 13 integration tests across 6 binaries passed (386
total, 0 failed). This is a real conflict between the mandate's `.scratch`-inside-worktree
TMPDIR instruction and this specific test suite; I resolved it by using `.scratch` for
build artefacts and fixtures (where repo-nesting does not matter) and the session
scratchpad for `cargo test`/`cargo clippy` (where it does), and I am reporting the
substitution rather than silently doing it.

The first `cargo clippy --all-targets -- -D warnings` attempt failed with `E0514` (crate
compiled by an incompatible rustc) and `Cli::parse` not found, both symptoms of a stale
`target/` built by a mismatched toolchain invocation earlier in this same session
(clippy's rustc was 1.88.0, the cached artefacts were built by a 1.98.0-nightly). `cargo
clean` followed by a properly `direnv`-prefixed `cargo clippy` resolved it with zero
warnings. Recorded as an environment artefact of my own earlier command sequence, not a
defect in the code change; noting it here because the instructions ask for failures
verbatim, and this one looked at first glance like it might be code-caused.

Final counts: `cargo test` 386 passed, 0 failed, across 7 test binaries (373 unit + 5 + 1
+ 1 + 3 + 1 + 2 integration). `cargo clippy --all-targets -- -D warnings`: 0 warnings, 0
errors.

Measurements:

1. Safe on existing projects, this repository's own correct case, unaffected:

```
$ ./target/debug/agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 235 records, valid
docs/plans/agent-scaffold.plan.toml: 93 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
EXIT:0
```

Identical to before the change, because `agent-scaffold.plan.toml` declares no `[meta].
project`, so the filter is a no-op.

2. The false pass, killed, once the fixture opts in by declaring a project id (added
`project = "fixture-repro"` to the fixture's `[meta]`, representing what `scaffold` would
assign at creation time in a shipped version of this idea):

```
$ ./target/debug/agent-scaffold validate --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --workflow
.../TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `agents-md-drift-guard` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"` for it if it predates logging or its review was skipped
EXIT:1
```

None of this worktree's 235 real records carry `project":"fixture-repro"`, so none match,
regardless of the borrowed slug.

Important limitation surfaced by this measurement, stated plainly: this mechanism only
protects a plan that HAS opted in by declaring a project id. The scaffold fixture as
generated today declares none, so the false pass as originally reproduced (no manual
edit) is UNCHANGED by this mechanism; I had to add the `project` field by hand to see the
fix engage. A shipped version would need `scaffold` to assign a project id automatically
(and the logging instructions to emit it on every record) for this to protect new
projects by default rather than opt-in. This mirrors the two-tier `--instrument` design
already in this codebase (opt-in, not core), which is a defensible shape here but is a
real cost this mechanism carries that a path-anchoring fix does not: path anchoring
protects every invocation by construction; this protects only invocations of projects
that adopted the field.

3. Positive case, two independently adopting projects with matching project ids and their
own correctly-tagged records, both hold correctly:

```
$ agent-scaffold validate --source fixture-A/.../TEMPLATE.plan.toml --metrics fixture-A/.../workflow.jsonl --workflow
... vs ...: workflow invariants hold
EXIT:0

$ agent-scaffold validate --source fixture-B/.../TEMPLATE.plan.toml --metrics fixture-B/.../workflow.jsonl --workflow
... vs ...: workflow invariants hold
EXIT:0
```

4. Cross pairing, same slug (`shared-slug`), both schema-valid, different declared
project ids, correctly rejected:

```
$ agent-scaffold validate --source fixture-A/.../TEMPLATE.plan.toml --metrics fixture-B/.../workflow.jsonl --workflow
... vs ...: Roadmap step `shared-slug` is `complete` but has no round records and no covering waiver; ...
EXIT:1
```

5. The shared-log case (concatenated fixture-A and fixture-B logs into one file, each
project's plan pointed at the merged file), the case a path-only mechanism structurally
cannot solve:

```
$ cat fixture-A/.../workflow.jsonl fixture-B/.../workflow.jsonl > merged-workflow.jsonl
$ agent-scaffold validate --source fixture-A/.../TEMPLATE.plan.toml --metrics merged-workflow.jsonl --workflow
merged-workflow.jsonl: 2 records, valid
... vs merged-workflow.jsonl: workflow invariants hold
EXIT:0

$ agent-scaffold validate --source fixture-B/.../TEMPLATE.plan.toml --metrics merged-workflow.jsonl --workflow
merged-workflow.jsonl: 2 records, valid
... vs merged-workflow.jsonl: workflow invariants hold
EXIT:0
```

Both projects hold correctly against a genuinely shared file containing both of their
records, because each project's own record carries its own id. This is the property no
path-anchoring or plan-declared-path mechanism can provide, since both of those decide
which FILE to open, and a legitimately shared file is one file containing both projects'
data.

## Part 3: attacking (a) and (d)

Neither candidate's actual code was available to me (built in parallel, in different
worktrees I was told not to touch). Where I could not run their literal implementation, I
simulated the mechanism AS DESCRIBED in the sidecar (the specification both explorers are
presumably building from) by hand-computing the path their rule would produce and passing
it to the real, current binary via an explicit `--metrics`, since the join logic itself
(`src/workflow.rs`, described by the sidecar as unchanged by either (a) or (d)) is exactly
what I already have. Flagging this plainly: these are attacks on the SPECIFICATION, run
against the current binary standing in for it, not verified against the sibling
explorers' actual committed code, which I have not seen.

Attack 1: shared log, hits (a) and (d) equally. Built two fresh projects, C and D, C with
a real converged `setup` step, D with `setup` falsely marked `complete` and zero review
records of its own, both plans pointed via `--metrics` at ONE shared log file containing
only C's real round (the scenario named in this task's own suggested attack list, "two
projects that legitimately share one log"). Neither declares a project id (representing
(a)/(d) after their path fix, since neither changes the record schema or the join):

```
$ agent-scaffold validate --source fixture-C/.../TEMPLATE.plan.toml --metrics shared-metrics/workflow.jsonl --workflow
... vs ...: workflow invariants hold
EXIT:0

$ agent-scaffold validate --source fixture-D/.../TEMPLATE.plan.toml --metrics shared-metrics/workflow.jsonl --workflow
... vs ...: workflow invariants hold
EXIT:0
```

D gets a green for a step it never reviewed, even though the file resolved is genuinely
the intended, shared, correctly-anchored file for both projects: correct file resolution
does not help here because the join inside that file is still by slug alone. Outcome:
SUCCEEDED against both (a) and (d) as specified.

Attack 2, specific to (a): what happens when `--source` is not laid out as
`<root>/docs/plans/<task>.plan.toml`. The sidecar names this as an explicitly open
question for (a)'s implementer ("deciding what happens when the source is NOT under
docs/plans/... the derivation has no convention to lean on"). Built a plan file directly
inside `.scratch` (one directory level, not the two `docs/plans` levels the convention
assumes) and simulated the most likely minimal-diff resolution choice: fall back to the
CWD-relative default when the convention match fails (a very plausible thing to write,
and the cheaper of the two forks named). Since this fallback is, by construction, exactly
today's unfixed default, running it directly against the real binary IS running the
simulated resolution:

```
$ ./target/debug/agent-scaffold validate --source "$TMPDIR/myplan.plan.toml" --workflow
docs/metrics/workflow.jsonl: 235 records, valid
.../myplan.plan.toml: 1 steps, 0 questions, valid
.../myplan.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
EXIT:0
```

The original defect reproduced verbatim for this input class. The OTHER fork (hard-error
when the convention does not match) is safe but means (a), as scoped, does not actually
solve non-conventional `--source` locations, it just converts them into a hard failure,
which is functionally closer to option (c) for that subset of inputs than to (a)'s own
stated goal. Outcome: SUCCEEDED against the fallback fork; the hard-error fork survives
this specific attack but narrows what (a) actually covers, which is itself worth stating
in the round that ships it.

Attack 3, specific to (d): a copied or monorepo-shared relative declaration escaping the
plan's own tree via `..`. Built `monorepo/original` (a real project with a converged
`shared-slug` step and its own log) and `monorepo/victim` (a sibling, freshly scaffolded,
`shared-slug` falsely marked complete, no log of its own), and hand-resolved what a
`[meta]` field declaring `metrics = "../../../original/docs/metrics/workflow.jsonl"`
(the kind of value a monorepo template or a careless copy-paste would carry over
unmodified) resolves to relative to victim's own source directory:

```
$ realpath -m victim/docs/plans/../../../original/docs/metrics/workflow.jsonl
.../monorepo/original/docs/metrics/workflow.jsonl   # exists: yes, original's real log
$ agent-scaffold validate --source monorepo/victim/docs/plans/TEMPLATE.plan.toml --metrics <that resolved path> --workflow
.../original/docs/metrics/workflow.jsonl: 1 records, valid
.../victim/docs/plans/TEMPLATE.plan.toml: ... valid
.../victim/docs/plans/TEMPLATE.plan.toml vs .../original/docs/metrics/workflow.jsonl: workflow invariants hold
EXIT:0
```

Victim gets a green sourced entirely from original's review. This is not a hypothetical
concern: this codebase already has a named precedent for exactly this failure mode.
`is_safe_sidecar_ref` (`src/plan/source.rs:497-499` onward) exists specifically to reject
an absolute path or a `..` component in the OTHER `[meta]`-declared relative refs (the
front/tail prose sidecars), with the comment stating the reasoning directly: "a crafted
`.plan.toml` [could] read a file OUTSIDE the plan directory," citing Principle 21
(validate external input where it enters) and Principle 18 (least authority). A `[meta].
metrics` field declared the same way, without the identical safety check, inherits the
exact vulnerability class this project already found and patched once for a sibling
field. Outcome: SUCCEEDED, and backed by an in-repo precedent, not merely a hypothetical.

Attack 4, hits (a) and (d) equally: an explicit but still-relative `--metrics` bypasses
both, because both are scoped to the DEFAULT resolution, not to an explicit override. (a)
says so directly ("when `--metrics` was not supplied"); nothing in (d)'s description
suggests an explicit `--metrics` flag would stop overriding the plan's declaration either,
since that is the ordinary and expected precedence for a CLI flag over a file-declared
default. Built a clean fixture (no project id) with the borrowed slug
`agents-md-drift-guard`, and passed the literal default string as an explicit flag:

```
$ ./target/debug/agent-scaffold validate --source fixture-G/.../TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl --workflow
docs/metrics/workflow.jsonl: 235 records, valid
.../fixture-G/.../TEMPLATE.plan.toml: 1 steps, 0 questions, valid
... vs docs/metrics/workflow.jsonl: workflow invariants hold
EXIT:0
```

The original defect reproduced verbatim. Nothing about this input is unusual or
adversarial: a user who copy-pastes `--metrics docs/metrics/workflow.jsonl` from an
example, or from muscle memory, while working with a `--source` elsewhere, defeats both
(a) and (d) exactly as today. Outcome: SUCCEEDED against both.

## What I did not verify

- The sibling explorers' actual (a) and (d) implementations. All Part 3 attacks are run
  against the specification in the sidecar, standing in with the current binary's
  unchanged join logic plus a hand-computed path, not against code I have read or run
  from their worktrees.
- Whether `scaffold` could reasonably be extended to auto-assign a project id at creation
  time (needed to make my Part 2 mechanism protect new projects by default rather than
  opt-in). I did not attempt this; it would touch the render/manifest pipeline, which is
  outside a metrics-path spike.
- Whether threading the project filter into `w3_problems` itself (rather than
  pre-filtering in `check_workflow_toml`) is mechanically straightforward across the ~25
  existing test call sites and `src/next.rs:1339`. I traced that `next` is unprotected by
  my current implementation but did not attempt the fuller threading.
- W4 and W5's exposure to the same cross-project join gap. My filter only touches the
  `rounds` fed to `run_checks`; `decisions` and `escalations` are read unfiltered in
  `check_workflow_toml`, so a foreign log's decision/escalation records could still
  satisfy W4/W5 by slug alone even with my change in place. Not attacked or measured;
  flagged as an open gap in my own spike, not resolved.
- The git-root-discovery and marker-file variants from my Part 1 list were not built or
  measured, only reasoned about.
- Whether clippy or test failures exist under the OTHER rustc/toolchain paths this
  project might be built with; I only exercised the `direnv`-provided Nix toolchain per
  the mandate's prefix.

## Recommendation

Ship the anchor, (a), as the primary fix, and treat project identity (my build) as a
follow-up worth queuing, not a competing choice for THIS step.

Reasoning against the named principles. Safe on existing projects is what decides this
first: (a) is a no-op for every plan that does not declare a project id and for every
invocation already run from the plan's own root, which is the normal case and the only
one the scaffolded guidance documents; my identity mechanism is ALSO a no-op for an
undeclared plan, but only protects an ADOPTING plan, which is a smaller default footprint
than (a)'s "every invocation, no adoption required." Minimal by default favours (a)
similarly: it requires zero authoring effort from the user, where identity requires
`scaffold` to mint an id and the logging instructions to emit it on every record, a bigger
surface for the same step. Make illegal states unrepresentable and Ground decisions in
evidence both cut toward doing (a) properly rather than half-measuring it: the sidecar's
own open question, what happens when `--source` is not under `docs/plans/`, must be
resolved as a hard, typed refusal (my Attack 2 shows the fallback-to-CWD fork silently
resurrects the exact bug being fixed), not left to whichever branch an implementer reaches
first.

Where I differ from treating (a) as sufficient on its own: Attack 1 (shared log) and
Attack 4 (explicit relative `--metrics`) both show that (a) does not close this defect for
every input, only for the specific shape of "no explicit `--metrics`, one project per
log." Prefer the cleaner long-term architecture over the smallest diff and Structured
data first, project for humans both point at the identity direction as the thing that
actually closes those two gaps, since they are gaps in WHAT the join keys on, not in
WHICH FILE gets opened, and no path fix, however carefully anchored, changes what the join
keys on. The sidecar already recognises this (its own text: "it does not add project
identity to the round record... that is a data-model question for the queued
validation-constraints step"), and my build only strengthens that recognition with a
running counterexample (the merged-log measurement) rather than overturning it.

Concretely: ship (a) now, written to hard-error (not silently fall back) when `--source`
does not resolve under a `docs/plans/`-shaped root, since that is the input Attack 2
found and the cheap, defensible half of the fork. Do not ship (d) as a parallel
mechanism for this step: Attack 3 shows it needs the exact `is_safe_sidecar_ref`-style
traversal guard this codebase has already had to write once for a sibling field, which
is scope this step's own text says (d) does not yet carry, and building it now
duplicates work the validation-constraints step is explicitly slated to do properly with
identity attached. Queue the project-identity direction there instead of here, since
Idempotent and Reproducible are properties of the CHECK, not of which file it read, and
identity is what makes the check's answer independent of file layout entirely, which
(a) alone cannot claim regardless of how carefully its anchor is written.

I do not think the whole framing is wrong. The problem (a plan and a log can be paired
wrong, silently) is real, reproduced twice more here on top of the sidecar's own
reproduction, and every mechanism in both lists is a genuine attempt at a real property
(Safe on existing projects vs. closing the gap for every input vs. minimal cost). The
open question worth raising for the human, not decided here, is whether inc1 (defect B,
the path fix) should ship (a) alone as scoped, or ship (a) plus a minimal, opt-in identity
field from the start, given that Attack 1 and Attack 4 show (a) alone leaves two
named-in-advance input shapes unfixed and the cost of adding the two optional fields
(`Round.project`, `Meta.project`) and the one filter in `check_workflow_toml` demonstrated
here is small relative to the step's existing size.
