# Review findings: `workflow-enforcement-tier-inc2`, ROUND 4, ADVERSARIAL CONSTRUCTION lens

LENS. Adversarial construction. Nothing below was concluded by reading the diff. Every claim was produced by building a project layout on disk, running a built binary against it, and recording the streams and the exit code. Every behavioural claim carries a differential against a second binary built from the parent commit, so "this commit changed it" is a measurement and not an inference.

ARTIFACT. Worktree `.claude/worktrees/r4-adversarial`, HEAD `b54ba3a` ("fix: scope the guessed anchor root and split 'missing' from 'cannot tell'"). The round 3 fix alone is `git diff HEAD~1..HEAD`; the whole increment is `git diff main..HEAD`. Specification: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`. All nine prior findings files and the three triages in this directory were read before any fixture was built.

BINARIES. `cargo build --release` at HEAD, and a second build of the tree exported from `HEAD~1` (`git archive HEAD~1 | tar -x -C <S>/r4adv-prev`, then `cargo build --release` there). Below, NEW is HEAD and OLD is `HEAD~1`.

SUITE STATE AT HEAD, measured before any finding was written. `cargo test --release`: 418 passed, 0 failed across 9 binaries (378 + 5 + 1 + 1 + 9 + 3 + 18 + 1 + 2). `cargo clippy --all-targets -- -D warnings`: clean. Every finding below reproduces against that green suite and that clean lint.

WHAT I ATTACKED. The fourth rewrite of the root-supply policy, `resume_roots` (`src/main.rs:1562-1572`), and specifically its two new lines: the `on_disk` filter `anchor.try_exists().unwrap_or(true)` and the `if on_disk.is_empty()` fallback. Reached by `run_status` and `run_next` through `containment_roots` (`src/main.rs:1393`) and by `run_resume` directly (`src/main.rs:1593`). Also the three-way `note_missing_anchors` (`src/main.rs:1116-1130`) that the same commit added, and the agreement of `next`, `next --json`, `status --json`, `status --resume` and `validate --workflow` across fourteen anchor configurations measured on both binaries.

THE HEADLINE, in one sentence. `try_exists().unwrap_or(true)` answers "is there any filesystem object at this path", not "is this anchor usable", so an anchor that IS NOT THERE but whose existence check returns `Err` is classified as ON DISK, and because that classification decides the whole `deciding` set it does the exact opposite of what its own doc comment claims: it REMOVES the other anchor's root rather than adding one. On that path another project's round log is counted and another project's `## RESUME STATE` block is echoed verbatim at exit 0 with both `--json` reason fields `null`, which is byte for byte the output signature of round 2's `R2A-1` (`high`, closed). The trigger needs no privileges: a trailing slash on an existing `--plan` file is enough.

---

## R4A-1: an anchor whose existence check ERRS is counted as on disk, which DROPS the other anchor's root, and another project's log and `## RESUME STATE` block are read at exit 0

SEVERITY: high.

CLAIM. `resume_roots`'s `on_disk` filter treats `try_exists() == Err` as present. A supplied anchor that does not exist and cannot be stat'd therefore makes `on_disk` non-empty, the `on_disk.is_empty()` fallback does not fire, and the OTHER supplied anchor's root is discarded although NO supplied anchor exists. With the two anchors in different projects the surviving root is the wrong project's, and `next`, `next --json`, `status`, `status --json` and `status --resume` all read, count and echo the artifacts of the project the operator did not name, at exit 0, with `metrics_absent_reason` and `resume_state_absent_reason` both `null`. `HEAD~1` refuses all of it on the identical command line, so this is new at `b54ba3a`.

WHY IT IS REACHABLE, and how wide. `Path::try_exists` returns `Ok(false)` only for `ENOENT`; every other `stat` failure is `Err`. Four independent trigger classes were built and all four reproduce, and only the first needs any permission manipulation:

- `EACCES`, an anchor under a directory the process cannot traverse (mode `000`).
- `ENOTDIR`, a path whose component is a regular file, which includes A TRAILING SLASH ON AN EXISTING FILE (`.../b.md/`), the spelling a shell completion or a copy-paste leaves behind.
- `ELOOP`, an anchor path through a symlink loop.
- `ENAMETOOLONG`, an anchor with a component over `NAME_MAX`.

