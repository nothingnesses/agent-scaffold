# `ship-v0-0-2-inc1` round 6 (verification, beyond the cap): TRIAGE

Independent triager. I did not write this change, I did not review it, and I did not triage rounds 1 to 5. I adjudicate the seven round 6 findings. I do not fix, and I do not take the human's decision.

## Artifact

- Worktree `.claude/worktrees/tri-r6`, detached at `bbcd10b` ("docs: cut the release notes to what is true of the published v0.0.1").
- Adjudicated: `git diff main..HEAD`, and for the last pass `git diff HEAD~2..HEAD`.
- Specification: `docs/plans/agent-scaffold.steps/ship-v0-0-2.md`.
- Findings: `v002-r6-reviewer-truth.md` (`R1` to `R4`, all `low`) and `v002-r6-reviewer-loss.md` (`L1` to `L3`, all `low`).
- Settled and not reopened: the five earlier triages, the recorded residuals, the containment mechanism, `A2`, `A3`, `A5`, the audit's `F2` and `F3`, the `superseded by` projection defect, the rename, ANSI escapes in a `dest`, and the plan-side sidecar symlink hole.

### The reviewers reviewed a different commit, and it does not matter

Both reviewers worked at `ba466c2`. My HEAD is `bbcd10b`. I checked before adjudicating rather than assuming:

```
git diff ba466c2 HEAD --stat
  docs/metrics/workflow.jsonl                          2 +
  .../v002-r6-reviewer-loss.md                       393 +
  .../v002-r6-reviewer-truth.md                      338 +
```

The only difference is the two reviewer files and two metrics records. `CHANGELOG.md` and `src/` are byte-identical between the two commits, so every finding transfers unchanged and every line number they cite is still correct at my HEAD. I confirmed each cited line number independently.

## Method, and which binary produced which result

The governing rule for this round is that a claim about 0.0.1 is measured against the PUBLISHED 0.0.1, never against `main`. Two release binaries, one `CARGO_TARGET_DIR` each, from a `git archive | tar -x` extract of the tag and from the worktree, confirmed distinct by `md5sum`:

```
c68928f1ea5ecc144faf7309699dc061  targets/tag001/release/agent-scaffold   (v0.0.1)  --version: agent-scaffold 0.0.1
d548a6bec39b1472d47cc7e2656f761e  targets/head/release/agent-scaffold     (bbcd10b) --version: agent-scaffold 0.0.2
```

Below, TAG is the 0.0.1 binary and HEAD is the 0.0.2 binary. **Every 0.0.1 result in this document was produced by TAG.** I built no `main` binary and I inferred no 0.0.1 behaviour from source reading alone, except the literal-read grep, which I then confirmed by running TAG.

`v0.0.1` is an annotated tag: the tag object is `0f33878` and the commit is `2bbce2e`. `git rev-list --count v0.0.1..HEAD` is 978 and `v0.0.1..main` is 967. The brief's 966 was measured at an earlier `main` tip.

All fixtures, packs, symbolic-link targets, output directories and target directories sit under my own scratch subdirectory. I used symbolic links and invalid UTF-8 rather than permission bits, so I set no `chmod` anywhere and there is none to restore. `git status --porcelain` is empty in this worktree and in the main repository apart from this file.

### Gates I ran myself at HEAD, with the HEAD binary

| gate | result |
| --- | --- |
| `cargo test --release` | exit 0, all suites pass |
| `render --check --strict docs/plans/agent-scaffold.plan.toml` | `up to date`, exit 0 |
| `validate --source ... --metrics ...` | `342 records, valid`, `99 steps, 76 questions, valid`, exit 0 |
| `validate --source ... --metrics ... --workflow` | `workflow invariants hold`, exit 0 |
| ASCII check, both changed files | 0 non-ASCII lines each |

### My own falsification sweep of all fifteen surviving bullets

I did not take the truth reviewer's table on trust. I constructed and ran my own case against every bullet that makes a runnable claim. The results below are mine.

