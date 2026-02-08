## Purpose
グローバル設定ディレクトリ（`~/.config/ovld/`）の管理、デフォルト儀式ファイルの自動展開、ローカル→グローバルのフォールバック解決を定義する。

## Requirements

### Requirement: グローバル設定ディレクトリ
システムは `~/.config/ovld/` をグローバル設定ディレクトリとして使用しなければならない (SHALL)。

#### Scenario: 設定ディレクトリの場所
- **WHEN** システムがグローバル設定にアクセスする時
- **THEN** `$HOME/.config/ovld/` パスを使用する

### Requirement: デフォルト儀式ディレクトリ
グローバル設定はデフォルト儀式ファイルを含む `rituals/` サブディレクトリを持たなければならない (SHALL)。

#### Scenario: デフォルト儀式の場所
- **WHEN** システムがグローバル儀式を参照する時
- **THEN** `~/.config/ovld/rituals/` を確認する

### Requirement: 儀式ファイルの解決順序
システムはローカル優先で儀式ファイルを解決しなければならない (SHALL): `./rituals/` → `~/.config/ovld/rituals/`。

#### Scenario: ローカル儀式が存在する場合
- **WHEN** カレントディレクトリに `./rituals/` が存在する時
- **THEN** `./rituals/` の儀式ファイルを使用する

#### Scenario: ローカル儀式が存在しない場合
- **WHEN** `./rituals/` が存在しない時
- **THEN** `~/.config/ovld/rituals/` にフォールバックする

### Requirement: デフォルト設定の自動作成
システムは初回使用時にデフォルト儀式を含むグローバル設定ディレクトリを自動作成しなければならない (SHALL)。

#### Scenario: 設定なしの初回実行
- **WHEN** `ovld summon` 実行時に `~/.config/ovld/` が存在しない時
- **THEN** `~/.config/ovld/rituals/` を作成する
- **THEN** 埋め込みデフォルト儀式をそのディレクトリに展開する

### Requirement: 埋め込みデフォルト儀式
バイナリは `include_str!` を使用してデフォルト儀式ファイルを埋め込まなければならない (SHALL)。

#### Scenario: デフォルト儀式の内容
- **WHEN** デフォルト儀式の作成が必要な時
- **THEN** overlord.md、strategist.md、inferno.md、glacier.md、shadow.md、storm.md の埋め込みコンテンツを書き出す

### Requirement: 設定ディレクトリのバリデーション
システムは解決されたディレクトリに必要な儀式ファイルが存在するか検証しなければならない (SHALL)。

#### Scenario: 儀式ファイルの欠落
- **WHEN** 解決された儀式ディレクトリに必要なファイルが欠けている時
- **THEN** 欠落ファイルを列挙するエラーを返す
