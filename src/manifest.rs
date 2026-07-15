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

/// One `[[var]]` entry: a variable a pack declares for `{{name}}` substitution.
/// A present `default` makes the variable optional; an absent one makes it
/// required, so it must be supplied with `--var`.
#[derive(Debug, Clone, Deserialize)]
struct VarSpec {
	/// The variable name, referenced in assets as `{{name}}`.
	name: String,
	/// The default value, or `None` when the variable is required.
	#[serde(default)]
	default: Option<String>,
}

/// The on-disk shape of a `pack.toml`. Unknown keys are ignored, so a future
/// `[[module]]` section can be added without breaking older loaders.
#[derive(Debug, Deserialize)]
struct Manifest {
	#[serde(default)]
	asset: Vec<AssetSpec>,
	#[serde(default)]
	var: Vec<VarSpec>,
}

/// Variable names the tool computes itself; a pack may neither declare them nor
/// override them with `--var`.
const RESERVED_VARS: &[&str] = &["principles"];

/// An error loading a pack: reading or parsing its files, or resolving the
/// variables its assets substitute.
#[derive(Debug)]
pub enum LoadError {
	/// Reading a pack file or parsing its `pack.toml` failed.
	Io(io::Error),
	/// A `--var` named a variable the pack does not declare.
	UndeclaredVar(String),
	/// A required variable (declared with no default) was not supplied.
	MissingRequiredVar(String),
	/// A reserved variable name was declared by the pack or set with `--var`.
	ReservedVar(String),
}

impl std::fmt::Display for LoadError {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		match self {
			LoadError::Io(error) => write!(f, "{error}"),
			LoadError::UndeclaredVar(name) =>
				write!(f, "no variable named `{name}` is declared by the pack"),
			LoadError::MissingRequiredVar(name) =>
				write!(f, "required variable `{name}` was not supplied (use --var {name}=...)"),
			LoadError::ReservedVar(name) =>
				write!(f, "`{name}` is a reserved variable and cannot be declared or set"),
		}
	}
}

impl std::error::Error for LoadError {}

impl From<io::Error> for LoadError {
	fn from(error: io::Error) -> Self {
		LoadError::Io(error)
	}
}

/// Resolve the final substitution map from the pack's declared variables, the
/// tool-computed built-in variables, and the `--var` overrides. Applies the
/// variable rules: no override may name an undeclared or reserved variable, no
/// pack may declare a reserved variable, and every required variable must be
/// supplied by an override.
fn resolve_vars(
	specs: &[VarSpec],
	builtin: &HashMap<String, String>,
	overrides: &HashMap<String, String>,
) -> Result<HashMap<String, String>, LoadError> {
	for spec in specs {
		if RESERVED_VARS.contains(&spec.name.as_str()) {
			return Err(LoadError::ReservedVar(spec.name.clone()));
		}
	}
	for key in overrides.keys() {
		if RESERVED_VARS.contains(&key.as_str()) {
			return Err(LoadError::ReservedVar(key.clone()));
		}
		if !specs.iter().any(|spec| &spec.name == key) {
			return Err(LoadError::UndeclaredVar(key.clone()));
		}
	}

	let mut resolved = builtin.clone();
	for spec in specs {
		let value = match overrides.get(&spec.name) {
			Some(value) => value.clone(),
			None => match &spec.default {
				Some(default) => default.clone(),
				None => return Err(LoadError::MissingRequiredVar(spec.name.clone())),
			},
		};
		resolved.insert(spec.name.clone(), value);
	}
	Ok(resolved)
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
	Directory(PathBuf),
}

