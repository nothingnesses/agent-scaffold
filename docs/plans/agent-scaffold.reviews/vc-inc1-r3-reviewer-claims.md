# `validation-constraints-inc1`, round 3: reviewer, the claims lens

Reviewer worktree: `.claude/worktrees/rev-inc1r3-claims`, branch `review/inc1r3-claims`, at `651ff63`.
Artifact: `git diff main..HEAD` (`.agents/AGENTS.reference.md`, `AGENTS.md`, `CHANGELOG.md`, `pack/instrument.md`, `src/plan/source.rs`, `src/workflow.rs`), three commits (`0110828` the implementation, `86e00ed` the round 1 fix pass, `651ff63` the round 2 fix pass, which is the shortening).
Specification: `docs/plans/agent-scaffold.steps/validation-constraints.md`, increment 1 and the Acceptance section.
Settled verdicts read before starting: `vc-inc1-r1-triage.md` and `vc-inc1-r2-triage.md`.

MY LENS: did the shortening work, and is what remains true. I enumerated every claim the change still makes about the ownership rule, established each by running the tool or by reading the code it describes, followed every pointer to its target, and measured what the shortening deleted.

EVERY MEASUREMENT BELOW IS MINE, taken in the trees named. Nothing is carried from a previous round's report on trust; where a previous round's fact is cited I re-took it.

## Result

TWO VALID FINDINGS, both `low`. NO `medium`, NO `high` AND NO `critical` FINDING WAS FOUND, and I say so explicitly rather than promoting a weaker one to fill the tier.

| id | claim | severity |
| --- | --- | --- |
| `W3A-1` | `src/workflow.rs:1656`: the unblocking test's comment infers that an increment id which does not end `-inc<alnum>` "can never equal the step slug", which is false by the one class the round 2 triage measured and had the `CHANGELOG` corrected for. | `low` |
| `W3A-2` | The shipped rule text in all three copies justifies the derived mark with "such a step is one the join computed rather than one the log carries", a categorical claim about the whole log that the per-increment mark does not support and that a fixture falsifies. Introduced BY the shortening, replacing a hedged "need not" the previous round measured as true. | `low` |

BOTH ARE IN WHAT THE CHANGE SAYS. NEITHER IS A DEFECT IN WHAT THE TOOL DOES. I found no tree on which the shipped build returns a wrong verdict, and I looked: eleven fixtures of my own construction across two binaries, plus the live plan and log, plus one mutation.

DID THE SHORTENING WORK? PARTLY, AND I MEASURED IT RATHER THAN JUDGING IT BY THE COMMIT MESSAGE. It closed the four claim findings it aimed at (`W2A-1`, `W2A-4`, `W2B-4`, `W2B-5`, and `W2B-6` and `W2B-7` alongside them), it cut production comment lines in `src/workflow.rs` from 331 to 320, and every pointer it introduced resolves to a target that says what the pointer claims. It did NOT reduce the NUMBER of sites that restate the rule, only the length of each, and one of my two findings is against a sentence the shortening itself wrote. That is the round 2 mechanism attenuated, not removed. Details under "Did the shortening remove the mechanism".

## Trees, binaries and fixtures

All under `<scratch>/rev-r3-claims`, a subdirectory of my own naming. ONE `CARGO_TARGET_DIR` PER BINARY, verified distinct:

```
bfb99d9d9af4e7fc02360fd9e1eef213  target-head/debug/agent-scaffold     (git archive HEAD, 651ff63)
eadf009b776f209652b675432805c85b  target-prefix/debug/agent-scaffold   (git archive main)
```

`target-drift` and `target-m10` are separate target directories again, used for the drift-guard reverts and for the one mutation. `TMPDIR` pointed at `<scratch>/rev-r3-claims/tmp`, outside every git repository, per the Acceptance preamble. `src/` was mutated ONLY in the scratch copy `<scratch>/rev-r3-claims/m10`; my worktree carries this file and nothing else.

Control, HEAD unmutated, in `<scratch>/rev-r3-claims/head`:

```
cargo test  ->  386 + 5 + 1 + 1 + 9 + 3 + 20 + 1 + 4 = 430 passed; 0 failed
cargo clippy --all-targets -- -D warnings  ->  exit 0
```

Live plan and log, unmodified, in my worktree (Acceptance item 2):

```
target-head    validate --source docs/plans/agent-scaffold.plan.toml --workflow
  -> 321 records valid, 96 steps, 70 questions, workflow invariants hold, exit 0
target-prefix  same command
  -> identical, exit 0
grep -c '^unit = "increment"$' docs/plans/agent-scaffold.plan.toml  -> 13
```

So the `CHANGELOG`'s "No waiver committed to this project's own plan is affected" is a substantive claim and not a vacuous one: thirteen increment-unit waivers are committed and all thirteen satisfy the new rule with no edit.

Eleven fixtures, each its own project root with `docs/plans/t.md` and `docs/metrics/workflow.jsonl`, built by `<scratch>/rev-r3-claims/mkfx.py` and `mkfx2.py`; two more TOML-substrate roots built by `mktoml.py`. Every round record is schema-complete and every increment's streaks are consistent, so the ONLY problem any fixture reports is the one it was built for. Command in every case: `agent-scaffold validate --plan <root>/docs/plans/t.md --workflow`, or `--source <root>/docs/plans/t.plan.toml --workflow` for the TOML pair.

VERDICTS, `prefix` AND `head`:

```
f-ormerge              prefix=1 head=1   one record declares `alpha`, one derives it
f-derived              prefix=1 head=1   only a pre-migration record
f-mixed-last           prefix=1 head=1   two owners, the derived one sorting last
f-allderived           prefix=1 head=1   every owner derived, plural
g9-cross-increment     (head=1)          another increment's record declares the marked step
h1-rehomed             prefix=0 head=1   records resolve but join to another step
h4-repaired            prefix=0 head=1   the same, from the h3 log repaired
g-noresolve            prefix=0 head=1   no record resolves to the waived increment
g12-retired-accepts    prefix=0 head=0   increment id EQUALS the step slug
unblock                prefix=1 head=0   an id that does not strip to its step, joined by the log
h3-malformed           prefix=1 head=1   a schema-rejected record carries the waived increment id
declared-but-rehomed   (head)            TOML: declared on `alpha`, log joins it to `beta`
joined-but-undeclared  (head)            TOML: joined to `alpha`, not declared on it
```

## The full claim enumeration

EVERY CLAIM THE CHANGE STILL MAKES ABOUT THE OWNERSHIP RULE IS BELOW, including the ones that are true, because an exhaustive sweep that finds almost nothing is the evidence that the mechanism is closing. "Read" means I read the code the claim describes; "ran" means a fixture or a mutation settled it.

### Production comments and messages, `src/workflow.rs`

| # | site | claim | established | verdict |
| --- | --- | --- | --- | --- |
| P1 | `:411-415` | `waiver_covers_round` joins on `round_increment_id` and `round_step_slug`, "so a record carrying the structured Inc 2 ids joins on them and a pre-migration record falls back per axis" | read `:426-433` against `:119-129` | TRUE |
| P2 | `:417-420` | W3 and W5 "both consult this one implementation, so the two cannot drift"; W3 asks it of a `complete` step's own records, W5 of EVERY round | RAN the mutation (below); read `:526` and `:625` | TRUE |
| P3 | `:422-425` | the predicate takes the round, so "a caller cannot pass the waiver's own `step` and collapse the comparison into comparing a value with itself, which is the mutation acceptance check 4b exists to catch (Principle 13)" | read the signature `:426-429`; acceptance 4b at `validation-constraints.md:123` names exactly that mutation; Principle 13 at `AGENTS.md:126` is "Make illegal states unrepresentable" | TRUE |
| P4 | `:517-523` | W3 exempts via the shared predicate over the increment's own records; "Every record in the group carries this increment and this step, so asking any one of them asks the group" | read `:472-473` (filter on `round_step_slug == step.slug`) and `:491-494` (key on `round_increment_id`) | TRUE |
| P5 | `:543-546` | `owners` maps each step to whether "a record OF THAT INCREMENT declared it in a structured `step` id"; one none declared "was produced by `round_step_slug`'s fallback and is marked derived, so the refusal never offers a computed step as a recorded one" | read `:631-640`; RAN `f-ormerge` (declared -> unmarked) and `f-derived` (none declared -> marked) | TRUE, and this is round 2's `W2A-1` CLOSED |
| P6 | `:546-547` | POINTER: "Which records reach which value is the accessor block's property, not restated here" | followed to `:98-111` plus the accessors at `:113-129` | TRUE, see "Pointers followed" |
| P7 | `:549-550` | "A marked owner names its own slug inside the mark, because a trailing parenthetical after a list can be read as qualifying the list (round 2, `W2B-6`)" | read `:560-565`; RAN `f-mixed-last` | TRUE, `W2B-6` CLOSED |
| P8 | `:574-578` | the ownership rule itself: "some `type:\"round\"` record must resolve to that increment id AND join to that step" | RAN `h1-rehomed`, `h4-repaired`, `unblock` | TRUE |
| P9 | `:579-582` | POINTER: "BOTH AXES STILL DEGRADE PER RECORD, per the accessor block above, so a step the refusal names MAY BE one `round_step_slug` computed rather than one a record carries"; "Retiring the derivation would mean changing `round_step_slug`, which W3 shares, and no decision has asked for that" | followed the pointer; read `:473` for the sharing; read the `Q-70` receipt's three options, none of which is retiring the derivation | TRUE |
| P10 | `:583-586` | the empty case is reported "(receipt `Q-70-emptycase`)"; it "NARROWS what a waiver may cover against the retired lexical rule, which accepted such a waiver silently" | receipt verified in the live log, options and `chosen` quoted below | TRUE under the narrowing reading; IMPRECISE, NOT RAISED, reasoning below |
| P11 | `:619-622` | ownership is "evidenced by the round log rather than by the WAIVED INCREMENT ID's leading slug (Q-70)", and "so is a waiver no round record resolves to, which owns nothing yet" | RAN `g-noresolve` | TRUE |
| P12 | `:626-630` | "Each step the log DOES join THIS increment to, mapped to whether one of its own records declared that step in a structured `step` id" | read `:631-640`; RAN `g9-cross-increment` (the per-increment scope is what the code applies) | TRUE, round 2's `W2B-4` scope half CLOSED |
| P13 | `:642-645` | the empty-owners refusal, including "(by its structured `increment` id, else its `task`; a record the schema check rejected is not read)" | read `parse_rounds` (`src/metrics.rs:660-711`) against `check_record` (`:435-474`) and `validate_log` (`:989-1007`); RAN `h3-malformed` and `h4-repaired` | TRUE, and complete; round 2's `W2B-7` CLOSED. See the analysis below |
| P14 | `:647-653` | the non-empty refusal, "the round log joins increment `X` to <attribution>" | RAN `f-ormerge`, `f-derived`, `f-mixed-last`, `f-allderived`, `g9-cross-increment`, `h1-rehomed` | TRUE |

