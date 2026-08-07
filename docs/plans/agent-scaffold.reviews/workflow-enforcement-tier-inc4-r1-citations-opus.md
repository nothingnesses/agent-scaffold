# `workflow-enforcement-tier-inc4`, round 1, reviewer A: citations, quotations and re-measurement

Diff range `363ac06..079d63f`, two commits, seven files. Lens: mechanical. Does every citation resolve, does every quotation match (or correctly fail to match) the file it is attributed to, and does every measured claim reproduce.

Everything below was run in the review worktree `.claude/worktrees/rev-inc4-r1-a` at `079d63f`, against a debug build of that tree. Fixtures were built under `<scratchpad>/rev-inc4-r1-a/` only. Every mode change was restored.

## Summary

Eight findings: three `medium`, five `low`. No `high` and no `critical`; I looked for one and did not find one, which is what I would expect from an increment that ships one doc comment and no behaviour.

- `R1A-1` (medium): the re-derived scratch-helper census in `checks-runner-worktree-name-collision.md` omits the two sites this step's own inc1 and inc2 created, and mis-describes them.
- `R1A-2` (medium): the recorded `#[serde(skip)]` negative result is now false on both halves, and its named site is wrong.
- `R1A-3` (medium): the `Q-55-jsonreason` problem statement asserts two facts in the present tense that inc2 falsified.
- `R1A-4` (low): two items of the four-bullet doc-claim list keep present-tense verbs for defects inc2 fixed, while the fifth item in the same list was re-tensed.
- `R1A-5` (low): one quotation re-tensed as historical still matches the tree literally.
- `R1A-6` (low): acceptance check 16's trailing-slash cell quotes a command line that does not reproduce its quoted output.
- `R1A-7` (low): the enclosing `Projection` doc comment's "a missing plan" clause does not name every route to `plan: null`; the implementer's argument holds for two routes and not the third.
- `R1A-8` (low): the one input on which `status` does fail rather than project a partial result, offered as information and possibly outside this step.

The re-measurement of acceptance check 16 (part C of the brief) found NOTHING. Every assertion in it reproduces, including the root behaviour. That result is set out in full below rather than asserted, because a clean check is worth what it actually ran.

---

## `R1A-1` (medium): the scratch-helper census omits the two sites this step created, and mis-describes them

`docs/plans/agent-scaffold.steps/checks-runner-worktree-name-collision.md`, the paragraph beginning "Checked and NOT affected". The pass rewrote this sentence: it re-pointed `src/main.rs:1726-1731` to `:2280-2285`, deleted the stale count word "Three", and expanded the `{pid}-{nanos}` enumeration from three entries to five. The expanded enumeration is still short by two, and both missing entries were created by this very step.

The sentence as it now stands makes two claims:

1. "the other scratch helpers discriminate by a per-test literal name rather than by the clock and so are unique by construction (`src/checks.rs:1037-1046`, `src/main.rs:2280-2285`, `src/manifest.rs:552-558`, `src/plan/render.rs:638`, `tests/audit_command.rs:20`, `tests/scaffold_precommit_hook.rs:14`, `tests/checks_staged_hook_env.rs:50`)".
2. "Integration-test sites do use `{pid}-{nanos}` (`tests/validate_workflow_toml_source_needs_no_plan.rs:97`, `:129`, `:190` and `:287`, `tests/validate_toml_primary_skips_markdown_plan.rs:74`)".

The census of `{pid}-{nanos}` sites in `tests/`:

```
$ grep -rn 'as_nanos' tests/
tests/validate_toml_primary_skips_markdown_plan.rs:77
tests/metrics_and_ledger_anchor_to_the_plan_source.rs:68
tests/validate_workflow_toml_source_needs_no_plan.rs:100
tests/validate_workflow_toml_source_needs_no_plan.rs:132
tests/validate_workflow_toml_source_needs_no_plan.rs:193
tests/validate_workflow_toml_source_needs_no_plan.rs:290
tests/unsafe_pairings_are_refused_and_omitted.rs:90
```

Seven sites across four files. The sentence names five across two. The two missing ones are:

- `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:63-69`, `fn scratch(name)`, building `agent-scaffold-anchor-{name}-{pid}-{nanos}`.
- `tests/unsafe_pairings_are_refused_and_omitted.rs:85-91`, `fn scratch(name)`, building `agent-scaffold-containment-{name}-{pid}-{nanos}`.