impl PackSource<'_> {
	/// Read a file within the pack by its relative path. Public so callers can
	/// read pack files the manifest does not itself resolve, such as the pack's
	/// `principles.toml`.
	pub fn read(
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
/// The substitution map is resolved from the pack's declared variables, the
/// tool-computed `builtin` variables (for example `{{principles}}`), and the
/// `--var` `overrides`; each asset is then read from the pack and rendered with
/// that map when the manifest marks it `render = true`.
pub fn load(
	source: &PackSource,
	builtin: &HashMap<String, String>,
	overrides: &HashMap<String, String>,
) -> Result<Vec<Asset>, LoadError> {
	let manifest = source.manifest()?;
	let vars = resolve_vars(&manifest.var, builtin, overrides)?;
	manifest
		.asset
		.into_iter()
		.map(|spec| {
			let raw = source.read(&spec.source)?;
			let contents = if spec.render { render(&raw, &vars) } else { raw };
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
		let assets = load(&builtin(), &HashMap::new(), &HashMap::new()).unwrap();
		let dests: Vec<&str> = assets.iter().map(|a| a.dest.as_str()).collect();
		assert_eq!(
			dests,
			vec![
				"AGENTS.md",
				"docs/plans/TEMPLATE.md",
				".agents/AGENTS.reference.md",
				".agents/prompts/orchestrator.md",
				".agents/prompts/planner.md",
				".agents/prompts/clarifying-questions.md",
				".agents/prompts/open-questions-gate.md",
				".agents/prompts/reviewer.md",
				".agents/prompts/triager.md",
				".agents/prompts/implementer.md",
				".agents/principles.toml",
				".agents/LEDGER.template.md",
				".agents/user-prompts/kickoff.md",
				".agents/user-prompts/compaction-prep.md",
				".agents/user-prompts/resume.md",
			]
		);
	}

	#[test]
	fn builtin_renders_only_the_rendered_assets() {
		let mut vars = HashMap::new();
		vars.insert("principles".to_string(), "SENTINEL-LIST".to_string());
		let assets = load(&builtin(), &vars, &HashMap::new()).unwrap();
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

	/// Write a filesystem pack fixture (a `pack.toml` plus one source file) and
	/// return its `Directory` source root.
	fn fixture_pack(
		name: &str,
		pack_toml: &str,
		source_name: &str,
		source_body: &str,
	) -> PathBuf {
		let root = scratch(name);
		fs::create_dir_all(&root).unwrap();
		fs::write(root.join("pack.toml"), pack_toml).unwrap();
		fs::write(root.join(source_name), source_body).unwrap();
		root
	}

	#[test]
	fn directory_source_loads_through_the_same_path() {
		// A minimal external pack on the filesystem exercises the Directory
		// source and proves the loader is source-agnostic.
		let root = fixture_pack(
			"dir-source",
			"[[asset]]\nsource = \"greeting.md\"\ndest = \"out/greeting.md\"\nownership = \"working\"\nrender = true\n",
			"greeting.md",
			"hi {{who}}\n",
		);
		// `who` supplied as a tool-side built-in variable to keep this test to the
		// source-reading path; declared-variable resolution is covered below.
		let mut builtin = HashMap::new();
		builtin.insert("who".to_string(), "there".to_string());
		let assets = load(&PackSource::Directory(root.clone()), &builtin, &HashMap::new()).unwrap();

		assert_eq!(assets.len(), 1);
		assert_eq!(assets[0].dest, "out/greeting.md");
		assert_eq!(assets[0].ownership, Ownership::Working);
		assert_eq!(assets[0].contents, "hi there\n");
		fs::remove_dir_all(&root).unwrap();
	}

	/// A pack declaring one optional variable (`greeting`, with a default) and
	/// one required variable (`who`, no default), used by the variable tests.
	fn var_fixture(name: &str) -> PathBuf {
		fixture_pack(
			name,
			"[[asset]]\nsource = \"msg.md\"\ndest = \"msg.md\"\nownership = \"working\"\nrender = true\n\n\
			 [[var]]\nname = \"greeting\"\ndefault = \"hi\"\n\n\
			 [[var]]\nname = \"who\"\n",
			"msg.md",
			"{{greeting}} {{who}}\n",
		)
	}

	#[test]
	fn declared_default_applies_and_override_wins() {
		let root = var_fixture("var-default");
		let source = PackSource::Directory(root.clone());
		let mut overrides = HashMap::new();
		overrides.insert("who".to_string(), "world".to_string());

		// `greeting` falls back to its default; `who` takes the override.
		let assets = load(&source, &HashMap::new(), &overrides).unwrap();
		assert_eq!(assets[0].contents, "hi world\n");

		// Overriding the optional variable too.
		overrides.insert("greeting".to_string(), "hey".to_string());
		let assets = load(&source, &HashMap::new(), &overrides).unwrap();
		assert_eq!(assets[0].contents, "hey world\n");
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn missing_required_variable_errors() {
		let root = var_fixture("var-missing");
		let result = load(&PackSource::Directory(root.clone()), &HashMap::new(), &HashMap::new());
		match result {
			Err(LoadError::MissingRequiredVar(name)) => assert_eq!(name, "who"),
			other => panic!("expected MissingRequiredVar, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn undeclared_override_errors() {
		let root = var_fixture("var-undeclared");
		let mut overrides = HashMap::new();
		overrides.insert("who".to_string(), "world".to_string());
		overrides.insert("nope".to_string(), "x".to_string());
		let result = load(&PackSource::Directory(root.clone()), &HashMap::new(), &overrides);
		match result {
			Err(LoadError::UndeclaredVar(name)) => assert_eq!(name, "nope"),
			other => panic!("expected UndeclaredVar, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn reserved_variable_is_rejected() {
		// A pack may not declare the reserved `principles` variable.
		let declared = fixture_pack(
			"var-reserved-declared",
			"[[asset]]\nsource = \"a.md\"\ndest = \"a.md\"\nownership = \"working\"\n\n\
			 [[var]]\nname = \"principles\"\ndefault = \"x\"\n",
			"a.md",
			"a\n",
		);
		match load(&PackSource::Directory(declared.clone()), &HashMap::new(), &HashMap::new()) {
			Err(LoadError::ReservedVar(name)) => assert_eq!(name, "principles"),
			other => panic!("expected ReservedVar for declaration, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&declared).unwrap();

		// Nor may `--var` set it.
		let root = var_fixture("var-reserved-override");
		let mut overrides = HashMap::new();
		overrides.insert("who".to_string(), "world".to_string());
		overrides.insert("principles".to_string(), "x".to_string());
		match load(&PackSource::Directory(root.clone()), &HashMap::new(), &overrides) {
			Err(LoadError::ReservedVar(name)) => assert_eq!(name, "principles"),
			other => panic!("expected ReservedVar for override, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&root).unwrap();
	}
}
