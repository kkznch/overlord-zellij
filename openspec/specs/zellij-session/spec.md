## Purpose
Zellij セッションの作成・管理・レイアウト構造（3タブ6ペイン）および relay ディレクトリ統合を定義する。

## Requirements

### Requirement: レイアウトを使ったセッション作成
`--new-session-with-layout` フラグで Zellij セッションを作成し、各ペインの Claude コマンドに `--mcp-config` を含めなければならない (SHALL)。

#### Scenario: セッション開始
- **WHEN** summon コマンドが実行された時
- **THEN** `zellij --session <name> --new-session-with-layout <path>` が実行される

#### Scenario: レイアウトに MCP 設定を含む
- **WHEN** レイアウトが生成された時
- **THEN** 各ペインの `claude` コマンドに `--mcp-config <role>.json` が含まれる

### Requirement: セッション存在確認
新規作成前にセッションが既に存在するか確認しなければならない (SHALL)。

#### Scenario: 既存セッションの確認
- **WHEN** セッションの存在を確認する時
- **THEN** `zellij list-sessions` を実行し、セッション名を出力から検索する

### Requirement: セッション終了
セッションの kill と任意の削除で終了処理を行わなければならない (SHALL)。

#### Scenario: セッション kill
- **WHEN** unsummon コマンドが実行された時
- **THEN** `zellij kill-session <name>` が実行される

#### Scenario: セッションデータ削除
- **WHEN** unsummon コマンドが完了した時
- **THEN** `zellij delete-session <name> --force` でクリーンアップが実行される

### Requirement: セッションアタッチ
重複作成せず、既存セッションにアタッチしなければならない (SHALL)。

#### Scenario: セッションへのアタッチ
- **WHEN** セッションが存在し、summon が呼ばれた時
- **THEN** `zellij attach <name>` が実行される

### Requirement: 3タブレイアウト構造
レイアウトは3つのタブを定義しなければならない (SHALL): command、battlefield、support。command タブがデフォルトで `focus=true` であること。

#### Scenario: タブ構造
- **WHEN** レイアウトが読み込まれた時
- **THEN** command タブに魔王と軍師のペイン (垂直分割) があり、`focus=true` である
- **THEN** battlefield タブに業火の単一ペイン (大型作業領域) がある
- **THEN** support タブに氷結・常闇・疾風のペイン (水平3分割) がある

### Requirement: 6ペイン合計
レイアウトは6つの役割に対して6つのペインを提供しなければならない (SHALL)。

#### Scenario: ペイン数
- **WHEN** 魔王軍レイアウトでセッションが開始された時
- **THEN** 3つのタブにわたって正確に6つのペインが存在する

### Requirement: command タブのレイアウト
command タブは左に魔王、右に軍師を配置しなければならない (SHALL)。

#### Scenario: command タブ
- **WHEN** ユーザーが command タブを表示した時
- **THEN** 魔王ペインが左側 (小さい) にある
- **THEN** 軍師ペインが右側 (大きい) にある

### Requirement: battlefield タブのレイアウト
battlefield タブは業火の単一大型ペインを持たなければならない (SHALL)。

#### Scenario: battlefield タブ
- **WHEN** ユーザーが battlefield タブを表示した時
- **THEN** "inferno" という名前の単一ペインがタブ全体を占める

### Requirement: support タブのレイアウト
support タブは氷結・常闇・疾風の3つの均等ペインを持たなければならない (SHALL)。

#### Scenario: support タブ
- **WHEN** ユーザーが support タブを表示した時
- **THEN** 3つの水平ペインが存在する (33%/33%/34%)
- **THEN** 順番は氷結、常闇、疾風

### Requirement: summon 時の relay ディレクトリ初期化
summon コマンドはセッション開始前に relay ディレクトリ構造を初期化し、各役割用の MCP 設定 JSON を生成しなければならない (SHALL)。

#### Scenario: relay セットアップ
- **WHEN** summon コマンドが実行された時
- **THEN** `~/.config/ovld/relay/` に relay ディレクトリ構造が作成される
- **THEN** `~/.config/ovld/relay/mcp/{role}.json` に各役割用の MCP 設定 JSON が生成される

### Requirement: unsummon 時の relay クリーンアップ
unsummon コマンドはセッション終了時に relay ディレクトリをクリーンアップしなければならない (SHALL)。

#### Scenario: relay クリーンアップ
- **WHEN** unsummon コマンドが実行された時
- **THEN** relay メッセージストアのクリーンアップが呼ばれる

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
