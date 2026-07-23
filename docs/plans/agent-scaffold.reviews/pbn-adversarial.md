# PBN adversarial review: phase -> principle-names map projection

Reviewer lens: adversarial / correctness. Target: `git diff main HEAD -- src/next.rs`.
Change under review: the escalate-only principle projection generalised to a
`phase_principle_names(state) -> &[&str]` map, projected by name via
`projected_principle_reminders(state, principles) -> Vec<String>`, extended
unconditionally into `principle_reminders` at the call site.

## Verdict: ZERO FINDINGS

The change is a faithful, behaviour-preserving generalisation. Escalate output is
byte-unchanged; the only new behaviour (ReadyToPlan now projects the grounding
principle) is intended and correctly ordered; the plural map path works; no
principle number was reintroduced.

## Adversarial checks performed and results

1. Escalate byte-unchanged (regression check). PASS.
   - `phase_principle_names(Escalate)` returns `&["Ground decisions in evidence"]`,
     exactly the value of the deleted `ESCALATE_PRINCIPLE_NAME` const.
   - The emit format string is byte-identical to the old
     `projected_principle_reminder`: `Ground the recommendation in the Project
     Principle "{name}" (plan principle {n}): {text}`.
   - Ordering unchanged: old code pushed the projected reminder, then (if writer)
     the isolation fragment; new code `extend`s the projected Vec, then (if writer)
     the isolation fragment. Same sequence. Escalate does not spawn a writer, so no
     isolation fragment either way.
   - `the_escalate_reminder_projects_a_real_plan_principle_by_name` and
     `the_escalate_reminder_degrades_when_the_principle_is_absent` still pass
     (renamed const to a test-local literal of the same string; no assertion weakened).

2. New ReadyToPlan projection: correct, no doubling/misorder. PASS.
   - ReadyToPlan maps to `&["Ground decisions in evidence"]`; before the diff it
     projected nothing (old guard was `if state == LoopState::Escalate`). This is the
     documented D3 generalisation, not an accidental regression.
   - ReadyToPlan `spawns_writer() == true`, so the full order is base(2 reminders)
     -> projected principle(1) -> isolation fragment(1). Verified the invariant
     base -> principles -> isolation holds.
   - No content doubling: the projected line ("Ground the recommendation ... plan
     principle N: ...") is distinct wording from both base reminders ("Raise and
     resolve the open questions ..." / "Give a recommendation with reasoning ...").
     It is not a byte-duplicate of anything already in the vector.
   - `the_ready_to_plan_reminder_projects_the_grounding_principle_by_name` passes.

3. Unmapped states emit no principle reminder; pinned snapshots byte-unchanged. PASS.
   - All non-{ReadyToPlan,Escalate} states map to `&[]` -> `filter_map` over an empty
     slice -> empty Vec -> nothing extended. Behaviour identical to before (those
     states never emitted a projected principle).
   - Golden snapshots (`golden_human_text` / `golden_json`) are at state
     `awaiting-reviewers` AND built with `principles: &[]`, so they are doubly immune
     (unmapped state and no principles). Both golden byte-compare tests pass; the
     `GOLDEN_HUMAN` / `GOLDEN_JSON` literals are unchanged in the diff.
   - New test `an_unmapped_state_projects_no_principle` (awaiting-fixes, plan DOES
     carry the grounding principle) confirms no leakage into unmapped states.

4. List-valued (plural) correctness. PASS (verified empirically).
   - `projected_principle_reminders` is `phase_principle_names(state).iter()
     .filter_map(find-by-name).map(format).collect()`: it iterates the slice in order,
     skips names absent from the plan, and emits one line per present name.
   - Empirical test: temporarily mapped ReadyToPlan to
     `["Absent principle name", "Ground decisions in evidence", "Minimal by default"]`
     with a plan carrying only the latter two, added a throwaway test, ran it, then
     reverted the edit (working tree restored to HEAD, verified with `git diff HEAD`).
     Result: exactly 2 projected lines, in slice order (grounding first, "Minimal by
     default" second), the absent name skipped, and no dangling "Absent principle
     name" text anywhere in the reminders. Plural path confirmed, not just the
     1-element case.

5. By-name robustness; no reintroduced hardcoded number. PASS.
   - Emit uses `principle.n` (the plan's own locator) only; no literal AGENTS.md
     principle number appears in output. Lookup is `principle.name == *name` (exact
     name match).
   - A mapped name not present in the plan is dropped by `filter_map` (see check 4:
     "Absent principle name" was skipped, never emitted as a dangling reference).
   - The Markdown substrate (no `[[principle]]`) yields an empty `principles` slice
     -> empty Vec, the intended degrade. Confirmed by the degrade test.

6. Design quality: is ReadyToPlan -> "Ground decisions in evidence" a fit or noise?
   Judged a genuine fit, not a finding.
   - The planner's base reminders already charge it to give a recommendation with
     reasoning and resolve open questions before implementing; grounding that
     recommendation in evidence is the same duty stated as the actual Project
     Principle with its plan locator and live text. Different wording from the base
     reminders (so it adds information, the real principle name/text/number, rather
     than repeating), and the map is documented as populated conservatively. Low
     value-add is possible but it is on-theme and drift-free (projected live from
     plan.toml), so I do not consider it clutter worth flagging.

7. Test suite. PASS. See command line below.

## Attacks tried that found nothing

- Looked for a format/wording/order drift at escalate: none; format string and
  sequence are byte-identical.
- Looked for double-emission at ReadyToPlan (base reminder duplicating the projected
  principle): none; distinct strings.
- Checked whether any pinned golden snapshot silently changed: no; goldens are at an
  unmapped state with empty principles and their literals are untouched.
- Probed the plural path for order loss or absent-name dangling reference: neither;
  order preserved, absent skipped.
- Checked for a reintroduced hardcoded principle number in emitted text: none; only
  the plan's `n` locator is used.
- Checked the Markdown / no-principles degrade: empty Vec, no dangling number.

## Test evidence

`cargo test`: 348 passed; 0 failed (lib/bin) plus all integration binaries green
(checks_staged 1/1, scaffold_precommit_hook 3/3, validate_toml_primary 1/1,
validate_workflow_toml_source 2/2, and the rest).
