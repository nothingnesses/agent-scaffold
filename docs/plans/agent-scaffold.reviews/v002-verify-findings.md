# `ship-v0-0-2-inc1`: targeted verification of the fix pass at `06eb186`

Independent verifier. I did not write this change, I did not review it, and I did not triage any round. My scope is the five items the round 6 triage named and nothing else.

## Binaries, and which one produced which result

Two release binaries, separate `CARGO_TARGET_DIR`s, confirmed distinct before use:

```
2a82c1f0cab6588d03ec18d91d1895a9  target-tag/release/agent-scaffold    --version: agent-scaffold 0.0.1
dac59d2da518c7fd99e534d1ee8d90c6  target-head/release/agent-scaffold   --version: agent-scaffold 0.0.2
```

- TAG is built from the worktree at `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/tag-v001`, a detached checkout of tag `v0.0.1` (commit `2bbce2e`).
- HEAD is built from `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/verify-v002`, branch `verify/v002` at `06eb186`.

Every result below names the binary that produced it. I ran no `main` build and I inferred no 0.0.1 behaviour from source reading. Where I needed a fact about the tag's manifest I read it out of the TAG BINARY itself with `strings`, not out of the tag's source tree.

A third binary, MUTANT, exists only for the non-vacuity check in item 1. It is `06eb186` with the new condition defeated, built from a `git archive` extract in my scratchpad. I did not modify either worktree. `git status --porcelain` is empty in both worktrees and in the main repository.

All fixtures, packs, symbolic-link targets and target directories are under `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/v002-verify/`. I used symbolic links and invalid UTF-8 rather than permission bits, so I set no `chmod` and there is none to restore.

## 1. Summary table

| item | subject | verdict |
| --- | --- | --- |
| 1 | the `TEMPLATE.md` refusal reproduction | clean |
| 2 | the four edited sites, re-read for the round 6 failure mode | finding (`X1`, `X2`) |
| 3 | the new BREAKING bullet's claims against the tag binary | clean |
| 4 | the seven release gates, `checks`, and the `F1` one-problem expectation | clean |

## 2. Findings table

| id | item | site | severity | summary | remedy class |
| --- | --- | --- | --- | --- | --- |
| `X1` | 2 | `README.md:270` | low | The measured-false sentence the pass deleted from `CHANGELOG.md:15` survives verbatim in `README.md`, which ships inside the published crate. | pure deletion (whole sentence) |
| `X2` | 2 | `CHANGELOG.md:34` | low | The bullet's last sentence has an indefinite subject whose domain the first sentence widened; the claim is measured false of `pack.toml` in both halves. Pre-existing, widened rather than created. | partial-sentence edit, or record as a residual |

Nothing above `low`. No finding causes a wrong result, an unsafe action, or a broken command. Both under-claim rather than over-claim, which is the direction that leaves no reader less careful than they should be.

## 3. Findings

### `X1` (low): the deleted false claim still ships, in `README.md`

**What I ran.** The round 6 triage's own rule for a whole-unit deletion is that it "requires only a check for inbound references from elsewhere". The pass's `V3` remedy is a whole-sentence deletion at `CHANGELOG.md:15`, so I ran that check across the release's own documents.

```
grep -nE "first increment|later increment|empty report|cargo-machete|rustc dead-code|signal harvest" \
  CHANGELOG.md README.md docs/plans/agent-scaffold.md
```

**What I observed.** `CHANGELOG.md` is clean: the sentence is gone and no other bullet refers to it. `README.md:270` ends:

> A human reads the report and decides each candidate; nothing is removed automatically. This first increment ships the schema, the projection, and the caveat with an empty report; the signal harvests are later increments.

I re-measured the three claims myself rather than taking the triage's word, with the HEAD binary, against HEAD's own tree:

```
HEAD  agent-scaffold audit --json
      "rustc_dead_code": false
      "source_scan":    true
      "cargo_machete":  false
      6 records, every one of kind "declared-reason", exit 0
```

- "with an empty report" is false on this project's own tree: six records.
- "the signal harvests are later increments" is false: `source_scan` is `true`, the scan runs, and it produces all six records.
- "This first increment" is false: `src/audit.rs:70` reads "Build the Increment 2 report for `task`: the source scan ran".

I then confirmed the sentence reaches users. `Cargo.toml:9` declares `readme = "README.md"`, and the `cargo publish --dry-run` package carries it:

```
grep -c "This first increment ships the schema" \
  target-head/package/agent-scaffold-0.0.2/README.md
1
```

