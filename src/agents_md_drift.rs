//! Whole-file drift guard for scaffold files this repo dogfoods: the generated
//! `AGENTS.md`, its tool-owned copy `.agents/AGENTS.reference.md`, and the role prompts
//! the COVERAGE block at the end of this comment defines. Read that block before relying
//! on what is and is not guarded; it is the one place in this file that states coverage,
//! and the rest of the file cites it rather than restating it.
//!
//! The per-fragment guards (`isolation_policy.rs`, `workflow_spec.rs`) each pin ONE
//! generated slot inside the committed scaffold with a `.contains()` check. Nothing
//! asserted that the WHOLE committed file still corresponds to a fresh render of the
//! pack, so a hand edit, a dropped slot, or a stale pack source outside those two
//! fragments could go unnoticed. This module closes that gap: it re-renders the
//! built-in pack under the exact `just scaffold-self` config and compares the result
//! against the committed files.
//!
//! It is a NORMALIZED IN-TEST guard, decided so the test stays hermetic (it never
//! shells out to the formatter). The committed files are `prettier(build_assets
//! output)`: the self-scaffold pipeline runs the render and then `nix fmt`, whose
//! prettier owns Markdown wrapping with `proseWrap=never`. A byte compare of the raw
//! render against the committed file would therefore fail on formatter reflow alone,
//! not on real drift (this is why the per-fragment guards use `.contains()`, and it
//! composes with the Q-57 decision that incidental formatter reflow is not a
//! finding). So both sides are passed through `normalize_wrapping`, which collapses the
//! whitespace degrees of freedom prettier exercises (see its doc comment for the exact
//! transform and the argument that, under its stated precondition, it cannot mask a
//! content change) before the equality check.
//!
//! Empirically, at the time this guard was written the raw render is already
//! byte-identical to both committed files (the pack authors each paragraph on a single
//! line, so `proseWrap=never` is a no-op on them). The normalization is nonetheless
//! applied so the guard keeps passing on incidental reflow if a future pack edit
//! introduces wrapped prose, rather than turning a formatter reflow into a false
//! failure.
//!
//! COVERAGE. Stated once, here, and operationally. Coverage claims in this module drifted
//! from what the code does at enough separate sites that fixing one contradicted another,
//! so the guarded set is defined by naming the filter rather than a category, the
//! complement is a rule rather than a list, and the residuals are numbered for other
//! comments to cite. Write a coverage claim here or not at all.
//!
//! GUARDED SET. Three comparisons make up the drift coverage; the other tests in this
//! file exercise the helpers and add none.
//!
//! 1. The committed root `AGENTS.md` against its fresh render.
//! 2. The committed `.agents/AGENTS.reference.md` against its fresh render.
//! 3. For each rendered asset whose `dest` starts with `PROMPT_DEST_PREFIX`: that asset
//!    against the file committed at the same relative path under `CARGO_MANIFEST_DIR`.
//!
//! The render in each case is the pinned self-scaffold config (see `self_scaffold_assets`),
//! and both sides of each comparison go through `assert_no_unprotected_construct` and then
//! `normalize_wrapping`. Check 3 is a filter over a rendered set, not a directory listing
//! and not a hand-written list, which is what makes it self-extending: an `[[asset]]` row
//! added to `pack/pack.toml` whose `dest` falls under the prefix is guarded with no edit
//! here. Checks 1 and 2 embed their committed side with `include_str!` and check 3 cannot,
//! since that macro needs a literal path, so it reads the working tree at test time. That
//! is what the self-extension costs: less hermetic than a compile-time snapshot, which is
//! acceptable for a repo-local guard whose purpose is to inspect the working tree.
//!
//! COMPLEMENT, AS A RULE. Anything else the scaffold emits, or the repo commits, is
//! unguarded by this module. A rule rather than an inventory, because an inventory carries
//! an obligation to stay complete that prose reliably fails; the `.agents/user-prompts/`
//! copies, `.agents/LEDGER.template.md`, the `.toml` copies under `.agents/`, and the
//! `docs/plans/TEMPLATE` family illustrate the rule and do not bound it. Leaving them
//! uncovered is a scope call whose cost is uneven: widening to the Markdown copies is a
//! small change to check 3's filter, since they are prose under the same prettier settings
//! and already satisfy the precondition, while the TOML copies need a comparison of their
//! own, since `.agents/principles.toml` carries lines outside canonical whitespace form
//! (indented multi-line array continuations among them) that
//! `assert_no_unprotected_construct` rejects, and `normalize_wrapping` is a Markdown prose
//! transform with no business canonicalizing TOML.
//!
//! R1, THE DERIVED-SET RESIDUAL (accepted, not a defect to fix here). Check 3 maps render
//! -> committed and asserts nothing in the other direction, so a committed file under the
//! prefix that the pinned render does not emit is invisible to it. Reached by deleting an
//! asset row from `pack/pack.toml`, by module-tagging one (the pinned config selects no
//! modules, so a tagged row is not rendered), by changing a row's `dest`, or by hand-placing
//! a stale extra file in `.agents/prompts/`.
//! The mirror case is harmless, since an unregistered file never ships: a file sitting in
//! `pack/prompts/` with no manifest row is neither rendered nor guarded, and has no
//! committed copy that could go stale. The standing benign instance is
//! `.agents/prompts/checks-reviewer.md`, whose row is module-gated in `pack/pack.toml`, so
//! the pinned render omits it, check 3 omits it, and the repo commits no copy for it to
//! drift from; it needs no explicit exclusion and has none. The non-vacuity assertion in the
//! prompt test catches check 3 collapsing entirely (the filter matching nothing at all), not
//! a partial collapse.
//!
//! R2, THE CROSS-LINE JOIN RESIDUAL (accepted, not a defect to fix here).
//! `assert_no_unprotected_construct` inspects one line at a time and asserts nothing about
//! the join `normalize_wrapping` performs across lines. So a construct whose lines are each
//! in canonical whitespace form, but whose meaning depends on those lines NOT being joined,
//! passes the precondition and is joined anyway, which can mask a content change inside it.
//! A raw HTML block is the known instance: prettier keeps such a block verbatim rather than
//! reflowing it, and `is_hard_start` does not recognise it, so its lines are treated as
//! paragraph continuations. Any line-structured construct `is_hard_start` misses reaches the
//! same place, which is why that predicate's precision belongs to this residual rather than
//! to presentation. Accepted rather than fixed because the tightening that would close it
//! (treat an unrecognised line-structured block as verbatim) was implemented and measured to
//! reject an ordinary soft-wrapped paragraph, which is the incidental formatter reflow this
//! guard exists to tolerate; a fix has to preserve soft-wrap tolerance, not just add a
//! rejection. No guarded file carries such a construct today, so R2 is latent.
//!
//! End of COVERAGE. Comments past this point cite it and do not restate it.

