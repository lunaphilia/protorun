# タスク定義: 構造体リテラルを式としてパース

## 1. 目的

-   式が期待される場所（例: `with` 式の `instance` 部分、`let` 束縛の右辺、関数引数など）で、構造体リテラル（`TypeName { field: value }`）を直接記述できるようにパーサーを修正する。
-   これにより、`with alias = TypeName { ... }` のような構文が正しくパースできるようになり、コードの表現力が向上する。

## 2. 完了条件

-   `src/protorun/ast/mod.rs` の `Expr` enum に、構造体インスタンス生成を表す新しいバリアント `StructLiteral { type_name_expr: Box<Expr>, fields: Vec<(String, Expr)>, span: Span }` が追加されている。（`type_name_expr` は `TypeName` や `module.TypeName` を表す `Expr::Identifier` または `Expr::MemberAccess` を保持する）
-   `src/protorun/parser/expressions.rs` に、構造体リテラルのフィールド部分 `{ field1: value1, ... }` をパースする補助関数 (`parse_struct_literal_fields` など) が実装されている。
-   `src/protorun/parser/expressions.rs` の `postfix` 関数が修正され、`primary` でパースされた `Expr::Identifier` または `Expr::MemberAccess` の後に `{` が続く場合に、補助関数を呼び出して `Expr::StructLiteral` を構築するように統合されている。
-   `src/protorun/parser/tests_expressions.rs` の `test_parse_with_expr` 内でコメントアウトされている複数束縛のテストケースが有効化され、パスする。
-   `src/protorun/parser/tests_expressions.rs` に構造体リテラル式に関する追加のテストケース (`test_parse_struct_literal` など) が追加され、すべてパスする（フィールドなし、1つ、複数、ネストした式、エラーケースなど）。
-   `docs/language-spec/06-expressions.md` に「構造体リテラル式」に関する説明が追記されている。
-   `docs/language-spec/12-grammar.md` に構造体リテラル式の文法規則が追記されている。
-   `docs/language-spec/11-examples.md` の既存サンプルコード内で類似構文が使われている箇所が正しく動作することを確認し、構造体リテラルの使い方を示す新しいサンプルコードが追加されている。

## 3. 計画

1.  **ドキュメント更新 (先行):**
    1.  `docs/language-spec/06-expressions.md` を更新し、構造体リテラル式の構文、意味、使用例を追加する。
    2.  `docs/language-spec/12-grammar.md` を更新し、式の文法定義に構造体リテラルを追加する。
    3.  `docs/language-spec/11-examples.md` を更新し、既存コードを確認、新規サンプルを追加する。
2.  **AST定義の追加:**
    1.  `src/protorun/ast/mod.rs` の `Expr` enum に `StructLiteral` バリアントを追加する。
3.  **パーサー関数の実装:**
    1.  `src/protorun/parser/expressions.rs` にフィールド部分 `{...}` をパースする補助関数 (`parse_struct_literal_fields`) を実装する。
4.  **既存パーサーへの統合:**
    1.  `src/protorun/parser/expressions.rs` の `postfix` 関数を修正し、`Identifier` または `MemberAccess` の後に `{` が続く場合に `StructLiteral` を構築するロジックを追加する。
5.  **テストの修正と追加:**
    1.  `src/protorun/parser/tests_expressions.rs` の `test_parse_with_expr` のコメントアウトを解除し、アサーションを `Expr::StructLiteral` に合わせて修正する。
    2.  同ファイルに `test_parse_struct_literal` を追加し、様々なケースをテストする。

## 4. 懸念事項

-   **構文の曖昧さ:** `identifier { ... }` が構造体リテラルなのか、単なるブロック式なのか。`postfix` でのチェックにより、`Identifier`/`MemberAccess` の直後に `{` が続く場合のみ構造体リテラルと解釈することで解決を図る。
-   **エラーメッセージ:** フィールド定義の構文エラー（`:` がない等）に対する分かりやすいエラーメッセージの実装が必要。
