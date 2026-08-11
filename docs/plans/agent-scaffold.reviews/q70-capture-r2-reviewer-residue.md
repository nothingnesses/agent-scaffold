# Q-70 capture, round 2, reviewer: FIX-INDUCED RESIDUE

Lens: what the round 1 fix pass (commit `129215d`, "docs: apply the round 1 remedies to Q-70") BROKE or LEFT HALF-DONE. Every new sentence is unreviewed content and was treated as such. The whole item was read against the text the pass did not change, because a half-fix is visible only there.

Artifact: `git diff main..HEAD` on `review/q70r2-residue`, commits `0a2e1e3`, `3a74e4e`, `129215d`. Binary: `target/debug/agent-scaffold` built from this worktree at HEAD. All measurement scripts ran read-only against the worktree; no fixture directory was needed and nothing outside the worktree was written or deleted.

Ledger discipline: I re-raise no finding settled in round 1. `R1B-3` (dismissed) and the four duplicates are untouched. Three of my four findings are about text that did not exist before `129215d`; the fourth is my ruling on a site the writer reported and the orchestrator routed to this round.

FOUR FINDINGS: one `high`, three `medium`. No `low` and no `critical`.

---

## R2A-1. The lettered list's new completeness guarantee is false, and it tells the reader not to check

Severity: `high`

Claim. Remedy C site 1 built the consolidated mandate list and asserted that it repeats every duty in the body. Remedy C site 2 wrote the same assertion into the opener. At least two duties stated in the body are not in the list, so a proposal that satisfies (a) to (g) in full, which the item declares sufficient, can still omit them.

The two assertions, both new in `129215d`:

- `docs/plans/agent-scaffold.plan.toml:1901`: "THIS LETTERED LIST IS THE COMPLETE MANDATE, and it is the only place in this item where the mandate is complete: every duty stated in the body above is repeated here, so a proposal that satisfies this list satisfies the item, and a proposal that satisfies only part of it is short whatever the body seemed to ask."
- `docs/plans/agent-scaffold.plan.toml:1883`: "The complete statement of what the pass must resolve is the lettered list in WHAT THE PASS OWES BACK at the end of this item; every duty in the body between here and there is repeated in it."

The two body duties that are not repeated, both at `docs/plans/agent-scaffold.plan.toml:1895`:

1. "Each proposal must state WHICH DIRECTION IT TAKES and WHETHER THAT DIRECTION IS ONE OF THE THREE NAMED ABOVE OR OUTSIDE THEM, **and must say what the other mechanism costs under that choice.**" The first half is carried by (c). The cross-pricing half is carried by nothing.
2. "the Markdown substrate declares no increments at all (`src/next.rs:520` and `:551`), so **this direction owes a ruling on what W5 does there rather than assuming a TOML plan**". Conditional on taking direction (i), and absent from (c), which asks only for "which source files it touches".

Reproduced by extracting the list and searching it:

```
$ sed -n '1901p' docs/plans/agent-scaffold.plan.toml \
    | sed 's/.*\((a) THE COUPLING RULING\)/\1/' > list.txt
$ for w in "cost" "other mechanism" "Markdown" "substrate"; do printf '%-18s -> ' "$w"; grep -ci "$w" list.txt; done
cost               -> 0
other mechanism    -> 0
Markdown           -> 0
substrate          -> 0
$ grep -o "([a-g]) [A-Z ]*" list.txt
(a) THE COUPLING RULING
(b) THE AUTHORITATIVE
(c) THE DIRECTION AND ITS EDIT SURFACE
(d) THE W
(e) THE SUB
(f) THE SCOPE OF MECHANISMS
(g) THE YAGNI BOUNDARY
```

The extraction runs from "(a) THE COUPLING RULING" to the end of the paragraph, so it covers every letter and the trailing edit-surface answer. Zero hits on all four terms.

Neither omission is covered under other words. (a) asks "whether W5's ownership check and the prospective W6 waiver-note join share a mechanism", which is a coupling verdict, not a price for the W6 join under a chosen W5 direction. (c) asks for a file list, not for a behavioural ruling about a second substrate.

