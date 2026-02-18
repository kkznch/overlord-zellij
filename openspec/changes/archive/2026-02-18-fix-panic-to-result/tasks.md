## 1. relay/server.rs

- [x] 1.1 `serve()`: replace `panic!` on missing `OVLD_ROLE` with `anyhow::bail!` or `.context()`
- [x] 1.2 `serve()`: replace `expect("HOME not set")` (L455, L470) with `.context()`
- [x] 1.3 `serve()`: replace `expect("Failed to start MCP server")` (L489) with `.context()` + `?`

## 2. relay/notify.rs

- [x] 2.1 `notify_pane()`: replace `unwrap_or_else(|| panic!(...))` on `pane_id_for_role` with `anyhow::bail!`

## 3. layout.rs

- [x] 3.1 Change `generate_layout()` return type from `String` to `Result<String>`
- [x] 3.2 Replace three `expect()` calls (add_template, get_template, render) with `?`
- [x] 3.3 Update `create_temp_layout()` to propagate the new `Result`
- [x] 3.4 Update test code to handle `Result` (add `.unwrap()` in tests)

## 4. commands/summon.rs

- [x] 4.1 Replace `layout_path.to_str().unwrap()` with `.context("layout path is not valid UTF-8")?`

## 5. main.rs

- [x] 5.1 Replace `Runtime::new().expect(...)` with `.context(...)` + `?`

## 6. Verification

- [x] 6.1 `cargo build` passes
- [x] 6.2 `cargo test` — all 90 tests pass
- [x] 6.3 `cargo clippy` — no new warnings (既存の11件は今回のスコープ外)
