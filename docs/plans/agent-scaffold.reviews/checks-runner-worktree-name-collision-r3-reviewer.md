# Plan review round 3, reviewer: `checks-runner-worktree-name-collision` (deferred step, order 93)

Lens: fix verification and fix-induced residue.

Artifact: primary target `git diff HEAD~1..HEAD` (the round-2 fix commit `4067c50`), whole fold `git diff 0ad43f0..4067c50`.
Reviewed in worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev3-testiso`, detached at `4067c50`, independent of the planner that wrote the fix and of the orchestrator driving the loop.
Rounds 1 and 2 were not seen; everything below is judged from the artifacts and from the two triage files.

The deferral is out of scope and is not re-litigated. No finding below rests on the fix not being implemented. I did not re-raise the settled (b)/(c) "implemented correctly" wording, the channel-D question, or the step title.

**Standard applied.** Same as both prior triages: this is a durable RECORD and a future BRIEF, so I weight accuracy of the stated facts above transient prose. Its reader is months out and has none of this loop's reasoning.

## Summary

| Item | Verdict |
| --- | --- |
| `T2-1` (grep claim) | CLOSED. My own whole-tree grep reproduces the sentence exactly. |
| `T2-2` (three vs four doc comments) | CLOSED on the substance. `:789-790` is reached by the acceptance bar. Two clause-level residues below. |
| `TR2-1` ("proportional minimum") | CLOSED. |
| Past-the-verdict edit (paragraph opening clause) | JUSTIFIED, and it introduced nothing false. Verified against source. |
| Citations | ALL RESOLVE. 30+ checked line by line, zero misnumbered. |
| Mechanics | `validate --workflow` and `render --check --strict` both exit 0. `plan.toml` untouched this round. No `[[question]]`, no decision receipt. |

Findings raised: **2, both `low`**. No `medium`, no `high`, no `critical` (stated explicitly rather than omitted). Both are one-clause repairs in the same sentence pair, and neither changes what an implementer must do.

## `T2-1`: CLOSED. The grep reproduces.

The round-2 fix replaced the closed enumeration plus "nothing else" universal with an invariant. Current text, sidecar `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md:94` (rendered `docs/plans/agent-scaffold.md:1373`):

> Outside the plan's own documents the name format is written down only in `src/`: `README.md`, `CHANGELOG.md`, and `pack/` carry no occurrence of `agent-scaffold-checks-run`, and every `docs/` occurrence is inside this plan's own material recording this defect, so nothing outside `src/` goes stale when the name format changes.

I ran the grep myself over the WHOLE tree at `4067c50` rather than reasoning about it. Verbatim output:

```
$ git grep -l "agent-scaffold-checks-run" HEAD
HEAD:docs/plans/agent-scaffold.md
HEAD:docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-r2-reviewer.md
HEAD:docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-r2-triage.md
HEAD:docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-reviewer.md
HEAD:docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-triage.md
HEAD:docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md
HEAD:docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md
HEAD:src/checks.rs

$ git grep -c "agent-scaffold-checks-run" HEAD -- README.md CHANGELOG.md pack/
(no output; exit 1)

$ grep -rn "agent-scaffold-checks-run" README.md CHANGELOG.md pack/ .agents/ AGENTS.md
(no output; exit 1)

