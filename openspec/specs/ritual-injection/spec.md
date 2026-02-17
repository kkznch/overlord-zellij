## Purpose
儀式プロンプトの読み込み・ペインターゲティング・write-chars テキスト注入・MCP通信プロトコル統合メカニズムを定義する。

## Requirements

### Requirement: 儀式ファイルの読み込み
rituals/ ディレクトリのマークダウンファイルから儀式内容を読み込まなければならない (SHALL)。

#### Scenario: 儀式の読み込み
- **WHEN** ある役割の儀式注入が開始された時
- **THEN** rituals/<role_ritual_file> から内容を読み込む

#### Scenario: 儀式ファイルが存在しない
- **WHEN** 儀式ファイルが存在しない時
- **THEN** ファイルパス付きのエラーを返す

### Requirement: 6つの儀式ファイル
rituals ディレクトリは6つのファイルを含まなければならない (SHALL): overlord.md, strategist.md, inferno.md, glacier.md, shadow.md, storm.md。

#### Scenario: 儀式ファイルの存在
- **WHEN** summon コマンドが実行された時
- **THEN** rituals/overlord.md を魔王に使用する
- **THEN** rituals/strategist.md を軍師に使用する
- **THEN** rituals/inferno.md を業火に使用する
- **THEN** rituals/glacier.md を氷結に使用する
- **THEN** rituals/shadow.md を常闇に使用する
- **THEN** rituals/storm.md を疾風に使用する

### Requirement: タブベースのナビゲーション
注入前に正しいタブに移動しなければならない (SHALL)。

#### Scenario: command タブへの注入
- **WHEN** 魔王または軍師に注入する時
- **THEN** "command" タブにフォーカスする

#### Scenario: battlefield タブへの注入
- **WHEN** 業火に注入する時
- **THEN** "battlefield" タブにフォーカスする

#### Scenario: support タブへの注入
- **WHEN** 氷結、常闇、疾風に注入する時
- **THEN** "support" タブにフォーカスする
- **THEN** 方向キーで正しいペインに移動する

### Requirement: write-chars によるテキスト注入
Zellij の write-chars アクションでプロンプトを注入しなければならない (SHALL)。

#### Scenario: プロンプトテキストの書き込み
- **WHEN** ペインにフォーカスした時
- **THEN** `zellij --session <name> action write-chars <text>` が実行される

#### Scenario: プロンプトの実行
- **WHEN** テキストが書き込まれた時
- **THEN** 200ms 待機後、`zellij action write-chars "\r"` で Enter キーを送信する

### Requirement: 注入のタイミング
ペインの準備を確保するため、操作間に遅延を含めなければならない (SHALL)。

#### Scenario: 注入前の遅延
- **WHEN** プロンプト注入直前
- **THEN** 書き込み前に 500ms 待機する

#### Scenario: 役割間の遅延
- **WHEN** 複数の役割に注入する時
- **THEN** 各役割間で 1 秒待機する

### Requirement: 儀式スキップオプション
--no-rituals フラグで儀式注入をスキップできなければならない (SHALL)。

#### Scenario: 儀式のスキップ
- **WHEN** ユーザーが `ovld summon --no-rituals` を実行した時
- **THEN** プロンプト注入なしでセッションが作成される

### Requirement: MCP通信プロトコルを儀式に含める
各役割の儀式には MCP 通信プロトコルの指示を含めなければならない (SHALL)（旧テキストベース報告形式を置き換え）。

#### Scenario: 魔王の儀式内容
- **WHEN** 魔王の儀式が読み込まれた時
- **THEN** MCP通信プロトコルセクションが含まれる
- **THEN** 軍師への `send_message` 使用指示が含まれる
- **THEN** `[MESSAGE from ...]` 表示時の `check_inbox` 使用指示が含まれる
- **THEN** 「確認してよろしいですか？」を禁句とする指示が含まれる
- **THEN** 裁量判断基準に従い宣言して動く指示が含まれる

#### Scenario: 軍師の儀式内容
- **WHEN** 軍師の儀式が読み込まれた時
- **THEN** MCP通信プロトコルセクションが含まれる
- **THEN** 4人の将への `send_message` 使用指示が含まれる
- **THEN** 全員への `broadcast` 使用指示が含まれる

#### Scenario: 四天王の儀式内容
- **WHEN** 四天王（業火、氷結、常闇、疾風）の儀式が読み込まれた時
- **THEN** MCP通信プロトコルセクションが含まれる
- **THEN** 軍師への報告用 `send_message` 使用指示が含まれる
- **THEN** 状態追跡用 `update_status` 使用指示が含まれる

### Requirement: 魔王の儀式に裁量判断基準を含める
魔王の儀式には三段階の裁量判断基準（即断即行・宣言即行・承認必須）を含めなければならない (SHALL)。

#### Scenario: 即断即行の記載
- **WHEN** 魔王の儀式が読み込まれた時
- **THEN** 確認不要で即実行する操作の一覧が含まれる（ファイル作成・コード修正・テスト追加・ドキュメント更新・依存追加）

#### Scenario: 宣言即行の記載
- **WHEN** 魔王の儀式が読み込まれた時
- **THEN** 宣言して即実行する操作の一覧が含まれる（API設計変更・DBスキーマ変更・大規模アーキテクチャ変更）

#### Scenario: 承認必須の記載
- **WHEN** 魔王の儀式が読み込まれた時
- **THEN** 承認を待つ操作の一覧が含まれる（本番デプロイ・認証変更・課金操作・不可逆データ削除）

### Requirement: 軍師の儀式に即応の法を含める
軍師の儀式には即応の法セクションを含めなければならない (SHALL)。

#### Scenario: 即応の法の記載
- **WHEN** 軍師の儀式が読み込まれた時
- **THEN** 魔王からの命令を即座にタスク分解・配分する指示が含まれる
- **THEN** 魔王に質問・確認を返さない指示が含まれる
- **THEN** 配分完了まで一切のアイドルを許さない指示が含まれる

### Requirement: 四天王の儀式に即動の法を含める
四天王（業火・氷結・常闇・疾風）の儀式には即動の法セクションを含めなければならない (SHALL)。

#### Scenario: 即動の法の記載
- **WHEN** 四天王の儀式が読み込まれた時
- **THEN** メッセージ受信後即座に作業開始する指示が含まれる
- **THEN** 不明点は自分の専門知識で判断する指示が含まれる
- **THEN** 完了後即座に次の工程に引き渡す指示が含まれる
