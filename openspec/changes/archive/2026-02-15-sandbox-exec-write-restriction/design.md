## Context

`ovld summon` launches 6 Claude Code agents inside Zellij panes. Each agent has unrestricted filesystem access. macOS provides `sandbox-exec` (Seatbelt) — a kernel-level sandbox that enforces access policies via SBPL profile files. The project already uses `tempfile::NamedTempFile` for layout files, so the same pattern applies to sandbox profiles.

Current agent launch flow: Zellij KDL layout specifies `command "claude"` with args for each pane. The layout is generated dynamically in `src/layout.rs`.

## Goals / Non-Goals

**Goals:**
- Restrict Claude Code agents to writing only within: project directory (CWD), relay directory, `~/.claude`, and temp directories
- Use macOS `sandbox-exec` as the enforcement mechanism
- Enable sandbox by default; provide `--no-sandbox` escape hatch
- Gracefully degrade on non-macOS platforms (warning, no crash)

**Non-Goals:**
- Linux sandboxing (bubblewrap, seccomp, etc.) — out of scope for now
- Read restrictions — agents need to read broadly for code analysis
- Network restrictions — not needed for this use case
- Sandboxing the `ovld` process itself — only agent subprocesses

## Decisions

### 1. `allow default` base profile (not `deny default`)

**Choice**: Start with `(allow default)` and selectively deny `file-write*`.

**Rationale**: A `deny default` profile would break many things (network, process spawning, IPC) and require exhaustive whitelisting. Since we only care about write restriction, `allow default` + `deny file-write*` is simpler and less fragile.

**Alternative considered**: Full `deny default` with granular allows — rejected due to complexity and brittleness across macOS versions.

### 2. Temporary `.sb` profile file (not inline)

**Choice**: Write the Seatbelt profile to a `NamedTempFile` and pass its path to `sandbox-exec -f`.

**Rationale**: `sandbox-exec -p` (inline profile string) has quoting issues with complex profiles. `-f` is cleaner and consistent with the existing `NamedTempFile` pattern used for layouts.

### 3. Wrapping at the KDL level (not process spawning)

**Choice**: Change the KDL `command` from `"claude"` to `"sandbox-exec"` and prepend `-f <profile>` to args.

**Rationale**: Zellij spawns the processes, so we control them via KDL layout. This is the simplest integration point — no changes to process management code needed.

**Alternative considered**: Wrapping in a shell script — rejected as unnecessary indirection.

### 4. macOS only with graceful degradation

**Choice**: Use `cfg!(target_os = "macos")` at runtime. Non-macOS prints a warning and proceeds without sandbox.

**Rationale**: `sandbox-exec` is macOS-specific. Linux equivalents (bubblewrap, firejail) have different APIs and would significantly increase scope. The warning ensures users aren't silently unprotected.

## Risks / Trade-offs

- **[Risk] Seatbelt deprecation**: Apple has soft-deprecated `sandbox-exec` (no man page updates). → **Mitigation**: It still works on current macOS and is used by Homebrew, nix, and Claude Code itself. Monitor for removal in future macOS releases.
- **[Risk] Insufficient write allowlist**: Some Claude operations may need write access to paths not in the allowlist. → **Mitigation**: The allowlist covers known paths (CWD, relay, ~/.claude, /tmp). Users can use `--no-sandbox` if issues arise. Log sandbox violations for debugging.
- **[Trade-off] macOS only**: Linux users get no sandboxing. → Acceptable per user decision; can be extended later.
- **[Trade-off] Profile path in KDL**: The sandbox profile path is embedded in the layout file, creating a dependency between two temp files. → Both use `NamedTempFile` and are kept alive for the session duration.
