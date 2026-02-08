## Purpose
メッセージ到着時の Zellij ペインへの自動通知トリガーと pending フラグによる重複排除メカニズムを定義する。

## Requirements

### Requirement: 役割によるペインターゲティング
通知システムは各役割名を Zellij のタブとペイン位置にマッピングし、正確にターゲティングしなければならない (SHALL)。

#### Scenario: 魔王ペインをターゲット
- **WHEN** 通知が魔王をターゲットする時
- **THEN** "command" タブに切り替え、左ペインにフォーカスする

#### Scenario: 軍師ペインをターゲット
- **WHEN** 通知が軍師をターゲットする時
- **THEN** "command" タブに切り替え、右ペインにフォーカスする

#### Scenario: 業火ペインをターゲット
- **WHEN** 通知が業火をターゲットする時
- **THEN** "battlefield" タブに切り替える（単一ペインのためナビゲーション不要）

#### Scenario: support タブのペインをターゲット
- **WHEN** 通知が氷結、常闇、疾風をターゲットする時
- **THEN** "support" タブに切り替え、方向キーフォーカスで正しいペインに移動する

### Requirement: テキスト注入と Enter キー
通知システムはターゲットペインにトリガーメッセージを注入し、Enter キー送信で送信しなければならない (SHALL)。

#### Scenario: 通知の注入
- **WHEN** ある役割にメッセージが送信された時
- **THEN** `write-chars` で `[MESSAGE from {sender}] check_inbox ツールで受信メッセージを確認して作業を開始してください。` を書き込む
- **THEN** 200ms 待機する
- **THEN** テキストを送信するために Enter キーを送る

### Requirement: pending フラグによる重複排除
通知システムは pending フラグを使用して、受信者が処理する前に複数メッセージが到着した場合のトリガー注入の重複を防止しなければならない (SHALL)。

#### Scenario: 最初のメッセージで pending 設定
- **WHEN** pending フラグがない役割に最初のメッセージが到着した時
- **THEN** pending フラグが作成される
- **THEN** ペインに通知が注入される

#### Scenario: 後続メッセージは通知をスキップ
- **WHEN** 既に pending フラグがある役割に追加メッセージが到着した時
- **THEN** メッセージは inbox に保存される
- **THEN** 通知の注入はスキップされる（フラグが既に設定済みのため）

#### Scenario: check_inbox で pending をクリア
- **WHEN** 受信者が `check_inbox` を呼んだ時
- **THEN** pending フラグがクリアされる
- **THEN** 今後のメッセージは再び通知をトリガーする
