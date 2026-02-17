## Purpose
macOS Seatbelt (sandbox-exec) を使用したプロセスレベルのファイル書き込み制限を定義する。

## Requirements

### Requirement: Seatbelt プロファイル生成
デフォルトで全 `file-write*` 操作を拒否し、指定ディレクトリへの書き込みのみ許可する macOS Seatbelt (.sb) プロファイルを生成しなければならない (SHALL)。

#### Scenario: 全書き込み拒否ルールを含む
- **WHEN** `generate_profile(cwd, config_dir)` が呼ばれた時
- **THEN** 出力に `(deny file-write* (subpath "/"))` が含まれる

#### Scenario: プロジェクトディレクトリへの書き込みを許可
- **WHEN** `generate_profile(cwd, config_dir)` が `cwd = "/home/user/project"` で呼ばれた時
- **THEN** 出力に `(allow file-write* (subpath "/home/user/project"))` が含まれる

#### Scenario: config ディレクトリへの書き込みを許可
- **WHEN** `generate_profile(cwd, config_dir)` が `config_dir = "/home/user/.config/ovld"` で呼ばれた時
- **THEN** 出力に `(allow file-write* (subpath "/home/user/.config/ovld"))` が含まれる
- **NOTE** relay_dir と knowledge_dir の両方をカバーする

#### Scenario: Claude 設定への書き込みを許可
- **WHEN** `generate_profile(cwd, config_dir)` が呼ばれた時
- **THEN** 出力に `(allow file-write* (subpath "{HOME}/.claude"))` が含まれる
- **THEN** 出力に `~/.claude.json` への書き込み許可（regex ルール）が含まれる

#### Scenario: Claude CLI キャッシュへの書き込みを許可
- **WHEN** `generate_profile(cwd, config_dir)` が呼ばれた時
- **THEN** 出力に `~/Library/Caches/claude-cli-nodejs` への書き込み許可が含まれる

#### Scenario: npm ログへの書き込みを許可
- **WHEN** `generate_profile(cwd, config_dir)` が呼ばれた時
- **THEN** 出力に `~/.npm/_logs` への書き込み許可が含まれる

#### Scenario: 一時ディレクトリへの書き込みを許可
- **WHEN** `generate_profile(cwd, config_dir)` が呼ばれた時
- **THEN** 出力に `/tmp`、`/private/tmp`、`/var/folders`、`/private/var/folders` への許可ルールが含まれる

#### Scenario: デバイスファイルへの書き込みを許可
- **WHEN** `generate_profile(cwd, config_dir)` が呼ばれた時
- **THEN** 出力に `/dev` サブパス全体への許可ルールが含まれる

#### Scenario: allow-default ベースプロファイル
- **WHEN** `generate_profile(cwd, config_dir)` が呼ばれた時
- **THEN** 出力に `(allow default)` が含まれ、読み取りおよびファイル以外の操作は制限されない

### Requirement: git リポジトリルートの書き込み許可
`cwd` が git リポジトリ内にある場合、リポジトリルートへの書き込みを許可しなければならない (SHALL)。worktree の場合はメインリポジトリのルートを許可する。`gix-discover` クレートを使用してリポジトリを検出する。

#### Scenario: cwd がリポジトリルートと同じ場合
- **WHEN** `cwd` が git リポジトリのルートディレクトリである時
- **THEN** `resolve_git_repo_root` は `None` を返す（cwd ルールで既にカバー済み）

#### Scenario: cwd がリポジトリ内のサブディレクトリの場合
- **WHEN** `cwd` が git リポジトリ内のサブディレクトリである時
- **THEN** `resolve_git_repo_root` はリポジトリルートの `PathBuf` を返す
- **THEN** プロファイルにリポジトリルートへの書き込み許可ルールが追加される

#### Scenario: cwd が git worktree 内の場合
- **WHEN** `cwd` が git worktree のルートまたはサブディレクトリである時
- **THEN** `resolve_git_repo_root` はメインリポジトリのルートを返す
- **THEN** メインリポジトリの `.git/objects/`、`.git/refs/` 等の共有リソースへの書き込みが可能になる

#### Scenario: git リポジトリ外の場合
- **WHEN** `cwd` が git リポジトリの外にある時
- **THEN** `resolve_git_repo_root` は `None` を返す
- **THEN** プロファイルに追加のリポジトリルールは含まれない

### Requirement: 一時プロファイルファイルの作成
生成した Seatbelt プロファイルを `.sb` 拡張子の一時ファイルに書き込み、ドロップ時に自動削除しなければならない (SHALL)。

#### Scenario: 一時ファイル作成
- **WHEN** `create_temp_profile(cwd, config_dir)` が呼ばれた時
- **THEN** プロファイルを含む `.sb` 拡張子のファイルが作成される
- **THEN** ファイルに `(version 1)` と `(deny file-write*` が含まれる

#### Scenario: 一時ファイルのクリーンアップ
- **WHEN** 返された `NamedTempFile` ハンドルがドロップされた時
- **THEN** 一時 `.sb` ファイルがディスクから削除される
