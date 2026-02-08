## ADDED Requirements

### Requirement: Global config directory
The system SHALL use `~/.config/ovld/` as the global configuration directory.

#### Scenario: Config directory location
- **WHEN** system needs to access global configuration
- **THEN** system uses `$HOME/.config/ovld/` path

### Requirement: Default rituals directory
The global config SHALL contain a `rituals/` subdirectory with default ritual files.

#### Scenario: Default rituals location
- **WHEN** system looks for global rituals
- **THEN** system checks `~/.config/ovld/rituals/`

### Requirement: Rituals resolution order
The system SHALL resolve ritual files with local-first priority: `./rituals/` → `~/.config/ovld/rituals/`.

#### Scenario: Local rituals exist
- **WHEN** `./rituals/` directory exists in current working directory
- **THEN** system uses ritual files from `./rituals/`

#### Scenario: Local rituals missing
- **WHEN** `./rituals/` directory does not exist
- **THEN** system falls back to `~/.config/ovld/rituals/`

### Requirement: Auto-create default config
The system SHALL automatically create the global config directory with default rituals on first use.

#### Scenario: First run without config
- **WHEN** `ovld summon` runs and `~/.config/ovld/` does not exist
- **THEN** system creates `~/.config/ovld/rituals/`
- **THEN** system extracts embedded default rituals to that directory

### Requirement: Embedded default rituals
The binary SHALL contain embedded default ritual files using `include_str!`.

#### Scenario: Default ritual content
- **WHEN** system needs to create default rituals
- **THEN** system writes embedded content for overlord.md, strategist.md, inferno.md, glacier.md, shadow.md, storm.md

### Requirement: Config directory validation
The system SHALL validate that required ritual files exist in the resolved directory.

#### Scenario: Missing ritual files
- **WHEN** resolved rituals directory is missing required files
- **THEN** system returns error listing missing files
