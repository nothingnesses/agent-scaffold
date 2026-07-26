//! The advisory `agent-scaffold audit` code-value report (Tier-0, Q-52).
//!
//! `audit` is read-mostly and advisory: it builds a machine-readable intermediate of
//! code that MAY not be earning its keep (dead-code and unused-dependency suspicions,
//! plus author-declared suppression reasons that are NOT candidates), then EITHER
//! serialises that typed value to JSON on stdout (`--json`) or projects it to a kept
//! `docs/plans/<task>.code-value-report.md`. It never edits `src/`, `Cargo.toml`, the
//! plan TOML, the sidecars, or the metrics log; it writes only its own report and never
//! deletes anything. A human reads the report and decides each candidate.
//!
//! This module is the schema plus the projection. Increment 1 (Q-52) ships the typed
//! `CodeValueReport`, its Markdown projection, the single-sourced mandatory caveat, and
//! an EMPTY report (no signal ran yet). The signal producers (the rustc dead-code
//! harvest, the `#[allow/expect(dead_code)]` / FFI source scan, and `cargo-machete`) are
//! later increments; they only add record producers, because the projection here is
//! already total over every `AuditRecord` kind.
//!
//! The "verdict" (a review-candidate versus a shown-not-candidate row) is DERIVED at
//! projection time, never stored: a `DeadCode` or `UnusedDep` with no exclusion is a
//! candidate, and any excluded `DeadCode` or any `DeclaredReason` is shown but not a
//! candidate. This mirrors how `status` and `next` are best-effort projections of
//! durable files rather than a source of truth.

use {
	serde::Serialize,
	std::{
		fs,
		io,
		path::{
			Path,
			PathBuf,
		},
	},
};

/// The single-sourced mandatory caveat: the "not evidence of absence" text carried both
/// in the JSON intermediate (`CodeValueReport::caveat`) and at the head of the Markdown,
/// so the two can never drift (one `const`, read in both places). It states the oracle
/// set explicitly (Ground decisions in evidence): a passing audit is only ever necessary,
/// never sufficient.
pub(crate) const AUDIT_CAVEAT: &str = "This report is advisory. \"Nothing flagged\" is necessary but not sufficient, and is only relative to the named signal set (rustc dead-code under this project's lint configuration, source suppression markers, and cargo-machete's source-grep heuristic). Suppressed, cfg-gated (non-analysed targets), FFI, dynamically dispatched, and reflection-reached code is not covered. A passing audit is not proof the codebase has no dead code.";

/// The canonical per-signal human labels, single-sourced (the same discipline `AUDIT_CAVEAT`
/// uses) so the signal-set disclosure line and each row's provenance name the same oracle
/// with the same spelling and cannot drift as later increments touch one but not the other.
/// The `SignalSet` disclosure, `DeadCodeSource`'s per-row label, and the constant label the
/// `UnusedDep` projection uses all read from these three (Structured data first, project for
/// humans: one source, projected in both places).
const LABEL_RUSTC_DEAD_CODE: &str = "rustc dead-code";
const LABEL_SOURCE_SCAN: &str = "source scan";
const LABEL_CARGO_MACHETE: &str = "cargo-machete";

/// The whole code-value report: the typed intermediate `--json` serialises and the
/// Markdown projection reads. `generated_from` records which signals actually ran, so an
/// absent signal WIDENS the caveat's "not covered" disclosure rather than silently passing.
/// The verdict is not stored here; it is derived by the projection.
#[derive(Debug, Serialize)]
pub(crate) struct CodeValueReport {
	/// The task slug (the `<task>` in `<task>.plan.toml`), for the report head.
	pub(crate) task: String,
	/// Which signals actually produced records for this run, for an accurate caveat.
	pub(crate) generated_from: SignalSet,
	/// The mandatory caveat (always `AUDIT_CAVEAT`), so the JSON carries it too.
	pub(crate) caveat: &'static str,
	/// The audit rows, one per suspicion or declared reason. Empty in Increment 1.
	pub(crate) records: Vec<AuditRecord>,
}

impl CodeValueReport {
	/// Build the Increment 2 report for `task`: the source scan ran (so `source_scan` is
	/// flagged) and its author-declared-reason `records` populate the report. The rustc and
	/// cargo-machete harvests (increments 3-4) have not run, so their signals stay `false` and
	/// widen the caveat's "not covered" disclosure. The schema and projection do not change;
	/// later increments only add record producers.
	pub(crate) fn from_source_scan(
		task: String,
		records: Vec<AuditRecord>,
	) -> Self {
		CodeValueReport {
			task,
			generated_from: SignalSet {
				rustc_dead_code: false,
				source_scan: true,
				cargo_machete: false,
			},
			caveat: AUDIT_CAVEAT,
			records,
		}
	}
}

/// Which of the Tier-0 signals actually ran for a report. A `false` signal did NOT run,
/// so its coverage is absent and the projection says so (an absent signal widens the
/// caveat rather than reading as a clean pass). Named booleans rather than a free `Vec` of signal flags
/// so the "ran vs not run" state is explicit and cannot carry a duplicate.
#[derive(Debug, Serialize)]
pub(crate) struct SignalSet {
	/// The rustc `dead_code` / `unused_*` harvest (`cargo check --message-format=json`).
	pub(crate) rustc_dead_code: bool,
	/// The source scan for `#[allow/expect(dead_code)]` markers and FFI attributes.
	pub(crate) source_scan: bool,
	/// The `cargo-machete` unused-dependency harvest.
	pub(crate) cargo_machete: bool,
}

impl SignalSet {
	/// The human labels of the signals that ran, in a fixed order.
	fn ran(&self) -> Vec<&'static str> {
		self.each().into_iter().filter(|(ran, _)| *ran).map(|(_, label)| label).collect()
	}

	/// The human labels of the signals that did NOT run, in a fixed order.
	fn not_run(&self) -> Vec<&'static str> {
		self.each().into_iter().filter(|(ran, _)| !*ran).map(|(_, label)| label).collect()
	}

	/// Each signal paired with its ran flag, in a fixed order, so `ran`/`not_run` cannot
	/// disagree on the set or its order.
	fn each(&self) -> [(bool, &'static str); 3] {
		[
			(self.rustc_dead_code, LABEL_RUSTC_DEAD_CODE),
			(self.source_scan, LABEL_SOURCE_SCAN),
			(self.cargo_machete, LABEL_CARGO_MACHETE),
		]
	}
}