```
:11  README rename section + contact route          README.md:7 heading, :11 contact route      TRUE
:15  audit subcommand                               see V3                                     ONE FALSE CLAUSE
:16  type:"waiver" + W5                             TAG src/ = main,manifest,pack,tui only      TRUE as an addition
:17  type:"decision", type:"baseline", W4           same; no metrics.rs at TAG                  TRUE as an addition
:18  validate --workflow cross-reference            TAG `validate` exit 2                       TRUE as an addition
:19  trivial/grandfathered retired same cycle       src/plan.rs:559-560 assert both rejected    TRUE
:20  --module isolation drops no files              31 files vs 31, identical paths; only       TRUE
                                                    AGENTS.md + its reference copy differ
:21  --module checks scaffolds four things          adds exactly checks.toml, checks/,          TRUE
                                                    hooks/pre-commit (mode 755), checks-
                                                    reviewer.md; 0 literal {{modules}} left
:25  BREAKING subcommand CLI                        see V4                                     TRUE but for one quantifier
:26  hardened workflow guidance                     TAG hits / HEAD hits: "total-round cap"     TRUE
                                                    0/2, "consecutive clean" 0/2, "intake"
                                                    0/5; pack/workflow.toml `cap = 5`
:27  README diagram split                           TAG README:87 one node; HEAD :73-74 two     TRUE
:28  diversity guidance generalised                 both quoted strings match verbatim          TRUE
:32  pack path read containment                     see V2; all leak claims re-measured         TRUE on fact
:33  literal-name files contained                   see V1; both leak claims re-measured        TRUE on fact
:34  dest write containment                         TAG writes outside at exit 0; HEAD exit 2   TRUE
                                                    on --write AND --dry-run, nothing written
```

The two escape reproductions at `:32` and `:33` that carry the section's 0.0.1 claims, measured by me at TAG:

```
principles.toml -> outside the pack     TAG exit 0, the outside file WAS read (22 hits of the
                                        planted principle name in the rendered AGENTS.md)
                                        HEAD exit 2, names principles.toml
pack.toml       -> outside the pack     TAG exit 0, and it OBEYED the outside manifest: PLANTED.md
                                        was created, a dest declared only in the outside manifest
                                        HEAD exit 2, names pack.toml
[[asset]] dest = "../ESCAPED.md"        TAG exit 0, file written outside --output-dir
                                        HEAD exit 2 on --write and on --dry-run, nothing written
```

## Verdicts

SEVEN findings raised, FIVE distinct after dedup, **FOUR valid**. All `low`.

**I raise nothing above `low`, and I state that explicitly as instructed.** No finding causes a wrong result, an unsafe action, or a broken command. The one false statement of fact under-claims the release rather than over-claiming it, which is the direction that leaves no reader less careful than they should be. This matches the severity rounds 4 and 5 gave the same class (`P2`, `T2`).

| id | raised as | site | verdict | severity | class | remedy class |
| --- | --- | --- | --- | --- | --- | --- |
| `V1` | `R2` + `L1` MERGED | `CHANGELOG.md:33` | valid | low | USER-FACING | authoring (minimal) |
| `V2` | `R3` + `L2` MERGED | `CHANGELOG.md:32` | valid | low | TEXT | pure deletion |
| `V3` | `R1` | `CHANGELOG.md:15` | valid | low | USER-FACING | pure deletion (whole sentence) |
| `V4` | `R4` | `CHANGELOG.md:25` | valid, optional | low | TEXT | partial-sentence edit |
| -- | `L3` | `CHANGELOG.md:33` | **NOT VALID** | -- | -- | none |

### Dedup rulings

**`R2` and `L1` MERGE into `V1`.** Same site, same sentence, same defect (the literal-name enumeration reads as exhaustive and omits `instrument.md`), same affected population, same remedy. `L1` adds one observation `R2` does not: the surviving "`pack.toml`, `principles.toml`," is a two-item list joined by a comma with no conjunction, with a comma still standing before the verb, so it reads as a list an item fell out of. That is evidence for the same defect, not a second defect. One finding.

**`R3` and `L2` MERGE into `V2`.** Same site, same word, same cause, same remedy. Both reviewers independently measured that the fact itself is intact and only the reference is broken. One finding.

**`L3` does NOT merge with `V1`.** It shares the site (`:33`) but concerns a different sentence and a different mechanism. I rule on it separately, and I rule it not valid.

**`R1` and `R4` stand alone.** Neither was raised by the loss lens and neither overlaps another finding.

## `V1` (valid, low, USER-FACING): the literal-name enumeration is incomplete

`CHANGELOG.md:33` opens: "The files the tool reads by literal name, `pack.toml`, `principles.toml`, are contained too". Before the cut it read "The three files the tool reads by literal name, `pack.toml`, `principles.toml` and `instrument.md`".

CONFIRMED. There are three production reads by literal name at HEAD and two at the tag. My own grep, then confirmed by running both binaries:

```
HEAD  src/manifest.rs:547  self.read("pack.toml")
HEAD  src/main.rs:264      source.read_optional("principles.toml")
HEAD  src/main.rs:298      source.read_optional("instrument.md")

TAG   src/manifest.rs:197  self.read("pack.toml")
TAG   src/main.rs:181      source.read("principles.toml")
TAG   occurrences of "instrument" in the whole of src/:  0
```

And `instrument.md` is contained by the same rule, measured, with a message of the same shape as the other two:

```
instrument.md -> outside, HEAD scaffold --instrument
  exit 2  error: could not read the pack's `instrument.md`: `instrument.md` is not a contained
          pack path (it resolves outside the pack directory, through a symbolic link); ...
instrument.md plain inside the pack, HEAD scaffold --instrument
  exit 0  AGENTS.md = "P:|I:INSIDE-PACK-INSTRUMENT|"
instrument.md -> outside, TAG (no --instrument flag exists)
  exit 0  AGENTS.md = "P:|I:{{instrument}}|"   the file is never read at 0.0.1
```

So both halves of the reviewers' case hold. Removing `instrument.md` from the SECOND sentence was required, because "In 0.0.1 each was read through a symbolic link out of the pack" is false of it. Removing it from the FIRST sentence as well went one deletion too far, because the first sentence describes HEAD, where the claim is true.

SEVERITY `low`, ruled. The direction is under-warning, which does not get the benefit of the "nobody is left less careful than they should be" reasoning that held round 5's `T2` at `low`. It stays at `low` on three grounds I measured. The affected population is a 0.0.2 pack author who ships an `instrument.md` and passes `--instrument`, and that population did not exist at 0.0.1 (the flag exits 2 at TAG), so round 2's `B2` bar is not met. The failure they meet is loud, exits 2, names the file and states the rule. The increment's own `README.md` states the complete boundary, naming all five paths including `instrument.md`.

Principle 6 (Ground decisions in evidence) is engaged mildly: the project holds the measurement that `instrument.md` is contained, and the section denies it by omission.

REMEDY, and what must NOT be edited. This is the one finding whose remedy is AUTHORING rather than deletion. Two forms:

- Drop the exhaustive framing with the smallest authored insertion, for example "Files the tool reads by literal name, among them `pack.toml` and `principles.toml`, are contained too". This asserts no new fact, removes the definite plural, and takes the missing conjunction with it. **Recommended**, because it is the smaller edit and it cannot re-break the second sentence.
- Or restore `instrument.md` to the first sentence and rescope the second.

**The second sentence, "In 0.0.1 each was read through a symbolic link out of the pack", must NOT be left to govern `instrument.md`.** That is exactly the trap that created this finding: the pass removed the file from both sentences because it was false in one of them. If `instrument.md` is restored to the first sentence, "each" in the second sentence reaches it and the bullet acquires a measured-false claim about 0.0.1, which is a worse defect than the one being fixed. The recommended form avoids the trap entirely by never restoring the name.

The rest of `:33` must not be touched: the "They are now refused by the same rule" sentence and the unreadable-file sentence are both measured true and are the bullet's actual delta from 0.0.1.

## `V2` (valid, low, TEXT): "neither" has no antecedent

`CHANGELOG.md:32` ends: "Each caller labels the refusal with its own field, and neither reports as a failed read, since nothing was opened."

CONFIRMED. My per-bullet word diff shows the cut removed the `[[module]]`'s `guidance` half of the pair, and the word "guidance" no longer appears anywhere in `:32`. The bullet now names exactly one field and then says "neither". "Each caller" governs one item.

The underlying fact is intact, which I measured: HEAD labels each refusal by its own field (`asset source ...`, ``module `leaky` guidance file ...``, `` `principles.toml` ``, `` `pack.toml` ``), and none is reported as a failed read. So this is a defect of reference, not of fact.

SEVERITY `low`. It costs a reader a re-read and nothing else. Round 3's text findings and round 4's `P4` both govern.

REMEDY: **pure deletion**, and it loses nothing. Delete ", and neither reports as a failed read, since nothing was opened".

I checked whether that deletion loses a fact before recommending it, and it does not. The same bullet already says, earlier: "Either refusal happens before the file is opened, so a path that escapes by either rule is never read rather than merely never used." The clause being deleted is redundant with text that survives in the same bullet. This makes the pure-deletion form strictly better than the one-word substitution both reviewers offered ("neither" to "no refusal"), because a one-word substitution is a partial-sentence edit, which is the class my partition below measures as the defect-producing one.

What must NOT be edited: the preceding sentence "Each caller labels the refusal with its own field" is measured true across four call sites and must stay. The final sentence about the message structure (value, cause in parentheses, rule) is measured true verbatim and must stay.

