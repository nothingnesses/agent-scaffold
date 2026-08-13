### `ship-v0-0-2`: fix the two audit defects, reserve `agent-flow`, and publish `agent-scaffold` 0.0.2 (`Q-71`, `Q-74`)

Decided (`Q-71`, human, 2026-08-13). The 2026-08-13 workflow audit (`docs/plans/workflow-calibration.explorations/2026-08-13-audit-when-the-loop-turned.md`) reports one release, `v0.0.1`, on project day 2, 937 commits ago, and eleven consecutive days producing zero completed steps across 196 commits. This step ends that. Its purpose is delivery, so its scope is closed and every addition to it defeats it.

WIDENED ONCE, by human decision `Q-75` (2026-08-13, decision receipt `type:"decision"` `q_id:"Q-75"`), to add `F4b`, the pack `source` read escape found during the implementation of `F4` and reported rather than fixed. The closed-scope sentence above stands and is not repealed: `Q-75` is the single authorised exception to it, recorded as an exception so it is visible as one, and a second addition needs its own human decision rather than this precedent.

SCOPE, exactly, and nothing else:

- Fix `F1` (the unescaped projection).
- Fix `F4` (the missing pack `dest` containment).
- Fix `F4b` (the missing pack `source` containment), the read-side twin of `F4`, admitted by `Q-75`.
- The README items the `Q-65` crates.io checklist requires: link to `agent-flow`, and state that the `agent-scaffold` name is free for whoever wants to reclaim it, by opening an issue on the `agent-flow` GitHub repository.
- Reserve `agent-flow` on crates.io at 0.0.2.
- Publish `agent-scaffold` 0.0.2, leaving the earlier `agent-scaffold` versions un-yanked so the name stays reclaimable.

Out of scope and named so it is not drawn in: the audit's `F2` (declared `[[step.increment]]` entries have no readers), its `F3`, its recommendations 2 to 10 (held in `workflow-audit-followups`), the rename itself (`rename-to-agent-flow`), and the pre-existing `superseded by` projection defect recorded under the `F1` acceptance criteria below.

Risk class (`Q-74`, human, 2026-08-13): `low_risk`, declared on `ship-v0-0-2-inc1`, so one clean round converges the loop.

### Defect `F1`: `render` interpolates free-text TOML into generated Markdown unescaped

Verified against the source at `a4394e4`, not against a summary. `escape_cell` (`src/plan/render.rs:538`) neutralises `|` and every line-ending form, and it is called at ONE site, `src/plan/render.rs:467`, the Roadmap Notes cell. Three further sites interpolate free-text TOML into the generated Markdown with no neutralisation at all:

- `src/plan/render.rs:296`, `[meta].title` into the `# <title> plan` heading.
- `src/plan/render.rs:402`, `principle.name` and `principle.text` into the numbered Project Principles list.
- `src/plan/render.rs:436`, `question.ask` into the Open-Questions queue line, which `src/plan/render.rs:424` documents as "strictly ONE line per item".

So the generated document's STRUCTURE is controlled by the source's free TEXT, and a `.plan.toml` that `validate --source` accepts can render a `<task>.md` that `validate --plan` rejects. Principle 8 (Structured data first, project for humans) is the principle this breaks: a projection that the projecting tool's own validator refuses is not a projection.

REPRODUCTION (written by this planning pass; the audit record carries none). Copy the render fixture out of the source tree, give one question a two-line `ask` whose second line looks like a queue item, then render and validate:

```
cp -r src/plan/testdata/render-fixture.* /tmp/f1/
# in /tmp/f1/render-fixture.plan.toml, replace Q-1's ask with:
#   ask = """An open ask still awaiting a decision.
#   - `Q-42` (undecided) a queue item nobody authored."""
cargo run -- validate --source /tmp/f1/render-fixture.plan.toml --metrics /dev/null
cargo run -- render /tmp/f1/render-fixture.plan.toml
cargo run -- validate --plan /tmp/f1/render-fixture.md --metrics /dev/null
```

