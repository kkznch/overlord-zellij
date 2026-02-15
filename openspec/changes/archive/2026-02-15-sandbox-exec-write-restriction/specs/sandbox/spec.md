## ADDED Requirements

### Requirement: Seatbelt profile generation
The system SHALL generate a macOS Seatbelt (.sb) profile that denies all `file-write*` operations by default and allows writes only to specified directories.

#### Scenario: Profile contains deny-all write rule
- **WHEN** `generate_profile(cwd, relay_dir)` is called
- **THEN** the output contains `(deny file-write* (subpath "/"))`

#### Scenario: Profile allows writes to project directory
- **WHEN** `generate_profile(cwd, relay_dir)` is called with `cwd = "/home/user/project"`
- **THEN** the output contains `(allow file-write* (subpath "/home/user/project"))`

#### Scenario: Profile allows writes to relay directory
- **WHEN** `generate_profile(cwd, relay_dir)` is called with `relay_dir = "/home/user/.config/ovld/relay"`
- **THEN** the output contains `(allow file-write* (subpath "/home/user/.config/ovld/relay"))`

#### Scenario: Profile allows writes to Claude config
- **WHEN** `generate_profile(cwd, relay_dir)` is called
- **THEN** the output contains `(allow file-write* (subpath "{HOME}/.claude"))`

#### Scenario: Profile allows writes to temp directories
- **WHEN** `generate_profile(cwd, relay_dir)` is called
- **THEN** the output contains allow rules for `/tmp`, `/private/tmp`, and `std::env::temp_dir()`

#### Scenario: Profile allows writes to standard devices
- **WHEN** `generate_profile(cwd, relay_dir)` is called
- **THEN** the output contains allow rules for `/dev/null` and `/dev/tty`

#### Scenario: Profile uses allow-default base
- **WHEN** `generate_profile(cwd, relay_dir)` is called
- **THEN** the output contains `(allow default)` so reads and non-file operations are unrestricted

### Requirement: Temporary profile file creation
The system SHALL write the generated Seatbelt profile to a temporary file with `.sb` extension that is automatically deleted when dropped.

#### Scenario: Temp file creation
- **WHEN** `create_temp_profile(cwd, relay_dir)` is called
- **THEN** a file with `.sb` extension is created containing the profile
- **THEN** the file contains `(version 1)` and `(deny file-write*`

#### Scenario: Temp file cleanup
- **WHEN** the returned `NamedTempFile` handle is dropped
- **THEN** the temporary `.sb` file is deleted from disk
