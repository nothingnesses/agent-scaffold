//! The built-in default principle pack and its data model.
//!
//! Principles are structured data (see `pack/principles.toml`) so the tool can
//! select a subset, order it, and render it at a chosen verbosity, and so that
//! bring-your-own packs can ship principles in the same shape.

use {
	clap::ValueEnum,
	serde::Deserialize,
};

/// One principle in a pack.
///
/// `id`, `tags`, and `related` are not read by the binary yet: they are part of
/// the pack schema, are exercised by the tests, and will drive the selection
/// flags and tag filtering as the tool grows.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Principle {
	/// Stable slug, used in flags, config, and the selection record.
	pub id: String,
	/// Short imperative title.
	pub name: String,
	/// One-sentence summary.
	pub summary: String,
	/// Why to adopt it and what it prevents.
	pub rationale: String,
	/// Category and applicability labels.
	pub tags: Vec<String>,
	/// Whether it is pre-checked in the sane-default set.
	pub default_selected: bool,
	/// Sort key for the output list.
	pub default_order: i64,
	/// Optional links or citations.
	#[serde(default)]
	pub references: Vec<String>,
	/// Optional related principle ids.
	#[serde(default)]
	pub related: Vec<String>,
}

/// The on-disk shape of a principles file: an array of `[[principle]]` tables.
#[derive(Debug, Deserialize)]
struct PrinciplesFile {
	#[serde(default)]
	principle: Vec<Principle>,
}

/// The embedded source of the built-in default pack.
pub const DEFAULT_PRINCIPLES_TOML: &str = include_str!("../pack/principles.toml");

/// Parse a principles file's TOML source into its principles.
pub fn parse_principles(source: &str) -> Result<Vec<Principle>, toml::de::Error> {
	Ok(toml::from_str::<PrinciplesFile>(source)?.principle)
}

/// The built-in default principles. Panics if the embedded pack is malformed,
/// which is a build-time invariant rather than a runtime condition.
pub fn default_principles() -> Vec<Principle> {
	parse_principles(DEFAULT_PRINCIPLES_TOML).expect("built-in principles.toml must parse")
}

/// Order a set of principle references by their `default_order`.
pub fn ordered_by_default(mut principles: Vec<&Principle>) -> Vec<&Principle> {
	principles.sort_by_key(|p| p.default_order);
	principles
}

/// An error resolving a `--principles` selection.
#[derive(Debug)]
pub enum SelectionError {
	/// A requested id is not present in the pack.
	UnknownId(String),
}

impl std::fmt::Display for SelectionError {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		match self {
			SelectionError::UnknownId(id) => write!(f, "unknown principle id: {id}"),
		}
	}
}

impl std::error::Error for SelectionError {}

/// Resolve a `--principles` selection spec into a principle list.
///
/// `spec` is `default`, `all`, `none`, or a comma-separated list of ids. The
/// `default` and `all` sets come out sorted by `default_order`; an explicit id
/// list preserves the order it was given, so a selection recorded by the
/// interactive selector (which overrides `default_order`) round-trips through
/// `--principles`.
pub fn resolve_selection<'a>(
	principles: &'a [Principle],
	spec: &str,
) -> Result<Vec<&'a Principle>, SelectionError> {
	match spec {
		"default" =>
			Ok(ordered_by_default(principles.iter().filter(|p| p.default_selected).collect())),
		"all" => Ok(ordered_by_default(principles.iter().collect())),
		"none" => Ok(Vec::new()),
		list => {
			let mut out = Vec::new();
			for id in list.split(',').map(str::trim).filter(|s| !s.is_empty()) {
				match principles.iter().find(|p| p.id == id) {
					Some(principle) => out.push(principle),
					None => return Err(SelectionError::UnknownId(id.to_string())),
				}
			}
			Ok(out)
		}
	}
}

/// How much of each principle to render into the output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Detail {
	/// Just the principle name.
	Name,
	/// Name and one-line summary.
	Summary,
	/// Name, rationale, and references.
	Full,
}

