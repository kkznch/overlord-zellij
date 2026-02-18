## MODIFIED Requirements

### Requirement: レイアウトを使ったセッション作成
`--new-session-with-layout` フラグで Zellij セッションを作成し、各ペインの Claude コマンドに `--mcp-config` を含めなければならない (SHALL)。レイアウト生成のテンプレート処理に失敗した場合、パニックせず `Result::Err` を返さなければならない (SHALL)。パスの UTF-8 変換に失敗した場合も同様に `Result::Err` を返さなければならない (SHALL)。

#### Scenario: セッション開始
- **WHEN** summon コマンドが実行された時
- **THEN** `zellij --session <name> --new-session-with-layout <path>` が実行される

#### Scenario: テンプレートレンダリング失敗
- **WHEN** KDL テンプレートのレンダリングに失敗した時
- **THEN** パニックせず、テンプレートエラーの詳細を含むエラーが返される

#### Scenario: パスの非UTF-8変換
- **WHEN** レイアウトパスが UTF-8 でない時
- **THEN** パニックせず、パスの問題を示すエラーが返される