P13 IS STRONGER THAN THE SENTENCE NEEDS AND I CHECKED IT RATHER THAN ACCEPTING IT. `parse_rounds` drops a `round` line when it is not JSON, not an object, has no string `task` or `artifact`, has no parseable `outcome` or `risk_class`, or has no `consecutive_clean` that reads as `u64`. `check_record` rejects a `round` on every one of those conditions (`require_str`, `require_enum`, `require_count`, all reusing the same parsers) and `validate_log` reports a non-JSON line as "invalid JSON". So the set of `round` records the check does not read is EXACTLY the set the schema check rejects, and the parenthetical is not merely safe but exhaustive. `h3-malformed` prints both lines in one run:

```
<root>/docs/metrics/workflow.jsonl:1: missing field `consecutive_clean`
<root>/... : round log line 2: increment waiver names increment `alpha-inc1`, which no `type:"round"` record resolves to (by its structured `increment` id, else its `task`; a record the schema check rejected is not read), so the round log joins it to no step
```

and `h4-repaired`, the identical log with the field supplied, refuses on the ownership rule instead.

P10, WHY I DO NOT RAISE IT. "That NARROWS what a waiver may cover against the retired lexical rule, WHICH ACCEPTED SUCH A WAIVER SILENTLY" carries no "whenever the id happened to strip to the step slug" qualifier, where both the `CHANGELOG` entry and this same file's sibling test comment (`:1619-1621`) do carry it. Taken as a general claim about the retired rule it is false: with a waiver naming step `alpha` for an unlogged increment `beta-fold`, the retired rule REPORTED rather than accepted. Taken as a claim about the narrowed population, which is what a sentence introduced by "That NARROWS" is about, every waiver in that population was accepted by the retired rule and the clause is true. The second reading is the natural one for the sentence's own subject, so I judge it imprecise rather than false and do not raise it. I record it here so a triager can disagree with a citation rather than having to find it.

### The `src/plan/source.rs` comment

| # | site | claim | established | verdict |
| --- | --- | --- | --- | --- |
| P15a | `:792-793` | the membership check "has no `check_record` ancestor: that arm reads one record and cannot see a step's declared increments" | read `check_record(value: &Value)` at `src/metrics.rs:435`, which takes exactly one record | TRUE |
| P15b | `:794-795` | POINTER: "an internal cross-reference of this document, the same class as `[step.provenance].decisions` above" | followed to `src/plan/source.rs:642-653`, which cross-references each provenance decision id against the same document's registered questions, and is above (`:642` < `:799`) | TRUE |
| P15c | `:795-798` | "this one asks whether the step DECLARES the increment, W5 asks whether the ROUND LOG joins the increment to the step (Q-70). Both must hold, and neither substitutes for the other" | RAN both directions on the TOML substrate | TRUE |

THE TWO DIRECTIONS, which is the only way to settle "neither substitutes for the other" rather than assert it. `declared-but-rehomed` declares `alpha-inc1` on step `alpha` and nests the waiver there, while the log joins that increment to `beta`:

```
head  <root>/docs/plans/t.plan.toml vs <root>/docs/metrics/workflow.jsonl: TOML waiver `w`: increment waiver names step `alpha` but the round log joins increment `alpha-inc1` to step `beta`
```

The membership check is silent (the increment IS declared) and W5 fires. `joined-but-undeclared` waives `alpha-inc9` on step `alpha` while declaring only `alpha-inc1`, with the log joining `alpha-inc9` to `alpha`:

```
head  <root>/docs/plans/t.plan.toml: waiver `w` on step `alpha` names increment `alpha-inc9`, which is not one of the step's increments
```

The membership check fires and W5 is silent. Both problems accumulate into one `validate` run (`src/main.rs:882` and `:1016` push into the same `problems` vector), so "both must hold" is the exit-0 condition rather than a figure of speech.

### Test comments

