## ADDED Requirements

### Requirement: セッション終了時の自動クリーンアップ
summon によって開始されたセッションが終了（Zellij プロセスが exit）した場合、EXITED 状態のセッションを自動的にクリーンアップしなければならない (SHALL)。

#### Scenario: Zellij 終了後のセッション削除
- **WHEN** `session.start()` が戻り、セッションが EXITED 状態で残っている時
- **THEN** `zellij kill-session` と `zellij delete-session --force` が実行される

#### Scenario: Zellij 終了後の relay クリーンアップ
- **WHEN** `session.start()` が戻り、セッションが EXITED 状態で残っている時
- **THEN** relay メッセージストアのクリーンアップが実行される

#### Scenario: detach 時はクリーンアップしない
- **WHEN** `session.start()` が戻り、セッションがまだ存在する（EXITED ではない）時
- **THEN** セッションの kill/delete は実行されない

#### Scenario: クリーンアップ失敗時
- **WHEN** クリーンアップ中に kill/delete が失敗した時
- **THEN** エラーは無視され、summon コマンドは正常終了する
