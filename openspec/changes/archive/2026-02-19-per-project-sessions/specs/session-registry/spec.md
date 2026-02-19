## ADDED Requirements

### Requirement: セッション名の動的導出
セッション名はカレントディレクトリ名から `ovld-{sanitized_dirname}` のパターンで導出しなければならない (SHALL)。dirname は小文字化し、`[a-z0-9_-]` 以外の文字を除去する。

#### Scenario: 通常のディレクトリ名
- **WHEN** cwd が `/Users/user/projects/my-project` の時
- **THEN** セッション名は `ovld-my-project` となる

#### Scenario: 大文字・記号を含むディレクトリ名
- **WHEN** cwd が `/Users/user/Projects/MyApp.v2` の時
- **THEN** セッション名は `ovld-myappv2` となる（大文字→小文字、`.` は除去）

#### Scenario: 名前衝突（同名ディレクトリ、異なる cwd）
- **WHEN** registry に `ovld-app` が別の cwd で既に登録されている時
- **THEN** セッション名は `ovld-app-2` となる（suffix で回避）

### Requirement: セッションレジストリ
`~/.config/ovld/registry.json` で複数セッションのライフサイクルを管理しなければならない (SHALL)。各エントリは cwd と開始時刻を保持する。

#### Scenario: セッション登録
- **WHEN** 新しいセッションが作成された時
- **THEN** registry.json にセッション名・cwd・開始時刻のエントリが追加される

#### Scenario: セッション解除
- **WHEN** セッションが終了した時
- **THEN** registry.json から該当エントリが削除される

#### Scenario: cwd によるセッション検索
- **WHEN** カレントディレクトリに対応するセッションを検索した時
- **THEN** cwd が一致するセッションエントリが返される
- **WHEN** 一致するエントリがない時
- **THEN** None が返される

### Requirement: セッション固有データディレクトリ
各セッションの relay データは `~/.config/ovld/sessions/{session_name}/relay/` に格納しなければならない (SHALL)。knowledge ディレクトリはグローバル (`~/.config/ovld/knowledge/`) のままとする。

#### Scenario: セッション固有の relay ディレクトリ
- **WHEN** セッション `ovld-myproject` の relay ディレクトリを取得する時
- **THEN** `~/.config/ovld/sessions/ovld-myproject/relay/` が返される

#### Scenario: セッションクリーンアップ
- **WHEN** セッションのクリーンアップが実行された時
- **THEN** `~/.config/ovld/sessions/{session_name}/` ディレクトリ全体が削除される
- **THEN** registry からエントリが削除される
- **THEN** knowledge ディレクトリには影響しない

### Requirement: 孤立セッション検知
registry に存在するが Zellij 側で死んでいるセッションを検知し、パージできなければならない (SHALL)。

#### Scenario: 孤立エントリの検知
- **WHEN** registry にエントリがあるが `zellij list-sessions` に該当セッションが存在しない時
- **THEN** そのエントリは孤立と判定される

#### Scenario: 孤立エントリのパージ
- **WHEN** 孤立セッションが検知され、同じ cwd で新規 summon が実行された時
- **THEN** registry エントリとセッションデータが削除され、新規セッションが作成される