Both files were created by this step:

```
$ git log --oneline --diff-filter=A -- tests/metrics_and_ledger_anchor_to_the_plan_source.rs tests/unsafe_pairings_are_refused_and_omitted.rs
8beb1c2 feat: refuse and omit on a round log or ledger the plan cannot vouch for   (inc2)
609ddcf fix: anchor the metrics log and the ledger to the plan source              (inc1)
```

Claim 1 is also wrong about them if they are read into it instead: both DO discriminate by a per-test literal name, and both ALSO use the clock, so "rather than by the clock" is false of them. They fall between the sentence's two lists rather than into either.

The verdict the paragraph reaches is not affected: `agent-scaffold-anchor-` and `agent-scaffold-containment-` are distinct literal prefixes, so "each carries a distinct literal prefix, so they cannot collide today" still holds over the full set of seven. This is an incomplete census, not a wrong conclusion, which is why it is `medium` and not higher.

In scope on the increment's own terms: acceptance check 21b says "every `src/main.rs` and `tests/` citation in the three is opened at its cited range and shown to hold its named subject", and the `Q-55-currencyscope` boundary is drift THIS step caused, which these two sites are by construction.

## `R1A-2` (medium): the recorded `#[serde(skip)]` negative result is now false on both halves

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:206`, the paragraph "WHAT THE SWEEP FOUND NOTHING OF, stated because a negative result is worth recording", which the pass left untouched:

> `#[serde(skip)]` appears exactly ONCE in the whole of `src/`, at `src/next.rs:NextProjection::no_active_loop_reason`, so there is no second silently-dropped field anywhere.

Measured:

```
$ grep -rn 'serde(skip' src/
src/next.rs:198:    #[serde(skip)]
src/next.rs:202:    #[serde(skip)]
```

Three separate errors in one sentence, all of them caused by this step's inc2:

- It appears TWICE, not once.
- Neither occurrence is on `no_active_loop_reason`. That field is at `src/next.rs:192`, carries no `#[serde(skip)]`, and its own doc comment at `:189-191` now says the opposite: "Serialised: `--json` is what an agent reads".
- The two occurrences ARE second (and third) silently-dropped fields: `metrics_absent_note` (`src/next.rs:199`) and `resume_state_absent_note` (`src/next.rs:203`), both added by inc2.

The next clause of the same sentence, "No `skip_serializing_if` appears in either `src/next.rs` or `src/main.rs`", is still true: `grep -rn 'skip_serializing_if' src/` hits only `src/plan/source.rs`. So the paragraph is half current and half inverted, which is worse than either.

This is exactly what acceptance check 21 promises does not remain ("EVERY CITATION AND EVERY QUOTATION IN THIS FILE RESOLVES"), and it is the failure mode the increment's own risk classification names first: a negative result that was measured once and is no longer true.

## `R1A-3` (medium): the `Q-55-jsonreason` problem statement asserts two facts inc2 falsified

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:195`, untouched by the pass:

> `no_active_loop_reason` is `#[serde(skip)]` (`src/next.rs:NextProjection::no_active_loop_reason`) and `status`'s `Projection` has no reason field at all, so under `--json` an omitted part serialises as a bare `null` with nothing distinguishing why.

Both halves are false in the tree at `079d63f`.

- `src/next.rs:189-192`: `no_active_loop_reason` has no `#[serde(skip)]` and is documented as serialised (see `R1A-2`).
- `src/main.rs:575-577`: `Projection` carries `metrics_absent_reason: Option<next::MetricsAbsentReason>`.

Measured on the binary rather than read:

```
$ agent-scaffold status --json --source docs/plans/p.plan.toml
{
  "plan": { "steps": [ { "slug": "only-step", "status": "not started" } ], "open_questions": [] },
  "metrics": { "records": 1 },
  "metrics_absent_reason": null
}
```

The sentence's framing, "THE PROBLEM, in the form that decided it", is a partial defence: the paragraph describes the state that motivated the decision. It is not a sufficient one here, because the pass applied the opposite standard to every neighbouring paragraph in the same section. `:189` was re-tensed ("ALREADY CARRIED", "ALREADY RENDERED"), `:204` was re-tensed ("SAID"), `:208` was re-tensed ("WAS UNGUARDED", "FAILED nothing"), `:225` was re-tensed ("WERE already distinguished", "COLLAPSED", "COST"). `:195` is the one paragraph in that run left in the present tense, and it is the one whose present-tense content is now flatly wrong.

