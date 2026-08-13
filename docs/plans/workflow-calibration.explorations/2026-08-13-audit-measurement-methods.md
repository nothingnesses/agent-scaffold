# Measurement methods: nine instruments for judging a process honestly

Rendered view: <https://claude.ai/code/artifact/0917fceb-e517-404f-a7bc-3ff04f7e8357>. Companion audit: `2026-08-13-audit-when-the-loop-turned.md` in this directory.

Prior art compiled for the 2026-08-13 audit, with the assumptions and failure modes of each method, and what each measures in a workflow like this one. Compiled from working knowledge with a mid-2026 cutoff and without checking sources at the time of writing. The ideas are reliable. The attributions are marked for confidence and must be verified before being quoted anywhere they matter.

## Why the obvious approach fails

Counting what a process produces and dividing by what it cost fails for three separate reasons, each with its own literature. Knowing which one is biting is most of the work.

### One. The measure becomes the target

Charles Goodhart observed in 1975 that a statistical regularity used for control collapses once you control for it. Donald Campbell said the same of social indicators at around the same time. The mechanism needs no bad faith: whatever you measure gets optimised, including the parts that were only ever proxies for the thing you cared about.

Count requirements closed and you get more, smaller, easier requirements. Count lines of code and you get verbose code. Count coverage and you get tests that execute lines without asserting anything.

The structural fix is not a better single metric. It is PAIRED OPPOSING METRICS, where improving one at the expense of the real goal necessarily worsens the other.

### Two. There is no oracle

To say a program is correct you need something to compare it against. In testing this is the ORACLE PROBLEM, a genuinely hard open problem rather than an oversight. A full specification is one oracle, and writing one is usually as much work as writing the program.

The productive move is not to give up but to reach for oracles that need no specification. Three are catalogued below as M4, M5 and M6.

### Three. The evaluator is a participant

When the thing being measured produces the measurements, the record is not independent evidence. A ledger written by the actor about its own performance is a source of hypotheses, not facts. It can be sincere and still be wrong, because the writer chooses what to record, when, and in what framing.

The controls are procedural rather than statistical. Fix the criteria before computing them. Pre-register what result would count as failure. Measure from sources the actor could not edit after the fact, which in a git repository means commit timestamps and diffs rather than prose. Have the measurement performed by parties who do not benefit from the answer.

## M1. Paired opposing metrics

IDEA. Pick at least one throughput measure and at least one stability measure and always report them together. The canonical instance is DORA, from `Accelerate` (Forsgren, Humble and Kim), pairing deployment frequency and lead time with change failure rate and time to restore. Shipping faster by shipping worse moves all four, visibly. The SPACE framework (Forsgren et al., ACM Queue, 2021) generalises this and argues that activity counts alone actively mislead.

ASSUMES. That you can observe failure. If defects are never detected the stability half reads perfect and the pair stops working.

FAILS WHEN. The pair is reported as an average or composite score, which destroys the tension that made it trustworthy.

HERE. Steps closed per week is gameable alone. Pair it with reopen rate and with defects found after a step was declared complete.

## M2. Size normalisation

IDEA. Counting requirements means something only if requirements are comparable in size. Lines of code failed as a productivity measure because the unit is not stable across languages, styles or authors. Function points (Albrecht at IBM, later IFPUG and COSMIC) were an attempt to size a requirement by what it does for the user, independently of implementation. Whatever you think of the counting rules, the discipline is right: normalise the unit before counting units.

ASSUMES. That the proxy correlates with real effort or value. Any proxy is contestable and must be stated openly so someone can argue with it.

FAILS WHEN. The proxy is chosen after seeing the data. This is where the honest version of gaming happens, and it happens to careful people.

HERE. Weight completed steps by behaviour-changing source lines, review rounds and elapsed days, and report weighted and unweighted side by side. Where they disagree, the disagreement is the finding.

## M3. Flow efficiency

IDEA. Lean value stream mapping separates PROCESS TIME, when someone is actively working on an item, from LEAD TIME, the total elapsed. The ratio is flow efficiency, commonly reported under 15% for knowledge work. Reinertsen (`The Principles of Product Development Flow`) argues from queueing theory that product development queues are invisible because they hold information rather than physical inventory, so they go unmeasured and unmanaged. Little's law gives the relation: average items in the system equals arrival rate times average time in the system.

ASSUMES. You can tell working time from waiting time, which needs timestamps finer than the waits you are detecting.

