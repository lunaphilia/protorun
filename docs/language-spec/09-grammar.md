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

Declaration ::= FunctionDecl | TypeDecl | TraitDecl | ImplDecl | EffectDecl | HandlerDecl

FunctionDecl ::= "fn" Identifier GenericParams? ParamList (":" Type)? ("&" EffectType)? "=" Expression

TypeDecl ::= "type" Identifier GenericParams? "=" (RecordType | Type)
           | "sealed" "trait" Identifier GenericParams? ("{" TraitMember* "}")? (":" TypeRef)?
           | "managed" Identifier GenericParams? "{" ManagedTypeMember* "}"

TraitDecl ::= "trait" Identifier GenericParams? ("{" TraitMember* "}")? (":" TypeRef)?

ImplDecl ::= "impl" GenericParams? TypeRef ":" TypeRef "{" ImplMember* "}"

EffectDecl ::= "effect" Identifier GenericParams? (":" "lifecycle")? "{" EffectOperation* "}"

HandlerDecl ::= "handler" Identifier GenericParams? ":" TypeRef "{" HandlerMember* "}"

RecordType ::= "{" (Identifier ":" Type ("," Identifier ":" Type)*)? "}"

TraitMember ::= FunctionDecl

ImplMember ::= FunctionDecl

HandlerMember ::= HandlerFunction | FieldDecl | FinalizeFunction

HandlerFunction ::= Identifier GenericParams? ParamList (":" ReturnType)? "=" Expression
                  | Identifier GenericParams? ParamList "," "resume" ":" ResumeType ":" ReturnType "=" Expression
                  | Identifier GenericParams? ParamList ":" "noresume" ReturnType "=" Expression
                  | Identifier GenericParams? ParamList ":" "multiresume" ReturnType "=" Expression

FinalizeFunction ::= "fn" "finalize" "(" ")" ":" "Unit" "=" Expression

ManagedTypeMember ::= FieldDecl | FunctionDecl

FieldDecl ::= Identifier ":" Type

EffectOperation ::= "fn" Identifier GenericParams? ParamList (":" Type)? ("with" "cleanup")? ";"
                 | "fn" "cleanup" ParamList (":" Type)? ";"

ParamList ::= "(" (Param ("," Param)*)? ")"

Param ::= Identifier ":" Type

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

FunctionType ::= "(" (Type ("," Type)*)? ")" "->" Type ("&" EffectType)?

TupleType ::= "(" Type ("," Type)+ ")"

ArrayType ::= "[" Type "]"

EffectType ::= TypeRef ("&" TypeRef)*

ResumeType ::= "(" (Type ("," Type)*)? ")" "->" ReturnType

ReturnType ::= Type | "Unit"

Statement ::= Expression ";"
            | "let" Pattern (":" Type)? "=" Expression ";"
            | "var" Identifier (":" Type)? "=" Expression ";"
            | "return" Expression? ";"

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

LiteralExpr ::= IntLiteral | FloatLiteral | StringLiteral | BoolLiteral | UnitLiteral

IdentifierExpr ::= Identifier

BlockExpr ::= "{" Statement* (Expression)? "}"

IfExpr ::= "if" Expression BlockExpr ("else" (IfExpr | BlockExpr))?

MatchExpr ::= "match" Expression "{" (Pattern ("if" Expression)? "=>" Expression ",")* "}"

CollectionComprehensionExpr ::= "[" Expression "for" Pattern "<-" Expression ("if" Expression)? "]"

BindExpr ::= "bind" "{" (Pattern "<-" Expression ";")* Expression "}"

LambdaExpr ::= ParamList "=>" Expression

CallExpr ::= Expression "(" (Expression ("," Expression)*)? ")"

MemberAccessExpr ::= Expression "." Identifier

BinaryExpr ::= Expression Operator Expression

UnaryExpr ::= Operator Expression

HandleExpr ::= "handle" Expression "{" (EffectCase)* "}"

WithExpr ::= "with" (Expression | TypeRef) (":" TypeRef)? BlockExpr

ScopedEffectExpr ::= "with" "scoped" Identifier BlockExpr

