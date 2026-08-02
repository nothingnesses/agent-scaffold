# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 3, REVIEWER: fix verification and residue

Reviewer: independent of the planner and of every prior reviewer and triager on this fold. Read-only with respect to the reviewed artifact; this file is the only thing written.

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-ep3-verify`, branch `review/q55-ep3-verify`, at `133324d` (the round 2 fix pass plus its one-word follow-up). Binary built at this commit with `cargo build`. All fixtures and probes ran under `TMPDIR=/tmp/claude-1000/rev-ep3-verify-scratch`, created for this review and removed at the end.

Repository guards re-run at the reviewed commit, both green:

```
$ cargo run -q -- render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date
exit: 0

$ cargo run -q -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 255 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

So the projection (`docs/plans/agent-scaffold.md`) is a faithful mechanical regeneration of the sidecar, and the log has grown to 255 records since the round 2 triage measured 253.

## Verdict summary

ONE finding, `medium`. All ten round 2 fixes (`R2A-1` through `R2A-7`, the merged `R2A-3`/`R2B-1`, and `R2A-4`'s two parts) are CLOSED at the sites the round 2 triage measured, in the class it prescribed. Neither residual (`R2B-2`, `R2B-3`) was re-raised.

| id | severity | one line |
| --- | --- | --- |
| `R3A-1` | medium | The PROVENANCE bullet list omits `Q-55-resumecost`, the receipt that authorises accepted cost (iv), even though the same fix pass cites that q_id twice elsewhere in the document. |

## Job 1: fix closure, checked at the site the triage measured

I read `git diff 525a3b0 133324d -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` (the whole round 2 fix, both commits `7e1a0ff` and `133324d`) hunk by hunk against the round 2 triage's prescriptions, then grepped the full tracked population (`docs/plans/agent-scaffold.steps/`, `docs/plans/agent-scaffold.plan.toml`, `docs/plans/agent-scaffold.md`) for every stale phrase the triage named, to confirm zero residue rather than trusting the diff alone.

- `R2A-3`/`R2B-1` (high, merged): DELETION confirmed at BOTH prescribed sites. Cost (iii), `workflow-enforcement-tier.md:269`, no longer carries "THE BOUND, measured: the same layout is ALREADY refused...". Check 19b, `:350`, no longer carries "The same layout in its no-`--source` spelling is refused too...". `grep -rn "no-\`--source\` spelling"` and `grep -rn "removed rescue\|removes a rescue\|new species\|introducing a species"` over the sidecar, `agent-scaffold.plan.toml` and `agent-scaffold.md` return ZERO hits; the phrase survives only inside `docs/plans/agent-scaffold.ledger.md:407` and `:421`, which is the orchestrator's own decision-time record and its APPENDED correction, out of scope per this round's brief. CLOSED, at both sites, by deletion as prescribed (not narrowed).
- `R2A-2` (high): NARROWING confirmed at both prescribed sites. Cost (iii)'s heading now reads "WITH A MARKDOWN-PRIMARY `--source` INSIDE ONE" (`:269`) and check 19b's fixture now reads "with `x.plan.toml` MARKDOWN-primary" (`:350`). `grep -c "MARKDOWN-PRIMARY\|MARKDOWN-primary"` returns 3 in the sidecar and 3 in the projection (matching, mechanical). CLOSED.
- `R2A-1` (medium): DELETION confirmed. The inc2 risk paragraph (`:313`) no longer carries "and only check 13b and 14g's fourth run catch"; it now ends "...so rooting the guard on the anchor is a defect that check 11 passes over." `grep -rn "only check 13b"` over the full population returns ZERO hits. CLOSED.
- `R2A-4` part 1 (medium, deletion available now): CONFIRMED. `:192` now reads "IT IS REACHED BY an explicit `--ledger-fragment` outside that root, and the DEFAULT ledger under a divergent pairing..." in place of "TWO CASES REACH IT:". `grep -rn "TWO CASES REACH IT"` returns ZERO hits. This also closes `R2B-2` as the triage predicted (the exhaustiveness foothold the indeterminacy reading depended on is gone).
- `R2A-4` part 2 (cost record, contingent on the human): the human decided `Q-55-resumecost` (workflow.jsonl record 255, `"chosen":"Accept as (iv), queue the shared cause"`, `ts:"2026-08-02"`), so the contingent fix landed: new accepted cost (iv) at `:271` ("`status --resume` ON THE SAME PAIR OMITS THE PROJECT'S OWN BLOCK, in EITHER `primary` spelling, so its population is WIDER than (iii)'s") and the ~8-word pin clause at check 19b, `:350` ("`status --resume` omits its block in EITHER `primary` spelling"). Both match the triage's word-count estimate and its instruction that the sentence must NOT inherit cost (iii)'s new `primary` qualifier, since the resume rule does not carry it; confirmed accurate: `project_root_of_source` (`src/main.rs`, its `parent.to_path_buf()` fallback at the end of the ancestor walk) computes the root from path shape alone and never reads `[meta].primary`, so the claim "in EITHER primary spelling" is correct. CLOSED.
- `R2A-5` (medium): DELETION confirmed. `:189` now ends "The predicate is never re-implemented per surface (One source of truth)." with the second clause removed. `grep -rn "enumerated at the end of the mechanism section"` over the full population returns ZERO hits, and I confirmed no other passage refers back to that deleted enumeration claim. CLOSED.
- `R2A-6` (low): DELETION confirmed. Check 14g (`:343`) no longer carries "and both exit 0"; `grep -rn "and both exit 0"` over the full population returns ZERO hits. CLOSED.
- `R2A-7` (low): DELETION confirmed. `:24` now reads "...the design pass, and further human decisions all landed after it." with "two" removed rather than corrected to a new number. `grep -rn "and two further human decisions"` returns ZERO hits. CLOSED, and durably: the true count of "further human decisions" is now SIX (`Q-55-refusalscope`, `Q-55-jsonreason`, `Q-55-endproperty`, `Q-55-conventionlesscost`, `Q-55-resumepairing`, `Q-55-resumecost`), one more than the round 2 triage's count of five, and the numeral-free phrasing absorbs that without going stale again.
- `R2B-2`, `R2B-3`: accepted residuals, not re-raised, as instructed.

Separately, the fix pass caught its own orchestrator-induced staleness: `:10`'s "with NINE decision receipts" was deleted in `133324d` after the `Q-55-resumecost` receipt landed mid-loop and pushed the true count to ten. `grep -rn "NINE decision receipts"` over the full population returns ZERO hits (it survives only inside the ledger's own historical quotation of the pre-correction text, out of scope). That numeral-deletion is correct and durable by the same "prefer no count" logic as `R2A-7`. But the bulleted list immediately below the same sentence was not given the same treatment, which is `R3A-1` below.