**Why it is a defect.** The triage ruled this the section's only false statement of fact and prescribed its deletion. The pass deleted one of the two copies. The release therefore ships a document telling the reader that a feature of this release does not run yet, on the page crates.io renders as the crate's front matter, while the release notes now say nothing either way. `README.md` is also a file this branch changed, so it is inside the release's own diff.

**Smallest remedy.** The same pure deletion the triage designated the safe class: delete "This first increment ships the schema, the projection, and the caveat with an empty report; the signal harvests are later increments." from `README.md:270`. Nothing is lost. The preceding sentences in the same paragraph already state the report is advisory and read-mostly, and the shipped Markdown report itself prints its own "Signals not run" line. The paragraph ends cleanly at "nothing is removed automatically."

The sentence is pre-existing on `main` rather than authored by this branch, which is why no earlier round caught it: every round measured the CHANGELOG.

### `X2` (low): `CHANGELOG.md:34`'s last sentence quantifies over a domain the first sentence widened

**The site.** The pass rewrote the bullet's first two sentences. The bullet now reads:

> Files the tool reads by literal name, among them `pack.toml` and `principles.toml`, are contained too (`src/manifest.rs`, `src/main.rs`). In 0.0.1 each of those two was read through a symbolic link out of the pack. They are now refused by the same rule. A file the tool cannot read (invalid UTF-8, or one it lacks permission to read) produced an empty principle set at exit 0 with empty stderr in 0.0.1; it now exits 2 naming the file, which matches a malformed `principles.toml`, already loud in 0.0.1.

Sentence 1 replaced a closed two-item enumeration with an open set. Sentence 4's subject, "A file the tool cannot read", is indefinite and takes its domain from sentence 1. That domain is now every file the tool reads by literal name, which I measured to be three: `pack.toml`, `principles.toml` and `instrument.md`.

**What I ran.** A minimal `--template` pack, with each literal-name file made invalid UTF-8 in turn.

```
TAG   principles.toml invalid UTF-8   exit 0, stderr 0 bytes, rendered "P:|"      (an empty principle set)
TAG   pack.toml       invalid UTF-8   exit 2, "error: stream did not contain valid UTF-8", 0 files written
HEAD  principles.toml invalid UTF-8   exit 2, "error: could not read the pack's principles.toml: stream
                                              did not contain valid UTF-8"        (names the file)
HEAD  pack.toml       invalid UTF-8   exit 2, "error: stream did not contain valid UTF-8"
                                                                                  (does NOT name the file)
```

**Why it is a defect.** Read over its stated domain, sentence 4 is false of `pack.toml` in both halves. It did not produce an empty principle set at exit 0 with empty stderr in 0.0.1, it exited 2 and wrote nothing. And it does not now exit 2 "naming the file", the message names neither `pack.toml` nor any path. It is also false of `instrument.md`, which 0.0.1 never read at all.

**What limits it, stated plainly.** The pass did not create this. The pre-pass wording, "The files the tool reads by literal name, `pack.toml`, `principles.toml`", was a closed set that already contained `pack.toml`, so sentence 4 already ranged over it and was already false of it. The pass widened an already-wrong domain rather than breaking a correct one. The sentence also part-scopes itself: "an empty principle set" and "which matches a malformed `principles.toml`" both tie it to the principles file, so a reader who tries to apply it to `pack.toml` hits an outcome that does not fit the subject and re-scopes it themselves.

**Smallest remedy.** Change "A file the tool cannot read" to "A `principles.toml` the tool cannot read". That is a partial-sentence edit, the class this loop measured at 1.00 valid findings per site across two sites, spent on a defect that pre-dates the pass and under-warns rather than over-claims. I recommend recording `X2` as a residual beside `V4` rather than spending that edit before publishing, on Principle 2 (Minimal by default). The human decides.

## 4. Evidence per item

### Item 1: the `TEMPLATE.md` refusal reproduction. CLEAN

**Setup, TAG binary.** Scaffolded a fresh fixture, then appended a hand-written section to `docs/plans/TEMPLATE.md`.

```
TAG   agent-scaffold --output-dir item1/proj --vcs none --write
      exit 0, 11 files, including "create  docs/plans/TEMPLATE.md"
      after the hand edit: md5 a43a040c337f014beba204644dacde0c, 2699 bytes
```

**Control, TAG binary, the same edited tree re-scaffolded by 0.0.1.**

