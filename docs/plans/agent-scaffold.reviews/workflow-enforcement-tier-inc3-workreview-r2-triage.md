# `workflow-enforcement-tier-inc3` work review, ROUND 2 TRIAGE

Triaged in worktree `.claude/worktrees/triage-inc3-r2` on branch `triage/inc3-r2` at `8e93410`, the tip of the branch under review with both reviewers' findings merged into it. `git diff 141cf1c HEAD -- src/ tests/` is EMPTY, so the product both reviewers measured is byte-for-byte the product triaged here; the two commits differ only in the merged review files. Governing specification: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`. Round 1's triage was read in full before any verdict and every ruling in it is treated as settled.

SEVEN RAW FINDINGS, SEVEN UNIQUE: the two lenses did not overlap on any of them, so no merge was possible and none was made.

## Method, because `Q-66` governs and nothing below is adjudicated by reading

Four binaries, all built from source in my own directory, none of them anyone else's worktree:

- NEW: this worktree at `8e93410`, `target/debug/agent-scaffold`.
- PREFIX: `6ec7601`, the branch tip immediately before the round 1 `T-1` fix commit `7a9df9c`, exported with `git archive` and built at `<scratch>/build/prefix`. (This is the commit the reviewers call `60679ca`; the branch was rebased between their round and mine, so every hash they cite has a new spelling in the final history. The mapping is in `R2B-1` below.)
- PRE: `9eeca42`, which predates the whole increment. Verified `git diff 9eeca42 18176fa -- src/ tests/ Cargo.toml` is EMPTY, so a PRE binary is the same product as one built from `18176fa`, which the brief names, and as one from `main`.
- Two REMEDY builds, one per candidate fix measured below (`R2A-1` and `R2A-2`), each an exported copy of `HEAD` edited in the copy. The reviewed worktree was never edited; `git status --short` on it is empty and was empty throughout.

TOOLCHAIN. Every `cargo` invocation went through `direnv allow && eval "$(direnv export bash)"` first, giving cargo 1.98.0-nightly / clippy 0.1.98 from the flake. THIS MATTERED THIS ROUND: `/usr/bin/cargo` 1.88.0 is on this machine's default PATH ahead of the nix store, so a shell that skips the direnv step silently gets the system toolchain. See "What the reviewers missed", item 1.

`TMPDIR` was pointed at `<scratch>/tri2/tmpdir`, outside any git repository, for every `cargo test`. All fixtures live under `<scratch>/tri2/`, a directory of my own naming; every directory chmodded to 600 or 000 was chmodded back and a closing `find <scratch>/tri2 -type d ! -perm -u+rwx` returns nothing. `<scratch>` abbreviates `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad`.

MEASURED SUITE COUNTS, used by several findings below, each a full `cargo test` on a build of that commit:

| Commit | As the user | As namespace root (`unshare -Ur`) | `#[test]` in `validate_workflow_toml_source_needs_no_plan.rs` |
| --- | --- | --- | --- |
| `main` | 420 passed, 0 failed | 420 passed, 0 failed | 2 |
| `18176fa` (docs only, code == main) | 420 passed, 0 failed | not run | 2 |
| `aeece6c` (build tip, round 1's tree) | 421 passed, 0 failed | 421 passed, 0 failed | 3 |
| `6ec7601` (PREFIX, mid fix pass) | 421 passed, 0 failed | 421 passed, 0 failed | 3 |
| `HEAD` | 422 passed, 0 failed | 421 passed, 1 FAILED | 4 |

## Deduplication

| Finding | Raw | Overlap | Verdict | Severity |
| --- | --- | --- | --- | --- |
| `V-1` | `R2A-1` | none | VALID | medium (upheld) |
| `V-2` | `R2A-2` | none | VALID | low (upheld) |
| `V-3` | `R2A-3` | none | VALID | low (upheld) |
| `V-4` | `R2B-1` | none | VALID | low (upheld) |
| `R-1` | `R2A-4` | none | ACCEPT AS RESIDUAL | low (upheld) |
| `X-1` | `R2B-2` | none | INVALID | n/a (reviewer said low) |
| `X-2` | `R2B-3` | none | INVALID, mis-scoped | n/a (reviewer said low) |

---

## `V-1` (`R2A-1`) VALID, `medium`: the fix pass's new test fails outright when the suite runs as root, on an assertion whose message describes the opposite of what happened

### What I reproduced

The failure reproduces, exactly as claimed. `unshare` is available at `/usr/bin/unshare`; `unshare -Ur` maps this uid to 0 inside the namespace, and I verified the DAC bypass on my own fixture before trusting it: `stat` on a file under a mode-600 directory is `Permission denied` as the user and succeeds inside the namespace.

```
$ unshare -Ur env PATH="$PATH" TMPDIR="$TMPDIR" HOME="$HOME" cargo test --test validate_workflow_toml_source_needs_no_plan
running 4 tests
test a_round_log_that_cannot_be_checked_is_not_reported_as_missing ... FAILED
panicked at tests/validate_workflow_toml_source_needs_no_plan.rs:313:5:
assertion `left == right` failed: a check that could not run must still refuse; stdout:
docs/metrics/workflow.jsonl: 1 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
  left: Some(0)
 right: Some(1)
test result: FAILED. 3 passed; 1 failed
```

The message asserts a check could not run, printed over stdout that shows the check ran and passed.

ATTRIBUTION, which is what makes this a finding against this increment rather than a suite property: the whole-suite column in the method table above. As namespace root, `main` is 420/0, the build tip is 421/0 and PREFIX is 421/0; `HEAD` is 421 passed with 1 failed, and the one that fails is the test the round 1 fix pass added. NO OTHER TEST IN THIS REPOSITORY FAILS AS ROOT. It is not the known `test-tmpdir-repo-assumption` class either: `TMPDIR` was outside any repository in every run in that table.

The fixture left behind by the panic is mode 755 and removable (`drwxr-xr-x` under my `TMPDIR`, inspected after the failure), so the hazard the commit message says it designed against is genuinely absent. That half is not a defect and I do not count it.

### Reasoning

The test computes `opaque` precisely because the mode may not bite, and its own doc comment says that when it does not "there is nothing to say". Line 313 then says something, and what it says is wrong in that environment: `PLAN_TOML`'s only step is `not-started`, so W3 has nothing to enforce, the readable log validates, and the run exits 0. This is a defect against the increment's OWN STATED DESIGN, written down in the test three lines above the assertion that contradicts it, not merely an environment quirk.

`medium` UPHELD, and I considered `low` seriously. Below `high` because the product is untouched: no user of the binary is affected, and a red gate cannot ship a wrong answer the way a false green can. Above `low` because `cargo test` is one of this project's four gates, the material is the newest in the tree, root-in-a-container is an ordinary CI environment, and this repository's own pack ships a container isolation tier (`pack/isolation-guidance.md`, the agent-box module) as the strongest one, so a future agent running the suite there gets a red gate with a message that points away from the cause. The cost is one wasted investigation per hit, and the remedy is one assertion relocated.

### Smallest remedy, MEASURED

Move the `assert_eq!(code, Some(1), ...)` at `:313-317` inside the existing `if opaque` block at `:318`. Nothing is added and nothing is authored; one statement moves and gains a tab. Measured on a `HEAD` copy with exactly that edit:

- As the user: 422 passed, 0 failed.
- As namespace root: 422 passed, 0 failed.

The closing control (mode 755 restored, exit 0, `workflow invariants hold`) is outside the guard and passes in both environments, so the test keeps its full meaning where the mode bites and makes only the claim its doc comment promises where it does not.

---

## `V-2` (`R2A-2`) VALID, `low`: the arm re-stats the log, so the sentence describes a later observation than the exit code does, and both disagreement directions are constructible

### What I reproduced

Both cells, deterministically, with the FIFO construction the reviewer describes. `run_validate` stats the log at `src/main.rs:845` before it reads the `--source` at `:880`, and `fs::read_to_string` on a FIFO blocks at `open` until a writer appears, so the window between the gate and the arm's second stat is under my control. The race needs no FIFO; the FIFO only widens it.

CELL 1, gate answers ENOENT, log created mid-run, arm's `try_exists()` answers `Ok(true)`:

```
$ mkfifo docs/plans/p.plan.toml            # docs/metrics/ exists, no log in it
$ agent-scaffold validate --source docs/plans/p.plan.toml --workflow &
$ sleep 1; cat out.stderr
no metrics log at docs/metrics/workflow.jsonl; nothing to validate        # the gate has answered
$ printf '%s\n' '{"type":"round","task":"only-step",...,"outcome":"clean",...}' > docs/metrics/workflow.jsonl
$ cat planbody.toml > docs/plans/p.plan.toml            # unblock the run
--workflow requested but no round log at docs/metrics/workflow.jsonl: ... or record the project's
  review rounds there                                                    exit=1
$ wc -c docs/metrics/workflow.jsonl
241 docs/metrics/workflow.jsonl
```

CELL 2, gate answers `Ok(false)` TRUTHFULLY, the directory becomes unsearchable afterwards, the log never exists at all:

```
NEW      --workflow requested but the round log at docs/metrics/workflow.jsonl could not be checked
         (Permission denied (os error 13)): ...                          exit=1
PREFIX   --workflow requested but no round log at docs/metrics/workflow.jsonl: ...                exit=1
$ ls -A docs/metrics                                                     # empty: no log, ever
```

I also confirmed the reviewer's negative result: on a static filesystem the two calls cannot disagree, because `exists()` is `metadata().is_ok()` and `try_exists()` is the same call with `Err(NotFound)` mapped to `Ok(false)`; reaching the arm at all requires `metadata` to have failed, so the arm sees `Ok(false)` or `Err` and never `Ok(true)`.

### THE QUESTION THE BRIEF PUT: did a round 1 fix re-open the defect it closed?

NO, and the distinction is worth stating precisely because the brief warns about the previous increment's byte-identical leaks.

- `T-1` IS CLOSED ON EVERY STATIC FILESYSTEM. On the mode-600 fixture that produced `T-1`, where `ls docs/metrics` prints `workflow.jsonl`, NEW prints `could not be checked (Permission denied (os error 13))` at exit 1 while PREFIX printed the false absent sentence with its remedy clause. Reproduced directly.
- CELL 1 IS NOT THE FIX'S DOING. PREFIX prints the identical sentence on the identical fixture, and so would any single-stat implementation, because the observation that decided the exit code was TRUE WHEN IT WAS MADE: at that instant there really was no log. This is ordinary TOCTOU, inherited rather than opened. It is not `T-1` re-opened: `T-1` was a static, every-time falsehood produced by an ordinary permission state, and this needs the filesystem to change inside a sub-second window.
- WHAT THE FIX DID INTRODUCE IS CELL 2, the opposite direction: a false "could not be checked" where the tool HAD successfully answered at the deciding moment and where PREFIX printed the true sentence. That is new, and it is the only new falsehood in this finding.
- WHAT IS ALSO WRONG IS THE COMMENT. The arm's own text says "`Ok` asserts absence" while the code maps `Ok(true)`, a positive observation of PRESENCE, into the absent sentence through an `Ok(_)` catch-all. The code knowingly discards what its second observation said, in one direction.

`low` UPHELD, and I considered raising it. The exit code is 1 and CORRECT in every cell, so no CI gate is misled and no false green exists anywhere in this finding; both cells need a mid-run filesystem change; and the reachable real-world shape is a concurrent first append to a project's round log inside the window in which `validate --workflow` reads its plan source, which happens at most once in a project's life. `T-1` was `medium` because an ordinary permission state produced its falsehood every time, with no race at all. That gap is the whole difference between the ratings, and it is a difference in reachability rather than in consequence.

### Smallest remedy, MEASURED, and one correction to how the reviewer framed it

The reviewer's candidate is right and I verified it on my own build rather than trusting the report. Two lines, no new prose:

```rust
let metrics_probe = metrics_path.try_exists();
let metrics_contents = if matches!(metrics_probe, Ok(true)) {   // was: if metrics_path.exists()
...
_ => problems.push(match &metrics_probe {                       // was: match metrics_path.try_exists()
```

`matches!(try_exists(), Ok(true))` is `metadata().is_ok()` by definition, and neither pattern binds, so nothing moves and the gate's predicate is unchanged. Measured on a build of that edit:

- Cell 2 now prints `no round log at docs/metrics/workflow.jsonl ...` at exit 1. The regression is gone.
- The static mode-600 case, the input `Q-55-existsgate` was decided on, still prints `could not be checked (Permission denied (os error 13))` at exit 1. The `T-1` fix's own purpose is preserved.
- The genuinely absent log still prints the absent sentence with its remedy clause at exit 1.
- PLAIN `validate` IS BYTE-IDENTICAL to PRE (`9eeca42`) AND TO `HEAD` ACROSS 18 INPUTS, stdout, stderr and exit code compared as one string: `docs/metrics` at modes 755, 600, 000 and 111 with the default and with an explicit `--metrics`; ENOTDIR; ELOOP; ENAMETOOLONG; a dangling symlink; the log path being a directory; a mode-000 log FILE; a genuinely absent log; no anchors at all; a missing `--source`; and a `--metrics` outside the root. `Q-55-existsgate`'s promise, which is the ground the one-token alternative was declined on, is untouched.
- `cargo test` 422 passed, 0 failed. `cargo clippy --all-targets -- -D warnings` exit 0, in the direnv environment.

THE CORRECTION: this remedy does NOT change cell 1's output. It still prints `no round log at <path> ... record the project's review rounds there` over a log that exists by the time the sentence is printed. What changes is that the sentence becomes a faithful report of the single observation that decided the exit code, instead of contradicting a second observation the code just made and threw away. The remedy's honest claim is "ask once, describe the answer you asked for", not "the `T-1` sentence goes away". A human weighing this should weigh it as that, because no implementation that stats once can do better.

THIS IS NOT `Q-55-existsgate`'s DECLINED CHANGE. That decision declined `exists()` to `try_exists()?`, which propagates the io error and alters plain `validate`. This preserves the gate's predicate exactly and is measured byte-identical on plain `validate` across the 18 inputs above, including the unsearchable-ancestor case the decision actually turned on.

---

## `V-3` (`R2A-3`) VALID, `low`: the `Err` branch fires on every errno, so the three classes where absence IS established lose a true sentence and its remedy clause

### What I reproduced

All four cases, three binaries each:

```
ENOTDIR   (--metrics docs/plans/p.plan.toml/workflow.jsonl)
  NEW     ... could not be checked (Not a directory (os error 20)): ...                     exit=1
  PREFIX  ... no round log at ...; pass a `--metrics` naming this project's log, or record
          the project's review rounds there                                                 exit=1
  PRE     --workflow has a plan source but the metrics log is missing; skipping             exit=0
ELOOP     (docs/loop/loopy -> loopy)      NEW: "Too many levels of symbolic links (os error 40)"
ENAMETOOLONG (a 300-character leaf)       NEW: "File name too long (os error 36)"
DANGLING SYMLINK, THE CONTROL             NEW: "no round log at docs/metrics/dangling.jsonl ..." exit=1
```

The control is what makes the finding rather than decorates it: `metadata` follows the link, gets ENOENT, `try_exists` returns `Ok(false)`, and NEW prints the absent sentence exactly as it should. So the split is real and it is drawn in the wrong place for three errnos.

### Reasoning

Round 1's triage enumerated these same three classes and ruled, on measurement, that "no round log at X" is "terse rather than false" for each, because nothing can exist at such a path. The fix routes exactly those into the sentence written for the ONE class where a real log does sit behind the error. The `Err` sentence is not false (the stat genuinely failed and the errno is named), but it disclaims knowledge the tool has and it drops the two-part remedy clause. The most likely way to reach ENOTDIR is a mistyped `--metrics`, and that operator now gets an errno and no instruction where round 1's build told them what to do.

`low` UPHELD. The exit code is right, the errno is named, and nobody is sent to fix a correct path, which is what put `T-1` at `medium`. This is message quality on a narrow input class. I also record, as the reviewer did, that round 1's triage RECOMMENDED this remedy shape, so the finding is about what that shape does on inputs the recommendation did not separate, not a departure from it.

### Smallest remedy

Append the clause the sibling arm already carries, verbatim, to the `Err` format string: "; pass a `--metrics` naming this project's log". Nine words COPIED from the neighbouring string rather than authored, one arm, no new branch, no new taxonomy, and it applies to every errno including EACCES, where "pass a `--metrics` naming this project's log" is also useful advice.

I REJECT the reviewer's option 2 (narrow the `Err` branch to the errors that genuinely leave the question open). It would author a classification of errnos that this tree does not have anywhere, and this project has six measurements that a fix pass which authors new structure manufactures the next round's finding. Doing nothing is also defensible here, and if the human prefers to spend no fix-pass words at all this is the finding to drop.

---

## `V-4` (`R2B-1`) VALID, `low`: the new test's doc comment cites a commit that no real clone of this repository has

### What I reproduced

```
$ git for-each-ref --format='%(refname)' | while read r;
    do git merge-base --is-ancestor 1799f8b "$r" && echo "reachable from $r"; done
(no output: unreachable from EVERY ref, not merely absent from branches and tags)

$ git clone --quiet --no-local <repo> <scratch>/freshclone
$ cd <scratch>/freshclone && git cat-file -t 1799f8b
fatal: Not a valid object name 1799f8b
```

`--no-local` forces the object-negotiation path; a same-filesystem clone without it copies the whole object store, dangling objects included, and would have hidden this. THE DISCRIMINATING CONTROL, which the reviewer did not run and which I added because a clone that carries nothing proves nothing: the same fresh clone DOES carry `de18155`, `aeece6c`, `6ec7601`, `7a9df9c` and `8e93410`, the current history of the branch under review, because it clones `origin/impl/wet-inc3` and `origin/triage/inc3-r2`. So the cited object is specifically absent, not incidentally missing.

The same clone is also missing `7ce4443`, `74e6426`, `af850b5`, `16531c5`, `691f88f`, `4801898`, `60679ca` and `6de3a8f`, every hash the round 1 and round 2 records cite, because this task rebases the branch on every loop.

### Reasoning

The claim "RED against `1799f8b`" is unverifiable outside this one working copy today, and unverifiable here once the reflog entry expires. `low` is right: nothing about the code or the product is wrong, and the red-then-green story itself is TRUE (I re-established it independently under `V-2`'s reasoning and via the PREFIX comparisons throughout this file). What is defective is a citation that ships in the test file indefinitely and is the one place a future maintainer would go to check the story.

I agree with the reviewer's own scoping that the commit message and the ledger paragraph are different in kind and should not be raised: a commit message is not editable shipped documentation, and a ledger entry is a dated journal record of a point in time.

### Smallest remedy

A PURE DELETION, and the deletion rather than a substitution, for a reason the reviewer's own text demonstrates: "RED against `1799f8b`, the round 1 tip: that build printed" becomes "RED before this commit: the prior build printed". Two words shorter and no object cited.

DO NOT SUBSTITUTE A CORRECTED HASH. The reviewer proposes `7ce4443`, which (a) is itself unreachable and absent from the same fresh clone, and (b) is not the commit immediately before the fix in its own ordering, which is `60679ca`. Any hash written into this file will be rewritten by the next rebase before the branch merges, which is the general fact the deletion removes.

---

## `R-1` (`R2A-4`) ACCEPT AS RESIDUAL, `low`: the fixed `--workflow` run still prints an asserted absence one line above the corrected sentence

### What I reproduced

Exactly as reported, on the fix's own fixture (mode 600 on `docs/metrics`, log present with a real round record):

```
$ ls docs/metrics
workflow.jsonl                                          # same user, same shell
$ agent-scaffold validate --source docs/plans/p.plan.toml --workflow
no metrics log at docs/metrics/workflow.jsonl; nothing to validate          <- line 1
--workflow requested but the round log at docs/metrics/workflow.jsonl could not be checked
  (Permission denied (os error 13)): ...                                    <- line 2
exit=1
```

Line 1 asserts there is no log at a path where a log with review evidence is sitting; line 2 says the tool cannot tell. Line 1 is unchanged across NEW, PREFIX and PRE, and it is emitted by the shared gate at `src/main.rs:866`, not by the new arm.

### Why RESIDUAL rather than VALID or INVALID

VALID would order a fix, and every fix here is either settled against or a fresh human decision:

- The prose half requires the note to move behind the same probe. That is `Q-55-existsgate`'s DECLINED change if done at the gate (it alters plain `validate`), and if done as a `--workflow`-only suppression it is a behaviour change to shipped stderr that removes a line users may be parsing. Either way it is a decision, not a triager's instruction. The reviewer says as much itself.
- The test half does not work as proposed. Adding `!stderr.contains("no metrics log at")` inside the `if opaque` block would FAIL today, because the note is there and is decided-keep behaviour. So that assertion cannot be added without the prose half.
- The existing assertion is not itself false. `!stderr.contains("no round log at")` with "the log is on disk, so this sentence is false" names one sentence and is true about it. `Q-55-existsgate` scoped the property to the ARM, and the arm is what the test pins.

INVALID would be too strong, because the composed stderr of the increment's OWN surface really does contradict itself in one run, and neither `Q-55-existsgate`'s promise nor the ledger's byte-identity paragraph examined that composition; both are about plain `validate`. That is a genuine new observation about this increment's surface and deserves to be on the record rather than dismissed.

RECORD IT WHERE THE RELATED ITEM ALREADY SITS: the queued validation-constraints step, beside the pre-existing plain-`validate` inconsistency, with the note that on the `--workflow` surface the stale note now sits directly above a sentence that contradicts it, so whoever takes that queue item has both halves in one place.

---

## `X-1` (`R2B-2`) INVALID: the deletion it asks for is text the governing specification requires, and the sentence's operative rule predicts correctly on both populations

### What I reproduced

The behavioural half is TRUE and I reproduced it in full:

```
$ agent-scaffold scaffold --instrument --write --output-dir with-inst
$ agent-scaffold scaffold             --write --output-dir without-inst
$ find with-inst without-inst \( -iname '*metrics*' -o -iname '*.jsonl' \)
(nothing: --instrument renders no docs/metrics and no log)
$ diff -rq with-inst without-inst
Files ... AGENTS.md and ... AGENTS.md differ
Files ... .agents/AGENTS.reference.md and ... differ            (and nothing else differs)
$ (cd with-inst    && validate --source docs/plans/TEMPLATE.plan.toml --workflow)   exit=1
$ (cd without-inst && validate --source docs/plans/TEMPLATE.plan.toml --workflow)   exit=1
   -> stdout, stderr and exit code BYTE-IDENTICAL between the two
```

### Why INVALID anyway, on two independent grounds

GROUND 1, DECISIVE, AND NEITHER REVIEWER CHECKED IT: the specification REQUIRES the clause the finding wants deleted, at two of its three sites.

- `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:370`, on `pack/AGENTS.md:93`: "WHAT IT MUST SAY: that this backstop belongs to the instrumented tier, THAT A PROJECT SCAFFOLDED WITHOUT `--instrument` HAS NO ROUND LOG FOR IT TO READ, and that on such a project the check now REFUSES rather than passing".
- The same entry on the opening clause the finding also objects to: "the existing 'When instrumentation is on' clauses at `pack/AGENTS.md:61` and `:63` ARE THE ESTABLISHED PHRASING FOR THIS CONDITIONAL AND THE QUALIFIER SHOULD MATCH THEM". Verified: `pack/AGENTS.md` uses that conditional at `:61`, `:63`, `:69`, `:93` and `:106`.
- `:372`, on the CHANGELOG: "The entry MUST NAME the exit-code flip AND THE POPULATION IT BREAKS, EVERY PROJECT SCAFFOLDED WITHOUT `--instrument`."

So the remedy would take the product OUT OF CONFORMANCE with its own governing specification at `pack/AGENTS.md` and `CHANGELOG.md`, and would delete a phrasing the specification names by line number as the house form. Changing what the spec requires is planner work, not a reviewer's remedy. The README site alone is not spec-bound, but splitting the three sentences apart is strictly worse than the alignment the round 1 fix established and which this reviewer itself verified.

GROUND 2, INDEPENDENT: the sentence is not false and does not fail acceptance check 20. Its operative rule is the LOG ("on a project with no round log yet, that check exits non-zero"), and a reader on EITHER population who asks "do I have a round log?" predicts exit 1 correctly, which the two-scaffold fixture above confirms rather than refutes. The reviewer concedes this in its own text ("defensible read narrowly"). What remains is an invited inference that the same sentence corrects in its next clause.

RELITIGATION: this is NOT a relitigation of `T-2` in intent. `T-2` ruled that predicating on the flag is false; this targets the residue of `T-2`'s own prescribed wording, which is exactly the fix-induced-residue lens's job and exactly the phenomenon this project has six measurements of. I record that honestly. But its remedy would undo the half of `T-2`'s remedy that kept the specification satisfied, and it is ruled on ground 1 for that reason and not on the ground that the reviewer looked backwards.

---

## `X-2` (`R2B-3`) INVALID, MIS-SCOPED: the ledger carries two gate sentences with two correctly named baselines, and both are true as measured

### What I measured, first-hand, before reading the orchestrator's reasoning

The counts are in the method table above. In the FINAL history: `main` 420 passed; `18176fa` 420; `aeece6c`, the build tip, 421; `6ec7601` (PREFIX) 421; `HEAD` 422. Test functions in the file: 2 at `main` and `18176fa`, 3 at the build tip, 4 after the `T-1` fix commit.

The two ledger sentences, read in full and in their own paragraphs:

- `docs/plans/agent-scaffold.ledger.md:567`, in the paragraph "INC3 IS BUILT AND ITS WORK REVIEW IS OPEN (2026-08-05)": "421 tests passing with 0 failures across nine binaries, WHICH IS MAIN'S 420 PLUS THE ONE NEW TEST". Measured: main 420, build tip 421, one test function added by the build. TRUE.
- `:535`, in the paragraph following "THE ROUND 1 FIX PASS IS DONE (2026-08-06 ...)": "422 tests passing with 0 failures, UP FROM 421 by the one new test". Measured: the pre-fix-pass branch tip is 421, the fix pass added one test function, `HEAD` is 422. TRUE.

### Where the finding went wrong

Its arithmetic is right and its attribution is not. It targets `git diff 18176fa..HEAD` as "the fix pass alone". That range is THE WHOLE INCREMENT, not the fix pass: this branch has been rebased so that the code commits sit ON TOP of the round 1 documentation commits, so `de18155` (the increment's build, which the reviewer calls `6de3a8f`) comes AFTER `18176fa` in the final history. The reviewer therefore counted the BUILD's one new test as the fix pass's, concluded the fix pass added two, and declared 420 the "true pre-fix-pass baseline". 420 is main's baseline and `18176fa`'s; the pre-fix-pass BRANCH TIP is 421, measured on a build of it.

I did not accept the orchestrator's reading because it was offered. I measured the counts and read both sentences before comparing, and my measurement agrees with the orchestrator's (2 test functions at main, 3 after the build, 4 after the fix pass). The orchestrator is right on this one.

ONE HONEST QUALIFICATION, offered as an observation and not as a finding: `:535` opens "against main the whole increment is EIGHT files ..." and then switches frame inside the same gate clause to "up from 421", which is against the pre-fix-pass tip. The frame switch is implicit. It is not ambiguous in context, because the other gate sentence names main's 420 explicitly, but a reader skimming one paragraph can land where this reviewer landed.

### AND IT SHOULD NOT BE IN THIS ROUND'S COUNT AT ALL, whichever way it had gone

The reviewed product is `git diff main...HEAD`: eight files, `src/main.rs`, two test files, `README.md`, `CHANGELOG.md`, `pack/AGENTS.md` and its two deployed copies. `docs/plans/agent-scaffold.ledger.md` IS NOT IN THAT DIFF. It is orchestrator-owned, and this task's own build record states that zero orchestrator-owned files were touched, checked by name-only diff, precisely because a worktree writing to them is a recorded defect here.

MY VIEW ON THE RIGHT HANDLING, since the brief asks: a defect in the ledger should be REPORTED to the orchestrator as a correction to the durable record, never counted in the round's valid-finding tally and never allowed to decide whether a round is clean. Two reasons. First, the tally drives convergence, and letting a journal entry break a clean round lets the record of the work fail the work. Second, the ledger is the orchestrator's own artifact, so counting findings against it in a round the orchestrator convenes puts the orchestrator in the position of grading itself, which is the exact separation this triage step exists to protect. The right channel is a note to the orchestrator, and I would keep the reviewers' latitude to look at it, because an independent reader checking the durable record is worth having; only the counting is wrong.

---

## Tally

| Severity | Valid | Findings |
| --- | --- | --- |
| critical | 0 | |
| high | 0 | |
| medium | 1 | `V-1` (`R2A-1`) |
| low | 3 | `V-2` (`R2A-2`), `V-3` (`R2A-3`), `V-4` (`R2B-1`) |
| TOTAL VALID | 4 | |

Not counted valid: `R-1` (`R2A-4`) ACCEPTED AS A RESIDUAL, `low`; `X-1` (`R2B-2`) INVALID; `X-2` (`R2B-3`) INVALID and out of scope.

NO SEVERITY WAS RE-RATED IN EITHER DIRECTION. I considered raising `R2A-2` to `medium` (rejected: `T-1`'s falsehood was static and every-time, this one needs a mid-run filesystem change, and the exit code is right in every cell) and lowering `R2A-1` to `low` (rejected: it is a red gate on the project's own newest material, with a message that describes the opposite of what happened, in an environment the project's own pack ships an isolation tier for).

REMEDY SHAPE, since this project measures it: of the four valid findings, one is a pure DELETION (`V-4`, two words), one is a MOVE of an existing statement with nothing added (`V-1`), one is a two-line REBINDING that authors no prose (`V-2`), and one is a nine-word clause COPIED from a neighbouring string (`V-3`). NO NEW SENTENCE IS AUTHORED ANYWHERE IN THIS ROUND'S REMEDIES. `V-3` is the only one that adds characters, and it is the one I would drop first if the human wants a zero-authored-words fix pass.

`V-1` and `V-2` touch the same test file and the same function's surroundings respectively but not the same lines; all four are independent and can land in any order.

## What the reviewers missed, that I found while reproducing

1. **THE CLIPPY DISAGREEMENT IS AN ENVIRONMENT DISAGREEMENT, AND THE RESIDUE REVIEWER WAS OUTSIDE THE PROJECT TOOLCHAIN.** The residue lens reports `cargo clippy --all-targets -- -D warnings` FAILING on a pre-existing `dead_code` lint over `enum_field!`'s `VARIANTS`; the adversarial lens reports it CLEAN. Both are reproducible, and the discriminator is the toolchain:

   ```
   direnv environment:  clippy 0.1.98 (nix flake)   -> exit 0, clean, at HEAD
   default PATH:        clippy 0.1.88 (/usr/bin)    -> exit 101, "associated constant `VARIANTS`
                                                       is never used" x2, at HEAD
   default PATH:        clippy 0.1.88               -> the SAME failure at `main`
   ```

   `/usr/bin/cargo` sits ahead of the nix store on this machine's default PATH, so any shell that skips `direnv allow && eval "$(direnv export bash)"` silently gets the system toolchain, which is the failure mode this project has recorded before. The project's gate is the flake toolchain and it is CLEAN at `HEAD`; the 1.88 failure also reproduces unchanged at `main`, so it is neither new nor this increment's. NOT A FINDING, but the residue lens's gate line should be read as measured outside the project environment, and the same caution applies to anything else in that file that depended on a build.

2. **THE SPECIFICATION REQUIRES THE TEXT `R2B-2` WANTS DELETED.** Set out under `X-1`. Neither reviewer opened `workflow-enforcement-tier.md:370-372` while forming a remedy against the three sentences it governs, and one of them proposed deleting a clause the spec names as mandatory and a phrasing the spec names by line number as the house form. A remedy has to be checked against the spec, not only against the behaviour.

3. **THE ROUND 2 REVIEW TARGET WAS MIS-FRAMED BY THE RESIDUE LENS.** It states its target as "the fix pass alone, `git diff 18176fa..HEAD`". Because of the rebase that put the code commits on top of the documentation commits, that range is the ENTIRE increment. No harm resulted (a wider target is safe), except in `R2B-3`, where the mis-framing produced the finding.

4. **`R2A-2`'s REMEDY DELIVERS LESS THAN ITS HEADLINE SUGGESTS, AND I MEASURED THE SHORTFALL.** Set out under `V-2`: the single probe does not change cell 1's output. A human reading only the finding's title ("one of those disagreements is verbatim `T-1`'s falsehood") could conclude the remedy removes that sentence. It does not, and no single-stat implementation can.

5. **THE REPLACEMENT HASH OFFERED IN `R2B-1`'s REMEDY IS ALSO WRONG**, twice over: `7ce4443` is unreachable from every ref and absent from a fresh clone, exactly like the hash it would replace, and it is not the commit immediately preceding the fix in the reviewer's own ordering. This does not weaken the finding; it strengthens the case for the deletion over any substitution.

6. **NO OTHER TEST IN THIS REPOSITORY IS ROOT-FRAGILE.** I ran the whole suite as namespace root at four commits (`main`, the build tip, PREFIX and `HEAD`) rather than only at `HEAD`, and the failure appears only at `HEAD` and only in the new test. That is the control that makes `V-1` an attribution rather than a suite property, and it also means fixing `V-1` restores a property the repository currently has everywhere else.

## Relitigation and constraints check

I checked every ruling above against the settled list. NOTHING here re-raises the four residuals (the in-root bound, the single-anchor `..` case, the earlier increment's rejected-ledger context slot, the off-convention `--source` surface), accepted costs (i) through (iv), round 1's `ADV-4` or `SC-3`, or the queued plain-`validate` inconsistency. `R-1` is adjacent to that last one, says so, and is routed INTO that queue rather than raised against this increment. `V-2`'s remedy is expressly NOT `Q-55-existsgate`'s declined `try_exists()?` change and is measured byte-identical on plain `validate` across 18 inputs to prove it. No line-length or wrapping observation appears anywhere in this file. I have no new evidence that any round 1 verdict or any human decision was wrong.

The reviewed worktree was never edited: `git status --short` is empty, both here and in the main repository. Both candidate remedies were built and measured in exported copies under `<scratch>/tri2/build/`. No `nix fmt` and no `just scaffold-self` was run.
