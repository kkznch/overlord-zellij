## Why

`SESSION_NAME = "overlord"` がハードコード定数で、relay ディレクトリも `~/.config/ovld/relay/` 固定。セッションは同時に1つしか存在できず、複数プロジェクトを並行して魔王軍に任せられない。プロジェクトごとに独立したセッションを持ち、同時に複数走らせたい。

## What Changes

- **BREAKING**: `SESSION_NAME` 定数を削除し、セッション名をプロジェクトディレクトリから動的に導出 (`ovld-{dirname}`)
- **BREAKING**: relay データを `~/.config/ovld/sessions/{session_name}/relay/` にセッション単位で分離
- `session.json` (単一ファイル) を `registry.json` (複数セッション管理) に置換
- `ovld summon`: 同一プロジェクトの既存セッションに auto-attach する機能を追加
- `ovld unsummon`: セッション名の引数指定 + `--all` フラグを追加
- `ovld status`: `--all` フラグで全セッション一覧を表示
- レイアウトの dashboard pane に `OVLD_SESSION` / `OVLD_RELAY_DIR` 環境変数を渡す
- `ZellijSession::exists()` の substring match を exact match に修正

## Capabilities

### New Capabilities
- `session-registry`: 複数セッションのライフサイクル管理（登録・検索・解除・名前導出）

### Modified Capabilities
- `zellij-session`: セッション名の動的導出、auto-attach、レイアウトへの環境変数注入
- `mcp-relay`: セッション固有 relay ディレクトリの使用、`OVLD_SESSION` 必須化
- `ovld-cli`: unsummon に引数 + `--all`、status に `--all`
- `dashboard`: 環境変数経由での relay ディレクトリ・セッション名解決

## Impact

- **Code**: lib.rs, config.rs, layout.rs, layout.kdl.j2, zellij/session.rs, commands/{summon,unsummon,status,mod,dashboard}.rs, relay/server.rs, main.rs
- **Data**: `~/.config/ovld/relay/` → `~/.config/ovld/sessions/*/relay/`、`session.json` → `registry.json`
- **Breaking**: 既存セッションが走っている場合、unsummon してから再 summon が必要
- **Dependencies**: 変更なし
- **Knowledge**: `~/.config/ovld/knowledge/` はグローバルのまま（影響なし）
