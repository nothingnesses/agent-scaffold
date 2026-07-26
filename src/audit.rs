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
	std::path::PathBuf,
};

/// The single-sourced mandatory caveat: the "not evidence of absence" text carried both
/// in the JSON intermediate (`CodeValueReport::caveat`) and at the head of the Markdown,
/// so the two can never drift (one `const`, read in both places). It states the oracle
/// set explicitly (Ground decisions in evidence): a passing audit is only ever necessary,
/// never sufficient.
pub(crate) const AUDIT_CAVEAT: &str = "This report is advisory. \"Nothing flagged\" is necessary but not sufficient, and is only relative to the named signal set (rustc dead-code under this project's lint configuration, source suppression markers, and cargo-machete's source-grep heuristic). Suppressed, cfg-gated (non-analysed targets), FFI, dynamically dispatched, and reflection-reached code is not covered. A passing audit is not proof the codebase has no dead code.";

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
	/// Build the Increment 1 report for `task`: no signals run, so the record list is
	/// empty and the caveat widens to disclose that nothing was analysed. Later increments
	/// replace this with a signal-harvesting builder; the schema and projection do not change.
	pub(crate) fn empty(task: String) -> Self {
		CodeValueReport {
			task,
			generated_from: SignalSet::none(),
			caveat: AUDIT_CAVEAT,
			records: Vec::new(),
		}
	}
}

/// Which of the Tier-0 signals actually ran for a report. A `false` signal did NOT run,
/// so its coverage is absent and the projection says so (an absent signal widens the
/// caveat rather than reading as a clean pass). Named booleans rather than a `Vec<Signal>`
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
	/// The Increment 1 state: no signal has run yet.
	fn none() -> Self {
		SignalSet {
			rustc_dead_code: false,
			source_scan: false,
			cargo_machete: false,
		}
	}

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
			(self.rustc_dead_code, "rustc dead-code"),
			(self.source_scan, "source suppression / FFI scan"),
			(self.cargo_machete, "cargo-machete unused dependencies"),
		]
	}
}

/// One audit row. The kind is an enum whose variants carry ONLY their own evidence, so an
/// illegal combination (a dependency row with a symbol span, a dead-code row with a
/// machete caveat) cannot be represented. Serialised with a `"kind"` discriminator.
#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
// Increment 1 constructs these variants only under `cfg(test)`; the release-build
// producers are the signal-harvest increments (2-4). This is the cfg-split form (as
// `LoopState::Done` at `src/next.rs:218`), where `allow` is correct rather than `expect`.
#[cfg_attr(
	not(test),
	allow(
		dead_code,
		reason = "the signal-harvest increments (2-4) and the tests construct these; Increment 1 ships the schema and an empty report, so the release build has no producer yet"
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
		/// Which signal produced this row.
		source: Signal,
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
		/// Which signal produced this row.
		source: Signal,
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

/// Which signal produced a row. A closed set (Make illegal states unrepresentable).
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(
	not(test),
	allow(
		dead_code,
		reason = "the signal-harvest increments (2-4) and the tests construct these; Increment 1 ships the schema and an empty report, so the release build has no producer yet"
	)
)]
pub(crate) enum Signal {
	/// The rustc dead-code harvest from `cargo check --message-format=json`.
	RustcBuildJson,
	/// The `cargo-machete` unused-dependency harvest.
	CargoMachete,
	/// The line-oriented source scan for suppression markers and FFI attributes.
	SourceScan,
}

/// A source suppression marker form.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(
	not(test),
	allow(
		dead_code,
		reason = "the signal-harvest increments (2-4) and the tests construct these; Increment 1 ships the schema and an empty report, so the release build has no producer yet"
	)
)]
pub(crate) enum Marker {
	/// `#[allow(dead_code)]`.
	Allow,
	/// `#[expect(dead_code)]`.
	Expect,
}

