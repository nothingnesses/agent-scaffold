# `workflow-enforcement-tier-inc2`, work review ROUND 4, ISOLATED TRIAGE

ARTIFACT. Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage-inc2-r4`, commit `94eb8ec` ("fix: scope the guessed anchor root and split \"missing\" from \"cannot tell\""), which is the same tree the three reviewers reviewed as `b54ba3a` in their own worktrees. The increment is the five commits `9dc41c6`, `33cefc2`, `af59695`, `7df4c94`, `94eb8ec` above `main`; the round 3 fix alone is `git diff HEAD~1..HEAD`. Specification: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`.

SOURCE FILES TRIAGED.

- `workflow-enforcement-tier-inc2-r4-reviewer-adversarial.md` (R4A-1 `high`, R4A-2 `medium`, R4A-3 `low`).
- `workflow-enforcement-tier-inc2-r4-reviewer-claims.md` (R4C-1 `medium`, R4C-2 `medium`).
- `workflow-enforcement-tier-inc2-r4-reviewer-regression.md` (R4R-1 `low`, R4R-2 `low`).

Rounds 1 to 3 reviewer files and all three prior triages in this directory were read before any fixture was built. Nothing settled there is re-litigated here.

BINARIES I BUILT AND COMPARED. Four, each `cargo build --release` from a `git archive` export into my own scratch directory, plus the worktree build at HEAD: `HEAD` (`94eb8ec`), `HEAD~1` (`7df4c94`, the round 3 PRE-FIX binary), `HEAD~2` (`af59695`, the pre-round-2-fix binary that G-EMPTYROOT was filed against), and `main` (pre-increment). Baseline suite at HEAD, measured myself: `cargo test --release` -> 418 passed, 0 failed across nine binaries (378 + 5 + 1 + 1 + 9 + 3 + 18 + 1 + 2), which matches both reviewers who reported it.

ONE PROCESS NOTE, RECORDED BECAUSE IT NEARLY COST ME A WRONG VERDICT. Partway through I ran a bisect against a binary I had left in a mutated state (mutation N2 still compiled in), and it produced a confident REFUSAL where the pristine binary leaks. I caught it by re-running the original attack, rebuilt the pristine binary, and re-ran every affected measurement. Every result reported below was taken either before any mutation or after the rebuild, and the two large spot-checks in section 5 were additionally re-run on the rebuilt binary and came out byte-identical. The mutation-era readings are discarded and none of them supports any verdict here.

---

## 1. THE RULING ON R4A-1

**VALID. `high`, UPHELD. It is a genuine REGRESSION introduced by `94eb8ec`, and it is a partial RE-OPENING of the round 2 `high` `G-EMPTYROOT` rather than a merely similar defect.**

I dismissed and downgraded nothing in the `high` band. The only severity movement in this round is downward on two `medium`s that merge into one `low` (section 3).

### 1.1 Reproduction, on my own fixture, on both binaries

Fixture `<S>/tri4-r1new` and `<S>/tri4-r1fresh`, built by `<S>/tri4-repro.sh`: two TOP-LEVEL SIBLING projects `alpha` and `beta` with no containment relation, alpha holding a one-record log and an `ALPHA PRIVATE RESUME STATE.` ledger, beta holding a three-record log and a `BETA PRIVATE RESUME STATE.` ledger. `--source` names a plan in ALPHA that has not been written. `--plan` names a path in BETA. The CONTROL and the ATTACK differ by ONE CHARACTER, a trailing slash on an existing beta file, and no file is created, deleted, moved or `chmod`'d between them.

CONTROL, `--plan <R>/beta/docs/plans/nope.md`, at HEAD:

```
note: --source <R>/alpha/docs/plans/ghost.plan.toml does not exist
note: --plan <R>/beta/docs/plans/nope.md does not exist
task: ghost
metrics: unavailable, the round log <R>/beta/docs/metrics/workflow.jsonl is not under the plan's project root <R>/alpha, so its records cannot be paired with this plan
the ledger <R>/beta/docs/plans/b.ledger.md is not under the plan's project root <R>/alpha; nothing to resume
exit=0
```

ATTACK, `--plan <R>/beta/docs/plans/b.md/`, at HEAD:

```
note: --source <R>/alpha/docs/plans/ghost.plan.toml does not exist
note: --plan <R>/beta/docs/plans/b.md/ could not be checked: Not a directory (os error 20)
task: ghost
metrics: 3 records

RESUME STATE (verbatim from the ledger):
## RESUME STATE

BETA PRIVATE RESUME STATE.
exit=0
```

The machine surface on the same anchors:

```
{
  "task": "ghost",
  "metrics": { "records": 3 },
  "metrics_absent_reason": null,
  "resume_state": "## RESUME STATE\n\nBETA PRIVATE RESUME STATE.",
  "resume_state_absent_reason": null,
  "no_active_loop_reason": "no-plan-steps"
}
exit=0
```

`status --resume` echoes BETA's block and `status --json` reports `"records": 3` with `metrics_absent_reason: null` on the same anchors. Five surfaces, all leaking, all at exit 0. Every clause of the adversarial lens's transcript reproduced verbatim on my fixture.

### 1.2 It is a regression, and the differential is unambiguous

The identical script against all three older binaries:

| Binary | CONTROL | ATTACK |
| --- | --- | --- |
| `HEAD` (`94eb8ec`) | refuses, both reasons populated | LEAKS, both reasons `null` |
| `HEAD~1` (`7df4c94`) | refuses, both reasons populated | refuses, both reasons populated |
| `HEAD~2` (`af59695`) | leaks | leaks |
| `main` | leaks | leaks |

