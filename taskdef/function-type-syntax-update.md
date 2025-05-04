# タスク定義: FunctionType 構文変更と仕様書整合性確保

## 1. 目的

-   Protorun 言語仕様における `FunctionType` および `FunctionExpr` の構文から `fn` キーワードを削除し、言語の簡潔性を向上させる。
-   `TupleType` の意味論を明確化し、`(Type)` が `Type` と同一として扱われることを仕様書に明記する。
-   上記変更に伴い、関連するすべての仕様書 (`docs/language-spec/*.md`) の記述とコード例を更新し、整合性を確保する。

## 2. 作業項目

1.  **`docs/language-spec/12-grammar.md` の修正:**
    *   EBNF 定義: `FunctionType` から `fn` を削除。
    *   EBNF 定義: `FunctionHeader` から `fn` を削除。
    *   説明 (`12.3.3`): `FunctionType` の説明と例を更新 (`fn` 削除)。
    *   説明 (`12.3.3`): `TupleType` の説明に `(Type)` の意味論に関する注釈を追加。
    *   説明 (`12.3.5`): `FunctionExpr` の説明と例を更新 (`fn` 削除)。
2.  **`docs/language-spec/03-type-system.md` の修正:**
    *   説明 (`3.3`): 関数型の説明と例を更新 (`fn` 削除)。
    *   説明 (`3.3`): タプル型の説明に `(Type)` の意味論に関する注釈を追加。
3.  **`docs/language-spec/04-declarations.md` の修正:**
    *   ファイル全体: コード例と型表記中の `fn` キーワードをすべて削除。
4.  **`docs/language-spec/06-expressions.md` の修正:**
    *   EBNF/説明/例 (`6.3.3`): `FunctionExpr` から `fn` を削除。
    *   説明 (`6.7`): タプル型の説明を `(Type)` の意味論に合わせて明確化。
    *   コード例 (`6.9`): 定義式内の `fn` を削除。
5.  **`docs/language-spec/08-algebraic-effects.md` の修正:**
    *   ファイル全体: コード例と型表記中の `fn` キーワードをすべて削除。
6.  **`docs/language-spec/11-examples.md` の修正:**
    *   ファイル全体: コード例と型表記中の `fn` キーワードをすべて削除。
7.  **その他の仕様書ファイルの確認と修正:**
    *   `docs/language-spec/` 内の他の `.md` ファイルを確認し、必要に応じて `fn` キーワードを削除。

## 3. 完了条件

-   `docs/language-spec/12-grammar.md` の `FunctionType` および `FunctionHeader` の EBNF 定義から `fn` が削除されている。
-   `docs/language-spec/` 配下のすべての `.md` ファイルにおいて、関数型を示す構文およびコード例から `fn` キーワードが削除されている。
-   `docs/language-spec/12-grammar.md` および `docs/language-spec/03-type-system.md` において、`(Type)` が `Type` と同一として扱われる旨が明記されている。
-   仕様書全体で、`FunctionType` および `TupleType` に関する記述に矛盾がない。

## 4. 作業上の懸念事項

-   修正漏れの可能性: 仕様書ファイルが複数にわたるため、`fn` キーワードの削除漏れがないか慎重に確認する必要がある。特にコード例中の型表記。
-   意味論的な影響の再確認: `fn` キーワード削除による構文上の曖昧さが本当にないか、修正後に再度確認することが望ましい。