```
   skip (exists)  AGENTS.md
   skip (exists)  docs/plans/TEMPLATE.md
         refresh  .agents/AGENTS.reference.md
         ... 8 more refresh lines ...
Wrote to item1/proj-tagcontrol (9 changed, 2 left untouched).
exit 0, md5 after: a43a040c337f014beba204644dacde0c  (unchanged)
```

**The upgrade, HEAD binary, over the same edited tree.** Full run, exit 0. The last outcome line and the summary line:

```
   skip (exists)  AGENTS.md
          create  docs/plans/TEMPLATE.plan.toml
          ... 19 more create lines and 9 refresh lines ...
   keep (edited)  docs/plans/TEMPLATE.md
Wrote to item1/proj-headupgrade (29 changed, 1 left untouched).
```

stderr, in full:

```
docs/plans/TEMPLATE.md already exists and differs from what this version generates, so it was left untouched. It is now a generated view of docs/plans/TEMPLATE.plan.toml: move your copy aside and run `agent-scaffold render docs/plans/TEMPLATE.plan.toml` to produce the current one.
```

All three required properties hold, measured with the HEAD binary:

- The bytes are unchanged. `md5 a43a040c337f014beba204644dacde0c`, and a `diff` against the pre-run copy reports no difference.
- The run reports `keep (edited)`.
- The run prints the command that produces the current view.

**The printed command works.** I moved my copy aside and ran the exact string the run printed, from inside the output directory, with the HEAD binary on PATH:

```
agent-scaffold render docs/plans/TEMPLATE.plan.toml
rendered docs/plans/TEMPLATE.md
exit 0
md5 ed298f0465e8a321825d962b7aaba924, byte-identical to a fresh HEAD scaffold's view
my moved-aside copy still carries VERIFIER-MARKER-9f3a
```

**The ordinary case still works, HEAD binary.** A view this version generated is regenerated normally, not refused.

```
first  scaffold  "render  docs/plans/TEMPLATE.md", exit 0, stderr 0 bytes, md5 ed298f0465e8a321825d962b7aaba924
second scaffold  "render  docs/plans/TEMPLATE.md", exit 0, stderr 0 bytes, md5 ed298f0465e8a321825d962b7aaba924
                 "Wrote to item1/ordinary (17 changed, 13 left untouched)."
                 no "keep (edited)" line, no refusal on stderr
```

**The refusal is non-vacuous.** I built MUTANT, `06eb186` with the condition changed to `existing.is_some_and(|committed| committed != rendered) && false`, in a scratchpad copy of the tree, and ran the two tests:

```
test an_edited_plan_view_is_kept_and_the_run_says_what_to_do ... FAILED
test an_unedited_plan_view_is_still_regenerated ... ok
test result: FAILED. 1 passed; 1 failed
```

The failure is the right one. The assertion that fires is "a pre-existing view whose bytes differ from a fresh render must be left untouched", and the diff in the panic shows the hand-written marker gone from the left side. This matches the commit message's claim exactly: the first test fails when the check is defeated, and the non-vacuity test still passes. Both tests pass at `06eb186` unmutated, inside gate 1.

**Two things I checked beyond the brief, both clean.**

- `--force`, HEAD binary, over the same edited tree: still `keep (edited)`, md5 unchanged, marker intact, exit 0. The TAG binary under `--force` prints `overwrite  docs/plans/TEMPLATE.md` and destroys the marker, so the new refusal is strictly safer than 0.0.1 was, and the bullet's unqualified "does NOT overwrite it" holds even under `--force`.
- `--dry-run`, HEAD binary: the preview names no view line at all, on a fresh directory or on the edited tree, and writes nothing. That is symmetric between the two cases, so the preview never promises a write the action would refuse. The render step sits inside the write path in both the pre-fix and post-fix code, so this is unchanged behaviour rather than a parity gap the fix introduced.

**On the summary line, which I checked rather than assumed.** The upgrade run prints 31 outcome lines (20 create, 9 refresh, 1 skip, 1 keep (edited)) and a summary of "29 changed, 1 left untouched". Two files were in fact left untouched, not one. This is not a defect and I do not file it: the counts are over manifest assets only, and the view line is excluded in both directions. A fresh HEAD run prints 30 create plus 1 render and reports "30 changed", so `render` is not counted either. The view not being an asset is the premise of the whole change.

### Item 2: the four edited sites. See `X1` and `X2` above; the rest is clean

