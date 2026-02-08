## Why

`slay`（撃滅）は `summon`（召喚）の対義語として不自然。`unsummon`（召喚解除）にリネームすることで、summon ↔ unsummon の対称性を明確にし、コマンド体系の直感性を向上させる。

## What Changes

- **BREAKING**: `ovld slay` コマンドを `ovld unsummon` にリネーム
- `--force` フラグはそのまま維持
- CLI ヘルプ・エラーメッセージ内の「撃滅」表現を「還送」に統一
- ファイル名 `src/commands/slay.rs` → `src/commands/unsummon.rs`

## Capabilities

### New Capabilities

(なし)

### Modified Capabilities

- `ovld-cli`: `slay` サブコマンドを `unsummon` にリネーム、ヘルプテキスト変更
- `zellij-session`: slay 関連の要件記述を unsummon に更新

## Impact

- `src/commands/slay.rs` → `src/commands/unsummon.rs` (リネーム)
- `src/commands/mod.rs` — モジュール宣言変更
- `src/main.rs` — Commands enum, use 文, match 分岐
- `src/commands/summon.rs`, `src/commands/status.rs` — エラーメッセージ・ヒントテキスト内の slay 参照
- `openspec/specs/ovld-cli/spec.md`, `openspec/specs/zellij-session/spec.md` — スペック内の slay 記述
