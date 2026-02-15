## Purpose
macOS Seatbelt (sandbox-exec) を使用したプロセスレベルのファイル書き込み制限を定義する。

## Requirements

### Requirement: Seatbelt プロファイル生成
デフォルトで全 `file-write*` 操作を拒否し、指定ディレクトリへの書き込みのみ許可する macOS Seatbelt (.sb) プロファイルを生成しなければならない (SHALL)。

#### Scenario: 全書き込み拒否ルールを含む
- **WHEN** `generate_profile(cwd, relay_dir)` が呼ばれた時
- **THEN** 出力に `(deny file-write* (subpath "/"))` が含まれる

#### Scenario: プロジェクトディレクトリへの書き込みを許可
- **WHEN** `generate_profile(cwd, relay_dir)` が `cwd = "/home/user/project"` で呼ばれた時
- **THEN** 出力に `(allow file-write* (subpath "/home/user/project"))` が含まれる

#### Scenario: relay ディレクトリへの書き込みを許可
- **WHEN** `generate_profile(cwd, relay_dir)` が `relay_dir = "/home/user/.config/ovld/relay"` で呼ばれた時
- **THEN** 出力に `(allow file-write* (subpath "/home/user/.config/ovld/relay"))` が含まれる

#### Scenario: Claude 設定への書き込みを許可
- **WHEN** `generate_profile(cwd, relay_dir)` が呼ばれた時
- **THEN** 出力に `(allow file-write* (subpath "{HOME}/.claude"))` が含まれる
- **THEN** 出力に `~/.claude.json` への書き込み許可（regex ルール）が含まれる

#### Scenario: Claude CLI キャッシュへの書き込みを許可
- **WHEN** `generate_profile(cwd, relay_dir)` が呼ばれた時
- **THEN** 出力に `~/Library/Caches/claude-cli-nodejs` への書き込み許可が含まれる

#### Scenario: npm ログへの書き込みを許可
- **WHEN** `generate_profile(cwd, relay_dir)` が呼ばれた時
- **THEN** 出力に `~/.npm/_logs` への書き込み許可が含まれる

#### Scenario: 一時ディレクトリへの書き込みを許可
- **WHEN** `generate_profile(cwd, relay_dir)` が呼ばれた時
- **THEN** 出力に `/tmp`、`/private/tmp`、`/var/folders`、`/private/var/folders` への許可ルールが含まれる

#### Scenario: デバイスファイルへの書き込みを許可
- **WHEN** `generate_profile(cwd, relay_dir)` が呼ばれた時
- **THEN** 出力に `/dev` サブパス全体への許可ルールが含まれる

#### Scenario: allow-default ベースプロファイル
- **WHEN** `generate_profile(cwd, relay_dir)` が呼ばれた時
- **THEN** 出力に `(allow default)` が含まれ、読み取りおよびファイル以外の操作は制限されない

### Requirement: 一時プロファイルファイルの作成
生成した Seatbelt プロファイルを `.sb` 拡張子の一時ファイルに書き込み、ドロップ時に自動削除しなければならない (SHALL)。

#### Scenario: 一時ファイル作成
- **WHEN** `create_temp_profile(cwd, relay_dir)` が呼ばれた時
- **THEN** プロファイルを含む `.sb` 拡張子のファイルが作成される
- **THEN** ファイルに `(version 1)` と `(deny file-write*` が含まれる

#### Scenario: 一時ファイルのクリーンアップ
- **WHEN** 返された `NamedTempFile` ハンドルがドロップされた時
- **THEN** 一時 `.sb` ファイルがディスクから削除される