Measured at `a4394e4`: `validate --source` reports "7 steps, 5 questions, valid" at exit 0, `render` writes the projection, and `validate --plan` reports "Open Questions item `Q-42` has an unknown status `undecided`" at exit 1. `Q-42` is in no `[[question]]` entry. A blank line in an `ask` produces the quieter form of the same defect: the queue list fragments and the trailing prose lands loose between items, which no check reports at all.

ACCEPTANCE CRITERIA for `F1`, each executable:

1. Red then green on the reproduction above. Before the fix `validate --plan` reports the fabricated `Q-42`; after it, the same source renders a projection whose queue carries exactly the `[[question]]` ids the source declares, and `validate --plan` says nothing about `Q-42`. The reproduction lands as a test, so the red state cannot return silently.
2. All four interpolation sites are covered, not only the queue. A test pins each of `src/plan/render.rs:296`, `:402`, `:436` and `:467` against a source whose corresponding free-text field carries a line ending, a `|`, and a leading `- ` list marker, and asserts the generated line count and the generated structure are what the source's STRUCTURE implies.
3. `render --check --strict docs/plans/agent-scaffold.plan.toml` reports up to date after the live projection is regenerated in the same change. The live projection WILL change: many live `ask` values are multi-paragraph, so they currently fragment the queue and will collapse to one line each. That is the contract being restored, not a regression.
4. `validate --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl` reports EXACTLY ONE problem afterwards, the pre-existing "Open Questions item `Q-43` has an unknown status `superseded by `Q-44``". That problem is NOT `F1` and is out of this step's scope: `question_status_display` (`src/plan/render.rs:445`) emits `superseded by <slug>` while `question_status_ok` (`src/plan.rs:378`) accepts only the exact statuses plus the `decided -> folded into ` prefix, so the two vocabularies disagree independently of any escaping. It is recorded here so a reviewer neither mistakes it for an `F1` escape nor widens the step to fix it.

FIX DIRECTION, recommended with its reasoning, and the alternative named. Neutralise every free-text value at the point where it is interpolated into a generated line, extending the one-line rule `escape_cell` already applies to the Notes cell, and keep the `|` escape for table cells where it matters. The alternative is to reject a multi-line `ask` in `validate --source` and move that prose into the question sidecars, which is the stronger Principle 5 form (Make illegal states unrepresentable). It is not taken here because it relocates prose across about twenty questions, which is exactly the churn this step exists to avoid (Principle 2, Minimal by default), and it stays available to a later step. State which was chosen in the outcome.

### Defect `F4`: a pack `dest` can write outside `--output-dir`

Verified against the source at `a4394e4`. `src/manifest.rs:46` documents `AssetSpec.dest` as "Destination path relative to the output directory" and `src/manifest.rs:271` says the same of `Asset.dest`. No containment check exists in `src/manifest.rs`, and the write site, `apply_asset` (`src/main.rs:119`), is a bare `root.join(&asset.dest)`, which a `..` component escapes and an absolute path discards the root of entirely. The run then reports "Wrote to `<output-dir>`".

Principle 3 (Safe on existing projects) is the principle this breaks: scaffolding must never clobber or silently overwrite existing files, and a write the tool reports as landing inside the output directory while it lands outside is that failure in its least visible form.

REPRODUCTION (written by this planning pass; the audit record carries none). Two shapes, both measured at `a4394e4`:

```
mkdir -p /tmp/f4/pack /tmp/f4/out
printf 'x\n' > /tmp/f4/pack/escape.md
printf '[[asset]]\nsource = "escape.md"\ndest = "../../escaped-outside.md"\nownership = "working"\n' > /tmp/f4/pack/pack.toml
cargo run -- scaffold --template /tmp/f4/pack --output-dir /tmp/f4/out --vcs none --write
```

Measured: exit 0, the plan line reads `create  ../../escaped-outside.md`, the summary reads "Wrote to /tmp/f4/out (1 changed, 0 left untouched)", and the file lands at `/tmp/escaped-outside.md`. Replacing the `dest` with an absolute path reproduces the same result with the absolute destination printed verbatim.

