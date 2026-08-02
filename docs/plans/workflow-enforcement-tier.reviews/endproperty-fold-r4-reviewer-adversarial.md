# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 4, ADVERSARIAL REVIEWER (mechanism attack)

Artifact: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, increment 2 (the containment predicate and its consumers). Increment 1 has landed; increment 2 has not been built.

Lens: BREAK THE MECHANISM. Not prose. The goal was to construct a case where increment 2 as specified produces a wrong answer that the document does not record as an accepted cost, either a FALSE GREEN (a pairing that should be caught and is not) or a FALSE REFUSAL / FALSE OMISSION (a layout that works today and stops working, unrecorded).

Base commit: `dd54227`. Binary: `target/debug/agent-scaffold` built from that commit (increment 1 in the tree, increment 2 absent). All probe fixtures were built under `TMPDIR=/tmp/claude-1000/r4adv`, outside any git repository other than the ones the fixtures scaffold for themselves.

THREE FINDINGS. One high (an unrecorded false green that survives increment 2 and that defeats the property checks 11 and 13b assert), one medium (an accepted cost whose stated scope is narrower than the mechanism's actual false-positive population), one low (an unrecorded false omission on `status --resume`).

WHERE I REASON ABOUT UNBUILT BEHAVIOUR, I SAY SO. Increment 2 does not exist, so no finding here can show its output. What I did instead was measure every INPUT to the specified predicate with the built binary, and compute the predicate's one unbuilt step (the containment test) by hand. The predicate as the document specifies it has three parts, and two of the three are already in the tree:

1. The root: `project_root_of_source` (`src/main.rs`) applied to the canonicalised location of the plan the check reads. `project_root_of_source` is BUILT and shipping, and the document confirms the guard reuses it, including its fallback ("after inc2 the checked plan's root is `<root>/notes`, through `src/main.rs:project_root_of_source`'s fallback to the checked plan's own directory"). Every root below is therefore MEASURED, by running the built binary against the canonical path and reading the resolved log path back out of its `no metrics log at <path>` note.
2. The resolved log: `resolve_metrics_path` (`src/main.rs`), BUILT and shipping, then "absolutising and canonicalising its longest existing ancestor and re-appending the components below it". Measured with the binary plus `realpath`.
3. The containment test: "push a problem when the resolved log is not under that root". THIS IS THE ONLY UNBUILT STEP, and it is a path-prefix comparison with no free parameters. I compute it below with a shell prefix match and show the two paths verbatim so a triager can check it by eye.

The model is validated against a case the document has already MEASURED: accepted cost (ii) says the symlinked-`docs/plans` layout goes "from reading its 37-record log to `exit=1 REFUSED`". My model reproduces exactly that verdict on that layout (fixture `P4` below). A model that reproduces the one refusal the design pass measured is the best available evidence that it predicts the others correctly.

---

## `R4A-1` (HIGH). FALSE GREEN, UNRECORDED. The containment predicate is vacuous whenever the checked plan's derived root is a directory that contains other projects, which is exactly what `Q-55-noconvention`'s fallback produces for a plan at a repository root. Both check 11's and check 13b's asserted properties fail on that layout, and `next` still emits `state: converged` / `next: mark the step complete`

### The claim

The predicate is CONTAINMENT: is the resolved log under the root of the plan the check reads. The root is `project_root_of_source` on the checked plan, INCLUDING its fallback, and `Q-55-noconvention` decided that fallback is "the source's own directory" rather than a hard error, "with the containment refusal layered on top".

For a plan under `<root>/docs/plans/`, the derived root is `<root>` and containment means "the log is inside this project". For a CONVENTIONLESS plan (one with no `docs/plans` ancestor, the layout `Q-55-noconvention` was decided in order to SUPPORT, whose stated justification case is the "`--source myplan.plan.toml`-at-a-repository-root case"), the derived root is the plan's own parent directory. That directory is a repository root, and a repository root routinely CONTAINS other scaffolded projects: a vendored dependency, a monorepo package, an `examples/` scaffold, a test fixture tree. Every one of those has its own conventional `docs/metrics/workflow.jsonl`, and every one of those logs satisfies containment against the outer plan's root.

So on a conventionless checked plan, the predicate does not ask "is this log mine". It asks "is this log somewhere in my repository", and answers yes for every other project the repository contains.

### Reproduction, part 1: the derived root, MEASURED

```sh
export TMPDIR=/tmp/claude-1000/r4adv
W=/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-ep4-adv
AS="$W/target/debug/agent-scaffold"
S=$TMPDIR/fix
SLUG=triager-runs-only-on-findings

mkdir -p "$S/repo"
# A second, separate, conventionally laid out project INSIDE the first one's directory.
"$AS" scaffold --output-dir "$S/repo/vendor/projA" --write --force --principles default
mkdir -p "$S/repo/vendor/projA/docs/metrics"
cp "$W/docs/metrics/workflow.jsonl" "$S/repo/vendor/projA/docs/metrics/workflow.jsonl"

# The conventionless checked plan: a TOML-primary plan.toml at the repository root,
# one step, the borrowed slug, marked complete, with NO review evidence of its own.
sed -e "s/^slug = \"example-step\"/slug = \"$SLUG\"/" \
    -e 's/^status = "not-started"/status = "complete"/' \
    "$S/repo/vendor/projA/docs/plans/TEMPLATE.plan.toml" > "$S/repo/myplan.plan.toml"
sed -i 's|TEMPLATE\.|myplan.|g' "$S/repo/myplan.plan.toml"
mkdir -p "$S/repo/myplan.steps"
cp "$S/repo/vendor/projA/docs/plans/TEMPLATE.steps/example-step.md" "$S/repo/myplan.steps/$SLUG.md"
```

The root is read straight off the binary, by asking it where the DEFAULT log is:

```
$ "$AS" validate --source "$S/repo/myplan.plan.toml" --workflow
no metrics log at /tmp/claude-1000/r4adv/fix/repo/docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
/tmp/claude-1000/r4adv/fix/repo/myplan.plan.toml: 1 steps, 0 questions, valid
exit=0
```

The derived root is `/tmp/claude-1000/r4adv/fix/repo`. There are no symlinks anywhere in this fixture, so the lexical root the binary just printed and the canonical root increment 2 would derive are the SAME PATH, and this measurement stands for both. Note also that the project's own conventional log does not exist, which is what makes the next run's log unambiguously a different project's.

### Reproduction, part 2: check 11's exact shape, unrefused

Check 11 is "the explicit-relative-`--metrics` false pass is refused ... exits NON-ZERO with the refusal naming both paths and the derived root". Same attack, same borrowed slug at `complete`, same explicit foreign `--metrics`, only the checked plan's LOCATION differs:

```
$ "$AS" validate --source "$S/repo/myplan.plan.toml" \
    --metrics "$S/repo/vendor/projA/docs/metrics/workflow.jsonl" --workflow
/tmp/claude-1000/r4adv/fix/repo/vendor/projA/docs/metrics/workflow.jsonl: 256 records, valid
/tmp/claude-1000/r4adv/fix/repo/myplan.plan.toml: 1 steps, 0 questions, valid
/tmp/claude-1000/r4adv/fix/repo/myplan.plan.toml vs /tmp/claude-1000/r4adv/fix/repo/vendor/projA/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

THE CONTAINMENT TEST, the one unbuilt step, computed by hand:

- root: `/tmp/claude-1000/r4adv/fix/repo`
- resolved log, canonicalised: `/tmp/claude-1000/r4adv/fix/repo/vendor/projA/docs/metrics/workflow.jsonl`
- the log IS under the root, so the predicate does not fire, so nothing is pushed to `problems`, so increment 2 prints the same three lines and exits 0.

`workflow invariants hold` for a plan whose only `complete` step has no round record anywhere in its own project, satisfied by another project's log. That is the end property's own words falsified: "`validate --workflow` must never pair a plan source with a metrics log belonging to a different project and report success."

### Reproduction, part 3: check 13b's shape, with NO explicit flag, and the agent-facing half

The same hole is reachable through the divergent `--source`/`--plan` pairing that `Q-55-endproperty` exists to close, with no `--metrics` and no `--ledger-fragment` anywhere on the command line. Fixture:

```sh
mkdir -p "$S/loose"
"$AS" scaffold --output-dir "$S/loose/projA" --write --force --principles default
mkdir -p "$S/loose/projA/docs/metrics"
cp "$W/docs/metrics/workflow.jsonl" "$S/loose/projA/docs/metrics/workflow.jsonl"
sed 's/^primary = "toml"/primary = "markdown"/' \
    "$S/loose/projA/docs/plans/TEMPLATE.plan.toml" > "$S/loose/projA/docs/plans/A.plan.toml"
sed -e "s/| \`example-step\` | not started |/| \`$SLUG\` | complete |/" \
    -e "s/### \`example-step\`:/### \`$SLUG\`:/" \
    "$S/loose/projA/docs/plans/TEMPLATE.md" > "$S/loose/p.md"
sed 's/| complete |/| in progress |/' "$S/loose/p.md" > "$S/loose/pip.md"
```

The checked plan's root, measured the same way (`"$AS" validate --plan "$S/loose/p.md" --workflow`):

```
no metrics log at /tmp/claude-1000/r4adv/fix/loose/docs/metrics/workflow.jsonl; nothing to validate
```

so the root is `/tmp/claude-1000/r4adv/fix/loose`, and `projA`'s log at `/tmp/claude-1000/r4adv/fix/loose/projA/docs/metrics/workflow.jsonl` is under it. The validator:

```
$ "$AS" validate --source "$S/loose/projA/docs/plans/A.plan.toml" --plan "$S/loose/p.md" --workflow
/tmp/claude-1000/r4adv/fix/loose/projA/docs/metrics/workflow.jsonl: 256 records, valid
/tmp/claude-1000/r4adv/fix/loose/projA/docs/plans/A.plan.toml: 1 steps, 0 questions, valid
/tmp/claude-1000/r4adv/fix/loose/p.md: 1 steps, 0 open-questions items, valid
/tmp/claude-1000/r4adv/fix/loose/p.md vs /tmp/claude-1000/r4adv/fix/loose/projA/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

That is check 13b's invocation shape exactly (a Markdown-primary `--source` in one project, a `--plan` in another, no explicit `--metrics`), and check 13b says it "must exit NON-ZERO with the refusal naming B's plan, A's log, and B's root". Under increment 2 it does not, because A's log is under B's root.

And the agent-facing half, on the same pairing with the step at `in progress`, which is check 14b's shape:

```
$ "$AS" next --source "$S/loose/projA/docs/plans/A.plan.toml" --plan "$S/loose/pip.md"
task: A
source: /tmp/claude-1000/r4adv/fix/loose/pip.md
metrics: 256 records

ACTIVE LOOP
  triager-runs-only-on-findings / triager-runs-only-on-findings-inc1  in progress -> mark-step-complete
  state: converged
  streak: 1/1
  rounds: 2/5
  isolation: unknown
  next: mark the step complete, re-render, and commit
  role: orchestrator
  prompt: .agents/prompts/orchestrator.md
exit=0
```

This is, line for line, the output the document names as "the specific output the fix must make unreachable": `state: converged`, `streak: 1/1`, `rounds: 2/5`, `next: mark the step complete, re-render, and commit`, at exit 0, for a project with zero rounds of its own. It survives increment 2 as specified, because the predicate that is supposed to omit the block never fires. `next --json` on the same inputs would carry `"metrics_absent_reason": null` and a fully populated `"active_loop"`, since increment 2 computes no reason where the predicate is silent.

### Why this is not any of the recorded residuals

- IT IS NOT THE COPIED-LOG RESIDUAL. That residual is stated as "A log COPIED into a project's own `docs/metrics/`", and its whole mechanism is that "the guard passes (the log IS under the fixture's root)" because the log sits at the CHECKED PLAN'S OWN conventional path. Here nothing is copied into the checked plan's own `docs/metrics/`; part 1 measures that the checked plan's own conventional log DOES NOT EXIST. The log that is read is a DIFFERENT project's log at THAT project's own conventional path, and it is reached by the two routes increment 2 is built to close (an explicit foreign `--metrics`, and the divergent pairing), not by a copy.
- IT IS NOT ACCEPTED COST (i). Cost (i) is a SILENT MISS ("the wrong path is still inside the right project"), a green that reads nothing. This is a green that reads another project's evidence and asserts an invariant over it.
- IT IS NOT COSTS (iii) OR (iv), AND THAT IS THE SHARPEST PART. Those two are the SAME fallback, and the document says so: "COSTS (iii) AND (iv) SHARE ONE ROOT CAUSE, `src/main.rs:project_root_of_source`'s fallback to the plan's own parent, and treating it ONCE IS QUEUED TO THE SAME STEP". But both are OVER-refusals: the fallback makes the root too NARROW (`<root>/notes`) and a legitimate log falls outside it. Nowhere does the document record that the same fallback can make the root too WIDE, and that the too-wide direction is a FALSE GREEN rather than a false refusal. An implementer of the queued step would read the recorded root cause as "the fallback refuses things it should not" and would not know it also greens things it should not.

### Severity

HIGH. It is an unrecorded false green, the rubric's own high-or-critical class. I am not calling it critical, and the reachability caveat should be read with the finding: it needs a checked plan with no `docs/plans` ancestor AND a second project nested under that plan's directory AND either an explicit `--metrics` or a divergent `--source`/`--plan` pairing. What keeps it at high rather than lower is that the layout is one this step DECIDED TO SUPPORT rather than one it excluded, that the two triggering routes are precisely the two the increment exists to close (explorer A's second false pass and `Q-55-endproperty`), and that the agent-facing surface still emits `mark the step complete`.

### What would close it, offered as evidence that the finding is actionable, not as a prescription

Either state the bound as a fifth accepted cost (the predicate's strength is a function of the checked plan's root depth, and a conventionless root can contain other projects, so the refusal does not reach a nested project's log), or add one run to check 13b on a conventionless B so the bound is pinned rather than assumed. The document's own standard applies: a check that pins a cost is worth more than a note asking people to remember it.

---

## `R4A-2` (MEDIUM). FALSE REFUSAL, UNRECORDED SCOPE. Accepted cost (ii)'s stated scope is one symlink placement; the mechanism refuses at least four, and for two of them cost (ii)'s stated MECHANISM does not even describe what happens

### The claim

Cost (ii) is stated as "A SYMLINKED `docs/plans` DIRECTORY BECOMES A FALSE POSITIVE ON THE PREDICATE. Where `<root>/docs/plans` is a symlink to `<root>/elsewhere`, the lexical default and the canonical guard disagree about which project the plan belongs to, and the guard wins", and check 19 pins exactly that one layout ("a layout where `<root>/docs/plans` is a SYMLINK to a sibling directory").

The real population is every symlink placement that makes the canonicalised checked plan and the canonicalised resolved log diverge. There are four such placements and cost (ii) names one. The other three are layouts that work today and stop working after increment 2, and none is recorded.

For two of them the plan side is not involved at all, so cost (ii)'s stated mechanism ("disagree about which project the PLAN belongs to") does not describe them: the plan's project is unambiguous, and it is the LOG that canonicalises out of the root.

### Reproduction

```sh
export TMPDIR=/tmp/claude-1000/r4adv
W=/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-ep4-adv
AS="$W/target/debug/agent-scaffold"
S=$TMPDIR/fix
rm -rf "$S/P0" "$S/P1" "$S/P2" "$S/P3" "$S/P4" "$S/shared-plans" "$S/shared-metrics" "$S/shared-log"

# P0, the control: a plain scaffolded project with an empty log of its own.
"$AS" scaffold --output-dir "$S/P0" --write --force --principles default
mkdir -p "$S/P0/docs/metrics"; : > "$S/P0/docs/metrics/workflow.jsonl"

# P1: the PLAN FILE is a symlink out of the project. docs/plans is a REAL directory.
cp -r "$S/P0" "$S/P1"; mkdir -p "$S/shared-plans"
mv "$S/P1/docs/plans/TEMPLATE.plan.toml" "$S/shared-plans/TEMPLATE.plan.toml"
ln -s "$S/shared-plans/TEMPLATE.plan.toml" "$S/P1/docs/plans/TEMPLATE.plan.toml"

# P2: docs/metrics is a symlink to a shared metrics directory.
cp -r "$S/P0" "$S/P2"; mkdir -p "$S/shared-metrics"; : > "$S/shared-metrics/workflow.jsonl"
rm -rf "$S/P2/docs/metrics"; ln -s "$S/shared-metrics" "$S/P2/docs/metrics"

# P3: the LOG FILE itself is a symlink to a shared log.
cp -r "$S/P0" "$S/P3"; mkdir -p "$S/shared-log"; : > "$S/shared-log/workflow.jsonl"
rm -f "$S/P3/docs/metrics/workflow.jsonl"
ln -s "$S/shared-log/workflow.jsonl" "$S/P3/docs/metrics/workflow.jsonl"

# P4: accepted cost (ii) itself, as the control on the model.
cp -r "$S/P0" "$S/P4"; mkdir -p "$S/P4/elsewhere"
mv "$S/P4/docs/plans/"* "$S/P4/elsewhere/"; rmdir "$S/P4/docs/plans"
ln -s "$S/P4/elsewhere" "$S/P4/docs/plans"
```

ALL FIVE WORK TODAY. Verbatim, for each of `P0` through `P4`, `"$AS" validate --source "$S/<P>/docs/plans/TEMPLATE.plan.toml" --workflow` prints its own three lines and exits 0, for example:

```
$ "$AS" validate --source "$S/P2/docs/plans/TEMPLATE.plan.toml" --workflow
/tmp/claude-1000/r4adv/fix/P2/docs/metrics/workflow.jsonl: 0 records, valid
/tmp/claude-1000/r4adv/fix/P2/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
/tmp/claude-1000/r4adv/fix/P2/docs/plans/TEMPLATE.plan.toml vs /tmp/claude-1000/r4adv/fix/P2/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

THE ROOTS ARE MEASURED, not assumed. For the two fixtures whose PLAN is symlinked, `project_root_of_source` runs against the canonical location, so I ran the binary against that location directly and read the root back out:

```
$ "$AS" validate --source "$S/shared-plans/TEMPLATE.plan.toml" --workflow     # P1's canonical plan
no metrics log at /tmp/claude-1000/r4adv/fix/shared-plans/docs/metrics/workflow.jsonl; nothing to validate

$ "$AS" validate --source "$S/P4/elsewhere/TEMPLATE.plan.toml" --workflow     # P4's canonical plan
no metrics log at /tmp/claude-1000/r4adv/fix/P4/elsewhere/docs/metrics/workflow.jsonl; nothing to validate
```

so P1's root is `.../fix/shared-plans` and P4's is `.../fix/P4/elsewhere`, both through the fallback. The containment table, with the canonical log from `realpath` and the one unbuilt step computed as a path-prefix match:

```
  P0  root=/tmp/claude-1000/r4adv/fix/P0            log=/tmp/claude-1000/r4adv/fix/P0/docs/metrics/workflow.jsonl  UNDER root -> predicate silent
  P1  root=/tmp/claude-1000/r4adv/fix/shared-plans  log=/tmp/claude-1000/r4adv/fix/P1/docs/metrics/workflow.jsonl  NOT under root -> REFUSE / OMIT
  P2  root=/tmp/claude-1000/r4adv/fix/P2            log=/tmp/claude-1000/r4adv/fix/shared-metrics/workflow.jsonl   NOT under root -> REFUSE / OMIT
  P3  root=/tmp/claude-1000/r4adv/fix/P3            log=/tmp/claude-1000/r4adv/fix/shared-log/workflow.jsonl       NOT under root -> REFUSE / OMIT
  P4  root=/tmp/claude-1000/r4adv/fix/P4/elsewhere  log=/tmp/claude-1000/r4adv/fix/P4/docs/metrics/workflow.jsonl  NOT under root -> REFUSE / OMIT
```

P4 IS THE MODEL'S CONTROL. It is accepted cost (ii)'s own layout and my model gives it the verdict the design pass MEASURED for it ("from reading its 37-record log to `exit=1 REFUSED`"). P1, P2 and P3 get the identical verdict from the identical computation, and none of the three is recorded anywhere in the document.

The quiet half is measured too. Today, on the two log-side layouts:

```
$ "$AS" status --source "$S/P2/docs/plans/TEMPLATE.plan.toml"
plan: 1 steps (1 not started); 0 open-questions items
metrics: 3 records
exit=0
```

(the same for P3). After increment 2 these lose their metrics half at exit 0 under `Q-55-refusalscope`, and `next` loses the whole `ACTIVE LOOP` block, which is the manifestation cost (ii) itself calls "the more expensive half of an accepted cost" and which is recorded only for the `docs/plans` layout.

### One reading-dependency, stated because a triager must check it

P3 (the log FILE is the symlink) depends on how "canonicalising its longest existing ancestor and re-appending the components below it" is implemented. If the longest existing ancestor is taken to include the path itself when the leaf exists, P3 canonicalises out of the root and is refused. If it is taken to mean the deepest existing DIRECTORY with the leaf name re-appended lexically, P3 stays under the root and is allowed, and the mechanism then reports a log whose real content lives outside the project, which is the opposite defect. Either way the document says nothing about it. P1 and P2 are refused under both readings and do not depend on this.

### Severity

MEDIUM. It is a false refusal, so the rubric turns on how ordinary the layout is, and these are not ordinary: a symlinked plan file (a plan shared into a project from a vendored or shared location), a symlinked `docs/metrics` (a data directory moved to a bigger volume), a symlinked log file (a log shared between two projects, which the document itself acknowledges as a real scenario when it says identity fields are "the ONLY mechanism that separates two projects LEGITIMATELY SHARING ONE MERGED LOG"). What lifts it above low is that the document's ONE sentence on the whole symlink class ("the GUARD is canonical so it cannot be spoofed by a symlinked source") frames symlinked sources purely as an attack to be defeated, and check 12 pins only the attack direction, so a reviewer working from this document has nothing telling them the same mechanism refuses the mirror-image legitimate layouts.

---

## `R4A-3` (LOW). FALSE OMISSION, UNRECORDED. `Q-55-resumepairing`'s agreement rule makes a NONEXISTENT `--plan` suppress a project's own, correctly located resume block, on a flag that is inert today

### The claim

`Q-55-resumepairing` says of `status --resume`: "a `--source` and a `--plan` both named must resolve to the SAME root or the block is omitted, and with one alone the anchor is the root, as today."

A `--plan` naming a path that does not exist is NAMED. It has no canonical location, so it yields no root under the canonical derivation the guard uses everywhere else, so it cannot "resolve to the same root" as the `--source`, so the block is omitted. The `--source` is meanwhile perfectly good and its ledger is exactly where it should be.

Today `--plan` is INERT on this surface whenever a `--source` is given: `run_resume` (`src/main.rs`) computes the task with `next::derive_task(&args.source, &args.plan)` and the path with `default_ledger_path(&task, &args.source, &args.plan)`, and both take `source.as_ref().or(plan.as_ref())`, so the `--plan` value is never consulted. Increment 2 gives that previously inert flag the power to suppress the output.

### Reproduction (today's behaviour, and the input that changes)

```sh
printf '## RESUME STATE\n\nMARKER-P0-BLOCK\n' > "$S/P0/docs/plans/TEMPLATE.ledger.md"

$ "$AS" status --resume --source "$S/P0/docs/plans/TEMPLATE.plan.toml" --plan "$S/P0/docs/plans/does-not-exist.md"
## RESUME STATE

MARKER-P0-BLOCK
exit=0

$ "$AS" status --resume --source "$S/P0/docs/plans/TEMPLATE.plan.toml"
## RESUME STATE

MARKER-P0-BLOCK
exit=0
```

The two runs agree today. Under increment 2 they do not: the second keeps printing (one anchor, the anchor is the root, the ledger is under it), the first prints a note and no block.

### Both readings, since this is an under-specification and a triager must check which one bites

- CANONICAL reading (the one the rest of the mechanism uses): a nonexistent `--plan` has no root, the agreement test fails, the block is omitted. This is the false omission.
- LEXICAL reading (evaluate the agreement on unresolved paths): `project_root_of_source` on `$S/P0/docs/plans/does-not-exist.md` is `$S/P0`, which equals the source's root, so the block prints and nothing changes. But this reading uses a lexical root for the agreement test and a canonical root for the containment test on the same surface, which is a second lexical/canonical split the document does not describe.

### Relationship to the already-ruled `R2B-2`

`R2B-2` was accepted as a residual and is NOT re-raised here. That one is about an explicit `--ledger-fragment` given ALONGSIDE a divergent pairing. This one has no `--ledger-fragment` at all, and its trigger is a `--plan` that does not exist, a case the document treats explicitly on the validator (check 13b's second run pins that a typo'd `--source` still refuses) and not at all here. If the triager judges the two to be one under-specification of one sentence, folding them is defensible; I raise it separately because the inputs and the consequence differ.

### Severity

LOW. It needs a `--plan` naming a nonexistent file, which is a typo or a plan whose Markdown projection has not been rendered yet. The consequence is real (an agent invoking `status --resume` loses the resume anchor the whole defect-C narrative is about, silently, at exit 0) but the trigger is narrow.

---

## ATTACK LOG

Twenty-four attacks. Three broke something. The failures are listed with the same care as the successes, because they are what makes the three findings mean something.

| # | attack | outcome |
| --- | --- | --- |
| 1 | Symlinked PLAN FILE inside a real `docs/plans`, pointing out of the project (`P1`) | BROKE. `R4A-2`. Root canonicalises to the symlink target's own directory through the fallback; the project's real log is not under it. |
| 2 | Symlinked `docs/metrics` DIRECTORY (`P2`) | BROKE. `R4A-2`. Plan root unambiguous; the LOG canonicalises out of the root. Cost (ii)'s stated mechanism does not describe this. |
| 3 | Symlinked LOG FILE at the conventional path (`P3`) | BROKE, reading-dependent. `R4A-2`. Refused if the leaf is canonicalised; if not, the tool reads a log whose content lives outside the project and says nothing. |
| 4 | Symlinked `docs/plans` DIRECTORY (`P4`), accepted cost (ii) itself | Did not break anything: reproduced the documented refusal exactly. Used as the CONTROL that validates the predicate model. |
| 5 | Conventionless checked plan at a repository root, foreign log at a NESTED project's conventional path, explicit `--metrics` (check 11's shape) | BROKE. `R4A-1`. `workflow invariants hold` at exit 0, predicate silent. |
| 6 | Same, through the divergent `--source`/`--plan` pairing with NO explicit flag (check 13b's shape) | BROKE. `R4A-1`. The case `Q-55-endproperty` exists for, unrefused. |
| 7 | Same on `next` with the step at `in progress` (check 14b's shape) | BROKE. `R4A-1`. `state: converged` / `next: mark the step complete` at exit 0, the exact output the document says the fix must make unreachable. |
| 8 | The same two attacks on a CONVENTIONAL checked plan (`$S/conv/A` vs `$S/conv/B`, and `P0` with a `..`-escaping `--metrics`) | Did not break anything. The predicate fires; checks 11 and 13b hold on their own fixture shapes. This is the contrast that isolates `R4A-1` to root depth. |
| 9 | `--metrics` escaping the root through a NONEXISTENT intermediate component, so the containment test sees an unresolved lexical tail that still starts with the root | Did not break anything. `.../docs/metrics/nope/../../../../fix/conv/A/docs/metrics/workflow.jsonl` is not openable at all (the kernel resolves components left to right), so the binary reports `no metrics log at <path>` and reads nothing. No escape exists through this route. |
| 10 | `--metrics` escaping through EXISTING components (check 13's shape) | Did not break anything. Fully canonicalises out of the root; refused. |
| 11 | Hard link to a foreign log at the project's OWN conventional path | Greens today and would green after increment 2 (a hard link has no target, so it canonicalises to the in-root path). NOT RAISED: this is the recorded copied-log residual in a different spelling, and it is a log at the checked plan's own conventional path, which is exactly what that residual describes. Same verdict for a bind mount. |
| 12 | `docs/plans` existing as a FILE rather than a directory, passed as `--source` | Did not break anything. Root falls back to `<root>/docs`, the source is malformed, and the match's `(None, None, _)` arm makes it a hard problem: exit 1. |
| 13 | Zero-byte plan source under a real `docs/plans` | Did not break anything. Malformed source plus the `(None, None, _)` hard problem: exit 1. |
| 14 | Zero-byte metrics log present at the right path | Did not break anything, and confirms the document's own "A CASE THAT IS NOT PART OF THIS": the check runs, `0 records, valid`, and the green is correct because the step is `not-started`. |
| 15 | `--source` naming a DIRECTORY | Did not break anything new. `Error: Os { code: 21, kind: IsADirectory }`, exit 1, before any workflow logic. Pre-existing, untouched by increment 2. |
| 16 | `--metrics` naming a DIRECTORY | Same as 15: `IsADirectory`, exit 1. Pre-existing. |
| 17 | `./` component mid-path in `--source` | Did not break anything. Root derives correctly, printed paths keep the caller's spelling. |
| 18 | Nested `docs/plans/docs/plans` | Resolves nearest-wins to the inner project, as measured. NOT RAISED: the document records nearest-wins as an unevidenced judgement and says explicitly that it "does not settle the NESTED `docs/plans` case on evidence". |
| 19 | Relative `--source` reached through a SYMLINKED current directory | Did not break anything. `getcwd` is physical, so the absolutised log and the canonicalised plan land on the same real prefix and the symlink cancels. Verified: `PWD=/tmp/.../fix/P0-link`, `pwd -P=/tmp/.../fix/P0`, run greens with relative printed paths. |
| 20 | `status` and `next` with NO plan read at all (a Markdown-primary `--source`, no `--plan`) plus an explicit foreign `--metrics` | Prints `plan: not provided` / `metrics: 256 records` and `no active review loop (no plan steps found)`, and increment 2 leaves it unchanged (no checked plan, so no root, so no predicate). NOT RAISED: no pairing is asserted, no loop is derived, and the count is of the file the caller explicitly named. Recorded because it is the one surface state where the specified "where NO plan is read there is no root" escape hatch is NOT backstopped by `validate`'s `(None, None, _)` hard problem. |
| 21 | TOML-primary `--source` in one project beside a `--plan` in ANOTHER | Prints `<foreign plan>: generated projection of a TOML-primary source; skipping the Markdown plan validator` at exit 0, asserting that another project's hand-authored plan is a projection of this one's source, and skipping its validation. NOT RAISED: it is a plan-versus-plan claim, not the plan-versus-log pairing the end property governs, it is pre-existing, and the two-root condition that would catch it was explicitly put to the human and rejected ("A SECOND CONDITION ... rejected as two conditions where one does the work"). |
| 22 | `status --resume` with a good `--source` and a NONEXISTENT `--plan` | BROKE. `R4A-3`. |
| 23 | `status --resume` with a single anchor | Did not break anything. Unchanged by the rule, as the rule says. |
| 24 | The reason vocabulary's precedence and correlation rules, reasoned against the specified variant sets (unsafe-plus-absent on both path fields; metrics unsafe while the loop's absence is step-derived; a consumer joining `log-not-this-project` with `metrics-not-this-project`) | Did not break anything. The precedence rule ("THE UNSAFE VARIANT WINS") and the correlation rule ("WHEN the loop's absence is metrics-derived rather than step-derived") between them resolve every overlap I could construct, including no-steps-plus-unsafe-log and all-terminal-plus-unsafe-log. |

THE MOST DANGEROUS CASE THAT DID NOT BREAK ANYTHING: attack 9, the `..` escape through a nonexistent intermediate component. If it had worked it would have been a clean false green with no unusual layout at all, because the specified resolution deliberately leaves an unresolved lexical tail below the longest existing ancestor, and a containment test on that tail is a pure prefix match that a `..` defeats. It fails only because the operating system will not open such a path, so the tool reads nothing and the guard is never the last line of defence. Worth stating for the implementer: the specified resolution is safe here by an OS property, not by construction, and a future change that normalises the tail lexically before testing must not normalise it into the root.

---

## Scratch hygiene

All probes ran under `TMPDIR=/tmp/claude-1000/r4adv`, created for this review and removed after the evidence above was captured. Directories left in `/tmp`: 0 (the scratch tree under `/tmp/claude-1000/` that the harness provides for this session is not counted; nothing was written to `/tmp` itself).

## Out of scope, confirmed untouched

Accepted costs (i) and (ii) AS STATED (only (ii)'s stated SCOPE is challenged, in `R4A-2`, which the brief permits explicitly); increments 1 and 3; the six human decisions; the present-tense `src/main.rs` claims increment 1 falsified; the `--ledger-fragment` interaction with the resume rule (`R2B-2`); the summary paragraphs naming three decisions; the end-property versus copied-log tension (met head-on in attack 11 and in `R4A-1`'s "why this is not any of the recorded residuals", and deliberately not raised as its own finding). No prose, wording, count, citation-format or enumeration-staleness finding is raised, in line with the lens.