A mode `111` directory (traversable, unreadable) does NOT trigger it: `stat` succeeds and the anchor is correctly classified `Ok(false)`. So the `EACCES` population is precisely "no search permission", not "unreadable".

REPRODUCTION. Self-contained, builds its own two-project fixture, no `scaffold` and no `chmod` required. Save as `repro.sh` and run `bash repro.sh <binary> <a scratch dir outside any repo>`.

```sh
set -eu
BIN="$1"; R="$2"
rm -rf "$R"
mkdir -p "$R/alpha/docs/plans" "$R/alpha/docs/metrics" "$R/beta/docs/plans" "$R/beta/docs/metrics"

printf '[meta]\ntitle = "alpha"\nprimary = "markdown"\n' > "$R/alpha/docs/plans/m.plan.toml"
printf '# alpha ledger\n\n## RESUME STATE\n\nALPHA PRIVATE RESUME STATE.\n\n## NEXT\n' \
  > "$R/alpha/docs/plans/m.ledger.md"
echo '{"type":"round","task":"m","artifact":"s","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"risky"}' \
  > "$R/alpha/docs/metrics/workflow.jsonl"

printf '# beta markdown plan\n\n## Roadmap\n' > "$R/beta/docs/plans/b.md"
printf '# beta ledger\n\n## RESUME STATE\n\nBETA PRIVATE RESUME STATE.\n\n## NEXT\n' \
  > "$R/beta/docs/plans/b.ledger.md"
for i in 1 2 3; do
  echo '{"type":"round","task":"b","artifact":"s","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":'"$i"',"risk_class":"low_risk"}'
done > "$R/beta/docs/metrics/workflow.jsonl"

GHOST="$R/alpha/docs/plans/ghost.plan.toml"   # a --source that does not exist
SLASH="$R/beta/docs/plans/b.md/"              # an existing FILE with a trailing slash: ENOTDIR
PLAIN="$R/beta/docs/plans/nope.md"            # an ordinary nonexistent path: ENOENT

echo "=== CONTROL (ENOENT): both anchors missing, ordinary spellings ==="
"$BIN" next --source "$GHOST" --plan "$PLAIN" \
    --metrics "$R/beta/docs/metrics/workflow.jsonl" \
    --ledger-fragment "$R/beta/docs/plans/b.ledger.md"; echo "exit=$?"

echo "=== ATTACK (ENOTDIR): the ONLY change is a trailing slash on the --plan ==="
"$BIN" next --source "$GHOST" --plan "$SLASH" \
    --metrics "$R/beta/docs/metrics/workflow.jsonl" \
    --ledger-fragment "$R/beta/docs/plans/b.ledger.md"; echo "exit=$?"

echo "=== the machine surface for the attack ==="
"$BIN" next --json --source "$GHOST" --plan "$SLASH" \
    --metrics "$R/beta/docs/metrics/workflow.jsonl" \
    --ledger-fragment "$R/beta/docs/plans/b.ledger.md" 2>/dev/null; echo "exit=$?"

echo "=== status --resume and status --json on the same anchors ==="
"$BIN" status --resume --source "$GHOST" --plan "$SLASH" \
    --ledger-fragment "$R/beta/docs/plans/b.ledger.md"; echo "exit=$?"
"$BIN" status --json --source "$GHOST" --plan "$SLASH" \
    --metrics "$R/beta/docs/metrics/workflow.jsonl" 2>/dev/null; echo "exit=$?"
```

OBSERVED at HEAD `b54ba3a`, verbatim, paths abbreviated to `<R>`.

The CONTROL, which is the correct answer and the behaviour the round 3 fix's own test pins:

```
note: --source <R>/alpha/docs/plans/ghost.plan.toml does not exist
note: --plan <R>/beta/docs/plans/nope.md does not exist
task: ghost
source: no plan source
metrics: unavailable, the round log <R>/beta/docs/metrics/workflow.jsonl is not under the plan's project root <R>/alpha, so its records cannot be paired with this plan

no active review loop (no plan steps found)

the ledger <R>/beta/docs/plans/b.ledger.md is not under the plan's project root <R>/alpha; nothing to resume
exit=0
```

The ATTACK. One character added to the `--plan` path. No file was created, deleted, moved or chmod'd between the two runs:

```
note: --source <R>/alpha/docs/plans/ghost.plan.toml does not exist
note: --plan <R>/beta/docs/plans/b.md/ could not be checked: Not a directory (os error 20)
task: ghost
source: no plan source
metrics: 3 records

no active review loop (no plan steps found)

RESUME STATE (verbatim from the ledger):
## RESUME STATE

BETA PRIVATE RESUME STATE.
exit=0
```

The machine surface, which is the part that matters most because an agent consumes it:

```
{
  "task": "ghost",
  "source": "no plan source",
  "metrics": {
    "records": 3
  },
  "metrics_absent_reason": null,
  "active_loop": null,
  "resume_state": "## RESUME STATE\n\nBETA PRIVATE RESUME STATE.",
  "resume_state_absent_reason": null,
  "no_active_loop_reason": "no-plan-steps"
}
exit=0
```

`status --resume` and `status --json` on the same anchors:

```
note: --source <R>/alpha/docs/plans/ghost.plan.toml does not exist
note: --plan <R>/beta/docs/plans/b.md/ could not be checked: Not a directory (os error 20)
## RESUME STATE

BETA PRIVATE RESUME STATE.
exit=0
{
  "plan": null,
  "metrics": {
    "records": 3
  },
  "metrics_absent_reason": null
}
exit=0
```

THE DIFFERENTIAL, which is what establishes that `b54ba3a` caused it. The identical script against the OLD binary, ATTACK block:

```
note: --plan <R>/beta/docs/plans/b.md/ does not exist
metrics: unavailable, the round log <R>/beta/docs/metrics/workflow.jsonl is not under the plan's project root <R>/alpha, so its records cannot be paired with this plan
the ledger <R>/beta/docs/plans/b.ledger.md is not under the plan's project root <R>/alpha; nothing to resume
"metrics_absent_reason": "log-not-this-project"
"resume_state_absent_reason": "ledger-not-this-project"
exit=0
```

OLD refuses on all five surface invocations. The CONTROL block is byte-identical between the two binaries, so the change is confined to the attack.

THE DEFAULT-PATH VARIANT, so the finding does not rest on explicit artifact flags. No `--metrics`, no `--ledger-fragment`, no `chmod`, the trigger moved to the `--source`:

```
$ agent-scaffold next --source "$R/beta/docs/plans/b.md/" --plan "$R/alpha/docs/plans/ghost.md"
NEW: note: --source <R>/beta/docs/plans/b.md/ could not be checked: Not a directory (os error 20)
     note: --plan <R>/alpha/docs/plans/ghost.md does not exist
     task: b
     metrics: 3 records
OLD: note: --source <R>/beta/docs/plans/b.md/ does not exist
     note: --plan <R>/alpha/docs/plans/ghost.md does not exist
     task: b
     metrics: unavailable, the round log <R>/beta/docs/metrics/workflow.jsonl is not under the plan's
     project root <R>/alpha, so its records cannot be paired with this plan
```

The two-anchor pairing rule `Q-55-resumepairing` decided, which the README states as "a `--source` and a `--plan` naming two different projects reject each other's artifacts", no longer fires at all.

A SECOND, PERMISSION-FREE TRIGGER FOR THE SAME MECHANISM, which is not an `Err` case but an `Ok(true)` one and shows the deciding rule is checking the wrong property either way: an existing DIRECTORY passed as `--plan` also counts as on disk and also discards a missing `--source`'s root. Measured on both binaries, with an explicit foreign `--ledger-fragment`:

```
--plan <R>/beta/docs/plans   NEW: ## RESUME STATE / BETA PRIVATE RESUME STATE.   OLD: nothing to resume
--plan /tmp                  NEW: ## RESUME STATE / BETA PRIVATE RESUME STATE.   OLD: nothing to resume
--plan <a dangling symlink>  NEW: nothing to resume                              OLD: nothing to resume
```

`--plan /tmp` derives the root `/`, under which every path on the machine is contained, so containment is switched off entirely by one existing directory beside one typo. The dangling-symlink row is the control: it is `Ok(false)`, it is correctly dropped, and the refusal stands. TREAT THESE THREE ROWS AS ILLUSTRATION AND NOT AS THE FINDING'S REPRODUCTION: each derives a root that CONTAINS the admitted artifact by layout, so each is separately absorbed by the recorded in-root bound (G-series runs the discriminating control on them). What they add is the mechanism, which is the same one: `deciding` is chosen by a test that answers "is there a filesystem object here", and both a directory and an unstattable path pass it while neither is an anchor. The trailing-slash script above is the reproduction, and it does not rest on any of these rows.

