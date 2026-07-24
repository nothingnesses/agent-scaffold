# Q-52 exploration lens 1: prior-art survey (does code earn its keep?)

Explorer lens: prior-art survey, WebSearch-backed (see the note on tooling below). This
document surveys the real literature and tools behind the orchestrator's starting thesis:
that a code artifact's usefulness is not intrinsic but RELATIONAL, and is made objective by
making it FALSIFIABLE, as the direct analog of mutation testing. Mutation asks "if I CHANGE
this, does a test catch it?"; the thesis's usefulness test asks "if I REMOVE this, does
anything observably notice?" (a test fails, the build breaks, a downstream dependency
breaks, a production metric or behavior changes, or a documented requirement or public-API
contract is violated).

The survey verifies each of six prior-art clusters against live sources (author, year, tool,
URL), gives a short "can and cannot prove" for each signal, then collects gaps and newer
work, then lists concrete corrections to the starting thesis.

Tooling note: the harness WebSearch and WebFetch tools were both down for this run (backend
returned "adaptive thinking is not supported on this model" on every call). I verified
citations instead with direct HTTP fetches (curl) against primary sources: GitHub raw
READMEs, arXiv abstract pages and the arXiv Atom API, dblp's publication API, and
research.google. Every citation below with a URL was fetched live during this run. The four
security-venue debloating papers (CHISEL, RAZOR, TRIMMER, piecewise) are not hosted on arXiv
or dblp's open API in a curl-friendly form; their author/venue/year are cited from prior
knowledge and flagged as NOT independently re-fetched this run. Everything else was confirmed
against a live page.

## Cluster 1: static unused-code and reachability detection

What it is: compiler and linter dead-code passes, call-graph reachability, and unused-symbol
or unused-dependency detectors that reason over a closed static world.

Verified sources (all fetched live):

- cargo-machete (Benjamin Bouvier, "bnjbvr"), https://github.com/bnjbvr/cargo-machete and
  writeup https://blog.benj.me/2022/04/27/cargo-machete/ . The README states it detects
  unused Rust dependencies "in a fast (yet imprecise) way" -- it is a source-grep heuristic
  (does the crate name appear in the source), so it self-documents false positives and
  negatives (macros, re-exports).
- cargo-udeps (est31), https://github.com/est31/cargo-udeps . Finds unused dependencies via
  the compiler's own analysis; requires the Rust nightly toolchain to run.
- vulture (Jendrik Seipp), https://github.com/jendrikseipp/vulture . Python dead-code finder;
  assigns a confidence value from 60% to 100%. Its README states the blind spot outright:
  "Due to Python's dynamic nature, static code analyzers like Vulture are likely to miss some
  dead code. Also, code that is only called implicitly may be reported as unused."
- staticcheck / honnef.co/go/tools (Dominik Honnef), https://staticcheck.dev and
  https://github.com/dominikh/go-tools . Go static analyzer; the U1000 class reports unused
  code.
- Knip (Lars Kappert, "webpro"), https://knip.dev and https://github.com/webpro-nl/knip .
  "finds and fixes unused dependencies, exports and files in your JavaScript and TypeScript
  projects."
- Tree-shaking: dead-code elimination in JS bundlers (term popularized by Rollup; also in
  webpack and esbuild). Relies on ES-module static structure and side-effect annotations;
  defeated by dynamic import, eval, and unmarked side effects.
- Compiler and optimizer DCE: rustc dead_code lint, Go's unused check, GCC/Clang -Wunused
  and link-time DCE. These are the deterministic baseline.

Can prove: that a symbol or dependency has NO static reference inside the analyzed closed
world, which makes it a safe-ish deletion CANDIDATE. This is a deterministic, reproducible
check given a fixed compilation closure.

Cannot prove: (a) that a referenced item is USEFUL -- reachability is not value; a symbol can
be reachable and never matter. (b) That an unreferenced item is deletable -- the blind spots
are reflection, FFI and dynamic linking (dlopen), dynamic dispatch and vtables, string-keyed
or serialization-driven dispatch, conditional compilation, and above all the PUBLIC-API
surface: a pub item with no in-repo caller is often the product itself. So static reachability
is necessary-not-sufficient in BOTH directions: reachable does not imply keep, unreachable
does not imply delete.

