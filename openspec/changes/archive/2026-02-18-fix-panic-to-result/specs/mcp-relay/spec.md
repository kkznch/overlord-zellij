## MODIFIED Requirements

### Requirement: 環境変数による MCP サーバー設定
MCP サーバーは `OVLD_ROLE`、`OVLD_RELAY_DIR`、`OVLD_SESSION` 環境変数を読み取って自身を識別し、メッセージストアの場所を特定しなければならない (SHALL)。環境変数が未設定の場合、パニックせず `Result::Err` を返さなければならない (SHALL)。

#### Scenario: サーバー起動
- **WHEN** 環境変数が設定された状態で `ovld relay` が実行された時
- **THEN** stdio トランスポートで5つのツールを持つ MCP サーバーとして登録される

#### Scenario: OVLD_ROLE 未設定
- **WHEN** `OVLD_ROLE` 環境変数が未設定の状態で `ovld relay` が実行された時
- **THEN** プロセスがパニックせず、エラーメッセージを返して終了する

#### Scenario: HOME 未設定でのフォールバック
- **WHEN** `HOME` 環境変数が未設定の状態で知見ディレクトリを解決する時
- **THEN** プロセスがパニックせず、エラーメッセージを返して終了する

## MODIFIED Requirements

### Requirement: send_message MCP ツール
MCP サーバーは `send_message` ツールを公開し、受信者の inbox にメッセージを保存して通知をトリガーしなければならない (SHALL)。通知のペイン ID 解決に失敗した場合、パニックせずエラーを返さなければならない (SHALL)。

#### Scenario: メッセージ送信
- **WHEN** Claude が `send_message(to="strategist", subject="task done", body="...")` を呼んだ時
- **THEN** `inbox/strategist/` に JSON メッセージファイルが作成される
- **THEN** 軍師ペインへの自動通知がトリガーされる

#### Scenario: 不明なロールへの通知
- **WHEN** 通知先のロールが PANE_ORDER に存在しない時
- **THEN** パニックせずエラーが返される
