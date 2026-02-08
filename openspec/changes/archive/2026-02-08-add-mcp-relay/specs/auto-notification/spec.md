## Added Requirements

### Requirement: ロール名によるペインターゲティング
通知システムは、各ロール名をZellijのタブおよびペイン位置にマッピングし、正確なターゲティングを行うこと。

#### Scenario: overlordペインのターゲット
- **WHEN** 通知がoverlordを対象とする場合
- **THEN** システムが「command」タブに切り替え、左ペインにフォーカスする

#### Scenario: strategistペインのターゲット
- **WHEN** 通知がstrategistを対象とする場合
- **THEN** システムが「command」タブに切り替え、右ペインにフォーカスする

#### Scenario: infernoペインのターゲット
- **WHEN** 通知がinfernoを対象とする場合
- **THEN** システムが「battlefield」タブに切り替える（単一ペインのためナビゲーション不要）

#### Scenario: supportタブのペインターゲット
- **WHEN** 通知がglacier・shadow・stormを対象とする場合
- **THEN** システムが「support」タブに切り替え、方向フォーカスで正しいペインに移動する

### Requirement: Enterキーによるテキスト注入
通知システムは、対象ペインにトリガーメッセージを注入し、Enterキーを送信して送信すること。

#### Scenario: 通知の注入
- **WHEN** ロールにメッセージが送信された場合
- **THEN** システムが`write-chars`で`[MESSAGE from {sender}] check_inbox ツールで受信メッセージを確認して作業を開始してください。`を書き込む
- **THEN** システムが200ms待機する
- **THEN** システムがEnterキーを送信してテキストを送信する

### Requirement: pendingフラグによる重複排除
通知システムは、受信者が処理する前に複数のメッセージが到着した際の重複トリガー注入を防ぐため、pendingフラグを使用すること。

#### Scenario: 最初のメッセージでpendingを設定
- **WHEN** pendingフラグのないロールに最初のメッセージが到着した場合
- **THEN** pendingフラグが作成される
- **THEN** ペインに通知が注入される

#### Scenario: 後続メッセージでは通知をスキップ
- **WHEN** 既にpendingフラグがあるロールに追加メッセージが到着した場合
- **THEN** メッセージはinboxに保存される
- **THEN** 通知の注入はスキップされる（フラグが既に設定済み）

#### Scenario: check_inboxでpendingをクリア
- **WHEN** 受信者が`check_inbox`を呼び出した場合
- **THEN** pendingフラグがクリアされる
- **THEN** 以降のメッセージで再び通知がトリガーされる
