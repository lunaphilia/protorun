# アクティブコンテキスト

## 現在の焦点
- 新しいプログラミング言語「Protorun」の設計と実装
- 言語仕様の詳細化と形式化
- 実装計画の策定
- サンプルコードの充実
- 言語仕様書の構造と内容の改善

## 最近の変更
- パーサーのデバッグコードの削除と括弧式の修正：
  - 括弧式のパース処理を修正し、`(1 + 2) * 3`のような式を正しく解析できるように
  - `expression`関数内での`paren_expr`と`logical_or`の順序を調整
  - `is_lambda_pattern`関数の呼び出しを削除（`lambda_expr`が`paren_expr`よりも先に試されるため不要）
  - すべてのデバッグコード（`println!`文）を削除してコードをクリーンアップ
  - テストケースの実行と検証

- シンボルテーブルの基本実装：
  - シンボル情報を表す`Symbol`構造体の実装
  - スコープの種類（グローバル、モジュール、関数、ブロック、ループ）の定義
  - シンボルの種類（変数、関数、型、パラメータ）の定義
  - シンボルの追加と検索機能の実装
  - 親スコープへの遡及検索機能の実装
  - `ParserContext`とシンボルテーブルの統合
  - シンボルテーブルのテストケースの追加と検証

- ラムダ式の構文解析器の実装：
  - ラムダ式（`(x) => x + 1`）の構文解析
  - 型注釈付きパラメータ（`(x: Int) => x * 2`）のサポート
  - 複数パラメータ（`(x, y) => x + y`）のサポート
  - 括弧式（`(1 + 2) * 3`）との曖昧性解決
  - 先読み（peek）を使用した構文パターン判別
  - テストケースの追加と検証

- コレクションリテラルの構文解析器の実装：
  - リストリテラル（`[1, 2, 3]`）の構文解析
  - マップリテラル（`{"key" -> 42}`）の構文解析
  - セットリテラル（`#{1, 2, 3}`）の構文解析
  - 空のコレクションリテラルのサポート
  - ネストされたコレクションリテラルのサポート
  - AST定義の拡張（ListLiteral、MapLiteral、SetLiteral）
  - テストケースの追加と検証

- パーサーモジュールのリファクタリング：
  - パーサーを機能別に6つのモジュールに分割（common, literals, patterns, types, statements, expressions）
  - 各モジュールの責任範囲を明確化
  - Span情報の計算を改善し、エラーメッセージをより詳細に
  - 循環参照の問題を解決（patterns.rsとexpressions.rsの相互依存を解消）
  - 共通パターンをヘルパー関数として抽出
  - コードの可読性、保守性、拡張性を向上
- 制御構造の構文解析器の実装と検証：
  - if式（条件分岐）の構文解析
  - match式（パターンマッチング）の構文解析
  - コレクション内包表記の構文解析（リスト、マップ、セット）
  - bind式（モナド的バインディング）の構文解析
  - with式（代数的効果のハンドラ）の構文解析
  - テストコードのクリーンアップとデバッグ出力の削除
  - 未使用変数の警告修正

- 複合型の構文解析器の実装と検証：
  - 配列型（`[Int]`）の構文解析
  - タプル型（`(Int, String)`）の構文解析
  - ジェネリック型（`Option<Int>`）の構文解析
  - 関数型（`(Int, Int) -> Int`）の構文解析
  - 効果付き型（`(String) -> String & IO`）の構文解析
  - 参照型（`&Int`, `&mut Int`）の構文解析
  - 所有権型（`own Resource`）の構文解析
  - 複雑な複合型（`Option<(Int, &mut String)>`）の構文解析
  - テストケースの追加と検証

- インタープリタの初期実装：
  - AST（抽象構文木）モジュールの実装：言語の構造を表現するデータ型を定義
  - エラー処理システムの実装：構文エラー、型エラーなど様々なエラーを統一的に扱う仕組み
  - パーサーの実装：Nomパーサーコンビネータを使用した効率的な構文解析
  - テストスイートの構築：各コンポーネントの単体テストを充実
  - サンプルコード（hello.pr）の作成

