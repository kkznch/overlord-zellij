## Why

Zellij上で複数のClaudeインスタンスを「魔王軍」として組織化し、自律的な開発サイクルを構築する。抽象的な要望（神託）を階層構造で具体的なタスクに分解し、相互レビューで品質を高める仕組みを実現する。

## What Changes

- Rust製CLIツール `ovld` で魔王軍セッションを管理
- KDLレイアウトで魔王・軍師・三兵団のペイン配置を定義
- 儀式プロンプトで各役職にシステムプロンプトを自動注入

## Capabilities

### New Capabilities

- `ovld-cli`: ovldコマンドラインツール（summon/slay/statusサブコマンド）
- `army-hierarchy`: 魔王軍の階層構造と各役職の責務定義
- `zellij-session`: Zellijセッション管理とKDLレイアウト
- `ritual-injection`: 儀式プロンプトの自動注入メカニズム

### Modified Capabilities

（なし）

## Impact

- Zellijがインストールされた環境で動作
- 各ペインでClaude CLIが利用可能であることを前提
- 5つのClaudeインスタンス（魔王、軍師、三兵団）が同時稼働
