### TUI polish and tag-based selection

Shared context for the four steps below (`mode-enum`, `tag-selection`, `available-filter`, `include-all-visible`); this umbrella is not itself a Roadmap entry. These are near-term selector improvements that build on the shipped TUI; each reuses the existing `App`/`update`/`ui` structure and the `next_event` seam. The sub-steps are ordered so each is validated before the next depends on it.

Decisions adopted from the resolved open questions:

- **Modes (from OQ-B).** A `Mode` enum (`Editing`, `Filtering`, `Confirming { button }`) replaces the `confirming` bool and `confirm_button`, so the modes are mutually exclusive by construction (make illegal states unrepresentable) and later modes do not multiply bools. The applied filter query lives on `App` (not inside `Filtering`) because its narrowing of the Available pane persists back in `Editing`; `Filtering` just means keystrokes edit that query, which also keeps `Mode` `Copy`.
- **Filter (from OQ-C).** The filter narrows the Available pane only, live and incremental, a case-insensitive substring over name, id, and tags, with a hand-rolled query string (no new dependency) and a visible-to-underlying index projection for the cursor and toggle. Included is never filtered, so reordering is never over a partial view.
- **Tags (from OQ-D).** `--principles` accepts `tag:<t>` tokens (bare tokens stay ids; `default`/`all`/`none` unchanged); each tag expands in `default_order`, the whole list is de-duplicated by first occurrence, and an unknown tag returns `SelectionError::UnknownTag`. Interactive tag selection reuses the filter (which matches tags), with an optional include-all-visible action on top.

Each sub-step below carries the evidence-grounding discipline: validate the adopted approach with a proof-of-concept (build plus tests plus, where relevant, a functional run); on failure fall back to the recorded next-best approach; if all are exhausted, raise the impasse rather than force an unvalidated approach.

#### `mode-enum`: Mode enum refactor

Replaced `confirming: bool` and `confirm_button` with `enum Mode { Editing, Confirming { button: Button } }`; `update` dispatches on `app.mode` and the focused button exists only inside `Confirming`. The `Filtering` variant is introduced in `available-filter`, where it is first used, so every commit stays free of unused-variant warnings (`available-filter` added it as a unit variant with the query on `App`). Validated by proof-of-concept: the 24 existing selector tests pass unchanged (retargeted from the old bool to `Mode`) and behaviour is identical (open/save/cancel, editing keys ignored while confirming); clippy `-D warnings` clean. The fallback (keep the bool with a `debug_assert`) was not needed.
