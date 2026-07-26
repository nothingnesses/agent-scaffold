# Triage: code-value-audit-static increment 2, review round 1

Adjudicated: Q-52 increment 2 (the suppression-marker + FFI source scan). Diff
`686d8ca..79f584b`. Intended scope: build-plan sections 4b, 5, 3, 7-inc2 (the scan +
`reclassify` hook + `run_audit` wiring); NO rustc harvest (inc3), NO cargo-machete / flake
(inc4). Read-only against the product; this triage file is the only write.

## Round outcome

Valid findings: 7 of 7 (every reviewer item stands on its evidence). Post-triage severities:
1 Medium (M2), 5 Low (M1 adjusted down from Medium, L1, N1, N2, N3), 1 nit (the
stale-comment note).

No Critical or High finding exists (both reviewers state so; the contract reviewer's ceiling
is Medium, the correctness reviewer's is Low). Therefore NO dismissal backstop is required.

Nothing here blocks the increment from landing: M2 is a required-coverage add (cheap,
in-scope), M1 is a stale help string, and the five design/robustness Lows are advisory with
no effect on the current tree. The one item I recommend fixing NOW for architecture-and-cost
reasons (not severity) is N2 (the `reclassify` join key), because inc3 is about to depend on
it; details below.

## Per-finding verdicts

### M1 (contract F1): stale `--dir` help text says the flag is not read -- VALID, ADJUST Medium -> Low

Confirmed at `src/main.rs:550`: the `AuditArgs::dir` doc-comment (the clap `--help` text)
still reads "The signal harvests will read it; this tier accepts it into the CLI contract but
does not yet read it." Increment 2 does read it: `run_audit` calls
`audit::scan_source(&args.dir)` (`src/main.rs:1260`), which walks `args.dir/src/**/*.rs`. The
diff updated the `run_audit` and inline comments (`src/main.rs:1248-1259`) but left line 550
stale.

Adjust to Low: the CLI still behaves correctly and produces no wrong output; the defect is a
misleading help string confined to `--help`, i.e. non-behavioral prose. It is genuinely
user-facing and worth fixing, but a stale help string is a Low contract-honesty defect, not
Medium. Fix: rewrite the doc-comment to say the source scan reads `src/**/*.rs` under this dir
now, and that the rustc / cargo-machete harvests are the later readers.

### M2 (contract F2): the `dead_code`-in-reason-string negative case is untested -- VALID (Medium upheld)

Confirmed. `lint_list_has_dead_code` (`src/audit.rs:498-504`) truncates `args` at the first
`"reason"` before the comma-split so that a `dead_code` occurring only inside the reason text
does not count; the `parse_suppression` doc affirmatively claims this
(`src/audit.rs:452-454`). No fixture exercises it: `marker_fixture`
(`src/audit.rs:965-989`) covers the `#[allow(unused)]` negative (`not_dead_code`) but has no
item whose ONLY `dead_code` is inside a reason string. I checked the removal claim: in every
current fixture `dead_code` precedes any `reason`, so the comma-split still finds `dead_code`
whether or not `args` is truncated; deleting the `args.find("reason")` guard fails no test.

Medium upheld: this is a doc that affirmatively asserts behavior with zero backing test (the
test-honesty defect the contract lens exists to catch), and the review brief named this exact
case as required coverage -- it is the one required fixture form that is missing. A regression
would emit a false-positive `DeclaredReason` fence. Fix: add one fixture item of the form
`#[allow(unused, reason = "replaces dead_code detection")]` and assert it produces no marker.

### L1 (contract F3): the balanced-parenthesis branch in `attr_args` is untested -- VALID (Low upheld)

Confirmed. `attr_args` (`src/audit.rs:474-493`) counts nested parens
(`b'(' => depth += 1`, line 482) so a reason string containing `(` does not close the arg list
early. The only fixture with an inner paren is `#[cfg_attr(not(test), allow(dead_code))]`, and
`attr_args` starts scanning AFTER the `allow(` keyword, so the `not(test)` paren sits before
the scanned region and never reaches the increment arm. Defensive code with no covering test;
no live misclassification today. Fix: add a fixture whose reason contains an inner paren, e.g.
`#[allow(dead_code, reason = "kept for foo(bar)")]`, and assert the reason and marker resolve.

### N1 (correctness F1): the scanner can over-report from block-comment bodies, raw strings, and commented-out attributes; the doc discloses only false negatives -- VALID (Low upheld)

All three constructs confirmed against `scan_file` / `is_comment`
(`src/audit.rs:383-448`):
- `is_comment` (446-448) matches only `//`, `/*`, `*`. A block-comment body line NOT led by
  `*` fails all three, is treated as an item line, and if it contains `extern "C"` records a
  spurious `Ffi` (symbol from the comment text).
- A raw string on an item line (`let s = r#"extern "C""#;`) contains the unescaped substring
  `extern "C"`, so the item-line probe at line 427 records a spurious `Ffi`. (An ordinary
  `"extern \"C\""` is safe: the inner quotes are escaped and do not match the
  `contains("extern \"C\"")` probe. Only raw strings defeat it.)
- A commented-out `#[allow(dead_code)]` on its own line: `is_comment` is checked first (line
  394) but the trimmed line starts with `#[`, not `//`/`/*`/`*`, so it is treated as a real
  attribute (`is_attr_line`, 397) and parsed as a suppression that fences the next item.

The module doc (`src/audit.rs:336-341`) frames the heuristic only as false NEGATIVES ("may be
missed"), so the over-report path is undisclosed. None of these constructs exist in the
current tree, so no live effect, but `--dir` is arbitrary. Resolution recommendation below.

### N2 (correctness F2): `reclassify` keyed on `(file, symbol)` collides when one file has two items with the same leading-identifier symbol -- VALID (Low upheld); recommend FIX NOW

Confirmed at `src/audit.rs:610-628`. `reclassify` matches any marker in the same file whose
`symbol` string equals the candidate's (line 617), where `symbol` is the leading-identifier
heuristic. Two items in one file that reduce to the same symbol (e.g. `fn new` in two `impl`
blocks) collide: a candidate for the UNmarked item gets excluded because a same-named marked
sibling exists -- a false negative in the tool's core direction (it hides a real candidate).
No live caller in inc2 (`reclassify` is `cfg_attr(not(test), allow(dead_code))`, wired in
inc3), so it lands safely. Low upheld. Resolution recommendation below.

### N3 (correctness F3): `collect_rs_files` follows symlinks, so a symlink cycle recurses to a stack-overflow abort -- VALID (Low upheld)

Confirmed at `src/audit.rs:369-370`: `if path.is_dir()` follows symlinks, and the recursion
has no visited set or depth bound, so a cycle under `src/` (e.g. `src/loop -> src`) recurses
unbounded to a stack-overflow abort rather than a clean `io::Error`. Pathological,
effectively self-inflicted input for a tool run against one's own crate; no real crate ships
such a cycle, so practical risk is low. Real, though: the walk is not cycle-safe and aborts
instead of failing loudly. Low upheld. Fix: skip symlinked directory entries during the walk
via `entry.file_type()?.is_symlink()` (do not follow symlinks), which removes the cycle by
construction. This also aligns with "Make illegal states unrepresentable" (the cycle becomes
unreachable) at a one-check cost.

### Correctness non-numbered note: `tests/audit_command.rs` comments describe increment-1 behaviour -- VALID (nit)

Confirmed concrete sites: the header (`tests/audit_command.rs:5-6`) says "Increment 1 emits
an EMPTY report", and the inline comments at lines 48 ("Increment 1 runs no signal") and 69
("empty in Increment 1") misdescribe the current run: `from_source_scan` always sets
`source_scan: true` (`src/audit.rs:83`), so the source scan DOES run; it simply finds nothing
because the scratch dirs have no `src/`. The assertions themselves remain correct (records is
`[]`, `rustc_dead_code` false). The file was not in the inc2 diff, but inc2's behavior change
is what staled these comments. Prose-only nit, no correctness impact. Fix (optional, cheap):
reword the three sites to "the scratch dir has no `src/`, so the source scan runs but finds
nothing," rather than "Increment 1 runs no signal."

## Design-flavored resolutions (by Project Principle, by name)

### N2 -- fix the `reclassify` key NOW, keying the join on `(file, item-line)`

Recommendation: FIX NOW, not defer. Add the annotated ITEM's line to `ScannedMarker` (the
scanner already has it: in `scan_file` the loop's `line` at the `extract_symbol` call
(`src/audit.rs:392,412`) is the item line, distinct from the stored attribute line) and key
`reclassify`'s join on `(file, item_line)`, carrying `symbol` only as a display label.

Why now, and why the item line:
- inc3 will consume `reclassify` to map rustc dead-code candidates to exclusions, and rustc
  reports each candidate by `file:line` of the ITEM (the `fn`/`struct` signature line), which
  is exactly the scanner's item line. Keying on `(file, item_line)` therefore both removes
  the collision (two distinct items have distinct lines) AND joins on the coordinate rustc
  actually emits, instead of re-deriving a symbol via a lossy leading-identifier heuristic
  that can also MISmatch rustc's own name. Principle: "Prefer the cleaner long-term
  architecture over the smallest diff" -- the join key should be rustc's real coordinate, not
  a heuristic proxy. Reinforced by "Make illegal states unrepresentable": distinct items have
  distinct lines, so the collision becomes structurally impossible rather than a tolerated
  runtime hazard.
- The reviewer called the key "forced" because the stored marker line is the ATTRIBUTE line
  while rustc reports the ITEM line. That is only forced under the CURRENT struct shape; the
  scanner has the item line in hand and can store it. The forcing dissolves once the struct
  carries the item line.
- Cost: it is cheaper now than after inc3. Today `reclassify` has no caller (it is
  `allow(dead_code)` until inc3 wires it), so changing the key is a contained edit (one
  struct field + swapping the match predicate + updating the two `reclassify` unit tests).
  Once inc3 builds its caller against the symbol key, changing it means reworking the caller
  too.
- The counterweight is "Minimal by default" -- accept the low-probability collision as an
  advisory residual and document it. I reject it here because the fix is small, it IMPROVES
  the inc3 join's correctness (exact-line beats symbol-heuristic), and inc3 imminently depends
  on this key, so P1 and P5 outweigh P2.

Note this fix ALSO removes N1's commented-out-attribute concern from the reclassify path only
partially (it does not, since a phantom fence still gets an item line); N1 is handled
separately below.

### N1 -- DISCLOSE the false-positive limitation (the smaller fix), do NOT harden the scanner now

Recommendation: the finding is valid, but resolve it with the SMALLER fix -- complete the
module doc's disclosure to include over-reporting, matching the existing false-negative
disclosure. Do NOT add comment/string skipping now. Principle: "Minimal by default", with
"Ground decisions in evidence" backing the defer.

Reasoning:
- The scan is by explicit design a heuristic line scan, not a parser: `scan_source`'s own doc
  says "a HEURISTIC line scan, not a syntax parse (Minimal by default: no `syn` dependency)"
  (`src/audit.rs:336-341`). The design already chose the heuristic over the lexer under
  Principle 2.
- A "cheap `//` and string skip" is not actually cheap-and-safe here. Doing it correctly (not
  stripping a `//` that sits inside a string, not misreading a `/*` inside a string,
  raw-string awareness, tracking block-comment depth across lines) is precisely the lexer work
  the design deliberately avoided; a naive partial version introduces its OWN false results.
  So the "cheap correctness fix" would drag the core toward a half-parser without reaching
  soundness -- the wrong trade under Principle 2.
- There is no live instance on the current tree (Principle 6: ground in evidence). The
  correctness exposure is latent, not real, so a one-line honest disclosure is the
  proportionate response, and it preserves the option to harden later behind a real observed
  over-report.
- Fix: extend the `scan_source` doc to add that the scan may also OVER-report -- a spurious
  FFI marker from a raw string or a block-comment body containing `extern "C"`, and a phantom
  suppression from an attribute commented out on its own line -- so the contract is honest in
  both directions.

Why the asymmetry with N2 (fix N2 now, defer N1): N2's fix REMOVES heuristic reliance (an
exact-line join replaces a lossy symbol match), is genuinely small, and has an imminent
dependent (inc3). N1's proper fix ADDS heuristic machinery toward the parser the design
rejected, a cheap partial version is unsafe, and nothing depends on it. The two land on
opposite sides of Principle 2 for principled reasons.

## Backstop

Not needed. No Critical or High finding exists (contract ceiling Medium, correctness ceiling
Low), so no dismissal-backstop justification is owed.
