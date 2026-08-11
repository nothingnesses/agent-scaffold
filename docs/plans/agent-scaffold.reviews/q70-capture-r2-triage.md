# Q-70 capture, round 2 triage

Triager verdicts on the eleven raw findings in `q70-capture-r2-reviewer-residue.md` (`R2A-*`), `q70-capture-r2-reviewer-sources.md` (`R2B-*`) and `q70-capture-r2-reviewer-surfaces.md` (`R2C-*`).

Artifact: `git diff main..HEAD` on `triage/q70-r2`, commits `96b459c`, `d68abca` and `1284fbf` (the same three commits the reviewers reviewed as `0a2e1e3`, `3a74e4e`, `129215d`; the branch was rebased between the spawns and the content is identical). The change adds the `[[question]] Q-70` entry, the `q70-capture` `[meta].orphan_tasks` token, an empty `docs/plans/agent-scaffold.questions/Q-70.md`, and the regenerated `docs/plans/agent-scaffold.md`.

Binary: `target/debug/agent-scaffold` built from this worktree at HEAD. Fixture root: `<scratch>/tri-q70-r2/`. Nothing outside that directory was written or deleted; no fixture was created at mode 000 or 600.

RESULT: NINE VALID, ONE DUPLICATE, ONE DISMISSED. Severity ceiling `high`, two of them.

BOTH `high` FINDINGS ARE VALID. No finding at `high` or above is dismissed, so NO BACKSTOP RE-CHECK IS OWED for this round. The one dismissal (`R2C-1`) the reviewer rated `medium` and I rate `low` had it been valid, which is below the backstop severity either way.

Two severities are corrected DOWNWARD (`R2A-2` `medium` -> `low`, `R2B-4` `medium` -> `low`). A downgrade is not a dismissal and engages no backstop: both findings stand as valid.

---

## Verdict table

| id | verdict | reviewer severity | my severity | settled by |
| --- | --- | --- | --- | --- |
| R2A-1 | VALID | high | high | running |
| R2A-2 | VALID | medium | low | running |
| R2A-3 | VALID | medium | medium | running |
| R2A-4 | VALID | medium | medium | running |
| R2B-1 | VALID | high | high | running plus reading |
| R2B-2 | VALID | medium | medium | running |
| R2B-3 | DUPLICATE OF R2A-1 | medium | (see R2A-1) | running |
| R2B-4 | VALID | medium | low | running |
| R2B-5 | VALID | low | low | running |
| R2B-6 | VALID | low | low | running |
| R2C-1 | DISMISSED | medium | low if valid | running |

---

## Per-finding verdicts

### R2A-1. The lettered list's new completeness guarantee is false, and it tells the reader not to check

VERDICT: VALID. SEVERITY: `high`, confirmed. Primary of the duplicate group with `R2B-3`.

Reproduced, and then extended by a systematic audit the reviewer did not run.

The reviewer's extraction reproduces byte for byte. Extracting the lettered list from `docs/plans/agent-scaffold.plan.toml:1901` and searching it returns zero hits on `cost`, `other mechanism`, `Markdown` and `substrate`, and the letters are exactly `(a)` to `(g)`. Both assertions are present as quoted, at `:1901` ("THIS LETTERED LIST IS THE COMPLETE MANDATE ... so a proposal that satisfies this list satisfies the item") and at `:1883` ("The complete statement of what the pass must resolve is the lettered list ... every duty in the body between here and there is repeated in it"). Both are new in the fix commit; the word-diff of `1284fbf` carries both sentences in full as additions.

I then measured the whole gap rather than checking the reviewer's two instances. I extracted every duty sentence in `:1883` to `:1903` and matched each against the list. Fourteen duty sentences, of which EXACTLY THREE are not carried by the list:

1. `:1895`, "and must say what the other mechanism costs under that choice", the second half of a sentence whose first half is `(c)`.
2. `:1895`, "so this direction owes a ruling on what W5 does there rather than assuming a TOML plan", conditional on direction (i).
3. `:1889`, "Whether that is a documentation defect, a deliberate design divergence, or correct as it stands is THE PASS'S RULING TO MAKE", which is `R2B-3`'s site.

`R2A-1` names the first two, `R2B-3` names the third. NEITHER REVIEWER STATES THE SET, and the remedy below carries all three.

The material duty is the first. `docs/plans/agent-scaffold.ledger.md:533`, the `Q-55-entryroute` record the item cites as its own authority, gives the human's ground for a design pass rather than a planner as "the choice between a lookup against the step's declared increments and a rework of waiver-unit naming must be made with W6 in view". That IS the cross-pricing duty, and the list that declares itself complete drops it. Verified at the line.

`high` confirmed. `R1B-2` was `high` because an explorer working to the deliverables paragraph could ship a compliant proposal that omitted four rulings. The gap is now three rather than four, but the paragraph has acquired an explicit guarantee that satisfying it satisfies the item, so the reader is told in the item's own words that re-reading the body is unnecessary. The check that would have caught the omission is the check the new sentence retires, and the omitted deliverable is the one the decision to run a pass was made to obtain. The absolute cost is a human deciding on a comparison that was never priced, folded into a step, which is the hard-to-reverse end of this artifact's risk.

THE SECONDARY INCONSISTENCY THE REVIEWER RECORDED IS NOT A DEFECT, and I rule it here so the fix pass does not chase it. The opener's "THE W5 FIX PLUS ALL THREE DETECTION MECHANISMS" and letter (f)'s "This item has deliberately never said either way" answer different questions. Being inside the mandate and being DESIGNED rather than BOUNDED are not the same claim: a proposal that bounds mechanisms 2 and 3 has engaged them and is not the narrow W5-only reading the opener forbids. Both sentences stand.

### R2A-2. The fix pass introduced one new count of a moving population, and it is wrong

VERDICT: VALID. SEVERITY: `low`, CORRECTED DOWN from `medium`.

Reproduced, AND THE FINDING'S OWN COUNT DOES NOT HOLD, in the same direction as the defect it reports.

The item at `:1899` says the ledger "says THREE in two live passages, both dated 2026-08-11". The reviewer reports a third at `:1337`. Measured:

```
$ grep -noiE "three (\`?agent-scaffold )?\`?next\`? defects" docs/plans/agent-scaffold.ledger.md
533:three `next` defects
571:three `agent-scaffold next` defects
1055:three `agent-scaffold next` defects
1259:three `agent-scaffold next` defects
1261:three `agent-scaffold next` defects
1337:three `agent-scaffold next` defects
```

SIX passages, not two and not three. `git blame` dates them to 2026-08-11 (`903b70b8`), 2026-08-11 (`8fa56939`), 2026-08-02 (`f9d589be`), 2026-07-31 (`12d6a01a`, twice) and 2026-07-30 (`90b92b2d`). So the item's enumeration is short by four, its "both dated 2026-08-11" is false as a description of the population, and the reviewer's own "three passages, not two" is short by three. The reviewer's displayed grep output omits `:1055`, `:1259` and `:1261` without saying why; the command as written returns all six.

The reviewer's material point survives all of that and is what makes the finding valid: `:1337` is the human decision of 2026-07-30 that the SAME SENTENCE names as the routing authority ("routed here by the human decision of 2026-07-30"), and the item's stated purpose for the passage, "recorded here so it is not lost", loses it.

