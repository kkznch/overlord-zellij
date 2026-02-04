# overlord-zellij

Zellij上で複数のClaudeインスタンスを「魔王軍」として組織化するCLIツール。

## 階層構造

- **魔王 (Overlord)** - ユーザーの要望を技術仕様に翻訳
- **闇の軍師 (Strategist)** - タスク分解・兵団への指示
- **第一兵団・剛腕** - 実装担当
- **第二兵団・処刑** - デバッグ・レビュー担当
- **第三兵団・記録** - ドキュメント担当

## インストール

```bash
cargo install --path .
```

## 使い方

```bash
# 魔王軍を召喚
ovld summon

# 軍勢の状況確認
ovld status

# 魔王軍を殲滅
ovld slay
```

### オプション

```bash
# カスタムセッション名
ovld summon --session myarmy

# カスタムレイアウト
ovld summon --layout minimal

# 儀式（プロンプト注入）をスキップ
ovld summon --no-rituals

# 確認なしで終了
ovld slay --force
```

## 必要環境

- [Zellij](https://zellij.dev/) がインストールされていること
- 各ペインで `claude` CLI が利用可能であること

## 仕様

詳細な仕様は `openspec/specs/` を参照:
- `ovld-cli/` - CLIコマンド仕様
- `army-hierarchy/` - 階層構造仕様
- `zellij-session/` - セッション管理仕様
- `ritual-injection/` - プロンプト注入仕様
