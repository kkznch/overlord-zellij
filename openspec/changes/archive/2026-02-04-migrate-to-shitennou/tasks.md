## 1. 役職システムの更新

- [x] 1.1 `src/army/roles.rs` を新しいRole enum（Overlord, Strategist, Inferno, Glacier, Shadow, Storm）に更新する
- [x] 1.2 各役職のdisplay_name()を日本語/英語名に更新する
- [x] 1.3 新しいファイルに対応するritual_file()のマッピングを更新する
- [x] 1.4 新しいタブ/ペイン構造に対応するpane_name()を更新する

## 2. KDLレイアウト

- [x] 2.1 `layouts/army.kdl` を3タブレイアウト（command, battlefield, support）に置き換える
- [x] 2.2 commandタブ: 魔王 + 軍師の縦分割
- [x] 2.3 battlefieldタブ: focus=trueの業火単一ペイン
- [x] 2.4 supportタブ: 氷結 + 常闇 + 疾風の横3分割

## 3. 儀式ファイル

- [x] 3.1 旧儀式ファイル（overlord.md, strategist.md, legion_*.md）を削除する
- [x] 3.2 rituals/overlord.md を司令層プロンプトで作成する
- [x] 3.3 rituals/strategist.md を作戦層プロンプトで作成する
- [x] 3.4 rituals/inferno.md をLogic & Coreプロンプト+ワークフロー指示で作成する
- [x] 3.5 rituals/glacier.md をArch & Refactorプロンプト+ワークフロー指示で作成する
- [x] 3.6 rituals/shadow.md をAudit & Securityプロンプト+ワークフロー指示で作成する
- [x] 3.7 rituals/storm.md をUI & Docsプロンプト+ワークフロー指示で作成する

## 4. 儀式注入ロジック

- [x] 4.1 `src/army/ritual.rs` を新しいタブナビゲーション（command/battlefield/support）に更新する
- [x] 4.2 3タブ構造に対応するペインフォーカスロジックを更新する

## 5. 検証

- [x] 5.1 ビルドしてコンパイルエラーがないことを確認する
- [x] 5.2 `ovld status` を実行して新しい役職表示を確認する
