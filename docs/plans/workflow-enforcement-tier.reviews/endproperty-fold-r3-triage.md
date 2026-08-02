# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 3, TRIAGE

Triager: independent of the planner, of both round 3 reviewers, of both round 2 reviewers, of both round 1 reviewers, and of both prior triagers. Read-only with respect to the reviewed artifact; this file is the only thing written. No fix is applied here.

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/triage-ep3`, branch `triage/q55-ep3`, cut from the reviewed tip `133324d` so every reviewer citation resolves against the reviewed text. Binary built at that commit, so INC2 IS NOT LANDED and every run below is a PRE-INC2 measurement. Scratch under `TMPDIR=/tmp/claude-1000/triage-ep3-scratch`, created for this triage and removed at the end.

Repository guards re-run at the reviewed commit, both green, so `docs/plans/agent-scaffold.md` is a faithful mechanical regeneration and never an independent authored site:

```
$ cargo run -q -- render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date
render exit: 0

$ cargo run -q -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 255 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
validate exit: 0
```

Population searched for every site count below: all of `docs/plans/agent-scaffold.steps/`, plus `docs/plans/agent-scaffold.plan.toml`, plus the generated `docs/plans/agent-scaffold.md`. Site counts separate AUTHORED sites (a human edits them) from MECHANICAL ones (`render` regenerates them).

## Verdict summary

THREE raw findings from two lenses. `R3A-1` and `R3B-2` are the SAME defect, confirmed and merged below, so TWO distinct findings. ONE is valid and requires a fix; ONE is dismissed.

DEDUPLICATED VALID COUNT: 1. SEVERITY LIST: 1 low.

| id | verdict | final severity | ground |
| --- | --- | --- | --- |
| `R3A-1` / `R3B-2` | VALID (merged) | low (`R3A-1` re-severitised DOWN from medium to `R3B-2`'s low) | Reproduced exactly: ten distinct `Q-55*` receipts exist in the log, the bulleted registry names nine, and `Q-55-resumecost` is the omission. Nothing asserted is false; the list asserts its count in unary, which is the same species the same fix pass already retired at the numeral one line above. |
| `R3B-1` | INVALID, dismissed | would have been medium at most | Legs 1 and 2 reproduce (check 9 is labelled AFTER INC1 and nothing in 11 to 19b re-runs it), but leg 3 is FALSE and the conclusion does not follow: `tests/metrics_and_ledger_anchor_to_the_plan_source.rs`'s `the_correct_case_prints_the_same_relative_paths_it_always_did` byte-compares the WHOLE stdout of a bare-relative `--source` run and is executed by check 1's unlabelled `cargo test`, which every increment owes. |

No high or critical was dismissed, so NO BACKSTOP RE-CHECK IS OWED. See "Backstop" below for the explicit statement.

## Deduplication: `R3A-1` and `R3B-2` ARE the same defect. CONFIRMED and MERGED

Both lenses cite the same passage (`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:10` and its nine bullets at `:12-20`), both run the same measurement (`grep -o '"q_id":"Q-55[^"]*"' docs/metrics/workflow.jsonl | sort -u` against the bullet count), both reach the same missing member (`Q-55-resumecost`), and both prescribe the same class of remedy (add one bullet). They are one defect found by two lenses, and the convergence is evidence of validity rather than a reason to count it twice. They are counted ONCE.

What each lens adds that the other does not, kept because both bear on the remedy:

- `R3A-1` supplies the provenance-of-the-provenance: the same fix pass deleted the stale numeral "NINE" one line above (commit `133324d`, whose own message reads "Deleting the count rather than restating it means the sentence has nothing left to go stale") and did not give the list below it the same treatment. That is the argument that the list and the numeral are the same claim in two notations.
- `R3B-2` supplies the cold reader's stake: `:263` instructs two audiences that "an implementer must NOT 'fix' them, and a reviewer must NOT raise them as defects", and the authority for that instruction is "each was put to the human". It also supplies the exculpatory history, that the tenth receipt did not exist when round 2 counted, so no prior round passed over this.

I verified `R3B-2`'s claim that the id appears exactly once in the document: `grep -c '`Q-55-resumecost`' docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` returns 1, at `:263`.

## `R3A-1` / `R3B-2`, VALID, low. The PROVENANCE registry omits `Q-55-resumecost`

### Reproduced

```
$ grep -o '"q_id":"Q-55[^"]*"' docs/metrics/workflow.jsonl | sort -u
"q_id":"Q-55"
"q_id":"Q-55-conventionlesscost"
"q_id":"Q-55-endproperty"
"q_id":"Q-55-jsonreason"
"q_id":"Q-55-mechanism"
"q_id":"Q-55-noconvention"
"q_id":"Q-55-refusalscope"
"q_id":"Q-55-resumecost"
"q_id":"Q-55-resumepairing"
"q_id":"Q-55-scope"
(10 distinct)

