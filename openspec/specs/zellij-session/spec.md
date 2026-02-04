## ADDED Requirements

### Requirement: Session creation with layout
The system SHALL create Zellij sessions using the specified KDL layout file.

#### Scenario: Start session
- **WHEN** summon command executes
- **THEN** system runs `zellij --session <name> --layout <path>`

### Requirement: Session existence check
The system SHALL check whether a session already exists before creating a new one.

#### Scenario: Check existing session
- **WHEN** system checks for session
- **THEN** system runs `zellij list-sessions` and searches for session name in output

### Requirement: Session termination
The system SHALL terminate sessions by killing and optionally deleting them.

#### Scenario: Kill session
- **WHEN** slay command executes
- **THEN** system runs `zellij kill-session <name>`

#### Scenario: Delete session data
- **WHEN** slay command completes
- **THEN** system runs `zellij delete-session <name> --force` for cleanup

### Requirement: Session attachment
The system SHALL attach to existing sessions instead of creating duplicates.

#### Scenario: Attach to session
- **WHEN** session exists and summon is called
- **THEN** system runs `zellij attach <name>`

### Requirement: KDL layout structure
The army.kdl layout SHALL define three tabs: overlord, strategist, and legions.

#### Scenario: Tab structure
- **WHEN** layout is loaded
- **THEN** overlord tab contains single pane with focus=true
- **THEN** strategist tab contains single pane
- **THEN** legions tab contains three horizontal panes (33%/33%/34%)
