//! agent-scaffold: proof-of-concept drop logic.
//!
//! This validates the two-tier ownership model from the spec before the rest
//! of the tool is built:
//!
//! - Reference assets (tool-owned) are always written, so they stay current.
//! - Working files (user-owned) are created only if absent, unless `--force`.
//!
//! The asset set here is stub content; the real guidance, prompts, plan
//! template, and principle data come later.

use {
	clap::Parser,
	std::{
		fs,
		io,
		path::{
			Path,
			PathBuf,
		},
	},
};

/// Whether a scaffolded asset is owned by the tool or by the user.
#[derive(Clone, Copy)]
enum Ownership {
	/// Tool-owned reference asset: always (re)written to stay current.
	Reference,
	/// User working file: created only if absent, unless forced.
	Working,
}

/// A single asset the tool drops into the output directory.
struct Asset {
	/// Path relative to the output directory.
	path: &'static str,
	contents: &'static str,
	ownership: Ownership,
}

/// The stub asset set for the proof-of-concept. The namespaced reference
/// directory defaults to `.agents/`; the working files are the root
/// `AGENTS.md` and the plan template under `docs/plans/`.
const ASSETS: &[Asset] = &[
	Asset {
		path: "AGENTS.md",
		contents: "# Agent guidance (stub)\n",
		ownership: Ownership::Working,
	},
	Asset {
		path: "docs/plans/TEMPLATE.md",
		contents: "# Plan template (stub)\n",
		ownership: Ownership::Working,
	},
	Asset {
		path: ".agents/AGENTS.reference.md",
		contents: "# Agent guidance, reference copy (stub)\n",
		ownership: Ownership::Reference,
	},
	Asset {
		path: ".agents/prompts/open-questions-gate.md",
		contents: "Open-questions gate prompt (stub)\n",
		ownership: Ownership::Reference,
	},
	Asset {
		path: ".agents/principles.toml",
		contents: "# principle data (stub)\n",
		ownership: Ownership::Reference,
	},
];

/// What happened to a single asset on a scaffold run.
#[derive(Debug, PartialEq, Eq)]
enum Outcome {
	Created,
	Refreshed,
	SkippedExisting,
	Overwritten,
}

impl Outcome {
	fn label(&self) -> &'static str {
		match self {
			Outcome::Created => "created",
			Outcome::Refreshed => "refreshed",
			Outcome::SkippedExisting => "skipped (exists)",
			Outcome::Overwritten => "overwritten",
		}
	}
}

/// Write one asset under `root`, honouring its ownership and `force`.
fn write_asset(
	root: &Path,
	asset: &Asset,
	force: bool,
) -> io::Result<Outcome> {
	let dest = root.join(asset.path);
	let exists = dest.exists();

	let outcome = match asset.ownership {
		Ownership::Reference =>
			if exists {
				Outcome::Refreshed
			} else {
				Outcome::Created
			},
		Ownership::Working =>
			if exists {
				if force {
					Outcome::Overwritten
				} else {
					return Ok(Outcome::SkippedExisting);
				}
			} else {
				Outcome::Created
			},
	};

	if let Some(parent) = dest.parent() {
		fs::create_dir_all(parent)?;
	}
	fs::write(&dest, asset.contents)?;
	Ok(outcome)
}

/// Scaffold the whole asset set, returning each asset's path and outcome.
fn scaffold(
	root: &Path,
	force: bool,
) -> io::Result<Vec<(&'static str, Outcome)>> {
	ASSETS
		.iter()
		.map(|asset| write_asset(root, asset, force).map(|outcome| (asset.path, outcome)))
		.collect()
}

/// Scaffold the agent workflow into a project.
#[derive(Parser)]
#[command(name = "agent-scaffold", about, version)]
struct Cli {
	/// Directory to scaffold into (defaults to the current directory).
	#[arg(long, default_value = ".")]
	output_dir: PathBuf,
	/// Overwrite existing user working files instead of leaving them untouched.
	#[arg(long)]
	force: bool,
}

fn main() -> io::Result<()> {
	let cli = Cli::parse();
	for (path, outcome) in scaffold(&cli.output_dir, cli.force)? {
		println!("{:>16}  {}", outcome.label(), path);
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	/// A unique scratch directory under the system temp dir for one test.
	fn scratch(name: &str) -> PathBuf {
		let dir = std::env::temp_dir().join(format!(
			"agent-scaffold-poc-{}-{}",
			std::process::id(),
			name
		));
		let _ = fs::remove_dir_all(&dir);
		dir
	}

	#[test]
	fn first_run_creates_everything() {
		let root = scratch("first-run");
		let results = scaffold(&root, false).unwrap();
		assert!(results.iter().all(|(_, o)| *o == Outcome::Created));
		assert!(root.join("AGENTS.md").exists());
		assert!(root.join(".agents/principles.toml").exists());
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn rerun_refreshes_reference_but_skips_working() {
		let root = scratch("rerun");
		scaffold(&root, false).unwrap();

		// A user edits a working file after the first run.
		fs::write(root.join("AGENTS.md"), "user edits\n").unwrap();

		let results = scaffold(&root, false).unwrap();
		let outcome_of =
			|p: &str| results.iter().find(|(path, _)| *path == p).map(|(_, o)| o).unwrap();

		// Reference assets are refreshed; the edited working file is left alone.
		assert_eq!(*outcome_of(".agents/principles.toml"), Outcome::Refreshed);
		assert_eq!(*outcome_of("AGENTS.md"), Outcome::SkippedExisting);
		assert_eq!(fs::read_to_string(root.join("AGENTS.md")).unwrap(), "user edits\n");
		fs::remove_dir_all(&root).unwrap();
	}

	#[test]
	fn force_overwrites_working_files() {
		let root = scratch("force");
		scaffold(&root, false).unwrap();
		fs::write(root.join("AGENTS.md"), "user edits\n").unwrap();

		let results = scaffold(&root, true).unwrap();
		let agents = results.iter().find(|(path, _)| *path == "AGENTS.md").map(|(_, o)| o).unwrap();
		assert_eq!(*agents, Outcome::Overwritten);
		assert_eq!(
			fs::read_to_string(root.join("AGENTS.md")).unwrap(),
			"# Agent guidance (stub)\n"
		);
		fs::remove_dir_all(&root).unwrap();
	}
}