$ grep -rl "agent-scaffold-checks-run" . --exclude-dir=.git --exclude-dir=target --exclude-dir=.claude
docs/plans/agent-scaffold.md
docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-r2-reviewer.md
docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-r2-triage.md
docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-reviewer.md
docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-triage.md
docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md
docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md
src/checks.rs
```

Clause by clause against that output:

1. "the name format is written down only in `src/`" outside the plan's own documents: the only non-`docs/` hit is `src/checks.rs` (`:73`, `:78`, `:401`). TRUE. I also checked the alternative spellings, `{pid}-{nanos}` and `RUNNER_PREFIX`, and outside `src/` they occur only in `docs/plans/agent-scaffold.md`, the sidecar, `docs/plans/agent-scaffold.plan.toml:1264` (the step title) and this plan's review files, all of which are the plan's own material. TRUE.
2. "`README.md`, `CHANGELOG.md`, and `pack/` carry no occurrence": exit 1 on both the tracked-content grep and the working-tree grep. TRUE. `.agents/` and `AGENTS.md`, which the same sentence asserts are untouched, are also clean.
3. "every `docs/` occurrence is inside this plan's own material recording this defect": all seven `docs/` hits are under `docs/plans/` and belong to the `agent-scaffold` plan. Five are this step's own sidecar, its rendered view, and its round-1/round-2 findings files. The sixth and seventh, the `prompt-drift-guard-r2-triage.md` hit, is the ORIGINAL sighting of this defect: `docs/plans/agent-scaffold.reviews/prompt-drift-guard-r2-triage.md:198` reads "A NEW ONE: A FLAKY TEST IN `src/checks.rs` ... Both failures cited the SAME runner worktree path, `agent-scaffold-checks-run-416707-1785235883764925866`", which is the text the sidecar quotes at `:34`. So it is this plan's material AND it is recording this defect. TRUE.

The sentence is true of the tree as it stands. It is also durable in the way the round-2 verdict required: after the orchestrator commit-deletes the five review files, the remaining hits are the sidecar and its rendered view, both still "this plan's own material recording this defect", so the invariant survives the deletion that would have falsified any enumeration. The verdict's instruction to state an invariant with no count, no file list and no "nothing else" was followed exactly; the sentence carries none of the three.

## `T2-2`: CLOSED on the substance; two clause-level residues raised below.

The two lists now agree, and they agree by reference rather than by a restated count, which is the form the verdict preferred.

Done conditions, sidecar `:62` (rendered `:1341`): "Every doc comment the documentation-impact section below names is corrected in the same change."
Documentation impact, sidecar `:94` (rendered `:1373`): "The change must correct four comments in `src/checks.rs`: `:72-77` (`RUNNER_PREFIX`) and `:400-402` (`owning_pid`), which spell the name format; `:845-847` (`nanos`), which states the false per-process-uniqueness premise; and `:789-790`, the comment on the naming site itself, which asserts the same false uniqueness ..."

There is now exactly ONE enumeration of the required work in the document, and no second count to drift against it. The omitted comment is covered: `src/checks.rs:789-790` is named explicitly, with its ground, in the only list. The section's own sentence is imperative ("The change must correct four comments"), so the requirement is binding wherever the reader enters. The round-2 triage's second, independent route to `:789-790` also still holds: done-condition `:59` requires "the argument for why is written in the code comment", and `:789-790` is the comment on the naming site at `:791-792`.

**The past-the-verdict edit: JUSTIFIED, and it introduced nothing false.** The verdict permitted "have the bullet point at that paragraph instead of restating a list" and said nothing about the paragraph's opening clause. The implementer also rewrote that clause, on the ground that "the three doc comments ... spell the name format literally" was a false category claim. I checked that ground against the source rather than accepting it:

- `src/checks.rs:73`: "/// system temp dir: `agent-scaffold-checks-run-{pid}-{nanos}`. The startup prune". Spells the format literally. TRUE.
- `src/checks.rs:401`: "/// `agent-scaffold-checks-run-{pid}-{nanos}`. Returns `None` when the name does not". Spells the format literally. TRUE.
- `src/checks.rs:847`: "/// the process id in the path already provides per-process uniqueness." Does NOT spell the format. So the old clause's category was indeed false of one of its own three members, and the sidecar conceded as much at `:22` ("states the premise that fails"). The edit fixes a real defect rather than adding decoration.

The replacement re-sorts the four by what each actually says, and each of the four descriptions is accurate against the source (all four ranges verified line by line in the citation check below). The replacement also loses nothing load-bearing: the "goes stale" rationale that the old clause carried is still carried by the paragraph's tail ("so nothing outside `src/` goes stale when the name format changes"). I judge the edit within the spirit of the verdict, well grounded, and clean. It is not an instance of the round-1/round-2 surplus pattern: it corrected a false statement rather than adding new explanatory material.

What it did leave behind is two cross-reference residues, `T3-1` and `T3-2` below. Both are in the sentence pair the edit touched, which is the pattern the round-2 triage predicted; both are one clause.

## `TR2-1`: CLOSED.

Sidecar `:85` (rendered `:1364`) now ends: "the cost is real machinery, and the requirements above are the proportional minimum." It names none and points at the bullets, which is one of the two forms the verdict accepted.

The three requirements are all present, distinct, and consistent:

- `:82` the property-level unit under test (N threads on a `std::sync::Barrier`, N * M distinct paths).
- `:83` the linkage command (exactly one `format!` in `src/checks.rs` builds a `RUNNER_PREFIX` name).
- `:84` red before green ("Show it RED before green ... it is the form `Q-66` names as strongest").

"the requirements above" reaches all three, including `:84`, the one the previous wording dropped. Bullet `:86` ("Report the measured numbers, not the word 'fixed'") sits BELOW `:85` and so is outside the phrase's reach; I considered raising that and decided against it, because `:86` is a duty on the report rather than a piece of demonstration machinery, and "proportional minimum" is contrasted against the optional higher-fidelity machinery in the same sentence. Reading it as excluding `:86` would be manufacturing a finding.

No count survives in that sentence, so there is nothing left to drift.

## Findings

### `T3-1` (`low`): the documentation-impact header still says the impact "is named above", which the `T2-2` fix made false and circular

**Evidence: citation, in the pair the fix touched.**

Sidecar `:92` (rendered `docs/plans/agent-scaffold.md:1371`), unchanged since the step was written:

> DOCUMENTATION IMPACT: in-code only, and it is named above.

Sidecar `:62` (rendered `:1341`), rewritten by this round's fix:

> - Every doc comment the documentation-impact section below names is corrected in the same change.

The two now point at each other. `:62` sends the reader DOWN to the documentation-impact section; that section's own header sends the reader back UP.

The header's claim is false as written. I grepped the sidecar for every one of the four cited comment ranges:

```
$ grep -n "72-77\|400-402\|845-847\|789-790" docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md
22:Its own doc comment (`src/checks.rs:845-847`) states the premise that fails: ...
94:The change must correct four comments in `src/checks.rs`: `:72-77` ... `:400-402` ... `:845-847` ... `:789-790` ...
```

So `:72-77`, `:400-402` and `:789-790` appear NOWHERE above line 92; the only earlier mention of any of the four is `:845-847` at line 22, and there it is cited as evidence of the mechanism, not as documentation the change must correct. The four comments are named BELOW the header, in the paragraph the header introduces.

**It is fix-induced, and I checked that rather than assuming it.** The clause was true when it was written. At `f18905e:87` the same header sat above a done-conditions bullet that did name three of the four (`git show f18905e:...` line 58: "The three doc comments that spell the name format literally are corrected in the same change: `src/checks.rs:72-77` ..., `:400-402` ..., `:845-847` ..."), so "named above" resolved. The round-1 fix (`74152e1`) weakened it by adding the fourth comment only to the paragraph. This round's fix removed the naming from the bullet entirely, which is what makes it false and circular.

**Severity `low`, and I considered and rejected higher.** The paragraph immediately under the header names all four, so a reader going top to bottom is not actually lost, and no implementer follows "named above" in preference to a list two lines further down. Against dismissing it entirely: this is a durable record whose cheap checkable claims are the ones a future reader tests first, which is the exact ground round 1 and round 2 both used on the grep claim, and a header that contradicts the bullet pointing at it is a cheap checkable claim. Cost of the fix is one clause: drop "and it is named above", or change it to "below".

### `T3-2` (`low`): the done-conditions bullet's restrictive noun "doc comment" excludes `src/checks.rs:789-790`, which is not a doc comment

**Evidence: citation plus a one-line command.**

Sidecar `:62`: "Every **doc comment** the documentation-impact section below names is corrected in the same change." The set that bullet requires is {x : x is a doc comment AND the documentation-impact section names x}.

The documentation-impact section deliberately does NOT say "doc comments"; the fix pass changed that noun to the general one: `:94` says "The change must correct four **comments** in `src/checks.rs`". That change was correct, because one of the four is not a doc comment:

```
$ sed -n '72p;400p;845p;789p' src/checks.rs
/// The file-name prefix of a runner's temporary worktree directory under the
/// Parse the owning pid out of a runner worktree directory name of the form
/// Nanoseconds since the epoch, for a unique temp path. Falls back to a fixed
	// A unique temp path OUTSIDE the repository; git worktree add creates it. The
