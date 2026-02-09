## Why

現状の儀式プロンプトでは各ロールが受動的に振る舞い、ユーザーへの確認待ちで実行が止まりがち。ユーザーが命令を出したら即座に自律的に実行し、重大な操作のみ確認を取る「魔王判断型」モデルに移行する。

## What Changes

- 魔王の儀式に **三段階裁量判断基準**（即断即行 / 宣言即行 / 承認必須）を追加
- 軍師の儀式に **即応の法** を追加（魔王に質問返さず即タスク分解）
- 四天王の儀式に **即動の法** を共通追加（受信即作業、確認禁止）
- 魔王の「重要」セクションを強化（「確認してよろしいですか？」を禁句に）

## Capabilities

### New Capabilities
（なし）

### Modified Capabilities
- `ritual-injection`: 各儀式に自律実行ドクトリン（裁量判断基準・即応の法・即動の法）の記載を要求する
- `workflow-protocol`: 四天王の即動の法による即時作業開始ルールを追加する

## Impact

- コード変更なし（儀式マークダウンファイルのみ）
- `include_str!` で埋め込まれるため再ビルドで反映
- 対象ファイル: `rituals/overlord.md`, `rituals/strategist.md`, `rituals/glacier.md`, `rituals/inferno.md`, `rituals/shadow.md`, `rituals/storm.md`
