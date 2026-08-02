# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 4, REVIEWER: deletion verification and whole-document consistency

Reviewer: independent of the planner, of every prior reviewer, and of every prior triager on this fold. Read-only with respect to the reviewed artifact; this file is the only thing written.

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-ep4-verify`, branch `review/q55-ep4-verify`, at `dd54227` (the round 3 fix pass: the pure-deletion PROVENANCE fix). Binary built at this commit with `cargo build`. All probes ran directly against the tracked worktree; the one scratch directory created for this review, `/tmp/claude-1000/rev-ep4-verify-scratch`, was never populated (no fixture was needed for this lens) and was removed immediately after creation, confirmed empty first.

Repository guards re-run at the reviewed commit, both green:

```
$ cargo run -q -- render docs/plans/agent-scaffold.plan.toml --check --strict
docs/plans/agent-scaffold.plan.toml: up to date
exit: 0

$ cargo run -q -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 256 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0

$ cargo test
(all suites) test result: ok, 0 failed, summed across 8 binaries (373 + 5 + 1 + 1 + 9 + 3 + 1 + 2 passed)

$ cargo clippy --all-targets -- -D warnings
(clean, no warnings)
```

The log has grown to 256 records since the round 3 triage measured 255, which is the orchestrator appending exactly as the round 3 triage predicted it would keep doing; this round's own round record is presumably the source of the 256th.

## Verdict summary

ZERO FINDINGS. The deletion landed exactly as round 3's triage prescribed, authored no words, broke nothing, and the whole-document sweep found no new count, enumeration, or cross-section defect. This is round 4 of a `risky` artifact requiring two consecutive clean rounds to converge (streak 0 going in); if round 3's dismissal-only status did not already start the streak, this round is the first clean one and one more clean round converges it.

## Job 1: the deletion, verified against the round 3 triage's exact prescription

`git diff HEAD~1 HEAD -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` shows exactly the two-site deletion the round 3 triage (`endproperty-fold-r3-triage.md:113-127`) prescribed and nothing else: the nine-bullet enumeration at the old `:11-20` is gone, the parenthetical `(the last taken on 2026-08-02, after inc1's work review)` is gone, and the sentence-terminating colon became a full stop. Confirmed with `grep -c` for every deleted phrase over the full tracked population (`docs/plans/agent-scaffold.steps/`, `docs/plans/agent-scaffold.plan.toml`, `docs/plans/agent-scaffold.md`): `grep -n "the last taken on 2026-08-02"`, `grep -n "^- \`q_id:\"Q-55"`, `grep -n "NINE decision receipts"` all return zero hits in the reviewed tree.

The surviving sentence, in full, at `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:10`:

```
PROVENANCE. `Q-55`, decided by the human on 2026-07-31, with decision receipts in `docs/metrics/workflow.jsonl`, all carrying `task:"workflow-enforcement-tier"`.
```

Grammatical: subject, appositive, two prepositional-phrase clauses, full stop. No dangling comma, no orphaned clause.

Word count, checked by diffing the sentence token by token rather than trusting the commit message: every word in the surviving sentence is a substring of the pre-deletion sentence in the same order (`PROVENANCE.` / `` `Q-55`, `` / `decided by the human on 2026-07-31,` / `with decision receipts in` / `` `docs/metrics/workflow.jsonl`, `` / `all carrying` / `` `task:"workflow-enforcement-tier"`. ``). The only edit inside the kept span is the terminal punctuation, `:` to `.`, which is not a word. AUTHORED WORD COUNT: 0, confirmed independently rather than taken from the commit message.

Regeneration, not hand-edit: `git diff HEAD~1 HEAD -- docs/plans/agent-scaffold.md`, with its hunk header's line-number offset stripped, is byte-identical to the sidecar's own diff body (`diff <(git diff HEAD~1 HEAD -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md | tail -n +6) <(git diff HEAD~1 HEAD -- docs/plans/agent-scaffold.md | tail -n +6)` returns no output). Independently, `cargo run -- render docs/plans/agent-scaffold.plan.toml --check --strict` exits 0 with `up to date` at the reviewed commit, so the projection is not merely diff-compatible with a hand-edit, it is what the renderer itself produces from the sidecar today.

Both previously-fixed stale-count passages elsewhere in the same file remain fixed and were not reverted by this pass: `:14` still reads "further human decisions" (not "two further human decisions"), and `:182` still reads "IT IS REACHED BY" (not "TWO CASES REACH IT:"). Grepped for both original phrasings plus three other round 2 deletions (`"and both exit 0"`, `"and only check 13b and 14g's fourth run catch"`, `"and neither is the selection of the plan to root on"`); all five return zero hits.

DELETION VERDICT: landed exactly as prescribed, zero words authored, yes.

## Job 2: the ten `Q-55*` receipts, each checked for a surviving citation

```
$ grep -o '"q_id":"Q-55[^"]*"' docs/metrics/workflow.jsonl | sort -u
"q_id":"Q-55"
"q_id":"Q-55-conventionlesscost"
"q_id":"Q-55-endproperty"
"q_id":"Q-55-jsonreason"
"q_id":"Q-55-mechanism"
"q_id":"Q-55-noconvention"
"q_id":"Q-55-refusalscope"
"q_id":"Q-55-resumecost"
"q_id":"Q-55-resumepairing"
"q_id":"Q-55-scope"
(10 distinct, unchanged from round 3's count)
```

For each, `grep -n "\`<id>\`" docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, counting only the exact backtick-delimited id so `Q-55` itself is not falsely matched inside `Q-55-anything`:

| receipt | cited at (line numbers) | count |
| --- | --- | --- |
| `Q-55` | 1, 5, 6, 10, 26, 44, 58, 263, 275, 381 | 10 |
| `Q-55-scope` | 275, 283 | 2 |
| `Q-55-mechanism` | 12, 145, 169, 269, 378 | 5 |
| `Q-55-noconvention` | 12, 153, 253, 259, 320 | 5 |
| `Q-55-refusalscope` | 161, 167, 195, 217, 257, 280, 287, 289, 303, 328, 352, 380 | 12 |
| `Q-55-jsonreason` | 191, 280, 303, 331, 361, 382, 383 | 7 |
| `Q-55-endproperty` | 280, 303, 309, 326, 333 | 5 |
| `Q-55-conventionlesscost` | 253, 259 | 2 |
| `Q-55-resumepairing` | 182, 329 | 2 |
| `Q-55-resumecost` | 253 | 1 |

All ten are cited at least once in the document body. `Q-55-resumecost`'s single citation is at `:253`, inside "The four accepted costs" preamble, exactly where the round 3 triage's ground 4 said it was ("`Q-55-resumecost` at `:263`" pre-deletion; the deletion's 10-line removal shifts every later line up by exactly 10, so `:263` becomes `:253`, confirmed by the shift being uniform across all nine other ids' line numbers too when compared against the round 3 triage's citations). The triager's claim that "every one of the ten receipts is separately cited in the body at the section it governs" holds under my own re-grep, not merely under the triager's.

## Job 3: whole-document consistency sweep

Read `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` in full (389 lines, both pages). Checked every explicit count and enumeration by recounting the items named, checked every affirmative exhaustiveness claim I could find, checked the accepted-cost cross-references, and checked several checkable factual claims against the actual codebase and the ledger.

COUNTS AND ENUMERATIONS RE-COUNTED, each against the document's own claim:

| claim | document says | I counted | match |
| --- | --- | --- | --- |
| exploration record lines | `1514` total (`521` + `483` + `510`) | `wc -l` on the three files: 521, 510, 483, total 1514 | yes |
| defects | "Four defects, one family" | A, B, C, D | yes, 4 |
| scope additions to the first pass | "Two human scope additions (defects C and D)" | C, D | yes, 2 |
| factual claims superseded from the first pass | "three of its factual claims are superseded" | the three `CORRECTION TO THE FIRST PASS...` sites at `:110` (status/next/`default_ledger_path` separability), `:129` (`default_ledger_path`'s assumed layout), `:247` (candidate (d)'s cost list); the fourth `CORRECTION` at `:44` corrects `Q-55`'s own wording, not the first pass, so it is correctly excluded from this count | yes, 3, and `:110`'s "the most consequential of the three" is self-consistent with this count |
| explorer A's two lesser measured costs on `value_source` | "Two lesser measured costs" | the `+96/-13` vs `+79/-15` diff-size cost, and the `[default:]` display cost | yes, 2 |
| adjustments to the refusal message shape | "TWO ADJUSTMENTS" | the first-slot naming clarification, and the remedy's third member | yes, 2 |
| things the implementer should carry (root-on-checked-plan section) | "Three things" | the one-predicate framing, the TOML-primary-mode reduction, and the typo'd-`--source` coverage | yes, 3 |
| alternatives rejected for `Q-55-endproperty` | "BOTH ALTERNATIVES" | the second-condition alternative, and the parse-the-triple alternative | yes, 2, and matches the `Q-55-endproperty` receipt's own three listed options (one chosen, two rejected) |
| doc comments falsified or made incomplete by `Q-55-jsonreason` | "Four doc claims" | the four bulleted at `:199-202`; the fifth item at `:204` is explicitly framed as pre-existing and not one of the four | yes, 4, and the "fifth item... not a consequence" framing is not double-counted |
| accepted costs | "The four accepted costs" | (i), (ii), (iii), (iv) | yes, 4 |
| red cases inc2 owes | "for inc2 there are FOUR" | check 11, check 13b, check 14b, check 14e | yes, 4 |

No stale count found among these, and none of the previously-fixed stale counts (five found across rounds 1-3 per the loop's own history: the "two further human decisions" numeral, the "TWO CASES REACH IT" numeral, the "and both exit 0" claim, the redundant per-surface enumeration pointer, and the PROVENANCE registry this round verifies) has resurfaced.

AFFIRMATIVE EXHAUSTIVENESS CLAIMS CHECKED AGAINST THE CODEBASE:

- `:206`, "`#[serde(skip)]` appears exactly ONCE in the whole of `src/`": `grep -rn 'serde(skip)' src/` returns exactly one hit, `src/next.rs:116`. Confirmed.
- `:206`, "No `skip_serializing_if` appears in either `src/next.rs` or `src/main.rs`": `grep -n 'skip_serializing_if' src/next.rs src/main.rs` returns zero hits in both files. Confirmed.
- `:245`/`:301`, `is_safe_sidecar_ref` at `src/plan/source.rs:480-495`: the function's doc comment starts at line 480 and its closing brace is at line 495 in the reviewed tree (confirmed by direct read). Confirmed.
- `:305`, the step 92 calibration citation ("six rounds, fifteen findings, all prose, zero mechanism defects, joint-third... against a project median of two"): re-derived from `docs/metrics/workflow.jsonl` directly rather than trusting the ledger's own restatement. `grep '"task":"prompt-drift-guard-inc1"' docs/metrics/workflow.jsonl` returns exactly 6 `"type":"round"` records, with `valid_findings` `4, 3, 5, 1, 2, 0`, summing to 15. Matches "six rounds and fifteen findings" exactly, and step 92 is closed (waived), so this citation cannot go stale on a future append the way a live count could.

CROSS-SECTION CONTRADICTIONS CHECKED, none found:

- Which plan each surface reads: `:163` ("`run_validate`'s `--workflow` match reads the TOML source... and the Markdown `--plan`... `run_status` and `run_next` each project from `toml_source(&args.source)`... `status --resume` is the one surface that reads NO plan") is consistent with every later reference to per-surface plan selection at `:179`, `:217`, `:229`, `:280`; no passage anywhere in the file states or implies the anchor is used as the checked plan outside TOML-primary mode.
- Which root the containment predicate uses: every passage after `Q-55-endproperty` (`:159` onward) states the root comes from the plan the check reads, never from the anchor; I found no residual passage still describing an anchor-rooted predicate as current behaviour.
- What each accepted cost covers: cost (iii) (`:259`) requires a MARKDOWN-primary `--source`, and the mechanism section explains why (in TOML-primary mode the checked plan is the anchor, so the divergence that produces the refusal cannot arise); cost (iv) (`:261`) covers `status --resume` "in EITHER primary spelling", consistent with `status --resume` reading no plan at all and so having no primary-mode dependency. Check 19b (`:340`) pins exactly this pairing. No contradiction between the costs section, the `Q-55-resumepairing` section, and check 19b.
- The "four surfaces" / "all three" wording at `:169`: `Q-55-mechanism`'s quoted text names four nouns (`validate`, `next`, `status`, "the ledger path"), and the decided receipt's own option text (verified verbatim against `docs/metrics/workflow.jsonl`: `"Wide: refusal on all three"`) counts from a different baseline, the three surfaces beyond the validator (which already refuses under either reading). Not a contradiction: "four" counts the full set including the validator, "three" counts the delta the decision was actually about, and the document's own quotation of the receipt is verbatim.

Nothing found. I looked for a sixth stale count and did not find one; I looked for a dangling reference to the deleted enumeration and did not find one; I looked for a cross-section contradiction on the three axes named in my brief and did not find one.

## Scratch hygiene

One scratch directory was created, `/tmp/claude-1000/rev-ep4-verify-scratch`, for the toolchain prefix's `TMPDIR`. It was never written to (no fixture build was needed for this lens; every check ran directly in the tracked worktree), confirmed empty (`ls -la` showed zero entries beyond `.`/`..`), and removed with `rmdir` before this file was written. DIRECTORIES LEFT IN `/tmp`: 0.
