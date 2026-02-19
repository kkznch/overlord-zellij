## 1. Foundation: config.rs + lib.rs

- [x] 1.1 `lib.rs`: `SESSION_NAME` 定数を削除
- [x] 1.2 `config.rs`: `SessionRegistry` / `SessionEntry` 型を追加（`HashMap<String, SessionEntry>`）
- [x] 1.3 `config.rs`: `registry_path()`, `load_registry()`, `save_registry()` を追加
- [x] 1.4 `config.rs`: `register_session(name, cwd)`, `unregister_session(name)`, `find_session_by_cwd(cwd)` を追加
- [x] 1.5 `config.rs`: `derive_session_name(cwd) -> String` を追加（小文字化、サニタイズ、衝突時 suffix）
- [x] 1.6 `config.rs`: `session_dir(session_name) -> Result<PathBuf>` を追加
- [x] 1.7 `config.rs`: `relay_dir()` を `relay_dir(session_name: &str)` に変更
- [x] 1.8 `config.rs`: 旧 `session_metadata_path`, `save_session_metadata`, `load_session_metadata`, `delete_session_metadata` を削除
- [x] 1.9 registry 関連のユニットテストを追加

## 2. ZellijSession: exact match 修正 + attach

- [x] 2.1 `zellij/session.rs`: `exists()` を exact match に修正（`line.contains` → 先頭トークン比較）
- [x] 2.2 `zellij/session.rs`: `attach()` メソッドを追加（`zellij attach <name>`）
- [x] 2.3 exists の exact match テストを追加（zellij が必要なため integration test 扱い）

## 3. Layout: 環境変数注入

- [x] 3.1 `layout.kdl.j2`: dashboard ペインに `env { OVLD_SESSION "..." OVLD_RELAY_DIR "..." }` を追加
- [x] 3.2 `layout.rs`: `generate_layout()` に `session_name: &str`, `relay_dir: &Path` パラメータ追加
- [x] 3.3 `layout.rs`: テンプレートコンテキストに `session_name`, `relay_dir` を渡す
- [x] 3.4 `layout.rs`: `create_temp_layout()` も同様にパラメータ追加
- [x] 3.5 `layout.rs`: 既存テストを新しいシグネチャに更新

## 4. Dashboard: 環境変数解決

- [x] 4.1 `commands/dashboard.rs`: relay_dir を `OVLD_RELAY_DIR` 環境変数から取得（フォールバック: グローバル relay_dir）
- [x] 4.2 `commands/dashboard.rs`: session_name を `OVLD_SESSION` 環境変数から取得（フォールバック: `"overlord"`）

## 5. Commands: summon

- [x] 5.1 `commands/summon.rs`: `SESSION_NAME` → `derive_session_name(&cwd)` に置換
- [x] 5.2 `commands/summon.rs`: registry で cwd 検索 → 実在なら auto-attach、死んでたらパージ＆新規作成
- [x] 5.3 `commands/summon.rs`: `relay_dir(&session_name)` でセッション固有パスを使用
- [x] 5.4 `commands/summon.rs`: `register_session()` で登録（`save_session_metadata` 置換）
- [x] 5.5 `commands/summon.rs`: cleanup 時に `unregister_session()` 呼び出し
- [x] 5.6 `commands/summon.rs`: `create_temp_layout()` 呼び出しに新パラメータ追加

## 6. Commands: unsummon

- [x] 6.1 `main.rs`: `Commands::Unsummon` に `name: Option<String>` 引数と `--all` フラグ追加
- [x] 6.2 `commands/unsummon.rs`: セッション解決ロジック（引数 / cwd / --all）
- [x] 6.3 `commands/unsummon.rs`: cleanup 時に `unregister_session()` 呼び出し

## 7. Commands: status

- [x] 7.1 `main.rs`: `Commands::Status` に `--all` フラグ追加
- [x] 7.2 `commands/status.rs`: `find_session_by_cwd` でセッション解決
- [x] 7.3 `commands/status.rs`: `--all` で全セッション一覧表示

## 8. Commands: mod.rs cleanup

- [x] 8.1 `commands/mod.rs`: `cleanup_session_data(session_name: &str)` にパラメータ化

## 9. Relay Server

- [x] 9.1 `relay/server.rs`: `use crate::SESSION_NAME` を削除
- [x] 9.2 `relay/server.rs`: `OVLD_SESSION` のフォールバック削除 → `.context()` で必須化

## 10. Verification

- [x] 10.1 `cargo build` passes
- [x] 10.2 `cargo test` — all tests pass
- [x] 10.3 `cargo clippy` — no new warnings