#[cfg(test)]
mod tests {
	use {
		crate::{
			build_assets,
			manifest,
			pack,
			pack_principles,
		},
		std::collections::HashMap,
	};

	/// The committed root `AGENTS.md`, embedded so the guard reads exactly the
	/// scaffold output the repo ships (dogfooded from the pack).
	const COMMITTED_AGENTS: &str = include_str!("../AGENTS.md");

	/// The committed `.agents/AGENTS.reference.md`, the tool-owned reference copy of
	/// the same generated guidance.
	const COMMITTED_REFERENCE: &str = include_str!("../.agents/AGENTS.reference.md");

	/// The `dest` prefix check 3 filters the rendered asset set by. What that makes
	/// guarded, what the complement rule leaves out, and the R1 residual it carries are in
	/// the COVERAGE block of the module doc.
	const PROMPT_DEST_PREFIX: &str = ".agents/prompts/";

	/// Re-render the self-scaffold asset set. This replicates the exact
	/// `just scaffold-self` invocation (`scaffold --principles default --instrument`):
	/// the built-in pack, the default principle selection, the default `Summary` detail,
	/// no `--var` overrides, and no `--module` selections. Any divergence from that
	/// config would compare the committed files against the wrong render, so it is
	/// pinned here to match the justfile recipe. The absent `--module` selection is
	/// load-bearing for what check 3 covers; see R1 in COVERAGE.
	fn self_scaffold_assets() -> Vec<manifest::Asset> {
		let source = manifest::builtin();
		let principles = pack_principles(&source).expect("the built-in principles.toml parses");
		let selected = pack::resolve_selection(&principles, "default")
			.expect("the default principle selection resolves");
		build_assets(&source, &selected, pack::Detail::Summary, &HashMap::new(), true, &[])
			.expect("build_assets succeeds for the self-scaffold config")
	}