- パーサーの改善：
  - レキサーモジュールを削除し、Nomパーサーコンビネータに完全に移行
  - 使用されていない関数（from_str）を削除
  - エラーメッセージの改善（コンテキスト情報を追加）
  - 比較演算子のパース順序を最適化（2文字演算子を先に試す）
  - コードの整理と簡素化

- テストの修正：
  - `test_parse_error_unexpected_eof`テストを削除
  - エラー処理を改善し、より明確なエラーメッセージを提供

- 文法（EBNF）の改善：
  - 言語仕様で解説されている構文要素を完全にカバーする文法を作成
  - 暗黙的パラメータ（ImplicitParamList）の明示的な定義
  - ライフサイクル管理効果の操作（`acquire`と`release`）を効果操作に追加
  - コレクションリテラルと内包表記（リスト、マップ、セット）の構文を明示的に定義
  - 効果ハンドラの構文（暗黙的/明示的な継続、noresume、multiresume）を明確に定義
  - 範囲式（RangeExpr）の追加
  - with式の拡張（複数のハンドラをカンマで区切って指定）
  - 所有権修飾子（`own`、`&`、`&mut`）の構文を明確に定義
  - 文書の簡潔化（詳細な説明を削除し、他の文書への参照を提供）

- 文書記述のルールの確立：
  - 仕様書は現在の状態と、その仕様としている意義を中心に記述
  - 他の文書で解説されている内容を重複して記述しない
  - 過去の経緯や廃止された構文に関する情報は記載しない
  - 文書間の参照を適切に行い、重複を避ける
  - 各文書は自身の責任範囲に集中し、他の文書の内容を参照する
  - 簡潔で一貫性のある記述を心がける

- エラー処理の一本化：
  - 例外処理（try/catch/throw）とResult型によるエラー伝搬の二重化を解消
  - Result型ベースのエラー処理に統一
  - Exception効果の定義を修正し、Result型を返すように変更
  - runWithExceptionの実装を簡素化
  - Result型に便利なユーティリティメソッドを追加（mapErr, flatMapErr, unwrap, unwrapOr, unwrapOrElse, isOk, isErr, unwrapErr）
  - Result型のユーティリティ関数を追加（all, any）
  - サンプルコード（calculator.pr, ownership_example.pr）を修正してResult型ベースのエラー処理を使用
  - ?演算子によるエラー伝搬の推奨

- ライフサイクル管理効果のモデル簡素化：
  - 効果ハンドラから冗長な`onClose`メソッドを削除
  - リソース管理をシンプル化：`acquire`で獲得したリソースはスコープ終了時に自動的に`release`で解放
  - 内部状態の追跡（activeConnectionsなど）を削除
  - C++のRAIIパターンやRustの所有権システムにより近いモデルに

- ライフサイクル管理効果と暗黙的パラメータの記述を05-algebraic-effects.mdに集約：
  - 02-type-system.md、04-ownership.md、03-expressions.md、08-examples.mdから関連する詳細な記述を削除
  - 05-algebraic-effects.mdの5.4節「ライフサイクル管理効果」と5.8節「暗黙的パラメータと効果システム」を拡充
  - 各ファイルに05-algebraic-effects.mdへの参照を追加
  - 重複を排除し、一貫性のある説明を提供
  - ライフサイクル管理効果の概念、目的、定義、使用方法、ハンドラ、利点などを詳細に説明
  - 暗黙的パラメータの概念、目的、宣言、提供、ライフサイクル管理効果との統合などを詳細に説明

- 暗黙的コンテキストと効果の統合：
  - 管理型を削除し、ライフサイクル管理効果（`LifecycleEffect<R>`型）に統合
  - 暗黙的パラメータの構文を`(with param: Type)`に統一
  - `lifecycle`キーワードと`with cleanup`修飾子を削除
  - リソース獲得と解放の関数名を`acquire`と`release`に固定
  - 効果ハンドラの`finalize`メソッドを`onClose`メソッドに変更
  - 言語仕様書の関連セクションを更新（05-algebraic-effects.md, 04-ownership.md, 03-expressions.md, 09-grammar.md）
