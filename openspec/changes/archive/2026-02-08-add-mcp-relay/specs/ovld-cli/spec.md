## Added Requirements

### Requirement: relayサブコマンド
システムは、エージェント間通信用のMCPサーバーを起動する非公開の`relay`サブコマンドを提供すること。

#### Scenario: リレーサーバーの起動
- **WHEN** `OVLD_ROLE`・`OVLD_RELAY_DIR`・`OVLD_SESSION`環境変数が設定された状態で`ovld relay`が実行された場合
- **THEN** システムがstdioトランスポート上で5つのツールを登録したMCPサーバーを起動する

#### Scenario: ヘルプに表示されないこと
- **WHEN** ユーザーが`ovld --help`を実行した場合
- **THEN** `relay`サブコマンドはヘルプ出力に表示されない
