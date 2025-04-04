# 進捗状況

## 完了した作業

### 言語設計
- 言語の基本コンセプトの確立
- 型システムの設計
- 代数的効果システムの設計
- 所有権システムの設計
- 効果と所有権の統合モデルの設計
- トレイトシステムの設計
- エラー処理の設計（Result型ベース）
- 制御構造の設計（if式、match式、コレクション内包表記、bind式）
- 関数合成機能の設計（パイプライン演算子、関数合成演算子）
- ライフサイクル管理効果の設計
- 暗黙的パラメータの設計と効果システムとの統合

### 言語仕様
- 言語仕様の基本構造の確立
- 型システムの仕様策定
- 式と文の仕様策定
- 所有権システムの仕様策定
- 代数的効果の仕様策定
- モジュールシステムの仕様策定
- 標準ライブラリの仕様策定（基本部分）
- 例の作成
- 文法（EBNF）の策定
- 将来の拡張に関する考察
- 参考文献の整理
- 言語仕様書の構造改善と内容の充実（宣言、文、式の分離）
- 文法（EBNF）の改善と完全化（文末セミコロン削除、IfExpr更新）
- 文書記述のルールの確立と適用（ブロック式の説明移動と明確化）
- 宣言 (`03-declarations.md`) と文 (`04-statements.md`) の仕様を更新・詳細化。
  - `let`/`var` を宣言として `03-declarations.md` に記述。
  - 式文と `return` 文を `04-statements.md` に記述。
  - `return` 文は維持する方針を決定。
- 言語仕様書の構成変更（型定義とトレイト定義の移動）：
  - `02-type-system.md` から型定義とトレイト定義のセクションを削除。
  - `03-declarations.md` に型定義とトレイト定義のセクションを追加。
  - `02-type-system.md` は型システムの概念に、`03-declarations.md` は宣言構文に焦点を当てる構成に変更。
- 言語仕様書の構成変更（型定義の細分化）：
  - `03-declarations.md` の型定義セクションを、レコード型、代数的データ型（enum）、型エイリアスのサブセクションに分割。
  - 各型定義の説明を追加・整理。

### サンプルコード
- 電卓アプリケーション（calculator.pr）
- 状態カウンター（state_counter.pr）
- 所有権の例（ownership_example.pr）
- 基本的な挨拶プログラム（hello.pr）

### 実装
- AST（抽象構文木）モジュールの実装
- エラー処理システムの実装
- パーサーの実装（Nomパーサーコンビネータを使用）
- パーサーのモジュール分割と整理
- 基本的な式の構文解析器の実装：
  - リテラル（整数、浮動小数点、文字列、真偽値、ユニット）
  - 識別子
  - 二項演算
  - 単項演算
  - 関数呼び出し
  - カッコで囲まれた式
- 制御構造の構文解析器の実装：
  - if式（条件分岐）
  - match式（パターンマッチング）
  - コレクション内包表記
  - bind式（モナド的バインディング）
  - with式（代数的効果のハンドラ）
- 複合型の構文解析器の実装：
  - 配列型（`[Int]`）
  // - タプル型（`(Int, String)`） // 廃止
  - ジェネリック型（`Option<Int>`）
  - 関数型（`(Int, Int) -> Int`）
  - 効果付き型（`(String) -> String & IO`）
  - 参照型（`&Int`, `&mut Int`）
  - 所有権型（`own Resource`）
- ラムダ式の構文解析器の実装
- コレクションリテラルの構文解析器の実装：
  - リストリテラル（`[1, 2, 3]`）
  - マップリテラル（`{"key" -> 42}`）
  - セットリテラル（`#{1, 2, 3}`）
- 基本的な文の構文解析器の実装：
  - let宣言文
  - var宣言文
  - return文
  - 式文
- 関数宣言の構文解析器の実装
- パーサーのテストを機能別に分割
- シンボルテーブルの基本実装
- テストスイートの構築

### 実装計画
- 実装アプローチの決定（インタープリタから始め、徐々にコンパイラ機能を追加）
- 実装計画の策定
- マイルストーンの設定

## 進行中の作業
- モジュールシステムの実装
- 型宣言パーサーの実装と拡張
- シンボルテーブルの機能強化
- パーサーの安定化と最適化
- 型チェッカーの設計準備
- 言語仕様の詳細化と形式化

