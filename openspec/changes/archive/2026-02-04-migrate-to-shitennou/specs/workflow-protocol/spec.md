## ADDED Requirements

### Requirement: Glacier provides structure first
氷結の将 SHALL define types, interfaces, and directory structure before Inferno begins implementation.

#### Scenario: Type definition handoff
- **WHEN** a new feature is requested
- **THEN** Glacier creates trait/struct definitions
- **THEN** Inferno implements logic using those definitions

### Requirement: Inferno focuses on pure logic
業火の将 SHALL focus exclusively on business logic and algorithms, without handling UI or documentation.

#### Scenario: Logic implementation
- **WHEN** Inferno receives type definitions from Glacier
- **THEN** Inferno implements core logic only
- **THEN** Inferno does not write CSS, README, or UI code

### Requirement: Shadow handles testing
常闘の将 SHALL generate test code and perform debugging on behalf of Inferno.

#### Scenario: Test generation
- **WHEN** Inferno completes a logic implementation
- **THEN** Shadow creates test cases for that implementation
- **THEN** Shadow reports bugs for Inferno to fix

### Requirement: Storm handles externals
疾風の将 SHALL create all UI components, documentation, and user-facing content in parallel.

#### Scenario: Documentation creation
- **WHEN** Inferno's logic is available
- **THEN** Storm creates README and API documentation
- **THEN** Storm implements frontend components if needed

### Requirement: Pipeline instructions in rituals
Each ritual prompt SHALL include instructions about what to receive from and pass to other generals.

#### Scenario: Ritual contains pipeline
- **WHEN** ritual is injected to a general
- **THEN** the prompt includes "receive from X" and "pass to Y" instructions
