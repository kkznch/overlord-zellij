## Changed Requirements

### Requirement: 6つの儀式ファイル
ritualsディレクトリは6つのファイルを含まなければならない（SHALL）: overlord.md、strategist.md、inferno.md、glacier.md、shadow.md、storm.md。

#### Scenario: 儀式ファイルの存在
- **WHEN** summonコマンドが実行されたとき
- **THEN** システムが魔王(Overlord)のために rituals/overlord.md を読み込むこと
- **THEN** システムが業火(Inferno)のために rituals/inferno.md を読み込むこと
- **THEN** システムが氷結(Glacier)のために rituals/glacier.md を読み込むこと
- **THEN** システムが常闇(Shadow)のために rituals/shadow.md を読み込むこと
- **THEN** システムが疾風(Storm)のために rituals/storm.md を読み込むこと

### Requirement: タブベースの注入ナビゲーション
システムは四天王への注入前に正しいタブに移動しなければならない（SHALL）。

#### Scenario: commandタブへの注入
- **WHEN** 魔王(Overlord)または軍師(Strategist)に注入するとき
- **THEN** システムが "command" タブにフォーカスすること

#### Scenario: battlefieldタブへの注入
- **WHEN** 業火(Inferno)に注入するとき
- **THEN** システムが "battlefield" タブにフォーカスすること

#### Scenario: supportタブへの注入
- **WHEN** 氷結(Glacier)、常闇(Shadow)、または疾風(Storm)に注入するとき
- **THEN** システムが "support" タブにフォーカスすること
- **THEN** focus-next-paneを使用して正しい将のペインに到達すること

### Requirement: プロンプトにワークフロー指示を含める
各将の儀式はワークフローパイプラインにおける自身の役割を含まなければならない（SHALL）。

#### Scenario: 氷結のプロンプト内容
- **WHEN** 氷結の儀式が読み込まれたとき
- **THEN** プロンプトに「型と構造を先に定義する」旨が含まれること
- **THEN** プロンプトに「定義を業火に渡す」旨が含まれること

#### Scenario: 業火のプロンプト内容
- **WHEN** 業火の儀式が読み込まれたとき
- **THEN** プロンプトに「氷結から構造を受け取る」旨が含まれること
- **THEN** プロンプトに「純粋なロジックのみに集中する」旨が含まれること

#### Scenario: 常闇のプロンプト内容
- **WHEN** 常闇の儀式が読み込まれたとき
- **THEN** プロンプトに「業火から実装を受け取る」旨が含まれること
- **THEN** プロンプトに「テストを生成しバグを報告する」旨が含まれること

#### Scenario: 疾風のプロンプト内容
- **WHEN** 疾風の儀式が読み込まれたとき
- **THEN** プロンプトに「業火からロジックを受け取る」旨が含まれること
- **THEN** プロンプトに「UIとドキュメントを作成する」旨が含まれること

## Removed Requirements

### Requirement: 兵団の儀式ファイル
**理由**: 四天王の儀式ファイルに置き換えられた
**移行**: legion_impl.md、legion_debug.md、legion_docs.md を削除し、inferno.md、glacier.md、shadow.md、storm.md を作成する