| # | site | claim | established | verdict |
| --- | --- | --- | --- | --- |
| T1 | `:812-816` | `owning_round_line`'s rule "consults only the two join axes, so the outcome, streak and risk class are filler here" | read every call site: each is a `w5_problems` call or a `check_workflow_toml` call whose steps are `in-progress`, so W3 never reads the filler; one record per increment keeps the consistency check quiet | TRUE |
| T2 | `:1013-1018` | "This case pins the increment axis on W3's side; `w5_flags_an_increment_waiver_whose_increment_has_no_round_records` pins it on W5's, and a build that dropped the axis fails both" | RAN the mutation: exactly those two tests failed | TRUE, round 2's `W2A-4` CLOSED |
| T3 | `:1379-1381`, `:1505-1507`, `:1563-1565`, `:2125-2126` | "the ONE problem asserted is the evidence join and not Q-70's ownership rule" | each test asserts `problems.len() == 1` and the suite is green | TRUE |
| T4 | `:1582-1592` | `beta` "goes unmarked in the message because the record DECLARES it in a structured `step`"; without the records "a build that compared the waiver's `step` with itself would still pass it" | read `owning_round_line`; traced the step-axis-dropped build against an empty log, where the empty-owners branch fires regardless | TRUE |
| T5 | `:1615-1631` | the human decided `Q-70-emptycase` as REPORT IT "over staying silent and over reporting only when the log is non-empty" | verified the receipt verbatim, quoted below | TRUE |
| T6 | `:1654-1658` | "an increment id that does not end `-inc<alnum>`, so `leading_slug` returns it unchanged AND IT CAN NEVER EQUAL THE STEP SLUG" | RAN `g12-retired-accepts` | FALSE by one class -> `W3A-1` |
| T7 | `:1674-1677` | "Here two records carry DIFFERENT structured `step` ids ... Both owners are declared, so neither is marked derived" | the test asserts `!contains("derived")` and is green | TRUE, and the retired "first of the two routes" enumeration is gone (`W2B-4` enumeration half CLOSED) |
| T8 | `:1704-1710`, `:1737-1740` | the derived value "need not be a Roadmap step and need not occur anywhere in the log"; the mark "names its own slug, so it cannot be read as qualifying the whole list"; "the derived owner sorts LAST here" | the test's own `assert_eq!` pins that `alpha-fold` is not a Roadmap step; `BTreeMap` orders `alpha` before `alpha-fold`; RAN `f-mixed-last` | TRUE |
| T9 | `:1755-1759` | the scan "must reach it by the structured `increment` id (not by `task`), and the mark must read the absent `step` (not the present `increment`)" | the test is green and its message names `zzz-task`, which is `leading_slug(task)` | TRUE |
| T10 | `:1783-1791` | the mark is per OWNER, and "the merge is a union and not first-write-wins" | the test runs both file orders and is green; read `*seen \|= declared` at `:638` | TRUE |
| T11 | `:2145-2152` | on the TOML substrate the message "names `beta` unmarked because the record DECLARES it in a structured `step`, not because the id strips to it"; "Both steps are `in-progress`, so only W5 speaks" | `leading_slug("shared-inc1")` is `shared`, not `beta`; the test asserts `problems.len() == 1` | TRUE |

The `Q-70` receipts, read from the live log, which settle T5 and P9 and P10:

```
jq -r 'select(.type=="decision" and (.q_id|test("^Q-70"))) | [.q_id, (.options|join(" | ")), .chosen] | @tsv' docs/metrics/workflow.jsonl

Q-70            Round-log join, direction (iii) | Retire W5 ownership rule, direction (iv) | Put both to a build, decide on measurement   -> Round-log join, direction (iii)
Q-70-emptycase  Report it | Stay silent on it | Report it, but only when the log is non-empty                                             -> Report it
Q-70-inc1close  Fix by shortening, then one more round | Fix, then accept and merge | ...                                                -> Fix by shortening, then one more round
```

### The three shipped prose copies

| # | claim | established | verdict |
| --- | --- | --- | --- |
| S1 | "an `increment`-unit waiver's `step` must own its `increment` (some `round` record must join that increment to that step ...)" | RAN `h1-rehomed`, `unblock` | TRUE |
| S2 | POINTER: "resolving each axis exactly as the escalation join below does" | followed to the escalation clause later in the same bullet, and compared it with the accessors | TRUE, see below |
| S3 | "an increment no `round` record resolves to is reported" | RAN `g-noresolve` | TRUE |
| S4 | "an owning step that no record of that increment declares in a structured `step` id is reported as derived" | RAN `f-ormerge` (unmarked), `f-derived` (marked), `g9-cross-increment` (marked, per-increment scope) | TRUE, `W2A-1` CLOSED |
| S5 | "since such a step is one the join computed rather than one the log carries" | RAN `g9-cross-increment` | FALSE -> `W3A-2` |

BYTE-IDENTICAL, one command over the `type: "waiver"` bullet in all three files:

```
for f in pack/instrument.md AGENTS.md .agents/AGENTS.reference.md; do grep '^- `type: "waiver"`' $f | md5sum; done
e74cc93448b5baecdf7de5d057e93af9  (pack/instrument.md,   line 11)
e74cc93448b5baecdf7de5d057e93af9  (AGENTS.md,            line 147)
e74cc93448b5baecdf7de5d057e93af9  (.agents/AGENTS.reference.md, line 147)
```

Acceptance item 7b, re-run because the shortening moved the sentence again:

```
grep -c -F "the increment's leading slug equals the step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md          -> 0, 0, 0
grep -c -F "some `round` record must join that increment to that step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md -> 1, 1, 1
grep -c -F "a step reached through the `leading_slug` fallback is reported as derived" ... (round 2's retired wording)      -> 0, 0, 0
```

DRIFT GUARD ESTABLISHED, not assumed, by reverting ONE file at a time in `<scratch>/rev-r3-claims/drift` (a copy of HEAD) and running `the_committed_scaffold_matches_a_fresh_render`. All three sites are guarded, which is more than the previous rounds demonstrated:

```
AGENTS.md alone reverted to main            -> FAILED, "root AGENTS.md has drifted from a fresh pack render"
.agents/AGENTS.reference.md alone reverted  -> FAILED, ".agents/AGENTS.reference.md has drifted from a fresh pack render"
pack/instrument.md alone reverted           -> FAILED, "root AGENTS.md has drifted from a fresh pack render"
```

So neither a generated copy nor the pack source can move without the other two, and a fix pass to `W3A-2` must move all three in one commit.

### The `CHANGELOG` entry, `CHANGELOG.md:32`

