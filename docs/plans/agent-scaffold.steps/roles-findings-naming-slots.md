### `roles-findings-naming-slots`: generated `{{roles}}` and `{{findings_naming}}` AGENTS.md slots, killing the `next.rs` duplicates (`Q-64`)

Add generated `{{roles}}` and `{{findings_naming}}` slots to the AGENTS.md template, single-sourced with the values the `next` driver already emits, so the role list and the findings-file naming convention live in one place and are projected into both the AGENTS.md slot and the runtime instruction. This kills the existing duplicated copies in `src/next.rs` (around `:825` for roles and `:848-852` for findings-naming), which are today a second, drift-prone copy of the same guidance (Q-64-generation-architecture; Q-64-delivery-fsm).

This is the proven `ISOLATION_POLICY_FRAGMENT` / `control_fragment` single-source pattern applied to two more genuinely-duplicated slices: one source of truth, consumed by both the AGENTS.md slot and the driver, byte-guarded so the copies cannot drift. It serves Structured data first, project for humans and Prefer the cleaner long-term architecture over the smallest diff by removing hand-maintained duplicates.

Endorsed-now core item under `generated-projection` (Q-64, Option A refined). Build now.
