# Doc-redundancy audit: pack prompts and guidance vs AGENTS.md

READ-ONLY audit. This file scopes a later human design decision; it proposes no edits and changes nothing. It mirrors the Q-44 phase-1 audit pattern (prose findings with `file:line` evidence, see `docs/plans/architecture-audit.explorations/audit-*.md`).

Scope: redundancy between `pack/AGENTS.md` (the canonical guidance) and the other pack documents that restate its rules: the role prompts `pack/prompts/*.md`, the human prompts `pack/user-prompts/*.md`, `pack/LEDGER.template.md`, `pack/instrument.md`, `pack/isolation-guidance.md`, and `pack/checks-guidance.md`. `README.md` is out of scope (a separate auditor owns README-vs-AGENTS), as are the generated `.agents/` copies.

Files read in full: `pack/AGENTS.md`, all eight `pack/prompts/*.md`, all six `pack/user-prompts/*.md`, `pack/LEDGER.template.md`, `pack/instrument.md`, `pack/isolation-guidance.md`, `pack/checks-guidance.md`, plus `pack/principles.toml` (for the two principles below).

---

## The core tension (frame for the human's decision)

Every finding below sits on a line between two of the project's own principles, and the point of this audit is to locate each instance on that line, NOT to decide it:

- Principle "One source of truth" (P16): "Keep one authoritative source for each piece of data and derive the rest, rather than duplicating it and risking the copies drifting apart." A restated cap value or clean-round count is exactly such a duplicate, and if `AGENTS.md` changed, the copy would silently drift.
- Principle "Make documentation self-contained" (P20): "Explain the names, acronyms, and domain terms a document relies on ... so it stands on its own rather than assuming the author's context; define a term where it first appears, or point to where it is defined, so a reader arriving cold can still follow." A role prompt is often handed to a fresh agent with only its own text plus `AGENTS.md`; P20 is why a prompt legitimately spells out what its role must DO.

The tension is real and unavoidable: a prompt that references everything and states nothing is useless to the agent holding it, and a prompt that restates every rule drifts. The judgement this audit isolates is A-vs-B:

- A role prompt telling its role what to DO (a procedure, an instruction) is that prompt's job; it is not a duplicate of a canonical RULE. This is type B, legitimate.
- A prompt RESTATING a canonical RULE, and especially a specific CONSTANT (the cap = five, the clean-round counts one/two, the four-level severity names, the findings-file filename templates, the risk-class definition), is the smell: those are single values that live authoritatively in `AGENTS.md`, and a copy of them is a drift risk. This is type A.
- A copy that already materially disagrees with `AGENTS.md` is type C.

Note on P20's own escape hatch: P20 explicitly offers "point to where it is defined" as an alternative to restating. So the self-contained principle does NOT actually require restating a constant; a reference satisfies it. That is what makes the type-A cases genuinely fixable without harming self-containment: `pack/isolation-guidance.md:3` is the model already in the tree ("It does not restate that rule ... which own the tier order and the worktree lifecycle"), and it is fully self-contained by pointing rather than copying.

Prior-work context: `pack/AGENTS.md:59` already defers the ledger's round-record SCHEMA to `pack/LEDGER.template.md` ("The ledger's format is pinned by the scaffolded template"), and the RESUME-STATE / round-record shape now lives in the template rather than being enumerated in the orchestrator prompt. So the schema is single-sourced. What remains duplicated is the numeric CONSTANTS and a few canonical RULES, catalogued below.

---

## Type A: real SSOT drift-risk (rule or constant restated)

### A-1: The total-round cap value (five) is restated in the orchestrator prompt

Canonical: `pack/AGENTS.md:53` "The total rounds on an artifact reach the total-round cap (default five)"; restated at `pack/AGENTS.md:59` too, but that is the same doc.

Restated:
- `pack/prompts/orchestrator.md:19` "escalate to a human when the total rounds on an artifact reach the total-round cap (default five)".

The literal `five` is duplicated across the doc boundary. If the default cap changed in `AGENTS.md`, the orchestrator prompt would keep saying five. The orchestrator does need to know a cap exists and to act on it, but it does not need the literal value inline: it reads `AGENTS.md`.

Cheapest single-source fix: drop the `(default five)` parenthetical in `orchestrator.md:19` and refer to "the total-round cap defined in the Convergence rule in `AGENTS.md`" (the prompt already says "presenting the decision per the human-input contract in `AGENTS.md`" two clauses later, so the referencing style is already in use here).