SEVERITY CORRECTED DOWN to `low`, on this loop's own precedent. `R1A-4` is the same site, the same class and the same bounded consequence, and round 1 ruled it `low` with the reasoning "The count `Q-70` states is the correct one, so nothing downstream is mis-scoped; what is false is the claim about where the wrong count lives". That is still exactly the position: FOUR is still the measured count of `agent-scaffold next` defects, no step is mis-scoped, and what under-delivers is an owed-correction record. The fix pass moved this defect from vague to precise-and-wrong without changing its impact, so the severity should not move either.

### R2A-3. A new claim about the declared-increment set contradicts the live data it is presented as measuring

VERDICT: VALID. SEVERITY: `medium`, confirmed.

Reproduced independently with my own `tomllib` pass over the live plan, not the reviewer's script:

```
declared increments: 45 distinct: 45
increment-unit waivers: 13      step-unit waivers: 12
declared ids ALSO waived: 13    declared ids with NO waiver: 32
steps declaring increments: 32  steps with increment waivers: 10
steps declaring but never waiving: 22
```

Every figure matches the reviewer's. The item's sentence at `:1895` says the set "is a by-product of the membership rule at `src/plan/source.rs:807`, so a step tends to declare an increment when a waiver needs one and not otherwise". 32 of the 45 declared ids are named by no waiver at all and 22 of the 32 declaring steps never waive one, so the membership rule cannot be why the dominant case exists. The claim is presented as "recorded as a measured input", which is the strongest authority label the item uses, and it is the one clause in the sentence that was not measured.

The adjacent clauses hold and I verified both rather than taking them: two `complete` steps declare zero increments while their rounds carry increment ids distinct from the step slug (`state-schema` with `-inc1/-inc2/-inc3`, `round-log-core` with `-incA/-incB`), and `is_kebab_case_token` at `src/plan/source.rs:477` rejects any ASCII uppercase byte, with the doc comment at `:475-476` naming `round-log-core-incA` as the excluded form.

`medium` confirmed. The sentence frames direction (i) at the point a proposal weighs it, and it frames that direction's key as an artefact of the waiver machinery rather than as a maintained declaration, which biases the comparison the pass exists to make. It is not higher because the next sentence tells the reader to compare the two sets with a command, so a pass that measures anything reaches the truth. It is not lower because a false causal claim carrying a "measured" label is worse than an omission: it discourages the measurement.

### R2A-4. "roughly eleven `src/checks.rs` citations" is measured at fifteen

VERDICT: VALID. SEVERITY: `medium`, confirmed.

Reproduced, and I rebuilt the staleness check rather than accepting the reviewer's table. In `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md`: 21 full-form `src/checks.rs:<line>` citations, 15 distinct. I extracted each distinct citation with its surrounding sentence and printed the actual lines it points at in the current `src/checks.rs`. Every one resolves to unrelated content. Examples, each checked at the line:

- `:78` is cited as `RUNNER_PREFIX`; line 78 is `PathBuf,` inside a `use` list, and `RUNNER_PREFIX` is at `:98`.
- `:329-342` is cited as the `WorktreeGuard` cleanup; those lines are `impl From<io::Error> for RunError` and the start of a doc comment, and `struct WorktreeGuard` is at `:345`.
- `:388-392` is cited as the dependency-discipline statement about libc; those lines are `git_command()`, and the libc sentence is at `:411-415`.
- `:400-405` is cited as `owning_pid`; those lines are `git_output`'s signature and body, and `fn owning_pid` is at `:561`.
- `:407-461` is cited as the startup prune; `fn prune_orphan_worktrees` is at `:588`.
- `:845-847` and `:848-852` are cited as `nanos()` and its doc comment; `fn nanos` is at `:1023`.

The ledger's own figure is "about eleven `src/checks.rs` citations there are stale" (`docs/plans/agent-scaffold.ledger.md:613`, the `Q-55-check21b` record), so the item relays it faithfully and the relay is wrong today by 15 to 11 on distinct citations, or 21 to 11 on occurrences. `src/checks.rs` last changed at `09a027c` (2026-07-31), before `Q-55-check21b` was decided on 2026-08-08, so the figure was already wrong when the decision recorded it.

IN SCOPE, and I checked the out-of-scope precedent rather than assuming. Condition 1 fails: the claim under review is `Q-70`'s own assertion, authored in `96b459c` inside the reviewed range (`git log -S "roughly eleven"` returns that commit and no other). Condition 2 fails for the same reason. The stale citations themselves are pre-existing and I raise nothing against them.

`medium` confirmed, on `R1A-1`'s precedent, which is the same class ruled at the same level: a relayed figure that measurement contradicts at the moment the item is written, inside a paragraph that twice refuses to state a count on the ground that a moving count must not be asserted. The direction of the error is conservative, which is what keeps it off `high`.

### R2B-1. The item omits a receipted human decision that queues a third change to the same record schema and the same join

VERDICT: VALID. SEVERITY: `high`, confirmed.

Every source reproduces. The receipt is real and reads exactly as quoted:

```
$ jq -c 'select(.q_id=="Q-55-mechanism")' docs/metrics/workflow.jsonl
{"type":"decision","task":"workflow-enforcement-tier","q_id":"Q-55-mechanism","options":["Anchor plus refusal, identity queued","Anchor, refusal, and identity fields now","Anchor only, minimal"],"recommendation":"Anchor plus refusal, identity queued","chosen":"Anchor plus refusal, identity queued","ts":"2026-07-31"}
```

The ledger paragraph at `:1081` carries the human's recorded reasoning verbatim as quoted, including "pulling identity in would edit the record schema that W3, W4 and W5 all read, which the calibration close already argued should be ONE deliberate edit rather than a rider on a path fix". The paragraph at `:1073` describes the queued work as "an optional `project` field on `Round` and on the plan's `[meta]`, filtering the join in `check_workflow_toml`". The work is unbuilt: `Round` at `src/metrics.rs:620-655` carries `line`, `task`, `artifact`, `outcome`, `consecutive_clean`, `risk_class`, `step` and `increment`, and no `project`.

The omission is total, measured over the whole `Q-70` entry (`sed -n '1880,1904p'`): zero occurrences of `Q-55-mechanism`, `Q-55-resumecost`, `project_root_of_source`, and the three occurrences of "project" are "this project's own", "project identically" and "project root".

THE RULING THE ORCHESTRATOR LEFT TO ME, whether `Q-70` is at fault for faithfully summarising a ledger whose own "It now carries FOUR things" is undercounted. IT IS AT FAULT, on three grounds, and the ledger's separate correction does not discharge it.

First, the item does not present itself as a relay. Its own paragraphs are headed "re-measured against the source rather than relayed", "MEASURED IN SCRATCH RATHER THAN READ", "Re-measured for this registration" and "each was re-verified for this registration". An item that sets first-hand re-measurement as its standard for the blocker, the escape routes, the round records, the `W6` count and the `blocked_by` field does not get to inherit an inventory unchecked, and the queued work is discoverable in the same ledger by the same reading.

Second, the item makes completeness claims about exactly this content. "THIS LETTERED LIST IS THE COMPLETE MANDATE" and "recorded as deferred inputs so nobody loses them" are promises about the inventory, and this project has already decided that class: `Q-55-impactclaim` (2026-08-10, receipt record 303) ruled that "A DOCUMENTATION-IMPACT LIST THAT ENUMERATES ITS OWN EXCLUSIONS IS A COMPLETENESS CLAIM" and chose to DROP the claim rather than maintain it.

