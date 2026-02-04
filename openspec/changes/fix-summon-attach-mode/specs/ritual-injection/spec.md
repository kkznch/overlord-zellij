## MODIFIED Requirements

### Requirement: Ritual file loading
The system SHALL load ritual content from markdown files in the resolved rituals directory.

#### Scenario: Load ritual
- **WHEN** generating KDL layout
- **THEN** system resolves ritual file paths using config-management resolution order

#### Scenario: Missing ritual file
- **WHEN** ritual file does not exist in resolved directory
- **THEN** system returns error with expected file path

### Requirement: Six ritual files
The rituals directory SHALL contain six files: overlord.md, strategist.md, inferno.md, glacier.md, shadow.md, storm.md.

#### Scenario: Ritual files exist
- **WHEN** summon command runs
- **THEN** system verifies overlord.md, strategist.md, inferno.md, glacier.md, shadow.md, storm.md exist

### Requirement: KDL-based ritual injection
The system SHALL inject rituals via KDL pane command configuration instead of external write-chars.

#### Scenario: Ritual injection method
- **WHEN** session starts
- **THEN** each pane's `command` directive specifies `claude --system-prompt-file <path>`
- **THEN** Claude starts with ritual already loaded

## REMOVED Requirements

### Requirement: Tab-based navigation for injection
**Reason**: No longer needed - rituals are injected via KDL startup, not external commands
**Migration**: Remove tab navigation logic from ritual.rs

### Requirement: Text injection via write-chars
**Reason**: Replaced by KDL command directive approach
**Migration**: Remove write-chars injection from ritual.rs

### Requirement: Injection timing
**Reason**: No longer needed - KDL handles startup timing
**Migration**: Remove delay logic from ritual.rs

### Requirement: Skip ritual option
**Reason**: Behavior changes - without rituals means bare bash panes
**Migration**: `--no-rituals` flag now generates KDL with `command "bash"` instead of `command "claude"`
