## ADDED Requirements

### Requirement: Five distinct roles
The system SHALL define five roles in the army hierarchy: Overlord, Strategist, LegionImpl, LegionDebug, LegionDocs.

#### Scenario: Role enumeration
- **WHEN** system initializes
- **THEN** all five roles are available with their display names in Japanese and English

### Requirement: Overlord role
The Overlord (魔王) SHALL translate user wishes into technical specifications and provide final judgment on deliverables.

#### Scenario: Overlord display
- **WHEN** status command lists roles
- **THEN** Overlord displays as "魔王 (Overlord)" with crown icon

### Requirement: Strategist role
The Strategist (闇の軍師) SHALL break down Overlord commands into concrete tasks and coordinate the legions.

#### Scenario: Strategist display
- **WHEN** status command lists roles
- **THEN** Strategist displays as "闘の軍師 (Dark Strategist)" with sword icon

### Requirement: Legion roles
The three legions SHALL have specialized responsibilities: implementation (剛腕), debugging (処刑), and documentation (記録).

#### Scenario: Legion display
- **WHEN** status command lists roles
- **THEN** LegionImpl displays as "第一兵団・剛腕" with muscle icon
- **THEN** LegionDebug displays as "第二兵団・処刑" with fire icon
- **THEN** LegionDocs displays as "第三兵団・記録" with scroll icon

### Requirement: Role ritual files
Each role SHALL have a corresponding ritual file that defines its system prompt.

#### Scenario: Ritual file mapping
- **WHEN** role is Overlord
- **THEN** ritual file is "overlord.md"
- **WHEN** role is Strategist
- **THEN** ritual file is "strategist.md"
- **WHEN** role is LegionImpl
- **THEN** ritual file is "legion_impl.md"