## `R1A-4` (low): the four-bullet doc-claim list keeps present-tense verbs for defects inc2 fixed

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:199-202`. Two of the four bullets use prospective verbs, which are fine as a specification of work owed: "BECOMES FALSE and must change" (`:199`), "BECOMES INCOMPLETE" (`:200`). The other two use present-tense verbs for a state that no longer exists:

- `:201`, "`status`'s `Projection` doc comment (`src/main.rs:Projection`) HAS THE SAME DEFECT: "Every part is optional so a missing plan or metrics file yields a partial projection rather than a failure"".
- `:202`, "`resume_state`'s doc comment (`src/next.rs:NextProjection::resume_state`) IS SHORT BY ONE IN THE SAME WAY: "or `None` when the ledger is absent or carries no such section"".

Neither quotation is in the tree:

```
$ grep -Fn 'Every part is optional so a missing plan or metrics file' src/main.rs   -> no output
$ grep -Fn 'None` when the ledger is absent or carries no such section' src/next.rs -> no output
```

The current texts are `src/main.rs:561-567` ("a missing plan, a missing metrics file, or a metrics file that cannot be paired with this plan") and `src/next.rs:184-186` ("absent, carries no such section, or is not this plan's"). Both defects are fixed.

What makes this a finding rather than a voice choice is that the very next paragraph, `:204`, is the fifth item of the same sweep and the pass DID re-tense it ("`active_loop`'s doc comment ... SAID it was `None` when ..."). So the pass applied its own rule inside this list and stopped after one item. That same paragraph is now internally mixed as a result: a past-tense comment ("SAID"), a present-tense code claim ("returns `Some(...)`"), and a present-tense conclusion ("the comment is the thing that is wrong"), where the comment in question has since been rewritten to `src/next.rs:181-182`.

`low` because a reader reaches the right facts either way; the bullets are visibly a record of work that has since been done.

## `R1A-5` (low): a quotation re-tensed as historical still matches the tree

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:129`, changed by the pass from "IT DOES NOT. It assumes ..." to:

> IT DID NOT. It ASSUMED THE CURRENT DIRECTORY WAS THE ROOT (`PathBuf::from(format!("docs/plans/{task}.ledger.md"))`, `src/main.rs:default_ledger_path`) and BROKE as soon as it was not.

The re-tensing is right about the function and wrong about the fragment, which is still there:

```
$ grep -Fn 'PathBuf::from(format!("docs/plans/{task}.ledger.md"))' src/main.rs
1544:        || PathBuf::from(format!("docs/plans/{task}.ledger.md")),
```

`src/main.rs:1538-1547` is `default_ledger_path`, and `:1544` is the `map_or_else` default arm. The spelling survives deliberately as the no-anchor fallback, documented three lines above it at `:1535-1537`: "With NEITHER a `--source` nor a `--plan` there is no directory to sit beside, so the historical current-directory-relative `docs/plans/<task>.ledger.md` stands".

So a reader who follows the citation to check the past-tense claim finds the quoted code present and still assuming the current directory is the root, and has to work out unaided that it now only does so when no anchor was given. The accurate correction is one clause; the current text reads as though the fragment went away.

`low`: the substantive claim (the function no longer assumes it in general) is true.

## `R1A-6` (low): check 16's trailing-slash cell quotes a command that does not reproduce its output

Acceptance check 16 says, of both `Err`-arm spellings:

> MEASURED at uid 1000 on both spellings: plain `validate --source docs/plans/p.plan.toml` exits 0 with `no metrics log at <the path as given>; nothing to validate`

For the mode-600 spelling that command line is complete and reproduces. For the trailing-slash spelling it is not, because the trailing slash lives in a `--metrics` that the quoted command omits. On the trailing-slash fixture, run literally:

```
$ agent-scaffold validate --source docs/plans/p.plan.toml
docs/metrics/workflow.jsonl: 1 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
exit: 0
```

