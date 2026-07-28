# Plan review round 3, triage: `checks-runner-worktree-name-collision` (deferred step, order 93)

Adjudicating `docs/plans/agent-scaffold.reviews/checks-runner-worktree-name-collision-r3-reviewer.md`.

Triaged in worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage3-testiso`, detached at `4067c50`, independent of the planner that wrote the fix and of the orchestrator driving the loop. Artifact: `git diff HEAD~1..HEAD` (the round-2 fix commit `4067c50`) and the whole fold `git diff 0ad43f0..4067c50`.

Standard applied, same as both prior triages: this is a durable RECORD and a future BRIEF for an implementer months out with none of this loop's context, so accuracy of the stated facts weighs more than transient prose would. The deferral is not re-litigated and no verdict below rests on the fix not being implemented. I did not re-open the settled (b)/(c) "implemented correctly" wording, the channel-D question, or the step title.

## Verdict summary

| Finding | Reviewer severity | Verdict | My severity | Reproduced |
| --- | --- | --- | --- | --- |
| `T3-1` (`:92` "it is named above" is false and circular) | `low` | VALID | `low` (confirmed) | Yes, in full |
| `T3-2` (`:62`'s "doc comment" excludes `src/checks.rs:789-790`) | `low` | VALID | `low` (confirmed) | Yes, in full |

Count to fix: **2**, both single-token repairs, in two lines 30 lines apart. No `medium`, no `high`, no `critical`. Nothing dismissed, so the high/critical backstop re-check does not apply to this round.

I also confirm the reviewer's three CLOSED calls on the round-2 findings (`T2-1`, `T2-2`, `TR2-1`), having re-run the load-bearing checks myself rather than accepting them; see "Independent re-verification" below.

## `T3-1`: VALID (`low`). Reproduced in full.

**The claim reproduces.** Sidecar `docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md:92` reads:

> DOCUMENTATION IMPACT: in-code only, and it is named above.

My own grep for the four cited ranges over the sidecar returns exactly two hits, matching the reviewer's:

```
$ grep -n "72-77\|400-402\|845-847\|789-790" docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md
22:Its own doc comment (`src/checks.rs:845-847`) states the premise that fails: ...
94:The change must correct four comments in `src/checks.rs`: `:72-77` ... `:400-402` ... `:845-847` ... `:789-790` ...
```

So of the four comments the documentation impact consists of, three (`:72-77`, `:400-402`, `:789-790`) appear nowhere above line 92, and the fourth appears at line 22 as evidence of the failing mechanism, not as documentation the change must correct. The impact is named at line 94, BELOW the header that claims it is named above. `:62` sends the reader DOWN to that section and the section's own opener sends the reader back UP, so the pair is also circular.

**It is fix-induced, and I checked the provenance rather than accepting the reviewer's word.** At `f18905e` (the step as first written) the done-conditions bullet at line 58 read "The three doc comments that spell the name format literally are corrected in the same change: `src/checks.rs:72-77` ..., `:400-402` ..., `:845-847` ...", sitting above the same header at line 87. "Named above" resolved then, for three of the eventual four. The round-1 fix added the fourth to the paragraph only, half-falsifying it; the round-2 fix replaced the bullet's enumeration with a delegation, which removed the last thing "above" pointed at. Confirmed by `git show f18905e:...` and by `git diff HEAD~1..HEAD`.

**Severity `low`, confirmed rather than raised.** The paragraph directly beneath the header names all four with their grounds and does so in the imperative, so no implementer does the wrong work: worst case is a few seconds looking upward and finding nothing. It is not `medium`, because the correct information is two lines away and nothing else in the record contradicts it.

**Why it is nonetheless valid rather than a residual to wave through.** The record's value is being trustworthy to a reader months out who has none of this reasoning, and its cheap checkable claims are the ones such a reader tests first. A one-clause navigational assertion that fails on inspection costs trust in the expensive claims around it (the measured clock numbers, the 200-collision split, the demonstration arithmetic), which are the parts a reader cannot cheaply re-derive. That is the same ground rounds 1 and 2 both used on the grep claim, and it applies here.

**What the fix must achieve.** Line 92 must stop asserting that the documentation impact is named above it. Two forms work; I recommend the first:

1. Delete the clause: `DOCUMENTATION IMPACT: in-code only.` This removes a directional cross-reference from the document rather than re-pointing it, so there is nothing left at that line for a later edit to falsify.
2. Change `above` to `below`. Correct, but keeps a pointer that any future re-ordering can break again.

## `T3-2`: VALID (`low`). Reproduced in full.

**The claim reproduces.** Sidecar `:62`: "Every **doc comment** the documentation-impact section below names is corrected in the same change." Sidecar `:94`: "The change must correct four **comments** in `src/checks.rs`". I read all four ranges in the source:

- `src/checks.rs:72` `/// The file-name prefix of a runner's temporary worktree directory under the` -> `///`, a doc comment.
- `src/checks.rs:400` `/// Parse the owning pid out of a runner worktree directory name of the form` -> `///`, a doc comment.
- `src/checks.rs:845` `/// Nanoseconds since the epoch, for a unique temp path. Falls back to a fixed` -> `///`, a doc comment.
- `src/checks.rs:789` `// A unique temp path OUTSIDE the repository; git worktree add creates it. The` -> `//`, an ordinary line comment inside the body of `run()`, not a doc comment.