## `V3` (valid, low, USER-FACING): the audit bullet defers a harvest that ships. RULED, and it is the only false statement of fact in the section

This is the finding both reviewers agreed is the priority. I was asked to confirm or refute it and to rule on whether it is the only false statement of fact. **CONFIRMED, and it is stronger than the truth reviewer stated.**

`CHANGELOG.md:15` ends:

> This first increment ships the schema, the projection, and the caveat with an empty report; the signal harvests (rustc dead-code, the suppression-marker and FFI source scan, and `cargo-machete` unused dependencies) are later increments.

Measured by me at HEAD, against HEAD's own tree:

```
agent-scaffold audit --json
  "generated_from": { "rustc_dead_code": false, "source_scan": true, "cargo_machete": false }
  6 records, every one of kind "declared-reason", each with a real file:line span
  exit 0, and the project's file set is byte-identical before and after (--json writes no file)
```

The bullet is wrong in THREE places in one sentence, not one:

1. **"the suppression-marker and FFI source scan ... are later increments" is false.** `source_scan` is `true` and the scan runs.
2. **The FFI half ships too**, which neither reviewer established. I checked rather than assuming the compound name was only half-implemented. `src/audit.rs` carries `MarkerKind::Ffi` (`:333`), `Exclusion::Ffi` (`:249`), and a scanner documented at `:397` as detecting "`#[no_mangle]` / `extern "C"` FFI markers". The module doc at `:14` names the signal as "the `#[allow/expect(dead_code)]` / FFI source scan", which is the CHANGELOG's own compound. The entire second harvest ships.
3. **"This first increment" is false.** `src/audit.rs:70` documents `from_source_scan` as "Build the Increment 2 report for `task`: the source scan ran". The code calls this increment 2 while the release notes call it the first.

"with an empty report" is false in general and true only on a project carrying no suppression marker. I measured both: a scratch project with a bare `fn main()` gives `"records": []`, and this project's own tree gives six records.

Everything else in the bullet is measured true: `--json` writes no file and prints the intermediate, the non-JSON form writes `docs/plans/<task>.code-value-report.md` (confirmed by running it in a scratch project), the report leads with the caveat, and `generated_from` widens the caveat rather than reading as a clean pass.

**RULING on whether it is the only false statement of fact: YES.** I swept all fifteen bullets myself with my own falsifying cases, and the table above records the result. Fourteen of fifteen are factually true of the published release. `V1` is an incomplete enumeration (an omission, nothing false asserted), `V2` is a broken reference (the fact is true), and `V4` is a universal quantifier with a literal exception whose impact is nil. `:15` carries the section's only measured-false assertion.

SEVERITY `low`, ruled, and I decline to raise it. The error under-claims the release. A reader who believes it simply does not run a feature that works, so nothing breaks and no result is wrong. The aggravating feature the truth reviewer named is real, that this claim is about THIS release's own contents and one command refutes it, but it aggravates embarrassment rather than harm.

REMEDY: **pure deletion of the whole final sentence**, which is the safe class.

Delete "This first increment ships the schema, the projection, and the caveat with an empty report; the signal harvests (rustc dead-code, the suppression-marker and FFI source scan, and `cargo-machete` unused dependencies) are later increments." entirely.

I recommend the whole-sentence deletion over the truth reviewer's surgical form (delete "with an empty report" and "the suppression-marker and FFI source scan, and"). The surgical form leaves "(rustc dead-code, and `cargo-machete` unused dependencies)", a comma-plus-conjunction seam requiring repair, and it leaves "This first increment", which I measured false on its own. It is a partial-sentence edit at three spans in one sentence, which is precisely the class that produced every defect this round.

Deleting the whole sentence loses nothing, which I verified before recommending it. The information that two harvests do not yet run is already carried twice over: by the preceding sentence in the same bullet ("a `generated_from` signal set records which signals ran so an absent signal widens the caveat rather than reading as a clean pass") and by the shipped artifact itself, whose Markdown report prints "Signals not run (their coverage is absent, widening the caveat above): rustc dead-code, cargo-machete."

What must NOT be edited: the rest of `:15` is measured true in full and must be left alone, in particular the `CodeValueReport` and `AuditRecord` description, the caveat sentence, the `--json` and `--out` sentence, and the read-mostly sentence.

## `V4` (valid but OPTIONAL, low, TEXT): one literal exception to "every option"

`CHANGELOG.md:25` says "every option 0.0.1 documented is still accepted there", where "there" is under `scaffold`.

CONFIRMED on the facts, measured by me:

```
TAG --help documents 12 options: the 10 functional ones plus -h/--help and -V/--version
HEAD  scaffold -V           exit 2, error: unexpected argument '-V' found
HEAD  scaffold --version    exit 2
HEAD  --version             exit 0, agent-scaffold 0.0.2
```

Under the reading a reader of release notes will most likely take, the options the notes' own `## [0.0.1]` section documents, the claim is true without exception: all nine are accepted with their documented values. Under the binary's `--help` reading, eleven of twelve are accepted and `-V`/`--version` is not.

I rule this **valid but optional**, and I recommend recording it as a residual rather than fixing it. My reasoning, which departs from the truth reviewer's:

- The impact is nil and I measured it. `--version` works where a user types it, at the top level. Every subcommand CLI places `--version` at the top level only, so a user who meets this meets the conventional behaviour rather than a break.
- The remedy is a **partial-sentence edit**, not a pure deletion. Dropping the universal requires "every option ... is" to become "the options ... are", which changes number agreement across the clause. The only pure-deletion form is to cut the whole clause, and that clause carries a genuinely useful reassurance to an upgrader, so cutting it costs more than the defect does.
- By my partition below, partial-sentence edits are the class that produced every defect the cut created. Spending one on a defect with nil impact is a bad trade.

The truth reviewer's ground for raising it, that this loop has now measured three counting quantifiers false in these same bullets, is a fair reason to RAISE it. It is not a reason to EDIT it, because the two are different questions and the risk runs the other way.

Principle 2 (Minimal by default) supports leaving it: the specification says the scope is closed and "every addition to it defeats it".

## `L3` (NOT VALID): the deletion of the "ABSENCE IS UNCHANGED" sentence was correct

`L3` says that deleting "ABSENCE IS UNCHANGED and stays silent: a pack shipping neither file still yields no principles and an empty instrumentation block, byte for byte what 0.0.1 produced" removed the only thing separating an unreadable `principles.toml` from an absent one.

The measurements are correct and I reproduced them:

```
pack shipping NO principles.toml          TAG  exit 0, stderr 0 bytes, "P:|I:{{instrument}}|"
                                          HEAD exit 0, stderr 0 bytes, "P:|I:|"
pack shipping an invalid-UTF-8 one        TAG  exit 0, stderr 0 bytes, "P:|I:{{instrument}}|"
                                          HEAD exit 2, "could not read the pack's principles.toml:
                                                        stream did not contain valid UTF-8"
```

So absence is genuinely unchanged in exit code and in silence, and the reviewer is right that "byte for byte" is false for a pack carrying the `{{instrument}}` placeholder.

I nonetheless rule the finding NOT VALID, on three grounds.

**The surviving sentence identifies its subject by cause, not by symptom.** It reads "A file the tool cannot read (invalid UTF-8, or one it lacks permission to read)". An absent file is not a file the tool cannot read, and the parenthesis names the two triggers explicitly. A pack author who ships no `principles.toml` is not described by the sentence at all. `L3` requires a reader who matches on the outcome clause while ignoring the subject clause in the same sentence.

**Keep a Changelog, which `CHANGELOG.md:5` names as the format, makes silence mean unchanged.** In a delta document, a behaviour with no entry did not change. The deleted sentence asserted a NON-change, which under the format's own convention did not belong in the section in any revision. Removing it made the section more conformant, not less, and the correct inference a reader draws from the section's silence about absence is exactly the truth I measured.

**Restoring it requires authoring a new claim.** The deleted sentence cannot be restored as it stood, because its "byte for byte" half is measured false. Any fix authors a new sentence into the highest-risk class, to state a fact the format already communicates by omission.

If the human wants absence stated for belt and braces, it is an optional addition, not a defect repair. I record it as such and not as a residual, because a residual implies an unfixed defect and there is none here.

## The deletion versus in-place-edit partition, measured by me

This is the fourth measurement of this mechanism and it is destined for `workflow-audit-followups`, so I measured the partition from the diff rather than accepting either reviewer's account. **The brief's framing is wrong in one place and I correct it below.**

### The partition, from a byte-identity comparison of every bullet

I extracted the 0.0.2 section pre-cut (`HEAD~1`) and post-cut (`HEAD`), then tested every surviving bullet for byte-identity against the pre-cut set:

```
0.0.2 section, bullets      pre-cut 21   post-cut 15
bullets deleted outright     7
bullets authored new         1
bullets rewritten in place   2
untouched survivors         12   (byte-identical to their pre-cut form)

  7 deleted + 12 untouched + 2 rewritten = 21 pre-cut.  12 + 2 + 1 = 15 post-cut.
```

