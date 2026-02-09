## Purpose
ovld CLIツールのサブコマンド（summon, unsummon, status, init, relay）を定義する。

## Requirements

### Requirement: summon コマンドでセッション開始
`summon` サブコマンドで新しい Zellij セッションを魔王軍レイアウトで開始しなければならない (SHALL)。出力メッセージは i18n モジュール経由で言語設定に従って表示する。

#### Scenario: 新規セッション作成
- **WHEN** ユーザーが `ovld summon` を実行した時
- **THEN** "overlord" という名前の Zellij セッションが army.kdl レイアウトで作成される
- **THEN** 出力メッセージは設定された言語で表示される

#### Scenario: セッションが既に存在
- **WHEN** ユーザーが `ovld summon` を実行し、セッション "overlord" が既に存在する時
- **THEN** 新規作成ではなく既存セッションにアタッチする

### Requirement: unsummon コマンドでセッション終了
`unsummon` サブコマンドで魔王軍セッションを終了（還送）し、プロセスをクリーンアップしなければならない (SHALL)。出力メッセージは i18n モジュール経由で言語設定に従って表示する。

#### Scenario: 確認付きセッション還送
- **WHEN** ユーザーが `ovld unsummon` を実行した時
- **THEN** 還送前に確認プロンプトが設定された言語で表示される

#### Scenario: 強制還送
- **WHEN** ユーザーが `ovld unsummon --force` を実行した時
- **THEN** 確認なしでセッションが還送される

#### Scenario: セッション未検出
- **WHEN** ユーザーが `ovld unsummon` を実行し、セッションが存在しない時
- **THEN** エラーメッセージが設定された言語で表示される

### Requirement: status コマンドで魔王軍の状態表示
`status` サブコマンドで現在の魔王軍セッションの状態を表示しなければならない (SHALL)。出力メッセージは i18n モジュール経由で言語設定に従って表示する。

#### Scenario: アクティブセッション
- **WHEN** ユーザーが `ovld status` を実行し、セッションが存在する時
- **THEN** セッション名、ステータス、階層リストが設定された言語で表示される

#### Scenario: セッションなし
- **WHEN** ユーザーが `ovld status` を実行し、セッションが存在しない時
- **THEN** ステータスと summon コマンドのヒントが設定された言語で表示される

### Requirement: init コマンドでグローバル設定を展開
`init` サブコマンドでグローバル設定ディレクトリにデフォルト儀式ファイルを展開しなければならない (SHALL)。出力メッセージは i18n モジュール経由で言語設定に従って表示する。

#### Scenario: 初回展開
- **WHEN** ユーザーが `ovld init` を実行し、`~/.config/ovld/rituals/` が存在しない時
- **THEN** デフォルト儀式ファイルと `config.toml` がグローバル設定ディレクトリに展開される

#### Scenario: 既存設定がある場合
- **WHEN** ユーザーが `ovld init` を実行し、`~/.config/ovld/rituals/` が既に存在する時
- **THEN** 既に展開済みであるメッセージが設定された言語で表示され、上書きはしない

#### Scenario: 強制再展開
- **WHEN** ユーザーが `ovld init --force` を実行した時
- **THEN** 既存の儀式ファイルと `config.toml` を上書きしてデフォルトに戻す

### Requirement: relay サブコマンド
エージェント間通信用の MCP サーバーを起動する隠しサブコマンド `relay` を提供しなければならない (SHALL)。

#### Scenario: relay サーバー起動
- **WHEN** `OVLD_ROLE`、`OVLD_RELAY_DIR`、`OVLD_SESSION` 環境変数付きで `ovld relay` を実行した時
- **THEN** stdio トランスポートで5つのツールが登録された MCP サーバーが起動する

#### Scenario: ヘルプに非表示
- **WHEN** ユーザーが `ovld --help` を実行した時
- **THEN** `relay` サブコマンドはヘルプ出力に表示されない
