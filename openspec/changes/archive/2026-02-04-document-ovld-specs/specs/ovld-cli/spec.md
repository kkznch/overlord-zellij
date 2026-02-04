## ADDED Requirements

### Requirement: summon command starts session
The system SHALL provide a `summon` subcommand that starts a new Zellij session with the army layout.

#### Scenario: New session creation
- **WHEN** user runs `ovld summon`
- **THEN** system creates a Zellij session named "overlord" with the army.kdl layout

#### Scenario: Session already exists
- **WHEN** user runs `ovld summon` and session "overlord" already exists
- **THEN** system attaches to the existing session instead of creating a new one

### Requirement: slay command terminates session
The system SHALL provide a `slay` subcommand that terminates the army session and cleans up processes.

#### Scenario: Session termination with confirmation
- **WHEN** user runs `ovld slay`
- **THEN** system prompts for confirmation before terminating

#### Scenario: Force termination
- **WHEN** user runs `ovld slay --force`
- **THEN** system terminates the session without confirmation

#### Scenario: Session not found
- **WHEN** user runs `ovld slay` and session does not exist
- **THEN** system displays error message "Session 'overlord' not found"

### Requirement: status command shows army state
The system SHALL provide a `status` subcommand that displays the current state of the army session.

#### Scenario: Active session
- **WHEN** user runs `ovld status` and session exists
- **THEN** system displays session name, status "ACTIVE", and army hierarchy listing

#### Scenario: No session
- **WHEN** user runs `ovld status` and session does not exist
- **THEN** system displays status "NOT FOUND" and hint to use summon command

### Requirement: Custom session name
The system SHALL allow specifying a custom session name via `--session` flag.

#### Scenario: Custom session name
- **WHEN** user runs `ovld summon --session myarmy`
- **THEN** system creates session with name "myarmy"

### Requirement: Custom layout
The system SHALL allow specifying a custom layout via `--layout` flag.

#### Scenario: Custom layout file
- **WHEN** user runs `ovld summon --layout minimal`
- **THEN** system uses layouts/minimal.kdl for the session
