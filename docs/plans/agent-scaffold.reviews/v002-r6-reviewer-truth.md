# `ship-v0-0-2-inc1` round 6 (verification, beyond the cap): REVIEWER, truth lens

Independent reviewer. I did not write this change and I did not review rounds 1 to 5. My lens is whether what REMAINS in the 0.0.2 section is true. A second reviewer holds the question of whether a deletion removed something that mattered.

Every figure below is my own measurement, made in this session, against a binary I built myself. Where I reproduce an earlier reviewer's result I say so and give my own numbers.

## Artifact and commits

- Worktree `.claude/worktrees/r6-truth`, detached at `ba466c2` ("docs: cut the release notes to what is true of the published v0.0.1").
- Reviewed: `git diff HEAD~2..HEAD`, 2 files, 4 insertions, 18 deletions. Two commits: `6050f93`, a pure `render` regeneration of `docs/plans/agent-scaffold.md`; and `ba466c2`, the release-notes cut.
- The 0.0.2 section went from 21 bullets to 15: seven deleted, one authored, two edited in place.
- Specification for the cut: `docs/plans/agent-scaffold.reviews/v002-r5-triage.md`, which enumerated the 21 bullets against the tag and found 4 fully true.
- `git status --porcelain` is empty in this worktree and in the main repository, apart from this file.

## Method, and which binary produced which result

The rule for this round is that a claim about 0.0.1 is checked against the PUBLISHED 0.0.1, not against a branch. Two release binaries, one `CARGO_TARGET_DIR` each, from two separate `git archive | tar -x` extracts, verified distinct by `md5sum`:

```
69d7c9270e3e82230834c9c5bb0817ff  tgt-tag/release/agent-scaffold    (tag v0.0.1)   --version: agent-scaffold 0.0.1
8ee8c5495d3f86458904d33613718a7c  tgt-head/release/agent-scaffold   (ba466c2)      --version: agent-scaffold 0.0.2
```

`v0.0.1` is an ANNOTATED tag: `git rev-parse v0.0.1` gives the tag object `0f33878`, and the commit is `2bbce2e` (2026-07-10), which agrees with round 5. From `v0.0.1` to this HEAD, `git rev-list --count` reports 977 commits and `git diff --shortstat -- src/ Cargo.toml pack/` reports 74 files changed, 19,659 insertions, 465 deletions.

Below, TAG means the 0.0.1 binary and HEAD means the 0.0.2 binary. Every claim about 0.0.1 behaviour in this document was produced by TAG. I used `main` as a proxy for nothing.

Every fixture, symbolic-link target and target directory sits under my own scratch subdirectory. I set `chmod 000` at two fixtures and restored both to `644`, verified by `stat -c %a`. No other permission bits were touched.

Gates I ran myself at HEAD, with the HEAD binary, for context rather than as findings:

| gate | result |
| --- | --- |
| `render --check --strict docs/plans/agent-scaffold.plan.toml` | `up to date`, exit 0. Round 5's `T1` is fixed by `6050f93` |
| `validate --source ... --metrics ...` | `340 records, valid`, `99 steps, 76 questions, valid`, exit 0 |
| `validate --source ... --metrics ... --workflow` | `workflow invariants hold`, exit 0 |

## The fifteen surviving bullets

Line numbers are `CHANGELOG.md` at `ba466c2`. For each bullet I state the case I constructed to falsify it and what running that case measured.

