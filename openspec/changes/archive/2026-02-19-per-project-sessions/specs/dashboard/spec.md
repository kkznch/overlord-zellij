## MODIFIED Requirements

### Requirement: 全ロールのステータスをテーブル表示
ダッシュボードは6つの全ロールのアイコン・名前・ステータス・タスク・最終更新からの経過時間をテーブルで表示しなければならない (SHALL)。relay ディレクトリは `OVLD_RELAY_DIR` 環境変数から取得しなければならない (SHALL)。環境変数が未設定の場合はグローバルの relay ディレクトリにフォールバックする。

#### Scenario: ステータステーブルの描画
- **WHEN** ダッシュボードが起動しリレーストアにステータスデータがある時
- **THEN** 6行のテーブルが表示される（overlord, strategist, inferno, glacier, shadow, storm）
- **THEN** 各行に絵文字アイコン・ロール名・ステータスシンボル+テキスト・タスク説明・経過時間が表示される

#### Scenario: 環境変数から relay ディレクトリを取得
- **WHEN** `OVLD_RELAY_DIR` 環境変数が設定されている時
- **THEN** その値を relay ディレクトリとして使用する

#### Scenario: 環境変数未設定時のフォールバック
- **WHEN** `OVLD_RELAY_DIR` 環境変数が未設定の時
- **THEN** `~/.config/ovld/relay/` をフォールバックとして使用する

## ADDED Requirements

### Requirement: セッション名の環境変数解決
ダッシュボードはヘルスチェック通知の送信先セッション名を `OVLD_SESSION` 環境変数から取得しなければならない (SHALL)。環境変数が未設定の場合は `"overlord"` にフォールバックする。

#### Scenario: 環境変数からセッション名を取得
- **WHEN** `OVLD_SESSION` 環境変数が `ovld-myproject` に設定されている時
- **THEN** ヘルスチェック通知はセッション `ovld-myproject` のペインに送信される

#### Scenario: 環境変数未設定時のフォールバック
- **WHEN** `OVLD_SESSION` 環境変数が未設定の時
- **THEN** ヘルスチェック通知はセッション `"overlord"` のペインに送信される
