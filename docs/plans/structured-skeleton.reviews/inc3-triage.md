# Inc 3 triage - render engine (`structured-skeleton-inc3`)

Triager verdicts on the three reviewer findings files for commit `17a328e` on base `9afe567`.
Independent adjudication: I did not write or review the code. Severity is absolute impact if left unfixed, on `low`/`medium`/`high`/`critical`. Fix-now means the fix belongs in this increment (Inc 3) before the enforcement swap (Inc 4) and the live migration (Inc 5); defer means it is legitimately settled later.

Read against `AGENTS.md`, `.agents/prompts/triager.md`, the Inc 3 bullet in `docs/plans/agent-scaffold.md:686`, and `docs/plans/structured-skeleton.explorations/synthesis.md` (section 3 and 3(f), section 4 Inc 5 expected-diffs). Confirmed against `src/plan/render.rs` and `src/plan/source.rs` at the commit, and by running `cargo test --all-targets` in the worktree.

No finding overlaps another. All are distinct. All are VALID; none dismissed. Verified facts feed the verdicts below.

---

## Correctness findings

### C1 (VALID, medium) - open-question count excludes `exploring`

`render.rs:333-334` counts only `QuestionStatus::Open`. The exclusion of `decided`/`superseded` is clearly right; excluding `exploring` is a genuine semantics choice, not a mechanical bug, and it was not consciously decided. The number is load-bearing (it is the whole reason the derived Status line exists) and it is UNTESTED: the fixture carries no `exploring` question, so no test pins either reading, and it will be baked into the committed golden either way. VALID at medium because a wrong reading is silently frozen into a load-bearing derived line that Inc 4/5 make authoritative.

- ORCHESTRATOR DECISION (semantics): see the C1 recommendation block below.
- Fix must ACHIEVE: a decided and documented count semantics, plus a render test containing an `exploring` question that pins the chosen count so it cannot regress. The test is owed regardless of which semantics is chosen.
- FIX-NOW. The golden is baked here and the live plan uses `exploring`; the count must be right before render becomes authoritative.

### C2 (VALID, low) - degenerate diff summary on a trailing-newline-only mismatch

`render.rs:210-226`. `str::lines()` discards a final trailing newline, so a trailing-`\n`-only mismatch (a common formatter mutation) yields identical lines and identical counts, and the summary reads "the committed file has N line(s); a fresh render has N" while claiming they differ. Reviewer reproduced it. Detection is correct (the byte compare in `check_render` still flags the mismatch); only the human explanation is degenerate. VALID low, diagnostic quality only.

- Fix must ACHIEVE: when no differing line is found, the summary distinguishes and words the trailing-whitespace / trailing-newline case from a byte comparison rather than from `lines().count()`.
- FIX-NOW. Cheap; it is the user-facing message of the increment's own guard.

### C3 (VALID, low) - present-but-unreadable committed `.md` misreported as absent

`render.rs:245-256`. The `Err(_)` arm collapses every read failure (invalid UTF-8, permission error) into `committed_exists: false`, which the CLI words as "does not exist". Reviewer reproduced it with invalid-UTF-8 bytes. This defeats the documented purpose of the `committed_exists` flag (`render.rs:66-68`). Low because render always writes valid UTF-8, so this only triggers on external corruption or a permissions problem; the Mismatch verdict itself stays correct. VALID low.

- Fix must ACHIEVE: only `ErrorKind::NotFound` (or a `try_exists` probe) reports the committed file as absent; any other read failure is reported as present-but-unreadable.
- FIX-NOW. Cheap; restores the flag's documented contract.

### C4 (VALID, low) - waiver-reason breakdown hardcodes the variants instead of an exhaustive `ALL`

