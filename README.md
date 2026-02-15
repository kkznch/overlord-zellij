# overlord-zellij

A CLI tool that orchestrates multiple Claude instances as a "Demon Army" on Zellij.

[日本語版 README](README.ja.md)

## Concept

**"The user only issues divine commands. The Demon Army handles everything else."**

Users simply convey vague requests in a single statement. The Overlord translates it into technical specifications, the Strategist breaks down tasks, and the Four Heavenly Kings autonomously complete implementation, testing, and documentation according to their specialties.

```
The Abyss's Will (User)
   │
   │ "Build something like X" (vague is fine)
   ↓
Overlord ─────────── Converts to requirements & technical specs
   ↓
Strategist ───────── Decomposes tasks & distributes to Four Kings
   ↓
┌──────────┬──────────┬──────────┬──────────┐
│ Glacier  │ Inferno  │  Shadow  │  Storm   │
│  Types   │  Logic   │  Tests   │ UI/Docs  │
└──────────┴──────────┴──────────┴──────────┘
        ↓
   Finished product delivered to user
```

No need for users to decompose tasks themselves or give individual instructions to multiple Claude instances.

## Hierarchy

### Command Layer
- **Overlord** - Transforms user's vague requests into technical specs & requirements
- **Strategist** - Decomposes tasks, distributes & commands the Four Heavenly Kings

### Four Heavenly Kings (Execution Layer)
| Name | Specialty | Role |
|------|-----------|------|
| Glacier | Arch & Refactor | Defines types & structures upfront, refactoring |
| Inferno | Logic & Core | Pure business logic & algorithm implementation |
| Shadow | Audit & Security | Test creation, bug hunting, vulnerability assessment |
| Storm | UI & Docs | UI implementation, documentation |

## Workflow Pipeline

The Four Heavenly Kings coordinate through this pipeline:

```
Glacier (Types) → Inferno (Logic) → Shadow (Tests)
                                  ↘ Storm (UI/Docs)
```

1. **Glacier** defines types, interfaces, and structures first
2. **Inferno** implements logic following those types
3. **Shadow** tests and debugs Inferno's code
4. **Storm** creates UI and documentation in parallel

This distributes workload across Claude instances and enables efficient development leveraging each one's specialty.

## Architecture

Each Claude instance communicates through an MCP relay server backed by file-based message storage:

```
┌─ Zellij Session ──────────────────────────────────┐
│                                                    │
│  [Overlord]   ←─ MCP ─→  ovld relay               │
│  [Strategist] ←─ MCP ─→  ovld relay               │
│  [Inferno]    ←─ MCP ─→  ovld relay               │
│  [Glacier]    ←─ MCP ─→  ovld relay               │
│  [Shadow]     ←─ MCP ─→  ovld relay               │
│  [Storm]      ←─ MCP ─→  ovld relay               │
│                    ↕                               │
│          ~/.config/ovld/relay/                     │
│          ├── inbox/{role}/     (messages)           │
│          ├── status/{role}.json                    │
│          └── pending/{role}    (notify flags)      │
│                    ↕                               │
│      zellij pipe → WASM plugin → pane STDIN        │
│      (auto-notification on new messages)           │
└────────────────────────────────────────────────────┘
```

**MCP Tools** available to each Claude instance:
| Tool | Description |
|------|-------------|
| `send_message` | Send a message to another role's inbox |
| `check_inbox` | Read unread messages (optionally mark as read) |
| `get_status` | Check a role's current status (or all roles) |
| `update_status` | Update own status (idle / working / blocked / done) |
| `broadcast` | Send a message to all other roles |

## Layout Structure

The Zellij session consists of 4 tabs:

```
┌─────────────────────────────────────────────┐
│ Tab 1: command (default focus)              │
│ ┌──────────┬────────────────────────────────┤
│ │ Overlord │        Strategist              │
│ │  (30%)   │          (70%)                 │
│ ├──────────┴────────────────────────────────┤
│ │ [notify plugin] (borderless, 1 line)      │
│ └───────────────────────────────────────────┤
├─────────────────────────────────────────────┤
│ Tab 2: battlefield                          │
│ ┌───────────────────────────────────────────┤
│ │                 Inferno                   │
│ │              (full size)                  │
│ └───────────────────────────────────────────┤
├─────────────────────────────────────────────┤
│ Tab 3: support                              │
│ ┌─────────────┬─────────────┬───────────────┤
│ │   Glacier   │   Shadow    │    Storm      │
│ │    (33%)    │    (33%)    │    (34%)      │
│ └─────────────┴─────────────┴───────────────┤
├─────────────────────────────────────────────┤
│ Tab 4: dashboard                            │
│ ┌───────────────────────────────────────────┤
│ │           ovld dashboard (TUI)            │
│ │          real-time army status            │
│ └───────────────────────────────────────────┘
```

- **command**: Headquarters. Requirements definition and task management
- **battlefield**: Main battlefield. Primary implementation work
- **support**: Support troops. Architecture, testing, documentation
- **dashboard**: Real-time TUI showing all role statuses, tasks, and recent messages

The notify plugin is a minimal WASM pane that routes inter-pane notifications without switching focus.

## Requirements