### A-2: The consecutive-clean-round counts (one / two) and the risk-class definition are restated near-verbatim in the orchestrator prompt

Canonical: `pack/AGENTS.md:52` "require consecutive clean rounds before converging ... one for a trivial or low-risk artifact, two for a risky or high-blast-radius one. An artifact is risky or high-blast-radius when a defect in it would be costly or hard to reverse: it is security-, safety-, data-, or money-sensitive, is widely depended on, or changes something hard to roll back."

Restated (almost word for word):
- `pack/prompts/orchestrator.md:19` "The required number is one for a trivial or low-risk artifact and two for a risky or high-blast-radius one; an artifact is risky or high-blast-radius when a defect in it would be costly or hard to reverse: it is security-, safety-, data-, or money-sensitive, is widely depended on, or changes something hard to roll back."

This is the single clearest drift-risk in the pack: an entire canonical definition (the counts AND the risk criteria) copied verbatim into a prompt. Two full sentences that must stay identical to `AGENTS.md:52` by hand.

Cheapest single-source fix: replace the copied definition with "the required consecutive-clean count for the artifact's risk class, per the Convergence rule in `AGENTS.md` (one for low-risk, two for risky)", keeping at most the two bare numbers as a gloss and dropping the copied risk-criteria sentence entirely. Or drop even the numbers and reference the rule, since the orchestrator reads `AGENTS.md`.

### A-3: The findings-file filename templates are restated in the orchestrator prompt

Canonical: `pack/AGENTS.md:63` "a reviewer's file is `<step>-<role>-<disambiguator>.md` ... the triager's is `<step>-triage.md`; and the backstop re-check triager's is `<step>-triage-recheck.md`".

Restated:
- `pack/prompts/orchestrator.md:7` "per the naming convention in `AGENTS.md` (reviewers `<step>-<role>-<disambiguator>.md`, the triager `<step>-triage.md`, the backstop re-check triager `<step>-triage-recheck.md`)".

This one is instructive: the prompt says "per the naming convention in `AGENTS.md`" AND THEN restates all three templates inline. The reference is already there; the inline copy is pure duplication of the exact filename constants, which is the highest-drift-risk kind of copy (a formatting template). If the triager filename convention changed, this parenthetical would drift.

Cheapest single-source fix: delete the parenthetical, leaving "per the naming convention in `AGENTS.md`". The orchestrator reads `AGENTS.md:63` for the templates when it assigns paths.

### A-4: The separate-triager rule and its rationale are restated in full in two prompts

Canonical: `pack/AGENTS.md:22` "The triager is always a separate agent (or a human), independent of both the agent that produced the artifact under review and the orchestrator ... The orchestrator drives the loop and owns convergence and cost, so it is biased toward dismissing findings to converge, and letting it triage would let that bias decide which findings count." (Restated in-doc at `AGENTS.md:15`.)

Restated (rule plus the same bias rationale):
- `pack/prompts/orchestrator.md:5` "The triager is the exception: it is always a separate agent (or a human), independent of both the producer and you, for every review round, never played by you. You own the loop's convergence and cost, so you are biased toward dismissing findings to converge; triaging them yourself would let that bias decide which findings count."
- `pack/prompts/triager.md:3` "You are always a separate agent (or a human), independent of both the agent that produced the artifact under review and the orchestrator: you must not be either. The orchestrator owns the review loop's convergence and cost and so is biased toward dismissing findings to converge; keeping triage independent of it stops that bias from deciding which findings count."

This is a genuine A-vs-B borderline, and I am classing it A because the RATIONALE (not just the instruction) is copied three times. The role-scoped part is legitimate B: the orchestrator must be told "do not triage", and the triager must be told "you must be independent". But the "biased toward dismissing to converge" justification is a canonical RULE-rationale, and having it in three places means a change to the reasoning (or the "or a human" carve-out, or "for every review round including trivial ones") must be edited in three places.

Cheapest single-source fix: keep the bare instruction in each prompt (orchestrator: "never triage; the triager is always separate, see the Triager role in `AGENTS.md`"; triager: "you must be independent of both producer and orchestrator, see `AGENTS.md`") and drop the copied bias-rationale sentence from both prompts, letting `AGENTS.md:22` carry the why.

### A-5: The four-level severity scale (its literal level names) is restated in five places

Canonical: the scale is defined implicitly across `AGENTS.md`; the level names appear at `pack/AGENTS.md:55` "high-or-above on the four-level `low`/`medium`/`high`/`critical` scale".