This confirms the loss reviewer's correction to the brief: SEVEN bullets were deleted outright, not six.

### Findings by class

| class | sites | findings raised | findings valid | valid per site |
| --- | --- | --- | --- | --- |
| whole-bullet deletion | 7 | **0** | **0** | **0.00** |
| untouched survivor | 12 | 1 (`V3`) | 1 | 0.08, and NOT caused by the cut |
| partial-sentence edit | 2 | 3 (`V1`, `V2`, `L3`) | 2 | **1.00** |
| authored bullet | 1 | 1 (`V4`) | 1 (optional) | **1.00** |

I confirmed `V3`'s bullet is an untouched survivor rather than assuming it: the audit bullet is byte-identical pre-cut and post-cut. It moved from line 16 to line 15 because a bullet above it was deleted. The defect predates the cut, so the cut neither caused it nor is credited with it.

### The correction the brief needs

The brief states: "Every other finding is at the two bullets the cut edited IN PLACE." **That is false.** `V4` (`R4`) is at `CHANGELOG.md:25`, the one AUTHORED bullet. The loss reviewer's own summary makes the same slip ("All three of my findings are at text the pass REWROTE"), which is true of the loss lens alone but not of the round. The correct statement is that every finding the cut CREATED is at either a partial-sentence edit or the authored bullet, and none is at a whole-bullet deletion.

The correction matters for the followups record, because it means the round did not measure only two classes. It measured four, and it separates the safe class from two unsafe ones rather than from one.

### Does this support the loss reviewer's proposed refinement?

The proposal is: cutting part of a sentence is not a pure deletion, and it behaves like authoring.

**YES, and my measurement supports it more strongly than the reviewers argued, but the stated mechanism is wrong and I refine it.**

The support is direct. Partial-sentence editing and authoring produced valid findings at the same rate, 1.00 per site, while whole-bullet deletion produced 0.00 across seven sites. On this round's evidence the two are indistinguishable in risk.

The mechanism is not what the phrase "behaves like authoring" implies. I ran a per-bullet word diff on both rewritten bullets to see what was actually added:

```
:32   66 words deleted,  2 "added":  "All"  "project."
:33   55 words deleted,  3 "added":  "`principles.toml`,"  "pack."  "0.0.1;"
```

Every one of those five additions is a punctuation or capitalisation repair at a cut seam. "all three shapes" became "All three shapes" when a mid-sentence clause became a sentence start. "project;" became "project." when the semicolon-joined second half was removed. "`principles.toml` and" became "`principles.toml`," when the third list item went. **Not one new content word was authored into either bullet, and both still produced findings.**

So the risk does not come from writing new claims. It comes from the fact that a partial cut silently RE-SCOPES the text that survives. In all three raised findings at these two bullets, the surviving text was calibrated to material the cut removed:

- `V1`: a definite plural and a two-item apposition that still read as a complete enumeration after the count word and one list item were deleted.
- `V2`: an anaphor ("neither") whose antecedent pair lost a member.
- `L3`: an outcome description that was disambiguated by a following sentence which was deleted.

The precise refinement I would record is therefore sharper than the reviewer's: **cutting within a sentence or a bullet is unsafe not because it authors new text but because quantifiers, anaphors, definite articles and contrast structures in the SURVIVING text were calibrated against the deleted text and are silently falsified by its removal. Deleting a whole bullet cannot do this, because nothing inside it survives to be re-scoped.** The practical rule that follows is that a partial cut requires a re-read of the whole surviving unit for counting words, pronouns, definite plurals and "the same"/"both"/"neither" constructions, whereas a whole-unit deletion requires only a check for inbound references from elsewhere.

