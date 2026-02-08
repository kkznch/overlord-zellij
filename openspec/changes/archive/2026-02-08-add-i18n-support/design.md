## Context

現在 CLI の出力メッセージ（約25箇所）は日本語文字列がハードコードされている。summon, unsummon, status, init の4コマンドに散在。外部 i18n ライブラリ（fluent, rust-i18n 等）は存在するが、このプロジェクトの規模（25メッセージ程度）には過剰。

## Goals / Non-Goals

**Goals:**
- CLI 出力メッセージを英語/日本語で切り替え可能にする
- `~/.config/ovld/config.toml` の `lang` フィールドで言語を管理
- デフォルト言語は英語

**Non-Goals:**
- ritual ファイル（`.md`）の多言語化（これはシステムプロンプトであり翻訳対象外）
- ランタイムでの動的言語切替（起動時に決定すれば十分）
- 3言語以上の対応（en/ja の2言語のみ）

## Decisions

### 自前の軽量 i18n モジュール

外部クレートを使わず、`src/i18n.rs` に enum ベースのメッセージ定義を置く。理由:
- メッセージ数が25程度で外部ライブラリは過剰
- 依存を増やさない
- コンパイル時にメッセージの網羅性を保証できる（match 式）

### メッセージ定義の構造

```rust
pub enum Msg {
    SummonStarting,
    SummonRitualFiles,
    SummonSessionEnded,
    // ...
}

impl Msg {
    pub fn text(&self, lang: Lang) -> String {
        match (self, lang) {
            (Msg::SummonStarting, Lang::En) => "Summoning the army at {:?}...",
            (Msg::SummonStarting, Lang::Ja) => "{:?} で魔王軍を召喚中...",
            // ...
        }
    }
}
```

各コマンドでは `Msg::SummonStarting.text(lang)` のように呼び出す。

### config.toml の形式

```toml
lang = "en"
```

シンプルな TOML ファイル。`serde` + `toml` クレートで読み書き。将来的に設定項目が増えた場合も拡張しやすい。

### 言語解決の優先順位

1. `~/.config/ovld/config.toml` の `lang` フィールド
2. 存在しない場合はデフォルト `en`

環境変数（`LANG`, `LC_ALL`）は使わない。明示的な設定のみを尊重する。

## Risks / Trade-offs

- **メッセージ追加時の手間**: 新メッセージを追加するたびに en/ja 両方を書く必要がある → match 式のため、片方を忘れるとコンパイルエラーになるので安全
- **format! 引数の不一致**: en/ja でフォーマット引数が異なるとランタイムエラーになる → メッセージは引数を受け取る関数として定義し、フォーマットは呼び出し側で行う方式にする
