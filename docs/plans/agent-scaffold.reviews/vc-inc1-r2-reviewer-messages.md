# `validation-constraints-inc1`, round 2: reviewer (messages and shipped prose)

Reviewer worktree: `.claude/worktrees/rev-inc1r2-messages`, branch `review/inc1r2-messages`, at `60ee7d0`.
Artifact: `git diff main..HEAD` (`main` at `76b28ef`), that is `6ec9f1a` (the first fix) plus `60ee7d0` (the round 1 fix pass).
Lens: is what the tool and the project now SAY actually true, as a human reads it.
Round 1's settled verdicts are in `vc-inc1-r1-triage.md`. No settled finding is re-raised below; where a finding touches the same text a round 1 remedy produced, the evidence is new and is stated as such.

Binaries, both built from a clean `git archive` or from the worktree, one `CARGO_TARGET_DIR` each, verified distinct:

```
2041cff81a46976bc4089c68edf0a7f5  target-post/debug/agent-scaffold   (worktree HEAD, 60ee7d0)
610548f03939e586bf3a9d1643f3f8fa  target-pre/debug/agent-scaffold    (git archive main, 76b28ef)
```

Suite in a scratch copy of HEAD, `TMPDIR` outside every repository: `cargo test` is 385 + 5 + 1 + 1 + 9 + 3 + 20 + 1 + 4 = 429 passed, 0 failed.

Every fixture below lives under my own scratch subdirectory. Each is a project root holding `docs/plans/t.md` (or `t.plan.toml`) and `docs/metrics/workflow.jsonl`, and the command in every case is `agent-scaffold validate --plan <root>/docs/plans/t.md --workflow` (or `--source <root>/docs/plans/t.plan.toml --workflow` on the TOML substrate). Paths are elided to `PLAN` and `LOG` in the quoted output.

## Findings

Seven findings: one `medium`, six `low`. NO `high` AND NO `critical` FINDING WAS FOUND, and I looked for one specifically in the marker's two failure directions (below, "the marker sweep"). No verdict is wrong on any tree I built: thirty-nine fixture trees, both substrates, both binaries, plus this repository's own plan and log unmodified.

### `W2B-1`: the shipped rule text and the `CHANGELOG` both say a step reached through the `leading_slug` fallback IS reported as derived; the per-owner merge means it is not

`low`. Three shipped copies plus the `CHANGELOG`.

The shipped clause, byte-identical in `pack/instrument.md:11`, `AGENTS.md:147` and `.agents/AGENTS.reference.md:147` (`md5sum` of the three lines: `b02e698ef4ac2f82a0cb742b06a30ac0` each), says:

```
an increment no `round` record resolves to is reported, and a step reached through the `leading_slug` fallback is reported as derived
```

and `CHANGELOG.md:32` says:

```
a pre-migration record carries no `step` id, so `round_step_slug` still derives its step, now from THAT RECORD'S `task` rather than from the waiver's increment id, and a refusal naming such a step marks it as derived
```

The mark is computed per OWNER with an OR across the increment's records (`src/workflow.rs:644-653`), not per record, so a step that one record reaches through the fallback is NOT marked when another record of the same increment declares it. Fixture `g8-or-merge`: increment `alpha-inc1` has one record carrying `"step":"alpha"` and one pre-migration record whose `task` is `alpha-inc1`, so `leading_slug` reaches `alpha` too; the waiver names `beta`.

```
POST: PLAN vs LOG: round log line 4: increment waiver names step `beta` but the round log joins increment `alpha-inc1` to step `alpha`
```

`alpha` was reached through the `leading_slug` fallback by the second record and is not reported as derived. The behaviour is right and is deliberately pinned by `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it` (`src/workflow.rs:1767`), whose own comment states the per-owner rule correctly. It is the two prose statements that describe the code per record.

This is not `W1B-4` re-raised: `W1B-4` was that the shipped clause stated NEITHER fallback, and the fix pass added both. This is a different sentence, added by that same fix, which states the marking rule more strongly than the code implements it.

REMEDY, over the whole clause in all three copies (which must move together or the drift guard fails, demonstrated below) and over the `CHANGELOG` sentence: say the mark is per owner, for example that a step NO record of that increment declares in a structured `step` id is reported as derived.

### `W2B-2`: the `CHANGELOG` names ONE narrowed population; there are two, and the second is the one a real project hits

`medium`. `CHANGELOG.md:32`.

