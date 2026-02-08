## ADDED Requirements

### Requirement: init コマンドでグローバル設定を展開
`init` サブコマンドでグローバル設定ディレクトリにデフォルト儀式ファイルを展開しなければならない (SHALL)。

#### Scenario: 初回展開
- **WHEN** ユーザーが `ovld init` を実行し、`~/.config/ovld/rituals/` が存在しない時
- **THEN** デフォルト儀式ファイルがグローバル設定ディレクトリに展開される

#### Scenario: 既存設定がある場合
- **WHEN** ユーザーが `ovld init` を実行し、`~/.config/ovld/rituals/` が既に存在する時
- **THEN** 既に展開済みであるメッセージが表示され、上書きはしない

#### Scenario: 強制再展開
- **WHEN** ユーザーが `ovld init --force` を実行した時
- **THEN** 既存の儀式ファイルを上書きしてデフォルトに戻す