That is a green record count, not the quoted absence note, because with no `--metrics` the anchored default resolves to the real readable log. The command that does reproduce is:

```
$ agent-scaffold validate --source docs/plans/p.plan.toml --metrics docs/metrics/workflow.jsonl/
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
no metrics log at docs/metrics/workflow.jsonl/; nothing to validate
exit: 0
```

"on both spellings" and "<the path as given>" carry the `--metrics` for a careful reader, so this is a compression rather than a false claim. It is still a finding under check 21's own standard, that the check is "mechanical rather than a reading" and check 16's own framing that a round is settled by running it: the one cell in the diff that a reviewer is told to reproduce by copying a command line cannot be reproduced by copying that command line.

`low`, and the fix is four words.

## `R1A-7` (low): the enclosing `Projection` doc comment does not name every route to `plan: null`

The brief asks whether the implementer's argument holds for leaving `src/main.rs:561-567` unchanged, on the ground that its "a missing plan" clause names one cause that both populate-routes satisfy.

The routes, read off `src/main.rs:1202-1219` and `toml_source` at `:1105-1128`. `plan` is `Some` when `toml_source(&args.source)` yields a source that parses AND declares `[meta].primary = "toml"`, else when `--plan` names a path that `exists()` and reads. `plan` is `None` otherwise. Measured, all at exit 0:

```
$ agent-scaffold status --json --source docs/plans/p.plan.toml            # TOML-primary, no --plan
"plan": { "steps": [ ... ], "open_questions": [] }

$ agent-scaffold status --json --source docs/plans/md.plan.toml           # Markdown-primary, no --plan
"plan": null

$ agent-scaffold status --json --source docs/plans/broken.plan.toml       # fails to parse, no --plan
note: --source docs/plans/broken.plan.toml did not parse as a `<task>.plan.toml`; projecting from --plan
"plan": null
```

The corrected field comment at `:570` is TRUE and complete on the positive direction: both populate-routes are named, and the TOML-primary route is measured by acceptance check 22. It does not state that the TOML-primary `--source` WINS when both are supplied, which the code decides at `:1203`; that is a gap but a small one, and the same level of detail as the `metrics` field beside it.

The argument for leaving `:561-567` holds for two of the three absent-routes and not the third. "a missing plan" fairly names "nothing was supplied" and fairly names "a Markdown-primary `--source` and no `--plan`" (no plan was supplied to project from). It does NOT name the parse-failure route: there the `--source` is PRESENT and MALFORMED, the tool says so on stderr, and the projection is still partial at exit 0. `README.md:238` distinguishes exactly this ("never fails on a missing OR MALFORMED file"); the struct comment says only "missing". So the enclosing comment is short by one in the same way the four bullets at `:199-202` were, on the route the corrected field comment newly makes reachable through `--source`.

`low`: no reader is misled about behaviour, and this may reasonably be judged inside the declined `Q-55-currencyscope` boundary rather than inside inc4. Reported because the brief asked for the judgement.

## `R1A-8` (low): the one input where `status` fails rather than projects, offered as information

Pre-existing and possibly outside this step; recorded because it is the only measured counterexample to "yields a partial projection rather than a failure" in the comment `R1A-7` is about, and because the corrected sentence's word "readable" is the only place the docs touch it.

```
$ chmod 000 docs/plans/p.md
$ agent-scaffold status --json --plan docs/plans/p.md
Error: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }
exit: 1
```

`src/main.rs:1211`'s `fs::read_to_string(path)?` propagates out of `run_status`. The corrected sentence does not assert what an unreadable `--plan` does, so it is not falsified; the enclosing comment's "rather than a failure" and `README.md:238`'s "Unlike `validate` it never fails" both read as though it could not happen. It is the same shape as the plain-`validate` mode-000-file residual the ledger already routed to the validation-constraints step, and it belongs there rather than here. Raised so a later reader does not have to rediscover it.

---

## Part C: re-measurement of acceptance check 16, in full. NOTHING FOUND.

Fixture built by hand from `PLAN_TOML` and `ROUND_RECORD` (`tests/validate_workflow_toml_source_needs_no_plan.rs:37-47` and `:70-74`): `docs/plans/p.plan.toml` plus a REAL one-record `docs/metrics/workflow.jsonl`. Binary is this worktree's debug build at `079d63f`. Every mode restored after every cell.

