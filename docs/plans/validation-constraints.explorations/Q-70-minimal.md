# validation-constraints (Q-70): the minimal fix, and what not to build (restraint lens)

This is one of several independent explorations of `Q-70`. My brief is narrow on purpose: price
the four candidate directions at what they actually cost against this repository, find the
smallest one that is still correct, and name what should not be built. Where the other
explorations argue for the cleanest architecture or for what the evidence already settles, I argue
for the smallest change that leaves the tool honest, and I try to talk myself out of it wherever
the evidence allows. It did not let me; the result below is a demonstrated, test-passing patch, not
a hypothesis.

Method note, stated once so it does not have to be repeated: every claim below marked VERIFIED was
established by running something against this worktree (`/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/exp-vc-minimal`)
or a disposable scratch copy of it, not by re-reading the item's own prose. Fixtures live under
`/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/q70-minimal/`
and are throwaway; the recipe to reproduce each result is given inline so the finding survives
after that directory is gone. I did not edit this worktree's `src/` or plan; every patch discussed
below was applied to a separate `rsync` copy of the tree under the scratch directory.

`W6` collision notice (duty (d), ruled up front so the rest of this document can just use the
word): I use `W6` below to mean only the waiver-note breakdown join, `Q-70`'s own mechanism (1).
I never mean `Q-59`'s session_state transition-legality check. Where the distinction matters I say
"mechanism (1)" instead.

## The question

How should W5's waiver-ownership check be fixed, what is the smallest correct fix, and which of
the pass's other commissioned work (the coupling ruling, the three detection mechanisms) should
actually be built now rather than later or never.

## Re-verifying the blocker, briefly

`Q-70` already measured the blocker and its escape routes in detail; I re-derived the load-bearing
ones myself rather than trusting the citation, because the item itself says every figure here has a
history of being wrong.

VERIFIED, the bug: `leading_slug(increment) != waiver.step` at `src/workflow.rs:564` is a lexical
strip of the waiver's own `increment` string, unrelated to which step the round log actually
recorded that increment against. VERIFIED, the double lock: I copied `docs/` into a scratch
directory, flipped `workflow-enforcement-tier` to `status = "complete"`, added the two owed waivers
(`workflow-enforcement-tier-w5` naming `increment = "workflow-enforcement-tier-fold"`, `-w6` naming
`workflow-enforcement-tier-endproperty-fold`, both `record-backed` / `accepted-at-escalation`), and
ran `agent-scaffold validate --source <toml> --workflow --metrics <jsonl>`. Undeclared: exit 1,
four problems, two from `src/plan/source.rs`'s membership check ("not one of the step's increments")
and two from W5's lexical check ("belongs to step `workflow-enforcement-tier-fold`", a step that
does not exist). Declared (adding both tokens as `[[step.increment]]`): exit 1, the two
`source.rs` problems vanish, the two W5 problems persist unchanged. This matches the item's own
measurement exactly and confirms the "double lock, then one refusal" shape is real, not stale.

VERIFIED, every escape route the item lists still holds, re-checked against the source rather than
relayed:

- Step-unit waiver only applies when a step has zero round records (`src/workflow.rs:450`); both
  fold tasks carry five records each (`jq -c 'select(.type=="round" and .task=="workflow-enforcement-tier-fold")' docs/metrics/workflow.jsonl | wc -l` returns 5, and the same for the
  `-endproperty-fold` task).
- A `[[step.waiver]]`'s `step` is not independently settable in TOML: `waivers_from_toml` sets
  `step: step.slug.clone()` from the containing `[[step]]` (`src/workflow.rs:258`), read directly.
- `[meta].orphan_tasks` never appears in `src/workflow.rs` (`grep -n orphan_tasks src/workflow.rs`
  returns nothing); it is consumed only by `src/plan/source.rs`'s own validation, so declaring
  either fold token there changes nothing W3 or W5 read.
- Declaring the tokens as `[[step.increment]]` entries alone, no code change, still fails: the
  scratch "declared" run above is exactly this case, and it still exits 1 on the two W5 problems.
- `plan::Step` (`src/plan.rs:55-60`) really does carry only `slug` and `status`; there is no
  declared-increment data in W5's structural input to consult even if the code wanted to.