`HEAD~1` refuses on the identical command line, and the CONTROL block is byte-identical between `HEAD` and `HEAD~1`, so the change is confined to the attack and it arrived at `94eb8ec`. That settles brief question 1: it is NEW at this commit and did not exist earlier in a different form. `HEAD~2` and `main` leak on BOTH rows, which is `G-EMPTYROOT` and the pre-increment state respectively, not an earlier form of this defect.

THE PART THAT DECIDES THE SEVERITY. I diffed HEAD's leaked stdout against `HEAD~2`'s leaked stdout on the same command shape:

```
$ diff <S>/tri4-head-attack.out <S>/tri4-pre2-attack.out && echo "BYTE-IDENTICAL"
BYTE-IDENTICAL
```

HEAD's output on the attack is byte-for-byte the output of the binary `G-EMPTYROOT` was filed against. This is not a defect that resembles the closed `high`; on this population it IS the closed `high`, reached through a different branch of the same function.

### 1.3 How reachable it is, measured rather than argued

The mechanism is `Path::try_exists` returning `Err` for every `stat` failure except `ENOENT`. I built four trigger classes and ran each against both binaries. Three need no permission manipulation at all:

- `ENOTDIR`, a trailing slash on an existing file (`.../b.md/`): LEAKS at HEAD, refuses at `HEAD~1`.
- `ELOOP`, an anchor path through a symlink loop: LEAKS at HEAD ("Too many levels of symbolic links (os error 40)"), refuses at `HEAD~1`.
- `ENAMETOOLONG`, a component over `NAME_MAX`: LEAKS at HEAD ("File name too long (os error 36)"), refuses at `HEAD~1`.
- `EACCES`, an anchor under a directory the process cannot traverse: LEAKS at HEAD when the anchor sits under the `docs/plans` convention, refuses at `HEAD~1`.

ONE CORRECTION TO THE ADVERSARIAL LENS, in its own favour on severity and against it on precision. Its "four independent trigger classes and all four reproduce" is true only for spellings whose derived root contains the artifact. I measured `--plan <R>/beta/locked/x.md` with `locked` at mode `000` and OUTSIDE `docs/plans`: the derived root is `<R>/beta/locked`, beta's own log is not under it, and the run REFUSES. Moving the same locked directory under `docs/plans` makes the derived root `<R>/beta` and the leak fires. So the misclassification is universal across the four classes; the LEAK additionally requires the derived root to contain the artifact. Recorded so the next round does not over-claim.

AND ONE CORRECTION AGAINST THE LENS'S OWN SEVERITY DISCOUNT, which is the more important of the two. The lens holds that the population is "one typo plus one unusual spelling" and says that is the only reason a triager might land on `medium`. That framing is too narrow. I reproduced the leak IN FULL with no unusual spelling anywhere, using an anchor that is an entirely ordinary path to a plan that GENUINELY EXISTS but sits under a directory the process cannot traverse:

```
$ agent-scaffold next --source <R>/alpha/docs/plans/ghost.plan.toml \
      --plan <R>/gamma/docs/plans/locked/p.plan.toml \
      --metrics <R>/gamma/docs/metrics/workflow.jsonl \
      --ledger-fragment <R>/gamma/docs/plans/g.ledger.md
NEW: note: --plan .../locked/p.plan.toml could not be checked: Permission denied (os error 13)
     metrics: 4 records
     GAMMA PRIVATE RESUME STATE.
OLD: metrics: unavailable, the round log ... is not under the plan's project root <R>/alpha
     the ledger ...; nothing to resume
```

Both anchors here are spelled exactly as an operator would spell them. The `--source` names a plan not yet written, which is the NORMAL state the `Q-55-emptyroot` decision was granted for ("a plan file that has not been written yet still reads its own project's log"), and the `--plan` names a real file in a directory the process happens not to be able to traverse, which is an ordinary condition in a shared checkout, a sandboxed CI step, or a tree with a restricted subdirectory. The population is therefore "one normal state plus one environment condition", not "two operator errors".

### 1.4 Severity, calibrated against the two prior `high`s

I apply the round 2 triage's own binding table (its section 1.6), which calibrated `G-EMPTYROOT` against round 1's `ADV-1`. Every row verified by me, not read across:

| | `ADV-1` (round 1, `high`) | `G-EMPTYROOT` (round 2, `high`) | R4A-1 |
| --- | --- | --- | --- |
| Payload | another project's `## RESUME STATE` verbatim plus a foreign record count | identical, on `next`, `status`, `status --resume` | identical, on all five surfaces, and BYTE-IDENTICAL stdout to the `HEAD~2` leak |
| Machine surface | `resume_state_absent_reason: null` | plus `metrics_absent_reason: null` | both `null`, verified |
| Exit code | 0 | 0 | 0 |
| Fabricated `next:` instruction | No | No | No (`active_loop: null`, `no-plan-steps`) |
| A surface that tells the truth | Yes (`status --resume` refused) | No | No |
| Suite visibility | None | None | None, and I PROVED it by mutation (section 1.5) |
| Documentation falsified | `README.md:236` sentence 1 | `README.md:236` sentences 1 and 2 | `README.md:236`, `CHANGELOG.md:23` and three clauses of `resume_roots`'s own doc comment |
| Trigger | one explicit flag, no typo | one typo plus one explicit flag | one not-yet-written plan plus one unstat-able anchor; no typo required |

Every aggravator the round 1 and round 2 triages named survives here, and the documentation row is worse than either. Two mitigations exist and I weighed both:

- THE POPULATION IS NARROWER THAN `G-EMPTYROOT`'S. True: `G-EMPTYROOT` fired on any single missing anchor, this needs a second anchor in the Err class. But section 1.3 shows that class is reachable with ordinary spellings and no privileges, and narrowness of population has never been the project's severity axis; the round 1 triage's stated standard is "a defect a user can hit today", and this is live on the shipped binary.
- STDERR PRINTS AN HONEST `note:`. True, and it is better than `HEAD~2`, which printed nothing. But the note says "could not be checked" and says nothing about containment being switched off, and on `--json` it is on stderr while the reason fields on stdout positively assert the artifacts are this plan's. The round 2 triage already ruled that a truthful human line does not offset a machine surface asserting the opposite.