	/// The contents of the freshly rendered self-scaffold asset at `dest`.
	fn self_scaffold_asset(dest: &str) -> String {
		self_scaffold_assets()
			.into_iter()
			.find(|asset| asset.dest == dest)
			.unwrap_or_else(|| panic!("the self-scaffold render includes an asset at {dest}"))
			.contents
	}

	/// The committed copy of the scaffolded asset at `dest`, read from the crate root at
	/// test time. This is the less hermetic read the derived guarded set buys (see the
	/// module doc); `include_str!` cannot serve here because it needs a literal path. A
	/// missing file panics rather than being skipped, since a copy that disappeared while
	/// its asset row remains is drift the guard exists to catch. The reverse case, a
	/// committed file the render does not produce, is R1 and is not reached from here.
	fn committed_asset(dest: &str) -> String {
		let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(dest);
		std::fs::read_to_string(&path).unwrap_or_else(|error| {
			panic!(
				"failed to read the committed {dest} at {}: {error}. The self-scaffold render produces this file, so the repo must commit it; run `just scaffold-self`",
				path.display()
			)
		})
	}

	/// Assert the precondition that `normalize_wrapping`'s safety argument depends on (see
	/// its doc comment): each non-fenced line of the guarded text `content` is already in
	/// canonical whitespace form, so equal normalization still implies identical
	/// non-whitespace content and block structure. Without this, the flat files could one
	/// day gain a nested list, indented code, or a whitespace-significant inline span and
	/// the guard would pass while real drift slipped through. This converts that latent gap
	/// into a loud failure at the moment such a construct is added.
	///
	/// It is a per-line check, so it does not cover the class of constructs described by
	/// residual R2 in COVERAGE. Do not read a pass here as the precondition being fully
	/// established.
	///
	/// A line is in canonical form when it already equals
	/// `line.split_whitespace().collect::<Vec<_>>().join(" ")`, i.e. no leading or
	/// trailing whitespace and no internal whitespace run of any kind beyond a single
	/// ASCII space. That single predicate subsumes the older leading-whitespace and
	/// double-space checks and additionally rejects NON-SPACE whitespace (a mid-line
	/// tab, an NBSP, a form feed) that `split_whitespace` collapses just like a space
	/// run: any such run makes the line differ from its canonical form, so it trips.
	/// A leading space or tab catches nested or continuation-indented list items and
	/// 4-space indented code; a trailing or internal whitespace run catches multi-space
	/// (or tab/NBSP-separated) inline code and any intra-line whitespace run.
	///
	/// FENCED CODE IS EXEMPT. Fence state is tracked with the SAME rule
	/// `normalize_wrapping` uses (a line whose `trim_start()` begins with ``` or ~~~
	/// toggles the fence), and fenced-content lines plus the fence marker lines
	/// themselves are skipped, because `normalize_wrapping` emits them VERBATIM: an
	/// indented example inside a fence is not a construct it collapses, so asserting on
	/// it would false-fail a legitimate fenced example.
	fn assert_no_unprotected_construct(
		name: &str,
		content: &str,
	) {
		// Whether the cursor is inside a fenced code block, tracked with the same fence
		// rule normalize_wrapping uses so the two agree on which lines are verbatim.
		let mut in_fence = false;
		for (index, line) in content.lines().enumerate() {
			let number = index + 1;
			// A fence delimiter line toggles verbatim mode; it and the lines inside the
			// fence pass through normalize_wrapping unchanged, so they are exempt here.
			let trimmed_start = line.trim_start();
			if trimmed_start.starts_with("```") || trimmed_start.starts_with("~~~") {
				in_fence = !in_fence;
				continue;
			}
			if in_fence {
				continue;
			}
			let canonical = line.split_whitespace().collect::<Vec<_>>().join(" ");
			assert!(
				line == canonical,
				"{name} line {number} is not in canonical whitespace form (it has leading/trailing whitespace, or an internal whitespace run beyond a single ASCII space such as a double space, tab, or NBSP). The line is {line:?}; its canonical form is {canonical:?}. Guidance gained a whitespace-significant construct (a nested or continuation-indented list item, 4-space indented code, or a multi-space / tab / NBSP inline span) that normalize_wrapping would collapse (it trims and collapses all whitespace runs to a single space), so equal normalization would no longer imply equal content and this could mask real drift. Harden normalize_wrapping (make list indentation significant, treat indented code verbatim) before adding such content."
			);
		}
	}

