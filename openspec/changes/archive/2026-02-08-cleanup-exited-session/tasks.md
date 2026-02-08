## 1. summon.rs にクリーンアップ処理を追加

- [x] 1.1 `session.start()` 戻り後、`session.exists()` で EXITED セッションが残っているか確認するコードを追加
- [x] 1.2 EXITED セッション検出時に `session.kill()` + `session.delete(true)` を best-effort で実行
- [x] 1.3 EXITED セッション検出時に relay メッセージストアの `cleanup()` を best-effort で実行

## 2. 動作確認

- [x] 2.1 `cargo build` が成功することを確認
- [x] 2.2 `cargo test` が成功することを確認