Against those sits a NEW aggravator neither prior `high` had: this is a REGRESSION of a closed `high`, introduced by a fix pass, and it lands on the machine surface that `next` exists to drive an agent with. `task: ghost`, derived from the ALPHA anchor, is printed three lines above BETA's private resume state in the same output.

RULING: `high`. If a backstop wishes to overturn this, the thing it must beat is the byte-identity in section 1.2 and the ordinary-spelling reproduction in section 1.3, not the trailing-slash framing.

### 1.5 The remedy, and its cost, measured end to end

THERE IS NO REMEDY THAT AVOIDS `resume_roots`. I looked. The deciding-set rule exists only at `src/main.rs:1566-1571`, and neither `containment_roots`, `checked_plan_root`, `resolve_for_containment` nor `note_missing_anchors` can express it. The honest options all touch the function, and the question is how much.

THE PRESCRIBED FIX IS ONE TOKEN. `src/main.rs:1569`:

```rust
supplied.iter().copied().filter(|anchor| anchor.try_exists().unwrap_or(true)).collect();
                                                              ^^^^ -> false
```

This is a fifth touch of `resume_roots` but it is NOT a fifth rewrite. The deciding-set rule, the `on_disk.is_empty()` fallback and the mapping to roots are all untouched; only the polarity of the `Err` classification inside the existing filter changes. It is the first change to this function that REMOVES a special case rather than adding one, and it is monotone: `on_disk` can only shrink, and `deciding` falls back to `supplied` when `on_disk` is empty while `supplied` is non-empty whenever an anchor was supplied, so no path can reach an empty vector and `G-EMPTYROOT`'s failure mode is structurally unreachable under it.

I APPLIED IT AS A TEMPORARY MUTATION AND MEASURED THE WHOLE BATTERY, then reverted it:

| Check | Result under the one-token fix |
| --- | --- |
| `cargo test --release` | 418 passed, 0 failed. UNCHANGED. |
| R4A-1's attack | CLOSED. Attack output is now byte-identical to the control, both reasons populated. |
| R4A-2's attack | CLOSED. Alpha's own log (`metrics: 1 records`) and own ledger are read. |
| `R3A-1`'s closure (C0 / C1 / C2) | PRESERVED. C1 identical to C0; C2 still refuses. |
| `G-EMPTYROOT` (rows A, B, C, D, E) | STILL CLOSED. Identical to pristine HEAD. |
| `ADV-1` (13 configurations) | STILL CLOSED. Identical to pristine HEAD. |
| `R3A-2`'s note text | UNAFFECTED. `note_missing_anchors` is not touched. |

THE COST HAS TWO PARTS, and the second is the real one.

- Touching the function a fifth time, at round 4 of a cap of 5. Section 8 weighs this against the convergence arithmetic, which has changed what that cost means.
- THE SUITE CANNOT SEE THIS LINE AT ALL. The 418 tests stay green under BOTH polarities, which is a mutation-proof that the `on_disk` MEMBERSHIP rule is unpinned in both directions. So the fix is owed two new red-then-green tests, one for each orientation of the two-anchor `Err` case (`Err` in the `--source` slot, `Err` in the `--plan` slot), and without them any disposition, including accepting the current behaviour, is unguarded.

---

## 2. VERDICT TABLE

| Raw id | Valid | Severity (mine) | Dedup group | One-line prescription |
| --- | --- | --- | --- | --- |
| R4A-1 | VALID | `high` (upheld) | G-ERRANCHOR | Classify a `try_exists` `Err` anchor as NOT on disk: `src/main.rs:1569`, `unwrap_or(true)` -> `unwrap_or(false)`, plus two red-then-green tests on the two-anchor `Err` case. |
| R4A-2 | VALID | `medium` (upheld) | G-ERRANCHOR | Same one-token fix closes it; counted separately, see section 4.2. |
| R4A-3 | VALID, MERGES with R4R-2 | `low` (upheld) | G-ERRDOC | Correct `src/main.rs:1557-1560`'s final clause; `README.md:236` and `CHANGELOG.md:23` need no edit IF R4A-1 is fixed, and need the `Err` case spelled out if it is accepted. |
| R4R-2 | VALID, DUPLICATE of R4A-3's doc-comment half | `low` (upheld) | G-ERRDOC | Same site, same clause, same fix. Counts once. |
| R4C-1 | VALID, MERGES into R4R-1 | `low` (DOWN from `medium`) | G-STALECLAIM | Qualify `src/main.rs:1150-1152`'s "still supplies a containment root". |
| R4C-2 | VALID, MERGES into R4R-1 | `low` (DOWN from `medium`) | G-STALECLAIM | Qualify `src/main.rs:1628-1629`'s "roots containment on an anchor that does not exist". |
| R4R-1 | VALID, SUPERSET of R4C-1 and R4C-2 | `low` (upheld) | G-STALECLAIM | Both sites in one sweep. Counts once for all three raw ids. |

INVALID: NONE. Every raw finding in this round reproduced against the shipped binary or the shipped text on my own fixtures.

SEVERITY MOVEMENT. Two downgrades, `R4C-1` and `R4C-2` from `medium` to `low`, both absorbed into one `low`. NO `high` OR `critical` WAS DISMISSED OR DOWNGRADED; the single `high` is UPHELD and section 1.3 strengthens its reachability argument beyond what the lens claimed.