"Doc comment" is a term of art in Rust and this document uses it precisely: `:22` correctly calls `src/checks.rs:845-847` a doc comment, and the round-2 fix deliberately changed `:94`'s noun from "doc comments" to "comments" because one of the four is not one. So the restrictive noun at `:62` was left behind by the same edit that corrected it at `:94`, which puts this squarely in the class this round was asked to hunt. Under the strict reading the bullet requires three of the four and drops `src/checks.rs:789-790`, the exact comment `T2-2` was raised to bring inside the acceptance bar.

**Severity `low`, confirmed.** The practical risk that `:789-790` actually rots is near zero: it sits two lines above the `format!` at `:791-792` and is inside any shared-generator diff regardless. The defect is an internal inconsistency about what work is required, not a likelihood of stale code. That matches the round-2 triage's own grounds for rating `T2-2` `low`.

**The reviewer's mitigation is right in substance, with one correction.** The reviewer offered two independent routes to `:789-790` and I agree only one of them is firm:

- `:94`'s own sentence is imperative and explicit ("The change must correct four comments ... and `:789-790`, the comment on the naming site itself"). This is unambiguous and binding wherever the reader enters. Firm.
- Done-condition `:59` ("the argument for why is written in the code comment, since the current comment is precisely where the wrong argument was written down"). The reviewer states this "independently reaches `:789-790`". It is less determinate than that: "the current comment" is ambiguous between `:789-790` (the comment on the construction site) and `:845-847` (the comment the sidecar quotes at `:22` as stating the premise that fails). Both carry a wrong argument. The natural reading does favour `:789-790`, since `:59` is about the path's construction, but this is a supporting route, not a second independent guarantee.

That correction does not change the verdict or the severity. The acceptance bar taken whole is still not under-inclusive, because `:94` alone carries it.

**What the fix must achieve.** `:62`'s noun must not be narrower than the membership of the section it delegates to. One word: "Every comment the documentation-impact section below names is corrected in the same change."

## Independent re-verification of the round-2 closures

I did not take the reviewer's CLOSED calls on trust, since the whole value of round 3 is fix verification.

**`T2-1` (the grep claim): CLOSED, and demonstrably durable.** My whole-tree grep at `4067c50` returns `src/checks.rs` as the only non-`docs/` hit; `README.md`, `CHANGELOG.md`, `pack/`, `.agents/` and `AGENTS.md` all exit 1. Every `docs/` hit is under `docs/plans/` and belongs to this plan, including `prompt-drift-guard-r2-triage.md:198`, which is the original sighting this step quotes at `:34` and so is both this plan's material and a record of this defect. The invariant form the round-2 verdict required is doing real work: my grep returns EIGHT `docs/` hits where the reviewer's returned seven, because the r3 reviewer's own findings file joined the population between the two runs, and the sentence is still true. Any enumeration or count would already have gone stale in the hours between those two runs.

**`T2-2` (three versus four): CLOSED on the substance.** There is now exactly one enumeration of the required work in the document and no second count to drift against it. The two clause-level residues are `T3-1` and `T3-2` above.

**`TR2-1` ("proportional minimum"): CLOSED.** `:85` now reads "the requirements above are the proportional minimum", which reaches `:82`, `:83` and `:84`, including the red-before-green step the previous wording dropped. I agree with the reviewer's decision not to raise `:86` being outside that reach: `:86` is a reporting duty rather than demonstration machinery, and raising it would be manufacturing a finding.

