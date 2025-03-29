# 10. 文法（EBNF）

## 10.1 文法の目的と概要

Protorun言語の文法は、言語の構文を形式的に定義するためのものです。この文法定義は以下の目的を持っています：

1. **言語の形式的定義**: 言語の構文を明確かつ曖昧さなく定義します。
2. **パーサー実装の基礎**: 言語のパーサーを実装するための基礎を提供します。
3. **言語の一貫性確保**: 言語の構文が一貫していることを保証します。
4. **開発者のガイド**: 言語を使用する開発者に正確な構文情報を提供します。
5. **言語の進化**: 言語の拡張や変更を形式的に追跡するための基盤を提供します。

以下の文法はEBNF（拡張バッカス・ナウア記法）形式で記述されており、Protorun言語の構文要素を定義しています。

## 10.2 EBNF文法

```ebnf
Program ::= (Declaration | Statement)*

Declaration ::= FunctionDecl | TypeDecl | TraitDecl | ImplDecl | EffectDecl | HandlerDecl | ExportDecl | EnumDecl

FunctionDecl ::= "fn" Identifier GenericParams? ParamList (":" Type)? ("&" EffectType)? "=" Expression
               | "fn" Identifier GenericParams? ParamList ImplicitParamList? (":" Type)? ("&" EffectType)? "=" Expression

TypeDecl ::= "type" Identifier GenericParams? "=" (RecordType | Type)

TraitDecl ::= "trait" Identifier GenericParams? ("{" TraitMember* "}")? (":" TypeRef)?

ImplDecl ::= "impl" GenericParams? TypeRef ":" TypeRef "{" ImplMember* "}"

EffectDecl ::= "effect" Identifier GenericParams? (":" TypeRef)? "{" EffectOperation* "}"

HandlerDecl ::= "handler" Identifier GenericParams? ":" TypeRef "{" HandlerMember* "}"

ExportDecl ::= "export" (FunctionDecl | TypeDecl | TraitDecl | EffectDecl | HandlerDecl | ExportList)
ExportList ::= "{" (Identifier ("," Identifier)*)? "}"

EnumDecl ::= "enum" Identifier GenericParams? "{" (EnumVariant ("," EnumVariant)*)? "}"
EnumVariant ::= Identifier ("(" Type ("," Type)* ")")?

RecordType ::= "{" (Identifier ":" Type ("," Identifier ":" Type)*)? "}"

TraitMember ::= FunctionDecl

ImplMember ::= FunctionDecl

HandlerMember ::= HandlerFunction | FieldDecl

HandlerFunction ::= Identifier GenericParams? ParamList (":" ReturnType)? "=" Expression
                  | Identifier GenericParams? ParamList "," "resume" ":" ResumeType (":" ReturnType)? "=" Expression
                  | Identifier GenericParams? ParamList ":" "noresume" ReturnType "=" Expression
                  | Identifier GenericParams? ParamList ":" "multiresume" ReturnType "=" Expression

FieldDecl ::= Identifier ":" Type

EffectOperation ::= "fn" Identifier GenericParams? ParamList (":" Type)? ";"
                  | "fn" "acquire" GenericParams? ParamList (":" Type)? ";"
                  | "fn" "release" "(" "resource" ":" TypeRef ")" ":" Type? ";"

ParamList ::= "(" (Param ("," Param)*)? ")"

ImplicitParamList ::= "(" "with" Param ("," Param)* ")"

Param ::= Identifier ":" Type

GenericParams ::= "<" (GenericParam ("," GenericParam)*)? ">"

GenericParam ::= Identifier (":" TypeConstraint)?

TypeConstraint ::= TypeRef (("&" | "|") TypeRef)*

Type ::= TypeRef
       | FunctionType
       | ArrayType

TypeRef ::= Identifier GenericArgs?
          | "own" TypeRef
          | "&" TypeRef
          | "&mut" TypeRef

GenericArgs ::= "<" (Type ("," Type)*)? ">"

FunctionType ::= "(" (Type ("," Type)*)? ")" "->" Type ("&" EffectType)?

ArrayType ::= "[" Type "]"

EffectType ::= TypeRef ("&" TypeRef)*

ResumeType ::= "(" (Type ("," Type)*)? ")" "->" ReturnType

ReturnType ::= Type | "Unit"

Statement ::= Expression
            | "let" Pattern (":" Type)? "=" Expression
            | "var" Identifier (":" Type)? "=" Expression
            | "return" Expression?

Expression ::= LiteralExpr
             | IdentifierExpr
             | BlockExpr
             | IfExpr
             | MatchExpr
             | CollectionComprehensionExpr
             | BindExpr
             | LambdaExpr
             | CallExpr
             | MemberAccessExpr
             | BinaryExpr
             | UnaryExpr
             | HandleExpr
             | WithExpr
             | ScopedEffectExpr
             | RangeExpr

LiteralExpr ::= IntLiteral | FloatLiteral | StringLiteral | BoolLiteral | UnitLiteral
              | ListLiteral | MapLiteral | SetLiteral

ListLiteral ::= "[" (Expression ("," Expression)*)? "]"

MapLiteral ::= "{" (Expression "->" Expression ("," Expression "->" Expression)*)? "}"

SetLiteral ::= "#{" (Expression ("," Expression)*)? "}"

IdentifierExpr ::= Identifier

BlockExpr ::= "{" Statement* (Expression)? "}"

IfExpr ::= "if" Expression (BlockExpr | Expression) ("else" (IfExpr | BlockExpr | Expression))?

MatchExpr ::= "match" Expression "{" (Pattern ("if" Expression)? "=>" Expression ",")* "}"

CollectionComprehensionExpr ::= ListComprehension | MapComprehension | SetComprehension

ListComprehension ::= "[" Expression "for" Pattern "<-" Expression ("if" Expression)? "]"
                    | "[" Expression "for" Pattern "<-" Expression "for" Pattern "<-" Expression ("if" Expression)? "]"

MapComprehension ::= "{" Expression "->" Expression "for" Pattern "<-" Expression ("if" Expression)? "}"

SetComprehension ::= "#{" Expression "for" Pattern "<-" Expression ("if" Expression)? "}"

BindExpr ::= "bind" "{" (Pattern "<-" Expression)* Expression "}"

LambdaExpr ::= ParamList "=>" Expression

CallExpr ::= Expression "(" (Expression ("," Expression)*)? ")"

MemberAccessExpr ::= Expression "." Identifier

BinaryExpr ::= Expression Operator Expression

UnaryExpr ::= Operator Expression

HandleExpr ::= "handle" Expression "{" (EffectCase)* "}"

WithExpr ::= "with" (Expression | TypeRef) (":" TypeRef)? BlockExpr
           | "with" (Expression | TypeRef) ("," (Expression | TypeRef))* BlockExpr

ScopedEffectExpr ::= "with" "scoped" Identifier BlockExpr

RangeExpr ::= Expression ".." Expression

EffectCase ::= QualifiedIdentifier ParamList "=>" BlockExpr
             | QualifiedIdentifier ParamList "," "resume" ":" ResumeType "=>" BlockExpr
             | "return" Pattern "=>" BlockExpr

Pattern ::= LiteralPattern
          | IdentifierPattern
          | TuplePattern
          | ConstructorPattern
          | WildcardPattern

LiteralPattern ::= LiteralExpr

IdentifierPattern ::= Identifier

TuplePattern ::= "(" Pattern ("," Pattern)* ")"

ConstructorPattern ::= QualifiedIdentifier ("(" Pattern ("," Pattern)* ")")?

WildcardPattern ::= "_"

QualifiedIdentifier ::= (Identifier ".")* Identifier

Operator ::= "+" | "-" | "*" | "/" | "%" | "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&" | "||" | "!" | "|>" | "|>*" | ">>>" | ">>>*"
```

