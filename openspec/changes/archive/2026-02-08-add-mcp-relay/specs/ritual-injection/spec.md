## Changed Requirements

### Requirement: プロンプトへのワークフロー指示の記載
各ロールのritualには、旧テキストベースのレポート形式に代わるMCP通信プロトコルの指示を含めること。

#### Scenario: Overlord ritualの内容
- **WHEN** Overlord ritualが読み込まれた場合
- **THEN** プロンプトにMCP通信プロトコルのセクションが含まれる
- **THEN** strategistへの連絡に`send_message`を使用する指示が含まれる
- **THEN** `[MESSAGE from ...]`が表示された際に`check_inbox`を使用する指示が含まれる

#### Scenario: Strategist ritualの内容
- **WHEN** Strategist ritualが読み込まれた場合
- **THEN** プロンプトにMCP通信プロトコルのセクションが含まれる
- **THEN** 4人の将それぞれへの`send_message`使用指示が含まれる
- **THEN** 全体連絡用の`broadcast`使用指示が含まれる

#### Scenario: 将のritualの内容
- **WHEN** いずれかの将のritual（inferno・glacier・shadow・storm）が読み込まれた場合
- **THEN** プロンプトにMCP通信プロトコルのセクションが含まれる
- **THEN** strategistへの報告に`send_message`を使用する指示が含まれる
- **THEN** ステータス追跡に`update_status`を使用する指示が含まれる

### Requirement: write-charsによるテキスト注入
システムはZellijのwrite-charsアクションを使用してプロンプトを注入すること。

#### Scenario: プロンプトテキストの書き込み
- **WHEN** ペインにフォーカスされた場合
- **THEN** システムが`zellij --session <name> action write-chars <text>`を実行する

#### Scenario: プロンプトの実行
- **WHEN** テキストが書き込まれた場合
- **THEN** システムが200ms待機してから`zellij action write-chars "\r"`でEnterキーを送信する