Third, the consequence is not bookkeeping. The item asks the pass to rule whether W5's ownership check and the prospective W6 join share a mechanism, prices direction (ii) as a rework reaching the JSONL `type:"waiver"` arm of `check_record`, and names direction (iii) as an ownership rule read out of `run_checks`'s `rounds`. The queued project-identity edit adds a field to that same record schema and filters that same `check_workflow_toml` join, and the human has already recorded a constraint on it ("ONE deliberate edit rather than a rider"). A proposal can satisfy every letter of the mandate, choose direction (ii) or (iii), and never learn that a receipted decision constrains the schema it proposes to change.

`high` confirmed. This is the same class as round 1's `R1C-2` and `R1C-3`, both `high`: it changes what the pass is told to consider, on the one axis the human said the pass exists for. It is not `critical` because nothing is asserted falsely and the receipt remains findable in the log.

NOT A RE-RAISE. `R1B-1` is about the opening `ask` under-framing the mandate `Q-55-entryroute` decided; `R1C-3` is about a third DIRECTION for the W5 fix. This is a third BODY OF WORK, decided by a different receipt seven weeks earlier, that the item never registers. I checked all four round 1 files for the token `Q-55-mechanism` and for "project identity" and found neither.

### R2B-2. "Recorded as deferred inputs so nobody loses them" is a completeness claim the list does not keep

VERDICT: VALID. SEVERITY: `medium`, confirmed.

Reproduced. The `Q-55-resumecost` receipt exists and reads as quoted (2026-08-02, `chosen` "Accept as (iv), queue the shared cause"), and the ledger paragraph at `:863` queues the shared root cause explicitly: "costs (iii) and (iv) share ONE root cause, `src/main.rs:project_root_of_source`'s fallback ... and treating it ONCE in the validation-constraints step is better than accumulating a fresh accepted cost on every new surface ... This joins the PROJECT-IDENTITY work already queued to that step." The fourth item reproduces too: the ledger at `:893` records that the ledger half of the `run_next` false green "BELONGS TO NEITHER" queued item and "currently has NO OWNER anywhere in the plan". The `Q-55-impactclaim` precedent the reviewer cites is at `:651` and reads exactly as quoted.

DISTINCT FROM `R2B-1`, and I checked rather than accepted the reviewer's assertion. `R2B-1`'s site is the coupling paragraph at `:1895` and its subject is what the PASS must weigh; this finding's site is the out-of-scope paragraph at `:1899` and its subject is what the eventual STEP carries. The two fixes are additive: registering project identity as a coupling participant does not add the `project_root_of_source` root cause to the deferred inventory, and completing the inventory does not tell the pass that a receipt constrains the schema. Different sentences, different remedies.

`medium` confirmed. The paragraph is the only inventory of the eventual step's inputs that lives in the plan rather than in the ledger, and it states its own purpose as preservation. The cost lands later, when a planner authors `validation-constraints` from an inventory that is short by two receipted entries and silent on the one item the record says nobody owns. It is not `high` because the receipts and the ledger remain the durable authority and a planner authoring that step reads both.

### R2B-3. The item declares its lettered list the COMPLETE MANDATE and its own body states a ruling the list does not carry

VERDICT: DUPLICATE OF `R2A-1`.

The claim is true and I verified it at the line. `:1889` states "Whether that is a documentation defect, a deliberate design divergence, or correct as it stands is THE PASS'S RULING TO MAKE and a reviewer's to raise; nothing here calls it either way", and the phrase "PASS'S RULING TO MAKE" occurs exactly once in the plan. The list's `(b)` carries the OTHER duty from the same passage, the authoritative-path ruling stated one sentence later at `:1889` under "WHAT THE PASS OWES ON THIS", and carries nothing about the comment's coverage.

Same defect, same two sentences, same fix as `R2A-1`: a sufficiency guarantee the list cannot keep. `R2A-1` is the better statement, because it reaches BOTH assertion sites (`:1901` and `:1883`), because it identifies the mechanism that makes the guarantee worse than the gap it papers over ("it tells the reader not to check"), and because the duty it names is the one the ledger records as the human's ground for commissioning the pass. `R2B-3` under-rates its own finding at `medium` where the shared defect is `high`.

`R2B-3` CARRIES TWO THINGS `R2A-1` DOES NOT, and neither is lost. The first is the third missing duty above, which remedy F site 1 adds by name. The second is that letter `(a)` is the coupling verdict with the costing duty stripped off it, which is remedy F site 3.

I RULE ON THE FINDING'S OWN NOT-A-RE-RAISE CLAIM, which it argues explicitly. IT IS NOT A RE-RAISE OF `R1B-2`, and I settled that by checking that `R1B-2`'s remedy landed in full rather than by reading the argument. Round 1 remedy C site 1 required seven duties in the consolidated list: the coupling ruling, the authoritative-path ruling, the `W6` duty, the sub-decision ruling, the DESIGNED-versus-BOUNDED scope of mechanisms 2 and 3, the edit surface, and the YAGNI boundary. The list carries all seven, as `(a)` to `(g)` plus the edit-surface answer. So the prescribed remedy is discharged and the defect is in the sentence the fix pass wrote on top of it. New evidence, new site, new defect.

### R2B-4. The list omits three of the five components the Design explorations rule it cites requires

VERDICT: VALID. SEVERITY: `low`, CORRECTED DOWN from `medium`.

Reproduced. `pack/AGENTS.md:65` requires of each exploration document "the question, the design space (the viable options), each option's trade-offs judged against the numbered Project Principles, a recommendation with its reasoning, and an explicit 'what not to build' (the YAGNI boundary)". The item's list carries the YAGNI boundary at `(g)` and adds an edit surface at `(c)` that the rule does not require. It asks for no design space, no principle-judged trade-offs and no recommendation. `grep -ic principle` over the whole `Q-70` entry returns 0, in a plan file that defines eight numbered principles by name.

The item's own closing sentence depends on the missing components: "The orchestrator then synthesises, moves this item to `open`, and puts the options to the human through the human-input contract", where the options are what the mandate never asks any proposal to produce.

NOT THE OUT-OF-SCOPE "the item fails to present options". The item carrying no options is correct and by design, and I am not faulting it for that. The subject here is what the item asks of the EXPLORERS.

KEPT DISTINCT FROM `R2A-1` rather than folded in, and the reason matters for the fix. `R2A-1`'s guarantee is explicitly body-scoped ("every duty stated in the body above is repeated here"), so completing the list with the three missing body duties satisfies it and leaves this finding untouched. Only the branch of remedy F that DROPS the sufficiency claim happens to discharge both. This project measured what happens when a fix pass takes the narrower branch of a shared remedy, so the two stay separate with their own site.

SEVERITY CORRECTED DOWN to `low`. The three components the finding names are required of every explorer by the rule the same sentence cites, and by the explorer role prompt, so they are supplied even if the item never asks. `Q-69`'s equivalent paragraph is thinner still and asks for none of them either, so the item exceeds the established convention rather than falling short of it. What is genuinely wrong is the unqualified "THIS LETTERED LIST IS THE COMPLETE MANDATE" reaching territory the list does not cover, and that residue is `low`.

### R2B-5. "The ledger's current next-action paragraph", used twice, names the superseded one

VERDICT: VALID. SEVERITY: `low`, confirmed.