WHY IT MATTERS. Three things, in the order I weigh them.

- IT IS THE OUTPUT SIGNATURE OF A CLOSED `high`. Round 2's `R2A-1` was rated `high` for exactly this: "another project's round log is counted and another project's `## RESUME STATE` block is echoed verbatim at exit 0, with `metrics_absent_reason: null` and `resume_state_absent_reason: null` on the machine surface positively asserting that both are this plan's". Every clause of that sentence is true of the transcript above. The population is narrower than round 2's (round 2 needed one typo, this needs one typo plus one unusual spelling), which is the only reason I do not call it a straight regression of that finding, and it is why a triager may reasonably land on `medium`. I file `high` because the direction is content injection into the surface `next` exists to drive an agent with, and because `task: ghost` (derived from the ALPHA source) sits three lines above BETA's private resume state in the same output.
- THE DOC COMMENT'S SAFETY ARGUMENT IS THE OPPOSITE OF THE MEASUREMENT. `src/main.rs:1557-1561` justifies the chosen direction with "of the two directions only this one can add a root rather than remove one". Treating the unstattable anchor as present is precisely what REMOVES the other anchor's root, because presence is what populates `on_disk` and a non-empty `on_disk` suppresses the fallback. The argument that authorised the line is falsified by the line's own behaviour. That is filed separately as R4A-3 because it needs correcting whatever the human decides about the behaviour.
- NOTHING PINS IT. The commit's own new test `an_anchor_that_cannot_be_checked_is_not_reported_as_missing` (`tests/unsafe_pairings_are_refused_and_omitted.rs:949`) passes ONE anchor, and asserts only on `stderr`: it checks the note's wording and nothing about which roots the run derived. `a_missing_anchor_does_not_overrule_an_anchor_that_exists` (`:851`) varies only `ENOENT` versus written. No test in the file passes an anchor whose existence check errs alongside a second anchor, so the entire `unwrap_or(true)` branch of the deciding rule is unguarded.

---

## R4A-2: an anchor that is not on disk still vetoes an anchor that is, whenever its existence check errs, so R3A-1 is not closed for that population

SEVERITY: medium.

CLAIM. The narrowing this commit added ("a guess does not overrule an anchor that is on disk") is keyed on `try_exists() == Ok(false)`, so it does not apply to an anchor that is absent for any other reason. Such an anchor still contributes a second containment root beside the root of a `--source` that does exist, and since containment requires the artifact to be under EVERY root, a project loses its OWN default log and its OWN default ledger with `log-not-this-project` and `ledger-not-this-project` asserted about them. That is R3A-1 verbatim, which this commit was written to close, surviving on the population R3A-2 was filed about. It is UNCHANGED from `HEAD~1`, so it is a residual and not a regression.

REPRODUCTION. Continues the same fixture as R4A-1, so `$R`, `$SLASH` and `$PLAIN` are as defined there. Three runs, two of them controls.

```sh
echo "=== CONTROL: alpha --source that EXISTS, no --plan ==="
"$BIN" next --source "$R/alpha/docs/plans/m.plan.toml"; echo "exit=$?"
echo "=== CONTROL: the same, plus an ORDINARY nonexistent beta --plan (the round 3 fix working) ==="
"$BIN" next --source "$R/alpha/docs/plans/m.plan.toml" --plan "$PLAIN"; echo "exit=$?"
echo "=== ATTACK: the same, with the trailing slash instead ==="
"$BIN" next --source "$R/alpha/docs/plans/m.plan.toml" --plan "$SLASH"; echo "exit=$?"
"$BIN" next --json --source "$R/alpha/docs/plans/m.plan.toml" --plan "$SLASH" 2>/dev/null
```

OBSERVED at HEAD. Both controls print alpha's own log and alpha's own block:

```
task: m
metrics: 1 records
RESUME STATE (verbatim from the ledger):
## RESUME STATE

ALPHA PRIVATE RESUME STATE.
exit=0
```

and the second control additionally prints `note: --plan <R>/beta/docs/plans/nope.md does not exist`. The ATTACK, same `--source`, same files on disk:

```
note: --plan <R>/beta/docs/plans/b.md/ could not be checked: Not a directory (os error 20)
task: m
source: no plan source
metrics: unavailable, the round log <R>/alpha/docs/metrics/workflow.jsonl is not under the plan's project root <R>/beta, so its records cannot be paired with this plan

no active review loop (no plan steps found)

the ledger <R>/alpha/docs/plans/m.ledger.md is not under the plan's project root <R>/beta; nothing to resume
exit=0
```

```
  "metrics_absent_reason": "log-not-this-project",
  "resume_state_absent_reason": "ledger-not-this-project",
```

THE SAME-PROJECT VARIANT, so the finding does not depend on a two-project fixture at all. One project, its own log, an `EACCES` trigger inside it:

```
$ mkdir -p "$R/alpha/locked" && chmod 000 "$R/alpha/locked"
$ agent-scaffold next --source "$R/alpha/docs/plans/m.plan.toml" --plan "$R/alpha/locked/nope.md"
note: --plan <R>/alpha/locked/nope.md could not be checked: Permission denied (os error 13)
metrics: unavailable, the round log <R>/alpha/docs/metrics/workflow.jsonl is not under the plan's
         project root <R>/alpha/locked, so its records cannot be paired with this plan
--- control, the same --plan in an ORDINARY directory of the same project: metrics: 1 records
--- control, no --plan at all:                                            metrics: 1 records
```

The printed root `<R>/alpha/locked` is a directory that exists but contains no plan and is not a project root.

WHY IT MATTERS. R3A-1 was triaged VALID at `medium` on three grounds, and all three still apply here without alteration: the human's own `Q-55-emptyroot` text declined an option because it "would also omit an artifact legitimately belonging to the anchor's own directory", the machine reason is a positive false assertion about the project's own log rather than a silence, and the trigger is an operator who has already made one mistake. The fix pass took the triage's Option B and implemented it against `Ok(false)` alone; the same doc comment then declares the other case safe on an argument R4A-3 shows to be wrong. I hold it at `medium` rather than lower because it is the recurrence of a `medium` the round was spent on, and rather than higher because the direction is a refusal at exit 0 with a `note:` printed, which is fail-safe.

---

## R4A-3: the doc comment's stated reason for the chosen direction is false, and the README and CHANGELOG sentences this commit added are falsified by the same runs

SEVERITY: low.

CLAIM. Three texts added or rewritten by `b54ba3a` state a rule the binary does not implement, and one of them is the safety argument that authorises the line R4A-1 turns on.

- `src/main.rs:1557-1561`: "AN ANCHOR WHOSE EXISTENCE CANNOT BE DETERMINED COUNTS AS EXISTING ... Guessing the other way would drop its root on the strength of an error, and of the two directions only this one can add a root rather than remove one." The final clause is false. Counting such an anchor as existing populates `on_disk`, a non-empty `on_disk` suppresses the `supplied` fallback, and the OTHER anchor's root is dropped. R4A-1's attack block is the measurement: the alpha root present at `HEAD~1` is gone at HEAD.
- `README.md:236`: "every `--source` or `--plan` you gave THAT IS ON DISK yields one" and "An anchor that is not on disk yields a root only when NO anchor you gave is on disk". In R4A-1's attack NEITHER anchor is on disk and exactly one yields a root, so the second sentence is false as written; and the anchor that yields it is not on disk, so the first is false too.
- `CHANGELOG.md:23`: the same two clauses, in the same words.

REPRODUCTION. R4A-1's script is the reproduction; no additional run is needed. `<R>/beta/docs/plans/b.md/` is not on disk (`stat` fails, `agent-scaffold` itself says "could not be checked"), `<R>/alpha/docs/plans/ghost.plan.toml` is not on disk, and the derived root is `<R>/beta`.

WHY IT MATTERS. The doc comment is not commentary here, it is the recorded justification for a specific choice between two directions, and a later reader deciding whether the line may be changed will read it as the reason not to. The README and CHANGELOG clauses are the user-facing contract for the behaviour, and they are the two sites this same commit edited to make them true after the narrowing. The class matches `R3ACC-1` and `R3F-2`, both held at `low`. NOTE FOR THE FIX PASS: if R4A-1's behaviour is repaired so that only anchors that genuinely exist populate `on_disk`, the README and CHANGELOG sentences become true as they stand and only the doc comment's final clause needs rewriting; if the behaviour is accepted and recorded instead, all three sites need the "cannot be determined" case spelled out.