FAILS WHEN. Working time is inferred from artifacts. Commit clustering undercounts thinking that produced no commit and overcounts bursts of trivial commits. State the bias direction rather than presenting inferred time as measured.

HERE. Three clocks, never summed: agent working time, human wait time, calendar time. Day-resolution log timestamps cannot separate the first two. Git commit timestamps can, and cover the whole history. Measured 2026-08-13: 24%, above the norm.

## M4. Differential testing

IDEA. Compare the program against another program instead of against a description. Build the previous version and the new one, run both across the same inputs, and enumerate every difference. Each is either an intended change or a defect, and someone must say which. You never had to write down what correct means, only what changed. Related: metamorphic testing, which checks relations between outputs when no single output can be checked, and property-based testing in the QuickCheck tradition (Claessen and Hughes, 2000).

ASSUMES. The old version was right about what you are not deliberately changing.

FAILS WHEN. Both versions share the same wrong assumption. It detects change, not truth.

HERE. Already in use and the strongest verification technique the project applies. Reviewers build a pre-change binary and run both against hand-built fixture trees. It is being done by hand.

## M5. Mutation testing

IDEA. Introduced by DeMillo, Lipton and Sayward in 1978. Introduce small faults one at a time, flip a comparison, drop a condition, invert a boolean, delete a statement, and run the suite against each mutant. A mutant the suite fails to detect demonstrates that some behaviour is unpinned. The mutation score is an objective measure of suite strength owing nothing to anyone's opinion of the tests.

ASSUMES. Small faults stand in for real ones, and that semantically equivalent mutants can be identified and excluded. Equivalent-mutant detection is undecidable in general, so this needs judgement.

FAILS WHEN. Treated as a target rather than a diagnostic. Chasing the score produces tests that kill mutants without asserting anything, which is Goodhart again.

HERE. Being done by hand at high cost, one mutation per build. `cargo-mutants` automates it for Rust and was used in the 2026-08-13 audit to score 144 of 150 viable mutants on `src/workflow.rs` and `src/plan/source.rs`.

## M6. Capture-recapture

IDEA. From ecology, applied to software inspection by Eick, Loader, Vander Wiel and Votta at Bell Labs in the early 1990s. Two reviewers inspect the same artifact independently. The overlap between what they found tells you how thoroughly it was searched, and therefore how much was probably missed.

The intuition: if two reviewers each find twenty defects and nineteen are the same nineteen, they are both finding the easy ones and the artifact is nearly exhausted. If they each find twenty and share two, they are sampling almost independently from a large pool, and the pool must be much bigger than forty.

Lincoln-Petersen makes that precise. With reviewer A finding n1, reviewer B finding n2, and m found by both:

```
N = (n1 * n2) / m

n1 = 20, n2 = 20, m = 19  ->  N = 21   (nearly exhausted)
n1 = 20, n2 = 20, m = 2   ->  N = 200  (barely scratched)
```

Better estimators exist for more than two reviewers and unequal ability: jackknife estimators, and Chao's estimators for heterogeneous detection.

ASSUMES. Reviewers work independently and every defect is roughly equally likely to be found. Both are routinely violated.

FAILS WHEN. Reviewers are given different lenses on purpose. Deliberate diversity breaks independence and homogeneity together and INFLATES the estimate, because low overlap then reflects assigned scope rather than a large hidden population.

HERE. CORRECTED AFTER MEASUREMENT. The initial claim in this document was that the data had been collected and never used. That is false. The log records aggregate per-reviewer counts and never which reviewer found which finding, so no capture history exists. The assumptions also fail: setting the population to the observed count, which maximises expected overlap, predicts 83 shared findings against 33 observed, so the reviewers are 2.5 times more disjoint than independence permits even if nothing escaped. A Chao lower bound came out above the total findings the project has ever recorded, which is the model announcing it does not fit. Applying this method here needs a schema change first: record a capture history per finding.

## M7. Inspection yield and diminishing returns

IDEA. Michael Fagan formalised software inspections at IBM in 1976 and the literature that followed is unusually consistent. Two findings transfer: reviewer yield falls sharply after roughly the third independent reviewer, and re-inspection recovers progressively fewer defects each round.

ASSUMES. Reviewers of comparable skill on a STABLE artifact. If the artifact changes between rounds, later rounds are inspecting something new and the decay curve does not apply.

FAILS WHEN. Each round's fix pass rewrites the artifact. Then you are not re-inspecting, and yield can stay flat indefinitely. See M9.

