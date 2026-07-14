# agent-isolation review (Sonnet reviewer)

Diff range: 57739c3..032964a

## S1 - medium: "Structural upgrade" relationship stated twice, in two sections that can drift

**Location:** `pack/AGENTS.md`, file-safety section (final prose sentence, before the rule list)
and `pack/AGENTS.md`, new writer-isolation section (second paragraph, first sentence).

**Evidence:**

File-safety section:
> "running writers under isolation is a structural upgrade layered on top of these
> rules, not a replacement for them."

New isolation section:
> "Isolation is the structural upgrade over the file-safety baseline: a killed or
> misbehaving isolated writer cannot touch the main tree, so its damage is contained
> rather than only recoverable after the fact."

Both sentences assert the same relationship: isolation is the structural upgrade layered on
file-safety. The second sentence adds useful rationale ("cannot touch the main tree...") but
leads with an identical claim. The review criteria asks explicitly whether this relationship
is "stated coherently and once, or redundantly in two places that could drift." This is the
redundant case.

If the first sentence's phrasing evolves (say "structural upgrade" becomes "structural
complement" or the layering metaphor changes), the two sentences diverge silently. Principle 2
(minimal by default) and the coherence goal of Principle 1 both point to stating it once and
referencing it from the other. The natural home is the isolation section (which owns the claim)
with the file-safety section pointing forward to it ("see the isolation rule below") rather
than pre-claiming the same point. Alternatively, the file-safety sentence could drop the
"structural upgrade" framing entirely and say only "not a replacement," leaving the upgrade
claim to the isolation section where it belongs.

## S2 - low: Isolation section re-enumerates the writer/read-only role classification instead of deferring to the Roles section's existing definition

**Location:** `pack/AGENTS.md`, Roles section (the paragraph beginning "Among the spawned
roles...") vs. the new writer-isolation section (opening sentence and read-only carve-out).

**Evidence:**

Roles section (unchanged, already present):
> "Among the spawned roles, the planner and the implementer are writers (they change the
> plan or the code) and the reviewers and the triager are read-only... 'Writer agent' below
> means a spawned writer role."

New isolation section:
> "Run each writer agent (the planner and the implementer) in the strongest isolation..."
> "Read-only agents (the reviewers and the triager) need no isolation..."

The Roles section already defines "writer agent" and names its members; it defines "read-only"
and names those members too. The isolation section re-enumerates both via parentheticals
instead of using the already-defined terms bare. The `agent-isolation` step detail in
`docs/plans/agent-scaffold.md` notes that the Roles section "defines 'writer agent', which
`agent-isolation` reuses for its read-only carve-out" - the intended pattern is reuse, not
re-enumeration.

This is a low-severity drift surface (Principle 2): if a new writer role is added or an
existing one renamed, both the Roles section and the parentheticals in the isolation section
need updating. The parentheticals could be dropped without loss since the terms are defined
earlier in the same document. This is not wrong as written, but it departs from the plan's
stated intent.
