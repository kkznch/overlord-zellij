## Context

現在 `ovld slay` コマンドでセッション終了を行っているが、summon（召喚）の対義語として不自然。`unsummon`（召喚解除 / 還送）にリネームする。純粋なリネーム変更で、ロジックの変更はない。

## Goals / Non-Goals

**Goals:**
- `ovld slay` → `ovld unsummon` にコマンド名を変更
- ファイル名・モジュール名・メッセージテキストをすべて統一
- 「撃滅」の表現を「還送」に変更

**Non-Goals:**
- unsummon の機能追加や動作変更（純粋なリネームのみ）
- 後方互換エイリアス（slay を残さない）

## Decisions

### 1. ファイルリネーム方式
`src/commands/slay.rs` を `src/commands/unsummon.rs` に `git mv` でリネーム。git の履歴追跡を保つため。

### 2. 日本語表現の統一
「撃滅」→「還送」に統一。召喚(summon)の対として「還送」(元の場所に送り返す) が自然。

### 3. CLI ヘルプテキスト
`unsummon - 魔王軍を還送（セッション終了）` とする。`--force` フラグの説明は「確認なしで強制還送」に変更。

## Risks / Trade-offs

- **[破壊的変更]** → slay を使っていたユーザーは unsummon に切り替える必要あり。ただし個人ツールのため影響は最小。
