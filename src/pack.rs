//! The built-in default principle pack and its data model.
//!
//! Principles are structured data (see `pack/principles.toml`) so the tool can
//! select a subset, order it, and render it at a chosen verbosity, and so that
//! bring-your-own packs can ship principles in the same shape.

use serde::Deserialize;

/// One principle in a pack.
///
/// Some fields are not read by the binary yet: they are part of the pack schema,
/// are exercised by the tests, and will drive rendering (summary, rationale,
/// references) and filtering (tags) as the tool grows.
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
const DEFAULT_PRINCIPLES_TOML: &str = include_str!("../pack/principles.toml");

/// Parse a principles file's TOML source into its principles.
pub fn parse_principles(source: &str) -> Result<Vec<Principle>, toml::de::Error> {
	Ok(toml::from_str::<PrinciplesFile>(source)?.principle)
}

/// The built-in default principles. Panics if the embedded pack is malformed,
/// which is a build-time invariant rather than a runtime condition.
pub fn default_principles() -> Vec<Principle> {
	parse_principles(DEFAULT_PRINCIPLES_TOML).expect("built-in principles.toml must parse")
}

/// The default-selected principles, ordered by `default_order`.
pub fn selected_ordered(principles: &[Principle]) -> Vec<&Principle> {
	let mut selected: Vec<&Principle> = principles.iter().filter(|p| p.default_selected).collect();
	selected.sort_by_key(|p| p.default_order);
	selected
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
		let selected = selected_ordered(&principles);
		assert!(!selected.is_empty());
		let orders: Vec<i64> = selected.iter().map(|p| p.default_order).collect();
		let mut sorted = orders.clone();
		sorted.sort_unstable();
		assert_eq!(orders, sorted, "selection must come out ordered");
	}
}