## 10.3 文法の説明

### 10.3.1 プログラム構造

Protorun言語のプログラムは、宣言（Declaration）と文（Statement）の集合で構成されます。宣言には関数、型、トレイト、実装、効果、ハンドラの定義が含まれます。

### 10.3.2 宣言

- **関数宣言（FunctionDecl）**: `fn`キーワードで始まり、関数名、ジェネリックパラメータ（オプション）、パラメータリスト、暗黙的パラメータリスト（オプション）、戻り値の型（オプション）、効果型（オプション）、関数本体（式）で構成されます。
- **型宣言（TypeDecl）**: レコード型、シールドトレイトの定義を含みます。
- **トレイト宣言（TraitDecl）**: インターフェースを定義します。
- **実装宣言（ImplDecl）**: トレイトの実装を定義します。
- **効果宣言（EffectDecl）**: 代数的効果を定義します。効果は他の効果（例：`LifecycleEffect<R>`）を継承することができます。
- **ハンドラ宣言（HandlerDecl）**: 効果ハンドラを定義します。

### 10.3.3 型システム

- **型参照（TypeRef）**: 型名とジェネリック引数で構成されます。所有権修飾子（`own`、`&`、`&mut`）を含むことができます。
- **関数型（FunctionType）**: パラメータ型、戻り値の型、効果型で構成されます。
- **配列型（ArrayType）**: 要素型の配列です。
- **効果型（EffectType）**: 関数が持つ可能性のある効果の型です。