The four sites `git show 06eb186 -- CHANGELOG.md` touches are `:15` (whole-sentence deletion), `:25` (authored), `:33` (clause deletion) and `:34` (one-clause edit). I read each bullet whole, not as diff lines, and resolved every quantifier, anaphor, definite plural and contrast structure in the surviving text.

**`:15`, the audit bullet, whole-sentence deletion. Clean inside the bullet.**

The deleted sentence was terminal, so nothing before it changed reference. The words that could have been calibrated against it:

- "carrying only each kind's own evidence": "each kind" binds to the three variants named in the same parenthesis, `DeadCode`, `UnusedDep`, `DeclaredReason`. Resolves.
- "only relative to the named signal set": this is quoted from inside the report's own caveat, and the next clause gives it a referent in the bullet as well ("a `generated_from` signal set records which signals ran"). Unchanged by the deletion, which came after it.
- "an absent signal widens the caveat rather than reading as a clean pass": self-contained contrast, both halves in the sentence.
- "a human decides each candidate": binds to "a candidate verdict is derived at projection time" earlier. Resolves.

The inbound-reference check is what produced `X1`.

**`:25`, the authored BREAKING bullet. Clean.**

Every "it" in the bullet binds to `docs/plans/TEMPLATE.md` and the chain is consistent. The insertion re-scopes nothing nearby: `grep -nE "BREAKING|breaking change" CHANGELOG.md README.md` returns exactly the two BREAKING bullets and no count word, no "the breaking change" singular, and no enumeration that a second BREAKING entry would falsify. `README.md:30` already describes `docs/plans/TEMPLATE.md` as "generated plan view (rendered from the skeleton; do not hand-edit)" and `README.md:26` already calls `TEMPLATE.plan.toml` the working file, so the README agrees with the new bullet rather than contradicting it. The bullet's own factual claims are item 3.

**`:33`, the pack-path bullet, clause deletion. Clean.**

The deleted clause was ", and neither reports as a failed read, since nothing was opened". What survives around it:

- "Each caller labels the refusal with its own field." "Caller" is introduced earlier in the same bullet ("so a later caller inherits it instead of having to remember it"). Resolves. I also measured the sentence true across all four call sites at HEAD, so the deletion did not leave a sentence standing on a fact it cannot carry:

```
HEAD  asset source escape     "error: asset source `leaky.md` is not a contained pack path ..."
HEAD  pack.toml escape        "error: `pack.toml` is not a contained pack path ..."
HEAD  principles.toml unread  "error: could not read the pack's principles.toml: ..."
HEAD  instrument.md escape    "error: could not read the pack's `instrument.md`: ..."
```

- The fact the clause carried does survive earlier in the same bullet, so the deletion loses nothing: "Either refusal happens before the file is opened, so a path that escapes by either rule is never read rather than merely never used." I confirmed that sentence is present verbatim.
- Counting words elsewhere in the bullet all resolve: "All three shapes" to the `..` component, the absolute path and the symbolic link named in the previous sentence; "two rules in order" and "either rule" to the two rules stated in the same sentence; "beside those two that survive" to the pack-internal link and the `--template` link named immediately before; "Each refusal message" to the same caller set.
- The contrast structures all keep both halves: "rather than a theoretical one", "instead of having to remember it", "rather than merely never used", "rather than about whether a link was involved", "rather than linking it", "beats silently reading".

**`:34`, the literal-name bullet, one-clause edit. The triage's constraint is met. `X2` is at sentence 4.**

The specific constraint I was told to check directly is that the fix must not let the second sentence govern `instrument.md`. It does not:

> Files the tool reads by literal name, among them `pack.toml` and `principles.toml`, are contained too. In 0.0.1 each of those two was read through a symbolic link out of the pack.

"each of those two" is a closed anaphor to the two files named in sentence 1. `instrument.md` is not named anywhere in the bullet, so the 0.0.1 sentence cannot reach it. The trap the triage identified is avoided.

The generalisation in sentence 1 is true and non-vacuous, which I checked rather than assumed, because widening an enumeration to a universal is falsified by any uncontained literal-name read. There are exactly three production reads by literal name at HEAD, all through the contained site:

```
src/manifest.rs:547  self.read("pack.toml")
src/main.rs:264      source.read_optional("principles.toml")
src/main.rs:298      source.read_optional("instrument.md")
```

Every other literal-name occurrence in `src/` is inside `#[cfg(test)]` fixtures. And `instrument.md`, the file the "among them" fix exists to stop excluding, is measurably contained at HEAD:

