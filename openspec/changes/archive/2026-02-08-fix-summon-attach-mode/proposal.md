## Why

`ovld summon` が「There is no active session!」エラーで正しく動作しない。現在の実装は `spawn()` でZellijを起動するが、TTYなしの環境ではセッションが正しく作成されない。また、儀式注入を外部から `zellij action` で行う設計は、セッション作成タイミングの問題で失敗する。

## What Changes

- **BREAKING**: `ovld summon` の動作を変更 - spawn + 外部注入 から 即時attach + KDL起動時注入へ
- KDLレイアウトで各ペインが `claude --system-prompt-file` を直接起動するように変更
- rituals ファイルの解決順序を追加: `./rituals/` → `~/.config/ovld/rituals/`
- 初回実行時にデフォルト設定を `~/.config/ovld/` に自動作成
- 既存の `ritual.rs` による外部注入ロジックを削除
- KDLレイアウトを実行時に動的生成（絶対パスで rituals を参照）

## Capabilities

### New Capabilities

- `config-management`: グローバル設定ディレクトリ（`~/.config/ovld/`）の管理、デフォルトrituals の自動展開、ローカル→グローバルのフォールバック解決

### Modified Capabilities

- `zellij-session`: セッション作成方式を `spawn()` から `status()` (attach) に変更、KDLレイアウトの動的生成
- `ritual-injection`: 外部 `zellij action` 注入からKDL `command` ディレクティブによる起動時注入に変更

## Impact

- `src/zellij/session.rs`: `start()` メソッドを attach モードに変更
- `src/commands/summon.rs`: KDL動的生成ロジック追加、config解決ロジック追加
- `src/army/ritual.rs`: 削除または大幅簡略化（外部注入不要に）
- `layouts/army.kdl`: テンプレートとして使用、実行時に絶対パス付きで生成
- 新規: `src/config.rs` - 設定ディレクトリ管理
- rituals/*.md: バイナリに埋め込み（`include_str!`）してデフォルトとして使用