---

## ATTACKS THAT FAILED

Everything below was built and run and did NOT break the increment. The negative results are scoped to the dimensions the fixtures varied and to the controls named against each.

A-SERIES, THE ROUND 3 DEFECT ITSELF. R3A-1 is closed on its own population, measured against the control the round 3 triage identified as the deciding one (C0, no `--plan` supplied at all), not merely against the plan-present control that could not show it. `next --source <alpha m.plan.toml, exists> --plan <beta>/docs/plans/nope.md` now prints `metrics: 1 records` and ALPHA's block, byte-identical to the same command with no `--plan` at all, where `HEAD~1` printed `metrics: unavailable` and refused alpha's ledger. `status --resume` and `next --json` agree. The `ENOTDIR`/`EACCES` residual is R4A-2 and is a different population, not a failure of this closure.

B-SERIES, CAN THE INTERSECTION ADMIT A FOREIGN ARTIFACT? Not by adding roots, and I could not build one. Containment over N roots requires the artifact under all of them, so adding a root can only narrow. What CHANGED at this commit is that roots can now be REMOVED, and removal is the only widening move available; every widening I found is R4A-1's mechanism. Two anchors that BOTH exist in disjoint projects still reject each other's artifacts (fixture: alpha Markdown-primary `--source` that exists plus an existing beta `--plan`, explicit beta `--metrics` and `--ledger-fragment`), which is `Q-55-endproperty` rooting on the plan actually read and is unchanged between the binaries.

C-SERIES, PATHOLOGICAL ANCHORS, each run as the `--plan` beside an EXISTING alpha `--source` with an explicit foreign `--metrics`, and again beside a MISSING alpha `--source` with an explicit foreign `--ledger-fragment`.

- A dangling symlink: `Ok(false)`, note says "does not exist", correctly dropped, foreign artifacts refused, identical on both binaries.
- A mode `111` directory (traversable, unreadable) in the path: `stat` succeeds, `Ok(false)`, correctly classified. This is the control that bounds the `EACCES` population to "no search permission" rather than "unreadable".
- An anchor that exists but is EMPTY or is not a plan (`: > empty.plan.toml`): counts as on disk, decides, root correct, alpha's own log read. Correct.
- `/dev/null`: parses as empty and Markdown-primary, root `/dev`, alpha's own ledger and beta's log both refused. Unchanged from round 3.
- An anchor that is a DIRECTORY beside an EXISTING `--source`: `next` and `status` still fail with `IsADirectory` at exit 1 before any projection, unchanged. Beside a MISSING `--source` it is R4A-1's second trigger and is filed there.
- An empty anchor (`--plan ""`): rejected by clap before any code under review runs. Not a route.

D-SERIES, THE FIVE SURFACES DIFFED AGAINST EACH OTHER ON ONE FIXTURE, fourteen anchor configurations crossing {`--source` exists / missing / unstattable / absent} with {`--plan` exists / missing / unstattable / absent}, each measured on BOTH binaries with the same explicit foreign `--metrics` and `--ledger-fragment`. Columns are 1 when the surface admits beta's artifact.

| configuration | n-log | n-led | s-log | r-led | validate |
| --- | --- | --- | --- | --- | --- |
| S1 src alpha EXISTS, no plan | 0 | 0 | 0 | 0 | 1 |
| S2 src alpha EXISTS, plan beta EXISTS | 1 | 1 | 1 | 0 | 1 |
| S3 src alpha EXISTS, plan beta MISSING | 0 | 0 | 0 | 0 | 1 |
| S4 src alpha EXISTS, plan beta UNSTATTABLE | 0 | 0 | 0 | 0 | 1 |
| S5 src alpha MISSING, no plan | 0 | 0 | 0 | 0 | 1 |
| S6 src alpha MISSING, plan beta EXISTS | 1 | 1 | 1 | 1 | 1 |
| S7 src alpha MISSING, plan beta MISSING | 0 | 0 | 0 | 0 | 1 |
| S8 src alpha MISSING, plan beta UNSTATTABLE | 1 | 1 | 1 | 1 | 1 |
| S9 src alpha UNSTATTABLE, no plan | 0 | 0 | 0 | 0 | 1 |
| S10 src alpha UNSTATTABLE, plan beta MISSING | 0 | 0 | 0 | 0 | 1 |
| S11 src alpha UNSTATTABLE, plan beta EXISTS | 1 | 1 | 1 | 0 | 1 |
| S12 no src, plan beta MISSING | 1 | 1 | 1 | 1 | 1 |
| S13 no src, plan beta UNSTATTABLE | 1 | 1 | 1 | 1 | 1 |
| S14 no anchors at all | 1 | 1 | 1 | 1 | 1 |