Why the cross-pricing duty is the material one. `docs/plans/agent-scaffold.ledger.md:533`, the `Q-55-entryroute` decision record this item cites as its own authority, states the human's ground for a design pass rather than a planner: "the choice between a lookup against the step's declared increments and a rework of waiver-unit naming must be made with W6 in view". That IS the cross-pricing duty. The list that claims to be the complete mandate drops the deliverable the decision to run a pass was made to obtain.

Impact if left unfixed. This reproduces `R1B-2`'s failure mode with an added suppression clause. `R1B-2` was `high` because an explorer working to the deliverables paragraph could ship a compliant proposal that omitted four rulings. The gap is now two rather than four, but the paragraph now carries an explicit guarantee that satisfying it satisfies the item, so the explorer has been told, in the item's own words, that re-reading the body for further duties is unnecessary. The check that would have caught the omission is exactly the check the new sentence retires.

A secondary inconsistency introduced by the same pair of edits, recorded here rather than as its own finding because one fix closes both. The opener at `:1883` now states the mandate as "THE W5 FIX PLUS ALL THREE DETECTION MECHANISMS", and says that taking the narrower reading "under-scopes a proposal by more than half". Letter (f) at `:1901` then says of mechanisms 2 and 3 that "This item has deliberately never said either way" whether they are DESIGNED or only BOUNDED. That was true of the item before `129215d`; the opener written in the same pass is a statement about their place in the mandate, and (f) permits a proposal to merely bound them, which under the opener's framing is close to the under-scoping the opener forbids. The two sentences were authored together and cannot both stand as written.

---

## R2A-2. The fix pass introduced one new count of a moving population, and it is wrong

Severity: `medium`

Claim. Remedy E site 1 rewrote the `agent-scaffold next` count passage and replaced a vague attribution with a precise, enumerable claim about the ledger. The enumeration is short by one, the date qualifier attached to it does not cover the missing member, and the missing member is the human decision the same sentence cites as its routing authority.

The new text, `docs/plans/agent-scaffold.plan.toml:1899`: "(b) The `agent-scaffold next` defects routed here by the human decision of 2026-07-30. THE DURABLE RECORD SAYS BOTH THREE AND FOUR, and FOUR is the measured count. It says THREE in **two live passages, both dated 2026-08-11**: the `Q-55-entryroute` decision record ... and the ledger's current next-action paragraph, item (4) ... THOSE **TWO** LIVE 'THREE'S ARE OWED A CORRECTION, which is not this item's to make and is recorded here so it is not lost."

There is a third live passage, and it is dated 2026-07-30:

```
$ grep -niE "three (\`?agent-scaffold )?\`?next\`? defects" docs/plans/agent-scaffold.ledger.md
533: ... the two inc3 defects plus the three `next` defects ...            (the Q-55-entryroute record, cited by the item)
571: ... (4) the three `agent-scaffold next` defects routed here ...       (the next-action paragraph, cited by the item)
1337:HUMAN DECIDED (2026-07-30) where the three `agent-scaffold next` defects go: FOLD THEM INTO GATE 4, the
     validation-constraints step, rather than giving them their own step ...                (NOT cited by the item)

$ git blame -L 1337,1337 --date=short HEAD -- docs/plans/agent-scaffold.ledger.md
90b92b2d (nothingnesses 2026-07-30 1337) HUMAN DECIDED (2026-07-30) where the three `agent-scaffold next` defects go: ...
```

So three things are wrong in one sentence: the population is three passages, not two; "both dated 2026-08-11" does not describe it, because the third is dated 2026-07-30; and the owed-correction record the item creates "so it is not lost" loses the one that matters most, since `:1337` is the human decision of 2026-07-30 that the same sentence names as the routing authority. Line `:533` and `:571` blame to 2026-08-11 as stated (`903b70b8` and `8fa56939`), so the two the item does list are correct.

Round 1's own reviewer opened that line and did not read it. `q70-capture-reviewer-premises.md` records "The human decision routing the `next` defects, at `:1323`, is dated 2026-07-30. Confirmed." That is the same passage at its pre-append line number, so remedy E was written from an enumeration of two because the reviewer that supplied it counted two.

