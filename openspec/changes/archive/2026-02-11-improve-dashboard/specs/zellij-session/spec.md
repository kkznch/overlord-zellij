## MODIFIED Requirements

### Requirement: 3タブレイアウト構造
レイアウトは4つのタブを定義しなければならない (SHALL): command、battlefield、support、dashboard。command タブがデフォルトで `focus=true` であること。

#### Scenario: タブ構造
- **WHEN** レイアウトが読み込まれた時
- **THEN** command タブに魔王と軍師のペイン (垂直分割) があり、`focus=true` である
- **THEN** battlefield タブに業火の単一ペイン (大型作業領域) がある
- **THEN** support タブに氷結・常闇・疾風のペイン (水平3分割) がある
- **THEN** dashboard タブに `ovld dashboard` を実行する単一ペインがある

### Requirement: 6ペイン合計
レイアウトは6つの役割に対して6つのペイン、および dashboard 用に1つのペインを提供しなければならない (SHALL)。合計7ペイン。

#### Scenario: ペイン数
- **WHEN** 魔王軍レイアウトでセッションが開始された時
- **THEN** 4つのタブにわたって正確に7つのペインが存在する（6役割 + 1 dashboard）