/// One audit row. The kind is an enum whose variants carry ONLY their own evidence, so an
/// illegal combination (a dependency row with a symbol span, a dead-code row with a
/// machete caveat) cannot be represented. Provenance is also constrained per variant (Make
/// illegal states unrepresentable): a `DeadCode` row's `source` is one of its two real
/// oracles (`DeadCodeSource`, never machete), and an `UnusedDep` row carries no `source`
/// field at all because only cargo-machete ever produces one. Serialised with a `"kind"`
/// discriminator.
#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
// `DeadCode` and `UnusedDep` are constructed only under `cfg(test)` until the rustc and
// cargo-machete harvests (increments 3-4) land; the increment-2 source scan already
// constructs `DeclaredReason` in the release build. This is the cfg-split form (as
// `LoopState::Done` at `src/next.rs:218`), where `allow` is correct rather than `expect`, and
// it covers the two variants that still have no release producer.
#[cfg_attr(
	not(test),
	allow(
		dead_code,
		reason = "DeadCode and UnusedDep are constructed by the rustc and cargo-machete harvests (increments 3-4) and the tests; only DeclaredReason has a release producer (the increment-2 source scan) so far"
	)
)]
pub(crate) enum AuditRecord {
	/// A rustc `dead_code` / `unused_*` suspicion (never a verdict). `exclusion` is
	/// `Some(..)` when the item is shown but not a candidate (a suppression, cfg-gate, FFI,
	/// or contract-surface reason keeps it out of the candidate set before a human sees it).
	DeadCode {
		/// The evidence anchor: the item's `file:line`.
		span: Span,
		/// The item name from the diagnostic.
		symbol: String,
		/// The lint that flagged it (for example `dead_code`, `unused_variables`).
		lint: String,
		/// Which of the two dead-code oracles produced this row (never machete).
		source: DeadCodeSource,
		/// `Some(..)` reclassifies the row as shown-but-not-a-candidate, with its reason.
		exclusion: Option<Exclusion>,
	},
	/// A `cargo-machete` unused-dependency suspicion. Carries machete's own imprecision
	/// note per row (a dep used only via a macro or a re-export can be a false positive),
	/// so the row is never auto-trusted.
	UnusedDep {
		/// The dependency's crate name.
		crate_name: String,
		/// The `Cargo.toml:line` of the dependency entry.
		manifest: Span,
		/// Machete's per-row imprecision note.
		caveat: &'static str,
	},
	/// An author-declared reason: a suppressed item shown as a fence, explicitly NOT a
	/// candidate. The author already declared why the item is not statically reachable yet,
	/// so the report shows the fence and its reason rather than proposing removal.
	DeclaredReason {
		/// The evidence anchor: the item's `file:line`.
		span: Span,
		/// The item the marker annotates.
		symbol: String,
		/// The `allow` or `expect` marker form.
		marker: Marker,
		/// The `reason = "..."` string when present; `None` for a bare marker whose reason
		/// lives in an adjacent comment (real data: an undeclared fence).
		reason: Option<String>,
	},
}

/// A `file:line` evidence anchor. The column is not carried in Tier-0 (the line is enough
/// for a human to open the site).
#[derive(Debug, Serialize)]
pub(crate) struct Span {
	/// The file, echoed verbatim (relative), so the output is identical on any machine.
	pub(crate) file: PathBuf,
	/// The 1-based line number.
	pub(crate) line: u32,
}

/// Which oracle produced a `DeadCode` row. A closed set narrowed to the two oracles that
/// genuinely emit a dead-code row (Make illegal states unrepresentable): the rustc harvest
/// and the source scan for cfg-gated items rustc never compiles. Never cargo-machete, which
/// only ever produces an `UnusedDep` row, so that case is not representable.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(
	not(test),
	allow(
		dead_code,
		reason = "the signal-harvest increments (2-4) and the tests construct these; Increment 1 ships the schema and an empty report, so the release build has no producer yet"
	)
)]
pub(crate) enum DeadCodeSource {
	/// The rustc dead-code harvest from `cargo check --message-format=json`.
	Rustc,
	/// The line-oriented source scan for cfg-gated items rustc never compiles.
	SourceScan,
}

/// A source suppression marker form. The source scan (increment 2) constructs both variants
/// in the release build, so no cfg-split `allow(dead_code)` is needed here.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Marker {
	/// `#[allow(dead_code)]`.
	Allow,
	/// `#[expect(dead_code)]`.
	Expect,
}

/// Why a harvested item is shown but excluded from the candidate set. A closed set: the
/// exclusion is always derived from ground truth (a marker, a cfg attribute, an FFI
/// attribute, or the compiler's own reachability), never a hand-curated denylist.
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(
	not(test),
	allow(
		dead_code,
		reason = "the rustc harvest (increment 3, via reclassify) and the tests construct these; increment 2 builds reclassify but has no release caller yet, so the release build still constructs no Exclusion"
	)
)]
pub(crate) enum Exclusion {
	/// Under a `#[cfg(...)]` that was not the analysed configuration.
	CfgGated,
	/// Carries `#[no_mangle]` or `extern "C"` (a foreign entry point).
	Ffi,
	/// Carries a `#[allow/expect(dead_code)]` suppression.
	Suppressed,
	/// Part of the CLI / output / emitted-pack contract surface.
	ContractSurface,
}

impl DeadCodeSource {
	/// The human label for the Markdown, read from the single-sourced per-signal labels so
	/// a row's provenance and the signal-set disclosure name the same oracle identically.
	fn label(&self) -> &'static str {
		match self {
			DeadCodeSource::Rustc => LABEL_RUSTC_DEAD_CODE,
			DeadCodeSource::SourceScan => LABEL_SOURCE_SCAN,
		}
	}
}

impl Marker {
	/// The human label for the Markdown.
	fn label(&self) -> &'static str {
		match self {
			Marker::Allow => "allow",
			Marker::Expect => "expect",
		}
	}
}

