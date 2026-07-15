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
		collections::{
			HashMap,
			HashSet,
		},
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
	/// The optional module this asset belongs to. `None` (an absent field) is a
	/// core asset, always dropped; `Some(name)` is dropped only when that module
	/// is selected with `--module <name>`. The `name` must be declared in a
	/// `[[module]]` section (validated in `load`).
	#[serde(default)]
	module: Option<String>,
}

/// One `[[module]]` entry: an optional module a pack declares. The `[[module]]`
/// section is the authoritative set of known module names, so both a `--module`
/// selection and an asset's `module` tag validate against it (no dangling
/// references). Membership itself is single-sourced on the assets' `module` tag;
/// this section only names each module and describes it.
#[derive(Debug, Clone, Deserialize)]
struct ModuleSpec {
	/// The module name, referenced by `--module <name>` and by an asset's
	/// `module = "<name>"` tag.
	name: String,
	/// A human-readable description of what the module adds.
	#[expect(dead_code, reason = "declared for the schema and TUI; not yet read by the loader")]
	description: String,
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
	/// The modules the pack declares. Defaults to empty, so a pack with no
	/// `[[module]]` section (for example the built-in pack) still parses and is
	/// module-free.
	#[serde(default)]
	module: Vec<ModuleSpec>,
}

/// Variable names the tool computes itself; a pack may neither declare them nor
/// override them with `--var`.
const RESERVED_VARS: &[&str] = &["principles", "instrument"];

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
	/// A `--module` named a module the pack does not declare (a usage error).
	UnknownModule(String),
	/// An `[[asset]]` was tagged with a module the pack does not declare in any
	/// `[[module]]` section (a pack-authoring error).
	UndeclaredAssetModule {
		/// The tagged asset's source path within the pack.
		asset: String,
		/// The undeclared module name the asset referenced.
		module: String,
	},
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
			LoadError::UnknownModule(name) =>
				write!(f, "no module named `{name}` is declared by the pack"),
			LoadError::UndeclaredAssetModule {
				asset,
				module,
			} => write!(
				f,
				"asset `{asset}` is tagged with module `{module}`, which no [[module]] declares"
			),
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
/// Placeholders with no matching variable are left as-is. The result is
/// normalised to end with exactly one trailing newline: a variable that
/// substitutes to empty (for example `{{instrument}}` when `--instrument` is
/// off) would otherwise leave the blank lines that surrounded its placeholder,
/// so the raw output the binary emits, before any external formatter, stays
/// byte-stable regardless of what a variable expands to. Only rendered
/// (`render = true`) assets pass through here; verbatim assets keep their exact
/// bytes.
pub fn render(
	template: &str,
	vars: &HashMap<String, String>,
) -> String {
	let mut out = template.to_owned();
	for (key, value) in vars {
		out = out.replace(&format!("{{{{{key}}}}}"), value);
	}
	format!("{}\n", out.trim_end())
}