- 言語仕様書の整合性向上：
  - EBNF文法から廃止された`context type`キーワードを削除
  - 最新の構文変更をEBNF文法に反映（`managed type`→`managed`、`extends`→`:`など）
  - 関数の効果注釈を`with EffectType`から`& EffectType`に更新
  - ライフサイクル管理効果の構文を`with lifecycle`から`: lifecycle`に更新
  - スコープ付き効果式の構文を`with scoped effect`から`with scoped`に更新
  - 所有権システムの章番号を「5.」から「4.」に修正
  - 文法の説明セクションを更新して最新の設計決定を反映
  - EBNF文法の`TypeRef`規則に参照型（`&`と`&mut`）の構文を追加
  - 所有権システムの章に参照型の構文と意味論に関する詳細な説明を追加
- コンテキスト型と管理型の統合：
  - コンテキスト型を削除し、管理型に暗黙的なパラメータとしての機能を追加
  - 関数シグネチャで`with db: Database`のように暗黙的なパラメータを宣言可能に
  - `with`式を使用して暗黙的なコンテキストを提供する構文を追加
  - 言語仕様の構成を整理し、ファイルの通し番号を修正
- 言語の基本コンセプトの確立
- 型システム、関数型パラダイム、メモリ安全性の統合アプローチの検討
- 代数的効果と所有権システムの統合設計
- トレイトとデータ型の継承に関する検討と決定
- nullの排除とOption/Result型の導入
- 効果スコープの導入
- 関数合成のサポート強化（パイプライン演算子、関数合成演算子）
- サンプルコードの更新と充実（calculator.pr, state_counter.pr, ownership_example.pr）
- 制御構造の検討と決定（if式とmatch式の両方を維持）
- 設計決定の文書化（docs/design-decisions/control-structures.md）
- 実装計画の詳細化（docs/implementation-plan.md）
- with式の返り値と用途に関する言語仕様の詳細化：
  - with式がブロック内の最後の式の評価結果を返すことを明確化
  - with式の返り値を使用する具体的な例と実用的なパターンの追加
  - with式の返り値の実用的な価値（効果の局所化と結果の取得、合成性の向上、効果の組み合わせと結果の合成、リソース管理の安全性）の説明
- 言語仕様の改善：
  - ライフサイクル管理効果の言語レベルサポート（`LifecycleEffect<R>`型）
  - 効果のスコープ化（with scoped effect構文）
  - 所有権を考慮した効果（ownキーワード）
  - 文法の拡張（新しい構文要素をEBNF文法に追加）
- 言語仕様の簡素化：
  - キーワード数の削減（7個削除、2個追加、純減5個）
  - 構文の統一と命名の一貫性向上
  - 複雑な機能の簡素化と直感的な表現
- 代数的効果の実装方法の更新：
  - 効果ハンドラの定義にimpl構文に似た`handler`構文を導入
  - 暗黙的な継続と明示的な継続のハイブリッドアプローチを採用
  - 特殊な継続制御（`noresume`、`multiresume`）のサポートを追加
  - 言語仕様を複数のファイルに分割して整理
- 言語仕様書の構造改善：
  - コンテキスト型と代数的効果の分離（新しいセクション「06-context-types.md」の作成）
  - セクション番号の調整（全12セクション）
  - 各セクションに仕様の理由や意図を追加（設計原則、利点、インスピレーションの源など）
  - 標準ライブラリセクションの充実
  - 参考文献セクションの拡充
- 制御構造の改善：
  - for式とdo式の廃止
  - Pythonスタイルのコレクションリテラル内包表記の導入（`[x * 2 for x <- numbers if x % 2 == 0]`）
  - モナド連鎖のためのbind式の導入（`bind { x <- expr1; y <- expr2; ... }`）
  - 用途に応じた構文の明確な区別（コレクション操作とモナド連鎖）
  - 言語仕様書の更新（03-expressions.md）
- 言語構文の一貫性向上：
  - 効果ハンドラの定義構文を`handler HandlerName for Effect`から`handler HandlerName: Effect`に変更
  - 効果ハンドラの使用構文を`with Effect handled by Handler`から`with Handler: Effect`に変更
  - 関数の効果注釈を`fn name(): ReturnType with Effect`から`fn name(): ReturnType & Effect`に変更
  - スコープ付き効果構文を`with scoped effect Name`から`with scoped Name`に変更
  - 型注釈パターン「エンティティ: 型」の一貫した使用による言語全体の統一性向上