$ grep -c '^- `q_id:"Q-55' docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md
9
```

Ten receipts, nine bullets, `Q-55-resumecost` missing. Confirmed.

Both reviewers' record-number citations resolve. `Q-55-resumecost` is at log line 255 and `Q-55-resumepairing` at 253, exactly as `R3A-1` states; the intervening 254 is the round 2 round record, which is why round 2's count of nine was correct when it was taken.

### One citation correction inside the valid finding (`Q-66`)

`R3A-1` writes that because both `Q-55-resumepairing` and `Q-55-resumecost` are dated `2026-08-02`, "the list's own qualifier 'the last taken on 2026-08-02' no longer identifies which receipt is actually last". THAT SUB-CLAIM IS WRONG and I do not carry it. The qualifier asserts a DATE, not an identity, and it never identified a receipt: FOUR of the ten receipts carry `ts:"2026-08-02"` (`Q-55-endproperty`, `Q-55-conventionlesscost`, `Q-55-resumepairing`, `Q-55-resumecost`), so the sentence was already a date claim about a set when it was written, and it is TRUE today, because the last receipt taken (record 255) does carry that date. The finding survives without the sub-claim; the sub-claim is corrected rather than inherited. It does, separately, bear on the prescription, for the reason given under "the second site" below.

### Severity: LOW, re-severitised DOWN from `R3A-1`'s medium

Against `R3A-1`'s medium argument, which rests on the list being "a structured, purpose-built registry" and on the project having treated the adjacent numeral as worth a same-round fix:

- NOTHING ASSERTED IS FALSE. This is the whole distance between the list and the numeral it sits under. "NINE decision receipts" was FALSE when ten existed; a nine-item list with no count over it is INCOMPLETE. This project already drew that line: the round 2 triage accepted `R2B-3` as residual on precisely the ground that "nothing asserted is false", against summary paragraphs naming three of five decisions.
- THE MISSING RECEIPT IS ATTRIBUTED IN THE DOCUMENT ANYWAY, at `:263`, in the paragraph that introduces the four accepted costs and states that each was measured, put to the human, and accepted. A reader of cost (iv) has a named receipt eight lines above it.
- NO CHECK'S EXIT CODE, AND NO SPECIFIED BEHAVIOUR, DEPENDS ON THE LIST.

Against `R3B-2`'s low argument, nothing: I agree with it. `R3A-1`'s distinction from `R2B-3` (a bulleted registry does hold itself out as exhaustive in a way a prose summary does not) is fair and is why this is a valid finding at all rather than a second accepted residual. It raises the finding above `R2B-3`; it does not raise it to medium.

The severity does not change the prescription, because the prescribed fix is deletion and costs nothing.

### THE REMEDY. Deleting the numeral was NOT sufficient, and the right remedy is (b), DELETE THE ENUMERATION

Ruling on the three options the brief names.

(a) ADD THE MISSING BULLET, which both reviewers prescribe: REJECTED. It is the one remedy the evidence in this loop has already falsified. The failure being repaired is not "a bullet was forgotten"; it is "a hand-maintained copy of an append-only log went out of sync with the log while the loop was open". The receipt landed at record 255 AFTER the round 2 triage verified the then-current registry, and the orchestrator will keep appending to that log for as long as this step is open. This is not speculative: FOUR of the ten receipts (`Q-55-endproperty`, `Q-55-conventionlesscost`, `Q-55-resumepairing`, `Q-55-resumecost`) landed during this very review loop, all dated `2026-08-02`, and rounds 4 and 5 plus their fix passes are still to come. Adding a tenth bullet restores sync for exactly as long as it takes the human to decide one more thing, and then round 4 or round 5 finds the identical finding with an eleventh receipt in it. A remedy that has to be re-applied every time the source changes is not a fix, it is a maintenance obligation, and this fold has now demonstrated twice in one loop that the obligation is not met.

(b) DELETE THE ENUMERATION: ADOPTED. Four grounds.

1. IT IS THE SAME TREATMENT THE SAME PASS ALREADY APPLIED ONE LINE ABOVE, and for the reason the orchestrator recorded as a standing rule: an artifact under review must not assert a count of anything the orchestrator appends to during the loop. A nine-item bulleted list under a preamble that says "with decision receipts in `docs/metrics/workflow.jsonl` ... all carrying `task:"workflow-enforcement-tier"`" ASSERTS THE COUNT IN UNARY. The standing rule already covers it; deleting the numeral and keeping the list satisfied the rule's letter and missed its subject.
2. IT IS THE ONLY REMEDY THAT SURVIVES CONTINUED APPENDS. What survives deletion is the SELECTOR, not a snapshot: `docs/metrics/workflow.jsonl` filtered on `task:"workflow-enforcement-tier"`. A selector stays correct no matter how many receipts land, because it names the query rather than the answer. The list can only ever be correct as of its last hand-edit.
3. IT IS WHAT ONE SOURCE OF TRUTH REQUIRES, and the document invokes that principle by name twice for its own mechanism (`:189` "The predicate is never re-implemented per surface (One source of truth)", `:229` on retyping rather than paralleling a reason field). The log is the source of truth for what was decided; the list is a second representation of the same fact that can disagree with it, which is precisely the shape `:229` rejects. It is also what this project's structured-data direction requires: the receipts are structured data, the document is a human view over it, and a human view should carry the query rather than a stale copy of the result.
4. IT COSTS NOTHING THAT IS NOT ALREADY PAID FOR ELSEWHERE IN THE SAME FILE. I checked this rather than assuming it. Every one of the ten receipts is separately cited in the body AT THE SECTION IT GOVERNS, which is where a reader actually needs it: `Q-55` at `:58`, `Q-55-scope` at `:285`, `Q-55-mechanism` at `:155`, `Q-55-noconvention` at `:163`, `Q-55-refusalscope` at `:179`, `Q-55-jsonreason` at `:203`, `Q-55-endproperty` at `:169`, `Q-55-conventionlesscost` at `:269`, `Q-55-resumepairing` at `:192`, `Q-55-resumecost` at `:263`. Eight of the ten carry a human-and-date attribution at that site. What deletion removes is a convenience index, not the provenance.

I considered and REJECT `R3B-2`'s explicit argument against deletion ("deleting it or its framing would lose the provenance for all ten rather than complete it for one"). It would be right if the preamble sentence went with the bullets. It does not: the sentence names the file and the exact filter, and the per-decision attributions listed above stay where they are. The provenance is not lost, it stops being duplicated.

(c) SOMETHING ELSE: considered and rejected, recorded so the fix pass does not reinvent either.

- NARROW THE FRAMING so the list is explicitly non-exhaustive (for example "the decisions this file turns on include:"). Rejected: it authors words, and worse, it does not actually close the finding, because `Q-55-resumecost` IS a decision this file turns on (it authorises accepted cost (iv), which check 19b pins), so the list would still be incomplete against its own narrowed claim. A narrowing vague enough to be safe would be vague enough to be useless in a provenance record.
- REPLACE THE BULLETS WITH A POINTER SENTENCE (for example "each stated at the section it governs"). Rejected: it authors words to assert a structural property of the document that a future edit can falsify, which is the re-seeding class, and it buys a reader nothing the selector sentence does not already give.

### THE PRESCRIBED MINIMAL FIX

TWO SITES IN ONE SENTENCE AND THE BLOCK BELOW IT, both deletions, ZERO authored words.

THE FIRST SITE, the enumeration. Delete `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:11-20` (the blank line and the nine bullets), and change the colon that terminates `:10` to a full stop.

THE SECOND SITE, in the same sentence: delete the parenthetical "(the last taken on 2026-08-02, after inc1's work review)". THIS IS TRIAGER-INITIATED, NOT FROM EITHER REVIEWER, and it is included because it is the SAME finding measured properly rather than a new one. The finding is "this passage asserts a property of a set the orchestrator appends to during the loop"; the bullets assert its cardinality and the parenthetical asserts its maximum. The parenthetical is TRUE TODAY (record 255 does carry `ts:"2026-08-02"`, verified above) and goes false the moment one more `Q-55` decision is taken on a later date, which is the same failure with the same cause and the same one-week horizon. Leaving it means the fix pass edits this sentence and leaves a live instance of the rule's own target inside it. Nothing is lost: the fact it carries, that decisions post-date inc1's work review, is stated at `:169` ("human, 2026-08-02 ... MEASURED AT WORK REVIEW on inc1") and at `:269`.

What remains after the fix, in full:

```
PROVENANCE. `Q-55`, decided by the human on 2026-07-31, with decision receipts in `docs/metrics/workflow.jsonl`, all carrying `task:"workflow-enforcement-tier"`.
```

"decided by the human on 2026-07-31" STAYS and is not a third instance: its subject is `Q-55` proper, whose receipt is at log line 234 carrying `ts:"2026-07-31"`, which I checked rather than assumed. It is a past fact about one receipt, not a property of the set, so no append can falsify it.

AUTHORED WORD COUNT: 0. No new prose. One character changes class (`:` to `.`), which is mechanical punctuation, not authorship.

DELETED: 10 lines and 123 words at the enumeration, plus 9 words at the parenthetical. 132 words deleted, 0 authored.

MEASURED SITE COUNT, grepped over ALL of `docs/plans/agent-scaffold.steps/`, `docs/plans/agent-scaffold.plan.toml` and `docs/plans/agent-scaffold.md`:

```
$ grep -rn '^- `q_id:"Q-55' docs/plans/agent-scaffold.steps/ docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md
(18 hits, summarised as their two contiguous runs)
docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:12 through :20   (9 lines, AUTHORED)
docs/plans/agent-scaffold.md:1407 through :1415                               (9 lines, MECHANICAL)