The truth reviewer performed exactly that inbound-reference check for the whole-unit deletions and found every remaining cross-reference resolves. I spot-checked three of them (`:19`'s "above" to `:16`, `:33`'s "the same rule" to `:32`, `:32`'s "below" to `:34`) and agree. That is why the seven deletions scored zero.

## Ownership and asset-status enumeration (context for the recommendation, not adjudicated)

I was asked whether any OTHER 0.0.1 asset crossed the ownership line that `docs/plans/TEMPLATE.md` crossed. Nobody had checked, and the one known instance was found by accident.

**ANSWER: NO. Exactly one of the tag's eleven assets changed asset status or ownership, and it is the one already known.**

I extracted `(dest, ownership)` from both manifests and joined them:

| tag `dest` | tag ownership | at HEAD | changed? |
| --- | --- | --- | --- |
| `AGENTS.md` | working | working | no |
| `docs/plans/TEMPLATE.md` | working | **NOT AN ASSET** | **YES** |
| `.agents/AGENTS.reference.md` | reference | reference | no |
| `.agents/prompts/orchestrator.md` | reference | reference | no |
| `.agents/prompts/planner.md` | reference | reference | no |
| `.agents/prompts/clarifying-questions.md` | reference | reference | no |
| `.agents/prompts/open-questions-gate.md` | reference | reference | no |
| `.agents/prompts/reviewer.md` | reference | reference | no |
| `.agents/prompts/triager.md` | reference | reference | no |
| `.agents/prompts/implementer.md` | reference | reference | no |
| `.agents/principles.toml` | reference | reference | no |

TAG declares 11 assets, HEAD declares 35. Ten of the eleven carry an identical ownership token at both versions.

I did not stop at the manifest, because a matching token does not prove matching behaviour. I ran the upgrade end to end: scaffold with TAG into an empty directory, append a marker to all eleven files, then run HEAD's `scaffold` over the same directory.

```
HEAD upgrade over a hand-edited 0.0.1 tree:
  skip (exists)  AGENTS.md                      marker PRESERVED
  refresh        .agents/... (9 reference)      marker DESTROYED
  render         docs/plans/TEMPLATE.md         marker DESTROYED
  "(29 changed, 1 left untouched)"

CONTROL, the TAG binary re-run over the same hand-edited tree:
  skip (exists)  AGENTS.md                      marker PRESERVED
  skip (exists)  docs/plans/TEMPLATE.md         marker PRESERVED
  refresh        .agents/... (9 reference)      marker DESTROYED
  "(9 changed, 2 left untouched)"
```

The control is what settles it. The nine `.agents/` files are destroyed by BOTH binaries, so they did not cross any line: refreshing tool-owned reference assets is 0.0.1's documented, unchanged behaviour, advertised in the `## [0.0.1]` section itself. `docs/plans/TEMPLATE.md` is the only path whose treatment differs between the two versions, from `skip (exists)` to `render`.

So the human's decided remedy covers the whole of the known exposure among 0.0.1 assets. Nothing else needs a refusal.

### One adjacent hazard I found, which is NOT another instance

While enumerating I checked the converse direction, which no one had asked about: HEAD's NEW `reference` assets landing on paths a 0.0.1 user could already occupy. Measured:

```
0.0.1 tree, user hand-creates files at paths that become HEAD assets, then HEAD scaffold:
  .agents/workflow.toml            refresh        DESTROYED
  .agents/LEDGER.template.md       refresh        DESTROYED
  .agents/user-prompts/kickoff.md  refresh        DESTROYED
  docs/plans/TEMPLATE.plan.toml    skip (exists)  PRESERVED
  docs/plans/TEMPLATE.motivations.md  skip (exists)  PRESERVED
```

I rule this NOT a second instance of the ownership crossing, and NOT something the decided remedy needs to cover. These paths were never tool-owned assets at 0.0.1, so no ownership changed. They sit under `.agents/`, which the `## [0.0.1]` section itself declares tool-owned ("refreshes tool-owned reference assets under `.agents/`"), so a user placing their own file there is acting against the documented contract. The `working` ones correctly skip. The population is speculative rather than measured, unlike `TEMPLATE.md`, whose destruction happens to every 0.0.1 user who edited the file the tool told them was theirs.

Recording it so the next person does not have to re-derive it. It is out of scope for this increment and belongs with the other `workflow-audit-followups` items.

I also observed that a pre-existing but malformed `docs/plans/TEMPLATE.plan.toml` makes HEAD's `scaffold` error out at the render step AFTER it has already refreshed nine files. Out of scope, contrived population, recorded for the same list.

## Round outcome

**NOT CLEAN.**

Four valid findings, all `low`, at three sites (`:15`, `:32`, `:33`) plus one optional at `:25`. Nothing above `low`, stated explicitly. One reviewer finding (`L3`) ruled not valid.

The specification declares the risk class `low_risk` on `ship-v0-0-2-inc1` (`Q-74`, human, 2026-08-13), under which one clean round converges the loop. This round is not clean, so it does not converge.

### Is the change safe to publish on my ruling alone?

Ignoring the separate `TEMPLATE.md` matter, as instructed: **yes.**

- No finding is above `low`, and none blocks a release on its own.
- Fourteen of fifteen surviving bullets are factually true of the published 0.0.1, measured by me against a binary I built from the tag. The one exception under-claims the release.
- Every gate I ran is green: `cargo test`, `render --check --strict`, both `validate` forms, and the ASCII check on both changed files.
- Every claim in the authored BREAKING bullet is true except one universal quantifier with nil impact.
- All three of the specification's defect fixes (`F1`, `F4`, `F4b`) are measured working, and the `dest` and `source` boundaries both refuse on `--write` and `--dry-run` without writing anything.

The section is in far better condition than when this loop began: round 5 measured 4 of 21 bullets fully true, and I measure 14 of 15 factually true now.

**But the change is NOT ready to publish**, and my ruling is not what stops it. The `TEMPLATE.md` destruction is a Principle 3 (Safe on existing projects) violation that publication would make real for every 0.0.1 user who edited that file. The human has already decided the remedy. That work must land first.

## Recommendation on how the increment ends

Framed for the human's decision, judged against the plan's Project Principles by name. I recommend, I do not decide.

**Recommended: ONE fix pass covering the decided `TEMPLATE.md` remedy and three of my four findings, then close the increment on a targeted verification rather than a seventh full review round.**

### Why one pass rather than two

The next pass must author regardless of my ruling. Principle 3 (Safe on existing projects) requires the refusal message and the BREAKING bullet the human decided on, and neither can be produced by deletion. **The deletion-only discipline that governed this pass is therefore already spent**, which removes the single reason rounds 5 and 6 gave for deferring `V1`: both reviewers recommended recording `V1` as a residual specifically because its fix "is not a pure deletion" and "cuts against the discipline this pass was given". That objection no longer applies. Once a pass is authoring, `V1`'s one-clause fix is close to free.

The pass should carry, in this order of confidence:

| item | remedy class | note |
| --- | --- | --- |
| the decided `TEMPLATE.md` refusal, instruction and BREAKING bullet | authoring | required by Principle 3, already decided |
| `V3` `:15` | pure deletion, whole sentence | the safe class, and it removes the section's only false statement of fact |
| `V2` `:32` | pure deletion, whole clause | the safe class, and the fact it carries survives elsewhere in the same bullet |
| `V1` `:33` | authoring, one clause | recommended form never restores `instrument.md`, so the second sentence cannot be re-broken |
| `V4` `:25` | none | record as a residual |

Two of the four are whole-unit deletions, the class this loop has now measured across seven sites in this round alone without a single finding. That is the deliberate design of the list: I converted `V3` and `V2` from the partial-sentence remedies the reviewers proposed into whole-unit deletions, after verifying in each case that nothing is lost.

### Why not a seventh full review round

Principle 6 (Ground decisions in evidence) cuts against it. The mechanism a seventh round would re-measure is now measured four times, and this round measured it with a word-level diff that isolates the cause. Another round buys a fifth data point on a settled question at the cost of two more reviewer passes and a triage.

Principle 2 (Minimal by default) and the specification cut against it harder. The step's own text says its "purpose is delivery, so its scope is closed and every addition to it defeats it", and the loop is already at round 6 against a cap of 5, running on the human's specific authorisation. The audit that motivated this step found eleven consecutive days producing zero completed steps. A seventh round is that pattern.

Instead, verify the fix pass with a targeted check by an independent agent, scoped to five things and nothing else:

1. The `TEMPLATE.md` refusal reproduction: TAG scaffold, hand-edit `docs/plans/TEMPLATE.md`, HEAD scaffold, confirm refusal rather than silent overwrite.
2. The four edited sites re-read in full for the failure mode this round isolated, namely quantifiers, anaphors, definite plurals and contrast structures calibrated against removed text.
3. The new BREAKING bullet's claims run against a binary built from the tag.
4. The seven release gates the specification's release criterion 4 names.
5. Nothing else. The rest of the section is measured.

### The alternative, and why I do not recommend it

Publish now with all four recorded as residuals, and fix `TEMPLATE.md` in a follow-up release. I reject this. Publishing 0.0.2 is what makes the destructive upgrade reachable, so a follow-up release repairs the damage after it has been done rather than preventing it. Principle 3 is a pre-condition of publishing, not a post-condition.

### The decision I am leaving to the human

Whether `V1` goes into this pass or is recorded as a residual. My recommendation is to include it, because the pass is authoring anyway and the direction of the error is under-warning about a security boundary. The case for excluding it is that the specification's scope is closed, the affected population did not exist at 0.0.1, and the increment's own `README.md` already states the boundary correctly. Both are defensible on Principle 2. I have set out the constraint that makes either choice safe: if `V1` is fixed, the second sentence of `:33` must not be allowed to govern `instrument.md`.
