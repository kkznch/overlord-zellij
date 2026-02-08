## 1. ファイルリネーム

- [x] 1.1 `src/commands/slay.rs` を `src/commands/unsummon.rs` に `git mv` でリネームする
- [x] 1.2 `src/commands/mod.rs` の `pub mod slay` を `pub mod unsummon` に変更する

## 2. main.rs の更新

- [x] 2.1 `use commands::{slay, ...}` を `use commands::{unsummon, ...}` に変更する
- [x] 2.2 `Commands` enum の `Slay` を `Unsummon` に変更し、ヘルプテキストを「還送」に更新する
- [x] 2.3 match 分岐の `Commands::Slay` → `Commands::Unsummon`、`slay::execute` → `unsummon::execute` に変更する

## 3. lib.rs の更新

- [x] 3.1 `slay` モジュール参照がある場合 `unsummon` に変更する（commands 経由のため不要の可能性あり）

## 4. メッセージテキストの更新

- [x] 4.1 `src/commands/unsummon.rs` 内の「撃滅」を「還送」に変更する
- [x] 4.2 `src/commands/summon.rs` 内の `ovld slay` 参照を `ovld unsummon` に変更する
- [x] 4.3 `src/commands/status.rs` 内の `ovld slay` 参照を `ovld unsummon` に変更する

## 5. 検証

- [x] 5.1 `cargo build` でコンパイルエラーがないことを確認する
- [x] 5.2 `cargo test` で全テストが通ることを確認する
- [x] 5.3 `ovld --help` で unsummon が表示されることを確認する
