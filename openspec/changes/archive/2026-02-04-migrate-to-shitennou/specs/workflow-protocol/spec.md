## Added Requirements

### Requirement: Glacierが先に構造を定義すること
氷結の将は、Infernoが実装を開始する前に型・インターフェース・ディレクトリ構造を定義すること。

#### Scenario: 型定義の引き継ぎ
- **WHEN** 新機能がリクエストされた場合
- **THEN** Glacierがtrait/struct定義を作成する
- **THEN** Infernoがその定義を使ってロジックを実装する

### Requirement: Infernoは純粋なロジックに専念すること
業火の将は、UIやドキュメントを扱わず、ビジネスロジックとアルゴリズムにのみ専念すること。

#### Scenario: ロジック実装
- **WHEN** InfernoがGlacierから型定義を受け取った場合
- **THEN** Infernoはコアロジックのみを実装する
- **THEN** InfernoはCSS・README・UIコードを書かない

### Requirement: Shadowがテストを担当すること
常闘の将は、Infernoに代わってテストコードの生成とデバッグを行うこと。

#### Scenario: テスト生成
- **WHEN** Infernoがロジック実装を完了した場合
- **THEN** Shadowがその実装に対するテストケースを作成する
- **THEN** ShadowがInfernoに修正すべきバグを報告する

### Requirement: Stormが外部向けコンテンツを担当すること
疾風の将は、すべてのUIコンポーネント・ドキュメント・ユーザー向けコンテンツを並行して作成すること。

#### Scenario: ドキュメント作成
- **WHEN** Infernoのロジックが利用可能になった場合
- **THEN** StormがREADMEとAPIドキュメントを作成する
- **THEN** 必要に応じてStormがフロントエンドコンポーネントを実装する

### Requirement: ritualにパイプライン指示を含めること
各ritualプロンプトには、他の将との間で何を受け取り何を渡すかの指示を含めること。

#### Scenario: ritualにパイプラインが含まれる
- **WHEN** ritualが将に注入された場合
- **THEN** プロンプトに「Xから受け取る」「Yに渡す」という指示が含まれる