Restated (the literal four names):
- `pack/prompts/reviewer.md:9` "Rate each finding's severity on a four-level scale: `low`, `medium`, `high`, or `critical`."
- `pack/prompts/triager.md:5` "Severity is a four-level scale: `low`, `medium`, `high`, or `critical`".
- `pack/prompts/checks-reviewer.md:13` "Rate each finding's severity on a four-level scale: `low`, `medium`, `high`, or `critical`."
- `pack/prompts/orchestrator.md:18` "high-or-above on the four-level `low`/`medium`/`high`/`critical` scale".
- `pack/instrument.md:5` "severity names on the four-level `low`/`medium`/`high`/`critical` scale".

Five copies of the same enumerated constant. This is the most-duplicated single item in the audit. It leans B for the reviewer / triager / checks-reviewer (a reviewer genuinely must know the four names to assign one, and a reference alone is thin here), but leans A overall because it is a fixed enumeration that must stay identical in five files, and `instrument.md` and `orchestrator.md` only USE the scale rather than assigning on it.

Cheapest single-source fix: two tiers. For the producers that must assign a severity (reviewer, checks-reviewer) keep the enumeration but treat one of them as canonical and have the others say "the four-level severity scale in `AGENTS.md` (`low`/`medium`/`high`/`critical`)"; for the pure consumers (`orchestrator.md:18`, `instrument.md:5`, and arguably `triager.md`) drop the inline enumeration and reference the scale. The "absolute rating, not a ranking relative to the other findings" gloss (see A-6) rides along with this and can be single-sourced the same way.

### A-6: The "absolute rating, not a relative ranking" severity gloss is restated in three prompts

Canonical rule surface: this gloss is stated by the prompts themselves rather than by a single `AGENTS.md` sentence, so it is a cross-prompt duplication with no owning source at all (a mild variant of the SSOT problem: three copies, no canonical home).

Restated:
- `pack/prompts/reviewer.md:9` "This is an absolute rating of the finding's impact if left unfixed, not a ranking relative to the other findings."
- `pack/prompts/triager.md:5` "an absolute rating of the finding's impact if left unfixed rather than a ranking relative to the other findings".
- `pack/prompts/checks-reviewer.md:13` "This is an absolute rating of the finding's impact if left unfixed, not a ranking relative to the other findings."

Cheapest single-source fix: state this gloss once in `AGENTS.md` alongside the severity-scale definition, then have the three prompts reference it (paired with the A-5 fix, since the two always travel together).

### A-7: The writer-isolation tier order is restated in the orchestrator prompt

Canonical: `pack/AGENTS.md:79-85` (the capability-tiered list: container, then worktree, then file-safety fallback).

Restated:
- `pack/prompts/orchestrator.md:11` "container isolation (for example agent-box / agent-images) if available, else a worktree, else the file-safety discipline as the fallback", already prefixed with "see the writer-isolation rule in `AGENTS.md`".
- Also touched at `pack/prompts/orchestrator.md:13` "writer work always runs in a separate, isolated (worktree-first where containers are not wired) agent".

Like A-3, the reference and the restatement sit in the same clause. `pack/isolation-guidance.md:3` is the counter-example done right ("It does not restate that rule ... which own the tier order"). The orchestrator prompt could adopt the same discipline.

Cheapest single-source fix: keep the reference, drop the inline "container ... else a worktree ... else file-safety" tier list, matching how `isolation-guidance.md` already handles it.

### A-8: The clean-round convergence constant is restated in LEDGER.template.md

Canonical: `pack/AGENTS.md:52` (the one/two clean-round counts by risk class).

Restated:
- `pack/LEDGER.template.md:9` "the artifact's risk classification (low-risk needs one clean round to converge, risky or high-blast-radius needs two)".

Nuance worth stating plainly for the human: `LEDGER.template.md` IS the canonical source for the round-record SCHEMA (`AGENTS.md:59` defers to it), so most of this file is a legitimate source, not a duplicate. But the parenthetical "low-risk needs one ... risky ... needs two" is a CONVERGENCE constant, which is owned by `AGENTS.md:52`, not by the ledger schema. So this one clause is a type-A copy embedded in an otherwise-authoritative file.

Cheapest single-source fix: change the parenthetical to "(the required clean-round count per the Convergence rule in `AGENTS.md`)", keeping the schema instruction ("record the artifact's risk classification") without the copied numbers.

### A-9: The acceptance-is-a-single-pass rule is restated in the orchestrator prompt