### 10.3.4 式

- **リテラル式（LiteralExpr）**: 整数、浮動小数点数、文字列、真偽値、ユニットのリテラル、およびコレクションリテラル（リスト、マップ、セット）です。
- **識別子式（IdentifierExpr）**: 変数や関数の名前です。
- **ブロック式（BlockExpr）**: 文の集合と最終的な式で構成されます。
- **条件式（IfExpr）**: 条件に基づいて異なる式を評価します。
- **パターンマッチング式（MatchExpr）**: 値をパターンと照合して異なる式を評価します。
- **コレクション内包表記式（CollectionComprehensionExpr）**: コレクションを反復処理して新しいコレクションを生成します。リスト、マップ、セットの内包表記をサポートします。
- **バインド式（BindExpr）**: モナド連鎖のための式です。
- **ラムダ式（LambdaExpr）**: 無名関数です。
- **関数呼び出し式（CallExpr）**: 関数を呼び出します。
- **メンバーアクセス式（MemberAccessExpr）**: オブジェクトのメンバーにアクセスします。
- **二項演算式（BinaryExpr）**: 二つの式を演算子で結合します。
- **単項演算式（UnaryExpr）**: 一つの式に演算子を適用します。
- **ハンドル式（HandleExpr）**: 効果をハンドルします。
- **with式（WithExpr）**: 効果ハンドラを適用するスコープを定義します。複数のハンドラをカンマで区切って指定することもできます。
- **スコープ付き効果式（ScopedEffectExpr）**: 効果のスコープを定義します。
- **範囲式（RangeExpr）**: 範囲を表現します。

### 10.3.5 パターン

- **リテラルパターン（LiteralPattern）**: リテラル値とのマッチングです。
- **識別子パターン（IdentifierPattern）**: 変数束縛です。
- **タプルパターン（TuplePattern）**: タプルの分解です。
- **コンストラクタパターン（ConstructorPattern）**: 代数的データ型の分解です。
- **ワイルドカードパターン（WildcardPattern）**: 任意の値とマッチングします。

## 10.4 特殊な構文要素

以下の特殊な構文要素の詳細については、対応する言語仕様の章を参照してください：

- **効果ハンドラ**: [7. 代数的効果](07-algebraic-effects.md)
- **ライフサイクル管理効果**: [7.4 ライフサイクル管理効果](07-algebraic-effects.md#74-ライフサイクル管理効果) <!-- Note: Assuming section numbers in 07-algebraic-effects.md will also be updated later if needed -->
- **暗黙的パラメータ**: [7.8 暗黙的パラメータと効果システム](07-algebraic-effects.md#78-暗黙的パラメータと効果システム) <!-- Note: Assuming section numbers in 07-algebraic-effects.md will also be updated later if needed -->
- **コレクション内包表記**: [5.2.1 コレクションリテラル内包表記](05-expressions.md#521-コレクションリテラル内包表記)
- **バインド式**: [5.2.2 バインド式](05-expressions.md#522-バインド式)
