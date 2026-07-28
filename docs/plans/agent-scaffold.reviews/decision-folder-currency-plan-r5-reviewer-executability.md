# Plan review, round 5, reviewer: executability

Artifact: the planner's fold at `72ca2d7` (diff `7707df2..72ca2d7`), adding step 90 `decision-folder-currency`, step 92 `prompt-drift-guard`, and the `exploring` question `Q-69`.

Lens: can this plan be carried out, correctly, by an implementer with no other context? Method: read `docs/plans/agent-scaffold.steps/decision-folder-currency.md` as that implementer, work out the edit for each of the four passages before looking at the files, then check the answer against the real files; then verify the regeneration command by running it; then check step 92 against the real code and `Q-69` against its cited evidence.

Worktree: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev5-exec` at `72ca2d7`. Every line cited below was re-read in this worktree; every command below was run in it (or, where it writes, into a scratch copy).

## FINDINGS: NONE

Zero findings at every severity.

- `critical`: none.
- `high`: none.
- `medium`: none.
- `low`: none.

I did not suppress anything. The one candidate I developed independently is recorded under "Candidate examined and not raised" below, with the reason it is not a finding and the settled verdict it would have re-litigated.

## Central question: could a competent implementer execute step 90 from its sidecar alone?

YES. I located all four passages from the sidecar alone, on the first attempt, with no ambiguity about which paragraph, which sentence, or which branch. The operation class the sidecar assigns to each passage is correct against the real line. The scope requirement is stated as a prohibition in the imperative ("do NOT write an unqualified 'the planner authors that fold' into any of the four passages", `decision-folder-currency.md:26`), which is not missable, and the two prescriptive instructions that do not use the word "non-trivial" (`:19`, the checkpoint) instead spell out the qualifier inline as "a decided decision's `[[question]]` or `[[step]]` fold", which IS the non-trivial case by `pack/AGENTS.md:41`'s inline definition. So the qualifier survives either route.

### Passage 1: `pack/prompts/orchestrator.md:27`, the checkpoint paragraph

Located from `decision-folder-currency.md:12` ("the paragraph beginning 'At each checkpoint, sync the durable state before moving on'"). The quoted clause "There, update the plan's Open Questions queue and push its open items to the human" reproduces verbatim at `:27`.

Operation class: ACTOR NAMED, WRONG ACTOR. Correct. `:27` is second-person imperative addressed to the orchestrator ("update the plan's Open Questions queue"), so it does name an actor and it is the one the rule reserves the work from.

The edit I would make, appending after "do not wait for the human to pull them.":

> Here updating the queue means raising and pushing the open items, not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job and which you route to it.

`:19` prescribes exactly this ("give it the SAME qualifier `pack/AGENTS.md:71` already carries ... match the guidance's existing clause, do not invent a different rule") and `:28` bounds it to `pack/AGENTS.md:71`'s main clause only. The resulting prose is consistent with the shipped `pack/AGENTS.md:71` main clause, so it creates no new drift source. It adds no enumeration of the four direct-on-main edits, satisfying `:34`.

The sidecar led me to the right edit.

### Passage 2: `pack/prompts/orchestrator.md:31`, branch 2 of the Socratic sentence

Located from `:13`. The quoted clause "for a question whose options are already clear, emit the block and record the resolved answer as a durable Open-Questions decision" reproduces verbatim at `:31`.

Operation class: ACTOR NAMED, WRONG ACTOR. Correct. The whole paragraph is second-person imperative ("Every time you put a decision to the human, emit ..."), so "record the resolved answer" is addressed to the orchestrator, and `pack/AGENTS.md:43` routes that fold "to the planner to author".

The edit I would make:

> ... for a question whose options are already clear, emit the block, then record the resolved answer as a durable Open-Questions decision, routing its non-trivial fold to the planner to author rather than editing the plan yourself; ...

This matches `pack/AGENTS.md:43` ("its non-trivial fold routed to the planner to author as above rather than edited in directly"), which is what `:20` requires.

Could I have edited the wrong branch? No. `:20` enumerates all three branches with a distinguishing quote for each, names branch 2 as the target in capitals ("EDIT THE SECOND BRANCH ONLY"), quotes branch 3's own text ("record it as an `exploring` Open-Questions item") so it can be told apart, and states in advance that the finished sentence "will read oddly until `Q-69` is resolved" so an implementer does not "fix" branch 3 for coherence. The three branches in the real sentence are exactly as described, separated by semicolons: branch 1 "answer a purely factual question directly", branch 2 "for a question whose options are already clear ...", branch 3 "for one whose design space is not yet decidable ...". The pre-emptive warning about the odd reading is the part that actually protects branch 3, and it is present.

The sidecar led me to the right edit and the right branch.

### Passage 3: `pack/prompts/orchestrator.md:33`, the ledger paragraph

Located from `:14`. The quoted clause "only durable decisions, the ones that change the plan, fold into it" reproduces verbatim at `:33`.

Operation class: ACTOR-LESS. Correct. The clause is impersonal; the sentence's only imperative ("do not put individual findings in the plan's Open Questions section") does not reach the folding clause.

The edit I would make, extending that clause:

> ... only durable decisions, the ones that change the plan, fold into it, and the planner authors that fold when it is non-trivial (a `[[question]]` or a `[[step]]`), routed by you.

`:21` sends me to SCOPE (`:26`) for the qualifier, which forbids the unqualified form. I would not have written an unqualified rule.

The sidecar's provenance claim for this passage checks out: the step-89 triager's accepted-residual note in the ledger names exactly `pack/prompts/orchestrator.md:33` and the parallel `pack/AGENTS.md:63` and no others.

```
$ grep -c 'pack/prompts/orchestrator.md:33` AND the parallel `pack/AGENTS.md:63`' docs/plans/agent-scaffold.ledger.md
1
```

The sidecar's warning not to cite that note by line number is well founded and worth keeping: the note now sits on ledger line 361 (a single 14822-character line), not the 345 or 355 an earlier draft carried.

The sidecar led me to the right edit.

### Passage 4: `pack/AGENTS.md:63`, the "Preventing relitigation (the ledger)" paragraph

Located from `:15`. The quoted clause "only durable decisions, the ones that change the plan, fold into the plan's steps" reproduces verbatim in the paragraph at `pack/AGENTS.md:63`, and the tail `:15` warns against dropping ("and a folded decision reopens only by evidence that beats its recorded reasoning") is the real continuation of that same sentence.

Operation class: ACTOR-LESS. Correct.

The edit I would make, inserting the actor clause before the existing tail:

> ... only durable decisions, the ones that change the plan, fold into the plan's steps, the planner authoring that fold when it is non-trivial, and a folded decision reopens only by evidence that beats its recorded reasoning.

`:29` is what makes this safe: it tells me the two ledger copies carry the same ACTOR CLAUSE and not the same sentence, and names the tail I must not drop. Without that I might have harmonised the two sentences and deleted the reopening clause. With it, the boundary is explicit.

The sidecar led me to the right edit.

### Would the produced prose diverge from the already-shipped guidance?

No. Each of the four edits above restates a clause that already ships verbatim in `pack/AGENTS.md` (`:41`, `:43`, `:71`), so the prompt converges on the guidance rather than introducing a fourth phrasing. `:26` forecloses the one way to diverge that matters (widening the rule by dropping the non-trivial qualifier), and `:32` forecloses the other (restating the rule in full instead of a short reinforcing clause, which would create a second authoritative statement).

## The regeneration step: verified by running it

`decision-folder-currency.md:44` substitutes a direct invocation for `just scaffold-self` and forbids the recipe because it also runs `nix fmt`.

The substituted command is correct and complete. It is byte-for-byte the render half of the recipe:

```
$ sed -n '46,48p' justfile
scaffold-self:
	{{ direnv_prefix }} cargo run -- scaffold --output-dir . --write --force --principles default --instrument
	{{ direnv_prefix }} nix fmt
