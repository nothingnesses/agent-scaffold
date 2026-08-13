# `ship-v0-0-2-inc1` round 1: TRIAGE

Independent of both reviewers, of the implementer, and of the orchestrator. Every figure below is my own measurement. Where it differs from a reviewer's, the difference is called out in place.

## Artifact and commits

`git diff main..HEAD` at `813fc02`, `main` at `c68f541`. Five commits:

1. `b32bf2b` fix: refuse a pack dest that leaves the output directory (`F4`)
2. `db9c0dd` fix: keep every interpolated free-text value on one generated line (`F1`)
3. `b3aad56` chore: release 0.0.2
4. `fb00404` fix: refuse a pack asset source that leaves the pack directory (`F4b`, initial)
5. `813fc02` fix: contain every pack-controlled path at the shared read site (`F4b`, final)

12 files changed, 961 insertions, 137 deletions.

NOTE ON COMMIT IDENTITY. Both reviewers reviewed the same tree under different hashes (`f86e529`, and `5bad30b`/`703a2e3`/`10694e1`/`d8aa12a` for the earlier four). The worktree I was given is detached at `813fc02` with an identical diff against the same `main`. The trees agree; only the hashes were rewritten between the review worktrees and mine. Nothing in either findings file turns on the hash.

Specification: `docs/plans/agent-scaffold.steps/ship-v0-0-2.md`, read in full.

## Method

Three release binaries, one `CARGO_TARGET_DIR` each, from three separate `git archive | tar -x` extracts, verified distinct by `md5sum` so no stale fingerprint could make two revisions look alike:

```
01cf789130c9f16d033cf89613ec91b6  tgt-head/release/agent-scaffold   (HEAD, 813fc02, reports 0.0.2)
092d8df849f106da99f4f3e8e847c379  tgt-main/release/agent-scaffold   (main, c68f541, reports 0.0.1)
2b2ebad34db6f4722d24f477254366e3  tgt-fix/release/agent-scaffold    (HEAD + my prototype resolved-path fix)
```

The third binary is mine. To answer the code-versus-wording question with a measurement rather than an estimate (Principle 6, Ground decisions in evidence), I built the candidate fix and ran it, instead of reasoning about what it would cost. What I changed and what it cost is in the ruling below.

Fixtures live under my own scratch subdirectory. Every escape target is inside it. No tracked file was modified anywhere except this triage file.

Gates I ran myself at HEAD, all green:

| Gate | Result |
| --- | --- |
| `cargo test` | 450 passed, 0 failed (400 + 50 across 10 further binaries) |
| `cargo clippy --all-targets -- -D warnings` | clean |
| `validate --source ... --metrics ...` | `328 records, valid`; `99 steps, 75 questions, valid`, exit 0 |
| `validate --source ... --workflow` | `workflow invariants hold`, exit 0 |
| `render --check --strict` | `up to date`, exit 0 |
| ASCII check on all 12 changed files | `0` on every file |
| `cargo publish --dry-run` | packaged, verified, `aborting upload due to dry run`, exit 0 |

`validate --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl` reports EXACTLY ONE problem, the pre-existing `` Open Questions item `Q-43` has an unknown status `superseded by `Q-44`` `` the spec excludes. `F1` criteria 3 and 4 hold.

ONE MEASUREMENT DIFFERENCE FROM BOTH REVIEWERS, immaterial: `cargo publish --dry-run` packaged 367 files for me against their 392. Mine ran in a `git archive` extract, which drops the untracked and ignored files a worktree carries. The packaged `README.md` is byte-identical to the working-tree one and carries both required items, so Release.2 holds on my run as on theirs.

I also confirmed the `F1` fix red-then-green myself, using the spec's own reproduction against `main` and HEAD rather than accepting the contract reviewer's table. At `main`, `validate --plan` on the freshly rendered fixture reports `` Open Questions item `Q-42` has an unknown status `undecided` ``. At HEAD it does not, and the two-line `ask` renders as the single line `` - `Q-1` (open) An open ask still awaiting a decision. - `Q-42` (undecided) a queue item nobody authored.`` with no text lost. `F1` is genuinely closed.

## Verdict table

Severity is ruled by me and is absolute impact if left unfixed. It rates the finding, not the exploit that exposed it.

| id | verdict | reviewer severity | MY severity | one line |
| --- | --- | --- | --- | --- |
| `A1` | VALID (surviving id of duplicate pair 1) | high | **high** | The containment predicate is lexical, so a pack shipping a symlink still reads outside the pack. |
| Contract Finding 2 | DUPLICATE of `A1`, merged | high | (merged) | Same defect, same mechanism, same two fields. |
| `A2` | VALID | low | **low** | A `dest` writes outside `--output-dir` through a symlinked directory the user already has in their output tree. |
| `A3` | VALID | low | **low** | Degenerate `dest` values pass the load-time predicate and fail at the write, one of them with a raw `Debug` `io::Error`. |
| `A4` | VALID | low | **low** | The refusal messages assert something untrue of the input they refuse. |
| `A5` | VALID (surviving id of duplicate pair 2) | low | **low** | An escaping `source` on an unselected module is not refused, while an escaping `dest` on the same asset is, and the two adjacent comments argue against each other. |
| Contract Finding 1 | DUPLICATE of `A5`, merged | low | (merged) | Same defect. Its mechanism analysis is the better of the two and is folded in below. |

