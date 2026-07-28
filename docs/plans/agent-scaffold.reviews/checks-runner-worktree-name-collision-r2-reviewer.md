# Plan review round 2: `checks-runner-worktree-name-collision` (deferred step, order 93)

Lens: fix verification and fix-induced residue. Artifact: `git diff a4f4c95..6d94cfc` (whole fold), primary target `git diff HEAD~1..HEAD` (the round-1 fix commit `6d94cfc`). Reviewed in worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev2-testiso` at `6d94cfc`, detached, independent of the planner and the orchestrator. I did not see round 1; every round-1 fact below was re-derived from the code, not taken from the round-1 files.

The human's decision to DEFER is out of scope and not re-litigated. No finding here amounts to "the fix is not implemented yet".

## Verdict summary

| Round-1 finding | Verdict | Evidence |
| --- | --- | --- |
| TI-1 part 1 (fixture pid facts) | CLOSED | `src/checks.rs:1438-1442`, `:1462`, `:1491`, `:1492` re-read; corrected text matches the code exactly |
| TI-1 part 2 (dead-pid structural constraint recorded) | CLOSED | Claim re-derived from `src/checks.rs:1440` and `:404`; true as stated |
| TI-1 part 3 (candidate (a) trade-off, load-bearing) | CLOSED | "leans on the pid exactly as today" is gone; replacement is accurate |
| TI-2 (demonstration red against a correct (d)) | CLOSED | Restatement walked against (a), (b), (c), (d) |
| TI-3 (demonstration pins the generator, not the call sites) | CLOSED | Linkage bullet added and it is a command, per `Q-66` proportionality |
| TI-4 (grep claim did not reproduce) | NOT CLOSED | See `T2-1`: the rescoped claim does not reproduce either |
| TR-1 (both-fail presentation) | CLOSED | `src/checks.rs:795-800` and `git_ok` verified; 200-race split reported accurately |

New findings: `T2-1` (`low`), `T2-2` (`low`). No `medium`, `high`, or `critical` finding. Severities are absolute.

## Round-1 fix verification

### TI-1 part 1: CLOSED

I opened all four sites rather than trusting either round-1 file:

- `src/checks.rs:1438-1442`: `fn dead_pid() -> u32 { let pid = u32::MAX; assert!(!pid_is_alive(pid), "expected pid {pid} to be dead"); pid }`.
- `src/checks.rs:1462`: `std::env::temp_dir().join(format!("{RUNNER_PREFIX}{}-{}", dead_pid(), nanos()))`.
- `src/checks.rs:1491`: `std::env::temp_dir().join(format!("{RUNNER_PREFIX}{}-{}", std::process::id(), nanos()))`.
- `src/checks.rs:1492`: `let dead = std::env::temp_dir().join(format!("{RUNNER_PREFIX}{}-{}", dead_pid(), nanos()));`.

Sidecar line 51 now says exactly that: one live-pid site (`:1491`), two constant-pid sites (`:1462`, `:1492`) using `dead_pid()`, the compile-time constant `u32::MAX` at `:1438-1442`. Every clause of the corrected sentence checks out, including the consequence ("the constant-pid pair ... carries no CROSS-PROCESS discriminator at all"): `u32::MAX` is identical in every process, so two concurrent `cargo test` processes share the whole name template at those two sites. The false clause "two of them with the live pid" is gone from both the sidecar and the rendered view.

### TI-1 part 2: CLOSED

The new paragraph at sidecar line 53 is true as stated, verified claim by claim:

- "`dead_pid()` asserts `!pid_is_alive(pid)`": `src/checks.rs:1440`, verbatim.
- "the tests exist to plant an orphan owned by a dead owner so the prune reclaims it": `src/checks.rs:1444-1474` (`a_startup_prune_reclaims_an_orphaned_runner_worktree`) and `:1476-1506` (`a_startup_prune_skips_a_live_owner_and_reclaims_a_dead_one`). Both assert reclamation of the dead-owner path, so a live pid there would break the test's premise, not just its style.
- "`owning_pid` (`src/checks.rs:400-405`) reads the FIRST `-`-separated segment": `src/checks.rs:404` is `dir_name.strip_prefix(RUNNER_PREFIX)?.split('-').next()?.parse().ok()`. Confirmed.
- "even the live pid as a later component (`{RUNNER_PREFIX}{dead_pid}-{live_pid}-{seq}` still parses as a dead owner)": traced by hand. `strip_prefix` leaves `4294967295-<live>-<seq>`, `split('-').next()` yields `4294967295`, `parse::<u32>()` yields `u32::MAX`, and `pid_is_alive(u32::MAX)` is false on this Linux target (`src/checks.rs:393-398`, `/proc/4294967295` does not exist and `/proc` does). So the example is correct, not just plausible.

### TI-1 part 3 (load-bearing): CLOSED

`git diff HEAD~1..HEAD` removes the clause "so it leans on the pid exactly as today for that half" from candidate (a). Grep confirms it survives nowhere: `grep -rn "leans on the pid" docs/` returns nothing outside the round-1 review files.

The replacement (sidecar line 67) is accurate. It splits the cross-process case in two, holds the old argument only where the pid is live ("two live processes hold different pids", which is what makes `run()` against `run()` safe across processes), and states plainly that the argument "does NOT hold at the two constant-pid fixtures (`src/checks.rs:1462` and `:1492`)". Its rate estimate ("of order 1e-6 per pair") and its disclosure of the null result ("found 0 in 40 pairs, which is the expected result whether or not the channel exists and so is not evidence either way") both match the round-1 record and are honest in the direction that costs the author something. The trap the triage named, an implementer picking (a) alone on a false premise, is closed: the text names the choice explicitly and requires the commit to "say ... which channels the chosen fix closes and which it leaves".

### TI-2 and TI-3: CLOSED

The restated bullet (sidecar line 82) defines the unit under test as "the call that yields the final path, the one `run()` and the fixtures actually use", asserting "N * M distinct paths and no error". I walked it against each candidate rather than accepting the sidecar's own summary:

- (a) atomic counter `{pid}-{nanos}-{seq}`: every call takes a distinct `seq`, so N * M distinct paths, no error. GREEN.
- (b) random suffix: distinct with probability 1 - O(k^2 / 2^b); at k = 8000 over a 64-bit suffix that is ~1.7e-12. GREEN in any realistic run.
- (c) `std::thread::current().id()`: cross-thread the id discriminates; same-thread the sidecar's own measurement carries it (`consecutive: n=100000 zero_deltas=0`, so consecutive same-thread `nanos()` reads never repeat). GREEN.
- (d) `create_dir` reservation plus retry: the second reserver gets `AlreadyExists` and retries with a fresh name, so the yielded paths are distinct and no error escapes. GREEN. This is the case the old wording got wrong, and it is now right.
- Today's code: the path is `{pid}-{nanos}` with a shared pid, and the sidecar's own N = 8, M = 1000 measurement is 2793 to 3354 duplicates in 8000. RED.

So the restatement is genuinely fix-independent in both directions, which is what TI-2 asked for. I checked the number it quotes against the round-1 triage probe (`threads=8 per=1000 total=8000 excess=2793 / 3354 / 3301`): the stated range 2793 to 3354 is that probe's min and max, correctly reported.

TI-3's linkage requirement is added as its own bullet (sidecar line 83): the test must be shown to cover the real construction sites, settled by a command ("show that exactly one `format!` in `src/checks.rs` builds a `RUNNER_PREFIX` name, reconciled against the sites the scope section enumerates") rather than by an extra test, which is the proportional form `Q-66` prescribes. Today that command returns four sites (`src/checks.rs:792`, `:1462`, `:1491`, `:1492`), exactly the set the scope section enumerates, so the reconciliation the bullet asks for is well defined. The AGENTS.md principle it cites exists with that wording at `AGENTS.md:124` ("11. Tests must actually exercise the code they claim to").

### TR-1: CLOSED

The new second-presentation paragraph (sidecar line 28) checks out:

- "`in run()` that becomes `RunError::WorktreeSetup` (`src/checks.rs:795-800`)": lines 795-800 are `if !added.status.success() { return Err(RunError::WorktreeSetup(format!("\`git worktree add\` failed: {}", ...))); }`. Exact.
- "at the fixtures it is a `git_ok` assertion failure": `git_ok` is `src/checks.rs:874-884` and does `assert!(status.status.success(), "git {args:?} failed: {}", ...)`; the fixtures reach it at `:1463`, `:1493`, `:1494`. Correct.
- The 200-race split "25 ... both, 160 ... exactly one, and 15 ... neither" and "the path registered in both repositories in every one of the 25" match the round-1 reviewer's reported `SUMMARY both=25 exactly_one=160 neither=15 both_registered_same_path=25` exactly, and sum to 200.
- The paragraph is consistent with the mechanism paragraph above it: line 26 establishes that git refuses a non-empty directory, which is precisely why the second add fails once the first has populated the path.

## Fix-induced residue sweep

I read the full current sidecar, not only the diff, and checked the six sections named in the brief against each other: mechanism (lines 5-28), evidence (30-38), severity (40-44), scope (46-55), done conditions (57-63), candidates (65-72), demonstration (74-86), documentation impact (92-94). Two residues found, both `low`. Details below.

The done-conditions block is byte-identical across the fix, verified independently rather than taken from the planner's report:

```
$ git show HEAD~1:docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md \
    | sed -n '/^WHAT "DONE" LOOKS LIKE/,/^CANDIDATE FIXES/p' > before.txt