```

The sidecar's command is `cargo run -- scaffold --output-dir . --write --force --principles default --instrument`: the same subcommand and the same four flags, dropping only the `direnv_prefix` (supplied by the implementer's own shell prefix) and the second line. The `justfile:46-48` citation is exact. Every flag exists on the subcommand: `--output-dir`, `--force`, `--write`, `--principles`, `--instrument` are all declared in `ScaffoldArgs` (`src/main.rs:383-420`). The prohibition's grounds also check out: "Format only your own files" is at `pack/AGENTS.md:79` and the incidental-reflow ruling is at `pack/AGENTS.md:108`, both as cited.

I ran it, without mutating any tracked file in this worktree.

Setup (a `git archive` of `72ca2d7` into a scratch directory, committed as a baseline):

```
$ git archive HEAD | tar -x -C <scratch> && git -C <scratch> init -q && git -C <scratch> add -A && git -C <scratch> commit -q -m base
```

Run 1, the exact command from the sidecar, targeting the scratch tree:

```
$ cargo run --quiet -- scaffold --output-dir <scratch> --write --force --principles default --instrument
...
         refresh  .agents/prompts/orchestrator.md
...
Wrote to <scratch> (30 changed, 0 left untouched).
$ git -C <scratch> status --porcelain
(no output)
```

So the render is already byte-identical to every committed scaffolded asset, and the command changes nothing else. This is the empirical form of the claim at `:44` that "the raw render therefore satisfies the guard on its own", and it also confirms `:42`'s "No other deployed file is affected".

Run 2, the pack-source direction (does a `pack/prompts/` edit actually reach the deployed copy through this command, given the pack is embedded at compile time?):

```
$ printf 'PACK EDIT MARKER SENTINEL\n' >> <scratch>/pack/prompts/orchestrator.md
$ cd <scratch> && CARGO_TARGET_DIR=<scratch-target> cargo run --quiet -- scaffold --output-dir <scratch> --write --force --principles default --instrument
Wrote to <scratch> (30 changed, 0 left untouched).
$ git -C <scratch> status --porcelain
 M .agents/prompts/orchestrator.md
 M pack/prompts/orchestrator.md