Impact if left unfixed. The class is remedy A's own: this is the ONLY new count of a moving thing the fix pass introduced, written in the same commit that stripped three other counts on the ground that a count expires, and the ledger is by definition a file the orchestrator appends to during the loop, which is the exact condition the project's standing cure names. The consequence is bounded, as `R1A-4`'s was: the count the item states for the defects themselves (four) is right, so nothing downstream is mis-scoped. What is lost is the correction owed against the human's own decision record, and the item's stated purpose for the sentence, "recorded here so it is not lost", is falsified for that member.

---

## R2A-3. A new claim about the declared-increment set contradicts the live data it is presented as measuring

Severity: `medium`

Claim. Remedy A site 3 added a causal account of what the `[[step.increment]]` set is, presented as "a measured input". It was not measured. The dominant case in the plan is the opposite of what it says.

The new text, `docs/plans/agent-scaffold.plan.toml:1895`, direction (i): "WHAT THAT SET ACTUALLY IS, **recorded as a measured input** and not as a ratio: it is not a model of the plan's increments. **It is a by-product of the membership rule at `src/plan/source.rs:807`, so a step tends to declare an increment when a waiver needs one and not otherwise**; `complete` steps exist that declare none at all while their round records carry increment ids; and an increment id may not contain an uppercase byte ..."

Measured over the live plan:

```
$ python3 (tomllib over docs/plans/agent-scaffold.plan.toml)
declared increments: 45
increment-unit waivers: 13
step-unit waivers: 12
declared ids that are ALSO waived: 13
declared ids with NO waiver: 32
steps declaring increments: 32   steps with increment waivers: 10
steps declaring but never waiving: 22
```

32 of the 45 declared increment ids are named by no waiver at all, and 22 of the 32 steps that declare increments never waive one. The membership rule at `src/plan/source.rs:807` cannot be why those 32 exist, because no waiver reaches them. The set is largely maintained independently of waivers, which is the reverse of what the sentence tells the pass.

The adjacent clause in the same sentence is TRUE, which is why this is a half-fix rather than a wrong paragraph. `complete` steps that declare zero increments while their round records carry increment ids do exist, and there are two:

```
complete steps declaring ZERO increments whose rounds carry increment ids: 2
   ('state-schema',   ['state-schema-inc1', 'state-schema-inc2', 'state-schema-inc3'])
   ('round-log-core', ['round-log-core-incA', 'round-log-core-incB'])
```

The uppercase exclusion in the same sentence is also true and is settled by citation: `is_kebab_case_token` at `src/plan/source.rs:475-477` rejects any uppercase byte, and `an_uppercase_increment_id_is_flagged` at `src/plan/source.rs:1221` pins it.

The provenance of the error is visible. `R1C-1`'s evidence supports the narrow observation that `optional-modules` declares exactly one increment and it is exactly the one it waives. The remedy generalised that one step into a claim about how the set comes to exist, and the generalisation does not survive the plan.

Impact if left unfixed. The sentence frames direction (i) at the point a proposal weighs it, and it frames the key that direction would use as an artefact of the waiver machinery rather than as a maintained declaration. Measurement makes direction (i)'s key look substantially better than the item paints it, which biases the comparison the pass exists to make. It stays `medium` rather than `high` because the item supplies the comparison command in the next sentence ("Compare the two sets rather than carrying a coverage figure from here"), so a pass that measures anything reaches this, and because the other clauses in the sentence hold.

---

## R2A-4. "roughly eleven `src/checks.rs` citations" is measured at fifteen, in the one paragraph that twice refuses to state a count

Severity: `medium`

This is my ruling on the first of the three sites the writer reported as reached by remedy A's class and deliberately left alone. The writer's ground was that a hedged and attributed figure is not a durable count. The ground is correct about durability and beside the point: the figure is not merely undurable, it is wrong today by measurement, and nobody in round 1 counted it. `q70-capture-reviewer-premises.md` records the gap in terms: "'the roughly eleven `src/checks.rs` citations'. I confirmed the ledger says 'about eleven'; I did not count the citations ... myself."

