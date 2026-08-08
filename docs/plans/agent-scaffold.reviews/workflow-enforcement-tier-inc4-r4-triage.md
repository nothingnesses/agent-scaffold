# `workflow-enforcement-tier-inc4`, round 4: triage

Triager worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage-inc4-r4`, branch `triage/wet-inc4-r4`, at `84f1905`. The branch check ran before the first edit and printed `triage/wet-inc4-r4`. Every command below named its tree by absolute path or ran with this worktree's root as the working directory.

`84f1905`'s tree differs from the reviewers' `7ab5d48` ONLY by the three round-4 findings files (`git diff --stat 7ab5d48 HEAD` returns those three and nothing else), so every sidecar, source and plan line number below is the reviewers' unchanged.

All fixtures under `<scratchpad>/triage-inc4-r4/` only. Nothing written to bare `/tmp`, nothing deleted outside that subdirectory, no `chmod` used anywhere so none is owed a restore. The one historical binary I built was built from a `git archive` extract into its own `CARGO_TARGET_DIR` under the scratchpad. `git status --short` in this worktree prints nothing but this file.

## Counts

| quantity | value |
| --- | --- |
| RAW findings | 6 |
| DEDUPLICATED | 6 (no duplicates across the three lenses) |
| VALID | 6 |
| IN-SCOPE VALID | 4 |
| OUT-OF-SCOPE VALID | 2 (`R4A-1`, `R4B-3`) |
| VALID BUT ACCEPT RESIDUAL | 0 |
| DISMISSED | 0 |
| dismissed at `high` or above | 0, SO NO BACKSTOP RE-CHECK IS OWED |

Severity distribution after correction: `critical` 0, `high` 0, `medium` 2, `low` 4. I changed no severity in either direction. Every reviewer rating survived its own stated ground, and I say below for each what I weighed before confirming it. I corrected TWO characterisations that do not move a verdict: `R4A-2`'s remedy class (the reviewer calls it deletion-class; it adds a requirement, so it is authored prose), and `R4B-1`'s "fourth recorded twin-site defect" ordinal, which is not reconstructable to a single number and is not load-bearing.

REPRODUCED FIRST-HAND, evidence re-run rather than read: ALL SIX. Nothing was judged on citation alone. Four needed purpose-built fixtures (`R4A-1`'s render mutation, `R4B-1`'s Markdown-primary no-plan pair, `R4B-3`'s two symlink orientations), one needed the increment's whole-range diff (`R4B-2`), and one needed `git log -S` over three files plus the four bullets and the `Q-55` record opened side by side (`R4B-4`). I also built one historical binary of my own to test the check-runner lens's red halves rather than accepting them.

THE ROUND IS NEW-VALID, AND NEW-VALID ON IN-SCOPE FINDINGS ALONE: four in-scope valid findings, two of them `medium`. The streak stays 0 of 2. This is round 4 of a cap of 5.

I was told the convergence arithmetic before I ruled, including that it is now deterministic and that my verdicts can neither save this loop from an escalation nor cause one. I record that it moved nothing, and I record the one place it could have shown: the two out-of-scope rulings are the only verdicts that could have been shaded to change the round's category, and the round is new-valid on `R4B-1`, `R4B-2`, `R4A-2` and `R4B-4` without either of them.

## Verdict table

| id | reviewer | reviewer severity | final severity | scope | verdict | remedy class |
| --- | --- | --- | --- | --- | --- | --- |
| `R4B-1` | crossartifact | medium | medium (confirmed) | IN SCOPE | VALID, fix required | DELETION (two sites) |
| `R4B-2` | crossartifact | medium | medium (confirmed) | IN SCOPE | VALID, fix required | AUTHORED PROSE (two bullets, one clause, no new facts) |
| `R4A-2` | checkrunner | low | low (confirmed) | IN SCOPE | VALID, fix required | AUTHORED PROSE (one clause) |
| `R4B-4` | crossartifact | low | low (confirmed) | IN SCOPE | VALID, fix required | DELETION (one sentence) |
| `R4A-1` | checkrunner | low | low (confirmed) | OUT OF SCOPE | VALID, minimal fix recorded | TOKEN (one flag) |
| `R4B-3` | crossartifact | low | low (confirmed) | OUT OF SCOPE | VALID, minimal fix recorded | TOKEN (one clause, copied from the tree) |

## Per-reviewer attribution, for the round record

| lens | file | raw | valid | in scope | out of scope |
| --- | --- | --- | --- | --- | --- |
| check-runner (opus) | `...-r4-checkrunner-opus.md` | 2 | 2 | 1 | 1 |
| cross-artifact (opus) | `...-r4-crossartifact-opus.md` | 4 | 4 | 3 | 1 |
| cold-read (sonnet) | `...-r4-coldread-sonnet.md` | 0 | 0 | 0 | 0 |

No finding was raised by two reviewers, so the per-reviewer valid counts sum to the deduplicated round total of 6 rather than exceeding it. The round-level `valid_findings` for the log is 4, the in-scope count, per the out-of-scope precedent.

Every lens was new this round. The cross-artifact lens is the productive one, 4 raw and 4 valid, and it is the first lens in this loop pointed BETWEEN artifacts rather than at one; three of its four findings are joins that no single-artifact lens could have reached, which is the lens-selection fact worth carrying.

## Deduplication, confirmed on evidence

NO DUPLICATES. The two candidate pairs were checked and both are distinct.

`R4A-1` AND `R4A-2` are both "a check that cannot fail", and they are two findings, not one. Different checks (1 and 21), different mechanisms (an exit code that is always 0 against a procedure with no failing branch), different remedies (add `--strict` to a command against add a resolution requirement to a procedure), different provenance (`1a04071`, before the step's increments, against `d8e8087`, the increment's own base commit) and, as ruled below, different scope. Nothing is shared but the reviewer's framing sentence.

`R4B-1` AND `R4B-2` SHARE A FIX EDGE BUT ARE TWO FINDINGS. If `R4B-1` is fixed, `src/next.rs` becomes a site the increment edited and `R4B-2`'s list gains a fourth missing bullet. That is a fix-ordering dependency, not a duplicate subject: `R4B-2` is already true of three sites before `src/next.rs` is touched at all, measured over the increment's whole range below. The fix brief must carry both and must apply `R4B-1` first.

## The two clean-lens denominators, assessed

Asked for explicitly, and this is the main thing a human weighs if the loop escalates. I sampled both rather than accepting either.

### The check-runner: 33 checks, 33 run, 33 PASS, 21 red halves. I BELIEVE IT.

WHAT I VERIFIED MYSELF.

The list really does have 33 entries. `grep -cE '^[0-9]+[a-h]?\. '` over `:316-348` returns 33, and the ids are `1 2 3 4 5 6 7 8 9 10 11 12 13 13b 14 14b 14c 14d 14e 14f 14g 14h 15 16 17 18 19 19b 20 21 21b 22 23`. The cold-read lens calls the same list "23 acceptance checks", counting top-level numbers only. Both counts are of the same text and neither is wrong; the check-runner's is the one that matches what has to be executed.

The three historical baselines are exactly what the lens claims. `git rev-parse <c>^` gives `1dac3dc` for `609ddcf`, `285a6a3` for `8beb1c2` and `5684b5f` for `6b1c847`, so all three are the true parents of each increment's first code commit rather than merge parents, which is the trap round 3's historical lens recorded and this lens avoided.

I BUILT ONE OF THE THREE HISTORICAL BINARIES AND RAN ONE RED HALF, because a claimed red is the cheapest thing in a review to assert and the most expensive to fake. `git archive 5684b5f` into `<scratch>/triage-inc4-r4/pre3`, built into its own `CARGO_TARGET_DIR`, `md5sum` distinct from the HEAD binary. Check 15's red:

```
$ cd <scratch>/triage-inc4-r4/fix && <pre3>/agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
exit=0
```

That is the reported red verbatim, against the reported green at HEAD (exit 1, "the workflow check could not run"), which I also re-ran.

Four further checks re-run at HEAD, all as reported: check 2 (`(30 changed, 0 left untouched)`, `ls docs` printing only `plans`), check 10 (exit 0 with the miss note), check 15 (exit 1) and check 23 (both halves, `up to date` and `296 records` / `95 steps, 69 questions` / `workflow invariants hold` at exit 0). Check 21's citation half re-run mechanically and independently: extracting every distinct `file:line` citation from the step file gives exactly 13, all in bounds, and opening each shows the named subject. That is the third independent arrival at 13 in this loop, after round 3's triage and this lens.

WHAT MAKES ME BELIEVE THE PART I DID NOT RE-RUN. The lens recorded a trap that produced a confidently wrong result before it was caught: one shared `CARGO_TARGET_DIR` across three `git archive` extracts reused a stale fingerprint and made `as-pre2` and `as-pre3` byte-identical, so every "before inc3" measurement would silently have been an "after inc2" one. It then named the fix, verified three distinct `md5sum`s and three distinct `--metrics` help strings, and published the trap for the next runner. A lens that had not actually built the binaries would not have invented that failure, and would certainly not have invented the control that detects it.

THE BOUND, WHICH THE LENS STATES ITSELF AND WHICH I READ AS THE HONEST FORM RATHER THAN A HEDGE. One platform, one build profile, uid 1000 with a single uid-0 cell under `unshare -Ur` for check 16, no concurrency, no `--instrument` scaffold. Fixtures were built by the HEAD binary and then run against the historical ones, which is correct for every check here because each names its fixture as an input, and the one check about `scaffold` OUTPUT (check 20) took its red from `git show 5684b5f:pack/AGENTS.md` instead. That is the right call and the stronger evidence. So the result reads as "33 of 33 pass on this axis set", which is what it claims.

### The cross-artifact lens: 71 facts mapped, 67 agreeing. I BELIEVE IT, WITH ONE NAMED SOFT SPOT.

Its two mechanical gates re-run here at HEAD, both as reported. Nine of its 67 agreeing facts sampled and confirmed: `pack/AGENTS.md:93`'s boundary sentence in the tree; the three exploration files at 521, 510 and 483 lines; the three waiver breakdowns summing to 13, 24 and 14; the four `[[step.increment]]` entries; the token vocabularies at `src/next.rs:100-144`; the 13 line citations. Its four disagreeing facts all reproduce, which is the strongest evidence the mapping was actually opened rather than remembered, because three of the four needed a fixture to establish which side is true and it built them.

THE SOFT SPOT, STATED BECAUSE IT IS THE ONE PLACE THE TWO CLEAN LENSES CONTRADICT EACH OTHER. Its fact 6, the project median of two rounds per artifact at sidecar `:306`, is recorded as AGREE, "computed over 85 artifacts, median 2, ranking 9, 7, 6, 6". The cold-read lens attempted the same computation, got 175 artifacts and median 1, judged its own grouping method unsound because the `artifact` string changes between rounds of one real loop, and declined to report either way. Two lenses in one round produce different denominators for one figure, and neither raised it. I did not adjudicate the figure, because no finding before me depends on it and reconstructing the true per-artifact round count from this log is the same problem the cold read correctly declined. I record it as the one entry in the 67 that I would not treat as independently established.

### The cold read: 100 percent of the sidecar, `Q-55`, four increments and three waivers, ZERO findings. I BELIEVE THE SWEEP, AND ITS ZERO IS BOUNDED BY ITS LENS RATHER THAN BY ITS EFFORT.

Everything I sampled from its list reproduces exactly: the exploration line counts, the three waiver sums, the five backlog step orders and statuses, `pack/AGENTS.md:93`, check 15's behaviour, the scaffold output line, and `F-5`, which I confirmed independently (`grep -c 'slug = "validation-constraints'` over the plan TOML returns 0 while the handle is named six times in the step file and three times in `Q-55`). Its citation spot-checks land on the same 13-of-13 I reached mechanically.

TWO THINGS RAISE MY CONFIDENCE RATHER THAN LOWER IT. It reached `F-5` independently, verified it BEFORE checking the review history, and then stopped at it with the ledger's own verdict, which is the settled-ground rule working rather than being recited. And it reported a reproduction it TRIED AND COULD NOT TRUST, the median figure above, and declined to raise a finding on a method it did not believe. A lens optimising for a clean report does not volunteer either.

THE BOUND, AND IT IS THE HONEST READING OF THE ZERO. Two of the cross-artifact lens's four findings sit inside text the cold read swept at 100 percent: `R4B-4`'s two halves (`Q-55` at plan TOML `:1726`, and the sidecar at `:14` and `:275-282`) and `R4B-2`'s impact list at `:382-388`. The cold read read all of them and did not find either. Neither is a false claim in isolation, which is what its lens was pointed at: `:1726`'s "THREE" was true when written, and an impact list's OMISSION is not a false statement. So the zero is "no false claim survives reproduction inside these artifacts", not "these artifacts are correct", and the difference is exactly the lens the cross-artifact pass ran. `R4B-1` and `R4A-1` are outside its assigned text altogether.

A ROUND'S CLEAN LENSES ARE WORTH SOMETHING HERE. Between the three, this round opened the largest denominators the loop has produced, and I could not falsify any of the three headline numbers I tested. The failure the round did produce is not effort and not honesty; it is that both clean lenses were pointed INSIDE artifacts and the productive one was pointed BETWEEN them.

## (1) `R4B-1`: the round's headline, and a decision applied at two sites of four

REPRODUCED IN FULL, including the behaviour, the provenance and the never-raised claim.

THE TWO SURVIVING SITES, opened at the cited ranges. `src/next.rs:105-106`: "The resolved path is not under the root of the plan this surface reads, so the tool cannot vouch that its records belong to that plan." `src/next.rs:140-142`: "The resolved ledger is not under the root of the plan this surface reads: either an explicit `--ledger-fragment` outside it, or a default ledger anchored on a `--source` that itself lies outside it." Both carry the clause `Q-55-reasondefs` deleted from sidecar `:217` and `:229` at this branch's `84f1905`, verbatim.

MEASURED FALSE, my own fixture, a Markdown-primary `--source` with no `--plan` at all:

```
$ ./target/debug/agent-scaffold status --source <S>/projA/docs/plans/p.plan.toml \
    --metrics <W>/docs/metrics/workflow.jsonl --json
{
  "plan": null,
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project"
}
exit=0

