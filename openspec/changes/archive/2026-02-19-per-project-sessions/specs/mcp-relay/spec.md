## MODIFIED Requirements

### Requirement: 環境変数による MCP サーバー設定
MCP サーバーは `OVLD_ROLE`、`OVLD_RELAY_DIR`、`OVLD_SESSION` 環境変数を読み取って自身を識別し、メッセージストアの場所を特定しなければならない (SHALL)。環境変数が未設定の場合、パニックせず `Result::Err` を返さなければならない (SHALL)。`OVLD_SESSION` はフォールバック無しの必須環境変数としなければならない (SHALL)。

#### Scenario: サーバー起動
- **WHEN** 環境変数が設定された状態で `ovld relay` が実行された時
- **THEN** stdio トランスポートで MCP サーバーとして登録される

#### Scenario: OVLD_ROLE 未設定
- **WHEN** `OVLD_ROLE` 環境変数が未設定の状態で `ovld relay` が実行された時
- **THEN** プロセスがパニックせず、エラーメッセージを返して終了する

#### Scenario: OVLD_SESSION 未設定
- **WHEN** `OVLD_SESSION` 環境変数が未設定の状態で `ovld relay` が実行された時
- **THEN** プロセスがパニックせず、エラーメッセージを返して終了する

#### Scenario: HOME 未設定でのフォールバック
- **WHEN** `HOME` 環境変数が未設定の状態で知見ディレクトリを解決する時
- **THEN** プロセスがパニックせず、エラーメッセージを返して終了する