impl Exclusion {
	/// The human label for the Markdown.
	fn label(&self) -> &'static str {
		match self {
			Exclusion::CfgGated => "cfg-gated",
			Exclusion::Ffi => "FFI",
			Exclusion::Suppressed => "suppressed",
			Exclusion::ContractSurface => "contract surface",
		}
	}
}

impl Span {
	/// The `file:line` anchor, echoed verbatim.
	fn anchor(&self) -> String {
		format!("{}:{}", self.file.display(), self.line)
	}
}

// -- Source scan (increment 2): author-declared reasons and FFI markers --

/// One suppression or FFI marker the source scan found, with the site it annotates. This is
/// the ground truth the exclusion hook (`reclassify`) reads and the fence inventory
/// (`declared_reasons`) projects: a dead-code candidate a later increment harvests at the
/// same `file` + `item_line` is reclassified from a candidate to a shown-not-candidate row by
/// the marker recorded here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ScannedMarker {
	/// The file the marker was found in, relative to the scanned crate root.
	pub(crate) file: PathBuf,
	/// The 1-based line of the attribute (the fence itself), the evidence anchor for the fence.
	pub(crate) line: u32,
	/// The 1-based line of the annotated item (the `fn` / `struct` signature line, which is the
	/// coordinate rustc reports a dead-code candidate by). The `reclassify` join key.
	pub(crate) item_line: u32,
	/// The item the marker annotates: the next code line's leading identifier (a heuristic). A
	/// display label only; the join key is `item_line`, not this.
	pub(crate) symbol: String,
	/// What the marker declares or suppresses.
	pub(crate) kind: MarkerKind,
}

/// What a scanned marker carries, each variant with only its own evidence: a
/// `#[allow/expect(dead_code)]` suppression (an author-declared reason / fence) or a
/// `#[no_mangle]` / `extern "C"` foreign entry point.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum MarkerKind {
	/// A `#[allow/expect(dead_code)]` suppression, with its form and its `reason = "..."`
	/// string when present (`None` for a bare marker whose reason lives in an adjacent comment).
	Suppression {
		/// The `allow` or `expect` form.
		marker: Marker,
		/// The `reason = "..."` string when present.
		reason: Option<String>,
	},
	/// A `#[no_mangle]` or `extern "C"` foreign entry point.
	Ffi,
}

/// Scan the crate's `src/**/*.rs` under `root` for `#[allow/expect(dead_code)]` suppression
/// markers and `#[no_mangle]` / `extern "C"` FFI markers, returning one `ScannedMarker` per
/// site in a deterministic order (files sorted, then by line).
///
/// This is a HEURISTIC line scan, not a syntax parse (Minimal by default: no `syn`
/// dependency), so it can both UNDER-report and OVER-report. It may MISS a marker split across
/// lines (as `src/audit.rs`'s own multi-line `cfg_attr` blocks are), an inner `#![...]`
/// attribute, or an unusually written attribute, and the annotated symbol is the next code
/// line's leading identifier by a simple token heuristic. It may also OVER-report: a
/// block-comment body line not led by `*`, or a raw string, that contains `extern "C"` records
/// a spurious FFI marker, and an `#[allow(dead_code)]` commented out on its own line is read as
/// a real suppression fence. This is acceptable for an advisory inventory. A `root` with no
/// `src` directory yields an empty scan (the audited dir need not be a crate).
pub(crate) fn scan_source(root: &Path) -> io::Result<Vec<ScannedMarker>> {
	let src = root.join("src");
	if !src.is_dir() {
		return Ok(Vec::new());
	}
	let mut files = Vec::new();
	collect_rs_files(&src, &mut files)?;
	// `read_dir` order is unspecified, so sort for a deterministic scan (and report) order.
	files.sort();
	let mut markers = Vec::new();
	for file in &files {
		let contents = fs::read_to_string(file)?;
		// Echo the path relative to the crate root, so the report is identical on any machine.
		let relative = file.strip_prefix(root).unwrap_or(file);
		scan_file(relative, &contents, &mut markers);
	}
	Ok(markers)
}

/// Recursively collect `*.rs` files under `dir` into `out`, using only `std::fs` (no
/// walk-crate dependency). The caller sorts the full list for a deterministic scan order.
fn collect_rs_files(
	dir: &Path,
	out: &mut Vec<PathBuf>,
) -> io::Result<()> {
	for entry in fs::read_dir(dir)? {
		let entry = entry?;
		// Skip symlinks and do not recurse into them: a symlinked directory can form a cycle
		// (for example `src/loop -> src`) that this unbounded recursion would follow to a
		// stack-overflow abort. Not following symlinks makes the walk cycle-safe by construction
		// (fail-safe, so a pathological tree cannot crash the tool).
		if entry.file_type()?.is_symlink() {
			continue;
		}
		let path = entry.path();
		if path.is_dir() {
			collect_rs_files(&path, out)?;
		} else if path.extension().is_some_and(|extension| extension == "rs") {
			out.push(path);
		}
	}
	Ok(())
}