Canonical: `pack/AGENTS.md:33` "Acceptance is a single reviewers-then-triager pass, not the consecutive-clean convergence loop: it does not require clean rounds and does not run its own round loop or cap".

Restated:
- `pack/prompts/orchestrator.md:23` "Acceptance is a single reviewers-then-triager pass, not the consecutive-clean convergence loop: it does not require clean rounds and does not run its own round loop or cap."

Near-verbatim. This is a borderline A/B (the orchestrator must know how to run acceptance, which is B), classed A because it is a verbatim sentence copy of a canonical rule rather than a role-specific procedure, so it can drift.

Cheapest single-source fix: compress to "run acceptance as the single reviewers-then-triager pass defined in phase 5 of `AGENTS.md` (no convergence loop, no cap)", dropping the verbatim clause.

---

## Type B: legitimate role-scoped procedure (states what the role DOES, not a duplicated rule)

These are NOT drift-smells: each tells its role what to do, which is the prompt's job, and is what P20 (self-contained) protects. Listed so the human can see where the line was drawn and disagree per-instance.

### B-1: Planner's plan-drafting and principle-seeding procedure

- `pack/prompts/planner.md:5` "begin with the `AGENTS.md` principles in order, then add the project-specific ones after them, consolidating any overlap into a single amended principle, and keep the list numbered". Mirrors `AGENTS.md:30`, but this is the planner's concrete procedure and the planner is the role that performs it. B, though the "consolidating any overlap" phrasing is copied and could be trimmed to "per the Plan phase in `AGENTS.md`" if the human wants to tighten it (a weak-A argument exists).

### B-2: Implementer's file-safety instructions

- `pack/prompts/implementer.md:7` "Format only the files you changed; do not run repo-wide formatters (for example `just fmt` or `nix fmt`) or `git checkout` / `git restore` on files you do not own ... Run any destructive validation in a temporary directory or a worktree". Mirrors `AGENTS.md:75-76`. Classed B because these are the implementer's direct do/don't instructions and it is prefixed "see the file-safety rules in `AGENTS.md`". The specific examples (`just fmt` / `nix fmt`) are a mild duplication but are illustrative, not a canonical constant.

### B-3: Implementer's and orchestrator's Roadmap-status instruction

- `pack/prompts/implementer.md:5` "set the step's Status cell in the Roadmap table, which is the single source of truth for status". Mirrors `AGENTS.md:57`. B: it tells the implementer where to write status; the "single source of truth" phrase is a pointer, not a copied constant.
- `pack/prompts/orchestrator.md:31` "the Roadmap is the single source of truth for step status; decisions live in the Open Questions queue". Same, B.

### B-4: Reviewer / triager / checks-reviewer line-length carve-out

- `pack/prompts/reviewer.md:13`, `pack/prompts/triager.md:5`, `pack/prompts/checks-reviewer.md:19` each state "line length / prose line-wrapping is never a finding". Mirrors `AGENTS.md:100`. B: this is a direct behavioural instruction the role must follow while reviewing, and it is the kind of thing an agent handed only its prompt must be told outright. (Three copies is a mild smell, but each is a role instruction, not a constant.)

### B-5: Reviewer / triager / checks-reviewer ledger re-raise instruction

- `pack/prompts/reviewer.md:15`, `pack/prompts/triager.md:7`, `pack/prompts/checks-reviewer.md:19` each state "if given a ledger, do not re-raise a settled finding without new evidence". Mirrors `AGENTS.md:59`. B: the anti-relitigation rule is enforced by the roles doing the reviewing, so each must carry the instruction.

### B-6: Backstop re-check instruction in orchestrator and triager

- `pack/prompts/orchestrator.md:18` and `pack/prompts/triager.md:5` state that a dismissed high/critical finding is re-checked by a second triager. Mirrors `AGENTS.md:55`. B for the behavioural instruction (the orchestrator must run the re-check; the triager must make its high/critical dismissal auditable). The four-level-scale enumeration that rides inside `orchestrator.md:18` is the A-5 item, separated out above.

### B-7: Human prompts are thin triggers that reference rather than restate

All six `pack/user-prompts/*.md` explicitly declare themselves thin triggers and defer to `AGENTS.md`: `kickoff.md:3` "it deliberately does not restate the workflow", `explore.md:3` "it is a thin trigger and does not restate the workflow", `review.md:3`, `pause.md:3`, `resume.md:3`, `compaction-prep.md:3`. This is the deferral discipline done right and is the model for the type-A fixes. B (exemplary). One near-miss noted in C-2 below.

