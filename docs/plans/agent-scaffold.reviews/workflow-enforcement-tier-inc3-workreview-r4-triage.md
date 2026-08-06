# `workflow-enforcement-tier-inc3` work review, ROUND 4, TRIAGE

Triaged in worktree `.claude/worktrees/triage-inc3-r4` on branch `triage/inc3-r4` at `30df7f4`, the tip of the branch under review with both round 4 reviewers' findings merged in. Two findings files were triaged: `...-r4-reviewer-adversarial.md` (one finding, `R4A-1`, `low`) and `...-r4-reviewer-cold.md` (zero findings, plus an acceptance-check table).

All three prior triage files were read in full before any ruling. The do-not-relitigate list is treated as settled and nothing below raises or reopens an item on it.

## Method

TOOLCHAIN, confirmed before every build-dependent claim, with no `2>/dev/null` on the export:

```
$ cd <worktree> && direnv allow && eval "$(direnv export bash)" && which cargo
/nix/store/76jaab43a2l7n7fiifxjngp68kk167vm-rust-mixed/bin/cargo
cargo 1.98.0-nightly (a335d47ff 2026-06-26)
```

TWO BINARIES, both built by me from source rather than taken from another tree:

| Name | Commit | Location |
| --- | --- | --- |
| NEW | `30df7f4` | this worktree's `target/debug/agent-scaffold` |
| PRE | `9eeca42` | `<scratch>/tri-r4/build/pre/target/debug/agent-scaffold`, exported with `git archive` and built independently |

`<scratch>` abbreviates the session scratchpad directory. Every fixture lives under `<scratch>/tri-r4/`, a directory of my own naming. `TMPDIR` pointed at `<scratch>/tri-r4/tmpdir`, outside any git repository, for `cargo test`.

GATES MEASURED BY ME on the tree as triaged: `cargo test` 422 passed / 0 failed across nine binaries; `cargo clippy --all-targets -- -D warnings` exit 0; `render --check` reports `up to date`. Both reviewers' gate reports reproduce. No source edit was made at any point; `git status --porcelain` is empty on this worktree and the main repository was not touched. No `nix fmt`, no `just scaffold-self`.

---

# Part 1: `R4A-1`

## Verdict: NOT VALID as stated. Severity, had it been valid: `low`. The measurement is accurate, is SHARPER than the reviewer had it, and is RECORDED below as a residual observation. No source change.

## What reproduces, exactly as reported

The reviewer's pair, rebuilt from its own recipe and compared with `cmp` on both streams and the exit code:

```
$ (cd dA && NEW validate --source docs/plans/p.plan.toml --workflow)      # non-instrumented
exit=1
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check
could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this
project's log, or record the project's review rounds there

$ (cd dB/docs/plans && NEW validate --source p.plan.toml --workflow)      # instrumented, cost (i)
(identical)
exit=1

cmp A.out B.out -> IDENTICAL   cmp A.err B.err -> IDENTICAL   exit -> IDENTICAL
```

The control holds: the same `dB` fixture run from its own root reads its real one-record log and exits 0 with `workflow invariants hold`, so `dB` is genuinely the instrumented population. PRE's stderr is identical across the same pair (the conflation is pre-existing), and PRE's stdout differs only in the echoed `--source` spelling, as the reviewer states and qualifies.

Provenance and scope both hold. `git log -S` puts the sentence in this increment's build commit, `git show 9eeca42:src/main.rs | grep -c mis-anchored` returns 0, and the ledger's locked scope for the increment says in terms that "9 of the 14 [added lines] are the comment". The sentence is this increment's to answer for.

## The construction the finding needed, in the direction that HURTS it, and it makes the measurement stronger

I first built what looked like a counterexample: a conventional instrumented project run mis-anchored a DIFFERENT way, from `<root>/docs` with `--source plans/p.plan.toml`. It reports `no round log at plans/docs/metrics/workflow.jsonl`, visibly unlike the non-instrumented case's `docs/metrics/workflow.jsonl`, which appears to satisfy the comment's claim in a sub-case.