THE PROBE, verified rather than assumed. At mode 600 on `docs/metrics`, `ls docs/metrics/workflow.jsonl` gives "Permission denied", so the directory is readable and not searchable and the probe reaches its `Err` arm.

CELL 1, unsearchable directory, uid 1000:

```
$ chmod 600 docs/metrics
$ agent-scaffold validate --source docs/plans/p.plan.toml
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
exit: 0

$ agent-scaffold validate --source docs/plans/p.plan.toml --workflow
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but the round log at docs/metrics/workflow.jsonl could not be checked (Permission denied (os error 13)): the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log
exit: 1
```

Matches the check on exit code, on the plain-`validate` note verbatim, and on `could not be checked (Permission denied (os error 13))` verbatim.

CELL 2, trailing slash, uid 1000:

```
$ agent-scaffold validate --source docs/plans/p.plan.toml --metrics docs/metrics/workflow.jsonl/
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
no metrics log at docs/metrics/workflow.jsonl/; nothing to validate
exit: 0

$ agent-scaffold validate --source docs/plans/p.plan.toml --metrics docs/metrics/workflow.jsonl/ --workflow
no metrics log at docs/metrics/workflow.jsonl/; nothing to validate
--workflow requested but the round log at docs/metrics/workflow.jsonl/ could not be checked (Not a directory (os error 20)): the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log
exit: 1
```

Matches, including `could not be checked (Not a directory (os error 20))` verbatim and the note echoing "the path as given" with its trailing slash. See `R1A-6` for the one wording defect in how the check states this cell's command.

CELL 2 AND CELL 1 AT UID 0, under `unshare -Ur`, which the check's "at every uid including root" claim demands:

```
$ unshare -Ur sh -c 'id -u; ...'
uid: 0
=== trailing slash, plain validate ===
no metrics log at docs/metrics/workflow.jsonl/; nothing to validate
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
exit: 0
=== trailing slash, --workflow ===
--workflow requested but the round log at docs/metrics/workflow.jsonl/ could not be checked (Not a directory (os error 20)): ...
exit: 1
=== mode-600 directory, --workflow (the degeneracy the check acknowledges) ===
docs/metrics/workflow.jsonl: 1 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0
```

Both halves of the check's uid claim hold exactly as written: the ENOTDIR spelling is uid-independent, and the mode-600 spelling degenerates at root, which is why the check keeps both.

THE RECORDED RESIDUAL, pinned by the check and not to be fixed. Re-measured only to confirm the check describes it ACCURATELY, which is the one thing a reviewer is allowed to raise here:

```
$ chmod 000 docs/metrics/workflow.jsonl        # the LOG FILE, parent searchable
$ agent-scaffold validate --source docs/plans/p.plan.toml
Error: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }
exit: 1
```

against cell 1's unsearchable DIRECTORY over the same log at exit 0. The check's wording, "a mode-000 log FILE exits 1, propagating a raw `Error: Os { code: 13, kind: PermissionDenied, .. }` out of `main` rather than a reported problem, while an unsearchable DIRECTORY over the same log exits 0", is accurate to the byte, with `..` eliding only the `message` field. No finding.

## Parts A and B: what resolved

Every `file:line` the diff added or changed, opened at its range. All resolve, all hold their named subject:

| Citation | Named subject | Verified at |
| --- | --- | --- |
| `src/main.rs:257-258` | the `source.read("instrument.md").unwrap_or_default()` read | `:257` is `let instrument_block =`, `:258` the read |
| `src/main.rs:2279-2287` | the second `fn scratch(name)` | exactly the function, signature to closing brace |
| `src/main.rs:2280-2285` | the same helper's name discrimination | body only; see the note below |
| `src/main.rs:2289-2305` | `init_plan_defaults_to_git_and_skips_inside_a_repo` | `#[test]` at `:2289` to `}` at `:2305` |
| `src/main.rs:2878-2889` | `install_precommit_hook_skips_a_non_repo` | `#[test]` at `:2878` to `}` at `:2889` |
| `tests/validate_workflow_toml_source_needs_no_plan.rs:97`, `:129`, `:190`, `:287` | four `{pid}-{nanos}` scratch sites | all four are `std::env::temp_dir().join(format!(` |
| `tests/validate_workflow_toml_source_needs_no_plan.rs:127-171` | `workflow_with_no_plan_source_hard_errors_instead_of_skipping` | `#[test]` at `:127` to `}` at `:171` |
| `README.md:210` | the `validate` paragraph | correct |
| `README.md:212-232` | the `validate` example block | `` ```sh `` at `:212`, closing fence at `:232` |
| `README.md:238` | the `status` paragraph | correct, and it carries the quoted never-fails sentence |
| `README.md:242-260` | the `status` example block | `` ```sh `` at `:242`, closing fence at `:260` |

