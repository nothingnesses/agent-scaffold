#### `include-all-visible`: Optional include-all-visible

Skipped by decision. A key to move every currently-visible Available match into Included at once (tag-based bulk selection on top of the filter). Skipped because the `available-filter` `/` filter already makes finding and adding matches quick (filter, then `i`/`a` on each), so bulk-add is not needed now; it stays minimal-by-default and can be revisited if adding many tagged principles at once becomes common. No `A` binding was added; the `selection-ui` key-bindings reference already includes `/` from `available-filter`.
