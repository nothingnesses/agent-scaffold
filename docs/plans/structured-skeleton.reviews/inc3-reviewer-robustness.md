# Inc 3 reviewer findings: adversarial robustness, failure-atomicity, path/input safety

Reviewer lens: writes-nothing-on-failure atomicity, clobber safety, path/input safety
(Principle "never trust external input; validate and parse it at the boundary"),
determinism, table-cell injection, panic-freeness.

Change reviewed: `17a328e` on base `9afe567`. Read via `git show 17a328e:src/plan/render.rs`,
`:src/main.rs`, `:src/plan/source.rs`, `:src/plan.rs`. Experiments run read-only in the
worktree build against crafted plans in a scratch temp dir (never the committed fixtures).

Build/test state: `cargo test --all-targets` passes (275 tests, not the 276 claimed; minor,
see note N1). `cargo clippy --all-targets -- -D warnings` is clean.

## Headline

The core "writes NOTHING on a schema violation / unresolved cross-reference / missing sidecar"
atomicity HOLDS. `render_plan` is pure (returns a `String`, writes nothing); the CLI writes
only after both `render_plan` and `rendered_path` return `Ok`, and all validation and every
sidecar read happen fully in memory before that single `fs::write`. A pre-existing `<task>.md`
is left byte-identical when render fails validation (proven: md5 unchanged). Clobber safety
holds: render opens the TOML and the sidecars for READ only and writes exactly one path,
`<task>.md`.

Findings: one medium (sidecar path traversal, R1), two low (R2 carriage-return table split,
R3 non-atomic successful write). No high or critical.

## R1 (medium): `[meta].sidecars` refs are unvalidated external input -> path traversal / absolute-path read outside the plan directory

`file: src/plan/render.rs:164-167` (and `validate_source` in `src/plan/source.rs:371-630`,
which never inspects `meta.sidecars`).

The front/tail sidecar references are free strings joined straight onto the plan's base dir:

```
let front_blobs = plan.meta.sidecars.front.iter().map(|reference| load(&base.join(reference)))...
let tail_blob   = plan.meta.sidecars.tail.as_ref().map(|reference| load(&base.join(reference)));
```

`validate_source` validates step slugs (kebab), question ids (`Q-<n>`), increment/waiver ids,
principle numbers, and the cross-references, but it NEVER validates `meta.sidecars.front` or
`meta.sidecars.tail`. `Path::join(reference)` with a `..`-bearing relative ref escapes the base
dir, and with an ABSOLUTE ref it discards the base entirely. So a crafted `.plan.toml` makes
render read any file the process can read and splice its bytes verbatim into `<task>.md`.

The other path-derived reads are safe by contrast: step sidecars use `steps_dir.join(format!("{}.md", step.slug))`
and question bodies use the `Q-<n>` id, both already constrained by `validate_source` to
contain no `/` or `..`. Only the front/tail refs are an unconstrained hole.

Repro (proven in the worktree build):

```
# plandir/evil.plan.toml
[meta]
title = "Traversal probe"
[meta.sidecars]
front = ["../secret/passwd.txt"]        # relative traversal, escapes plandir/
tail  = "/abs/path/secret/abs.txt"      # absolute, ignores base entirely
```

`agent-scaffold render plandir/evil.plan.toml` exits 0 and the generated `plandir/evil.md`
contains, verbatim, the body of `../secret/passwd.txt` (spliced as a front section) and the
body of the absolute-path file (spliced as the tail). Both files live OUTSIDE `plandir/`.

Impact: read-only (render never writes outside `<task>.md`, so this is not an out-of-dir write
or a clobber), and the `.plan.toml` is an in-repo source, which caps severity at medium today.
But it defeats the banner's own stated model ("names the real sources by their task-relative
names ... deterministic regardless of where the repository is checked out") and violates the
external-input-at-the-boundary principle: a `.plan.toml` obtained from an untrusted source
(note the planned `git-url-fetch` feature) could exfiltrate arbitrary file contents into a
committed, agent-read artifact, or make the render machine-dependent by pulling in `/etc/...`.

Direction: validate the front/tail refs at the boundary, in `validate_source` (so `render`,
`render --check`, and `validate --source` all reject the same thing). Reject any ref that is
absolute or whose components contain `..` (a `.plan.toml`'s sidecars are by contract
task-relative and single-segment or under the task tree). Alternatively canonicalize
`base.join(ref)` and assert the result stays within a canonicalized `base`, but the
component-level reject is simpler and does not depend on the files existing yet. Consider the
same rule for the step/question sidecar convention for defense in depth, though those are
already slug/id-constrained.

## R2 (low): `escape_cell` neutralizes `|` and `\n` but not `\r`, so a waiver `note` carriage return can split a Roadmap table row

`file: src/plan/render.rs:476-478`

```
fn escape_cell(text: &str) -> String {
    text.replace('|', "\\|").replace('\n', " ")
}
```

The only free-text input reaching a table cell is a waiver `note` (via `waiver_note` ->
`notes_cell` -> `escape_cell`); slugs are kebab-validated and status/reason/tier are enums, so
they cannot carry a `|` or a newline. A `note` containing a lone `\r`, or a `\r\n` (whose `\n`
is turned into a space, leaving a stray `\r`), passes through unescaped. CommonMark treats a
lone carriage return as a line ending, so a compliant renderer splits the Roadmap row at the
`\r`, breaking the table (or, with crafted content, injecting a fake row).

