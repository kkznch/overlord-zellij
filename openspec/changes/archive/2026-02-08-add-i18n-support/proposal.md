## Why

CLI の出力メッセージが日本語にハードコードされている。英語圏のユーザーや英語環境での利用を想定し、表示言語を切り替え可能にしたい。デフォルトは英語とし、設定で日本語に変更できるようにする。

## What Changes

- `~/.config/ovld/config.toml` に `lang` フィールドを追加（`en` / `ja`、デフォルト `en`）
- 各コマンドの出力メッセージを言語設定に応じて切り替える i18n モジュールを追加
- `ovld init` 時に `config.toml` も生成する

## Capabilities

### New Capabilities

- `i18n`: CLI 出力メッセージの多言語対応（メッセージ定義、言語解決、メッセージ取得）

### Modified Capabilities

- `config-management`: `config.toml` に `lang` フィールドを追加し、init 時に生成
- `ovld-cli`: 全サブコマンドの出力メッセージを i18n 経由に変更

## Impact

- `src/i18n.rs` — 新規追加（メッセージ定義・取得）
- `src/config.rs` — config.toml の読み書き追加
- `src/commands/*.rs` — 全コマンドのメッセージを i18n 経由に変更
- `src/commands/init.rs` — config.toml 生成を追加
