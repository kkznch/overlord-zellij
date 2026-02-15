## MODIFIED Requirements

### Requirement: レイアウトを使ったセッション作成
`--new-session-with-layout` フラグで Zellij セッションを作成し、各ペインの Claude コマンドに `--mcp-config` を含めなければならない (SHALL)。サンドボックスプロファイルが指定された場合、`claude` コマンドを `sandbox-exec -f <profile> claude` でラップしなければならない (SHALL)。

#### Scenario: セッション開始
- **WHEN** summon コマンドが実行された時
- **THEN** `zellij --session <name> --new-session-with-layout <path>` が実行される

#### Scenario: レイアウトに MCP 設定を含む
- **WHEN** レイアウトが生成された時
- **THEN** 各ペインの `claude` コマンドに `--mcp-config <role>.json` が含まれる

#### Scenario: サンドボックスなしのレイアウト生成
- **WHEN** `sandbox_profile = None` でレイアウトが生成された時
- **THEN** 各ペインは `command "claude"` を使用する

#### Scenario: サンドボックスありのレイアウト生成
- **WHEN** `sandbox_profile = Some(path)` でレイアウトが生成された時
- **THEN** 各ペインは `command "sandbox-exec"` を使用する
- **THEN** args の先頭に `"-f" "<profile_path>" "claude"` が含まれる
- **THEN** `command "claude"` はレイアウトに含まれない
