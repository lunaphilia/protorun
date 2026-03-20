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
Program ::= (Declaration | Expression)*

Declaration ::= LetDecl | ImplDecl | ModuleDecl | ImportDecl

Statement ::= ReturnStatement
ReturnStatement ::= "return" Expression?

Identifier ::= IDENTIFIER_START IDENTIFIER_CONTINUE*

IDENTIFIER_START ::= [a-zA-Z_]
IDENTIFIER_CONTINUE ::= [a-zA-Z0-9_]

Type ::= TypeRef
       | FunctionType

TypeRef ::= Identifier GenericArgs?
GenericArgs ::= "[" (Type ("," Type)*)? "]"

FunctionType ::= ParamTypeList "->" ReturnType
ParamTypeList ::= "(" (ParamType ("," ParamType)*)? ")"
ParamType ::= Type
            | ImplicitParamType
            | EffectParamType
            | GenericParamType
GenericParamType ::= Identifier
ImplicitParamType ::= "with" TypeRef
EffectParamType ::= "effect" TypeRef
ReturnType ::= TypeRef

LetDecl ::= "let" "mut"? IrrefutablePattern (":" Type)? ("=" Expression)?

Expression ::= LiteralExpr
             | IdentifierExpr
             | BlockExpr
             | IfExpr
             | WhileExpr
             | ForExpr
             | LoopExpr
             | BreakExpr
             | ContinueExpr
             | MatchExpr
             | ListComprehension
             | WithExpr
             | CallExpr
             | MemberAccessExpr
             | BinaryOperatorExpr
             | UnaryOperatorExpr
             | AssignmentExpr
             | RangeExpr
             | StringInterpolation
             | GroupedExpr
             | TypeDefinitionExpr
             | AliasDefinitionExpr
             | FunctionDefinitionExpr
              | TraitDefinitionExpr
              | EffectDefinitionExpr
              | HandlerDefinitionExpr
              | RecordExpr

LiteralExpr ::= IntLiteral | FloatLiteral | StringLiteral | BoolLiteral | UnitLiteral | ListLiteral | TupleLiteral | MapLiteral | SetLiteral

IntLiteral ::= DecimalLiteral | HexLiteral | BinaryLiteral | OctalLiteral

DecimalLiteral ::= DIGIT+ ("_" DIGIT+)* IntSuffix?
HexLiteral ::= "0x" HEX_DIGIT+ ("_" HEX_DIGIT+)* IntSuffix?
BinaryLiteral ::= "0b" BINARY_DIGIT+ ("_" BINARY_DIGIT+)* IntSuffix?
OctalLiteral ::= "0o" OCTAL_DIGIT+ ("_" OCTAL_DIGIT+)* IntSuffix?

IntSuffix ::= "i8" | "i16" | "i32" | "i64" | "i128" 
            | "u8" | "u16" | "u32" | "u64" | "u128"

DIGIT ::= [0-9]
HEX_DIGIT ::= [0-9a-fA-F]
BINARY_DIGIT ::= [01]
OCTAL_DIGIT ::= [0-7]

FloatLiteral ::= DecimalFloatLiteral | HexFloatLiteral

DecimalFloatLiteral ::= DIGIT+ ("_" DIGIT+)* "." DIGIT+ ("_" DIGIT+)* Exponent? FloatSuffix?
                      | DIGIT+ ("_" DIGIT+)* Exponent FloatSuffix?
                      | DIGIT+ ("_" DIGIT+)* FloatSuffix

HexFloatLiteral ::= "0x" HEX_DIGIT+ ("_" HEX_DIGIT+)* "." HEX_DIGIT+ ("_" HEX_DIGIT+)* HexExponent? FloatSuffix?
                  | "0x" HEX_DIGIT+ ("_" HEX_DIGIT+)* HexExponent FloatSuffix?

Exponent ::= ("e" | "E") ("+" | "-")? DIGIT+ ("_" DIGIT+)*
HexExponent ::= ("p" | "P") ("+" | "-")? DIGIT+ ("_" DIGIT+)*

FloatSuffix ::= "f32" | "f64"

StringLiteral ::= RawStringLiteral | RegularStringLiteral