$ tail -1 <scratch>/.agents/prompts/orchestrator.md
PACK EDIT MARKER SENTINEL
$ diff <scratch>/pack/prompts/orchestrator.md <scratch>/.agents/prompts/orchestrator.md && echo EMPTY
EMPTY
```

The pack edit propagated through the rebuild (`build.rs` emits `cargo:rerun-if-changed` for `pack/` and every file under it, so no `cargo clean` is needed), the deployed copy was regenerated, and exactly two files changed. The mid-run hand edit I also tried (appending a marker to the deployed `.agents/prompts/orchestrator.md` and re-running) was restored to the pack's content, confirming the deployed copy is derived, not authoritative.

The acceptance criterion at `:30` is therefore objective and satisfiable: `diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md` produces no output after the regeneration. Its supporting claim is also true today:

```
$ grep -c "{{" pack/prompts/*.md
pack/prompts/checks-reviewer.md:0
pack/prompts/clarifying-questions.md:0
pack/prompts/implementer.md:0
pack/prompts/open-questions-gate.md:0
pack/prompts/orchestrator.md:0
pack/prompts/planner.md:0
pack/prompts/reviewer.md:0
pack/prompts/triager.md:0
```

No pack prompt carries a render slot, so each deployed prompt is a verbatim copy and a byte diff is the right check.

## Acceptance criteria: could the implementer tell it was done?

Yes. Every requirement has a check, and I could name the check for each:

- `:26` (scope), `:27` (actor at point of use), `:28` (main clause not the trailing rationale clause), `:29` (same actor clause, keep the guidance's tail), `:32` (short clause not a restatement), `:34` (no enumeration of the four direct-on-main edits): prose requirements, each checkable by reading the four edited paragraphs against the named `pack/AGENTS.md` lines. `:34`'s prohibition is additionally mechanically checkable, since the four-item enumeration is a fixed string in `ISOLATION_POLICY_FRAGMENT` and can be grepped for in `pack/prompts/orchestrator.md`.
- `:30` (deployed prompt regenerated): the stated `diff` command, run and shown above.
- `:39`, `:40` (the two guidance copies regenerated): enforced by `cargo test` via the whole-file drift guard, exactly as `:42` says. I confirmed the suite is green on this worktree (`cargo test`, exit 0), so a failure after the step would be attributable to the step.
- `:44` (do not run `just scaffold-self`): a repo-wide `nix fmt` would show as a tree-wide diff, so the prohibition is visible in review.

I found no requirement without a corresponding check.

## Step 92 (`prompt-drift-guard`): executable

Verified against the real code rather than the sidecar's summary.

- The guard covers exactly two files. `src/agents_md_drift.rs:45` embeds `../AGENTS.md`; `:49` embeds `../.agents/AGENTS.reference.md`. Nothing else is embedded there.
- The `include_str!` sweep is accurate. `grep -rn "include_str!" src/` yields 15 macro invocations across 9 files, and the sidecar names those 9 (`recommendation_rule.rs`, `agents_md_drift.rs`, `workflow_spec.rs`, `isolation_policy.rs`, `findings_naming.rs`, `pack.rs`, `plan/source.rs`, `plan/render.rs`, `metrics.rs`). The embedded paths are `../pack/principles.toml`, `../pack/workflow.toml`, `../pack/instrument.md`, `testdata/render-fixture.md`, `testdata/skeleton.plan.toml`, and repeated `../AGENTS.md` / `../.agents/AGENTS.reference.md`. None is under `.agents/prompts/`. (`src/main.rs:2025` mentions `include_str!` in a comment only; it is not a tenth embedding site.)
- The gap statement holds. `grep -rn "\.agents/prompts" src/ tests/` returns only manifest destination strings (`src/manifest.rs:604-610`, the module entry at `:658`, the ownership assertion at `:685`), a module-list entry in `src/main.rs:2094`, and path-shape construction in `src/next.rs`. No test compares a committed prompt copy against a render. So an unregenerated `pack/prompts/*` edit does ship silently today, as claimed.
- The reuse targets exist and are reachable. `normalize_wrapping` (`src/agents_md_drift.rs:232`) and `assert_no_unprotected_construct` (`:99`) are private helpers inside that file's `#[cfg(test)] mod tests`, so a new test added to the same module (which is what "extend `src/agents_md_drift.rs`" means) can call both without changing visibility.
- The derived-from-manifest form is buildable from what exists. `self_scaffold_asset` (`:58`) already renders the whole set via `build_assets(&source, &selected, pack::Detail::Summary, &HashMap::new(), true, &[])` and then finds one asset by `dest`; keeping every asset whose `dest` starts with `.agents/prompts/` is a filter over the same collection, and each asset carries `dest` and `contents` (used at `:67` and `:69`). Reading the committed side from `CARGO_MANIFEST_DIR` is the standard form and needs no new dependency.
- The `checks-reviewer.md` caveat is real and correctly stated. `pack/prompts/` holds 8 files, `.agents/prompts/` holds 7, and the missing one is `checks-reviewer.md`. My scaffold run above refreshed exactly 7 prompt assets and did not emit `.agents/prompts/checks-reviewer.md`, so a guard that expected it would indeed fail on a correct tree.
- The precondition warning is actionable and, checked early as the sidecar advises, comes back clean. I ran the `assert_no_unprotected_construct` predicate (a line equals its `split_whitespace().join(" ")` canonical form, fenced blocks exempt) over all 8 pack prompts and all 7 deployed prompts: zero non-canonical lines. A separate `grep -nP '[^\x00-\x7F]'` over the same 15 files returns nothing, so there is no NBSP or other non-space whitespace that the ASCII-only scan would miss. The implementer will not hit the impasse the sidecar prepares for, but the preparation is correct to have.
- The two comparison steps it points at exist as steps: `decision-receipt` (`docs/plans/agent-scaffold.plan.toml:549`) and `waiver-model` (`:559`), both with sidecars. The CHANGELOG-convention instruction is followable.

## `Q-69`: an explorer could start from it

Judged only on whether the pass is startable, as instructed. It is.

Both premises are stated as questions with a named crux and named consequences, not as leanings.

- Premise 1 (is the generated enumeration exhaustive or illustrative?) states both readings, names what turns on each (whether a direction must amend `src/isolation_policy.rs`), and lists three consequences including one that runs against a previously deployed argument. An explorer knows what it must rule on and what changes if it rules either way.
- Premise 2 (must the placeholder live in the plan at all?) states the boundary question, supplies the evidence against relocating (the typed `Exploring` variant and `pack/AGENTS.md:65`'s points-at-the-exploration requirement), and explicitly labels that evidence as evidence rather than a ruling.

The evidence is sound. Every citation reproduces in this worktree:

- `pack/AGENTS.md:71` main clause and trailing rationale clause: both quoted verbatim and correctly split at the colon.
- The three conflicting call sites: `pack/AGENTS.md:45` ("the orchestrator records the question as an Open-Questions item with status `exploring`"), `pack/prompts/orchestrator.md:31` branch 3 ("record it as an `exploring` Open-Questions item"), `pack/user-prompts/explore.md:13` ("record this as an `exploring` open question"), restated at `:3` and under "Act as the orchestrator" at `:7`. All four verbatim.
- The fragment: `src/isolation_policy.rs:33` closes with "The only edits made directly on main are ..." and names four items; the const contains zero case-insensitive occurrences of "question" (`sed -n '33p' src/isolation_policy.rs | grep -oic question` -> `0`), as claimed. The `{{isolation_policy}}` slot is at `pack/AGENTS.md:91`.
- The schema evidence: `Exploring` is a typed `QuestionStatus` variant (`src/plan/source.rs:337`, `:352`, `:363`), and `pack/AGENTS.md:65` carries the points-at-the-exploration-by-path requirement.
- The single observed instance: `b6ba317` is "docs: capture Q-68 exploring backlog item for structured-first ledger" and touches `docs/plans/agent-scaffold.plan.toml` and the rendered view; `grep -c "Q-68" docs/metrics/workflow.jsonl` -> `0`; and the ledger carries "NEW BACKLOG (captured 2026-07-26 by a planner, per Q-67)" (`grep -c` -> `1`). The hedging around it ("relies on that record as it stands", "not as a finding of a misstep against any role") is proportionate to what a single contemporaneous ledger line supports.
- The fourth-asset citations: `pack/pack.toml:166-167` are the `source`/`dest` lines for `explore.md`, and `src/manifest.rs:615` is `.agents/user-prompts/explore.md`. Both exact.

The exploration path `docs/plans/exploring-item-actor-boundary.explorations/` is topic-named rather than task-named. I checked whether that departs from practice: of the 15 existing `docs/plans/*.explorations/` directories, 14 are topic-named and one is task-named, so the item follows the dominant convention, no tooling in `src/` reads the path, and an explorer given that path can act on it without ambiguity. Not a finding.

## Candidate examined and not raised

While simulating passage 1 I derived, independently and before opening any prior findings file, a tension between `decision-folder-currency.md:28` ("NOT its trailing rationale clause after the colon") and `:34` ("copy the pointing, not the list", whose preceding parenthetical quotes that same trailing clause in full). I am not raising it, for three reasons, and I record it so the triager can see it was considered rather than missed.

1. It is settled. The round-1 triager dismissed `T-5` on the reading that "copy the pointing, not the list" names the FORM (point rather than enumerate), not an instruction to transcribe a string, and pointed at the next sentence, which is permissive ("The prompt MAY reference the rule in `AGENTS.md` ...") and prohibitive about the wrong form. The round-4 triager considered the interaction with `:28` explicitly, judged the residual second-order, and it is the change that produced `:28`'s current "NOT its trailing rationale clause" wording.
2. I have no new evidence that either verdict was wrong. The text at `:34` is unchanged since those verdicts; only `:28` moved, and it moved in the direction that closes the risk.
3. It did not actually mislead me in execution. Working the passage from `:19` (which spells out the target prose and closes "match the guidance's existing clause, do not invent a different rule") and `:28`, I produced the main clause with no pointing clause and no enumeration. The instruction that governs the edit is unambiguous even if one adjacent phrase, read alone, is loose.

## What I verified clean

Documentation and citation currency, in this worktree at `72ca2d7`:

- All four target passages reproduce verbatim at the cited lines: `pack/prompts/orchestrator.md:27`, `:31`, `:33`, and the `pack/AGENTS.md:63` paragraph.
- All supporting `pack/AGENTS.md` citations reproduce at the cited lines: `:39`, `:41`, `:43`, `:45`, `:63`, `:71`, `:79`, `:91` (the `{{isolation_policy}}` slot), `:108`.
- `pack/LEDGER.template.md:3` carries "Durable decisions do not live here; they fold into the plan.", so the out-of-scope exclusion at `:49` describes a real passage.
- `justfile:46-48` is the `scaffold-self` recipe as described; `src/agents_md_drift.rs` module docs and `the_committed_scaffold_matches_a_fresh_render` say what `:44` attributes to them.
- The step-89 triager's residual note exists in the ledger with the quoted text and names only the two actor-less passages, matching the sidecar's class split.

No third consumer goes stale. `grep -rn "update the plan's Open Questions queue and push"` and `grep -rn "only durable decisions, the ones that change the plan"` across the repo return hits only in the pack sources, their deployed copies, and plan/review documents. Nothing in `src/` (including the driver's generated output in `src/next.rs`) reproduces either sentence, so `:42`'s "No other deployed file is affected" holds.

Tooling green on this worktree, so the checks the plan leans on are executable:

- `cargo test`: exit 0, all suites pass.
- `cargo run -- render docs/plans/agent-scaffold.plan.toml --check`: "docs/plans/agent-scaffold.plan.toml: up to date".
- `cargo run -- validate`: "docs/metrics/workflow.jsonl: 213 records, valid".
- The rendered `docs/plans/agent-scaffold.md` matches the sidecars (step 90 at `:1192`, step 92 at `:1253`, `Q-69` at `:194`, Roadmap rows at `:303-304`).

Structural facts checked and found in order:

- Step 90 is `order = 90`, `status = "next"`, with `[step.provenance] decisions = ["Q-67"]`; step 92 is `order = 92`, `status = "not-started"`. The gap at 91 is the known intentional one and `validate` does not object.
- The empty `docs/plans/agent-scaffold.questions/Q-69.md` sidecar is the project's convention, not an omission: all 24 question sidecars are 0 bytes and every question body lives in the TOML `ask` field.
- `.agents/prompts/` holds 7 files, `pack/prompts/` holds 8, and the deployed `orchestrator.md` is currently byte-identical to its pack source.

Deliberately not covered, per the brief: whether the specific round-4 fixes closed, and fix-induced residue from them, which the parallel reviewer owns.
