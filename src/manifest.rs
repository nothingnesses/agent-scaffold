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
	/// Whether the dropped file is made executable (mode bit set). Defaults to
	/// `false`; set for scripts that must run directly, such as the checks module's
	/// `.agents/hooks/pre-commit`. Ignored on non-Unix platforms.
	#[serde(default)]
	executable: bool,
	/// The optional module this asset belongs to. `None` (an absent field) is a
	/// core asset, always dropped; `Some(name)` is dropped only when that module
	/// is enabled (selected directly with `--module <name>`, or pulled in
	/// transitively by another module's `requires`). The `name` must be declared in
	/// a `[[module]]` section (validated in `load`).
	#[serde(default)]
	module: Option<String>,
}

/// One `[[module]]` entry: an optional module a pack declares. The `[[module]]`
/// section is the authoritative set of known module names, so both a `--module`
/// selection and an asset's `module` tag validate against it (no dangling
/// references). Membership itself is single-sourced on the assets' `module` tag;
/// this section names each module, describes it, and carries its `guidance`
/// partial (a fragment filename governing what the module renders into
/// `{{modules}}`) and its `requires` list (the modules it auto-enables).
#[derive(Debug, Clone, Deserialize)]
struct ModuleSpec {
	/// The module name, referenced by `--module <name>` and by an asset's
	/// `module = "<name>"` tag.
	name: String,
	/// A human-readable description of what the module adds.
	#[expect(dead_code, reason = "declared for the schema and TUI; not yet read by the loader")]
	description: String,
	/// The optional guidance partial this module contributes to the `{{modules}}`
	/// render slot: a fragment filename in the pack (read via the pack source). When
	/// the module is enabled its partial is concatenated into `{{modules}}`; `None`
	/// (an absent field) contributes nothing. Only enabled modules contribute.
	/// Declaring `guidance = "file.md"` makes that file required: if an enabled
	/// module names a partial the pack does not ship, the load fails. This is unlike
	/// the tool-computed `instrument.md`, which is silently optional; a declared
	/// guidance file is not.
	#[serde(default)]
	guidance: Option<String>,
	/// The modules this module auto-enables (transitively) when it is selected. A
	/// name here must be declared in a `[[module]]` section (validated in
	/// `expand_modules`);
	/// selecting this module enables everything it `requires` as well, so a module
	/// can depend on another without the user naming both. Defaults to empty (no
	/// dependencies). A `requires` cycle is tolerated (the expansion is a fixed
	/// point, not a recursion), so a pack that declares one still loads.
	#[serde(default)]
	requires: Vec<String>,
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
	/// The optional module this variable belongs to. `None` (an absent field) is a
	/// core variable, always resolved; `Some(name)` participates only when that
	/// module is enabled (selected directly with `--module <name>`, or pulled in
	/// transitively by another module's `requires`), and is skipped entirely
	/// otherwise (not required, not defaulted, absent from the substitution map).
	/// The `name` must be declared in a `[[module]]` section (validated in `load`).
	#[serde(default)]
	module: Option<String>,
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
const RESERVED_VARS: &[&str] = &["principles", "instrument", "modules"];

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
	/// Two `[[module]]` sections declared the same name (a pack-authoring error).
	DuplicateModule(String),
	/// An `[[asset]]` or `[[var]]` was tagged with a module the pack does not
	/// declare in any `[[module]]` section (a pack-authoring error). Shared across
	/// both entry kinds so one validation path covers every module tag.
	UndeclaredModuleTag {
		/// The kind of entry carrying the dangling tag: `"asset"` or `"var"`.
		kind: &'static str,
		/// The tagged entry's identifier: an asset's source path or a variable's
		/// name.
		entry: String,
		/// The undeclared module name the entry referenced.
		module: String,
	},
	/// A `[[module]]`'s `requires` named a module the pack does not declare in any
	/// `[[module]]` section (a pack-authoring error). Distinct from
	/// `UndeclaredModuleTag` because the reference is between two modules, not from
	/// a tagged asset or variable, so its message names the requiring module.
	UndeclaredModuleRequire {
		/// The module whose `requires` names the missing dependency.
		module: String,
		/// The undeclared module name it required.
		requires: String,
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
			LoadError::DuplicateModule(name) =>
				write!(f, "module `{name}` is declared by more than one [[module]] section"),
			LoadError::UndeclaredModuleTag {
				kind,
				entry,
				module,
			} => write!(
				f,
				"{kind} `{entry}` is tagged with module `{module}`, which no [[module]] declares"
			),
			LoadError::UndeclaredModuleRequire {
				module,
				requires,
			} => write!(f, "module `{module}` requires `{requires}`, which no [[module]] declares"),
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
	/// Whether the dropped file is made executable (mode bit set on Unix).
	pub executable: bool,
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

/// Resolve the set of ENABLED modules from the pack's `[[module]]` declarations
/// and the `selected` names (`--module`): the transitive closure of `selected`
/// under each module's `requires`. Validates the module references shared by
/// every module-aware path: a `[[module]]` name declared twice is a
/// pack-authoring error, a `selected` name the pack does not declare is a usage
/// error, and a `requires` naming a module no `[[module]]` declares is a
/// pack-authoring error. The expansion is a fixed point over a visited set, so a
/// `requires` cycle terminates rather than looping forever. Both asset/variable
/// filtering in `load` and the guidance concatenation in `module_guidance` call
/// this function, so the enable-and-validate logic lives in one place. It is the
/// single source of the algorithm, not of a shared result: each caller computes
/// its own set, but from this one implementation over the same inputs, so the
/// two cannot disagree on which modules are on.
fn expand_modules(
	modules: &[ModuleSpec],
	selected: &[String],
) -> Result<HashSet<String>, LoadError> {
	// The authoritative set of declared module names. A name declared by two
	// `[[module]]` sections is a pack-authoring error rather than a silent dedupe.
	let mut declared: HashSet<&str> = HashSet::new();
	for module in modules {
		if !declared.insert(module.name.as_str()) {
			return Err(LoadError::DuplicateModule(module.name.clone()));
		}
	}
	// A `--module` naming a module the pack does not declare is a usage error.
	for name in selected {
		if !declared.contains(name.as_str()) {
			return Err(LoadError::UnknownModule(name.clone()));
		}
	}
	// A `requires` naming a module no `[[module]]` declares is a pack-authoring
	// error, checked for every module regardless of selection.
	for module in modules {
		for req in &module.requires {
			if !declared.contains(req.as_str()) {
				return Err(LoadError::UndeclaredModuleRequire {
					module: module.name.clone(),
					requires: req.clone(),
				});
			}
		}
	}
	// Transitive closure over `requires`, guarded by the visited set (`enabled`):
	// each module is expanded once, so a `requires` cycle cannot loop forever.
	let by_name: HashMap<&str, &ModuleSpec> =
		modules.iter().map(|module| (module.name.as_str(), module)).collect();
	let mut enabled: HashSet<String> = HashSet::new();
	let mut pending: Vec<String> = selected.to_vec();
	while let Some(name) = pending.pop() {
		if enabled.insert(name.clone()) {
			if let Some(module) = by_name.get(name.as_str()) {
				for req in &module.requires {
					if !enabled.contains(req) {
						pending.push(req.clone());
					}
				}
			}
		}
	}
	Ok(enabled)
}

/// Compute the `{{modules}}` render block: the guidance partials of the ENABLED
/// modules (the transitive closure of `selected` under `requires`, see
/// `expand_modules`), concatenated in `[[module]]` declaration order. Each
/// enabled module that declares a `guidance` partial contributes that partial
/// (read from the pack source, like `instrument.md`); a module with no `guidance`
/// contributes nothing, and with no module enabled the block is empty. Each
/// partial has its trailing whitespace trimmed and is separated by a blank line;
/// the whole block is substituted into `{{modules}}` and the asset's `render`
/// then normalises the trailing newline, so an empty block leaves the output
/// byte-identical (the built-in pack declares no modules, so its `{{modules}}` is
/// always empty).
/// Validates the same module references `load` does, so a bad pack or `--module`
/// fails here too.
pub fn module_guidance(
	source: &PackSource,
	selected: &[String],
) -> Result<String, LoadError> {
	let manifest = source.manifest()?;
	let enabled = expand_modules(&manifest.module, selected)?;
	let mut block = String::new();
	for module in &manifest.module {
		if enabled.contains(module.name.as_str()) {
			if let Some(guidance) = &module.guidance {
				// A declared guidance file is required: name the module and the
				// missing file so a bare `No such file or directory` is not the only
				// clue. This wraps only this call site; `PackSource::read` is
				// unchanged for every other caller.
				let partial = source.read(guidance).map_err(|error| {
					LoadError::Io(io::Error::new(
						error.kind(),
						format!(
							"module `{}` guidance file `{guidance}` could not be read: {error}",
							module.name
						),
					))
				})?;
				block.push_str(partial.trim_end());
				block.push_str("\n\n");
			}
		}
	}
	Ok(block)
}

/// Load a pack from `source`, producing the assets to drop in manifest order.
/// The substitution map is resolved from the pack's declared variables, the
/// tool-computed `builtin` variables (for example `{{principles}}`), and the
/// `--var` `overrides`; each asset is then read from the pack and rendered with
/// that map when the manifest marks it `render = true`.
///
/// `selected_modules` are the module names passed with `--module`; the enabled
/// set is their transitive closure under each module's `requires` (a selected
/// module auto-enables its dependencies, see `expand_modules`). A core asset or
/// variable (no `module` tag) is always included; a tagged one participates only
/// when its module is enabled: an unenabled module's assets are dropped and its
/// variables are skipped entirely (not required, not defaulted, absent from the
/// substitution map, so a `--var` naming one is an `UndeclaredVar` error just as
/// for a variable the pack never declared). Both a selected module and an entry's
/// tag must name a module the pack declares in a `[[module]]` section: an unknown
/// `--module` is a usage error, a tag with no matching `[[module]]` is a
/// pack-authoring error, a `requires` naming an undeclared module is a
/// pack-authoring error, and a `[[module]]` name declared twice is a pack-authoring
/// error; any of these fails the load so nothing is written (no dangling or
/// ambiguous references).
pub fn load(
	source: &PackSource,
	builtin: &HashMap<String, String>,
	overrides: &HashMap<String, String>,
	selected_modules: &[String],
) -> Result<Vec<Asset>, LoadError> {
	let manifest = source.manifest()?;
	// The enabled module set: `--module` selections closed transitively under each
	// module's `requires`. This also validates the duplicate/unknown/dangling-
	// requires module references (see `expand_modules`), so those errors fire
	// before any asset is read.
	let enabled = expand_modules(&manifest.module, selected_modules)?;
	// The declared module names, for the asset/variable tag checks below.
	// `expand_modules` already rejected a duplicate declaration, so a plain set of
	// names is enough here.
	let declared: HashSet<&str> = manifest.module.iter().map(|m| m.name.as_str()).collect();
	// An asset or variable tagged with a module the pack does not declare is a
	// pack-authoring error, checked for every entry regardless of selection.
	for spec in &manifest.asset {
		if let Some(module) = &spec.module {
			if !declared.contains(module.as_str()) {
				return Err(LoadError::UndeclaredModuleTag {
					kind: "asset",
					entry: spec.source.clone(),
					module: module.clone(),
				});
			}
		}
	}
	for spec in &manifest.var {
		if let Some(module) = &spec.module {
			if !declared.contains(module.as_str()) {
				return Err(LoadError::UndeclaredModuleTag {
					kind: "var",
					entry: spec.name.clone(),
					module: module.clone(),
				});
			}
		}
	}
	// Only core variables and those whose module is enabled participate in
	// resolution; a variable tagged with an unenabled module is skipped entirely,
	// so its requirement never fires and a stray `--var` for it is undeclared.
	let active_vars: Vec<VarSpec> = manifest
		.var
		.iter()
		.filter(|spec| match &spec.module {
			None => true,
			Some(module) => enabled.contains(module.as_str()),
		})
		.cloned()
		.collect();
	let vars = resolve_vars(&active_vars, builtin, overrides)?;
	manifest
		.asset
		.into_iter()
		.filter(|spec| match &spec.module {
			// Core assets always load; a module's assets only when it is enabled.
			None => true,
			Some(module) => enabled.contains(module.as_str()),
		})
		.map(|spec| {
			let raw = source.read(&spec.source)?;
			let contents = if spec.render { render(&raw, &vars) } else { raw };
			Ok(Asset {
				dest: spec.dest,
				contents,
				ownership: spec.ownership,
				executable: spec.executable,
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
				".agents/user-prompts/explore.md",
				".agents/user-prompts/review.md",
				".agents/user-prompts/pause.md",
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

	#[test]
	fn builtin_checks_module_adds_its_five_assets() {
		// The built-in `checks` module tags five assets. With no module selected they
		// are filtered out (the default list stays byte-identical, pinned by
		// `builtin_manifest_lists_the_expected_assets`).
		let core = load(&builtin(), &HashMap::new(), &HashMap::new(), &[]).unwrap();
		let core_dests: Vec<&str> = core.iter().map(|a| a.dest.as_str()).collect();
		for absent in [
			".agents/checks.toml",
			".agents/prompts/checks-reviewer.md",
			".agents/checks/ast-grep/sgconfig.yml",
			".agents/checks/ast-grep/rules/no-dbg-macro.yml",
			".agents/hooks/pre-commit",
		] {
			assert!(!core_dests.contains(&absent), "{absent} stays out when the module is off");
		}

		// `--module checks` drops exactly the five checks assets on top of the core
		// set, each with its declared ownership.
		let with_checks =
			load(&builtin(), &HashMap::new(), &HashMap::new(), &["checks".to_string()]).unwrap();
		let asset = |dest: &str| {
			with_checks
				.iter()
				.find(|a| a.dest == dest)
				.unwrap_or_else(|| panic!("{dest} drops when the module is selected"))
		};
		// The config, the ast-grep scaffold, and the example rule are user working
		// files; the checks-reviewer role prompt and the pre-commit hook are tool-owned
		// reference assets.
		assert_eq!(asset(".agents/checks.toml").ownership, Ownership::Working);
		assert_eq!(asset(".agents/checks/ast-grep/sgconfig.yml").ownership, Ownership::Working);
		assert_eq!(
			asset(".agents/checks/ast-grep/rules/no-dbg-macro.yml").ownership,
			Ownership::Working
		);
		assert_eq!(asset(".agents/prompts/checks-reviewer.md").ownership, Ownership::Reference);
		assert_eq!(asset(".agents/hooks/pre-commit").ownership, Ownership::Reference);
		// The hook is the one executable asset; nothing else is.
		assert!(asset(".agents/hooks/pre-commit").executable, "the hook drops executable");
		assert!(!asset(".agents/checks.toml").executable, "the config is not executable");
		// Exactly the five module assets are added, nothing else.
		assert_eq!(with_checks.len(), core.len() + 5);
	}

	#[test]
	fn builtin_isolation_module_renders_its_guidance_only_when_selected() {
		// The built-in `isolation` module is guidance-only: it drops zero assets, so
		// the asset list is byte-identical whether or not it is selected (the same
		// list `builtin_manifest_lists_the_expected_assets` pins).
		let core = load(&builtin(), &HashMap::new(), &HashMap::new(), &[]).unwrap();
		let with_isolation =
			load(&builtin(), &HashMap::new(), &HashMap::new(), &["isolation".to_string()]).unwrap();
		let core_dests: Vec<&str> = core.iter().map(|a| a.dest.as_str()).collect();
		let isolation_dests: Vec<&str> = with_isolation.iter().map(|a| a.dest.as_str()).collect();
		assert_eq!(isolation_dests, core_dests, "the isolation module drops no assets");

		// With no module selected the `{{modules}}` block is empty, so the scaffold
		// stays byte-identical to core.
		assert_eq!(module_guidance(&builtin(), &[]).unwrap(), "");

		// Selecting `isolation` renders its guidance partial into the `{{modules}}`
		// block: its heading and the agent-box/agent-images pointers appear.
		let guidance = module_guidance(&builtin(), &["isolation".to_string()]).unwrap();
		assert!(
			guidance.contains("## Writer isolation via agent-box and agent-images"),
			"the isolation guidance heading should render"
		);
		assert!(
			guidance.contains("github.com/0xferrous/agent-box"),
			"the agent-box pointer should render"
		);
		assert!(
			guidance.contains("github.com/nothingnesses/agent-images"),
			"the agent-images pointer should render"
		);
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
			Err(LoadError::UndeclaredModuleTag {
				kind,
				entry,
				module,
			}) => {
				assert_eq!(kind, "asset");
				assert_eq!(entry, "ghost.md");
				assert_eq!(module, "ghost");
			}
			other => panic!("expected UndeclaredModuleTag, got {:?}", other.map(|_| ())),
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

	/// A pack with a `[[module]]` named `extras`, a core asset that renders a core
	/// variable, and a REQUIRED variable (`extra`, no default) tagged to `extras`
	/// whose value renders into a `module`-tagged asset. The required tagged
	/// variable lets the tests prove it is only demanded when its module is
	/// selected.
	fn module_var_fixture(name: &str) -> PathBuf {
		let root = scratch(name);
		fs::create_dir_all(&root).unwrap();
		fs::write(
			root.join("pack.toml"),
			"[[module]]\nname = \"extras\"\ndescription = \"opt-in extras\"\n\n\
			 [[asset]]\nsource = \"core.md\"\ndest = \"core.md\"\nownership = \"working\"\nrender = true\n\n\
			 [[asset]]\nsource = \"extra.md\"\ndest = \"extra.md\"\nownership = \"working\"\nrender = true\nmodule = \"extras\"\n\n\
			 [[var]]\nname = \"who\"\ndefault = \"world\"\n\n\
			 [[var]]\nname = \"extra\"\nmodule = \"extras\"\n",
		)
		.unwrap();
		fs::write(root.join("core.md"), "hi {{who}}\n").unwrap();
		fs::write(root.join("extra.md"), "extra {{extra}}\n").unwrap();
		root
	}

	#[test]
	fn a_module_tagged_var_is_not_required_when_its_module_is_unselected() {
		let root = module_var_fixture("module-var-unselected");
		let source = PackSource::Directory(root.clone());

		// No module selected: the required `extra` var is skipped, so the load
		// succeeds without it and only the core asset drops.
		let assets = load(&source, &HashMap::new(), &HashMap::new(), &[]).unwrap();
		let dests: Vec<&str> = assets.iter().map(|a| a.dest.as_str()).collect();
		assert_eq!(dests, vec!["core.md"]);
		assert_eq!(assets[0].contents, "hi world\n");

		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn a_module_tagged_var_is_required_and_resolved_when_its_module_is_selected() {
		let root = module_var_fixture("module-var-selected");
		let source = PackSource::Directory(root.clone());

		// Module selected but the required var unsupplied: the load fails.
		match load(&source, &HashMap::new(), &HashMap::new(), &["extras".to_string()]) {
			Err(LoadError::MissingRequiredVar(name)) => assert_eq!(name, "extra"),
			other => panic!("expected MissingRequiredVar, got {:?}", other.map(|_| ())),
		}

		// Module selected and the var supplied: both assets drop and the tagged
		// var renders into the tagged asset.
		let mut overrides = HashMap::new();
		overrides.insert("extra".to_string(), "value".to_string());
		let assets = load(&source, &HashMap::new(), &overrides, &["extras".to_string()]).unwrap();
		let dests: Vec<&str> = assets.iter().map(|a| a.dest.as_str()).collect();
		assert_eq!(dests, vec!["core.md", "extra.md"]);
		assert_eq!(assets[1].contents, "extra value\n");

		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn overriding_a_skipped_module_var_is_an_undeclared_var_error() {
		// A `--var` for a var whose module is not selected is treated exactly like
		// a var the pack never declared: the var is skipped entirely, so the
		// override names nothing and fails as UndeclaredVar.
		let root = module_var_fixture("module-var-skipped-override");
		let mut overrides = HashMap::new();
		overrides.insert("extra".to_string(), "value".to_string());
		match load(&PackSource::Directory(root.clone()), &HashMap::new(), &overrides, &[]) {
			Err(LoadError::UndeclaredVar(name)) => assert_eq!(name, "extra"),
			other => panic!("expected UndeclaredVar, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn a_var_tagged_with_an_undeclared_module_errors() {
		// A `[[var]]` tagging a module no `[[module]]` declares is a pack-authoring
		// error, caught even with no module selected.
		let root = scratch("module-var-dangling-tag");
		fs::create_dir_all(&root).unwrap();
		fs::write(
			root.join("pack.toml"),
			"[[asset]]\nsource = \"a.md\"\ndest = \"a.md\"\nownership = \"working\"\n\n\
			 [[var]]\nname = \"ghost\"\nmodule = \"ghost\"\n",
		)
		.unwrap();
		fs::write(root.join("a.md"), "a\n").unwrap();
		match load(&PackSource::Directory(root.clone()), &HashMap::new(), &HashMap::new(), &[]) {
			Err(LoadError::UndeclaredModuleTag {
				kind,
				entry,
				module,
			}) => {
				assert_eq!(kind, "var");
				assert_eq!(entry, "ghost");
				assert_eq!(module, "ghost");
			}
			other => panic!("expected UndeclaredModuleTag, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&root).unwrap();
	}

	/// A filesystem pack fixture exercising the `{{modules}}` guidance slot and the
	/// `requires` auto-enable. Two modules are declared in order: `base` (a guidance
	/// partial plus one tagged asset) and `extra` (a guidance partial, one tagged
	/// asset, and `requires = ["base"]`). Selecting `extra` must therefore pull in
	/// `base` too, and the guidance concatenates in declaration order (`base` first).
	fn module_guidance_fixture(name: &str) -> PathBuf {
		let root = scratch(name);
		fs::create_dir_all(&root).unwrap();
		fs::write(
			root.join("pack.toml"),
			"[[module]]\nname = \"base\"\ndescription = \"base\"\nguidance = \"base-guide.md\"\n\n\
			 [[module]]\nname = \"extra\"\ndescription = \"extra\"\nguidance = \"extra-guide.md\"\nrequires = [\"base\"]\n\n\
			 [[asset]]\nsource = \"core.md\"\ndest = \"core.md\"\nownership = \"working\"\n\n\
			 [[asset]]\nsource = \"base-asset.md\"\ndest = \"base-asset.md\"\nownership = \"working\"\nmodule = \"base\"\n\n\
			 [[asset]]\nsource = \"extra-asset.md\"\ndest = \"extra-asset.md\"\nownership = \"working\"\nmodule = \"extra\"\n",
		)
		.unwrap();
		fs::write(root.join("core.md"), "core\n").unwrap();
		fs::write(root.join("base-asset.md"), "base asset\n").unwrap();
		fs::write(root.join("extra-asset.md"), "extra asset\n").unwrap();
		fs::write(root.join("base-guide.md"), "BASE GUIDANCE\n").unwrap();
		fs::write(root.join("extra-guide.md"), "EXTRA GUIDANCE\n").unwrap();
		root
	}

	#[test]
	fn module_guidance_is_empty_with_nothing_enabled() {
		// No module selected: the block is empty, so a `{{modules}}` slot renders to
		// nothing and the output stays byte-identical to a module-free pack.
		let root = module_guidance_fixture("modules-guidance-empty");
		let source = PackSource::Directory(root.clone());
		assert_eq!(module_guidance(&source, &[]).unwrap(), "");
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn module_guidance_includes_an_enabled_modules_partial() {
		let root = module_guidance_fixture("modules-guidance-one");
		let source = PackSource::Directory(root.clone());
		// `base` selected: only its partial appears, trimmed with a trailing blank
		// line so a later render normalises the tail.
		assert_eq!(module_guidance(&source, &["base".to_string()]).unwrap(), "BASE GUIDANCE\n\n");
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn requires_auto_enables_a_dependencys_guidance_and_assets() {
		let root = module_guidance_fixture("modules-guidance-requires");
		let source = PackSource::Directory(root.clone());

		// Selecting `extra` pulls in `base` (its `requires`): both partials appear,
		// in `[[module]]` declaration order (`base` before `extra`).
		assert_eq!(
			module_guidance(&source, &["extra".to_string()]).unwrap(),
			"BASE GUIDANCE\n\nEXTRA GUIDANCE\n\n"
		);

		// The auto-enable also reaches asset filtering: selecting only `extra` drops
		// the core asset plus both the `base`- and `extra`-tagged assets.
		let assets =
			load(&source, &HashMap::new(), &HashMap::new(), &["extra".to_string()]).unwrap();
		let dests: Vec<&str> = assets.iter().map(|a| a.dest.as_str()).collect();
		assert_eq!(dests, vec!["core.md", "base-asset.md", "extra-asset.md"]);

		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn a_requires_naming_an_undeclared_module_errors() {
		// A `[[module]]` whose `requires` names a module no `[[module]]` declares is
		// a pack-authoring error, caught before anything is read.
		let root = scratch("modules-requires-dangling");
		fs::create_dir_all(&root).unwrap();
		fs::write(
			root.join("pack.toml"),
			"[[module]]\nname = \"extra\"\ndescription = \"extra\"\nrequires = [\"ghost\"]\n\n\
			 [[asset]]\nsource = \"core.md\"\ndest = \"core.md\"\nownership = \"working\"\n",
		)
		.unwrap();
		fs::write(root.join("core.md"), "core\n").unwrap();
		let source = PackSource::Directory(root.clone());
		// Both the guidance path and the load path report it.
		for result in [
			module_guidance(&source, &["extra".to_string()]).map(|_| ()),
			load(&source, &HashMap::new(), &HashMap::new(), &["extra".to_string()]).map(|_| ()),
		] {
			match result {
				Err(LoadError::UndeclaredModuleRequire {
					module,
					requires,
				}) => {
					assert_eq!(module, "extra");
					assert_eq!(requires, "ghost");
				}
				other => panic!("expected UndeclaredModuleRequire, got {other:?}"),
			}
		}
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn a_requires_cycle_terminates() {
		// Two modules that require each other must not loop: the fixed-point
		// expansion enables both and returns.
		let root = scratch("modules-requires-cycle");
		fs::create_dir_all(&root).unwrap();
		fs::write(
			root.join("pack.toml"),
			"[[module]]\nname = \"a\"\ndescription = \"a\"\nguidance = \"a.md\"\nrequires = [\"b\"]\n\n\
			 [[module]]\nname = \"b\"\ndescription = \"b\"\nguidance = \"b.md\"\nrequires = [\"a\"]\n\n\
			 [[asset]]\nsource = \"core.md\"\ndest = \"core.md\"\nownership = \"working\"\n",
		)
		.unwrap();
		fs::write(root.join("core.md"), "core\n").unwrap();
		fs::write(root.join("a.md"), "A\n").unwrap();
		fs::write(root.join("b.md"), "B\n").unwrap();
		let source = PackSource::Directory(root.clone());
		// Selecting either module enables both; guidance concatenates in declaration
		// order (`a` before `b`) without looping.
		assert_eq!(module_guidance(&source, &["a".to_string()]).unwrap(), "A\n\nB\n\n");
		load(&source, &HashMap::new(), &HashMap::new(), &["b".to_string()]).unwrap();
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn reserved_modules_variable_is_rejected() {
		// A pack may not declare the reserved `modules` variable.
		let declared = fixture_pack(
			"var-reserved-modules-declared",
			"[[asset]]\nsource = \"a.md\"\ndest = \"a.md\"\nownership = \"working\"\n\n\
			 [[var]]\nname = \"modules\"\ndefault = \"x\"\n",
			"a.md",
			"a\n",
		);
		match load(&PackSource::Directory(declared.clone()), &HashMap::new(), &HashMap::new(), &[])
		{
			Err(LoadError::ReservedVar(name)) => assert_eq!(name, "modules"),
			other => panic!("expected ReservedVar for declaration, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&declared).unwrap();

		// Nor may `--var` set it.
		let root = var_fixture("var-reserved-modules-override");
		let mut overrides = HashMap::new();
		overrides.insert("who".to_string(), "world".to_string());
		overrides.insert("modules".to_string(), "x".to_string());
		match load(&PackSource::Directory(root.clone()), &HashMap::new(), &overrides, &[]) {
			Err(LoadError::ReservedVar(name)) => assert_eq!(name, "modules"),
			other => panic!("expected ReservedVar for override, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn duplicate_module_names_error() {
		// Two `[[module]]` sections with the same name is a pack-authoring error,
		// not a silent dedupe.
		let root = scratch("module-duplicate");
		fs::create_dir_all(&root).unwrap();
		fs::write(
			root.join("pack.toml"),
			"[[module]]\nname = \"extras\"\ndescription = \"first\"\n\n\
			 [[module]]\nname = \"extras\"\ndescription = \"second\"\n\n\
			 [[asset]]\nsource = \"core.md\"\ndest = \"core.md\"\nownership = \"working\"\n",
		)
		.unwrap();
		fs::write(root.join("core.md"), "core\n").unwrap();
		match load(&PackSource::Directory(root.clone()), &HashMap::new(), &HashMap::new(), &[]) {
			Err(LoadError::DuplicateModule(name)) => assert_eq!(name, "extras"),
			other => panic!("expected DuplicateModule, got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn a_missing_guidance_file_errors_when_its_module_is_enabled() {
		// A `[[module]]` whose `guidance` names a partial the pack does not ship is a
		// hard failure once that module is enabled: a declared guidance file is
		// required, not silently skipped the way `instrument.md` is. This pins the
		// hard-fail contract against a regression to a silent success.
		let root = scratch("modules-guidance-missing");
		fs::create_dir_all(&root).unwrap();
		fs::write(
			root.join("pack.toml"),
			"[[module]]\nname = \"base\"\ndescription = \"base\"\nguidance = \"absent-guide.md\"\n\n\
			 [[asset]]\nsource = \"core.md\"\ndest = \"core.md\"\nownership = \"working\"\n",
		)
		.unwrap();
		fs::write(root.join("core.md"), "core\n").unwrap();
		// `absent-guide.md` is deliberately not written.
		let source = PackSource::Directory(root.clone());

		// Enabling `base` must fail on the missing guidance file, not succeed silently.
		match module_guidance(&source, &["base".to_string()]) {
			Err(LoadError::Io(error)) => {
				assert_eq!(error.kind(), io::ErrorKind::NotFound);
				// The wrapped message names the module and the missing file.
				let message = error.to_string();
				assert!(message.contains("base"), "message should name the module: {message}");
				assert!(
					message.contains("absent-guide.md"),
					"message should name the guidance file: {message}"
				);
			}
			other => panic!("expected Io(NotFound), got {:?}", other.map(|_| ())),
		}
		fs::remove_dir_all(&root).unwrap();
	}
}