## Cluster 2: dynamic and production usage

What it is: evidence that code actually ran, from test coverage, from production profiling or
sampling ("dark code" detection), and from feature-usage telemetry that ties a code path to a
real user action.

Verified sources:

- Code coverage as necessary-not-sufficient. The classic caution is Brian Marick, "How to
  Misuse Code Coverage" (1999). Coverage records that code was EXECUTED under a test, not that
  removing it would be noticed; the gap between "executed" and "its removal is caught" is
  exactly what cluster 4 measures.
- "Code Coverage at Google" (Marko Ivankovic, Goran Petrovic, Rene Just, Gordon Fraser),
  ESEC/FSE 2019; verified via https://research.google/pubs/code-coverage-at-google/ . A
  large-scale account of coverage in industrial practice, used as a heuristic with explicit
  cautions that it is not a direct measure of test effectiveness or code value.
- Production coverage / continuous profiling / "dark code": always-on samplers (the lineage
  of Google-Wide Profiling and modern continuous profilers) and distributed tracing identify
  code paths never executed under real traffic; feature-usage telemetry maps a path to actual
  user behavior. This is the industrial form of the deletion oracle "a production metric or
  behavior changes."
- Coverage-Based Debloating for Java Bytecode (Cesar Soto-Valero, Thomas Durieux, Nicolas
  Harrand, Benoit Baudry), arXiv:2008.08401, 2020; the JDBL tool. It removes bytecode not
  covered by a given workload. Its own result is the cautionary datum: after debloating, only
  81.5% of client projects still compile and pass their suites, so about 18.5% BROKE because
  their real usage diverged from the profiling workload. Concrete proof that dynamic-usage
  deletion under-approximates.

Can prove: that code did or did not execute under an observed set of tests or real traffic.
Non-execution over a long, representative production window is strong (not conclusive)
evidence of cruft, and it is a NECESSARY condition for value.

Cannot prove: sufficiency (executed code can still be valueless, e.g. logging nobody reads),
and it systematically MISSES rare-but-critical cold paths -- error handling, disaster
recovery, leap-year and legal-compliance branches -- that are legitimately un-executed in any
finite window. The signal is only as representative as the observation window, and cold does
not imply worthless.

## Cluster 3: debloating research

What it is: automated program-size reduction that keeps only the code needed to satisfy a
supplied specification (a test suite, a workload, or a feature set). This is the closest
existing engineering realization of "minimize the program subject to an observable predicate."

Sources (the four seminal tools are cited from prior knowledge and NOT re-fetched this run,
because they are security-venue papers not on the open APIs I could reach; the evaluation and
newer papers WERE fetched live):

- CHISEL: Kihong Heo, Woosuk Lee, Pardis Pashakhanloo, Mayur Naik, "Effective Program
  Debloating via Reinforcement Learning," ACM CCS 2018. Delta-debugging-style reduction guided
  by reinforcement learning against a user-supplied high-level spec. (Not re-fetched.)
- RAZOR: Chenxiong Qian, Hong Hu, Mansour Alharthi, Pak Ho Chung, Taesoo Kim, Wenke Lee,
  "RAZOR: A Framework for Post-deployment Software Debloating," USENIX Security 2019. Traces
  test-case executions plus heuristic control-flow inference to keep needed code. (Not
  re-fetched.)
- TRIMMER: Hashim Sharif, Muhammad Abubakar, Ashish Gehani, Fareed Zaffar, "TRIMMER:
  Application Specialization for Code Debloating," ASE 2018. Static specialization via
  constant propagation from a usage spec. (Not re-fetched.)
- Piecewise: Anh Quach, Aravind Prakash, Lok Yan, "Debloating Software through Piece-Wise
  Compilation and Loading," USENIX Security 2018. Builds an accurate dependency graph at
  compile time and loads only needed functions. (Not re-fetched.)
- Delta debugging and 1-minimality: Andreas Zeller and Ralf Hildebrandt, "Simplifying and
  Isolating Failure-Inducing Input," IEEE TSE 2002 (the ddmin algorithm); and Zeller,
  "Yesterday, my program worked. Today, it does not. Why?," ESEC/FSE 1999. Verified via dblp:
  ddmin computes a 1-minimal subset with respect to a test PREDICATE. This is the formal core
  of the deletion experiment -- minimize subject to "the predicate still holds." C-Reduce
  (Regehr et al.) applies the same idea to source programs.

