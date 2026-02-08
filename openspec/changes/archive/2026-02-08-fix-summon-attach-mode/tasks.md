## 1. Config Management Module

- [x] 1.1 Create `src/config.rs` with config directory constants
- [x] 1.2 Implement `resolve_rituals_dir()` - check `./rituals/` then `~/.config/ovld/rituals/`
- [x] 1.3 Implement `ensure_default_config()` - create `~/.config/ovld/rituals/` if missing
- [x] 1.4 Add `include_str!` for all 6 ritual files (overlord, strategist, inferno, glacier, shadow, storm)
- [x] 1.5 Implement `extract_default_rituals()` - write embedded content to config dir

## 2. KDL Layout Generation

- [x] 2.1 Create `src/layout.rs` with `generate_layout()` function
- [x] 2.2 Implement KDL generation with absolute paths for ritual files
- [x] 2.3 Add `--no-rituals` support - generate with `command "bash"` instead of `command "claude"`
- [x] 2.4 Implement temp file creation and cleanup for generated KDL

## 3. Session Management Update

- [x] 3.1 Modify `ZellijSession::start()` to use `status()` instead of `spawn()`
- [x] 3.2 Update `start()` to accept generated KDL path
- [x] 3.3 Ensure CLI blocks until Zellij exits

## 4. Summon Command Refactor

- [x] 4.1 Update `summon.rs` to call `ensure_default_config()` on startup
- [x] 4.2 Replace `find_layout_file()` with `generate_layout()` call
- [x] 4.3 Remove ritual injection logic (no longer needed)
- [x] 4.4 Update `find_rituals_dir()` to use config module's `resolve_rituals_dir()`

## 5. Cleanup Old Code

- [x] 5.1 Remove or simplify `src/army/ritual.rs` (external injection no longer needed)
- [x] 5.2 Remove `src/zellij/pane.rs` if no longer used
- [x] 5.3 Update module exports in `src/lib.rs` and `src/zellij/mod.rs`

## 6. Verification

- [ ] 6.1 Test `ovld summon` from project directory with local `./rituals/`
- [ ] 6.2 Test `ovld summon` from arbitrary directory (uses global config)
- [ ] 6.3 Test first run - verify `~/.config/ovld/rituals/` is auto-created
- [ ] 6.4 Test `ovld summon --no-rituals` - verify bash panes instead of Claude
- [ ] 6.5 Verify all 6 panes start Claude with correct ritual files
