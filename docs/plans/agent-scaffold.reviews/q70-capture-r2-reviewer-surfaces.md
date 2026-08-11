# Q-70 capture, round 2, reviewer: surfaces and executability

Artifact: `git diff main..HEAD` on branch `review/q70r2-surfaces`, commits `0a2e1e3`, `3a74e4e`, `129215d`. The third commit is the round-1 remedy pass; this round reviews the post-remedy state. Round-1 record: `docs/plans/agent-scaffold.reviews/q70-capture-triage.md` (eleven valid, one dismissed, four duplicate, all remedies verified applied below).

Binary: `target/debug/agent-scaffold` (`agent-scaffold 0.0.1`) built from this worktree at HEAD via `cargo build`. Fixture root: `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/q70r2/`. Every fixture was built from a fresh `cp -r docs/`; nothing outside that directory was written or deleted.

Lens: the two surfaces round 1 skipped (`[meta].orphan_tasks` and the sidecar), and the executability of every command the item instructs a reader to run.

Result: ONE VALID FINDING (`R2C-1`, `medium`), zero `high` or `critical`, so no backstop re-check is owed. Everything else I checked (the orphan-task line, the sidecar, the rendered view, the Status line, and four of five reproduction commands) reproduces exactly as the item describes. I explicitly checked for re-raises against the round-1 ledger and found none: `R2C-1` is new ground (shell portability), never examined by any round-1 lens.

---

## R2C-1. Three of the item's five reproduction commands fail immediately under `nu`, this project's own configured interactive shell

SEVERITY: `medium`.