	/// Whether `line` (already trimmed and space-collapsed) begins its own logical line
	/// rather than continuing the previous one: an ATX heading, an unordered or ordered
	/// list item, a blockquote, a table row, or a thematic break. Prettier keeps each of
	/// these on its own line and joins just a paragraph's or list item's wrapped
	/// CONTINUATION lines, so `normalize_wrapping` treats a hard-start line as a fresh
	/// logical line and joins the soft (continuation) lines onto it.
	///
	/// The set of markers this recognises is part of residual R2 in COVERAGE, not a
	/// presentation detail: a line-structured construct missing from the list above is
	/// classified soft and joined onto the preceding logical line, which is how R2 is
	/// reached. Widen the list before adding such a construct to a guarded file, and see
	/// R2 first for why widening it is not by itself a safe fix.
	fn is_hard_start(line: &str) -> bool {
		let bytes = line.as_bytes();
		// Heading (`#`), blockquote (`>`), or table row (`|`).
		if matches!(bytes.first(), Some(b'#' | b'>' | b'|')) {
			return true;
		}
		// Unordered list marker: `- `, `* `, or `+ ` (a marker char then a space).
		if matches!(bytes.first(), Some(b'-' | b'*' | b'+')) && bytes.get(1) == Some(&b' ') {
			return true;
		}
		// Ordered list marker: one or more digits, then `.` or `)`, then a space.
		let digits = bytes.iter().take_while(|b| b.is_ascii_digit()).count();
		if digits > 0
			&& matches!(bytes.get(digits), Some(b'.' | b')'))
			&& bytes.get(digits + 1) == Some(&b' ')
		{
			return true;
		}
		// Thematic break: three or more of the same marker (`-`, `*`, or `_`) and
		// nothing else once spaces are removed.
		if line.len() >= 3 {
			for marker in *b"-*_" {
				if bytes.iter().all(|&b| b == marker) {
					return true;
				}
			}
		}
		false
	}

	/// Move the pending logical line, if any, into the output.
	fn flush(
		pending: &mut Option<String>,
		out: &mut Vec<String>,
	) {
		if let Some(line) = pending.take() {
			out.push(line);
		}
	}