$ ./target/debug/agent-scaffold next --source <S>/projA/docs/plans/p.plan.toml \
    --ledger-fragment <S>/foreign.ledger.md --json
  "source": "no plan source",
  "resume_state_absent_reason": "ledger-not-this-project",
  "no_active_loop_reason": "no-plan-steps"
exit=0
```

`"plan": null` and `"source": "no plan source"` are the tool reporting that it read no plan. Both reasons fire anyway. The sidecar is right and `src/next.rs` is wrong, on both halves.

NOT A RE-RAISE. `grep -rn 'LogNotThisProject\|LedgerNotThisProject' docs/plans/agent-scaffold.reviews/` returns exactly one hit outside this round's cross-artifact file, in the round-4 cold read, and it is a note about `run_next`'s branching rather than about these comments. Twelve inc4 findings and triage files, zero mentions of the two definitions.

NOT THE SITES RULED OUT OF SCOPE ENTIRELY. My brief excludes `src/next.rs:162` and `:181-183`. Those are `NextProjection`'s own doc comment and `active_loop`'s, both verified present and correct at those lines by this round's cross-artifact facts 35 and 38. `:105` and `:140` are different lines in different items, and they were authored by this step's own inc2 rather than predating the step, which is the property `Q-55-currencyscope` used to draw its exclusion.

### SCOPE: IN SCOPE. Ruled against all four conditions.

1. PROVENANCE PREDATES THE BASE COMMIT: HOLDS. `git log --oneline -S 'not under the root of the plan this surface reads' -- src/next.rs` returns exactly `8beb1c2` (2026-08-03, "feat: refuse and omit on a round log or ledger the plan cannot vouch for"), which is inc2's feature commit and predates the base.
2. NO COMMIT IN RANGE MODIFIES THE CLAIM'S LINES: HOLDS. `src/next.rs` does not appear in `git diff --stat d8e8087^ HEAD` at all.
3. INDEPENDENT SUBJECT: FAILS, ON THE SECOND LIMB, AND I VERIFIED THE FALSIFIER RATHER THAN ACCEPTING THE CITATION. The comments were TRUE when written: before `269d075`, `status` and `next` fell through with NO root where they read no plan, so both containment filters went vacuous and neither reason could fire on that path. That commit's own message says so ("`status` and `next` fell through with NO root where they read no plan ... the second, decided by `Q-55-resumepairing` for the surface that reads no plan, was applied only in `run_resume`"), and its diff introduces `containment_roots` and the anchor-supplied root that make both reasons reachable with `"plan": null`. `269d075` is dated 2026-08-03 and is an inc2 fix-round commit, so THIS STEP'S OWN INCREMENT IS WHAT FALSIFIED THEM. That is `R3B-3`'s provenance shape exactly, and `R3B-3` was ruled in scope on it. The reading is settled in this task by three triage applications (`R1C-3`, `R1C-4`, `R2B-1`, then `R3B-3`) and one human decision (`Q-55-twinsites`, "a stale claim THE INCREMENT'S OWN CHANGE BROKE is in scope regardless of authorship"). The first limb fails too: the sidecar site that OWED this change is `:217` and `:229`, closed one round ago by `Q-55-reasondefs`, so the claim is about what the increment changed.
4. NO SHARED FIX: does not matter, since condition 3 already fails and all four are required. For the record it also fails, because fixing `src/next.rs` adds a site to `R4B-2`'s list.

THE COUNTER-ARGUMENT, PUT AND ANSWERED. `:388` excludes shipped prose because "inc4 changes no behaviour, so no shipped prose goes stale". These are not shipped prose; they are source doc comments, and `src/main.rs` is already in the impact list at `:386` on the same footing. `Q-55-currencyscope`'s declined third option covers claims that PREDATE the step, traced to `8017a2c` and `f230f80`; these were authored inside the step by inc2.

### SEVERITY: `medium` CONFIRMED, and I weighed `low` before confirming it.

FOR `low`: no behaviour is wrong, no user-visible string is wrong, and four words of a qualifier are a small thing.

WHAT CARRIES `medium`, and it is the same ground the round 3 triage confirmed `R3B-1` at `medium` on, with the same deletion-only remedy shape. These two comments are the ONLY definitions of two serialised contract tokens anywhere in the source. `README.md:240` names the tokens but does not define their trigger, and the sidecar's definitions are a plan artifact rather than something a consumer's author reads beside the type. `Q-55-jsonreason` exists precisely so a machine consumer can tell the causes apart, and a consumer author reading `:105` would conclude that a projection reporting `"plan": null` cannot also report `log-not-this-project`, when it demonstrably does. Severity is the impact if left unfixed, and what is left unfixed here is a false definition of a contract in the file that defines it.

THE AGGRAVATION THAT DECIDED IT RATHER THAN A COUNT OF TWIN-SITE DEFECTS. `Q-55-reasondefs` was decided one round ago, and its third option, widening the sweep bound, was PUT AND DECLINED. That option was whole-SIDECAR-file, not whole-repository, so it would not have reached `src/next.rs` either: the human did not knowingly accept these two sites, nobody opened them. The fix that corrected the SPECIFICATION did not check the CODE THE SPECIFICATION WAS WRITTEN FOR, which is the pattern `Q-55-twinsites` already recorded ("this task has been bitten THREE TIMES by a fix landing at one site while its twin survived a literal grep") and already prescribed checking. On the ordinal: the reviewer calls this the fourth recorded twin-site defect and the orchestrator's brief calls it the fifth. The ledger supports "at least the fifth" if `Q-55-w1figure`'s self-created twin-site disagreement and `Q-55-helpsurface` are counted, and the number is not reconstructable to one value. It is not load-bearing and I do not rule on it.

MINIMAL REMEDY: DELETION at both sites, the same four words the sidecar fix deleted, every remaining word drawn from the already-corrected twin. `src/next.rs:105` becomes "The resolved path is not under the root, so the tool cannot vouch that its records belong to that plan." `src/next.rs:140` becomes "The resolved ledger is not under the root: either an explicit `--ledger-fragment` outside it, or a default ledger anchored on a `--source` that itself lies outside it." NOTHING IS AUTHORED. Fix this BEFORE `R4B-2`, so that `R4B-2`'s bullet count is final when it is written.

## (2) `R4B-2`: it bears on whether the step may close, and the diff says the reviewer is right

REPRODUCED. I ran the whole-range diff myself rather than reading the reviewer's, and it matches byte for byte:

```
$ git diff --stat d8e8087^ HEAD -- . ':(exclude)docs/plans/agent-scaffold.ledger.md' \
    ':(exclude)docs/plans/agent-scaffold.reviews' ':(exclude)docs/metrics/workflow.jsonl'
 docs/plans/agent-scaffold.md                       | 175 ++++++------
 docs/plans/agent-scaffold.plan.toml                |  14 +-
 .../checks-runner-worktree-name-collision.md       |   2 +-
 .../instrument-magic-filename.md                   |   2 +-
 .../status-resume-ignores-json.md                  |   4 +-
 .../test-tmpdir-repo-assumption.md                 |   6 +-
 .../workflow-enforcement-tier.md                   | 147 +++++-----
 src/main.rs                                        |   8 +-
 tests/unsafe_pairings_are_refused_and_omitted.rs   |   5 +-
