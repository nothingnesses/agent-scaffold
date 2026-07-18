# optional-modules increment 3 (isolation module) - round 2 reviewer (confirming)

Verdict: CLEAN. All five round-1 fixes landed and are accurate; the fix edit introduced nothing new wrong; nothing regressed.

Scope reviewed: branch `impl/inc3-isolation` at tip `87ebc6e`. The only change since round 1 is commit `87ebc6e` ("docs: address isolation-module review findings"), which touches only `pack/isolation-guidance.md` (`git diff --stat fbc3b8f..87ebc6e` = 6 insertions, 4 deletions, one file). No asset row, module row, `pack.toml`, or `src/manifest.rs` change; the module stays guidance-only and byte-identical when off.

## Confirmation of the five settled findings

Ground truth: agent-images README `/home/jessea/Documents/projects/agent-images/README.md` and `AGENTS.md` (Writer isolation rule, lines 79-85; preflight, line 98).

- V1 (preflight detection cue): LANDED and accurate. `pack/isolation-guidance.md:37` now appends the concrete cue: container tier detected by `ab` on PATH AND the configured agent-images image present in `podman images` / `docker images`, else fall back to worktree then file-safety baseline. Loaded images do appear at `localhost/agent-images/<agent>:latest` in `podman images` (README:90,117), so the cue is technically correct, and the tier order matches AGENTS.md:79-85 and the preflight description at AGENTS.md:98.
- V2 (`--git` vs tier-2 terminology): LANDED and correct. `isolation-guidance.md:30` adds the disambiguation that agent-box `--git` mode is still tier-1 container isolation that additionally uses a git worktree for file visibility, distinct from the bare tier-2 "Worktree isolation" (a git worktree with no container). This is consistent with the README's "Worktree Mode (Sandboxed)" (README:157-159, a sandboxed container plus worktree) and with AGENTS.md:82 (tier-2 worktree isolates the filesystem only).
- V3 (`workspace_dir`): LANDED and matches upstream. `isolation-guidance.md:19` = `~/.local/agent-box/workspaces`, byte-identical to README:136.
- V4 ("committed/tracked files"): LANDED. `isolation-guidance.md:30` now reads "committed/tracked files (gitignored/untracked files are not visible)", matching README:159.
- V5 (docker acknowledgement): LANDED and correct. `isolation-guidance.md:12` adds `docker load < result` (matches README:89) and `:23` adds `backend = "podman" # or "docker"`; Docker is a supported backend per README:65.

## New-regression checks (all pass)

- Placeholder-free: no `{{...}}` in the file.
- ASCII-only: `grep -nP '[^\x00-\x7F]'` returns nothing; no em/en dashes, emoji, or non-ASCII. The only `--` occurrences are legitimate CLI flags (`--module`, `--git`, `--local`), not dash substitutes. `->`/`>=`/`!=` conventions are not needed and none are misused.
- Heading and URLs intact (the `src/manifest.rs` test asserts these): `## Writer isolation via agent-box and agent-images` at line 1; `github.com/nothingnesses/agent-images` at line 14; `github.com/0xferrous/agent-box` at line 28.
- No over/under-claiming: the file still frames itself as inert guidance the orchestrator acts on, taking no runtime dependency; the `--git` "container isolation (tier 1) plus worktree" description is accurate against the README.
- Tone consistent with `pack/checks-guidance.md` (same leading-imperative prose paragraphs with parenthetical clarifications).
- Tests: `just test` in the worktree passes (165 + 1 + 3 across the suites; 0 failed), including `builtin_isolation_module_renders_its_guidance_only_when_selected` and `builtin_manifest_lists_the_expected_assets`.

No new findings.
