# Triage: driver-output-generation increment 1

Triager: independent of the two reviewers, the implementer, and the orchestrator.
Diff `main (041c581) .. impl/dog-inc1 (9a8ea5a)`, reviewed in worktree
`.claude/worktrees/dog-inc1`. All five findings are LOW severity per both
reviewers. Verdicts below judge validity, actionability this round, and the
smallest correct fix.

Shared fix locus: D1-1 (fragment const), D1-2 (build-path test), and D1s-1
(orchestrator preamble) are the three actionable items. D1-1 and D1s-1 both edit
isolation-policy CONTENT and must be followed by one `just scaffold-self` so the
byte-guard and the verbatim-copied prompt regenerate; batch them in a single
scaffold-self run.

---

## D1-1: VALID. Actionable this round: YES.

Evidence: `src/isolation_policy.rs:30` (rendered verbatim at `AGENTS.md:91` and
`.agents/AGENTS.reference.md:91`). The fragment's first sentence states the
tier-general rule ("every spawned writer runs in the strongest isolation the
harness supports, per the capability-tiered tier order above"), but the second
sentence pins the WORKTREE tier: "each runs worktree-isolated exactly as the
implementer does, and the orchestrator merges its branch back on convergence."
That contradicts the tier order the same sentence's predecessor references
(container is tier 1, worktree tier 2, file-safety fallback tier 3, per
`AGENTS.md:83-88`), and it is stricter than the worktree-lifecycle paragraph
directly below, which is careful to gate on "When the resolved isolation tier is
a worktree" (`AGENTS.md:93`). The design intent is tier-general: "planners are
writers and are isolated under the same tier order" (`driver-output-generation.design.md:105`),
not pinned to worktree.

Why actionable now, not deferrable: this is the ONE canonical single source
(Principle 8, one source of truth); inc2 projects it VERBATIM into the driver's
writer-state reminder. A project resolved to the container or file-safety tier
would then read tier-wrong wording ("worktree-isolated", "merges its branch
back") in its driver output. Tightening the single source is cheaper before inc2
depends on it than after. Hardcoding is NOT acceptable on the "accurate to this
repo's practice" ground, because the fragment ships in the pack for any
scaffolded project and its own first clause promises tier-generality.

Smallest correct fix: reword only the second sentence of `ISOLATION_POLICY_FRAGMENT`
(`src/isolation_policy.rs:30`) to be tier-agnostic. Change:

  "A spawned planner is a writer, and so is any design or exploration writer that
  authors content: each runs worktree-isolated exactly as the implementer does,
  and the orchestrator merges its branch back on convergence."

to:

  "A spawned planner is a writer, and so is any design or exploration writer that
  authors content: each runs isolated under the same tier order as the
  implementer, and the orchestrator integrates its result on convergence."

Then run `just scaffold-self` to regenerate `AGENTS.md` and
`.agents/AGENTS.reference.md`; the byte-guard
`the_committed_scaffold_carries_the_isolation_policy_fragment` will fail until it
is re-scaffolded, which is the intended drift signal. The content-pin test
`the_fragment_states_the_writer_classification` still passes: the reword keeps
"A spawned planner is a writer" and leaves the "distinct from the orchestrator's
own integration-level edits on main" sentence unchanged.

---

## D1-2: VALID (real parity gap). Actionable this round: PARTIAL (add the build-path test now; defer/decline the reserved-var test).

Evidence: the precedent DOES carry both guards. `workflow_control_slot_renders_the_generated_fragment`
(`src/main.rs:1888-1917`) drives `build_assets` directly in-test and asserts the
`{{workflow_control}}` placeholder is gone and the fragment present; there is no
equivalent build-path test for `{{isolation_policy}}`.
`reserved_workflow_control_variable_is_rejected` (`src/manifest.rs:1208-1235`)
and `reserved_modules_variable_is_rejected` (`src/manifest.rs:1238`) pin that a
pack cannot declare or override those reserved vars; there is no
`isolation_policy` counterpart.

What the byte-guard does and does not cover: the drift-guard
(`src/isolation_policy.rs:63-78`) reads the committed files via `include_str!`
and does not re-run `build_assets`, so a broken or mis-keyed `build_assets`
insert (`src/main.rs:267-270`) passes the current suite and only fails after the
NEXT `just scaffold-self` overwrites `AGENTS.md` and the byte-guard then reads
the regenerated file. The build-path test closes that window: it exercises
`build_assets` in-test, catching a broken insert immediately with no re-scaffold.
That is a direct, cheap analog of the workflow_control precedent and worth adding
now on a foundation increment inc2 builds on.

The reserved-var test is marginal: the check at `src/manifest.rs:235` and `:240`
is `RESERVED_VARS.contains(...)` over a plain slice, fully generic and already
pinned twice (workflow_control and modules). The only uncovered case is a TYPO in
the `"isolation_policy"` slice entry (`src/manifest.rs:141`), which is negligible
risk and does not justify the ceremony at LOW severity. Decline it (or defer to
inc2 if a reminder-emission test is added there anyway).

Smallest correct fix (build-path test only): add a test mirroring
`workflow_control_slot_renders_the_generated_fragment`, swapping the slot name
and fragment source:

  #[test]
  fn isolation_policy_slot_renders_the_generated_fragment() {
      let principles = pack::default_principles();
      let selected = pack::resolve_selection(&principles, "default").unwrap();
      let assets = build_assets(
          &manifest::builtin(),
          &selected,
          Detail::Summary,
          &HashMap::new(),
          true,
          &[],
      )
      .unwrap();
      let fragment = isolation_policy::ISOLATION_POLICY_FRAGMENT;
      for dest in ["AGENTS.md", ".agents/AGENTS.reference.md"] {
          let asset = assets.iter().find(|a| a.dest == dest).unwrap();
          assert!(
              !asset.contents.contains("{{isolation_policy}}"),
              "{dest} should substitute the isolation_policy slot"
          );
          assert!(
              asset.contents.contains(fragment),
              "{dest} should carry the generated isolation-policy fragment"
          );
      }
  }

Place it beside the workflow_control test in the `src/main.rs` test module. If
D1-1 lands first, the fragment source is unchanged in shape, so the test needs no
edit. (Confirm the imports it references, `pack`, `manifest`, `Detail`,
`HashMap`, `build_assets`, resolve in that module, as the sibling test does.)

---

## D1-3: INVALID (accept as written). Actionable this round: NO.

Evidence: `src/isolation_policy.rs:30` opening clause ("every spawned writer runs
in the strongest isolation the harness supports ... while read-only agents need
none") does mildly restate the Writer-isolation rule (`AGENTS.md:83`) and the
read-only paragraph (`AGENTS.md:89`) directly above the slot. Opus itself rates
it "a deliberate lead-in" and confirms it REFERENCES, not duplicates, the tier
list.

Why not actionable: the fragment is DUAL-USE. Inc2 emits it standalone as the
driver's writer-state reminder, with no adjacent tier list or read-only
paragraph. The opening clause that looks redundant in the `AGENTS.md` context is
load-bearing self-containment in the standalone reminder. Trimming it to remove
the local `AGENTS.md` redundancy would degrade the second consumer. Since it
references rather than second-sources the tier order, there is no one-source
violation. Accept.

Note for inc2 (not a fix here): the phrase "per the capability-tiered tier order
above" is a dangling reference when the fragment is emitted standalone in a `next`
output (there is no tier order "above"). Inc2, which owns the reminder emission,
should resolve that reference at the emission site (a pointer to `AGENTS.md`, or
the projected resolved tier the design calls for at `driver-output-generation.design.md:106`).
Out of scope for inc1.

---

## D1s-1: VALID. Actionable this round: YES.

Evidence: `pack/prompts/orchestrator.md:13` (and its byte-identical verbatim copy
`.agents/prompts/orchestrator.md:13`). The parenthetical "(a spawned planner or
exploration writer is a writer, distinct from your own integration-level edits on
main)" paraphrases the fragment's classification ("A spawned planner is a writer,
and so is any design or exploration writer that authors content" / "distinct from
the orchestrator's own integration-level edits on main",
`src/isolation_policy.rs:30`). The prompt is `ownership = "reference"` with no
`render = true`, so it is copied VERBATIM, not substituted, and NO drift-guard
covers it (confirmed by the sonnet reviewer against `pack/pack.toml` and
`src/manifest.rs:316-334`). If the fragment's classification is later edited and
the prompt is not, the paraphrase silently diverges from the canonical source
(Principle 8).

Why actionable now: same content-precision locus as D1-1, same cheap-before-inc2
argument, and the fix has zero content loss because the surrounding sentence
already points at the full rule in `AGENTS.md`. This matches the Q-54 restraint
(name the slot, do not summarize its content, `pack/prompts/orchestrator.md:31`).

Smallest correct fix: delete the parenthetical, so the sentence names the slots
and points at `AGENTS.md`. Change:

  "for the policy itself, including which roles count as writers (a spawned
  planner or exploration writer is a writer, distinct from your own
  integration-level edits on main) and the authoring-versus-integration line, it
  references the Writer isolation rule in `AGENTS.md` rather than restating it, so
  the one source stays authoritative."