| # | line | bullet | falsifying case tried | measured result |
| --- | --- | --- | --- | --- |
| 1 | `:11` | Deprecated, the `agent-scaffold` name | Does `README.md` carry the "The `agent-flow` rename" section the bullet defers to, including a contact route? | TRUE. `README.md:7` is `## The `agent-flow` rename`; `:11` gives the contact route (open an issue on the named repository). Non-comparative, so the tag is not implicated. |
| 2 | `:15` | `agent-scaffold audit` | Run `audit --json` and `audit` at HEAD. Does `--json` write a file? Does the non-JSON form write `docs/plans/<task>.code-value-report.md`? Does the report lead with the caveat? Is the report empty and are all three signal harvests deferred, as the last clause says? | **CONTAINS ONE FALSE CLAIM.** `--json` writes no file (file set byte-identical before and after) and prints the intermediate; the non-JSON form writes the named report and `--out` redirects it; the report leads with the caveat and carries `generated_from`, and an absent signal widens the caveat in the report's own words. BUT the last clause is false. See `R1`. |
| 3 | `:16` | `type:"waiver"` record and W5 | Are the record, the named `reason` and `evidence_tier` tokens, `parse_waivers`, `parse_escalations` and W5 present at HEAD and absent at the tag? | TRUE as an addition. `src/workflow.rs` carries W3 (40 references), W4 (14) and W5 (37); the tokens `predates-logging`, `review-skipped`, `accepted-at-escalation`, `self-declared`, `record-backed` are all in `src/metrics.rs`. The tag has NO `src/metrics.rs`, NO `src/workflow.rs` and NO `pack/instrument.md`. |
| 4 | `:17` | `type:"decision"`, `type:"baseline"`, W4 | Is `chosen` actually enforced as a member of `options`, or merely documented? | TRUE as an addition. `src/metrics.rs:511-514` reads `let chosen = require_str(obj, "chosen")?; if !options.contains(&chosen)` and errors with the offending value. `parse_decisions` and `parse_baseline` both exist. Absent at the tag. |
| 5 | `:18` | `validate --workflow` cross-reference | Run `validate --workflow` and see whether a `complete` step with no round records is actually reported. | TRUE as an addition, measured. HEAD reports `Roadmap step `core-assets` is `complete` but has no round records and no covering waiver`, which is also the bullet at `:16`'s "convergence-OR-waiver" wording in the message itself. `validate` does not exist at the tag (exit 2). |
| 6 | `:19` | `trivial`/`grandfathered`, RETIRED in the same cycle | The strongest falsification is that the statuses still work. Are they accepted at HEAD? | TRUE, measured. `src/plan.rs:559-560` asserts `!roadmap_status_ok("trivial")` and `!roadmap_status_ok("grandfathered")`. `skipped` remains a status, as the bullet says. Net effect on a 0.0.1 user is nothing, and the bullet says so. |
| 7 | `:20` | `--module isolation` | The bullet says it drops NO files into the project tree. Count them. | TRUE, measured. A no-module scaffold and a `--module isolation` scaffold both write 31 files; the only two differing paths are `AGENTS.md` and `.agents/AGENTS.reference.md`. The `ab new`/`ab spawn`/agent-box/agent-images pointers appear in the rendered `AGENTS.md` (16 matching lines). |
| 8 | `:21` | Optional modules and `--module checks` | Does `--module checks` actually scaffold the four things named? Does `{{modules}}` leave a literal placeholder when no module is selected? | TRUE, measured. `--module checks` adds exactly `.agents/checks.toml`, `.agents/checks/`, `.agents/hooks/` and `.agents/prompts/checks-reviewer.md`. Zero literal `{{modules}}` remain in a no-module render. |
| 9 | `:25` | BREAKING, the subcommand CLI (the one authored bullet) | Eight clauses, each run separately. See the next section. | TRUE on seven clauses. One universal quantifier has a single literal exception with no user impact. See `R4`. |
| 10 | `:26` | Hardened the scaffolded workflow guidance | Every sub-claim checked as a present/absent pair between the tag's `pack/AGENTS.md` and HEAD's. | TRUE, measured, in full. Tag `:95-108` reads "The ledger is transient working state, keep it as scratch notes (not in the plan), discard it when the task closes" and "a clean round ends the loop; the default of three contested rounds triggers escalation". HEAD `:63` reads "keep it in a file tracked in version control beside its plan ... and commit it, so it survives the orchestrator losing context and travels across machines and sessions". `total-round cap` and `consecutive clean`: 0 hits at the tag, 2 each at HEAD; `intake`: 0 at the tag, 5 at HEAD. The cap default is `cap = 5` in `pack/workflow.toml:26`, matching "(default five)". The acceptance triager is HEAD `:33`, absent from the tag's `:55`. The dismissal backstop is HEAD `:59`, absent at the tag. |
| 11 | `:27` | Split review and triage in the README diagram | Read both diagrams. | TRUE, measured. Tag `README.md:87` has ONE node, `preview["Review the plan, then triage<br/>(reviewers, triager)"]`. HEAD `README.md:73-74` has two, `preview["Review the plan<br/>(reviewers)"]` and `ptriage["Triage the findings<br/>(triager)"]`. |
| 12 | `:28` | Generalised the diversity guidance | The bullet quotes a "from" string. Does the tag contain it verbatim? | TRUE, measured, verbatim. Tag `pack/AGENTS.md:28-29` reads "different models\nwhere available, since same-model reviewers share blind spots". HEAD `:21` reads "different models or harnesses where available, since same-model and same-harness reviewers share blind spots". Both the quoted "from" and the quoted "to" match. |
| 13 | `:32` | A pack path can no longer read outside the pack | All three shapes run at the TAG through `[[asset]]`'s `source`, plus every surviving side-claim: the two shapes that keep working, the shape that stops, the `cp -rL` recourse, the single containment site, and the message structure. | FACTUALLY TRUE on every claim I could run. One referential defect the cut created. See `R3` and the detail below. |
| 14 | `:33` | The literal-name files are contained too | Symlink `principles.toml` and `pack.toml` out of the pack at the TAG; invalid UTF-8 and `chmod 000` at both; malformed TOML at both. Then ask whether the enumeration is complete at HEAD. | TRUE on every 0.0.1 claim, measured. The enumeration is incomplete as a statement of HEAD. See `R2` and the detail below. |
| 15 | `:34` | A pack `[[asset]]`'s `dest` | Both shapes at the TAG; both at HEAD under `--write` AND `--dry-run`; a disabled module's escaping `dest`; whether stdout carries a plan line before the refusal. | **TRUE, MEASURED, IN FULL.** The only bullet in the section on which I could not falsify a single clause. |