Five valid findings after merging: one `high`, four `low`. No `medium`, no `critical`. No finding invalid, none out of scope.

I lowered no `high`, so no dismissed-or-downgraded-high re-check obligation arises from this triage.

TWO duplicate pairs, not one. The orchestrator anticipated the symlink pair. The disabled-module asymmetry was also raised independently by both reviewers (`A5` and Contract Finding 1) and is merged the same way.

## `A1` (high): the containment is lexical, so a symlinked pack path still reads outside the pack

MERGED: `A1` survives; Contract Finding 2 is folded into it. `A1` is the more complete record (it carries the directory-symlink escalation and the git delivery vector, which Contract Finding 2 does not). Contract Finding 2's contribution is the precise mechanism statement and the explicit list of what is falsified; both are reflected below.

### My measurements

`safe_path::is_contained_relative` (`src/safe_path.rs:29-35`) inspects `Path::new(rel).components()` and rejects only an absolute root and a `..` component. `PackSource::read`'s `Directory` arm (`src/manifest.rs:430-435`) runs that check and then calls `fs::read_to_string(root.join(rel))`, which follows symlinks. A bare filename that is a symlink passes the string check unconditionally.

Three shapes, all built and run against the HEAD binary (md5 `01cf789...`):

Asset `source` through a symlink:

```
ln -sf ../secret.md $SB/t1/pack/link.md
# pack.toml: [[asset]] source="link.md" dest="leaked.md" ownership="working"
$BIN scaffold --template $SB/t1/pack --output-dir $SB/t1/out --vcs none --dry-run
          create  leaked.md
Dry run; nothing written. Pass --write to apply.        exit=0
$BIN scaffold --template $SB/t1/pack --output-dir $SB/t1/out --vcs none --write
          create  leaked.md
Wrote to .../t1/out (1 changed, 0 left untouched).      exit=0
cat $SB/t1/out/leaked.md
TOP SECRET TRIAGE
```

This is byte-for-byte the outcome the spec records for `F4b` at `d06f1b5`, the state the fix exists to end. The dry run is the same: the read has already happened by the time the plan line prints, so `F4b` criterion 2 fails for this shape.

Module `guidance` through a symlink:

```
ln -sf $SB/t2/outside-guidance.md $SB/t2/pack/g.md
# [[module]] name="m" guidance="g.md"; one rendered asset carrying {{modules}}
$BIN scaffold --template $SB/t2/pack --output-dir $SB/t2/out --vcs none --module m --write
          create  body.md
Wrote to .../t2/out (1 changed, 0 left untouched).      exit=0
cat $SB/t2/out/body.md
MODULES BLOCK:
SECRET GUIDANCE TRIAGE
```

The guidance shape is the worse of the two: no plan line names the file, because guidance renders into another asset's body.

One directory symlink restores arbitrary read. This is the shape that matters most, because it defeats the absolute-path refusal using a string that is relative and carries no `..`:

```
ln -sfn / $SB/t3/pack/root
# [[asset]] source="root/<abs path>/id_rsa" dest=".agents/notes.md"
$BIN scaffold --template $SB/t3/pack --output-dir $SB/t3/out --vcs none --write
          create  .agents/notes.md
Wrote to .../t3/out (1 changed, 0 left untouched).      exit=0
cat $SB/t3/out/.agents/notes.md
PRIVATE KEY MATERIAL TRIAGE
```

The delivery vector is real. `git` stores and restores symlinks, so a fetched pack carries the payload:

```
git -C $SB/t11/origin init -q && git -C $SB/t11/origin add -A && git -C $SB/t11/origin commit -qm p
git clone -q $SB/t11/origin $SB/t11/clone
ls -l $SB/t11/clone/pack/
lrwxrwxrwx  link.md -> ../../secret.md
$BIN scaffold --template $SB/t11/clone/pack --output-dir $SB/t11/out --vcs none --write
          create  leaked.md
Wrote to .../t11/out (1 changed, 0 left untouched).     exit=0
cat $SB/t11/out/leaked.md
SECRET VIA GIT
```

The orchestrator's own measurement (a pack shipping `link.md` as a symlink to `../secret.md` gives exit 0, the ordinary `create leaked.md` plan line, and the outside file's contents in the scaffolded project) is CORRECT in every particular.

### Severity: `high`, ruled

This is a live wrong behaviour: an actual file outside the pack is actually read and actually spliced into the scaffolded project, at exit 0, on both consumer fields, in both `--write` and `--dry-run`. The project's calibration says a live wrong behaviour is `high`, and this reaches that bar without argument.