/// Load a pack from `source`, producing the assets to drop in manifest order.
/// The substitution map is resolved from the pack's declared variables, the
/// tool-computed `builtin` variables (for example `{{principles}}`), and the
/// `--var` `overrides`; each asset is then read from the pack and rendered with
/// that map when the manifest marks it `render = true`.
///
/// `selected_modules` are the module names passed with `--module`. A core asset
/// (no `module` tag) is always included; a tagged asset is included only when its
/// module is selected. Both a selected module and an asset's tag must name a
/// module the pack declares in a `[[module]]` section: an unknown `--module` is a
/// usage error and a tag with no matching `[[module]]` is a pack-authoring error,
/// and either fails the load so nothing is written (no dangling references).
pub fn load(
	source: &PackSource,
	builtin: &HashMap<String, String>,
	overrides: &HashMap<String, String>,
	selected_modules: &[String],
) -> Result<Vec<Asset>, LoadError> {
	let manifest = source.manifest()?;
	// The authoritative set of module names the pack declares.
	let declared: HashSet<&str> = manifest.module.iter().map(|m| m.name.as_str()).collect();
	// A `--module` naming a module the pack does not declare is a usage error.
	for name in selected_modules {
		if !declared.contains(name.as_str()) {
			return Err(LoadError::UnknownModule(name.clone()));
		}
	}
	// An asset tagged with a module the pack does not declare is a pack-authoring
	// error, checked for every asset regardless of selection.
	for spec in &manifest.asset {
		if let Some(module) = &spec.module {
			if !declared.contains(module.as_str()) {
				return Err(LoadError::UndeclaredAssetModule {
					asset: spec.source.clone(),
					module: module.clone(),
				});
			}
		}
	}
	let selected: HashSet<&str> = selected_modules.iter().map(String::as_str).collect();
	let vars = resolve_vars(&manifest.var, builtin, overrides)?;
	manifest
		.asset
		.into_iter()
		.filter(|spec| match &spec.module {
			// Core assets always load; a module's assets only when it is selected.
			None => true,
			Some(module) => selected.contains(module.as_str()),
		})
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
		// render normalises to a single trailing newline, so the substituted body
		// gains one even though the template had none.
		let out = render("hello {{name}}, {{missing}}", &vars);
		assert_eq!(out, "hello world, {{missing}}\n");
	}

	#[test]
	fn render_normalises_to_a_single_trailing_newline() {
		let vars = HashMap::new();
		// Trailing blank lines (as an empty {{var}} substitution would leave) are
		// collapsed to exactly one newline; a body with none gains one.
		assert_eq!(render("body\n\n\n", &vars), "body\n");
		assert_eq!(render("body", &vars), "body\n");
		assert!(render("body\n\n", &vars).ends_with('\n'));
		assert!(!render("body\n\n", &vars).ends_with("\n\n"));
	}

	#[test]
	fn builtin_manifest_lists_the_expected_assets() {
		let assets = load(&builtin(), &HashMap::new(), &HashMap::new(), &[]).unwrap();
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
		let assets = load(&builtin(), &vars, &HashMap::new(), &[]).unwrap();
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
		let assets =
			load(&PackSource::Directory(root.clone()), &builtin, &HashMap::new(), &[]).unwrap();

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
		let assets = load(&source, &HashMap::new(), &overrides, &[]).unwrap();
		assert_eq!(assets[0].contents, "hi world\n");

		// Overriding the optional variable too.
		overrides.insert("greeting".to_string(), "hey".to_string());
		let assets = load(&source, &HashMap::new(), &overrides, &[]).unwrap();
		assert_eq!(assets[0].contents, "hey world\n");
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn missing_required_variable_errors() {
		let root = var_fixture("var-missing");
		let result =
			load(&PackSource::Directory(root.clone()), &HashMap::new(), &HashMap::new(), &[]);
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
		let result = load(&PackSource::Directory(root.clone()), &HashMap::new(), &overrides, &[]);
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
		match load(&PackSource::Directory(declared.clone()), &HashMap::new(), &HashMap::new(), &[])
		{
			Err(LoadError::ReservedVar(name)) => assert_eq!(name, "principles"),
			other => panic!("expected ReservedVar for declaration, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&declared).unwrap();

		// Nor may `--var` set it.
		let root = var_fixture("var-reserved-override");
		let mut overrides = HashMap::new();
		overrides.insert("who".to_string(), "world".to_string());
		overrides.insert("principles".to_string(), "x".to_string());
		match load(&PackSource::Directory(root.clone()), &HashMap::new(), &overrides, &[]) {
			Err(LoadError::ReservedVar(name)) => assert_eq!(name, "principles"),
			other => panic!("expected ReservedVar for override, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn reserved_instrument_variable_is_rejected() {
		// A pack may not declare the reserved `instrument` variable.
		let declared = fixture_pack(
			"var-reserved-instrument-declared",
			"[[asset]]\nsource = \"a.md\"\ndest = \"a.md\"\nownership = \"working\"\n\n\
			 [[var]]\nname = \"instrument\"\ndefault = \"x\"\n",
			"a.md",
			"a\n",
		);
		match load(&PackSource::Directory(declared.clone()), &HashMap::new(), &HashMap::new(), &[])
		{
			Err(LoadError::ReservedVar(name)) => assert_eq!(name, "instrument"),
			other => panic!("expected ReservedVar for declaration, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&declared).unwrap();

		// Nor may `--var` set it.
		let root = var_fixture("var-reserved-instrument-override");
		let mut overrides = HashMap::new();
		overrides.insert("who".to_string(), "world".to_string());
		overrides.insert("instrument".to_string(), "x".to_string());
		match load(&PackSource::Directory(root.clone()), &HashMap::new(), &overrides, &[]) {
			Err(LoadError::ReservedVar(name)) => assert_eq!(name, "instrument"),
			other => panic!("expected ReservedVar for override, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&root).unwrap();
	}

	/// A filesystem pack fixture with one `[[module]]` named `extras`, one core
	/// (untagged) asset, and one asset tagged `module = "extras"`, used by the
	/// module-filtering tests. The core asset is declared first so a passing test
	/// also pins that manifest order is preserved for the included assets.
	fn module_fixture(name: &str) -> PathBuf {
		let root = scratch(name);
		fs::create_dir_all(&root).unwrap();
		fs::write(
			root.join("pack.toml"),
			"[[module]]\nname = \"extras\"\ndescription = \"extra opt-in assets\"\n\n\
			 [[asset]]\nsource = \"core.md\"\ndest = \"core.md\"\nownership = \"working\"\n\n\
			 [[asset]]\nsource = \"extra.md\"\ndest = \"extra.md\"\nownership = \"working\"\nmodule = \"extras\"\n",
		)
		.unwrap();
		fs::write(root.join("core.md"), "core\n").unwrap();
		fs::write(root.join("extra.md"), "extra\n").unwrap();
		root
	}

	#[test]
	fn a_module_asset_drops_only_when_its_module_is_selected() {
		let root = module_fixture("module-filter");
		let source = PackSource::Directory(root.clone());

		// With no module selected, only the core asset loads; the tagged one is absent.
		let core_only = load(&source, &HashMap::new(), &HashMap::new(), &[]).unwrap();
		let dests: Vec<&str> = core_only.iter().map(|a| a.dest.as_str()).collect();
		assert_eq!(dests, vec!["core.md"]);

		// With the module selected, both load, and manifest order is preserved.
		let both =
			load(&source, &HashMap::new(), &HashMap::new(), &["extras".to_string()]).unwrap();
		let dests: Vec<&str> = both.iter().map(|a| a.dest.as_str()).collect();
		assert_eq!(dests, vec!["core.md", "extra.md"]);

		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn selecting_an_undeclared_module_errors() {
		let root = module_fixture("module-unknown");
		let result = load(
			&PackSource::Directory(root.clone()),
			&HashMap::new(),
			&HashMap::new(),
			&["bogus".to_string()],
		);
		match result {
			Err(LoadError::UnknownModule(name)) => assert_eq!(name, "bogus"),
			other => panic!("expected UnknownModule, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn an_asset_tagged_with_an_undeclared_module_errors() {
		// The pack ships no `[[module]]` at all, so the tagged asset dangles. This
		// is caught even with no module selected: it is a pack-authoring error.
		let root = scratch("module-dangling-tag");
		fs::create_dir_all(&root).unwrap();
		fs::write(
			root.join("pack.toml"),
			"[[asset]]\nsource = \"ghost.md\"\ndest = \"ghost.md\"\nownership = \"working\"\nmodule = \"ghost\"\n",
		)
		.unwrap();
		fs::write(root.join("ghost.md"), "ghost\n").unwrap();
		match load(&PackSource::Directory(root.clone()), &HashMap::new(), &HashMap::new(), &[]) {
			Err(LoadError::UndeclaredAssetModule {
				asset,
				module,
			}) => {
				assert_eq!(asset, "ghost.md");
				assert_eq!(module, "ghost");
			}
			other => panic!("expected UndeclaredAssetModule, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn selecting_a_declared_module_with_no_assets_drops_nothing_extra() {
		// A declared module that tags no asset is a valid selection: it simply adds
		// nothing, so only the core asset loads.
		let root = scratch("module-empty");
		fs::create_dir_all(&root).unwrap();
		fs::write(
			root.join("pack.toml"),
			"[[module]]\nname = \"empty\"\ndescription = \"declares nothing\"\n\n\
			 [[asset]]\nsource = \"core.md\"\ndest = \"core.md\"\nownership = \"working\"\n",
		)
		.unwrap();
		fs::write(root.join("core.md"), "core\n").unwrap();
		let assets = load(
			&PackSource::Directory(root.clone()),
			&HashMap::new(),
			&HashMap::new(),
			&["empty".to_string()],
		)
		.unwrap();
		let dests: Vec<&str> = assets.iter().map(|a| a.dest.as_str()).collect();
		assert_eq!(dests, vec!["core.md"]);
		fs::remove_dir_all(&root).unwrap();
	}
}
