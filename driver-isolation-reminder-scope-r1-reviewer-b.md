5 FINDINGS

---

## B1

**Severity:** medium
**Location:** `src/next.rs:64`

**Description:** The emitted string value of `TIER_RESOLVE_NOTE` says "before spawning the writer" but after Q-62 this text fires at reviewer-spawn states (`AwaitingFirstReview`, `AwaitingReviewers`) as well as writer states.

**Evidence:** At line 849, when `isolation_tier == "unknown"` the lead is formatted as:
```
format!("Agent isolation (tier not yet resolved). {TIER_RESOLVE_NOTE}")
```
The lead correctly says "Agent isolation" (updated by this change) but `TIER_RESOLVE_NOTE` immediately appends "Resolve the isolation tier per the AGENTS.md tier policy before spawning the writer." An orchestrator at a reviewer-spawn state reads a reminder that says "Agent isolation" in one clause and "before spawning the writer" in the next, naming the wrong role. The two clauses contradict each other within the same emitted string.

**Why valid:** The value at line 64 was not updated alongside the lead wording and the predicate rename; only the surrounding code and doc-comments were updated. The actual output the tool produces to the operator is now internally inconsistent at reviewer-spawn states when the tier is unknown.

**Suggested direction:** Replace "before spawning the writer" with "before spawning the agent" (or "the isolated agent") in the `TIER_RESOLVE_NOTE` string, matching the generalization applied to the lead text and to the predicate name.

**Principle:** P1 (prefer the cleaner long-term architecture) - the update was partial; the shared const's value was left behind while its call sites were updated.

---

## B2

**Severity:** low
**Location:** `src/next.rs:57-62`

**Description:** The doc-comment on `TIER_RESOLVE_NOTE` describes it as being "folded into the writer-isolation reminder" (line 57) and states it fires at "every writer state regardless of tier" (line 61). Both descriptions are stale after Q-62.

**Evidence:**
- Line 57: `/// The resolve-the-tier note folded into the writer-isolation reminder when the tier`
- Line 61: `/// every writer state regardless of tier), this note fires only when the tier is still`

After Q-62 the enclosing block fires at all four agent-spawn states, not only writer states, and the reminder was renamed to "agent-isolation reminder" in the surrounding code.

**Why valid:** The doc-comment misrepresents the current trigger scope and uses the old "writer-isolation reminder" label. A reader relying on this comment to understand when the note fires will have an incorrect mental model.

**Suggested direction:** Update to "agent-isolation reminder" and "every agent-spawn state" (or "every spawn state") to match the updated predicate and surrounding doc-comments.

**Principle:** Accuracy over the smallest diff (P1 adjacent; correctness of documentation).

---

## B3

**Severity:** low
**Location:** `src/next.rs:177`

**Description:** The `principle_reminders` field doc-comment on the `Instruction` struct says "at writer states, the generated isolation-policy fragment." This description no longer covers the full trigger scope.

**Evidence:** Line 177:
```
/// mapped principle(s), and, at writer states, the generated isolation-policy
```
After Q-62 the fragment fires at all four agent-spawn states (planner, implementer, and both reviewer-spawn states).

**Why valid:** The struct-level doc-comment is the canonical description of the field. A reader of the struct definition who has not read `build_instruction` or `spawns_isolated_agent` will incorrectly infer the fragment is writer-state-only.

**Suggested direction:** Update to "at agent-spawn states" or "at planner, implementer, and reviewer-spawn states" to match the new scope.

**Principle:** Consistency between declaration-level docs and actual behaviour.

---

## B4

**Severity:** low
**Location:** `src/next.rs:1414`

**Description:** The test-section header `// -- The always-on writer-isolation reminder --` was not updated when the surrounding code was renamed from "writer-isolation reminder" to "agent-isolation reminder."

**Evidence:** Line 1414 still reads `// -- The always-on writer-isolation reminder --`. The corresponding production code at line 844 was updated to `// The always-on agent-isolation reminder:`.

**Why valid:** The test section header is stale relative to both the renamed predicate (`spawns_isolated_agent`) and the renamed inline comment at line 844. The inconsistency is minor but a reader navigating between the test section and the production section encounters two different names for the same concept.

**Suggested direction:** Rename to `// -- The always-on agent-isolation reminder --` to match the production-side inline comment.

**Principle:** Internal consistency.

---

## B5

**Severity:** low
**Location:** `src/next.rs:1428`

**Description:** The doc-comment for the `reminders_carry_the_resolve_note` helper says the note fires "at a writer state." After Q-62 the note fires at reviewer-spawn states too (when tier is unknown).

**Evidence:** Line 1428:
```
/// Whether any reminder carries the resolve-the-tier note (fired only when the tier is
/// still `unknown` at a writer state).
```
The resolve note fires via `TIER_RESOLVE_NOTE` inside the `spawns_isolated_agent` branch (line 847-853), which now includes reviewer states. The helper function itself is correct (it matches on the fragment content, not on state), but its description is wrong.

**Why valid:** The comment "at a writer state" was accurate before Q-62 and should have been updated. A future test author reading this helper's doc may incorrectly expect the note to be writer-state-only.

**Suggested direction:** Update to "at any agent-spawn state" (or "at any spawn state") when tier is unknown.

**Principle:** Internal consistency.

---

## Summary notes

**Golden fixture / P8:** The `__ISOLATION_FRAGMENT__` placeholder approach (substituted from the shared const in `golden_with_fragment`) is the right call. It eliminates a hand-copied duplicate and makes the golden tests self-consistent with the const without drift. This directly serves P8 (structured data first, project for humans) and the anti-drift direction. Not a concern.

**ASCII cleanliness:** No non-ASCII characters found in `src/next.rs` (grep for `[^\x00-\x7F]` returned nothing). Clean.

**"Still-open decision" language:** Fully removed. The old `spawns_writer` doc-comment carried "a separate, still-open driver-scoping decision, not settled by the fragment change alone"; the replacement `spawns_isolated_agent` doc-comment has no such language.

**AGENTS.md section name reference:** The doc-comment at line 293 references "`pack/AGENTS.md`, Writer isolation." The actual section is named "Writer isolation (capability-tiered)." Partial-name reference is accurate and unambiguous. Not a finding.

**Naming accuracy of `spawns_isolated_agent`:** Accurate. The four covered states all spawn agents that run in their own isolation tier because they perform a write (the reviewers write findings files). The name is a correct generalization from `spawns_writer`.

**"Agent isolation" lead wording at four spawn states:** Accurate. The planner, implementer, and both reviewer-spawn states all receive "Agent isolation (resolved tier: ...)." which correctly characterizes all four roles. Consistent with the uniform-isolation rule in AGENTS.md.