**The past-the-verdict edit at `:94`: JUSTIFIED.** The old clause's category claim ("the three doc comments that spell the name format literally") was false of `src/checks.rs:845-847`, which states the uniqueness premise without spelling the format. I read all three source ranges and confirm this. The edit corrected a false statement rather than adding explanatory surplus, so it is not an instance of the round-1/round-2 pattern. It did, however, produce both of this round's findings, which is the relevant fact for the convergence read below.

**Load-bearing code claims, re-run.** `grep -cE "\brun\(" src/checks.rs` returns 23, the definition at `:734` plus the 22 call sites `:50` claims. `grep -n 'format!("{RUNNER_PREFIX}' src/checks.rs` returns exactly four sites, `:792` (production, `std::process::id()`), `:1462` and `:1492` (`dead_pid()`), `:1491` (`std::process::id()`), which matches `:51`'s claim that exactly one fixture carries the live pid and that the constant-pid pair carries no cross-process discriminator. `src/checks.rs:1438-1442` confirms `dead_pid()` is `u32::MAX` behind a `!pid_is_alive` assertion, so `:53`'s design constraint is accurate. `render docs/plans/agent-scaffold.plan.toml --check --strict` exits 0 with "up to date", so the rendered view carries the same text as the sidecar and no fix landed in only one of the two.

## Convergence read (advisory, for the human)

The orchestrator owns convergence; this section is input to that decision, not a ruling.

**The finding rate is falling, and the CLASS is collapsing, which matters more than the count.**

| Round | Findings | Classes present |
| --- | --- | --- |
| 1 | 5 | One `medium` factual error about the code (`TI-1`, inverted which fixtures have cross-process protection); two design errors in the prescribed demonstration (`TI-2`, `TI-3`); one tree-fact claim that did not reproduce (`TI-4`); plus the triager-raised item. |
| 2 | 3 | Zero design errors; one re-seed of the same tree-fact claim (`T2-1`); two cross-reference defects (`T2-2`, `TR2-1`). |
| 3 | 2 | Zero design errors, zero factual errors about the code or the tree; two cross-reference defects (`T3-1`, `T3-2`). |

Round 1 found things that would have made an implementer build the wrong thing. Round 3 found nothing of the kind. Every round-3 finding is one class: a claim in one place about the membership or location of material in another place.

**That class has a finite, enumerable population in this document, and I enumerated it.** Grepping the sidecar for every internal cross-reference gives nine:

| Line | Reference | Status |
| --- | --- | --- |
| `:42` | "(see the scope section)" | Sound. Direction-free, no membership claim. |
| `:62` | "the documentation-impact section below names" | Direction correct; the restrictive noun is `T3-2`. |
| `:63` | "(see the demonstration section ...)" | Sound. Direction-free, no membership claim. |
| `:67` | "the (a) + (d) composition below" | Sound. The composition is at `:72`. |
| `:76` | "for the reason below" | Sound. The arithmetic is at `:78`. |
| `:83` | "the sites the scope section enumerates" | Sound. An instruction to reconcile, not a claim about membership. |
| `:85` | "the linkage command above" / "the requirements above" | Sound. `:83`, and `:82-:84`. |
| `:90` | "the demonstration above" | Sound. `:84`. |
| `:92` | "it is named above" | FALSE. `T3-1`. |

Seven of nine are sound. The two that are not are exactly this round's findings. There is no third instance waiting to be found.

**So: is another pass likely to close this, or is this a floor? I judge it likely to close, and the reason is structural rather than optimistic.**

Rounds 1 and 2 re-seeded because each fix REWROTE a sentence while leaving a pointer to it standing somewhere else. Round 1's fix added the fourth comment to the paragraph and left the bullet's count of three, which became `T2-2`, and half-falsified `:92`. Round 2's fix rewrote the bullet to delegate, which fully falsified `:92` (`T3-1`) and carried a narrower noun than the target (`T3-2`). In both cases the edit preserved or moved a synchronisation point.

Both round-3 repairs are DELETIONS of synchronisation points, not rewrites. Deleting "and it is named above" removes a directional claim, and it cannot be re-falsified because it no longer claims anything. Deleting "doc " makes the bullet's set identical to the section's set, so it is satisfied by any membership the section ever has. After both, the count of fragile cross-references in the document is zero, and no new claim has been introduced for a round-4 reviewer to check. That is a checkable prediction, not a hope.

