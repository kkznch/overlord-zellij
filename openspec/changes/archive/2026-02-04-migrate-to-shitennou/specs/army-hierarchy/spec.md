## MODIFIED Requirements

### Requirement: Six distinct roles
The system SHALL define six roles: Overlord, Strategist, Inferno, Glacier, Shadow, Storm.

#### Scenario: Role enumeration
- **WHEN** system initializes
- **THEN** all six roles are available with display names in Japanese and English

### Requirement: Four Generals (Shitennou)
The system SHALL define four generals replacing the three legions: Inferno (業火), Glacier (氷結), Shadow (常闇), Storm (疾風).

#### Scenario: Generals display
- **WHEN** status command lists roles
- **THEN** Inferno displays as "業火の将 (Inferno)" with fire icon
- **THEN** Glacier displays as "氷結の将 (Glacier)" with ice icon
- **THEN** Shadow displays as "常闘の将 (Shadow)" with shadow icon
- **THEN** Storm displays as "疾風の将 (Storm)" with wind icon

### Requirement: Four-layer hierarchy
The system SHALL maintain four layers: User (深淵の意志) → Overlord (魔王) → Strategist (軍師) → Four Generals (四天王).

#### Scenario: Hierarchy display
- **WHEN** status command shows hierarchy
- **THEN** Overlord and Strategist appear as command layer
- **THEN** Four Generals appear as execution layer

### Requirement: Role ritual files
Each role SHALL have a corresponding ritual file.

#### Scenario: Ritual file mapping
- **WHEN** role is Inferno
- **THEN** ritual file is "inferno.md"
- **WHEN** role is Glacier
- **THEN** ritual file is "glacier.md"
- **WHEN** role is Shadow
- **THEN** ritual file is "shadow.md"
- **WHEN** role is Storm
- **THEN** ritual file is "storm.md"

## REMOVED Requirements

### Requirement: Three legions
**Reason**: Replaced by Four Generals (Shitennou) system
**Migration**: LegionImpl → Inferno, LegionDebug → Shadow, LegionDocs → Storm, (new) Glacier