I found no sixth escape route. I specifically checked three more that are not in the item's list,
because a restraint-lens pass should try harder than "the record says none exist" before agreeing:
waiving at the STEP unit instead of the increment unit is achievable in principle but throws away
the four real, hard-won `-inc1`..`-inc4` waivers' worth of increment-level evidence to dodge a
tooling bug, which is a worse dishonesty than the one being fixed. Leaving the step `in-progress`
indefinitely to avoid triggering W3 at all is the `pause.md` failure mode in reverse (an
indefinitely-wrong status instead of a falsely-complete one) and contradicts the step's own stated
purpose. Running `validate` without `--workflow` going forward defeats the enforcement tier this
very step exists to add. None of these is a resolution; all three are ways of not paying a cost
that still needs paying. **There is no cheaper resolution to the original problem than a code
change.** The two owed waivers cannot be written under the current rule, in any combination of
declaration or waiver-unit choice, without either fixing the ownership check or accepting a worse
trade than the one it currently makes.

## The design space: pricing the four candidates against the repository

### Direction (iii): key the ownership check off the round log, like W3 does

This is the direction I built and tested, not merely priced. `src/workflow.rs`'s `run_checks`
(`:206-221`) already holds `rounds: &[Round]` and already hands it to `w3_problems`; it simply does
not hand it to `w5_problems`. The patch: add a `rounds: &[Round]` parameter to `w5_problems`, build
`let increment_step: BTreeMap<&str, &str> = rounds.iter().map(|r| (round_increment_id(r), round_step_slug(r))).collect();`
(both accessors already exist, at `:119` and `:127`, and already prefer the structured `step`/
`increment` ids over the `leading_slug` shim when present), and replace the `leading_slug`
comparison with a lookup in that map, reporting a mismatch only when the map has an entry that
disagrees. An increment absent from the round log is left unchecked here (nothing to attribute
yet); see Limits below for exactly what that costs.

VERIFIED, correctness: applied this patch plus the declaration (the two `[[step.increment]]`
entries) to a scratch copy of the whole repository (`rsync --exclude target --exclude .git`), ran
`cargo build` (clean), then ran `validate --source <toml> --workflow --metrics <jsonl>` against the
same fixture that exits 1 unpatched: exit 0, `workflow invariants hold`. VERIFIED, no regression:
after mechanically adding the missing 4th argument to the 14 existing test call sites in
`src/workflow.rs`'s `#[cfg(test)]` module, `cargo test` in the scratch copy passes 378 of 378 unit
tests plus every integration test, zero failures. Exactly one of the 14 call sites needed more than
a mechanical `&[]`: `w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment`
(`:1442-1458`), the one test that actually exercises a genuine mis-scoped waiver, needed a
synthesized `Round` (via the existing `structured_round_line`/`rounds()` helpers already in that
test module) establishing that `beta-incB` belongs to `beta`, because the check's evidence now
comes from the round log rather than from string-splitting the waiver's own claim. I made that
change and the test still asserts the same mismatch, correctly, once the fixture supplies real
data. `cargo clippy --all-targets -- -D warnings` on the patch is clean. VERIFIED, the false-fact
bug is also fixed: the item's own `totally-not-a-step-inc1` fixture (a waiver naming an increment
with no round records at all) produces, unpatched, "belongs to step `totally-not-a-step`" even
though `grep -c 'slug = "totally-not-a-step"'` over the plan returns 0; patched, W5 is silent on
that waiver (nothing to attribute) and the pre-existing `src/plan/source.rs` membership check still
catches it correctly ("not one of the step's increments") as long as it stays undeclared. See
Limits for the one case where declaring a never-logged token changes that.

