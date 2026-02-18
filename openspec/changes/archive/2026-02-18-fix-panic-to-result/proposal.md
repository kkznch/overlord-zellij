## Why

Production code contains `unwrap()`, `expect()`, and `panic!()` calls that will crash the process on unexpected input (missing env vars, non-UTF-8 paths, template errors). These should return `Result` for graceful error handling.

## What Changes

- Replace `panic!()` / `unwrap()` / `expect()` in non-test code with proper `Result` propagation
- Affected modules: `relay/server.rs`, `relay/notify.rs`, `layout.rs`, `commands/summon.rs`, `main.rs`
- No behavioral change — same error conditions, but graceful exit with error message instead of panic

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `mcp-relay`: relay server startup and notification must return errors instead of panicking
- `zellij-session`: layout generation and session startup must propagate errors

## Impact

- `src/relay/server.rs` — `serve()` env var handling, `HOME` fallback
- `src/relay/notify.rs` — `notify_pane()` role lookup
- `src/layout.rs` — `generate_layout()` template rendering
- `src/commands/summon.rs` — path conversion
- `src/main.rs` — tokio runtime creation