The entry states:

```
THE POPULATION THIS NARROWS is an increment-unit waiver whose `increment` NO round record resolves to
```

That phrasing is exhaustive and the population is not. A waiver whose increment id DOES strip to its `step` (so the retired rule accepted it) and whose records join that increment to ANOTHER step is accepted by the pre-fix binary and refused by the fixed one. It appears nowhere in the entry.

Fixture `h1-rehomed-declared`, one record carrying `"increment":"alpha-inc1","step":"beta"`, waiver `step = alpha`, `increment = alpha-inc1`:

```
POST: PLAN vs LOG: round log line 3: increment waiver names step `alpha` but the round log joins increment `alpha-inc1` to step `beta`
PRE : (no ownership problem; the retired rule accepted it, since leading_slug("alpha-inc1") == "alpha")
```

Fixture `h2-rehomed-derived`, the same population reached without any mis-declared `step`, by the increment-only record the accessor block at `src/workflow.rs:98-111` documents (`"task":"beta-inc1","increment":"alpha-inc1"`, no `step`):

```
POST: PLAN vs LOG: round log line 3: increment waiver names step `alpha` but the round log joins increment `alpha-inc1` to step `beta` (derived from a record's `task`)
PRE : (no ownership problem)
```

Why this matters more than the empty case the entry does name: the empty case is a waiver that evidences nothing and grants nothing, and the entry itself says so ("a dead waiver becomes visible"). This second population is a waiver that was silently ACCEPTED while the records contradicted it, which is the defect the increment exists to close, and a reader of the entry cannot predict the new refusal from what it says. The step's own Documentation impact item requires the affected population be named (`validation-constraints.md:145`, "the population it affects must be named").

REMEDY: `CHANGELOG.md:32`, the `THE POPULATION THIS NARROWS` sentence. Name both: an increment no round record resolves to, AND an increment the records resolve but join to a different step than the waiver names. The second half of the same sentence, "the retired rule accepted it silently whenever the id happened to strip to the step slug", is the condition under which BOTH populations were accepted, so it needs no separate change.

### `W2B-3`: the `CHANGELOG`'s account of what the RETIRED rule did is false in one case, measured

`low`. `CHANGELOG.md:32`.

```
it refused every increment id that does not end `-inc<x>` even when the step's own round records join it to that step
```

