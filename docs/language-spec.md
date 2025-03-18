# Protorun言語仕様

Protorun言語の仕様は以下のセクションに分かれています。各セクションをクリックして詳細を確認してください。

## 目次

1. [序論と字句構造](language-spec/01-introduction.md)
   - 設計理念
   - コア機能の優先順位
   - キーワード
   - 演算子
   - リテラル
   - コメント

2. [型システム](language-spec/02-type-system.md)
   - 基本型
   - 複合型
   - 型定義
   - トレイト（インターフェース）
   - リソース型
   - リソースパターン型クラス

3. [式と文](language-spec/03-expressions.md)
   - 変数宣言
   - 関数定義
   - 制御構造
   - パターンマッチング
   - 関数合成

4. [所有権システム](language-spec/04-ownership.md)
   - 所有権
   - 借用
   - ライフタイム
   - リソース管理

5. [代数的効果](language-spec/05-algebraic-effects.md)
   - 効果の定義
   - 効果ハンドラの定義
   - ライフサイクル管理を持つ効果
   - コンテキスト型
   - 効果の使用
   - 効果ハンドラの使用
   - 効果の合成

6. [モジュールシステムと標準ライブラリ](language-spec/06-modules.md)
   - モジュール定義
   - インポート
   - コアデータ構造
   - I/O操作
   - 並行処理

7. [サンプルプログラム](language-spec/07-examples.md)
   - 簡単な計算機
   - 状態を持つカウンター
   - ファイル処理（ライフサイクル管理効果を使用）

8. [文法（EBNF）](language-spec/08-grammar.md)

9. [今後の展望と参考文献](language-spec/09-future.md)
   - 今後の展望
   - 参考文献

## 更新履歴

- 2025-03-18: 言語構文の一貫性向上
  - 効果ハンドラの定義構文を`handler HandlerName for Effect`から`handler HandlerName: Effect`に変更
  - 効果ハンドラの使用構文を`with Effect handled by Handler`から`with Handler: Effect`に変更
  - 関数の効果注釈を`fn name(): ReturnType with Effect`から`fn name(): ReturnType & Effect`に変更
  - スコープ付き効果構文を`with scoped effect Name`から`with scoped Name`に変更
  - ライフサイクル管理効果構文を`effect Name with lifecycle`から`effect Name: lifecycle`に変更
  - トレイト継承構文を`trait Child extends Parent`から`trait Child: Parent`に変更
  - トレイト実装構文を`impl Trait for Type`から`impl Type: Trait`に変更
  - 管理型定義構文を`managed type Name`から`managed Name`に変更
  - 型注釈パターン「エンティティ: 型」の一貫した使用による言語全体の統一性向上

- 2025-03-17: 代数的効果の実装方法を更新
  - 効果ハンドラの定義にimpl構文に似た`handler`構文を導入
  - 暗黙的な継続と明示的な継続のハイブリッドアプローチを採用
  - 特殊な継続制御（`noresume`、`multiresume`）のサポートを追加
