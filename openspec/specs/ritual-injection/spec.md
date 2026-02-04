## ADDED Requirements

### Requirement: Ritual file loading
The system SHALL load ritual content from markdown files in the rituals/ directory.

#### Scenario: Load ritual
- **WHEN** ritual injection starts for a role
- **THEN** system reads content from rituals/<role_ritual_file>

#### Scenario: Missing ritual file
- **WHEN** ritual file does not exist
- **THEN** system returns error with file path

### Requirement: Pane targeting via tabs
The system SHALL navigate to the correct pane by first focusing the appropriate tab.

#### Scenario: Target overlord pane
- **WHEN** injecting to Overlord
- **THEN** system runs `zellij action go-to-tab-name overlord`

#### Scenario: Target legion panes
- **WHEN** injecting to LegionDebug
- **THEN** system focuses legions tab, then runs `focus-next-pane` once

### Requirement: Text injection via write-chars
The system SHALL inject prompts using Zellij's write-chars action.

#### Scenario: Write prompt text
- **WHEN** pane is focused
- **THEN** system runs `zellij --session <name> action write-chars <text>`

#### Scenario: Execute prompt
- **WHEN** text is written
- **THEN** system sends Enter key via `zellij action write 13`

### Requirement: Injection timing
The system SHALL include delays between operations to ensure pane readiness.

#### Scenario: Pre-injection delay
- **WHEN** about to inject prompt
- **THEN** system waits 500ms before writing

#### Scenario: Inter-role delay
- **WHEN** injecting to multiple roles
- **THEN** system waits 1 second between each role

### Requirement: Skip ritual option
The system SHALL allow skipping ritual injection via --no-rituals flag.

#### Scenario: Skip rituals
- **WHEN** user runs `ovld summon --no-rituals`
- **THEN** system creates session without injecting any prompts