Edit surface: `src/workflow.rs` only for code (one function signature, about ten lines of body,
one call site, 14 mechanical test-call-site edits, one substantive test-fixture addition). No
change to `plan::Step`, `Waiver`, `Round`, `render.rs`, or any TOML schema. It does, however, touch
prose that is drift-guarded (see Edit surface section below): the sentence "an `increment`-unit
waiver's `step` must own its `increment` (the increment's leading slug equals the step)" is stated
verbatim in `pack/instrument.md:11`, `AGENTS.md:147`, and `.agents/AGENTS.reference.md:147`
(VERIFIED with `sed -n '145,149p'` on both generated files and the corresponding line in the pack
source; all three are byte-identical for this sentence), guarded by
`the_committed_scaffold_matches_a_fresh_render` (`src/agents_md_drift.rs:40-55`). Fixing the code
without updating this sentence in all three places leaves the scaffolded documentation asserting
the bug as the intended design, so the prose edit is not optional polish, it is part of the fix.
Regeneration is `just scaffold-self`, whose second line is `nix fmt` over the whole tree; this
repository is not formatter-clean at HEAD, so that command will reflow files well outside this
change and its diff must be scoped back down to the three prose files plus the code before
committing.

### Direction (i): narrow lookup against declared `[[step.increment]]`