	/// Canonicalize the wrapping/whitespace degrees of freedom prettier exercises under
	/// `proseWrap=never`, so a formatter reflow of the render does not read as drift, while
	/// a real content change survives.
	///
	/// EXACT transform. Outside fenced code blocks, the input is grouped into blocks
	/// separated by blank lines. Within a block, each hard-start line (see `is_hard_start`:
	/// heading, list item, blockquote, table row, thematic break) starts a new logical line,
	/// and each following soft (continuation) line is joined onto it with a single space;
	/// runs of inter-word whitespace collapse to a single space and leading/trailing
	/// whitespace is trimmed. Runs of blank lines collapse to a single block boundary, and a
	/// trailing boundary is dropped so a differing final-newline count does not register.
	/// Lines inside a fenced code block (delimited by ``` or ~~~) pass through VERBATIM,
	/// since prettier never reflows code and whitespace there is significant.
	///
	/// WHY IT PRESERVES A CONTENT CHANGE, AND THE PRECONDITION THAT BUYS THAT. The guarantee
	/// is conditional. It holds while the guarded text carries no indentation-significant
	/// construct (a nested or continuation-indented list item, a 4-space indented code block)
	/// and no whitespace-significant inline construct (a run of two or more spaces, including
	/// inside an inline code span), because trimming and collapsing whitespace discards
	/// exactly what those encode: the transform cannot tell `  - child` from `- child`, a
	/// list-continuation de-indent from a new line, indented code from a plain line, or a
	/// multi-space inline span from a single-spaced one. Given the precondition, the
	/// transform deletes and collapses whitespace between non-whitespace tokens and does
	/// nothing further: it never deletes, adds, or reorders a non-whitespace character, it
	/// preserves blank-line block boundaries (collapsed, not removed), and it passes fenced
	/// lines through verbatim. Two inputs then normalize equal only when they carry the same
	/// ordered stream of non-whitespace characters, the same block-boundary structure up to
	/// blank-run collapsing, and byte-identical fences. Real drift (a reworded, added,
	/// dropped, or reordered word, list item, or slot) changes that token stream; merging or
	/// splitting a paragraph changes the boundary count; editing a fence changes a verbatim
	/// line. What the comparison discards on both sides: where a line is wrapped, how many
	/// spaces sit between words, how many blank lines separate blocks, and line-ending style,
	/// since `str::lines()` strips a trailing CR so a CRLF and an LF file normalize alike.
	///
	/// `assert_no_unprotected_construct` asserts that precondition on both sides of each
	/// comparison and fails loudly the day guidance gains one of those constructs, so the
	/// gap is a fail-safe rather than a silent hole. The class it cannot see is residual R2
	/// in COVERAGE. Harden this transform (make list indentation significant, treat indented
	/// code and HTML blocks verbatim, without losing the soft-wrap tolerance R2 explains)
	/// before adding such content to a guarded file.
	fn normalize_wrapping(input: &str) -> String {
		let mut out: Vec<String> = Vec::new();
		// The logical line being accumulated (a paragraph or a list item plus its
		// wrapped continuation lines), or None between blocks.
		let mut pending: Option<String> = None;
		// Whether the cursor is inside a fenced code block, whose lines pass verbatim.
		let mut in_fence = false;

		for raw_line in input.lines() {
			let trimmed_start = raw_line.trim_start();
			// A fence delimiter toggles verbatim mode. The delimiter line and the lines
			// inside the fence are emitted exactly as written, so a real whitespace change
			// inside code is caught, not masked.
			if trimmed_start.starts_with("```") || trimmed_start.starts_with("~~~") {
				flush(&mut pending, &mut out);
				out.push(raw_line.to_string());
				in_fence = !in_fence;
				continue;
			}
			if in_fence {
				out.push(raw_line.to_string());
				continue;
			}

			let trimmed = raw_line.trim();
			if trimmed.is_empty() {
				// A blank line ends the current block. Consecutive blanks collapse to one
				// boundary, recorded just when the last emitted item is not already one.
				flush(&mut pending, &mut out);
				if out.last().is_some_and(|line| !line.is_empty()) {
					out.push(String::new());
				}
				continue;
			}

			// Collapse inter-word whitespace runs to single spaces.
			let collapsed = trimmed.split_whitespace().collect::<Vec<_>>().join(" ");

			if is_hard_start(&collapsed) {
				// A structural line begins its own logical line.
				flush(&mut pending, &mut out);
				pending = Some(collapsed);
			} else if let Some(current) = pending.as_mut() {
				// A continuation of the current paragraph or list item: undo the soft wrap.
				current.push(' ');
				current.push_str(&collapsed);
			} else {
				pending = Some(collapsed);
			}
		}
		flush(&mut pending, &mut out);
		// Drop a trailing boundary so a differing final-newline count does not register.
		while out.last().is_some_and(String::is_empty) {
			out.pop();
		}
		out.join("\n")
	}

