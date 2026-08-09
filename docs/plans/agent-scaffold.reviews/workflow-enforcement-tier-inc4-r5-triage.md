# `workflow-enforcement-tier-inc4` round 5: triage

Triager worktree `.claude/worktrees/triage-inc4-r5`, branch `triage/wet-inc4-r5`, at `507456a`. Binary built `--release` from this tree; `cargo test --release` green at this tree. Increment base is `c775872^` (`800d359`). Fixtures under `<scratchpad>/triage-inc4-r5/` only; every `chmod` restored to `drwxr-xr-x` and verified with `ls -ld`.

I was told the convergence arithmetic before I ruled and I record that I was told: this is the CAP round, the loop escalates whatever I return, so no verdict of mine can save or cause an escalation. It did not move a verdict. The place that pressure would show is `R5A-4`, the one dismissal, so that entry carries the control I ran rather than an assertion.

## Counts

| | |
| --- | --- |
| RAW findings | 6 |
| After deduplication | 6 (no duplicate pair; the two productive lenses ran on disjoint territory) |
| VALID | 5 |
| DISMISSED | 1 (`R5A-4`, `low`, so NO backstop re-check is owed) |
| VALID AND IN SCOPE | 5 |
| VALID AND OUT OF SCOPE | 0 |
| Severities corrected | 1, downward (`R5B-1`, `medium` to `low`). None corrected upward. |

Per reviewer:

| Lens | Raw | Valid | In scope |
| --- | --- | --- | --- |
| `r5-sourceclaims-opus` (`R5A-1` to `R5A-4`) | 4 | 3 | 3 |
| `r5-residue-sonnet` (`R5B-1`, `R5B-2`) | 2 | 2 | 2 |
| `r5-exhaustion-opus` | 0 | 0 | 0 |

Severity distribution of the five valid findings: `medium` 3 (`R5A-1`, `R5A-2`, `R5A-3`), `low` 2 (`R5B-1`, `R5B-2`). ZERO `high`, ZERO `critical`. Nothing is dismissed at `high` or above, so no independent re-check is triggered.

**THE ROUND IS NEW-VALID**, on `R5A-1`, `R5A-2`, `R5A-3`, `R5B-1` and `R5B-2`. It is new-valid on the three `medium` findings alone and does not depend on either `low` ruling or on the `R5A-4` dismissal.

