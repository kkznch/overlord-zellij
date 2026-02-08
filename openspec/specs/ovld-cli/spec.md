## Purpose
ovld CLIツールのサブコマンド（summon, unsummon, status, relay）とオプション（--session, --layout）を定義する。

## Requirements

### Requirement: summon コマンドでセッション開始
`summon` サブコマンドで新しい Zellij セッションを魔王軍レイアウトで開始しなければならない (SHALL)。

#### Scenario: 新規セッション作成
- **WHEN** ユーザーが `ovld summon` を実行した時
- **THEN** "overlord" という名前の Zellij セッションが army.kdl レイアウトで作成される

#### Scenario: セッションが既に存在
- **WHEN** ユーザーが `ovld summon` を実行し、セッション "overlord" が既に存在する時
- **THEN** 新規作成ではなく既存セッションにアタッチする

### Requirement: unsummon コマンドでセッション終了
`unsummon` サブコマンドで魔王軍セッションを終了（還送）し、プロセスをクリーンアップしなければならない (SHALL)。

#### Scenario: 確認付きセッション還送
- **WHEN** ユーザーが `ovld unsummon` を実行した時
- **THEN** 還送前に確認プロンプトが表示される

#### Scenario: 強制還送
- **WHEN** ユーザーが `ovld unsummon --force` を実行した時
- **THEN** 確認なしでセッションが還送される

#### Scenario: セッション未検出
- **WHEN** ユーザーが `ovld unsummon` を実行し、セッションが存在しない時
- **THEN** "セッション 'overlord' が見つかりません" というエラーが表示される

### Requirement: status コマンドで魔王軍の状態表示
`status` サブコマンドで現在の魔王軍セッションの状態を表示しなければならない (SHALL)。

#### Scenario: アクティブセッション
- **WHEN** ユーザーが `ovld status` を実行し、セッションが存在する時
- **THEN** セッション名、ステータス "ACTIVE"、魔王軍階層リストが表示される

#### Scenario: セッションなし
- **WHEN** ユーザーが `ovld status` を実行し、セッションが存在しない時
- **THEN** ステータス "NOT FOUND" と summon コマンドのヒントが表示される

### Requirement: カスタムセッション名
`--session` フラグでカスタムセッション名を指定できなければならない (SHALL)。

#### Scenario: カスタムセッション名
- **WHEN** ユーザーが `ovld summon --session myarmy` を実行した時
- **THEN** "myarmy" という名前のセッションが作成される

### Requirement: カスタムレイアウト
`--layout` フラグでカスタムレイアウトを指定できなければならない (SHALL)。

#### Scenario: カスタムレイアウトファイル
- **WHEN** ユーザーが `ovld summon --layout minimal` を実行した時
- **THEN** layouts/minimal.kdl がセッションに使用される

### Requirement: relay サブコマンド
エージェント間通信用の MCP サーバーを起動する隠しサブコマンド `relay` を提供しなければならない (SHALL)。

#### Scenario: relay サーバー起動
- **WHEN** `OVLD_ROLE`、`OVLD_RELAY_DIR`、`OVLD_SESSION` 環境変数付きで `ovld relay` を実行した時
- **THEN** stdio トランスポートで5つのツールが登録された MCP サーバーが起動する

#### Scenario: ヘルプに非表示
- **WHEN** ユーザーが `ovld --help` を実行した時
- **THEN** `relay` サブコマンドはヘルプ出力に表示されない
