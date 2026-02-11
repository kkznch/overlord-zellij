## ADDED Requirements

### Requirement: dashboard コマンドでリアルタイムステータス表示
`dashboard` サブコマンドでリアルタイム魔王軍ステータスダッシュボード（TUI）を起動しなければならない (SHALL)。

#### Scenario: ダッシュボード起動
- **WHEN** ユーザーが `ovld dashboard` を実行した時
- **THEN** ratatui ベースの TUI ダッシュボードがターミナル全画面で起動する

#### Scenario: ヘルプに表示
- **WHEN** ユーザーが `ovld --help` を実行した時
- **THEN** `dashboard` サブコマンドがヘルプ出力に表示される