```
HEAD  scaffold --instrument, pack's instrument.md symlinked outside the pack
      exit 2  error: could not read the pack's `instrument.md`: `instrument.md` is not a contained
              pack path (it resolves outside the pack directory, through a symbolic link); a pack
              path must be relative, carry no `..` component, and resolve to a location inside the
              pack directory
```

Sentence 3, "They are now refused by the same rule", is true under either antecedent, "those two" or the open set of sentence 1, since all three literal-name files are refused. Sentence 4 is `X2`.

### Item 3: the BREAKING bullet's claims. CLEAN

Each claim, with the command that tested it and the binary that produced the result.

| claim | binary | command | observed |
| --- | --- | --- | --- |
| "In 0.0.1 it was a manifest asset with `ownership = \"working\"`" | TAG | `strings target-tag/release/agent-scaffold` | The binary's own embedded manifest carries `[[asset]] source = "plan-template.md" / dest = "docs/plans/TEMPLATE.md" / ownership = "working"`. |
| the same, behaviourally | TAG | `--output-dir <edited tree> --vcs none --write` | `skip (exists)  docs/plans/TEMPLATE.md`, md5 unchanged. Create-if-absent is the `working` signature; the nine `reference` assets print `refresh` in the same run. |
| the same, under `--force` | TAG | `... --write --force` | `overwrite  docs/plans/TEMPLATE.md`, marker destroyed, which is the documented `working` behaviour ("Existing working files WILL be overwritten (--force)"). |
| "a re-scaffold printed `skip (exists)` and left your edits alone" | TAG | as above | Exact string `   skip (exists)  docs/plans/TEMPLATE.md`, md5 `a43a040c...` before and after. TRUE verbatim. |
| "`docs/plans/TEMPLATE.md` is no longer a file you own" | HEAD | `strings target-head/release/agent-scaffold \| grep 'dest = "docs/plans/TEMPLATE'` | Twelve `TEMPLATE.*` dests, none of them `docs/plans/TEMPLATE.md`. The binary's manifest even carries the comment "# (the generated view is not a manifest asset; it is regenerated by `render`)." TRUE. |
| "It is now a view generated from `docs/plans/TEMPLATE.plan.toml`" | HEAD | `scaffold --write` on a fresh directory | `render  docs/plans/TEMPLATE.md` after `create  docs/plans/TEMPLATE.plan.toml`. TRUE. |
| "which is the working file you edit instead" | HEAD | embedded manifest, then a second `scaffold --write` | `dest = "docs/plans/TEMPLATE.plan.toml"` with `ownership = "working"`, and the re-run prints `skip (exists)  docs/plans/TEMPLATE.plan.toml`. TRUE. |
| "Scaffolding over a project set up by 0.0.1 does NOT overwrite it" | HEAD | `scaffold --output-dir <0.0.1 tree, edited> --write` | md5 `a43a040c...` unchanged, byte-identical by `diff`. TRUE, and it also holds under `--force`. |
| "reported as `keep (edited)` and left exactly as it is" | HEAD | as above | `   keep (edited)  docs/plans/TEMPLATE.md`. TRUE verbatim. |
| "the run prints the command that produces the current view once you have moved your copy aside" | HEAD | as above, then the printed command | stderr prints ``run `agent-scaffold render docs/plans/TEMPLATE.plan.toml` ``; running it after moving the copy aside exits 0 and produces a view byte-identical to a fresh render. TRUE. |
| "A view this version generated is regenerated." | HEAD | two consecutive `scaffold --write` runs | Second run prints `render  docs/plans/TEMPLATE.md`, no refusal, view byte-identical. TRUE. |

Every 0.0.1 claim in the bullet is measured against the TAG binary and holds. Every 0.0.2 claim is measured against the `06eb186` binary and holds. I found nothing in this bullet to correct.

### Item 4: the seven release gates, `checks`, and the `F1` expectation. CLEAN

All run on `06eb186` in the verify worktree, with the `06eb186` binary where a binary is used.

| gate | command | exit | output |
| --- | --- | --- | --- |
| 1 | `cargo test` | 0 | 12 suites, 470 tests, `test result: ok` on every one, 0 `FAILED`. Includes `tests/scaffold_keeps_an_edited_generated_view.rs`, 2 passed. |
| 2 | `cargo clippy --all-targets -- -D warnings` | 0 | `Finished dev profile`, no warnings emitted. |
| 3 | `validate --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/workflow.jsonl` | 0 | `342 records, valid` / `99 steps, 76 questions, valid`. |
| 4 | `validate --source docs/plans/agent-scaffold.plan.toml --workflow` | 0 | as gate 3, plus `workflow invariants hold`. |
| 5 | `render docs/plans/agent-scaffold.plan.toml --check --strict` | 0 | `docs/plans/agent-scaffold.plan.toml: up to date`. |
| 6 | `LC_ALL=C grep -cP '[^\t\x20-\x7e]' <file>` on every changed file | n/a | 0 on all 14. |
| 7 | `cargo publish --dry-run` | 0 | `Packaged 412 files, 6.6MiB (2.0MiB compressed)`, verification build finished, `warning: aborting upload due to dry run`. |