## 最近の変更
- 型宣言パーサーの実装：
  - AST定義の拡張：`TypeDecl`、`EnumVariant`、`TraitDecl`、`ImplDecl`などの型宣言関連の構造体・列挙型を追加
  - `Program`構造体に型宣言関連のフィールドを追加（`type_declarations`、`trait_declarations`、`impl_declarations`）
  - シンボルテーブルの拡張：`TypeInfo`構造体に型宣言の詳細情報を保持するフィールドを追加
  - 型宣言パーサーの実装：レコード型宣言、列挙型宣言、型エイリアス、トレイト宣言、実装宣言のパーサーを実装
  - パーサーモジュールの統合：新しい型宣言パーサーをエクスポート
  - プログラムパーサーの更新：`program`関数を更新して型宣言をパースできるように
  - テストの実装と検証：各型宣言パーサーのテストケースを追加し、すべてのテストが正常に通過することを確認

- メンバーアクセス式（MemberAccessExpr）の実装：
  - AST拡張：`Expr` 列挙型に `MemberAccess` バリアントを追加
  - パーサー実装：`function_call` 関数を `postfix` 関数にリネームし、関数呼び出しとメンバーアクセスの両方をパースできるように拡張
  - メンバーアクセス式（`obj.property`）と関数呼び出し式（`func(args)`）を左結合性を持つ後置演算子として実装
  - チェーンされたメンバーアクセス（`obj.inner.property`）やメンバーアクセス後の関数呼び出し（`obj.method(args)`）のサポート
  - テストケースの追加と検証

- パーサーコードのリファクタリング：
  - `src/protorun/parser/mod.rs`の`parse_expression`メソッドからデバッグ用の`println!`文を削除
  - `src/protorun/parser/expressions.rs`から未使用の`is_lambda_pattern`関数を削除
  - `src/protorun/parser/literals.rs`の`tuple`関数のインポートを修正
  - `src/protorun/parser/types.rs`の`type_parser`関数を`parse_type`に名前変更し、より一貫性のある命名規則に変更
  - 関連するすべての参照も更新
  - テストを実行し、すべてのテストが正常に通過することを確認

- シンボルテーブルを使ったパーサーの構造整理：
  - ParserContextのenter_scopeとexit_scopeメソッドを内部可変性を使って実装し、不変参照で呼び出せるように修正
  - パーサー関数を不変参照(&ParserContext)を受け取るように修正
  - ブロック式のパース時にスコープの開始と終了を行うように実装
  - スコープ管理のテストを追加し、正しく機能することを確認

- シンボルテーブルの拡張と機能強化：
  - `Symbol`構造体に型情報と使用状態を追跡するフィールドを追加
  - 型定義の詳細情報を表す`TypeInfo`構造体と`TypeKind`列挙型を追加
  - シンボルの使用をマークするメソッド`mark_symbol_used`の実装
  - 未使用シンボルを検出するメソッド`find_unused_symbols`の実装
  - 特定の種類のシンボルを検索するメソッド`find_symbols_by_kind`の実装
  - スコープ内のすべてのシンボルを取得するメソッド`get_all_symbols`の実装
  - 型定義のシンボル登録ヘルパー関数`register_type_symbol`の追加

- ParserContextの拡張：
  - シンボルテーブルとの連携を強化
  - スコープのネスト深度を取得するメソッド`scope_depth`の追加
  - 現在のスコープの種類を取得するメソッド`current_scope_kind`の追加

- 識別子パーサーの拡張：
  - シンボルの使用を追跡する機能を追加
  - 識別子が参照されたときに自動的に使用状態をマーク

- テストの追加：
  - 単体テストの追加（型シンボル登録、シンボル使用追跡、シンボル種類検索）
  - プロパティベーステストの追加（ランダムな入力に対するテスト）
  - proptestライブラリの導入

## 次のステップ
- 型チェッカーの設計と実装：
  - 基本的な型チェック機能の設計
  - 型推論アルゴリズムの実装
  - 型エラーの報告機能の実装
  - シンボルテーブルとの統合