VERIFIED costly in a way (iii) is not. `PlanToml::step_views()` (`src/plan/source.rs:422-430`)
projects only `slug` and the rendered `status` label into `plan::Step`; `plan::Step`
(`src/plan.rs:55-60`) is `Serialize` and is exactly the `steps` field of the `status --json`
machine payload (`PlanProjection.steps`, confirmed by reading `src/main.rs` around the projection
struct). Widening it to carry declared increments changes a machine output contract that ships
today, not an internal detail. It also does not work on the Markdown substrate at all: no
increments are declared there (`src/next.rs:520,551`), and `src/next.rs:513-523` states, in its own
doc comment, a "parity property" the project has *already chosen*: the declared
`[[step.increment]].risk_class` is deliberately NOT carried into that projection specifically so the
Markdown and TOML substrates project identically and the forward (`next`) and backward (`validate`)
verdicts cannot diverge. Direction (i) pushes directly against a design decision this same
codebase has already made and documented, for the same general class of question (what an
increment's convergence-relevant metadata should be keyed from). And even after paying that cost,
it still needs the same declaration step that (iii) needs (the increments must exist in the plan
for there to be anything to look up), so it does not remove the plan-edit half of the fix either.
It buys nothing (iii) does not already buy, at a real and specific added cost. I priced it fully
and it is not close.

### Direction (ii): rework how a waiver names its unit

This reaches three representations at once: the JSONL `type:"waiver"` arm of `check_record`
(`src/metrics.rs:539-601`), the TOML typed `Waiver` struct (`src/plan/source.rs:279-300`), and the
`waivers_from_toml` flattening that reconciles them (`src/workflow.rs:237-267`). It is the only one
of the four directions that touches the waiver's own schema rather than how the ownership check
reads existing fields. Nothing about the blocking bug requires this: I demonstrated a working,
fully-tested fix (iii) that does not touch the `Waiver` shape at all, on either substrate. Reworking
the representation would be solving a problem that is not the one blocking the two waivers ("how
should a waiver name its unit", a real but separate question) instead of the one that is ("does
the ownership check believe a true fact about an already-correctly-named unit"). This is the
direction I am most confident should not be taken now; not because it is wrong in the abstract, but
because it is strictly larger than a direction that already fixes the actual defect, and nothing
here forces it.

### Direction (iv): scope or retire the rule per substrate

VERIFIED partially (I relied on the item's own scratch measurement for the "disable and it goes
green" claim rather than re-deriving it myself, since I was not going to recommend this direction;
I did independently verify its stated precondition, that TOML nesting forces `waiver.step`
non-negotiably at `src/workflow.rs:258`, which is the fact the "premise does not hold on TOML"
argument rests on). Two forms exist. Fully retiring the rule is the single smallest code change of
all four (delete or no-op the block), but it is not merely "small", it removes a real check on the
substrate where its premise genuinely does hold: on JSONL, `waiver.step` is a free field, not forced
by any nesting, so a mis-scoped JSONL waiver is a real defect this rule is the only thing that
catches, and retiring it globally leaves the tool silent on that. That is the "leaves the tool
asserting something false" failure my brief asked me to watch for, and this is where it shows up:
the smallest member of the design space is the one that is actually wrong. Scoping the rule to run
only on the substrate where it is meaningful is defensible in isolation, but it forks `run_checks`,
which the module doc for `check_workflow_toml` (`src/workflow.rs:196-200`) states is deliberately
ONE implementation across both substrates, citing Principle 16 by name. Direction (iii) achieves the
same "correct on both substrates" outcome (VERIFIED above: the fix is substrate-agnostic, since the
round log is always JSONL regardless of which plan substrate produced the steps) without forking
anything. Direction (iv) is therefore dominated: whatever it is trying to buy, (iii) buys more
cleanly.

### Cost comparison

| Direction | Files touched (code) | New/widened schema | Works on Markdown substrate | Fixes both blocking waivers | Fixes the false-fact bug |
| --- | --- | --- | --- | --- | --- |
| (i) narrow declared-set lookup | `src/plan/source.rs`, `src/plan.rs`, `src/main.rs` (JSON contract), `src/workflow.rs` | widens `plan::Step` (machine output) | No (owes a ruling the item itself flags) | Yes, plus the same declaration step (iii) needs | Only where declared |
| (ii) rework waiver-unit representation | `src/metrics.rs`, `src/plan/source.rs`, `src/workflow.rs`, `pack/plan-template.plan.toml`, `src/plan/render.rs` | reshapes `Waiver` on both substrates | Yes | Yes, scope depends on the rework chosen | Depends on the rework chosen |
| (iii) round-log join (RECOMMENDED) | `src/workflow.rs` only, plus 3 drift-guarded prose copies | none | Yes, by construction | Yes (VERIFIED, exit 0) | Yes (VERIFIED) |
| (iv) scope/retire per substrate | `src/workflow.rs` (forked) or delete | none | Retiring: yes, silently wrong on JSONL. Scoping: yes, but forks Principle 16 | Yes | Retiring: no, removed. Scoping: only on TOML |

## Trade-offs against the Project Principles

**Principle 1** ("prefer the cleaner long-term architecture over the smallest diff"). Direction
(iii) is not merely the smallest diff by accident; it is the smallest diff because it is the
architecturally consistent one. `src/next.rs`'s own parity-property comment shows this project has
already chosen, elsewhere in this same enforcement family, to derive convergence-relevant facts
from the round log rather than from declared plan metadata, specifically to keep the two substrates
identical and the forward/backward checks non-divergent. Fixing W5 the same way is not a shortcut
against Principle 1, it is following the precedent Principle 1 already produced once in this
codebase. Direction (i) is the one that trades architecture for a declared-data shortcut and, per
the comparison table, still does not avoid the declaration step.

**Principle 2** ("minimal by default... adding a module must not complicate the core"). Directions
(ii) and the forking half of (iv) both add a substrate-conditional branch or a schema-wide rework to
what is currently one shared implementation; (iii) adds one parameter to one existing function.

**Principle 5** ("make illegal states unrepresentable"). None of the four directions changes what
states are representable; all are checks on existing representable states. Where this principle
bears is on the mechanisms (below): a detection mechanism whose naive form cannot distinguish a
legitimate convention from a real defect is not making an illegal state unrepresentable, it is
making a legal one look illegal, which is its own failure of this principle in the opposite
direction.

**Principle 6** ("ground decisions in evidence"). This is the principle the human's own decision to
run this pass at all was attached to (`Q-55-entryroute`). Every comparative claim in the cost table
above and every yield claim in the mechanisms section below was run, not read off the item's prose;
where I could not run something (direction (iv)'s exact "disable and it goes green" figure) I said
so explicitly rather than silently inheriting it.

**Principle 16** ("one implementation" across substrates, named in `src/workflow.rs:196-200` and
cited by the item itself under direction (iv)). Directly favors (iii) over the forking half of
(iv) and over any version of (ii) that needs substrate-specific handling.

**Principle 8** ("structured data first... prioritise the best long-term design... over
minimal-diff blast radius, at this pre-adoption stage"). This is the one principle that could argue
against my recommendation: it explicitly says minimal-diff should not win over the best long-term
design at this stage of the project. I take this seriously rather than waving it off, because my
whole lens is "argue for restraint" and this principle argues the other way. My answer is that (iii)
is not a minimal-diff compromise against a cleaner alternative that Principle 8 would prefer; per
Principle 1's analysis above, it *is* the design this codebase's own precedent (the `next.rs` parity
property) already committed to. Principle 8 would have force against me if a genuinely cleaner
design existed and I were recommending against it to save lines. I priced (i), (ii), and (iv)
concretely above and none of them is cleaner; two are actively less consistent with what this
project has already decided elsewhere. Principle 8 and Principle 1 point the same way here.

## Recommendation

Take direction (iii): teach W5's increment-ownership check the round log's own structured/lexical
step association, the same one W3 already uses, by threading `rounds` into `w5_problems` and
replacing the `leading_slug` string comparison with a lookup keyed on `round_increment_id`. This is
demonstrated, not proposed: it builds, it fixes both owed waivers, it fixes the false-fact
substring bug as a side effect, and the full existing test suite (378 unit tests plus integration
tests) passes against it with one test rewritten to supply real data instead of a mechanical stub.
Pair it with the plan-content edit that is required either way regardless of which direction is
chosen: declare `workflow-enforcement-tier-fold` and `workflow-enforcement-tier-endproperty-fold`
as `[[step.increment]]` entries (`risk_class = "risky"`, matching what the round log already
records for both, VERIFIED by `jq`), then author the two owed waivers per the ledger's drafted
notes, then flip the step to `complete`. VERIFIED that this declaration, on its own, changes nothing
about the rendered `.md` (`render --check --strict` reports "up to date" both before and after
adding the two increments to a fixture that is otherwise byte-identical to the live plan) and
passes `validate --source` cleanly, so it carries none of the render-drift risk the item flags for
direction (ii).

## The three detection mechanisms: what to build now

My brief asks me to be adversarial here specifically, and the evidence supports being adversarial.

### Mechanism (1), the W6 waiver-note breakdown join: do not build now

The item's own yield claim is "red on nothing today... a regression guard on a convention held
correctly by hand." I spot-checked this rather than accepting it on faith: `workflow-enforcement-
tier-w1`'s note claims "13 valid findings (3, 4, 6)" for `workflow-enforcement-tier-inc1`; `jq -r
--arg t workflow-enforcement-tier-inc1 'select(.type=="round" and ((.increment // .task)==$t)) |
.valid_findings' docs/metrics/workflow.jsonl` returns `3`, `4`, `6`, matching exactly. I did not
re-check every waiver in the plan (that would be disproportionate to a mechanism whose own stated
yield is zero), but the one I checked confirms the claim rather than undermining it. Building a
check with zero current yield is a maintenance cost with no return, full stop.

It is also not free to build: per the item's own measurement, it needs `Round` to carry
`valid_findings` (currently explicitly discarded by `parse_rounds`, `src/metrics.rs:660-711`), a
field the ALREADY-QUEUED project-identity edit (`Q-55-mechanism`, decided 2026-07-31) also wants to
add to the same struct, under an explicit human constraint that this shared schema take ONE
deliberate edit, not a rider on a path fix. I verified `Round` (`src/metrics.rs:620-651`) carries no
`project` field today, so that edit is genuinely still pending. Building mechanism (1) now, before
that coordination happens, is exactly the kind of uncoordinated rider the constraint exists to
prevent. Coupling ruling (part of duty (a)): mechanism (1) and my recommended W5 fix share the
underlying "index round records by increment id" join pattern (my `increment_step` map is a
degenerate case of the fuller "increment to its list of rounds" grouping mechanism (1) would build)
but share no data; my fix adds zero fields to `Round`. Project identity and mechanism (1) DO share a
mechanism, the `Round` schema itself, and that is the pairing the human's constraint actually
governs, not the W5 fix.

### Mechanism (2), dangling decision-receipt detection: do not build in its literal form, possibly ever in that form

This is the sharpest finding in this document, and it goes further than the item's own framing.
VERIFIED: every distinct `type:"decision"` `q_id` in the log against every registered `[[question]]`
id: 63 distinct receipt ids exist; 41 of them (65 percent) do not literally match a registered
question id. I then split that 41 by whether it is a bare `Q-<n>` or a `Q-<n>-<suffix>` sub-decision
id: **all 41**, with zero exceptions, are `Q-<n>-<suffix>` forms, and every single one of the 41 is
a `Q-55-<suffix>` sub-decision (confirmed by stripping each unregistered id back to its `Q-<n>`
prefix and grouping: the count is 41 under `Q-55` and 0 under any other prefix).
The 22 bare `Q-<n>` receipt ids all resolve cleanly to a registered question. So the literal
"every `q_id` must resolve to a registered question" check the item states as mechanism (2)'s
"stage one" would be permanently red on 65 percent of every decision this project has ever recorded,
with a false-positive rate of 100 percent against the actual defect class (a genuinely dangling
receipt), because there are currently zero of those in the data. This directly answers duty (e):
the `Q-<n>-<suffix>` ids are not dangling receipts, they are the dominant, load-bearing convention
for recording sub-decisions inside one larger decision, and any dangling-receipt detector MUST
resolve a suffixed id to its `Q-<n>` prefix before checking registration, or it is not a detector,
it is a permanent false alarm generator on its own most common case. My brief asked me to say
plainly if a mechanism's naive form is worse than nothing; this is that case, just as a false-red
rather than a false-green. A check that is wrong two-thirds of the time trains an orchestrator to
ignore its output, which is a worse outcome than not having the check, because a real future
dangling receipt would arrive already inside a sea of routine noise. Do not build the literal
stage-one check. If mechanism (2) is built later, it must be built prefix-aware from the start; that
is a bound on its future design, not a design of it (duty (f), below).

### Mechanism (3), the quotation resolver: do not build now, and only with two escapes if ever

I spot-checked one of the item's stale-citation claims rather than trusting the list wholesale:
`src/checks.rs:78` resolves to `PathBuf,` inside an import block, plainly unrelated to whatever the
worktree-name-collision doc originally cited it for. That is consistent with the item's claim that
every citation in that file is now stale, though I did not re-check all fifteen. Beyond the item's
own two named design constraints (the self-quoting-record trap and the runtime-substituted-output
trap, both plausible on inspection and not independently re-derived by me here since I had no
cheap way to re-run them), the practical case for not building this now is simpler: its own
described "opening red-list" is the entire stale citation set, so a freshly-built resolver's first
real output would be a wall of already-known, already-accepted staleness, not a new finding. A
mechanism whose first run reproduces a fact already on record is not yet earning its cost.

## The YAGNI boundary

Specific things a reader of `Q-70` might reasonably reach for, that this exploration says not to
build, and why:

- Do not widen `plan::Step`/`step_views()` to carry declared increments into the enforcement path
  (direction (i)). It changes a machine output contract (`status --json`), does not work on the
  Markdown substrate without a further special case, contradicts the parity property `src/next.rs`
  already committed to for the same class of question, and does not even remove the declaration
  step my recommended fix also needs.
- Do not rework the waiver-unit representation across its three forms (direction (ii)) to fix this
  blocker. It is solving "how should a waiver name its unit", a real but separate question, when
  the actual blocker is "does the ownership check believe a true fact about an already-correctly-
  named unit". A demonstrated, fully-tested, single-file fix exists that never touches the
  `Waiver` shape.
- Do not fork `run_checks` or retire the ownership rule globally (direction (iv)) to make the TOML
  substrate pass. Forking contradicts Principle 16, stated by name in the code it would fork.
  Retiring globally removes a real check on the JSONL substrate, where a waiver's `step` really can
  diverge from its increment independently, which is exactly the class of silent wrongness this
  whole pass exists to avoid introducing.
- Do not build mechanism (1) (the W6 note-join) now. Measured zero current yield (spot-checked, not
  merely relayed), and it needs a `Round` schema edit that is better batched with the already-queued
  project-identity edit than sprung as an independent rider.
- Do not build mechanism (2) (dangling-receipt detection) in the literal form the item states.
  Measured 100 percent false-positive rate against its own defect class today (41 of 41
  "unregistered" ids are the legitimate sub-decision convention, zero are genuine errors). If it is
  ever built, it must be prefix-aware of `Q-<n>-<suffix>` from its first line of design, not added
  as a patch after the false alarms are noticed.
- Do not build mechanism (3) (the quotation resolver) now. Its first real run would mostly
  reproduce an already-recorded staleness fact, and it needs the self-quoting and runtime-output
  escapes designed in before it produces any information a human does not already have.
- Do not pull the `Round.project` field (queued by `Q-55-mechanism`) into this fix. Nothing in the
  demonstrated W5 patch touches `Round` at all; adding an unrelated field here would be exactly the
  "rider on a path fix" the human's own recorded constraint forbids.
- Do not resolve the `W6` naming collision with `Q-59` as part of this fix. It is a registry/
  documentation cleanup (rename one of the two), entirely orthogonal to shipping the ownership-
  check patch, and bundling it risks conflating two unrelated review histories in one diff.
- Do not expand this fix's scope to specifically also address the `workflow-driver` declared-
  increment case (`-stage0a`/`-stage0b`/`-stage1`, none ending `-inc<alnum>`) as a separate line
  item. It is real, and it is fixed as a free side effect of (iii) once that step's round records
  carry structured ids (the same accessors, `round_step_slug`/`round_increment_id`, already prefer
  the structured fields over the lexical shim wherever they are present); it does not need, and
  should not get, its own bespoke handling.

## Rulings on the lettered duties

**(a) The coupling ruling.** W5's fix (direction (iii)) and mechanism (1) share a JOIN PATTERN
(indexing round records by increment id) but not DATA: my patch adds no field to any struct.
Project identity (`Q-55-mechanism`) shares neither the join pattern nor any data with the W5 fix; it
filters `check_workflow_toml`'s inputs on an orthogonal attribution axis and touches `Round` only
because mechanism (1) also wants to touch `Round`, for an unrelated field. So: W5-fix and mechanism
(1), shallow coupling (pattern only); W5-fix and project identity, no coupling; mechanism (1) and
project identity, real coupling (both want to widen the same struct, which is exactly what the
human's "one deliberate edit" constraint is about). What mechanism (1) costs under my recommended
direction: unchanged from what the item already priced, since direction (iii) makes zero changes to
`Round`, `parse_rounds`, or `check_record`. That is itself informative: choosing (iii) does not make
mechanism (1) any cheaper or any more urgent than it already was.

**(b) The authoritative-path ruling.** Both paths should keep running, each on the axis it is
actually suited to, which is what naturally falls out once W5 is fixed rather than something that
needs deciding separately. `src/plan/source.rs:807` owns a plan-authoring correctness question ("is
this a token the step's own TOML declares"), necessarily TOML-only. A fixed W5 owns an evidence
correctness question ("does the round log agree this increment was actually reviewed under this
step"), substrate-independent because the round log is always JSONL. These are not redundant once
W5 stops asserting a false fact: I verified that on the correctly declared-and-scoped case both are
silent (exit 0), and on the bogus-but-undeclared case (my own `totally-not-a-step-inc1` test) the
`source.rs` path alone still catches it once W5 is fixed, so nothing is lost by leaving both in
place.

**(c) The direction and its edit surface.** Direction (iii), one of the four named candidates, not
outside them. Full edit surface given in its own section above and repeated in the Edit surface
section below.

**(d) The W6 disambiguation.** Stated at the top of this document: every unqualified "W6" here means
the waiver-note breakdown join only.

**(e) The sub-decision ruling.** `Q-<n>-<suffix>` ids are a legitimate, heavily used convention, not
dangling receipts. VERIFIED as above: 41 of the 63 distinct decision `q_id`s ever recorded are
`Q-55-<suffix>` forms, and zero bare `Q-<n>` receipts are genuinely unregistered. Any
dangling-receipt mechanism must resolve to the `Q-<n>` prefix before checking registration.

**(f) The scope of mechanisms (2) and (3).** Bounded by this pass, not designed by it. I have stated
the traps each must avoid (prefix-awareness for (2); the self-quoting and runtime-output escapes
for (3)) because those are cheap to state and expensive to discover after the fact, but neither
mechanism should be built now, so committing to their exact data structures or algorithms here would
be designing ahead of need.

**(g) The YAGNI boundary.** Given as its own section above.

**(h) The comment-coverage ruling.** Documentation defect, not a deliberate divergence and not
correct as it stands. The comment at `src/plan/source.rs:785-790` states the block's origin as
rules "moved from the round log's `check_record` waiver arm... and the W5 `reason`<->`evidence_tier`
pairing," and that is true of the presence rules and the pairing. The membership check at `:807`
(is this increment one of the step's own declared increments) is neither of those: `check_record`
has no access to a step's declared increments at all (the JSONL substrate has no
`[[step.increment]]` concept), so this is not a migrated rule with a `check_record` counterpart, it
is genuinely new validation the TOML substrate's structure makes possible for the first time. The
fix is a one-clause addition to the comment naming this third rule, not a design change.

## The edit surface

Code: `src/workflow.rs` only. `w5_problems`'s signature gains `rounds: &[Round]`; its body's
increment-ownership branch is replaced with a `BTreeMap` lookup built from the two existing
accessors `round_increment_id`/`round_step_slug`; its one call site inside `run_checks` passes the
`rounds` parameter `run_checks` already receives; 14 existing `#[cfg(test)]` call sites gain the
new argument (13 mechanically, 1 needs a synthesized `Round` fixture, both VERIFIED above). No
change to `plan::Step`, `plan::Question`, `Waiver` (either representation), `Round`, `render.rs`, or
any `*.plan.toml` schema file.

Drift-guarded prose: `pack/instrument.md:11`, `AGENTS.md:147`, and `.agents/AGENTS.reference.md:147`
all state the buggy lexical rule verbatim ("the increment's leading slug equals the step") and must
be edited together to describe the round-log join instead, or
`the_committed_scaffold_matches_a_fresh_render` (`src/agents_md_drift.rs:40-55`) fails.
Regeneration is `just scaffold-self`; its second line runs `nix fmt` over the whole tree, which this
repository's own state does not tolerate cleanly, so the resulting diff needs scoping back to the
three prose files and the code before it is committed.

Plan content, not code, and needed under every one of the four directions, not specific to (iii):
`docs/plans/agent-scaffold.plan.toml` gains two `[[step.increment]]` entries under
`workflow-enforcement-tier` (`workflow-enforcement-tier-fold` and `-endproperty-fold`, both
`risk_class = "risky"`), the two owed `[[step.waiver]]` entries per the ledger's drafted notes, and
the step's `status` flips to `complete`. VERIFIED this declaration alone, before any code change,
neither changes the rendered `.md` (`render --check --strict`: "up to date") nor fails
`validate --source`. No golden fixture or drift-guarded generated file is touched by this half of
the change, because `step.increments` is read only by `src/plan/source.rs` (confirmed:
`grep -n '\.increments\b' src/plan/render.rs` returns nothing).

## Limits of the recommendation, found while testing it

Direction (iii) is not free of residual risk, and I would rather name the one I found than let a
reviewer find it first. A waiver naming an increment that IS declared as a `[[step.increment]]` but
has NEVER been logged in the round log (zero round records for that id, ever) now passes W5's
ownership check silently, because the round-log join has nothing to compare against. VERIFIED: I
declared a fake `totally-not-a-step-inc1` increment under `workflow-enforcement-tier` (satisfying
`src/plan/source.rs`'s membership check) with a matching self-declared waiver and no round records
anywhere for that id; patched, `validate` exits 0 on it. Unpatched, the same fixture is (wrongly, for
the wrong reason, but still) flagged, because the old lexical check does not care whether the
increment was ever logged. This is a genuine, narrow trade: it requires a plan author to both
declare a never-reviewed increment and write a waiver for it, a deliberate double act rather than an
accident, and it does not weaken anything about the two waivers this fix exists to unblock (both
have five real round records each). I am recording it rather than treating it as disqualifying,
because the alternative (keeping the lexical check as an additional, redundant guard) would
reintroduce the false-fact bug for every case that is NOT this one, which is the far larger
population. If this residual case ever matters in practice, the cheap mitigation is for
`src/plan/source.rs`'s existing per-step loop to warn when a declared increment has zero round
records of its own, a check that loop does not currently make and that is outside this fix's scope
to add.
