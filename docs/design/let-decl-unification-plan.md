# LetDecl / LetMutDecl 統合計画

## 1. 目的

`LetDecl` と `LetMutDecl` を単一の文法定義 `LetDecl` に統合し、文法を簡潔にする。

## 2. 統合後の構文

```ebnf
LetDecl ::= ("export")? "let" ("mut")? LetPattern (":" Type)? "=" Expression
```

## 3. `LetPattern` の定義 (Letプレフィックス統一)

`let` 宣言の左辺で許可されるパターン（Identifier, Tuple, Record）を限定する新しい非終端記号を導入する。

```ebnf
LetPattern ::= LetIdentifierPattern
             | LetTuplePattern
             | LetRecordPattern

LetIdentifierPattern ::= Identifier

LetTuplePattern ::= "(" (LetPattern ("," LetPattern)*)? ")"

LetRecordPattern ::= TypeRef "{" (LetRecordFieldPattern ("," LetRecordFieldPattern)*)? ("," "..")? "}"

LetRecordFieldPattern ::= Identifier (":" LetPattern)?
```

## 4. 既存の `Pattern` 定義は維持

`match` 式などで使用される汎用の `Pattern`, `IdentifierPattern`, `TuplePattern`, `RecordPattern`, `RecordFieldPattern` などはそのまま維持する。

```ebnf
Pattern ::= LiteralPattern
          | IdentifierPattern
          | TuplePattern
          | ConstructorPattern
          | RecordPattern
          | WildcardPattern

IdentifierPattern ::= ("ref")? ("mut")? Identifier
TuplePattern ::= "(" (Pattern ("," Pattern)*)? ")"
RecordPattern ::= TypeRef "{" (RecordFieldPattern ("," RecordFieldPattern)*)? ("," "..")? "}"
RecordFieldPattern ::= Identifier (":" Pattern)?
(* 他のパターン定義も同様 *)
```

## 5. `let mut` の制約

`mut` キーワードが存在する場合、`LetPattern` が `LetIdentifierPattern` でなければならないという制約は、**意味論解析（セマンティックチェック）**の段階で検証する。

## 6. 変更対象ファイル

-   `docs/language-spec/12-grammar.md`: EBNF定義の更新、関連説明の修正。
-   `docs/language-spec/04-declarations.md`: 宣言の概要、束縛宣言の詳細説明、EBNF定義の更新、`let mut` 制約の説明更新。
-   `docs/language-spec/02-lexical-structure.md`: キーワード表の参照更新。
-   `docs/language-spec/09-modules.md`: `export` やモジュールインターフェースの説明更新。

## 7. 実装への影響（想定）

-   **パーサー:** 新しい `LetPattern` の定義に従って解析するように変更。`let` の後にオプションで `mut` を認識。
-   **意味解析器:** `mut` が指定された `LetDecl` ノードに対して、その `LetPattern` が `LetIdentifierPattern` であることをチェックする機能を追加。

## 8. 計画の視覚化 (Mermaid)

```mermaid
graph TD
    subgraph Before
        Declaration_Before["Declaration ::= LetDecl | LetMutDecl | ImplDecl"]
        LetDecl_Before["LetDecl ::= ('export')? 'let' Pattern (':' Type)? '=' Expression"]
        LetMutDecl_Before["LetMutDecl ::= ('export')? 'let' 'mut' Identifier (':' Type)? '=' Expression"]
        Pattern_Before["Pattern ::= ... | IdentifierPattern | TuplePattern | RecordPattern | ..."]
        IdentifierPattern_Before["IdentifierPattern ::= ('ref')? ('mut')? Identifier"]
        TuplePattern_Before["TuplePattern ::= '(' (Pattern (',' Pattern)*)? ')'"]
        RecordPattern_Before["RecordPattern ::= TypeRef '{' ... '}'"]
    end

    subgraph After
        Declaration_After["Declaration ::= LetDecl | ImplDecl"]
        LetDecl_After["LetDecl ::= ('export')? 'let' ('mut')? LetPattern (':' Type)? '=' Expression"]
        LetPattern["LetPattern ::= LetIdentifierPattern | LetTuplePattern | LetRecordPattern"]
        LetIdentifierPattern["LetIdentifierPattern ::= Identifier"]
        LetTuplePattern["LetTuplePattern ::= '(' (LetPattern (',' LetPattern)*)? ')'"]
        LetRecordPattern["LetRecordPattern ::= TypeRef '{' (LetRecordFieldPattern (',' LetRecordFieldPattern)*)? (',' '..')? '}'"]
        LetRecordFieldPattern["LetRecordFieldPattern ::= Identifier (':' LetPattern)?"]
        SemanticCheck["Semantic Check:\n- if 'mut', LetPattern must be LetIdentifierPattern"]
        Pattern_After["Pattern ::= ... | IdentifierPattern | TuplePattern | RecordPattern | ...\n(Used in match, etc.)"]
        IdentifierPattern_After["IdentifierPattern ::= ('ref')? ('mut')? Identifier\n(Used in match, etc.)"]
        TuplePattern_After["TuplePattern ::= '(' (Pattern (',' Pattern)*)? ')'\n(Used in match, etc.)"]
        RecordPattern_After["RecordPattern ::= TypeRef '{' ... '}'\n(Used in match, etc.)"]

        LetDecl_After --> LetPattern
        LetPattern --> LetIdentifierPattern
        LetPattern --> LetTuplePattern
        LetPattern --> LetRecordPattern
        LetTuplePattern -- contains --> LetPattern
        LetRecordPattern -- contains --> LetRecordFieldPattern
        LetRecordFieldPattern -- contains --> LetPattern
        LetDecl_After --> SemanticCheck
    end

    Before --> After;