Reproduced in full, and the supersession structure is worse than the finding states. Both quotations live at `docs/plans/agent-scaffold.ledger.md:571`, in the paragraph beginning "THE IMMEDIATE NEXT ACTION IS THE `validation-constraints` STEP", which `git blame` dates to `8fa56939` (2026-08-11 12:28). A second paragraph beginning "THE IMMEDIATE NEXT ACTION, IN ORDER" sits at `:535`, blames to `903b70b8` (2026-08-11 17:29), and is the paragraph that commissioned this registration: its item (1) is "A WORKTREE-ISOLATED PLANNER registers the design question as a NEW `[[question]]` with `status = "exploring"`". Its item (4) is "A PLANNER then authors the `validation-constraints` step with its increments", which is not what `Q-70` attributes to "item (4)". The 12:28 paragraph also sits below a marker at `:563` reading "SUPERSEDED 2026-08-11, READ THIS PARAGRAPH AND IGNORE EVERY ONE BELOW IT".

I record one measured aggravation the reviewer did not reach: the quoted-text handle the item substitutes for a line number is no longer unique. `grep -c "teaching W5 the structured step association W3 already uses"` returns 2, because the ledger's own round 1 record at `:555` now quotes the same sentence.

`low` confirmed. The item's find-by-quoted-text convention limits the damage, both quotations are exact, and the current anchor at `:531` does endorse the older block's content. What is wrong is the word "current", twice, on a paragraph published under a supersession notice, plus the item number that follows from it. The fix is a relabel.

### R2B-6. The entry-route decision's ground is relayed without the three Project Principles the record attaches to it

VERDICT: VALID. SEVERITY: `low`, confirmed.

Reproduced at the line. The ledger at `:533` reads: "THE GROUND: W5's ownership check and a prospective W6 waiver-note join BOTH KEY ON HOW A WAIVER NAMES ITS UNIT, and whether they share a mechanism is a claim NOBODY HAS MEASURED, which is what an exploration exists for ('Ground decisions in evidence'); the choice between a lookup ... must be made with W6 in view ('Prefer the cleaner long-term architecture over the smallest diff'); and the two inc3 defects plus the three `next` defects are already-diagnosed point defects with NO OPEN DESIGN SPACE ... ('Minimal by default')."

All three are `[[principle]]` entries in the same plan file, and the reviewer's numbering is right: n = 6 "Ground decisions in evidence", n = 1 "Prefer the cleaner long-term architecture over the smallest diff", n = 2 "Minimal by default". `Q-70` carries all three limbs and names no principle; `grep -ic principle` over the entry returns 0.

`low` confirmed. The substance of the ground is relayed correctly and the principles are recoverable from the ledger, so nothing is falsified. What is lost is what the deciding authority weighed, in an item whose consumers are required to judge their own trade-offs against those same numbered principles. Three parentheticals close it.

Distinct from `R2B-4`: that finding is about what the MANDATE asks proposals to do, this is about the item's own relay of a decision's ground. Different sentences, and neither fix implies the other. They share remedy J.

### R2C-1. Three of the item's five reproduction commands fail under `nu`

VERDICT: DISMISSED. Severity had it been valid: `low`. The backstop covers dismissals at `high` or above, so none is engaged.

THE DEMONSTRATION REPRODUCES EXACTLY. `nu 0.108.0`, `nu -c 'which sort grep awk jq'` reports `sort` as `built-in` with no path and the other three as external. All three `sort -u` pipelines fail:

```
$ nu -c 'jq -r "select(.type==\"decision\") | .q_id" docs/metrics/workflow.jsonl | sort -u'
Error: nu::parser::unknown_flag
  x The `sort` command doesn't have flag `-u`.
$ nu -c 'jq -r "..." docs/metrics/workflow.jsonl | sort'
Error: nu::shell::only_supports_this_input_type
  x Input type not supported.
```

and the grep-only command at `:1893` runs cleanly under `nu`, returning the same four lines as under `bash`. So the measurement is sound and I dismiss on the defect, not on the evidence.

GROUND, IN FOUR PARTS.

First, THE PREMISE THAT DECIDES THE FINDING DOES NOT REPRODUCE. The finding's heading calls `nu` "this project's own configured interactive shell". The project configures `bash`: `justfile:1` is `set shell := ["bash", "-c"]`, `.envrc:1` is `# shellcheck shell=bash`, and `grep -il nu -- justfile .envrc flake.nix AGENTS.md` returns nothing. `nu` is this MACHINE's login shell, which the finding's body states correctly, and a machine-level shell preference is not a property of the artifact under review. A POSIX pipeline written for the project's own configured shell is not defective for failing in a non-POSIX shell the project does not use.

Second, THE CONVENTION PREDATES THE ITEM AND IS THE PROJECT'S OWN. `git grep -c "sort -u" main` finds the same construct at `docs/plans/workflow-calibration.explorations/finding-provenance-extract-a.md` (4) and `-b.md` (3). Those are EXPLORATION DOCUMENTS, which is precisely the artifact type `Q-70` commissions, so the item is following the project's established practice for reproduction commands in durable design records rather than departing from it. This does not make the finding out of scope, and I checked: the out-of-scope precedent's conditions 1 and 2 both fail, because these three commands were authored inside the reviewed range (`git log -S "sort -u"` returns `96b459c` and `1284fbf`, and `git show main:...plan.toml | grep -c "sort -u"` returns 0). It is in scope and I dismiss it on its merits.

Third, THE REVIEWER'S OWN SEVERITY ARGUMENT IS THE DISMISSAL ARGUMENT. It concedes that the failure is a loud parse error rather than a wrong result, that the escape is trivial, and that the primary consumer is a spawned explorer running through a `bash`-backed tool, so the primary execution path is unaffected. What is left is a human at their own prompt getting an immediate error they resolve in one edit.

Fourth, THE STANDARD THE FINDING INVOKES DOES NOT EXIST. It cites "the item's own 'worse than no command' standard". `grep -rn "worse than no command"` over `docs/`, `pack/` and `AGENTS.md` returns only the reviewer's own findings file. The item states no such standard.

WHAT THE FIX WOULD COST, which is why this is a dismissal and not a `low` valid finding. There is no single form that runs in both shells: `^sort -u` fails under `bash`. The only real remedy is a project-wide convention decision about which shell documented commands assume, which would reach every shell snippet in the plan, the ledger, the explorations and the findings files. That is a question the human may reasonably want to settle in its own item, and the orchestrator can route it, but it is not a `Q-70` defect and `Q-70` is not the place to decide it unilaterally.

---

## Deduplication map

| duplicate | primary | why |
| --- | --- | --- |
| R2B-3 | R2A-1 | Same claim, same two sentences (`plan.toml:1901` and `:1883`), same fix: a sufficiency guarantee the list cannot keep. R2A-1 reaches both assertion sites, names the suppression mechanism, and names the duty the ledger records as the human's ground for the pass. R2B-3's unique site (the `:1889` comment-coverage ruling) and its observation that `(a)` drops the costing duty are carried as remedy F sites 1 and 3. |

GENUINELY DISTINCT, though adjacent, and each kept because collapsing it would lose a site:

- `R2A-1` and `R2B-4` both attack the same paragraph. `R2A-1`'s guarantee is body-scoped, so completing the list with the three missing body duties discharges it and leaves `R2B-4` wholly unfixed. Only the drop-the-claim branch of remedy F discharges both, and the fix pass may legitimately take the other branch.
- `R2B-1` and `R2B-2` are one omission seen at two sites with two different consequences: what the PASS must weigh (`:1895`) and what the eventual STEP carries (`:1899`). Fixing either leaves the other short.
- `R2A-2`, `R2A-3` and `R2A-4` are three instances of round 1 remedy A's class at three sites. They share remedy I and none implies another.
- `R2B-5` and `R2B-6` are two defects in the item's handling of one source paragraph, a wrong label and a dropped attribution. They share remedy J as separate sites.

---

## Remedies

