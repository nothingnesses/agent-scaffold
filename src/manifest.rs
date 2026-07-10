//! Pack manifest: the data model and loader that turn a pack directory into the
//! set of assets to drop.
//!
//! A pack is a directory with a `pack.toml` manifest declaring its assets: for
//! each, which source file it comes from, where it lands, whether it is a
//! tool-owned reference asset or a user working file, and whether it is rendered
//! (`{{var}}` substitution) or copied verbatim. The built-in pack ships the same
//! manifest and loads through this one path, so there is no built-in-versus-
//! external special-casing.

use {
	include_dir::{
		Dir,
		include_dir,
	},
	serde::Deserialize,
	std::{
		collections::HashMap,
		fs,
		io,
		path::PathBuf,
	},
};

/// The built-in default pack, embedded at compile time.
static BUILTIN_PACK: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/pack");

/// Whether a scaffolded asset is owned by the tool or by the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Ownership {
	/// Tool-owned reference asset: always (re)written to stay current.
	Reference,
	/// User working file: created only if absent, unless forced.
	Working,
}

/// One `[[asset]]` entry in a pack manifest.
#[derive(Debug, Clone, Deserialize)]
struct AssetSpec {
	/// Path of the source file within the pack.
	source: String,
	/// Destination path relative to the output directory.
	dest: String,
	/// Whether the dropped file is tool-owned or a user working file.
	ownership: Ownership,
	/// Whether to render (`{{var}}` substitution) rather than copy verbatim.
	#[serde(default)]
	render: bool,
}

/// The on-disk shape of a `pack.toml`. Unknown keys are ignored, so a future
/// `[[module]]` section can be added without breaking older loaders.
#[derive(Debug, Deserialize)]
struct Manifest {
	#[serde(default)]
	asset: Vec<AssetSpec>,
}

/// A resolved asset ready to drop: destination, final contents, and ownership.
pub struct Asset {
	/// Destination path relative to the output directory.
	pub dest: String,
	/// Final file contents (rendered when the manifest marked it so).
	pub contents: String,
	/// Whether the dropped file is tool-owned or a user working file.
	pub ownership: Ownership,
}

/// The source of a pack's files: the embedded built-in directory or a directory
/// on the filesystem. Both present the same read-by-relative-path interface, so
/// the loader treats built-in and external packs identically.
pub enum PackSource<'a> {
	/// The built-in pack, embedded at compile time.
	Embedded(&'a Dir<'a>),
	/// An external pack loaded from a filesystem directory.
	// `allow`, not `expect`: the loader test constructs this, so it is live in
	// the test build (where `expect` would be unfulfilled); the binary does not
	// construct it until Step 6b wires up `--template`.
	#[allow(dead_code)]
	Directory(PathBuf),
}

impl PackSource<'_> {
	/// Read a file within the pack by its relative path.
	fn read(
		&self,
		rel: &str,
	) -> io::Result<String> {
		match self {
			PackSource::Embedded(dir) => dir
				.get_file(rel)
				.and_then(|file| file.contents_utf8())
				.map(str::to_owned)
				.ok_or_else(|| {
					io::Error::new(io::ErrorKind::NotFound, format!("pack file not found: {rel}"))
				}),
			PackSource::Directory(root) => fs::read_to_string(root.join(rel)),
		}
	}

	/// Parse the pack's `pack.toml` manifest.
	fn manifest(&self) -> io::Result<Manifest> {
		let source = self.read("pack.toml")?;
		toml::from_str(&source).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
	}
}

/// The built-in default pack as a source.
pub fn builtin() -> PackSource<'static> {
	PackSource::Embedded(&BUILTIN_PACK)
}

/// Substitute `{{key}}` placeholders in `template` with their values in `vars`.
/// Placeholders with no matching variable are left as-is.
pub fn render(
	template: &str,
	vars: &HashMap<String, String>,
) -> String {
	let mut out = template.to_owned();
	for (key, value) in vars {
		out = out.replace(&format!("{{{{{key}}}}}"), value);
	}
	out
}

