# `ship-v0-0-2-inc1` round 1: CLAIMS AND CONTRACT review

Reviewer lens: does what the change SAYS match what it DOES, and does it meet the criteria it was written against. Independent of the mechanism-by-construction review.

Note on this file's history: my first pass through this review was written to this path, then lost to an unrelated worktree-removal accident before it was harvested (not caused by anything in this review). This is a full rewrite from my own transcript of that pass, plus one addendum (marked below) added after the rewrite request, which reports a defect I verified myself with a fresh reproduction rather than one carried over from memory.

## Artifact reviewed

`git diff main..HEAD` in `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-contract`, a detached checkout at `f86e529d9b2c6535df5a084c1894392652573088`. Five commits (chronological):

1. `5bad30b` fix: refuse a pack dest that leaves the output directory (`F4`)
2. `703a2e3` fix: keep every interpolated free-text value on one generated line (`F1`)
3. `10694e1` chore: release 0.0.2
4. `d8aa12a` fix: refuse a pack asset source that leaves the pack directory (`F4b`, initial)
5. `f86e529` fix: contain every pack-controlled path at the shared read site (`F4b`, final: moves the check into `PackSource::read`)

Specification: `docs/plans/agent-scaffold.steps/ship-v0-0-2.md`.

12 files changed, 961 insertions, 137 deletions. All 450 tests pass (`cargo test`), `cargo clippy --all-targets -- -D warnings` is clean, and `cargo publish --dry-run` succeeds for `agent-scaffold` 0.0.2.

## Acceptance criteria, executed

