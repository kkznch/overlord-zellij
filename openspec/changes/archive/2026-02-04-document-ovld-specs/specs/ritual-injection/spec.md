## Added Requirements

### Requirement: 儀式ファイルの読み込み
システムは rituals/ ディレクトリ内のMarkdownファイルから儀式の内容を読み込まなければならない（SHALL）。

#### Scenario: 儀式の読み込み
- **WHEN** ある役職に対して儀式の注入が開始されたとき
- **THEN** システムが rituals/<role_ritual_file> から内容を読み込むこと

#### Scenario: 儀式ファイルが存在しない場合
- **WHEN** 儀式ファイルが存在しないとき
- **THEN** ファイルパス付きのエラーを返すこと

### Requirement: タブ経由でのペイン特定
システムは適切なタブにフォーカスしてから正しいペインに移動しなければならない（SHALL）。

#### Scenario: 魔王ペインへのターゲット
- **WHEN** 魔王(Overlord)に注入するとき
- **THEN** システムが `zellij action go-to-tab-name overlord` を実行すること

#### Scenario: 兵団ペインへのターゲット
- **WHEN** LegionDebugに注入するとき
- **THEN** legionsタブにフォーカスし、`focus-next-pane` を1回実行すること

### Requirement: write-charsによるテキスト注入
システムはZellijのwrite-charsアクションを使用してプロンプトを注入しなければならない（SHALL）。

#### Scenario: プロンプトテキストの書き込み
- **WHEN** ペインがフォーカスされているとき
- **THEN** システムが `zellij --session <name> action write-chars <text>` を実行すること

#### Scenario: プロンプトの実行
- **WHEN** テキストが書き込まれたとき
- **THEN** システムが `zellij action write 13` でEnterキーを送信すること

### Requirement: 注入タイミング
システムはペインの準備完了を待つために操作間に遅延を含めなければならない（SHALL）。

#### Scenario: 注入前の遅延
- **WHEN** プロンプトを注入しようとするとき
- **THEN** 書き込み前に500ms待機すること

#### Scenario: 役職間の遅延
- **WHEN** 複数の役職に注入するとき
- **THEN** 各役職間で1秒待機すること

### Requirement: 儀式スキップオプション
システムは --no-rituals フラグで儀式の注入をスキップできなければならない（SHALL）。

#### Scenario: 儀式のスキップ
- **WHEN** ユーザーが `ovld summon --no-rituals` を実行したとき
- **THEN** プロンプトを注入せずにセッションを作成すること