```

`:72-77`, `:400-402` and `:845-847` are `///` doc comments. `:789-790` is a pair of ordinary `//` line comments inside the body of `run()`. "Doc comment" is a term of art in Rust (`///`, `//!`, `/** */`), and this is a Rust codebase that uses the term precisely. Under the strict reading the bullet requires three of the four, and the one it drops is `src/checks.rs:789-790`: the same comment, on the same ground, that `T2-2` was raised to bring inside the acceptance bar.

**Ruling I would put to the triager, stated with its counter-argument.** This is structurally the identical defect to `T2-2`, a restrictive clause in the acceptance-bar bullet that cuts one member out of the list it references, and the round-2 triage explicitly rejected the analogous charitable reading of the previous restrictive clause ("that spell the name format literally") on exactly the ground that the list's own membership defeated it. The same test applies here: the section's membership is four comments, three of which are doc comments. Consistency says raise it.

The counter-argument, which is why this is the weaker of my two and why I would not open an edit pass for it alone: unlike `T2-2`, the acceptance bar taken whole is NOT under-inclusive here. The section's own sentence is imperative and unambiguous ("The change must correct four comments"), the bullet delegates to it rather than carrying a competing count, and done-condition `:59` independently reaches `:789-790`. So a triager could reasonably accept this as a residual rather than a defect. I raise it because it is verifiable, it is exactly the residue class this round was asked to hunt, and the repair is one word: "Every comment the documentation-impact section below names".

