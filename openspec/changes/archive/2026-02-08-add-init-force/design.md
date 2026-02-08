## Context

`ensure_default_config()` はディレクトリ存在チェックで早期リターンするため、一度展開された rituals は二度と更新されない。ユーザーがバイナリを再ビルドしても `~/.config/ovld/rituals/` は古いまま。

## Goals / Non-Goals

**Goals:**
- `ovld init` で手動でグローバル設定を（再）展開可能にする
- `--force` で既存ファイルを上書きできるようにする

**Non-Goals:**
- ローカル `./rituals/` への展開（ローカルはユーザーが手動管理する前提）
- rituals のバージョン管理やdiff表示

## Decisions

### `init` サブコマンドとして追加

`summon --init-force` のようなフラグではなく独立サブコマンドにする。理由:
- セッション起動とは無関係な操作
- `summon` の責務を増やさない

### `--force` なしは既存動作と同じ

フラグなしの `ovld init` は `ensure_default_config()` と同じ動作（ディレクトリが無ければ作成）。`--force` 指定時のみ既存ファイルを上書きする。

### 実装は `extract_rituals_to` の再利用

既存の `extract_rituals_to()` は無条件でファイルを書き出すため、`--force` 時はディレクトリ存在チェックをスキップして直接呼べばいい。

## Risks / Trade-offs

- **ユーザーのカスタマイズが上書きされる**: `--force` 使用時にユーザーが手動編集した rituals が上書きされる → これは意図した動作であり、ヘルプ文とコンソール出力で明示する