EXACTLY TWO CELLS MOVED between `HEAD~1` and HEAD, and I name the baseline explicitly because round 3's own lesson was that a coverage claim is worth only the dimensions it crosses and only the control it is taken against. The baseline here is the SAME fourteen rows run on the `HEAD~1` binary against a freshly rebuilt copy of the same fixture, which is a baseline that can show the change because it varies the one dimension the fix touched.

- S8, all four columns, `0/0/0/0` to `1/1/1/1`. That is R4A-1.
- S6, the `r-led` column alone, `0` to `1`. A typo'd `--source` beside an EXISTING foreign `--plan` now lets `status --resume` echo a `--ledger-fragment` from the `--plan`'s project. I do NOT file it: it is the decided narrowing applied literally (the anchor on disk decides, and the admitted ledger is that anchor's own project's), and its effect is to make `status --resume` AGREE with `next`, which admitted the same ledger at `HEAD~1` for the `Q-55-endproperty` reason. The movement is toward agreement and away from round 1's ADV-1 class, so recording it is the right treatment, not filing it.

The one-way strictness property round 3 established still holds at HEAD, checked two ways. By measurement: no row has `r-led` greater than `n-led`, so `status --resume` never accepts what `next` refuses. By reading: `checked_plan_root` returns a root only when the checked plan CANONICALISES, which implies `try_exists` is `Ok(true)`, which puts that anchor in `on_disk`, so `[checked_plan_root]` is still a subset of `resume_roots` even after the deciding-set narrowing. I could not construct a path that canonicalises while `try_exists` errs, or the reverse; `EACCES`, `ENOTDIR` and `ELOOP` fail both calls together. The `validate` column is uninformative in this fixture (a Markdown-primary `--source` with no readable `--plan` reaches its own `(None, None, _)` arm) and `validate --workflow`'s non-use of `containment_roots` is out of scope by the brief.

E-SERIES, THE NOTE. Beyond what R4A-3 says about its doc comment, I could not break it.

