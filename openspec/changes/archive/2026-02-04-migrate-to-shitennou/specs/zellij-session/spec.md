## MODIFIED Requirements

### Requirement: Three-tab layout structure
The army.kdl layout SHALL define three tabs: command, battlefield, support.

#### Scenario: Tab structure
- **WHEN** layout is loaded
- **THEN** command tab contains Overlord and Strategist panes (vertical split)
- **THEN** battlefield tab contains single Inferno pane (large workspace)
- **THEN** support tab contains Glacier, Shadow, Storm panes (horizontal 3-split)

### Requirement: Six panes total
The layout SHALL provide six panes for six roles.

#### Scenario: Pane count
- **WHEN** session starts with army layout
- **THEN** exactly six panes exist across three tabs

### Requirement: Command tab layout
The command tab SHALL have Overlord on left and Strategist on right.

#### Scenario: Command tab
- **WHEN** user views command tab
- **THEN** Overlord pane is on the left (smaller)
- **THEN** Strategist pane is on the right (larger)

### Requirement: Battlefield tab layout
The battlefield tab SHALL have a single large pane for Inferno.

#### Scenario: Battlefield tab
- **WHEN** user views battlefield tab
- **THEN** single pane named "inferno" fills the tab
- **THEN** this tab has focus=true by default

### Requirement: Support tab layout
The support tab SHALL have three equal panes for Glacier, Shadow, Storm.

#### Scenario: Support tab
- **WHEN** user views support tab
- **THEN** three horizontal panes exist (33%/33%/34%)
- **THEN** order is Glacier, Shadow, Storm from left to right