Critical newer counter-evidence (fetched live, and the strongest result in this whole survey):

- "A Broad Comparative Evaluation of Software Debloating Tools" (Michael D. Brown, Adam Meily,
  Brian Fairservice, Akshay Sood, Jonathan Dorn, Eric Kilmer, Ronald Eytchison),
  arXiv:2312.13274, 2023. Evaluated 10 debloating tools on 20 benchmarks with a differential
  fuzzer (DIFFER). Findings that "contradict the prevailing narrative": only a 22% success
  rate at producing a passable debloated version of medium- and high-complexity programs, and
  only 13% of debloating attempts produced a sound AND robust program.
- "Revisiting Code Debloating with Ground Truth-based Evaluation" (Muhammad Bilal, Moiz Ali,
  Mohit Kumar, Fareed Zaffar, Fahad Shaon, Ashish Gehani, Sazzadur Rahaman),
  arXiv:2604.17717, 2026 (post-cutoff). Ground-truth evaluation of eight debloaters (Blade,
  Chisel, Cov, CovA, Lmcas, Occam, Razor, Trimmer). Dynamic-analysis tools "often remove up to
  94% of code that should be retained"; static tools show the opposite, high FALSE-RETENTION
  from coarse over-approximation, and can even ADD code via specialized variants.

Can prove: given a FIXED predicate or spec, you can compute a program that is minimal with
respect to it. This is a rigorous operationalization of "earns its keep iff observably needed
by the predicate."

Cannot prove: correctness beyond the predicate. The predicate is the entire ballgame: an
incomplete spec makes the minimizer delete code that IS needed for unobserved behaviors. The
13% to 22% soundness numbers and the 94% over-removal figure quantify precisely how badly the
deletion experiment misfires when the observation set is incomplete. Debloating thus both
VALIDATES the deletion-experiment idea and measures its central danger.

## Cluster 4: mutation testing and its inverse, pseudo-tested methods

What it is: mutation testing perturbs code and asks whether a test catches the change. Its
inverse -- extreme mutation / pseudo-tested methods -- nulls a whole method body and asks
whether ANY test notices. That inverse is the exact analog the thesis wants.

Verified sources:

- Origin: Richard DeMillo, Richard Lipton, Frederick Sayward, "Hints on Test Data Selection:
  Help for the Practicing Programmer," IEEE Computer, 1978.
- Survey: Yue Jia and Mark Harman, "An Analysis and Survey of the Development of Mutation
  Testing," IEEE Trans. Software Eng. 37(5):649-678, 2011, DOI 10.1109/TSE.2010.62 (verified
  via dblp). Tools in current use: PIT/pitest (Java), mutmut and cosmic-ray (Python),
  cargo-mutants (Rust).
- The inverse, pseudo-tested methods: Rainer Niedermayr, Elmar Juergens, Stefan Wagner, "Will
  My Tests Tell Me If I Break This Code?," CSED @ ICSE 2016, DOI 10.1145/2896941.2896944,
  arXiv:1611.07163 (abstract verified live). "Extreme mutation" nulls the entire method body;
  a pseudo-tested method is one that is covered yet no test fails when its effects are
  suppressed. Their result: pseudo-tested methods are systematically present even in
  high-statement-coverage projects, and coverage is a valid effectiveness indicator only for
  unit tests, not system tests.
- Replication and characterization: Oscar Luis Vera-Perez, Benjamin Danglot, Martin
  Monperrus, Benoit Baudry, "A Comprehensive Study of Pseudo-tested Methods," Empirical
  Software Engineering, 2019, arXiv:1807.05030 (abstract verified live), plus the Descartes
  tool, a PITest engine, arXiv:1811.03045. The definition, verbatim from the abstract, IS the
  thesis's deletion experiment at method granularity: pseudo-tested methods are "covered by
  the test suite, yet no test case fails when the method body is removed, i.e., when all the
  effects of this method are suppressed."

Can prove: if suppressing a method's effects changes NO test outcome, the test suite does not
pin that method's value -- its behavior is unconstrained by the current oracle. This is a
deterministic, reproducible experiment.