| # | claim | established | verdict |
| --- | --- | --- | --- |
| C1 | the rule "now decides which step owns a waived increment from the ROUND LOG" | RAN `h1-rehomed`, `unblock` | TRUE |
| C2 | "The retired rule required the waived increment id's leading slug to equal the waiver's `step`" | read `main:src/workflow.rs:564`, `if leading_slug(increment) != waiver.step` | TRUE |
| C3 | "an increment id that does not strip to its step could not be waived at all" | RAN `unblock`: `prefix` exits 1 on the ownership rule, `head` exits 0 | TRUE, and the wording CORRECTLY EXCLUDES the degenerate class, so round 2's `W2B-3` is CLOSED |
| C4 | "so the step carrying it could not be marked `complete`" | settled by round 1's `W1C-1` dismissal against `validation-constraints.md:3` | SETTLED, not re-raised |
| C5 | "the refusal named a step derived from that id, which need not be a Roadmap step" (of the RETIRED rule) | RAN `f-derived` on `prefix`: "increment `alpha-fold` belongs to step `alpha-fold`", and the Roadmap carries `alpha` and `beta` only | TRUE |
| C6 | "both checks now consult ONE predicate over `round_step_slug`/`round_increment_id` and cannot drift" | RAN the mutation: one edit, one W3 test and one W5 test red | TRUE |
| C7 | "TWO POPULATIONS ARE NARROWED, both of which the retired rule accepted whenever the waived id happened to strip to the step slug" | RAN `g-noresolve` (prefix 0 -> head 1) and `h1-rehomed`/`h4-repaired` (prefix 0 -> head 1); in each the waived id's leading slug equals the waiver's `step` | TRUE, round 2's `W2B-2` CLOSED |
| C8 | "that case grants nothing under W3 either (W3 builds its increments from the records ...)" | read `:491-494`; the parenthetical is attached to population 1 only, which is the scoping round 2's remedy required | TRUE |
| C9 | "No waiver committed to this project's own plan is affected" | RAN the live plan on both binaries, exit 0 both; 13 increment-unit waivers committed | TRUE |
| C10 | "DERIVATION IS NOT FULLY RETIRED and a refusal marks where it happened ..." | RAN `f-derived`, `f-allderived` | TRUE |
| C11 | "The rule text ships in `pack/instrument.md` and its two generated copies" | byte-identity and the drift guard above | TRUE |
| C12 | NO FREQUENCY CLAIM about either population | a case-insensitive word grep over the entry for common/commonly/typical/typically/often/frequent/frequently/most/usual/usually/rare/rarely/likely/"real project"/"in practice" returns NOTHING | TRUE |

C12 IS THE ONE I WAS ASKED TO CHECK HARDEST, because round 2's triager measured zero instances of the second population in this repository's live log and explicitly rejected the finding's "the second is the one a real project hits" framing. The entry names the second population by its shape and by its consequence ("is now reported instead of accepted while the records contradict it, which is the case this change exists to close") and asserts nothing about how often it occurs. "The case this change exists to close" is a purpose claim, and it is the triager's own language from `vc-inc1-r2-triage.md:264`. I re-measured the repository's own log for the shape and reached the same zero, so the entry claiming frequency would have been a finding and it does not.

## Pointers followed

FOUR POINTERS, each followed to its target with the target read.

1. `pack/instrument.md:11` and its two copies, "resolving each axis exactly as the escalation join below does". TARGET: the escalation clause later in the SAME bullet, which spells both fallbacks verbatim: "the escalation's structured `increment` id, or its `task` when that id is absent, equals the waived increment; or its structured `step` slug, or `leading_slug(task)` when that id is absent, equals the waived step". COMPARED against the accessors: `round_increment_id` is the structured `increment` else `task` (`src/workflow.rs:127-129`), `round_step_slug` is the structured `step` else `leading_slug(task)` (`:119-121`). IDENTICAL on both axes. The pointer says what its target says, the target is "below" as claimed, and the AND across the two axes is carried by the pointer's own first half ("some `round` record must join that increment to that step") rather than left to the target. ROUND 1's `W1B-4` STAYS CLOSED: a reader is no longer told one join degrades and left to infer about the other, because the sentence now says the two resolve identically and the reader can read the fallbacks in the clause it names.
2. `src/workflow.rs:546-547`, "Which records reach which value is the accessor block's property, not restated here". TARGET: `:98-111`, which states that the two axes resolve INDEPENDENTLY, that each accessor "falls back on its OWN field alone", and that an `increment`-only record still resolves its step through `leading_slug(task)`; plus the four accessors at `:113-143` immediately below it. The target does state which records reach which value. The referent is unnamed in the source (the block calls the accessors "the four join accessors", not "the accessor block"), but a grep for "accessor" in the file lands on it and on the two references to it, so the pointer is followable.
3. `src/workflow.rs:579`, "per the accessor block above". SAME TARGET, and here it is qualified with "above", which is correct (`:98` < `:579`). The claim it carries, "BOTH AXES STILL DEGRADE PER RECORD", is exactly what the target states.
4. `src/plan/source.rs:794-795`, "the same class as `[step.provenance].decisions` above". TARGET: `src/plan/source.rs:642-653`, which flags a provenance decision id that "names no question", that is, an internal cross-reference of the same TOML document against its own registered questions. Same class, and above.

NO POINTER POINTS AT THE WRONG PLACE, AND NO TARGET FAILS TO SAY WHAT ITS POINTER CLAIMS. That is the strongest single result of this round, because a bad pointer would have been worse than the restatement it replaced.

## What the shortening lost