## 今後の作業
- 型チェッカーの実装：
  - 基本的な型チェック機能
  - 型推論アルゴリズム
  - 型エラーの報告機能
- 効果推論システムの実装：
  - 効果推論アルゴリズム
  - 効果の型チェック
  - 効果エラーの報告
- 所有権チェッカーの実装：
  - 所有権と借用の検証
  - ライフタイム解析
  - 所有権エラーの報告
- インタープリタの実装：
  - 評価器
  - 実行環境
  - プリミティブ操作
- 標準ライブラリの実装
- 開発ツールの開発（LSP、デバッガなど）

## 現在の課題
- 型チェッカーの設計における型推論、効果推論、所有権推論の相互作用の複雑さ
- 代数的効果システムの実装方法（特に継続の扱い）
- AST構造と型チェッカーの連携設計
- シンボルテーブルの拡張と型情報の管理
- パーサーの性能最適化

## 最近の成果
- コードのクリーンアップ：
  - 未使用のインポートを削除：`cargo fix --allow-dirty`を使用して、プロジェクト全体から未使用のインポートを削除
  - 主な修正ファイル：`common.rs`（5箇所）、`modules.rs`（3箇所）、`expressions.rs`（4箇所）、`literals.rs`（2箇所）など
  - 合計29箇所の未使用インポートを削除
  - コードの可読性と保守性の向上
  - 残りの警告（未使用の変数、関数、列挙型のバリアントなど）は将来の開発で対応予定

- パーサーのマルチパスアプローチへの変更：
  - パーサーの実装をマルチパスアプローチに変更し、コンテキスト情報の引き回しを削除
  - `simple`という接尾辞がついた関数（コンテキストを使わない純粋なパーサー関数）を標準の関数として採用し、コンテキストを使う関数を削除
  - `modules.rs`ファイルから、`parse_module_simple`などの関数を削除
  - `statements.rs`ファイルで、`let_statement_simple`などの関数の接尾辞を削除
  - `expressions.rs`ファイルで、`match_expr_simple`などの関数の接尾辞を削除
  - `declarations.rs`ファイルで、`parse_record_type_declaration_simple`などの関数の接尾辞を削除
  - デバッグ用のprint文を削除（`expressions.rs`と`modules.rs`ファイルから）
  - マルチパスアプローチでは、最初のパスでASTを構築し、その後の別のパスでコンテキスト情報（スコープなど）を処理

- モジュール構造の実装：
  - AST定義の拡張：`Module`、`ExportDecl`、`ImportDecl`、`ImportItem`などのモジュール関連の構造体・列挙型を追加
  - モジュール宣言パーサーの実装：`module Name { ... }`構文の解析
  - エクスポート宣言パーサーの実装：`export fn name() { ... }`や`export { name1, name2 }`構文の解析
  - インポート宣言パーサーの実装：`import { name1, name2 as alias } from "Module"`や`import "Module" as Alias`構文の解析
  - シンボルテーブルとの統合：モジュールスコープの開始と終了、エクスポートされた関数のシンボル登録

- 型宣言パーサーの実装：
  - AST定義の拡張：`TypeDecl`、`EnumVariant`、`TraitDecl`、`ImplDecl`などの型宣言関連の構造体・列挙型を追加
  - `Program`構造体に型宣言関連のフィールドを追加（`type_declarations`、`trait_declarations`、`impl_declarations`）
  - シンボルテーブルの拡張：`TypeInfo`構造体に型宣言の詳細情報を保持するフィールドを追加
  - 型宣言パーサーの実装：レコード型宣言、列挙型宣言、型エイリアス、トレイト宣言、実装宣言のパーサーを実装

- メンバーアクセス式（MemberAccessExpr）の実装：
  - AST拡張：`Expr` 列挙型に `MemberAccess` バリアントを追加
  - パーサー実装：`function_call` 関数を `postfix` 関数にリネームし、関数呼び出しとメンバーアクセスの両方をパースできるように拡張
  - メンバーアクセス式（`obj.property`）と関数呼び出し式（`func(args)`）を左結合性を持つ後置演算子として実装
  - チェーンされたメンバーアクセス（`obj.inner.property`）やメンバーアクセス後の関数呼び出し（`obj.method(args)`）のサポート