The claim, `docs/plans/agent-scaffold.plan.toml:1893`, mechanism (3): "A QUOTATION RESOLVER, automating what acceptance check 21 already instructs, with the recorded caveat that it would immediately go red on the **roughly eleven** `src/checks.rs` citations `Q-55-check21b` deliberately left stale."

The population lives in `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md`, which `Q-55-check21b` (`docs/plans/agent-scaffold.ledger.md:613`) names and whose decision text reads "over re-pointing all eleven" and "all eleven belong to the owning step TOGETHER rather than ten being stale and one arbitrarily current".

Measured. The document carries 21 full-form `src/checks.rs:<line>` citations, 15 of them distinct, plus four more written in bare `:N-M` form at line 94 (`:72-77`, `:400-402`, `:845-847`, `:789-790`). Every one of the 15 distinct full-form citations was opened and checked for the token the document says lives there:

```
src/checks.rs:78         expects RUNNER_PREFIX          -> STALE
src/checks.rs:329-342    expects remove_dir_all         -> STALE
src/checks.rs:388-392    expects libc                   -> STALE
src/checks.rs:400-405    expects owning_pid             -> STALE
src/checks.rs:407-461    expects prune_orphan_worktrees -> STALE
src/checks.rs:425-428    expects pid_is_alive           -> STALE
src/checks.rs:791-792    expects RUNNER_PREFIX          -> STALE
src/checks.rs:795-800    expects WorktreeSetup          -> STALE
src/checks.rs:845-847    expects uniqueness             -> STALE
src/checks.rs:848-852    expects UNIX_EPOCH             -> STALE
src/checks.rs:862-871    expects scratch                -> STALE
src/checks.rs:1438-1442  expects u32::MAX               -> STALE
src/checks.rs:1462       expects RUNNER_PREFIX          -> STALE
src/checks.rs:1491       expects std::process::id()     -> STALE
src/checks.rs:1492       expects dead_pid               -> STALE

STALE: 15 of 15 distinct citations
```

Corroborated by locating every symbol the document names by line:

```
$ grep -n "RUNNER_PREFIX\|fn nanos\|fn owning_pid\|fn dead_pid\|WorktreeGuard\|fn pid_is_alive\|fn prune" src/checks.rs
98:   const RUNNER_PREFIX          (cited at :78 and :791-792 and :1462)
345:  struct WorktreeGuard         (cited at :329-342)
416:  fn pid_is_alive              (cited at :425-428)
561:  fn owning_pid                (cited at :400-405 and :400-402)
588:  fn prune_orphan_worktrees    (cited at :407-461)
1023: fn nanos                     (cited at :845-847 and :848-852)
1613: fn dead_pid                  (cited at :1438-1442)
```

So the resolver's immediate red-list is at least 15 distinct citations, or 21 occurrences, or 25 counting the bare form. "Roughly eleven" understates it by a third to a half, and the hedge "roughly" does not stretch that far.

Why this is in scope rather than inherited. The stale citations themselves are pre-existing and out of scope, and I raise nothing against that document. What is in the diff is `Q-70`'s own assertion of a figure. The precedent is `R1A-1`, ruled valid on exactly this ground: a relayed figure that measurement contradicts at the moment the item is written. `src/checks.rs` last changed on 2026-07-31 (`09a027c`), a week before `Q-55-check21b` was decided on 2026-08-08, so "eleven" was already wrong when the decision recorded it and was still wrong when `Q-70` relayed it.

Impact if left unfixed. Mechanism (3) is one of the three the pass must scope, and the item hands it a red-list two thirds of its real size. Severity matches `R1A-1`'s confirmed `medium` for the same reason the triage gave there: the paragraph refuses to state a count twice in its own text ("NO COUNT OF THOSE SITES IS STATED" for mechanism 1, "NO COUNT IS STATED HERE, DELIBERATELY" for mechanism 2) and then states one, so the project's standing cure is self-contradicted inside one paragraph. The direction of the error is conservative, which is why it is not higher.

---

# Verdicts on the three writer-reported sites

## Site 1, "roughly eleven `src/checks.rs` citations": RAISE. See `R2A-4`.