**Severity `low`**, on the round-2 triage's own grounds for `T2-2`: an internal inconsistency in one document about what work is required, not a likelihood that the comment rots, since `:789-790` sits two lines above the `format!` at `:791-792` and is in any shared-generator diff regardless.

## Residue sweep: everything else AGREES

I read the full current sidecar, not just the diff, and cross-checked the six sections the fixes could have destabilised. They agree with each other.

**Severity paragraph (`:42`) against scope (`:51`) against candidate (a) (`:67`).** All three now carry the same channel-D facts and none contradicts another: `:42` "the live pid separates two processes at `src/checks.rs:791-792`, but it separates nothing at the two constant-pid test fixtures (see the scope section)"; `:51` "Exactly ONE of them carries the live pid: `:1491` uses `std::process::id()`, while `:1462` and `:1492` use `dead_pid()`"; `:67` "That argument does NOT hold at the two constant-pid fixtures (`src/checks.rs:1462` and `:1492`)". Verified against `src/checks.rs:1491` (`std::process::id()`), `:1462` and `:1492` (`dead_pid()`), and `:1438-1442` (`fn dead_pid` asserting `!pid_is_alive(u32::MAX)`). Untouched this round and still consistent.

**Mechanism (`:22`, `:26`, `:28`) against demonstration (`:82-:86`).** `:85` cites "the cross-repository `.git` corruption (or the `WorktreeSetup` failure, which is the more common shape)"; `:28` establishes both shapes and calls the loud one common (160 + 15 of 200 versus 25). Consistent. `:28`'s arithmetic checks: 25 + 160 + 15 = 200.

