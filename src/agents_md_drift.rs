//! Whole-file drift guard for the generated `AGENTS.md` and its tool-owned copy
//! `.agents/AGENTS.reference.md`.
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
//! finding). So both sides are passed through `normalize_wrapping`, which collapses
//! ONLY the whitespace degrees of freedom prettier exercises (see its doc comment for
//! the exact transform and the argument that it cannot mask a content change) before
//! the equality check.
//!
//! Empirically, at the time this guard was written the raw render is already
//! byte-identical to both committed files (the pack authors each paragraph on a single
//! line, so `proseWrap=never` is a no-op on them). The normalization is nonetheless
//! applied so the guard keeps passing on incidental reflow if a future pack edit
//! introduces wrapped prose, rather than turning a formatter reflow into a false
//! failure.

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

	/// Re-render the self-scaffold asset set and return the contents of the asset at
	/// `dest`. This replicates the exact `just scaffold-self` invocation
	/// (`scaffold --principles default --instrument`): the built-in pack, the default
	/// principle selection, the default `Summary` detail, no `--var` overrides, and no
	/// `--module` selections. Any divergence from that config would compare the
	/// committed files against the wrong render, so it is pinned here to match the
	/// justfile recipe.
	fn self_scaffold_asset(dest: &str) -> String {
		let source = manifest::builtin();
		let principles = pack_principles(&source).expect("the built-in principles.toml parses");
		let selected = pack::resolve_selection(&principles, "default")
			.expect("the default principle selection resolves");
		let assets = build_assets(&source, &selected, pack::Detail::Summary, &HashMap::new(), true, &[])
			.expect("build_assets succeeds for the self-scaffold config");
		assets
			.into_iter()
			.find(|asset| asset.dest == dest)
			.unwrap_or_else(|| panic!("the self-scaffold render includes an asset at {dest}"))
			.contents
	}

	/// Whether `line` (already trimmed and space-collapsed) begins its own logical
	/// line rather than continuing the previous one: an ATX heading, an unordered or
	/// ordered list item, a blockquote, a table row, or a thematic break. Prettier
	/// keeps each of these on its own line and only joins a paragraph's or list item's
	/// wrapped CONTINUATION lines, so `normalize_wrapping` treats a hard-start line as
	/// a fresh logical line and joins only the soft (continuation) lines onto it.
	///
	/// Precision here affects only how closely the canonical form mirrors prettier and
	/// how readable a failure diff is; it does NOT affect correctness. The transform
	/// only ever deletes or collapses whitespace and is applied identically to both
	/// sides, so misclassifying a structural line can at most change a newline into a
	/// space (or vice versa) on both sides equally; it can never merge two distinct
	/// non-whitespace tokens into one.
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

	/// Canonicalize the wrapping/whitespace degrees of freedom prettier exercises
	/// under `proseWrap=never`, so a formatter reflow of the render does not read as
	/// drift, while every real content change survives.
	///
	/// EXACT transform. Outside fenced code blocks, the input is grouped into blocks
	/// separated by blank lines. Within a block, each hard-start line (see
	/// `is_hard_start`: heading, list item, blockquote, table row, thematic break)
	/// starts a new logical line, and every following soft (continuation) line is
	/// joined onto it with a single space; runs of inter-word whitespace collapse to a
	/// single space and leading/trailing whitespace is trimmed. Runs of blank lines
	/// collapse to a single block boundary, and a trailing boundary is dropped so a
	/// differing final-newline count does not register. Lines inside a fenced code
	/// block (delimited by ``` or ~~~) pass through VERBATIM, since prettier never
	/// reflows code and whitespace there is significant.
	///
	/// WHY it cannot mask a content change. The transform only ever deletes or
	/// collapses WHITESPACE (spaces, tabs, the newlines joined within a block, and
	/// runs of blank lines); it never deletes, adds, or reorders a non-whitespace
	/// character, and it preserves blank-line block boundaries (collapsed, not
	/// removed) and every fenced code line verbatim. So two inputs normalize equal
	/// only when they carry the identical ordered stream of non-whitespace characters,
	/// the identical block-boundary structure up to blank-run collapsing, and
	/// byte-identical code fences. Any real drift, a reworded, added, dropped, or
	/// reordered word or list item or slot, changes that token stream; merging or
	/// splitting a paragraph changes the block-boundary count; editing a code fence
	/// changes a verbatim line. Only prettier's own freedoms, where a line is wrapped,
	/// how many spaces sit between words, how many blank lines separate blocks, are
	/// discarded. A reviewer trying to construct a hidden drift must therefore change
	/// only whitespace, which is by definition the reflow Q-57 rules out as a finding.
	fn normalize_wrapping(input: &str) -> String {
		let mut out: Vec<String> = Vec::new();
		// The logical line being accumulated (a paragraph or a list item plus its
		// wrapped continuation lines), or None between blocks.
		let mut pending: Option<String> = None;
		// Whether the cursor is inside a fenced code block, whose lines pass verbatim.
		let mut in_fence = false;

		for raw_line in input.lines() {
			let trimmed_start = raw_line.trim_start();
			// A fence delimiter toggles verbatim mode. The delimiter line and every line
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
				// boundary, recorded only when the last emitted item is not already one.
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
		// Whole-file drift guard on the PACK generation path: the committed root
		// `AGENTS.md` and its reference copy must match a fresh render of the built-in
		// pack under the self-scaffold config, once prettier's wrapping/whitespace is
		// normalized away on both sides. This fails on any real content drift, a hand
		// edit, a dropped slot, or a stale pack source, that the per-fragment guards do
		// not cover, while tolerating an incidental formatter reflow. The fix is
		// `just scaffold-self`.
		assert_eq!(
			normalize_wrapping(&self_scaffold_asset("AGENTS.md")),
			normalize_wrapping(COMMITTED_AGENTS),
			"root AGENTS.md has drifted from a fresh pack render (ignoring prettier wrapping); run `just scaffold-self`"
		);
		assert_eq!(
			normalize_wrapping(&self_scaffold_asset(".agents/AGENTS.reference.md")),
			normalize_wrapping(COMMITTED_REFERENCE),
			".agents/AGENTS.reference.md has drifted from a fresh pack render (ignoring prettier wrapping); run `just scaffold-self`"
		);
	}

	#[test]
	fn normalization_tolerates_wrapping_but_not_content_change() {
		// This pins the load-bearing property the whole-file guard relies on: the
		// normalization discards prettier's reflow but preserves every content change.
		// The real committed files cannot exercise it (the render is already
		// byte-identical to them), so a constructed pair stands in as the guard's own
		// proof-of-concept.
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
		let dropped_word = "# Title\n\nThe quick fox jumps over the lazy dog.\n\n- first item\n- second item\n";
		assert_ne!(
			normalize_wrapping(canonical),
			normalize_wrapping(dropped_word),
			"a dropped word must not normalize away"
		);

		// A dropped list item is real drift and must survive.
		let dropped_item = "# Title\n\nThe quick brown fox jumps over the lazy dog.\n\n- first item\n";
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
}