I considered `critical` and did not take it. The project's calibration defines no bar above "live wrong behaviour", and inventing one here would be me ruling on a scale the project has not set. Two facts do sit at the top of the `high` band and the orchestrator must weigh them: the guidance shape leaks with NO plan line naming the file, so the run's own report cannot show it; and a scaffolded project is normally committed and pushed, so a local read becomes a publication one ordinary step later. If this project ever defines `critical`, this is the shape that tests the definition.

I did not lower any reviewer's severity, so nothing here creates a re-check obligation.

## The ruling this round turns on: CODE and WORDING, answered separately

The orchestrator asked for these to be separated because they may be answered differently and only one is optional. They are, and only one is.

### The CODE question: does the containment need to resolve real on-disk paths?

YES, if the property claimed is "a pack path cannot read outside the pack directory". A lexical check cannot decide a question about where a path LANDS, and the symlink case is exactly the gap between the two.

WHAT IT COSTS AND BREAKS, measured rather than estimated. I built the candidate fix and ran everything against it. The change is the `Directory` arm of `PackSource::read` only, keeping the existing lexical check in front of it as the cheap fail-fast:

```rust
PackSource::Directory(root) => {
	if !crate::safe_path::is_contained_relative(rel) {
		return Err(ReadError::Escapes(rel.to_string()));
	}
	let joined = root.join(rel);
	let real_root = fs::canonicalize(root).map_err(ReadError::Io)?;
	let real = fs::canonicalize(&joined).map_err(ReadError::Io)?;
	if !real.starts_with(&real_root) {
		return Err(ReadError::Escapes(rel.to_string()));
	}
	fs::read_to_string(&real).map_err(ReadError::Io)
}
```

Measured against that binary:

| What I checked | Result |
| --- | --- |
| Symlinked asset `source` (`A1` shape 1) | REFUSED, exit 2 |
| Symlinked module `guidance` (`A1` shape 2) | REFUSED, exit 2 |
| `root -> /` directory symlink (`A1` shape 3) | REFUSED, exit 2 |
| `cargo test` | 450 passed, 0 failed. Identical counts to HEAD. ZERO existing tests needed editing. |
| `cargo clippy --all-targets -- -D warnings` | clean |
| Legitimate pack-INTERNAL symlink (`alias.md -> sub/real.md`, both inside the pack) | still loads, correct contents |
| Legitimate SYMLINKED PACK ROOT (`--template` naming a symlink to the pack directory) | still works, correct contents |
| Missing declared `guidance` file, error shape | byte-identical message to HEAD |
| Full scaffold of the shipped `pack/` with `--module checks`, HEAD versus fix | `diff -r` reports no differences |
| Runtime, 38-file directory pack, 3 runs each | HEAD 7/4/4 ms, fix 5/5/5 ms. In the noise. |
| Symlinks in the repository's own shipped `pack/` | 0, out of 38 files. The built-in pack uses the `Embedded` arm and is untouched regardless. |

So the cost is about ten lines in one function, no test edits, no measured behaviour change on any legitimate input, and no measurable runtime cost. `A1`'s own write-up calls this "a design decision with a cost (a canonicalize per read, and behaviour on a legitimate symlinked pack to settle)". I MEASURED both of those concerns and neither materialises: the canonicalize is free at pack scale, and both legitimate symlinked-pack shapes I could construct keep working. This is my most important disagreement with a reviewer, because `A1`'s framing invites the human to treat the fix as expensive and it is not.

Two things the fix does change, which the remedy must carry:

- `fs::canonicalize` resolves the path, so it stats and reads links. The contents are still never read, so `PackSource::read`'s "an escaping path is never read" survives; but "needs no filesystem access" would no longer be true OF THE READ SITE. It stays true of `is_contained_relative` itself, which is unchanged, so `validate --source`, `render --check` and the plan-side boundary are untouched.
- The refusal message becomes wrong for the newly refused inputs. I measured the fix refusing `link.md` with `` a source must be a relative path with no `..` component ``, and `link.md` IS a relative path with no `..`. This is `A4`'s defect, widened by `A1`'s remedy, which is why the two remedies are coupled.

IS HOLDING THE NEW BOUNDARY TO A WEAKER STANDARD DEFENSIBLE? No, on this repository's own terms.

- The repository already applies the resolved-path standard to its plan-side boundary and documents it in `README.md:242`: the artifact must live under the plan's project root, "resolving both through their real on-disk locations so a symlink cannot disguise one as the other". It went further and accepted BREAKING a symlinked `docs/plans` layout for it, on the recorded ground that "a loud refusal beats silently reading the wrong file".
- `fs::canonicalize` is at four live call sites in `src/main.rs` (`:1379`, `:1475`, `:1924`, `:1949`), plus one doc mention (`:1571`) and two in tests (`:3009`, `:3034`). I verified this count myself. It is an established technique here, not a new dependency.
- The threat models point the OPPOSITE way from the standards applied. The plan-side boundary governs the user's OWN `.plan.toml` in their OWN repository and gets the STRONGER standard. The pack boundary governs "a `--template` pack the user may have fetched from anywhere", which is the step's own words, used TWICE as the reason the `src/checks.rs` trusted-config argument does not carry over, and it gets the WEAKER standard. Nothing in the change records a reason for the inversion.

