## Context

Production code contains `unwrap()`, `expect()`, and `panic!()` in paths that can be triggered by missing environment variables, non-UTF-8 paths, or template errors. These crash the process instead of providing graceful error messages.

Affected locations:
- `src/relay/server.rs:444` — `panic!` on missing `OVLD_ROLE`
- `src/relay/server.rs:455,470` — `expect()` on missing `HOME`
- `src/relay/server.rs:489` — `expect()` on MCP server start failure
- `src/relay/notify.rs:32` — `panic!` on unknown role in PANE_ORDER
- `src/layout.rs:96,99,110` — `expect()` on template add/get/render
- `src/commands/summon.rs:109` — `unwrap()` on path UTF-8 conversion
- `src/main.rs:71` — `expect()` on tokio runtime creation

## Goals / Non-Goals

**Goals:**
- Replace all `panic!`/`unwrap()`/`expect()` in non-test code with `Result` propagation using `anyhow`
- Maintain identical behavior for success paths
- Provide clear error messages for each failure mode

**Non-Goals:**
- Fixing `unwrap()` in test code (test panics are expected behavior)
- Adding retry logic or recovery mechanisms
- Changing function signatures beyond adding `Result` return types where needed

## Decisions

### 1. Use `anyhow::Context` for wrapping errors

All conversions use `.context("descriptive message")` or `.with_context(|| ...)` for consistency with the existing codebase pattern.

**Why:** Already used throughout `config.rs`, `summon.rs`, and other modules. No new dependencies needed.

### 2. `generate_layout()` returns `Result<String>`

Currently returns `String` with three `expect()` calls. Change signature to `Result<String>` and propagate errors. Callers (`create_temp_layout`) already return `Result`.

**Why:** Template errors are actionable (misconfigured paths) and should be reported, not crashed on.

### 3. `notify_pane()` returns `Result` (already does, but panics internally)

Replace `unwrap_or_else(|| panic!(...))` with `anyhow::bail!` or `.context()`. The function already returns `Result<()>`.

**Why:** Callers already handle the error with `let _ = notify::notify_pane(...)`.

### 4. `serve()` propagates env var errors

Replace `panic!` and `expect()` with `?` operator. `serve()` already returns `Result<()>`.

**Why:** The caller in `main.rs` already handles `Result` from the relay command.

### 5. `main.rs` tokio runtime: use `?` in main

Replace `.expect()` with `.context()` and `?`. `main()` uses `anyhow::Result`.

**Why:** Consistent with other error handling in main.

## Risks / Trade-offs

- [Risk] Changing `generate_layout()` signature is a breaking change for callers → Only `create_temp_layout()` calls it, and it already returns `Result`. Low risk.
- [Risk] Tests that call `generate_layout()` will need updating → Add `?` or `.unwrap()` in tests (acceptable for test code).