EffectCase ::= QualifiedIdentifier ParamList "=>" BlockExpr

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
```

## 10.3 文法の説明

### 10.3.1 プログラム構造

Protorun言語のプログラムは、宣言（Declaration）と文（Statement）の集合で構成されます。宣言には関数、型、トレイト、実装、効果、ハンドラの定義が含まれます。

### 10.3.2 宣言

- **関数宣言（FunctionDecl）**: `fn`キーワードで始まり、関数名、ジェネリックパラメータ（オプション）、パラメータリスト、戻り値の型（オプション）、効果型（オプション）、関数本体（式）で構成されます。
- **型宣言（TypeDecl）**: レコード型、シールドトレイト、管理型の定義を含みます。
- **トレイト宣言（TraitDecl）**: インターフェースを定義します。
- **実装宣言（ImplDecl）**: トレイトの実装を定義します。
- **効果宣言（EffectDecl）**: 代数的効果を定義します。
- **ハンドラ宣言（HandlerDecl）**: 効果ハンドラを定義します。

### 10.3.3 型システム

- **型参照（TypeRef）**: 型名とジェネリック引数で構成されます。
- **関数型（FunctionType）**: パラメータ型、戻り値の型、効果型で構成されます。
- **タプル型（TupleType）**: 複数の型の組み合わせです。
- **配列型（ArrayType）**: 要素型の配列です。
- **効果型（EffectType）**: 関数が持つ可能性のある効果の型です。

### 10.3.4 式

- **リテラル式（LiteralExpr）**: 整数、浮動小数点数、文字列、真偽値、ユニットのリテラルです。
- **識別子式（IdentifierExpr）**: 変数や関数の名前です。
- **ブロック式（BlockExpr）**: 文の集合と最終的な式で構成されます。
- **条件式（IfExpr）**: 条件に基づいて異なる式を評価します。
- **パターンマッチング式（MatchExpr）**: 値をパターンと照合して異なる式を評価します。
- **コレクション内包表記式（CollectionComprehensionExpr）**: コレクションを反復処理して新しいコレクションを生成します。
- **バインド式（BindExpr）**: モナド連鎖のための式です。
- **ラムダ式（LambdaExpr）**: 無名関数です。
- **関数呼び出し式（CallExpr）**: 関数を呼び出します。
- **メンバーアクセス式（MemberAccessExpr）**: オブジェクトのメンバーにアクセスします。
- **二項演算式（BinaryExpr）**: 二つの式を演算子で結合します。
- **単項演算式（UnaryExpr）**: 一つの式に演算子を適用します。
- **ハンドル式（HandleExpr）**: 効果をハンドルします。
- **with式（WithExpr）**: 効果ハンドラを適用するスコープを定義します。
- **スコープ付き効果式（ScopedEffectExpr）**: 効果のスコープを定義します。

### 10.3.5 パターン

- **リテラルパターン（LiteralPattern）**: リテラル値とのマッチングです。
- **識別子パターン（IdentifierPattern）**: 変数束縛です。
- **タプルパターン（TuplePattern）**: タプルの分解です。
- **コンストラクタパターン（ConstructorPattern）**: 代数的データ型の分解です。
- **ワイルドカードパターン（WildcardPattern）**: 任意の値とマッチングします。

## 10.4 特殊な構文要素

### 10.4.1 効果ハンドラの構文

効果ハンドラは、`handler`キーワードを使用して定義されます。ハンドラ関数には以下の形式があります：

1. **暗黙的な継続**: 通常の関数と同様の構文で、継続は暗黙的に呼び出されます。
2. **明示的な継続**: `resume`パラメータを持ち、継続を明示的に呼び出します。
3. **継続を呼び出さないハンドラ**: `noresume`キーワードを使用し、継続を呼び出しません。
4. **複数回継続を呼び出すハンドラ**: `multiresume`キーワードを使用し、継続を複数回呼び出します。

### 10.4.2 ライフサイクル管理効果

ライフサイクル管理効果は、`effect ... : lifecycle`構文を使用して定義されます。これには以下の特殊な操作が含まれます：

1. **獲得操作**: `with cleanup`修飾子を持ち、リソースの獲得と自動解放を管理します。
2. **解放操作**: `cleanup`関数として定義され、リソースの解放を実装します。

### 10.4.3 管理型の暗黙的な使用

管理型は、`managed`キーワードを使用して定義され、暗黙的なコンテキスト渡しの機能を持っています。これにより、関数シグネチャで`with db: Database`のように暗黙的なパラメータを宣言し、`with`式を使用して暗黙的なコンテキストを提供することができます。

## 10.5 文法の進化

Protorun言語の文法は、言語の進化に伴って拡張されています。最近の追加には以下が含まれます：

1. **効果ハンドラの構文**: `handler`キーワードと特殊な継続制御（`noresume`、`multiresume`）
2. **ライフサイクル管理効果**: `: lifecycle`と`with cleanup`修飾子
3. **管理型の暗黙的な使用**: 管理型に暗黙的なパラメータとしての機能を追加
4. **所有権修飾子**: `own`キーワード
5. **コレクション内包表記とバインド式**: `for`式と`do`式の代わりに導入
6. **型注釈パターン**: 「エンティティ: 型」パターンの一貫した使用

これらの追加は、言語の表現力と安全性を向上させるために設計されています。