	#[test]
	fn the_committed_scaffold_matches_a_fresh_render() {
		// Checks 1 and 2 of COVERAGE, the drift guard on the PACK generation path: the
		// committed root `AGENTS.md` and its reference copy must match a fresh render of
		// the built-in pack under the self-scaffold config, once prettier's
		// wrapping/whitespace is normalized away on both sides. This fails on a real
		// content drift, a hand edit, a dropped slot, or a stale pack source that the
		// per-fragment guards do not cover, while tolerating an incidental formatter
		// reflow. The fix is `just scaffold-self`.
		let rendered_agents = self_scaffold_asset("AGENTS.md");
		let rendered_reference = self_scaffold_asset(".agents/AGENTS.reference.md");

		// Precondition for normalize_wrapping's safety argument (see its doc comment):
		// both the fresh render and the committed copy must be free of any
		// indentation- or whitespace-significant construct, or equal normalization
		// would no longer imply equal content and the equality checks below could pass
		// on masked drift. Asserted on both sides.
		assert_no_unprotected_construct("committed AGENTS.md", COMMITTED_AGENTS);
		assert_no_unprotected_construct("rendered AGENTS.md", &rendered_agents);
		assert_no_unprotected_construct(
			"committed .agents/AGENTS.reference.md",
			COMMITTED_REFERENCE,
		);
		assert_no_unprotected_construct(
			"rendered .agents/AGENTS.reference.md",
			&rendered_reference,
		);

		assert_eq!(
			normalize_wrapping(&rendered_agents),
			normalize_wrapping(COMMITTED_AGENTS),
			"root AGENTS.md has drifted from a fresh pack render (ignoring prettier wrapping); run `just scaffold-self`"
		);
		assert_eq!(
			normalize_wrapping(&rendered_reference),
			normalize_wrapping(COMMITTED_REFERENCE),
			".agents/AGENTS.reference.md has drifted from a fresh pack render (ignoring prettier wrapping); run `just scaffold-self`"
		);
	}

	#[test]
	fn the_committed_role_prompts_match_a_fresh_render() {
		// Check 3 of COVERAGE. Before it, the prompt copies appeared in the test suite as
		// destination strings in the manifest's expected-asset list, which asserts what the
		// scaffold EMITS and never that the committed copy still matches it, so editing a
		// `pack/prompts/<role>.md` and forgetting to regenerate shipped a stale prompt with
		// the suite green.
		//
		// Two-way in CONTENT: because it compares a fresh render against the committed
		// bytes, a pack edit with a stale copy and a hand edit of the copy with the pack
		// left alone both fail, and the fix in either direction is to edit the pack source
		// and run `just scaffold-self`. One-way in SET MEMBERSHIP, which is residual R1.
		let prompts: Vec<_> = self_scaffold_assets()
			.into_iter()
			.filter(|asset| asset.dest.starts_with(PROMPT_DEST_PREFIX))
			.collect();

		// A derived set can go vacuously empty (the prompts move, the manifest renames the
		// destination directory) and a guard over an empty set passes silently, which is
		// worse than no guard because it reads as coverage. Pin that the filter matched.
		assert!(
			!prompts.is_empty(),
			"the self-scaffold render dropped no asset under {PROMPT_DEST_PREFIX}, so this guard checked nothing; if the role prompts moved, point PROMPT_DEST_PREFIX at their new destination"
		);

		for asset in prompts {
			let dest = asset.dest.as_str();
			let committed = committed_asset(dest);

			// The precondition normalize_wrapping's safety argument depends on, asserted on
			// both sides for the same reason as above: without it a prompt could gain a
			// nested list, indented code, or a multi-space inline span and the equality
			// check below would keep passing over masked drift.
			assert_no_unprotected_construct(&format!("committed {dest}"), &committed);
			assert_no_unprotected_construct(&format!("rendered {dest}"), &asset.contents);

			assert_eq!(
				normalize_wrapping(&asset.contents),
				normalize_wrapping(&committed),
				"{dest} has drifted from a fresh render of the pack's prompts (ignoring prettier wrapping): either its `pack/prompts/` source was edited without regenerating, or the committed copy was hand edited. Edit the pack source, not the copy, then run `just scaffold-self`"
			);
		}
	}