DEDUPLICATION NOTES. R4A-3 and R4R-2 collapse completely: same file, same lines, same clause, same measurement, same correction. R4C-1, R4C-2 and R4R-1 collapse into one finding with two sites, following the round 3 precedent that gave `R3F-2` one count for three sites. R4A-1 and R4A-2 do NOT collapse; see section 4.2.

---

## 3. VALID COUNT AND ROUND OUTCOME

**NEW VALID FINDINGS. THE ROUND IS NOT CLEAN.**

VALID COUNT AFTER DEDUP: **4**.

- `high`: 1 (R4A-1).
- `medium`: 1 (R4A-2).
- `low`: 2 (G-ERRDOC from R4A-3 and R4R-2; G-STALECLAIM from R4C-1, R4C-2 and R4R-1).

---

## 4. PER-FINDING RULINGS

### 4.1 R4A-1: VALID, `high`, G-ERRANCHOR

Section 1 in full. WHAT I RAN: `<S>/tri4-repro.sh` against four built binaries; the four trigger classes individually; the default-path variant with no explicit artifact flags; the present-but-unreadable variant; a `diff` of HEAD's leaked stdout against `HEAD~2`'s. WHAT I OBSERVED: reproduced in every particular, plus two facts the lens did not have (byte-identity with the closed `high`'s output, and an ordinary-spelling trigger).

THE DEFAULT-PATH VARIANT holds too, so the finding does not rest on explicit artifact flags:

```
$ agent-scaffold next --source <R>/beta/docs/plans/b.md/ --plan <R>/alpha/docs/plans/ghost.md
NEW: note: --source ... could not be checked: Not a directory (os error 20)
     task: b   metrics: 3 records   BETA PRIVATE RESUME STATE.
OLD: note: --source ... does not exist
     task: b   metrics: unavailable, the round log ... is not under the plan's project root <R>/alpha
```

I ACCEPT THE LENS'S IN-ROOT DISCRIMINATION and checked it rather than taking it. R4A-1's fixture is two top-level siblings with no containment relation; the root that would cover both is never derived; the surviving root is `<R>/beta` while the `--source` names `<R>/alpha`; and the discriminating control holds the ENTIRE LAYOUT FIXED and varies only the stat class of one anchor. A layout-dependent explanation cannot survive a control in which no file moves. The lens's `--plan /tmp` and `--plan <a directory>` rows ARE in-root by construction and it presented them as illustration rather than reproduction, which is the correct treatment; I do not attribute the finding to them and I do not re-raise them. I also confirm the lens built its G-series disjoint control WRONG the first time (its "disjoint" beta was still under the derived root), threw it away, rebuilt it so the derived root is the fixture root, and reached the correct conclusion that THAT construction is the recorded bound. Its corrected conclusion stands.

PRESCRIBED FIX. `src/main.rs:1569`: `anchor.try_exists().unwrap_or(true)` -> `anchor.try_exists().unwrap_or(false)`. Owed with it: two red-then-green tests in `tests/unsafe_pairings_are_refused_and_omitted.rs` covering `Err` in the `--source` slot beside a missing `--plan`, and `Err` in the `--plan` slot beside a missing `--source`, each asserting the foreign artifact is refused and each asserting the machine reason is populated. Owed after it: `src/main.rs:1557-1560` rewritten (G-ERRDOC).

### 4.2 R4A-2: VALID, `medium`, G-ERRANCHOR, and it stands SEPARATELY

WHAT I RAN, on the pristine HEAD binary and on `HEAD~1`, same fixture as R4A-1, three runs of which two are controls.

```
CONTROL 1  --source <alpha m.plan.toml, EXISTS>, no --plan
           -> task: m   metrics: 1 records   ALPHA PRIVATE RESUME STATE.
CONTROL 2  the same plus an ORDINARY nonexistent beta --plan
           -> note: --plan .../nope.md does not exist
              task: m   metrics: 1 records   ALPHA PRIVATE RESUME STATE.
ATTACK     the same with the trailing slash instead
           -> note: --plan .../b.md/ could not be checked: Not a directory (os error 20)
              task: m
              metrics: unavailable, the round log <R>/alpha/docs/metrics/workflow.jsonl is not
                       under the plan's project root <R>/beta
              the ledger <R>/alpha/docs/plans/m.ledger.md is not under the plan's project root
                       <R>/beta; nothing to resume
              "metrics_absent_reason": "log-not-this-project",
              "resume_state_absent_reason": "ledger-not-this-project",
```

`HEAD~1` produces the SAME refusal on the ATTACK line, so the behaviour is UNCHANGED across the round 3 fix. THE LENS'S CHARACTERISATION IS CORRECT: this is a residual, not a regression. It is `R3A-1` surviving on the population `R3A-2` was filed about, and all three grounds the round 3 triage upheld `R3A-1` on at `medium` apply here without alteration: the human's own `Q-55-emptyroot` text declined an option because it "would also omit an artifact legitimately belonging to the anchor's own directory"; the machine reason is a positive false assertion about the project's OWN log rather than a silence; and the direction is a refusal at exit 0 with a `note:` printed, which is fail-safe and holds it at `medium` rather than higher.

IT DOES NOT MERGE WITH R4A-1, and I apply the round 2 triage's own merge test. `R2A-1` and `FV-1` collapsed because they shared code path, trigger, fixture shape AND symptoms. R4A-1 and R4A-2 share the code path and the trigger class but differ on the other two: the fixture shape differs (other anchor MISSING versus other anchor EXISTING) and the symptoms are opposite in direction (a foreign read versus a refusal of the project's own artifacts). They also differ on the fact the round turns on, which is that one is new at this commit and the other is not. Grouped as G-ERRANCHOR because one token closes both; counted twice because they are separately reachable populations with opposite harms.

PRESCRIBED FIX. The same `src/main.rs:1569` change, verified above to produce `metrics: 1 records` and `ALPHA PRIVATE RESUME STATE.` on the ATTACK line.