Repro (proven): a step with `note = "pipe | here\nnewline row\rCRrow | end"` renders the cell as
`... waived: step predates-logging (self-declared) - pipe \| here newline row^MCRrow \| end`.
The `|` is escaped and the `\n` is a space, but the `^M` (CR) survives in the cell.

Impact: low. The `note` is in-repo TOML, and the visible damage is a broken table in a
generated file that `render --check` would still round-trip byte-for-byte (so CI catches drift,
not this). But the escape exists precisely to make cell content structural-injection-proof, and
it is incomplete.

Direction: also replace `\r` (and `\r\n`) with a space, e.g. normalize `\r\n` and lone `\r` to
`\n` first, then replace `\n` with a space; or `text.replace(['|'], "\\|").replace(['\n','\r'], " ")`.

## R3 (low): the successful write is non-atomic (`fs::write` truncate-then-write), so an interrupted write can corrupt a previously-good `<task>.md`

`file: src/main.rs` `run_render` non-check branch: `fs::write(&out, rendered)?`

The writes-nothing-on-FAILURE contract holds for the failures it names (schema / xref / missing
sidecar): those are all detected in memory before any write, and a pre-existing `<task>.md` is
left byte-identical (verified). The remaining gap is the SUCCESS path: `fs::write` creates or
TRUNCATES `<task>.md` and then streams the bytes. If that write is interrupted (disk full,
process killed, permissions change mid-write), the previously-valid `<task>.md` has already been
truncated and is left partial or empty. An agent could then read a truncated-but-present plan as
authoritative, and `render --check` would report a mismatch for a file that was correct moments
earlier. This is the "what happens if the final write itself fails" the brief asks about.

Impact: low (requires an I/O failure at the write, not a crafted input), and no partial write
occurs on any of the validated failure modes.

Direction: write to a temp file in the same directory and `fs::rename` it into place (atomic
replace on the same filesystem), so an interrupted write leaves the prior `<task>.md` intact.
This also matches the increment's own "a broken source never yields a partial plan" intent,
extending it to a broken write.

## Attacks tried and RULED OUT (no finding)

- Writes-nothing-on-failure (the central contract): HOLDS. `render_plan` returns a `String` and
  writes nothing; validation (`validate_source`), the parse, and EVERY sidecar read run before
  the CLI's single `fs::write`. Missing sidecar, dangling xref, and schema-invalid TOML each
  exit 1 with no `<task>.md` produced, and a pre-existing `<task>.md` is left byte-identical
  (md5 unchanged across a failed render). No truncate-then-fail path exists for these modes.
- Clobber safety: HOLDS. The TOML and sidecars are opened only via `fs::read_to_string`; the
  only write is `fs::write(rendered_path, ...)` = `<task>.md`. No path writes a sidecar or the
  `.plan.toml`.
- Slug / id path traversal: BLOCKED. Step sidecars key on `step.slug` (kebab-validated: no `/`,
  no `..`, no uppercase) and question bodies on `Q-<n>` (validated), so those joins cannot
  escape. Only the free-string front/tail refs escape (R1).
- Determinism under environment: HOLDS. `render` uses no `fs::read_dir` (the only `read_dir` is
  in a test helper), no `HashMap`/`HashSet` iteration, and no absolute paths in output. Steps
  sort by `order` then `slug`, questions by `Q-<n>` index, principles by `n`, statuses by
  `StepStatus::ALL`, waiver reasons by a fixed list, front sidecars in declared order. Same
  inputs -> same bytes on another machine, so `render --check` in CI is stable.
- task_name / output-name derivation: robust and panic-free. `.plan.toml`-suffix miss ->
  usage error; bare `.plan.toml` (empty task) -> usage error; a bare filename -> base `"."`;
  a path ending in `..` -> `file_name()` is `None` -> "not a readable file name" error;
  multi-dot names strip only the suffix. The derived task has no `/`, so `base.join(task.steps)`
  cannot traverse. All handled as clean errors, no panic.
- Panic-freeness: no `unwrap`/`expect`/indexing/slicing on parsed or filesystem data in the
  render path. `question_id_index` returns `Option` (used post-validation, never unwrapped in a
  panicking way); `folded_into`/`superseded_by` use `as_deref().unwrap_or("")`; `truncate` uses
  `chars().take(..)`. A sidecar that is a directory or an empty-string ref yields a clean
  "missing or unreadable sidecar" error, not a panic (verified).
- Banner / section spoof via sidecar prose: not a defeat of the guard. The real do-not-edit
  banner is section 0 (line 1); a sidecar echoing a fake banner or a fake `## Roadmap` is
  spliced verbatim, becomes part of the committed golden, and re-renders identically, so
  `render --check`'s byte compare is unaffected. Acceptable under the trusted-source model.
- Table injection via slug / status / blocked_by: BLOCKED. Those cells are kebab slugs or enum
  labels, none of which can contain `|` or a newline; only the free-text waiver `note` is a
  vector, and only via `\r` (R2).

## Note

- N1: the brief claims 276 tests; `cargo test --all-targets` reports 275 passing (271 + 1 + 3).
  Not a defect, just a count to reconcile.