### B-8: isolation-guidance.md explicitly defers the tier rule

- `pack/isolation-guidance.md:3` "It does not restate that rule (see 'Writer isolation (capability-tiered)' and 'Worktree lifecycle and merge-back' above, which own the tier order and the worktree lifecycle); it supplies only the setup that makes the container tier available." Model deferral. B (exemplary); this is the phrasing the type-A fixes should copy.

### B-9: instrument.md module schema

`pack/instrument.md` is the canonical source for the JSONL round-log schema, so its field definitions are not duplicates. Its restatement of the four-level severity names (`instrument.md:5`) is the A-5 item; its `risk_class` gloss ("the convergence tier ... which sets how many clean rounds it takes to converge", `instrument.md:5`) references the convergence concept rather than restating the counts, so that part is B.

---

## Type C: already drifted (copy materially disagrees with AGENTS.md now)

### C-1: checks-reviewer.md and checks-guidance.md both restate the treefmt-v2 format-check caveat, and neither derives it from AGENTS.md

Not a divergence FROM `AGENTS.md` (the caveat is not in `AGENTS.md` at all), but a same-fact duplication between two pack docs with no single source, which is the same SSOT hazard one layer down:

- `pack/prompts/checks-reviewer.md:10` "Some formatters have no dry-run (treefmt v2, via `nix fmt --fail-on-change`, formats in place and then reports what changed), so that check-command mutates the tree: run it only in an environment where those changes are safe to discard".
- `pack/checks-guidance.md:9` "Some formatters have no dry-run: treefmt v2 (via `nix fmt --fail-on-change`) formats in place and then reports what changed, so that check mutates the tree and must be run where the changes are safe to discard".

These two are consistent TODAY (so strictly this is an A-style duplication, not yet a C divergence), but they already differ in a detail: `checks-reviewer.md:10` adds "(the orchestrator arranges this)" and a "degraded finding" rule for a format check with no `check` command, which `checks-guidance.md:9` does not mention, while `checks-guidance.md:9` adds "(the isolation mechanism for that is a separate concern this module does not settle)". So the two descriptions of the SAME mechanism have already begun to diverge in what they say around it. Flagging as C-borderline because the drift has started even though the core claim still agrees.

Cheapest single-source fix: pick one home for the treefmt-v2 caveat (the module guidance `checks-guidance.md` is the natural owner, since it is the module's explanatory doc) and have `checks-reviewer.md:10` reference it ("some formatters format-in-place, see `checks-guidance.md`"), keeping only the reviewer's own action (run the check where changes are safe to discard).

No hard type-C (a copy stating a value that contradicts `AGENTS.md`) was found: the numeric constants that are copied (A-1, A-2, A-8) currently all agree with `AGENTS.md`. The drift is latent, not yet realised, which is the argument for fixing the type-A cases before an `AGENTS.md` edit realises it.

---

## Counts

- Type A (real SSOT drift-risk): 9 (A-1 .. A-9).
- Type B (legitimate role-scoped): 9 groups (B-1 .. B-9), spanning ~18 individual restatements across the prompts.
- Type C (already drifted): 1 borderline (C-1: two pack docs describing one mechanism, divergence begun).

## Highest-value single-source fixes (ranked)

1. A-2 (`orchestrator.md:19`): remove the verbatim two-sentence copy of the clean-round counts AND the risk-class definition; reference `AGENTS.md:52`. This is the largest single block of copied canonical text in the pack and the highest drift-risk.
2. A-1 + A-8 (`orchestrator.md:19`, `LEDGER.template.md:9`): drop the literal cap value `five` and the "one clean round / two" numbers from the two files that copy them; reference the Convergence rule. Numeric constants are the sharpest drift edges.
3. A-3 + A-7 (`orchestrator.md:7`, `orchestrator.md:11`): delete the two inline restatements (findings-file filename templates; isolation tier order) that sit right next to an existing "see `AGENTS.md`" reference, matching the `isolation-guidance.md:3` model.
4. A-5 + A-6 (five files): give the four-level severity scale and its "absolute not relative" gloss ONE canonical statement in `AGENTS.md`, keep the enumeration only in the two producer prompts that assign severities (reviewer, checks-reviewer), and reference it from the consumers (orchestrator, instrument, triager).
5. A-4 (`orchestrator.md:5`, `triager.md:3`): keep the bare do/don't instruction in each prompt but drop the copied "biased toward dismissing to converge" rationale, letting `AGENTS.md:22` own the why.
