## ADDED Requirements

### Requirement: グローバル設定の強制再展開
`--force` フラグによりグローバル設定ディレクトリの儀式ファイルを強制的に上書き再展開できなければならない (SHALL)。

#### Scenario: 強制再展開時の動作
- **WHEN** 強制再展開が要求された時
- **THEN** ディレクトリ存在チェックをスキップし、`extract_rituals_to()` で全ファイルを上書きする