```

(`d8e8087` on this branch is the reviewer's `42ba172`, "docs: make the step's own claims current and specify inc4"; `docs/plans/agent-scaffold.md` is the generated view and is not an impact-list item.)

THE LIST, `:384-388`, names: THIS FILE; the three sidecars of check 21b; `src/main.rs:Projection`'s `plan` doc comment; `tests/unsafe_pairings_are_refused_and_omitted.rs`; then NOT `README.md`, NOT `pack/AGENTS.md`, NOT the deployed `.agents/` copies, NOT `CHANGELOG.md`. `Q-55-impactlist` was decided on the reading that enumerating the exclusions makes the list read as exhaustive, recorded at ledger `:561`, so that reading is settled by a human decision and is not mine to re-derive.

ALL THREE OMISSIONS CONFIRMED AGAINST THE DIFF ITSELF.

(a) `status-resume-ignores-json.md`, a FOURTH sidecar, edited by `c3ca69a` ("docs: apply the inc4 round 3 remedies", the reviewer's `7a2e776`). I confirmed the aggravating half: THE SAME COMMIT that edited this sidecar also amended the exclusions bullet at `:388` to record the `--help` change, and did not add a bullet for the file it had just edited. `git show c3ca69a` carries both hunks. The file is covered by neither check: check 21 is scoped to "THIS FILE" plus two regions of the plan TOML, and check 21b names three sidecars "AND ONLY THOSE".

(b) `docs/plans/agent-scaffold.plan.toml`, 14 lines across the `w1` waiver note and the `Q-55` record. THIS IS THE ONE THAT CONTRADICTS AN ACCEPTANCE CHECK THREE SECTIONS ABOVE IT IN THE SAME FILE. Check 21 at `:345` reads, verbatim: "THE PLAN SOURCES ARE TWO REGIONS OF `docs/plans/agent-scaffold.plan.toml`, a file this increment edited". The list, which enumerates its exclusions, does not list it. A reader auditing "did inc4 touch anything it did not declare" gets `No` from `:384-388` and `Yes` from both `git diff` and `:345`.

(c) `src/main.rs:run_status`'s comment at `:1192-1195`. The `src/main.rs` diff has exactly three hunks: `StatusArgs::resume`'s `--help` at `:461` (named in the exclusions bullet), `Projection.plan`'s doc at `:570` (named at `:386`), and this one, named nowhere.

SCOPE: IN SCOPE, and it is not close. Condition 1 FAILS: the whole INC4 section of the list was authored at `d8e8087`, the base commit itself, and `:388` was amended inside the range at `c3ca69a`. Condition 2 FAILS for the same reason. All four are required.

SEVERITY `medium` CONFIRMED. I weighed `low`, on the ground that no behaviour is wrong and no reader is misled about the tool. What carries `medium` is the ground `Q-55-impactlist` was itself decided on, plus two aggravations the earlier decision did not have. The step goes `complete` after this loop, so this list becomes the PERMANENT record of what the increment touched. And one omission is a direct contradiction with an acceptance criterion in the same document, which is exactly the shape `R3B-2` was confirmed at `medium` on ("a document whose definition of done contradicts its own acceptance criterion").

MINIMAL REMEDY: AUTHORED PROSE, two bullets and one clause, but every fact in them is already settled and recorded elsewhere, so nothing new is asserted. This is the same class and roughly the same size as the bullet `Q-55-impactlist` itself authorised.

- One bullet for `docs/plans/agent-scaffold.steps/status-resume-ignores-json.md`, in the `tests/...` bullet's exact form: name it and say no acceptance check states it.
- One bullet, or a clause on the `THIS FILE` bullet, for the two plan TOML regions, drawn verbatim from check 21's own sentence at `:345`.
- "and `run_status`'s comment" appended to the `src/main.rs` bullet at `:386`.
- AND, IF `R4B-1` IS FIXED FIRST AS PRESCRIBED, a fourth bullet for `src/next.rs`. Fix `R4B-1` first so this list is written once.

DOES IT BEAR ON WHETHER THE STEP MAY CLOSE? It bears on WHAT IS TRUE WHEN IT CLOSES, not on whether it may. Nothing in the increment is unbuilt or unmet; a documentation-impact list is short by three entries. Fixed, the list is exhaustive and check 21's sentence is consistent with it. Unfixed, the step closes with its own definition of done contradicting its own acceptance criterion, which is the defect class the increment exists to remove, permanently.

## (3) `R4A-1` and `R4A-2`: both "a check that cannot fail", and they rule differently

### `R4A-1`: VALID, `low`, OUT OF SCOPE. NOT a re-raise, and the mutation is real.

REPRODUCED, in a copy of `docs/plans` under the scratchpad, and the output matches the reviewer's byte for byte including the line counts:

```
$ agent-scaffold render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date
exit=0
$ printf '\nMUTATION-PROBE-LINE\n' >> docs/plans/agent-scaffold.md
$ agent-scaffold render docs/plans/agent-scaffold.plan.toml --check
warning: docs/plans/agent-scaffold.md differs from a fresh render (a hand-edit, or a stale render after a source edit) (the committed file has 2042 line(s); a fresh render has 2040); re-render with `agent-scaffold render docs/plans/agent-scaffold.plan.toml`
exit=0
$ agent-scaffold render docs/plans/agent-scaffold.plan.toml --check --strict
error: docs/plans/agent-scaffold.md differs from a fresh render (a hand-edit, or a stale render after a source edit) (the committed file has 2042 line(s); a fresh render has 2040)
exit=1
```

The in-tree precedent holds too: `.agents/checks.toml` declares `agent-scaffold render --check docs/plans/agent-scaffold.plan.toml --strict` as the project's render gate, so the remedy copies rather than invents.

IS IT A LIVE DEFECT OR A GAP IN A CHECK? A GAP, and a narrow one. Check 1's other three clauses (`cargo build`, `cargo test`, `cargo clippy`) all fail on a broken tree, so check 1 as a whole is falsifiable; only its fourth clause is inert under the preamble's own protocol at `:312` ("Every claim below is a command with an expected exit code, so a round is settled by running it rather than by reading the diff"). And check 23 at `:348` names the SAME command and asserts its OUTPUT, so the LIST is not blind to a divergent render even though this clause is. The defect is a clause that reads as a gate and is not one, in a list whose preamble asserts every entry is settled by an exit code.

IS THE EVIDENCE NEW AGAINST A SETTLED VERDICT, OR A RE-RAISE? NEITHER, AND THE REVIEWER FRAMED IT ONE STEP TOO DEFENSIVELY. `R3C-5` was ruled VALID and out of scope, not dismissed, so there is no dismissal to re-raise; and its subject was which FORM of the gate the eight inc4 review passes RAN, with its remedy placed in the orchestrator's transcript convention, outside the plan. I re-read the round 3 verdict and the round 3 detectability finding to confirm: neither names check 1, and check 1's text was untouched by the round 3 remedy by construction. The underlying mechanism fact (the plain form warns at exit 0) IS old, measured by round 3's `M1` and `M5`. What is new is the application to check 1's clause, which nobody had run. So `R4A-1` is a new finding on an adjacent subject, and the question of new-evidence-against-a-settled-verdict does not arise.

SCOPE: OUT OF SCOPE, argued against all four conditions, and it lands on the `R2B-4` precedent almost exactly.

1. PROVENANCE PREDATES THE BASE COMMIT: HOLDS. `git log --oneline -S 'Plan render pinned' -- <step file>` returns `1a04071`, the original `Q-55` fold, long before inc4's base.
2. NO COMMIT IN RANGE MODIFIES THE CLAIM'S LINES: HOLDS ON THE OPERATIVE LINE, AND I STATE THE WRINKLE RATHER THAN HIDE IT. Check 1 at `:316` does not appear in the inc4 diff at all. The PREAMBLE at `:312` DOES appear in the diff, but the change is confined to the red-then-green enumeration ("EACH INCREMENT" becoming "EACH INCREMENT THAT CHANGES BEHAVIOUR", plus a floor-not-a-total sentence); the clause the finding turns on, "settled by running it rather than by reading the diff", is byte-identical before and after. So no commit in range modified the claim.
3. INDEPENDENT SUBJECT: HOLDS, BOTH LIMBS. The clause has been inert since `1a04071`, so no increment falsified it, and check 1 is not what inc4 changed: the checks inc4 authored or widened are 16, 19, 21, 21b, 22 and 23, confirmed from the diff.
4. NO SHARED FIX: HOLDS. No in-scope remedy touches check 1.

That is `R2B-4`'s shape: a defect in an acceptance check's own text, never true, not falsified by any increment. Ruling it in would do what the round 2 triage warned against, make condition 3 unsatisfiable for a documentation-currency increment and quietly retire the precedent.

SEVERITY `low` CONFIRMED. Nothing is misreported and the property is covered elsewhere in the same list.

MINIMAL REMEDY, RECORDED PER THE PRECEDENT'S GUARD: TOKEN. Add `--strict` to check 1's render clause, or assert its output the way check 23 does. One flag, copied from `.agents/checks.toml`.

### `R4A-2`: VALID, `low`, IN SCOPE. The check IS in the increment's own range.

REPRODUCED BY READING THE PROCEDURE, which is the right evidence for a claim about what a procedure does not do. Check 21 at `:345`: "run each quoted fragment ... as a literal search against the file it is attributed to ... A quotation with no match in the tree is either RE-TENSED, so the sentence describes the pre-increment state it was written about, or DELETED where the sentence carries nothing else". The two outcomes for a non-matching quotation are accept-if-past and fix-if-present. Nothing asks whether a past-tense quotation ever matched ANY revision, so a fabricated or drifted historical quotation passes by being written in the past tense. The clause "it is NOT re-pointed at a similar-looking sentence" forbids one specific abuse and requires no verification.

IS IT A LIVE DEFECT OR A GAP? A GAP, AND THE REVIEWER CLOSED IT ITSELF, which is the reason it stays `low`. I sampled the missing half it ran, whitespace-normalised as this task has done since round 3, because two of the eleven listed HITs fail a naive literal grep for the ordinary reason that a doc comment wraps across `//` lines:

- `git show 5684b5f:pack/AGENTS.md | grep -c "the deterministic \`validate --workflow\` check, once built, is the backstop that the required reviewed rounds happened before a step is marked complete"` returns 1, at line 93, which is the cited line. The same string is absent from `pack/AGENTS.md` today. That is the step file's most load-bearing historical quotation and it resolves exactly where its tense says.
- `git show 1dac3dc:src/main.rs` carries `PathBuf::from(format!("docs/plans/{task}.ledger.md"))` literally.
- `git show 285a6a3:src/next.rs:114` carries "Why there is no active loop, for the human renderer. Not serialised (the JSON" and continues on the next `///` line, and `git show 5684b5f:src/main.rs:804` carries "An absent file (the metrics log, or a `--plan` path) is not a validation" and continues. Both are HITs normalised and misses literal, which is the same false-alarm class the round 3 triage recorded at sidecar `:52`.

So no historical quotation in the file is fabricated, and the finding is a hole in the procedure rather than a false claim shipping.

SCOPE: IN SCOPE, AND CONDITION 1 SETTLES IT ALONE. `git log --oneline -S 'EVERY CITATION AND EVERY QUOTATION IN THIS FILE' -- <step file>` and `git log --oneline -S 'is either RE-TENSED' -- <step file>` both return `d8e8087`, the increment's own base commit. Check 21 was authored BY inc4 TO VERIFY inc4, so its provenance is inside the range, condition 1 fails, and all four conditions are required for out-of-scope. Round 3 already amended it twice on the same ground.