- [Rust](https://www.rust-lang.org/) (for building)
- [Zellij](https://zellij.dev/) installed
- [Claude Code](https://docs.anthropic.com/en/docs/claude-code) (`claude` CLI) available in PATH

## Installation

```bash
make install
```

This automatically installs the `wasm32-wasip1` target, builds the WASM notify plugin, and installs the `ovld` CLI.

## Usage

```bash
# Summon the Demon Army
ovld summon

# Summon with debug logging
ovld summon --debug

# Summon without sandbox (allow writes outside project directory)
ovld summon --no-sandbox

# Real-time army status dashboard (TUI)
ovld dashboard

# Check army status (one-shot)
ovld status

# Unsummon the Demon Army
ovld unsummon

# Force unsummon without confirmation
ovld unsummon --force

# Deploy/redeploy global config
ovld init
ovld init --force   # Overwrite existing config
```

Debug logs are written to `~/.config/ovld/logs/`.

## Sandbox

On macOS, `ovld summon` runs each Claude agent inside a [Seatbelt](https://reverse.put.as/wp-content/uploads/2011/09/Apple-Sandbox-Guide-v1.0.pdf) sandbox by default. File writes are restricted to prevent agents from modifying files outside the project directory.

**Allowed write paths:**
- Project directory (current working directory)
- Relay directory (`~/.config/ovld/relay/`)
- Claude config (`~/.claude/`, `~/.claude.json`)
- Claude CLI cache (`~/Library/Caches/claude-cli-nodejs/`)
- npm logs (`~/.npm/_logs/`)
- Temp directories (`/tmp`, `/var/folders`)
- Device files (`/dev`)

Agents are launched with `--dangerously-skip-permissions` to run fully autonomously — the sandbox provides kernel-level protection instead of relying on interactive permission prompts.

Use `--no-sandbox` to disable sandboxing (agents will still be launched with `--dangerously-skip-permissions`). Non-macOS platforms skip sandboxing automatically with a warning.

## Configuration

### Language Setting
CLI output messages support English and Japanese. Configure via `~/.config/ovld/config.toml`:

```toml
lang = "en"   # English (default)
# lang = "ja" # Japanese
```

Run `ovld init` to generate the default config file, or `ovld init --force` to reset it.

### Ritual Files Location
Rituals are resolved with local-first priority:
1. `./rituals/` - Project-local rituals (customize per project)
2. `~/.config/ovld/rituals/` - Global rituals (default)

### Customizing Rituals
Copy the default rituals to your project and modify:
```bash
cp -r ~/.config/ovld/rituals ./rituals
# Edit ./rituals/*.md as needed
```

## How It Works

### 1. Ritual Resolution
When `ovld summon` is executed:
1. Checks for local `./rituals/` directory first
2. Falls back to global `~/.config/ovld/rituals/` if not found
3. Auto-creates default rituals in global config on first run

### 2. Dynamic Layout Generation
1. Generates a KDL layout dynamically with absolute paths to ritual files and MCP configs
2. Each pane starts `claude --dangerously-skip-permissions --system-prompt-file <ritual> --mcp-config <mcp_config>`
3. On macOS with sandbox enabled, `claude` is wrapped with `sandbox-exec -f <profile>`
4. MCP relay tools are auto-approved via `--allowedTools "mcp__ovld-relay__*"`
5. A WASM notify plugin pane is included for inter-pane notification routing
6. Creates a temporary KDL file that's cleaned up after session ends

### 3. Session Management
1. Creates a new Zellij session with the generated layout
2. CLI blocks until the Zellij session ends (user exits or detaches)
3. Automatically cleans up EXITED sessions, metadata, and relay data on exit

### 4. MCP Relay Communication
Each Claude pane spawns an `ovld relay` process as its MCP server. The relay uses a shared file-based store:

- **Inbox**: Messages are written as JSON files to `~/.config/ovld/relay/inbox/{role}/`
- **Status**: Each role's status is stored in `~/.config/ovld/relay/status/{role}.json`
- **Pending**: Flag files in `~/.config/ovld/relay/pending/` track which roles have unread messages

Environment variables passed to each relay: `OVLD_ROLE`, `OVLD_RELAY_DIR`, `OVLD_SESSION`, `OVLD_PLUGIN_PATH`.

### 5. Auto-Notification
When a message is sent via `send_message`:
1. The message is saved to the target role's inbox
2. A pending flag is set for the target role (deduplicated — only once per check cycle)
3. `zellij pipe` sends a JSON payload to the WASM notify plugin (runs in a background thread)
4. The plugin writes a notification text directly to the target pane's STDIN
5. The receiving Claude instance sees the notification and calls `check_inbox` to retrieve messages

### 6. Operational Flow
1. Issue requirements to Overlord in **command** tab
2. Strategist decomposes tasks and directs Four Heavenly Kings
3. **Inferno** does main implementation in **battlefield** tab
4. **Glacier/Shadow/Storm** provide support in **support** tab
5. Roles communicate autonomously via MCP relay tools

## Directory Structure

```
overlord-zellij/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Library exports & constants
│   ├── config.rs         # Config, ritual resolution, MCP config generation
│   ├── layout.rs         # Dynamic KDL layout generation
│   ├── sandbox.rs        # macOS Seatbelt sandbox profile generation
│   ├── logging.rs        # Debug logging (--debug)
│   ├── i18n.rs           # i18n (en/ja)
│   ├── commands/         # summon / unsummon / status / init / dashboard
│   ├── zellij/           # Zellij session management
│   ├── army/             # Role definitions & icons
│   └── relay/            # MCP relay server
│       ├── server.rs     # 5 MCP tools (send_message, check_inbox, etc.)
│       ├── store.rs      # File-based message persistence
│       ├── notify.rs     # Zellij pipe notification
│       └── types.rs      # Message, RoleStatus, Priority types
├── plugin/               # Zellij WASM plugin (pane notification)
├── rituals/              # System prompts for each role
│   ├── overlord.md
│   ├── strategist.md
│   ├── inferno.md
│   ├── glacier.md
│   ├── shadow.md
│   └── storm.md
└── openspec/             # Specification documents
```

## Specifications

For detailed specifications, see `openspec/specs/`:
- `ovld-cli/` - CLI command specification
- `army-hierarchy/` - Hierarchy & role specification
- `zellij-session/` - Session management & layout specification
- `config-management/` - Global config & ritual resolution specification
- `dashboard/` - Real-time TUI dashboard specification
- `sandbox/` - macOS Seatbelt sandbox specification
- `i18n/` - Internationalization (en/ja) specification
- `ritual-injection/` - Prompt injection specification
- `workflow-protocol/` - Four Heavenly Kings coordination protocol
- `mcp-relay/` - MCP relay server specification
- `auto-notification/` - Inter-pane auto-notification specification
