## Why

Dashboard tab switching between 6 agent panes is tedious. A single TUI view showing all roles' status, pending messages, and recent activity at a glance eliminates context-switching overhead.

## What Changes

- Add `ovld dashboard` subcommand: ratatui-based TUI that polls relay store every 2s
- Add 4th Zellij tab ("dashboard") running `ovld dashboard` automatically on summon
- Add `MessageStore::has_pending()` and `MessageStore::recent_messages(limit)` for read-only dashboard data access
- Add `ratatui` and `crossterm` dependencies

## Capabilities

### New Capabilities
- `dashboard`: Real-time TUI showing army status table, pending message indicators, worker count summary, and recent message feed with timestamps

### Modified Capabilities
- `ovld-cli`: New `dashboard` subcommand added
- `zellij-session`: Layout changes from 3 tabs to 4 tabs (command, battlefield, support, dashboard)

## Impact

- `Cargo.toml`: ratatui 0.29, crossterm 0.28 added
- `src/commands/dashboard.rs`: New file (~190 lines)
- `src/commands/mod.rs`: dashboard module registered
- `src/main.rs`: Dashboard variant in Commands enum
- `src/layout.rs`: 4th tab added, uses `std::env::current_exe()` for ovld binary path
- `src/relay/store.rs`: `has_pending()` and `recent_messages()` methods added
- Pane ID mapping (0-5) unchanged; dashboard pane gets ID 6 (last tab)