**Done conditions (`:59-:63`) accuracy, all five re-checked against the sections around them.**

1. `:59` unique by construction plus the argument in the code comment. Accurate; lands at `:789-790`, which `:94` now names explicitly, so the two reinforce rather than conflict.
2. `:60` one generator used by `run()` and the three fixtures. "Three fixtures" matches `:51`'s three sites and `:83`'s "the state the three fixtures are in today". Verified: exactly four `format!("{RUNNER_PREFIX}` sites exist, `:792` (production) plus `:1462`, `:1491`, `:1492` (the three fixtures).
3. `:61` `owning_pid` still parses the pid, pid stays first. Verified at `src/checks.rs:403-404`: `dir_name.strip_prefix(RUNNER_PREFIX)?.split('-').next()?.parse().ok()`. Accurate.
4. `:62` the doc-comment bullet. Agrees with `:94` on the substance; the two clause-level residues are `T3-1` and `T3-2`.
5. `:63` uniqueness pinned by a test that FAILS without the fix, pointing at the demonstration section. Agrees with `:84`, which the `TR2-1` fix left intact and now brings inside "the requirements above".

The block is still five bullets, and none of them was made stale by anything changed around them.

**Numeral check, run independently rather than trusted.** `grep -n "three\|four\|five\|Three\|Four\|Five"` over the sidecar returns exactly five hits, and I checked each against the list it counts:

- `:34` "five later runs passed": inside the verbatim block quote of the original sighting. Not the sidecar's own count; unchanged.
- `:55` "Three integration-test sites do use `{pid}-{nanos}`": three cited, `tests/validate_workflow_toml_source_needs_no_plan.rs:58` and `:90` and `tests/validate_toml_primary_skips_markdown_plan.rs:74`. I opened all three. Each is a `format!` whose second component is `std::time::SystemTime::now().duration_since(...).as_nanos()` and whose literal prefix is distinct (`agent-scaffold-validate-toml-only-`, `agent-scaffold-validate-workflow-no-source-`, `agent-scaffold-validate-projection-`). Count and claim both correct.
- `:60` "the three fixtures": matches, above.
- `:83` "the three fixtures are in today": matches, above.
- `:94` "four comments": four cited, four verified. Matches.

`grep -n "proportional minimum"` returns one hit, `:85`, which now carries no count at all. So no numeral in the document fails to match the list it counts, and the one phrase that used to carry an implicit count no longer does. The implementer's reported check reproduces.

## Citation check: ALL RESOLVE

I re-read every cited range rather than sampling, given the four misnumbered-citation defects this project produced this week. Zero misnumbered.

