# `ship-v0-0-2-inc1` round 2: REVIEWER (claims lens)

Independent reviewer. I did not write this change and did not review round 1. Every figure below is my own measurement, made in a worktree detached at `f92e6df` against `main` at `c68f541`.

## Artifact reviewed

`git diff main..HEAD`, 12 files, 1367 insertions, 137 deletions. Six commits, of which one, `f92e6df` ("fix: contain a pack path by where it resolves, not by its string alone"), is the round 1 fix pass. That pass rewrote four claim sites, three refusal messages and the `safe_path` module, and added seven tests.

Specification read in full: `docs/plans/agent-scaffold.steps/ship-v0-0-2.md`. Round 1 adjudication read in full: `docs/plans/agent-scaffold.reviews/v002-r1-triage.md`.

Method:

- One release binary built from the worktree (`cargo build --release`), used for every behavioural claim. A dozen scratch packs under my own scratch directory; every escape target is inside it.
- Syscall evidence from `strace -f -y -e trace=openat,open,stat,lstat,newfstatat,readlink,readlinkat,statx` for the "before the file is opened" and "never reads the contents" claims.
- Mutation testing for the added tests: six mutations of `PackSource::read` / `safe_path::resolved_within` in a `git archive` copy of the tree, each run against the suite to see which test dies. No tracked file in this worktree or the main repository was modified except this findings file.
- `cargo test` at HEAD: 461 passed, 0 failed.

Assertions checked: 62. Confirmed: 51. Falsified: 10, grouped into 5 findings, all `low`. One partial (`C12`), folded into finding 1 rather than raised on its own. One (`C3`) is a historical claim about the pre-fix code that I accepted from round 1's measurement rather than re-running. Separately, 13 tests were checked for non-vacuity, 11 of them by mutating the behaviour each names.

## Assertion table

Every assertion I checked, the case I constructed to falsify it, and the measured result. `CL` is `CHANGELOG.md`, `MF` is `src/manifest.rs`, `SP` is `src/safe_path.rs`, `PS` is `src/plan/source.rs`.

### The 0.0.2 `Fixed` bullet on the pack read escape (`CL:32`)

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| C1 | "A pack path can no longer read outside the pack directory" | 8 shapes at `--write` and `--dry-run`: `../secret.md`; an absolute path; `link.md -> ../secret.md`; `up -> ..` plus `up/secret.md`; `root -> /` plus an absolute-looking suffix; `nearby -> ../pack-evil` plus `nearby/x.md`; the same through `[[module]].guidance`; a symlinked `pack.toml` | Every shape refused at exit 2, with the output directory empty after every run. CONFIRMED |
| C2 | "TWO pack-controlled fields reach that join" | Grepped every `PackSource::read` caller | 5 callers: `MF:489` (`pack.toml`), `MF:611` (`guidance`), `MF:735` (`spec.source`), `main.rs:229` (`principles.toml`), `main.rs:259` (`instrument.md`). Exactly 2 carry pack-controlled text. CONFIRMED |
| C3 | The three pre-fix leak shapes, through both fields | Not re-run against `main` | Measured by round 1's triage at the pre-fix tree and by the spec at `d06f1b5`. Accepted, not re-measured |
| C4 | "applied once at `PackSource::read`, the single site every pack path reaches the filesystem through" | Searched for a pack-controlled path reaching the filesystem elsewhere | Only `[[asset]].dest`, which is a write and is checked separately at `MF:684`. CONFIRMED for reads |
| C5 | "It applies two rules in order" | Read `MF:465-481`; ran a lexically bad path against a nonexistent file | Lexical first (`../absent.md` refused as an escape without a stat), resolved second. CONFIRMED |
| C6 | "Either refusal happens before the file is opened, so a path that escapes by either rule is never read" | `strace` of the `link.md -> ../secret.md` run, grepping every `open`/`openat` | The only `openat` calls are for `pack.toml`. No open of `secret.md`. CONFIRMED |
| C7 | "the read site now stats and follows links before refusing" | Same trace | `readlink(".../pack/link.md", "../secret.md") = 12`, then `readlink(".../secret.md") = -1 EINVAL`. The link is followed and the target resolved. CONFIRMED |
| C8 | "it never reads the contents of a path it refuses" | Same trace; also checked the refused target is never opened for reading | No `open`, no read of the target in any refusal. CONFIRMED |
| C9 | "`PackSource::Embedded` gets no check and needs none" | Read `MF:455-464`; ran a built-in scaffold | No filesystem access in that arm; built-in scaffold unchanged. CONFIRMED |
| C10 | "A pack-internal link ... keeps working" | `alias.md -> sub/real.md`, and a second with an absolute target inside the pack | Both scaffolded at exit 0 with the linked contents. CONFIRMED |
| C11 | "a `--template` naming a link to the pack directory itself ... keeps working" | `--template linked-pack` where `linked-pack -> real-pack` | Exit 0, correct contents. CONFIRMED |
| C12 | "Each caller labels the refusal with its own field" | Fired the refusal from the three literal callers | True of the two consumer fields. FALSE if read to cover the literal callers: `pack.toml` reports as an `io::Error` with no field, and `principles.toml` / `instrument.md` swallow the refusal silently. Evidence carried in finding 1 |
| C13 | "an escaping `guidance` reports as a module guidance problem and never as an asset `source` one" | Ran all three guidance causes | Every message names the module and the guidance path, never "asset source". CONFIRMED |
| C14 | "neither reports as a failed read, since nothing was opened" | Grepped every refusal message for "could not be read" | Absent from all six read-side refusals. CONFIRMED |
| C15 | "one link to a directory restores arbitrary reach using a path string that is relative and carries no `..`" | Built `root -> /` and `up -> ..` packs | Both are now refused; the historical claim matches round 1's measurement. CONFIRMED |

