## Purpose
MCP ベースのエージェント間メッセージングシステム（ファイルストア、5つの MCP ツール、環境変数設定）を定義する。

## Requirements

### Requirement: メッセージ型と役割
relay システムは送信者、受信者、件名、本文、優先度、タイムスタンプのフィールドを持つメッセージ型を定義しなければならない (SHALL)。6つの有効な役割を認識すること: overlord, strategist, inferno, glacier, shadow, storm。

#### Scenario: 有効なメッセージの作成
- **WHEN** 全必須フィールド付きでメッセージを送信した時
- **THEN** 受信者の inbox ディレクトリに JSON ファイルとして保存される

#### Scenario: 無効な役割の拒否
- **WHEN** 認識されない役割名にメッセージを送信した時
- **THEN** 不明な役割であるエラーが返される

### Requirement: ファイルベースのメッセージストア
relay システムは `~/.config/ovld/relay/` 配下のディレクトリ構造で JSON ファイルとしてメッセージを保存しなければならない (SHALL)。サブディレクトリ: `inbox/{role}/`、`status/{role}.json`、`pending/{role}`。

#### Scenario: ストアの初期化
- **WHEN** relay ストアが初期化された時
- **THEN** 6つの役割すべてに inbox ディレクトリが作成される
- **THEN** 初期状態ファイル付きの status ディレクトリが作成される
- **THEN** pending ディレクトリが作成される

#### Scenario: ストアのクリーンアップ
- **WHEN** クリーンアップが呼ばれた時
- **THEN** inbox、status、pending の全ディレクトリが削除される

### Requirement: send_message MCP ツール
MCP サーバーは `send_message` ツールを公開し、受信者の inbox にメッセージを保存して通知をトリガーしなければならない (SHALL)。通知のペイン ID 解決に失敗した場合、パニックせずエラーを返さなければならない (SHALL)。

#### Scenario: メッセージ送信
- **WHEN** Claude が `send_message(to="strategist", subject="task done", body="...")` を呼んだ時
- **THEN** `inbox/strategist/` に JSON メッセージファイルが作成される
- **THEN** 軍師ペインへの自動通知がトリガーされる

#### Scenario: 不明なロールへの通知
- **WHEN** 通知先のロールが PANE_ORDER に存在しない時
- **THEN** パニックせずエラーが返される

### Requirement: check_inbox MCP ツール
MCP サーバーは `check_inbox` ツールを公開し、全未読メッセージを返して pending フラグをクリアしなければならない (SHALL)。

#### Scenario: メッセージありの inbox チェック
- **WHEN** Claude が `check_inbox()` を呼んだ時
- **THEN** その役割の inbox ディレクトリの全 JSON ファイルが返される
- **THEN** 読み取り後に inbox ファイルが削除される
- **THEN** その役割の pending フラグがクリアされる

#### Scenario: 空の inbox チェック
- **WHEN** メッセージがない状態で Claude が `check_inbox()` を呼んだ時
- **THEN** 空のリストが返される

### Requirement: get_status MCP ツール
MCP サーバーは `get_status` ツールを公開し、1つまたは全役割の現在のステータスを返さなければならない (SHALL)。

#### Scenario: 単一役割のステータス取得
- **WHEN** Claude が `get_status(role="inferno")` を呼んだ時
- **THEN** 業火の現在のステータス JSON が返される

#### Scenario: 全ステータスの取得
- **WHEN** Claude が `get_status(role="all")` を呼んだ時
- **THEN** 6つの役割すべてのステータスが返される

### Requirement: update_status MCP ツール
MCP サーバーは `update_status` ツールを公開し、呼び出し元の役割の現在のステータスとタスクを設定しなければならない (SHALL)。

#### Scenario: ステータス更新
- **WHEN** Claude が `update_status(status="working", task="認証の実装")` を呼んだ時
- **THEN** その役割のステータスファイルが新しいステータスとタスクで更新される

### Requirement: broadcast MCP ツール
MCP サーバーは `broadcast` ツールを公開し、同じメッセージを他の全役割に送信しなければならない (SHALL)。

#### Scenario: メッセージのブロードキャスト
- **WHEN** Claude が `broadcast(subject="sync", body="...")` を呼んだ時
- **THEN** 他の全役割の inbox (5つ) にメッセージが保存される
- **THEN** 各受信者への自動通知がトリガーされる

### Requirement: 環境変数による MCP サーバー設定
MCP サーバーは `OVLD_ROLE`、`OVLD_RELAY_DIR`、`OVLD_SESSION` 環境変数を読み取って自身を識別し、メッセージストアの場所を特定しなければならない (SHALL)。環境変数が未設定の場合、パニックせず `Result::Err` を返さなければならない (SHALL)。

#### Scenario: サーバー起動
- **WHEN** 環境変数が設定された状態で `ovld relay` が実行された時
- **THEN** stdio トランスポートで5つのツールを持つ MCP サーバーとして登録される

#### Scenario: OVLD_ROLE 未設定
- **WHEN** `OVLD_ROLE` 環境変数が未設定の状態で `ovld relay` が実行された時
- **THEN** プロセスがパニックせず、エラーメッセージを返して終了する

#### Scenario: HOME 未設定でのフォールバック
- **WHEN** `HOME` 環境変数が未設定の状態で知見ディレクトリを解決する時
- **THEN** プロセスがパニックせず、エラーメッセージを返して終了する