### The WORDING question: is any wording acceptable that leaves the symlink case open?

Yes, but ONLY wording that states the limitation. This half is not optional and does not depend on the code decision. 0.0.2 is about to be published; a CHANGELOG entry is a durable public claim, and a user who reads "a pack path can no longer read outside the pack directory" reasonably stops inspecting fetched packs for symlinks. A false security claim in a published changelog is worse than no claim, because it transfers the reader's caution to a guarantee that does not exist.

EXACTLY WHAT WOULD HAVE TO CHANGE. Both reviewers name three sites. I measure FOUR that are false as shipped, plus a fifth tied to `A2`. The fourth is on `PackSource::read` itself and neither reviewer named it.

1. `CHANGELOG.md:32`, the 0.0.2 Fixed bullet, TWO clauses. The opening, "A pack path can no longer read outside the pack directory", must be scoped to the path STRING, for example "A pack path that NAMES a location outside the pack directory is now refused". And the closing, "it refuses before the join is opened, so an escaping path is never read rather than merely never used", must be scoped the same way, since it is true only of a string-escaping path. The bullet must then STATE the limitation: the containment is a check on the path string, so a pack that ships a symlink whose target is outside the pack is still followed and still read.
2. `src/manifest.rs:46` (`AssetSpec.source`): "so the within-the-pack claim holds rather than being merely documented". The within-the-pack claim does not hold. It must say the string-level rule holds and name the symlink limitation.
3. `src/manifest.rs:99` (`ModuleSpec.guidance`): "so a guidance partial cannot splice a file from outside the pack into `{{modules}}`". Measurably false; I spliced one. This clause must go, replaced by the string-level rule plus the limitation.
4. `src/manifest.rs:410` (`PackSource::read`): "The refusal happens before the join is opened, so an escaping path is never read, not merely never used." `an escaping path` is unqualified here, and a symlinked path escapes and IS read. NEITHER REVIEWER NAMED THIS SITE. It needs the same scoping as the CHANGELOG's closing clause. The neighbouring "THE ONE containment boundary for pack-controlled paths" (`:403`) is a claim about the SITE being single, which is true, and can stand.
5. `src/manifest.rs:53` (`AssetSpec.dest`), "so the relative claim holds rather than being merely documented", is weaker than it sounds for the same reason on the write side (`A2`), but the claim it makes is about the string being relative, which is true. Lowest priority; fold it in only if `A2` is fixed.

Checked and found TRUE, so needing no change: `ReadError::Escapes`'s doc (`src/manifest.rs:367`), because it self-scopes to "it is absolute, or it carries a `..` component" and the variant is only produced for those shapes as shipped; and `src/safe_path.rs:1`, which says "free-string path" and is honest about being lexical.

## `A2` (low): a `dest` writes outside `--output-dir` through a pre-existing symlinked directory

VALID. Reproduced:

```
ln -sfn $SB/t4/elsewhere $SB/t4/out/docs
# [[asset]] source="x.md" dest="docs/dropped.md"
$BIN scaffold --template $SB/t4/pack --output-dir $SB/t4/out --vcs none --write
          create  docs/dropped.md
Wrote to .../t4/out (1 changed, 0 left untouched).       exit=0
ls -l $SB/t4/elsewhere
-rw-r--r-- 1 jessea users 15 dropped.md
```

Severity `low`, agreeing with the reviewer but on a sharper ground than theirs. Their ground is that the symlink must pre-exist. Mine is stronger: the pack CANNOT supply the link at all. `apply_asset` writes through `fs::write`, so a pack can only ever produce regular files and directories, never a link. Every reachable instance of this therefore involves the tool writing through a symlink the USER created in the USER's own output tree, which is closer to honouring the user's own layout than to a pack escaping anything. The genuine defect is narrower: the run reports "Wrote to `<output-dir>`" while a file landed elsewhere, and with `ownership = "reference"` it could overwrite a file on the far side of a link the user forgot about (Principle 3, Safe on existing projects). That is a reporting and clobber-risk problem, not a containment bypass, and it cannot change a verdict or an outcome. `low` is right.

## `A3` (low): degenerate `dest` values pass the load-time boundary and fail at the write

VALID. Reproduced all four shapes:

```
--- dest=[]      skip (exists)          Wrote to ... (0 changed, 1 left untouched).   exit=0
--- dest=[.]     skip (exists)  .       Wrote to ... (0 changed, 1 left untouched).   exit=0
--- dest=[./]    skip (exists)  ./      Wrote to ... (0 changed, 1 left untouched).   exit=0
--- dest=[sub/]  create  sub/           Error: Os { code: 21, kind: IsADirectory, ... }  exit=1
```

