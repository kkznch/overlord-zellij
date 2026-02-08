## Purpose
魔王軍の階層構造と6つの役割（魔王、軍師、四天王4人）の責務・表示名・儀式ファイル対応を定義する。

## Requirements

### Requirement: 6つの役割
システムは6つの役割を定義しなければならない (SHALL): 魔王、軍師、業火、氷結、常闇、疾風。

#### Scenario: 役割の列挙
- **WHEN** システム初期化時
- **THEN** 6つの役割すべてが日本語・英語の表示名で利用可能であること

### Requirement: 魔王の役割
魔王 (Overlord) はユーザーの要望を技術仕様に変換し、成果物の最終検収を行わなければならない (SHALL)。

#### Scenario: 魔王の表示
- **WHEN** status コマンドで役割を表示する時
- **THEN** 魔王は「魔王 (Overlord)」と王冠アイコンで表示される

### Requirement: 軍師の役割
軍師 (Strategist) は魔王の命令を具体的なタスクに分解し、四天王を指揮しなければならない (SHALL)。

#### Scenario: 軍師の表示
- **WHEN** status コマンドで役割を表示する時
- **THEN** 軍師は「闘の軍師 (Strategist)」と剣アイコンで表示される

### Requirement: 四天王
システムは4人の将を定義しなければならない (SHALL): 業火 (Inferno)、氷結 (Glacier)、常闇 (Shadow)、疾風 (Storm)。

#### Scenario: 四天王の表示
- **WHEN** status コマンドで役割を表示する時
- **THEN** 業火は「業火の将 (Inferno)」と炎アイコンで表示される
- **THEN** 氷結は「氷結の将 (Glacier)」と氷アイコンで表示される
- **THEN** 常闇は「常闇の将 (Shadow)」と影アイコンで表示される
- **THEN** 疾風は「疾風の将 (Storm)」と風アイコンで表示される

### Requirement: 4層階層
システムは4つの階層を維持しなければならない (SHALL): ユーザー (深淵の意志) → 魔王 → 軍師 → 四天王。

#### Scenario: 階層の表示
- **WHEN** status コマンドで階層を表示する時
- **THEN** 魔王と軍師が司令層として表示される
- **THEN** 四天王が実行層として表示される

### Requirement: 儀式ファイルの対応
各役割は対応する儀式ファイルを持たなければならない (SHALL)。

#### Scenario: 儀式ファイルのマッピング
- **WHEN** 魔王の場合 → 儀式ファイルは "overlord.md"
- **WHEN** 軍師の場合 → 儀式ファイルは "strategist.md"
- **WHEN** 業火の場合 → 儀式ファイルは "inferno.md"
- **WHEN** 氷結の場合 → 儀式ファイルは "glacier.md"
- **WHEN** 常闇の場合 → 儀式ファイルは "shadow.md"
- **WHEN** 疾風の場合 → 儀式ファイルは "storm.md"
