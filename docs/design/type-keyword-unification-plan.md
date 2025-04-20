# 変更計画: 型定義キーワードの `type` への統一

**1. 方針:**

*   型定義キーワードを `type` に統一し、`enum` キーワードを廃止する。
*   `let Name = type { ... }` 構文を使用し、`{...}` 内の最初の要素の形式（識別子の後のトークン）でレコード型定義かヴァリアント型定義かを区別する。
*   命名規則（ヴァリアント名/フィールド名）は言語仕様として強制しない。
*   エラーメッセージ、拡張性、ツールサポート、学習コストに関する懸念は認識するが、現時点での推測よりも、仕様定義後の実装や運用を通じて評価・改善する。

**2. 詳細な変更内容 (言語仕様ドキュメント):**

*   **`docs/language-spec/02-lexical-structure.md`:**
    *   キーワードリストから `enum` を削除する。
    *   `type` キーワードの説明を「レコード型またはヴァリアント型（代数的データ型）を定義する」のように更新する。
*   **`docs/language-spec/04-declarations.md`:**
    *   セクション4.3のタイトルを「4.3 型定義 (type)」に変更する。
    *   レコード型定義の説明は維持する。
    *   「4.3.2 代数的データ型定義 (enum)」を「4.3.2 ヴァリアント型定義 (type)」に変更する。
    *   `enum` キーワードの代わりに `type` キーワードを使用し、`{...}` 内にヴァリアントをカンマ区切りで記述する新しい構文を説明する。構文例 (`Option`, `Result`, `Shape` など) を新しい構文に修正する。
*   **`docs/language-spec/06-expressions.md`:**
    *   セクション6.9.1のタイトルを「6.9.1 型定義式 (type)」に変更する。
    *   `EnumDefinitionExpr` の説明を削除する。
    *   `TypeDefinitionExpr` の説明を更新し、レコード型とヴァリアント型の両方を定義できること、および構文の区別方法（識別子の後のトークン）を説明する。構文例 (`Point`, `Option` など) を新しい構文に修正する。
    *   `match` 式の説明や例で `enum` に言及している箇所を「ヴァリアント型」などに修正する。コンストラクタパターンの例が新しい定義構文と整合するように確認・修正する。
*   **`docs/language-spec/08-algebraic-effects.md`:**
    *   `FileMode` の定義例を新しい `type` 構文に修正する。
    *   `Result`, `Option`, `DbError` の使用例は、型の使用方法自体は変わらないため、基本的には変更不要だが、定義箇所への参照があれば確認する。
*   **`docs/language-spec/10-standard-library.md`:**
    *   `List`, `Option`, `Result`, `IOError`, `NetworkError` の定義例を新しい `type` 構文に修正する。
    *   これらの型の使用例（パターンマッチなど）は、使用方法自体は変わらないため、基本的には変更不要だが、定義箇所への参照があれば確認する。
*   **`docs/language-spec/11-examples.md`:**
    *   `Expr`, `FileMode`, `DbError` の定義例を新しい `type` 構文に修正する。
    *   これらの型の使用例（パターンマッチ、インスタンス生成）は、使用方法自体は変わらないため、基本的には変更不要だが、定義箇所への参照があれば確認する。
*   **`docs/language-spec/12-grammar.md`:**
    *   `EnumDefinitionExpr` ルールを削除する。
    *   `TypeDefinitionExpr` ルールを修正し、レコード形式とヴァリアント形式の両方を受け入れられるようにする（例: `TypeDefinitionExpr ::= "type" GenericParams? "{" (FieldDefinitionList | VariantDefinitionList)? "}"` のような形。具体的な実装は将来のパーサーに依存）。
    *   `EnumVariant` ルールを `VariantDefinition` に改名または統合し、`TypeDefinitionExpr` の一部として参照されるようにする。
    *   `ConstructorPattern` ルールは維持する（これが `type` で定義されたヴァリアント型とマッチすることを意味するように、説明文などで補足する）。
*   **その他:**
    *   `docs/language-spec/03-type-system.md` のADTに関する説明箇所で、定義方法が `type` キーワードに統一されたことを反映させる。
    *   他のドキュメントファイル (`.md`) 内で `enum` キーワードや古い定義構文に言及している箇所があれば修正する。

**3. 実施順序 (推奨):**

1.  文法定義 (`12-grammar.md`) を修正する。
2.  字句構造 (`02-lexical-structure.md`) を修正する。
3.  型定義の主要な説明箇所 (`04-declarations.md`, `06-expressions.md`) を修正する。
4.  標準ライブラリと例 (`10-standard-library.md`, `11-examples.md`) の定義例を修正する。
5.  その他の関連ファイル (`03-type-system.md`, `08-algebraic-effects.md` など) の記述を修正する。
6.  全体を通して一貫性を確認する。

**4. リスクと懸念:**

*   ドキュメント間の矛盾。
*   将来の実装可能性（特にパーサーロジック）。
*   可読性。