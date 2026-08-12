# `validation-constraints-inc1` round 1: reviewer, the cases nobody specified

Lens: the input space the spec, the design pass and the plan review did not reason about. Every case below was run through the built binary rather than read off the source.

## Method

Two binaries, built into separate `CARGO_TARGET_DIR`s and confirmed distinct by `md5sum`:

- FIXED, this worktree at `fe5b31a`: `cc12f4a4309bdf3b45df393da9f5c2af`.
- PREFIX, a `git archive main` extract built in its own tree: `60d6e8769d97e61dca1941736f1cb713`.

Fixtures live under `<scratchpad>/edges/work/<case>`, one project root each (`docs/plans/t.md` or `docs/plans/t.plan.toml`, plus `docs/metrics/workflow.jsonl`), so the metrics path resolves naturally and the containment guard is satisfied. The generator is `<scratchpad>/edges/cases.sh`. Nothing outside the scratchpad was written.

`cargo test` under this worktree with `TMPDIR` outside any repository: 382 + 20 + 9 + 5 + 4 + 3 + 1 + 1 + 1 passed, 0 failed.

## Findings

Three findings, all in the message and the claims made about it. NO finding at `critical` or `high`. I found no case in which the new rule returns a wrong VERDICT: no false green and no false red beyond the narrowing the step decided.

### `W1B-1`: the refusal still names a step derived from the increment id whenever the joining record is pre-migration on the step axis, so the `src/` defect the step records as "closed BY CONSTRUCTION" is still live

Severity: `medium`.

`round_step_slug` (`src/workflow.rs:119-121`) falls back to `leading_slug(&round.task)` when a record carries no structured `step`. `w5_problems` builds `owners` from that accessor (`src/workflow.rs:612-616`), so on a pre-migration record the step the refusal names is computed from a `task` string, not read from the log. It can be a substring of the increment id, another record's raw `task`, or any string that names no Roadmap step.

Reproduced against THIS REPOSITORY'S OWN LOG. `q70-capture` is not a Roadmap slug (`grep -c '^slug = "q70-capture"$' docs/plans/agent-scaffold.plan.toml` returns 0; it is an orphan task). Extract the live `type:"round"` records, append one increment waiver naming a real step, and run the FIXED binary:

```
increment waiver names step `structured-skeleton` but the round log attributes increment `q70-capture` to step `q70-capture`
```

The step `q70-capture` exists in no plan. The PREFIX binary emits the same wrong step name, so this is NOT a regression; it is the recorded defect surviving the change that is documented as closing it.

Three constructed shapes widen the class:

- A true substring. Plan steps `alpha` and `beta`, a pre-migration record with `"task":"gamma-incidental"`, a waiver naming `alpha` for increment `gamma-incidental` -> "the round log attributes increment `gamma-incidental` to step `gamma`". `gamma` is a literal prefix of the id and appears nowhere in the log.
- Another record's task. A record with a structured `increment` of `alpha-fold` and NO `step`, whose `task` is `zzz-task` -> "the round log attributes increment `alpha-fold` to step `zzz-task`".
- The plural form. One record with `"step":"alpha"` and one pre-migration record for the same increment -> "attributes increment `alpha-fold` to steps `alpha`, `alpha-fold`", where `alpha-fold` is not a step.

What this contradicts:

- `src/workflow.rs:610-611`, added by this diff: "The steps the log DOES join this increment to, so the refusal names what the records say instead of a step derived from the id."
- `src/workflow.rs:565-566`, added by this diff: "the refusal states a fact the records carry instead of a substring of the id."
- `src/workflow.rs`'s new test comment on `w5_flags_an_increment_waiver_whose_increment_has_no_round_records`: "the retired rule reported a step derived from the id, which need not exist in the plan."
- `CHANGELOG.md`, `### Fixed`: "its refusal named a step derived from the id, which need not be a Roadmap step" (given as the retired rule's defect) and "A refusal now names the step or steps the records actually attribute the increment to."
- `docs/plans/agent-scaffold.steps/validation-constraints.md:23`: the defect "is closed BY CONSTRUCTION by inc1 and is not separately scheduled". If the round accepts that, the defect leaves the plan while still live.

Population, measured rather than asserted: `jq -r 'select(.type=="round") | if .step then "structured" else "premigration" end' docs/metrics/workflow.jsonl | sort | uniq -c` returns 113 pre-migration against 123 structured. Just under half of this project's own round records reach the message through the fallback, and every record of any project that adopted the tool before Inc 2 does.

Why `medium` and not higher: the VERDICT is right in every one of these cases. The log genuinely does not join the increment to the waiver's step, so refusing is correct and no waiver is wrongly admitted or wrongly blocked. What is wrong is the remedy the message hands the author, and the durable claim that a recorded defect is closed.

### `W1B-2`: the no-records message asserts an absence a reader can falsify with one grep

Severity: `low`.

The empty-owners branch prints "increment waiver names increment `X`, which has no `type:"round"` records". `round_increment_id` prefers the structured `increment` id, so a record whose `task` IS `X` while its `increment` is something else does not count, and the sentence reads as false to anyone who greps the log for `X`.

Reproduced against this repository's own log. `grep -c '"task":"backlog-clearing"' docs/metrics/workflow.jsonl` returns 5. Feed those live rounds plus a waiver for increment `backlog-clearing` to the FIXED binary:

```
increment waiver names increment `backlog-clearing`, which has no `type:"round"` records, so the round log attributes it to no step
```

Five `type:"round"` records carry that string as their `task`. The claim the code means is "no record RESOLVES to that increment id", which is not what the sentence says. Constructed minimal form: one record with `"task":"alpha-inc1","increment":"alpha-inc2"` and a waiver for `alpha-inc1` produces the same sentence.

`low`, because the verdict is correct under the Inc 2 identity model and only the wording over-claims. Note that this shape is not exotic on this project: the live log routinely carries a `task` that differs from the structured `increment` (`backlog-clearing`, `lifecycle-capture`, `uniform-isolation`).

### `W1B-3`: `step_attribution`'s doc gives one cause for several owners and the fallback supplies another

Severity: `low`.

`src/workflow.rs:544-545`: "Several is authorable on the JSONL substrate, where a record's `step` is a free string." That is one route. The other needs no free-string abuse at all: two well-formed records for one increment, one carrying a structured `step` and one pre-migration, produce two owners because the second resolves through `leading_slug`. Fixture `x4` above yields "steps `alpha`, `alpha-fold`" from exactly that pair. The same root cause as `W1B-1`, recorded separately because it is a different sentence and a reader who trusts it will not look for the fallback route.

### `W1B-4`: the shipped rule text states the escalation join's fallback and not the round join's

Severity: `low`.

The rewritten bullet in `pack/instrument.md`, `AGENTS.md` and `.agents/AGENTS.reference.md` spells out the escalation join's shim explicitly, "the escalation's structured `increment` id, or its `task` when that id is absent, ... or its structured `step` slug, or `leading_slug(task)` when that id is absent", and then states the new ownership rule with no fallback clause at all: "the round log must join that increment to that step, so an increment with no round records at all is reported". A reader of a scaffolded project's `AGENTS.md` is told one join degrades gracefully and left to infer the other does not, when both do. This text ships into every project the tool scaffolds.

## The full case table

`W3+` marks a case that also exercises W3 or the round-log consistency check. "Verdict" is right / wrong / unspecified-but-defensible (UBD).

| Case | What was constructed | FIXED result | PREFIX result | Verdict |
| ---- | -------------------- | ------------ | ------------- | ------- |
| `c1` | Markdown baseline: two `complete` steps, one converged pre-migration round each, no waiver | `workflow invariants hold`, exit 0 | same | Right |
| `e1` | Pre-migration record `"task":"alpha-fold"`, waiver `alpha` / `alpha-fold` | refused, names step `alpha-fold` (not a Roadmap step) | refused, same wrong step | See `W1B-1` |
| `e1b` | Pre-migration record `"task":"gamma-incidental"`, waiver `alpha` / `gamma-incidental` | refused, names step `gamma` (a substring of the id) | refused, same | See `W1B-1` |
| `e2b` | Log EXISTS and is EMPTY, one increment waiver, step `in progress` | refused, "has no `type:"round"` records", exit 1 | `workflow invariants hold`, exit 0 | Right; the decided narrowing, and it is not scoped to `complete` steps |
| `e3` | Log file ABSENT entirely | "--workflow requested but no round log at ...", exit 1, check does not run | same | Right; a different case from an empty log and correctly kept so |
| `e4` | One increment, records under `alpha` AND `beta`, waiver names `alpha` | accepted, exit 0 | refused | UBD; the rule is existential by design ("some record"), and W3 still judges `beta`'s group separately |
| `e4b` | Same, waiver names a third real step `gamma` | refused, "to steps `alpha`, `beta`" | refused, names `shared` | Right |
| `e4c` | `e4b` with the two records in REVERSE file order | byte-identical message | byte-identical | Right; `BTreeSet` makes the owner list order-independent |
| `t1` | TOML substrate, declared increment, log joins it to `beta`, waiver nests on `alpha` | refused, "TOML waiver `alpha-w1`: ... attributes increment `shared-inc1` to step `beta`" | refused, names `shared` | Right |
| `t2` | TOML substrate, DECLARED increment, no round records at all | refused, no-records message | refused, wrong step | Right; the declared-set gate passes and W5 still catches it, which is the residual the step's open point names |
| `t3` | TOML substrate, log joins the increment to the waiver's own step | `workflow invariants hold`, exit 0 | refused | Right; the unblocking on the TOML path |
| `s1` | `W3+` `complete` step with NO records, STEP-unit waiver | holds, exit 0 | same | Right; no regression on the step-unit path |
| `s2` | `W3+` `complete` step with NO records, no waiver | "is `complete` but has no round records and no covering waiver" | same | Right |
| `s3` | `W3+` `complete` step with NO records, only an INCREMENT-unit waiver | W3 still refuses, plus the new no-records report | W3 refuses only | Right; the predicate's unit check keeps an increment waiver out of the step-unit path |
| `s4` | STEP-unit waiver carrying an `increment` field | dropped by the projection, `validate_log` reports the malformed record | same | Right |
| `s5` | `W3+` short `risky` increment, covering increment waiver | holds, exit 0 | same | Right; W3's exemption is byte-equivalent |
| `s6` | `W3+` short `risky` increment, waiver names a real-but-WRONG step | W3 refuses AND W5 refuses with the true owner | same two problems, W5 names the lexical owner | Right |
| `x1` | Increment-only record (structured `increment`, NO `step`), `"task":"zzz-task"` | refused, names step `zzz-task` | refused, names `alpha-fold` | See `W1B-1` |
| `x2` | Step-only record (structured `step`, NO `increment`), the LIVE fold shape | holds, exit 0 | refused | Right; this is the shape the two owed waivers need |
| `x3` | One structured-step record and one pre-migration record for one increment, waiver names the structured step | holds, exit 0 | refused | Right; the existential join finds the good record |
| `x4` | `x3` with the waiver naming a step neither record joins | refused, "to steps `alpha`, `alpha-fold`" | refused, names `alpha-fold` | See `W1B-1`, `W1B-3` |
| `x5` | Record `"step":"Alpha"`, waiver step `alpha` (case mismatch) | refused | holds, exit 0 | UBD; the join is an exact string compare and the log genuinely does not say `alpha`. A NEW red the lexical rule missed |
| `x6` | Two identical increment waivers for one unowned increment | two identical problems, one per waiver | holds | UBD; one problem per waiver is W5's existing shape |
| `y2` | `W3+` both steps `complete`, one increment split across them, `risky`, waiver names `alpha` | holds, exit 0 | refused | Right; `alpha`'s group is exempted by the waiver and `beta`'s group converged on its own |
| `y4` | `W3+` self-contradictory log where the streak recomputation also fires | consistency problem only | consistency problem plus the lexical W5 refusal | Right; the split log usually trips the consistency check independently |
| `z1` | `W3+` `complete` step, short `risky` increment, only a STEP-unit waiver | W3 refuses (a step waiver does not cover an increment shortfall) | same | Right; pre-existing and unchanged |
| `z2` | The full unblocking: fold token, structured `step`, `accepted-at-escalation` waiver, matching escalation | holds, exit 0 | refused on ownership | Right; end-to-end demonstration of what inc1 unblocks |
| `z3` | `z2` with the escalation removed | ONE problem, the evidence join | TWO problems, ownership plus evidence | Right; the ownership arm no longer fires on a correctly-owned waiver |
| `z4` | Dangling waiver `step` AND an increment with no records | two problems, dangling step and no-records | one problem, dangling step | Right |
| `w1` | Record `"task":"alpha-inc1","increment":"alpha-inc2"`, waiver for `alpha-inc1` | refused, "has no `type:"round"` records" | holds | See `W1B-2` |
| `w2` | Waived increment present only on an ESCALATION record | refused, no-records message | holds | Right; the message correctly says `type:"round"` and not "the log" |
| `d1` | Duplicate and repeated records for one increment under two steps | owners de-duplicated, "steps `beta`, `gamma`" | refused, names `shared` | Right |
| `live` | This worktree's live plan and log, both binaries | `workflow invariants hold`, exit 0 | `workflow invariants hold`, exit 0 | Right; acceptance item 2's precondition holds |
| `live1` | Live rounds plus a synthetic waiver for increment `backlog-clearing` | no-records message over 5 records carrying that `task` | lexical refusal | See `W1B-2` |
| `live2` | Live rounds plus a synthetic waiver for increment `q70-capture` | names step `q70-capture`, absent from the plan | same wrong step | See `W1B-1` |
| `scale` | 20000 round records, 60 unowned increment waivers | 60 problems, 1.375s wall | 60 problems, 1.389s wall | Right; the added `O(waivers * rounds)` scan is not measurable |
| `owners` | One increment attributed to 200 distinct steps | one 1917-byte message listing all 200 | 417 bytes | UBD; `step_attribution` has no cap, but the realistic population of distinct step values per increment is small |

## Cases checked by reading rather than by running, and why

- A `unit == increment` waiver with no `increment` value cannot reach the ownership arm: `metrics::parse_waivers` drops it (`src/metrics.rs:885-890`) and `waivers_from_toml` `continue`s (`src/workflow.rs:242-247`).
- An empty `step` or `increment` on a round is rejected by `require_structured_ids` (`src/metrics.rs:383-390`) and filtered again by `parse_rounds` (`src/metrics.rs:697-698`), so a blank string can never become a join key or appear in the owner list.
- W3's exemption is byte-equivalent to the retired inline match: `records` are pre-filtered on `round_step_slug(round) == step.slug` (`src/workflow.rs:472-473`) and grouped on `round_increment_id` (`:493`), so `waiver_covers_round` over that group reduces to the old `waiver.step == step.slug && waiver.increment == Some(increment)`. Cases `s1` to `s6`, `y2`, `z1` and `z2` confirm it behaviourally.
- No third copy of the relation exists: `src/next.rs` never mentions `waiver` or `Waiver`, so `next` cannot drift from the shared predicate because it does not consult waivers at all.
- W5 is reachable only through `validate --workflow` (`src/main.rs:1016` and `:1030`), so no other surface inherits the narrowing.

## Explicitly not raised

- The pre-existing stale Status narrative at `docs/plans/agent-scaffold.md:7`, and any pre-existing false doc claim predating this change.
- Line length and prose wrapping.
- Anything belonging to increments 2 to 6, including inc6's identity-filter blast radius, which the step already records as inc6's own review obligation.
- Pre-existing import-ordering drift.
- The plan-side unblocking (the two `[[step.increment]]` declarations, the two owed waivers, and the status flip) is absent from `git diff main..HEAD`, so acceptance item 3 is not settled by this artifact. Recorded as an observation about scope, not as a finding, since the orchestrator owns those edits directly on main.
