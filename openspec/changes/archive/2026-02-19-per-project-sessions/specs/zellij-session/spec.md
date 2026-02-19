## MODIFIED Requirements

### Requirement: レイアウトを使ったセッション作成
`--new-session-with-layout` フラグで Zellij セッションを作成し、各ペインの Claude コマンドに `--mcp-config` を含めなければならない (SHALL)。サンドボックスプロファイルが指定された場合、`claude` コマンドを `sandbox-exec -f <profile> claude` でラップしなければならない (SHALL)。レイアウト生成のテンプレート処理に失敗した場合、パニックせず `Result::Err` を返さなければならない (SHALL)。パスの UTF-8 変換に失敗した場合も同様に `Result::Err` を返さなければならない (SHALL)。セッション名はプロジェクトディレクトリから動的に導出しなければならない (SHALL)。

#### Scenario: セッション開始
- **WHEN** summon コマンドが実行された時
- **THEN** cwd から導出されたセッション名で `zellij --session <derived_name> --new-session-with-layout <path>` が実行される

#### Scenario: テンプレートレンダリング失敗
- **WHEN** KDL テンプレートのレンダリングに失敗した時
- **THEN** パニックせず、テンプレートエラーの詳細を含むエラーが返される

#### Scenario: パスの非UTF-8変換
- **WHEN** レイアウトパスが UTF-8 でない時
- **THEN** パニックせず、パスの問題を示すエラーが返される

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

## ADDED Requirements

### Requirement: dashboard ペインへの環境変数注入
レイアウトの dashboard ペインに `OVLD_SESSION` と `OVLD_RELAY_DIR` 環境変数を設定しなければならない (SHALL)。

#### Scenario: dashboard pane の環境変数
- **WHEN** レイアウトが生成された時
- **THEN** dashboard ペインに `OVLD_SESSION` 環境変数としてセッション名が設定される
- **THEN** dashboard ペインに `OVLD_RELAY_DIR` 環境変数としてセッション固有の relay ディレクトリパスが設定される

### Requirement: 既存セッションへの auto-attach
同じプロジェクトのセッションが既に存在する場合、新規作成ではなく `zellij attach` で接続しなければならない (SHALL)。

#### Scenario: 既存セッションに auto-attach
- **WHEN** summon が実行され、registry に同一 cwd のセッションがあり、Zellij 側でも実在する時
- **THEN** `zellij attach <session_name>` が実行される
- **THEN** セッションデータの再初期化やクリーンアップは行わない

#### Scenario: 孤立セッション時の再作成
- **WHEN** summon が実行され、registry にエントリがあるが Zellij 側で死んでいる時
- **THEN** registry エントリとセッションデータをパージし、新規セッションを作成する

### Requirement: セッション存在確認の exact match
`zellij list-sessions` の出力でセッション名を検索する際、部分一致ではなく完全一致で判定しなければならない (SHALL)。

#### Scenario: 類似名セッションの区別
- **WHEN** `ovld-app` と `ovld-app-2` の両方が存在する時
- **THEN** `ovld-app` の存在確認は `ovld-app-2` にマッチしない
