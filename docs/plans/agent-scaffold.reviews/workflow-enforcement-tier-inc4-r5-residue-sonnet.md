# `workflow-enforcement-tier-inc4`, round 5: residue lens (sonnet)

Scope: exactly what the round 3 and round 4 fix passes changed, on branch `review/wet-inc4-r5-b`
at `cf9ff9c`. The four commits audited, oldest first: `b1a7ab6` (round 3 remedies), `25e419a`
(closes `Q-55-reasondefs` in the two `.md` files), `9c9aa00` (closes the same clause in
`src/next.rs`), `cf9ff9c` (round 4 remedies: the impact list, check 21's tense branch, the
planner-pass sentence). Full range diffed as `git diff a0e6432..cf9ff9c` (5 files,
37 insertions, 31 deletions) and cross-checked commit by commit with `git show`.

I did not touch the other two lenses' territory (the source-side claim surface generally, or
which lens types remain unrun). Everything below is confined to the four commits' own content.

## Counts

Authored prose checked: all four new/amended `INC4:` impact-list bullets (the two new bullets,
the amended `src/main.rs` bullet, and the new `src/next.rs` bullet) plus check 21's new
revision-matching clause, roughly 150 words together. Of the four bullets, three verified fully
accurate; one is incomplete (finding `R5B-1`). The check 21 clause is not false but is
under-specified in a way that produces false negatives under a literal reading (finding
`R5B-2`).

Deletions checked: five sites across three commits: `25e419a`'s two (`log-not-this-project`,
`ledger-not-this-project` in the two `.md` files), `9c9aa00`'s two (the same clause in
`src/next.rs`), and `cf9ff9c`'s one (the "THIS FILE IS THE SECOND PLANNER PASS" sentence in the
`Q-55` record). All five confirmed TRUE deletions (nothing false was left behind) and none
produced a finding; see "Checked, no finding" below for the reasoning on each.

Round 3's four other token/deletion fixes (`R3B-1`'s two sites, `R3B-2`, `R3B-4`, `R3B-5`) were
re-verified against the current tree and against a built binary where behavioural; all four
still hold exactly as the round 3 triage specified them, with no regression from round 4's later
edits.

## Findings

### `R5B-1` (medium): the INC4 impact list still omits a site the increment edited

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:382-391` and the mirrored block at
`docs/plans/agent-scaffold.md:1777-1786` (the `INC4:` documentation-impact list, as closed by
`cf9ff9c`/`R4B-2`).

`R4B-2`'s own ground for existing was that this list "omitted ... sites the increment edited,
and enumerates its exclusions, so it read as exhaustive." The round 4 fix added four things: a
bullet for the two plan TOML regions, a bullet for the fourth sidecar
(`status-resume-ignores-json.md`), a bullet for `src/next.rs`'s two reason definitions, and
`run_status`'s comment tacked onto the existing `src/main.rs:Projection` bullet. Checking the
full `src/main.rs` diff across the whole branch (`git diff main..HEAD -- src/main.rs`) shows
THREE distinct edited sites in that file, not two:

1. `src/main.rs:570`, the `Projection.plan` field doc comment. Covered by check 22 (named in the
   bullet).
2. `src/main.rs:1194`, `run_status`'s inline comment. Named in the bullet ("`run_status`'s
   comment, which no acceptance check states").
3. `src/main.rs:461`, `StatusArgs::resume`'s field doc comment, which is the CLI's `--help`
   text. Edited in `b1a7ab6` (`R3B-3`) at the same time as site 2, for the identical reason
   (both under-enumerated the third cause). NOT named in the bullet.

I confirmed site 3 is a real, distinct, currently-live edit: `./target/debug/agent-scaffold
status --help` (built from this worktree) prints "Exits 0 with a note when the ledger is
absent, carries no such section, or is not this plan's" for `--resume`, matching `R3B-3`'s
prescribed text exactly, so the fix itself is correct. What is missing is the impact-list
bullet naming it.

Site 3 is not silently absent from the whole document: the closing bullet, "NOT `README.md`,
NOT `pack/AGENTS.md` ... NOT `CHANGELOG.md`, for the same reason: inc4 corrects one user-visible
string, `src/main.rs:StatusArgs::resume`'s `--help` text, and a corrected help string is a
documentation fix rather than a change to read about," does cite it by name. But that bullet's
job is to explain why `CHANGELOG.md` is unaffected; it is not the affirmative "this is a site
this increment touched, and here is what covers it" bullet every other edited site gets. No
check number covers site 3 (there is no dedicated acceptance check for the `--help` text, the
way check 22 covers site 1), so the correct treatment, on the pattern the list itself uses
everywhere else, is "no acceptance check states it" alongside `run_status`'s comment, which sits
one clause away in the very sentence `R4B-2` amended.

This is the same defect shape `R4B-2` was raised and fixed for, one round later, on the fix that
was supposed to have closed it: a documentation-impact list drawn up as if exhaustive, missing
one of the sites its own describing prose (`b1a7ab6`'s commit message: "The `--help` correction
is inc4's first edit to a user-visible surface") already knew about. Rated `medium` rather than
`low` on the same ground round 4's triage used for `R4B-2` itself: the list is the durable
record the queued `validation-constraints` step inherits, and a reader using it to answer "what
did this increment touch" gets an incomplete answer on the one user-visible surface inc4 has
ever changed.

MINIMAL REMEDY, stated for the record though authoring it is the next round's job: append
", and `StatusArgs::resume`'s `--help` string, which no acceptance check states either" to the
end of the existing `src/main.rs:Projection` bullet. No new fact; the text is already settled
by `b1a7ab6`'s own commit message.

### `R5B-2` (low): check 21's new revision-matching clause is ambiguous under a literal reading

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:345` (and the mirrored
`docs/plans/agent-scaffold.md:1740`), the clause `cf9ff9c`/`R4A-2` added: "a RE-TENSED quotation
must then match at the revision its tense names (`git show <pre-increment sha>:<path>`)".