Checked and NOT a finding: `src/main.rs:2280-2285` starts one line inside `fn scratch` and stops two lines before its end, where the sibling `src/checks.rs:1037-1046` in the same sentence spans the whole function. That asymmetry is inherited, not introduced. The pre-drift citation was `src/main.rs:1726-1731`, and `git show c44d8d1:src/main.rs` shows `:1725` was the `fn` signature and `:1726-1731` the same body-only span. The re-point preserved the original convention exactly.

Quotations the diff RE-TENSED as historical. Each must be absent from the tree. All are, checked with `grep -F`:

- "the deterministic `validate --workflow` check, once built, is the backstop ..." absent from `pack/AGENTS.md`. `pack/AGENTS.md:93` now carries the qualified form, ending "and on a project with no round log yet, which every project scaffolded without `--instrument` remains, that check exits non-zero reporting that it could not run rather than passing."
- `default_value = "docs/metrics/workflow.jsonl"` absent from `src/main.rs`.
- "the same treatment for both, so the behaviour is consistent" absent from `src/main.rs`.
- "skipping the workflow check" absent from `src/main.rs`.
- "with a source present but metrics missing the tool still soft-skips" absent from the whole of `tests/` and `src/`.
- "all steps complete, every pending step blocked, or no plan source" absent from `src/next.rs`, whose `active_loop` comment is now `:181-182`.
- The one exception is `R1A-5`.

Quotations presented as CURRENT. All match:

- "This section is present only because instrumentation was enabled; a scaffold without `--instrument` omits it entirely" at `pack/instrument.md:15`.
- "Unlike `validate` it never fails on a missing or malformed file (a missing part is simply left out of the projection)" at `README.md:238`.
- "exits non-zero if any exist, so it can gate a commit or run in CI" at `README.md:210`.
- `round_step_slug(round) == step.slug` at `src/workflow.rs:449`.
- "no metrics log at <path>; nothing to validate" at `src/main.rs:867`.
- "`--workflow` was explicitly requested, so skipping would green-pass while checking nothing; make it a hard problem instead." at `src/main.rs:1039-1041`. A literal `grep -F` of the whole sentence FAILS, but only because the source comment wraps between "hard problem" and "instead." and omits the backticks around `--workflow`. Reading the three lines settles it. Not a finding.

Structural claims the pass left in the present tense, checked and still TRUE:

- "a four-arm `match` ... over `(toml_primary, &plan_contents, &metrics_contents)`": `src/main.rs:1005`, arms at `:1010`, `:1024`, `:1042`, `:1067`.
- `run_validate`'s two reading arms `(Some(source), _, Some(metrics_text))` and `(None, Some(plan_text), Some(metrics_text))`: `:1010` and `:1024`.
- "`run_next` ... the `else` arm of its `metrics_path.exists()` branch": `src/main.rs:1738-1743`.
- "`status`'s `Projection` carries `metrics: Option<MetricsProjection>` the same way": `src/main.rs:574`.
- "The only two places `pack/AGENTS.md` mentions `docs/metrics/workflow.jsonl` outside the instrumentation section are `pack/AGENTS.md:61` and `:63`": `grep -n` returns exactly those two, so inc3's qualifier at `:93` did not break this.
- `pack/AGENTS.md:116` is `{{instrument}}`.
- `justfile:46-48` is the `scaffold-self` recipe, the render then `nix fmt`.
- `src/plan/source.rs:480-495` is `is_safe_sidecar_ref`; `:102` is `#[serde(deny_unknown_fields)]` on `Meta`; `src/plan/render.rs:296` is the `meta.title` read; `:167-169` the `meta.sidecars` read; `src/workflow.rs:180-195` is `check_workflow_toml`; `src/findings_naming.rs:52-55` is `join_dir`; `src/checks.rs:1477-1486` and `:1488-1498` are the two named `checks` tests; `src/checks.rs:1037-1046`, `src/manifest.rs:552-558`, `src/plan/render.rs:638`, `tests/audit_command.rs:20`, `tests/scaffold_precommit_hook.rs:14`, `tests/checks_staged_hook_env.rs:50`, `tests/validate_toml_primary_skips_markdown_plan.rs:74` are all the scratch helpers they are named as.
- `src/next.rs:has_risk_class_conflict` (`:854`), `build_context` (`:995`), `select_active_loop` (`:711`) and its `build_pending_loop(step, LoopState::Blocked, ...)` return (`:733`) all exist.
- `src/main.rs:StatusArgs::ledger_fragment` carries `requires = "resume"` (`:465`); `AuditArgs::out` carries `conflicts_with = "json"` (`:557`).

