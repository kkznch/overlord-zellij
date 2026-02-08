## Added Requirements

### Requirement: レイアウトを使用したセッション作成
システムは指定されたKDLレイアウトファイルを使用してZellijセッションを作成しなければならない（SHALL）。

#### Scenario: セッションの開始
- **WHEN** summonコマンドが実行されたとき
- **THEN** システムが `zellij --session <name> --layout <path>` を実行すること

### Requirement: セッション存在チェック
システムは新規作成前にセッションが既に存在するかどうかを確認しなければならない（SHALL）。

#### Scenario: 既存セッションの確認
- **WHEN** システムがセッションを確認するとき
- **THEN** `zellij list-sessions` を実行し、出力からセッション名を検索すること

### Requirement: セッション終了
システムはセッションをキルし、オプションで削除することによって終了しなければならない（SHALL）。

#### Scenario: セッションのキル
- **WHEN** slayコマンドが実行されたとき
- **THEN** システムが `zellij kill-session <name>` を実行すること

#### Scenario: セッションデータの削除
- **WHEN** slayコマンドが完了したとき
- **THEN** クリーンアップのためにシステムが `zellij delete-session <name> --force` を実行すること

### Requirement: セッションアタッチ
システムは重複作成ではなく既存セッションにアタッチしなければならない（SHALL）。

#### Scenario: セッションへのアタッチ
- **WHEN** セッションが存在し、summonが呼ばれたとき
- **THEN** システムが `zellij attach <name>` を実行すること

### Requirement: KDLレイアウト構造
army.kdl レイアウトは3つのタブを定義しなければならない（SHALL）: overlord、strategist、legions。

#### Scenario: タブ構造
- **WHEN** レイアウトが読み込まれたとき
- **THEN** overlordタブがfocus=trueの単一ペインを含むこと
- **THEN** strategistタブが単一ペインを含むこと
- **THEN** legionsタブが3つの水平ペイン（33%/33%/34%）を含むこと
