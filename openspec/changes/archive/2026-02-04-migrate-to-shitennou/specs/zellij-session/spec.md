## Changed Requirements

### Requirement: 3タブレイアウト構造
army.kdlレイアウトは、command・battlefield・supportの3タブを定義すること。

#### Scenario: タブ構造
- **WHEN** レイアウトが読み込まれた場合
- **THEN** commandタブにOverlordとStrategistのペイン（縦分割）が含まれる
- **THEN** battlefieldタブにInfernoの単一ペイン（広いワークスペース）が含まれる
- **THEN** supportタブにGlacier・Shadow・Stormのペイン（横3分割）が含まれる

### Requirement: 合計6ペインであること
レイアウトは6つのロールに対応する6つのペインを提供すること。

#### Scenario: ペイン数
- **WHEN** armyレイアウトでセッションが開始された場合
- **THEN** 3タブにまたがって正確に6つのペインが存在する

### Requirement: commandタブのレイアウト
commandタブは左側にOverlord、右側にStrategistを配置すること。

#### Scenario: commandタブ
- **WHEN** ユーザーがcommandタブを表示した場合
- **THEN** Overlordペインが左側（小さめ）にある
- **THEN** Strategistペインが右側（大きめ）にある

### Requirement: battlefieldタブのレイアウト
battlefieldタブはInferno用の単一の大きなペインを持つこと。

#### Scenario: battlefieldタブ
- **WHEN** ユーザーがbattlefieldタブを表示した場合
- **THEN** 「inferno」という名前の単一ペインがタブ全体を占める
- **THEN** このタブはデフォルトでfocus=trueである

### Requirement: supportタブのレイアウト
supportタブはGlacier・Shadow・Stormの3つの均等ペインを持つこと。

#### Scenario: supportタブ
- **WHEN** ユーザーがsupportタブを表示した場合
- **THEN** 3つの横並びペインが存在する（33%/33%/34%）
- **THEN** 左からGlacier・Shadow・Stormの順に配置される