	#[test]
	fn normalization_tolerates_wrapping_but_not_content_change() {
		// This pins the load-bearing property the whole-file guard relies on: the
		// normalization discards prettier's reflow but preserves a content change. The
		// real committed files cannot exercise it (the render is already byte-identical
		// to them), so a constructed pair stands in as the guard's own proof-of-concept.
		let canonical = "# Title\n\nThe quick brown fox jumps over the lazy dog.\n\n- first item\n- second item\n";

		// A soft wrap inside the paragraph normalizes away.
		let wrapped = "# Title\n\nThe quick brown fox\njumps over the lazy dog.\n\n- first item\n- second item\n";
		assert_eq!(
			normalize_wrapping(canonical),
			normalize_wrapping(wrapped),
			"an intra-paragraph soft wrap must normalize away"
		);

		// Collapsed inter-word spaces and blank-line runs normalize away too.
		let respaced = "# Title\n\n\nThe quick   brown fox jumps over the lazy dog.\n\n- first item\n- second item";
		assert_eq!(
			normalize_wrapping(canonical),
			normalize_wrapping(respaced),
			"collapsed spaces and blank-line runs must normalize away"
		);

		// A dropped word is real drift and must survive.
		let dropped_word =
			"# Title\n\nThe quick fox jumps over the lazy dog.\n\n- first item\n- second item\n";
		assert_ne!(
			normalize_wrapping(canonical),
			normalize_wrapping(dropped_word),
			"a dropped word must not normalize away"
		);

		// A dropped list item is real drift and must survive.
		let dropped_item =
			"# Title\n\nThe quick brown fox jumps over the lazy dog.\n\n- first item\n";
		assert_ne!(
			normalize_wrapping(canonical),
			normalize_wrapping(dropped_item),
			"a dropped list item must not normalize away"
		);

		// Merging two paragraphs (removing a block boundary) is real drift and must
		// survive, since block boundaries are preserved.
		let merged_blocks = "# Title\nThe quick brown fox jumps over the lazy dog.\n\n- first item\n- second item\n";
		assert_ne!(
			normalize_wrapping(canonical),
			normalize_wrapping(merged_blocks),
			"removing a block boundary must not normalize away"
		);
	}

	/// Run `assert_no_unprotected_construct` on `content` and report whether it
	/// panicked (rejected the content), so a regression test can assert acceptance or
	/// rejection without unwinding the whole test binary. The panic hook is silenced
	/// around the intended panic so a rejected fixture does not print a backtrace on an
	/// otherwise-passing run; it is restored before returning.
	fn precondition_rejects(content: &str) -> bool {
		let previous_hook = std::panic::take_hook();
		std::panic::set_hook(Box::new(|_| {}));
		let rejected =
			std::panic::catch_unwind(|| assert_no_unprotected_construct("fixture", content))
				.is_err();
		std::panic::set_hook(previous_hook);
		rejected
	}

	#[test]
	fn precondition_rejects_non_space_whitespace_and_round_one_cases() {
		// F1: a mid-line tab inside a non-fenced line normalizes equal to a single-space
		// version (split_whitespace collapses it), so it must be rejected. Before the fix
		// the `contains("  ")` check missed this non-space whitespace.
		assert!(
			precondition_rejects("text with a\ttab inside"),
			"a mid-line tab must be rejected (F1)"
		);
		// An NBSP (U+00A0) is likewise collapsed by split_whitespace and must be rejected.
		assert!(
			precondition_rejects("text with a\u{00a0}nbsp inside"),
			"a mid-line NBSP must be rejected (F1)"
		);
		// Round-1 cases must still reject: a nested list item (leading indentation) and a
		// double space.
		assert!(
			precondition_rejects("- parent\n  - child"),
			"a nested (leading-indented) list item must still be rejected"
		);
		assert!(
			precondition_rejects("a paragraph with  two spaces"),
			"a double space must still be rejected"
		);
	}

	#[test]
	fn precondition_exempts_fenced_indented_lines_but_not_bare_ones() {
		// F2: an indented example line INSIDE a ``` fence passes normalize_wrapping
		// verbatim, so the precondition must not trip on it.
		assert!(
			!precondition_rejects("# Title\n\n```\n    let indented = example;\n```\n"),
			"an indented line inside a fence must be accepted (F2)"
		);
		// The SAME indented line OUTSIDE a fence is an unprotected indentation-significant
		// construct and must be rejected.
		assert!(
			precondition_rejects("# Title\n\n    let indented = example;\n"),
			"the same indented line outside a fence must be rejected"
		);
	}
}
