## 1. Sandbox Module

- [x] 1.1 Create `src/sandbox.rs` with `generate_profile(cwd, relay_dir)` that produces a Seatbelt SBPL profile string
- [x] 1.2 Implement `create_temp_profile(cwd, relay_dir)` returning `(NamedTempFile, PathBuf)`
- [x] 1.3 Add unit tests: profile contains deny-write, allows CWD, relay_dir, ~/.claude, /tmp, /dev/null, /dev/tty, allow-default
- [x] 1.4 Add unit test: temp file creation and `.sb` extension

## 2. Layout Integration

- [x] 2.1 Add `sandbox_profile: Option<&Path>` parameter to `generate_layout()` and `create_temp_layout()`
- [x] 2.2 Implement KDL command switching: `sandbox-exec -f <profile> claude ...` when `Some`, `claude ...` when `None`
- [x] 2.3 Update existing layout tests to pass `None` for new parameter
- [x] 2.4 Add test for sandbox-enabled layout (assert `command "sandbox-exec"`, no `command "claude"`)

## 3. Summon Flow

- [x] 3.1 Add `sandbox: bool` parameter to `summon::execute()`
- [x] 3.2 Conditionally create sandbox profile: `cfg!(target_os = "macos")` check, warning on non-macOS
- [x] 3.3 Pass `sandbox_path` to `create_temp_layout()`
- [x] 3.4 Print "Sandbox enabled" info message when sandbox is active

## 4. CLI Flag

- [x] 4.1 Add `--no-sandbox` flag to `Summon` variant in `Commands` enum
- [x] 4.2 Pass `!no_sandbox` to `summon::execute()` in match arm

## 5. Module Registration

- [x] 5.1 Add `pub mod sandbox;` to `src/lib.rs`

## 6. Verification

- [x] 6.1 `cargo test` — all tests pass
- [x] 6.2 `cargo build --release` — build succeeds without warnings
