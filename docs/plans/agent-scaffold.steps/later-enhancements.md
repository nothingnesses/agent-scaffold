### `later-enhancements`: Optional later enhancements

Not started; conditional and needs a design-questions pass first. Two independent enhancements, to be taken up only if the create-if-absent plus `--force`-overwrite model (see the Ownership decision) proves too blunt in practice:

- Marked-block augmentation of an existing `AGENTS.md`: insert and update a tool-managed block between explicit markers, idempotently, so a hand-written `AGENTS.md` can carry the generated principles without being overwritten. Hard parts to design: the marker format, idempotent re-runs (replace the block in place, never duplicate it), and what belongs inside the block versus the user's own prose.
- An opt-in `update` command doing a 3-way merge: merge upstream pack changes into a user-edited working file. Hard part: where the base (the version last scaffolded) is stored, since a 3-way merge needs a base, the user's edits, and the new version.

Each is evidence-grounded (proof-of-concept plus tests, with a byte-identical-when-unchanged invariant for the augmentation case); record the design questions in Open Questions before implementing.
