## MODIFIED Requirements

### Requirement: Session creation with layout
The system SHALL create Zellij sessions using a dynamically generated KDL layout and immediately attach.

#### Scenario: Start session
- **WHEN** summon command executes
- **THEN** system generates KDL layout with absolute paths to ritual files
- **THEN** system writes layout to temporary file
- **THEN** system runs `zellij --session <name> --layout <temp_path>` and blocks until Zellij exits

### Requirement: Dynamic layout generation
The system SHALL generate KDL layout at runtime with absolute paths.

#### Scenario: Generate layout
- **WHEN** preparing to start session
- **THEN** system resolves rituals directory (local or global)
- **THEN** system generates KDL with absolute paths to ritual files
- **THEN** system includes `command "claude"` with `args ["--system-prompt-file", "<absolute_path>"]` for each pane

## ADDED Requirements

### Requirement: Panes start Claude directly
Each pane in the layout SHALL start Claude CLI with the appropriate ritual file.

#### Scenario: Overlord pane startup
- **WHEN** session starts
- **THEN** overlord pane runs `claude --system-prompt-file /path/to/rituals/overlord.md`

#### Scenario: All panes start Claude
- **WHEN** session starts
- **THEN** each of the six panes starts claude with its respective ritual file
- **THEN** no manual Claude startup is required

### Requirement: Temporary layout cleanup
The system SHALL clean up temporary layout files after session creation.

#### Scenario: Cleanup after attach
- **WHEN** Zellij session ends (user exits or detaches)
- **THEN** system deletes the temporary KDL file