/// Render one principle as a numbered list item at the given detail level.
fn render_one(
	index: usize,
	principle: &Principle,
	detail: Detail,
) -> String {
	match detail {
		Detail::Name => format!("{index}. {}", principle.name),
		Detail::Summary => format!("{index}. {} - {}", principle.name, principle.summary),
		Detail::Full => {
			let mut rendered = format!("{index}. {} - {}", principle.name, principle.rationale);
			if !principle.references.is_empty() {
				rendered.push_str("\n   References: ");
				rendered.push_str(&principle.references.join(", "));
			}
			rendered
		}
	}
}

/// Render principles as a numbered list at the given detail level.
pub fn render_principles(
	selected: &[&Principle],
	detail: Detail,
) -> String {
	selected
		.iter()
		.enumerate()
		.map(|(i, principle)| render_one(i + 1, principle, detail))
		.collect::<Vec<_>>()
		.join("\n")
}

/// Render an `AGENTS.md` template, substituting the selected principles for the
/// `{{principles}}` placeholder.
pub fn render_agents(
	template: &str,
	selected: &[&Principle],
	detail: Detail,
) -> String {
	template.replace("{{principles}}", &render_principles(selected, detail))
}

#[cfg(test)]
mod tests {
	use {
		super::*,
		std::collections::HashSet,
	};

	#[test]
	fn embedded_pack_parses() {
		let principles = default_principles();
		assert!(!principles.is_empty());
	}

	#[test]
	fn ids_are_unique() {
		let principles = default_principles();
		let ids: HashSet<&str> = principles.iter().map(|p| p.id.as_str()).collect();
		assert_eq!(ids.len(), principles.len(), "principle ids must be unique");
	}

	#[test]
	fn default_orders_are_unique() {
		let principles = default_principles();
		let orders: HashSet<i64> = principles.iter().map(|p| p.default_order).collect();
		assert_eq!(orders.len(), principles.len(), "default_order values must be unique");
	}

	#[test]
	fn every_principle_has_tags() {
		for p in default_principles() {
			assert!(!p.tags.is_empty(), "{} has no tags", p.id);
		}
	}

	#[test]
	fn related_ids_resolve() {
		let principles = default_principles();
		let ids: HashSet<&str> = principles.iter().map(|p| p.id.as_str()).collect();
		for p in &principles {
			for rel in &p.related {
				assert!(ids.contains(rel.as_str()), "{} relates to unknown {}", p.id, rel);
			}
		}
	}

	#[test]
	fn selection_is_ordered_and_nonempty() {
		let principles = default_principles();
		let selected = resolve_selection(&principles, "default").unwrap();
		assert!(!selected.is_empty());
		let orders: Vec<i64> = selected.iter().map(|p| p.default_order).collect();
		let mut sorted = orders.clone();
		sorted.sort_unstable();
		assert_eq!(orders, sorted, "selection must come out ordered");
	}

	#[test]
	fn rendering_substitutes_and_numbers() {
		let principles = default_principles();
		let selected = resolve_selection(&principles, "default").unwrap();
		let out = render_agents("before\n{{principles}}\nafter", &selected, Detail::Summary);
		assert!(!out.contains("{{principles}}"), "placeholder must be replaced");
		assert!(out.contains("1. ") && out.contains(" - "), "list must be numbered and formatted");
		assert!(out.contains("before") && out.contains("after"), "surrounding text kept");
	}

	#[test]
	fn selection_modes_resolve() {
		let principles = default_principles();
		assert_eq!(resolve_selection(&principles, "all").unwrap().len(), principles.len());
		assert!(resolve_selection(&principles, "none").unwrap().is_empty());
		assert_eq!(
			resolve_selection(&principles, "default").unwrap().len(),
			principles.iter().filter(|p| p.default_selected).count()
		);

		// An explicit id list preserves the given order (not default_order:
		// kiss is 350, after verify-dont-trust at 60), so a reordered selection
		// round-trips through `--principles`.
		let two = resolve_selection(&principles, "kiss, verify-dont-trust").unwrap();
		assert_eq!(two.len(), 2);
		assert_eq!(two[0].id, "kiss");
		assert_eq!(two[1].id, "verify-dont-trust");

		assert!(resolve_selection(&principles, "no-such-id").is_err());
	}
}
