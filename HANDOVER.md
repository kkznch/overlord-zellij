# 引き継ぎノート (2026-02-10)

## 今回やったこと
- Claude起動引数に `--allowedTools "mcp__ovld-relay__*"` 追加 → MCPツール確認ダイアログ省略
- `zellij pipe` を `thread::spawn` でバックグラウンド実行 → MCP応答ブロッキング解消
- JSONペイロードを `serde_json::json!` で構築 → エスケープ問題修正
- 親プロセスの `ZELLIJ` 環境変数を継承 → セッション内IPC直接接続

## 残ってる問題

### 1. zellij pipe が初回ハングする（最優先）
- セッション開始後、最初の `zellij pipe` 呼び出しがハングする
- しばらく待つと動き始める（トリガー不明）
- 原因仮説: プラグイン権限承認ダイアログが `size=1 borderless=true` ペーンで見えない
- 調べたいこと:
  - プラグインのペーンサイズを一時的に大きくして権限ダイアログが見えるか確認
  - `zellij action dump-layout` でプラグインURLを確認し、`--plugin` のURLと一致するか比較
  - プラグインの `pipe()` が `false` を返してる → `true` + `unblock_cli_pipe_input()` に変更
  - Zellijの設定で権限自動承認できないか調査

### 2. zellij pipe が数分ブロックする
- 動いても `.output()` が4分以上ブロックすることがある
- `thread::spawn` で緩和済みだが根本解決ではない

### 3. ペーンIDがハードコード
- `pane_id()` でロール→ID変換がハードコード（overlord=0, strategist=1, ...）
- レイアウト変更すると壊れる

## 関連ファイル
- `src/layout.rs` — レイアウト生成（`--allowedTools`追加済み）
- `src/relay/notify.rs` — 通知送信（`thread::spawn`化済み）
- `plugin/src/main.rs` — WASMプラグイン（権限リクエスト、pipeハンドラ）