I MEASURED ONE SHAPE THE REVIEWER DID NOT, and it is the worse one. All four of theirs used `ownership = "working"`, which reaches the create-if-absent branch and skips. With `ownership = "reference"` the file is always refreshed, so `dest = ""` reaches the write:

```
# [[asset]] source="ok.md" dest="" ownership="reference"
         refresh  
Error: Os { code: 21, kind: IsADirectory, message: "Is a directory" }    exit=1
```

So the raw `Debug`-formatted `io::Error` is reachable from `dest = ""` too, not only from `dest = "sub/"`. Nothing is written outside the output directory in any of the five, no verdict or outcome changes, and the failure is loud. `low` confirmed.

## `A4` (low): the refusal messages assert something untrue of the input

VALID. Reproduced:

```
# [[asset]] dest = "a/../b.md"
main:  create  a/../b.md ... Dry run; nothing written.                              exit=0
head:  error: asset `x.md` has dest `a/../b.md`, which leaves the output directory;
       a dest must be a relative path with no `..` component                        exit=2
```

The refusal is correct and deliberate, and `src/safe_path.rs:56-59` records the reasoning and pins it. The MESSAGE is not: `a/../b.md` does not leave the output directory, and the first clause says it does. Four messages share the shape: `UnsafeAssetSource`, `UnsafeModuleGuidance`, `UnsafeAssetDest` and `ReadError::Escapes`.

`low`: no outcome changes, only what the reader is told about their own input. Confirmed.

RAISED IN PRIORITY BY `A1`, not in severity. I measured that under the resolved-path fix the same messages refuse `link.md` with `` a source must be a relative path with no `..` component ``, which is false of `link.md` on both clauses rather than one. If `A1`'s code fix lands, `A4` must land with it in the same pass, or the change makes a shipped message more wrong than it found it.

## `A5` (low): the source check and the dest check disagree about disabled modules

MERGED: `A5` survives; Contract Finding 1 is folded into it. Contract Finding 1 has the better mechanism evidence and it is used below.

VALID. Reproduced both halves:

```
-- escaping SOURCE on an unselected module, module OFF:
          create  ok.md
Wrote to .../t7/out (1 changed, 0 left untouched).      exit=0     (NOT refused)
-- same pack, module ON:
error: asset source `../secret.md` leaves the pack directory; ...   exit=2

-- escaping DEST on an unselected module, module OFF:
error: asset `ok.md` has dest `../../escaped.md`, which leaves the output directory; ...  exit=2
```

The mechanism, from Contract Finding 1 and verified by me in the source: `manifest::load` checks `dest` in a loop that runs BEFORE the module-enabled filter (`src/manifest.rs:624-643`), while `source` is reached only inside the `.filter(...).map(...)` chain (`src/manifest.rs:678-694`) whose filter drops unselected assets before `source.read` is called.

The orchestrator's own measurement (an escaping `source` on an unselected module's asset is not refused, and nothing leaks because nothing is read) is CORRECT.

Severity `low`. Nothing is read, so nothing leaks, and no verdict or outcome changes; the project's calibration keeps that below `medium`. What is genuinely defective is that the two adjacent comments argue against each other: `src/manifest.rs:630-632` justifies the `dest` behaviour because "an escaping dest is a pack-authoring error whether or not its module is on", which is equally true of `source`, and `src/manifest.rs:634-637` declines to apply it. One of the two must change whichever way this is settled.

## Out-of-scope items, ruled

Both reviewers recorded these as pre-existing rather than as findings. I checked both against `main` myself rather than accepting the label.

- ANSI ESCAPES IN A `dest` REWRITING THE PREVIEW. GENUINELY OUT OF SCOPE. I ran the same pack against both binaries and the stdout is identical: both print `create ^[[1A^[[2Khidden.md` and both create the file. Not a regression, not named by the spec. Correctly excluded.
- THE PLAN-SIDE SIDECAR SYMLINK HOLE. GENUINELY OUT OF SCOPE as a finding against this artifact. I ran a `[meta].sidecars` front ref naming a symlink to an outside file against both binaries: both render at exit 0 and both splice the outside content into `<task>.md`. I also confirmed the `../` shape is refused identically on both, so the lexical plan-side check is pre-existing and unchanged. Not a regression. It bears on `A1` only in the remedy's shape, noted below, and it is a live README-versus-code gap of its own (`README.md:242` promises resolved-path treatment for the metrics and ledger boundary, while the sidecar read boundary is lexical) that deserves its own follow-up step rather than a place in this one.
- THE COMMIT ORDERING (`b3aad56` "chore: release 0.0.2" lands before the `F4b` fixes). Correctly not a finding: only HEAD ships, and HEAD's CHANGELOG and code agree modulo `A1`. Worth keeping in the record for anyone who ever rewrites or partially applies this history.

## Scope recommendation, for the human to decide

This is a genuine decision and I am not taking it. The step's sidecar declares its scope closed, says "every addition to it defeats it", and records that it was already widened once by `Q-75`, which explicitly says a second addition needs its own human decision rather than the `Q-75` precedent. The 2026-08-13 audit measured steps generated by the process itself rising from 8.3% to 54.2%, monotonic, and eleven consecutive days producing zero completed steps across 196 commits. Both pressures are real and they point opposite ways.

