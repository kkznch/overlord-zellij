## Why

`ovld summon` でセッションを起動後、ユーザーが Zellij を終了（ctrl+g → ctrl+q）すると、セッションが `EXITED` 状態で残る。`zellij list-sessions` に `overlord [Created ...] (EXITED - attach to resurrect)` と表示され、次回の `ovld summon` でも既存セッションとして検出されてしまう。セッション終了時にクリーンアップが自動実行されるべき。

## What Changes

- `summon` コマンドの `session.start()` 戻り後に、セッションの kill + delete + relay クリーンアップを自動実行する
- 現在 `unsummon` にしかないクリーンアップ処理を、summon の正常終了パスにも組み込む

## Capabilities

### New Capabilities

（なし）

### Modified Capabilities

- `zellij-session`: セッション終了時の自動クリーンアップ要件を追加（start 戻り後に kill/delete を実行）

## Impact

- `src/commands/summon.rs` — start 戻り後にクリーンアップ処理を追加
- `src/zellij/session.rs` — 変更なし（既存の kill/delete メソッドを利用）
