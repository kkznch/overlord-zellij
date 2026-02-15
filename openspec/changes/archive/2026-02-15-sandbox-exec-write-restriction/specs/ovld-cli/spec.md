## MODIFIED Requirements

### Requirement: summon コマンドでセッション開始
`summon` サブコマンドで新しい Zellij セッションを魔王軍レイアウトで開始しなければならない (SHALL)。出力メッセージは i18n モジュール経由で言語設定に従って表示する。デフォルトでサンドボックスが有効であり、`--no-sandbox` フラグで無効化できなければならない (SHALL)。

#### Scenario: 新規セッション作成
- **WHEN** ユーザーが `ovld summon` を実行した時
- **THEN** "overlord" という名前の Zellij セッションが army.kdl レイアウトで作成される
- **THEN** 出力メッセージは設定された言語で表示される

#### Scenario: セッションが既に存在
- **WHEN** ユーザーが `ovld summon` を実行し、セッション "overlord" が既に存在する時
- **THEN** 新規作成ではなく既存セッションにアタッチする

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
