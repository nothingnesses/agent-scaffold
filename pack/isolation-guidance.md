## Writer isolation via agent-box and agent-images

This project scaffolded the isolation module (`--module isolation`), the concrete wiring that the container tier of the Writer isolation rule resolves to. It does not restate that rule (see "Writer isolation (capability-tiered)" and "Worktree lifecycle and merge-back" above, which own the tier order and the worktree lifecycle); it supplies only the setup that makes the container tier available. When agent-box and an agent-images image are present the orchestrator runs writer agents under container isolation; when they are not, the same rule falls back to worktree isolation, then to the file-safety baseline, so nothing here is required to run the workflow. The module takes no runtime dependency on agent-box or agent-images and the tool never shells out to them: this is inert guidance the orchestrator acts on in the harness.

Install an agent image. agent-images builds OCI images with Nix; build the one you want and load it into podman:

```
nix build .#<agent>
podman load < result
```

(`docker load < result` works too if you run the Docker backend.)

That yields the image `localhost/agent-images/<agent>:latest`. See https://github.com/nothingnesses/agent-images for the list of available agents and the `mkAgentImage` customisation for building your own.

Configure agent-box. agent-box reads a single GLOBAL config from `~/.agent-box.toml` in your home directory. This is a per-user machine file, not a project file, which is why the module scaffolds no config into the repository: a project-tracked copy would sit in the wrong place and never be read. Create it yourself with `workspace_dir` and `base_repo_dir` and a `[runtime]` table naming the `backend`, the `image`, and the `env_passthrough` list:

```
workspace_dir = "~/.local/agent-box/workspaces"
base_repo_dir = "~/agent-box/repos"

[runtime]
backend = "podman" # or "docker"
image = "localhost/agent-images/<agent>:latest"
env_passthrough = ["ANTHROPIC_API_KEY"]
```

See https://github.com/0xferrous/agent-box for the tool and the agent-images README for the canonical, up-to-date example of this config.

Run a writer under isolation. For the worktree lifecycle the orchestrator already owns, use agent-box's sandboxed worktree mode (`--git`) so a writer agent sees only committed/tracked files (gitignored/untracked files are not visible): create a session from the repository, then spawn the writer against it. Note this `--git` mode is still container isolation (tier 1) that additionally uses a git worktree for file visibility; it is not the bare tier-2 "Worktree isolation" the rule names, which is a git worktree with no container.

```
ab new <repo> -s <session> --git
ab spawn -s <session> --git
```

`ab spawn --local` is the un-sandboxed alternative that mounts the current directory instead of a committed worktree; prefer the `--git` sandbox for writer agents. The orchestrator states the resolved isolation tier at preflight, so you know before a run whether a writer will run in a container, a worktree, or under the file-safety fallback. It detects the container tier by checking that agent-box (`ab` on PATH) is installed AND the configured agent-images image is loaded (present in `podman images` / `docker images`); if both hold it resolves to container isolation, otherwise it falls back to worktree isolation and then the file-safety baseline. Every spawned agent, including the reviewers, the triager, and the explorers, runs under the resolved isolation tier (container, else worktree, else the file-safety fallback), and the orchestrator merges their outputs onto main.

A role to launch-command spawn map and an agent-box container-wrapper shim are a possible future addition, not part of this module. This module ships only the pointers above; nothing here is generated or executed on your behalf.