- シンボルテーブルの拡張と機能強化：
  - `Symbol`構造体に型情報と使用状態を追跡するフィールドを追加
  - 型定義の詳細情報を表す`TypeInfo`構造体と`TypeKind`列挙型を追加
  - シンボルの使用をマークするメソッド`mark_symbol_used`の実装
  - 未使用シンボルを検出するメソッド`find_unused_symbols`の実装
  - 特定の種類のシンボルを検索するメソッド`find_symbols_by_kind`の実装
  - スコープ内のすべてのシンボルを取得するメソッド`get_all_symbols`の実装
  - 型定義のシンボル登録ヘルパー関数`register_type_symbol`の追加

- テストの追加：
  - 単体テストの追加（型シンボル登録、シンボル使用追跡、シンボル種類検索）
  - プロパティベーステストの追加（ランダムな入力に対するテスト）
  - proptestライブラリの導入

- タプル型の廃止：
  - 構文の曖昧さ（型のグループ化 `(T)` と単一要素タプル `(T,)` の区別）と、タプル型による可読性低下の懸念から、タプル型を言語仕様から削除
  - 文法定義 (`docs/language-spec/09-grammar.md`) から `TupleType` を削除
  - AST (`src/protorun/ast/mod.rs`) の `Type` enum から `Tuple` バリアントを削除
  - パーサー (`src/protorun/parser/types.rs`) から `tuple_type` 関数と関連コードを削除
  - テストコード (`src/protorun/parser/tests_types.rs`) からタプル型関連のテストを削除
  - 関連するヘルパー関数 (`src/main.rs` の `type_to_string`) を修正

- 言語仕様ドキュメントの整合性向上：
  - `docs/language-spec/` ディレクトリ内の全ドキュメントを確認し、矛盾や一貫性のない記述を修正。
  - 文法定義 (`11-grammar.md`) を更新し、`export` キーワードと `enum` キーワードのルールを追加。
  - 廃止されたタプル*型*に関する記述を削除 (`02-type-system.md`)。
  - メンバーアクセス式 (`.`) の説明を追加 (`05-expressions.md`)。
  - 可視性キーワードを `pub` から `export` に統一 (`08-modules.md`, `09-standard-library.md`)。
  - `List` の定義を `enum` 構文に統一 (`09-standard-library.md`)。
  - 効果ハンドラの定義・適用構文を `:` を使う形式に統一 (`09-standard-library.md`, `10-examples.md`)。
  - 未定義の `try...catch` 構文を使用していた例を、`Result` 型と `noresume` ハンドラを使用する形に修正 (`07-algebraic-effects.md`, `10-examples.md`)。
  - その他、不完全な記述や重複部分を整理 (`07-algebraic-effects.md`)。
  - 言語仕様書の構成を宣言、文、式に分離 (`03-declarations.md`, `04-statements.md`, `05-expressions.md` を作成/更新)。
  - 文末および `BindExpr` 内のセミコロンを不要とする仕様変更を反映 (`11-grammar.md`, `04-statements.md`, `05-expressions.md`, `10-examples.md`)。
  - ブロック式の役割を明確化し、`IfExpr` で単一式を許容するように文法と説明を更新 (`11-grammar.md`, `04-statements.md`, `05-expressions.md`)。
  - ブロック式の説明を `04-statements.md` から `05-expressions.md` に移動。
- 宣言と文に関するドキュメントの整理:
  - `03-declarations.md` に `let` と `var` の詳細説明を追加。
  - `04-statements.md` に式文と `return` 文の詳細説明を追加。
  - 仕様書の構成を整理し、`let`/`var` を宣言、式文/`return` を文として分類。

## 次のマイルストーン
1. **言語仕様の完成**（目標：2025年4月末）
   - 言語仕様書の完成
   - 文法の完全な形式化
   - 標準ライブラリの仕様完成

2. **プロトタイプインタープリタの実装**（目標：2025年6月末）
   - 基本的な構文解析（完了）
   - 型チェック
   - 効果推論
   - 所有権チェック
   - 簡単なプログラムの実行

3. **コンパイラの基本機能実装**（目標：2025年9月末）
   - 中間表現（IR）の設計と実装
   - 最適化パスの実装
   - バックエンドの選定と実装

4. **言語ツールの開発**（目標：2025年12月末）
   - LSP（Language Server Protocol）の実装
   - デバッガの実装
   - パッケージマネージャの実装