I diffed `86e00ed` (before the shortening) against `651ff63` (after) and checked each deleted clause for a fact no remaining site carries.

DELETED, AND THE FACT SURVIVES ELSEWHERE:

- The shipped prose's verbatim per-axis enumeration ("the record's structured `increment` id, or its `task` when that id is absent ..."). REPLACED BY POINTER 1, whose target states both fallbacks. No loss.
- `step_attribution`'s "SEVERAL OWNERS ARISE TWO WAYS AND NEITHER NEEDS A MALFORMED LOG" enumeration. DELETED OUTRIGHT, which is correct: round 2's `W2B-4` found the enumeration incomplete, and the replacement points at the accessor block's per-axis property instead of counting record kinds. No loss, and the defect is closed rather than restated.
- `w5_problems`'s "So the step a refusal names is READ from a record's `step` id where one exists and is DERIVED otherwise". REPLACED by "may be one `round_step_slug` computed rather than one a record carries; `step_attribution` marks that case". The `W2B-4` scope defect goes with it. No loss.
- The W3 sibling test comment's "would report `workflow invariants hold` at exit 0 over an unconverged `risky` increment, with the whole suite green". DELETED, which is correct: round 2's `W2A-4` measured the second half false. The behaviour it described is carried by the test's own fixture and assertion. No loss.

DELETED, AND THE FACT NOW SURVIVES ONLY IN A TEST:

- "a derived value NEED NOT BE A ROADMAP STEP or occur anywhere in the log", which stood at three sites before the shortening (the shipped prose in all three copies, `w5_problems`'s bullet, and `step_attribution`'s doc). Measured after the shortening:

```
grep -rn "need not be a Roadmap step\|need not appear in the Roadmap" --include=*.rs --include=*.md .   (review records and explorations excluded)
CHANGELOG.md:32               (about the RETIRED rule, not the surviving derivation)
src/workflow.rs:1706          (a test comment)
src/workflow.rs:1734          (that test's assertion message)
```

So the Roadmap half of the fact is gone from every production site and from the shipped text, and survives at `src/workflow.rs:1706` with a pinning assertion at `:1734` (`assert_eq!(... .filter(|step| step.slug == "alpha-fold").count(), 0, "the fixture's point is that the derived owner is not a Roadmap step")`). I DO NOT RAISE THIS AS A FINDING: a remaining site does carry the fact, it is carried with a test that fails if it stops being true, and deleting explanatory clauses is precisely what the human decided this pass should do. I record it because the LOG half of the same sentence was not merely deleted but replaced with a categorical claim, and that replacement is `W3A-2`.

## Did the shortening remove the mechanism, or move it

MEASURED RATHER THAN JUDGED. Production comment lines in `src/workflow.rs`, counting lines before `mod tests` whose first non-space characters are `//`:

```
main      623 production lines, 279 comment lines
86e00ed   719 production lines, 331 comment lines   (before the shortening)
651ff63   706 production lines, 320 comment lines   (after)
```

So the shortening cut 11 production comment lines and the change still adds 41 over `main`. The implementer's report of falling comment lines is true and modest.

WHAT DID NOT CHANGE IS THE NUMBER OF SURFACES. The ownership relation is still stated at ten places: `waiver_covers_round`'s doc, W3's inline comment, `step_attribution`'s doc, `w5_problems`'s bullet, W5's two inline comments, the two refusal strings, `src/plan/source.rs`'s comment, the `CHANGELOG` entry, and the shipped clause in three drift-guarded copies. The shortening made each shorter and pointed three of them at one authority; it retired none of them. THE HONEST STATEMENT IS THAT THE MECHANISM IS ATTENUATED AND NOT REMOVED, and the round's own numbers say the same thing: five valid findings, then nine, now two, with one of the two written by the shortening itself. That is the same shape round 2's triager described (four of nine were against sentences the round 1 fix pass wrote), at a much lower rate.

THE TWO SITES THE IMPLEMENTER NAMED, FOUND INDEPENDENTLY AND JUDGED. I did not have the implementer's report, so I looked for them by inspection.

- WHERE IT KEPT A POINTER RATHER THAN AN ENUMERATION: the shipped clause's "resolving each axis exactly as the escalation join below does", which replaced a verbatim restatement of both fallbacks. JUDGED SOUND. The target is in the same document, so it travels into every scaffolded project with the pointer; it states both fallbacks verbatim; and the two joins genuinely do resolve identically, which I verified against the accessors rather than taking the sentence's word for it. This is the one place where a pointer could have re-opened round 1's `W1B-4` and it does not.
- WHERE IT JUDGED A POINTER UNAVAILABLE: the empty-owners REFUSAL STRING at `:642-645`, which kept an inline enumeration, "(by its structured `increment` id, else its `task`; a record the schema check rejected is not read)". JUDGED CORRECT. A runtime message has no document the reader can follow, and its reader is an author who has just been refused, so an inline parenthetical is the only form available. It also grew rather than shrank in this pass, absorbing round 2's `W2B-7`, which is the right trade. The `CHANGELOG` entry is the other candidate for this description and it too enumerates rather than points, for the same reason: a changelog reader may have nothing else in front of them. Both are correct calls.

## `W3A-1`: the unblocking test's comment infers a property of the retired rule that is false by one class

SEVERITY `low`. ABSOLUTE IMPACT IF LEFT UNFIXED: an in-code test comment misdescribes the population the retired rule blocked. No verdict, no shipped text, no emitted message.