SEVERITY `low` CONFIRMED, and I weighed `medium` on the `R3C-4` parallel. `R3C-4` earned `medium` partly because check 21 was FAILING at that commit, at `:367`. Here nothing fails, the reviewer ran the uncovered half to empty, and the gap's consequence is measured rather than assumed. `low`.

MINIMAL REMEDY: ONE CLAUSE ON CHECK 21, requiring that a re-tensed quotation resolve at the revision its tense names, naming the existing procedure (`git show <pre-increment sha>:<path>`). CLASS: AUTHORED PROSE, and I am CORRECTING THE REVIEWER, which calls this "a DELETION-CLASS one rather than authored prose". It is not. It replaces an accepting branch with a conditionally accepting one, which adds a requirement; the words are new even if few.

THE ALTERNATIVE THE REVIEWER OFFERED, RULED RATHER THAN LEFT OPEN. Recording the gap in the ledger instead of amending check 21 a third time is a defensible disposition, and the reviewer was right to put it. I rule FIX IT, for two reasons. Check 21's stated post-condition is "EVERY CITATION AND EVERY QUOTATION ... RESOLVES", and re-tensing is the DOMINANT remedy class of the very increment this check verifies (18 sites under `Q-55-spectime`, 8 more under `Q-55-receiptcurrency`), so the branch the procedure does not cover is the branch the increment mostly used. And the check is what a later reader will believe about what was verified, once the step is `complete`. One clause is a proportionate price. If the orchestrator judges a third amendment to one check too expensive, that is a decision for the human and not a re-adjudication of this verdict.

