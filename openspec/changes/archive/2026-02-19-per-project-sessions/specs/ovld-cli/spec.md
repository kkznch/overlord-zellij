## MODIFIED Requirements

### Requirement: summon コマンドでセッション開始
`summon` サブコマンドで新しい Zellij セッションを魔王軍レイアウトで開始しなければならない (SHALL)。セッション名はカレントディレクトリから動的に導出する。出力メッセージは i18n モジュール経由で言語設定に従って表示する。デフォルトでサンドボックスが有効であり、`--no-sandbox` フラグで無効化できなければならない (SHALL)。

#### Scenario: 新規セッション作成
- **WHEN** ユーザーが `ovld summon` を実行した時
- **THEN** cwd から導出されたセッション名で Zellij セッションが作成される
- **THEN** 出力メッセージは設定された言語で表示される

#### Scenario: セッションが既に存在
- **WHEN** ユーザーが `ovld summon` を実行し、同一 cwd のセッションが既に存在する時
- **THEN** 新規作成ではなく既存セッションに `zellij attach` する

#### Scenario: デフォルトでサンドボックス有効
- **WHEN** ユーザーが `ovld summon` を macOS で実行した時
- **THEN** Seatbelt サンドボックスプロファイルが生成される
- **THEN** "Sandbox enabled" メッセージが表示される
- **THEN** エージェントは `sandbox-exec` 経由で起動される

#### Scenario: サンドボックス無効化
- **WHEN** ユーザーが `ovld summon --no-sandbox` を実行した時
- **THEN** サンドボックスプロファイルは生成されない
- **THEN** エージェントは直接 `claude` コマンドで起動される

#### Scenario: 非 macOS でのサンドボックス
- **WHEN** ユーザーが macOS 以外のプラットフォームで `ovld summon` を実行した時
- **THEN** "Sandbox is only supported on macOS. Skipping." 警告が表示される
- **THEN** エージェントはサンドボックスなしで起動される

### Requirement: unsummon コマンドでセッション終了
`unsummon` サブコマンドで魔王軍セッションを終了（還送）し、プロセスをクリーンアップしなければならない (SHALL)。引数でセッション名を指定可能とする。`--all` フラグで全セッションを一括停止できなければならない (SHALL)。出力メッセージは i18n モジュール経由で言語設定に従って表示する。

#### Scenario: cwd のセッション還送
- **WHEN** ユーザーが `ovld unsummon` を引数なしで実行した時
- **THEN** カレントディレクトリに対応するセッションが還送される

#### Scenario: 名前指定でのセッション還送
- **WHEN** ユーザーが `ovld unsummon ovld-myproject` を実行した時
- **THEN** 指定された名前のセッションが還送される

#### Scenario: 全セッション一括還送
- **WHEN** ユーザーが `ovld unsummon --all` を実行した時
- **THEN** registry に登録された全セッションが順に還送される

#### Scenario: 強制還送
- **WHEN** ユーザーが `ovld unsummon --force` を実行した時
- **THEN** 確認なしでセッションが還送される

#### Scenario: セッション未検出
- **WHEN** ユーザーが `ovld unsummon` を実行し、cwd に対応するセッションが存在しない時
- **THEN** エラーメッセージが設定された言語で表示される

### Requirement: status コマンドで魔王軍の状態表示
`status` サブコマンドで現在の魔王軍セッションの状態を表示しなければならない (SHALL)。`--all` フラグで全セッションの一覧を表示できなければならない (SHALL)。出力メッセージは i18n モジュール経由で言語設定に従って表示する。

#### Scenario: アクティブセッション
- **WHEN** ユーザーが `ovld status` を実行し、cwd にセッションが存在する時
- **THEN** セッション名、ステータス、階層リストが設定された言語で表示される

#### Scenario: 全セッション一覧
- **WHEN** ユーザーが `ovld status --all` を実行した時
- **THEN** registry に登録された全セッションの名前・cwd・開始時刻が表示される

#### Scenario: セッションなし
- **WHEN** ユーザーが `ovld status` を実行し、cwd にセッションが存在しない時
- **THEN** ステータスと summon コマンドのヒントが設定された言語で表示される
