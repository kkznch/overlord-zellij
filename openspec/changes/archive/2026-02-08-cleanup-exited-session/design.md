## Context

`ovld summon` は `zellij --session overlord --new-session-with-layout <path>` を実行し、Zellij が終了するまでブロックする。ユーザーが ctrl+g → ctrl+q で Zellij を終了すると、`session.start()` が戻るが、その後セッションメタデータの削除しか行われない。Zellij セッション自体は EXITED 状態で残り、`zellij list-sessions` に表示され続ける。

現在のクリーンアップ処理は `unsummon` コマンドにのみ実装されている（kill → delete → relay cleanup）。

## Goals / Non-Goals

**Goals:**
- Zellij セッション終了後に EXITED セッションが残らないようにする
- `summon` の正常終了パスでクリーンアップを自動実行する

**Non-Goals:**
- `unsummon` コマンドのクリーンアップ処理を変更すること
- detach（ctrl+g → d）時のクリーンアップ（detach は再接続前提のため残すべき）

## Decisions

### `summon.rs` の start 戻り後にクリーンアップを追加

`session.start()` が戻った後（= Zellij プロセスが終了した後）、既存メタデータ削除に加えて kill → delete → relay cleanup を実行する。

`unsummon` と同じクリーンアップ処理だが、共通関数への抽出は行わない。理由:
- `unsummon` は確認プロンプト + エラーハンドリング付き
- `summon` 側は best-effort（失敗しても無視）
- 性質が異なるため無理に統合する必要はない

### クリーンアップは best-effort

Zellij 終了後のクリーンアップは `let _ =` で失敗を無視する。理由:
- セッション終了自体は成功しているため、クリーンアップ失敗でエラー表示する必要はない
- Zellij が既にセッションを完全に削除している場合、kill/delete が失敗するのは正常

## Risks / Trade-offs

- **detach vs exit の区別**: `session.start()` は detach でも exit でも戻る。detach 時にもクリーンアップが走ると、再接続できなくなる → `session.exists()` で EXITED 状態を確認してからクリーンアップを実行することで回避