THE CLAIM, `src/workflow.rs:1654-1658`, added by `0110828` and untouched by both fix passes:

```
// THE UNBLOCKING (Q-70). This is the shape the retired lexical rule made
// unwritable: an increment id that does not end `-inc<alnum>`, so
// `leading_slug` returns it unchanged and it can never equal the step slug, while
// the round log joins it to that step.
```

The "so" makes both conjuncts consequences of "does not end `-inc<alnum>`". The first is always true. THE SECOND IS FALSE WHENEVER THE INCREMENT ID IS THE STEP SLUG: `leading_slug` returns it unchanged, and unchanged is exactly equal to the step slug.

REPRODUCED, fixture `g12-retired-accepts` (Roadmap step `plan-fold`; one record joining increment `plan-fold` to step `plan-fold`; waiver `step = plan-fold`, `increment = plan-fold`):

```
prefix  exit=0  <root>/docs/plans/t.md vs <root>/docs/metrics/workflow.jsonl: workflow invariants hold
head    exit=0  identical
```

`plan-fold` does not end `-inc<alnum>`, `leading_slug("plan-fold")` is `plan-fold`, that value EQUALS the waiver's `step`, and the PRE-FIX binary accepted the waiver. So the shape the comment calls "unwritable" contains at least one writable member, and the inference the comment draws does not hold.

THIS IS NOT A RE-RAISE. Round 2's `W2B-3` was the same defect class at a DIFFERENT site, `CHANGELOG.md:32`'s "it refused every increment id that does not end `-inc<x>`", and its remedy was scoped to that sentence ("Fold into the single `### Fixed` rework"), with no sweep of `src/`. The fix pass corrected the `CHANGELOG` (verified above at C3) and left this comment making the equivalent inference. The triage's own class framing, "every site that ...", is what makes this one owed rather than new.

THE CHARITABLE READING, STATED SO A TRIAGER CAN WEIGH IT. If "it can never equal the step slug" is read as part of the SHAPE'S DEFINITION rather than as a consequence of the preceding clause, the sentence is describing this fixture and is true of it. I judge that reading unavailable because "so" introduces the whole conjunction, and because the test's own assertion message pins only the other half ("the fixture must use an id the shim leaves unstripped"), so the comment claims more than anything in the test holds.

REMEDY, ONE CLAUSE, SCOPED TO THE CLASS (every site in the diff that states which ids the retired rule blocked):

- `src/workflow.rs:1655-1657`: state the retired rule itself, as the `CHANGELOG` now does, and let the blocked class follow. For example: an increment id whose leading slug is not the waiver's `step`, which for an id that does not end `-inc<alnum>` means any id that is not the step slug itself.
- `CHANGELOG.md:32`: NO EDIT. Already correct (C3), and the correction is the model for this one.
- `src/workflow.rs:1619-1621`, the empty-case test's comment: NO EDIT. It already carries the qualifier ("whenever the id happened to strip to the step slug").
- The fixture and assertions at `:1659-1669`: NO EDIT. `beta-fold` on step `beta` is a correct instance of the unblocking and `workflow-enforcement-tier-fold` is correctly named as the live one.

## `W3A-2`: the shipped rule text justifies the derived mark with a categorical claim about the log that a fixture falsifies

SEVERITY `low`. ABSOLUTE IMPACT IF LEFT UNFIXED: a false sentence in the rule text that ships into every project this tool scaffolds. No verdict moves, no emitted message is false, and the mark itself is correct and safe-direction, which is what bounds it below `medium` on this project's own calibration (round 2's Ruling 3 rated the same class `low` for the same reason).

THE CLAIM, in `pack/instrument.md:11`, `AGENTS.md:147` and `.agents/AGENTS.reference.md:147`, byte-identical, WRITTEN BY THE SHORTENING (`651ff63`):

```
... an increment no `round` record resolves to is reported, and an owning step that no record of that increment declares in a structured `step` id is reported as derived, SINCE SUCH A STEP IS ONE THE JOIN COMPUTED RATHER THAN ONE THE LOG CARRIES ...
```

The rule half is correctly scoped to the increment's own records. THE JUSTIFICATION HALF DROPS THAT SCOPE and asserts a universal about the log: a marked step is not one the log carries.

REPRODUCED, fixture `g9-cross-increment`. The log has two records: one joining a DIFFERENT increment to step `gamma` on a structured `step` id, and one whose only structured id is `increment: alpha-fold`, with `task: gamma-inc1`:

```
{"type":"round","task":"x","artifact":"a",...,"step":"gamma","increment":"other-inc1"}
{"type":"round","task":"gamma-inc1","artifact":"a",...,"increment":"alpha-fold"}
{"type":"waiver",...,"unit":"increment","step":"beta","increment":"alpha-fold",...}

head  exit=1  round log line 3: increment waiver names step `beta` but the round log joins increment `alpha-fold` to step `gamma` (derived from a record's `task`)

grep -c '"step":"gamma"' <root>/docs/metrics/workflow.jsonl  -> 1
```

`gamma` is marked derived, correctly, because no record OF `alpha-fold` declares it. AND THE LOG CARRIES `gamma` AS A STRUCTURED `step` ID, verbatim, on line 1. So the shipped sentence tells a reader that a marked step is not one the log carries, and the reader can falsify that with one grep of the log the message just refused over.

