## Why

rituals ファイルを更新してバイナリを再ビルドしても、`~/.config/ovld/rituals/` にある古いファイルは上書きされない。`ensure_default_config()` はディレクトリが存在すればスキップするため、グローバル設定を最新に同期する手段がない。

## What Changes

- `ovld init` サブコマンドを追加し、グローバル設定を（再）展開できるようにする
- `--force` フラグで既存ファイルを強制上書き
- フラグなしの場合は既存ディレクトリがあればスキップ（現在の `ensure_default_config` と同じ動作）

## Capabilities

### New Capabilities

（なし）

### Modified Capabilities

- `ovld-cli`: `init` サブコマンドを追加
- `config-management`: `--force` によるグローバル設定の強制再展開要件を追加

## Impact

- `src/main.rs` — `Init` サブコマンド追加
- `src/commands/` — `init.rs` 新規追加
- `src/config.rs` — 変更なし（既存の `extract_rituals_to` を利用）