The writer's stated ground, that a hedged and attributed figure is not a durable count, is true and does not settle it. The site needed changing for a reason the writer was not testing for: the figure is wrong now, by 15 to 11 on distinct citations. This is the same shape the round 1 triage recorded for the writer's self-raised item, where "declining to choose was the right call on the wrong question".

## Site 2, "the token `W6` occurs exactly once ... outside this item": LEAVE. Not a defect.

Verified true, and true in the strict sense the sentence asserts:

```
$ grep -n "W6" docs/plans/agent-scaffold.plan.toml
1774   (Q-59's ask)
1883, 1893, 1895, 1897, 1901   (all inside Q-70)
$ sed -n '1774p' docs/plans/agent-scaffold.plan.toml | grep -o "W6" | wc -l
1
```

One line outside the item, one occurrence on it, and it is `Q-59`'s. Two grounds for leaving it, and the writer's is only the weaker of them.

First, it is hedged in the form the round 1 triage already ruled sufficient. The triage's remedy A left `plan.toml:1899`'s `blocked_by` re-measurement alone with the verdict "it is stated as a measurement made for this registration rather than as a durable count", and this sentence opens with the identical framing, "Measured for this registration". Applying the same rule to the same form is consistency, not residue.

Second, and this is what actually settles it, the load-bearing claim in that paragraph is not the count. It is "'W6' ALREADY NAMES TWO DISTINCT UNBUILT CHECKS in the durable record", and that is an enumeration of two named checks, one in this item's scope and one belonging to `Q-59`. A third meaning for `W6` would have to be invented by a future author, not accreted by the loop the way waiver notes and decision receipts are, so the population does not have the growth property that makes remedy A's class bite. The count sentence is corroboration for the enumeration, not the claim.

I checked the wider durable record for a third check named `W6` and found none: the ledger's four `W6` lines (`:533`, `:535`, `:571`, `:635`) all refer to the waiver-note breakdown join, which is the first of the two.

## Site 3, "they continue the established `-w1` to `-w4` waiver-id sequence this step already carries": LEAVE. Not a defect.

Verified. All four ids exist on the `workflow-enforcement-tier` step:

```
docs/plans/agent-scaffold.plan.toml:1325   id = "workflow-enforcement-tier-w1"
docs/plans/agent-scaffold.plan.toml:1334   id = "workflow-enforcement-tier-w2"
docs/plans/agent-scaffold.plan.toml:1343   id = "workflow-enforcement-tier-w4"
docs/plans/agent-scaffold.plan.toml:1352   id = "workflow-enforcement-tier-w3"
```

This is not a count of a population at all, and the writer's durability ground is not needed. The sentence asserts that a naming convention exists and that the two owed ids continue it, and it names the range by its endpoints. Every id in the range is pinned in the plan under the step whose records the paragraph is about, so the statement is checkable in one grep and cannot expire while those waivers stand. Adding a `-w5` and `-w6` extends the sequence rather than falsifying the claim that it is established.

---

# Fix-induced residue looked for and NOT found

Recorded so the class is visibly covered rather than silently skipped, since two clean rounds are what this artifact needs.

