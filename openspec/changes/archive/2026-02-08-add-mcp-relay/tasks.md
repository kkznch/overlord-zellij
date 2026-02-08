## 1. メッセージストア & 型定義

- [x] 1.1 `src/relay/types.rs` に Message, Priority, Status, RoleStatus 構造体と ALL_ROLES 定数を作成
- [x] 1.2 `src/relay/store.rs` に MessageStore を作成（init, send_message, check_inbox, set_pending, update_status, get_status, get_all_statuses, cleanup を実装）
- [x] 1.3 `src/relay/mod.rs` で store, types, server, notify モジュールをエクスポート
- [x] 1.4 ストアのユニットテストを追加: 送受信、複数メッセージ、pending フラグ、ステータス、クリーンアップ、空 inbox

## 2. MCP サーバー

- [x] 2.1 Cargo.toml に rmcp, tokio, schemars の依存を追加
- [x] 2.2 `src/relay/server.rs` に rmcp 0.14 の tool_router/tool_handler パターンで RelayService を作成（5つの MCP ツール: send_message, check_inbox, get_status, update_status, broadcast）
- [x] 2.3 OVLD_ROLE, OVLD_RELAY_DIR, OVLD_SESSION 環境変数を読む `serve()` 関数を実装
- [x] 2.4 `src/lib.rs` に `pub mod relay` を追加

## 3. 自動通知

- [x] 3.1 `src/relay/notify.rs` に pane_target マッピング（役割 → タブ + focus_steps）を作成
- [x] 3.2 notify_pane 関数を実装（タブ切り替え、ペインナビゲーション、write-chars 注入、Enter キー送信）
- [x] 3.3 send_message と broadcast ツールに pending フラグチェックを統合（既に pending ならば通知をスキップ）

## 4. CLI 統合

- [x] 4.1 `src/main.rs` に隠し `Relay` サブコマンドを追加（tokio ランタイム経由で `relay::serve()` を呼ぶ）
- [x] 4.2 `src/config.rs` に `relay_dir()` と `generate_mcp_configs()` ヘルパーを追加
- [x] 4.3 `src/layout.rs` を更新し mcp_dir パラメータを受け取り、各ペインの claude 引数に `--mcp-config` を追加
- [x] 4.4 `src/commands/summon.rs` を更新し、relay ストア初期化、MCP 設定生成、mcp_dir をレイアウトに渡す
- [x] 4.5 `src/commands/slay.rs` を更新し、relay ストアのクリーンアップを呼ぶ

## 5. レイアウト調整

- [x] 5.1 `focus=true` を battlefield タブから command タブに移動し、魔王をデフォルトビューに
- [x] 5.2 レイアウトテストを更新し、生成レイアウトに `--mcp-config` が含まれることを検証

## 6. 儀式更新

- [x] 6.1 `rituals/overlord.md` に MCP 通信プロトコルセクションを追加（send_message, check_inbox, update_status, get_status）
- [x] 6.2 `rituals/strategist.md` に MCP 通信プロトコルセクションを追加（四天王全員への送信、broadcast、ステータス管理）
- [x] 6.3 `rituals/inferno.md` に MCP 通信プロトコルセクションを追加
- [x] 6.4 `rituals/glacier.md` に MCP 通信プロトコルセクションを追加
- [x] 6.5 `rituals/shadow.md` に MCP 通信プロトコルセクションを追加
- [x] 6.6 `rituals/storm.md` に MCP 通信プロトコルセクションを追加

## 7. 検証

- [x] 7.1 `cargo build` を実行しエラーがないことを確認
- [x] 7.2 `cargo test` を実行し全テストが合格することを確認
- [x] 7.3 `cargo install --path .` を実行し `ovld relay` サブコマンドが存在する（隠し）ことを確認
