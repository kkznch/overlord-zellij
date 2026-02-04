## Context

現在の三兵団体制（5ペイン: 魔王、軍師、剛腕、処刑、記録）から、専門特化した四天王体制（6ペイン: 魔王、軍師、業火、氷結、常闇、疾風）へ移行する。業火への一極集中を防ぎ、連動の法に基づく負荷分散を実現する。

## Goals / Non-Goals

**Goals:**
- Role enumを6役職（Overlord, Strategist, Inferno, Glacier, Shadow, Storm）に変更
- KDLレイアウトを左翼/中央/右翼の3エリア構成に再設計
- 儀式プロンプトに「連動の法」（パイプライン指示）を組み込む
- 既存コマンド（summon/slay/status）の動作を維持しつつ役職を更新

**Non-Goals:**
- 四天王間の自動通信実装（手動コピペで対応）
- プロセス監視・自動再起動機能（将来課題）

## Decisions

### 1. Role enumの設計
6役職をフラットに定義。階層情報はdisplay_name()で表現。
```rust
enum Role { Overlord, Strategist, Inferno, Glacier, Shadow, Storm }
```

### 2. KDLレイアウト構成
- **Tab 1 (command)**: 魔王 + 軍師（縦分割）
- **Tab 2 (battlefield)**: 業火（中央主戦場、大きめ）
- **Tab 3 (support)**: 氷結 + 常闇 + 疾風（横3分割）

### 3. 儀式プロンプトの構造
各プロンプトに「己の専門領域」と「連携先への受け渡し指示」を含める。

### 4. 既存ファイル削除
`rituals/`配下の旧ファイル（overlord.md, strategist.md, legion_*.md）は削除し、新規作成。

## Risks / Trade-offs

- **[レイアウト互換性]** → 旧army.kdlは削除、新規作成で対応
- **[儀式ファイル互換性]** → 旧ファイル削除のため、ロールバック時は手動復元が必要
- **[6ペインの画面サイズ]** → 小さい画面では視認性低下の可能性

## Migration Plan

1. `src/army/roles.rs` を新Role enumに書き換え
2. `layouts/army.kdl` を新レイアウトに置き換え
3. `rituals/` 配下を全削除→新規6ファイル作成
4. `src/army/ritual.rs` のタブ/ペイン名を更新
5. ビルド・動作確認