### `:32` in detail, the read escape

Run at the TAG, a pack whose `[[asset]]` `source` escapes by each of the three shapes:

```
source = "../outside/secret.md"    TAG exit 0, plan line `create  leaked.md`, leaked.md = TOP-SECRET-OUTSIDE-THE-PACK
source = "<absolute path>"         TAG exit 0, plan line `create  leaked.md`, leaked.md = TOP-SECRET-OUTSIDE-THE-PACK
source = "link.md" -> outside      TAG exit 0, plan line `create  leaked.md`, leaked.md = TOP-SECRET-OUTSIDE-THE-PACK
```

So "All three shapes leaked through an `[[asset]]`'s `source`, where the run exited 0, printed its ordinary `create <dest>` plan line, and copied the outside file into the scaffolded project" is true of the published release in every one of its four parts.

HEAD refuses all three at exit 2, and the message structure the bullet promises (value, then cause in parentheses, then the rule) holds exactly:

```
asset source `../outside/secret.md` is not a contained pack path (it carries a `..` component); a source must be relative, ...
asset source `/tmp/.../secret.md` is not a contained pack path (it is an absolute path); a source must be relative, ...
asset source `link.md` is not a contained pack path (it resolves outside the pack directory, through a symbolic link); a source must be relative, ...
```

The three parenthesised causes are the three the bullet enumerates, in the bullet's own words.

Side-claims, each run:

```
pack-INTERNAL link (internal.md -> sub/real.md)      TAG exit 0, HEAD exit 0, body copied     keeps working: TRUE
--template naming a link TO the pack directory       TAG exit 0, HEAD exit 0                  keeps working: TRUE
stow-style pack, every file a link outside           TAG exit 0 (scaffolds), HEAD exit 2      stops working: TRUE
   and the HEAD refusal names `pack.toml`, so it IS the first thing you hit: TRUE
recourse `cp -rL` on that same pack                  HEAD exit 0                              TRUE
```

Two source claims I checked rather than assumed. "The containment is applied once at `PackSource::read`, the single site every pack path reaches the filesystem through": `read_optional` at `src/manifest.rs:532` is `match self.read(rel)`, so it inherits rather than duplicating. TRUE. "`PackSource::Embedded` gets no check and needs none: it resolves against a compile-time map and touches no filesystem": `src/manifest.rs:483-484` is `PackSource::Embedded(dir) => dir.get_file(rel)`. TRUE.

### `:33` in detail, the literal-name files

Run at the TAG:

```
principles.toml -> ../outside/principles.toml    TAG exit 0, rendered AGENTS.md = "P:1. LEAKED-PRINCIPLE - OUTSIDE-PRINCIPLE-SUMMARY"
                                                 the outside file WAS read.  HEAD exit 2, names principles.toml.
pack.toml       -> ../outside/pack.toml          TAG exit 0, and it OBEYED the outside manifest: it created `PLANTED.md`,
                                                 a dest declared only in the outside manifest.  HEAD exit 2, names pack.toml.
principles.toml invalid UTF-8                    TAG exit 0, stderr EXACTLY 0 bytes, rendered "P:" (empty principle set)
                                                 HEAD exit 2, "could not read the pack's principles.toml: stream did not contain valid UTF-8"
principles.toml chmod 000                        TAG exit 0, stderr EXACTLY 0 bytes, rendered "P:"
                                                 HEAD exit 2, "could not read the pack's principles.toml: Permission denied (os error 13)"
principles.toml malformed TOML                   TAG exit 2, "TOML parse error at line 1, column 6"
                                                 HEAD exit 2, byte-identical message.  "already loud in 0.0.1": TRUE
```

So every claim the bullet makes about 0.0.1 is true of the published 0.0.1, and the "it now exits 2 naming the file" claim holds for both unreadable variants. The tag's `src/main.rs:181` swallows the read error with `Err(_) => Ok(Vec::new())`, which is the mechanism behind the silent empty set.

The defect is on the HEAD side of the sentence, not the 0.0.1 side. See `R2`.

## The authored bullet, clause by clause

`CHANGELOG.md:25`. Eight checkable claims. Each was run separately; TAG and HEAD as defined above.

| # | clause | falsifying case | measured |
| --- | --- | --- | --- |
| C1 | "the command line is now subcommand-based" | Compare usage lines. | TRUE. TAG: `Usage: agent-scaffold [OPTIONS]`. HEAD: `Usage: agent-scaffold <COMMAND>`. |
| C2 | "In 0.0.1 `agent-scaffold` took its options directly, as `agent-scaffold --output-dir <dir> --write`" | Run it at the TAG. | TRUE. Exit 0, 11 files written. |
| C3 | "or `agent-scaffold --list-principles`" | Run it at the TAG. | TRUE. Exit 0, prints the numbered selection. |
| C4 | "each of those now exits 2 with `Usage: agent-scaffold <COMMAND>`" | Run BOTH at HEAD and check the exit code AND the usage string. | TRUE for both. `--output-dir ... --write`: exit 2, `error: unexpected argument '--output-dir' found`, `Usage: agent-scaffold <COMMAND>`. `--list-principles`: exit 2, `error: unexpected argument '--list-principles' found`, same usage line. |
| C5 | "Insert `scaffold` to restore them, as `agent-scaffold scaffold --output-dir <dir> --write`" | Run both restored forms at HEAD. | TRUE for both. `scaffold --output-dir <d> --vcs none --write`: exit 0, 31 files. `scaffold --list-principles`: exit 0, same selection output as the TAG. |
| C6 | "and every option 0.0.1 documented is still accepted there" | Enumerate the option groups both ways (the CHANGELOG's own 0.0.1 section, and the TAG's `--help`) and run every one under `scaffold`, including documented VALUES. | TRUE under the CHANGELOG reading, ONE literal exception under the `--help` reading. See below and `R4`. |
| C7 | "The other subcommands, `validate`, `status`, `next`, `checks`, `render` and `audit`, are all new in this release" | Is the list of "other subcommands" correct and complete? Does each exist at HEAD? | TRUE. HEAD's command list is exactly `scaffold validate status next checks render audit help`; the six named are the non-`scaffold`, non-`help` commands, and `<cmd> --help` exits 0 for each. |
| C8 | "none of them exists in 0.0.1" | Run all six at the TAG. | TRUE. Every one exits 2 with `error: unexpected argument '<name>' found`. (`scaffold` and `help` do too.) |

### C6 enumerated

The CHANGELOG's own `## [0.0.1]` section names nine options: `--principles`, `--principle-detail`, `--list-principles`, `--write`, `--dry-run`, `--force`, `--template`, `--var`, `--vcs`. The TAG's `--help` documents ten functional options (those nine plus `--output-dir`) and two clap built-ins (`-h, --help` and `-V, --version`).

Run under HEAD's `scaffold`, with the values 0.0.1 documented:

```
--output-dir <d>                     OK      --principles default / all / none     OK
--force                              OK      --principles <id> and <id>,<id>       OK
--vcs git / --vcs none               OK      --principles tag:universal            OK
--write (real write, exit 0)         OK      --principle-detail name/summary/full  OK
--dry-run                            OK      --list-principles                     OK
--template <0.0.1-era pack dir>      OK      -h                                    OK
--var k=v, and repeated --var        OK      -V / --version                        EXIT 2
```

`scaffold --help` lists exactly the ten 0.0.1 functional options plus three new ones (`--module`, `--instrument`, `--with-precommit-hook`). All nine options the release notes' own 0.0.1 section names are accepted, as are all ten the TAG's help documents. `--template` was run against the TAG's own `pack/` directory copied out verbatim, and HEAD scaffolds from that 0.0.1-era pack at exit 0, so the flag is not merely parsed.

The single exception is `-V` / `--version`, which the TAG's `--help` documents as an option and which HEAD's `scaffold` rejects at exit 2. It remains accepted at the top level (`agent-scaffold --version` prints `agent-scaffold 0.0.2`).

## Verdicts

FOUR findings, all `low`. No `critical`, no `high`, no `medium`.

| id | severity | class | site | one line |
| --- | --- | --- | --- | --- |
| `R1` | low | USER-FACING | `CHANGELOG.md:15` | The audit bullet says the source-scan signal harvest is a later increment. It ships in this release and emits records on this project's own tree. |
| `R2` | low | USER-FACING | `CHANGELOG.md:33` | The cut made the literal-name enumeration incomplete: `instrument.md` is a third literal-name read at HEAD and is contained by the same rule. |
| `R3` | low | TEXT | `CHANGELOG.md:32` | "neither" now has no antecedent: the cut removed the second of the two fields it referred to. |
| `R4` | low | TEXT | `CHANGELOG.md:25` | "every option 0.0.1 documented is still accepted there" has one literal exception, `-V`/`--version`. |

Only `R1` is a false factual claim. `R2` is an incomplete enumeration in the under-claiming direction, and `R3` and `R4` are defects of reference and of a universal quantifier.

`R2` and `R3` are the two defects the cut itself created, and both are at `:32` and `:33`, the only two surviving bullets the cut edited in place rather than leaving alone. The twelve untouched survivors produced one finding between them (`R1`), and that one predates the cut.

## `R1` (low, USER-FACING): the audit bullet defers a signal harvest that shipped

`CHANGELOG.md:15` ends:

> This first increment ships the schema, the projection, and the caveat with an empty report; the signal harvests (rustc dead-code, the suppression-marker and FFI source scan, and `cargo-machete` unused dependencies) are later increments.

Run at HEAD against HEAD's own extracted source tree:

```
agent-scaffold audit --json
  "generated_from": { "rustc_dead_code": false, "source_scan": true, "cargo_machete": false }
  record count: 6, every one of kind "declared-reason"
  first record: {"kind":"declared-reason","span":{"file":"src/checks.rs","line":155},"symbol":"budget",
                 "marker":"allow","reason":"parsed for the schema; used by the later mutation module"}
```

The Markdown report says the same thing in its own words: "Signals run: source scan. Signals not run (their coverage is absent, widening the caveat above): rustc dead-code, cargo-machete."

So the bullet is right about two of the three named harvests and wrong about the third. The suppression-marker source scan is not a later increment; it is in this release, it ran, and it produced six records with real `file:line` spans. "with an empty report" is likewise false on any project carrying a suppression marker: my empty scratch project gave an empty report, this project's own source gave six records.

