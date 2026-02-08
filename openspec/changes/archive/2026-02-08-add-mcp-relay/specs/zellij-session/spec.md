## Changed Requirements

### Requirement: レイアウトを使用したセッション作成
システムは`--new-session-with-layout`フラグを使用してZellijセッションを作成し、各ペインのClaudeコマンド引数に`--mcp-config`を含めること。

#### Scenario: セッション開始
- **WHEN** summonコマンドが実行された場合
- **THEN** システムが`zellij --session <name> --new-session-with-layout <path>`を実行する

#### Scenario: レイアウトにMCP設定を含める
- **WHEN** レイアウトが生成された場合
- **THEN** 各ペインの`claude`コマンドに`--mcp-config <role>.json`が含まれる

### Requirement: battlefieldタブのレイアウト
battlefieldタブはInferno用の単一の大きなペインを持つこと。

#### Scenario: battlefieldタブ
- **WHEN** ユーザーがbattlefieldタブを表示した場合
- **THEN** 「inferno」という名前の単一ペインがタブ全体を占める

### Requirement: 3タブレイアウト構造
army.kdlレイアウトは、command・battlefield・supportの3タブを定義すること。commandタブはデフォルトでfocus=trueとすること。

#### Scenario: タブ構造
- **WHEN** レイアウトが読み込まれた場合
- **THEN** commandタブにOverlordとStrategistのペイン（縦分割）がfocus=trueで含まれる
- **THEN** battlefieldタブにInfernoの単一ペイン（広いワークスペース）が含まれる
- **THEN** supportタブにGlacier・Shadow・Stormのペイン（横3分割）が含まれる

## Added Requirements

### Requirement: summon時のリレーディレクトリ初期化
summonコマンドは、セッション開始前にリレーディレクトリ構造を初期化し、ロールごとのMCP設定JSONファイルを生成すること。

#### Scenario: リレーのセットアップ
- **WHEN** summonコマンドが実行された場合
- **THEN** `~/.config/ovld/relay/`にリレーディレクトリ構造が作成される
- **THEN** `~/.config/ovld/relay/mcp/{role}.json`にロールごとのMCP設定JSONファイルが生成される

### Requirement: slay時のリレークリーンアップ
slayコマンドは、セッション終了時にリレーディレクトリをクリーンアップすること。

#### Scenario: リレーのクリーンアップ
- **WHEN** slayコマンドが実行された場合
- **THEN** リレーメッセージストアのクリーンアップが呼び出される