### 4.3 G-ERRDOC (R4A-3 and R4R-2): VALID, `low`, ONE finding

Both lenses target `src/main.rs:1557-1560`, and both measure the same clause false:

```
/// Guessing the other way would drop its root on the strength of an error, and of the two
/// directions only this one can add a root rather than remove one.
```

Counting an unstat-able anchor as existing populates `on_disk`, a non-empty `on_disk` suppresses the `supplied` fallback, and the OTHER anchor's root is REMOVED. R4A-1's differential is the measurement, and R4R-2's own `<S>/r4reg-errdir.sh` CASE B versus CASE D pair is a second independent measurement of the same removal. The argument that authorised the line is falsified by the line's own behaviour. The two filings collapse completely and count once.

IS THE README/CHANGELOG HALF SEPARABLE? Yes, and its separability is CONDITIONAL ON THE R4A-1 DISPOSITION, which is why it does not earn a second count.

- If R4A-1 is FIXED as prescribed, I verified that `README.md:236`'s "every `--source` or `--plan` you gave THAT IS ON DISK yields one", "An anchor that is not on disk yields a root only when NO anchor you gave is on disk" and "Beside an anchor that IS on disk it yields nothing" all become TRUE as they stand, and `CHANGELOG.md:23`'s identical clauses with them. Only the doc comment needs editing, and it needs editing anyway.
- If R4A-1 is ACCEPTED and RECORDED, all three sites need the "cannot be determined" case spelled out, and the recorded cost needs a test.

SEVERITY `low`, matching what the project gave `R3ACC-1`, `R3F-2` and `R3A-3`'s doc half. PRESCRIBED FIX: `src/main.rs:1557-1560`, plus `README.md:236` and `CHANGELOG.md:23` only under the accept-and-record disposition. See section 6 for two further false clauses at these sites that neither lens named.

### 4.4 G-STALECLAIM (R4C-1, R4C-2 and R4R-1): VALID, `low`, ONE finding, TWO sites

Two lenses found these independently, and R4R-1 is the union of R4C-1 and R4C-2. ONE FINDING, not two and not three: it is one class, at two sites, correctable in one sweep, which is exactly the treatment the round 3 triage gave `R3F-2` across three sites.

WHAT I RAN, on the rebuilt pristine HEAD binary, from a foreign directory:

```
$ agent-scaffold status --json --source <R>/alpha/docs/plans/m.plan.toml \
                              --plan <R>/beta/docs/plans/nope.md
note: --plan <R>/beta/docs/plans/nope.md does not exist
    "records": 1
  "metrics_absent_reason": null
$ agent-scaffold next --json  <the same two anchors>
note: --plan <R>/beta/docs/plans/nope.md does not exist
    "records": 1
  "metrics_absent_reason": null,
```

Alpha's own log is read, so the missing `--plan` supplied NO containment root. `src/main.rs:1151`'s "A missing anchor still supplies a containment root" and `src/main.rs:1628-1629`'s "`next` roots containment on an anchor that does not exist rather than falling through with none" are both false on the exact configuration the round 3 fix was written FOR. `note_missing_anchors`'s own doc comment at `src/main.rs:1105-1109` states the rule CORRECTLY ("only the anchors that are ON DISK decide unless no supplied anchor is"), so the increment ships one function whose contract is written three times and disagrees with itself twice. I confirmed all three texts by reading them.

SEVERITY `low`, DOWN from the claims lens's `medium` on both sites. The class is comment-only with no behavioural consequence, at internal call sites rather than user-facing text, and the project has rated every member of this class `low` (`R3ACC-1`, `R3F-2`, `R3A-3`'s doc half). Applying `medium` here would make this round's severities incomparable with round 3's, which is the same objection the round 2 triage raised against mixed standards. The one aggravator I weighed and rejected as insufficient: these two comments JUSTIFY CALLING `note_missing_anchors`, so a maintainer who believes the false premise might conclude the call is unnecessary and delete it, which is `R3F-1`'s defect. That is a real but remote hazard and it does not lift a comment-only finding out of `low`.

PRESCRIBED FIX. `src/main.rs:1150-1152` and `src/main.rs:1628-1629`, one clause each, in one sweep with G-ERRDOC. The claims lens's suggested wordings are correct and can be taken as written.

---

## 5. SPOT-CHECK OF THE REGRESSION LENS'S LARGE POSITIVE RESULT

**IT HOLDS. I could not break any of the four claims I checked, and the two I was asked to verify reproduce on my own fixtures with controls that demonstrably show the change.**

### 5.1 `G-EMPTYROOT`: CONFIRMED STILL CLOSED

`<S>/tri4-emptyroot.sh`, my own fixture (a foreign `home` with a three-record log and a `HOME resume state.` ledger, explicit `--metrics` and `--ledger-fragment` naming both), five anchor configurations, three binaries.

```
########## HEAD ##########
  A --source MISSING only     next:rec3=n homeblock=n | status:rec3=n | resume:homeblock=n | m="log-not-this-project" l="ledger-not-this-project"
  B --plan   MISSING only     next:rec3=n homeblock=n | status:rec3=n | resume:homeblock=n | m="log-not-this-project" l="ledger-not-this-project"
  C both anchors MISSING      next:rec3=n homeblock=n | status:rec3=n | resume:homeblock=n | m="log-not-this-project" l="ledger-not-this-project"
  D --source EXISTS (control) next:rec3=n homeblock=n | status:rec3=n | resume:homeblock=n | m="log-not-this-project" l="ledger-not-this-project"
  E NEITHER anchor supplied   next:rec3=Y homeblock=Y | status:rec3=Y | resume:homeblock=Y | m=null l=null
########## HEAD~2 (the binary G-EMPTYROOT was filed against) ##########
  A/B/C leak on all three surfaces with both reasons null;  D refuses;  E leaks.
########## main ##########
  A/B/C/D/E all leak.
```

