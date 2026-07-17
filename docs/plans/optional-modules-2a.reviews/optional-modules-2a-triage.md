# Triage: optional-modules sub-increment 2a (`{{modules}}` slot, `guidance`, `requires`)

Triager: independent adjudication of the opus (correctness/fuzzing) and sonnet (design/docs) findings files. Change under review: commit `de70ad0` on `impl/inc2a-modules-slot`, diff range `b565103..de70ad0`. Severity scale: absolute four-level `low`/`medium`/`high`/`critical`, rated by impact if left unfixed.

Deduplication note: sonnet H1 and H2 are two distinct stale-doc issues; sonnet L1 is a third (finer) doc-precision issue of the same class. The missing-guidance-file concern appears across opus finding 1, opus finding 2, sonnet M2, and sonnet L2; I split it into three facets (untested, bare error message, undocumented required-ness) and give one verdict per facet. sonnet M3 and opus's non-defect re-parse note are the same underlying observation, adjudicated once with its two facets (doc claim vs perf refactor).

---

## A. Stale field docs on `AssetSpec.module` and `VarSpec.module` (sonnet H1)

- Verdict: VALID.
- Severity: medium (sonnet rated high; corrected down).
- Evidence: `src/manifest.rs:53-56` (asset) and `src/manifest.rs:86-90` (var) both say a tagged entry activates only "when that module is selected with `--module <name>`". After 2a a module can be enabled transitively via `requires` without being named on `--module`, and `load` correctly gates on `enabled.contains(...)`. The comments now contradict the just-added behavior.
- Reasoning on severity: this is a genuine doc/behavior drift (Principles 19/20) and actively misdescribes a feature added in the same commit, so not low. But it is an internal doc comment: no runtime behavior is wrong, no user-facing output is affected, and the only cost is future-maintainer confusion. That is the medium band on an impact-if-unfixed scale. High is reserved for findings that cause wrong behavior or user-facing harm, which this does not. Correcting both comments to say "enabled (by `--module` or by another module's `requires`)".
- Owner: implementer.
- Round needed: yes (doc-comment edit in `src/`).

## B. Stale `ModuleSpec` struct comment "only names each module" (sonnet H2)

- Verdict: VALID.
- Severity: medium (sonnet rated high; corrected down).
- Evidence: `src/manifest.rs:64-65` says "Membership itself is single-sourced on the assets' `module` tag; this section only names each module and describes it." The section now also carries `guidance` (governs `{{modules}}` rendering) and `requires` (governs the auto-enable graph), so "only names ... and describes" is flatly false.
- Reasoning on severity: same class and same reasoning as A. Real drift, actively misleading about the changed struct, but internal-only impact. Medium, not high.
- Owner: implementer.
- Round needed: yes.

## C. `module_guidance` doc says "trimmed" but code `trim_end`s (sonnet M1)

