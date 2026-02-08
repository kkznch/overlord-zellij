## MODIFIED Requirements

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
