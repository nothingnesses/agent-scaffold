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
#[derive(Debug, Clone, Deserialize)]
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
	/// Optional related principle ids. Part of the pack schema and exercised by
	/// the tests, but not yet read by the binary.
	#[serde(default)]
	// `allow`, not `expect`: the field is read in the test build, so `expect`
	// would be unfulfilled there.
	#[allow(dead_code)]
	pub related: Vec<String>,
}

/// The on-disk shape of a principles file: an array of `[[principle]]` tables.
#[derive(Debug, Deserialize)]
struct PrinciplesFile {
	#[serde(default)]
	principle: Vec<Principle>,
}

/// Parse a principles file's TOML source into its principles.
pub fn parse_principles(source: &str) -> Result<Vec<Principle>, toml::de::Error> {
	Ok(toml::from_str::<PrinciplesFile>(source)?.principle)
}

/// The embedded source of the built-in default pack, for tests that need the
/// built-in principle set directly. Production code reads `principles.toml`
/// through the active `PackSource` instead (see `main`), so this is test-only.
#[cfg(test)]
pub const DEFAULT_PRINCIPLES_TOML: &str = include_str!("../pack/principles.toml");

/// The built-in default principles, for tests. Panics if the embedded pack is
/// malformed, which is a build-time invariant rather than a runtime condition.
#[cfg(test)]
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
	/// A `tag:<t>` token names a tag that no principle carries.
	UnknownTag(String),
}

impl std::fmt::Display for SelectionError {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		match self {
			SelectionError::UnknownId(id) => write!(f, "unknown principle id: {id}"),
			SelectionError::UnknownTag(tag) => write!(f, "unknown tag: {tag}"),
		}
	}
}

impl std::error::Error for SelectionError {}

/// Resolve a `--principles` selection spec into a principle list.
///
/// `spec` is a comma-separated list of tokens, each one of:
///
/// - `default` / `all` / `none`: the default-selected set, every principle, or
///   nothing (the first two ordered by `default_order`).
/// - `tag:<t>`: every principle carrying tag `<t>`, ordered by `default_order`;
///   a tag no principle carries is a `SelectionError::UnknownTag`.
/// - a bare id: that one principle; an id not in the pack is a
///   `SelectionError::UnknownId`.
///
/// Tokens are concatenated in the order given and de-duplicated by first
/// occurrence, so a bare id list preserves its order (a selection recorded by
/// the interactive selector round-trips through `--principles`) while keyword
/// and tag tokens contribute their `default_order` expansions.
pub fn resolve_selection<'a>(
	principles: &'a [Principle],
	spec: &str,
) -> Result<Vec<&'a Principle>, SelectionError> {
	use std::collections::HashSet;

	let mut out: Vec<&Principle> = Vec::new();
	let mut seen: HashSet<&str> = HashSet::new();

	for token in spec.split(',').map(str::trim).filter(|s| !s.is_empty()) {
		let expansion: Vec<&Principle> = match token {
			"default" =>
				ordered_by_default(principles.iter().filter(|p| p.default_selected).collect()),
			"all" => ordered_by_default(principles.iter().collect()),
			"none" => Vec::new(),
			_ =>
				if let Some(tag) = token.strip_prefix("tag:") {
					let matches = ordered_by_default(
						principles.iter().filter(|p| p.tags.iter().any(|t| t == tag)).collect(),
					);
					if matches.is_empty() {
						return Err(SelectionError::UnknownTag(tag.to_string()));
					}
					matches
				} else {
					match principles.iter().find(|p| p.id == token) {
						Some(principle) => vec![principle],
						None => return Err(SelectionError::UnknownId(token.to_string())),
					}
				},
		};
		for principle in expansion {
			if seen.insert(principle.id.as_str()) {
				out.push(principle);
			}
		}
	}

	Ok(out)
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

/// Render principles as a numbered list at the given detail level. This is the
/// value substituted for the `{{principles}}` variable when a pack asset is
/// rendered (see the `manifest` module).
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
	fn rendering_numbers_and_formats() {
		let principles = default_principles();
		let selected = resolve_selection(&principles, "default").unwrap();
		let out = render_principles(&selected, Detail::Summary);
		assert!(out.contains("1. ") && out.contains(" - "), "list must be numbered and formatted");
		// Each principle is on its own line, so the count of lines matches.
		assert_eq!(out.lines().count(), selected.len());
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

	/// The ids of the pack's principles carrying `tag`, in `default_order`.
	fn ids_with_tag(
		principles: &[Principle],
		tag: &str,
	) -> Vec<String> {
		ordered_by_default(principles.iter().filter(|p| p.tags.iter().any(|t| t == tag)).collect())
			.iter()
			.map(|p| p.id.clone())
			.collect()
	}

	#[test]
	fn tag_token_expands_in_default_order() {
		let principles = default_principles();
		let resolved: Vec<String> = resolve_selection(&principles, "tag:fp")
			.unwrap()
			.iter()
			.map(|p| p.id.clone())
			.collect();
		let expected = ids_with_tag(&principles, "fp");
		assert!(!expected.is_empty(), "the pack must carry fp-tagged principles for this test");
		assert_eq!(resolved, expected);
	}

	#[test]
	fn keywords_ids_and_tags_compose_with_dedup() {
		let principles = default_principles();
		let resolved = resolve_selection(&principles, "default,tag:fp").unwrap();
		let ids: Vec<&str> = resolved.iter().map(|p| p.id.as_str()).collect();

		// No duplicates across the concatenated expansions.
		let unique: HashSet<&str> = ids.iter().copied().collect();
		assert_eq!(unique.len(), ids.len(), "the result must be de-duplicated");

		// The default set comes first (first occurrence wins), in default_order.
		let default_ids: Vec<&str> =
			ordered_by_default(principles.iter().filter(|p| p.default_selected).collect())
				.iter()
				.map(|p| p.id.as_str())
				.collect();
		assert_eq!(&ids[.. default_ids.len()], default_ids.as_slice());

		// Every fp-tagged principle is present.
		for id in ids_with_tag(&principles, "fp") {
			assert!(unique.contains(id.as_str()), "{id} should be included");
		}
	}

	#[test]
	fn unknown_tag_is_an_error() {
		let principles = default_principles();
		match resolve_selection(&principles, "tag:no-such-tag") {
			Err(SelectionError::UnknownTag(tag)) => assert_eq!(tag, "no-such-tag"),
			other => panic!("expected UnknownTag, got {other:?}"),
		}
	}

	#[test]
	fn no_id_collides_with_a_reserved_selection_token() {
		for p in default_principles() {
			assert!(
				!matches!(p.id.as_str(), "default" | "all" | "none"),
				"id {} collides with a keyword token",
				p.id
			);
			assert!(!p.id.starts_with("tag:"), "id {} collides with the tag prefix", p.id);
		}
	}
}
