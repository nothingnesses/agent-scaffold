# Q-70 review: downstream-consumer lens

Reviewer: round-1 reviewer, consumer lens (what Q-70 will CAUSE an explorer to do, not
whether its claims are true). Target: `Q-70` as added to
`docs/plans/agent-scaffold.plan.toml` (and its rendering into `agent-scaffold.md`) by
this branch's diff against `main`. Read as a fresh explorer with no prior context,
per the brief.

## R1B-1 (severity: high)

Claim: `Q-70`'s opening `ask` sentence frames the owed design pass as covering only
two things (the W5 fix and whether it shares a mechanism with the W6 waiver-note
join), but the decision that Q-70 itself cites as the reason a pass is owed at all
(`Q-55-entryroute`) recorded a broader mandate: the W5 fix plus THREE detection
mechanisms. A fresh explorer who takes the opening sentence at face value will
under-scope their proposal.

Evidence. `Q-70`'s `ask` opens: "how to fix W5's waiver-ownership check, and whether
that fix shares a mechanism with the prospective W6 waiver-note join, so the two are
designed together rather than one at a time" (`docs/plans/agent-scaffold.plan.toml:1883`).
That is the only scope statement a reader encounters before the rest of the item's
detail.

The decision receipt this same item cites, `Q-55-entryroute`
(`docs/metrics/workflow.jsonl:308`), records `"chosen":"Design pass, validator
cluster only"` against three rejected alternatives including `"Design pass over all
four bodies"` and `"Split out the W5 fix first"` (the latter being exactly the
narrower, W5-only pass the opening sentence describes, and it was rejected). The
ledger's prose record of that same receipt is explicit about what "validator cluster
only" means: "ENTER `validation-constraints` VIA A DESIGN PASS OVER THE VALIDATOR
CLUSTER ONLY, meaning the W5 fix plus the three detection mechanisms"
(`docs/plans/agent-scaffold.ledger.md:533`).

So the human decision Q-70 relies on for its own existence assigns the pass four
things to resolve (the W5 fix, plus three named mechanisms), while Q-70's own
headline sentence names only two of the four (the W5 fix, and coupling with one of
the three, the W6 join). A reader who does not chase the `Q-55-entryroute` citation
back to the round log and the ledger, which the item gives them no reason to do
since the opening sentence reads as a complete scope statement, walks away with a
scope that is half of what was actually decided.

This is a judgement about what a fresh reader will conclude from the ordering and
framing of the text, not a claim that any individual sentence in Q-70 is false in
isolation: the opening sentence is not false, it is just incomplete in a way the
document does not flag at the point a reader would form their first mental model of
the task.

## R1B-2 (severity: high)

Claim: "WHAT THE PASS OWES BACK," the paragraph that (by its heading and by the
parallel structure Q-68 and Q-69 use) reads as the consolidated deliverables
checklist, does not ask for three things the item's own body states the pass "must"
produce. An explorer could satisfy every sentence of that paragraph and still ship a
proposal missing those three things.

Evidence. The full text of "WHAT THE PASS OWES BACK"
(`docs/plans/agent-scaffold.plan.toml:1901`): "Explorers write to
`docs/plans/validation-constraints.explorations/` ... Each proposal must rule
EXPLICITLY on the coupling hypothesis above, state the edit surface its direction
implies (naming which source files it touches, and in particular whether any
generated const or drift-guarded file is involved), and carry an explicit 'what not
to build' YAGNI boundary." That is three deliverables: a coupling ruling, an edit
surface, and a YAGNI boundary.

Elsewhere in the same item, three further "must" statements appear that this
paragraph does not restate:

1. "WHAT THE PASS OWES ON THIS: a ruling on WHICH PATH IS AUTHORITATIVE for waiver
   ownership, or whether both should be" (`docs/plans/agent-scaffold.plan.toml:1889`,
   inside "A SECOND WAIVER-VALIDATION PATH EXISTS").
2. "The pass must therefore STATE WHICH CHECK IT MEANS wherever it writes 'W6'"
   (`docs/plans/agent-scaffold.plan.toml:1897`, inside "A NAMING COLLISION THE PASS
   MUST NOT WALK INTO").
3. Design proposals at all for the two other items "THE THREE DETECTION MECHANISMS
   IN THE PASS'S SCOPE" (`docs/plans/agent-scaffold.plan.toml:1893`) names besides
   the W6 join: "DANGLING DECISION-RECEIPT DETECTION" and "A QUOTATION RESOLVER."
   The heading itself says these are "IN THE PASS'S SCOPE," and the paragraph
   immediately after contrasts them with items that are explicitly "OUT OF THE
   DESIGN PASS" (`:1899`), so the document's own structure marks these two as in
   scope, yet the deliverables paragraph never asks a proposal to address them.

Compare `Q-68`, which instead of a "WHAT THE PASS OWES BACK" paragraph gives an
exhaustive lettered list, "OPEN DESIGN QUESTIONS the pass must resolve (none
pre-decided): (a) ... (b) ... (c) ... (d) ... (e) ..."
(`docs/plans/agent-scaffold.plan.toml:1857`), so every question the pass owes an
answer to is in one place. Q-70 has no equivalent consolidated list; its "musts" are
scattered across four separate paragraphs and only one of the four is carried into
the closing checklist.

This is the concrete form of the review brief's "satisfies every instruction and is
still useless" case: a proposal that rules on the coupling hypothesis, states an
edit surface, and states a YAGNI boundary is fully compliant with "WHAT THE PASS
OWES BACK" as written, while omitting the authoritative-path ruling, the W6
disambiguation, and any design for two of the three mechanisms the item itself says
are in scope.