$ grep -rn 'the last taken on 2026-08-02' docs/plans/agent-scaffold.steps/ docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.md
docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:10      (AUTHORED)
docs/plans/agent-scaffold.md:1405                                    (MECHANICAL)
```

1 AUTHORED SITE (the sidecar passage `:10-20`), 1 MECHANICAL SITE (`docs/plans/agent-scaffold.md:1405-1415`, regenerated by `render`, never hand-edited). `docs/plans/agent-scaffold.plan.toml` carries ZERO sites: it names six `Q-55` receipts inline in its own decision narrative (`Q-55`, `Q-55-scope` twice, `Q-55-mechanism`, `Q-55-noconvention`, `Q-55-refusalscope`, `Q-55-jsonreason`) but has NO bulleted registry and asserts no count over them, so it neither needs the fix nor acquires the defect. No other step sidecar carries a `Q-55` registry.

A NOTE FOR THE FIX PASS, because a triager's site count is a measurement and not an instruction: if applying this deletion turns out to leave some other passage referring back to the enumeration, report and widen rather than applying it literally. I looked and found none, but the reference class here is `INC2-6`, where a prescription was too small.

A NOTE FOR ROUND 4 AND ROUND 5, so this deletion is not re-raised as a fidelity finding: the enumeration was deleted DELIBERATELY, on the ruling above, and its removal is not an oversight. The provenance route is the selector in the surviving sentence plus the ten per-decision attributions enumerated in ground 4.

## `R3B-1`, INVALID, dismissed. The property IS pinned, by a stronger check than the one said to be missing

I verified all three legs, as instructed. The first two hold. The third is false, and it is the one the conclusion rests on.

### LEG 1, the label: CONFIRMED

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:331` reads "9. AFTER INC1, NO REGRESSION ON THE CORRECT CASE ... its three stdout lines are BYTE-IDENTICAL to the pre-fix binary's". The label is AFTER INC1 and names no later increment. True as cited.