The `Q-55-twinsites` deletion, checked against its own historical premise. The sidecar re-tensed rather than deleted its half: `:208` now reads "`status --json` has NO golden, and HAD no test on its serialisation at all". A re-tensed false claim would be worse than the original, so I checked the past tense against the commit the sentence was written at:

```
$ git log --oneline -S 'no test on its serialisation at all' -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md
75c962d docs: fold the JSON-reason decision and specify its field vocabulary
$ git ls-tree --name-only 75c962d tests/
audit_command.rs  checks_missing_tmpdir.rs  checks_staged_hook_env.rs
scaffold_precommit_hook.rs  validate_toml_primary_skips_markdown_plan.rs
validate_workflow_toml_source_needs_no_plan.rs
```

No `status --json` test in any of them, and no `Projection` serialisation test in `src/main.rs`'s `mod tests` at that commit. The past tense is historically TRUE. The two code twins were deleted as decided (`tests/unsafe_pairings_are_refused_and_omitted.rs:155` and `:1370`). No finding.

Other acceptance checks re-run:

- Check 22: reproduces, output quoted under `R1A-3`.
- Check 23: `render docs/plans/agent-scaffold.plan.toml --check` gives "up to date" at exit 0; `validate --source docs/plans/agent-scaffold.plan.toml --workflow` gives "286 records, valid" and "workflow invariants hold" at exit 0.
- `cargo test` with `TMPDIR` outside any repository: 378 + 20 + 9 + 5 + 4 + 3 + 1 + 1 + 1 passed, 0 failed.
- `cargo test` with `TMPDIR` inside a git repository: exactly three fail, and they are exactly the three `test-tmpdir-repo-assumption.md` names. That sidecar's "Three tests" claim is still current after this step's three increments.

## What this lens varied, and what it held fixed

Stated so no one reads the clean part of this report as wider than it is.

VARIED: uid (1000 and 0 under `unshare -Ur`); probe failure class (EACCES from an unsearchable ancestor at mode 600, ENOTDIR from a trailing slash, EACCES on the log file itself at mode 000, and the healthy control); `--workflow` present and absent; `--metrics` explicit and anchored-default; `--source` kind (TOML-primary, Markdown-primary, parse-failure, absent); `--plan` state (absent, present and readable, present and unreadable); `TMPDIR` inside and outside a git repository.

HELD FIXED: one platform (Linux, local filesystem), one build profile (debug), one binary (`079d63f`), one project layout (`docs/plans` and `docs/metrics` under a single root). I built no symlinked layout, no nested-project layout and no cross-project pairing, so I re-measured NONE of checks 11, 13b, 14a-14h, 18, 19 or 19b; those are inc2's evidence and inc4 does not restate them. I ran no concurrency or TOCTOU case. I did not review prose, wording, tense style as such except where a tense makes a mechanically checkable claim about the tree, and I raised nothing about line length or wrapping.

WHAT I DID NOT CHECK, deliberately, per the brief: every `src/checks.rs` citation in `checks-runner-worktree-name-collision.md` (out of scope, and acceptance check 21b names it so); `run_validate`'s `--plan` clap claims; `src/next.rs:162` and its `active_loop` disjunct at `:181-183`; `docs/plans/agent-scaffold.md:7`. I re-raised none of inc2's four or inc3's four recorded residuals; check 16's pinned residual was re-measured only to confirm the check describes it accurately, which it does.