- 効果推論システムの設計と実装：
  - 効果推論アルゴリズムの設計
  - 効果の型チェック機能の実装
  - 効果エラーの報告機能の実装

- 所有権チェッカーの設計と実装：
  - 所有権と借用の検証機能の設計
  - ライフタイム解析の実装
  - 所有権エラーの報告機能の実装

- 言語仕様の継続的な改善と形式化
- コンパイラ/インタープリタの実装計画の詳細化
- 開発環境のセットアップ
- プロトタイプ実装の開始
- 型推論、効果推論、所有権推論の相互作用の詳細設計
- 標準ライブラリの設計と仕様策定の継続
- EBNF文法の完全な形式化
- 効果ハンドラの実装方法の詳細設計
- 言語仕様書の継続的な改善（例の追加、説明の充実）
- サンプルコードの更新（新しいコレクションリテラル内包表記とbind式を使用）
- 新しい構文に合わせたサンプルコードの更新
- 文書記述のルールの徹底と一貫した適用

## 開発ワークフロールール
- プロダクションコードを小さく修正し、こまめにテストコードを書き、テストが通ることを確認してから次のプロダクションコードの修正を行う
- テストが失敗したときには思い込みで修正する前にデバッグを行って事実から原因を特定し、的確にかつ簡潔に修正する
- テストは最上位から入力を通すのではなく、関数ごとに書いてボトムアップに関数の振る舞いが正しいことを確認する。それを組み合わせたより大きな振る舞いのテストを書く際は、部品となる関数をモック化しても良い
- コミット前に必ず`git status`を実行し、変更内容を確認する
- 追加・修正・削除・リネームしたファイルをすべて`git add`または`git rm`で追跡対象に含める
- 関連する変更は一つのコミットにまとめる
- コミットメッセージは英語で、変更内容を明確に記述する
- 作業完了後は必ず`git push`を実行する

## 文書記述のルール
- 仕様書は現在の状態と、その仕様としている意義を中心に記述する
- 他の文書で解説されている内容を重複して記述しない
- 過去の経緯や廃止された構文に関する情報は記載しない
- 文書間の参照を適切に行い、重複を避ける
- 各文書は自身の責任範囲に集中し、他の文書の内容を参照する
- 簡潔で一貫性のある記述を心がける
- 文法ファイルは言語の構文を形式的に定義することに集中し、詳細な説明は他の文書に委ねる
- 特殊な構文要素の詳細な説明は、対応する言語仕様の章への参照を提供する

## アクティブな決定事項
- 言語名：Protorun
- 主要パラダイム：関数型プログラミングとメモリ安全性の統合
- 型システム：強力な静的型付けと型推論
- 効果システム：代数的効果による副作用の制御
- 所有権モデル：Rustに似た所有権と借用の概念
- 継承システム：トレイトの単一継承のみをサポート、データ型の継承を制限
- nullの排除：値の存在/不在はOption型で表現
- エラー処理：Result型ベースのエラー処理を採用、?演算子によるエラー伝搬をサポート
- 暗黙的パラメータ：関数シグネチャで`(with param: Type)`構文を使用した暗黙的なパラメータ渡し
- 効果スコープ：効果の局所的な使用のためのwith式
- 関数合成：パイプライン演算子と関数合成演算子の導入
- 制御構造：if式とmatch式の両方を維持（それぞれ異なる用途に最適化）
- コレクション操作：Pythonスタイルのコレクションリテラル内包表記
- モナド連鎖：bind式
- 実装アプローチ：段階的実装（インタープリタから始め、徐々にコンパイラ機能を追加）
- ライフサイクル管理効果：リソース管理と効果システムの統合（`LifecycleEffect<R>`型）
- 効果ハンドラの実装：トレイト実装に似た`handler`構文を使用
- 継続の扱い：暗黙的な継続をデフォルトとし、必要に応じて明示的な制御も可能なハイブリッドアプローチ
- 型注釈パターン：「エンティティ: 型」パターンを言語全体で一貫して使用
- 効果注釈：関数の効果を`&`演算子で型と合成（`ReturnType & Effect`）
- パーサー実装：Nomパーサーコンビネータを使用した宣言的で効率的な構文解析