Identical to the lens's table row for row. The control is demonstrably capable of showing the change: `HEAD~2` leaks on exactly the rows HEAD refuses, and row D is the control that refuses on both. Row E is the decided neither-anchor case and is identical on all three binaries. Re-run on the rebuilt pristine binary after all mutations: byte-identical.

### 5.2 `ADV-1`: CONFIRMED STILL CLOSED

`<S>/tri4-adv1.sh`, thirteen configurations, HEAD against `main`, counting whether each surface echoes the foreign `HOME resume state.` block.

At HEAD the `ADV-1` signature (`next` echoes a block that `status --resume` refuses) appears in ZERO of thirteen. The only foreign echo at HEAD is Q12, no anchors supplied, where BOTH surfaces echo it and both agree; that is the decided neither-anchor case and it is identical to `main`. My control leaks in 13 of 13 rather than the lens's 6 of 13, because every row of my fixture carries an explicit foreign `--ledger-fragment` whereas the lens varied that; the control is therefore MORE capable of showing the change than the one the lens used, and the negative result at HEAD is correspondingly stronger. Re-run on the rebuilt pristine binary: byte-identical.

### 5.3 The structural claim in the lens's section 3.4: CONFIRMED, with one qualification the lens should have made

I applied both mutations myself and reverted each.

```
N1  src/main.rs:1570, `let deciding = &on_disk;`
    -> FAILED. 17 passed; 1 failed: an_anchor_that_does_not_exist_still_supplies_a_root
N2  src/main.rs:1570, `let deciding = &supplied;`
    -> FAILED. 17 passed; 1 failed: a_missing_anchor_does_not_overrule_an_anchor_that_exists
```

Exactly one failure each, disjoint, as reported. The lens's conclusion that `deciding` is for the first time pinned on both sides is CORRECT.

THE QUALIFICATION. The lens presents this as "`resume_roots`'s policy has been pinned on both sides". It has not. The line directly above `deciding`, the `on_disk` MEMBERSHIP rule, is pinned on NEITHER side: flipping `unwrap_or(true)` to `unwrap_or(false)` leaves all 418 tests green, which I proved by mutation. The function's policy has two halves and only one of them is guarded.

### 5.4 Verdict on that lens's work

Its baseline discipline this round is sound and is a real change from rounds 2 and 3: every "unchanged" and every "still closed" names a BUILT BINARY rather than a neighbouring command line, and each control is checked for the property that it CAN show the change. Everything it MEASURED, I could reproduce. Its mutation sample, its 45-cell sweep attribution and its "no test's meaning changed" result are the kind of evidence this loop has been short of. Section 6 states precisely where it went wrong, and the failure is one of adjudication rather than of coverage or method.

---

## 6. WHAT ALL THREE LENSES MISSED

### 6.1 THE REGRESSION LENS DID NOT MISS THE CELL. IT MEASURED R4A-1 AND RULED IT FINE

The brief asks me to confirm or refute that the gap is a missing cell, `ABSENT`-but-unstat-able being a different cell from `PRESENT`-but-unreadable in that lens's D4 dimension. **I REFUTE IT, and the truth is less flattering to the lens than the proposed explanation.**

Those are not two cells. `resume_roots` sees only `try_exists() == Err`, and both spellings produce it. I reproduced R4A-1 IN FULL using an anchor that is present but unreadable, which is literally D4's stated third value, quoted in section 1.3: `--plan <gamma>/docs/plans/locked/p.plan.toml` where the file genuinely exists and `locked` is mode `000`. HEAD reads gamma's four-record log and echoes `GAMMA PRIVATE RESUME STATE.`; `HEAD~1` refuses both. So the lens's D4 dimension covered the cell.

More than that: THE LENS RAN R4A-1's MECHANISM AND RECORDED THE MEASUREMENT. Its own `R4R-2` CASE B versus CASE D pair is exactly this configuration in the other orientation, and its own prose states the mechanism correctly ("Because an anchor that cannot be checked is placed in `on_disk`, it makes `on_disk` non-empty, which REMOVES every other supplied anchor's root from `deciding`"). It then wrote "THE BEHAVIOUR ITSELF IS FINE and I am not asking for it to change" and filed the result as a `low` doc-only finding.

I built both orientations to isolate why. One fixture, one erring anchor, one missing anchor in another project, gamma's artifacts named explicitly:

```
ORIENTATION 1 (the lens's own CASE B): the ERRING anchor is the --source
  NEW: task: p      metrics: 4 records   GAMMA PRIVATE RESUME STATE.
  OLD: task: p      metrics: unavailable, ... not under the plan's project root <R>/alpha
ORIENTATION 2 (run by no lens): the ERRING anchor is the --plan
  NEW: task: ghost  metrics: 4 records   GAMMA PRIVATE RESUME STATE.
  OLD: task: ghost  metrics: unavailable, ... not under the plan's project root <R>/alpha
```

The behaviour is the SAME in both; only the derived task name differs. In orientation 1 the task is derived from the erring anchor, so reading that anchor's project's log looks correct and the lens read it as an improvement. In orientation 2 the task is derived from the OTHER project, so the same read is a leak. The lens crossed D4's `Err` value only in orientation 1, and therefore only in the arrangement in which the root removal looks benign.

THE PRECISE DIAGNOSIS, which is what the next round should carry: the lens's coverage claim is SOUND and its dimensions are honest. What failed is the adjudication rule. Faced with a measured difference from the pre-fix binary, it asked "is the admitted artifact the anchor's own?" and stopped, instead of asking "what did the root that was removed previously refuse?". Its section 3.1 shows the same stop in the other place it touched this branch: it re-checked the erring branch explicitly and reasoned only about EMPTINESS ("no third path reaches an empty vector"), which tests the branch against `G-EMPTYROOT`'s failure mode and not against the new failure mode the narrowing created, which is root REMOVAL rather than root ABSENCE. A lens that has just watched a fix change which roots survive owes a discriminating control on the artifact side, and this one ran it on the artifact belonging to the surviving root's own project only.