ACCEPTANCE CRITERIA for `F4`, each executable:

1. Red then green on both shapes above. Before the fix each writes outside `--output-dir` at exit 0; after it each is refused at a non-zero exit, the message names the offending `dest`, and nothing is written anywhere, inside or outside. Both land as tests.
2. The refusal happens when the manifest is loaded, before any write and before the plan preview is printed, so `--dry-run` cannot preview an escaping asset either and the preview never promises a write the action refuses.
3. The predicate is not authored twice. `is_safe_sidecar_ref` (`src/plan/source.rs:488`) already rejects an absolute path and any `..` component, for this exact reason, and is private to `plan::source`. The fix either lifts it to a shared home or records why a second copy is correct (Principle 1, Prefer the cleaner long-term architecture over the smallest diff).
4. Nothing else moves: `cargo test` passes with the built-in asset list at `src/manifest.rs:611` unchanged, and a normal `scaffold` run drops the same set of files it dropped before.

MAKE THE CLAIM TRUE RATHER THAN DELETE IT, recommended. Deleting the claim is cheaper and is what `src/checks.rs:17-26` does for `.agents/checks.toml`, arguing the trusted-config boundary explicitly. That argument does not carry over: a `.agents/checks.toml` is authored by the user in their own repository, whereas `--template` names a pack the user may have fetched from anywhere, so the pack is external input rather than the user's own configuration. Principle 3 (Safe on existing projects) settles it, and rejecting at the boundary rather than guarding at the write is Principle 5 (Make illegal states unrepresentable). If the implementer takes the delete option instead, the doc comments at `src/manifest.rs:46` and `:271` must stop saying "relative to the output directory" and the README must state the real contract, so the step still closes the gap between what is claimed and what holds.

### Defect `F4b`: a pack `source` can read outside the pack directory

NOT an audit finding. It was found during this step's own implementation of `F4`, reported rather than fixed, and admitted here by `Q-75` (human, 2026-08-13), which is the one authorised widening of the scope above. The number carries `4` because it is the read-side twin of `F4`, in the same struct and closed by the same predicate, and the letter because the audit's `F1` to `F4` numbering is a closed record this defect is not part of.

Verified against the source at `d06f1b5`, whose `src/` is identical to `a4394e4`. `src/manifest.rs:44` documents `AssetSpec.source` as "Path of the source file within the pack". Nothing enforces that. The read site, `PackSource::Directory::read` (`src/manifest.rs:307`), is a bare `fs::read_to_string(root.join(rel))`, which a `..` component escapes and an absolute path discards the root of entirely, exactly as `apply_asset` does on the write side. Two callers pass pack-controlled text to it: `src/manifest.rs:532` (`spec.source`, once per asset) and `src/manifest.rs:433` (`module.guidance`, once per enabled module). The three literal callers (`pack.toml`, `principles.toml`, `instrument.md`) pass fixed strings and are not affected.

Principle 5 (Make illegal states unrepresentable) is the principle this breaks: a doc comment declares the state illegal and no code makes that true, which is the admit-then-guard shape the principle exists to prevent, except that here nothing guards either. Principle 1 (Prefer the cleaner long-term architecture over the smallest diff) is what settles it inside this step rather than after it: once `F4` lands, `AssetSpec` enforces containment on `dest` and not on `source`, which is an incoherent boundary rather than a smaller one.

REPRODUCTION (measured by this planning pass at `d06f1b5`; paths shortened for legibility, the run used a scratch directory):

```
mkdir -p /tmp/f4b/pack /tmp/f4b/out
printf 'TOP SECRET\n' > /tmp/f4b/secret.md
printf '[[asset]]\nsource = "../secret.md"\ndest = "leaked.md"\nownership = "working"\n' > /tmp/f4b/pack/pack.toml
cargo run -- scaffold --template /tmp/f4b/pack --output-dir /tmp/f4b/out --vcs none --write
```

