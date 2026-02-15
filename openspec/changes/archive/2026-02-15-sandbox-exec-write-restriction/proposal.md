## Why

Claude Code agents launched by `ovld summon` can write files anywhere on the filesystem. This is a security risk — agents should only be able to modify files within the project directory and necessary system paths (relay store, temp, Claude config). macOS provides kernel-level process sandboxing via `sandbox-exec` (Seatbelt) that can restrict file writes at the OS level.

## What Changes

- Add a new `sandbox` module that generates macOS Seatbelt profiles restricting `file-write*` operations
- Modify the Zellij layout generator to optionally wrap `claude` commands with `sandbox-exec -f <profile>`
- Add `--no-sandbox` CLI flag to `ovld summon` (sandbox is enabled by default)
- Non-macOS platforms print a warning and skip sandboxing gracefully

## Capabilities

### New Capabilities
- `sandbox`: Process-level file write restriction using macOS Seatbelt profiles. Generates `.sb` profiles that deny all writes except to CWD, relay directory, `~/.claude`, and temp directories. Wraps Claude invocations with `sandbox-exec`.

### Modified Capabilities
- `zellij-session`: Layout generation now accepts an optional sandbox profile path, switching the pane command from `claude` to `sandbox-exec -f <profile> claude` when provided.
- `ovld-cli`: `summon` subcommand gains `--no-sandbox` flag and `sandbox: bool` parameter.

## Impact

- **Files**: `src/sandbox.rs` (new), `src/layout.rs`, `src/commands/summon.rs`, `src/main.rs`, `src/lib.rs`
- **Dependencies**: Uses existing `tempfile` crate for temporary `.sb` profile files (auto-cleaned on drop)
- **Platform**: macOS only. Non-macOS silently degrades with a warning.
- **CLI**: New `--no-sandbox` flag on `ovld summon`
