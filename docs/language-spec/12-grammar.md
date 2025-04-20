# 12. 文法（EBNF）

## 12.1 文法の目的と概要

Protorun言語の文法は、言語の構文を形式的に定義するためのものです。この文法定義は以下の目的を持っています：

1.  **言語の形式的定義**: 言語の構文を明確かつ曖昧さなく定義します。
2.  **パーサー実装の基礎**: 言語のパーサーを実装するための基礎を提供します。
3.  **言語の一貫性確保**: 言語の構文が一貫していることを保証します。
4.  **開発者のガイド**: 言語を使用する開発者に正確な構文情報を提供します。
5.  **言語の進化**: 言語の拡張や変更を形式的に追跡するための基盤を提供します。

以下の文法はEBNF（拡張バッカス・ナウア記法）形式で記述されており、Protorun言語の構文要素を定義しています。

## 12.2 EBNF文法

```ebnf
Program ::= TopLevelItem*

TopLevelItem ::= Declaration | Expression

Declaration ::= LetDecl | ImplDecl

LetDecl ::= ("export")? "let" ("mut")? LetPattern (":" Type)? ("=" Expression)?
ImplDecl ::= ("export")? "impl" GenericParams? TypeRef ("for" TypeRef)? WhereClause? "{" ImplItem* "}"

Expression ::= LiteralExpr
             | IdentifierExpr
             | BlockExpr
             | IfExpr
             | MatchExpr
             | CollectionComprehensionExpr
             | BindExpr
             | FunctionExpr
             | CallExpr
             | MemberAccessExpr
             | BinaryExpr
             | UnaryExpr
             | EffectOperationCallExpr
             | WithExpr
             | RangeExpr
             | AssignmentExpr
             | TupleExpr
             | GroupedExpr
             | PartialApplicationExpr
             | RecordExpr
             | TypeDefinitionExpr
             | TraitDefinitionExpr
             | EffectDefinitionExpr
             | HandlerDefinitionExpr
             | AliasDefinitionExpr

TypeDefinitionExpr ::= "type" GenericParams? "{" (FieldDefinitionList | VariantDefinitionList)? "}"

FieldDefinitionList ::= FieldDefinition ("," FieldDefinition)*
FieldDefinition ::= Identifier ":" Type

VariantDefinitionList ::= VariantDefinition ("," VariantDefinition)*
VariantDefinition ::= Identifier ("(" TypeList? ")")?
                    | Identifier "{" (FieldDefinition ("," FieldDefinition)*)? "}"

TraitDefinitionExpr ::= "trait" GenericParams? (":" TypeRef)? "{" TraitItem* "}"
TraitItem ::= LetDecl

EffectDefinitionExpr ::= "effect" GenericParams? "{" EffectItem* "}"
EffectItem ::= LetDecl

HandlerDefinitionExpr ::= "handler" GenericParams? TypeRef "for" TypeRef WhereClause? "{" HandlerItem* "}"
HandlerItem ::= LetDecl

AliasDefinitionExpr ::= "alias" GenericParams? Type

ImplItem ::= LetDecl

ParamList ::= "(" (Param ("," Param)*)? ")"
ImplicitParamList ::= "(" "with" Param ("," Param)* ")"
EffectParamList ::= "(" (EffectParam ("," EffectParam)*)? ")"

Param ::= Identifier ":" Type
EffectParam ::= "effect" Identifier ":" TypeRef

GenericParams ::= "<" (GenericParam ("," GenericParam)*)? ">"
GenericParam ::= Identifier (":" TypeConstraint)?

TypeConstraint ::= TypeRef (("&" | "|") TypeRef)*

Type ::= TypeRef
       | FunctionType
       | TupleType
       | ArrayType

TypeRef ::= Identifier GenericArgs?
          | "own" TypeRef
          | "&" TypeRef
          | "&mut" TypeRef

GenericArgs ::= "<" (Type ("," Type)*)? ">"

FunctionType ::= "fn" GenericParams? ParamListType? EffectParamListType? ImplicitParamListType? "->" ReturnType

ParamListType ::= "(" (Type ("," Type)*)? ")"
EffectParamListType ::= "(" (EffectParamType ("," EffectParamType)*)? ")"
ImplicitParamListType ::= "(" "with" Type ("," Type)* ")"
EffectParamType ::= "effect" TypeRef

TupleType ::= "(" TypeList? ")"

TypeList ::= Type ("," Type)*

ArrayType ::= "[" Type "]"

ReturnType ::= Type | "Unit"

Statement ::= ReturnStatement

ReturnStatement ::= "return" Expression?

LiteralExpr ::= IntLiteral | FloatLiteral | StringLiteral | BoolLiteral | UnitLiteral
              | ListLiteral | MapLiteral | SetLiteral

ListLiteral ::= "[" (Expression ("," Expression)*)? "]"
MapLiteral ::= "{" (Expression "->" Expression ("," Expression "->" Expression)*)? "}"
SetLiteral ::= "#{" (Expression ("," Expression)*)? "}"

TupleExpr ::= "(" Expression "," TypeList ")"
            | "(" ")"

GroupedExpr ::= "(" Expression ")"

IdentifierExpr ::= Identifier

BlockExpr ::= "{" BlockItem* "}"
BlockItem ::= Declaration | Statement | Expression

IfExpr ::= "if" Expression BlockExpr ("else" (IfExpr | BlockExpr))?

MatchExpr ::= "match" Expression "{" (MatchArm ("," MatchArm)*)? "}"
MatchArm ::= Pattern ("if" Expression)? "=>" Expression

CollectionComprehensionExpr ::= ListComprehension | MapComprehension | SetComprehension
ListComprehension ::= "[" Expression "for" Pattern "<-" Expression ("if" Expression)? "]"
MapComprehension ::= "{" Expression "->" Expression "for" Pattern "<-" Expression ("if" Expression)? "}"
SetComprehension ::= "#{" Expression "for" Pattern "<-" Expression ("if" Expression)? "}"

BindExpr ::= "bind" "{" (Pattern "<-" Expression)* Expression "}"

FunctionHeader ::= "fn" GenericParams? ParamList? EffectParamList? ImplicitParamList? ("->" ReturnType)?

FunctionExpr ::= FunctionHeader "=>" Expression

CallExpr ::= Expression "(" ArgList? ")"
ArgList ::= Expression ("," Expression)*

MemberAccessExpr ::= Expression "." Identifier

EffectOperationCallExpr ::= Expression "." Identifier "(" ArgList? ")"

BinaryExpr ::= Expression Operator Expression

UnaryExpr ::= Operator Expression

WithExpr ::= "with" WithBinding ("," WithBinding)* BlockExpr
WithBinding ::= Identifier "=" Expression (":" TypeRef)?

RangeExpr ::= Expression ".." Expression

AssignmentExpr ::= LValue "=" Expression
LValue ::= IdentifierExpr | MemberAccessExpr

RecordExpr ::= TypeRef "{" (RecordFieldInit ("," RecordFieldInit)*)? "}"
RecordFieldInit ::= Identifier ":" Expression

Pattern ::= LiteralPattern
          | IdentifierPattern
          | TuplePattern
          | ConstructorPattern
          | RecordPattern
          | WildcardPattern

LetPattern ::= LetIdentifierPattern
             | LetTuplePattern
             | LetRecordPattern

LetIdentifierPattern ::= Identifier

LetTuplePattern ::= "(" (LetPattern ("," LetPattern)*)? ")"

LetRecordPattern ::= TypeRef "{" (LetRecordFieldPattern ("," LetRecordFieldPattern)*)? ("," "..")? "}"

LetRecordFieldPattern ::= Identifier (":" LetPattern)?


LiteralPattern ::= LiteralExpr

IdentifierPattern ::= ("ref")? ("mut")? Identifier

TuplePattern ::= "(" (Pattern ("," Pattern)*)? ")"

ConstructorPattern ::= QualifiedIdentifier ("(" (Pattern ("," Pattern)*)? ")")?

RecordPattern ::= TypeRef "{" (RecordFieldPattern ("," RecordFieldPattern)*)? ("," "..")? "}"
RecordFieldPattern ::= Identifier (":" Pattern)?

WildcardPattern ::= "_"

QualifiedIdentifier ::= (Identifier ".")* Identifier

Operator ::= "+" | "-" | "*" | "/" | "%" | "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&" | "||" | "!" | "|>" | "|>*" | ">>>" | ">>>*"

WhereClause ::= "where" WherePredicate ("," WherePredicate)*
WherePredicate ::= TypeRef ":" TypeConstraint
```

