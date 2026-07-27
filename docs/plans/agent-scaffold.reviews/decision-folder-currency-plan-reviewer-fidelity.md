# Plan review, step 90 `decision-folder-currency`, reviewer lens: FIDELITY

Artifact: diff `8d12264..5bafb42` (commits `c54b0ae`, `5bafb42`) in worktree `.claude/worktrees/rev-dfc-fidelity`.
Files changed: `docs/plans/agent-scaffold.plan.toml`, `docs/plans/agent-scaffold.steps/decision-folder-currency.md`, `docs/plans/agent-scaffold.md`.

5 findings: 1 medium, 4 low. Zero critical, zero high.

---

## FID-1: the fold leaves the `Q-67` queue entry stale, and the entry now contradicts the step it schedules

Severity: `medium`.

Evidence. The `Q-67` decision record says, verbatim, that the work it schedules touches no prompt:

```
$ grep -n 'The work this schedules edits' docs/plans/agent-scaffold.plan.toml
1696:ask = """how to fix the actor ambiguity ...
```

`docs/plans/agent-scaffold.plan.toml:1696` (the `Q-67` `ask` body), projected verbatim to `docs/plans/agent-scaffold.md:188`, contains both of these clauses:

- "this pass restates none of that list and edits only the three actor-less `pack/AGENTS.md` prose points"
- "The work this schedules edits `pack/AGENTS.md` only (guidance, no prompt or source change), then regenerates the deployed `AGENTS.md` and `.agents/AGENTS.reference.md` via `just scaffold-self`."

The new step declares `Q-67` as its provenance (`docs/plans/agent-scaffold.plan.toml:1249-1250`, `[step.provenance]` / `decisions = ["Q-67"]`) and its scope is three passages in `pack/prompts/orchestrator.md` plus one in `pack/AGENTS.md`, with `.agents/prompts/orchestrator.md` added to the regeneration set (`docs/plans/agent-scaffold.steps/decision-folder-currency.md`, the "The passages" and "Documentation currency" sections).

So the plan now simultaneously asserts (a) `Q-67`'s work is `pack/AGENTS.md`-only with "no prompt or source change" and two regenerated deployed files, and (b) a `Q-67` step that changes a prompt and regenerates three deployed files. Reproduce the contradiction directly:

```
$ grep -c 'no prompt or source change' docs/plans/agent-scaffold.md
1
$ grep -c 'pack/prompts/orchestrator.md' docs/plans/agent-scaffold.steps/decision-folder-currency.md
9
```

Why it matters. The Open Questions queue is the durable decision record; a later reader, or the reviewer of the eventual step-90 implementation, checking the built change against `Q-67` would read the entry and correctly conclude that editing `pack/prompts/orchestrator.md` is out of scope for `Q-67`. That is exactly the "was this asked for?" check a work review runs. The planner extended a decided decision's implementation scope without updating the decision's own record.

Note on the schema: `folded_into` is a single `Option<String>` (`src/plan/source.rs:314-317`, "The step slug this decision was folded into"), so it cannot name both steps; leaving it at `planner-folds-decisions` is forced by the schema and is not itself the defect. The defect is the prose.

Suggested fix. Amend the `Q-67` `ask` body to record the 2026-07-27 scheduling extension: that the decision's remaining currency work was scheduled as a second step (`decision-folder-currency`, order 90) covering four further passages, three of them in `pack/prompts/orchestrator.md`, and that the regeneration set for the decision as a whole therefore includes `.agents/prompts/orchestrator.md`. Replace or qualify the "edits `pack/AGENTS.md` only (guidance, no prompt or source change)" clause so it describes step 89's pass specifically rather than the decision's whole implementation.

---

## FID-2: the diagnosis mischaracterises two of the four passages, and contradicts itself

Severity: `low`.

Evidence. The sidecar's opening paragraph (`docs/plans/agent-scaffold.steps/decision-folder-currency.md:3`) states:

> "FOUR passages still leave the actor unnamed, three of them in `pack/prompts/orchestrator.md` and one in `pack/AGENTS.md`. None of them contradicts the now-explicit rule; they are incomplete"

`pack/prompts/orchestrator.md` is written in the second person, addressed to the orchestrator. The actual lines:

```
$ grep -n "There, update the plan's Open Questions queue" pack/prompts/orchestrator.md
27:At each checkpoint, sync the durable state before moving on. ... There, update the plan's Open Questions queue and push its open items to the human, each per the human-input contract in `AGENTS.md`; ...
$ grep -n "emit the block and record the resolved answer" pack/prompts/orchestrator.md
31:Every time you put a decision to the human, emit a structured block ... for a question whose options are already clear, emit the block and record the resolved answer as a durable Open-Questions decision; ...
```

(Both lines are elided with `...` for length only; the quoted fragments are verbatim and the grep reproduces the full line.)

At `:27` ("update") and `:31` ("record") the actor is not unnamed: it is named, in the imperative, and it is the ORCHESTRATOR. Those two passages therefore do not merely omit the rule of `pack/AGENTS.md:41` / `:43`; they instruct the orchestrator to perform the act those lines reserve for the planner. Only `pack/prompts/orchestrator.md:33` ("only durable decisions ... fold into it") and `pack/AGENTS.md:63` ("only durable decisions ... fold into the plan's steps") are genuinely actor-less, and those are exactly the two the step-89 triager described as "passively (silent on the actor, NOT contradicting the now-explicit `AGENTS.md:41`)" (`docs/plans/agent-scaffold.ledger.md:337`).

The sidecar concedes the point two sentences later in its own first bullet and so contradicts its opening claim:

> "so the guidance and the prompt currently disagree about what the verb licenses"

Why it matters. Two things. First, "None of them contradicts the now-explicit rule" is false for `:27` and `:31`, which understates the defect: a prompt that tells the orchestrator to do planner work is a live contradiction, not an omission, and is the more urgent half of the step. Second, "name the actor" is the wrong operation at `:31`: the actor there is already named and is the wrong one, so the fix is a carve-out or reassignment, not an addition. The per-passage implementer bullets happen to state the right operation ("keep the orchestrator's own duty (routing it) explicit"), which is what keeps this at `low` rather than higher, but an implementer who works from the framing sentence rather than the bullets could add a planner clause and leave the orchestrator's imperative standing, producing a self-contradictory paragraph.

The step TITLE's chosen word "actor-ambiguous" is defensible for all four (at `:27` and `:31` what is ambiguous is the scope of the verb, not the identity of the subject). The body's stronger claim, "leave the actor unnamed", is not.

Suggested fix. Split the framing: two passages (`pack/prompts/orchestrator.md:33`, `pack/AGENTS.md:63`) are actor-less; two (`pack/prompts/orchestrator.md:27`, `:31`) name the orchestrator and so currently license it to do the planner's job. Drop "None of them contradicts the now-explicit rule; they are incomplete", or restrict it to the two actor-less ones.

---

## FID-3: the sidecar restates the generated closed list in the same paragraph that forbids restating it

Severity: `low`.

Evidence. `docs/plans/agent-scaffold.steps/decision-folder-currency.md`, the "Single source of the closed list" paragraph, opens with an enumeration and then denies making one:

> "The orchestrator's closed list of direct-on-main integration edits (a step's status flip, an increment declaration, a round record, and the ledger's resume anchor) is NOT hand-authored prose: it is the generated `ISOLATION_POLICY_FRAGMENT` (`src/isolation_policy.rs`) ... This step therefore restates NONE of that list either"

The four items are exactly the generated source's four items (`src/isolation_policy.rs:33`):

> "flipping a step's status, declaring an increment, recording a round record, and moving the ledger's resume anchor."

No guard covers the sidecar copy. `grep -rn 'include_str!' src/ --include=*.rs` shows the only embedded plan-side files are `AGENTS.md` and `.agents/AGENTS.reference.md`; nothing embeds or compares `docs/plans/agent-scaffold.steps/*.md`, so if `ISOLATION_POLICY_FRAGMENT` changes, this hand copy rots silently. That is the same class of duplication the paragraph itself calls a violation of "One source of truth" and Principle 8.

Precedent disclosed, because the triager should weigh it. The identical parenthetical already exists in two converged artifacts: `docs/plans/agent-scaffold.steps/planner-folds-decisions.md` ("a step's status flip, an increment declaration, a round record, and the ledger's resume anchor") and the `Q-67` `ask` at `docs/plans/agent-scaffold.plan.toml:1696`. Step 89's F1 was raised against the PACK edit, not against the sidecar, so this specific instance has not been settled, but it is a pre-existing plan-prose convention and may reasonably be judged accepted residual.

