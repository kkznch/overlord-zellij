---
name: ovld-grow
description: >-
  Self-growth skill for the Overlord army. Automatically records learnings,
  patterns, and debugging insights to persistent memory during development.
  When sufficient insights accumulate on a topic, proposes crystallizing them
  into a new dedicated Skill. Auto-trigger: after resolving bugs, discovering
  non-obvious patterns, or gaining architectural understanding.
---

# Overlord Self-Growth

Record learnings from development sessions to build collective knowledge that persists across sessions.

## Two-Phase Growth Model

### Phase 1: Accumulation
Record insights to auto memory as topic-specific files.

**Storage**: `~/.claude/projects/.../memory/`
- `MEMORY.md` — summary index (loaded into system prompt, keep under 200 lines)
- `{topic}.md` — detailed notes per topic (e.g., `zellij.md`, `relay.md`, `rust-patterns.md`)

### Phase 2: Crystallization
When a topic file has 3+ distinct insights, evaluate whether it should become a dedicated Skill.

**Output**: `.claude/skills/ovld-{topic}/SKILL.md`

## Auto-Trigger Conditions

Record an insight automatically (no user confirmation needed) when:

1. **Bug resolved**: A non-trivial debugging session completes — record the root cause, symptoms, and fix
2. **Pattern discovered**: A reusable code pattern, architectural convention, or idiom is found in the codebase
3. **Gotcha encountered**: An API, library, or tool behaves unexpectedly
4. **Architecture understood**: A key design decision or module relationship becomes clear
5. **Performance insight**: A bottleneck is identified or an optimization technique is discovered

## Recording Format

When recording to a topic file in auto memory:

```markdown
### <Title> (<date>)

**Category**: architecture | debugging | pattern | gotcha | performance
**Context**: <brief description of what was being done>

<What was learned, why it matters, how to apply it>
```

## MEMORY.md Update Rules

After recording to a topic file, update MEMORY.md if:
- A new topic file was created (add a one-line summary)
- An existing topic gained significant new insight (update the summary)

Keep MEMORY.md concise — it is loaded into every session's system prompt.

## Crystallization Rules

When evaluating whether to create a new Skill from accumulated insights:

1. Check if the topic file has 3+ distinct, actionable insights
2. Check if the insights form a coherent domain of expertise
3. If yes, propose to the user: "Topic '{topic}' has enough insights to become a dedicated Skill. Create `.claude/skills/ovld-{topic}/SKILL.md`?"
4. Only create after user confirmation
5. The new Skill should encode the accumulated knowledge as actionable instructions

## Manual Trigger (/ovld-grow)

When invoked manually:

1. If arguments are provided, use them as the insight to record
2. If no arguments, review the current session for unrecorded insights
3. Show what will be recorded and to which topic file
4. Record without additional confirmation (manual trigger implies intent)
5. Check crystallization conditions

## Army Integration

The army roles (overlord, strategist, shitennoh) share insights during sessions via
`share_insight` / `query_insights` MCP tools. These persist at `~/.config/ovld/knowledge/`.

When working in Claude Code (outside army sessions), also check the army's knowledge base
if relevant to the current task. The two systems complement each other:

- **Army knowledge** (`~/.config/ovld/knowledge/`): shared across all roles during sessions
- **Auto memory** (`~/.claude/projects/.../memory/`): loaded into Claude Code system prompt
- **Crystallized Skills** (`.claude/skills/ovld-*/`): structured, actionable expertise