/// Scan one file's `contents` (line by line, 1-based) for suppression and FFI markers,
/// appending each to `out` under the relative `file` path. An attribute attaches to the next
/// non-attribute, non-comment, non-blank line (the annotated item); comments and blank lines
/// between an attribute and its item do not break the association. An `extern "C"` on the item
/// line itself is an FFI entry point unless a `#[no_mangle]` already recorded that site.
fn scan_file(
	file: &Path,
	contents: &str,
	out: &mut Vec<ScannedMarker>,
) {
	// The suppression / no_mangle attributes seen since the last item, each with its own line,
	// waiting to attach to the next item line.
	let mut pending: Vec<(u32, MarkerKind)> = Vec::new();
	for (index, raw) in contents.lines().enumerate() {
		let line = (index + 1) as u32;
		let trimmed = raw.trim_start();
		if trimmed.is_empty() || is_comment(trimmed) {
			continue;
		}
		if is_attr_line(trimmed) {
			if let Some((marker, reason)) = parse_suppression(trimmed) {
				pending.push((
					line,
					MarkerKind::Suppression {
						marker,
						reason,
					},
				));
			} else if trimmed.contains("no_mangle") {
				pending.push((line, MarkerKind::Ffi));
			}
			continue;
		}
		// This is the annotated item; attach every pending attribute to its symbol. `line` here is
		// the item line (the join coordinate rustc reports), distinct from each attribute's own
		// fence line.
		let symbol = extract_symbol(trimmed);
		let mut recorded_ffi = false;
		for (attr_line, kind) in pending.drain(..) {
			if matches!(kind, MarkerKind::Ffi) {
				recorded_ffi = true;
			}
			out.push(ScannedMarker {
				file: file.to_path_buf(),
				line: attr_line,
				item_line: line,
				symbol: symbol.clone(),
				kind,
			});
		}
		// A bare `extern "C" fn`/block is an FFI entry point in its own right; skip it only when
		// a `#[no_mangle]` attribute already recorded this same site. Its fence line and item line
		// are the same line.
		if !recorded_ffi && trimmed.contains("extern \"C\"") {
			out.push(ScannedMarker {
				file: file.to_path_buf(),
				line,
				item_line: line,
				symbol,
				kind: MarkerKind::Ffi,
			});
		}
	}
}

/// Whether a trimmed line is an attribute (`#[...]` outer or `#![...]` inner), which is never
/// the annotated item.
fn is_attr_line(trimmed: &str) -> bool {
	trimmed.starts_with("#[") || trimmed.starts_with("#![")
}

/// Whether a trimmed line is a comment (line, doc, or a block-comment body), which does not
/// separate an attribute from its item. A `*`-led line catches block-comment continuations.
fn is_comment(trimmed: &str) -> bool {
	trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*')
}

/// Parse a trimmed outer-attribute line into its `#[allow/expect(dead_code)]` marker and its
/// optional `reason = "..."` string, or `None` when the line is not a dead-code suppression.
/// Only `dead_code` in the lint list counts: `#[allow(unused)]` does not match, and a
/// `dead_code` mentioned only inside a `reason` string does not match (the lint names precede
/// any `reason`). Handles the `#[cfg_attr(..., allow(dead_code))]` form by reading the inner
/// `allow(...)` / `expect(...)`.
fn parse_suppression(trimmed: &str) -> Option<(Marker, Option<String>)> {
	if !trimmed.starts_with("#[") {
		return None;
	}
	// Check `expect` before `allow`: a line carries at most one of them.
	for (keyword, marker) in [("expect(", Marker::Expect), ("allow(", Marker::Allow)] {
		if let Some(args) = attr_args(trimmed, keyword) {
			if lint_list_has_dead_code(args) {
				return Some((marker, extract_reason(args)));
			}
		}
	}
	None
}

/// The balanced-parenthesis argument string of the first `keyword` (`"allow("` or `"expect("`)
/// in `attr`, or `None` when it is absent. Balancing lets a `reason` string containing
/// parentheses not close the argument list early.
fn attr_args<'a>(
	attr: &'a str,
	keyword: &str,
) -> Option<&'a str> {
	let start = attr.find(keyword)? + keyword.len();
	let mut depth = 1usize;
	for (offset, byte) in attr[start ..].bytes().enumerate() {
		match byte {
			b'(' => depth += 1,
			b')' => {
				depth -= 1;
				if depth == 0 {
					return Some(&attr[start .. start + offset]);
				}
			}
			_ => {}
		}
	}
	None
}

/// Whether the lint list in an `allow`/`expect` argument string names `dead_code`. Only the
/// names before any `reason = "..."` count, so a `dead_code` inside the reason text does not
/// match.
fn lint_list_has_dead_code(args: &str) -> bool {
	let names = match args.find("reason") {
		Some(index) => &args[.. index],
		None => args,
	};
	names.split(',').any(|name| name.trim() == "dead_code")
}

/// The `reason = "..."` string of an `allow`/`expect` argument string, or `None` when absent. A
/// heuristic: it reads the first double-quoted run after `reason` and does not handle escaped
/// quotes (the tree's reasons have none).
fn extract_reason(args: &str) -> Option<String> {
	let after = &args[args.find("reason")? ..];
	let open = after.find('"')?;
	let rest = &after[open + 1 ..];
	let close = rest.find('"')?;
	Some(rest[.. close].to_string())
}

/// The item name an attribute annotates: the first token of the item line that is not a
/// visibility or item keyword and starts with an identifier character, trimmed to its leading
/// identifier. A heuristic that resolves fields (`pub budget: ...` -> `budget`), enum variants
/// (`Done,` -> `Done`), and functions (`$vis fn parse(...)` -> `parse`); it falls back to the
/// trimmed line when no identifier is found.
fn extract_symbol(item_line: &str) -> String {
	for token in item_line.split_whitespace() {
		if is_skippable_prefix(token) {
			continue;
		}
		if let Some(identifier) = leading_identifier(token) {
			return identifier;
		}
	}
	item_line.trim().to_string()
}

/// Whether a token is a visibility or item keyword to skip before the item name (`pub`,
/// `pub(crate)`, `fn`, `struct`, and the like).
fn is_skippable_prefix(token: &str) -> bool {
	token.starts_with("pub(")
		|| matches!(
			token,
			"pub"
				| "fn" | "struct"
				| "enum" | "trait"
				| "const" | "static"
				| "type" | "mod"
				| "union" | "unsafe"
				| "async" | "impl"
				| "extern" | "default"
				| "move" | "dyn"
				| "let" | "mut"
				| "ref"
		)
}

/// The leading identifier of `token` (alphanumeric or `_`, starting with a letter or `_`), or
/// `None` when the token does not start with an identifier character (for example `$vis` or a
/// string literal).
fn leading_identifier(token: &str) -> Option<String> {
	let first = token.chars().next()?;
	if !(first.is_ascii_alphabetic() || first == '_') {
		return None;
	}
	Some(
		token
			.chars()
			.take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
			.collect(),
	)
}

