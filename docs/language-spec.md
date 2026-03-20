# Protorun言語仕様

Protorun言語の仕様は以下のセクションに分かれています。

## 目次

1. [序論](language-spec/01-introduction.md)
   * 1.1 言語の概要と目的
   * 1.2 設計理念
   * 1.3 コア機能の優先順位
   * 1.4 言語の進化と将来計画

2. [字句構造](language-spec/02-lexical-structure.md)
   * 2.1 字句構造の設計原則
   * 2.2 キーワード
   * 2.3 演算子と記号
   * 2.4 リテラル
   * 2.5 コメント

3. [型システム](language-spec/03-type-system.md)
   * 3.1 型システムの概要と目的
   * 3.2 基本型
   * 3.3 複合型
   * 3.4 代数的効果と型システム
   * 3.5 イテレータプロトコル

4. [宣言と文](language-spec/04-declarations-and-statements.md)
   * 4.1 概要
   * 4.2 変数宣言 (LetDecl)
   * 4.3 関数定義 (FunctionDefinitionExpr)
   * 4.4 型定義 (TypeDefinitionExpr)
   * 4.5 型エイリアス (AliasDefinitionExpr)
   * 4.6 トレイト定義 (TraitDefinitionExpr)
   * 4.7 効果インターフェース定義 (EffectDefinitionExpr)
   * 4.8 ハンドラ型定義 (HandlerDefinitionExpr)
   * 4.9 トレイト実装 (ImplDecl)
   * 4.10 return 文
   * 4.11 ループ構文

5. [式](language-spec/05-expressions.md)
   * 5.1 式の概要と設計原則
   * 5.2 ブロック式 (`BlockExpr`)
   * 5.3 条件式 (`IfExpr`)
   * 5.4 パターンマッチ (`MatchExpr`)
   * 5.5 リテラル式 (`LiteralExpr`, `StringInterpolation`)
   * 5.6 関数呼び出し式 (`CallExpr`)
   * 5.7 メンバーアクセス式 (`MemberAccessExpr`) と UFCS
   * 5.8 単項・二項演算式
   * 5.9 代入式 (`AssignmentExpr`)
   * 5.10 範囲式 (`RangeExpr`)
   * 5.11 グループ化式 (`GroupedExpr`)
   * 5.12 リスト内包表記 (`ListComprehension`)
   * 5.13 レコード構築式 (`RecordExpr`)
   * 5.14 非採用/将来検討

6. [代数的効果](language-spec/06-algebraic-effects.md)
   * 6.1 概念と目的
   * 6.2 効果インターフェースの定義 (EffectDefinitionExpr)
   * 6.3 ハンドラ型の定義 (HandlerDefinitionExpr)
   * 6.4 Effect パラメータ
   * 6.5 効果ハンドリング (WithExpr)
   * 6.6 効果操作の呼び出し
   * 6.7 継続制御
   * 6.8 break/continue の代数的効果セマンティクス
   * 6.9 ライフサイクル管理効果
   * 6.10 依存性注入としてのEffectパラメータ
   * 6.11 設計上の考慮事項

7. [モジュールシステム](language-spec/07-modules.md)
   * 7.1 モジュールシステムの概念と目的
   * 7.2 モジュール定義
   * 7.3 インポート
   * 7.4 モジュールの階層構造
   * 7.5 モジュールとファイルシステム
   * 7.6 エントリポイント
   * 7.7 モジュールの設計上の考慮事項

8. [標準ライブラリ](language-spec/08-standard-library.md)
   * 8.1 概要
   * 8.2 コアデータ型
   * 8.3 コアトレイト
   * 8.4 数値トレイト
   * 8.5 コア効果
   * 8.6 基本関数
   * 8.7 デシュガー規則

9. [サンプルプログラム](language-spec/09-examples.md)
   * 9.1 サンプルプログラムの目的と概要
   * 9.2 Hello World
   * 9.3 オプション型の定義と使用
   * 9.4 リスト処理
   * 9.5 バイナリツリー
   * 9.6 Result型とエラー処理
   * 9.7 マップとペアの操作
   * 9.8 型エイリアスと抽象化
   * 9.9 ループ
   * 9.10 文字列補間

10. [文法（EBNF）](language-spec/10-grammar.md)
    * 10.1 文法の目的と概要
    * 10.2 EBNF文法
    * 10.3 文法の説明
    * 10.4 特殊な構文要素