HERE. DID NOT REPRODUCE, and the reason is instructive. Per-pass yield measured flat from two reviewers to three (2.35 then 2.33), with 38% of a three-reviewer round's findings outside what the best single reviewer found. The classical studies gave every reviewer the SAME brief, so they resampled one population. This project partitions the space by lens, so the third reviewer opens new territory. The diminishing-returns curve is a property of the review design, not a law. A project that partitions lenses must find its own knee.

## M8. Defect containment

IDEA. For each stage that could detect a defect, count defects found there and defects that escaped to a later stage. The share caught is that stage's containment effectiveness. It attributes value to specific activities rather than to the process as a whole, which is what lets you delete the parts earning nothing.

ASSUMES. Escapes are eventually observed. Undetected defects look identical to absent defects.

FAILS WHEN. The product has no users. With no field usage the only visible escapes are those a later internal stage caught, which systematically understates the miss rate.

HERE. Attribute every real defect to its finder: reviewer, triager, deterministic check, or implementer self-check. Measured 2026-08-13: reviewers 7 to 8, dogfooding and audits 3, deterministic gates 2, triager 1, implementer 1, across roughly 14 identifiable genuine catches.

## M9. Rework defect injection

IDEA. A fix is a change, and changes introduce defects at some rate. The consequence for an iterative review loop is structural: if each fix pass reliably introduces at least one new finding, a rule requiring consecutive defect-free rounds can never be satisfied. The loop terminates by exhaustion or by decree, not by convergence.

ASSUMES. You can tell a new defect from a pre-existing one, which needs the diff and the finding's citation checked against each other.

FAILS WHEN. Findings are counted without asking who wrote the text they are against. The rate is invisible unless you look for it deliberately.

HERE. This is the degeneracy metric proper, and it is the highest-leverage variable the audit found. Measured 49% strict and 61% broad across 189 findings. Crucially it is controlled by what the fix pass DOES: deletion-only re-seeds at close to zero, authored prose at close to 100%.

## Composing them without cheating

FIX THE CRITERIA BEFORE COMPUTING THEM. Choosing metrics after seeing the data is how sincere people produce flattering conclusions.

PRE-REGISTER THE FAILURE CONDITION. State in advance what result would count as failure, specifically enough that it could actually occur. A criterion that cannot come out badly is not a criterion.

SEPARATE THE QUESTIONS THAT CAN DISAGREE. A method can be sound and badly executed. A product can be good and expensive. A process can be wasteful and still be the reason the product is correct. Averaging destroys the information.

THE SINGLE MOST USEFUL HABIT. For every claim you are about to make about the process, ask which primary artifact would falsify it, and go and look. Prose written about the work by whoever did the work is evidence about beliefs, not about events.

## Confidence

| Claim | Confidence | Note |
| --- | --- | --- |
| M1 DORA, SPACE | Firm | Well documented and widely replicated. |
| M2 Function points | Check | Albrecht and the 1979 date are from memory. |
| M3 Flow, Little's law | Firm | The under-15% figure is a common report, not a constant. |
| M4 Differential, metamorphic | Firm | Standard technique, no controversy. |
| M5 Mutation testing | Firm | DeMillo, Lipton and Sayward 1978 is well established. |
| M6 Capture-recapture | Check | The Bell Labs attribution and dating are from memory. The mathematics is standard. |
| M7 Fagan inspections | Firm | 1976 IBM Systems Journal. The exact reviewer optimum varies by study. |
| M8 Defect containment | Firm | Long-standing practice, many variant names. |
| M9 Rework injection | Check | The phenomenon is well attested. Published rates vary widely; quote none as canonical. |
| Agent workflow evaluation | Thin | Fast-moving and sparse. The transferable idea is held-out acceptance criteria written before the work and hidden from the implementer. |

## Sources worth going to directly

- Forsgren, Humble and Kim, `Accelerate`, for the paired-metric design and the research method behind it.
- Forsgren et al., the SPACE framework, ACM Queue, for why single-dimension measurement misleads.
- Reinertsen, `The Principles of Product Development Flow`, for queues, batch size and the economics of delay.
- Fagan, design and code inspections, IBM Systems Journal 1976, for the original inspection process and its data.
- DeMillo, Lipton and Sayward, hints on test data selection, 1978, for mutation testing's origin.
- Eick, Loader, Vander Wiel and Votta, on estimating remaining defects from inspection data.
- Claessen and Hughes, QuickCheck, 2000, for property-based testing as a spec-free oracle.
- Any current survey of the test oracle problem, for the framing above.
