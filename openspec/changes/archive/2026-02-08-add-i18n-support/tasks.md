## 1. 依存追加

- [x] 1.1 `Cargo.toml` に `toml` クレートを追加

## 2. i18n モジュール

- [x] 2.1 `src/i18n.rs` を新規作成（`Lang` enum、`Msg` enum、各メッセージの en/ja テキスト）
- [x] 2.2 `src/main.rs` に `mod i18n;` を追加

## 3. config.toml 対応

- [x] 3.1 `src/config.rs` に `AppConfig` 構造体と `load_config()` / `save_default_config()` を追加
- [x] 3.2 `src/commands/init.rs` に `config.toml` 生成を追加

## 4. コマンドの i18n 対応

- [x] 4.1 `src/commands/summon.rs` のメッセージを i18n 経由に変更
- [x] 4.2 `src/commands/unsummon.rs` のメッセージを i18n 経由に変更
- [x] 4.3 `src/commands/status.rs` のメッセージを i18n 経由に変更
- [x] 4.4 `src/commands/init.rs` のメッセージを i18n 経由に変更

## 5. 動作確認

- [x] 5.1 `cargo build` が成功することを確認
- [x] 5.2 `cargo test` が成功することを確認
- [x] 5.3 `ovld --help` が正常表示されることを確認