- Verdict: VALID (as a doc-precision fix, not a behavior change).
- Severity: low.
- Evidence: doc `src/manifest.rs:387` says "Each partial is trimmed"; code `block.push_str(partial.trim_end())`. Leading whitespace/blank lines in a partial are preserved.
- Reasoning: `trim_end` looks intentional (it mirrors `render`'s own `format!("{}\n", out.trim_end())` tail normalisation), so this is a doc wording nit, not a bug: "trimmed" should read "trailing whitespace trimmed". I am not requiring the code switch to `trim()`; leading-content preservation is a defensible choice and no evidence shows it is wrong. Fix the word.
- Owner: implementer.
- Round needed: yes (bundle with the other doc edits).

## D. Missing-guidance-file behavior is untested (opus finding 1, sonnet L2)

- Verdict: VALID.
- Severity: medium.
- Evidence: a module whose `guidance` names an absent file errors and writes nothing (`module_guidance` -> `source.read(guidance)?` at `src/manifest.rs` propagates `LoadError::Io`), verified empirically by opus. No test pins this. Every other error path added in 2a (`UndeclaredModuleRequire`, duplicate, reserved var, cycle) has a dedicated test.
- Reasoning: Principle 11. A missing/misspelled guidance filename is the single likeliest pack-authoring mistake the new field invites, and the error contract (hard fail, not silent empty) is exactly the kind of behavior a future refactor could silently flip to `unwrap_or_default()`. Worth requiring, not merely nice-to-have. A fixture test asserting the load fails and drops nothing pins it. This is the one test gap I require.
- Owner: implementer.
- Round needed: yes.

## E. Missing-guidance-file error names neither file nor module (opus finding 2)

- Verdict: VALID.
- Severity: low.
- Evidence: `PackSource::Directory::read` returns the bare `fs::read_to_string` error (`src/manifest.rs:255`, e.g. `No such file or directory (os error 2)`), while the `Embedded` arm wraps with `pack file not found: {rel}` (`src/manifest.rs:253`). For a directory pack the user sees no filename, no module, no hint it concerns guidance.
- Reasoning: the root cause is pre-existing (the `Directory` arm has always returned a bare error), so this is not a 2a regression, which caps the severity at low. 2a does make it newly relevant because `guidance` is a filename a pack author types by hand. A reasonable fix is either to have `Directory::read` include `rel` in its error (parity with the `Embedded` arm) or to map the read error in `module_guidance` to a variant naming the module and file. Improvement, not a blocker.
- Owner: implementer (optional; could also be deferred as a small standalone tidy).
- Round needed: not strictly required for 2a; fold into the doc round if one runs.

## F. Missing-guidance required-ness undocumented / "like `instrument.md`" misleads (sonnet M2, opus's asymmetry note)

- Verdict: VALID.
- Severity: low.
- Evidence: doc `src/manifest.rs:385` reads guidance "like `instrument.md`", but `instrument.md` is read `.unwrap_or_default()` (`src/main.rs:218`, absent = silent empty) whereas guidance is read with `?` (absent = hard `LoadError::Io`). The asymmetry is deliberate and defensible (an explicitly declared filename should exist; instrument absence is the normal no-instrument case) but is nowhere stated, so a reader could "fix" the inconsistency by swallowing the guidance error.
- Reasoning: the "like `instrument.md`" analogy is really about the read mechanism (inlined, not dropped as its own asset), not about absence-tolerance, so I read it as ambiguous rather than outright wrong, hence low. The fix is a one-line note that a declared guidance file must exist (or clarifying the analogy is about how it is read). This is a doc clarification, distinct from D (the test) and E (the message text).
- Owner: implementer.
- Round needed: yes (bundle with the doc edits).

## G. "one source of truth for the enabled set" claim vs the double compute (sonnet M3, opus non-defect note)

- Verdict: doc facet VALID (low); perf-refactor facet INVALID / out of scope.
- Severity: low.
- Evidence: `build_assets` calls `module_guidance` then `load` (`src/main.rs:224` then `226`); each independently calls `source.manifest()` and `expand_modules`, so the manifest is parsed twice and the enabled set is computed twice. The `expand_modules` doc (`src/manifest.rs:328-330`) says the returned set "drives both ... so those two agree ... (one source of truth for the enabled set)."
- Reasoning: opus and sonnet describe the same fact from two angles. opus is correct that it is not a correctness risk: both calls receive the identical `modules` slice and the same immutable pack, so they cannot disagree short of a mid-load TOCTOU edit; the single-source claim for the enabled SET holds in substance. So the "compute once, pass down" refactor is a performance/tidiness nicety, not a defect, and is out of scope for 2a. sonnet's doc point is the salvageable part: the phrase "one source of truth for the enabled set" could be misread as "computed once and shared" when it means "one algorithm, so the two call sites cannot diverge." A minor reword (e.g. "the single algorithm both paths use, so they cannot disagree on which modules are on") removes the ambiguity. Low, optional.
- Owner: implementer (doc reword only, if a round runs); the perf refactor is not required.
- Round needed: no round on its own account; reword only if bundled.

## H. `requires` doc says "validated in `load`" but validation lives in `expand_modules` (sonnet L1)

- Verdict: VALID.
- Severity: low.
- Evidence: `ModuleSpec.requires` doc `src/manifest.rs:82-84` says "validated in `load`"; the dangling-requires check is in `expand_modules` (which `load` calls). The identical phrasing on `AssetSpec.module`/`VarSpec.module` is accurate there (the tag check is still inline in `load`), but for `requires` it points one level too shallow.
- Reasoning: minor doc-precision nit, same class as A/B/C. "validated in `expand_modules`" or "validated before any asset is read" is precise. Low.
- Owner: implementer.
- Round needed: yes (bundle with the doc edits).

## I. Test gaps beyond the missing-guidance test: diamond, self-require, arg-order, enabled-without-guidance (opus finding 4)

- Verdict: VALID (as coverage observations); NOT required for convergence.
- Severity: low.
- Evidence: opus verified all four empirically; none is pinned by a test. The cycle test proves termination but not diamond dedup; no self-require test; no test that guidance order is independent of `--module` argument order; `requires_auto_enables...` happens to give both modules guidance, so "enabled-via-requires module with no `guidance` contributes nothing" is unexercised.
- Reasoning: these harden determinism/dedup against future `expand_modules` refactors, but the properties are structurally guaranteed (declaration-order iteration + `enabled` set membership), and the cycle test already exercises the fixed-point guard. Unlike facet D (the likeliest real pack error, an untested hard-fail contract), these are nice-to-have. I recommend the implementer add the enabled-without-guidance and diamond cases opportunistically if a round runs anyway, but I do not block on any of them.
- Owner: implementer (optional).
- Round needed: no (do not gate convergence on these).

## J. Internal blank-line run when `--instrument` off and a module contributes guidance (opus finding 3)

- Verdict: VALID observation; out of scope for a required 2a fix.
- Severity: low.
- Evidence: with the tail `{{instrument}}\n\n{{modules}}`, instrument off but a module contributing guidance yields an internal run of blank lines (`HEAD\n\n\n\nA...`); `render` normalises only trailing whitespace (`src/manifest.rs:288`), not internal runs.
- Reasoning: zero impact in 2a: the built-in pack declares no modules, so `{{modules}}` is always empty and the verified byte-identity holds. It can only manifest once a real module with guidance ships (2b). Correctly flagged for 2b; no 2a code path exhibits it, so it is not a 2a defect to fix now.
- Owner: orchestrator to carry into 2b planning (implementer fixes it there).
- Round needed: no (2a).

## K. CHANGELOG entry for `{{modules}}` / `guidance` / `requires` (sonnet L3)

- Verdict: VALID but orchestrator-owned merge-time bookkeeping, not an implementer round.
- Severity: low.
- Evidence: the three features are user-visible pack-authoring additions with no CHANGELOG entry.
- Reasoning: per the workflow model the CHANGELOG is orchestrator-owned, and the plan may write a single entry at the end of increment 2 rather than per sub-increment. sonnet correctly flags it for the orchestrator to confirm intent and not close increment 2 without it.
- Owner: orchestrator.
- Round needed: no (does not spawn an implementer round).

## L. Keep the dedicated `UndeclaredModuleRequire` variant (both reviewers)

- Verdict: AFFIRMED. Keep the dedicated variant; do not fold into `UndeclaredModuleTag`.
- Severity: n/a (design affirmation, no defect).
- Reasoning: `UndeclaredModuleTag` carries `kind: "asset" | "var"` and an `entry` identifier, and its message reads "{kind} `{entry}` is tagged with module `{module}`". A `requires` reference is module-to-module, not a tag from an asset/var; folding it in forces either a wrong "tagged with" message or a `kind: "module"` hack with a misused `entry` field. The dedicated variant's message ("module `X` requires `Y`, which no [[module]] declares") is accurate and the cost is one enum variant. Both reviewers converge; I concur.
- Owner: n/a.
- Round needed: no.

---

## Summary

Distinct valid findings: A, B, C, D, E, F, G (doc facet), H, I, J, K.

Requires an implementer round (doc/test edits in `src/`):

- A, B: stale field/struct doc comments (medium x2, corrected down from high).
- D: missing-guidance-file test (medium) - the one test gap I require.
- C, F, G(doc), H: low doc-precision fixes, bundle into the same round.
- E: low, optional error-message improvement, fold in if the round runs.

Not blocking convergence:

- I: extra test cases (low, nice-to-have; add opportunistically).
- J: 2b cosmetic (low, no 2a code path exhibits it).
- K: CHANGELOG (orchestrator-owned).
- L: design affirmation, no change.

Severity corrections from the reviewers: sonnet H1 and H2 downgraded high -> medium (correct behavior, internal-doc-only impact). No finding is upgraded. No critical/high finding is dismissed as invalid, so the high/critical dismissal backstop does not apply. The highest valid severity this round is medium.