## R1B-3 (severity: low)

Claim: Q-70's citation for the Design-explorations rule drops the line number that
the otherwise near-identical sentence in Q-69 carries, a small but checkable
precision regression against its nearest precedent.

Evidence. Q-70: "per the Design explorations rule in `pack/AGENTS.md`"
(`docs/plans/agent-scaffold.plan.toml:1901`). Q-69's equivalent sentence: "per the
Design explorations rule in `pack/AGENTS.md:65`"
(`docs/plans/agent-scaffold.plan.toml:1876`). The rule is in fact at line 65 in both
`pack/AGENTS.md:65` and the rendered `AGENTS.md:65` (verified by
`grep -n "Design explorations" pack/AGENTS.md AGENTS.md`, both returning `65:`).
Dropping the line number costs a reader one extra search in a roughly 150-line file;
not a blocker, but a needless step back from a precedent this same item otherwise
follows ("the same shape `Q-68` and `Q-69` use," `:1903`).

## R1B-4 (severity: low)

Claim: Q-70 states "This item carries NO options and NO recommendation,
deliberately" up front, but later names two concrete candidate fix shapes without
Q-69's caveat that such named directions are candidates, not a decided option set.

Evidence. Opening: "This item carries NO options and NO recommendation,
deliberately: it registers the question and the pass's inputs, which is what
`status = "exploring"` is for" (`docs/plans/agent-scaffold.plan.toml:1883`). Later,
"THE COUPLING HYPOTHESIS THE PASS MUST SETTLE" paragraph: "the answer decides the
shape of the fix: a narrow lookup of the waived increment against the step's
declared `[[step.increment]]` set, or a rework of how a waiver names its unit"
(`docs/plans/agent-scaffold.plan.toml:1895`). That sentence names exactly two
candidate fix shapes.

Q-69 does the analogous thing (naming concrete directions inside an `exploring`
item) but explicitly labels them as such: "CANDIDATE DIRECTIONS for the pass to
weigh, extend, or discard. NOT a decided option set, no recommendation attached"
(`docs/plans/agent-scaffold.plan.toml:1872`). Q-70 has no equivalent label on its two
named shapes, so a literal reading of "NO options" sits next to a sentence that
names two. I read this as a minor internal-consistency wrinkle rather than a
practical steer: the two shapes are offered only as illustrations of what the
coupling ruling controls, not dressed up with trade-offs or a recommendation, so a
careful reader is unlikely to mistake them for a decided pair. Recorded because the
"no options" claim is not, strictly, true of the whole item.

## The routed item: escape route 4 versus the second-waiver-path paragraph

Judgement (not a checkable fact): read in order, a fresh explorer would NOT be
misled by these two passages, though the second paragraph asks the reader to hold an
apparent tension for its length before resolving it.

Escape route 4 (`docs/plans/agent-scaffold.plan.toml:1887`) makes a claim scoped
specifically to W5: "W5's check is lexical on the token rather than a lookup against
the step's declared increments... never reads `step.increments`." It does not claim
that no code anywhere performs such a lookup.

"A SECOND WAIVER-VALIDATION PATH EXISTS" (`:1889`) opens with an explicit forward
link: "MEASURED AFTER ESCAPE ROUTE 4 WAS WRITTEN AND BEARING DIRECTLY ON IT," which
primes the reader that what follows qualifies rather than contradicts route 4. It
then describes a second, independent path (`src/plan/source.rs`) that does perform
exactly the lookup route 4 says W5 lacks, walks through a declared-vs-undeclared
fixture, and closes with an explicit reconciliation: "Escape route 4 is therefore
CONFIRMED BY MEASUREMENT rather than by reading, declaring the increment still not
admitting the waiver; what this item did not previously record is that double
lock." That sentence tells the reader plainly that route 4's conclusion holds and
names exactly what is new (the double-lock detail), so by the end of the paragraph
there is no live contradiction to resolve on one's own.

The cost to the reader is real but small: they must carry the two claims
("W5 lacks the lookup" and "a lookup exists on a different path") as compatible
rather than competing until the paragraph's own closing sentence says so explicitly.
That is a demand on attention, consistent with this document's dense style
throughout (not unique to this passage), rather than a passage that asserts
something a careful reader would later have to unlearn. I did not find wording in
either passage that a fresh reader would take to mean the opposite of what is true.

## Pre-decide check

Claim checked, not asserted as a defect: does Q-70 pre-decide the coupling
hypothesis or the naming collision it names as the pass's own ruling to make?

No. For the coupling hypothesis: "It is not pre-decided here and the pass must rule
on it explicitly" (`docs/plans/agent-scaffold.plan.toml:1895`). For the naming
collision: "nothing here pre-decides whether either check is renumbered, or which
one" (`:1897`). Both disclaimers are accurate against the surrounding text: neither
paragraph states or implies which way the ruling should go. (R1B-4 above notes that
the coupling paragraph names two candidate shapes, but naming what is at stake in a
ruling is not the same as pre-deciding it, and the item does not attach a
recommendation to either shape.)

## Severities not found

I found no `medium` or `critical` severity issues under this lens beyond the `high`
findings above. I looked for, and did not find, a case where Q-70 recommends,
ranks, or otherwise smuggles in an option (beyond R1B-4's minor wrinkle), and I did
not find a factual claim in the item that is false outright (the citations I spot-
checked, `src/workflow.rs:88,119,127,141,258,321,445,450,549,553,564` and
`src/plan/source.rs:785-843,791-793,807-811`, all matched the code at those lines).

## Findings file

`docs/plans/agent-scaffold.reviews/q70-capture-reviewer-consumer.md` (this file), in
worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-q70-consumer`.