to:

  "for the policy itself, including which roles count as writers and the
  authoring-versus-integration line, it references the Writer isolation rule in
  `AGENTS.md` rather than restating it, so the one source stays authoritative."

Edit `pack/prompts/orchestrator.md` (the pack source) and run `just scaffold-self`
to regenerate the verbatim `.agents/prompts/orchestrator.md` copy; batch with the
D1-1 re-scaffold.

---

## D1s-2: INVALID (benign process note, no fix). Actionable this round: NO.

Evidence confirmed empirically: `git stash show --stat "stash@{0}"` ("nix fmt
reflow (dog-inc1)") lists `src/next.rs`, `src/plan/source.rs`, and thirteen docs,
but NOT `src/main.rs`. The committed `src/main.rs` diff is clean (14 insertions, 1
deletion: the `mod isolation_policy;` declaration, the `build_assets` doc-comment
update, and the `builtin.insert("isolation_policy", ...)` call). So the unrelated
`src/main.rs` `nix fmt` reflow hunks were reverted (via restore/checkout, or never
applied) rather than stashed.

Why not a defect: the project's prefer-stash-over-restore discipline exists to
keep IRRECOVERABLE work retrievable. The discarded content here is deterministic
`nix fmt` output, regenerable byte-for-byte by re-running `nix fmt`, so the risk
the discipline guards against does not apply. Nothing unrecoverable was lost and
the committed result is correct. Accurate observation, no action.

---

## Verdict summary

| id    | verdict | actionable? | one-line fix |
|-------|---------|-------------|--------------|
| D1-1  | VALID   | YES | Reword fragment sentence 2 to "runs isolated under the same tier order as the implementer, and the orchestrator integrates its result on convergence"; re-scaffold. |
| D1-2  | VALID   | PARTIAL | Add the `build_assets` slot test for `isolation_policy` (mirror the workflow_control one); decline the reserved-var test as marginal. |
| D1-3  | INVALID | NO | Accept: the lead-in is load-bearing self-containment for the standalone inc2 driver reminder; it references, not duplicates. |
| D1s-1 | VALID   | YES | Delete the classification parenthetical in `pack/prompts/orchestrator.md:13`; re-scaffold the verbatim `.agents` copy. |
| D1s-2 | INVALID | NO | Non-issue: discarded main.rs reflows are deterministic `nix fmt` output, regenerable; committed diff clean. |