Why it matters. Low. The duplication is confined to the plan record, not the shipped pack, and the sidecar points at the source in the same sentence. The concrete cost is the self-contradiction ("restates NONE of that list" is false as written), which weakens the constraint it is trying to impress on the implementer.

Suggested fix. Drop the parenthetical and refer to the list by name only ("the orchestrator's closed list of direct-on-main integration edits, generated as `ISOLATION_POLICY_FRAGMENT` in `src/isolation_policy.rs` and rendered into `{{isolation_policy}}` at `pack/AGENTS.md:91`"). The instruction loses nothing: the implementer is told not to reproduce the items, so it does not need them listed.

---

## FID-4: "copy the pointing, not the list" is not directly executable in the prompt

Severity: `low`.

Evidence. The sidecar instructs, for the checkpoint paragraph:

> "its `pack/AGENTS.md:71` model ends by POINTING at the fragment ("the generated isolation-policy fragment below lists the orchestrator's closed set of direct-on-main edits ..."): copy the pointing, not the list."

That pointing is positional ("the fragment below") and only works inside `pack/AGENTS.md`, where the slot sits 20 lines later. The prompt has no such fragment:

```
$ grep -c '{{' pack/prompts/orchestrator.md
0
$ grep -n 'fragment' pack/prompts/orchestrator.md
(no match)
```

So an implementer that literally copies the pointing writes a dangling "the generated isolation-policy fragment below" into a file that contains no fragment, above or below.

Why it matters. Low: the very next sentence supplies the resolution ("The prompt may reference the rule in `AGENTS.md` rather than reproduce the fragment's contents"), and the prompt already has three worked examples of the correct form (`pack/prompts/orchestrator.md:11`, `:13`, `:15`, each "see the Writer isolation rule in `AGENTS.md`"). But the brief asked whether the instruction is unambiguous and achievable from the sidecar alone, and on this one point it requires the reader to notice the two sentences pull in different directions.

Suggested fix. State the prompt-side form directly: in the prompt, point at the checkpoint rule in `AGENTS.md` (the same "see the ... rule in `AGENTS.md`" form the prompt already uses), not at a fragment "below".

---

## FID-5: the rendered heading understates the scope that the TOML `title` states, and `step.title` is never projected

Severity: `low`.

Evidence. The two names for this step disagree:

```
$ grep -n '### `decision-folder-currency`' docs/plans/agent-scaffold.md
1176:### `decision-folder-currency`: name the planner as the folder at the remaining actor-ambiguous decision-folding points (`Q-67`)
```

versus `docs/plans/agent-scaffold.plan.toml:1241`:

> `title = "name the PLANNER as the folder at the four remaining actor-ambiguous decision-folding points (the checkpoint / queue-push, Socratic-mode, and ledger paragraphs of `pack/prompts/orchestrator.md`, and the ledger paragraph of `pack/AGENTS.md`), and regenerate the deployed copies (`Q-67`)"`

The TOML `title` is accurate and fully scoped. It is also invisible to every human reader of the plan, because `render` never projects it:

```
$ grep -rn '\.title' src/ --include=*.rs | grep -v 'meta\.title'
src/tui.rs:631: ... Block::bordered().title(title) ...
src/tui.rs:692: ... .title("Details") ...
src/tui.rs:764: ... .title("Save?") ...
src/plan/render.rs:694: // The title heading from `[meta].title`.
```

Only `[meta].title` reaches the render (`src/plan/render.rs:296`); the Roadmap table row is slug / status / provenance (`| `decision-folder-currency` | next | why: decisions Q-67 |`, `docs/plans/agent-scaffold.md:288`) and the Step Details heading comes from the sidecar's own `###` line. So the human-facing name of this step is the vaguer one.

The vaguer one also slightly overclaims: "the remaining actor-ambiguous decision-folding points" reads as ALL of them, while `pack/LEDGER.template.md:3` is deliberately excluded and the same sidecar calls it "Also actor-less". The TOML title's "four remaining" plus enumeration does not have that problem.