REPRODUCED FIRST-HAND, evidence re-run rather than read: ALL SIX. Nothing was judged on citation alone. Three required purpose-built fixtures (`R5A-1`, `R5A-2`, `R5A-3`), one required a HISTORICAL BINARY built from `269d075^` in its own target directory (`R5A-3`'s falsifier), one required running the reviewer's own literal-grep procedure at the named revisions (`R5B-2`), and one required a control the reviewer had not run (`R5A-4`). I additionally re-ran the whole of the exhaustion audit's part B arithmetic, since the brief asked for it.

## Remedy class, stated up front because it is the round's operative question

The audit recommends constraining any further fix pass to DELETIONS. Whether that is possible for these five:

| Finding | Minimal remedy | Class | Deletion-only achievable |
| --- | --- | --- | --- |
| `R5A-1` | Delete four words at two string sites, plus 12 mechanical test-assertion updates | DELETION + TOKEN | YES |
| `R5A-2` | Insert one word at four sites (`at` becomes `found at`) | TOKEN | NO, but one token per site |
| `R5A-3` | Delete the trailing clause at five sites | DELETION | YES |
| `R5B-1` | Append a ~12-word clause to an existing bullet | AUTHORED PROSE | NO |
| `R5B-2` | Append a ~6-word qualifier, OR delete the single word `literal` | TOKEN, or DELETION with a cost | PARTIALLY |

So a deletion-and-token-only fix pass closes `R5A-1`, `R5A-2`, `R5A-3` and `R5B-2` (four of five, and 9 of the 12 sites), and CANNOT close `R5B-1`. `R5B-1` is the only finding in this round whose remedy is the class this project has six measurements against.

---

## (1) `R5A-1`: VALID, `medium` CONFIRMED, IN SCOPE. Round 4's `R4B-1` is NOT closed, and the round 4 triage's severity ground is refuted

### The quoted premise reproduces, verbatim, and it is false

`workflow-enforcement-tier-inc4-r4-triage.md:150` reads exactly:

> FOR `low`: no behaviour is wrong, no user-visible string is wrong, and four words of a qualifier are a small thing.

**I SAY THIS PLAINLY BECAUSE THE BRIEF ASKED FOR IT: THAT PREMISE WAS WRONG WHEN IT WAS WRITTEN.** Two user-visible strings carried and still carry the same falsehood, in the same file the round 4 finding was about, and a `grep` refutes it. The round 4 triager weighed `low` against a fact it had not checked, confirmed `medium` anyway, and the error therefore changed no verdict. What it changed is the SCOPE OF THE REMEDY: the fix brief was written from a triage that had recorded the user-visible surface as clean, so the fix pass had no reason to look there.

One correction to the reviewer's transcript, which does not touch the substance. Its two-line `grep -n "plan's project root" src/main.rs` is INCOMPLETE: the current tree returns THREE hits, `:1000`, `:1509` and `:1522`. The reviewer discusses `:1000` at length in its own remedy section and rules it correct, so the omission is a transcript error rather than a missed site, and I verified `:1000` separately below.

### Reproduced in full, all three invocations

My own fixture, a Markdown-primary `--source` that PARSES (the reviewer's shape), a foreign log and a foreign ledger, no `--plan`:

```
$ agent-scaffold status --source <S>/projA/docs/plans/p.plan.toml --metrics <S>/foreign/docs/metrics/workflow.jsonl
plan: not provided
metrics: unavailable, the round log <S>/foreign/docs/metrics/workflow.jsonl is not under the plan's project root <S>/projA, so its records cannot be paired with this plan
exit=0

$ agent-scaffold next --source ... --metrics ... --ledger-fragment <S>/foreign/p.ledger.md
task: p
source: no plan source
metrics: unavailable, the round log ... is not under the plan's project root <S>/projA, ...
no active review loop (no plan steps found)
the ledger <S>/foreign/p.ledger.md is not under the plan's project root <S>/projA; nothing to resume
exit=0

$ agent-scaffold status --resume --source ... --ledger-fragment <S>/foreign/p.ledger.md
the ledger <S>/foreign/p.ledger.md is not under the plan's project root <S>/projA; nothing to resume
exit=0
```

Byte-identical to the reviewer's transcript modulo paths, including the SOLE-OUTPUT case. `status --resume` never calls `toml_source` and never opens a plan (`run_resume`, `src/main.rs:1640-1665`); its one line of output attributes the root to a plan it did not read. That is the sharpest instance and it is not a re-raise of anything.

### Scope: IN SCOPE, ruled against all four conditions

1. PROVENANCE PREDATES THE BASE. HOLDS. `git log --oneline -S "is not under the plan's project root" -- src/main.rs` returns exactly `8beb1c2` (inc2's feature commit), and `git merge-base --is-ancestor 8beb1c2 800d359` succeeds.
2. NO COMMIT IN RANGE MODIFIES THE LINES. HOLDS. `git diff -U0 800d359 HEAD -- src/main.rs` has exactly three hunks: `@@ -461`, `@@ -570`, `@@ -1191,8`. Neither `:1509` nor `:1522` is touched.
3. INDEPENDENT SUBJECT. **FAILS**, and I verified the falsifier rather than accepting the citation. I built a binary from `269d075^` in its own `CARGO_TARGET_DIR` with the sources touched (the recorded shared-fingerprint trap), confirmed by `md5sum` that it differs from the current binary, and ran the same fixture:

```
$ <269d075^ binary> status --json --source <S>/projA/docs/plans/p.plan.toml --metrics <S>/foreign/.../workflow.jsonl
  "metrics": { "records": 1 },
  "metrics_absent_reason": null
```

Before `269d075`, `unpairable_log_note` was UNREACHABLE where no plan was read: the log was simply counted. So the note only ever printed where a plan HAD been read, where the root DID come from that plan, and `:1509` was TRUE when written. `269d075`, an inc2 fix-round commit, is what falsified it. Limb 2 fails. This is `R4B-1`'s provenance shape exactly and `Q-55-twinsites` settled the reading.

I RECORD A DISTINCTION THE REVIEWER DID NOT DRAW, because it matters to how the human reads the finding. The LEDGER half at `:1522` was NOT falsified by `269d075`. My same pre-`269d075` binary prints the identical ledger sentence, because `resume_roots` was introduced by `8beb1c2` itself and `status --resume` has taken its root from the anchors since the day the sentence was written. **The ledger half was BORN FALSE inside this step, on the surface whose whole definition is that it reads no plan.** That is a worse fact than the reviewer's framing, not a better one, and it is why the finding cannot be split: both sentences are one fix and condition 4 fails for the pair.

4. NO SHARED FIX. FAILS. The two sites take one four-word deletion prescribed together, and correcting them adds a site to `R5B-1`'s list.

All four conditions are required and two fail. IN SCOPE.

### Severity: `medium` CONFIRMED, and I weighed `low` and `high`

AGAINST `high`: nobody acts wrongly. The withholding is correct, the JSON reason token is correct, both paths named in the sentence are accurate, and the remedy the operator needs is the same under either wording.

AGAINST `low`: this is the exact ground the round 4 triage confirmed `R4B-1` on, with the mitigation it relied on removed. `R4B-1` was `medium` because two doc comments are the only definitions of two serialised contract tokens. A string the operator READS is at least as load-bearing as a comment beside the type, and on `status --resume` there is no `--json` surface at all, so the sentence is the only output that surface produces. `medium`.

### Minimal remedy: DELETION, four words, two sites, plus 12 mechanical test updates

- `src/main.rs:1509` becomes "the round log {} is not under the project root {}, so its records cannot be paired with this plan".
- `src/main.rs:1522` becomes "the ledger {} is not under the project root {}; nothing to resume".

NOTHING IS AUTHORED. Every word retained is already in the sentence.

**I CORRECT AND SHARPEN THE REVIEWER'S TEST-ASSERTION WARNING, which is right in substance and wrong in its discriminator.** The 16 hits in `tests/unsafe_pairings_are_refused_and_omitted.rs` reproduce at exactly the lines it names. But the split is NOT "carries a path prefix"; it is WHICH STREAM the assertion reads:

- MUST CHANGE, 12 assertions on STDOUT from `status` / `next` / `status --resume`: `:457`, `:501`, `:525`, `:602`, `:612`, `:707`, `:749`, `:1080`, `:1237`, `:1357`, `:1520`, `:1564`.
- MUST NOT CHANGE, 4 assertions on STDERR from `validate --workflow`, which reads `src/main.rs:1000`: `:222`, `:376`, `:555`, `:1634`.

The reviewer named `:222`, `:376` and `:555` "and some others" without closing the set, and it did NOT identify `:1357`, which carries no path prefix and asserts the bare substring on `status --resume` STDOUT, so it must change. A fix pass working from the reviewer's rule would have left `:1357` and the suite would have gone red. The stdout-versus-stderr rule closes the set exactly.

`src/main.rs:1000` MUST NOT CHANGE and I verified why rather than taking it: a Markdown-primary `--source` with no `--plan` on `validate --workflow` never reaches that string, it hits the `(None, None, _)` arm and prints "--workflow requested but no plan source resolved". Measured. Where `:1000` does print, a plan was read and the root does come from it.

### The twin-site sweep the reviewer did not run, which I ran, and its result

This task has been bitten five times by a fix landing at one site while a twin survived, so I swept the phrase across the whole tree rather than taking the reviewer's site list.

- `README.md:229` carries the phrase inside a transcript of `validate --workflow`'s output. CORRECT there, must not change.
- `CHANGELOG.md:24` describes BOTH root-supply policies accurately, including "Where no plan is read at all ... those three take their roots from the anchors instead". CORRECT, must not change.
- `src/next.rs:157`, `NoActiveLoopReason::MetricsNotThisProject::human_text()`, is a THIRD user-visible string carrying the phrase. The reviewer opened it and ruled it correct on a structural argument, and I verified that argument independently rather than taking it: `canonical_project_root` returns `None` only when `fs::canonicalize` fails, `MetricsNotThisProject` requires steps to have been read, steps require the plan file to exist, so `checked_plan_root` always returns `Some` on that path and the root always is the plan's. THE REVIEWER IS RIGHT AND `:157` MUST NOT CHANGE.
- SECONDARY SITES, all doc comments carrying the same attribution and all false in the anchor-supplied case, to be fixed in the same change so this does not recur a third time: `src/main.rs:573`, `src/next.rs:176-177`, `src/next.rs:184-185`.

So the complete site set is TWO strings and THREE doc comments, and the reviewer's list is complete.

---

## (2) `R5A-2`: VALID, `medium` CONFIRMED, IN SCOPE. It is NOT the recorded residual and NOT an instance of it

### Reproduced in full, including the single-invocation contradiction

Fixture at `<scratchpad>/triage-inc4-r5/mode600`, a conventional layout with a TOML-primary plan, a REAL log holding one valid `round` record and a REAL ledger holding a `## RESUME STATE` block.

```
$ agent-scaffold status --json --source docs/plans/p.plan.toml     # control, mode 755
  "metrics": { "records": 1 },  "metrics_absent_reason": null
$ chmod 600 docs/metrics
$ agent-scaffold status --json --source docs/plans/p.plan.toml
  "metrics": null,  "metrics_absent_reason": "log-absent"
$ agent-scaffold status --source docs/plans/p.plan.toml
metrics: no log found
$ chmod 755 docs/metrics                                           # control restored
  "metrics": { "records": 1 },  "metrics_absent_reason": null
```

Nothing changed but the mode, and the file the tool called absent counts one record. The ledger half reproduces the same way, and the `status --resume` run reproduces the contradiction exactly as reported:

```
$ chmod 600 docs/plans
$ agent-scaffold status --resume --source <D>/docs/plans/p.plan.toml
note: --source <D>/docs/plans/p.plan.toml could not be checked: Permission denied (os error 13)
no ledger at <D>/docs/plans/p.ledger.md; nothing to resume
exit=0
```

ONE invocation. Two files in the SAME unreadable directory. On stderr, "could not be checked" with the errno. On stdout, "no ledger at". `note_missing_anchors` uses `try_exists` with the three-way split; `run_resume` uses `ledger_path.exists()`. Both modes restored, verified.

### It is not the recorded residual, and I establish that rather than assert it

The brief asked me to rule this carefully. I read `Q-55-existsgate`'s decision as it is recorded in the source at `src/main.rs:1056-1066` and in acceptance check 16 at `docs/plans/agent-scaffold.md:1734`. Check 16 records TWO things in this area:

- The ARM-SCOPING, pinned as "EXPECTED-FOR-NOW AND NOT AS CORRECT": "plain `validate` does not separate `Err` from `Ok(false)`, so it says a log is absent while a real one sits behind the error".
- The exit-code inconsistency: "under plain `validate` a mode-000 log FILE exits 1 ... while an unsearchable DIRECTORY over the same log exits 0. It is a RECORDED RESIDUAL routed to the validation-constraints step; an implementer must not 'fix' it here and a reviewer must not raise it."

The prohibition is scoped, by its own words, to plain `validate`'s exit-code inconsistency. `R5A-2` does not raise it and says so.

MY RULING: `R5A-2` IS A DISTINCT DEFECT ON A DIFFERENT SURFACE, on three separable grounds.

1. DIFFERENT SURFACE. The residual is plain `validate`. `R5A-2` is `status`, `next` and `status --resume`, which the arm-scoping sentence at `src/main.rs:1064-1066` does not name at all. That sentence says "plain `validate` is untouched", which is TRUE as written, and names ONE untouched surface where there are THREE. The reviewer is right not to raise the sentence and right that the count is understated.
2. DIFFERENT ARTIFACT CLASS. The residual is a note and an exit code, both acknowledged as behaviour the project has decided to live with. `R5A-2`'s subject is the ONLY DEFINITION of two SERIALISED CONTRACT TOKENS, in the file that defines the type. `Q-55-jsonreason` reserved that vocabulary to a human decision.
3. DIFFERENT REMEDY. The residual's remedy is a behaviour change routed to `validation-constraints`. `R5A-2` explicitly declines to prescribe one and asks only that the definitions stop asserting what the probe cannot establish. A doc-only remedy cannot be a re-raise of a behaviour residual.

So the underlying `exists()` collapse is recorded and accepted; what is not recorded anywhere is that it reaches two JSON tokens whose definitions assert the opposite. That gap is the finding.

I checked the never-raised claim myself: `grep -rn "LogAbsent\|log-absent\|LedgerAbsent\|ledger-absent" docs/plans/agent-scaffold.reviews/` returns hits that quote the tokens in JSON samples, inventory them as vocabulary or assert the precedence rule. None examines the definition against the tree. The reviewer's account is accurate.

### Scope: IN SCOPE, on the round 2 triage's own stated test

This is the delicate one, because the definitions were BORN FALSE at `8beb1c2` rather than falsified later, which is `R2B-4`'s shape and `R2B-4` was ruled OUT OF SCOPE. I apply the round 2 triage's own three-part limb-1 test rather than a fresh judgement.

1. Condition 1 HOLDS: `git log --oneline -S "No file at the resolved" -- src/next.rs` returns exactly `8beb1c2`, an ancestor of the base.
2. Condition 2 HOLDS: `git diff -U0 800d359 HEAD -- src/next.rs` has hunks at `@@ -105` and `@@ -140`. Lines `:103` and `:136` are untouched.
3. Condition 3, limb 2, HOLDS on the `R2B-4` reading: nothing falsified them, they were wrong on the day they were written.
4. Condition 3, limb 1, **FAILS**, and this is what decides it. The round 2 triage's limb-1 test was three-part, and its second part reads: "Neither the build pass nor the fix pass opened `:342` OR ANYTHING IN ITS BLOCK". Applied here, the round 4 fix pass DID open this block: it edited `MetricsAbsentReason::LogNotThisProject` at `:105` and `ResumeStateAbsentReason::LedgerNotThisProject` at `:140`, two and four lines from the claims, inside the same two enums, and the false lines appear in that commit's own diff context. Further, the increment's impact list as amended by round 4 now declares "`src/next.rs`'s two containment reason definitions" as a site this increment edited. The claim IS about what the increment changed.

Condition 3 fails, all four are required, IN SCOPE. I record the counter-argument rather than hide it: a reader who weighs only "was it ever true" reaches OUT OF SCOPE on the `R2B-4` precedent. I rule against that because `R2B-4`'s block was never opened by any pass and this one was opened one round ago, which is the discriminator the round 2 triage itself wrote down.

### Severity: `medium` CONFIRMED, and I weighed `low`

FOR `low`: the population is narrow, no behaviour is wrong, the exit code is unaffected, and the artifact already records the underlying collapse as accepted on a neighbouring surface.

WHAT CARRIES `medium` is the ground this project has now confirmed twice, at `R3B-1` and at `R4B-1`: these two comments are the ONLY definitions of two serialised contract tokens anywhere in the source. `Q-55-jsonreason` exists so a machine consumer can tell the causes apart. A consumer author reading `src/next.rs:103` concludes `log-absent` means the file is not there; an agent reading `next --json` and told `log-absent` concludes the project is not instrumented, when it is instrumented and merely unreadable from where it stands. That is the false-green family this step exists to remove, one register quieter. `medium`.

### Minimal remedy: TOKEN, one word per site, four sites

I take the reviewer's diagnosis and REJECT its prescribed wording, because a rewrite is authoring and a token change is available.

- `src/next.rs:103` becomes "No file found at the resolved metrics path."
- `src/next.rs:136` becomes "No file found at the resolved ledger path."
- The twin sites carrying the same sentence, which MUST be corrected in the same change or `R4B-1`'s pattern repeats a third time: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:216` and `:227`. I verified both by grep.
- The RENDERED VIEW at `docs/plans/agent-scaffold.md:1611` and `:1622` carries the same two sentences and follows mechanically from the sidecar; acceptance check 23 requires the re-render.

One word, `found`, inserted four times. It asserts what the probe returned and drops the assertion the code cannot make.

I DECLINE the reviewer's fourth site. `src/main.rs:461`'s `StatusArgs::resume` help ("Exits 0 with a note when the ledger is absent, carries no such section, or is not this plan's") is a claim about the EXIT CODE and the note, not a definition of what absence means. It is not the same claim and it must not be swept in.

I AGREE WITH THE REVIEWER THAT NO SOURCE CHANGE IS PRESCRIBED. Splitting the probe on the three surfaces would mint tokens on a documented JSON contract, which `Q-55-jsonreason` reserved to a human decision. That is a question to put, and it belongs beside the recorded plain-`validate` residual in the `validation-constraints` step.

---

## (3) `R5A-3`: VALID, `medium` CONFIRMED, IN SCOPE. Five sites promise a Markdown-primary `--source` costs nothing

### Reproduced, and the falsification measured on a historical binary

All five sites open at the cited ranges: `src/main.rs:452`, `:476`, `:1096-1098`, `:1199-1201`, `src/plan/source.rs:406-411`. The divergence reproduces on identical inputs with one variable:

```
$ agent-scaffold status --json --metrics <S>/foreign/docs/metrics/workflow.jsonl
  "plan": null,  "metrics": { "records": 1 },  "metrics_absent_reason": null
$ agent-scaffold status --json --source <S>/projA/docs/plans/p.plan.toml --metrics <S>/foreign/.../workflow.jsonl
  "plan": null,  "metrics": null,  "metrics_absent_reason": "log-not-this-project"
```

`"plan": null` in both, so the Markdown-primary source is genuinely not read as a plan, which is what the sentence is about. The log is COUNTED in the first and WITHHELD in the second. `next` diverges the same way and additionally changes the derived task slug from `task` to `p`, and therefore the default ledger path.

### Scope: IN SCOPE, and the falsifier is measured rather than cited

1. Condition 1 HOLDS. `-S` returns `e05e71f` and `e30bba8` for the two `--help` sites, `e30bba8` for the `src/plan/source.rs` site and for `toml_source`. All three are ancestors of the base. All five sites were TRUE when written.
2. Condition 2 HOLDS. None of the five lines is in the range's three `src/main.rs` hunks, and `src/plan/source.rs` is not in the range diff at all.
3. Condition 3, limb 2, **FAILS**, MEASURED. My `269d075^` binary counts the foreign log with the Markdown-primary `--source` supplied. The current binary withholds it. The two populations the sentence groups behaved alike before that commit and diverge after it. `269d075`'s own message names the case by name: "a Markdown-primary `--source` with no `--plan`". That is this step's own inc2, so the second limb fails, exactly as it fails for `R4B-1` and `R3B-3`.
4. Condition 4 fails too, since three of the five sites share their sentence class with `R5A-1`'s secondary sites.

IN SCOPE. The brief flagged this as condition 3's second limb and that is the right reading; the reviewer's phrasing "all predate the step" is about condition 1, which holds and is not sufficient on its own.

### Severity: `medium` CONFIRMED, and I weighed `high` and `low`

AGAINST `high`: the divergent behaviour is the DESIGNED and CORRECT one, it is the fail-safe direction, and it reports its own reason on both surfaces. Nobody is led into a wrong action, only into surprise.

AGAINST `low`: two of the five sites are clap `--help` text, which is the tool's primary self-description and the first thing an operator reads. The promise is precisely that supplying this flag COSTS NOTHING, and the actual cost is that a run stops counting the log it counted a moment earlier. Five sites of one false universal across three files is also the twin-site pattern this task records being bitten by repeatedly. `medium`.

### Minimal remedy: DELETION at all five, nothing authored

Delete the trailing "so ... is unaffected" clause at each site. Every sentence carries its real content, which substrate is read, before that clause. The clause is an affirmative exhaustiveness claim about derived output, which is the class the human authorised DELETING rather than narrowing, and commit `8060898` ("docs: drop an exhaustiveness claim the Err arm's message outgrew") already applied that class inside this step. I verified `8060898` exists in this tree.

`src/workflow.rs:151-152` carries a sibling "byte-for-byte unaffected" sentence. The reviewer confirmed it TRUE as scoped to `check_workflow`'s own path and I confirm it must NOT be changed. My own sweep of "is unaffected" across `docs/`, `README.md`, `pack/` and `CHANGELOG.md` found no further site of this claim class: the hits are about `--ledger-fragment`'s `requires` constraint, about acceptance check 10's plain-`validate` claim and about a different step, none of which is this sentence.

---

## (4) `R5A-4`: DISMISSED. `low`, so no backstop re-check is owed

Its DISTINCTNESS FROM `R1A-7` IS ESTABLISHED and I dismiss it on the evidence rather than as a re-raise. The round 1 triage dismissed `R1A-7` partly on the ground that its subject, the ENCLOSING STRUCT comment at `:561-567`, "is a different comment, is not on the closed list". `R5A-4`'s subject is the FIELD comment's second sentence, and the same triage's own words put that site ON the list: "`Q-55-currencyscope` names `Projection.plan`'s doc comment, which is the FIELD comment at `src/main.rs:570-571`". The reviewer is right about this and I confirm it.

### What reproduces, and the control that refutes the inference

The transcript reproduces literally. Five inputs, one bare `null`, three different stderr notes. But the transcript does not demonstrate what the finding claims. THE CLAIM IS THAT "the tool distinguishes causes that the comment says number one." I ran the control the reviewer did not, holding the surface fixed and varying only the route:

```
route:                                          a       b       c       d       e
status  (human)   ->  plan:                 not provided everywhere, byte-identical
status  --json    ->  "plan":               null everywhere, byte-identical
next              ->  source:               no plan source everywhere, byte-identical
```

All five routes produce byte-identical output for the field the comment is about, on all three surfaces. The tool does NOT distinguish the causes of `plan`'s absence. It distinguishes whether the ANCHOR THE OPERATOR TYPED exists and parses, which is a different mechanism (`note_missing_anchors` and `toml_source`) reporting a different fact.

THE REVIEWER'S OWN QUOTED EVIDENCE CUTS THE OTHER WAY. It quotes `note_missing_anchors`' doc comment as support: "`source: no plan source` prints identically for 'no plan was asked for' and 'the plan you named is not there'". Read in full, that sentence ASSERTS the identity the finding denies, and is the premise the stderr notes exist to remedy at the anchor level precisely because the field level cannot carry it.

### The artifact's own settled definition covers route (b), which is the route the finding turns on

Sidecar `:237` defines the single cause: "that field has exactly ONE cause (no readable plan source was given), so a reason field there would carry a single value and inform nobody". Acceptance check 22, which INC4 ITSELF AUTHORED, closes by routing the reader to exactly that text: the correction "does not add a reason beside it, which stays refused for the reason given at the end of the `Q-55-jsonreason` section".

Route (b) is a Markdown-primary `--source`. The finding argues it is a plan source that is "SUPPLIED, PRESENT, READABLE and VALID", so it breaks the single-cause reading. THE TOOL'S OWN VOCABULARY DISAGREES: with a Markdown-primary `--source` supplied, `next` prints `source: no plan source`, identically to supplying nothing. Under the artifact's own definition, route (b) IS "no readable plan source was given". The finding asserts the contrary rather than measuring it, and the measurement goes against it.

### Weighed before dismissing

I weighed VALID AT `low`. "There is exactly one cause" is formally an exhaustiveness claim, and round 3 ruled `R3B-1`'s "only" VALID on that form. What defeats it here is that the claim is not FALSE: the five routes are one state by the artifact's definition and by the tool's own output, and the second sentence's meaning is unchanged by inc4's edit to the first. Inc4's edit added a MENTION of `--source` as a supplier; it did not create route (b), which has existed since `e30bba8`, well before the step.

DISMISSED. `low`, below the backstop severity, so no independent re-check is owed.

RECORDED FOR THE HUMAN RATHER THAN HIDDEN, because this is the cap round and the call is close: if a future reader disagrees with me, the remedy is one clause and costs nothing. `src/main.rs:571` becomes "It carries no reason field." The design rationale already lives at sidecar `:237` where a human decided it. I do not prescribe it, because I do not find the sentence false, and this project's settled line is that a compressed sentence whose fuller reading its own author states elsewhere is under-description rather than mis-statement.

---

## (5) `R5B-1`: VALID, severity CORRECTED DOWN from `medium` to `low`, IN SCOPE

### Reproduced against the increment's own commit range

`git diff -U0 800d359 HEAD -- src/main.rs` has exactly THREE hunks: `:461` (`StatusArgs::resume`'s `--help`), `:570` (`Projection.plan`'s doc) and `:1191-1198` (`run_status`'s comment). The impact list's `src/main.rs` bullet at `:386` names two of the three. The third gets no affirmative bullet. Confirmed, and the built binary prints the corrected help text, so the FIX is correct and only the list entry is missing.

### New omission, or one the round 4 remedy did not reach? IT DID NOT REACH IT, and the evidence is explicit

The brief asked me to settle this and the answer is unambiguous. I read the impact list at `6ec1955`, the round 3 fix tip, BEFORE round 4 touched it: `:461` had no affirmative bullet there either. So the omission predates round 4 entirely.

More decisively, the round 4 reviewer's own file records why it was not fixed. `workflow-enforcement-tier-inc4-r4-crossartifact-opus.md:158` reads: "The list names `Projection`'s `plan` doc comment and the `StatusArgs::resume` help string, and stops there." The round 4 reviewer COUNTED the exclusions-bullet mention as the list naming the site, enumerated three other omissions, and the round 4 triage prescribed remedies for those three.

**SO THIS IS THE ENUMERATION BOUND RECURRING, NOT NEW AUTHORED FALSITY.** That is the systemic defect the round 2 triage diagnosed and the round 3 triage refined into a rule: a remedy's reach is set by the reviewer's enumeration rather than by the class. It has now produced a finding in rounds 2, 3, 4 and 5. This matters for the human's reading of the loop, because it means round 5's residue finding is NOT evidence that round 4's authored prose introduced something false. Round 4's bullets are accurate; the enumeration behind them was short by one, and had been short by one since round 3.

### Scope: IN SCOPE, not close

Condition 1 FAILS: the entire `INC4:` section was authored at the base commit `c775872` and amended in range at `51f7c79`, `6ec1955` and `507456a`. Condition 2 FAILS for the same reason.

### Severity: CORRECTED DOWN to `low`. I do not confirm `medium`

The reviewer rated `medium` "on the same ground round 4's triage used for `R4B-2` itself". THAT GROUND DOES NOT TRANSFER, on two counts, and I checked both.

- `R4B-2` was `medium` because three sites were GENUINELY ABSENT from the list. `:461` is NOT absent. It is named in the same list, by exact symbol (`src/main.rs:StatusArgs::resume`'s `--help` text), and expressly identified as something "inc4 corrects". A reader of the whole list cannot come away believing inc4 did not touch it.
- `R4B-2`'s decisive aggravation was that one omission DIRECTLY CONTRADICTED acceptance check 21 three sections above it in the same file. Nothing here contradicts anything. I looked for a contradiction and there is none.

What survives is real: after `R4B-2`'s fix the list's convention is one affirmative bullet per edited site stating its acceptance-check coverage, and `:461` is the one edited site with no such bullet and no coverage statement. A reader using the affirmative bullets as the enumeration is short by one, and the step closes with this list as the permanent record. That is a `low`.

The severity correction changes nothing about the round's outcome, which is new-valid on the three `medium` findings alone.

### Minimal remedy: AUTHORED PROSE, about twelve words, and this is the one that cannot be a deletion

Append to the existing `src/main.rs` bullet at `:386`: ", and `StatusArgs::resume`'s `--help` string, which no acceptance check states either". Every fact in it is already settled, by `Q-55-helpsurface` and by `b1a7ab6`'s own commit message. The twin at `docs/plans/agent-scaffold.md:1780`-equivalent follows from the re-render, which check 23 requires.

I LOOKED FOR A DELETION-CLASS ALTERNATIVE AND THERE IS NONE THAT IMPROVES THINGS. Deleting the exclusions bullet's clause would remove the only mention of the site and make the list strictly worse. This finding is the reason a deletions-only fix pass cannot close round 5 completely.

---

## (6) `R5B-2`: VALID, `low` CONFIRMED, IN SCOPE

### Reproduced exactly, and the reviewer's count is right

I ran the check's own prescribed procedure, a literal search of each re-tensed quotation against the revision its tense names, on all six non-ellipsis historical quotations the reviewer identified, and then re-ran each with comment markers and whitespace normalised:

| Sidecar site | Literal single-line grep | Marker and whitespace normalised |
| --- | --- | --- |
| `:199` `no_active_loop_reason`'s doc, at `8beb1c2^` | MISS | MATCH |
| `:200` `NextProjection`'s own doc, at `8beb1c2^` | MISS | MATCH |
| `:201` `Projection`'s doc, at `8beb1c2^` | MISS | MATCH |
| `:202` `resume_state`'s doc, at `8beb1c2^` | MISS | MATCH |
| `:347` check 22's quote, at the build pass parent | MATCH | MATCH |
| `:367` `run_resume`'s doc, at `8beb1c2^` | MATCH | MATCH |

FOUR OF SIX. The quotations are TRUE and correctly re-tensed; only the procedure check 21 states fails on them, and it fails purely because the Rust `///` lines wrap at a point the quotation crosses. The reviewer's ruling on the wrinkle the task posed is correct and its count of four, not two, is correct.

### Scope: IN SCOPE

Conditions 1 and 2 both FAIL. Check 21 was authored at the base commit `c775872` and its block was edited by EVERY subsequent pass, most recently at `507456a`, which is the very clause the finding is about.

### Severity: `low` CONFIRMED, and I weighed `medium`

FOR `medium`: check 21 is the increment's principal acceptance criterion, and under a literal reading it fails right now against its own file, which is the "definition of done contradicts its own acceptance criterion" shape that carried `medium` at `R3B-2` and `R4B-2`.

WHAT HOLDS IT AT `low`, and it is a measured distinction rather than a preference: the check is AMBIGUOUS AND RESOLVED, not failing. Three separate executors have run it and all three reported it passing, and round 3's triage explicitly resolved the ambiguity by running "an independent whitespace-normalised sweep" and recorded that it had. Nobody has been misled, no live text is false, and the ambiguity is inherited from the pre-existing "literal search" sentence rather than created by the clause `R4A-2` added, whose own severity was `low`. `low`.

### Minimal remedy, and I record BOTH options because the human's deletion question turns on it

- TOKEN, and the truer of the two: append ", with comment markers and whitespace normalised" once to the existing "as a literal search" sentence, which is the general rule rather than the new clause. It names a procedure round 3's triage already ran; it invents nothing.
- DELETION, available and cheaper but with a stated cost: delete the single word `literal`. That removes the failing prescription, but it also weakens the guard the check states its own ground for ("it is NOT re-pointed at a similar-looking sentence"), so it trades a false red for a weaker check.

I AGREE WITH THE REVIEWER THAT A NORMALISATION PARAGRAPH MUST NOT BE ADDED. Check 21 has been amended three times in two rounds and has produced a finding after every amendment. Its third option, recording this as an accepted procedural note for the next reviewer, is also defensible and authors nothing in the check itself.

---

## The disagreement between the two lenses, adjudicated

The exhaustion audit concludes the remaining unexamined space is "small, and now measured", puts the unread surface at eight sidecar lines plus four in `src/next.rs`, and recommends accepting. The source-claims lens, running blind to it at the same time, found four findings on a source-side surface. **The human needs one answer and here it is: THE SOURCE-CLAIMS LENS IS RIGHT ON THE QUESTION BEING DECIDED, AND THE AUDIT IS RIGHT ABOUT A SMALLER SPACE THAN THE ONE IT SPOKE FOR.**

Both are partly right, so here is exactly where the line falls.

**WITHIN THE PLAN DOCUMENTS, THE AUDIT IS SUPPORTED, AND ITS PREDICTIONS LANDED.** It said that if a further round found anything it would be at `:345` or `:385-389`, that it would be re-seeded rather than discovered, and that it would be `low` or `medium` and never higher. Round 5's plan-document findings are `R5B-2` at `:345` and `R5B-1` at `:385-389`. Both sites, exactly. Both re-seeded rather than discovered. Both `low` after my correction. That is a prediction made blind and confirmed on both coordinates, and it is strong evidence that the audit understands the plan-document surface.

**OUTSIDE THEM, THE AUDIT'S FRONTIER IS REFUTED, AND THE BRIEF'S HYPOTHESIS ABOUT WHY IS CORRECT.** Its consolidated-residue table has NO ROW for the source-side claim surface. Its "unread surface is eight lines" is a measurement of TEXT THE INCREMENT CHANGED, and its "never-verified residue is empty" is a measurement of FIGURES REVIEWERS DECLINED. Neither is a measurement of source-side claims about behaviour the step changed. The source-claims lens then measured exactly that surface at 156 claims and returned 10 false ones, none of which lies inside the audit's twelve unread lines.

THE MECHANISM IS THE ONE THE BRIEF SUSPECTED, AND THE AUDIT NAMED IT ITSELF WITHOUT APPLYING IT. Its frontier is the UNION OF TWELVE PASSES' SELF-REPORTED BLIND SPOTS. A reviewer does not report as "not reached" a region it never framed as in scope. Eleven of the twelve passes were pointed at plan documents, and not one of them wrote "I did not inventory the doc comments in `src/`", so that surface is invisible to a union of their disclaimers. The audit's own blind-spot section states this exactly ("An audit of coverage can only see what the record says, and reviewers systematically under-report what they did not do") and then does not carry it into part D's conclusion. Its own part A even records the tell: `R4B-1` came from "an artifact NO LENS IN FOUR ROUNDS HAD OPENED", and pass 12 opened `src/next.rs` for the first time in four rounds. A frontier built from twelve self-reports had just been shown to miss a whole file, one round earlier.

SO THE PRECISE LINE: the audit's conclusion is sound as a statement about the sidecar, the rendered view, the plan TOML regions and the four sibling sidecars. It is unsound as a statement about the increment, because the increment's subject reaches source-side claims and the audit's denominator did not. The correct reading of "small, and now measured" is "small relative to what twelve plan-document passes believe they covered", which is the bound the audit itself wrote down.

I DO NOT TREAT THIS AS A DEFECT IN THE AUDIT'S CONDUCT. It ran a lens with genuinely zero prior coverage, closed five never-verified claims, and stated its own bound honestly. The error is in the SCOPE OF THE CONCLUSION, not in the work.

---

## The audit's correction of the orchestrator's diagnosis, checked

The brief asked me to verify this arithmetic myself because it will be recorded permanently. I re-ran every command in the audit's part B against this tree.

**EVERY TRANSCRIPT REPRODUCES BYTE FOR BYTE.** Line 157 of the sidecar is `md5 674ac31d0ee208b517026e40b78919b9` at all three of `363ac06`, `ce65169` and `5eeb93b`, exactly as reported, so round 1's 100-percent sweep passed a line that round 2's identically-stated 100-percent sweep found false. All four fix-pass touched-line sets reproduce exactly (`195 201,2 206 255 257 259 304 308 339 345,2` / `157 195 204 206 282 304 342 346 386` / `104 157 163 179 345 367 388` / `14 217 229 345 385 387,3`). Both `git log -L` block histories reproduce exactly, five commits each. `git diff --stat b1a7ab6 cf9ff9c` reproduces exactly.

**THE ATTRIBUTION TABLE CHECKS OUT.** Round 2's four re-seeded findings sit at `:195`, `:206`, `:304` and `:346`, and all four are in the round 1 fix set. Round 3's sit at `:157` and in the `:345-346` block; `:157` and `:346` are both in the round 2 fix set. Round 4's sit at `:345` and `:388`, both in the round 3 fix set. The "genuinely new" columns are the complements and they add up: 4+4+1=9, 2+3+1=6, 2+2=4.

**THE CONCLUSION THAT THE DIAGNOSIS IS HALF RIGHT IS CORRECT AND I ADOPT IT.** The recorded single-cause account, that each round's findings came from a lens type never run before, is refuted as a complete account. Round 2's cold read was itself a repeat of round 1's completeness lens by both files' own stated method, and it produced four of round 2's nine in-scope findings, one of them on a byte-identical line the earlier sweep had passed. Fixes do re-seed.

**BUT ONE NUMBER IN IT IS WRONG, AND IT IS IN THE HEADLINE.** The audit's evidence-2 heading reads "Fixes demonstrably re-seed, AT A RATE THAT RISES ACROSS ROUNDS." Its own table does not support "rises", under either available reading:

| Round | Strict column (on the previous fix's lines) | Inclusive (adding `R2A-3`, `R3B-1`) |
| --- | --- | --- |
| 2 | 4 of 9 = 44.4 percent | 5 of 9 = 55.6 percent |
| 3 | 2 of 6 = 33.3 percent | 3 of 6 = 50.0 percent |
| 4 | 2 of 4 = 50.0 percent | 2 of 4 = 50.0 percent |

The strict series falls then rises. The inclusive series falls then flattens. Neither rises. The ABSOLUTE counts fall monotonically: 4, 2, 2.

**THE DEFENSIBLE STATEMENT, which is what should be recorded instead:** across rounds 2, 3 and 4 the re-seeded share of each round's in-scope findings is ROUGHLY STABLE at between a third and a half, while the absolute count falls with the round total. Re-seeding is therefore a PERSISTENT PROPORTION of a shrinking population, not an accelerating one. That is still a strong result and still refutes the single-cause diagnosis; it is simply not the stronger result the audit claimed.

I flag this prominently because the brief asked me to: a wrong number here would have joined the three counts this increment has already had to correct, and this one was heading into a permanent calibration record.

**ONE PREDICTION OF THE AUDIT'S IS CONFIRMED BY THIS VERY ROUND AND IT COULD NOT HAVE KNOWN IT.** It measured that acceptance check 21 was edited by the build pass and by all four fix passes and produced a finding after each, naming `R1B-2`, `R2A-2`, `R3C-4` and `R4A-2` for the first four. Round 4's fix edited `:345` a fifth time. `R5B-2` is a finding on `:345`, on the exact clause that fifth edit added. FIVE EDITS, FIVE FINDINGS. The pattern is unbroken and is now the best-evidenced calibration result in this loop.

---

## Recorded residuals, settled dismissals and declined items

I checked every finding against inc2's four (the in-root bound; the single-anchor `..` case with its widened bound; `ADV-2`'s rejected-ledger context slot; `R2A-2`'s off-convention `--source` surface), inc3's four (`R3A-1`'s inert remedy clause; `R4A-1`'s reader-level discrimination, both INC3-era ids; the plain-`validate` mode-000-file-versus-unsearchable-directory inconsistency; the containment TOCTOU), `F-5`, the five settled dismissals (`R1A-5`, `R1A-7`, `R1A-8`, `R1B-3`, `R2A-5`), the six valid-but-out-of-scope findings of rounds 3 and 4, and the four declined items (`run_validate`'s clap-required claims, `src/next.rs:162` and its `active_loop` disjunct, `docs/plans/agent-scaffold.md:7`, and the `src/checks.rs` citations in `checks-runner-worktree-name-collision.md`).

NOT ONE OF THE SIX FINDINGS RE-RAISES ANY OF THEM. The two that come closest are ruled explicitly above: `R5A-2` against the plain-`validate` residual, on three separable grounds, and `R5A-4` against the `R1A-7` dismissal, where the distinctness holds and the dismissal rests on new evidence instead. No finding concerns line length or prose wrapping.

I confirm the exhaustion audit's own check of the same list. Its part C touched `justfile:46-48`, the exploration records, the deleted inc1 and inc3 findings files and the ledger's derivation, none of which is a residual or a declined item.

---

## For the human at the escalation

**WHAT THE ARTIFACT'S STATE ACTUALLY IS.** The plan documents are in good condition and are now well evidenced as such. Twelve passes have swept the sidecar end to end at least three times, executed all 33 acceptance checks from their own preconditions, cross-checked 71 multiply-stated facts, and this round closed the last five claims that no pass had ever verified. Round 5 found two things in that whole space, both `low`, both on the two sites the exhaustion audit named in advance as the only hot ones. Nothing in five rounds has found a defect in what the tool DOES. There is still no `high` and no `critical` anywhere in this loop.

**WHAT REMAINS.** Three `medium` findings, all of them false statements in `src/` about behaviour this step introduced, on a surface eleven of the twelve prior passes were not pointed at. Two are strings an operator reads; the rest are doc comments and `--help` text. One of them, `R5A-1`, means a round 4 finding you were told was closed is closed at two sites of four, and the two it never reached are the two an operator actually sees. The remedies for all three are deletions and single tokens. Two `low` findings remain in the plan documents, one of which, `R5B-1`, needs about twelve authored words and is the only thing here a deletions-only fix pass cannot close.

**WHAT I WOULD WANT TO KNOW IF I WERE DECIDING.**

First, whether the increment's declared scope was ever meant to include the source-side claim surface. `Q-55-currencyscope` closed inc4's scope around the sidecar, three sibling sidecars and `Projection.plan`'s doc comment. Every source-side finding since round 3 (`R3B-3`, `R4B-1`, and now `R5A-1`, `R5A-2`, `R5A-3`) has been ruled in on the same precedent, that a claim this step's own change falsified is in scope regardless of authorship. That precedent is settled and I applied it. But it means the increment's real subject has grown, one triage ruling at a time, well past the boundary you drew, and nobody has put that to you as a question. If the answer is that the source surface belongs to a later step, three of this round's five findings change category.

Second, how much weight to put on a review process that has now demonstrated it can be exhausted only within the frame it is pointed at. Three separate clean results were offered as evidence of exhaustion, and a fourth lens pointed one artifact-class sideways returned 10 false claims out of 156 on its first run. The honest reading is that this artifact converges per-lens and not globally, and that the next unrun lens is worth more than another round of run ones.

Third, whether the reason this loop never converged is now settled well enough to be worth the rounds it cost. I think it is. The calibration result, once its one wrong word is corrected, is that BOTH causes are real: roughly a third to a half of each round's findings are re-seeded by the preceding remedy, at a stable proportion, and the rest are new ground reached by a new lens. Acceptance check 21 has produced a finding after all five times it was edited, without exception. That is a transferable result about how this project should budget review rounds and constrain fix passes, and it did not exist before this loop.

I present these as what I would want to know. The options are the orchestrator's to put and the decision is yours.