## 12.3 文法の説明

### 12.3.1 プログラム構造

Protorun言語のプログラムは、トップレベルに配置できる宣言（Declaration）と式（Expression）のシーケンスで構成されます。

### 12.3.2 宣言 (Declaration)

Protorunの宣言は、主に `let` キーワードを用いた束縛宣言と、`impl` キーワードを用いたトレイト実装宣言があります。

- **`LetDecl` (束縛宣言)**: `let` キーワードで始まり、オプションで `mut` キーワード、束縛パターン (`LetPattern`)、オプションの型注釈、そして式 (`Expression`) が続きます。
    - `mut` キーワードがない場合は**不変束縛**となり、`mut` キーワードがある場合は**可変束縛**となります。
    - 左辺の `LetPattern` には、識別子 (`LetIdentifierPattern`)、タプル (`LetTuplePattern`)、レコード (`LetRecordPattern`) のいずれかの形式のパターンを指定できます。これにより、値の分解束縛が可能です。
    - `let mut` を使用する場合、左辺の `LetPattern` は単一の識別子 (`LetIdentifierPattern`) でなければなりません。この制約は文法レベルではなく、**意味論解析（セマンティックチェック）**で検証されます。
    - 右辺の式 (`Expression`) には、通常の計算式だけでなく、関数式や型定義式なども含まれます。