### 6.2 `resume_roots`'s doc comment has THREE false clauses, and the lenses named ONE

R4A-3 and R4R-2 both name only the final paragraph at `src/main.rs:1557-1560`. Two clauses in the paragraph ABOVE it are falsified by R4A-2's own run, which both lenses had in hand:

- `src/main.rs:1542-1544`: "BUT SUCH A ROOT IS A GUESS, AND A GUESS DOES NOT OVERRULE AN ANCHOR THAT IS ON DISK. Where at least one supplied anchor exists, only the anchors that exist decide, and the guessed one is left out." In R4A-2's attack the `--source` exists, the trailing-slash `--plan` does NOT exist, and it decides anyway. FALSE.
- `src/main.rs:1553-1555`: "a missing anchor beside an existing one defers to the existing one, which is the root that invocation had before the `--plan` was typed at all." FALSE on the same run.

Whoever fixes G-ERRDOC must sweep the whole doc comment, not the one paragraph the two lenses named. This is the fourth consecutive round in which a stale-claim finding named a strict subset of its own sites.

### 6.3 `README.md:236` and `CHANGELOG.md:23` carry a THIRD false clause, and the claims lens's sweep of those two lines produced a false negative

Both lines say: "Beside an anchor that IS on disk it yields nothing and the one on disk decides, so naming a plan file you have not written does not withhold the other anchor's own log and ledger." R4A-2 falsifies it directly: an anchor that is not on disk, beside one that is, yields a root and DOES withhold the other anchor's own log and ledger. R4A-3 named only the first two clauses of these lines.

This matters beyond the extra site. The claims lens swept `README.md:236` and `CHANGELOG.md:23` CLAUSE BY CLAUSE, listed five clauses, named the run it checked each against, and concluded "No finding: every clause matched a real run". Not one of those runs put an anchor in the `Err` class, so the sweep's negative result is bounded to `Ok(true)` and `Ok(false)` anchors only, and it does not say so. A clause-by-clause sweep that states no bound reads as exhaustive; this one was two thirds of a dimension short.

### 6.4 THE `on_disk` MEMBERSHIP RULE IS UNGUARDED, PROVEN RATHER THAN ARGUED

The adversarial lens argued from reading the tests that nothing pins the `unwrap_or(true)` branch. I proved it: flipping the polarity leaves 418 of 418 green. Combined with 5.3, the position is that the round 3 fix added a two-part policy to the increment's most sensitive function, and shipped a guard for one part. This is the same defect class as `R3F-1` (`medium`, an unguarded call site), and it is the reason a fix pass that changes this line must bring its own red-then-green tests rather than relying on the suite.

### 6.5 The `EACCES` trigger class is narrower than claimed

Section 1.3. Recorded so the next round does not inherit an over-claim.

---

## 7. SCOPE ITEMS I HELD, AND ONE I CHECKED RATHER THAN ASSUMED

- THE IN-ROOT BOUND is RECORDED, NOT CLOSED, and I required a discriminating control before accepting any attribution to it. The adversarial lens's G-series control was mis-built first (its "disjoint" fixture was still under the derived root, so the disjoint arrangement reproduced and proved nothing), it threw that away and rebuilt it so the derived root is the fixture root, and its corrected conclusion is CORRECT: the `--plan <a directory>` construction is the bound and it rightly filed nothing. I applied the same test to R4A-1 and it comes out the other way, which is why one is filed and the other is not (section 4.1).
- THE SINGLE-ANCHOR `..` RESIDUAL is recorded and deliberately unfixed. No lens re-raised it and I do not.
- `ADV-2`, `R2A-2`, `R2C-2`, the stale "FOUR owed red-then-green demonstrations" count and the `Q-55-emptyroot` fix SITE are human-closed. Not re-examined.
- The four accepted costs (specification line 251), `validate --workflow`'s non-use of `containment_roots`, project identity, line length and prose hard-wrapping: out of scope, and no finding above depends on any of them.
- THE SPECIFICATION. I checked whether R4A-1's behaviour is authorised anywhere in it: `grep` for `try_exists`, "cannot be determined", "could not be checked" and "unreadable" over `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` returns nothing. The `Err` policy is an implementation choice made in the round 3 fix pass, not a specified one, so no specification defect blocks the remedy and none is raised.

---

## 8. MY RECOMMENDATION FOR THE ESCALATION

### 8.1 The convergence arithmetic, and what it changes

Four rounds are spent, none was clean, the streak is 0 of the 2 this `risky` increment needs, and the cap is 5. A clean round 5 reaches a streak of 1, so convergence before the cap is arithmetically impossible and this loop escalates to the human regardless of what is done next. No verdict in sections 1 to 7 was influenced by that; I would file the same four against a fresh increment on day one.

WHAT IT DOES CHANGE IS THE DISPOSITION CALCULUS, and the change is large enough that the human should see it stated. The round 3 triage recommended against a third rewrite of `resume_roots` being weighed lightly precisely because it was "the highest-variance action available" with two clean rounds still theoretically reachable. That consideration is now GONE. There is no longer a cheap path to convergence to protect, so the argument for leaving a known `high` in place to keep round 5 clean has lost its object: round 5 cannot deliver convergence whether it is clean or not. The calculus now favours correctness over stability, and I weigh it that way below.

### 8.2 What MUST be fixed before this increment can ship