Lettering continues round 1's A to E so a site reference is unambiguous across the two rounds. Each remedy is scoped to its CLASS over the whole enclosing sentence and paragraph, not to the quoted fragment. Every site any reviewer named is accounted for, including the sites I decide to leave alone.

### Remedy F. A completeness claim this item cannot keep is dropped, not patched

Discharges `R2A-1` (with `R2B-3` folded in).

THE CLASS: the item asserts, in two places, that one paragraph is the complete statement of the pass's duties, and three duties its own body states are not in it. This project has already decided this class. `Q-55-impactclaim` (2026-08-10, receipt record 303, `docs/plans/agent-scaffold.ledger.md:651`) chose "DROP THE COMPLETENESS CLAIM AND KEEP THE BULLETS" over "completing the list properly", on the ground that "A COMPLETENESS CLAIM ABOUT AN INCREMENT STILL BEING EDITED CANNOT BE KEPT TRUE BY ANY AMOUNT OF DILIGENCE". `Q-70` is still being edited, and each round has added duties to its body. DO THE SAME HERE: drop the guarantee AND add the three duties, so the list is more useful without promising what it cannot hold.

Site 1, `plan.toml:1901`. Delete the sentence "THIS LETTERED LIST IS THE COMPLETE MANDATE, and it is the only place in this item where the mandate is complete: every duty stated in the body above is repeated here, so a proposal that satisfies this list satisfies the item, and a proposal that satisfies only part of it is short whatever the body seemed to ask." Replace it with a claim the item can keep, that the list collects the rulings the pass owes and the body states each duty in the paragraph that raises it, so a proposal reads the whole item. Then add the three duties the list is missing today, measured over `:1883` to `:1903`:

1. What the OTHER mechanism costs under the chosen direction, currently only in the second half of the sentence at `:1895` whose first half is `(c)`. Record that this is the human's stated ground for commissioning a pass, quoting `docs/plans/agent-scaffold.ledger.md:533`, "must be made with W6 in view".
2. Under direction (i), a ruling on what W5 does on the Markdown substrate, currently only inside direction (i)'s pricing at `:1895`.
3. Whether the comment at `src/plan/source.rs:785-790` failing to reach the membership check at `:807` is a documentation defect, a deliberate design divergence, or correct as it stands, currently only at `:1889` where the item itself calls it "THE PASS'S RULING TO MAKE".

DO NOT re-assert completeness after adding them.

Site 2, `plan.toml:1883`. The opener's "The complete statement of what the pass must resolve is the lettered list in WHAT THE PASS OWES BACK at the end of this item; every duty in the body between here and there is repeated in it" takes the same treatment: point at the list as the collected deliverables, without claiming it exhausts the body.

Site 3, `plan.toml:1901`, letter `(a)`. "(a) THE COUPLING RULING: whether W5's ownership check and the prospective W6 waiver-note join share a mechanism, ruled explicitly rather than left implied" is the coupling verdict with the costing duty stripped off. Either attach the costing duty to `(a)` or give it its own letter; do not leave it stated only in the body.

Sites left alone, with a verdict for each:

- `plan.toml:1901`, letters `(b)`, `(d)`, `(e)`, `(f)`, `(g)` and the closing edit-surface answer. CORRECT AS THEY STAND. I matched each against the body: `(b)` to `:1889`, `(d)` to `:1897`, `(e)` to `:1893`, and `(f)` and `(g)` are list-only by design. Do not touch them.
- `plan.toml:1895`, "Each proposal must state WHICH DIRECTION IT TAKES and WHETHER THAT DIRECTION IS ONE OF THE THREE NAMED ABOVE OR OUTSIDE THEM, and must say what the other mechanism costs under that choice." LEAVE AS WRITTEN. The body sentence is correct and complete; the defect is that the list carries half of it.
- The opener's "THE W5 FIX PLUS ALL THREE DETECTION MECHANISMS" against letter `(f)`'s "This item has deliberately never said either way", which the residue reviewer recorded as a secondary inconsistency. NOT A DEFECT, ruled above. Both sentences stand and no edit is owed.

### Remedy G. Register the third body of work queued to the same step

Discharges `R2B-1`.

THE CLASS: the item states the coupling question's participants from an inventory it inherited without re-measuring, and the inventory is short. The ledger's own "It now carries FOUR things" is undercounted; that correction is the orchestrator's and does not discharge this one.