THE SHORTENING INTRODUCED THIS, WHICH IS WHY IT MATTERS MORE THAN ITS SIZE. The sentence it replaced was `86e00ed`'s "because such a step need not appear in the Roadmap or anywhere in the log", a POSSIBILITY claim, which round 2's messages reviewer measured as true on both a case where the step is absent from the log and a case where it is present. The shortening turned a hedged true claim into a categorical false one. It is the same defect round 2 raised as `W2B-4`'s scope half, and the same commit FIXED that defect in the in-code comments (`:544` and `:626-627` now say "a record OF THAT INCREMENT" and "one of its own records") while writing the unscoped form into the copy that ships.

NOT A RE-RAISE OF `W2A-1` OR `W2B-4`. `W2A-1` was that the shipped clause stated the mark's trigger PER RECORD where the code applies it per owner; that clause is gone and its replacement is correct (S4, verified on `f-ormerge`). `W2B-4`'s scope half was raised against `src/workflow.rs:545`, `:589` and `:640`, all of which are now correctly scoped. This is a NEW sentence in a DIFFERENT file, written after both verdicts.

REMEDY, SCOPED TO THE CLASS (every site that justifies the derived mark with a claim about what the log does or does not carry):

- `pack/instrument.md`, the `type: "waiver"` bullet's W5 clause, over the JUSTIFICATION only: either carry the per-increment scope into it, or restore a possibility claim. The shortest correct forms are "since no record of that increment states it" or "since such a step need not appear in the Roadmap or anywhere in the log". The second also restores the Roadmap fact the shortening dropped from every production site, at no extra length.
- `AGENTS.md:147` and `.agents/AGENTS.reference.md:147`: the same clause. THE THREE MUST MOVE IN ONE COMMIT or the drift guard fails, which I demonstrated above by reverting each of the three separately.
- Acceptance item 7b's fixed-string command must be RE-RUN after the edit. Its replacement wording, "some `round` record must join that increment to that step", sits at the START of the same parenthetical, so a careless edit can move it; it must still report 0, 0, 0 and 1, 1, 1.
- CARRY THE REGENERATION HAZARD the step records at `validation-constraints.md:142`: do not run `just scaffold-self` naively, because its second line runs `nix fmt` over a tree that is not formatter-clean at HEAD.
- `src/workflow.rs:543-547` and `:626-630`: NO EDIT. Both are correctly scoped to the increment's own records and both are the text the shipped clause must be made to agree with.
- `src/workflow.rs:631-640`, the owners map, and `:647-653`, the message: NO EDIT. The verdict and the mark are correct on `g9-cross-increment`, which round 2's triage already ruled ("THE MESSAGE STAYS TRUE ... and the per-increment scope is the right scope").
- `CHANGELOG.md:32`: NO EDIT. Its version of the same sentence carries no such justification.

## What I looked for and did not find

STATED EXPLICITLY, because a clean result is only evidence if the search that produced it is described.

- NO WRONG VERDICT. Eleven Markdown-substrate fixtures and two TOML ones, on two binaries, with the expected answer computed by hand from the documented accessors before running: `head` matched my expectation on every one, and `prefix` differed from `head` on exactly three (`g-noresolve`, `h1-rehomed`/`h4-repaired`, `unblock`), each a documented direction (iii) narrowing or the unblocking. The live plan is green over 321 records, 96 steps, 70 questions and 13 increment-unit waivers.
- NO BROKEN POINTER. Four pointers, four targets read, four saying what their pointer claims.
- NO SILENT DRIFT BETWEEN THE THREE PROSE COPIES. Byte-identical, and each of the three separately reverted reddens the guard.
- NO FREQUENCY CLAIM IN THE `CHANGELOG`, which is what round 2's triager measured and rejected.
- NO RESIDUE OF `W2B-5`. The only "attribut" left in `src/workflow.rs` is the private helper's own name, which round 2 ruled needs no rename.
- NO NON-ASCII AND NO DASH SUBSTITUTE anywhere in the six changed files (`LC_ALL=C grep -cP '[^\t\x20-\x7e]'` returns 0 for each; the added lines carry no ` -- `).
- NO NEW UNPINNED CLAIM I COULD FIND IN THE TEST COMMENTS beyond `W3A-1`. Eleven test-comment claims checked, ten true.
- ONE PRE-EXISTING IMPRECISION LEFT ALONE AND RECORDED: the same shipped bullet's W3 half says an increment is exempt "when an `increment`-unit waiver names that increment's `task`", where the key has been the structured `increment` id when present since Inc 2. That sentence is byte-identical on both sides of `git diff main..HEAD`, so it predates this change and is out of scope by the standing rule. Recorded for whoever next edits that bullet, since `W3A-2`'s fix pass will be in the same sentence.
- I DO NOT REOPEN DIRECTION (iii), `Q-70-emptycase`, OR THE `### Fixed` PLACEMENT, and I have no evidence beating any of them. I re-read the receipts and both are implemented as receipted.

TWO THINGS THE ORCHESTRATOR STILL OWES, NEITHER A FINDING AGAINST THIS ARTIFACT, both unchanged from rounds 1 and 2. Acceptance item 3, the plan-side unblocking (the two `[[step.increment]]` declarations, the two owed waivers, the `workflow-enforcement-tier` status flip), is still absent from `git diff main..HEAD`, and the step assigns those edits to the orchestrator and the planner. And the post-merge planner pass still owes the sidecar the three facts round 2 recorded, to which this round adds a fourth: item 7b's replacement wording is unchanged but the sentence around it has moved twice, so the item should name the whole parenthetical rather than one fixed string inside it.