Measured: exit 0, the plan line reads `create  leaked.md`, the summary reads "Wrote to /tmp/f4b/out (1 changed, 0 left untouched)", and `/tmp/f4b/out/leaked.md` contains `TOP SECRET`. An absolute `source` reproduces the same result. Two differences from `F4` matter for the acceptance criteria. First, the `dest` is legal, so the output names only `leaked.md` and the leak is invisible in what the run reports. Second, `--dry-run` in place of `--write` prints the same `create  leaked.md` at exit 0, and the outside file has already been read by then, because `load` (`src/manifest.rs:532`) reads the contents into `Asset.contents` before any plan line is printed.

ACCEPTANCE CRITERIA for `F4b`, each executable:

1. Red then green on both shapes, the `..` and the absolute. Before the fix each copies an outside file into the scaffolded project at exit 0; after it each is refused at a non-zero exit, the message names the offending `source`, and nothing is written. Both land as tests.
2. `--dry-run` is refused too, and the outside file is never opened, not merely never written. This is `F4`'s criterion 2 on the read side, and here it is the whole of the defect rather than a preview detail: the read is the leak.
3. One predicate, one site. The containment predicate `F4`'s fix introduces, named `safe_path::is_contained_relative` on the implementer's branch as of this writing and absent at `d06f1b5`, is applied at the `PackSource::Directory` arm of `src/manifest.rs:307`, so `spec.source` and `module.guidance` are both covered by one application (Principle 1, Prefer the cleaner long-term architecture over the smallest diff). A fix that checks `spec.source` alone leaves `module.guidance` open and does not close this defect. `PackSource::Embedded` needs no check and gets none: its paths are compile-time and its lookup touches no filesystem. If the implementer instead rejects at manifest-parse time, the outcome must say how `module.guidance` is covered, since it is not an `[[asset]]` field.
4. Nothing else moves: `cargo test` passes with the built-in asset list unchanged, and a normal `scaffold` run drops the same set of files it dropped before.

FIX DIRECTION is settled by `Q-75` and no alternative is open. `F4`'s delete-the-claim option does not exist here: a `--template` pack is external input the user may have fetched from anywhere, so the `src/checks.rs:17-26` trusted-config argument does not carry over, and `Q-75` records the human rejecting the document-the-boundary option on that ground.

### Release mechanics

Ordering matters here, because the README's link has to resolve when the crate is published:

1. Land the three fixes (`F1`, `F4`, `F4b`) and regenerate the projection.
2. Set `Cargo.toml` `version` to `0.0.2` and close the `CHANGELOG.md` `## [Unreleased]` section as `## [0.0.2]`, dated.
3. Reserve `agent-flow` on crates.io at 0.0.2, so the README link has a target.
4. Add the two README items. crates.io treats `-` and `_` as one name, so reserving `agent-flow` also reserves `agent_flow`.
5. Publish `agent-scaffold` 0.0.2. Leave every earlier `agent-scaffold` version un-yanked.

One detail the `Q-65` checklist leaves open and this step must settle rather than assume: the checklist says to contact by opening an issue on the "`agent-flow` GitHub repository", which does not exist under that name until `rename-to-agent-flow` runs. The cheapest resolution is for the 0.0.2 README to name the current repository as the one that will carry the rename, and to link the crates.io `agent-flow` page for the crate itself. Record what was done.

ACCEPTANCE CRITERIA for the release half:

1. `cargo publish --dry-run` succeeds for `agent-scaffold` at 0.0.2.
2. The published README (the one in the 0.0.2 artifact, not only the one on the default branch) contains a link to `agent-flow` and a sentence stating the `agent-scaffold` name is reclaimable, with the contact route.
3. `agent-flow` resolves on crates.io at 0.0.2, and `agent-scaffold` 0.0.1 is still installable.
4. The seven gates are green on the release commit: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `validate --source ... --metrics ...`, `validate --source ... --workflow`, `render --check --strict`, the ASCII check `LC_ALL=C grep -cP '[^\t\x20-\x7e]'` returning 0 on every changed file, and `cargo publish --dry-run`.