Why it matters. Low, but cheap to fix and directly on the lesson this step inherits: the step-89 known nit recorded at `docs/plans/agent-scaffold.ledger.md:345` was precisely a title that did not match the step's real scope ("step 89's TITLE (`plan.toml:1222`) still names the dropped Part-2 closed-list reinforce; its sidecar and ask are correct"). Here the mismatch runs the other way, and it is the accurate name that is unreachable.

Suggested fix. Bring the sidecar `###` heading in line with the TOML title: name "four" and the two files, e.g. "name the planner as the folder at the four remaining actor-ambiguous decision-folding points (three in `pack/prompts/orchestrator.md`, one in `pack/AGENTS.md`) (`Q-67`)", then re-render.

---

## Checked and found clean (non-findings)

Recorded so the triager and a later reader can see the coverage, and so these are not re-checked.

1. Every quoted clause verified verbatim at the stated line in the stated file. `pack/prompts/orchestrator.md:27` ("There, update the plan's Open Questions queue and push its open items to the human"), `:31` ("for a question whose options are already clear, emit the block and record the resolved answer as a durable Open-Questions decision"), `:33` ("only durable decisions, the ones that change the plan, fold into it"), `pack/AGENTS.md:63` ("only durable decisions, the ones that change the plan, fold into the plan's steps"). The paragraph-opening phrases used as anchors ("At each checkpoint, sync the durable state before moving on", "Every time you put a decision to the human, emit a structured block at that gate per the human-input contract", "The ledger is separate from the plan", "Preventing relitigation (the ledger)") all match. No misquote, no wrong line number.

2. Supporting citations verified. `pack/AGENTS.md:41` is the human-input contract; `:43` is the Socratic-mode paragraph and does say "routed to the planner to author"; `:71` is the checkpoint paragraph and carries both quoted clauses verbatim, including the isolation-policy pointer; `:91` is the `{{isolation_policy}}` slot. The fragment rationale quoted in the sidecar ("author no reviewed product content and so stay the orchestrator's direct job rather than a spawned agent's") is verbatim from `src/isolation_policy.rs:33`.

3. Scope is exactly the four intended passages. Nothing smuggled in, nothing dropped: the four listed passages map one-to-one onto the four "What the implementer changes" bullets. No fifth file, no source change, no template change.

4. The `pack/LEDGER.template.md:3` exclusion is recorded with reasoning and with the human's confirmation ("The human confirmed this exclusion when they set the step's scope (2026-07-27)"), and the quoted text matches the file. The scope-history paragraph records the `:27` inclusion overrule as decided.

5. Structured fields correct. `docs/plans/agent-scaffold.plan.toml:1239-1250`: `slug = "decision-folder-currency"` (:1240), `status = "next"` (:1242, the only `status = "next"` in the file), `order = 90` (:1243, previous max 89), `increment = []` (:1246) with no `[[step.increment]]` table anywhere in the diff, `[step.provenance]` / `decisions = ["Q-67"]` (:1249-1250).

6. No question and no receipt were added, verified by counting rather than by reading the claim.

```
$ grep -c '^\[\[question\]\]' docs/plans/agent-scaffold.plan.toml
68
$ git diff 8d12264..5bafb42 --name-only
docs/plans/agent-scaffold.md
docs/plans/agent-scaffold.plan.toml
docs/plans/agent-scaffold.steps/decision-folder-currency.md
```

`docs/metrics/workflow.jsonl` is untouched, and `validate` reports 209 records both sides. The `Q-52` precedent the sidecar cites is real and stated in the same terms at `docs/plans/agent-scaffold.ledger.md:345` ("recorded here as build-plan refinements, NOT new `[[question]]` ids, so no decision receipt is owed").

7. The known trap, restating the closed list in the PACK, was avoided. The sidecar's implementer instructions never enumerate the four direct-on-main edits as something to write into `pack/`; they say the opposite ("it must not enumerate the four direct-on-main edits itself", "CROSS-REFERENCE the fragment; do not copy from it"). The only enumeration is in the sidecar's own prose, raised separately as FID-3.

8. The documentation-impact list is correct AND complete, verified independently of the planner's claim. Grepping for each of the three affected clauses across the tree returns exactly the pack sources, the plan documents, and three deployed files:

```
$ grep -rln "only durable decisions, the ones that change the plan" . --exclude-dir=.git
AGENTS.md
docs/plans/agent-scaffold.steps/decision-folder-currency.md
.agents/prompts/orchestrator.md
docs/plans/agent-scaffold.md
pack/prompts/orchestrator.md
.agents/AGENTS.reference.md
pack/AGENTS.md
$ grep -rln "record the resolved answer as a durable Open-Questions decision" . --exclude-dir=.git
docs/plans/agent-scaffold.md
docs/plans/agent-scaffold.steps/decision-folder-currency.md
.agents/prompts/orchestrator.md
pack/prompts/orchestrator.md
$ grep -rln "update the plan's Open Questions queue and push its open items to the human" . --exclude-dir=.git
docs/plans/agent-scaffold.steps/decision-folder-currency.md
docs/plans/agent-scaffold.md
.agents/prompts/orchestrator.md
pack/prompts/orchestrator.md
```

The deployed set is exactly `.agents/prompts/orchestrator.md`, `AGENTS.md`, `.agents/AGENTS.reference.md`, as claimed. No fourth deployed file is affected, and `diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md` is currently empty, confirming the "verbatim copy" characterisation.

9. The drift-guard claim is TRUE, verified against the source rather than trusted. `src/agents_md_drift.rs:290-323` (`the_committed_scaffold_matches_a_fresh_render`) covers only `AGENTS.md` and `.agents/AGENTS.reference.md` (`self_scaffold_asset("AGENTS.md")` and `self_scaffold_asset(".agents/AGENTS.reference.md")`). No test in the tree embeds or compares `.agents/prompts/orchestrator.md`: `grep -rn 'include_str!' src/ --include=*.rs` yields only `../AGENTS.md`, `../.agents/AGENTS.reference.md`, `../pack/workflow.toml`, `../pack/principles.toml`, `../pack/instrument.md`. `grep -rn "agents/prompts" src tests build.rs justfile` shows the path appears only in the manifest asset list (`src/manifest.rs:604`) and in `next.rs` path-shape strings, never in a content comparison. So the planner's warning that skipping the `.agents/prompts/orchestrator.md` regeneration would be a SILENT staleness is accurate and is the most useful sentence in the sidecar. No mutation was run, because the citations settle it.

10. The regeneration recipe is as described: `justfile:46-48`, `scaffold-self` runs `cargo run -- scaffold --output-dir . --write --force --principles default --instrument` then `nix fmt`.

11. Toolchain state at `5bafb42` is green and the generated view is current:

```
$ cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml
docs/metrics/workflow.jsonl: 209 records, valid
docs/plans/agent-scaffold.plan.toml: 90 steps, 68 questions, valid
$ cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
$ cargo run --quiet -- render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date
```

12. Suspicion I could NOT ground, reported as a note rather than dressed up as a finding. The new sidecar opens with the word "Next.", duplicating `status = "next"` in prose, and that duplication demonstrably rots: `docs/plans/agent-scaffold.steps/planner-folds-decisions.md` still opens "Not started." while `docs/plans/agent-scaffold.plan.toml:1226` says `status = "complete"`, and the same mismatch holds for `decision-receipt`, `round-log-core`, `session-preflight`, `structured-skeleton`, `workflow-invariants`, and `reviewer-reproducible-evidence` (all `complete` in the TOML, all opening "Next" in prose). I am NOT raising this as a finding against this diff: the lead-status-word is a pervasive pre-existing convention across the sidecar set, the new instance is currently accurate, and the fix is a project-wide cleanup rather than a defect in this fold. It is the same shape as step 88's LOW-1, which the triager ruled valid-but-accept-residual for exactly that reason. Recorded here so it is visible without being counted.

13. Also checked and NOT raised: the sidecar's "This is the same sentence as the one above" for `pack/AGENTS.md:63` is loose (the two copies already differ, "fold into it" versus "fold into the plan's steps", and `:63` continues ", and a folded decision reopens only by evidence that beats its recorded reasoning"), but the sidecar quotes both clauses verbatim immediately above, so a reader cannot be misled and the conclusion drawn from it (fix both) is right. Similarly, the sidecar attributes only `pack/prompts/orchestrator.md:33` to the step-89 triager's accepted-residual note, whereas `docs/plans/agent-scaffold.ledger.md:337` shows the triager named `:33` AND `pack/AGENTS.md:63`; this under-credits the triager by one passage but changes no scope and no conclusion. Both are below the bar for a finding.