- **`ImplDecl` (トレイト実装)**: `impl` キーワードで始まり、特定の型に対するトレイトの実装を定義します。

**`export` 修飾子**: `let` (不変・可変問わず) および `impl` 宣言の前に `export` キーワードを付与することで、その宣言をモジュール外に公開できます。（詳細は [9. モジュール](09-modules.md) を参照）

### 12.3.3 型システム (Type System)

- **型参照 (`TypeRef`)**: 型名とジェネリック引数で構成されます。所有権修飾子（`own`, `&`, `&mut`）を含むことができます（所有権システムの詳細は [7. 所有権](07-ownership.md) を参照）。
- **関数型 (`FunctionType`)**: `fn(ParamList) -> ReturnType` の形式で、パラメータ型、戻り値の型、およびオプションの効果指定 (`EffectSpecifier`) で構成されます。
- **タプル型 (`TupleType`)**: `(Type1, Type2, ...)` の形式で、複数の型を組み合わせた型です。要素を持たない `()` はユニット型を表します。
- **ジェネリクス (`GenericParams`, `GenericArgs`)**: 型、関数、トレイトなどに型パラメータを導入し、多相的なコードを可能にします。
- **トレイト制約 (`TypeConstraint`, `WhereClause`)**: ジェネリックパラメータが満たすべきトレイトを指定します。

### 12.3.4 文 (Statement)

文は主に制御フローを変更するために使用されます。

- **`ReturnStatement`**: `return Expression?` の形式で、現在の関数から値を返します。

ブロック式 (`BlockExpr`) 内では、宣言 (`Declaration`)、文 (`Statement`)、式 (`Expression`) を記述できます。最後の式がブロックの値となります。副作用のためだけに式を実行する場合も `BlockItem` として扱われます。

### 12.3.5 式 (Expression)

式は評価されて値を生成します。Protorunでは、型定義なども式の一種として扱われます。