/// Load a pack from `source`, producing the assets to drop in manifest order.
/// Each asset's content is read from the pack and rendered with `vars` when the
/// manifest marks it `render = true`.
pub fn load(
	source: &PackSource,
	vars: &HashMap<String, String>,
) -> io::Result<Vec<Asset>> {
	let manifest = source.manifest()?;
	manifest
		.asset
		.into_iter()
		.map(|spec| {
			let raw = source.read(&spec.source)?;
			let contents = if spec.render { render(&raw, vars) } else { raw };
			Ok(Asset {
				dest: spec.dest,
				contents,
				ownership: spec.ownership,
			})
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use {
		super::*,
		std::fs,
	};

	/// A unique scratch directory under the system temp dir for one test.
	fn scratch(name: &str) -> PathBuf {
		let dir = std::env::temp_dir().join(format!(
			"agent-scaffold-manifest-{}-{}",
			std::process::id(),
			name
		));
		let _ = fs::remove_dir_all(&dir);
		dir
	}

	#[test]
	fn render_substitutes_known_and_leaves_unknown() {
		let mut vars = HashMap::new();
		vars.insert("name".to_string(), "world".to_string());
		let out = render("hello {{name}}, {{missing}}", &vars);
		assert_eq!(out, "hello world, {{missing}}");
	}

	#[test]
	fn builtin_manifest_lists_the_expected_assets() {
		let assets = load(&builtin(), &HashMap::new()).unwrap();
		let dests: Vec<&str> = assets.iter().map(|a| a.dest.as_str()).collect();
		assert_eq!(
			dests,
			vec![
				"AGENTS.md",
				"docs/plans/TEMPLATE.md",
				".agents/AGENTS.reference.md",
				".agents/prompts/clarifying-questions.md",
				".agents/prompts/open-questions-gate.md",
				".agents/prompts/adversarial-review.md",
				".agents/principles.toml",
			]
		);
	}

	#[test]
	fn builtin_renders_only_the_rendered_assets() {
		let mut vars = HashMap::new();
		vars.insert("principles".to_string(), "SENTINEL-LIST".to_string());
		let assets = load(&builtin(), &vars).unwrap();
		let by_dest = |dest: &str| assets.iter().find(|a| a.dest == dest).unwrap();

		// The two guidance copies are rendered: the placeholder is gone and the
		// substituted value is present.
		for dest in ["AGENTS.md", ".agents/AGENTS.reference.md"] {
			let asset = by_dest(dest);
			assert!(!asset.contents.contains("{{principles}}"), "{dest} should be rendered");
			assert!(asset.contents.contains("SENTINEL-LIST"), "{dest} should carry the value");
			assert_eq!(
				asset.ownership,
				if dest == "AGENTS.md" { Ownership::Working } else { Ownership::Reference }
			);
		}

		// The principles data is copied verbatim (no substitution applied).
		let principles = by_dest(".agents/principles.toml");
		assert_eq!(principles.ownership, Ownership::Reference);
		assert!(!principles.contents.contains("SENTINEL-LIST"));
	}

	#[test]
	fn directory_source_loads_through_the_same_path() {
		// A minimal external pack on the filesystem exercises the Directory
		// source and proves the loader is source-agnostic.
		let root = scratch("dir-source");
		fs::create_dir_all(&root).unwrap();
		fs::write(
			root.join("pack.toml"),
			"[[asset]]\nsource = \"greeting.md\"\ndest = \"out/greeting.md\"\nownership = \"working\"\nrender = true\n",
		)
		.unwrap();
		fs::write(root.join("greeting.md"), "hi {{who}}\n").unwrap();

		let mut vars = HashMap::new();
		vars.insert("who".to_string(), "there".to_string());
		let assets = load(&PackSource::Directory(root.clone()), &vars).unwrap();

		assert_eq!(assets.len(), 1);
		assert_eq!(assets[0].dest, "out/greeting.md");
		assert_eq!(assets[0].ownership, Ownership::Working);
		assert_eq!(assets[0].contents, "hi there\n");
		fs::remove_dir_all(&root).unwrap();
	}
}