THAT COUNTEREXAMPLE IS AN ARTIFACT OF VARYING THE WRONG DIMENSION, which is the trap this brief names, and I record my own near-miss because the corrected experiment is the load-bearing one. It varies the SPELLING of the run, not the POPULATION. The correct experiment holds the spelling fixed and varies only whether the project is instrumented:

| Spelling, held fixed | instrumented (real log at the project's own `docs/metrics`) | NOT instrumented (no log anywhere) | population varied |
| --- | --- | --- | --- |
| `cd docs/plans; --source p.plan.toml` (cost (i)) | `no round log at docs/metrics/workflow.jsonl`, exit 1 | identical, exit 1 | stdout, stderr, exit ALL IDENTICAL |
| `cd docs; --source plans/p.plan.toml` | `no round log at plans/docs/metrics/workflow.jsonl`, exit 1 | identical, exit 1 | stdout, stderr, exit ALL IDENTICAL |
| absolute `--source` from an unrelated cwd | reads the log, `workflow invariants hold`, exit 0 | the problem naming the absolute path, exit 1 | DIFFERENT, and this run is not mis-anchored at all, so it is not a case of the two populations |

So the message-level identity is not confined to accepted cost (i): it is UNIVERSAL over every spelling that misses. The resolved path is a function of the arguments and the cwd alone, and the problem text is a function of the resolved path alone, so nothing in the output can vary with the presence of a log the run never looked at. The reviewer measured one pair and framed the identity as cost (i)'s; it is the mechanism's, everywhere.

## Why the finding is nonetheless NOT VALID

The sharper measurement kills the sub-case defence (there is no spelling at which the message alone separates the two populations), so the ruling rests on one question only: DISTINGUISHABLE BY WHOM. The finding requires the sentence to be FALSE, and it is false only under the reading in which "distinguishable" means "the tool emits different output". Three things decide against that reading.

FIRST, THE INCREMENT SAYS WHO THE OBSERVER IS, in a second site the finding does not cite. `tests/validate_workflow_toml_source_needs_no_plan.rs:202-204`, authored by the same increment about the same message, states the same claim with the observer named: "The problem names the path the tool looked for, so A READER CAN TELL a non-instrumented project from a mis-anchored run." That sentence is true, and it is the same author's gloss on the terser one at `src/main.rs:1051-1052`. A sentence with a true reading and a false one, whose author states the true reading explicitly elsewhere in the same increment, is compressed, not false.

SECOND, THE READER-LEVEL CLAIM IS TRUE, AND NAMING THE PATH IS WHAT MAKES IT TRUE. The printed path is the lexical resolution (`resolve_metrics_path`, kept lexical on purpose so a resolved path keeps the caller's spelling), so it is exactly the path handed to `try_exists` and is resolved against the same cwd the reader ran in. The reader therefore learns precisely which file was probed. In the mis-anchored run that file is not their log; in the non-instrumented one it is where their log would be and is not there. Before this increment the message named no path at all and told a mis-anchored operator "the metrics log is missing", which is flatly false for them and gave them nothing to work from. The byte-identity the finding measures is a comparison across two different cwds that no single operator ever makes.

THIRD, THE MESSAGE'S OWN TWO REMEDIES ARE THE DESIGN THE SENTENCE JUSTIFIES, and both are live, one per population. Measured:

```
mis-anchored reader obeys the FIRST clause ("pass a `--metrics` naming this project's log"):
$ (cd dB/docs/plans && NEW validate --source p.plan.toml --workflow \
                                    --metrics ../../docs/metrics/workflow.jsonl)
  ../../docs/metrics/workflow.jsonl: 1 records, valid
  p.plan.toml vs ../../docs/metrics/workflow.jsonl: workflow invariants hold          exit=0

non-instrumented reader obeys the same clause: nothing to name, the problem repeats,
so the SECOND clause ("record the project's review rounds there") is that reader's.  exit=1
```

One message, the path named, both remedies carried, and the reader's own knowledge selects. That is a coherent design and it is what the sentence asserts. It is the opposite of a claim that the tool separates the two.

AGAINST THE PROJECT'S OWN SETTLED LINE, this sits on the not-a-finding side. Round 3 ruled `R3B-1` VALID because "only" is an EXHAUSTIVENESS word that one counterexample falsifies, and in the same file ruled the neighbouring sentence NOT A FINDING because it "under-describes rather than mis-states". `R3A-2` was VALID because a comment named a gate the code no longer had, a flat error about the code checkable by grep. This sentence carries no exhaustiveness word, and its factual half (the problem names the resolved path) is true of the code. What it omits is that the distinction is the reader's to draw and is weakest exactly where the printed relative path is spelled identically to the project-relative path the reader calls their log. That is under-description.

WHAT THE REMEDY WOULD REMOVE, since the brief asks. The proposed ten-word deletion removes a claim that is true under the reading its sibling states, and removes it from ONE of the TWO sites that make it. A fix pass acting on the finding as written would leave `tests/...:203` asserting the same thing in plainer words, which is the shape this project has five retrospective and one prospective measurement against: the next round finds the site the last fix pass did not touch. If a human prefers the sentence gone, it is a deletion at two sites and no behaviour, message, test or exit code moves; nothing in this ruling makes that harder.

SEVERITY, RATED MYSELF. Had I upheld it, `low` and not higher, calibrated against `R3A-2` and `R3B-1` in this same increment: comment-only, no shipped text repeating it (I re-ran the reviewer's grep over `README.md`, `CHANGELOG.md`, `pack/`, `AGENTS.md` and `.agents/` and confirm the claim appears in no shipped surface), no behaviour, no exit code, no false green. The increment's subject matter does not raise it: a false claim about a distinction is not made worse by the distinction being the topic when nothing acts on the claim.

## RECORDED AS A RESIDUAL, because the measurement should not be rediscovered

One line for the ledger's residual list, no source or prose change: `validate --workflow` cannot distinguish an uninstrumented project from a mis-anchored run on ANY spelling, since the problem text is a function of the resolved path alone; the distinction the arm's comment relies on is drawn by the reader from the named path plus knowledge of where their own log is, and it is weakest under accepted cost (i), where the printed cwd-relative path is spelled identically to the project-relative path the reader calls their log. The only mechanism that would make the tool itself discriminate is naming the DERIVED PROJECT ROOT beside the path, which `unpairable_log_note` already does on the containment surface. That is an addition, it is nobody's finding this round, and it belongs to whoever next owns this message, not to a fix pass here.

---

# Part 2: spot-check of the cold reviewer's zero-finding report

A zero-finding report is a claim like any other. I re-ran four of the five acceptance checks in the table against BOTH binaries, on fixtures I scaffolded myself. Check 16's vacuous pass is already recorded and scheduled and is not re-raised.

| Check | Cold reviewer's claim | My measurement | Verdict |
| --- | --- | --- | --- |
| 15 | Discriminates: NEW exits 1 with the new problem, PRE exits 0 with the skip note | On a fresh non-instrumented scaffold: NEW exit 1 naming the resolved path and saying the check could not run; PRE exit 0 with `--workflow has a plan source but the metrics log is missing; skipping the workflow check` plus a `valid` summary | CONFIRMED |
| 17 | A CONTROL, not a discriminator: NEW and PRE byte-identical | On a `complete` step with an empty log: both builds print the same W3 sentence (`has no round records and no covering waiver ...`) at exit 1 | CONFIRMED, and the characterisation is the right one: W3 is untouched by this diff, so the increment is additive rather than a replacement of a working check |
| 18 | Discriminates, and pins accepted cost (i) | `cd docs/plans && validate --source TEMPLATE.plan.toml --workflow`: NEW exit 1, PRE exit 0. Repeated on a genuinely INSTRUMENTED project (cost (i) proper, which the check's own wording is about): NEW exit 1, PRE exit 0, and neither build reads the project's real log | CONFIRMED |
| 20 | The qualifier is new to this increment, and the sentence predicts check 15's exit code on BOTH populations | A PRE-scaffolded fixture's `AGENTS.md:93` reads "the deterministic `validate --workflow` check, ONCE BUILT, is the backstop", with no log-scoped qualifier; the NEW-scaffolded one reads "and on a project with NO ROUND LOG YET, which every project scaffolded without `--instrument` remains, that check exits non-zero reporting that it could not run rather than passing". A freshly `--instrument`ed fixture carries the same sentence (`grep -c` = 1), renders no `docs/metrics` and no `*.jsonl` at all, and exits 1 on check 15, which the sentence's operative rule predicts correctly | CONFIRMED, including the half `T-2` turned on |

The cold reviewer's `cargo test` 422/0, clippy exit 0 and `render --check` up to date all reproduce on my own build. Its PRE binary was borrowed from a pre-existing worktree rather than built; mine was built from `git archive 9eeca42`, and every PRE result it reports reproduces on my independent build, so that shortcut cost nothing.

I found nothing in the cold reviewer's report that is wrong, and no gap in it beyond the coverage its own text states.

---

# Part 3: the containment TOCTOU, attribution

## Ruling: REAL, PRE-EXISTING, OUT OF SCOPE for this increment, ROUTED to the validation-constraints step. The adversarial reviewer's decision not to count it is correct.

Reproduced by construction on both binaries, not read. Fixture: a FIFO at `proj/docs/plans/p.plan.toml`; an empty in-root `proj/docs/metrics/decoy.jsonl`; `proj/docs/metrics/workflow.jsonl` a SYMLINK initially pointing at `foreign/foreign.jsonl` OUTSIDE the project root; the foreign log holding a converged round record for the plan's `complete` step. The run is started, the symlink is swapped to the in-root decoy while the run blocks on the FIFO, then the plan body is fed in.

```
RACE, NEW (30df7f4)                          RACE, PRE (9eeca42)
  docs/metrics/workflow.jsonl: 1 records, valid          (byte-identical stdout and stderr,
  docs/plans/p.plan.toml: 1 steps, ..., valid            confirmed with diff on both streams)
  ... vs docs/metrics/workflow.jsonl: workflow invariants hold
  exit=0                                                 exit=0

BOTH STATIC ENDPOINTS REFUSE, same binary, same fixture:
  symlink left at the FOREIGN log  -> the containment refusal naming the root   exit=1
  symlink left at the in-root DECOY -> W3: `only-step` is `complete` but has no
                                       round records and no covering waiver     exit=1
```

So a green is produced over evidence read from outside the project root, only by the interleaving, and it is byte-identical on a commit that predates the whole increment.

ATTRIBUTION, on the mechanism and not only on the byte-identity. The log READ sits at `src/main.rs:845-847` and the containment guard at `:990-992`; at `9eeca42` the same two sit at `:837-838` and `:981`, in the same order, with the same blocking source read between them (`:881` here, `:872` there). The guard is unchanged CONTEXT in `git diff main...HEAD`, and the only line this diff changes in that stretch is the probe (`metrics_path.exists()` -> `metrics_path.try_exists()`), which cannot affect this race: the symlink's target exists at probe time under either predicate. The increment neither opened the window nor widened it.

ROUTED, not raised, to the validation-constraints step beside `R2A-4`, `R3A-3` and the queued plain-`validate` inconsistency: same family, a guard answering from an observation other than the one that decides. Its threat model is weaker than the rest of that queue (it needs a concurrent writer or an adversary already inside the repository), which is the routing note and not a reason to drop it. A false green is exactly the defect class this step exists to remove, so the attribution mattered in both directions and it was measured in both.

---

# Part 4: what the reviewers missed

Three, and the first two bear on the ruling above.

1. **A SECOND SITE MAKES THE SAME CLAIM.** `tests/validate_workflow_toml_source_needs_no_plan.rs:202-204` says "The problem names the path the tool looked for, so a reader can tell a non-instrumented project from a mis-anchored run." The adversarial reviewer cites that file's ASSERTION at `:212` as evidence for the finding but not the COMMENT four lines above it, which states the finding's target claim in plainer words. It matters twice: it is the disambiguator that decides the ruling (the observer is named), and it means the proposed one-site deletion would have been incomplete.

2. **THE MESSAGE-LEVEL IDENTITY IS UNIVERSAL, NOT COST (i)'s.** Part 1's table: hold the spelling fixed, vary only the population, and stdout, stderr and exit code are identical at every spelling that misses. The finding frames the identity as a property of accepted cost (i) and derives some of its weight from that pinning; it is a property of the mechanism at large. This strengthens the reviewer's measurement while removing the sub-case defence its verdict might otherwise have rested on, and it is the fact worth carrying forward.

3. **BOTH REMEDY CLAUSES ARE LIVE ON THIS ARM, ONE PER POPULATION, MEASURED.** The adversarial reviewer's coverage gap 7 records the two clauses as "each live on at least one reachable input" but did not run the mis-anchored case end to end. It converts a correct exit 1 into a correct exit 0 over the project's real log (Part 1). Recorded so a later round does not re-derive it, and noting that this is the `Ok` arm's analogue of the settled `R3A-1` question about the `Err` arm.

I found nothing else. I did not conduct an independent adversarial pass of my own, which is not this role's job; everything above came from testing the two reports' claims.

---

# Tally

| Severity | Valid | Findings |
| --- | --- | --- |
| critical | 0 | |
| high | 0 | |
| medium | 0 | |
| low | 0 | |

`R4A-1`: NOT VALID (would have been `low`). Its measurement is accurate, is sharper than reported, and is recorded as a residual observation with no change made.

ROUND 4 HAS ZERO VALID FINDINGS. Nothing is sent back to the implementer and no fix pass is required or wanted; this round's correct product action is none at all.

NO BACKSTOP RE-CHECK IS TRIGGERED. The one dismissal is at `low`, below the backstop severity of `high`.

FOR THE ORCHESTRATOR, stated plainly because the arithmetic is close and must not be inferred from prose: round 4 is CLEAN. I record that I was told the convergence arithmetic before ruling, that a valid finding here would have made convergence before the cap impossible, and that the ruling rests on the constructions in Part 1, which I ran in both directions and one of which corrected my own first attempt against the verdict I reached. Had the sentence been false I would have said so and let the loop escalate.

# Relitigation and constraints check

Nothing above raises or reopens the four standing residuals (the in-root bound, the single-anchor `..` case, the earlier increment's rejected-ledger context slot, the off-convention `--source` surface); accepted costs (i) to (iv), which are treated as PINNED EXPECTED BEHAVIOUR and used only as fixtures and controls; round 1's `ADV-4` or `SC-3`; round 2's `R2A-4`, `R2B-2` or `R2B-3`; round 3's `R3A-1` or `R3A-3`; the queued plain-`validate` inconsistency; the check-16 vacuous pass, which Part 2 excludes by name; or `Q-55-existsgate`'s declined `try_exists()?` gate change. No line-length, prose-wrapping or comment-raggedness observation appears anywhere in this file.

FIXTURE HYGIENE: all fixtures under `<scratch>/tri-r4`, a directory of my own naming; nothing outside it was written or deleted. Every FIFO created for the TOCTOU reproduction was removed; the closing `find <scratch>/tri-r4 -type p`, `-type d ! -perm -u+rwx` and `-type f -perm 000` all return nothing, and no fixture was chmodded to 000 or 600 at any point. `TMPDIR` pointed outside any git repository for `cargo test`. No source edit was made, `git status --porcelain` is empty on this worktree, and the main repository was not touched.