ONE FACT COLLAPSES MOST OF THE APPARENT BALANCE, and the human should have it before choosing. Option B does not avoid another round. A wording-only change is still a change to the artifact, so it still needs an implementer pass and still needs a review round before this increment can converge. The process cost of A over B is therefore not "one more round versus none". It is about ten lines of code and three added tests, inside a pass that is happening either way. I measured that ten-line version passing all 450 tests with no test edits.

### Option A: fix the resolution AND the wording in this step

COSTS. A second widening of a scope declared closed, needing its own decision receipt on the `Q-75` pattern. About ten lines in `PackSource::read`, four doc and CHANGELOG wording sites, the `A4` message fix that `A1` forces along with it, and three or four added tests. One implementer pass, one review round.

GAINS. 0.0.2 ships with the defect class `F4b` exists to close actually closed, and every claim in the published artifact true.

### Option B: fix only the wording, route the resolution to a follow-up

COSTS. 0.0.2 publishes with a known, documented, open hole in the exact class the release exists to close. The follow-up becomes process-generated work of precisely the kind the audit measured at 54.2%, and it will need its own step, its own increment and its own review loop, which is MORE total process than folding it in here. Same one pass and one round as Option A.

GAINS. The closed-scope rule stays unbroken and no second widening is needed.

### Option C, which I think is better than either as stated

Widen by EXACTLY `A1` and its coupled `A4` message fix, take the wording at all four sites, and route `A2`, `A3` and `A5` to a follow-up. `A4`'s message fix is not a separate widening: `A1`'s remedy makes those messages more wrong than it found them, so it comes along as part of doing `A1` correctly rather than as an addition. This is Option A with the widening drawn as narrowly as the defect allows, and it is what I recommend.

### Recommendation and reasoning, against the plan's Project Principles by name

I RECOMMEND OPTION C.

- MAKE ILLEGAL STATES UNREPRESENTABLE (Principle 5). Four sites in the shipped code declare the state illegal and nothing makes it illegal. This is the admit-then-guard shape the principle exists to prevent, in its worst form, because here the guard does not even cover the case. Option B makes that declaration false in a PUBLISHED artifact and defers making it true; Option C makes it true.
- PREFER THE CLEANER LONG-TERM ARCHITECTURE OVER THE SMALLEST DIFF (Principle 1). This is the decisive one, because it is the SAME argument the step itself already accepted. `Q-75` admitted `F4b` on the ground that "once `F4` lands, `AssetSpec` enforces containment on `dest` and not on `source`, which is an incoherent boundary rather than a smaller one". That argument applies verbatim one level down: once this change lands, the repository enforces resolved-path containment on the plan-side metrics boundary, which reads the user's own files, and lexical-only containment on the pack boundary, which reads files the user fetched from anywhere. That is the same incoherence, with the weaker standard on the more hostile input, and rejecting it here is consistent with the reasoning the human already accepted rather than a new demand.
- SAFE ON EXISTING PROJECTS (Principle 3). The leak reads a file outside the pack into a project the user will normally commit and push. The guidance shape does it with no plan line naming the file, so the run's own report cannot show the user what happened.
- GROUND DECISIONS IN EVIDENCE (Principle 6). The cost argument for deferring rests on the fix being expensive. I built it and it is not: ten lines, zero test edits, 450 of 450 passing, clippy clean, no measured behaviour change on any legitimate pack, runtime in the noise. The human should decide against the measurement rather than the estimate.
- MINIMAL BY DEFAULT (Principle 2) is the counterweight and it is honestly weaker here than it first looks. Principle 2 as written is about the core doing one thing well while everything else is an opt-in module; it is not a rule about diff size. The real counterweight is the step's closed-scope sentence, which is a process rule rather than a principle, and the audit's 54.2%, which is a warning about work the process invents for itself. `A1` is not invented work: it is the defect the step's own `F4b` names, in a carrier the acceptance criteria did not enumerate. Fixing it is finishing the declared scope, not extending it. That is what makes Option C narrow enough to be consistent with the closed-scope rule, and it is why I would draw the line at `A1` plus `A4`'s message and refuse `A2`, `A3` and `A5` here.

## Remedies

Scoped to the class, not the instance. Each names what must NOT be edited and why. Where a fixture is the only thing pinning an axis, the remedy ADDS rather than replaces, per the previous increment's failure in which a remedy substituted the only fixture holding an axis and silently retired it.

### `A1` remedy (code half, if Option A or C is taken)

CLASS: a pack-controlled path must be contained by its RESOLVED on-disk location, not by its string alone.

