## 1. 依存関係追加

- [x] 1.1 `Cargo.toml` に ratatui 0.29 と crossterm 0.28 を追加

## 2. リレーストア拡張

- [x] 2.1 `MessageStore::has_pending(role)` メソッド追加（ペンディングフラグの存在確認）
- [x] 2.2 `MessageStore::recent_messages(limit)` メソッド追加（全受信箱から最新N件を既読マークなしで取得）
- [x] 2.3 `has_pending` と `recent_messages` のユニットテスト追加

## 3. ダッシュボード TUI 実装

- [x] 3.1 `src/commands/dashboard.rs` 新規作成: ターミナル初期化・復元の `execute()` 関数
- [x] 3.2 ステータステーブル描画（6ロール × 5列: アイコン・ロール名・ステータス・タスク・経過時間）
- [x] 3.3 ステータスの色分け（working=緑, blocked=赤, done=シアン, idle=暗灰）
- [x] 3.4 ペンディングメッセージインジケータ（ロール名横の `*` マーク）
- [x] 3.5 ワーカー数サマリー行（Workers: N/6 + Pending ロール一覧）
- [x] 3.6 最新メッセージフィード（ローカルタイムスタンプ HH:MM:SS 付き、最大5件）
- [x] 3.7 2秒ポーリング + `q` キー終了ハンドリング
- [x] 3.8 絵文字アイコン表示（👑⚔🔥❄🌑⚡）

## 4. CLI サブコマンド登録

- [x] 4.1 `src/commands/mod.rs` に `dashboard` モジュール追加
- [x] 4.2 `src/main.rs` に `Dashboard` variant と `dashboard::execute()` ハンドラ追加

## 5. Zellij レイアウト統合

- [x] 5.1 `src/layout.rs` に 4 つ目の dashboard タブ追加（`ovld dashboard` コマンド実行）
- [x] 5.2 `std::env::current_exe()` で ovld バイナリパスを取得
- [x] 5.3 レイアウトテストに dashboard タブのアサーション追加

## 6. 検証

- [x] 6.1 `cargo build` 成功
- [x] 6.2 `cargo test` 全テスト（60件）合格