## `R3A-1`, medium. The PROVENANCE bullet list omits `Q-55-resumecost`

EVIDENCE. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:10-20` reads:

```
PROVENANCE. `Q-55`, decided by the human on 2026-07-31, with decision receipts in `docs/metrics/workflow.jsonl` (the last taken on 2026-08-02, after inc1's work review), all carrying `task:"workflow-enforcement-tier"`:

- `q_id:"Q-55"`, ...
- `q_id:"Q-55-scope"`, ...
- `q_id:"Q-55-mechanism"`, ...
- `q_id:"Q-55-noconvention"`, ...
- `q_id:"Q-55-refusalscope"`, ...
- `q_id:"Q-55-jsonreason"`, ...
- `q_id:"Q-55-endproperty"`, ...
- `q_id:"Q-55-conventionlesscost"`, ...
- `q_id:"Q-55-resumepairing"`, ...
```

NINE bullets. I counted the actual distinct `Q-55*` receipts in the log myself rather than trusting the list:

```
$ grep -o '"q_id":"Q-55[^"]*"' docs/metrics/workflow.jsonl | sort -u | wc -l
10

$ grep -n '"q_id":"Q-55-resumecost"' docs/metrics/workflow.jsonl
255:{"type":"decision","task":"workflow-enforcement-tier","q_id":"Q-55-resumecost","options":["Accept as (iv), queue the shared cause","Accept as cost (iv), nothing queued","Carve out the conventionless case"],"recommendation":"Accept as (iv), queue the shared cause","chosen":"Accept as (iv), queue the shared cause","ts":"2026-08-02"}
```

TEN receipts exist; the bulleted list names NINE. `Q-55-resumecost` is the missing one, and it is not an orphaned or unused id: the SAME fix pass cites it twice elsewhere in this document, at `:263` ("(iv) as `Q-55-resumecost`") and by implication at check 19b's heading ("ACCEPTED COSTS (iii) AND (iv)"). It is also record 255, the newest receipt in the log, one entry after `Q-55-resumepairing` at record 253 and both dated `2026-08-02`, so the list's own qualifier "the last taken on 2026-08-02" no longer identifies which receipt is actually last.

WHY THIS IS NOT `R2B-3` AGAIN. `R2B-3` was accepted as residual because its two summary paragraphs (`:290`, `:313`) never held themselves out as an enumeration and nothing in them was false. The PROVENANCE list is different in kind: it is a structured bulleted registry whose stated purpose is to ground every claim in the document to a receipt in `docs/metrics/workflow.jsonl`, which is this project's own `Q-66` reproducible-evidence discipline applied to decision provenance. The same fix pass already treated this exact registry's completeness as worth fixing once this round, when it deleted the stale "NINE" numeral at `:10` for precisely this reason (a receipt landed mid-loop and pushed the count up); it did not apply the same correction to the list one line below, which is the same defect at the list level that was just fixed at the numeral level.

WHY MEDIUM, NOT HIGH OR LOW. Not high: nothing false was relayed to the human, no acceptance check's exit code depends on this list, and the missing citation is redundantly available two other places in the same document, so no reader is left without a route to `Q-55-resumecost`'s authorization. Not low: this is a structured, purpose-built registry (not a casual prose aside like `R2A-6`/`R2A-7`), it is incomplete for the exact decision this round's fix pass exists to record, and the project already treated an adjacent instance of the same hazard (the numeral) as worth a same-round fix rather than deferral.

MINIMAL FIX. This is an omission, not a false or stale claim, so deletion does not apply; the minimal fix is to ADD one bullet in the list's own established format, for example: "`q_id:"Q-55-resumecost"`, accept the `status --resume` cost as (iv) and queue the shared root cause." roughly matching the length of the other bullets. Do not add a numeral anywhere in this passage; the list should stay a plain enumeration, not a counted one, per the same logic that just retired "NINE".

## Job 2: hunted residue from the 77 authored words

Checked each authored addition against the code and against the rest of the document, not just against its own internal consistency:

- New cost (iv) paragraph (`:271`) and its "in EITHER `primary` spelling" claim: verified against `src/main.rs:project_root_of_source` (the `parent.to_path_buf()` fallback taken when no `docs/plans`-shaped ancestor is found on the walk) that root derivation never reads `[meta].primary`, so the claim holds for both spellings. No residue.
- Check 19b's pin clause: matches the ~8-word estimate and does not inherit cost (iii)'s `primary` qualifier, which the triage specifically warned against. No residue.
- The queued-root-cause paragraph (`:281`, "COSTS (iii) AND (iv) SHARE ONE ROOT CAUSE..."): cross-checked against `docs/plans/agent-scaffold.ledger.md:423`, the `Q-55-resumecost` decision record ("THE QUEUED ITEM: costs (iii) and (iv) share ONE root cause, `src/main.rs:project_root_of_source`'s fallback to the plan's own parent... treating it ONCE in the validation-constraints step..."). The sidecar paragraph is a faithful, slightly compressed transcription of a decision the human actually took, not an invented commitment. No residue.
- `R2A-2`'s narrowing and `R2A-1`'s and `R2A-5`'s deletions: checked for dangling antecedents by grepping the deleted clauses' exact wording across the whole tracked population; all return zero hits outside the ledger's own historical quotations, and I read the sentences immediately before and after each edit site to confirm nothing else refers back to the removed text. None found.
- Title/count adjustments ("The four accepted costs" at `:261` and its cross-reference at `:368`, "IT IS REACHED BY" at `:192"): both consistent with the new cost (iv) and with each other.

No other fix-induced defect found beyond `R3A-1`.

## Job 3: counts, re-counted myself

| claim | document | my count | method |
| --- | --- | --- | --- |
| defects (A-D) | four | 4 | counted bullets at `:5-8` |
| accepted costs | four ((i)-(iv)) | 4 | counted `:265`, `:267`, `:269`, `:271` |
| increments | three (inc1-inc3) | 3 | counted bullets at `:289-291` |
| doc comments `Q-55-jsonreason` falsifies | four | 4 | counted bullets at `:209-212`; a stated 5th (`:214`) is explicitly excluded from the four, consistent |
| design-pass exploration records, total lines | 3 records, 1514 lines (521+483+510) | 3 records, 1514 lines | `wc -l docs/plans/workflow-enforcement-tier.explorations/*.md` |
| PROVENANCE bullet list | nine bullets, implicitly presented as the receipt registry | 9 bullets, but 10 distinct `Q-55*` receipts actually exist in the log | `grep -o '"q_id":"Q-55[^"]*"' docs/metrics/workflow.jsonl \| sort -u \| wc -l` -> 10; see `R3A-1` |
| "and two further human decisions" | no numeral (deleted by `R2A-7`'s fix) | actual count now 6, was 5 at round 2 triage | counted the receipts named in `:12-20` plus `Q-55-mechanism`/`Q-55-noconvention` excluded as "the design pass" per the round 2 triage's own method; the numeral-free phrasing does not need updating |
| "NINE decision receipts" | deleted | N/A, correctly no longer asserted | `grep -rn "NINE decision receipts"` -> 0 hits outside the ledger |
| workflow.jsonl total records | not asserted as a fixed figure anywhere load-bearing | 255 | `validate --workflow` output; up from 253 at the round 2 triage and 254 (untraced query) by the time of this round, consistent with the log's own documented property of growing during the loop |

No count re-broke among the ones round 2 fixed. The one newly-stale item is `R3A-1`, caused by the same mid-loop receipt append the project's own standing rule (ledger, "an artifact under review MUST NOT assert a count of anything the orchestrator appends to during the loop") was written to guard against, applied here to a bulleted list rather than a numeral.

## Out of scope, checked and not raised

`docs/plans/agent-scaffold.plan.toml` was not touched by either round 2 fix commit (`git diff 525a3b0 133324d -- docs/plans/agent-scaffold.plan.toml` is empty); its own decision narrative predates `Q-55-endproperty` entirely and was not raised by round 1 or round 2, so it stays out of scope here. `docs/plans/agent-scaffold.ledger.md` is not part of the amended artifact (it carries the orchestrator's own APPENDED corrections and round narration, untouched by `7e1a0ff`/`133324d`); I read it for grounding but raised nothing in it. `R2B-2` and `R2B-3` were not re-raised. The `--metrics` relative-default text, the `default_ledger_path` current-directory text, the "Documentation impact INC1" sub-list, and the two help-string descriptions were not touched and were not raised. Accepted costs (i) and (ii), increments 1 and 3, and the four settled human decisions themselves were not revisited; only whether they are recorded correctly and executably was in scope, and only `R3A-1` bears on that. Line length and hard-wrapping were not raised. `grep -nP '[^\x00-\x7F]'` over this findings file returns nothing.

## Scratch hygiene

All probes ran under `TMPDIR=/tmp/claude-1000/rev-ep3-verify-scratch`, created for this review. No fixtures were built under it beyond directory creation for the `TMPDIR` itself (this round's verification was closable by static analysis, code citation, and the two repository-guard commands above, so no scaffold fixture was needed). The directory was removed when this review finished. DIRECTORIES LEFT IN `/tmp`: 0.