/// The author-declared-reason records the scan found: every `#[allow/expect(dead_code)]`
/// suppression becomes a `DeclaredReason` fence (explicitly NOT a candidate). FFI markers are
/// not fences; they feed `reclassify` only, so they produce no record here.
pub(crate) fn declared_reasons(markers: &[ScannedMarker]) -> Vec<AuditRecord> {
	markers
		.iter()
		.filter_map(|marker| match &marker.kind {
			MarkerKind::Suppression {
				marker: form,
				reason,
			} => Some(AuditRecord::DeclaredReason {
				span: Span {
					file: marker.file.clone(),
					line: marker.line,
				},
				symbol: marker.symbol.clone(),
				marker: form.clone(),
				reason: reason.clone(),
			}),
			MarkerKind::Ffi => None,
		})
		.collect()
}

/// The exclusion hook the rustc harvest (increment 3) will consume. Given the markers the
/// source scan found and a dead-code candidate's site (its `file` and item `line`, the
/// `file:line` coordinate a rustc diagnostic reports it by), decide whether the candidate is
/// shown-but-not-a-candidate and why: an FFI marker at the site reclassifies it
/// `Exclusion::Ffi`, a suppression marker reclassifies it `Exclusion::Suppressed`, and no
/// marker leaves it a candidate (`None`). FFI takes precedence when a site carries both (a
/// foreign entry point is the stronger structural reason it is statically unreachable). The
/// join is on `(file, item_line)`, not `(file, symbol)`: two items in one file that reduce to
/// the same leading-identifier symbol have distinct item lines, so the collision is
/// structurally impossible and the key is rustc's own coordinate rather than a lossy symbol
/// heuristic. Increment 2 harvests no dead-code candidates yet, so this has no caller in the
/// report path and is exercised only by its unit tests; increment 3 connects it.
#[cfg_attr(
	not(test),
	allow(
		dead_code,
		reason = "the rustc harvest (increment 3) is the caller; increment 2 builds and unit-tests the hook but has no dead-code candidates to reclassify yet"
	)
)]
pub(crate) fn reclassify(
	markers: &[ScannedMarker],
	file: &Path,
	line: u32,
) -> Option<Exclusion> {
	let mut suppressed = false;
	for marker in markers {
		if marker.file.as_path() != file || marker.item_line != line {
			continue;
		}
		match marker.kind {
			MarkerKind::Ffi => return Some(Exclusion::Ffi),
			MarkerKind::Suppression {
				..
			} => suppressed = true,
		}
	}
	suppressed.then_some(Exclusion::Suppressed)
}

/// Project a report to the deterministic Markdown: a head carrying the mandatory caveat
/// and the signal-set disclosure, then the four record buckets (dead-code candidates,
/// unused-dependency candidates, author-declared fences, and excluded rows), each with a
/// `file:line` anchor. Total over every record kind and deterministic (records keep their
/// input order); an empty report is the caveat plus four empty sections. No timestamps.
pub(crate) fn render_markdown(report: &CodeValueReport) -> String {
	let mut out = String::new();
	out.push_str(&format!("# Code-value audit: {}\n\n", report.task));
	// The caveat is read from the report field (the same `AUDIT_CAVEAT` const), so the head
	// text cannot drift from the JSON intermediate's `caveat`.
	out.push_str(&format!("> {}\n\n", report.caveat));
	render_signal_disclosure(&mut out, &report.generated_from);

	let mut dead_code_candidates: Vec<String> = Vec::new();
	let mut unused_dep_candidates: Vec<String> = Vec::new();
	let mut declared_reasons: Vec<String> = Vec::new();
	let mut excluded: Vec<String> = Vec::new();
	for record in &report.records {
		match record {
			AuditRecord::DeadCode {
				span,
				symbol,
				lint,
				source,
				exclusion,
			} => {
				let line = format!(
					"- `{symbol}` at `{}` (lint `{lint}`, from {})",
					span.anchor(),
					source.label()
				);
				match exclusion {
					None => dead_code_candidates.push(line),
					Some(exclusion) =>
						excluded.push(format!("{line}; excluded: {}", exclusion.label())),
				}
			}
			AuditRecord::UnusedDep {
				crate_name,
				manifest,
				caveat,
			} => unused_dep_candidates.push(format!(
				"- `{crate_name}` at `{}` (from {LABEL_CARGO_MACHETE}); caveat: {caveat}",
				manifest.anchor(),
			)),
			AuditRecord::DeclaredReason {
				span,
				symbol,
				marker,
				reason,
			} => {
				let reason = reason
					.as_deref()
					.unwrap_or("no machine-readable reason (see the adjacent comment)");
				declared_reasons.push(format!(
					"- `{symbol}` at `{}` (`{}`): {reason}",
					span.anchor(),
					marker.label()
				));
			}
		}
	}

	render_section(&mut out, "Candidates: dead code", &dead_code_candidates);
	render_section(&mut out, "Candidates: unused dependencies", &unused_dep_candidates);
	render_section(&mut out, "Author-declared reasons (Chesterton's Fences)", &declared_reasons);
	render_section(&mut out, "Excluded (shown, not candidates)", &excluded);
	out.trim_end_matches('\n').to_string()
}

/// Render the signal-set disclosure: which signals ran and which did not. A signal that
/// did not run is disclosed as widening the caveat above, so "nothing flagged" is never
/// read as coverage the run did not have.
fn render_signal_disclosure(
	out: &mut String,
	signals: &SignalSet,
) {
	let ran = signals.ran();
	let not_run = signals.not_run();
	let ran = if ran.is_empty() {
		"none (this report analysed nothing yet)".to_string()
	} else {
		ran.join(", ")
	};
	out.push_str(&format!("Signals run: {ran}.\n"));
	if !not_run.is_empty() {
		out.push_str(&format!(
			"Signals not run (their coverage is absent, widening the caveat above): {}.\n",
			not_run.join(", ")
		));
	}
	out.push('\n');
}

