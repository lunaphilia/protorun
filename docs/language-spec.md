# Protorun言語仕様

Protorun言語の仕様は以下のセクションに分かれています。各セクションをクリックして詳細を確認してください。

## 目次

1.  [序論](language-spec/01-introduction.md)
    *   1.1 設計理念
    *   1.2 コア機能の優先順位
2.  [字句構造](language-spec/02-lexical-structure.md)
    *   2.1 キーワード
    *   2.2 演算子
    *   2.3 リテラル
    *   2.4 コメント
3.  [型システム](language-spec/03-type-system.md)
    *   3.1 型システムの概要と目的
    *   3.2 基本型
    *   3.3 複合型
    *   3.4 代数的効果と型システム
4.  [宣言](language-spec/04-declarations.md)
    *   4.1 宣言の概要
    *   4.2 変数宣言 (`let`, `var`)
    *   4.3 関数定義 (`fn`)
    *   4.4 型定義 (`type`, `enum`)
    *   4.5 効果インターフェースとハンドラ型の定義 (`effect`, `handler`)
    *   4.6 トレイト定義と実装 (`trait`, `impl`)
5.  [文](language-spec/05-statements.md)
    *   5.1 文の概要
    *   5.2 return文 (`ReturnStatement`)
6.  [式](language-spec/06-expressions.md)
    *   6.1 式の概要と設計原則
    *   6.2 ブロック式 (`BlockExpr`)
    *   6.3 制御構造 (`if`, `match`, `bind`, `with`)
    *   6.4 パターンマッチング
    *   6.5 関数合成
    *   6.6 効果操作呼び出し式
    *   6.7 メンバーアクセス式
7.  [所有権システム](language-spec/07-ownership.md)
    *   7.1 所有権
    *   7.2 借用
    *   7.3 ライフタイム
    *   7.4 リソース管理
8.  [代数的効果](language-spec/08-algebraic-effects.md)
    *   8.1 代数的効果の概念と目的
    *   8.2 効果インターフェースの定義 (`effect`)
    *   8.3 ハンドラ型の定義 (`handler`)
    *   8.4 Effect パラメータによる効果の宣言
    *   8.5 効果ハンドラインスタンスの提供 (`with` 構文)
    *   8.6 継続制御
    *   8.7 ライフサイクル管理効果 (RAII連携)
    *   8.8 Effect パラメータと依存性注入
    *   8.9 設計上の考慮事項 (更新)
9.  [モジュールシステム](language-spec/09-modules.md)
    *   9.1 モジュール定義
    *   9.2 インポート
    *   9.3 エクスポート
10. [標準ライブラリ](language-spec/10-standard-library.md)
    *   10.1 コアデータ構造
    *   10.2 I/O操作
    *   10.3 並行処理
11. [サンプルプログラム](language-spec/11-examples.md)
    *   11.1 サンプルプログラムの目的と概要
    *   11.2 簡単な計算機 (例外効果)
    *   11.3 状態を持つカウンター (状態効果)
    *   11.4 ファイル処理 (効果とハンドラ)
    *   11.5 依存性注入としての Effect パラメータ
12. [文法（EBNF）](language-spec/12-grammar.md)
    *   12.1 文法の目的と概要
    *   12.2 EBNF文法
    *   12.3 文法の説明
    *   12.4 特殊な構文要素

## 更新履歴

- 2025-03-19: コンテキスト型と管理型の統合
  - コンテキスト型を削除し、管理型に暗黙的なパラメータとしての機能を追加
  - 関数シグネチャで`with db: Database`のように暗黙的なパラメータを宣言可能に
  - `with`式を使用して暗黙的なコンテキストを提供する構文を追加
  - 言語仕様の構成を整理し、ファイルの通し番号を修正

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
