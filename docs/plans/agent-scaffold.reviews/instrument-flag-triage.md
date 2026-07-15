# Triage verdicts: `instrument-flag` (round 1)

Triager: opus, independent of the implementer and orchestrator. Diff range: `a5264a2..1cd3211`. Judged against `AGENTS.md` Principles 1-22 and the plan's `instrument-flag` step (`docs/plans/agent-scaffold.md` lines 518-524), Pack format section (lines 61-67), and `Q-24`.

Inputs read directly: `instrument-flag-reviewer-opus.md` (L1-L4, all low), `instrument-flag-reviewer-sonnet.md` (F-1, F-2, F-3, all medium).

Own verification (Principle 6, not trusting the reviewers' transcription):

- Built the binary and scaffolded a `--instrument`-OFF run into scratch, then `diff <scratch>/off/AGENTS.md AGENTS.md` -> `114,115d113`: the raw OFF render carries two extra trailing blank lines vs the committed (formatter-cleaned) `AGENTS.md`. `tail -4 | cat -A` shows two trailing `$`-only lines. Byte-identity confirmed to hold only after `nix fmt`.
- Both `render = true` assets (`AGENTS.md`, `.agents/AGENTS.reference.md`) already end with exactly one trailing newline (`tail -c 3 | cat -A` -> `s.` + newline), so the proposed "normalise to one trailing newline" fix is a verified no-op for existing output.

Artifact risk classification: LOW-risk (opt-in flag, additive, no correctness/data/security blast radius; the one behavioural change is inert trailing whitespace). Required consecutive clean rounds to converge: one.

---

## V-1 (dedup of opus L1 + sonnet F-2 + opus L4): raw OFF render is not byte-identical; the claim is overstated and untested

Verdict: VALID. Severity: MEDIUM (correcting sonnet F-2's medium; opus rated the two sub-parts low as L1 and L4, but the composite, an incorrect evidence-grounding claim in the plan plus commit plus no test, is medium).

Reasoning:

- Confirmed by own diff: `pack/AGENTS.md` now ends `{{principles}}\n\n{{instrument}}\n`; with the flag off `{{instrument}}` substitutes to `""` and `render()` does plain `str::replace` with no whitespace normalisation, so the raw tail is `<last principle>\n\n\n`, two blank lines the pre-instrument binary never emitted. The committed dogfood output is byte-identical only because `just scaffold-self` runs `nix fmt` afterwards.
- The functional impact alone is low: trailing whitespace, semantically inert, cleaned by any Markdown formatter, affecting only a downstream user who runs the binary without `--instrument` and without a formatter.
- What lifts this to medium is the evidence claim, not the whitespace. The plan's evidence-grounding (line 524, "with it off, the scaffold output is byte-identical to the non-instrumented run") and the commit message ("empty and byte-identical to today when off", "byte-identical-when-off verified") both assert something false for the tool's own output. The whole workflow is evidence-grounded and self-dogfooding; a plan that records a verified guarantee which does not hold undermines Principle 3 (ground decisions in evidence) and Principle 6 (verify, don't trust). No automated test pins byte-identity (opus L4): `instrument_off_omits_the_block_and_on_includes_it` only asserts heading presence/absence and that no literal `{{instrument}}` remains; it never compares OFF against a pre-instrument baseline, so the regression that produced the trailing blanks is invisible to the suite. The only guard is a human running `just scaffold-self` + `git diff`, which itself depends on the formatter.
- Not high: no correctness, data, money, or security impact; fully reversible; one clean fix converges.

Recommended fix (the clean route, preferred over the wording-only route, per Principle 17 cleaner-long-term-architecture):

1. Normalise each rendered asset to end with exactly one trailing newline before writing (formatter-independent). Verified a no-op for all existing rendered assets (both already end with one newline), so it changes only the pathological empty-substitution tail. This makes the OFF render byte-identical to the pre-instrument output at the raw-tool level, so the plan's existing "byte-identical" wording becomes true rather than needing to be weakened.
2. Add an automated test that pins the invariant: assert the OFF `build_assets` render of `AGENTS.md` equals the pre-instrument baseline (or, more cheaply, contains no run of two-or-more trailing newlines / no trailing blank line). This satisfies Principle 11 (a test must exercise the path it claims to cover) for the byte-identity KEY INVARIANT that today only a manual check covers.
3. Leave the commit message as history (immutable); once the normalisation lands, the plan's line-524 claim is accurate and needs no change. If the implementer instead declines the normalisation and only re-words, then line 524 and the commit-claim intent must be corrected to "the instrumentation section is absent" rather than "byte-identical", but this is the weaker route and leaves the raw non-identity in place.

---

## V-2 (sonnet F-1): Pack format section omits `{{instrument}}` reserved and `instrument.md` as a special fragment

Verdict: VALID. Severity: MEDIUM (confirming sonnet's rating; borderline low/medium, held at medium because this section is the documented reserved-vars/pack contract and a later step builds on it).

Reasoning:

- Confirmed: `docs/plans/agent-scaffold.md` lines 66-67 name only `{{principles}}` as the reserved tool-computed variable and only `principles.toml` as a special pack file. After this commit `src/manifest.rs` line 76 is `RESERVED_VARS = &["principles", "instrument"]` and `instrument.md` is a special render fragment read via `source.read("instrument.md")`, neither documented in the Pack format section.
- A custom-pack author reading this section cannot discover that `{{instrument}}` is reserved (they would hit a `LoadError::ReservedVar` with no documented reason) or that shipping `instrument.md` enables instrumentation. This is the section the plan frames as the pack-format contract, so the omission breaks Principle 16 (one source of truth) and Principle 19/20 (document the constraint, self-contained). The upcoming `state-schema` step also depends on `instrument.md` being the single documented source.
- Held at medium rather than low because it is the contract description, not an incidental comment, and the reserved-var behaviour is only discoverable from code today. Not high: documentation completeness in an internal plan section; the runtime behaviour fails loudly (`ReservedVar`), so there is no correctness risk.

Recommended fix: in lines 66-67, extend the reserved sentence to name both `principles` and `instrument` as reserved tool-computed variables, and add that a pack shipping `instrument.md` supplies the `{{instrument}}` fragment (rendered only under `--instrument`), noting the fragment-not-an-asset pattern (a special pack file read directly, like `principles.toml`, not declared in `pack.toml` and not dropped as an asset).

---

## V-3 (opus L2): `unwrap_or_default()` masks non-NotFound IO errors on the fragment read

Verdict: VALID. Severity: LOW. Disposition: ACCEPT-WITH-RATIONALE (accepted residual risk; does not block convergence).

Reasoning:

- Confirmed: `if instrument { source.read("instrument.md").unwrap_or_default() } else { String::new() }` swallows every `io::Result` error kind, so a present-but-unreadable `instrument.md` (permission denied) or invalid UTF-8 silently yields an empty instrumentation block despite `--instrument` being requested (tension with Principle 12 fail-fast and Principle 15 make-absence-explicit).
- This is a pre-existing codebase-wide convention, identical to `pack_principles` (`Err(_) => Ok(Vec::new())`), not introduced by this step. The common case (genuine absence -> empty) is correct, and for the built-in pack the fragment is embedded so the error path is pathological. Tightening it here (distinguishing `NotFound` -> empty from other errors -> fail) would either diverge from the established `pack_principles` behaviour or require changing that too, which is scope beyond Q-4/5/6 (Principle 8, no silent scope expansion).
- Accept with rationale: consistent with the existing convention, low impact, not a regression. If the shared read-swallowing convention is worth tightening, it is a separate, dedicated change covering both read sites, not a bolt-on to this step. Recorded as accepted residual risk.

Recommended fix: none required for this step. Optional follow-up (out of scope): a dedicated change that has both `pack_principles` and the instrument read distinguish `ErrorKind::NotFound` (-> empty) from other errors (-> fail loudly), applied uniformly.

---

## V-4 (opus L3): records carry no task id or timestamp, limiting cross-task aggregation

Verdict: VALID as a design observation. Severity: LOW. Disposition: DEFER to `state-schema` / `workflow-calibration` (must be carried forward, not dropped).

Reasoning:

- The observation is real: `pack/instrument.md` gives each record only `artifact` as an identifier, while the log "accumulates across tasks" (instrument.md; plan line 522 "a small script aggregates many tasks"). Two tasks that each review an artifact named e.g. `plan` append interleaved records a script cannot segment by task; `consecutive_clean` resets give only a heuristic boundary, and `workflow-calibration`'s key signal (a round after a clean round finding a valid issue on an unchanged artifact) needs rounds reliably grouped within one artifact's loop within a task.
- But this step faithfully implements the decided design. The schema in `instrument.md` matches the plan's decided field list exactly, and `workflow-calibration`'s own "Data to gather" list omits a task key too, so adding a `task` and/or timestamp field now would be silent scope expansion beyond Q-4/5/6 (Principle 8). The schema's owner is the deferred `state-schema` step (Q-24/Q-11), which will validate this schema against `instrument.md` as the single source of truth and is the correct place to decide the task/timestamp question deliberately with the human.
- Forward-compatible: the plan specifies JSONL "tolerant of fields added over time" (line 522), so a `task`/`ts` field can be added later without breaking existing readers. That is why deferral is safe rather than lossy, provided it is not forgotten.
- Not fixed here, but must not be dropped: this is a genuine gap against the stated cross-task-aggregation purpose. Recommend the orchestrator carry it as an explicit input to `state-schema` (and note it against `workflow-calibration`) so the schema owner resolves whether to add a `task` identifier and/or a timestamp. If `state-schema` also omits it, it becomes a real defect there.

Recommended fix: none in `instrument-flag`. Orchestrator action: record this observation as an open input to the `state-schema` step so the per-task grouping question is decided when that schema is finalised.

---

## V-5 (sonnet F-3): Roadmap/Status left at `in progress`

Verdict: INVALID. (Reviewer misread the Documentation Protocol; the status is correct as-is.)

Reasoning:

- The step is mid-review (round 1), not complete. Per `AGENTS.md` phase 4 and the convergence rule, the orchestrator marks a step `complete` in the Roadmap only at convergence (after the review loop closes), not when the implementation commit lands. The `in progress` status was set in `a5264a2`, the first commit of this range, and correctly left unchanged by the implementation commit `1cd3211` (confirmed: `git diff a5264a2..1cd3211 -- docs/plans/agent-scaffold.md` is empty).
- "The implementer keeps the Roadmap current" means current for where the work actually is; for a step still under review the current status is `in progress`, not `complete`. Marking it `complete` now would falsely assert a convergence that has not happened and would pre-empt this very review round. F-3's premise (that the commit "appears to complete this step" and so the Roadmap "diverges from reality") is the misread: the reality is that the step is implemented-but-not-yet-converged, which `in progress` states correctly.
- No plan edit is warranted. The Roadmap is accurate. INVALID.
- Note (backstop): F-3 is medium severity; the high/critical dismissal backstop (`AGENTS.md`) does not apply, so no second-triager re-check is required. Reasoning recorded here regardless for auditability.

---

## Round outcome

Round 1 is NEW-VALID (not clean): the consecutive-clean streak stays at zero.

- V-1 (medium) and V-2 (medium) are valid and need implementer action (the trailing-newline normalisation plus a byte-identity test, and the Pack format documentation).
- V-3 (low) is accepted-with-rationale: no action, does not block convergence.
- V-4 (low) is deferred to `state-schema`: no action in this step, but the orchestrator must carry it forward as an input to that step.
- V-5 is invalid: no action.

No high or critical findings were raised, so none were upheld or dismissed and the dismissal backstop was not triggered.

Next: the implementer addresses V-1 and V-2; the orchestrator spawns a fresh round on the revised change. As a LOW-risk artifact, `instrument-flag` needs one clean round to converge.
