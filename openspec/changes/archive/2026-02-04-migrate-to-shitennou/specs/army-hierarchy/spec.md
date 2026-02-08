## Changed Requirements

### Requirement: 6つの異なる役職
システムは6つの役職を定義しなければならない（SHALL）: 魔王(Overlord)、軍師(Strategist)、業火(Inferno)、氷結(Glacier)、常闇(Shadow)、疾風(Storm)。

#### Scenario: 役職の列挙
- **WHEN** システムが初期化されたとき
- **THEN** 6つすべての役職が日本語と英語の表示名で利用可能であること

### Requirement: 四天王
システムは三兵団に代わる4人の将を定義しなければならない（SHALL）: 業火(Inferno)、氷結(Glacier)、常闇(Shadow)、疾風(Storm)。

#### Scenario: 四天王の表示
- **WHEN** statusコマンドが役職を一覧表示したとき
- **THEN** 業火が「業火の将 (Inferno)」として炎アイコン付きで表示されること
- **THEN** 氷結が「氷結の将 (Glacier)」として氷アイコン付きで表示されること
- **THEN** 常闇が「常闘の将 (Shadow)」として影アイコン付きで表示されること
- **THEN** 疾風が「疾風の将 (Storm)」として風アイコン付きで表示されること

### Requirement: 4階層構造
システムは4つの階層を維持しなければならない（SHALL）: ユーザー（深淵の意志）→ 魔王(Overlord) → 軍師(Strategist) → 四天王。

#### Scenario: 階層の表示
- **WHEN** statusコマンドが階層を表示したとき
- **THEN** 魔王と軍師が司令層として表示されること
- **THEN** 四天王が実行層として表示されること

### Requirement: 役職の儀式ファイル
各役職は対応する儀式ファイルを持たなければならない（SHALL）。

#### Scenario: 儀式ファイルの対応関係
- **WHEN** 役職が業火(Inferno)の場合
- **THEN** 儀式ファイルは "inferno.md" であること
- **WHEN** 役職が氷結(Glacier)の場合
- **THEN** 儀式ファイルは "glacier.md" であること
- **WHEN** 役職が常闇(Shadow)の場合
- **THEN** 儀式ファイルは "shadow.md" であること
- **WHEN** 役職が疾風(Storm)の場合
- **THEN** 儀式ファイルは "storm.md" であること

## Removed Requirements

### Requirement: 三兵団
**理由**: 四天王体制に置き換えられた
**移行**: LegionImpl → 業火(Inferno)、LegionDebug → 常闇(Shadow)、LegionDocs → 疾風(Storm)、（新規）氷結(Glacier)