- EVERY new command the fix pass added reproduces exactly and returns the set it claims. The blocker-population pipeline at `:1885` returns the six identities across three steps; the waiver-note grep at `:1893` returns exactly the four `note` lines and no other. I also checked that the grep misses nothing: only 5 `note` lines exist in the plan and only 4 carry a parenthetical breakdown, so there is no single-round `(3)` form the `{1,6}` repetition would skip.
- EVERY new source citation the fix pass added resolves at the line. Checked individually: `src/plan.rs:55-60`, `src/workflow.rs:64-68`, `:206-221`, `:237-267`, `:445-447`, `:498-502`, `:549`, `:564`, `src/plan/source.rs:279-300`, `:422-430`, `:475-477`, `:791-793`, `:807-811`, `:854-856`, `src/main.rs:582-585`, `src/next.rs:517-523`, `:520`, `:551`, `src/metrics.rs:539-601`, `src/agents_md_drift.rs:41-55`. The remedy E site 2 correction is right: the per-step waiver loop does close at `:856`, so `:791-856` covers the pairing check the same paragraph attributes to the block.
- The new claim that the breakdown convention is "carried in the `note` field of `[[step.waiver]]` entries" is structurally true: `note: Option<String>` is a field of the TOML `Waiver` struct at `src/plan/source.rs:297-300`, and all four breakdown sites are `note` lines on `[[step.waiver]]` entries of `workflow-enforcement-tier`.
- The new edit-surface answer holds end to end. The W5 clause is present at `AGENTS.md:147`, `.agents/AGENTS.reference.md:147` and `pack/instrument.md:11`; `the_committed_scaffold_matches_a_fresh_render` exists at `src/agents_md_drift.rs:375`; `just scaffold-self`'s second line is `nix fmt`; `waiver_note` is at `src/plan/render.rs:516`; the commented `[[step.waiver]]` example is at `pack/plan-template.plan.toml:44`. I did not re-run `R1C-5`'s drift mutation, because the triage already reproduced it and a citation settles the claim.
- The new dates in the rewritten out-of-scope paragraph are right. `git blame` gives 2026-08-11 for both `:533` and `:571`, and 2026-08-01 for `:981`, matching the item.
- Every quoted-text handle the item substitutes for a line number resolves in the ledger: "TWO WAIVERS ARE OWED AND CANNOT YET BE WRITTEN" (1 hit), "teaching W5 the structured step association W3 already uses" (2), "THREE DEFECTS IN `agent-scaffold next`" (1), "A FOURTH `agent-scaffold next` DEFECT" (2), and both quoted "three" passages (1 each). This mattered: the ledger has grown since round 1 and the line numbers the triage used have all moved, while every quoted handle still lands.
- Four is still the right count of `next` defects. No fifth is recorded anywhere in the ledger, and the `blocked_by` re-measurement reproduces (95 steps, all `[]`, none populated).
- No rewritten paragraph contradicts an unedited one. The naming-collision paragraph and "WHAT IS BLOCKED ON IT" were not touched, and both remain consistent with the rewritten blocker and second-path paragraphs. The one internal inconsistency I did find is between two paragraphs the SAME pass rewrote, and it is recorded under `R2A-1`.
- The binary-choice defect (`R1C-3`, `R1B-4`) is genuinely closed rather than half-closed. The three directions are labelled "CANDIDATE DIRECTIONS for the pass to weigh, extend, or discard. NOT a decided option set, NOT exhaustive", direction (iii) is named and marked "THIS IS RECORDED, NOT RECOMMENDED", and no residual binary phrasing survives elsewhere in the item.
- Mechanical state, checked independently: `render docs/plans/agent-scaffold.plan.toml --check` reports "up to date" at exit 0; `validate --source ... --workflow` reports 309 records valid, 95 steps and 70 questions valid, "workflow invariants hold" at exit 0; all three changed files return 0 under `LC_ALL=C grep -cP '[^\t\x20-\x7e]'`; the `Q-70.md` sidecar is 0 bytes, as all 70 question sidecars are.
- The triage's observation that `q70-capture` was the only declared orphan task with no round record is now moot: the log carries one `type:"round"` record for `q70-capture` (round 1 of this loop), which is why the record count moved from 308 to 309.

---

# Source-code observations, NOT findings against this artifact

None. Nothing in `src/` was found defective in the course of this review. The two source observations routed by the round 1 source lens are unchanged and are not restated here.

---

# What was settled by running something, and what by reading

Run: `R2A-1`'s list extraction and term search; `R2A-2`'s ledger grep and three `git blame`s; `R2A-3`'s two `tomllib` plus JSONL measurements; `R2A-4`'s per-citation staleness check, the symbol-location grep and the `git log` on `src/checks.rs`; both of the item's own new reproduction commands; the waiver-note completeness check; the `W6` occurrence counts; the four waiver-id greps; `render --check`, `validate --source ... --workflow` and the ASCII checks.

Read: every source citation the fix pass added, which are structural claims a citation settles; the ledger's quoted-text handles, verified by exact-string grep rather than by reading around them; the six paragraphs of round 1 findings and verdicts I had to hold to avoid a re-raise.
