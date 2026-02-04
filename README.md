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

## Layout Structure

The Zellij session consists of 3 tabs:

```
┌─────────────────────────────────────────────┐
│ Tab 1: command                              │
│ ┌──────────┬────────────────────────────────┤
│ │ Overlord │        Strategist              │
│ │  (30%)   │          (70%)                 │
│ └──────────┴────────────────────────────────┤
├─────────────────────────────────────────────┤
│ Tab 2: battlefield (default focus)          │
│ ┌───────────────────────────────────────────┤
│ │                 Inferno                   │
│ │              (full size)                  │
│ └───────────────────────────────────────────┤
├─────────────────────────────────────────────┤
│ Tab 3: support                              │
│ ┌─────────────┬─────────────┬───────────────┤
│ │   Glacier   │   Shadow    │    Storm      │
│ │    (33%)    │    (33%)    │    (34%)      │
│ └─────────────┴─────────────┴───────────────┘
```

- **command**: Headquarters. Requirements definition and task management
- **battlefield**: Main battlefield. Primary implementation work
- **support**: Support troops. Architecture, testing, documentation

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Summon the Demon Army
ovld summon

# Check army status
ovld status

# Slay the Demon Army
ovld slay
```

### Options

```bash
# Custom session name
ovld summon --session myarmy

# Skip ritual injection (starts bash instead of Claude)
ovld summon --no-rituals

# Force kill without confirmation
ovld slay --force
```

## How It Works

### 1. Ritual Resolution
When `ovld summon` is executed:
1. Checks for local `./rituals/` directory first
2. Falls back to global `~/.config/ovld/rituals/` if not found
3. Auto-creates default rituals in global config on first run

### 2. Dynamic Layout Generation
1. Generates KDL layout dynamically with absolute paths to ritual files
2. Each pane starts `claude --system-prompt-file <ritual_path>`
3. Creates temporary KDL file that's cleaned up after session ends

### 3. Session Management
1. If session already exists, attaches to it
2. Otherwise, creates new Zellij session with generated layout
3. CLI blocks until Zellij session ends (user exits or detaches)

### 4. Operational Flow
1. Issue requirements to Overlord in **command** tab
2. Strategist decomposes tasks and directs Four Heavenly Kings
3. **Inferno** does main implementation in **battlefield** tab
4. **Glacier/Shadow/Storm** provide support in **support** tab

## Requirements

- [Zellij](https://zellij.dev/) installed
- `claude` CLI available in PATH

## Directory Structure

```
overlord-zellij/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── config.rs         # Config & ritual resolution
│   ├── layout.rs         # Dynamic KDL generation
│   ├── commands/         # summon/slay/status commands
│   ├── zellij/           # Zellij session management
│   └── army/             # Role definitions
├── layouts/
│   └── army.kdl          # Zellij layout definition (reference)
├── rituals/              # System prompts for each role
│   ├── overlord.md
│   ├── strategist.md
│   ├── inferno.md
│   ├── glacier.md
│   ├── shadow.md
│   └── storm.md
└── openspec/             # Specification documents
```

## Configuration

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

## Specifications

For detailed specifications, see `openspec/specs/`:
- `ovld-cli/` - CLI command specification
- `army-hierarchy/` - Hierarchy & role specification
- `zellij-session/` - Session management & layout specification
- `ritual-injection/` - Prompt injection specification
- `workflow-protocol/` - Four Heavenly Kings coordination protocol