### The 0.0.2 `Fixed` bullet on the refusal messages (`CL:33`)

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| C16 | All four messages "opened with 'leaves the ... directory'" | Read the pre-fix text in `git show f92e6df` | All four did. CONFIRMED |
| C17 | "`a/../b.md` is refused for carrying a `..` component and does not leave anything" | Ran `a/../b.md` as a `source`, a `guidance` and a `dest` | All three refused, all three name the `..` component and assert nothing about leaving. CONFIRMED |
| C18 | "Each message now says the value is not a contained path, names the specific cause in parentheses ..., and then states the whole rule" | Ran all 8 reachable (message, cause) pairs: `source` and `guidance` with absolute / `..` / link; `dest` with absolute / `..` | Every message has the three parts, and the cause matches the input in all 8. CONFIRMED |
| C19 | "The `dest` message states only the string half of the rule" | Read the emitted text | "a dest must be relative and carry no `..` component", with no resolution clause. CONFIRMED |
| C20 | "because the write side applies only that half" | Read `MF:684` and `apply_asset` (`main.rs:119-129`) | The load check is `is_contained_relative` alone; `apply_asset` is a bare `root.join(&asset.dest)` with no resolution. CONFIRMED |

### The 0.0.2 `Fixed` bullet on the `dest` write escape (`CL:34`)

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| C21 | Refused "when the manifest is LOADED, before any asset is read and before the plan preview is printed" | Ran both shapes at `--write` | Exit 2, no `create` line, nothing written. CONFIRMED |
| C22 | "the message names the offending asset and its `dest`" | Ran both shapes | Both name the asset source and the dest. CONFIRMED |
| C23 | "A dry run refuses on the same ground" | Ran both shapes at `--dry-run` | Byte-identical messages, exit 2. CONFIRMED |
| C24 | "covers every declared asset, whether or not its module is enabled" | Escaping `dest` on an asset tagged with an unselected module | Refused at exit 2. CONFIRMED |
| C25 | "the read boundary and the write boundary cannot drift apart" | Checked whether the two named boundaries share one predicate | `is_safe_sidecar_ref` (`PS:480`) now delegates to `is_contained_relative`, which is also the dest check. CONFIRMED on the sidecar-read / dest-write reading, which the sentence's own antecedents give it. See observation 1 for the looser reading |

