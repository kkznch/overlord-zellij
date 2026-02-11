# overlord-zellij

Zellij上で複数のClaudeインスタンスを「魔王軍」として組織化するCLIツール。

## コンセプト

**「ユーザーは神託を下すだけ。あとは魔王軍がすべてやる。」**

ユーザーは曖昧な要望を一言伝えるだけでいい。魔王がそれを技術仕様に翻訳し、軍師がタスクを分解し、四天王が専門分野に応じて実装・テスト・ドキュメント作成まで自律的に完遂する。

```
深淵の意志（ユーザー）
   │
   │ 「〇〇作って」（曖昧でOK）
   ↓
魔王 (Overlord) ─── 要件定義・技術仕様に変換
   ↓
軍師 (Strategist) ─ タスク分解・四天王へ配分
   ↓
┌──────┬──────┬──────┬──────┐
│ 氷結  │ 業火  │ 常闇  │ 疾風  │
│型定義 │実装   │テスト │UI/Docs│
└──────┴──────┴──────┴──────┘
        ↓
   完成品がユーザーに届く
```

ユーザーが自分でタスク分解したり、複数のClaudeに個別に指示を出す必要はない。

## 階層構造

### 司令部（Command Layer）
- **魔王 (Overlord)** - ユーザーの曖昧な要望を技術仕様・要件定義に変換
- **軍師 (Strategist)** - タスクを分解し、四天王へ配分・指揮

### 四天王（Execution Layer）
| 名前 | 専門領域 | 役割 |
|------|----------|------|
| 氷結の将 (Glacier) | Arch & Refactor | 型・構造を先行定義、リファクタリング |
| 業火の将 (Inferno) | Logic & Core | 純粋なビジネスロジック・アルゴリズム実装 |
| 常闇の将 (Shadow) | Audit & Security | テスト作成、バグ狩り、脆弱性診断 |
| 疾風の将 (Storm) | UI & Docs | UI実装、ドキュメント作成 |

## 連動の法（ワークフロー）

四天王は以下のパイプラインで連携する：

```
Glacier (型定義) → Inferno (ロジック実装) → Shadow (テスト)
                                         ↘ Storm (UI/ドキュメント)
```

1. **Glacier** が先に型・interface・構造を定義
2. **Inferno** がその型に沿ってロジックを実装
3. **Shadow** が Inferno のコードをテスト・デバッグ
4. **Storm** が並列で UI・ドキュメントを作成

これにより各Claudeの負荷を分散し、専門性を活かした効率的な開発が可能。

## アーキテクチャ

各Claudeインスタンスはファイルベースのメッセージストアを持つMCPリレーサーバーを通じて通信する：

```
┌─ Zellij Session ──────────────────────────────────┐
│                                                    │
│  [Overlord]   ←─ MCP ─→  ovld relay               │
│  [Strategist] ←─ MCP ─→  ovld relay               │
│  [Inferno]    ←─ MCP ─→  ovld relay               │
│  [Glacier]    ←─ MCP ─→  ovld relay               │
│  [Shadow]     ←─ MCP ─→  ovld relay               │
│  [Storm]      ←─ MCP ─→  ovld relay               │
│                    ↕                               │
│          ~/.config/ovld/relay/                     │
│          ├── inbox/{role}/     (メッセージ)          │
│          ├── status/{role}.json                    │
│          └── pending/{role}    (通知フラグ)          │
│                    ↕                               │
│      zellij pipe → WASMプラグイン → ペインSTDIN      │
│      (新着メッセージ時に自動通知)                      │
└────────────────────────────────────────────────────┘
```

各Claudeインスタンスが使える **MCPツール**：
| ツール | 説明 |
|--------|------|
| `send_message` | 他のロールの受信箱にメッセージを送信 |
| `check_inbox` | 未読メッセージを取得（任意で既読化） |
| `get_status` | ロールの現在のステータスを確認（全員分も可） |
| `update_status` | 自分のステータスを更新（idle / working / blocked / done） |
| `broadcast` | 他の全ロールにメッセージを一斉送信 |

## レイアウト構成

Zellijセッションは3つのタブで構成：

```
┌─────────────────────────────────────────────┐
│ Tab 1: command（デフォルトフォーカス）          │
│ ┌──────────┬────────────────────────────────┤
│ │ Overlord │        Strategist              │
│ │  (30%)   │          (70%)                 │
│ ├──────────┴────────────────────────────────┤
│ │ [通知プラグイン] (borderless, 1行)          │
│ └───────────────────────────────────────────┤
├─────────────────────────────────────────────┤
│ Tab 2: battlefield                          │
│ ┌───────────────────────────────────────────┤
│ │                 Inferno                   │
│ │             (フルサイズ)                    │
│ └───────────────────────────────────────────┤
├─────────────────────────────────────────────┤
│ Tab 3: support                              │
│ ┌─────────────┬─────────────┬───────────────┤
│ │   Glacier   │   Shadow    │    Storm      │
│ │    (33%)    │    (33%)    │    (34%)      │
│ └─────────────┴─────────────┴───────────────┘
```

- **command**: 司令部。要件定義とタスク管理
- **battlefield**: 主戦場。メインの実装作業
- **support**: 補助部隊。アーキテクチャ・テスト・ドキュメント

通知プラグインはフォーカスを切り替えずにペイン間通知をルーティングする最小限のWASMペイン。

## 必要環境

