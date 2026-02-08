## ADDED Requirements

### Requirement: 設定ファイル (config.toml)
システムは `~/.config/ovld/config.toml` で CLI 設定を管理しなければならない (SHALL)。

#### Scenario: 設定ファイルの読み込み
- **WHEN** CLI コマンドが実行された時
- **THEN** `~/.config/ovld/config.toml` から設定を読み込む

#### Scenario: 設定ファイルが存在しない場合
- **WHEN** `config.toml` が存在しない時
- **THEN** デフォルト設定（`lang = "en"`）を使用する

### Requirement: init 時の config.toml 生成
`ovld init` はデフォルトの `config.toml` を生成しなければならない (SHALL)。

#### Scenario: init で config.toml 生成
- **WHEN** `ovld init` が実行された時
- **THEN** `~/.config/ovld/config.toml` にデフォルト設定が書き出される
