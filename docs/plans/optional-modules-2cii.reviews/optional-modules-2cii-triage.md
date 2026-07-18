# Triage: optional-modules increment 2c-ii (`--staged` + `--with-precommit-hook`)

Triager: independent, RISKY increment (two consecutive clean rounds required). Range: `cbf4a15..49a377f`. Verdicts adjudicate the opus reviewer (F1-F4) and the sonnet reviewer (F-1 to F-6). The off-PATH skip-with-note default and the deferred increment-2 CHANGELOG are settled, not adjudicated here.

Verification performed by the triager: built the binary in a throwaway worktree at `49a377f` and drove it against scratch git repos. Empirically confirmed F1 (dangling-symlink write-through), F2 (write error fails the whole scaffold), the staged initial-commit path (works: pass on clean, fail on staged violation, no HEAD), and the failed-`exec` exit behaviour in both bash-as-`sh` and dash (relevant to sonnet F-3). Throwaway worktree removed after.

## Must-fix summary (blocks a clean round)

| ID | Verdict | Severity | Owner | Reason |
| --- | --- | --- | --- | --- |
| opus F1 | VALID | medium | implementer | Dangling-symlink never-clobber gap + write-escape outside `.git/hooks/`, false success. |
| opus F2 | VALID | low | implementer | Hook write error fails the whole scaffold, contradicting the documented "install never fails the scaffold" contract. |
| opus F3 | VALID | low (test) | implementer | No symlink never-clobber test; pins the F1 fix (Principle 11). |
| sonnet F-1 | VALID | low | implementer | New public `executable` manifest field undocumented in the README pack-format reference. |
| sonnet F-2 | VALID | low (test) | implementer | No test for `Staged` on an empty repo (first-commit scenario); pins a documented claim. |

## Defer / not converge-blockers

| ID | Verdict | Severity | Owner | Disposition |
| --- | --- | --- | --- | --- |
| sonnet F-4 | VALID | low | orchestrator | Plan schema line missing `executable`; batch at increment-2 close (like the CHANGELOG). |
| sonnet F-6 | VALID | low (test) | implementer | Broad e2e `run_scaffold --with-precommit-hook` coverage; recommended, not blocking. |
| sonnet F-5 | VALID | low | implementer | Redundant "check for an existing hook" note in the `Exists` branch; cosmetic. |
| opus F4 | VALID | very low | implementer | Bare-repo install reported "Installed" though inert; informational. |

## Invalid

| ID | Verdict | Reason |
| --- | --- | --- |
| sonnet F-3 | INVALID | Premise is factually wrong: a failed `exec` in a non-interactive POSIX shell exits 126/127 (which FAILS the commit), it does not fall through to exit 0. |

---

## Detailed verdicts

### opus F1 (dangling-symlink write-through) - VALID, medium, implementer, MUST-FIX

Confirmed empirically. With `.git/hooks/pre-commit -> ../../victim/gone` (target absent), `agent-scaffold scaffold --module checks --with-precommit-hook --write` wrote the full delegate script to `victim/gone` (a path OUTSIDE `.git/hooks/`), chmodded it `0o755`, printed "Installed the pre-commit hook at ./.git/hooks/pre-commit", and exited 0. The symlink still points at `victim/gone`, so the pre-existing symlink hook is silently repurposed to run agent-scaffold.

Root cause is exactly as reported: `hook.exists()` at `src/main.rs:744` follows the symlink and stats the (missing) target, so a dangling symlink reads as absent; the `fs::write` at `:747` then follows the symlink and creates the target. This is a genuine violation of the increment's core never-clobber promise, PLUS an out-of-directory executable write, PLUS a false success message.

Severity: medium is correct, not high/critical. It does not destroy existing hook CONTENT (the dangling target was by definition missing), and the trigger is a pre-existing dangling symlink at `.git/hooks/pre-commit`, which is uncommon and already requires write access to `.git/`. But it is the highest-priority must-fix here: it breaks the exact invariant this RISKY increment is built to guarantee.

Fix confirmed correct: replace `hook.exists()` with an lstat-based check, e.g. `if hook.symlink_metadata().is_ok()`. `symlink_metadata` does not follow the link, so a dangling symlink, a valid symlink, and a regular file all report `Exists` (never written through), while a truly absent path still errors and takes the write branch. No regression to the valid-symlink or absent cases.

### opus F2 (hook write error fails the whole scaffold) - VALID, low, implementer, MUST-FIX

Confirmed empirically. With `.git/hooks/pre-commit -> /no/such/parent/target` (missing parent, so the write genuinely fails), the run printed "Wrote to . (23 changed, 0 left untouched)." and then died with `Error: Os { code: 2, kind: NotFound }` and exit code 1, AFTER all assets had landed.

This contradicts the contract stated in the code's own doc comments (`src/main.rs:733-737` and `:930-936`): only a non-repo output dir is swallowed (`NotARepo`), so a hook-install problem never fails a scaffold whose assets already succeeded. The `?` on `install_precommit_hook(&args.output_dir)?` at `:938` leaks any `io::Error` from `create_dir_all`/`fs::write`.