RegularStringLiteral ::= "\"" StringContent* "\""
StringContent ::= EscapeSequence | ~["\" | "\n"]

EscapeSequence ::= "\\" EscapeChar
EscapeChar ::= "n"      // 改行
             | "r"      // キャリッジリターン
             | "t"      // タブ
             | "\\"     // バックスラッシュ
             | "\""     // ダブルクォート
             | "'"      // シングルクォート
             | "0"      // ヌル文字
             | "x" HEX_DIGIT HEX_DIGIT                    // ASCII文字 (\x7F)
             | "u" "{" HEX_DIGIT+ "}"                     // Unicode文字 (\u{1F600})

RawStringLiteral ::= "r" "#"* "\"" RawStringContent* "\"" "#"*
RawStringContent ::= ~"\""  // エスケープなしの任意の文字

BoolLiteral ::= "True" | "False"
UnitLiteral ::= "(" ")"
ListLiteral ::= "[" (Expression ("," Expression)*)? "]"
TupleLiteral ::= "(" (Expression ("," Expression)*)? ")"
MapLiteral ::= "{" (MapEntry ("," MapEntry)*)? "}"
SetLiteral ::= "#{" (Expression ("," Expression)*)? "}"
MapEntry ::= Expression ":" Expression

IdentifierExpr ::= Identifier

BlockExpr ::= "{" BlockItem* "}"
BlockItem ::= LetDecl | Statement | Expression

IfExpr ::= "if" Expression "then" Expression ("elif" Expression "then" Expression)* ("else" Expression)?
WhileExpr ::= "while" Expression "then" Expression
ForExpr ::= "for" Pattern "in" Expression "then" Expression
LoopExpr ::= "loop" Expression
BreakExpr ::= "break" Expression?
ContinueExpr ::= "continue"

MatchExpr ::= "match" Expression "{" MatchArmList? "}"
MatchArmList ::= MatchArm (NEWLINE MatchArm)* NEWLINE?
MatchArm ::= Pattern ("if" Expression)? "=>" Expression

Pattern ::= LiteralPattern
          | Identifier
          | TuplePattern
          | ConstructorPattern
          | WildcardPattern

IrrefutablePattern ::= Identifier
                     | IrrefutableTuplePattern
                     | IrrefutableRecordPattern
                     | WildcardPattern
IrrefutableTuplePattern ::= "(" (IrrefutablePattern ("," IrrefutablePattern)*)? ")"
IrrefutableRecordPattern ::= TypeRef "{" (IrrefutableRecordFieldPattern ("," IrrefutableRecordFieldPattern)*)? ("," "..")? "}"
IrrefutableRecordFieldPattern ::= Identifier (":" IrrefutablePattern)?

LiteralPattern ::= LiteralExpr
TuplePattern ::= "(" (Pattern ("," Pattern)*)? ")"
ConstructorPattern ::= QualifiedIdentifier ("(" (Pattern ("," Pattern)*)? ")")?
QualifiedIdentifier ::= (Identifier ".")* Identifier
WildcardPattern ::= "_"

NEWLINE ::= "\n"+

ListComprehension ::= "[" Expression "for" Pattern "<-" Expression ("if" Expression)? "]"

WithExpr ::= "with" WithBinding ("," WithBinding)* BlockExpr
WithBinding ::= Identifier "=" Expression (":" TypeRef)?

CallExpr ::= Expression "(" ArgList? ")"
ArgList ::= Expression ("," Expression)*

MemberAccessExpr ::= Expression "." Identifier

BinaryOperatorExpr ::= Expression (ArithmeticOperator | ConditionOperator | LogicalOperator | BitwiseOperator) Expression
ArithmeticOperator ::= "+" | "-" | "*" | "/" | "%" 
ConditionOperator ::= "==" | "!=" | "<" | ">" | "<=" | ">=" 
LogicalOperator ::= "&&" | "||" 
BitwiseOperator ::= "&" | "|" | "^" | "<<" | ">>" 

UnaryOperatorExpr ::= UnaryOperator Expression
UnaryOperator ::= "!" | "-" | "~"

AssignmentExpr ::= LValue "=" Expression
LValue ::= IdentifierExpr | MemberAccessExpr

RangeExpr ::= Expression ".." Expression
            | Expression "..=" Expression

GroupedExpr ::= "(" Expression ")"

TypeDefinitionExpr ::= "type" GenericParams? "{" (FieldDefinitionList | VariantDefinitionList)? "}"

FieldDefinitionList ::= FieldDefinition ("," FieldDefinition)*
FieldDefinition ::= "mut"? Identifier ":" Type

VariantDefinitionList ::= VariantDefinition ("," VariantDefinition)*
VariantDefinition ::= Identifier ("(" Identifier? ")")?
                    | Identifier "{" FieldDefinitionList? "}"

GenericParams ::= "[" (GenericParam ("," GenericParam)*)? "]"
GenericParam ::= Identifier (":" TypeConstraint)?
TypeConstraint ::= TypeRef ("+" TypeRef)*

AliasDefinitionExpr ::= "alias" GenericParams? Type

FunctionDefinitionExpr ::= FunctionHeader "=" Expression
FunctionHeader ::= GenericParams? ParamList ("->" ReturnType)?

ParamList ::= "(" (Param ("," Param)*)? ")"
Param ::= SimpleParam | ImplicitParam | EffectParam
SimpleParam ::= Identifier ":" TypeRef
ImplicitParam ::= "with" SimpleParam
EffectParam ::= "effect" SimpleParam

TraitDefinitionExpr ::= "trait" GenericParams? (":" TypeRef)? "{" TraitItem* "}"
TraitItem ::= LetDecl

EffectDefinitionExpr ::= "effect" GenericParams? "{" EffectItem* "}"
EffectItem ::= LetDecl

HandlerDefinitionExpr ::= "handler" GenericParams? TypeRef "for" TypeRef WhereClause? "{" HandlerItem* "}"
HandlerItem ::= LetDecl

RecordExpr ::= TypeRef "{" (RecordFieldInit ("," RecordFieldInit)*)? "}"
RecordFieldInit ::= Identifier ":" Expression

ImplDecl ::= "impl" GenericParams? TypeRef ("for" TypeRef)? WhereClause? "{" ImplItem* "}"
WhereClause ::= "where" WherePredicate ("," WherePredicate)*
WherePredicate ::= TypeRef ":" TypeConstraint
ImplItem ::= LetDecl

ModuleDecl ::= "module" QualifiedIdentifier "{" (Declaration | Statement | Expression)* "}"
ImportDecl ::= "import" QualifiedIdentifier ("." "{" Identifier ("," Identifier)* "}")?
             | "import" QualifiedIdentifier ("as" Identifier)?

StringInterpolation ::= "f" "\"" (StringContent | "{" Expression "}")* "\""

Keyword ::= "let" | "mut" | "impl" | "return" | "if" | "elif" | "else"
          | "match" | "for" | "in" | "while" | "loop" | "break" | "continue" | "then"
          | "trait" | "type" | "alias" | "module" | "import" | "as"
          | "with" | "effect" | "where" | "True" | "False"
```

## 12.3 文法の説明

### 12.3.1 プログラム構造

Protorun言語のプログラムは、トップレベルに配置できる宣言（Declaration）と式（Expression）のシーケンスで構成されます。

### 12.3.2 宣言 (Declaration)

Protorunの宣言は、主に `let` キーワードを用いた束縛宣言と、`impl` キーワードを用いたトレイト実装宣言があります。

- **`LetDecl` (束縛宣言)**: `let` キーワードで始まり、オプションで `mut` キーワード、反駁不可能パターン、オプションの型注釈、そして式 (`Expression`) が続きます。
    - `mut` キーワードがない場合は**不変束縛**となり、`mut` キーワードがある場合は**可変束縛**となります。
    - 左辺には反駁不可能パターン（識別子、タプル、レコード、ワイルドカード）を指定できます。`let mut` の場合は単一の識別子のみです。
    - 右辺の式 (`Expression`) には、通常の計算式だけでなく、関数式や型定義式なども含まれます。
- **`ImplDecl` (トレイト実装)**: `impl` キーワードで始まり、特定の型に対するトレイトの実装を定義します。

**`export` 修飾子**: `let` (不変・可変問わず) および `impl` 宣言の前に `export` キーワードを付与することで、その宣言をモジュール外に公開できます。（詳細は [9. モジュール](09-modules.md) を参照）

### 12.3.3 型システム (Type System)

- **型参照 (`TypeRef`)**: 型名とジェネリック引数で構成されます。タプル型やリスト型は `Tuple[T, U]` や `List[T]` のようにジェネリック型として表現されます（ジェネリクスは `[T]` の角括弧記法を使用）。
- **関数型 (`FunctionType`)**: `(ParamListType?) -> ReturnType` の形式で、パラメータ型と戻り値の型で構成されます。例: `(Int, String) -> Bool`。
- **ジェネリクス (`GenericParams`, `GenericArgs`)**: 型、関数、トレイトなどに型パラメータを導入し、多相的なコードを可能にします。
- **トレイト制約 (`TypeConstraint`, `WhereClause`)**: ジェネリックパラメータが満たすべきトレイトを指定します。

### 12.3.4 文 (Statement)

文は主に制御フローを変更するために使用されます。

- **`ReturnStatement`**: `return Expression?` の形式で、現在の関数から値を返します。

ブロック式 (`BlockExpr`) 内では、宣言 (`Declaration`)、文 (`Statement`)、式 (`Expression`) を記述できます。最後の式がブロックの値となります。副作用のためだけに式を実行する場合も `BlockItem` として扱われます。

### 12.3.5 式 (Expression)

式は評価されて値を生成します。Protorunでは、型定義なども式の一種として扱われます。

- **リテラル (`LiteralExpr`)**: 数値、文字列、真偽値、ユニット `()` など。コレクションリテラル（`ListLiteral`, `TupleLiteral`）も含まれますが、標準ライブラリの型（例: `List[Item]`）で代替される可能性があります。
- **識別子 (`IdentifierExpr`)**: 変数や関数名など。
- **ブロック (`BlockExpr`)**: `{ BlockItem* }` 形式。
- **条件 (`IfExpr`)**: `if cond then { ... } else { ... }` 形式。
- **パターンマッチ (`MatchExpr`)**: `match value { Pattern => Expr` を改行区切りで並べる形式。
- **関数 (`FunctionDefinitionExpr`)**: `[GenericParams]?(Params) (-> ReturnType)? = Expr` 形式の無名関数。
- **呼び出し (`CallExpr`)**: `func(Args)` 形式。
- **メンバーアクセス (`MemberAccessExpr`)**: `expr.identifier` 形式。
- **レコード構築 (`RecordExpr`)**: `TypeName { field: value, ... }` 形式。
- **二項/単項演算 (`BinaryOperatorExpr`, `UnaryOperatorExpr`)**: 演算子を用いた式。
- **代入 (`AssignmentExpr`)**: `lvalue = expr` 形式。`lvalue` は識別子やメンバーアクセスなど。
- **グループ化 (`GroupedExpr`)**: `(expr)` 形式。評価順序の制御。
- **定義式**:
    - **`TypeDefinitionExpr`**: `type [GenericParams]? { ... }`
    - **`TraitDefinitionExpr`**: `trait [GenericParams]? (: SuperTrait)? { ... }` - 効果（effect）もtraitとして定義されます
    - **`HandlerDefinitionExpr`**: `handler [GenericParams]? Effect for Type { ... }`
    - **`AliasDefinitionExpr`**: `alias [GenericParams]? Type`
- **その他**: `WithExpr`（効果ハンドリング）などが含まれます。`CollectionComprehensionExpr`、`BindExpr`、`PartialApplicationExpr` は現時点のEBNF（12.2）には定義されておらず、将来検討項目です。

### 12.3.6 パターン (Pattern)

パターンは主に `match` 式で使用され、値の構造と照合します。`let` 束縛で使用できるパターンは `IrrefutablePattern` として別途定義されており、より限定されています。

**汎用パターン (`Pattern`)**: `match` 式などで使用されます。

- **リテラル (`LiteralPattern`)**: リテラル値とのマッチング。
- **識別子 (`Identifier`)**: 新しい変数を束縛します。
- **タプル (`TuplePattern`)**: `(Pattern1, Pattern2, ...)` 形式。
- **コンストラクタ (`ConstructorPattern`)**: `VariantName(Pattern1, ...)` 形式で、`type` で定義されたヴァリアント型のヴァリアントとマッチングします。
- **ワイルドカード (`WildcardPattern`)**: `_` で任意の値とマッチングし、束縛しません。

`let` 宣言では反駁不可能パターンのみが使用でき、反駁可能なパターン（リテラルやコンストラクタなど）は `match` 式を使用する必要があります。

## 12.4 特殊な構文要素

以下の特殊な構文要素の詳細については、対応する言語仕様の章を参照してください：

- **代数的効果とハンドラ (`effect`, `handler`, `WithExpr`)**: [8. 代数的効果](08-algebraic-effects.md)
- **所有権と借用**: 将来フェーズで導入予定。詳細は [7. 所有権](07-ownership.md) を参照（現在はMVPスコープ外）。
- **モジュールと可視性 (`export`)**: [9. モジュール](09-modules.md)