Gate 6 in full. The changed set is the union of `git diff --name-only main...HEAD` (13 files) and `main..HEAD` (14, adding the ledger). I ran the class exactly as specified, with the hard tab excluded, and did not chain the `grep -c` with `&&`:

```
0  Cargo.lock
0  Cargo.toml
0  CHANGELOG.md
0  docs/plans/agent-scaffold.ledger.md
0  docs/plans/agent-scaffold.md
0  README.md
0  src/main.rs
0  src/manifest.rs
0  src/plan/render.rs
0  src/plan/source.rs
0  src/safe_path.rs
0  tests/pack_dest_stays_inside_the_output_dir.rs
0  tests/pack_source_stays_inside_the_pack.rs
0  tests/scaffold_keeps_an_edited_generated_view.rs
```

**`agent-scaffold checks`**, run with the `06eb186` binary on PATH inside the verify worktree:

```
        pass  render-check (lint)
checks: 1 passed, 0 failed, 0 skipped
exit 0
```

**The `F1` acceptance criterion 4 expectation**, exactly one problem:

```
agent-scaffold validate --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl
docs/plans/agent-scaffold.md: Open Questions item `Q-43` has an unknown status `superseded by `Q-44``
exit 1
```

One problem, and it is the pre-existing `Q-43` unknown-status one the specification names as known and out of scope. I do not report it as a finding. Exit 1 is the expected code while that problem stands.

## 5. OUT OF SCOPE, RAISED NOT FILED

Empty. I found nothing outside my five items that I judge serious enough that shipping without it would be wrong.

Two observations I met while measuring, recorded here only so nobody re-derives them, neither of which meets that bar and neither of which is a finding:

- `scaffold --dry-run` never previews the generated view, on a fresh directory or on an existing one. It is symmetric and pre-existing, the render step sits inside the write path in both the pre-fix and post-fix code, and it makes no promise the action then refuses.
- The run summary counts manifest assets only, so neither `render` nor `keep (edited)` is counted in "changed" or "left untouched". Consistent in both directions and unchanged by this pass.

I did not review code quality, naming, test design, documentation elsewhere, or any bullet the pass did not touch.

## 6. Verdict

**`ship-v0-0-2-inc1` is ready for the human to publish, with one one-sentence deletion I recommend making first.**

What the verification establishes:

- The condition that blocked publication is fixed and the fix is real. A 0.0.1 user's hand-edited `docs/plans/TEMPLATE.md` survives the upgrade byte for byte, the run says so and says what to do, the ordinary re-scaffold is unaffected, `--force` does not defeat it, and the check is non-vacuous under mutation. Principle 3 (Safe on existing projects) is satisfied, measured with a binary built from the tag rather than from a branch.
- Every claim in the new BREAKING bullet is true, each 0.0.1 claim measured against the TAG binary and each 0.0.2 claim against the `06eb186` binary.
- All seven release gates are green, `checks` passes, and the `F1` one-problem expectation holds exactly.
- The round 6 failure mode did not recur in the CHANGELOG. Three of the four edited sites are clean on it, and the specific trap the triage warned about, letting the 0.0.1 sentence at `:34` come to govern `instrument.md`, is avoided by "each of those two".

What I would fix first, and it is not a safety matter:

- `X1`. The false sentence the pass deleted from `CHANGELOG.md:15` still stands in `README.md:270`, and `README.md` ships inside the crate. The release would go out asserting that its own `audit` source scan is a later increment while the shipped binary runs it and reports six records. The remedy is a whole-sentence deletion of exactly the class the triage designated safe, it loses no information, and it needs no new authoring. Publishing without it is not unsafe, it is inaccurate on the crate's front page.
- `X2` is a residual in my view, not a pre-publication fix. It pre-dates the pass, it under-warns, the sentence part-scopes itself, and the remedy is a partial-sentence edit, the one class this loop has measured as defect-producing. Record it beside `V4`.