**R4A-1, and only R4A-1.** It is a foreign-content leak at exit 0 with both machine reason fields `null`, on five surfaces, whose stdout is byte-identical to the output of the binary the project already rated `high` in round 2, reachable with ordinary path spellings and no privileges. Shipping it means shipping, on a live population, the exact defect the increment exists to prevent. I would not sign this increment off with it open.

### 8.3 What could be ACCEPTED AND RECORDED

- **R4A-2** (`medium`). The direction is fail-safe (a refusal, at exit 0, with a truthful `note:` printed), the population is the same narrow one, and the round 3 triage already routed the underlying policy to the human as an open question. It is defensible to record it as a fifth accepted cost, pinned by a test, IF the human chooses a disposition for R4A-1 that does not close it as a side effect. Note that the recommended remedy closes it for free, so accepting it is only meaningful under options B or C below.
- **G-ERRDOC** and **G-STALECLAIM** (both `low`, both text). These are cheap enough that recording them costs about as much as fixing them, and the project has fixed every prior member of the class. My advice is to fix them; but if the human wants to freeze the code entirely and ship, recording them is defensible, with the caveat from 6.2 and 6.3 that the true site list is larger than the lenses' and must be swept in full rather than patched clause by clause.

### 8.4 Would I fix R4A-1 by touching `resume_roots` a fifth time, or another way?

I looked for a remedy that avoids the function and there is not one: the deciding-set rule exists nowhere else. So the real question is HOW MUCH to touch it, and here are the three serious options with what each costs.

**OPTION A (RECOMMENDED). The one-token polarity change at `src/main.rs:1569`, plus two red-then-green tests and the doc-comment sweep.**

- BENEFIT. Closes the `high` AND the `medium` together. Makes `README.md:236` and `CHANGELOG.md:23` true as they stand, so G-ERRDOC shrinks to one doc comment. I MEASURED the entire battery under it and reported it in section 1.5: suite 418 green and unchanged, R4A-1 closed, R4A-2 closed, `R3A-1`'s closure preserved on all three of its controls, `G-EMPTYROOT` closed, `ADV-1` closed, `R3A-2`'s note text untouched.
- COST. It is a fifth touch of the function that has produced a finding after every previous touch. That record is four for four and a human is entitled to weigh it heavily.
- THE HONEST COUNTERWEIGHT TO THAT RECORD, which I offer as an argument and not as a guarantee. Every previous change to this function ADDED a rule: round 2 added partial resolution, round 3 added the narrowing plus the `Err` classification. This one REMOVES a special case, and it is monotone, `on_disk` can only shrink under it, with the `supplied` fallback unchanged and `supplied` non-empty whenever an anchor was supplied. So the empty-vector failure mode that caused `G-EMPTYROOT` is unreachable by construction and not merely by measurement. It is the first change to this function whose case count goes down.
- JUDGED AGAINST THE PLAN'S OWN PRINCIPLES: "Safe on existing projects" is what the leak violates and what this restores; "One source of truth" favours a deciding set that means what its name says; "Minimal by default" is satisfied at one token plus two tests.

**OPTION B. Accept R4A-1 and record it as a fifth accepted cost, pinned by a test, and rewrite all three text sites to state the `Err` case.**

- BENEFIT. No product logic changes at all, so round 5 is a text-and-tests round, which is the shape this project has clean-round evidence for.
- COST. Ships a foreign-content leak at exit 0 with both machine reasons `null` on the surface an agent consumes, in the exact output shape rated `high` twice already, on a population that includes an ordinary path spelling. It also means writing an accepted-cost paragraph that says, in effect, that a plan not yet written beside a plan under an unreadable directory may cause another project's resume state to be printed as this plan's. I do not recommend it and I would say so at the escalation.

**OPTION C. Partial revert of the narrowing only, restoring `HEAD~1`'s deciding rule, and accept `R3A-1` instead.**

- WHAT IT IS, measured: this is exactly mutation N2, `let deciding = &supplied;` with the `on_disk` computation deleted, which I ran and which reds exactly one test, `a_missing_anchor_does_not_overrule_an_anchor_that_exists`, the round 3 fix's own. It is NOT a revert of commit `94eb8ec`, which would also discard the `R3A-2`, `R3A-3`, `R3F-1`, `R3F-2` and `R3ACC-1` fixes.
- BENEFIT. No live `high` and no new logic. `resume_roots` returns to a shape measured clean on this axis, because at `HEAD~1` the `Err` classification had no effect on the deciding set at all. This is the option that trades a `high` for a `medium` without authoring anything.
- COST. Reinstates `R3A-1` (`medium`): a project loses its own default log and its own default ledger, with `log-not-this-project` and `ledger-not-this-project` asserted about them, whenever a `--plan` names a file not yet written. It deletes the test that pins the narrowing, and it requires `README.md:236`, `CHANGELOG.md:23` and the doc comment to be reverted to their pre-narrowing statements. It also means the round 3 review round bought nothing on its headline finding.

MY RECOMMENDATION IS OPTION A. Option C is the serious alternative and the human should see it, but it pays a `medium` on a population the human's own `Q-55-emptyroot` text explicitly declined to burden, in order to avoid a change I have measured against the full battery. Option B is the only one that ships a known live `high`, and 8.1 removes the one argument that made it attractive.

WHATEVER IS CHOSEN, ONE THING IS OWED IN EVERY CASE. Section 6.4 shows the suite cannot see the `on_disk` membership rule in either direction. Any disposition, including accepting the current behaviour unchanged, must arrive with a test on the two-anchor `Err` case in both orientations. Without it the next person to touch this function will change this behaviour without knowing, and this loop will find it again.

---

`git status --short` in this worktree shows only this file. Three mutations were applied one at a time in my own worktree, measured, and reverted, and the tree was confirmed clean after each; the pristine binary was rebuilt afterwards and every affected measurement re-taken. All fixtures live under `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/tri4-*` and nothing outside them was created or deleted.