Cannot prove: that the method is USELESS. It distinguishes "not pinned by the TESTS" from "not
needed by the SYSTEM." A pseudo-tested method can be exercised and valuable in production; the
signal indicts the TEST SUITE's grip, not the code's worth, unless the observation set is
widened past tests to production behavior and API contracts. This is the sharpest lesson for
the thesis (see corrections).

## Cluster 5: behavioral-code-analysis and maintenance-cost hotspots

What it is: mining version-control history to locate where maintenance COST concentrates,
rather than where code is unused.

Verified sources:

- Adam Tornhill / CodeScene: "Your Code as a Crime Scene" (Pragmatic Bookshelf, 2015) and
  "Software Design X-Rays" (Pragmatic Bookshelf, 2018). A hotspot is complexity (or CodeScene's
  Code Health) multiplied by change frequency (churn) from VCS history. Verified live against
  the CodeScene hotspots docs: "Most development activity tends to be located in relatively
  few modules ... Low code health in a development hotspot is expensive. Prioritize
  improvements here. Low code health in stable parts of the codebase ... has lower priority."
- Michael Feathers, "Working Effectively with Legacy Code" (Prentice Hall, 2004), and his
  empirical churn-versus-complexity writing ("Getting Empirical about Refactoring"), plus
  characterization tests as the technique for pinning down behavior before change.

Can prove: WHERE maintenance cost concentrates (high churn crossed with high complexity), i.e.
where cruft HURTS most and where remediation pays back fastest.

Cannot prove: whether a hotspot earns its keep. A hotspot is frequently the MOST valuable,
most-edited core, not cruft. Conversely, the better cruft candidate is the opposite profile:
low churn, low coverage, no references. Hotspots rank REMEDIATION, not REMOVAL; they are the
COST lens, complementary to the VALUE lens of clusters 1 through 4. The thesis models only the
value/necessity side and should be paired with this cost side (see gaps).

## Cluster 6: traceability and provenance

What it is: links from code to the requirement, test, API contract, or caller it exists to
serve. Code that maps to nothing is suspect.

Verified source:

- Orlena Gotel and Anthony Finkelstein, "An Analysis of the Requirements Traceability
  Problem," ICRE 1994, pages 94-101, DOI 10.1109/ICRE.1994.292398 (verified via dblp). The
  foundational traceability paper. Its central finding is important here and cuts AGAINST naive
  use: the dominant problem in practice is MISSING and under-recorded links, not absent value.

Can prove: code with NO recorded link to any requirement, test, contract, or caller is a
suspicion CANDIDATE (a necessary condition for cruft).

Cannot prove: uselessness. Because traceability data is chronically incomplete (Gotel and
Finkelstein's own thesis), "no recorded link" is dominated by missing documentation, not by
absent value. Provenance is a triage prior, never a verdict.

## Gaps and newer work

- The debloating soundness crisis is the strongest empirical caution in the whole field and
  post-dates most informal intuitions about deletion: Brown et al. 2023 (arXiv:2312.13274,
  22% success, 13% sound-and-robust, via the DIFFER differential fuzzer) and the post-cutoff
  Bilal et al. 2026 (arXiv:2604.17717, up to 94% over-removal by dynamic tools, high false
  retention by static tools). Any "remove it and see" design must budget for this failure rate
  and treat "nothing noticed" as a candidate, not a proof.
- Coverage-based debloating (JDBL, Soto-Valero et al. 2020, arXiv:2008.08401) gives the
  concrete under-approximation number: 18.5% of clients broke. It is the cleanest single
  illustration that dynamic-usage deletion is only as complete as the workload.
- The public-API / library problem is under-covered by all six clusters and deserves its own
  treatment. None of the in-repo removal signals see callers outside the repository. The real
  "does anything notice" oracle for a library is semver plus downstream CI at ecosystem scale
  (for Rust, the crater tool that rebuilds the crates.io universe against a change). This is the
  concrete mechanism behind the thesis's own "a downstream dependency breaks" clause, and it is
  an EXTERNAL, higher-latency oracle distinct from the local test/build.
- Semantic-equivalence / differential checking (DIFFER, differential fuzzing, regression
  verification) is the rigorous oracle that should sit BEHIND any deletion experiment: does the
  program with X removed behave identically on all differential inputs? This is a named
  technique missing from the six clusters and is what separates a sound removal from a lucky
  one.
- The economics framing is absent from the thesis but implicit across clusters. "Earns its
  keep" is value MINUS cost. Clusters 1 through 4 estimate the VALUE / necessity side (does
  removal cause an observable loss). Cluster 5 estimates the COST side (churn-weighted
  maintenance burden). A complete objective judgment needs both; the thesis currently models
  only the necessity side and would rank a zero-cost, never-touched, never-called dead function
  the same as a high-churn liability.