### LEG 2, the absence in 11 to 19b: CONFIRMED

I enumerated every check's own increment label rather than trusting the reviewer's list:

```
1  (unlabelled: build, cargo test, clippy, render --check)
2  (unlabelled: rebuild the fixture)
3,4,5,6,7,8,9,10           AFTER INC1
11,12,13,13b,14,14b,14c,14d,14e,14f,14g,14h   AFTER INC2
15,16,17,20                AFTER INC3
18,19,19b                  accepted-cost pins, spanning increments
```

Reading 11 to 19b as a set: 11, 12, 13's first half, 13b's first two runs, 19 and 19b assert REFUSALS or omissions; 13's second half and 13b's third run are the only positive validator cases and assert exit code and which log was read, never a printed spelling; 14 asserts scoping; 14b to 14g are the projections and the machine surface; 14h is the JSON contract and reaches no printed path. NONE of them re-runs check 9. The reviewer's reading of the check set is accurate.

### LEG 3, that the suite cannot stand in: FALSIFIED

`R3B-1` writes: "THE SUITE DOES NOT COVER IT EITHER, so check 1's `cargo test` does not stand in. `tests/metrics_and_ledger_anchor_to_the_plan_source.rs` passes every fixture path as an ABSOLUTE string ... and asserts which log was read by record count and by exit code, never a relative printed spelling."

