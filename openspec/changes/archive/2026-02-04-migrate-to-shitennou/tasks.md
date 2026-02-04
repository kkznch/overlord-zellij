## 1. Role System Update

- [x] 1.1 Update `src/army/roles.rs` with new Role enum (Overlord, Strategist, Inferno, Glacier, Shadow, Storm)
- [x] 1.2 Update display_name() for each role with Japanese/English names
- [x] 1.3 Update ritual_file() mapping for new files
- [x] 1.4 Update pane_name() for new tab/pane structure

## 2. KDL Layout

- [x] 2.1 Replace `layouts/army.kdl` with 3-tab layout (command, battlefield, support)
- [x] 2.2 Command tab: Overlord + Strategist vertical split
- [x] 2.3 Battlefield tab: Single Inferno pane with focus=true
- [x] 2.4 Support tab: Glacier + Shadow + Storm horizontal 3-split

## 3. Ritual Files

- [x] 3.1 Delete old ritual files (overlord.md, strategist.md, legion_*.md)
- [x] 3.2 Create rituals/overlord.md with command layer prompt
- [x] 3.3 Create rituals/strategist.md with planning layer prompt
- [x] 3.4 Create rituals/inferno.md with Logic & Core prompt + workflow instructions
- [x] 3.5 Create rituals/glacier.md with Arch & Refactor prompt + workflow instructions
- [x] 3.6 Create rituals/shadow.md with Audit & Security prompt + workflow instructions
- [x] 3.7 Create rituals/storm.md with UI & Docs prompt + workflow instructions

## 4. Ritual Injection Logic

- [x] 4.1 Update `src/army/ritual.rs` for new tab navigation (command/battlefield/support)
- [x] 4.2 Update pane focus logic for 3-tab structure

## 5. Verification

- [x] 5.1 Build and verify no compilation errors
- [x] 5.2 Run `ovld status` and verify new role display