RULING ON THE WRINKLE THE TASK POSED: yes, the underlying premise is real, and I independently
reproduced it, though the count I found is larger than "two of eleven." I identified six
quoted, re-tensed, non-ellipsis fragments in this file that attribute to a Rust source file and
checked each against the pre-fix revision named by its own tense (the commit whose parent
diff removed the quoted text):

- `:199`-equivalent (`src/next.rs:NextProjection::no_active_loop_reason`, "Why there is no
  active loop, for the human renderer. Not serialised (the JSON contract is exactly the fields
  above)"), checked at `8beb1c2^`. `git show 8beb1c2^:src/next.rs | grep -F "<quote>"` exits 1:
  the source comment reads "Why there is no active loop, for the human renderer. Not serialised
  (the JSON" on one `///` line and "contract is exactly the fields above); recomputed each
  call, never stored." on the next. MISS, by wrapping.
- `:200`-equivalent (`NextProjection`'s own doc comment, "Every derived part is optional so a
  missing plan or log yields a partial projection rather than a failure (mirrors `status`'s
  `Projection`)"), same revision. Source wraps "so" onto the end of one line and "yields" onto
  the next before wrapping again at "(mirrors" / "`status`'s `Projection`)". MISS, by wrapping.
- `:201`-equivalent (`src/main.rs:Projection`'s doc comment, "Every part is optional so a
  missing plan or metrics file yields a partial projection rather than a failure"), checked at
  the same `8beb1c2^` (confirmed via `git show 8beb1c2 -- src/main.rs`, which touches this exact
  comment). Source wraps "yields a" onto one line and "partial projection rather than a
  failure" onto the next. MISS, by wrapping.
- `:202`-equivalent (`src/next.rs:NextProjection::resume_state`'s field doc comment, "or `None`
  when the ledger is absent or carries no such section"), same revision. Source wraps "or
  `None` when the" onto one line and "ledger is absent or carries no such section." onto the
  next. MISS, by wrapping.
- `:347`-equivalent (check 22's quote, `src/main.rs:Projection.plan`, "present only when a
  readable `--plan` was given"), checked at `ce65169^`. Sits entirely on one line in the source.
  MATCHES a literal grep.
- `:367`-equivalent (`src/main.rs:run_resume`'s doc comment, "A missing ledger or absent section
  prints a note and exits 0"), checked at `8beb1c2^`. Sits entirely on one line. MATCHES.

So four of six non-ellipsis historical quotations in this file alone miss a literal
single-line `grep`, not two of eleven; I did not extend the sweep to `pack/AGENTS.md`,
`README.md`, or the test file's quotation, so the true denominator across everything check 21
now covers is larger than six. The mechanism is exactly as described: a naive
`grep -F "<quote>" <(git show <sha>:<path>)` fails on true, correctly-re-tensed quotations
purely because the Rust doc comment they quote wraps its `///` lines at a point the quotation
crosses, and check 21 as amended does not say the match tolerates that.

WHY I NEVERTHELESS RATE THIS `low` AND NOT `medium`. The ambiguity is not new to `R4A-2`'s
clause; it is inherited from check 21's original "run each quoted fragment ... as a literal
search" sentence, which predates this round's amendment and was already the sentence round 3's
own triage worked around (its `R3B-4` investigation found the identical class at `:52`, "failed
only a literal grep because the comment wraps across `//` lines," and resolved it by running "an
independent whitespace-normalised sweep" rather than by amending the check). `R4A-2`'s own
severity, set by round 4's triage, was `low`. A defect surfaced inside that same clause,
which changes no live text and misleads no user, is proportionate at the same level.

MINIMAL REMEDY, and the ruling the task asked for. Do NOT add a normalisation paragraph: check
21 has been amended three times in two rounds, and a fix pass that authors procedural prose here
is the exact class that produced `R4B-2` and now `R5B-1`. The smallest true fix is a token-level
qualifier on the existing "literal search" sentence (the general rule, not just the new clause,
since the ambiguity lives there): something on the order of ", with comment markers and
whitespace normalised" appended once, reusing the procedure round 3's triage already ran and
named rather than inventing a new one. If the orchestrator would rather not touch check 21 a
fourth time inside this cap round, recording this as an accepted procedural note for the next
reviewer (parallel to how the "ACCEPTED COST" items are pinned elsewhere in this file) is the
other defensible option; either is proportionate, a fresh paragraph of new policy is not.

## Checked, no finding

**The `TWO REGIONS OF docs/plans/agent-scaffold.plan.toml` bullet and check 21's identical
sentence** (`cf9ff9c`, drawn from `b1a7ab6`/`R3C-4`). Read literally, "a file this increment
edited: the `Q-55` question record and the three `workflow-enforcement-tier-w*` waiver notes"
could be misread as a claim that inc4 edited all three waiver notes individually. `git diff
main..HEAD -- docs/plans/agent-scaffold.plan.toml` shows only the `w1` note's figure and parts
of the `Q-55` ask text changed across the whole branch; `w2` and `w3` are untouched. But the
sentence's grammar names plan.toml as the edited file (true) and then lists the two SCOPE
REGIONS check 21 must verify citations against (a scoping statement, not an edit-history claim),
and all three waiver ids (`workflow-enforcement-tier-w1/w2/w3`) genuinely exist as one
contiguous, nameable group. Not false; I would not have written it identically, but that is not
the bar.

**The FOURTH sidecar bullet** (`status-resume-ignores-json.md`, `cf9ff9c`). Verified against
`git diff main..HEAD` for that file (matches the `run_status` comment fix exactly) and against
check 21b's own text, which names exactly three sidecars and none of them is this one. TRUE.

**The `src/next.rs`'s two containment reason definitions bullet** (`cf9ff9c`). Verified against
`9c9aa00`'s diff and against checks 1-23, none of which names `MetricsAbsentReason` or
`ResumeStateAbsentReason`'s doc comments. TRUE.

**The two `src/next.rs` deletions** (`9c9aa00`) and **the mirrored `.md` deletions**
(`25e419a`). Both leave "the root" without an explicit antecedent inside the same doc comment
(`src/next.rs:105-107`, `140-143`): "The resolved path is not under the root, so the tool cannot
vouch ..." does not itself say root of what. I checked whether the enclosing enum's own doc
comment (`src/next.rs:95-99` for `MetricsAbsentReason`) supplies it: it does not, so a reader
who lands on the variant in isolation (rustdoc, an IDE hover) has no antecedent in this file. I
then checked whether "the sidecar it copied from has the same property," as the round 4 writer
noted: at the sentence level, yes (`workflow-enforcement-tier.md:217`, `:229` read the same way
standing alone). But the sidecar is a linear narrative, and roughly sixty lines earlier the
section "WHY THE ROOT COMES FROM THE CHECKED PLAN AND NOT FROM THE ANCHOR" establishes what
"the root" means for a reader proceeding top to bottom, so the sidecar's version is materially
less orphaned than the code's. Even so, this is not a FALSE statement in either location, both
enum-level doc comments describe the "root" concept adequately for the code's own stated
design (`:212`: "THE ENUM IS THE MACHINE VALUE ONLY ... the human text still names the log and
the root," i.e. the doc comment is not supposed to carry that detail; the constructed message
does), and round 3's own precedent (`R3B-1`: "I recommend taking the deletion and ACCEPTING the
residual, and I record it here so a round-4 reviewer raising it is met with a decision rather
than a surprise") treats an acknowledged, minimal residual like this as accepted rather than a
defect calling for more authored prose. No finding.

**The `Q-55` record deletion** (`cf9ff9c`/`R4B-4`, "THIS FILE IS THE SECOND PLANNER PASS."
removed from `workflow-enforcement-tier.md:14`). Checked for a dangling antecedent: the
following sentence now opens "The first pass scoped two defects and two increments ..." and
later says "... are superseded here," where "here" previously resolved to the deleted "THIS
FILE." I checked every other use of "the second planner pass" in both files (five occurrences,
`workflow-enforcement-tier.md:169,193` and `agent-scaffold.md:160,1564,1588`); none depends on
the deleted sentence as an antecedent, each is self-contained ("the second planner pass raised
X ..."). "The first pass" and "here" remain comprehensible from the immediately surrounding
sentences (the paragraph is still clearly contrasting an earlier, narrower scoping against the
current, corrected one) and from the preceding paragraph's mention of "a DESIGN PASS," so this
is looser writing, not a false or unresolvable claim. No finding. I separately confirmed the
deletion's own premise: the second planner pass "re-derived the set as THREE" per
`docs/plans/agent-scaffold.plan.toml:1726` (`Q-55-scope`'s note), while the file as it stands
carries FOUR increments (inc4 was added later by a further human decision), so
"THIS FILE IS THE SECOND PLANNER PASS" was itself an overclaim and removing it is correct.

**Round 3's fixes** (`b1a7ab6`), re-verified independently of round 4's own review:

- `R3B-1` (the two union-scoped deletions). Current text at `workflow-enforcement-tier.md:163`
  ("`status --resume` reads NO plan ...") and `:179` ("The trigger is the SAME containment
  predicate the validator's refusal uses. The predicate is never re-implemented per surface")
  is unchanged since `b1a7ab6` and matches the round 3 triage's prescribed remedy exactly, word
  for word.
- `R3B-2` (the symlink-divergence token fix at `:104`-equivalent). Current text reads "except
  for the symlink divergence recorded below as accepted cost (ii)," matching check 19's own
  wording, and check 19 still pins the second (log-side) symlink layout the original text
  excluded. Consistent.
- `R3B-3` (the `--help` string and `run_status`'s comment). Verified behaviourally: built this
  worktree (`cargo build`) and ran `./target/debug/agent-scaffold status --help`, which prints
  the corrected three-cause text verbatim. `src/main.rs:1194`'s comment matches too.
- `R3B-4` (the tense fix at `:367`-equivalent, "GAINED the unsafe-fragment case"). Present tense
  is gone, past tense is in place, and `run_resume`'s doc comment at `src/main.rs:1628-1636`
  does carry the three-cause list the sentence now describes as already landed.
- `R3B-5` (deleted "immediately" at `:157`-equivalent). Confirmed via diff; the surrounding
  sentence's substantive claim ("available BEFORE the match," "does not force the guard down
  into the arms") is otherwise unchanged and still true (`toml_primary` at `src/main.rs:979`,
  the match at `:1005`, matching round 3's own line count).
- The two closed reason definitions (`25e419a`, closing `Q-55-reasondefs` in the `.md` files).
  Covered above under "Checked, no finding" together with `9c9aa00`'s mirrored code deletion;
  no new issue beyond what is discussed there.

None of the six round 3 items produced a new finding on re-check; all six hold exactly as round
3's triage left them.

## What I did not reach

I did not sweep `pack/AGENTS.md`, `README.md`, or the test file's quotation for further
`R5B-2`-class wrapping misses beyond the six checked in this file; the true count across
everything check 21 now names as in scope is very likely higher than the six I sampled. I did
not re-audit the source-side claim surface generally (paragraphs and citations this round's fix
did not touch), and I did not enumerate which lens types have run across all five rounds; both
are the other two reviewers' assignments this round.