- **リテラル (`LiteralExpr`)**: 数値、文字列、真偽値、ユニット `()` など。コレクションリテラル（`ListLiteral`, `MapLiteral`, `SetLiteral`）も含まれますが、標準ライブラリの型（例: `List<T>`）で代替される可能性があります。
- **識別子 (`IdentifierExpr`)**: 変数や関数名など。
- **ブロック (`BlockExpr`)**: `{ BlockItem* }` 形式。
- **条件 (`IfExpr`)**: `if cond { ... } else { ... }` 形式。
- **パターンマッチ (`MatchExpr`)**: `match value { Pattern => Expr, ... }` 形式。
- **関数 (`FunctionExpr`)**: `fn <GenericParams>? (Params) => Expr` 形式の無名関数。
- **呼び出し (`CallExpr`)**: `func(Args)` 形式。
- **メンバーアクセス (`MemberAccessExpr`)**: `expr.identifier` 形式。
- **レコード構築 (`RecordExpr`)**: `TypeName { field: value, ... }` 形式。
- **二項/単項演算 (`BinaryExpr`, `UnaryExpr`)**: 演算子を用いた式。
- **代入 (`AssignmentExpr`)**: `lvalue = expr` 形式。`lvalue` は識別子やメンバーアクセスなど。
- **タプル (`TupleExpr`)**: `(expr1, expr2, ...)` 形式（要素2つ以上）。
- **グループ化 (`GroupedExpr`)**: `(expr)` 形式。評価順序の制御。
- **定義式**:
    - **`TypeDefinitionExpr`**: `type <GenericParams>? { ... }`
    - **`TraitDefinitionExpr`**: `trait <GenericParams>? (: SuperTrait)? { ... }`
    - **`EffectDefinitionExpr`**: `effect <GenericParams>? { ... }`
    - **`HandlerDefinitionExpr`**: `handler <GenericParams>? Effect for Type { ... }`
    - **`AliasDefinitionExpr`**: `alias <GenericParams>? Type`
- **その他**: `WithExpr`（効果ハンドリング）、`RangeExpr` などが含まれますが、仕様変更の可能性があります。`CollectionComprehensionExpr`, `BindExpr`, `PartialApplicationExpr` も同様に見直される可能性があります。

### 12.3.6 パターン (Pattern)

パターンは主に `match` 式で使用され、値の構造と照合します。`let` 束縛で使用できるパターンは `LetPattern` として別途定義されており、より限定されています。

**汎用パターン (`Pattern`)**: `match` 式などで使用されます。

- **リテラル (`LiteralPattern`)**: リテラル値とのマッチング。
- **識別子 (`IdentifierPattern`)**: 新しい変数を束縛します。`match` 式内では `ref` や `mut` 修飾子を伴うことがあります（所有権関連）。
- **タプル (`TuplePattern`)**: `(Pattern1, Pattern2, ...)` 形式。
- **コンストラクタ (`ConstructorPattern`)**: `VariantName(Pattern1, ...)` または `VariantName { field: Pattern, ... }` 形式で、`type` で定義されたヴァリアント型のヴァリアントとマッチングします。
- **レコード (`RecordPattern`)**: `TypeName { field: Pattern, ... }` 形式で、`type` で定義されたレコード型とマッチングします。`..` で残りのフィールドを無視できます。
- **ワイルドカード (`WildcardPattern`)**: `_` で任意の値とマッチングし、束縛しません。

**let束縛用パターン (`LetPattern`)**: `let` 宣言の左辺で使用されます。

- **識別子 (`LetIdentifierPattern`)**: 単一の識別子による束縛。
- **タプル (`LetTuplePattern`)**: タプル構造の分解束縛。
- **レコード (`LetRecordPattern`)**: レコード構造の分解束縛。

`LetPattern` は、マッチに失敗する可能性のある `LiteralPattern` や `ConstructorPattern`、および `let` 文脈では冗長な `WildcardPattern` を除外しています。

## 12.4 特殊な構文要素

以下の特殊な構文要素の詳細については、対応する言語仕様の章を参照してください：

- **代数的効果とハンドラ (`effect`, `handler`, `WithExpr`)**: [8. 代数的効果](08-algebraic-effects.md)
- **所有権と借用 (`own`, `&`, `&mut`, `ref`)**: [7. 所有権](07-ownership.md)
- **モジュールと可視性 (`export`)**: [9. モジュール](09-modules.md)