`render.rs:341-351` iterates a hand-written literal array of the three `WaiverReason` variants, while the rest of the module iterates `StepStatus::ALL` / `QuestionStatus::ALL` for exactly this determinism/exhaustiveness reason. Correct today, but a fourth `WaiverReason` would be counted by `waivers.len()` (the total) yet silently dropped from the parenthetical breakdown, with no compile error. Latent-drift correctness gap. VALID low.

- Fix must ACHIEVE: the breakdown iterates a compiler-checked exhaustive source (a `WaiverReason::ALL` constant beside the other `ALL` arrays), so the parts always sum to the total.
- FIX-NOW. Cheap; matches the module's established pattern and forecloses a silent-drift regression.

### C5 (VALID, low) - numeric-ordering determinism claims are under-tested

`render.rs` tests + `render-fixture.*`. The code is correct (questions sort by `question_id_index`, a `u64` parse, confirmed `Q-10` after `Q-9` by the reviewer's live render), but NO test guards it: the fixture queue is `Q-1..Q-3`, which sort identically lexically and numerically, so a regression to a lexical sort stays green. Two sibling claims are likewise unpinned: the equal-`order` slug tiebreak (`render.rs:175`; fixture uses distinct orders 1..4) and the `skipped`/`optional`/`deferred` Status-line buckets (absent from the fixture). Determinism/ordering is the core promise of this risky increment (Principle 11: tests must exercise the claimed behavior). VALID low.

- Fix must ACHIEVE: a test (extended fixture or a focused `assemble` case) that pins numeric `Q-10`-vs-`Q-9` order, an equal-`order` slug tiebreak, and at least one `skipped`/`optional`/`deferred` step in an assembled Status line.
- FIX-NOW. This guards the increment's central claim; cheap to add.

---

## Robustness findings

### R1 (VALID, medium) - `[meta].sidecars.front`/`tail` refs are unvalidated -> path traversal / absolute-path read outside the plan dir

`render.rs:164-167` joins the free-string front/tail refs straight onto the base dir; `validate_source` (`source.rs`) validates slugs, ids, the orphan list, and the W4 baseline but NEVER inspects `meta.sidecars` (confirmed: it handles `w4_baseline` at `source.rs:531` and `orphan_tasks` at `:542`, nothing for `front`/`tail`). A `..`-bearing ref escapes the base dir and an absolute ref discards it, so a crafted `.plan.toml` makes render read any process-readable file and splice its bytes verbatim into `<task>.md`. Reviewer reproduced both. Violates Principle 21 (validate/parse external input at the boundary) and Principle 18 (least authority), and defeats the banner's own "task-relative names, deterministic regardless of checkout location" model. Read-only and the source is in-repo today, which caps it at medium; the planned `git-url-fetch` path would let an untrusted `.plan.toml` exfiltrate arbitrary file contents into a committed, agent-read artifact. VALID medium.

- Fix must ACHIEVE: front/tail refs are validated at the boundary in `validate_source` (so `render`, `render --check`, and `validate --source` all reject the same thing), rejecting any ref that is absolute or whose components contain `..`. The step/question sidecar joins are already slug/id-constrained; extending the same rule to them is optional defense in depth.
- FIX-NOW. Inc 3 is the first consumer of these refs; the boundary check belongs with the code that introduces the read, and it is cheap (component-level reject, no dependence on the files existing).

### R2 (VALID, low) - `escape_cell` neutralizes `|` and `\n` but not `\r`

`render.rs:476-478`. The only free-text reaching a table cell is a waiver `note`; a lone `\r` (or the `\r` left when `\r\n`'s `\n` becomes a space) passes through unescaped, and CommonMark treats a lone CR as a line ending, splitting the Roadmap row. Reviewer reproduced it. In-repo TOML and `render --check` still round-trips byte-for-byte, so CI catches drift not this; but the escape exists precisely to be injection-proof and is incomplete. VALID low.

- Fix must ACHIEVE: `escape_cell` neutralizes `\r` (and `\r\n`) as it does `\n`, so no cell text can carry a line ending into a table row.
- FIX-NOW. One-line completion of an escape the increment introduced.

### R3 (VALID, low) - the successful write is truncate-then-write, not atomic

`run_render` non-check branch, `fs::write(&out, rendered)`. The writes-nothing-on-FAILURE contract holds for the named failure modes (schema / xref / missing sidecar are all detected in memory before any write; a pre-existing `<task>.md` is left byte-identical, verified by the reviewer). The gap is the SUCCESS path: `fs::write` truncates first, so a write interrupted by an I/O failure (disk full, kill, permission change mid-write) leaves a previously-valid `<task>.md` partial or empty, which an agent could read as authoritative. Low: requires an I/O failure at the write, not a crafted input. VALID low.

- Fix must ACHIEVE: the write is atomic-replace (write to a temp file in the same directory, then `fs::rename` into place), so an interrupted write leaves the prior `<task>.md` intact, extending the increment's "never a partial plan" intent to a broken write.
- FIX-NOW. Small, standard, and it is the one write path this increment introduces.

---

## Fidelity findings

### F1 (VALID, medium) - question bodies inlined in the queue, not relocated to a `## Question Details` section

`render.rs:390-406`, `questions_section()`. Synthesis 3(f) carries an explicit ADOPT: "Relocate question bodies to a `## Question Details` section mirroring `## Step Details`", whose stated reason is a scannable one-line-per-item queue. The render engine is the Inc 3 deliverable and is where this output-format decision lands. The code instead writes each body inline after its one-line item, separated by a blank line; confirmed in code that the blank line before the body ends the Markdown list, so a following `Q-<n>` starts a NEW list, fragmenting the queue -- the exact opposite of the ADOPT's stated reason. This is a genuine deviation from a settled synthesis decision that the owning increment implements, not a migration detail. VALID medium.

- ORCHESTRATOR DECISION (confirm the ADOPT still holds): see the F1/F2/F3 recommendation block. The ADOPT is already recorded in 3(f), so this is a confirmation, not a fresh open question.
- Fix must ACHIEVE: question body sidecars render in a dedicated `## Question Details` section (mirroring `step_details_section`), leaving the queue a clean one-line-per-item list; a question with no body prose contributes no entry.
- FIX-NOW. It is a settled output-structure decision the render engine owns, and the current output is a broken list. Getting it right in Inc 3 is what lets Inc 5's "question-body relocation" show up as an expected diff rather than never happening.

### F2 (VALID, low) - fixed section order cannot place Repository Layout in its live-plan position

`render.rs:275-303`, `assemble()`. The section order is fixed with front sidecars BEFORE the generated Principles/Vocabulary, and there is no mechanism to place a sidecar between two generated sections. The live plan puts `## Repository Layout` AFTER `## Documentation Protocol` (the generated Vocabulary), so at Inc 5 it becomes either a front sidecar (wrong order, before Principles) or a second tail (the single tail is taken by Success Criteria). Section reordering is NOT in the synthesis's stated Inc 5 expected diffs ("banner, derived Status line, question-body relocation"), so either that list is incomplete or the render order needs an interleave mechanism. The finding (a real gap between the fixed order and the live-plan order, not covered by the expected-diffs list) is correct; the render engine is internally consistent and this only bites at migration. VALID low.

- ORCHESTRATOR DECISION: see the F2 recommendation block.
- Fix must ACHIEVE (whichever the orchestrator picks): either the Inc 5 expected-diffs list is amended to include the Repository-Layout reorder (accepting the minor reorder), or the render engine gains a documented mechanism (for example a `[meta].sidecars.mid` slot between Vocabulary and Questions) to place a sidecar between generated sections. Either resolves the gap.
- DEFER to Inc 5, but decide the direction now. The mechanism, if wanted, is cheaper to add while the section-ordering design is fresh than to retrofit; the orchestrator should choose before Inc 5 begins.

### F3 (VALID, low) - generated `## Documentation Protocol` heading collides with the live plan's section of the same name

`render.rs:376-387`, `vocabulary_section()`. The generated 2-line status-vocabulary fragment is headed `## Documentation Protocol`, the same heading the live plan uses (`agent-scaffold.md:29-71`) for ~6 paragraphs of hand-authored formatting rules that agents reference by name. Post-migration the generated stub claims a heading whose live content is not reproducible from the TOML, so a reader sees a subset masquerading as the section. The synthesis does not pin this heading, so it is an implementation choice; renaming decouples the generated fragment from the live section and is cheap. VALID low. Two parts: the heading name (an Inc 3 output decision, baked into the golden now) and preserving the live formatting-rules prose (an Inc 5 sidecar/expected-diff decision).

- ORCHESTRATOR DECISION: see the F3 recommendation block.
- Fix must ACHIEVE: the generated vocabulary section carries a heading that does not collide with the live plan's `## Documentation Protocol` (for example `## Status Vocabulary`), so the live formatting-rules prose can later live under its own heading without ambiguity; and the Inc 5 expected-diffs list records that the live section's full content is not TOML-reproducible.
- FIX-NOW for the rename (cheap, avoids baking a collision into the golden); DEFER the formatting-rules preservation to Inc 5.

### F4 (VALID, low) - `render_sha256` fixture placeholder is an all-zeros value resembling a real hash

`render-fixture.plan.toml` `[meta]`. The field is correctly documented as vestigial for this increment (design A byte-compares against the committed `.md`), but the fixture's all-zeros value reads like a real zeroed hash rather than a placeholder, and the schema accepts it silently, so a reader who has not read the module doc may assume it is computed. The field is `Option<String>` and is exercised by no render test. VALID low, fixture hygiene.

- Fix must ACHIEVE: the fixture no longer implies the field is active, either by omitting `render_sha256` (it is optional and unused) or by giving it an obviously-placeholder value.
- FIX-NOW. Trivial and removes a latent misreading before the fixture is copied for the pilot.

### F5 (VALID, low) - the no-clobber test asserts the output path but not that sources are unchanged

`render.rs:590-609`, `render_writes_exactly_one_file_and_check_is_green_after`. The comment claims "no sidecar or the TOML changed," but the assertions only check the output path and its existence, not that the sources are unmodified or that no extra file was created. The no-round-trip / no-clobber property is a principal guarantee of the increment (synthesis section 1, section 5), so the test should enforce it (Principle 11: a test must exercise what it claims). The code is correct in practice; this is a coverage gap. VALID low.

- Fix must ACHIEVE: the test enforces the no-clobber invariant, for example by recording the sources' mtimes/contents before the render and asserting they are unchanged after, and that no file beyond `<task>.md` was created.
- FIX-NOW. Cheap; it pins a load-bearing invariant the increment is built on.

---

## Non-finding: the 275-vs-276 test count

Both the correctness reviewer (header) and the robustness reviewer (N1) noted the implementer's claim of 276 tests against an actual 275. I reran `cargo test --all-targets` in the worktree: 271 (unit) + 1 (`checks_staged_hook_env`) + 3 (`scaffold_precommit_hook`) = 275, with no doctests executed. The 276 is a plain miscount, not a defect and not a code finding. Worth reconciling in the ledger/implementer note only; no remediation.

---

## Orchestrator-decision inputs

These are recommendations for the human/orchestrator to settle, per the human-input contract. I recommend but do not decide.

### C1 - open-question count semantics

Options: (a) count `Open` only (current); (b) count `Open | Exploring`.
Recommendation: (b) count `Open | Exploring`, and consider naming the number to match its meaning (for example "unresolved questions"). Reasoning: AGENTS.md and the schema (`plan.rs:82-84`) describe `exploring` as an UNRESOLVED sub-state of open (a design pass is still owed), and the plan's queue is the single human-decision queue; the Status line's job is to convey how many decisions are still outstanding, and an exploring item is one the human still owes. Excluding it undercounts outstanding work. The existing `status`/`validate` path uses a third metric again (`open_questions.len()`, the whole queue, `main.rs:573`/`:719`), so there is no single precedent to inherit; whichever is chosen, add the pinning test.

### F1 - question bodies to a `## Question Details` section

Recommendation: implement it (fix-now). Reasoning: 3(f) already records this as ADOPT; the render engine is where it lands; and the current inline output is a broken Markdown list, contrary to the ADOPT's stated reason. This is confirming an already-settled decision, not opening a new one.

### F2 - Repository Layout section position at migration

Recommendation: accept the reorder as an added Inc 5 expected diff (document it) rather than building a `mid`-sidecar mechanism now, UNLESS exact live-plan section fidelity is a hard requirement. Reasoning: a `mid` mechanism is added schema and render surface for one section's placement (YAGNI unless fidelity is required), while a documented reorder is free and the plan's readers tolerate a section-order change a reviewer signs off once (parallel to how 3(f) treats the question-body relocation). Decide before Inc 5 either way, since the mechanism is cheaper to add now than to retrofit.

### F3 - generated vocabulary section heading

Recommendation: rename the generated section (for example `## Status Vocabulary`) now, and record in the Inc 5 expected-diffs that the live `## Documentation Protocol` formatting-rules prose is not TOML-reproducible (it moves to a sidecar or is pruned at migration). Reasoning: the heading is an unpinned implementation choice; keeping `## Documentation Protocol` for a 2-line stub collides with a live section agents reference by name and bakes that collision into the golden now. Renaming is cheap and decouples the two cleanly.

---

## Tally

Valid findings: 13 (all findings VALID; none dismissed). No high or critical, so no dismissal backstop re-check is triggered.

- Medium: 3 - C1, R1, F1.
- Low: 10 - C2, C3, C4, C5, R2, R3, F2, F3, F4, F5.

Non-finding noted: the 275-vs-276 test-count miscount (reconcile in the ledger, no fix).

## Fix-now remediation list (Inc 3)

Route to an isolated implementer for the Inc 3 fix round, after the two orchestrator decisions (C1 semantics, F1 confirm) are settled:

1. R1 (medium): validate `meta.sidecars.front`/`tail` at the boundary in `validate_source` (reject absolute and `..`-bearing refs).
2. F1 (medium): relocate question bodies to a `## Question Details` section; queue becomes one-line-per-item.
3. C1 (medium): apply the decided count semantics and add a render test with an `exploring` question.
4. C5 (low): add ordering tests (numeric `Q-10`-vs-`Q-9`, equal-`order` slug tiebreak, a `skipped`/`optional`/`deferred` Status-line bucket).
5. C2 (low): word the no-differing-line diff summary from a byte comparison, not `lines().count()`.
6. C3 (low): report a non-`NotFound` read failure as present-but-unreadable, not absent.
7. C4 (low): iterate a `WaiverReason::ALL` exhaustive constant for the reason breakdown.
8. R2 (low): also escape `\r` (and `\r\n`) in `escape_cell`.
9. R3 (low): make the successful write atomic (temp file + `fs::rename`).
10. F3 (low, rename part): rename the generated vocabulary section heading off `## Documentation Protocol`.
11. F4 (low): drop or clearly-mark the `render_sha256` placeholder in the fixture.
12. F5 (low): assert the no-clobber invariant (sources unchanged, no extra file) in `render_writes_exactly_one_file...`.

Deferred to Inc 5:

- F2: decide (now) then apply at migration - document the Repository-Layout reorder as an expected diff, or add a `mid`-sidecar mechanism.
- F3 (content part): preserve the live `## Documentation Protocol` formatting-rules prose as a sidecar (or prune it) and record it in the Inc 5 expected-diffs.
