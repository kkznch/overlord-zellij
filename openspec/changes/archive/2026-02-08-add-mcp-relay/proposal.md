## なぜ

現在、Zellijペインで動作する6つのClaudeインスタンス（魔王、軍師、四天王）には互いに通信する手段がない。ユーザーがペイン間でメッセージを手動コピペする必要があり、マルチエージェント階層の目的を損なっている。ユーザーは魔王にだけ話しかければ、タスクが指揮系統を通じて自動的にカスケードする、完全自動・トークン効率の良いエージェント間通信システムが必要。

## 変更内容

- `~/.config/ovld/relay/` 配下にファイルベースのメッセージ relay システムを追加（役割ごとの inbox、status、pending ディレクトリ）
- 各 Claude インスタンスが `--mcp-config` で接続する MCP (Model Context Protocol) サーバーを追加。5つのツール公開: `send_message`、`check_inbox`、`get_status`、`update_status`、`broadcast`
- メッセージ到着時に Zellij の `write-chars` + `write` アクションでターゲットペインにトリガーテキストを注入する自動通知機能を追加
- 複数メッセージ同時到着時の重複通知を防ぐ pending フラグ機構を追加
- `ovld summon` 時に各役割用の MCP 設定 JSON ファイルを生成し、`ovld slay` 時にクリーンアップ
- Zellij レイアウトの各 Claude ペインに `--mcp-config` 引数を追加
- 全6つの儀式ファイルに MCP 通信プロトコルの指示を追加

## 機能 (Capabilities)

### 新規機能
- `mcp-relay`: ファイルベースのメッセージ relay システム。MCP サーバーインターフェース、送受信・ブロードキャスト・ステータス操作をサポート
- `auto-notification`: Zellij write-chars ベースの自動トリガーシステム。pending フラグによる重複排除付き

### 変更機能
- `zellij-session`: レイアウトに各ペインの `--mcp-config` を追加。summon で relay ディレクトリと MCP 設定を初期化。slay で relay ディレクトリをクリーンアップ
- `ritual-injection`: 全6儀式ファイルに MCP 通信プロトコルセクションを追加（旧テキストベース報告形式を置き換え）
- `ovld-cli`: MCP サーバーモード用の隠し `relay` サブコマンドを追加

## 影響範囲

- **新規ファイル**: `src/relay/` モジュール (mod.rs, types.rs, store.rs, server.rs, notify.rs)
- **変更ファイル**: `src/config.rs`、`src/layout.rs`、`src/commands/summon.rs`、`src/commands/slay.rs`、`src/main.rs`、`src/lib.rs`、全 `rituals/*.md`
- **新規依存**: `rmcp` (MCP SDK)、`tokio` (非同期ランタイム)、`schemars` (JSON Schema derive)
- **API**: 各 Claude インスタンスが stdio トランスポート経由で5つの MCP ツールを公開