That is TRUE OF THE CROSS-PROJECT TESTS IN THAT FILE, and the file's own module doc says so ("the cross-project tests build several projects in one scratch tree and run the binary from the WRONG one, so which file was read is identified by CONTENT rather than asserted from the path"). The reviewer read that half and stopped. The SAME FILE contains a test that does exactly the opposite:

```rust
/// Acceptance check 9, the Safe on existing projects pin: a run made from the plan's own
/// project root with a BARE RELATIVE `--source`, which is the normal invocation, is
/// UNCHANGED, byte for byte. The whole stdout is compared rather than searched, because the
/// property is that the printed paths stay RELATIVE: an "improvement" that canonicalised
/// the default would still read the right file and still pass a `contains` assertion while
/// changing two of these three lines to absolute, machine-specific paths.
#[test]
fn the_correct_case_prints_the_same_relative_paths_it_always_did() {
	...
	let (code, stdout, stderr) =
		run(&home, &["validate", "--source", "docs/plans/p.plan.toml", "--workflow"]);
	assert_eq!(code, Some(0), ...);
	assert_eq!(
		stdout,
		"docs/metrics/workflow.jsonl: 3 records, valid\n\
		 docs/plans/p.plan.toml: 1 steps, 0 questions, valid\n\
		 docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold\n",
		"this spelling's output must be byte-identical to the pre-anchoring binary's"
	);
```

Every element the finding says is absent is present. It is a BARE RELATIVE `--source` (not an absolute one), run from the project's own root via the harness's `.current_dir(dir)`, and the assertion is `assert_eq!` over the WHOLE stdout (not a record count, not a `contains`), with the three relative spellings written out as the expected literal. Its doc comment names acceptance check 9 explicitly and names the exact collapse hazard `:175` warns about, in the same words.

It runs and passes at the reviewed commit:

```
$ cargo test --test metrics_and_ledger_anchor_to_the_plan_source
running 9 tests
...
test the_correct_case_prints_the_same_relative_paths_it_always_did ... ok
test result: ok. 9 passed; 0 failed
```

And it is executed at EVERY increment, because check 1 ("Suite and lint: `cargo test` and `cargo clippy --all-targets -- -D warnings`, both clean") carries no `AFTER INCn` label, unlike checks 3 to 20. Checks 1 and 2 are the preamble every increment owes; `:319` reinforces this by requiring each increment's own red-then-green test to be in that suite.

### That the pin actually catches the collapse, measured rather than reasoned

The finding's premise is that the printed spelling follows the resolved metrics path, so I measured it rather than inferring it:

```
$ cargo run -q -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 255 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold

$ cargo run -q -- validate --source "$PWD/docs/plans/agent-scaffold.plan.toml" --workflow
/home/.../triage-ep3/docs/metrics/workflow.jsonl: 255 records, valid
/home/.../triage-ep3/docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
/home/.../triage-ep3/docs/plans/agent-scaffold.plan.toml vs /home/.../triage-ep3/docs/metrics/workflow.jsonl: workflow invariants hold
```

