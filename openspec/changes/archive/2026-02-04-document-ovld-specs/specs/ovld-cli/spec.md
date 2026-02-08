## Added Requirements

### Requirement: summonコマンドでセッションを開始する
システムは魔王軍レイアウトで新しいZellijセッションを開始する `summon` サブコマンドを提供しなければならない（SHALL）。

#### Scenario: 新規セッションの作成
- **WHEN** ユーザーが `ovld summon` を実行したとき
- **THEN** システムが army.kdl レイアウトで "overlord" という名前のZellijセッションを作成すること

#### Scenario: セッションが既に存在する場合
- **WHEN** ユーザーが `ovld summon` を実行し、セッション "overlord" が既に存在するとき
- **THEN** 新規作成ではなく既存セッションにアタッチすること

### Requirement: slayコマンドでセッションを終了する
システムは魔王軍セッションを終了しプロセスをクリーンアップする `slay` サブコマンドを提供しなければならない（SHALL）。

#### Scenario: 確認付きセッション終了
- **WHEN** ユーザーが `ovld slay` を実行したとき
- **THEN** 終了前に確認プロンプトを表示すること

#### Scenario: 強制終了
- **WHEN** ユーザーが `ovld slay --force` を実行したとき
- **THEN** 確認なしでセッションを終了すること

#### Scenario: セッションが見つからない場合
- **WHEN** ユーザーが `ovld slay` を実行し、セッションが存在しないとき
- **THEN** エラーメッセージ "Session 'overlord' not found" を表示すること

### Requirement: statusコマンドで魔王軍の状態を表示する
システムは魔王軍セッションの現在の状態を表示する `status` サブコマンドを提供しなければならない（SHALL）。

#### Scenario: アクティブなセッション
- **WHEN** ユーザーが `ovld status` を実行し、セッションが存在するとき
- **THEN** セッション名、ステータス "ACTIVE"、および魔王軍の階層一覧を表示すること

#### Scenario: セッションなし
- **WHEN** ユーザーが `ovld status` を実行し、セッションが存在しないとき
- **THEN** ステータス "NOT FOUND" とsummonコマンドの使用ヒントを表示すること

### Requirement: カスタムセッション名
システムは `--session` フラグでカスタムセッション名を指定できなければならない（SHALL）。

#### Scenario: カスタムセッション名
- **WHEN** ユーザーが `ovld summon --session myarmy` を実行したとき
- **THEN** "myarmy" という名前でセッションを作成すること

### Requirement: カスタムレイアウト
システムは `--layout` フラグでカスタムレイアウトを指定できなければならない（SHALL）。

#### Scenario: カスタムレイアウトファイル
- **WHEN** ユーザーが `ovld summon --layout minimal` を実行したとき
- **THEN** セッションに layouts/minimal.kdl を使用すること
