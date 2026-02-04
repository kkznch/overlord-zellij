## MODIFIED Requirements

### Requirement: Six ritual files
The rituals directory SHALL contain six files: overlord.md, strategist.md, inferno.md, glacier.md, shadow.md, storm.md.

#### Scenario: Ritual files exist
- **WHEN** summon command runs
- **THEN** system reads from rituals/overlord.md for Overlord
- **THEN** system reads from rituals/inferno.md for Inferno
- **THEN** system reads from rituals/glacier.md for Glacier
- **THEN** system reads from rituals/shadow.md for Shadow
- **THEN** system reads from rituals/storm.md for Storm

### Requirement: Tab-based navigation for injection
The system SHALL navigate to correct tab before injecting to generals.

#### Scenario: Command tab injection
- **WHEN** injecting to Overlord or Strategist
- **THEN** system focuses "command" tab

#### Scenario: Battlefield tab injection
- **WHEN** injecting to Inferno
- **THEN** system focuses "battlefield" tab

#### Scenario: Support tab injection
- **WHEN** injecting to Glacier, Shadow, or Storm
- **THEN** system focuses "support" tab
- **THEN** system uses focus-next-pane to reach correct general

### Requirement: Workflow instructions in prompts
Each general's ritual SHALL include their role in the workflow pipeline.

#### Scenario: Glacier prompt content
- **WHEN** Glacier ritual is loaded
- **THEN** prompt includes "define types and structures first"
- **THEN** prompt includes "pass definitions to Inferno"

#### Scenario: Inferno prompt content
- **WHEN** Inferno ritual is loaded
- **THEN** prompt includes "receive structures from Glacier"
- **THEN** prompt includes "focus on pure logic only"

#### Scenario: Shadow prompt content
- **WHEN** Shadow ritual is loaded
- **THEN** prompt includes "receive implementation from Inferno"
- **THEN** prompt includes "generate tests and report bugs"

#### Scenario: Storm prompt content
- **WHEN** Storm ritual is loaded
- **THEN** prompt includes "receive logic from Inferno"
- **THEN** prompt includes "create UI and documentation"

## REMOVED Requirements

### Requirement: Legion ritual files
**Reason**: Replaced by Four Generals ritual files
**Migration**: Delete legion_impl.md, legion_debug.md, legion_docs.md; create inferno.md, glacier.md, shadow.md, storm.md
