## MODIFIED Requirements

### Requirement: セッション終了
セッションの kill と任意の削除で終了処理を行わなければならない (SHALL)。

#### Scenario: セッション kill
- **WHEN** unsummon コマンドが実行された時
- **THEN** `zellij kill-session <name>` が実行される

#### Scenario: セッションデータ削除
- **WHEN** unsummon コマンドが完了した時
- **THEN** `zellij delete-session <name> --force` でクリーンアップが実行される

### Requirement: unsummon 時の relay クリーンアップ
unsummon コマンドはセッション終了時に relay ディレクトリをクリーンアップしなければならない (SHALL)。

#### Scenario: relay クリーンアップ
- **WHEN** unsummon コマンドが実行された時
- **THEN** relay メッセージストアのクリーンアップが呼ばれる