CHANGE ONE SITE: the `Directory` arm of `PackSource::read` (`src/manifest.rs:430-435`). Keep the existing lexical check in front of the resolved one, as the fail-fast that lets a refusal cost nothing and keeps the string-only property available. Author the resolved predicate as a SECOND function in `src/safe_path.rs` (something in the shape of `resolved_within(root: &Path, rel: &str) -> io::Result<Option<PathBuf>>`) rather than inline in `manifest.rs`, so the plan-side sidecar boundary can adopt it in its own follow-up without re-authoring the rule (Principle 1). This is the same reason `F4`'s criterion 3 gave for lifting `is_safe_sidecar_ref`.

MUST NOT BE EDITED:

- `is_contained_relative` itself (`src/safe_path.rs:29-35`). It has three callers, and `is_safe_sidecar_ref` (`src/plan/source.rs:480`) depends on its no-filesystem-access form so that `validate --source`, `render` and `render --check` refuse without touching disk. Changing it would silently move the plan-side boundary, which is out of scope this round. Add beside it; do not modify it.
- Its two tests, `a_task_relative_reference_is_contained` and `an_absolute_or_parent_bearing_reference_is_not_contained` (`src/safe_path.rs:41-60`). They are the ONLY pins on the lexical axis, including the deliberate `a/../b.md` refusal that `A4` discusses. If they were replaced by resolved-path tests, the lexical axis would be silently retired, which is the exact failure this instruction exists to prevent.
- The `Embedded` arm. It resolves against a compile-time map and touches no filesystem, so it correctly gets no check. Its exemption is pinned by the `assert!(builtin().read("pack.toml").is_ok())` line inside `the_read_site_contains_every_pack_controlled_path`; keep that line.
- The built-in asset list around `src/manifest.rs:611` (old numbering), per `F4`/`F4b` criterion 4.

TESTS, ALL ADDITIONS:

- `the_read_site_contains_every_pack_controlled_path` (`src/manifest.rs:1035`) currently pins four things: a contained path returns its own bytes; `../outside.md` and `/etc/passwd` both refuse AS `ReadError::Escapes` rather than as I/O errors, reporting the offending string; and the embedded arm is exempt. All four must survive. ADD a symlink case to the same loop (a `link.md` inside the pack pointing at `../outside.md`), because this test is the only unit-level pin on the one-site property. Do not swap an existing shape out for the symlink one.
- `a_nested_relative_source_still_loads` (`src/manifest.rs:1064`) is the only non-vacuity pin on the source side, and its comment says so. Do not touch it. ADD a companion pinning that a pack-INTERNAL symlink still loads (`alias.md -> sub/real.md`, both inside the pack). I measured this passing under the prototype; without a pin, nothing stops a later over-tightening such as "refuse any symlink" from breaking a legitimate pack, and the acceptance criterion "a normal scaffold run drops the same set of files" would not catch it because the shipped pack has zero symlinks.
- `tests/pack_source_stays_inside_the_pack.rs` currently pins four integration cases at both `--write` and `--dry-run`: parent-dir `source`, absolute `source`, parent-dir `guidance`, absolute `guidance`. ADD two, a symlinked `source` and a symlinked `guidance`. Do not replace the four.
- ADD one integration case for the directory-symlink escalation (`root -> /` plus an absolute-looking suffix). It is a distinct shape, not a variant of the other two: it defeats the ABSOLUTE-path refusal using a string that is relative and carries no `..`, so neither existing test covers it.

DISCLOSE IN THE OUTCOME: the resolved check stats and reads links, so the read site now touches the filesystem before refusing. The file contents are still never read, so "never read" survives; "no filesystem access" must stop being claimed of the read site. It remains true of `is_contained_relative`, which is unchanged.

### `A1` remedy (wording half, MANDATORY regardless of the code decision)

The four sites enumerated in the wording ruling above: `CHANGELOG.md:32` (two clauses), `src/manifest.rs:46`, `src/manifest.rs:99`, and `src/manifest.rs:410`. If the code fix lands, these get rewritten to the TRUE stronger claim; if it does not, they get rewritten to the true weaker claim plus the stated limitation. There is no third option in which they stay as they are.

Nothing pins doc-comment or CHANGELOG text, so no fixture is at risk here. That absence is itself worth the orchestrator's attention: the claims this round is about are held by nothing but review.

### `A2` remedy

CLASS: the write side resolves nothing either. Route to a follow-up; do not fix here.

If taken, the site is `apply_asset` (`src/main.rs:119-127`), and the mechanism must DIFFER from the read side's: the leaf does not exist yet, so `fs::canonicalize` on the destination fails and the check has to be on the resolved PARENT after `create_dir_all`, or on the longest existing ancestor. `src/main.rs:1924` and `:1949` already do the existing-ancestor form for the hooks directory and are the pattern to follow. Do not bolt this onto `PackSource::read`, which is a read-side boundary.

MUST NOT BE EDITED: the two tests in `tests/pack_dest_stays_inside_the_output_dir.rs` (`a_parent_dir_dest_is_refused_and_writes_nothing`, `an_absolute_dest_is_refused_and_writes_nothing`). They are the only integration pins on the two `dest` shapes `F4` closed. A resolved-path test ADDS a third case.

### `A3` remedy