/// Why a harvested item is shown but excluded from the candidate set. A closed set: the
/// exclusion is always derived from ground truth (a marker, a cfg attribute, an FFI
/// attribute, or the compiler's own reachability), never a hand-curated denylist.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(
	not(test),
	allow(
		dead_code,
		reason = "the signal-harvest increments (2-4) and the tests construct these; Increment 1 ships the schema and an empty report, so the release build has no producer yet"
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

impl Signal {
	/// The human label for the Markdown.
	fn label(&self) -> &'static str {
		match self {
			Signal::RustcBuildJson => "rustc dead-code",
			Signal::CargoMachete => "cargo-machete",
			Signal::SourceScan => "source scan",
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
				source,
				caveat,
			} => unused_dep_candidates.push(format!(
				"- `{crate_name}` at `{}` (from {}); caveat: {caveat}",
				manifest.anchor(),
				source.label()
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
					source: Signal::RustcBuildJson,
					exclusion: None,
				},
				AuditRecord::DeadCode {
					span: Span {
						file: PathBuf::from("src/bar.rs"),
						line: 3,
					},
					symbol: "ffi_entry".to_string(),
					lint: "dead_code".to_string(),
					source: Signal::RustcBuildJson,
					exclusion: Some(Exclusion::Ffi),
				},
				AuditRecord::UnusedDep {
					crate_name: "leftover".to_string(),
					manifest: Span {
						file: PathBuf::from("Cargo.toml"),
						line: 20,
					},
					source: Signal::CargoMachete,
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

	#[test]
	fn empty_report_is_caveat_plus_empty_sections() {
		let report = CodeValueReport::empty("mytask".to_string());
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
		assert!(markdown.contains("cargo-machete unused dependencies"));
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
		let report = CodeValueReport::empty("t".to_string());
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
			"# Code-value audit: demo\n\n> {AUDIT_CAVEAT}\n\nSignals run: rustc dead-code, source suppression / FFI scan, cargo-machete unused dependencies.\n\n## Candidates: dead code\n\n- `unused_fn` at `src/foo.rs:12` (lint `dead_code`, from rustc dead-code)\n\n## Candidates: unused dependencies\n\n- `leftover` at `Cargo.toml:20` (from cargo-machete); caveat: used only via a macro or a re-export can be a false positive\n\n## Author-declared reasons (Chesterton's Fences)\n\n- `Check::budget` at `src/checks.rs:135` (`allow`): parsed for the schema; used by the later mutation module\n- `bare_field` at `src/pack.rs:37` (`allow`): no machine-readable reason (see the adjacent comment)\n\n## Excluded (shown, not candidates)\n\n- `ffi_entry` at `src/bar.rs:3` (lint `dead_code`, from rustc dead-code); excluded: FFI"
		);
		assert_eq!(markdown, expected);
	}

	#[test]
	fn all_signals_run_disclosure_lists_none_not_run() {
		// When every signal ran, the disclosure lists them as run and omits the not-run line.
		let markdown = render_markdown(&populated_report());
		assert!(markdown.contains("Signals run: rustc dead-code, source suppression / FFI scan, cargo-machete unused dependencies."));
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
					source: Signal::SourceScan,
					exclusion: Some(Exclusion::CfgGated),
				},
				AuditRecord::DeadCode {
					span: Span {
						file: PathBuf::from("src/b.rs"),
						line: 2,
					},
					symbol: "hushed".to_string(),
					lint: "dead_code".to_string(),
					source: Signal::RustcBuildJson,
					exclusion: Some(Exclusion::Suppressed),
				},
				AuditRecord::DeadCode {
					span: Span {
						file: PathBuf::from("src/c.rs"),
						line: 3,
					},
					symbol: "handler".to_string(),
					lint: "dead_code".to_string(),
					source: Signal::RustcBuildJson,
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
		assert!(markdown.contains("Signals not run (their coverage is absent, widening the caveat above): cargo-machete unused dependencies."));
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
}