## (4) `R4B-3`: the new evidence qualifies, and the finding is still out of scope

DOES THE EVIDENCE QUALIFY? YES, AND THE CONTRACT IT IS TESTED AGAINST IS THE WRONG ONE, WHICH MAKES THE ANSWER EASIER. The ledger rule protects a SETTLED FINDING from being re-raised without new evidence its verdict was wrong. `R2B-4`'s verdict was "valid, out of scope, token remedy" and it is not contested here; its subject, acceptance check 19's second layout, has since been reworded to "a SYMLINK out of the plan's project root" and I confirmed that at `:342`. What `R4B-3` contests is an assertion made IN PASSING inside that verdict, at ledger `:565`, that "`:257`, `README.md:236` and `CHANGELOG.md` all state the general rule correctly". `README.md:236` was never itself a finding, so there is no settled finding to re-raise. And the reviewer brought a measurement where the round 2 triage had none, which would qualify as new evidence even if there were.

REPRODUCED, BOTH ORIENTATIONS, MY OWN FIXTURES.

Case A, `<root>/docs/metrics` a symlink to the in-root sibling `<root>/elsewhere`, plan where it belongs, log at `<root>/elsewhere/workflow.jsonl`:

```
$ cd <S>/symA/root && agent-scaffold validate --source docs/plans/p.plan.toml --workflow
docs/metrics/workflow.jsonl: 0 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

Case B, the mirror, `<root>/docs/plans` a symlink to `<root>/elsewhere`:

```
$ cd <S>/symB/root && agent-scaffold validate --source docs/plans/p.plan.toml --workflow
--workflow would join docs/plans/p.plan.toml against docs/metrics/workflow.jsonl, which is not under the plan's project root <S>/symB/root/elsewhere; pass a `--metrics` under that root, run against the plan's own log, or correct the `--source` and `--plan` pair
exit=1
```

WHICH SENTENCE IS FALSE, TESTED CLAUSE BY CLAUSE. `README.md:236` conditions on the symlink TARGET: "a symlink pointing somewhere the other one is not under". In case A the target is `<root>/elsewhere` and `docs/plans` is not under it, so the README's condition HOLDS and the README predicts a refusal that does not happen. `CHANGELOG.md:24` and sidecar `:257` condition on the two REAL locations landing under different roots; in case A both land under `<root>`, so both correctly predict no refusal. I tried to find a reading of the README sentence on which it is true, taking "the other one" as the plan FILE rather than the directory, and it fails identically. The README's condition is symmetric and the behaviour is not.

SCOPE: OUT OF SCOPE. All four conditions hold, and the reviewer expected this and argued it rather than hiding it.

1. PROVENANCE PREDATES THE BASE COMMIT: HOLDS. `git log --oneline -S 'is a symlink pointing somewhere the other one is not under' -- README.md` returns `b236b10`, inc2's documentation commit.
2. NO COMMIT IN RANGE MODIFIES THE CLAIM'S LINES: HOLDS. `git diff --stat d8e8087^ HEAD -- README.md CHANGELOG.md pack/ .agents/ AGENTS.md` is EMPTY.
3. INDEPENDENT SUBJECT: HOLDS, BOTH LIMBS. The sentence was wrong on the day it was written, so no increment of this step falsified it, and shipped README prose is not what inc4 changed. `:388` excludes `README.md` explicitly and on a ground that is true here.
4. NO SHARED FIX: HOLDS. Nothing in-scope touches `README.md`.

This is the same ruling `R2B-4` got, on the same axis, and consistency is itself a reason: the two findings are the same population measured against two different sentences.

SEVERITY `low` CONFIRMED. The failure direction is benign: a user with the case-A layout is told their layout will be refused and finds it works. No wrong value, no false green over foreign evidence, and the same README paragraph's closing sentence states the true general rule ("The rule is CONTAINMENT, not identity"). What keeps it a finding at all is that two shipped surfaces state one rule with two different conditions, and only one of them is right.

MINIMAL REMEDY, RECORDED PER THE PRECEDENT'S GUARD: TOKEN. Replace the README's condition with the CHANGELOG's, which is already in the tree and already correct: "a symlink pointing somewhere the other one is not under" becomes "a symlink that lands the plan and the log under different real roots". NOTHING AUTHORED.

## (5) The out-of-scope precedent, applied per finding

Applied individually above. The summary, so the arithmetic is auditable:

| id | 1. provenance predates base | 2. no commit in range on the lines | 3. independent subject | 4. no shared fix | ruling |
| --- | --- | --- | --- | --- | --- |
| `R4B-1` | holds (`8beb1c2`) | holds (`src/next.rs` untouched) | FAILS, second limb (`269d075`, inc2) | fails | IN SCOPE |
| `R4B-2` | FAILS (`d8e8087`, the base) | FAILS (`c3ca69a` amended `:388`) | fails | n/a | IN SCOPE |
| `R4A-2` | FAILS (`d8e8087`, the base) | FAILS | fails | n/a | IN SCOPE |
| `R4B-4` | holds for `:14` and `:1726` | FAILS (`:282` added at `d8e8087`) | FAILS, second limb | holds | IN SCOPE |
| `R4A-1` | holds (`1a04071`) | holds | holds, both limbs | holds | OUT OF SCOPE |
| `R4B-3` | holds (`b236b10`) | holds | holds, both limbs | holds | OUT OF SCOPE |

THE GUARDS THE PRECEDENT CARRIES ARE APPLIED. Both out-of-scope findings carry their minimal fix above. The counts report IN-SCOPE VALID 4 and OUT-OF-SCOPE VALID 2 explicitly rather than as a bare clean. Neither out-of-scope finding resets the streak, and neither is load-bearing for this round's category: the round is new-valid on the four in-scope findings alone.

## `R4B-4`: VALID, `low`, IN SCOPE

REPRODUCED, all four sites opened.

`docs/plans/agent-scaffold.plan.toml:1726`, inside the `Q-55` `ask`: "the second planner pass re-derived the set as THREE (the resolution rule and all its call sites; the containment refusal; the tier policy plus the documentation half)".

Sidecar `:14`: "THIS FILE IS THE SECOND PLANNER PASS." Sidecar `:275`, opening the increments section: "the set below is that principle re-applied to the widened scope, not an appendix to the old pair", and the set below is FOUR bullets at `:279-282`. The plan TOML carries four `[[step.increment]]` entries.

BOTH ARE TRUE AT DIFFERENT TIMES AND THE JOIN IS FALSE NOW, which is why it is a cross-artifact finding rather than a false claim. `git log --oneline -S 'THIS FILE IS THE SECOND PLANNER PASS'` and `-S 're-derived the set as THREE'` both return `7807c6b`, one commit, the second planner pass writing both. `git log --oneline -S 'THE DOCUMENTATION-CURRENCY PASS THAT CLOSES THE STEP'` returns `d8e8087`, the inc4 base: THE FOURTH BULLET WAS ADDED BY THIS INCREMENT. So a reader joining the record a later reader consults for this step's scope with that pass's own file gets three and four for one question.

I TESTED THE DISMISSAL CASE BEFORE CONFIRMING. Does the file resolve it elsewhere? `:308` says inc4 was "classified at loop-open", which is about the RISK CLASS being assigned then and does not say the increment was added then; a reader can take it as an increment that existed and was classified late. `:294-296` mentions inc4 without dating it. So nothing in either artifact lets the reader resolve the join without `git log`, which is the reviewer's claim exactly.

SCOPE: IN SCOPE. Condition 2 FAILS: `:282`, the bullet that turns three into four, was added inside the range at `d8e8087`. Condition 3 FAILS on the second limb: the increment's own change is what created the disagreement.

SEVERITY `low` CONFIRMED. Nobody acts wrongly on it. The authoritative increment set is `[[step.increment]]`, which is correct, and the check that reads it passes. It is a finding because the step closes on this loop and `Q-55` is what a later reader opens to learn what the step was scoped as.

MINIMAL REMEDY, AND I CHOOSE, BECAUSE THE ROUND 3 RULE REQUIRES A TRIAGER'S REMEDY TO ACCOUNT FOR EVERY SITE THE REVIEWER NAMED. The reviewer named two options and declined to pick. TAKE THE SIDECAR SIDE: DELETE the four-word sentence "THIS FILE IS THE SECOND PLANNER PASS." from `:14`. The surrounding sentences already carry everything it conveys ("The first pass scoped two defects and two increments", "superseded here"), and the paragraph reads correctly without it. DELETION class, nothing authored.

DO NOT TAKE THE PLAN TOML SIDE, and the ground is recorded rather than asserted: ledger `:695` records the convention "APPEND a correction rather than rewrite a decision record", and `Q-55-receiptcurrency` had to go to the HUMAN to authorise a mere TENSE change inside this same record on the argument that a tense revises nothing the convention protects. Deleting a COUNT from a decision receipt is more than a tense change, so it is a human decision and not a triager's minimal remedy. `:1726` is left alone as a dated record, which is what the append convention is for. If the orchestrator prefers that side anyway, it is an escalation, not a fix.

## The three ways a review is defeated, applied

DIMENSIONS. The check-runner names its unvaried axes explicitly (one platform, one profile, uid 1000 plus one uid-0 cell, no concurrency, no `--instrument` scaffold) and its result is a set of POSITIVE measurements, 33 passes and 21 reds, each refuted or confirmed by a single named configuration, so the axis bound limits how far the passes generalise rather than making them wrong. The one place a NEGATIVE result carries the axis bound is its "NO behavioural defect was found", and I read that as "none on this axis set" exactly as round 3's triage read the detectability lens's null. The cold read's negative carries a sharper bound and it is a LENS bound rather than an axis bound: it swept its assigned text completely and found no false claim, while two of this round's findings sit inside that same text as cross-artifact JOINS, which its lens does not form. I say so under its own entry above rather than letting the zero read wider than it is. `R4B-1`'s fixture varies the one dimension that matters, whether a plan is read, and a single configuration is sufficient to refute a universal.

CONTROLS. `R4A-1`'s control is the `--strict` run, and it discriminates the two hypotheses cleanly: if the plain form were merely quiet, `--strict` would also exit 0; it exits 1 on the same tree, so the divergence is real and the plain form's exit code is blind to it. `R4B-3`'s control is the mirrored orientation, and it is the whole finding: without case B, case A's green could mean the guard is off; with both, the asymmetry is established and the README's symmetric condition is refuted. `R4B-1`'s control is `"plan": null` and `"source": "no plan source"` in the same output as the reason, which is the tool's own report that it read no plan, so no separate argument about what "reads" means is needed. The check-runner's own control is the one I judged most load-bearing and the one I re-ran: three distinct `md5sum`s and three distinct `--metrics` help strings, without which its `as-pre2` and `as-pre3` measurements would have been the same binary twice.

ADJUDICATION. The trap this round sets is `R4B-1`. The tempting move is to read the sidecar at `:217` and `:229`, see the corrected text a human decision landed one round ago, and conclude the claim is closed. That asks what REMAINS rather than what the decision REACHED, and what it reached was two of four sites. I checked what the fix did not touch, by grepping twelve findings and triage files for the enum names and getting zero hits, and by measuring the behaviour against the surviving text rather than against the corrected text. The same trap sits on `R4A-2`: check 21 PASSES today, and reading "the check passes" as "the check is sound" is the same error. The reviewer avoided it by running the branch the procedure does not cover, and I sampled that run rather than accepting it.

## Recorded residuals and settled dismissals

I checked all six findings against inc2's four recorded residuals (the in-root bound; the single-anchor `..` case with its widened bound; `ADV-2`'s rejected-ledger context slot; `R2A-2`'s off-convention `--source` surface, an INC2-era id), against inc3's four (`R3A-1`'s inert remedy clause and `R4A-1`'s reader-level discrimination, both INC3-era ids and both UNRELATED to this round's `R4A-1`, which is a check-runner id on check 1's render clause; the plain-`validate` mode-000-file-versus-unsearchable-directory inconsistency; and the containment TOCTOU under a FIFO-widened mid-run symlink swap), against `F-5`, and against the five settled dismissals `R1A-5`, `R1A-7`, `R1A-8`, `R1B-3` and `R2A-5`.

NONE OF THE SIX IS A RE-RAISE. Three came close enough to state in place, and all three are argued in their own entries rather than asserted: `R4A-1` against `R3C-5`, `R4A-2` against `R3C-4`, and `R4B-3` against `R2B-4`. The cold read reached `F-5` independently and stopped at it with the ledger's own verdict and no new evidence, which is correct handling and not a finding. The check-runner measured the plain-`validate` mode-000 residual because check 16 pins it, reported that it reproduces as pinned, and did not raise it, which is also correct handling.

I raised nothing on the items ruled out of scope entirely in my brief: `run_validate`'s "`--plan` still clap-required" claims, `src/next.rs:162` and `:181-183`, the Status narrative at `docs/plans/agent-scaffold.md:7`, and the `src/checks.rs` citations in `checks-runner-worktree-name-collision.md`. On `src/next.rs`, I verified specifically that `R4B-1`'s two sites are `:105` and `:140`, in `MetricsAbsentReason` and `ResumeStateAbsentReason`, and are NOT the excluded `:162` and `:181-183`, which are `NextProjection`'s own doc and `active_loop`'s and which this round's cross-artifact facts 35 and 38 confirm are present and correct. No finding this round concerns line length or prose wrapping, and none of the four items round 3 ruled into a future backlog step was re-raised.

NOTHING WAS DISMISSED AT ALL THIS ROUND, so nothing was dismissed at `high` or above and NO INDEPENDENT DISMISSAL RE-CHECK IS OWED.

## Round outcome

NEW-VALID, AND NEW-VALID ON IN-SCOPE FINDINGS ALONE.

Four in-scope valid findings (`R4B-1` and `R4B-2` at `medium`, `R4A-2` and `R4B-4` at `low`), two out-of-scope valid (`R4A-1`, `R4B-3`) which do not reset the streak under the recorded precedent and whose minimal fixes are recorded above per its guard. Zero dismissals. Severity ceiling `medium`.

The streak stays 0 of 2. This is round 4 of a cap of 5, so round 5 escalates to the human whatever it returns.

FIX ORDER FOR THE BRIEF, because two of the four interact: `R4B-1` first, `R4B-2` second so its bullet count is final when written, then `R4A-2` and `R4B-4` in either order. Three of the four remedies are deletion or copied-fact class; the one that authors anything (`R4B-2`) authors two bullets and a clause whose every fact is already settled and recorded, which is the class `Q-55-impactlist` itself authorised.

## Fixture hygiene

All fixtures under `<scratchpad>/triage-inc4-r4/` only: `fix`, `projA`, `renderprobe`, `symA`, `symB`, `pre3` and `pre3-target`. Nothing written to bare `/tmp`. Nothing deleted outside that subdirectory. No `chmod` was used anywhere, so none is owed a restore. The render mutation was applied to a COPY of `docs/plans` in the scratchpad, never to the worktree. Nothing in the main repository or in any other worktree was created, modified or deleted. `git status --short` in this worktree shows only this file.

## ASCII check

`LC_ALL=C grep -n '[^ -~]' docs/plans/agent-scaffold.reviews/workflow-enforcement-tier-inc4-r4-triage.md` returns 0 hits, verified before commit.