- [Rust](https://www.rust-lang.org/)（ビルド用）
- [Zellij](https://zellij.dev/) がインストールされていること
- [Claude Code](https://docs.anthropic.com/en/docs/claude-code)（`claude` CLI）が利用可能であること

## インストール

```bash
cargo install --path .
```

## 使い方

```bash
# 魔王軍を召喚
ovld summon

# デバッグログ付きで召喚
ovld summon --debug

# 軍勢の状況確認
ovld status

# 魔王軍を還送
ovld unsummon

# 確認なしで還送
ovld unsummon --force

# グローバル設定を（再）展開
ovld init
ovld init --force   # 既存設定を上書き
```

デバッグログは `~/.config/ovld/logs/` に出力される。

## 設定

### 言語設定
CLI出力メッセージは英語と日本語に対応。`~/.config/ovld/config.toml` で設定：

```toml
lang = "en"   # 英語（デフォルト）
# lang = "ja" # 日本語
```

`ovld init` でデフォルト設定ファイルを生成、`ovld init --force` でリセット。

### 儀式ファイルの配置
儀式ファイルはローカル優先で解決される：
1. `./rituals/` - プロジェクトローカル儀式（プロジェクトごとにカスタマイズ）
2. `~/.config/ovld/rituals/` - グローバル儀式（デフォルト）

### 儀式のカスタマイズ
デフォルト儀式をプロジェクトにコピーして編集：
```bash
cp -r ~/.config/ovld/rituals ./rituals
# ./rituals/*.md を必要に応じて編集
```

## 動作の仕組み

### 1. 儀式の解決（Ritual Resolution）
`ovld summon` を実行すると：
1. ローカルの `./rituals/` ディレクトリを優先的に確認
2. 無ければグローバルの `~/.config/ovld/rituals/` にフォールバック
3. 初回実行時はデフォルト儀式を自動作成

### 2. 動的レイアウト生成
1. 儀式ファイルとMCP設定への絶対パスを含むKDLレイアウトを動的生成
2. 各ペインで `claude --system-prompt-file <ritual> --mcp-config <mcp_config>` を起動
3. MCPリレーツールは `--allowedTools "mcp__ovld-relay__*"` で自動承認
4. ペイン間通知ルーティング用のWASM通知プラグインペインを含む
5. セッション終了後にテンポラリKDLファイルを自動クリーンアップ

### 3. セッション管理
1. 生成したレイアウトで新しいZellijセッションを作成
2. CLIはZellijセッションが終了するまでブロック
3. 終了時にEXITEDセッション・メタデータ・リレーデータを自動クリーンアップ

### 4. MCPリレー通信
各Claudeペインが `ovld relay` プロセスをMCPサーバーとして起動。リレーは共有ファイルベースストアを使用：

- **受信箱**: メッセージはJSONファイルとして `~/.config/ovld/relay/inbox/{role}/` に保存
- **ステータス**: 各ロールの状態が `~/.config/ovld/relay/status/{role}.json` に格納
- **保留通知**: `~/.config/ovld/relay/pending/` のフラグファイルで未読メッセージを持つロールを追跡

各リレーに渡される環境変数: `OVLD_ROLE`, `OVLD_RELAY_DIR`, `OVLD_SESSION`, `OVLD_PLUGIN_PATH`

### 5. 自動通知
`send_message` でメッセージが送信されると：
1. メッセージがターゲットロールの受信箱に保存される
2. ターゲットロールにpendingフラグが設定される（重複排除 — チェックサイクルごとに1回のみ）
3. `zellij pipe` がJSONペイロードをWASM通知プラグインに送信（バックグラウンドスレッドで実行）
4. プラグインがターゲットペインのSTDINに通知テキストを直接書き込む
5. 受信側のClaudeインスタンスが通知を確認し、`check_inbox` でメッセージを取得

### 6. 運用フロー
1. **command** タブで魔王に要件を伝える
2. 軍師がタスクを分解し、四天王に指示
3. **battlefield** タブで **Inferno** がメイン実装
4. **support** タブで **Glacier/Shadow/Storm** が支援
5. ロール間はMCPリレーツールで自律的に通信

## ディレクトリ構成

```
overlord-zellij/
├── src/
│   ├── main.rs           # CLIエントリーポイント
│   ├── lib.rs            # ライブラリエクスポート・定数
│   ├── config.rs         # 設定、儀式解決、MCP設定生成
│   ├── layout.rs         # 動的KDLレイアウト生成
│   ├── logging.rs        # デバッグログ (--debug)
│   ├── i18n.rs           # 多言語対応 (en/ja)
│   ├── commands/         # summon / unsummon / status / init
│   ├── zellij/           # Zellijセッション管理
│   ├── army/             # 役職定義・アイコン
│   └── relay/            # MCPリレーサーバー
│       ├── server.rs     # 5つのMCPツール (send_message, check_inbox 等)
│       ├── store.rs      # ファイルベースのメッセージ永続化
│       ├── notify.rs     # Zellij pipe通知
│       └── types.rs      # Message, RoleStatus, Priority 型定義
├── plugin/               # Zellij WASMプラグイン（ペイン通知）
├── rituals/              # 各役職のシステムプロンプト
│   ├── overlord.md
│   ├── strategist.md
│   ├── inferno.md
│   ├── glacier.md
│   ├── shadow.md
│   └── storm.md
└── openspec/             # 仕様ドキュメント
```

## 仕様

詳細な仕様は `openspec/specs/` を参照：
- `ovld-cli/` - CLIコマンド仕様
- `army-hierarchy/` - 階層構造・役職仕様
- `zellij-session/` - セッション管理・レイアウト仕様
- `config-management/` - グローバル設定・儀式解決仕様
- `i18n/` - 多言語対応（英語/日本語）仕様
- `ritual-injection/` - プロンプト注入仕様
- `workflow-protocol/` - 四天王の連動プロトコル
- `mcp-relay/` - MCPリレーサーバー仕様
- `auto-notification/` - ペイン間自動通知仕様