CLAIM: the three "reproduce the current set with ..." commands that pipe through a bare `sort -u` (`plan.toml:1885`, `:1893`, `:1895`; rendered `agent-scaffold.md:229`, `:237`, `:239`) do not run in `nu` (this machine's actual configured shell: `nu 0.108.0`, confirmed via `which nu` -> `/etc/profiles/per-user/jessea/bin/nu`), because `nu` ships a builtin `sort` that shadows the external GNU `sort`, accepts no `-u` flag, and in any case does not accept a byte/text stream from an external command as input at all.

EVIDENCE. Copied verbatim from the rendered view (`docs/plans/agent-scaffold.md:229`) into a `.nu` script and run:

```
$ jq -r 'select(.type=="round") | [(.step // (.task|sub("-inc[a-zA-Z0-9]+$";""))), (.increment // .task)] | join(" ")' docs/metrics/workflow.jsonl | sort -u | awk '{lead=$2; sub(/-inc[a-zA-Z0-9]+$/,"",lead); if (lead != $1) print $1, $2}'
```
under `nu`:
```
Error: nu::parser::unknown_flag
  x The `sort` command doesn't have flag `-u`.
   ...
   `-- unknown flag
EXIT=1
```

The same failure reproduces identically for the other two `sort -u` commands (`plan.toml:1893`'s dangling-receipt extraction, and `plan.toml:1895`'s direction-(i) logged-identities extraction):

```
$ jq -r 'select(.type=="decision") | .q_id' docs/metrics/workflow.jsonl | sort -u
Error: nu::parser::unknown_flag  (identical)
$ jq -r 'select(.type=="round") | (.increment // .task)' docs/metrics/workflow.jsonl | sort -u
Error: nu::parser::unknown_flag  (identical)
```

Isolating the cause: even `sort` with no flags, piped from `jq`, fails under `nu` with a second, distinct error, confirming the builtin does not accept external-command text input at all, flag aside:

```
$ jq -r '...' docs/metrics/workflow.jsonl | sort
Error: nu::shell::only_supports_this_input_type
  x Input type not supported.
```

`nu -c 'which sort'` reports `sort` as `built-in` (no path); `which grep`, `which awk`, `which jq` all report `external` with real paths. So the failure is specific to `sort` and does not extend to the rest of the pipeline: the fourth command (mechanism (1), `grep -onE "[(][0-9]+(, [0-9]+){1,6}[)]" docs/plans/agent-scaffold.plan.toml`, `plan.toml:1893`/`md:237`) has no `sort` and runs cleanly under `nu`, producing the same four lines as under `bash`.

WHAT THE PROSE IMPLIED VS WHAT HAPPENED. The item's whole design is "no count is stated, reproduce it yourself" (stated explicitly at three of these four sites). A reader following that instruction in this project's own shell gets no output and no reproduction, only a parser error, for three of the four "reproduce with" commands the item gives (the fourth, grep-only, works).

WHY `medium` AND NOT HIGHER. The failure is loud, not silent: it is an immediate parse-time error, not a wrong count or a wrong conclusion, so it cannot mislead a reader the way the item's own "worse than no command" standard describes for a silent bad output. It is also cheaply fixed by anyone who knows `nu`'s escape (`^sort -u`, or wrapping the pipeline in `bash -c "..."`), and the item's primary consumer, per its own "WHAT THE PASS OWES BACK" clause, is a spawned explorer agent, which runs shell commands through its own tooling (in this harness, a `bash`-backed tool) rather than through the human's interactive `nu` session, so the primary execution path is unaffected. It affects a human auditing the item at their own prompt, a real but secondary use of these commands, not the primary one. I hold this at `medium` rather than `low` because it is reproducible on THREE separate commands (not one), it is newly introduced by this diff rather than inherited (see below), and the item's own no-counts design rests on the reader being able to run exactly these commands.

NOT PRE-EXISTING. `git show main:docs/plans/agent-scaffold.plan.toml | grep -c "sort -u"` returns `0`; the current tree returns `3`, all three inside the `Q-70` block (`plan.toml:1885`, `:1893`, `:1895`). This pattern is new to this diff, not an inherited convention from elsewhere in the plan, so it is squarely in scope.

CHECKED AND NOT A FACTOR: escaping divergence between TOML and rendered view. The task brief flagged that a command "passes through different escaping" between the TOML source and the rendered `.md`, since that is where a reader copies from. I extracted the exact command substrings from both files with `grep -oE` for all three `sort -u` commands and the grep-only one and diffed them: byte-identical in every case (TOML `ask = """..."""` is a basic multi-line string, but none of these commands contain a character TOML would re-escape, so nothing drifts). The `nu` failure is identical regardless of which file the reader copies from.

---

## The surfaces round 1 skipped

### The `q70-capture` orphan-task entry

CORRECT, correctly placed, and does what the field is documented to do. `src/plan/source.rs:113-116` documents the field: "tasks that appear in the round log but own no Roadmap step, declared here so they are visible rather than inferred." `q70-capture` is exactly that: the review-loop bookkeeping task for capturing the `Q-70` registration itself (its round record's `artifact` field reads "the Q-70 registration on plan/vc-question: the exploring question commissioning the W5 waiver-ownership design pass, its orphan-task line and its sidecar"), and it owns no Roadmap step. It is alphabetically placed between `q66-q67-fold` and `uniform-isolation`, consistent with the list's existing sort order (verified the whole 17-entry list is in strict lexical order). It follows the exact convention its siblings (`q65-capture`, `q64-capture`, `q66-q67-fold`) already establish, confirmed by reading their own round records (`q65-capture`, `q66-q67-fold` in `docs/metrics/workflow.jsonl` are the same "review the capture/fold of a question" pattern).

WHAT `[meta].orphan_tasks` ACTUALLY AFFECTS, measured. It participates in exactly one thing: `validate_source` in `src/plan/source.rs:767-783`, which checks each token is a well-formed kebab-case token, unique within the list, and NOT also a declared step slug (a genuine-orphan check). `grep -c orphan_tasks src/workflow.rs` returns `0`: it plays no role in W3, W5, or any workflow check, and no role in `render` (it never appears in `src/plan/render.rs` and does not affect the rendered `.md` at all). This is exactly what `Q-70`'s own "escape route (3)" claims of it, and I verified the claim against the code rather than taking it on the item's word.

`agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --metrics docs/metrics/workflow.jsonl --workflow` on the live tree: `309 records, valid`; `95 steps, 70 questions, valid`; `workflow invariants hold`; exit 0. `q70-capture` passes every check the field is subject to.

IS `q70-capture` THE ONLY DECLARED ORPHAN TASK WITH NO ROUND RECORD, as round 1 recorded. VERIFIED, but the verdict has FLIPPED since round 1 wrote it, for a reason inherent to what the field measures rather than any defect. I ran, per orphan task, `grep -c "\"task\":\"<token>\"" docs/metrics/workflow.jsonl` for all 17 declared tokens: every one of them, including `q70-capture`, now returns `>= 1`. `q70-capture` itself returns `1`, and that one record IS round 1's own review of this diff (`{"type":"round","task":"q70-capture", ..., "valid_findings":11, ...}`, matching round 1's eleven-valid result exactly). Round 1's triager measured this before its own round got appended to the log; by the time round 2 opens, the round that made the observation has itself become the counter-evidence. It does not matter: the observation was never a claim about the diff's correctness, `[meta].orphan_tasks` performs no check that depends on round-record presence (that check belongs to W3/W5, which this field does not participate in, see above), and the flip is a property of measuring a moving population at two different times, not a mistake by any round-1 lens.

### The empty sidecar `docs/plans/agent-scaffold.questions/Q-70.md`

VERIFIED empty, and verified it is the whole population, not a sample: `find docs/plans/agent-scaffold.questions -type f -name '*.md' -size 0c | wc -l` returns `70`; `find ... -name '*.md' | wc -l` also returns `70`. Every question sidecar in the plan is exactly zero bytes, `Q-70.md` included (confirmed byte-for-byte with `wc -c`).

NOT CARGO-CULTED: it is load-bearing. `render_plan` (`src/plan/render.rs:135-195`) requires a sidecar file to exist at the fixed path `<task>.questions/<id>.md` for every declared question, unconditionally, via `read_sidecar` (`src/plan/render.rs:125-128`), which wraps `fs::read_to_string` and turns a missing file into a hard render failure. An EMPTY file reads fine (`Ok("")`); an ABSENT file does not. I tested the absent case directly rather than reading the code alone: copied `docs/`, deleted the `Q-70.md` sidecar, ran `render --check` and plain `render` against the copy. Both fail identically:

```
$ agent-scaffold render <copy>/docs/plans/agent-scaffold.plan.toml --check
<copy>/.../agent-scaffold.plan.toml: missing or unreadable sidecar <copy>/.../Q-70.md: No such file or directory (os error 2)
EXIT=1
```

So the empty file is exactly correct for this artifact given the current mechanism: every question sidecar must exist, its content becomes additional prose appended under the question in the rendered `.md` (`src/plan/render.rs:183-188`), and `Q-70` (like all 69 of its siblings) currently carries its whole body inline in the TOML `ask` field and uses no sidecar prose. This is a live, unanimous, unbroken convention across the whole plan, not something `Q-70` invented or got wrong.

### The regenerated view `docs/plans/agent-scaffold.md`

RENDERS CORRECTLY. `render docs/plans/agent-scaffold.plan.toml --check` reports `up to date`, exit 0, so the committed `.md` is byte-identical to a fresh render; this is a tool-enforced guarantee, not something I need to eyeball. The `Q-70` entry appears at `agent-scaffold.md:227` (the bulleted opening line) through `:247` (the closing "NOT DECIDED, and NO STEP YET" paragraph), immediately followed by `## Roadmap` at `:249`; no truncation, no bleed into the next section.

Status line, checked specifically: `-Status: ... 5 open questions; ...` -> `+Status: ... 6 open questions; ...` (the only other change in the `.md` diff besides the new `Q-70` paragraphs; `git diff main..HEAD -- docs/plans/agent-scaffold.md` has exactly two hunks). Verified the count independently of `render --check`: `awk` over the TOML's `[[question]]`/`status` pairs gives 63 `decided`, 4 `exploring`, 2 `open`, 1 `superseded` = 70 total; `src/plan/render.rs:363-373` counts `Open | Exploring` as outstanding (documented reasoning: "Both `open` and `exploring` are UNRESOLVED ... `decided` and `superseded` are resolved and excluded"), giving `4 + 2 = 6`, matching the rendered line exactly.

ONE PRE-EXISTING RENDERING PROPERTY, NOT A DEFECT IN THIS DIFF: a multi-paragraph `ask` (paragraphs separated by blank lines in the TOML triple-quoted string) renders with the `- \`Q-ID\` (status) ...` bullet marker on only its FIRST paragraph; every subsequent paragraph (`Q-70`'s "THE BLOCKER", "EVERY ESCAPE ROUTE", etc., at `md:229` onward) renders as a plain, unbulleted top-level paragraph with no visual marker tying it back to the `Q-70` entry. I checked this is not something `Q-70` introduced: `Q-68` (`md:207-211`, five paragraphs) and `Q-69` (`md:212-224`, six paragraphs) show byte-identical behaviour, both predating this diff. Out of scope under this round's rule (a pre-existing, inherited renderer behaviour, not something this diff changed or got wrong), recorded here only because the task asked whether the rendered form differs from the source in a way that matters to a reader; the answer is yes for a reader who scans by bullet, but the behaviour is inherited and unanimous across every multi-paragraph question in the plan, not a `Q-70` defect.

---

## Commands run, with exact output and verdict

| command (as rendered, `agent-scaffold.md` line) | ran | output matched prose | verdict |
| --- | --- | --- | --- |
| `jq ... \| sort -u \| awk ...` (THE BLOCKER, `:229`) under `bash` | yes | yes, byte-identical to round 1's six-identity result | clean |
| same command under `nu` | fails at parse | no output at all | `R2C-1` |
| `grep -onE "[(][0-9]+(, [0-9]+){1,6}[)]" ...` (mechanism 1, `:237`) under `bash` | yes | yes, 4 lines, matches "no count stated" | clean |
| same command under `nu` | yes | yes, identical 4 lines | clean |
| `jq -r 'select(.type=="decision") | .q_id' ... \| sort -u` (mechanism 2, `:237`) under `bash` | yes | 62 ids; set difference against 70 registered gives 40 dangling, all `Q-55-*`, matching round 1 exactly | clean |
| same command under `nu` | fails at parse | no output at all | `R2C-1` |
| `jq -r 'select(.type=="round") | (.increment // .task)' ... \| sort -u` (direction i, `:239`) under `bash` | yes | 95 logged identities vs 45 declared `[[step.increment]]` ids; item asks only for a comparison, not a ratio, and gives none | clean |
| same command under `nu` | fails at parse | no output at all | `R2C-1` |
| "Measured for this registration: the token `W6` occurs exactly once ... outside this item" (`:241`, prose, no literal command given) | yes (I derived one: `grep -n W6` filtered to lines outside the `Q-70` TOML block, `1880-1904`) | true: exactly one hit outside the block, at `plan.toml:1774`, `Q-59`'s `ask` | clean |
| fixture recipe: inject `workflow-enforcement-tier-w5` waiver, undeclared then declared (`:233`, prose) | yes, rebuilt from scratch in my own fixture dirs | undeclared: both the source-path and W5 messages fire, byte-identical to the item's two quoted strings, exit 1; declared: the source-path message disappears, the W5 message still fires, exit 1 | clean |
| `agent-scaffold validate --source ... --workflow` on the live tree | yes | `309 records, valid`; `95 steps, 70 questions, valid`; `workflow invariants hold`; exit 0 | clean |
| `agent-scaffold render ... --check` on the live tree | yes | `up to date`, exit 0 | clean |
| `LC_ALL=C grep -cP '[^\t\x20-\x7e]'` on all three changed files | yes | `0` for all three | clean |

## What I settled by running vs. by reading

RUN: the four distinct reproduction commands (under both `bash` and `nu`), the `nu` isolation test (`sort` alone, no flags), the `nu`-vs-external command-resolution check (`which sort/grep/awk/jq`), the TOML-vs-rendered escaping diff for all four commands, the orphan-task round-record census (all 17 tokens), the render-with-absent-sidecar test, the sidecar zero-byte census (all 70), the Status-line open-question count, and the full `validate --workflow` / `render --check` / ASCII sweep on the live tree.

READ: the `orphan_tasks` doc comment and validation code (`src/plan/source.rs:113-116`, `:767-783`), the sidecar-loading code (`src/plan/render.rs:125-195`), the Status-line derivation (`src/plan/render.rs:360-392`), and the `Q-68`/`Q-69` comparison for the multi-paragraph bullet behaviour.

Nothing above is presented as measured that was not run.
