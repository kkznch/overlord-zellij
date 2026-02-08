## Added Requirements

### Requirement: メッセージ型とロールの定義
リレーシステムは、sender・recipient・subject・body・priority・timestampフィールドを持つメッセージ型を定義すること。有効なロールとしてoverlord・strategist・inferno・glacier・shadow・stormの6つを認識すること。

#### Scenario: 有効なメッセージの作成
- **WHEN** ロールが必須フィールドをすべて含むメッセージを送信した場合
- **THEN** メッセージが受信者のinboxディレクトリにJSONファイルとして保存される

#### Scenario: 無効なロールの拒否
- **WHEN** ロールが認識されていないロール名に対してメッセージを送信した場合
- **THEN** システムがロール不明のエラーを返す

### Requirement: ファイルベースのメッセージストア
リレーシステムは、`~/.config/ovld/relay/`配下のディレクトリ構造にJSONファイルとしてメッセージを保存すること。サブディレクトリは`inbox/{role}/`・`status/{role}.json`・`pending/{role}`とすること。

#### Scenario: ストアの初期化
- **WHEN** リレーストアが初期化された場合
- **THEN** 全6ロールのinboxディレクトリが作成される
- **THEN** 初期ステータスファイルを含むstatusディレクトリが作成される
- **THEN** pendingディレクトリが作成される

#### Scenario: ストアのクリーンアップ
- **WHEN** クリーンアップが呼び出された場合
- **THEN** すべてのinbox・status・pendingディレクトリが削除される

### Requirement: send_message MCPツール
MCPサーバーは、受信者のinboxにメッセージを保存し通知をトリガーする`send_message`ツールを公開すること。

#### Scenario: メッセージ送信
- **WHEN** Claudeが`send_message(to="strategist", subject="task done", body="...")`を呼び出した場合
- **THEN** `inbox/strategist/`にJSONメッセージファイルが作成される
- **THEN** strategistペインへの自動通知がトリガーされる

### Requirement: check_inbox MCPツール
MCPサーバーは、すべての未読メッセージを返しpendingフラグをクリアする`check_inbox`ツールを公開すること。

#### Scenario: メッセージありのinbox確認
- **WHEN** Claudeが`check_inbox()`を呼び出した場合
- **THEN** ロールのinboxディレクトリ内のすべてのJSONファイルが返される
- **THEN** 読み取り後にinboxファイルが削除される
- **THEN** そのロールのpendingフラグがクリアされる

#### Scenario: 空のinbox確認
- **WHEN** Claudeがメッセージなしの状態で`check_inbox()`を呼び出した場合
- **THEN** 空のリストが返される

### Requirement: get_status MCPツール
MCPサーバーは、1つまたはすべてのロールの現在のステータスを返す`get_status`ツールを公開すること。

#### Scenario: 単一ロールのステータス取得
- **WHEN** Claudeが`get_status(role="inferno")`を呼び出した場合
- **THEN** infernoの現在のステータスJSONが返される

#### Scenario: 全ステータスの取得
- **WHEN** Claudeが`get_status(role="all")`を呼び出した場合
- **THEN** 全6ロールのステータスが返される

### Requirement: update_status MCPツール
MCPサーバーは、呼び出し元ロールの現在のステータスとタスクを設定する`update_status`ツールを公開すること。

#### Scenario: ステータス更新
- **WHEN** Claudeが`update_status(status="working", task="implementing auth")`を呼び出した場合
- **THEN** そのロールのステータスファイルが新しいステータスとタスクで更新される

### Requirement: broadcast MCPツール
MCPサーバーは、同じメッセージを他のすべてのロールに送信する`broadcast`ツールを公開すること。

#### Scenario: メッセージのブロードキャスト
- **WHEN** Claudeが`broadcast(subject="sync", body="...")`を呼び出した場合
- **THEN** 他のすべてのロールのinbox（5つ）にメッセージが保存される
- **THEN** 各受信者への自動通知がトリガーされる

### Requirement: 環境変数によるMCPサーバー設定
MCPサーバーは、自身の識別とメッセージストアの場所特定のために`OVLD_ROLE`・`OVLD_RELAY_DIR`・`OVLD_SESSION`環境変数を読み取ること。

#### Scenario: サーバー起動
- **WHEN** 環境変数が設定された状態で`ovld relay`が実行された場合
- **THEN** サーバーがstdioトランスポート上で5つのツールを登録したMCPサーバーとして起動する