### The 0.0.2 `Fixed` bullet on the render escape (`CL:35`)

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| C26 | "Four sites interpolated free-text TOML into a generated line with no neutralisation", enumerated | Read `render.rs` at `main` and at HEAD | The four enumerated values were unneutralised at `main` and all four pass through `one_line` at HEAD. CONFIRMED |
| C27 | "Every line ending in a value written onto a generated line is now neutralised to a space and the result trimmed" | Injected `\n`, `\r\n` and `\r` plus injected headings, list markers and table rows into `[meta].title`, `principle.name`, `principle.text`, `question.ask`, a waiver `note` and a `[step.provenance].findings` ref of the render fixture, then rendered | `validate --source` accepts, every injected value renders on ONE line, no fabricated heading, principle, queue item or table row. CONFIRMED |
| C28 | "the `|` escape stays specific to a table cell" | Same fixture; pipes in the title, a principle, a queue item and the Notes cell | Escaped only in the Notes cell (`\|`); left raw elsewhere and no table forms. CONFIRMED |
| C29 | "The opaque prose sidecars are unaffected ... spliced verbatim" | Same render, compared the sidecar sections | Spliced verbatim. CONFIRMED |
| C30 | "no text is lost" | Same render | Every injected substring is present in the output. CONFIRMED |
| C31 | "the queue's items are again exactly the `[[question]]` entries the source declares" | Injected a `- `Q-42` (undecided) ...` second line into an `ask` and a `- `Q-43` (open) ...` into a findings ref | `validate --plan` reports neither; the only problem is the pre-existing `superseded by` one the spec excludes. CONFIRMED |

### The CHANGELOG against the release

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| C32 | The 0.0.2 section describes everything user-visible in `main..HEAD` | Enumerated the user-visible changes in the diff and matched each to a bullet | `F1`, `F4`, `F4b`/`A1`, `A4` and the version bump are all covered. The README `agent-flow` rename notice is not. FINDING 4 |
| C33 | The 0.0.2 section describes nothing absent from the release | Spot-ran three of the `Added` bullets | `audit --help` works; `status --json` carries `metrics_absent_reason`; `--module isolation` and `--module checks` both scaffold. CONFIRMED on the sample |

