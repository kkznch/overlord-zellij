## Context

`SESSION_NAME = "overlord"` がハードコード定数で、relay ディレクトリも `~/.config/ovld/relay/` 固定。セッションは同時に1つしか存在できない。複数プロジェクトを並行運用するため、セッション名・データ・コマンドすべてをプロジェクト単位で分離する必要がある。

影響範囲: lib.rs, config.rs, layout.rs, layout.kdl.j2, zellij/session.rs, commands/{summon,unsummon,status,mod,dashboard}.rs, relay/server.rs, main.rs

## Goals / Non-Goals

**Goals:**
- プロジェクトごとに独立したセッションを同時実行可能にする
- relay データをセッション単位で分離し、データ競合を防ぐ
- 既存コマンドを cwd ベースで自動的に正しいセッションを解決する
- 同一プロジェクトの2回目の summon で auto-attach する

**Non-Goals:**
- knowledge ディレクトリのセッション分離（グローバルのままで正しい）
- registry.json のファイルロック（同時書き込みは稀なエッジケース、v1 では対応しない）
- セッション名のユーザーカスタマイズ（自動導出のみ）

## Decisions

### 1. セッション名の導出パターン: `ovld-{sanitized_dirname}`

cwd の最後のディレクトリ名を小文字化し、`[a-z0-9_-]` 以外を除去して `ovld-` プレフィックスを付ける。

**Why:** 人間が読める名前で、`zellij list-sessions` で一目でわかる。プレフィックスにより他の Zellij セッションと衝突しない。

**Alternative considered:** cwd のハッシュ → 一意だが人間が読めない。

### 2. データレイアウト: `~/.config/ovld/sessions/{name}/relay/`

各セッションのデータを `sessions/` サブディレクトリで分離。relay (ephemeral) のみセッション固有、knowledge (permanent) はグローバル。

**Why:** クリーンアップがディレクトリ単位で完結する。既存の MessageStore はコンストラクタで base_dir を受け取るため、パスを変えるだけで動く。

### 3. registry.json による複数セッション管理

`HashMap<String, SessionEntry>` 構造の JSON ファイルで全アクティブセッションを追跡。SessionMetadata (単一ファイル) を置換。

**Why:** 全セッションの一覧取得、cwd→session_name の逆引き、`--all` 操作に必要。

### 4. dashboard への環境変数注入 (layout template)

KDL テンプレートの dashboard ペインに `env { OVLD_SESSION "..." OVLD_RELAY_DIR "..." }` を追加。

**Why:** dashboard は Zellij セッション内のサブプロセスとして起動されるため、どのセッションのデータを読むか知る手段が必要。`ZELLIJ_SESSION_NAME` に依存するより、明示的に渡す方が確実。

**Alternative considered:** `ZELLIJ_SESSION_NAME` を読んで registry から逆引き → Zellij 内部の環境変数名に依存するのは fragile。

### 5. ZellijSession::exists() の exact match 修正

`line.contains(&self.name)` → 行をスペースで split して先頭トークンと完全一致比較。

**Why:** `ovld-app` が `ovld-app-2` にマッチしてしまうバグを防ぐ。Zellij の `list-sessions` 出力は `<name> [Created ... ago]` 形式。

### 6. OVLD_SESSION のフォールバック削除 (relay server)

`env::var("OVLD_SESSION").unwrap_or_else(|_| SESSION_NAME.to_string())` → `env::var("OVLD_SESSION").context("...")?` に変更。

**Why:** MCP config が必ず OVLD_SESSION を設定するため、未設定はプログラミングエラー。フォールバックが原因でデバッグしにくいバグを防ぐ。

### 7. unsummon の引数 + --all

`ovld unsummon [name] [--all] [--force]` で3パターンをサポート: 引数なし → cwd 解決、引数あり → 名前指定、--all → 全セッション。

**Why:** 複数セッション環境では特定のセッションを指定して停止できる必要がある。

## Risks / Trade-offs

- [Risk] registry.json の同時書き込みで競合 → v1 では発生確率が低いため許容。将来的にファイルロック (`fs2` crate) で対応可能
- [Risk] cwd がリネーム/移動された場合、`find_session_by_cwd` が失敗 → `ovld unsummon <name>` で名前指定停止可能
- [Risk] `generate_layout()` のパラメータ増加でテスト更新が広範囲 → テストは機械的な引数追加なので低リスク
- [Trade-off] `SESSION_NAME` 定数の完全削除は breaking change → 既存ユーザーは unsummon → summon で移行