CLASS: a `dest` that names no file at all. Route to a follow-up.

The rule belongs in `manifest::load`'s dest loop (`src/manifest.rs:624-643`), NOT in `is_contained_relative`. "Names a file" is a destination-specific requirement, while `is_contained_relative` is shared with the read side and the plan side, where a `.` component legitimately names the base and is accepted by design (`src/safe_path.rs:23`). Putting it in the shared predicate would change the plan-side boundary as a side effect. Reject a `dest` whose component list is empty or contains only `CurDir`, and a `dest` with a trailing separator.

Separately and in the same class, `apply_asset`'s error path surfaces a `Debug`-formatted `io::Error` struct, which no other refusal in this change does. Reachable from both `dest = "sub/"` and `dest = ""` with `ownership = "reference"`.

MUST NOT BE EDITED: `src/safe_path.rs`'s tests, for the reason above. ADD a `manifest::tests` case beside `an_escaping_dest_is_refused_at_load`.

### `A4` remedy

CLASS: a refusal message that asserts a property of the INPUT rather than stating the RULE. Four messages share it: `UnsafeAssetSource` (`src/manifest.rs:256-262`), `UnsafeModuleGuidance` (`:263-270`), `UnsafeAssetDest` (`:271-278`), `ReadError::Escapes` (`:379-383`). Drop the "which leaves the ... directory" assertion, state the rule, and name the offending component. Fix all four together; fixing one leaves three siblings making the same wrong statement.

COUPLED TO `A1`: if the code fix lands, this must land in the same pass, because the resolved check refuses inputs of which BOTH clauses are false.

MUST NOT BE EDITED WITHOUT CARE: `assert_refused` and `assert_guidance_refused` in the two integration test files match on message text. The property they pin is that the message NAMES the offending value. Keep that assertion and update the matched substring; do not delete the matcher to make the test pass.

### `A5` remedy

CLASS: which pack-authoring errors are reported eagerly, and the two comments that disagree about it. Route to a follow-up.

Two shapes, and the human or implementer picks one:

1. Add a load-time LEXICAL `source` and `guidance` check beside the `dest` one, covering every declared entry regardless of selection, and reword `src/manifest.rs:634-637` to say the load-time check is an eager pack-authoring diagnostic while `PackSource::read` remains the security boundary. This is the Principle 5 form and it makes the two fields symmetric.
2. Narrow `src/manifest.rs:630-632` so it stops asserting a general regardless-of-selection rule that the twin field does not follow. Minimal, and it leaves the asymmetry in place but honestly described.

Either way ONE of the two comments must change, because as shipped each argues against the other.

MUST NOT BE EDITED: `an_escaping_dest_on_an_unselected_module_is_still_refused` (`src/manifest.rs`, in the `manifest::tests` block). It is the only pin on the dest-side eager-check axis. Shape 1 ADDS a source-side twin beside it; it does not replace it.

## Round outcome

ROUND OUTCOME: `new_valid`.

- Valid findings: 5 (after merging two duplicate pairs from 7 reported).
- Severity list: `high` x1 (`A1`), `low` x4 (`A2`, `A3`, `A4`, `A5`).
- No `medium`, no `critical`, none invalid, none out of scope.
- No `high` was lowered or dismissed, so this triage creates no re-check obligation.

THIS ROUND DOES NOT CONVERGE THE INCREMENT. `ship-v0-0-2-inc1` is declared `low_risk` by `Q-74`, so one CLEAN round converges it. This round is not clean. What would converge it: one subsequent round that produces zero valid findings, run after the remedies land. On the evidence here that is a realistic next round rather than an open-ended loop, because the `high` has a measured ten-line fix and the four `low` findings are either deferrable or mechanical.

## Merge and publish

THE CHANGE IS NOT SAFE TO MERGE AND PUBLISH AS IT STANDS.

The blocking reason is not the symlink hole by itself. It is that the artifact ships four assertions, one of them in a CHANGELOG bound for crates.io, stating a property the code does not have, in the exact defect class this release exists to close. A user who reads "a pack path can no longer read outside the pack directory" and stops inspecting fetched packs is worse off than a user who was told nothing.

MINIMUM SET THAT MAKES IT SAFE TO PUBLISH: the wording, at all four sites in the wording ruling. `CHANGELOG.md:32` (both clauses), `src/manifest.rs:46`, `src/manifest.rs:99` and `src/manifest.rs:410`, rewritten to claim the string-level containment that genuinely holds and to state the symlink limitation plainly. With that alone, everything the 0.0.2 artifact says is true of the 0.0.2 code, and `F1`, `F4` and the `..`/absolute half of `F4b` are all genuinely fixed and gated green.

WHAT I RECOMMEND SHIPPING INSTEAD: that wording plus the ten-line resolved-path fix and the `A4` message correction it forces, which is Option C. Then the stronger sentence is the true one and the release closes the class it was written to close.

`A2`, `A3` and `A5` do not block publication. None can change a verdict or an outcome, and none contradicts anything the artifact claims.