- Feature-flag / dead-flag cleanup and continuous profiling are the industrial, deployed forms
  of "production notices removal," and are worth citing as practice even though they lack a
  single canonical paper.
- LLM-assisted dead-code review is emerging but is NON-deterministic, so it fails the thesis's
  own "deterministic" requirement and should be explicitly excluded from the objective core (it
  can at most propose candidates for a deterministic oracle to confirm).

## Corrections to the starting thesis

1. Confirmed and strengthened: "if I REMOVE this, does anything observably notice?" is not just
   analogous to mutation testing, it is a NAMED, published construct -- the pseudo-tested method
   (Niedermayr et al. 2016; Vera-Perez et al. 2019), which nulls a method body and checks
   whether any test fails. Cite that directly; it is a tighter analog than "mutation testing" in
   general, and ddmin (Zeller and Hildebrandt 2002) is the formal "minimize subject to a
   predicate" core. The thesis's central intuition is sound and has real prior art.

2. Sharpen the analogy: mutation testing asks "does a test catch a CHANGE?"; the thesis wants
   "does anything catch a REMOVAL?". These are different. The load-bearing analog is EXTREME
   mutation (body-nulling / effect-suppression), a specific named subset, not mutation testing
   as a whole. State that precisely so the design does not inherit ordinary mutation testing's
   operator zoo, which is not what is wanted.

3. Main correction: the operational definition is objective only RELATIVE TO its observation
   set. "Does anything notice" is not a property of the code; it is a property of the pair
   (code, oracle set). The pseudo-tested-methods literature shows the SAME method reads as
   worthless against a weak suite and valuable against a strong one, and the debloating
   soundness numbers (13% to 22% sound, up to 94% over-removal) quantify how an incomplete
   oracle misfires. So "nothing noticed" is necessary-not-sufficient for deletion: a candidate
   pending a stronger oracle, never a positive proof of uselessness. The design must name its
   oracle set explicitly and report against it, not claim oracle-independent objectivity.

4. Separate in-tree oracles from out-of-tree oracles. The thesis's own list mixes them: test
   fails and build breaks are in-tree, deterministic, and fast; downstream dependency breaks,
   production-metric changes, and public-API-contract violations are OUT-of-tree, higher
   latency, and (for telemetry) statistical. The public-API / library case in particular breaks
   the "remove it, see if anything notices" test inside a single repo, because the noticing
   party is downstream and out-of-tree (crater-style ecosystem CI, semver). Model these two
   oracle tiers separately; they have different determinism, latency, and completeness.

5. Clarify what "deterministic, statistical-evidence-backed" can mean, since those two words
   pull apart. Static reachability is deterministic but UNSOUND for reflection, FFI, and dynamic
   dispatch (cluster 1). Dynamic and production evidence is sound-for-what-ran but INCOMPLETE and
   non-deterministic across observation windows (cluster 2). The honest architecture is a
   COMBINATION: deterministic static candidate-generation, then statistical confidence
   accumulated from test and production observation (the vulture confidence-value model and the
   Google coverage-sampling model), not a single deterministic verdict. Attach the word
   "deterministic" to the CHECK given a fixed oracle set, not to the value judgment itself.

6. Add the cost axis. "Earns its keep" is value minus cost. The thesis defines the value /
   necessity side well but omits maintenance cost, which cluster 5 (Tornhill / CodeScene churn
   x complexity; Feathers) measures directly. Without it, a harmless zero-cost dead function and
   a high-churn liability score identically. Pair the deletion experiment (necessity) with a
   churn-weighted cost signal (burden) to get a defensible "earns its keep" verdict.