### `AssetSpec.source`, `AssetSpec.dest` and `ModuleSpec.guidance` doc comments

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| M1 | `MF:44-50`: refused "on both counts", and "the within-the-pack claim holds rather than being merely documented" | The 8 escape shapes above, through `source` | Both counts enforced; no shape reached an outside file. CONFIRMED |
| M2 | `MF:52-54`: an absolute or `..`-bearing `dest` "is refused at load ..., so the relative claim holds" | Both shapes, plus `a/../b.md` and `./ok.md` | Refused at load; `./ok.md` still writes. CONFIRMED (round 1's `A2` is about resolution, which this sentence does not claim) |
| M3 | `MF:97-102`: refuses absolute, `..`-bearing, or landing outside once links are followed, "before it opens the file. So a guidance partial cannot splice a file from outside the pack into `{{modules}}`" | All three causes through `guidance`, at `--write` and `--dry-run`; `strace` for the open | All refused, nothing opened, `{{modules}}` never rendered. CONFIRMED |

### `PackSource::read` and the error types

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| M4 | `MF:427-429`: "THE ONE containment boundary for pack-controlled paths. Every path a pack author writes reaches the filesystem through here" | Looked for a pack-author path that reaches the filesystem elsewhere | `[[asset]].dest` does, at `apply_asset`. The parenthetical that follows defines the set as `source`, `guidance` and the three literals, and round 1 ruled the neighbouring sentence can stand. Not raised; see observation 2 |
| M5 | `MF:429-430`: "the fixed `pack.toml`, `principles.toml` and `instrument.md` literals pass through too and can never escape" | Made each literal a symlink to a file outside the pack | `pack.toml` refused at exit 2; the other two silently swallowed. FALSIFIED. FINDING 1 |
| M6 | `MF:435-440`: two rules, "Either refusal happens before the file is opened, so a path that escapes by EITHER rule is never read" | `strace` on both a lexical refusal and a resolved refusal | No open of the target in either. CONFIRMED |
| M7 | `MF:442-445`: "this site stats and follows links before it refuses. It never reads the CONTENTS of a path it goes on to refuse" | Same trace | `readlink` on the link and its target, no open, no read. CONFIRMED |
| M8 | `MF:447-449`: "Only the `Directory` arm is checked" | Read the code; ran the built-in pack | CONFIRMED, and pinned by the `builtin().read("pack.toml")` assertion in the unit test |
| M9 | `MF:485-487`: "The path is a fixed literal, so the containment refusal in `read` cannot fire here" | Symlinked `pack.toml` to a manifest outside the pack | The refusal fired: `` error: `pack.toml` is not a contained pack path ... ``, exit 2. FALSIFIED. FINDING 1 |
| M10 | `MF:378-379`: "the relative path was refused because it leaves the pack" | `a/../b.md`, which is refused and leaves nothing | FALSIFIED. FINDING 2 |
| M11 | `MF:381-382`: "The path never reached the filesystem, so reporting it as a read error would describe an attempt that never happened" | `strace` of a resolved refusal | The path reached the filesystem: two `readlink` calls before the refusal. FALSIFIED. FINDING 2 |
| M12 | `MF:389-390`: `Escapes` fires when "it is absolute, or it carries a `..` component" | `link.md`, which is neither and fires it | FALSIFIED. FINDING 2 |
| M13 | `MF:191-192`: `UnsafeAssetSource` fires when the source "is absolute, or it carries a `..` component" | Same | FALSIFIED. FINDING 2 |
| M14 | `MF:200-201`: `UnsafeModuleGuidance`, same enumeration | Symlinked `guidance` | FALSIFIED. FINDING 2 |
| M15 | `MF:212-213`: `UnsafeAssetDest` fires when the dest "is absolute, or it carries a `..` component" | Looked for a third cause on the write side | There is none: the load check is lexical only. CONFIRMED, and the discriminator that shows M13/M14 are not a blanket complaint |
| M16 | `MF:233-239`: the resolved phrase is "the only remaining way a path the string rule accepts can be refused at the read site" | Looked for a non-link way to be lexically clean and land outside | None reachable without concurrent modification of the pack during the run. CONFIRMED |
| M17 | `MF:296-298`: "every refusal here has a lexical cause and the fallback is unreachable" | Constructed the complement of `is_contained_relative` that `lexical_failure` returns `None` for | Empty on Unix. Non-empty on Windows: `C:foo.md` has a `Prefix` component, is not absolute, and carries no `..`. FALSIFIED on Windows. FINDING 5 |
| M18 | `MF:680-683`: "A `source` reaches the filesystem only through `PackSource::read`" | Grepped for another read of a `source` | None. CONFIRMED |
| M19 | `MF:606-610`: a path leaving the pack "is reported as the refusal it is rather than dressed up as a failed read" | Ran all three guidance causes | No "could not be read" in any. CONFIRMED |

### `safe_path`

| # | Assertion | Falsifying case tried | Measured result |
| --- | --- | --- | --- |
| S1 | `SP:4-6`: `plan::source` joins a `[meta].sidecars` ref "(and a `[step.provenance].findings` ref) onto the plan directory to READ it" | Traced every use of a findings ref | Sidecar refs are joined and read (`render.rs:167-169`). Findings refs are shape-checked only and never joined; `PS:235-237` and `PS:245-247` say so in the code. Rendering a findings ref naming no file succeeds. FALSIFIED. FINDING 3 |
| S2 | `SP:16-18`: the lexical rule "needs no filesystem access, and so holds for a path that does not exist" | Ran `validate --source` on a plan whose sidecar refs name absent files | Refuses or accepts without touching them. CONFIRMED |
| S3 | `SP:19-21`: the resolved rule canonicalises both ends, requires the path to exist, touches the filesystem | Read the body; ran the unit test's absent-path case | `resolved_within(pack, "absent.md")` is `Err`. CONFIRMED |
| S4 | `SP:23-26`: an airtight read boundary uses lexical then resolved; a boundary that must answer without disk has only the lexical rule | Checked the read site's order and the plan-side callers | Read site does lexical then resolved; the plan side is lexical only. CONFIRMED |
| S5 | `SP:45-48`: the lexical rule "holds whether or not the referenced path exists ... and a `scaffold` dry run all refuse exactly what a write refuses" | Ran every source, guidance and dest shape at `--dry-run` and `--write` | Byte-identical refusals in all 16 pairs. CONFIRMED |
| S6 | `SP:57-59`: `lexical_failure` is `None` for a path refused on the resolved rule, so the caller supplies its own phrase | Ran `link.md` | Cause reported as "it resolves outside the pack directory, through a symbolic link". CONFIRMED |
| S7 | `SP:75-78`: `Ok(Some(real))` contained, `Ok(None)` outside, `Err` when either end cannot be canonicalised | Exercised all three through the unit test and the binary | CONFIRMED |
| S8 | `SP:83-86`: `Path::starts_with` compares whole components, so a name-prefix sibling is not a child | `pack/nearby -> ../pack-evil`, then `nearby/x.md` | Refused at exit 2. Mutating to a string prefix makes the run succeed and kills the unit test. CONFIRMED |
| S9 | `SP:88-91`: "It does not read the file's contents, so a refused path is still never read" | `strace` | No open of any refused path. CONFIRMED |
| P1 | `PS:473-479`: the rule "lives in `safe_path::is_contained_relative`, shared with the pack manifest's `dest` check" | Read both call sites | One function, both callers. CONFIRMED |

### The tests added this pass

Each was checked by mutating the behaviour it names and confirming it dies. Six mutations, each applied alone in a copy of the tree.

| # | Test | Mutation applied | Result |
| --- | --- | --- | --- |
| T1 | `the_read_site_contains_every_pack_controlled_path` (`MF:1081`) | M1: read site reverted to lexical-only | FAILED as required. Its four pre-existing pins all survive the edit: the contained read, the two string shapes refusing AS `Escapes` and reporting the offending string, and the `builtin()` embedded-exemption line. The implementer's statement about this test is accurate |
| T2 | `a_pack_internal_symlink_still_loads` (`MF:1140`) | M3: refuse any symlink | FAILED as required |
| T3 | `a_symlinked_pack_root_still_works` (`MF:1164`) | M2: canonicalise the path but not the root | FAILED as required, and every escape test stayed green under M2, which is exactly what the test's comment claims it exists for |
| T4 | `a_symlinked_source_or_guidance_is_refused_with_its_own_field` (`MF:1189`) | M1, and M5: resolved cause phrase changed | FAILED under both |
| T5 | `a_missing_pack_file_still_reports_as_missing_not_as_an_escape` (`MF:1243`) | M9: canonicalise error mapped to `Escapes` | FAILED as required |
| T6 | `the_lexical_rule_names_the_component_that_failed_it` (`SP:127`) | Not mutated | Asserts exact return values for five inputs, so any phrase or ordering change fails it. Non-vacuous by inspection |
| T7 | `the_resolved_rule_follows_links_and_answers_where_a_path_lands` (`SP:151`) | M3 | FAILED as required |
| T8 | `a_sibling_sharing_a_name_prefix_is_not_mistaken_for_a_child` (`SP:187`) | M6: `starts_with` replaced by a string prefix test | FAILED as required |
| T9 | `a_symlinked_source_is_refused_and_reads_nothing_outside_the_pack` | M1 | FAILED as required |
| T10 | `a_symlinked_module_guidance_is_refused_and_reads_nothing_outside_the_pack` | M1 | FAILED as required |
| T11 | `a_directory_symlink_cannot_restore_arbitrary_reach` | M1 | FAILED as required |
| T12 | `a_pack_internal_symlink_still_scaffolds` | M3 | FAILED as required |
| T13 | `one_line_neutralizes_every_line_ending_and_trims` (`render.rs:1112`) | Not mutated | Asserts exact outputs for five inputs including the trim case. Non-vacuous by inspection |

No test's name or comment claims a property its body does not exercise, and no existing assertion was weakened to make a new case pass: the two integration matchers (`assert_refused`, `assert_guidance_refused`) are unchanged and still require the message to name the offending value and its field.

## Verdict table

Severity is absolute impact if left unfixed.

| id | severity | site | one line |
| --- | --- | --- | --- |
| 1 | low | `MF:486`, `MF:429-430` | The literal callers are documented as unable to trigger the containment refusal, and one of them now triggers it while two swallow it silently. |
| 2 | low | `MF:378-379`, `MF:381-382`, `MF:389-390`, `MF:191-192`, `MF:200-201` | The `ReadError` and `LoadError` doc block still describes the one-rule world the fix pass replaced, including "the path never reached the filesystem", which the same file now discloses is untrue. |
| 3 | low | `SP:4-6` | `safe_path`'s module doc says a `[step.provenance].findings` ref is joined onto the plan directory and read; it is shape-checked only and never read. |
| 4 | low | `CL:7-35` | The 0.0.2 section omits the release's own README `agent-flow` rename notice, the most consequential user-visible fact in the release. |
| 5 | low | `MF:296-298` | "the fallback is unreachable" is true on Unix and false on Windows, where a prefix-bearing `dest` reaches it. |

Five findings, all `low`. No `medium`, no `high`, no `critical`.

Nothing in the four rewritten claim sites is false. All four survive every falsification I could construct, including at syscall level. The findings are in prose the fix pass did NOT rewrite, whose truth the fix pass changed by adding the second rule, plus one CHANGELOG omission.

## Finding 1 (low): the literal callers are documented as immune to a refusal that now fires on them

QUOTED, `src/manifest.rs:485-487`:

> Parse the pack's `pack.toml` manifest. The path is a fixed literal, so the containment refusal in `read` cannot fire here; it is mapped rather than special-cased so this stays a plain read.

QUOTED, `src/manifest.rs:429-430`, inside `PackSource::read`'s doc:

> the fixed `pack.toml`, `principles.toml` and `instrument.md` literals pass through too and can never escape

Both were true while the rule was lexical: a fixed literal such as `pack.toml` is relative and carries no `..`, so it could never be refused. The resolved rule decides where the literal LANDS, and a pack that ships its control file as a symbolic link lands outside.

EVIDENCE, measured against the HEAD binary. A pack whose `pack.toml` is `../shared/pack.toml`:

```
error: `pack.toml` is not a contained pack path (it resolves outside the pack directory,
through a symbolic link); a pack path must be relative, carry no `..` component, and
resolve to a location inside the pack directory
exit=2
```

The refusal fired, at the exact call site whose doc says it cannot.

The other two literals produce a quieter result, because their callers discard the error. With `principles.toml` a symlink to a shared file outside the pack, the run exits 0 and the scaffolded `{{principles}}` block is EMPTY (`main.rs:229-233` maps every `Err` to an empty principle set). With `instrument.md` a symlink, `--instrument` renders an empty block (`main.rs:259`, `unwrap_or_default`). Both measured: exit 0, file dropped, content silently missing.

WHY IT MATTERS. The sentence tells a maintainer the branch is unreachable, which invites replacing the mapping with an `expect` or deleting the case, and it is the reason nothing in the release discloses that a directory pack assembled with symlinked control files is now refused (loudly for `pack.toml`) or silently degraded (for the other two). This is also the reading under which `CHANGELOG.md:32`'s closing sentence, "Each caller labels the refusal with its own field", is false: the literal callers label it with no field at all. Under the bullet's own stated two-field scope that sentence is true, so I do not raise it separately.

The silent degradation is a behaviour question rather than a claims one and I am not ruling on it. The triage may want to weigh it on its own terms, since a scaffolded `AGENTS.md` with no principles at exit 0 is a wrong output that the run's own report cannot show.

## Finding 2 (low): the error-type docs still describe the one-rule world

QUOTED, `src/manifest.rs:378-382`:

> Why a `PackSource::read` did not return the file's bytes: the relative path was refused because it leaves the pack, or the read itself failed.
>
> A refusal is NOT an I/O failure and is not typed as one. The path never reached the filesystem, so reporting it as a read error would describe an attempt that never happened

QUOTED, `src/manifest.rs:389-390`:

> `rel` is not contained by the pack directory: it is absolute, or it carries a `..` component. Refused BEFORE the read, so the file outside is never opened.

QUOTED, `src/manifest.rs:191-192` (`UnsafeAssetSource`) and `:200-201` (`UnsafeModuleGuidance`), the same enumeration:

> is not contained by the pack directory: it is absolute, or it carries a `..` component

EVIDENCE. Three separate falsifications:

1. "the relative path was refused because it leaves the pack" is the exact assertion round 1's `A4` found untrue and this pass removed from all four MESSAGES. It survives here. `a/../b.md` is refused and leaves nothing: measured as a `source`, a `guidance` and a `dest`, all three naming the `..` component and asserting nothing about leaving.
2. "The path never reached the filesystem" is falsified by the strace of a resolved refusal: `readlink(".../pack/link.md", "../secret.md") = 12` followed by `readlink(".../secret.md")`, both before the refusal. `PackSource::read`'s own doc, 60 lines later at `MF:442-443`, states the opposite ("this site stats and follows links before it refuses"), so the file now contradicts itself. The narrower claim the fix pass took care to preserve, that the CONTENTS are never read, does hold.
3. The three "it is absolute, or it carries a `..` component" enumerations are now incomplete. `link.md` is neither and produces all three variants. Round 1's triage checked the `ReadError::Escapes` doc and ruled it TRUE, on the recorded ground that "the variant is only produced for those shapes as shipped". The fix pass removed that ground and did not revisit the sentence.

`UnsafeAssetDest`'s identical enumeration at `MF:212-213` is still correct and must not be swept up in a fix: the write side applies the lexical rule only, so absolute and `..` really are its whole cause set.

## Finding 3 (low): `safe_path` says a findings ref is read, and it is not

QUOTED, `src/safe_path.rs:4-8`:

> Several callers join such a string onto a directory they own: `plan::source` joins a `[meta].sidecars` front/tail ref (and a `[step.provenance].findings` ref) onto the plan directory to READ it

EVIDENCE. The sidecar half is true: `render.rs:167` and `:169` join the front and tail refs onto the base directory and read them. The parenthetical is false. A findings ref is never joined onto anything and never opened. The code says so twice, in the same struct the module doc points at:

- `src/plan/source.rs:235-237`: "`commits` and `findings` are external references (a git object, a findings artifact on disk), so they are shape-checked only, never resolved, keeping `validate_source` a pure function over the string."
- `src/plan/source.rs:245-247`: "Shape-checked via `is_safe_sidecar_ref`, NOT existence-checked (a findings file is committed then deleted at task close, so a valid historical pointer may name an absent path)."

Measured: I set a step's `findings` to a value naming no file at all, ran `validate --source` (accepted, exit 0) and `render` (exit 0), and the value appears in the Roadmap Notes cell as text. Nothing was opened.

WHY IT MATTERS. This module doc is the map a later change will navigate by, and the same file (`SP:19-21`) offers `resolved_within` for the plan-side follow-up round 1's remedy anticipates. A maintainer who believes a findings ref is read may apply the resolved rule to it, which requires the path to exist and would break the deliberate not-existence-checked contract above. The real reason a findings ref is shape-checked is different from the reason a sidecar ref is, and the doc currently erases that difference.

## Finding 4 (low): the 0.0.2 section omits the release's rename notice

QUOTED, `CHANGELOG.md:3`:

> All notable changes to this project will be documented in this file.

The 0.0.2 section (`CHANGELOG.md:7-35`) has `Added`, `Changed` and `Fixed`, and every code change in `main..HEAD` maps to a bullet. One user-visible change in the same diff does not: `README.md:7-11`, added by this release, announces that the project is being renamed to `agent-flow`, that the crate name is reserved, that releases move there once the rename lands, and that `agent-scaffold` is then reclaimable by opening an issue.

EVIDENCE. I enumerated the user-visible changes in `git diff main..HEAD` and matched each to a bullet: `F1` -> `CL:35`, `F4` -> `CL:34`, `F4b` and `A1` -> `CL:32`, `A4` -> `CL:33`, the version bump -> the heading. The README rename section matches nothing. Both files ship in the published artifact, so a reader of the 0.0.2 changelog learns about four containment fixes and not that the crate they are pinning is moving.

The step's release mechanics (`ship-v0-0-2.md:112-114`) close the changelog at step 2 and add the README items at step 4, which is how the gap arose, and the spec does not require a changelog entry for them. So this is a judgement about the CHANGELOG's own coverage claim rather than a missed acceptance criterion, and I flag it as the weakest-grounded of the four content findings. A `Deprecated` bullet naming the move is the whole remedy.

## Finding 5 (low): "the fallback is unreachable" is false on Windows

QUOTED, `src/manifest.rs:296-298`:

> The write side applies the lexical rule only, so every refusal here has a lexical cause and the fallback is unreachable. It is worded as an absence rather than as a resolution claim the write side does not make.

EVIDENCE. `UnsafeAssetDest` fires exactly when `is_contained_relative(dest)` is false (`MF:684`), and the cause comes from `lexical_failure(dest)` with a fallback for `None`. The two disagree on a path carrying a `Prefix` component:

- `is_contained_relative` requires every component to be `Normal` or `CurDir`, so a `Prefix` component makes it false.
- `lexical_failure` returns `Some` only for `is_absolute()` or a `ParentDir` component. On Windows `Path::new("C:foo.md").is_absolute()` is false, because a Windows path is absolute only with a prefix AND a root, and it carries no `..`. So it returns `None` and the fallback fires.

On Unix the complement is empty and the sentence is true, which is what I measured: no dest I could construct reaches the fallback. I cannot run Windows here, so this rests on documented `std::path` semantics rather than on a run, and it is the weakest finding in this set. The impact if left alone is only that a maintainer trusting "unreachable" may replace the fallback with an `unreachable!()` or an `expect`, turning a tautological message into a panic on a supported platform. The code compiles for Windows deliberately (`MF:61-65`, `main.rs` `make_executable`, and the `#[cfg(not(unix))]` arm inside the read-site unit test), so the platform is not hypothetical.

## Out-of-scope observations

Not findings. Recorded so the triage can see they were considered.

1. `CHANGELOG.md:34`'s "so the read boundary and the write boundary cannot drift apart" is true on its own antecedents, which are the sidecar READ boundary and the dest WRITE boundary, and those do share one predicate. A reader who carries over the previous bullet's "This is the read half of the `dest` write escape below" may take "the read boundary" to be the pack read boundary, which now applies a strictly stronger rule than the write side. The sentence is defensible as written and I do not raise it; a two-word narrowing to "the sidecar read boundary" would remove the ambiguity.
2. `MF:427-429`'s "Every path a pack author writes reaches the filesystem through here" excludes `[[asset]].dest`, which a pack author also writes and which reaches the filesystem at `apply_asset`. The parenthetical immediately after defines the set as `source`, `guidance` and the three literals, and round 1 ruled the neighbouring "THE ONE containment boundary" sentence can stand. Not raised.
3. The resolved rule is not race-free. `resolved_within` returns the canonical path and the read uses it, which closes the ordinary swap-the-symlink race, but a pack directory mutated concurrently with the run can still move a component between the canonicalise and the read. A pack is static during a scaffold in every realistic setting, and no claim in the change asserts atomicity, so this is a note rather than a finding.
4. A hard link inside the pack to an outside file is read. Its canonical path is inside the pack, so the rule's own terms ("the location it lands on") are satisfied; git cannot ship a hard link, and a pack author who can create one can equally copy the file. Not a gap in the claim.
5. `README.md:9`'s "that crate name is reserved at <https://crates.io/crates/agent-flow>" and "every published `agent-scaffold` version stays installable" could not be verified: crates.io returns 403 from this environment. They are release criteria 2 and 3 in the spec and need a network-capable check.
6. Round 1's `A2`, `A3`, `A5`, the audit's `F2`/`F3`, the pre-existing `superseded by` projection defect, the rename, ANSI escapes in a `dest`, and the plan-side sidecar symlink hole were excluded from this round and are not raised. I did re-measure `A5` incidentally (an escaping `source` on an unselected module still loads at exit 0, nothing read, nothing leaked) and the pre-existing `superseded by` problem (it is the only `validate --plan` problem on my injected fixture), and both behave exactly as round 1 recorded.