The printed paths are the resolved paths verbatim. An inc2 that collapsed the split by canonicalising the metrics resolution in place would turn the first and third lines absolute while the source line stayed relative, which is exactly `:175`'s "two of the three printed lines change", and the `assert_eq!` above would fail with a diff naming the property. The suite pin is STRICTER than check 9 as written, because check 9 asks a human to compare against "the pre-fix binary's" output while the test carries the expected bytes inline.

### Ruling

The finding's two citation legs reproduce and its third does not, and the third is load-bearing: with it gone, "so nothing pins the lexical/canonical split that the document says MUST NOT BE COLLAPSED" is false. Something does pin it, permanently, at every increment, more strictly than the check the finding wants re-labelled. This is the second half of `Q-66`'s two jobs, a finding whose citations mostly reproduce but whose conclusion does not follow.

I also decline the fix on its own terms. Adding "AND AGAIN AFTER INC2" to check 9 would author four words to schedule a manual re-run of a property a byte-compare test already asserts on every `cargo test`, which is authored prose in the class this project has measured as re-seeding, bought for nothing.

DISMISSED. Not a residual: there is no residue to accept, because the artifact is not short of anything.

WHAT THE FINDING GOT RIGHT, recorded because it is worth carrying: its reading of the acceptance-check set is accurate and its identification of `:175` as the property at risk in inc2 is correct. It is a good finding with one unchecked negative in it, and the negative is the kind this project's own `Q-66` rule exists to force ("a grep returning zero is evidence about the PATTERN as much as about the repo", ledger orchestrator defect (4)). The reviewer grepped the cross-project half of one file and generalised to the file.

## Backstop

I dismissed ONE finding, `R3B-1`. The reviewer rated it `medium`, and my own assessment is that it would have been `medium` at most had leg 3 held, since the regression it fails to catch is an output-spelling change rather than a wrong verdict. NO HIGH OR CRITICAL FINDING WAS DISMISSED, so NO BACKSTOP RE-CHECK IS OWED.

## Residuals not re-raised, and out-of-scope items confirmed out

- `R2B-2` (the `--ledger-fragment` interaction) and `R2B-3` (summary paragraphs naming three decisions): accepted residuals, not re-raised by either round 3 lens, and not re-raised here.
- `EX-5`'s corrected site and the end-property versus copied-log tension: the cold-read lens reproduced the copied-log green and explicitly did NOT raise it, recording it as the honest answer to its own governing question. That is the correct handling of a round 1 ruling and I confirm it: `:290`'s inc2 bullet carries no end-property claim after the round 1 deletion, and the residual is recorded at `:277` and `:388`.
- The present-tense `src/main.rs` claims falsified by inc1 (the `--metrics` relative-default text, the `default_ledger_path` current-directory text, the "Documentation impact INC1" sub-list, the two help-string descriptions): deferred to the post-inc3 documentation-currency pass. Neither lens raised them as findings; both listed them as deliberately excluded. Confirmed out of scope.
- Accepted costs (i) and (ii), increments 1 and 3, and the six human decisions themselves: not revisited. Only whether the document records them correctly and executably was in scope, and only the merged finding bears on that.
- Line length and hard-wrapping: not raised by either lens and never a finding here.

## Round 3 fix pass, shape

ONE finding, ONE authored site, ZERO authored words, 132 words deleted. This is the smallest and best-conditioned fix pass of the three by the measure this project has calibration data for. Round 1 authored 498 words and round 2 found nine findings; round 2 authored 77 and round 3 found three raw, one valid. Round 3 authors nothing.

The pass has no prose to get wrong, so the usual escape valve has little to catch. The one thing to watch is the mechanical half: `docs/plans/agent-scaffold.md` must be REGENERATED by `cargo run -- render docs/plans/agent-scaffold.plan.toml` and never hand-edited, and its diff should be exactly the sidecar's ten deleted lines plus the two changed lines, nothing else.

## Scratch hygiene

All runs under `TMPDIR=/tmp/claude-1000/triage-ep3-scratch`, created for this triage and removed at the end. Nothing was written to bare `/tmp`. DIRECTORIES LEFT IN `/tmp`: 0.