Severity low is correct: it needs a genuine write failure (unwritable hooks dir, `hooks` being a file, or the F1 dangling-into-missing-parent case). After the F1 lstat fix the dangling-symlink trigger disappears, but the permissions/`hooks`-is-a-file triggers remain, so F2 is independent of F1. MUST-FIX because it is a documented-contract violation that dumps a raw io error after reporting asset success, and the fix is small and clearly correct: map an install error to a printed note plus the manual instructions (as `NotARepo` already does) instead of propagating via `?`. Natural to fix in the same pass as F1 (same function's error handling).

### opus F3 (no symlink never-clobber test) - VALID, low (test), implementer, MUST-FIX

Valid and coupled to F1. The never-clobber suite covers a regular existing file and a non-repo skip, but no symlink case, which is exactly where F1 lives; a test would have caught it. On a RISKY never-clobber increment a fix without a pinning test is incomplete (Principle 11). Add a test asserting `Exists` and byte-unchanged (and no write-through to the target) for BOTH a valid and a dangling symlink hook. MUST-FIX, bundled with the F1 fix.

### opus F4 (bare-repo install reported "Installed") - VALID, very low, implementer, DEFER

Valid and inert: a bare repo never runs `pre-commit`, so the written hook does nothing, but the message says "Installed". Cosmetic only; the reviewer themselves judged it not worth changing. Defer (optional polish, e.g. detect a bare repo and note the hook is inert). Not a converge-blocker.

### sonnet F-1 (README missing `executable`) - VALID, low, implementer, MUST-FIX

Confirmed: the README pack-format `[[asset]]` example (around `README.md:222-228`) shows `source`, `dest`, `ownership`, `render` but not the new `executable` field, so an external pack author shipping an executable asset has no user-facing documentation. The `pack/pack.toml` comment and the Rust field docs are not user-facing. Severity low (a doc omission, not a functional defect; sonnet's medium is a touch high). MUST-FIX: shipping a new public manifest field without its user-facing reference entry leaves the feature incomplete, and the fix is one commented example line plus a clause of prose. Owner implementer (user-facing repo doc, added in this diff's scope).

### sonnet F-2 (no `Staged` empty-repo/initial-commit test) - VALID, low (test), implementer, MUST-FIX

The behaviour is CORRECT: I verified that `checks --staged` on a repo with staged files and no HEAD passes on clean staged content (exit 0) and fails on a staged violation (exit 1). `git write-tree` + `git commit-tree` (no `-p`) makes a root commit that `git worktree add --detach` checks out. So the doc claim ("does not need a prior commit") holds; this is a pure test gap, not a bug. Pre-commit hooks DO run on the first commit, so it is the primary scenario, and the claim is explicit and untested (Principle 11). MUST-FIX (cheap, pins a documented claim about the most common user action). This is the softest of the must-fixes, but on a RISKY increment the targeted pin is worth it. Add `run(&dir, Isolation::Staged)` on an empty repo with staged files, asserting no error and the check passes.

### sonnet F-3 (delegate lacks `set -eu`; failed `exec` silently skips) - INVALID

The premise is factually wrong. I tested a failed `exec` (nonexistent target, and a non-executable target) in both bash-as-`sh` and dash: a non-interactive POSIX shell that fails to `exec` EXITS with 127 (not found) or 126 (not executable); it does not continue to the end of the script and exit 0. That non-zero exit correctly FAILS the commit. The described silent-pass does not occur. The installed delegate is a three-line `exec` wrapper with no variable references, so `set -eu` adds no real protection here; adding it for cosmetic consistency with the tracked `pack/hooks/pre-commit` is optional, not a correctness fix. INVALID as a finding.

### sonnet F-4 (plan schema line missing `executable`) - VALID, low, orchestrator, DEFER

Confirmed: `docs/plans/agent-scaffold.md` line 64 lists `{ source, dest, ownership, render, module? }` without `executable`. Real internal-schema drift. But the plan is an orchestrator-owned design doc, and orchestrator-owned doc updates for this increment are being batched at the imminent increment-2 close (the same disposition as the CHANGELOG). Defer to that close; not a converge-blocker for the implementer's clean rounds. Owner orchestrator.

### sonnet F-5 (redundant "check first" note in `Exists`) - VALID, low, implementer, DEFER

Confirmed: on `HookInstall::Exists` the caller prints "A pre-commit hook already exists ...; leaving it untouched." and then `print_manual_hook_instructions` appends "(check for an existing .git/hooks/pre-commit first)." Mildly redundant/contradictory in that branch (the `ln -s` one-liner itself is still useful). Cosmetic; not a converge-blocker. Optional polish: reword the shared note, or split the helper so the `Exists` branch omits the "check first" clause. Owner implementer.

### sonnet F-6 (no e2e `run_scaffold --with-precommit-hook` test) - VALID, low (test), implementer, DEFER

Valid: the `run_scaffold` wiring guarded by `args.with_precommit_hook` (coherence `exit(2)`, the preview line, and the `Installed`/`Exists`/`NotARepo` prints) has no end-to-end test; `install_precommit_hook` and `precommit_coherent` are unit-tested in isolation. I exercised these paths manually (coherence, install, dangling-symlink, write-failure) and they behave as expected, so the regression risk is lower than the targeted F-2 gap. Recommended (a `run_scaffold`-level test asserting the hook is created and executable would be worthwhile), but not a hard converge-blocker. Owner implementer.

## Note on the settled off-PATH design (not a finding)

Both reviewers independently endorsed skip-with-note, which is confirmed. Sonnet's side observation that hook stderr is invisible in some GUI git clients is a possible one-sentence guidance addition (orchestrator, optional), not a code change and not a finding.

## MUST-FIX to converge

opus F1, opus F2, opus F3, sonnet F-1, sonnet F-2. F1/F2/F3 cluster in `install_precommit_hook` and its tests (fix and pin together). F-1 and F-2 are cheap completeness (README doc, one test). Everything else is defer/optional or, for sonnet F-3, invalid.
