## Context

現在の `ovld summon` は以下のフローで動作する：
1. `spawn()` で Zellij をバックグラウンド起動
2. 2秒待機
3. `zellij --session X action write-chars` で各ペインに儀式を注入

この設計の問題：
- `spawn()` は TTY なしで実行されるため、Zellij セッションが正しく作成されない
- 外部からの `zellij action` コマンドは「There is no active session!」で失敗

## Goals / Non-Goals

**Goals:**
- `ovld summon` 1コマンドで Zellij 内に入り、全ペインで Claude が儀式付きで起動済みの状態にする
- どのディレクトリからでも `ovld summon` が動作する
- プロジェクト固有の rituals カスタマイズをサポート

**Non-Goals:**
- 複数セッションの同時管理（今回は単一セッションのみ）
- rituals のホットリロード（セッション再作成が必要）
- Zellij 以外のターミナルマルチプレクサ対応

## Decisions

### Decision 1: セッション作成方式

**選択**: `spawn()` + 外部注入 → `status()` による即時 attach

**理由**:
- Zellij は TTY を必要とし、`spawn()` では正しく動作しない
- `status()` を使えばユーザーは Zellij 内に直接入る
- 儀式注入を KDL 起動コマンドに移すことで、外部からの注入が不要に

**代替案**:
- `fork()` + `setsid()` でデーモン化 → 複雑で、結局 TTY 問題が残る
- `expect`/PTY ライブラリ使用 → 依存関係が増える

### Decision 2: 儀式注入方式

**選択**: KDL の `command` ディレクティブで `claude --system-prompt-file` を直接起動

**理由**:
- セッション起動と同時に Claude が儀式付きで起動
- 外部注入のタイミング問題を完全に解消
- シンプルで信頼性が高い

**代替案**:
- `zellij run` コマンド → セッション内からしか使えない
- bash 経由で claude 起動 → 不必要に複雑

### Decision 3: Rituals ファイルの解決

**選択**: `./rituals/` → `~/.config/ovld/rituals/` のフォールバック

**理由**:
- プロジェクト固有カスタマイズ（ローカル）と汎用設定（グローバル）の両立
- ユーザーは普段はグローバル設定で動作、必要時のみローカルに rituals を置く

**代替案**:
- グローバルのみ → プロジェクト固有カスタマイズ不可
- ローカルのみ → 毎回 rituals を用意する必要あり

### Decision 4: KDL レイアウトの動的生成

**選択**: 実行時に KDL を文字列として生成し、一時ファイルに書き出し

**理由**:
- rituals の絶対パスを埋め込む必要がある
- `layouts/army.kdl` をテンプレートとして参照しつつ、パスを動的に置換

**実装**:
```rust
fn generate_layout(rituals_dir: &Path) -> String {
    format!(r#"
layout {{
    tab name="command" {{
        pane split_direction="vertical" {{
            pane name="overlord" size="30%" {{
                command "claude"
                args ["--system-prompt-file", "{}"]
            }}
            ...
        }}
    }}
    ...
}}
"#, rituals_dir.join("overlord.md").display())
}
```

### Decision 5: デフォルト Rituals の埋め込み

**選択**: `include_str!` でバイナリに埋め込み、初回実行時に展開

**理由**:
- インストール後すぐに動作（追加ファイル配置不要）
- バージョン管理された rituals がバイナリに含まれる

**実装**:
```rust
const DEFAULT_OVERLORD_RITUAL: &str = include_str!("../../rituals/overlord.md");
// ... 他の rituals も同様
```

## Risks / Trade-offs

**[Risk] CLI が Zellij に attach してブロックする** → ユーザーが期待する動作。Zellij 終了で CLI も終了。

**[Risk] KDL 生成のバグで起動失敗** → 生成した KDL を検証するテストを追加。

**[Risk] `claude` CLI が PATH にない場合** → 起動時にエラーメッセージを表示。将来的に `which claude` チェックを追加可能。

**[Trade-off] rituals 変更時はセッション再作成が必要** → 許容範囲。頻繁に変更するものではない。