| # | Criterion (paraphrased) | Command(s) run | Measured result |
|---|---|---|---|
| F1.1 | Red-then-green on the reproduction; lands as a test | `cp -r src/plan/testdata/render-fixture.* <scratch>/f1/`; edited `Q-1`'s `ask` to a two-line value whose second line reads `` - `Q-42` (undecided) a queue item nobody authored.``; then `cargo run -- validate --source <scratch>/f1/render-fixture.plan.toml --metrics /dev/null`, `cargo run -- render <scratch>/f1/render-fixture.plan.toml`, `cargo run -- validate --plan <scratch>/f1/render-fixture.md --metrics /dev/null` | MET. `validate --source`: `7 steps, 5 questions, valid`, exit 0. `render`: exit 0. `validate --plan` on the fresh render: reports only the fixture's own pre-existing `` Open Questions item `Q-3` has an unknown status `superseded by `Q-1``  `` line, exit 1; no mention of `Q-42` anywhere. Rendered line confirmed by `grep`: `` - `Q-1` (open) An open ask still awaiting a decision. - `Q-42` (undecided) a queue item nobody authored.`` on one line. Pinned as `plan::render::tests::a_multi_line_ask_cannot_fabricate_a_queue_item` (`src/plan/render.rs:1170`). |
| F1.2 | All four interpolation sites covered by one test, each field carrying a line ending, a `\|`, and a leading `- ` marker; asserts line count and structure | `cargo test --bin agent-scaffold plan::render::` | MET. `plan::render::tests::every_interpolated_free_text_site_stays_on_one_generated_line` (`src/plan/render.rs:1198`) exercises the title, one principle's name+text, one question's ask, and one `[[step.waiver]]` note in the Roadmap table, asserting exact line counts (1 heading line, 1 numbered principle line, 1 queue item, 3 table lines: header/delimiter/one row) and that none of the five injected `` - `Q-9x` `` fragments starts a new line. Test passed. |
| F1.3 | `render --check --strict docs/plans/agent-scaffold.plan.toml` reports up to date after regeneration | `cargo run -- render --check --strict docs/plans/agent-scaffold.plan.toml`; then `git show main:docs/plans/agent-scaffold.md \| tr -s ' \n' ' '` vs `git show HEAD:docs/plans/agent-scaffold.md \| tr -s ' \n' ' '`, both piped to `wc -c`, then `diff` | MET. `docs/plans/agent-scaffold.plan.toml: up to date`, exit 0. Whitespace-normalised `main` and `HEAD` projections are byte-identical: **827264 bytes each, zero-length `diff` output.** Method: `tr -s ' \n' ' '` collapses all runs of spaces and newlines to single spaces in both files, so the comparison is blind to exactly the kind of change this fix makes (line-ending-to-space neutralisation, i.e. re-wrapping where the paragraph breaks fall) while remaining sensitive to any actual word being added, removed or reordered. Equal byte counts plus an empty `diff` on the normalised text is direct evidence that no content word was lost when a multi-paragraph `ask` (for example `Q-52`'s, which visibly collapses from several blank-line-separated paragraphs to one queue line in the raw diff) collapsed to one line, confirming the CHANGELOG's "no text is lost" claim by comparison rather than by reading the diff and trusting it. |
| F1.4 | `validate --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl` reports EXACTLY ONE problem, the pre-existing `Q-43`/`Q-44` "superseded by" vocabulary mismatch | `cargo run -- validate --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl` | MET, exact text match: `` docs/plans/agent-scaffold.md: Open Questions item `Q-43` has an unknown status `superseded by `Q-44`` ``, exit 1, no other line. |
| F4.1 | Red-then-green on both shapes (`..` and absolute `dest`); refused non-zero, message names the `dest`, nothing written; lands as tests | Built a one-asset pack with `dest = "../../escaped-outside.md"`, then with an absolute `dest`; ran `cargo run -- scaffold --template <pack> --output-dir <out> --vcs none --write` for each | MET. Both refused at exit 2: `` error: asset `escape.md` has dest `<dest>`, which leaves the output directory; a dest must be a relative path with no `..` component ``. Nothing written inside or outside the output dir (checked with `find`/`ls`). Pinned in `manifest::tests::an_escaping_dest_is_refused_at_load` (`src/manifest.rs:930`) and integration-tested in `tests/pack_dest_stays_inside_the_output_dir.rs`. |
| F4.2 | Refused at load, before any write and before the plan preview, so `--dry-run` also refuses | Same two packs, `--dry-run` instead of `--write` | MET. Identical refusal message and exit 2 for both shapes; no `create` line and no `Wrote to` line in stdout. |
| F4.3 | Predicate not authored twice; `is_safe_sidecar_ref` lifted to a shared home or the duplication justified | `git diff main..HEAD -- src/plan/source.rs` | MET. `is_safe_sidecar_ref` (`src/plan/source.rs:480`) now delegates to `crate::safe_path::is_contained_relative` (`src/safe_path.rs:23`), the same predicate `manifest::load` uses for `dest` (`src/manifest.rs:638`) and `PackSource::read` uses for `source`/`guidance` (`src/manifest.rs:431`). One predicate, three call sites. |
| F4.4 | Nothing else moves: built-in asset list unchanged, normal `scaffold` run drops the same files | `cargo test --bin agent-scaffold manifest::` (includes `builtin_manifest_lists_the_expected_assets`); `git diff main..HEAD -- src/manifest.rs \| grep -c '^-.*"docs/\|^-.*"\.agents/'`; exported `main` via `git archive main \| tar -x` into a scratch dir, built it, ran `cargo run -q -- scaffold --output-dir <out-main> --vcs none --module checks --module isolation --write` there and the equivalent against `HEAD` into `<out-head>`, then `diff <(find <out-main> -type f \| sort) <(find <out-head> -type f \| sort)` and `diff -rq <out-main> <out-head>` | MET. Test passed; the asset-list grep returned `0` (no built-in asset string literal removed). Both scaffold runs dropped an identical 35-file listing (`35 changed, 0 left untouched` on both); `diff` of the sorted file lists was empty; `diff -rq` reported zero content differences. |
| F4b.1 | Red-then-green on both shapes (`..` and absolute `source`); refused non-zero, message names the `source`, nothing written; lands as tests | Built a one-asset pack with `source = "../secret.md"`, then with an absolute `source`; ran `scaffold --write` for each | MET **for the two shapes the spec's own reproduction and this AC name**. Both refused at exit 2: `` error: asset source `<source>` leaves the pack directory; a source must be a relative path with no `..` component ``; nothing landed in the output dir. Pinned in `manifest::tests::an_escaping_source_is_refused_at_load` (`src/manifest.rs:962`) and `tests/pack_source_stays_inside_the_pack.rs`. See the addendum below: a third shape, outside this AC's literal wording, is NOT refused. |
| F4b.2 | `--dry-run` refused too, outside file never opened (not merely never written) | Same two packs, `--dry-run`; separately, a pack whose `..`-relative `source` names a file that does not exist anywhere, `--write` | MET **for the two named shapes**. `--dry-run` produces the identical refusal for both. The nonexistent-file probe still reports the containment refusal rather than a not-found I/O error, which is only possible if the containment check runs strictly before the filesystem is touched, confirming "never opened" rather than merely "never written" for these shapes. See the addendum: this guarantee does not extend to the symlink shape. |
| F4b.3 | One predicate, one site, at the `PackSource::Directory` arm of `read`, covering both `spec.source` and `module.guidance` | Read `src/manifest.rs:429-436`, `:566-571`, `:689-693`; ran the `--module evil --write` guidance-escape reproduction | MET for the `..`/absolute shapes on both fields. `PackSource::read`'s `Directory` arm is the sole call to `is_contained_relative` for read-side paths; `manifest::load` maps its `ReadError::Escapes` to `LoadError::UnsafeAssetSource`, and `module_guidance` maps it to `LoadError::UnsafeModuleGuidance`. Both refuse for the tested shapes. The addendum below shows this same single site is where the symlink gap lives, for both fields identically. |
| F4b.4 | Nothing else moves | Same combined `scaffold --module checks --module isolation --write` comparison as F4.4 | MET, same evidence as F4.4. |
| Release.1 | `cargo publish --dry-run` succeeds for `agent-scaffold` 0.0.2 | `cargo publish --dry-run` | MET. `Packaged 392 files, 5.9MiB (1.8MiB compressed)`, `Verifying agent-scaffold v0.0.2`, `Uploading agent-scaffold v0.0.2`, `warning: aborting upload due to dry run`, exit 0. |
| Release.2 | The published README (0.0.2 artifact) links `agent-flow` and states `agent-scaffold` is reclaimable, with a contact route | `grep -n "agent-flow\|reclaim\|github.com/nothingnesses" target/package/agent-scaffold-0.0.2/README.md` (the tree `cargo publish --dry-run` packages) | MET, via the packaged artifact as the best available proxy for "published" (no real publish has happened; see Release.3). `target/package/agent-scaffold-0.0.2/README.md:7-11` matches the working-tree `README.md:7-11`: links `https://crates.io/crates/agent-flow`, states "The `agent-scaffold` name is then free for whoever wants to reclaim it", and gives the contact route (an issue on the current GitHub repo, named as the one that will carry the rename, which is what the ship-v0-0-2 spec's own release-mechanics section directs since the `agent-flow` repo does not exist yet). |
| Release.3 | `agent-flow` resolves on crates.io at 0.0.2, `agent-scaffold` 0.0.1 still installable | Not run | UNVERIFIABLE by me. Both halves depend on crates.io actions (reserving `agent-flow`, then publishing `agent-scaffold` 0.0.2 for real) that the human has not yet taken. I did not attempt to guess an outcome or simulate one. |
| Release.4 | Seven gates green on the release commit | `cargo test`; `cargo clippy --all-targets -- -D warnings`; `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/workflow.jsonl`; `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow`; `cargo run -- render --check --strict docs/plans/agent-scaffold.plan.toml`; `git diff --name-only main..HEAD \| while read f; do LC_ALL=C grep -cP '[^\t\x20-\x7e]' "$f"; done`; `cargo publish --dry-run` | MET, all seven, run locally against `HEAD`: `cargo test` (450 passed, 0 failed); clippy clean; `validate --metrics` (`328 records, valid`; `99 steps, 75 questions, valid`); `validate --workflow` (`workflow invariants hold`); `render --check --strict` (`up to date`); ASCII check returned `0` on all 12 changed files; `cargo publish --dry-run` per Release.1. |

16 of 16 executable criteria met AS LITERALLY WRITTEN; one (Release.3) correctly out of my reach, as anticipated. The addendum below narrows what "met" means for `F4b`'s claims beyond the AC's own two named shapes.

## Verdict table

| # | Finding | Severity |
|---|---|---|
| 1 | Asset `source` containment is checked only for assets whose module is enabled; an escaping `source` on an unselected module's asset loads silently (no live leak, since the asset is never read) | Low |
| 2 (addendum) | The containment predicate is a check on the path STRING; `PackSource::read`'s filesystem read follows symlinks, so a pack shipping a plain-looking `source` or `guidance` filename that is actually a symlink to a file outside the pack still reads and leaks that file's contents. Falsifies the CHANGELOG's "a pack path can no longer read outside the pack directory" claim and the `AssetSpec.source`/`ModuleSpec.guidance` doc comments' "the within-the-pack claim holds" language | High |

## Finding 1: `source` containment is selection-scoped; `dest` containment is not, and the adjacent comment invites the wrong inference (Low)

**Evidence.** `manifest::load` (`src/manifest.rs:606-704`) checks `dest` containment for every declared `[[asset]]`, in the loop at `src/manifest.rs:624-643`, which runs BEFORE the module-enabled filter and is explicitly commented "Checked for every declared entry regardless of selection, like the module tag below, since an escaping dest is a pack-authoring error either way" (`src/manifest.rs:630-632`). `source` containment, by contrast, is reached only inside the `.filter(|spec| ...).map(|spec| { source.read(&spec.source) ... })` chain at `src/manifest.rs:678-694`, where the filter (`src/manifest.rs:681-685`) drops any asset whose module is not enabled BEFORE `source.read` (and therefore the containment check inside it) is ever reached.

Reproduced:

```
printf 'TOP SECRET\n' > /tmp/.../f4b_unselected/secret.md
cat > /tmp/.../f4b_unselected/pack/pack.toml <<'EOF'
[[module]]
name = "extras"
description = "d"

[[asset]]
source = "a.md"
dest = "a.md"
ownership = "working"

[[asset]]
source = "../secret.md"
dest = "leaked.md"
ownership = "working"
module = "extras"
EOF
cargo run -- scaffold --template /tmp/.../f4b_unselected/pack --output-dir /tmp/.../f4b_unselected/out --vcs none --write
```

Measured: exit 0, only `a.md` created, no refusal and no mention of the escaping `extras` asset at all. Selecting the module (`--module extras`) DOES correctly refuse: `error: asset source \`../secret.md\` leaves the pack directory; ...`, exit 2. So the escape is never actually read while its module stays unselected (matching `PackSource::read`'s doc claim that nothing reaches the filesystem outside the check), meaning there is no live leak in either state; the gap is in EARLY detection, not in safety.

The comment directly above the `dest` check (`src/manifest.rs:634-637`) reads: "The matching READ-side rule is not repeated here. A `source` reaches the filesystem only through `PackSource::read`, which contains it there for every caller, so a second check at this site would be the same predicate applied twice and could drift from the one that actually guards the read." This is true taken narrowly (every actual invocation of `read` is protected), but it sits two sentences below the `dest` comment's "regardless of selection" claim, in the same paragraph, about the same struct's twin fields, and a maintainer skimming both comments together could reasonably conclude the two fields are checked with the same coverage. They are not: a pack author who ships a broken or malicious `source` behind an unselected `[[module]]` gets no `pack.toml`-authoring feedback at all, ever, unless and until someone selects that module, whereas the same author gets IMMEDIATE feedback for the equivalent `dest` mistake regardless of selection.

**Why Low, not Medium.** No exercised code path actually reads a file outside the pack in this scenario (verified above); this does not change any of the sixteen acceptance criteria's verdicts, all of which are scoped to the CORE-asset reproduction the spec describes, and F4b's own text never claims parity with `dest`'s "regardless of selection" behaviour. Per this project's calibration a defect that cannot change a verdict does not reach `medium`; this is a comment-accuracy gap that could send a maintainer to the wrong conclusion about coverage, which keeps it at `low` rather than dropping it to no finding at all.

## Finding 2 (addendum): the containment check is string-only and `PackSource::read` follows symlinks, so a symlinked pack path still reads outside the pack (High)

**How this entered the review.** This finding was not in my first pass. After that pass's findings file was lost and I was asked to rewrite it, the coordinator reported that a parallel adversarial-lens review had found this bypass and asked me to consider whether it bore on my `low` rating for Finding 1. Rather than transcribe that report, I reproduced it myself, independently, in this worktree, before writing it up here. Everything below is my own measurement.

**The mechanism.** `is_contained_relative` (`src/safe_path.rs:19-24`) is, by its own doc comment, explicitly "a check on the STRING": it inspects `Path::new(reference).components()` and rejects only an absolute root or a `..` component. A plain filename such as `link.md`, with no `..` and no leading `/`, passes unconditionally, REGARDLESS of what that path resolves to on disk. `PackSource::read`'s `Directory` arm (`src/manifest.rs:429-436`) runs this string check and then calls `fs::read_to_string(root.join(rel))`, which is an ordinary filesystem read: if `root.join(rel)` is a symlink, the OS follows it to its target, inside or outside the pack, without agent-scaffold ever being asked.

**Reproduction, asset `source`:**

```
mkdir -p <scratch>/symlink-verify/pack <scratch>/symlink-verify/out
printf 'TOP SECRET VIA SYMLINK\n' > <scratch>/symlink-verify/secret.md
ln -s <scratch>/symlink-verify/secret.md <scratch>/symlink-verify/pack/link.md
printf '[[asset]]\nsource = "link.md"\ndest = "leaked.md"\nownership = "working"\n' > <scratch>/symlink-verify/pack/pack.toml
cargo run -- scaffold --template <scratch>/symlink-verify/pack --output-dir <scratch>/symlink-verify/out --vcs none --write
```

Measured: exit 0, stdout `          create  leaked.md` then `Wrote to <out> (1 changed, 0 left untouched).`, and `<out>/leaked.md` contains `TOP SECRET VIA SYMLINK`. Re-run with `--dry-run` in place of `--write`: identical `create leaked.md` line at exit 0 (the read already happened by then, same as the `..`/absolute shapes; only the write to disk is skipped).

**Reproduction, module `guidance`:**

```
mkdir -p <scratch>/symlink-guidance/pack <scratch>/symlink-guidance/out
printf 'TOP SECRET GUIDANCE VIA SYMLINK\n' > <scratch>/symlink-guidance/secret-guidance.md
ln -s <scratch>/symlink-guidance/secret-guidance.md <scratch>/symlink-guidance/pack/guidance-link.md
cat > <scratch>/symlink-guidance/pack/pack.toml <<'EOF'
[[module]]
name = "evil"
description = "d"
guidance = "guidance-link.md"

[[asset]]
source = "body.md"
dest = "body.md"
ownership = "working"
render = true
EOF
printf 'before\n{{modules}}\nafter\n' > <scratch>/symlink-guidance/pack/body.md
cargo run -- scaffold --template <scratch>/symlink-guidance/pack --output-dir <scratch>/symlink-guidance/out --module evil --vcs none --write
```

Measured: exit 0, `<out>/body.md` reads:

```
before
TOP SECRET GUIDANCE VIA SYMLINK


after
```

confirming the symlinked guidance file's contents were spliced into `{{modules}}`.

**What this falsifies.** The CHANGELOG's new "Fixed" bullet (`CHANGELOG.md:32`) states: "A pack path can no longer read outside the pack directory... Rather than a check per field, the containment is applied once at `PackSource::read`, the single site every pack path reaches the filesystem through, so a later caller inherits it instead of having to remember it, and it refuses before the join is opened, so an escaping path is never read rather than merely never used." As measured above, a pack path CAN still read outside the pack directory, via a symlinked filename, and the join IS opened (the read happens; the string check never fires because the string itself never leaves the pack). The doc comments on `AssetSpec.source` (`src/manifest.rs:44-49`, "the within-the-pack claim holds rather than being merely documented") and `ModuleSpec.guidance` (`src/manifest.rs:96-100`, "so a guidance partial cannot splice a file from outside the pack into `{{modules}}`") make the same overclaim for the same reason. `F4b`'s own acceptance criteria are not violated by the letter (they name only the `..` and absolute shapes, both of which are genuinely closed), but the CHANGELOG's and the doc comments' claims are broader than the criteria that were checked, and that broader claim is false as shipped.

This also means my earlier statement, in the first pass of this review, that "every doc comment... [and] the CHANGELOG's three new entries... are true of the shipped code" was wrong for the `F4b` bullet and its two associated doc comments. I am correcting that here rather than leaving it standing.

**Severity: High.** This is a live wrong behaviour, not a documentation-only gap: an actual file outside the pack is actually read and actually spliced into the scaffolded output, exit 0, for both of `F4b`'s two consumer fields, in both `--write` and `--dry-run`, using an attack shape (a symlink shipped inside a fetched `--template` pack) that is at least as easy to construct as the `..`/absolute shapes the fix does close, and arguably easier to miss on manual inspection of a pack's `pack.toml` alone (the `pack.toml` shows an innocuous bare filename; the danger is in the pack's file tree, one `ls -la` away from invisible). Per this project's calibration, "a live wrong behaviour is high"; I am not aware of a documented bar for `critical` beyond that language, so I am reporting `high` as the best-supported reading rather than reaching further on my own judgement, but flag for the triager that this defeats the exact class of defect `F4b` exists to close, on the exact release meant to close it.

**What I did not test.** I did not check whether an analogous symlink shape affects `F4`'s `dest`/write side (for example, a pre-existing symlink inside `--output-dir` at the `dest` path); that is a different mechanism (the containment check there guards the `dest` STRING against escaping the output directory, not against following a link once inside it) and was outside what I reproduced. I also did not determine whether this is fixable by the same `is_contained_relative` predicate or needs a different check (for example, resolving the join and comparing canonical paths, or rejecting a pack asset whose resolved file type is a symlink); that is a mechanism/fix-direction question for the other lens and for the implementer, not mine to answer here.

## Out-of-scope observations

- The five commits' chronological order lands the version bump and CHANGELOG close (`10694e1`, "chore: release 0.0.2") BEFORE `F4b` is fixed (`d8aa12a`, `f86e529`): at `10694e1` alone, `Cargo.toml` already says `0.0.2` and the CHANGELOG's `## [0.0.2]` section lists only the `F1` and `dest` fixes (verified with `git show 10694e1:CHANGELOG.md`), with the pack-source-read escape still live in that commit's tree. This is not a defect in the artifact under review, since only `HEAD`, the tip that would actually be published, is in scope, and `HEAD`'s CHANGELOG and code agree (modulo Finding 2 above). Flagging only because a bisect or a partial cherry-pick landing on `10694e1` alone would produce a "0.0.2" build with `F4b` unfixed and a CHANGELOG silent about it; worth knowing if this history is ever rewritten or partially applied.
- The `Q-75` widening's own decision record and the `ship-v0-0-2` step's "Record what was done" instruction (for the `agent-flow` GitHub-repo substitution and for which `F1` fix direction was chosen) are workflow/ledger concerns, not something I found recorded or missing in the reviewed diff; I did not chase this further since round-log/ledger completeness is outside the CLAIMS AND CONTRACT lens as scoped to me.
- A methodological note, since the coordinator asked me to record it: my first pass validated the CHANGELOG's `F4b` claim as true by testing exactly the two shapes the spec's own reproduction and acceptance criteria name (`..`-relative and absolute). That is what a contract lens does well: it can confirm or falsify a stated claim against a stated test. It does not, by itself, generate the THIRD shape nobody had named yet; that came from a construction-first lens asking "what else could make this string safe-looking but the read unsafe" rather than "does the named test pass." Once told where to look, I could and did verify it directly rather than take it on trust, and the two lenses' outputs now agree because they were checked against the same running binary, not because either one deferred to the other.
