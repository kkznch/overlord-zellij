## 1. init コマンドの追加

- [x] 1.1 `src/commands/init.rs` を新規作成（`execute(force: bool)` 関数）
- [x] 1.2 `src/commands/mod.rs` に `pub mod init;` を追加
- [x] 1.3 `src/main.rs` に `Init` サブコマンドを追加（`--force` フラグ付き）

## 2. 動作確認

- [x] 2.1 `cargo build` が成功することを確認
- [x] 2.2 `cargo test` が成功することを確認
- [x] 2.3 `ovld init --help` でヘルプが表示されることを確認