- The `Err` arm is REACHABLE and its text is CORRECT in all four trigger classes: "could not be checked: Permission denied (os error 13)", "Not a directory (os error 20)", "Too many levels of symbolic links (os error 40)", "File name too long (os error 36)".
- It never says "does not exist" about something that exists. The round 3 finding R3A-2's own fixture (a plan on disk under a mode `000` directory) now prints "could not be checked", and the trailing-slash case, which OLD called "does not exist" for a file that is there, now also prints "could not be checked".
- It cannot contaminate `--json`. With the `Err` arm firing, `next --json` stdout parses on its own and `jq -r '.task, .metrics_absent_reason'` returns `m` and `log-not-this-project`; the note is the only line on stderr.
- It does not fire when nothing is missing: every control run in this review with both anchors present printed no `note:` line.
- It fires once per anchor, in flag order, on `next`, `status`, `status --json` and `status --resume` alike, and both lines appear when both anchors are affected (R4A-1's attack transcript shows one of each on one run).
- It is HONEST in R4A-1's attack: both notes are printed and both are true. The leak is not a silent one on the human surface. It IS silent on `--json`, where the reason fields say `null`, which is the ground round 2 filed its own `high` on.

F-SERIES, THE RECORDED SINGLE-ANCHOR `..` RESIDUAL. Not re-raised, and I looked for a case outside its stated bound as the brief permits. Beside a SECOND anchor that exists, the ghost-`..` anchor is dropped exactly as the doc comment claims, and the run's remaining failure (`metrics: no log found`) comes from the LEXICAL default resolution being unopenable rather than from containment, which is a different mechanism and is not a containment verdict at all. Beside a second anchor that is UNSTATTABLE, the ghost-`..` anchor is also dropped, so the residual is not reachable through R4A-1's mechanism either. I found no `..` case outside the recorded bound and file none.

G-SERIES, THE IN-ROOT BOUND, WITH THE DISCRIMINATING CONTROL RUN AND THEN REBUILT. One construction of mine touches the bound and I attribute it there, having run the control rather than asserted it, and having thrown the first control away because it could not discriminate.

A `--plan` naming an existing DIRECTORY derives the root from that directory's PARENT, so a directory high in the tree switches containment off over a wide subtree. Beside a `--source` that does not exist it is the sole root, and an explicit beta `--ledger-fragment` under it is echoed. My first control was mis-built: I put the `--plan` at the fixture root itself, which derives the root ONE LEVEL ABOVE the fixture, and then placed the "disjoint" beta in a sibling fixture directory that was still under that root, so the disjoint arrangement reproduced too and proved nothing. Rebuilt so the derived root is the fixture root itself (`--plan <D>/holder`, an empty directory, root `<D>`), the two arrangements separate cleanly:

```
NESTED   (--plan <N>/holder, beta at <N>/beta, INSIDE the derived root)
  -> ## RESUME STATE / BETA PRIVATE RESUME STATE.
DISJOINT (--plan <D>/holder, beta moved OUTSIDE <D>, nothing else changed)
  -> the ledger <S>/r4adv-d2beta/docs/plans/b.ledger.md is not under the plan's project root <S>/r4adv-d2;
     nothing to resume
```

The disjoint arrangement does not reproduce, so THIS construction is the recorded bound and I do not file it. I record the mis-built first attempt because it is the same failure round 3's triage diagnosed: a control that cannot show the change is worse than no control, since it produces a confident negative.

R4A-1 IS NOT ATTRIBUTED TO THE BOUND, and the discrimination is not a judgement call. Its fixture is two TOP-LEVEL SIBLINGS with no containment relation between them; the `<R>` root that would cover both is never derived; the surviving root is `<R>/beta` while the `--source` names `<R>/alpha`; and the control that separates it holds the ENTIRE LAYOUT FIXED and varies only the STAT CLASS of one anchor, `ENOTDIR` against `ENOENT`, one character in the command line. A layout-dependent explanation, nesting included, cannot survive a control in which no file moves. The `--plan /tmp` and `--plan <a directory>` rows shown under R4A-1 ARE in-root by construction, which is why they are presented there as illustrations of how far the mechanism reaches and never as the finding's reproduction; the reproduction is the trailing-slash script, which does not depend on them.

DIMENSIONS THE FIXTURES VARIED. Which anchors are supplied (`--source` only, `--plan` only, both, neither); the STAT CLASS of each anchor independently (present, `ENOENT`, `EACCES`, `ENOTDIR` including the trailing-slash spelling, `ELOOP`, `ENAMETOOLONG`, dangling symlink, mode `111` ancestor); which project each anchor names (same, disjoint sibling, parent directory, `/`); whether the `--source` is TOML-primary, Markdown-primary, empty, a device or a directory; whether each artifact is defaulted or named explicitly; both flag positions and both orders for every anchor pairing; all five surfaces (`next`, `next --json`, `status`, `status --json`, `status --resume`, plus `validate --workflow` for the exit code); and both binaries on every configuration that produced a claim.

DIMENSIONS THEY DID NOT VARY, so no negative result above should be read past them. Symlinked project layouts beyond a dangling link and a loop, in particular a symlinked `docs/plans`, which is accepted cost (ii) and which I did not attack. Nested projects, except in the one G-series discriminating control; every other fixture is top-level siblings. Relative anchor spellings and the process working directory: every run above used absolute paths from a directory outside both projects, so nothing here re-tests the CWD dimension round 3 covered. Plans with more than one step or steps in other states, so nothing here exercises the loop projection's arithmetic. Logs longer than three records, or with malformed lines. Concurrent modification between the `try_exists` call in `note_missing_anchors` and the second one in `resume_roots`, which is a real TOCTOU window I did not attempt to win. Non-UTF-8 path components. Multiple mount points, bind mounts and hard links. Windows and macOS path semantics; everything here is Linux, and the `Err` classification of `try_exists` is where platform differences would land.

NOT RAISED, per the brief: the in-root bound (control run and reported in G-series), `ADV-2`, `R2A-2`, `R2C-2`, the stale owed-demonstration count, the `Q-55-emptyroot` fix site, the single-anchor `..` residual (F-series), the four accepted costs, `validate --workflow`'s non-use of `containment_roots`, project identity, line length, prose wrapping, and the specification's own wording.