`src/checks.rs`: `:72-77` (the `RUNNER_PREFIX` doc comment, ending immediately above the `const` at `:78`), `:78` (`const RUNNER_PREFIX: &str = "agent-scaffold-checks-run-";`), `:329-342` (`impl Drop for WorktreeGuard`, opening to closing brace), `:388-392` (the `pid_is_alive` doc comment, the "no libc crate is pulled in just for a `kill(pid, 0)`" line at `:390`), `:400-402` (the `owning_pid` doc comment) and `:400-405` (comment plus function), `:407-461` (the `prune_orphan_worktrees` doc comment through the function's closing brace at `:461`), `:425-428` (the pid-reuse benign edge), `:734` (`pub fn run(`), `:789-790` (verbatim as quoted at `:94`), `:791-792` (the naming site), `:795-800` (the `RunError::WorktreeSetup` branch), `:845-847` (the `nanos` doc comment) and `:848-852` (`fn nanos` body), `:862-871` (`fn scratch`, literal-name discriminator), `:1438-1442` (`fn dead_pid`), `:1462`, `:1491`, `:1492`.

Other files: `src/main.rs:1726-1731`, `src/manifest.rs:552-558`, `src/plan/render.rs:638`, `tests/audit_command.rs:20`, `tests/scaffold_precommit_hook.rs:14`, `tests/checks_staged_hook_env.rs:50`, `tests/validate_workflow_toml_source_needs_no_plan.rs:58` and `:90`, `tests/validate_toml_primary_skips_markdown_plan.rs:74`. All resolve, and all seven "checked and NOT affected" helpers do discriminate by a per-test literal name as claimed.

Cross-references: `AGENTS.md:124` is principle 11, "Tests must actually exercise the code they claim to - A test must run the code path it claims to cover", quoted correctly at `:83`. Plan `[[principle]]` `n = 1` is "Prefer the cleaner long-term architecture over the smallest diff" (`:60`), `n = 5` is "Make illegal states unrepresentable" (`:72`), `n = 6` is "Ground decisions in evidence" (`:76`); all three use the plan's own numbering, which is the correct one. `Q-66` is `folded_into = "reviewer-reproducible-evidence"`, whose `order = 88`, so "`Q-66`, step 88" at `:76` is right. Step 85 is `drift-guard-test-hook-hygiene` and step 92 is `prompt-drift-guard`, matching `:88` and `:32`. `grep -cE "\brun\(" src/checks.rs` returns 23, the definition at `:734` plus the 22 call sites `:50` claims.

## Mechanical output

```
$ cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 219 records, valid
docs/plans/agent-scaffold.plan.toml: 92 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit 0

$ cargo run -- render docs/plans/agent-scaffold.plan.toml --check --strict
docs/plans/agent-scaffold.plan.toml: up to date
exit 0
```

92 steps and 69 questions as expected; 219 records, which is correct and includes another loop's round records.

Scope. `git diff --stat HEAD~1..HEAD` touches exactly two files, the sidecar and its regenerated view, 6 insertions and 6 deletions. `docs/plans/agent-scaffold.plan.toml` is UNTOUCHED this round, as the round-2 triage required. Over the whole fold the only added TOML table is the single `+[[step]]`; no `[[question]]` and no `type = "decision"` receipt was added anywhere. The sidecar's own claim at `:3` that the step "carries no `[[question]]` and no decision receipt" is therefore true of the tree.

`render --check --strict` returning "up to date" is also the confirmation that the rendered view at `docs/plans/agent-scaffold.md:1341`, `:1364`, `:1371` and `:1373` carries the same text as the sidecar, so no fix landed in only one of the two.

## Deliberately not raised

- **The "reproduced every one of those numbers" looseness at `:24`.** Settled by the round-2 triage as a non-finding, with its reasoning spelled out. I have no new evidence that its verdict was wrong, so it stays settled.
- **The `:82` "green against any of (a), (b), (c), (d) implemented correctly" versus done-condition 1.** Settled twice; the wording is the round-1 triage's own prescription. Not re-raised.
- **The channel-D question and the step title.** Both explicitly settled, and a triager recommended AGAINST broadening the title. Not re-raised.
- **`:86` ("Report the measured numbers") being outside the reach of "the requirements above".** Argued above; it is a reporting duty rather than demonstration machinery, and raising it would be manufacturing.
- **`:94`'s loss of the explicit "so they go stale" rationale for `:72-77` and `:400-402`.** The paragraph's tail still carries the staleness framing. No substantive loss.
- **The provenance of the probe numbers at `:24` and `:82` becoming unlocatable once the review files are deleted.** Noted and not raised by the round-2 triage on the ground that the numbers are quoted inline and the conclusion is stated as a floor. I agree, and the `T2-1` fix does not change it.

## Tree state

`git status --porcelain` in this worktree is EMPTY before this findings file is written, and after it reports only this one untracked file. No plan file, no sidecar, no source file was edited. Nothing was committed. No formatter was run. `validate` and `render --check` were run read-only. No probe was needed: every claim in this round is a documentation claim settled by a `file:line` citation or a re-runnable command, per `Q-66` proportionality, so no contrived test was written and nothing outside the repository was created beyond cargo's own `target/`.