/// Render one `## <title>` section: the bullet lines, or `_None._` when the bucket is empty.
fn render_section(
	out: &mut String,
	title: &str,
	lines: &[String],
) {
	out.push_str(&format!("## {title}\n\n"));
	if lines.is_empty() {
		out.push_str("_None._\n\n");
	} else {
		for line in lines {
			out.push_str(line);
			out.push('\n');
		}
		out.push('\n');
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// A populated fixture exercising every record kind and every bucket, so the projection
	/// is proven total and deterministic before the signal producers land.
	fn populated_report() -> CodeValueReport {
		CodeValueReport {
			task: "demo".to_string(),
			generated_from: SignalSet {
				rustc_dead_code: true,
				source_scan: true,
				cargo_machete: true,
			},
			caveat: AUDIT_CAVEAT,
			records: vec![
				AuditRecord::DeadCode {
					span: Span {
						file: PathBuf::from("src/foo.rs"),
						line: 12,
					},
					symbol: "unused_fn".to_string(),
					lint: "dead_code".to_string(),
					source: DeadCodeSource::Rustc,
					exclusion: None,
				},
				AuditRecord::DeadCode {
					span: Span {
						file: PathBuf::from("src/bar.rs"),
						line: 3,
					},
					symbol: "ffi_entry".to_string(),
					lint: "dead_code".to_string(),
					source: DeadCodeSource::Rustc,
					exclusion: Some(Exclusion::Ffi),
				},
				AuditRecord::UnusedDep {
					crate_name: "leftover".to_string(),
					manifest: Span {
						file: PathBuf::from("Cargo.toml"),
						line: 20,
					},
					caveat: "used only via a macro or a re-export can be a false positive",
				},
				AuditRecord::DeclaredReason {
					span: Span {
						file: PathBuf::from("src/checks.rs"),
						line: 135,
					},
					symbol: "Check::budget".to_string(),
					marker: Marker::Allow,
					reason: Some(
						"parsed for the schema; used by the later mutation module".to_string(),
					),
				},
				AuditRecord::DeclaredReason {
					span: Span {
						file: PathBuf::from("src/pack.rs"),
						line: 37,
					},
					symbol: "bare_field".to_string(),
					marker: Marker::Allow,
					reason: None,
				},
			],
		}
	}

	/// The all-signals-unrun report the projection must still render (the caveat plus four empty
	/// sections). No release path produces this now that the source scan always runs, so it is a
	/// test fixture for the projection's empty-disclosure branch.
	fn none_report(task: &str) -> CodeValueReport {
		CodeValueReport {
			task: task.to_string(),
			generated_from: SignalSet {
				rustc_dead_code: false,
				source_scan: false,
				cargo_machete: false,
			},
			caveat: AUDIT_CAVEAT,
			records: Vec::new(),
		}
	}

	#[test]
	fn empty_report_is_caveat_plus_empty_sections() {
		let report = none_report("mytask");
		let markdown = render_markdown(&report);
		// The caveat leads the report head (line 3, after the title and its blank line), so a
		// reader sees the "not evidence of absence" framing before any finding.
		let head: Vec<&str> = markdown.lines().take(3).collect();
		assert_eq!(head[0], "# Code-value audit: mytask");
		assert_eq!(head[1], "");
		assert_eq!(head[2], format!("> {AUDIT_CAVEAT}"));
		// No signal ran, so the disclosure says the report analysed nothing and lists every
		// signal as not run.
		assert!(markdown.contains("Signals run: none (this report analysed nothing yet)."));
		assert!(markdown.contains("Signals not run"));
		assert!(markdown.contains("rustc dead-code"));
		assert!(markdown.contains("cargo-machete"));
		// Every bucket is present and empty.
		assert!(markdown.contains("## Candidates: dead code\n\n_None._"));
		assert!(markdown.contains("## Candidates: unused dependencies\n\n_None._"));
		assert!(markdown.contains("## Author-declared reasons (Chesterton's Fences)\n\n_None._"));
		assert!(markdown.contains("## Excluded (shown, not candidates)\n\n_None._"));
		// No records at all.
		assert!(report.records.is_empty());
	}

	#[test]
	fn caveat_is_the_single_sourced_field() {
		// The head caveat is the SAME const as the JSON `caveat` field, so they cannot drift.
		let report = none_report("t");
		assert_eq!(report.caveat, AUDIT_CAVEAT);
		let markdown = render_markdown(&report);
		assert!(markdown.contains(&format!("> {}", report.caveat)));
	}

	#[test]
	fn projection_is_total_over_every_record_kind() {
		// The golden projection of a fixture touching every bucket, pinning the deterministic
		// grouping (candidate versus excluded versus fence) the verdict is derived from.
		let markdown = render_markdown(&populated_report());
		let expected = format!(
			"# Code-value audit: demo\n\n> {AUDIT_CAVEAT}\n\nSignals run: rustc dead-code, source scan, cargo-machete.\n\n## Candidates: dead code\n\n- `unused_fn` at `src/foo.rs:12` (lint `dead_code`, from rustc dead-code)\n\n## Candidates: unused dependencies\n\n- `leftover` at `Cargo.toml:20` (from cargo-machete); caveat: used only via a macro or a re-export can be a false positive\n\n## Author-declared reasons (Chesterton's Fences)\n\n- `Check::budget` at `src/checks.rs:135` (`allow`): parsed for the schema; used by the later mutation module\n- `bare_field` at `src/pack.rs:37` (`allow`): no machine-readable reason (see the adjacent comment)\n\n## Excluded (shown, not candidates)\n\n- `ffi_entry` at `src/bar.rs:3` (lint `dead_code`, from rustc dead-code); excluded: FFI"
		);
		assert_eq!(markdown, expected);
	}

	#[test]
	fn all_signals_run_disclosure_lists_none_not_run() {
		// When every signal ran, the disclosure lists them as run and omits the not-run line.
		let markdown = render_markdown(&populated_report());
		assert!(markdown.contains("Signals run: rustc dead-code, source scan, cargo-machete."));
		assert!(!markdown.contains("Signals not run"));
	}

	#[test]
	fn every_signal_marker_and_exclusion_label_renders() {
		// Exercise the label paths the golden fixture does not: the source-scan signal, the
		// `expect` marker, and the three non-FFI exclusions, so every schema variant is proven
		// to project (and every variant is constructed under test, which the cfg-split
		// `allow(dead_code)` on the release build relies on).
		let report = CodeValueReport {
			task: "labels".to_string(),
			generated_from: SignalSet {
				rustc_dead_code: true,
				source_scan: true,
				cargo_machete: false,
			},
			caveat: AUDIT_CAVEAT,
			records: vec![
				AuditRecord::DeadCode {
					span: Span {
						file: PathBuf::from("src/a.rs"),
						line: 1,
					},
					symbol: "cfg_only".to_string(),
					lint: "dead_code".to_string(),
					source: DeadCodeSource::SourceScan,
					exclusion: Some(Exclusion::CfgGated),
				},
				AuditRecord::DeadCode {
					span: Span {
						file: PathBuf::from("src/b.rs"),
						line: 2,
					},
					symbol: "hushed".to_string(),
					lint: "dead_code".to_string(),
					source: DeadCodeSource::Rustc,
					exclusion: Some(Exclusion::Suppressed),
				},
				AuditRecord::DeadCode {
					span: Span {
						file: PathBuf::from("src/c.rs"),
						line: 3,
					},
					symbol: "handler".to_string(),
					lint: "dead_code".to_string(),
					source: DeadCodeSource::Rustc,
					exclusion: Some(Exclusion::ContractSurface),
				},
				AuditRecord::DeclaredReason {
					span: Span {
						file: PathBuf::from("src/d.rs"),
						line: 4,
					},
					symbol: "future_field".to_string(),
					marker: Marker::Expect,
					reason: Some("declared for the schema".to_string()),
				},
			],
		};
		let markdown = render_markdown(&report);
		// A missing signal is disclosed as widening the caveat.
		assert!(markdown.contains(
			"Signals not run (their coverage is absent, widening the caveat above): cargo-machete."
		));
		// The source-scan signal label and the `expect` marker label.
		assert!(markdown.contains("from source scan"));
		assert!(markdown.contains("(`expect`): declared for the schema"));
		// Every exclusion label projects.
		assert!(markdown.contains("excluded: cfg-gated"));
		assert!(markdown.contains("excluded: suppressed"));
		assert!(markdown.contains("excluded: contract surface"));
	}

	#[test]
	fn intermediate_serialises_with_kind_discriminators() {
		// The `--json` intermediate carries the caveat and a `"kind"` discriminator per row,
		// so a downstream tool can read the typed shape.
		let json = serde_json::to_string_pretty(&populated_report()).unwrap();
		assert!(json.contains("\"caveat\":"));
		assert!(json.contains("\"kind\": \"dead-code\""));
		assert!(json.contains("\"kind\": \"unused-dep\""));
		assert!(json.contains("\"kind\": \"declared-reason\""));
		assert!(json.contains("\"rustc_dead_code\": true"));
	}

	/// A fixture crate source covering every attribute form the marker scan must resolve, so
	/// the parser is tested against inputs, not the (evolving) live tree. Built with `concat!`
	/// of per-line literals so no source line here looks like a bare attribute to the scanner
	/// (the scan of `src/audit.rs` itself must not pick these fixture lines up).
	fn marker_fixture() -> &'static str {
		concat!(
			"#[allow(dead_code)]\n",
			"fn bare_allow() {}\n",
			"\n",
			"#[allow(dead_code, reason = \"kept for the schema\")]\n",
			"fn allow_with_reason() {}\n",
			"\n",
			"#[expect(dead_code, reason = \"declared for later\")]\n",
			"fn expect_with_reason() {}\n",
			"\n",
			"#[allow(dead_code, reason = \"kept for foo(bar)\")]\n",
			"fn paren_reason() {}\n",
			"\n",
			"#[cfg_attr(not(test), allow(dead_code))]\n",
			"struct CfgSplit;\n",
			"\n",
			"#[allow(unused)]\n",
			"fn not_dead_code() {}\n",
			"\n",
			"#[allow(unused, reason = \"silences unused, dead_code, unreachable\")]\n",
			"fn dead_code_only_in_reason() {}\n",
			"\n",
			"#[no_mangle]\n",
			"pub extern \"C\" fn exported() {}\n",
			"\n",
			"extern \"C\" fn other_ffi() {}\n",
			"\n",
			"fn plain_item() {}\n",
		)
	}

	/// Scan `marker_fixture` under a fixed relative file, so the tests read the markers a real
	/// scan would produce.
	fn scan_fixture() -> Vec<ScannedMarker> {
		let mut markers = Vec::new();
		scan_file(Path::new("src/fixture.rs"), marker_fixture(), &mut markers);
		markers
	}

	fn find_marker<'a>(
		markers: &'a [ScannedMarker],
		symbol: &str,
	) -> &'a ScannedMarker {
		markers
			.iter()
			.find(|marker| marker.symbol == symbol)
			.unwrap_or_else(|| panic!("no marker for `{symbol}`"))
	}

	#[test]
	fn marker_scan_resolves_each_attribute_form() {
		let markers = scan_fixture();
		// Only dead-code suppressions and FFI entry points are markers: `#[allow(unused)]` and
		// the plain item contribute nothing.
		assert!(markers.iter().all(|marker| marker.symbol != "not_dead_code"));
		assert!(markers.iter().all(|marker| marker.symbol != "plain_item"));
		// A `dead_code` appearing ONLY inside the reason string is not a lint-list name, so it
		// produces no marker: the reason-stripping guard in `lint_list_has_dead_code` excludes it.
		// (The reason lists `unused, dead_code, unreachable`; removing that guard would let the
		// comma-split find the `dead_code` token in the reason and wrongly fence this item.)
		assert!(markers.iter().all(|marker| marker.symbol != "dead_code_only_in_reason"));
		// A reason string with an inner `(` exercises the balanced-parenthesis argument scan in
		// `attr_args`: the arg list closes on the matching outer `)`, not the `)` inside
		// `foo(bar)`, so the reason is captured whole. (Without balancing the arg list would close
		// early at the inner `)` and the reason would not resolve.)
		assert_eq!(
			find_marker(&markers, "paren_reason").kind,
			MarkerKind::Suppression {
				marker: Marker::Allow,
				reason: Some("kept for foo(bar)".to_string()),
			}
		);
		// A bare `#[allow(dead_code)]`: Allow, no reason.
		assert_eq!(
			find_marker(&markers, "bare_allow").kind,
			MarkerKind::Suppression {
				marker: Marker::Allow,
				reason: None,
			}
		);
		// `#[allow(dead_code, reason = "...")]`: Allow with the reason captured.
		assert_eq!(
			find_marker(&markers, "allow_with_reason").kind,
			MarkerKind::Suppression {
				marker: Marker::Allow,
				reason: Some("kept for the schema".to_string()),
			}
		);
		// `#[expect(dead_code, reason = "...")]`: Expect with the reason captured.
		assert_eq!(
			find_marker(&markers, "expect_with_reason").kind,
			MarkerKind::Suppression {
				marker: Marker::Expect,
				reason: Some("declared for later".to_string()),
			}
		);
		// `#[cfg_attr(not(test), allow(dead_code))]`: Allow, no reason.
		assert_eq!(
			find_marker(&markers, "CfgSplit").kind,
			MarkerKind::Suppression {
				marker: Marker::Allow,
				reason: None,
			}
		);
		// FFI: `#[no_mangle]` + `extern "C"` records the site once (deduped), and a bare
		// `extern "C" fn` records its own site.
		assert_eq!(find_marker(&markers, "exported").kind, MarkerKind::Ffi);
		assert_eq!(find_marker(&markers, "other_ffi").kind, MarkerKind::Ffi);
		assert_eq!(markers.iter().filter(|marker| marker.symbol == "exported").count(), 1);
	}

	#[test]
	fn declared_reasons_are_the_suppressions_only() {
		let markers = scan_fixture();
		let records = declared_reasons(&markers);
		// Five suppressions become fences (bare_allow, allow_with_reason, expect_with_reason,
		// paren_reason, CfgSplit); the two FFI markers and the two non-dead-code negatives do not.
		assert_eq!(records.len(), 5);
		assert!(records.iter().all(|record| matches!(record, AuditRecord::DeclaredReason { .. })));
		// The bare-reason fence carries `None`; its marker form is `Allow`.
		let AuditRecord::DeclaredReason {
			marker,
			reason,
			..
		} = records
			.iter()
			.find(
				|record| matches!(record, AuditRecord::DeclaredReason { symbol, .. } if symbol == "bare_allow"),
			)
			.expect("bare_allow fence")
		else {
			panic!("expected a declared reason");
		};
		assert_eq!(*marker, Marker::Allow);
		assert_eq!(*reason, None);
	}

	#[test]
	fn reclassify_maps_a_site_to_its_exclusion() {
		let markers = scan_fixture();
		let file = Path::new("src/fixture.rs");
		// The join key is the annotated item's line (rustc's `file:line`), looked up here by
		// symbol so the test does not hard-code fixture line numbers.
		let item_line = |symbol: &str| find_marker(&markers, symbol).item_line;
		// An FFI site -> Ffi; a suppressed site -> Suppressed; an unmarked line -> None.
		assert_eq!(reclassify(&markers, file, item_line("exported")), Some(Exclusion::Ffi));
		assert_eq!(reclassify(&markers, file, item_line("other_ffi")), Some(Exclusion::Ffi));
		assert_eq!(
			reclassify(&markers, file, item_line("bare_allow")),
			Some(Exclusion::Suppressed)
		);
		assert_eq!(
			reclassify(&markers, file, item_line("expect_with_reason")),
			Some(Exclusion::Suppressed)
		);
		// A line carrying no marker (beyond the fixture's own lines) is not a candidate site.
		assert_eq!(reclassify(&markers, file, 9999), None);
		// The same item line in a different file is not the same site.
		assert_eq!(reclassify(&markers, Path::new("src/other.rs"), item_line("bare_allow")), None);
	}

	#[test]
	fn reclassify_prefers_ffi_when_a_site_carries_both() {
		// A hand-built site carrying both a suppression and an FFI marker: FFI wins, because a
		// foreign entry point is the stronger structural reason the item is statically
		// unreachable. There is no FFI in the tree today, so this and the fixture FFI cases are
		// how the FFI path is exercised at all.
		let file = PathBuf::from("src/both.rs");
		// Both attributes annotate the same item (item line 3), on distinct fence lines (1 and 2).
		let markers = vec![
			ScannedMarker {
				file: file.clone(),
				line: 1,
				item_line: 3,
				symbol: "dual".to_string(),
				kind: MarkerKind::Suppression {
					marker: Marker::Allow,
					reason: None,
				},
			},
			ScannedMarker {
				file: file.clone(),
				line: 2,
				item_line: 3,
				symbol: "dual".to_string(),
				kind: MarkerKind::Ffi,
			},
		];
		assert_eq!(reclassify(&markers, &file, 3), Some(Exclusion::Ffi));
	}

	#[test]
	fn extract_symbol_reads_the_annotated_item_name() {
		assert_eq!(extract_symbol("pub budget: Option<String>,"), "budget");
		assert_eq!(extract_symbol("Done,"), "Done");
		assert_eq!(extract_symbol("$vis fn parse(text: &str) -> Option<Self> {"), "parse");
		assert_eq!(extract_symbol("pub(crate) fn run() {}"), "run");
		assert_eq!(extract_symbol("description: String,"), "description");
	}

	#[test]
	fn source_scan_finds_known_live_suppression_sites() {
		// A smoke test over the real crate: assert KNOWN sites are present by symbol WITHOUT
		// pinning a total (the count changes as the tree evolves, and `src/audit.rs`'s own
		// multi-line `cfg_attr` markers are deliberately not seen by this single-line scan).
		let markers = scan_source(Path::new(env!("CARGO_MANIFEST_DIR"))).expect("scan the crate");
		let has_suppression = |file: &str, symbol: &str| {
			markers.iter().any(|marker| {
				marker.file.as_path() == Path::new(file)
					&& marker.symbol == symbol
					&& matches!(marker.kind, MarkerKind::Suppression { .. })
			})
		};
		assert!(has_suppression("src/checks.rs", "budget"), "checks.rs budget fence");
		assert!(has_suppression("src/checks.rs", "threshold"), "checks.rs threshold fence");
		assert!(has_suppression("src/manifest.rs", "description"), "manifest.rs description fence");
	}
}