Site 1, `plan.toml:1895`, the coupling paragraph. Register the queued project-identity work as a candidate participant. State the receipt (`Q-55-mechanism`, 2026-07-31, `chosen` "Anchor plus refusal, identity queued", the declined wider option being "Anchor, refusal, and identity fields now"), what it queued (an optional `project` field on `Round` and on the plan's `[meta]`, filtering the join in `check_workflow_toml`), that it is unbuilt (`Round`, `src/metrics.rs:620-655`, carries no `project` field), and the constraint the human's recorded reasoning attaches, that the record schema W3, W4 and W5 all read "should be ONE deliberate edit rather than a rider on a path fix". DO NOT rule whether it couples: that is the pass's, and the item carries no options by design.

Site 2, `plan.toml:1901`. Add the corresponding duty to the list: whether the queued project-identity edit shares the mechanism, given that it filters the same `check_workflow_toml` join direction (iii) would read and edits the same record schema direction (ii) would rework.

Site left alone, with a verdict: `plan.toml:1895`'s existing two-body framing of W5 and the prospective W6 join. KEEP IT. It is what `Q-55-entryroute` decided and what the ledger's ground states. Add the third participant; do not replace the pair.

### Remedy H. The deferred-inputs paragraph stops promising to be the inventory

Discharges `R2B-2`.

THE CLASS: remedy F's class at a different list. `:1899` opens "recorded as deferred inputs so nobody loses them" and enumerates two entries where the record carries more.

Site 1, `plan.toml:1899`. Either complete the list or drop the preservation promise, preferring the drop on the `Q-55-impactclaim` precedent, since the ledger keeps extending this set. The measured missing entries, if it is kept: the `project_root_of_source` shared root cause queued by `Q-55-resumecost` (2026-08-02, receipt, ledger paragraph beginning "`Q-55-resumecost` DECIDED"); the project-identity work of `Q-55-mechanism`, unless remedy G places it inside the pass, in which case say so here and cross-reference; and the ledger half of the `run_next` false green, which the ledger paragraph beginning "THE BACKSTOP CORRECTED BOTH EARLIER AGENTS ON OWNERSHIP" records as belonging to neither queued item and as having "NO OWNER anywhere in the plan". Locate all three by quoted text, per the item's own convention.

Site left alone, with a verdict: the clause "The pass does NOT weigh these: they are already-diagnosed point defects with NO OPEN DESIGN SPACE". CORRECT for entries (a) and (b), and it is the `Q-55-entryroute` ground verbatim. It MUST NOT be extended over any newly added entry without checking that entry: the `project_root_of_source` root cause is diagnosed, the unowned ledger half is not, and the project-identity work has a measured proposal but no decision.

### Remedy I. Every count of a moving population follows the rule the item already applies to two of them

Discharges `R2A-2`, `R2A-3`, `R2A-4`.

THE CLASS: this is round 1 remedy A, reproducing inside and around the fix that closed it. The prohibition is unchanged and binding: DO NOT SUBSTITUTE A CORRECTED FIGURE. A corrected figure is the same defect with a later expiry date.

Site 1, `plan.toml:1899`, "It says THREE in two live passages, both dated 2026-08-11 ... THOSE TWO LIVE 'THREE'S ARE OWED A CORRECTION". Delete the enumeration and the date qualifier. Measured: the phrase occurs SIX times in the ledger, at `:533`, `:571`, `:1055`, `:1259`, `:1261` and `:1337`, dated 2026-08-11 (twice), 2026-08-02, 2026-07-31 (twice) and 2026-07-30, and the 2026-07-30 one is the human decision this same sentence names as its routing authority. State the property instead (the ledger says "three" in several places and "four" in one, four is the measured count, and the "three"s are owed a correction that is not this item's to make) and give the reproduction ``grep -niE "three (\`?agent-scaffold )?\`?next\`? defects" docs/plans/agent-scaffold.ledger.md``. Keep the four, keep the four defects' descriptions, keep the find-by-quoted-text instruction. DO NOT WRITE "SIX".

Site 2, `plan.toml:1895`, direction (i)'s "It is a by-product of the membership rule at `src/plan/source.rs:807`, so a step tends to declare an increment when a waiver needs one and not otherwise". Delete the causal claim; it is the one clause in a sentence labelled "recorded as a measured input" that was not measured. Measured over the live plan: 45 declared increment ids, 13 named by an increment-unit waiver and 32 named by none; 32 steps declare increments and 10 carry an increment waiver, so 22 declare without ever waiving. Replace with what holds and does not expire: the set is hand-maintained in the plan and is not derived from the round log, so it is not a model of the identities the checks operate on. Keep the two adjacent clauses, which I verified: the `complete` steps that declare none while their rounds carry increment ids, and the uppercase exclusion at `src/plan/source.rs:475-477`. Keep the comparison command in the following sentence.

Site 3, `plan.toml:1893`, mechanism (3)'s "roughly eleven `src/checks.rs` citations". Delete the figure. Measured in `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md`: 21 occurrences, 15 distinct, and every one of the 15 resolves to unrelated content in the current `src/checks.rs`. State the property (every `src/checks.rs` citation in that document is stale, so the resolver goes red on all of them) and give the reproduction `grep -oE 'src/checks\.rs:[0-9]+(-[0-9]+)?' docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md | sort -u | wc -l`. DO NOT WRITE "FIFTEEN". Keep the attribution to `Q-55-check21b` and its deliberate-staleness ruling, which is accurate.

Sites left alone, with a verdict for each:

- `plan.toml:1893`, mechanism (1)'s "NO COUNT OF THOSE SITES IS STATED" and mechanism (2)'s "NO COUNT IS STATED HERE, DELIBERATELY". CORRECT, and they are the model the three sites above must follow. I re-ran both reproductions: the waiver-note grep returns exactly four lines, and the receipt extraction returns 62 distinct ids against 70 registered questions, 40 dangling, all `Q-55-`.
- `plan.toml:1897`, "Measured for this registration: the token `W6` occurs exactly once in `docs/plans/agent-scaffold.plan.toml` outside this item". LEAVE, and I agree with the residue reviewer's second ground rather than only its first. Re-measured: one line outside the item (`:1774`), one occurrence on it, `Q-59`'s. The load-bearing claim is the enumeration of two named checks, not the count; the ledger's four `W6` lines all mean the waiver-note join, and `grep -rn W6 pack/ src/ AGENTS.md .agents/` returns nothing, so no third meaning exists to accrete. A third would have to be authored, not accumulated, so the population lacks the growth property that makes this class bite. Round 1 remedy A left the identically framed `blocked_by` re-measurement alone on the same ground.
- `plan.toml:1897`, "they continue the established `-w1` to `-w4` waiver-id sequence this step already carries". LEAVE. Re-measured: all four ids exist under the step at `plan.toml:1325`, `:1334`, `:1343` and `:1352`. This asserts a convention by its endpoints, not a population, and adding `-w5` and `-w6` extends it rather than falsifying it.
- `plan.toml:1899`, the `blocked_by` re-measurement, and `plan.toml:1887`, escape route 1's "these two have five each". LEAVE, as round 1 ruled. Both still reproduce.
- `plan.toml:1885`, "TWO MEMBERS OF THAT POPULATION BLOCK A STEP TODAY". LEAVE. It names two tokens as the two that block a step today, not as the population, and the next sentence says the population is not confined to them. The pipeline reproduces and returns the same six identities across three steps.

### Remedy J. Relabel the ledger paragraph, and restore the principle names its source attaches

Discharges `R2B-5`, `R2B-6`.

THE CLASS: the item's pointers into the ledger name the wrong paragraph and drop the deciding authority's own attributions.

Site 1, `plan.toml:1895`, "the ledger's current next-action paragraph describes the fix as ...". Relabel. The paragraph carrying that quotation is the 12:28 one, published under "SUPERSEDED 2026-08-11, READ THIS PARAGRAPH AND IGNORE EVERY ONE BELOW IT", and a newer next-action paragraph exists from 17:29 that carries neither quotation. "The ledger's `validation-constraints` routing paragraph" is true and still finds by quoted text. Add enough surrounding words to the handle to disambiguate, because the quotation now resolves to two ledger paragraphs, the routing paragraph and the ledger's own round 1 record quoting it.

Site 2, `plan.toml:1899`, "and the ledger's current next-action paragraph, item (4)". Same relabel. The item number is correct only for the older paragraph: the newer paragraph's item (4) is "A PLANNER then authors the `validation-constraints` step with its increments".

Site 3, `plan.toml:1895` and `:1899`, the `Q-55-entryroute` ground. Restore the three principle names the record attaches to the three limbs: "Ground decisions in evidence" (n = 6) on the unmeasured-coupling limb, "Prefer the cleaner long-term architecture over the smallest diff" (n = 1) on the with-W6-in-view limb, and "Minimal by default" (n = 2) on the already-diagnosed-defects limb. All three are `[[principle]]` entries in the same file.

Site left alone, with a verdict: the item's other find-by-quoted-text handles. LEAVE. I checked all five and every one resolves; four resolve uniquely.

### Remedy K. The mandate asks for what the rule it cites requires

Discharges `R2B-4`.

Site 1, `plan.toml:1901`. The paragraph cites the Design explorations rule in `pack/AGENTS.md` for where explorers write and omits three of the five components the same rule requires of every exploration document: the design space (the viable options), each option's trade-offs judged against the numbered Project Principles, and a recommendation with its reasoning. Either add them to the list, or state that the list is ADDITIONAL to the rule's five components rather than a replacement for them. The item's closing sentence needs them: "puts the options to the human through the human-input contract" requires options the list never asks any proposal to produce. If remedy F drops the sufficiency claim, the lighter of these two forms is enough.

Site left alone, with a verdict: `Q-69`'s thinner equivalent paragraph, named by the reviewer as the comparator. LEAVE, and out of this artifact besides. It makes no sufficiency claim, which is what makes the gap a defect here and not there.

---

## Overall assessment

WHAT THE ROUND'S REAL RESULT IS. Nine valid findings, ceiling `high`, two at `high`, on a document whose factual spine remains sound. I re-ran both of the item's own reproduction commands and both return exactly what it describes; I re-derived every source citation the fix pass added and each resolves at the line; I re-measured the blocker population (six identities across three steps), the dangling-receipt set (62 receipt ids, 70 registered questions, 40 dangling, all `Q-55-`), the `W6` occurrence count, the waiver-id sequence, the `blocked_by` field and the item's two fixture recipes, and all of them hold. As in round 1, NO FINDING SHOWS `Q-70` ASSERTING SOMETHING THE TOOL CONTRADICTS. Three findings show it asserting something a MEASUREMENT OF THE PROJECT'S OWN RECORDS contradicts (`R2A-2`, `R2A-3`, `R2A-4`), which is a different and milder class, and the rest are about what the item omits and what it instructs.

ONE SYSTEMIC DEFECT OR MANY. Two, and the second is new.

The first is THE MOVING POPULATION STATED AS A FIXED FIGURE, which is round 1's own systemic defect reproducing inside and around the pass that fixed it. `R2A-2` is a new instance authored by the fix commit itself, in the same commit that stripped three other counts on the ground that a count expires. `R2A-3` is a new causal claim labelled "measured" that the plan contradicts. `R2A-4` survived the fix pass because the writer tested it for durability rather than for truth. That the class reproduced twice inside its own remedy is the strongest argument yet for remedy I's prohibition on substituting a corrected figure.

The second is THE COMPLETENESS CLAIM, and it is a defect the round 1 fix pass INTRODUCED. Round 1's remedy C asked for a consolidated list. The fix pass built it correctly, carrying all seven duties the remedy enumerated, and then attached a guarantee nobody prescribed: that the list is the complete mandate and that satisfying it satisfies the item. Three duties in the body are not in it (`R2A-1`, `R2B-3`), three components the cited rule requires are not in it (`R2B-4`), and a receipted body of work queued to the same step is nowhere in the item at all (`R2B-1`), while a second list in the same item promises preservation it does not deliver (`R2B-2`). Five of the nine valid findings are this one class. The project has already decided how to treat it, in `Q-55-impactclaim`, and the remedies follow that decision rather than inventing a treatment.

The residue is two pointer defects, `R2B-5` and `R2B-6`, both `low`, both ordinary.

DID THE FIX PASS MAKE THINGS BETTER OR WORSE ON NET. BETTER, clearly, and with one specific regression. All five round 1 remedies landed and I verified each rather than accepting the reviewers' word: remedy A's three sites carry properties and commands instead of figures, remedy B's four sites price both directions at their real surface, remedy C's list carries all seven enumerated duties, remedy D's three candidate directions are labelled non-exhaustive with direction (iii) named and marked "RECORDED, NOT RECOMMENDED", and remedy E's two citation corrections are right, including the `src/plan/source.rs:791-856` range which does close at `:856`. The item is materially more useful to a pass than it was. The regression is that the consolidation acquired a guarantee, and that guarantee is why the round still carries a `high`: the gap it papers over is smaller than round 1's, and the instruction not to look for it is new. A fix that adds duties and keeps the guarantee will produce the same finding again in round 3.

WHAT THE THREE LENSES COLLECTIVELY MISSED. Four things, all measured.

1. NOBODY MEASURED THE COMPLETENESS GAP. Two lenses found subsets of it independently, two duties and one duty, and neither states the set. I enumerated all fourteen duty sentences in `:1883` to `:1903` and matched each against the list: the missing set is exactly three, and no more. A fix written from either finding alone lands a half-fix, which is the failure mode this project measured and wrote the class-scoping rule for.

2. THE ROUND'S OWN COUNT DEFECT CARRIES A COUNT DEFECT. `R2A-2` reports that the item's enumeration of ledger "three"s is short, and its own enumeration is short by three more: the phrase occurs six times, not three, and the reviewer's displayed grep output drops `:1055`, `:1259` and `:1261` that the command as written returns. This is the loop's standing caution firing again, now inside a finding written specifically about that caution.

3. THE FIND-BY-QUOTED-TEXT CONVENTION HAS STARTED TO DEGRADE, and no lens tested it for uniqueness. Round 1 dismissed `R1B-3` partly on the strength of that convention. Measured now, "teaching W5 the structured step association W3 already uses" resolves to TWO ledger paragraphs, because the ledger's own round 1 record quotes the sentence. The handle still locates the content, so this does not reopen `R1B-3` and I raise no finding on it. It is the first measured cost of the convention and it will grow every time the ledger quotes itself, which this loop does routinely.

4. THE POPULATIONS MOVED AGAIN DURING THIS ROUND, exactly as the standing caution predicts. Round 1 measured 94 round-log increment identities and 308 log records; both are now 95 and 309, because round 1's own record was appended. The declared increment set is 45, of which 43 appear in the log. Any figure in a fix written from this triage will be wrong before round 3 opens, which is why every remedy above prescribes a property and a command rather than a number.

MECHANICAL STATE OF THE ARTIFACT, checked independently: `render docs/plans/agent-scaffold.plan.toml --check` reports "up to date" at exit 0; `validate --source ... --metrics ... --workflow` reports 309 records valid, 95 steps and 70 questions valid, and "workflow invariants hold" at exit 0; all three changed files return 0 under `LC_ALL=C grep -cP '[^\t\x20-\x7e]'`. `Q-70`'s claim that "NO step exists yet" holds: `grep -n 'slug = "validation-constraints"'` returns nothing.

---

## Commands that decided a verdict, with their output

Every command ran against `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/tri-q70-r2` or a scratch file derived from it.

`R2A-1` and `R2B-3`, the list extraction and the duty audit:

```
$ sed -n '1901p' docs/plans/agent-scaffold.plan.toml | sed 's/.*\((a) THE COUPLING RULING\)/\1/' > list.txt
$ for w in cost "other mechanism" Markdown substrate principle; do grep -ci "$w" list.txt; done
0 0 0 0 0
$ grep -o "([a-g]) [A-Z ]*" list.txt
(a) THE COUPLING RULING / (b) THE AUTHORITATIVE / (c) THE DIRECTION AND ITS EDIT SURFACE /
(d) THE W / (e) THE SUB / (f) THE SCOPE OF MECHANISMS / (g) THE YAGNI BOUNDARY
$ python3 (duty-sentence extraction over lines 1883-1903, matched against list.txt)
14 duty sentences; 3 not carried by the list: :1895 cross-pricing, :1895 Markdown-substrate
ruling, :1889 "THE PASS'S RULING TO MAKE"
$ grep -o "PASS'S RULING TO MAKE" docs/plans/agent-scaffold.plan.toml | wc -l
1
```

`R2A-2`, the ledger population and its dates:

```
$ grep -noiE "three (\`?agent-scaffold )?\`?next\`? defects" docs/plans/agent-scaffold.ledger.md
533, 571, 1055, 1259, 1261, 1337        (six matches)
$ git blame -L <each>,<each> --date=short HEAD -- docs/plans/agent-scaffold.ledger.md
533 -> 903b70b8 2026-08-11    571 -> 8fa56939 2026-08-11    1055 -> f9d589be 2026-08-02
1259 -> 12d6a01a 2026-07-31   1261 -> 12d6a01a 2026-07-31   1337 -> 90b92b2d 2026-07-30
```

`R2A-3`, the declared-increment measurement (`tomllib` over the plan, `json` over the log):

```
declared increments: 45   increment-unit waivers: 13   step-unit waivers: 12
declared ids ALSO waived: 13     declared ids with NO waiver: 32
steps declaring increments: 32   steps with increment waivers: 10
steps declaring but never waiving: 22
logged identities: 95   logged and declared: 43   logged not declared: 52
complete steps declaring zero increments whose rounds carry a DISTINCT increment id: 2
   state-schema (-inc1/-inc2/-inc3), round-log-core (-incA/-incB)
```

`R2A-4`, the citation census and the staleness check:

```
$ grep -oE 'src/checks\.rs:[0-9]+(-[0-9]+)?' docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md | wc -l
21
$ ... | sort -u | wc -l
15
$ python3 (each distinct citation printed with its sidecar sentence and the actual source lines)
15 of 15 resolve to unrelated content; RUNNER_PREFIX is at :98 not :78, WorktreeGuard at :345
not :329-342, the libc sentence at :411-415 not :388-392, owning_pid at :561 not :400-405,
prune_orphan_worktrees at :588 not :407-461, nanos at :1023 not :845-852
$ git log -1 --date=short --format='%h %ad' -- src/checks.rs
09a027c 2026-07-31
$ git log --oneline -S "roughly eleven" -- docs/plans/agent-scaffold.plan.toml
96b459c   (authored inside the reviewed range)
```

`R2B-1` and `R2B-2`, the receipts, the ledger paragraphs and the schema:

```
$ jq -c 'select(.q_id=="Q-55-mechanism" or .q_id=="Q-55-resumecost")' docs/metrics/workflow.jsonl
both present, chosen "Anchor plus refusal, identity queued" (2026-07-31) and
"Accept as (iv), queue the shared cause" (2026-08-02)
$ sed -n '1081p;1073p;863p;893p;651p' docs/plans/agent-scaffold.ledger.md
all five quoted passages reproduce verbatim, including "ONE deliberate edit rather than a
rider on a path fix", "an optional `project` field on `Round` and on the plan's `[meta]`",
"This joins the PROJECT-IDENTITY work already queued to that step", "NO OWNER anywhere in
the plan", and Q-55-impactclaim's "A DOCUMENTATION-IMPACT LIST THAT ENUMERATES ITS OWN
EXCLUSIONS IS A COMPLETENESS CLAIM"
$ sed -n '610,655p' src/metrics.rs
struct Round { line, task, artifact, outcome, consecutive_clean, risk_class, step, increment }
(no `project` field)
$ sed -n '1880,1904p' docs/plans/agent-scaffold.plan.toml > q70body.txt
$ grep -c 'Q-55-mechanism' q70body.txt -> 0 ; 'Q-55-resumecost' -> 0 ; 'principle' -> 0
```

`R2B-4` and `R2B-6`, the rule and the principles:

```
$ sed -n '65p' pack/AGENTS.md | grep -o "Each document follows the human-input contract.*"
"... the question, the design space (the viable options), each option's trade-offs judged
against the numbered Project Principles, a recommendation with its reasoning, and an explicit
'what not to build' (the YAGNI boundary)."
$ awk over [[principle]] blocks in docs/plans/agent-scaffold.plan.toml
n=1 "Prefer the cleaner long-term architecture over the smallest diff"
n=2 "Minimal by default"   n=6 "Ground decisions in evidence"   (8 in total)
$ sed -n '533p' docs/plans/agent-scaffold.ledger.md | grep -o "THE GROUND:.*"
carries all three principle names in parentheses, one per limb
```

`R2B-5`, the two next-action paragraphs:

```
$ grep -n "SUPERSEDED\|READ THIS PARAGRAPH" docs/plans/agent-scaffold.ledger.md
531 (current anchor), 563 ("SUPERSEDED 2026-08-11, READ THIS PARAGRAPH AND IGNORE EVERY ONE
BELOW IT"), ...
$ sed -n '535p;571p' docs/plans/agent-scaffold.ledger.md
535: "THE IMMEDIATE NEXT ACTION, IN ORDER. (1) A WORKTREE-ISOLATED PLANNER registers ...
      (4) A PLANNER then authors the `validation-constraints` step with its increments."
571: "THE IMMEDIATE NEXT ACTION IS THE `validation-constraints` STEP ... It now carries FOUR
      things ... (4) the three `agent-scaffold next` defects routed here ..."
$ git blame  ->  535 = 903b70b8 (17:29), 571 = 8fa56939 (12:28)
$ grep -c "teaching W5 the structured step association W3 already uses" ...ledger.md
2        (at :555 and :571)
```

`R2C-1`, the dismissal:

```
$ nu --version -> 0.108.0
$ nu -c 'which sort grep awk jq'
sort -> built-in (no path); grep, awk, jq -> external
$ nu -c 'jq -r "..." docs/metrics/workflow.jsonl | sort -u'
Error: nu::parser::unknown_flag  x The `sort` command doesn't have flag `-u`.
$ nu -c 'grep -onE "[(][0-9]+(, [0-9]+){1,6}[)]" docs/plans/agent-scaffold.plan.toml'
1331:(3, 4, 6) / 1340:(9, 5, 6, 4) / 1349:(11, 9, 6, 4, 5) / 1358:(6, 4, 2, 0, 2)   (clean)
$ head -1 justfile
set shell := ["bash", "-c"]
$ head -1 .envrc
# shellcheck shell=bash
$ git grep -il "nushell\|\bnu\b" -- justfile .envrc flake.nix AGENTS.md     -> (empty)
$ git grep -c "sort -u" main -- .
docs/plans/workflow-calibration.explorations/finding-provenance-extract-a.md:4
docs/plans/workflow-calibration.explorations/finding-provenance-extract-b.md:3
(plus the round 1 findings files)
$ grep -rn "worse than no command" docs/ pack/ AGENTS.md
only q70-capture-r2-reviewer-surfaces.md itself
```

The item's own reproduction commands and the artifact's mechanical state:

```
$ jq -r 'select(.type=="round") | [(.step // (.task|sub("-inc[a-zA-Z0-9]+$";""))), (.increment // .task)] | join(" ")' docs/metrics/workflow.jsonl | sort -u | awk '{lead=$2; sub(/-inc[a-zA-Z0-9]+$/,"",lead); if (lead != $1) print $1, $2}'
decision-folder-currency decision-folder-currency-fold
workflow-driver workflow-driver-stage0a / -stage0b / -stage1
workflow-enforcement-tier workflow-enforcement-tier-endproperty-fold / -fold
$ receipt ids 62, registered questions 70, dangling 40, non-`Q-55-` dangling 0
$ agent-scaffold render docs/plans/agent-scaffold.plan.toml --check      -> up to date, EXIT 0
$ agent-scaffold validate --source ... --metrics ... --workflow
309 records valid; 95 steps, 70 questions valid; workflow invariants hold; EXIT 0
$ LC_ALL=C grep -cP '[^\t\x20-\x7e]' on all three changed files      -> 0, 0, 0
$ grep -n 'slug = "validation-constraints"' docs/plans/agent-scaffold.plan.toml  -> (none)
```

WHAT I SETTLED BY RUNNING AND WHAT BY READING.

RUN: `R2A-1` (list extraction, term search, and my own duty audit), `R2A-2` (the ledger grep and six blames), `R2A-3` (an independent `tomllib` plus JSONL measurement), `R2A-4` (the citation census and a line-by-line staleness check of all fifteen), `R2B-2` (the receipt lookup), `R2B-4` and `R2B-6` (the zero-principle counts and the principle table), `R2B-5` (the blames and the two paragraphs), `R2C-1` (the `nu` reproductions, the shell-resolution table, the project shell configuration and the prior-art census), the two writer-reported sites the residue lens left alone, both of the item's own reproduction commands, and the artifact's `render --check`, `validate --workflow` and ASCII sweep.

RUN AND READ: `R2B-1`, whose receipt and schema claims I ran and whose ledger reasoning I read at the paragraph.

READ: `R2B-3`, which is a claim about the item's own text against its own list and is settled by opening `:1883`, `:1889` and `:1901`; the fix commit's full word-diff, so every new sentence was judged as new content; and round 1's findings and verdicts, held throughout to keep a settled finding from being re-raised.

Nothing above is presented as measured that was not run.