**The one thing that would re-seed it is the failure mode both prior passes exhibited: an implementer improving the surrounding prose while it is open.** Round 2's implementer did exactly that (the `:94` rewrite past the verdict's letter), and that edit produced both of this round's findings. It was a justified edit, which is precisely why it is a hazard: the pattern is not carelessness, it is a competent implementer noticing a real imperfection in adjacent text.

**Concrete recommendation: one tightly-scoped fix pass, with the residual pre-accepted at round 4.**

1. Fix both, as exactly two deletions and nothing else: strike "and it is named above" at `:92`; strike "doc " at `:62`. Then re-render. The constraint is mechanically checkable before the round-4 review opens: `git diff --stat` must show the sidecar and the regenerated `docs/plans/agent-scaffold.md` and nothing else, and the sidecar diff must be two hunks, each a pure deletion within one line. Any other word changed is out of scope and should be reverted, whatever its merit, because improving adjacent prose is the documented re-seed mechanism here and the improvement can be made when the step is picked up.
2. Tell the round-4 reviewer the diff is two deletions, so the round is cheap.
3. Decide NOW, before round 4 runs, that anything it raises below `medium` on this artifact is accepted as a residual and recorded, per the Convergence rule's clause that a valid finding may be resolved by consciously accepting its residual risk and that an accepted risk does not block convergence. This bounds the total cost at one more round while still getting a record with no false statement in it.

Judged against the plan's own principles: this is the case where Principle 1 (prefer the cleaner long-term architecture over the smallest diff) and the smallest diff AGREE, because the cleaner form of a durable record is the one with fewer cross-references to keep synchronised, and that form is also two deletions. There is no trade-off to make. Principle 6 (ground decisions in evidence) is served by the enumeration above rather than by another sampling round: the population of this defect class is nine, seven are sound, two are the findings, and a fourth round is not needed to discover that.

**If instead you accept both as residuals and converge at round 3, is the record correct enough for its actual purpose?** Mostly yes, and the two findings are not equal here:

- Accepting `T3-2` is clean. `:62` would be narrower than `:94`, but it is not false, and `:94`'s imperative names all four comments with their grounds. An implementer following the record does the right work. Nothing in the document contradicts anything else. If only one is fixed, this is the one to accept.
- Accepting `T3-1` leaves one sentence in the record that is false. The work requirements stay correct and complete, so the implementer still does the right thing, but a reader who checks the claim finds it does not hold, in a document whose value rests on its expensive claims being trustworthy.

For a DEFERRED step whose sidecar is re-read at pickup, that is survivable: the reader who trips on `:92` is the same reader who can strike five words. So accepting is defensible and in-rule, and I would not call it wrong. I recommend against it only because the repair is two deletions, the loop has two rounds of headroom before the cap, and a record that says something false is a worse artifact to hand forward than one that costs a round to finish. If the cap were close, or if the repairs required re-derived prose rather than deletions, I would recommend accepting instead.

## What the reviewer missed or overstated

Nothing missed that rises to a finding. Two notes:

1. **Overstated:** `:59` as an independent second route to `src/checks.rs:789-790` (argued under `T3-2` above). "The current comment" is ambiguous between `:789-790` and `:845-847`. `T3-2`'s mitigation rests on `:94`'s imperative alone, which is enough, but the reviewer presented two guarantees where there is one plus a hint.
2. **Not raised, and correctly so, but recorded here so it is not re-found:** `:94` describes `src/checks.rs:789-790` as asserting "the same false uniqueness" as `:845-847`. Strictly, `:789-790` asserts uniqueness ("A unique temp path OUTSIDE the repository") but attributes the embedded pid to prune recognition rather than to uniqueness, so the two comments assert the false uniqueness by slightly different routes. The quotation at `:94` is verbatim-accurate and the comment does need correcting, so this is a shade of wording with no consequence for the implementer. Raising it would be manufacturing a finding, and a future round should not.

I also confirm the reviewer's five "deliberately not raised" items stay settled: I found no new evidence that any of their verdicts was wrong.

## Tree state

`git status --porcelain` in this worktree reports only untracked files under `docs/plans/agent-scaffold.reviews/` (the round-3 reviewer's findings file, copied in, and this file). No plan file, no sidecar, and no source file was edited; nothing was committed; no formatter was run. `render --check --strict` and the greps above are read-only. No probe was needed: every claim in this round is settled by a `file:line` citation or a re-runnable command, per `Q-66` proportionality, so no contrived test was written and nothing was created outside the repository beyond cargo's own `target/`.