$ sed -n '/^WHAT "DONE" LOOKS LIKE/,/^CANDIDATE FIXES/p' \
    docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md > after.txt
$ diff before.txt after.txt && echo "DONE BLOCK IDENTICAL"
DONE BLOCK IDENTICAL
```

## `T2-1` (`low`): the rescoped `docs/` grep claim still does not reproduce

The TI-4 fix replaced "A grep for `agent-scaffold-checks-run` finds no hit in `README.md`, `CHANGELOG.md`, `docs/`, or `pack/`" with a rescoped claim at sidecar line 94:

> `README.md`, `CHANGELOG.md`, and `pack/` carry no occurrence of `agent-scaffold-checks-run`, and the only `docs/` hits are this plan's own record of the defect (this sidecar, the rendered `docs/plans/agent-scaffold.md`, and the transient step-92 findings file), so a grep run later will find those and nothing else.

Run at the commit that wrote it:

```
$ git -C <worktree> grep -c "agent-scaffold-checks-run" HEAD -- README.md CHANGELOG.md docs/ pack/
HEAD:docs/plans/agent-scaffold.md:3
HEAD:docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-reviewer.md:9
HEAD:docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-triage.md:4
HEAD:docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md:1
HEAD:docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md:3
```

Five files, not the three enumerated. The two extra hits are this step's own round-1 reviewer and triage files, committed at `a4f4c95`, which is the fold's own starting commit, so they were already in the tree when the corrected sentence was written. The `README.md` / `CHANGELOG.md` / `pack/` half of the claim is correct (no hits, confirmed by the same command returning nothing for those paths).

The parenthetical is a closed enumeration and the sentence's tail is an explicit universal ("so a grep run later will find those and nothing else"), so the charitable reading in which "this plan's own record of the defect" silently covers the review files is defeated by the sentence's own wording. A reader who runs the command in the sentence gets a superset on the first attempt, which is the same defect TI-4 named, in the same paragraph, after the fix.

Severity `low`, matching the triage's own rating of TI-4 on identical grounds. The substantive conclusion is unaffected and I confirm it independently: outside the plan's own documents the name format appears only in `src/`, so no separate documentation step is owed. The cost is the one the triage already articulated, that the record's cheapest checkable claim fails to check.

A wording that reproduces would drop the enumeration and state the invariant instead, for instance that `README.md`, `CHANGELOG.md` and `pack/` are clean and that every `docs/` hit is inside this plan's own record of the defect (the sidecar, its rendered view, and the review-round files), so nothing outside `src/` goes stale when the name format changes.

## `T2-2` (`low`): the done-conditions checklist requires three doc comments corrected; the documentation-impact section now requires four

The fix pass edited the documentation-impact paragraph (sidecar line 94) to add a fourth comment and to state that all four are corrected:

> The three doc comments at `src/checks.rs:72-77`, `:400-402`, and `:845-847` spell the name format literally, so they go stale with any change to it, and a fourth, the comment on the naming site itself at `src/checks.rs:789-790` ... asserts the uniqueness that is currently false. All four are corrected by the same implementer ...

The done-conditions block, which is the step's acceptance bar and is byte-identical across the fix (evidence above), still reads at sidecar line 62:

> - The three doc comments that spell the name format literally are corrected in the same change: `src/checks.rs:72-77` (`RUNNER_PREFIX`), `src/checks.rs:400-402` (`owning_pid`), `src/checks.rs:845-847` (`nanos`, the false premise).

So one document now carries two lists of what must be corrected, of different lengths, and the shorter one is the acceptance checklist. An implementer who works the "WHAT DONE LOOKS LIKE" block, which is what that block exists for, corrects three and can close the step with `src/checks.rs:789-790` untouched. That comment is the one the same fix pass identified as asserting the uniqueness that is currently false, verified verbatim at `src/checks.rs:789-790`:

```
789	// A unique temp path OUTSIDE the repository; git worktree add creates it. The
790	// `RUNNER_PREFIX` (with the embedded pid) is what the startup prune recognises.
```

There is a further reason to think the checklist is where the addition was meant to land. The round-1 triage wrote "the sidecar's documentation-impact list (line 58) names three doc comments ... Worth adding to the list if the sidecar is being edited". Line 58 of the PRE-FIX sidecar is not the documentation-impact paragraph (that was pre-fix line 89, cited as such under TI-4); it is the done-conditions item quoted above:

```
$ git show HEAD~1:docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md | sed -n '58p'
- The three doc comments that spell the name format literally are corrected in the same change: `src/checks.rs:72-77` (`RUNNER_PREFIX`), ...
```

So the item the triage pointed at is the one that did not get the addition.

Severity `low`, not higher, for two mitigations I checked rather than assumed. First, done-condition line 59 independently requires "the argument for why is written in the code comment, since the current comment is precisely where the wrong argument was written down", and the comment on the naming site is one natural reading of that, which is the ground on which the triage declined to raise `:789-790` as a finding of its own. Second, an implementer changing the name format edits `:789-792` anyway because the comment sits directly on the line being changed. The residue is the inconsistency between two enumerations in one document, not a certainty that the comment rots.

## Checked and deliberately NOT raised

These were examined and judged not to be findings. I list them so the triager can see the boundary I drew rather than guess at it.

- **"green against any of (a), (b), (c), (d) implemented correctly" versus done-condition "unique per call BY CONSTRUCTION".** Candidate (b) is probabilistic and (c) is described in the same document as falling back on the clock for two calls on one thread, so neither is unique by construction, yet the demonstration section says the test is green against both. Not raised: the wording is the round-1 triage's own prescription adopted verbatim, the claim is true in substance ((b) collides at ~1e-12 over 8000 draws; (c) is carried same-thread by the measured `zero_deltas=0`), the candidate list already argues (b) and (c) down explicitly, and the bullet is about the test's fix-independence rather than about candidate selection. Re-raising it would re-litigate a settled instruction without new evidence.
- **"An independent probe ... reproduced every one of those numbers and measured the two-thread rate higher still."** Strictly, the two-thread figure 8679/100000 was exceeded rather than reproduced (10933, 14680, 16386 over three runs), and the median was 21 ns on one of the probe's two consecutive-read runs. Not raised: the qualifier is in the same sentence, the range 10933 to 16386 is exactly the round-1 probe's min and max, and the derived conclusion is stated as a floor. This is stylistic looseness, not an inaccurate record.
- **The step title names only the take-over presentation.** `docs/plans/agent-scaffold.plan.toml` gives the title as "... take over each other's isolated worktree; observed as a non-deterministic `cargo test` failure", while the TR-1 correction establishes the loud failure as the more common shape. Not raised: the title was equally partial before the fix, so nothing was made stale; the take-over clause is true of the one recorded sighting; and the title's closing clause ("a non-deterministic `cargo test` failure") covers both shapes.
- **Channel D versus the five done-conditions.** The round-1 triage explicitly ruled that the corrected facts do not require the done-conditions to be rewritten, and located the fix in candidate (a)'s trade-off instead. I have no new evidence that verdict was wrong, so I do not re-raise it.

## Citation check

Every citation in the current sidecar was resolved against the tree at `6d94cfc`, including the ones the fix did not touch. All resolve.

- `src/checks.rs`: `:72-77` (`RUNNER_PREFIX` doc comment), `:78` (the constant), `:329-342` (`impl Drop for WorktreeGuard`), `:388-392` (`pid_is_alive` doc, the dependency-discipline sentence), `:400-402` and `:400-405` (`owning_pid` doc and body), `:407-461` (`prune_orphan_worktrees` doc plus body), `:425-428` (the pid-reuse benign-edge sentence), `:789-790` (the naming-site comment), `:791-792` (the naming site), `:795-800` (`RunError::WorktreeSetup`), `:845-847` (`nanos` doc, the false premise), `:848-852` (`nanos` body), `:862-871` (`scratch`), `:1438-1442` (`dead_pid`), `:1462`, `:1491`, `:1492`.
- Other files: `src/main.rs:1726-1731`, `src/manifest.rs:552-558`, `src/plan/render.rs:638`, `tests/audit_command.rs:20`, `tests/scaffold_precommit_hook.rs:14`, `tests/checks_staged_hook_env.rs:50`, `tests/validate_workflow_toml_source_needs_no_plan.rs:58` and `:90`, `tests/validate_toml_primary_skips_markdown_plan.rs:74`. All are the scratch or temp-path construction the sidecar describes. I also confirmed the substantive claim about them: the first seven discriminate by a per-test literal name, and the last three do use a clock reading (`SystemTime::now() ... .as_nanos()` at `:61`, `:93`, `:77` respectively) but each carries a distinct literal prefix (`agent-scaffold-validate-toml-only-`, `agent-scaffold-validate-workflow-no-source-`, `agent-scaffold-validate-projection-`), so the "cannot collide today" claim holds.
- Cross-references: `Q-66` exists, `status = "decided"`, `folded_into = "reviewer-reproducible-evidence"`, which is `order = 88`, so "(`Q-66`, step 88)" is right. Step 85 is `drift-guard-test-hook-hygiene` and `src/agents_md_drift.rs` exists, so the relation section is right. Plan principles cited by name match the plan's own numbering: 1 "Prefer the cleaner long-term architecture over the smallest diff", 5 "Make illegal states unrepresentable", 6 "Ground decisions in evidence". The AGENTS.md principle quoted in the demonstration section is `AGENTS.md:124`.
- Counts: `grep -cE "\brun\(" src/checks.rs` returns 23, which is the definition at `:734` plus the 22 call sites the scope section claims.
- Arithmetic in the demonstration-problem paragraph: (5/6)^6 = 0.3349, (5/6)^16 = 0.0541, (5/6)^17 = 0.0451, so "0.33" and "17 consecutive clean runs" are both right.

## Mechanical checks

```
$ cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 217 records, valid
docs/plans/agent-scaffold.plan.toml: 92 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold

$ cargo run --quiet -- render docs/plans/agent-scaffold.plan.toml --check --strict
docs/plans/agent-scaffold.plan.toml: up to date
EXIT=0
```

92 steps, 69 questions, 217 records, as expected. The rendered `docs/plans/agent-scaffold.md` matches the sidecar, so no correction landed in one view and not the other.

No `[[question]]` and no `type:"decision"` receipt were added. `git diff a4f4c95..6d94cfc -- docs/plans/agent-scaffold.plan.toml` adds a single `[[step]]` table (`slug`, `title`, `status = "deferred"`, `order = 93`, empty `blocked_by` / `folds` / `increment` / `waiver`) and nothing else; `git diff a4f4c95..6d94cfc -- docs/metrics/workflow.jsonl` is empty, and the only `workflow.jsonl` record naming this slug is a `type:"round"` record, not a decision receipt. The fix commit `6d94cfc` touches only the sidecar and its rendered view.

The sidecar is ASCII-clean: `grep -nP "[^\x00-\x7F]"` returns nothing, and the only `--` occurrences are the `git worktree add --detach` flag in two prose citations, not dashes.

## Tree state

`git status --porcelain` in this worktree reports one untracked file, this findings file. No plan file, sidecar, or source file was edited; nothing was committed; no formatter was run. No probe program was needed: every claim above is settled by a citation or a re-runnable command, which is the proportional evidence form `Q-66` prescribes for a documentation claim. The two scratch files used for the done-block diff were written under the session scratchpad, outside the repository.
