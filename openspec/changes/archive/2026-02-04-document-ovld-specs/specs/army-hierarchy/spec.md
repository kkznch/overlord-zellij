## Added Requirements

### Requirement: 5つの異なる役職
システムは魔王軍の階層構造として5つの役職を定義しなければならない（SHALL）: 魔王(Overlord)、軍師(Strategist)、LegionImpl、LegionDebug、LegionDocs。

#### Scenario: 役職の列挙
- **WHEN** システムが初期化されたとき
- **THEN** 5つすべての役職が日本語と英語の表示名で利用可能であること

### Requirement: 魔王の役職
魔王(Overlord)はユーザーの要望を技術仕様に変換し、成果物に対して最終判断を下さなければならない（SHALL）。

#### Scenario: 魔王の表示
- **WHEN** statusコマンドが役職を一覧表示したとき
- **THEN** 魔王が「魔王 (Overlord)」として王冠アイコン付きで表示されること

### Requirement: 軍師の役職
軍師(闇の軍師)は魔王の命令を具体的なタスクに分解し、兵団を統率しなければならない（SHALL）。

#### Scenario: 軍師の表示
- **WHEN** statusコマンドが役職を一覧表示したとき
- **THEN** 軍師が「闘の軍師 (Dark Strategist)」として剣アイコン付きで表示されること

### Requirement: 兵団の役職
3つの兵団はそれぞれ専門的な責務を持たなければならない（SHALL）: 実装（剛腕）、デバッグ（処刑）、ドキュメント（記録）。

#### Scenario: 兵団の表示
- **WHEN** statusコマンドが役職を一覧表示したとき
- **THEN** LegionImplが「第一兵団・剛腕」として筋肉アイコン付きで表示されること
- **THEN** LegionDebugが「第二兵団・処刑」として炎アイコン付きで表示されること
- **THEN** LegionDocsが「第三兵団・記録」として巻物アイコン付きで表示されること

### Requirement: 役職の儀式ファイル
各役職はシステムプロンプトを定義する対応する儀式ファイルを持たなければならない（SHALL）。

#### Scenario: 儀式ファイルの対応関係
- **WHEN** 役職が魔王(Overlord)の場合
- **THEN** 儀式ファイルは "overlord.md" であること
- **WHEN** 役職が軍師(Strategist)の場合
- **THEN** 儀式ファイルは "strategist.md" であること
- **WHEN** 役職がLegionImplの場合
- **THEN** 儀式ファイルは "legion_impl.md" であること