The retired rule refused iff `leading_slug(increment) != waiver.step`. For an id that does not end `-inc<x>`, `leading_slug` returns it unchanged, so the rule refused it UNLESS the id IS the step slug, which is a shape the log already produces (`plan-fold`, `vc-step-fold` and three other live tokens resolve their step to the whole token, per the round 1 triage's own measurement).

Fixture `g12-retired-accepts`, Roadmap step `plan-fold`, one record joining increment `plan-fold` to step `plan-fold`, waiver `step = plan-fold`, `increment = plan-fold`:

```
POST: (accepted, no ownership problem)
PRE : (accepted, no ownership problem)
```

The pre-fix binary accepted an increment id that does not end `-inc<x>` whose round records join it to the waiver's step, which is exactly the case the sentence says it refused.

REMEDY: `CHANGELOG.md:32`, one clause. Say it refused every such id EXCEPT one identical to the step slug, or state the rule itself ("it required the increment id's leading slug to equal the step") and let the class follow.

### `W2B-4`: the doc comments describing the derived mark say "any record" where the code means "any record of this increment", and say several owners arise "two ways" where a third exists

`low`. `src/workflow.rs:545`, `:551-555`, `:589`, `:640`, `:1688-1689`.

Scope half. `step_attribution`'s doc says "Each owner maps to whether ANY record declared it in a structured `step` id; one no record declared is marked as derived" (`:545`), the inline comment says "an owner no record declares was computed by the join rather than carried by the log" (`:640`), and the `w5_problems` bullet says the named step "is READ from a record's `step` id where one exists and is DERIVED otherwise" (`:589`). The loop is filtered to records that resolve to THIS increment (`:645-647`), so a step another increment's record declares is still marked derived.

Fixture `g9-cross-increment`: a pre-migration record for `alpha-fold`, plus a second record carrying `"step":"alpha-fold","increment":"other-inc1"`.

```
POST: PLAN vs LOG: round log line 4: increment waiver names step `beta` but the round log joins increment `alpha-fold` to step `alpha-fold` (derived from a record's `task`)
```

A record DOES declare `alpha-fold` in a structured `step` id, and the owner is marked derived. The message's own words stay true (that ATTRIBUTION is derived, and the per-increment scope is the right scope), so this is a comment defect and not a message defect.

Enumeration half. The same doc says "SEVERAL OWNERS ARISE TWO WAYS AND NEITHER NEEDS A MALFORMED LOG" (`:551`) and lists two structured `step` ids, or one structured record plus one pre-migration record; the test comment at `:1688-1689` calls its case "the first of the two routes". A third route needs neither: two records for one increment, both carrying a structured `increment` and no `step`, with different `task` values. Fixture `h5-two-derived`, a log with no schema problems at all:

```
POST: PLAN vs LOG: round log line 4: increment waiver names step `mmm` but the round log joins increment `shared-x` to steps `aaa` (derived from a record's `task`), `bbb` (derived from a record's `task`)
```

Both owners are derived and neither listed route produced them.

This is not `W1B-3` re-raised: `W1B-3` was that the OLD doc gave free-string authoring as the only cause, and the fix added a second. The evidence here is against the NEW two-route enumeration and against the new "any record" scoping, neither of which existed at round 1.

REMEDY: `src/workflow.rs:543-555`, `:589` and `:640`, over the whole statements: scope the declaration test to the increment's own records, and either drop the "two ways" count or add the partial-record route. `src/workflow.rs:1688-1689`, the test comment's "the first of the two routes", moves with it.

### `W2B-5`: the fix changed the verb for this relation from "attributes" to "joins" everywhere except the doc comment on the predicate itself

`low`. `src/workflow.rs:411-412`, and the helper name `step_attribution` at `:556`.

`60ee7d0` replaced "attributes" with "joins" in both refusal strings, in the `w5_problems` bullet, in the inline comments, in the test comments and in a test NAME (`w5_names_every_step_the_log_attributes_a_waived_increment_to` became `w5_names_every_step_the_log_joins_a_waived_increment_to`). Command:

```
git diff 6ec9f1a..60ee7d0 -- src/workflow.rs | grep -E "^[-+].*(attribut|joins)"
```

reports twelve removed lines carrying "attribut" against one added line carrying it, and that one added line is the helper's own name (`fn step_attribution`). No removed line is `src/workflow.rs:412`, which still reads:

```
/// Whether `waiver` exempts the increment `round` belongs to, as the ROUND LOG
/// attributes it: ...
```

That doc comment is the definition of the relation both checks consult, so it is the one site where the retired word is most likely to be read as naming a different thing. The helper is also still `step_attribution` while the test that asserts its output was renamed away from "attributes" in the same commit, so the split is visible inside one commit rather than across two.

`src/metrics.rs:639` and `AGENTS.md:141` describe the `step` FIELD with "belongs to". Those predate this change and I do not raise them.

REMEDY: `src/workflow.rs:411-412`, one word. The helper name is a judgement call and I do not press it; if it moves, `step_attribution` to `step_join_phrase` or similar, and the doc at `:543` already says "joins".

### `W2B-6`: the plural refusal's trailing parenthetical does not say which owner it qualifies

`low`. `src/workflow.rs:556-573`.

`step_attribution` appends the mark to the owner it belongs to, which is unambiguous when the marked owner sorts first or when every owner is marked, and ambiguous when a marked owner sorts last. Fixture `g4-mixed-last` (a structured record joining `alpha-fold` to `alpha`, plus a pre-migration record for `alpha-fold`):

```
POST: PLAN vs LOG: round log line 4: increment waiver names step `beta` but the round log joins increment `alpha-fold` to steps `alpha`, `alpha-fold` (derived from a record's `task`)
```

A reader can attach "(derived from a record's `task`)" to the list rather than to `alpha-fold`, and conclude that `alpha` is derived too, which is the exact mis-statement of provenance the mark exists to prevent. Contrast `g5-mixed-first`, where the same code reads correctly because the marked owner sorts first:

```
POST: PLAN vs LOG: round log line 4: increment waiver names step `mmm` but the round log joins increment `zeta-inc1` to steps `aaa` (derived from a record's `task`), `zzz`
```

REMEDY: `src/workflow.rs:559-565`, the per-owner arm. Bind the mark to its slug so the reading cannot slide, for example `` `alpha-fold`, derived from a record's `task` `` becomes a form that repeats the slug, or list the derived owners in their own trailing phrase. The single-owner form needs no change.

### `W2B-7`: the empty-owners refusal asserts that no `type:"round"` record resolves to the increment when one does but was dropped as malformed

`low`. `src/workflow.rs:655-658`.

The message states a fact about the log, and the log is the thing the reader will grep. `metrics::parse_rounds` is best effort and drops a record missing any of `task`, `artifact`, `outcome`, `consecutive_clean` or `risk_class` (`src/metrics.rs:660-698`), so a record that carries `"increment":"<the waived id>"` can be invisible to the check while visible to the reader.

Fixture `h3-malformed`, one record carrying `"increment":"alpha-inc1","step":"beta"` and no `consecutive_clean`:

```
POST: LOG:1: missing field `valid_findings`
      PLAN vs LOG: round log line 3: increment waiver names increment `alpha-inc1`, which no `type:"round"` record resolves to (a record resolves to its structured `increment` id, or to its `task` when that id is absent), so the round log joins it to no step
```

Fixture `h4-malformed-repaired`, the identical log with `consecutive_clean` supplied:

```
POST: PLAN vs LOG: round log line 3: increment waiver names step `alpha` but the round log joins increment `alpha-inc1` to step `beta`
```

By the message's own stated rule the line 1 record DOES resolve to `alpha-inc1`. The schema problem printed beside it names `valid_findings`, a different field from the one that caused the drop, so the two lines do not connect for the reader. The verdict is defensible (a malformed record evidences nothing) and `validate` exits non-zero either way, which is why this is `low` and not higher.

This is not `W1B-2` re-raised: `W1B-2`'s remedy was to say which identity is matched, and the added parenthetical does that correctly (verified at `g7-taskonly` below). The malformed-record path is a different way for the same sentence to be false, and it survives the parenthetical.

REMEDY, if taken: `src/workflow.rs:655-658`, one clause noting that a record the log projection could not read does not count, or nothing, on the ground that the malformed line is always reported in the same run. I record it rather than rule it, since the choice is the implementer's.

## The full enumeration: every refusal string the changed code can emit

The changed code is `w3_problems`, `w5_problems`, `waiver_covers_round` and `step_attribution` (`src/plan/source.rs`'s change is a comment and emits nothing). Every `format!` site in those functions is listed, with an input that reaches it and the exact string the tool printed. The two locator forms (`round log line <n>` for JSONL, ``TOML waiver `<id>` `` for TOML) are covered.

| id | site | fixture | exact string emitted | true of that input |
| --- | --- | --- | --- | --- |
| M1 | `w3_problems`, `:481-484` | `x1-norecords`: `alpha` `complete`, empty log | ``Roadmap step `alpha` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"` for it if it predates logging or its review was skipped`` | YES. Unchanged text, and the only message in the changed code that states a remedy. |
| M2 | `w3_problems`, `:500-503` | `x2-riskclass`: one increment logging `low_risk` then `risky` | ``Roadmap step `alpha` increment `alpha-inc1` logs inconsistent risk_class values`` | YES. Unchanged text and unchanged path. |
| M3 | `w3_problems`, `:528-535`, guarded by the new shared predicate | `g10-w3-sibling`: `stall-incA` short, the only waiver names sibling `stall-incB` | ``Roadmap step `stall` increment `stall-incA` reached a consecutive-clean streak of 1 but its `risky` risk class needs 2`` | YES. The waiver names the right step and the wrong increment, so it exempts nothing, and the peak is 1 against 2. |
| M4 | `w5_problems`, `:625-628` | `x3-dangling`: `step`-unit waiver for `ghost` | ``round log line 1: `type:"waiver"` names step `ghost`, which is not a Roadmap step`` | YES. Unchanged text. |
| M5 | `w5_problems`, `:655-658`, EMPTY owners | `g6-empty`: waiver for `alpha-inc1`, log carries only `alpha-other` | ``round log line 3: increment waiver names increment `alpha-inc1`, which no `type:"round"` record resolves to (a record resolves to its structured `increment` id, or to its `task` when that id is absent), so the round log joins it to no step`` | YES. |
| M5b | same | `g7-taskonly`: one record with `"task":"alpha-inc1","increment":"alpha-inc2"` | identical string | YES, and this is the `W1B-2` case: the id IS a record's `task`, and the parenthetical is what makes the sentence checkable by a reader who greps for it. |
| M5c | same | `h3-malformed`: a record for the increment, dropped by the projection | identical string | NO, see `W2B-7`. |
| M6 | `w5_problems`, `:660-666` with `step_attribution`, ONE DECLARED owner | `g1-declared`: record joins `beta-incB` to `beta`, waiver names `alpha` | ``round log line 3: increment waiver names step `alpha` but the round log joins increment `beta-incB` to step `beta``` | YES. `beta` is carried verbatim in the record's `step`. |
| M7 | same, ONE DERIVED owner | `g2-derived`: pre-migration record `"task":"alpha-fold"`, waiver names `beta` | ``round log line 3: increment waiver names step `beta` but the round log joins increment `alpha-fold` to step `alpha-fold` (derived from a record's `task`)`` | YES. `alpha-fold` is not a Roadmap step in that plan and appears in no `step` field; the mark says where it came from. This is the recorded `src/` defect, now disclosed. |
| M8 | same, SEVERAL, all declared | `g3-plural-declared`: two records joining `shared-inc1` to `beta` and to `gamma` | ``round log line 4: increment waiver names step `alpha` but the round log joins increment `shared-inc1` to steps `beta`, `gamma``` | YES. Both are carried verbatim; no mark, correctly. |
| M9 | same, SEVERAL, mixed, derived LAST | `g4-mixed-last` | ``round log line 4: increment waiver names step `beta` but the round log joins increment `alpha-fold` to steps `alpha`, `alpha-fold` (derived from a record's `task`)`` | The facts are right, the attachment of the parenthetical is ambiguous. See `W2B-6`. |
| M10 | same, SEVERAL, mixed, derived FIRST | `g5-mixed-first`: an increment-only record with `"task":"aaa"`, plus a record declaring `zzz` | ``round log line 4: increment waiver names step `mmm` but the round log joins increment `zeta-inc1` to steps `aaa` (derived from a record's `task`), `zzz``` | YES, and unambiguous in this order. |
| M11 | same, SEVERAL, all derived | `h5-two-derived`: two increment-only records with different `task` values | ``round log line 4: increment waiver names step `mmm` but the round log joins increment `shared-x` to steps `aaa` (derived from a record's `task`), `bbb` (derived from a record's `task`)`` | YES. The mark repeats per owner. |
| M12 | same, TOML locator, declared | `t1-declared`: `[[step.waiver]]` nested on `alpha`, records join `shared-inc1` to `beta` | ``TOML waiver `w`: increment waiver names step `alpha` but the round log joins increment `shared-inc1` to step `beta``` | YES. The waiver's `step` comes from the step it nests on, so the contradiction is authored by the nesting. |
| M13 | same, TOML locator, derived | `t2-derived`: `[[step.waiver]]` nested on `beta`, pre-migration record for `alpha-fold` | ``TOML waiver `w`: increment waiver names step `beta` but the round log joins increment `alpha-fold` to step `alpha-fold` (derived from a record's `task`)`` | YES. This is the pair the round 1 triage used to settle `W1B-1`; post-fix the derived value is marked. |
| M14 | `w5_problems`, `:696-699` | `x4-evidence`: record-backed waiver, no escalation | ``round log line 2: `record-backed` waiver cites evidence `alpha-inc1` but no `type:"escalation"` record with `human_decision` `decision` is scoped to this waiver's unit`` | YES. Unchanged text; the fixture gives the waiver its ownership records so this is the only problem. |
| M15 | `w5_problems`, `:708-713` | `x5-pairing`: `accepted-at-escalation` with `self-declared` | ``round log line 1: waiver reason `accepted-at-escalation` must not carry evidence tier `self-declared``` | YES. Unchanged text. |

Two properties of the set, checked because the lens asks for them:

- SELF-CONTAINED. Every term a message uses is explained where the reader is: `type:"round"` and `type:"waiver"` are the schema's own names, the empty-owners message explains inline which identity it matched, and the derived mark names the field the value came from. No message names an internal function: `leading_slug` appears in the shipped rule text and in no emitted string.
- WHAT TO DO. Only M1 states a remedy. The five W5 messages state the contradiction and stop, which is the pre-existing house form for W5 (M4, M14, M15 are unchanged and do the same), so the new messages are consistent with their siblings rather than a regression. I raise no finding on it and record it so a later pass does not have to re-derive it.

## The marker sweep, both directions

The mark is `declared = round.step.is_some()`, merged per owner with `|=` over the increment's records (`src/workflow.rs:644-653`). I tried to break it in both directions and could not, within the per-increment scope the code chose.

- AN OWNER MARKED DERIVED THAT THE LOG DOES STATE: reachable only ACROSS increments (`g9-cross-increment`), which makes the comments wrong (`W2B-4`) and leaves the message true.
- AN OWNER LEFT UNMARKED THAT NO RECORD STATES: not reachable. `declared` is true only for a record whose `step` is `Some`, and `round_step_slug` returns exactly that value, so an unmarked owner is always a byte-identical `step` value on some record of that increment. The one candidate hole, an empty `"step":""`, is closed upstream: `parse_rounds` filters an empty string to `None` (`src/metrics.rs:697`), so such a record falls back to the shim and is marked derived rather than declaring an empty slug. Fixture `f12-emptystep` confirms it end to end: the record's step resolves to `alpha` through `leading_slug("alpha-inc1")`, the waiver names `alpha`, and W5 stays silent.
- THE PER-OWNER OR AT ITS BOUNDARY: `g8-or-merge` (one record declares what another derives, one owner, unmarked) and `h5-two-derived` (no record declares anything, every owner marked). Both behave as the code's own comments at `:637-643` describe and as `w5_does_not_mark_an_owner_a_record_declares_even_when_another_derives_it` pins.

## The three shipped prose copies

BYTE-IDENTICAL, verified rather than assumed:

```
sed -n '147p' AGENTS.md; sed -n '147p' .agents/AGENTS.reference.md; sed -n '11p' pack/instrument.md
-> md5 b02e698ef4ac2f82a0cb742b06a30ac0 for all three, `diff` empty both ways
```

Acceptance item 7b, re-run because the fix pass rewrote the sentence it greps for:

```
grep -c -F "the increment's leading slug equals the step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md   -> 0, 0, 0
grep -c -F "some `round` record must join that increment to that step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md   -> 1, 1, 1
```

No fourth shipped copy exists: a tree-wide grep for the retired wording outside `docs/plans/` returns nothing, `README.md` states no ownership rule, and the remaining hits are the self-quoting records item 7b's own note anticipates.

EACH HALF OF THE NEW CLAUSE, CHECKED AGAINST THE TOOL RATHER THAN AGAINST THE CODE:

| clause | fixture | verdict |
| --- | --- | --- |
| "some `round` record must join that increment to that step" | `g1`, `g13-unblocking` | TRUE. `g13` is the unblocking: an id the shim leaves whole (`beta-fold`), joined by the records to `beta`, refused by `PRE` and accepted by `POST`. |
| "the record's structured `increment` id, or its `task` when that id is absent, equals the waived increment" | `g7-taskonly` (a record whose `task` is the waived id and whose structured `increment` is not) | TRUE. The record does not count, and the message says why. |
| "its structured `step` slug, or `leading_slug(task)` when that id is absent, equals the waived step" | `g2-derived`, `h2-rehomed-derived` | TRUE on both routes. |
| "resolving each axis exactly as the escalation join below does" | read against `escalation_step_slug`/`escalation_increment_id` at `src/workflow.rs:134-143` | TRUE. The accessors mirror, and the clause restates both fallbacks verbatim rather than relying on the cross-reference, which is `W1B-4`'s remedy landed. |
| "an increment no `round` record resolves to is reported" | `g6-empty` | TRUE. |
| "a step reached through the `leading_slug` fallback is reported as derived" | `g8-or-merge` | FALSE. See `W2B-1`. |
| "because such a step need not appear in the Roadmap or anywhere in the log" | `g2-derived` (absent from both), `g9-cross-increment` (present in the log) | TRUE. "need not" is a possibility claim and both cases occur. |

## The drift-guard demonstration

Established by experiment in a scratch copy of HEAD, one file reverted at a time, `CARGO_TARGET_DIR` outside every tree. The guard is `agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render`.

```
baseline (all three copies in step)          -> ok. 1 passed
AGENTS.md alone reverted to main             -> FAILED: root AGENTS.md has drifted from a fresh pack render
.agents/AGENTS.reference.md alone reverted   -> FAILED: .agents/AGENTS.reference.md has drifted from a fresh pack render
pack/instrument.md alone reverted            -> FAILED: root AGENTS.md has drifted from a fresh pack render
restored                                     -> ok. 1 passed
```

So the three copies cannot drift apart silently, and a fix pass that treats `W2B-1` must move all three. The guard is still blind to the thing `W2B-1` is: it compares the committed copies against a fresh render of the pack and never reads `src/`, exactly as acceptance item 1 warns, so a sentence that is false of the code passes it. Item 7b is a fixed-string check on one phrase and does not reach the marking clause either.

## Checked and correct

Recorded because an exhaustive sweep that finds nothing is itself the evidence.

- `CHANGELOG.md:32`, "A refusal now names the step each round record joins the increment to, read from that record's structured `step` id where it carries one": TRUE (`g1`, `g3`, `t1`).
- `CHANGELOG.md:32`, "both checks now consult ONE predicate over `round_step_slug`/`round_increment_id` and cannot drift": TRUE. W3's exemption test is `waiver_covers_round` over the increment's own records (`:524-526`), W5's is the same function over every round (`:636`).
- `CHANGELOG.md:32`, "That case grants nothing under W3 either (W3 builds its increments from the records, so an increment with none never enters the loop)": TRUE, and it also holds of the population `W2B-2` names, since W3 requires the waiver's `step` to be the step the records join to.
- `CHANGELOG.md:32`, "No waiver committed to this project's own plan is affected": TRUE, measured on the live plan and log in this worktree, unmodified. `PRE` and `POST` both report `workflow invariants hold` at exit 0 over 319 records, 96 steps and 70 questions.
- `CHANGELOG.md:32`, "the step carrying it could not be marked `complete`": NOT RAISED. `W1C-1` settled it and I have no evidence beating that ruling.
- `src/plan/source.rs:792-798`, the added paragraph ("this one asks whether the step DECLARES the increment, W5 asks whether the ROUND LOG joins the increment to the step ... Both must hold, and neither substitutes for the other"): TRUE, demonstrated at `t3-undeclared`, where one run prints both ``SRC: waiver `w` on step `alpha` names increment `shared-inc1`, which is not one of the step's increments`` and the W5 ownership refusal, from the two different checks.
- `src/workflow.rs:517-523`, W3's new inline comment ("Every record in the group carries this increment and this step, so asking any one of them asks the group"): TRUE. The group is filtered on `round_step_slug(round) == step.slug` and keyed on `round_increment_id`, so both axes are constant within it.
- `src/workflow.rs:422-425`, `waiver_covers_round`'s signature argument ("A caller cannot pass the waiver's own `step` and collapse the comparison into comparing a value with itself"): TRUE of the signature, which takes `&Round`.
- `src/workflow.rs:1636`, the empty-case test comment ("it names no step at all", round 1's `:1564-1566`): TRUE, unchanged from the round 1 ruling that owed it no edit.
- `leading_slug`'s doc comment (`:71-87`) and the four-accessor block (`:98-111`): unchanged by this diff and still true after it; the fix REDUCES `leading_slug`'s call sites and contradicts nothing they say.
- The module-level W5 summary (`:21-26`) is mechanism-agnostic ("must own its `increment`") and did not need to move.

## Notes for the post-merge planner, not findings against this artifact

The sidecar and the `Q-70` entry are known stale and their staleness is not a finding. Two specific facts a later pass would otherwise miss:

- Acceptance item 5 (`validation-constraints.md:124`) says to "confirm W5 no longer reports a step derived from the id". After this fix W5 STILL reports a derived step on the non-empty pre-migration branch, by design and marked as derived (`g2-derived`, `t2-derived`); what it no longer does is report one on the branch item 5's own fixture reaches. The item's sentence needs re-scoping to the empty-owners branch, or it will read as falsified by a correct build.
- Acceptance item 7b's replacement-wording grep now has to be the new sentence, "some `round` record must join that increment to that step", which I ran above and which reports 1, 1, 1. The retired-wording half still reports 0, 0, 0.

## Overall

The messages are in much better shape than round 1 found them: every string the changed code emits is true of its input except in the one corner `W2B-7` names, with one wording ambiguity at `W2B-6`; the derived mark is exactly right within the scope the code chose; and the recorded `src/` defect is now disclosed rather than presented as fact. What is still wrong is what the change SAYS ABOUT ITSELF: one shipped sentence and one `CHANGELOG` sentence state the marking rule per record where the code merges per owner, the `CHANGELOG` names one narrowed population out of two, its account of the retired rule overstates by one case, and three doc comments describe the mark more broadly than the code computes it. The ceiling is `medium`, carried by the `CHANGELOG`'s population claim, which is the entry's own headline disclosure of a deliberate break.