Everything else in the bullet is true, measured: `--json` writes no file (the project's file set is byte-identical before and after the run) and prints the intermediate to stdout; the non-JSON form writes `docs/plans/task.code-value-report.md` and `--out` redirects it; the report leads with the caveat; `generated_from` is present and an absent signal widens the caveat rather than reading as a clean pass; `AuditRecord` is an enum whose first variant is `DeadCode` at `src/audit.rs:149-153`.

SEVERITY `low`. The error under-claims the release rather than over-claiming it, so no reader is left less careful than they should be, nothing breaks, and no result is wrong. It is the class round 4's `P2` and round 5's `T2` were both rated `low` for: a false factual sentence in the shipped release notes. The one aggravating feature, which is why I raise it rather than record it as an observation, is that this claim is about THIS release's own contents rather than about a predecessor, so any reader can refute it with one command against the binary the notes ship with.

The pure-deletion fix is available. Deleting "the suppression-marker and FFI source scan, and" from the parenthesis, and "with an empty report" from the clause before it, leaves a sentence I measured true.

## `R2` (low, USER-FACING): the literal-name enumeration is now incomplete

`CHANGELOG.md:33` opens: "The files the tool reads by literal name, `pack.toml`, `principles.toml`, are contained too". Before the cut it read "The three files the tool reads by literal name, `pack.toml`, `principles.toml` and `instrument.md`".

At HEAD there are THREE production reads by literal name, not two:

```
src/manifest.rs:547   self.read("pack.toml")
src/main.rs:264       source.read_optional("principles.toml")
src/main.rs:298       source.read_optional("instrument.md")
```

The third is behind `--instrument`, which is why my first attempt did not reach it. Run with the flag:

```
instrument.md -> ../outside/instrument.md,  HEAD scaffold --instrument
  exit 2, "could not read the pack's `instrument.md`: `instrument.md` is not a contained pack path
           (it resolves outside the pack directory, through a symbolic link); ..."
pack-internal instrument.md,                HEAD scaffold --instrument
  exit 0, rendered "I:INSIDE-PACK-INSTRUMENT"
```

So `instrument.md` is a literal-name read AND it is contained by the same rule, and HEAD's own test at `src/manifest.rs:1347-1350` loops over exactly `["principles.toml", "instrument.md"]` to prove it. The definite plural plus the two-item apposition reads as a complete enumeration, and at HEAD it is not one.

Why the cut produced this. `instrument.md` had to leave the SECOND sentence, because round 5 measured "In 0.0.1 each was read through a symbolic link out of the pack" false of it (the tag's `src/` contains zero occurrences of `instrument`, which I confirmed). Removing it from the first sentence as well was one deletion too far: the first sentence is about HEAD, where the claim is true, and only the second is about 0.0.1.

SEVERITY `low`. The direction is under-claiming: the notes credit the fix with covering two files when it covers three. Nobody is told something is protected that is not, and the omitted file is the one a 0.0.1 user has never heard of, since `--instrument` does not exist at the tag (exit 2). This is round 4's `P4` class, an omission with nothing false published, except that the definite article makes it slightly more than an omission.

The fix restores `instrument.md` to the first sentence only, leaving the "In 0.0.1" sentence scoped to the two files that existed. That is not a pure deletion, so if the loop's deletion-only discipline is being held to strictly, recording this as a residual is defensible on `P4`'s ground.

## `R3` (low, TEXT): "neither" lost its antecedent

`CHANGELOG.md:32` still ends: "Each caller labels the refusal with its own field, and neither reports as a failed read, since nothing was opened."

Before the cut, "each caller" and "neither" referred to the two pack-controlled fields the bullet had named a few sentences earlier, `[[asset]]`'s `source` and `[[module]]`'s `guidance`. The cut correctly removed the `guidance` half, because round 5 measured it false of 0.0.1 (the tag has no `--module` flag and no `module` section in its manifest). It also removed the clause "so an escaping `guidance` reports as a module guidance problem and never as an asset `source` one", which was the other half of the pair.

The word "guidance" no longer appears anywhere in `:32`. The bullet now names exactly ONE field, and then says "neither".

The underlying claim is still true of the code. Each site labels its refusal with its own field, which I measured across four of them: `asset source \`...\``, `module \`leaky\` guidance file ...`, `` `principles.toml` ``, `` `pack.toml` ``. And none reports as a failed read. So this is a defect of reference, not of fact.

SEVERITY `low`. It costs a reader a re-read and nothing else. It is the same class as round 3's text findings. The pure-deletion fix is to cut ", and neither reports as a failed read, since nothing was opened", or to replace "neither" with "no refusal".

## `R4` (low, TEXT): one literal exception to "every option"

`CHANGELOG.md:25` says "every option 0.0.1 documented is still accepted there", where "there" is under `scaffold`.

Under the reading a reader of these notes will most likely take, the options the release notes' own `## [0.0.1]` section documents, the claim is TRUE without exception: all nine are accepted, with their documented values. Under the other available reading, the options the 0.0.1 binary's `--help` documents, eleven of twelve are accepted and `-V` / `--version` is not:

```
agent-scaffold scaffold -V           exit 2, error: unexpected argument '-V' found
agent-scaffold scaffold --version    exit 2, error: unexpected argument '--version' found
agent-scaffold --version             exit 0, agent-scaffold 0.0.2
```

SEVERITY `low`, and I raise it only because of what this loop's record says about universal quantifiers in this section. Round 5 measured "TWO pack-controlled fields" and "The three files the tool reads by literal name" false, and both were counting quantifiers in the same three bullets. "every option" is the third. The impact here is nil, because nothing is lost: `--version` still works at the top level, which is where a user types it.

If it is worth an edit, the pure-deletion form is "and the options 0.0.1 documented are still accepted there", which drops the universal without adding a clause. Recording it as a residual is entirely reasonable.

## The file-count discrepancy, settled

The orchestrator asked me to rule between the implementer's 11-and-31 and the orchestrator's own 3-and-3 for `--output-dir <d> --vcs none --write`.

**The implementer is right, and the two of you measured different things.** Both numbers are correct for what they count.

```
TAG   --output-dir <d> --vcs none --write
  stdout: 11 `create` plan lines, then "Wrote to <d> (11 changed, 0 left untouched)."
  find <d> -type f   ->  11
  ls -A <d>          ->   3      (AGENTS.md, .agents, docs)

HEAD  scaffold --output-dir <d> --vcs none --write
  stdout: 30 `create` plan lines plus one `render  docs/plans/TEMPLATE.md` line,
          then "Wrote to <d> (30 changed, 0 left untouched)."
  find <d> -type f   ->  31
  ls -A <d>          ->   3      (AGENTS.md, .agents, docs)
```

11 and 31 are files written. 3 and 3 is the number of TOP-LEVEL entries in the output directory, which is 3 at both versions because both scaffold into `AGENTS.md`, `.agents/` and `docs/`. The tag writes `AGENTS.md`, `docs/plans/TEMPLATE.md`, seven role prompts, `.agents/AGENTS.reference.md` and `.agents/principles.toml`. HEAD adds the ten `TEMPLATE.*` sidecars plus two `.gitkeep` files, `.agents/LEDGER.template.md`, `.agents/workflow.toml`, six `.agents/user-prompts/*`, and renders `docs/plans/TEMPLATE.md` from the new `TEMPLATE.plan.toml`.

The tool's own summary says "30 changed" at HEAD rather than 31, because the `render` line is not counted as a change. Nothing published turns on any of these numbers: I grepped the 0.0.2 section and neither 11, 30 nor 31 appears in it.

## Did the cut introduce a new false claim by changing a survivor's meaning?

This was the fourth thing I was asked to check. I approached it two ways.

First, I diffed at word level to establish which surviving bullets the cut EDITED rather than merely surrounded. Exactly two: `:32` and `:33`. Everything else in the section is either untouched or the one authored bullet. So the search space for a meaning change is small and I read both bullets in full, before and after.

Second, I scanned the whole surviving section for anaphors and cross-references whose antecedent could have been deleted: `neither`, `both`, `the two`, `those two`, `three`, `above`, `below`, `the same rule`.

The results:

| site | reference | resolves? |
| --- | --- | --- |
| `:32` | "and **neither** reports as a failed read" | **NO.** The second of the two fields was deleted. This is `R3`. |
| `:32` | "**both** keep working", "beside **those two** that survive" | Yes, to the pack-internal link and the pack-directory link named in the same sentence. |
| `:32` | "the `dest` write escape **below**" | Yes, to `:34`, which survives. |
| `:33` | "refused by **the same rule**" | Yes, to `:32`, which survives. |
| `:34` | "**both** shapes are now refused", "the read boundary and the write boundary" | Yes, both internal to the bullet. |
| `:19` | "the unified `type:"waiver"` record **above**" | Yes, to `:16`, which survives. |
| `:16` | "a step with **neither** converging rounds nor a covering waiver" | Yes, internal. |
| `:11` | "so **the two** cannot drift" | Yes, the README section and this entry. |

I also checked whether the cut orphaned a named check. Before the cut, W3 was mentioned twice (in the surviving waiver bullet, and in the deleted W5 waiver-ownership bullet) and W5 twice. After, each is mentioned once. NEITHER was ever introduced by a bullet, before or after: both were already bare names when the section had 21 bullets. So the cut reduced the number of unintroduced mentions and created no new dangling name. This is inherited, not a finding against this change.

So: one meaning change, `R3`, and it is referential rather than factual.

## The rollup against round 5

Round 5 measured 4 of 21 bullets fully true against the published `v0.0.1`. Measuring the same way against the same binary:

```
fully true, no defect found:                    11 of 15
   :11 :16 :17 :18 :19 :20 :21 :26 :27 :28 :34

factually true, one non-factual defect:          3 of 15
   :25 (R4, a universal quantifier)
   :32 (R3, a dangling anaphor)
   :33 (R2, an incomplete enumeration)

carries a measured-false claim:                  1 of 15
   :15 (R1)
```

So 14 of 15 surviving bullets are factually true of the published release, against 4 of 21 before. Every one of round 5's five measured-false claims about 0.0.1 is gone, and I could not reconstruct any of them from what remains. The five bullets round 5 ruled VACUOUS are all deleted. The three bullets round 5 measured fully true (`:30`, `:31`, `:32` in its numbering, now `:26`, `:27`, `:28`) I re-measured independently and confirm, as I do `:38` (now `:34`), which remains the one bullet on which I could not falsify a single clause.

`T3`, round 5's undisclosed CLI break, is disclosed, and seven of the authored bullet's eight claims are true without qualification. `T1` is fixed: `render --check --strict` is exit 0 on this tree.

## Round outcome, from my lens

**NOT CLEAN, but only just.** Four findings, all `low`, none of which blocks a release on its own and three of which are defensible as recorded residuals under round 4's `P4` test.

If I were asked what a fix pass should touch, it is `R1` alone. It is the only false statement of fact, it is about this release's own contents, and it is repairable by pure deletion, which is the one operation this loop's record shows has never produced a finding across six sites in rounds 4 and 5 and now two more here. `R2`, `R3` and `R4` are a completeness defect, a pronoun and a quantifier; fixing `R2` requires restoring deleted words rather than deleting more, which cuts against the discipline this pass was given.

I say plainly what I could not falsify, since that is the more useful half of this report. Eleven of fifteen bullets survived every case I could construct against them, including all four Fixed bullets' claims about what the published 0.0.1 actually did, which is where five rounds of this loop found the most defects and where I spent most of my measurement.

## Out-of-scope observations

Real, reproducible, and NOT findings against this change.

1. **The commit message's baseline figures do not match this tree.** `ba466c2`'s message says "964 commits and 18,589 source insertions". From `v0.0.1` to `ba466c2` I measure 977 commits and 19,659 insertions over `src/ Cargo.toml pack/`. The difference is that the figures were quoted from round 5's measurement against a then-current `main` and HEAD has advanced since. Nothing shipped carries either number, and the CHANGELOG states no commit count at all, so nothing published is wrong. Raising it only so the record is not later mistaken for a measurement of this tree.

2. **`v0.0.1` is an annotated tag.** `git rev-parse v0.0.1` returns the tag object `0f33878`, not the commit. Anyone re-running this round's measurements should use `v0.0.1^{commit}` (`2bbce2e`) when comparing revisions, or they will get a confusing mismatch against round 5's recorded hash. `git archive v0.0.1` handles it correctly either way.

3. **`validate --plan` on this project's Markdown view exits 101.** Running the Markdown-path form rather than the TOML-primary gate reports the pre-existing `Q-43` `superseded by` problem plus a W3 complaint per legacy `complete` step, and exits 101 rather than a small conventional code. 101 is Rust's panic exit code, which makes a genuine panic and a validation failure indistinguishable to a CI gate reading the status. The project's own release gate uses `--source` and exits 0, so nothing here is blocked. This touches the `superseded by` projection defect, which is settled and which I am not reopening; the exit-code observation is the only part I have not seen recorded.

4. **`:16` names W3 and `:17` names W4 without either being introduced in the section.** Pre-existing, unchanged by this cut, and reduced rather than worsened by it (W3 and W5 mentions each went from two to one). A reader upgrading from 0.0.1 has no way to know what W3 is. Noted for whoever owns the section's next revision, not as a finding against this change.

5. **The section carries semicolons**, which the project's Simplified Technical English rule bans for technical artefacts. `:11`, `:25`, `:26` and `:32` all use them. This is consistent across the file including the untouched 0.0.1 section, so it is a whole-file convention question rather than something this cut introduced, and it is outside my truth lens entirely.
