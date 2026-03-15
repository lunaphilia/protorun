# Protorun言語仕様

## 1. 言語概要

Protorun は、関数型プログラミング、静的型付け、代数的データ型、トレイト、代数的効果を統合した式ベースの言語です。

この仕様で前提とする主要な設計は以下の通りです。

- **式ベース**: ほとんどの構文要素は値を返す式として扱われる
- **`let` による宣言の統一**: 値、関数、型、トレイト、効果、型エイリアスは `let` で束縛する
- **`impl` は独立した宣言**: トレイト実装と効果実装は `impl` を使う
- **不変が既定**: 可変性は `mut` で明示する
- **副作用は effect パラメータで宣言**: 関数は依存する効果を `effect alias: EffectType` で宣言する
- **`with` によるハンドラ提供**: 効果実装をスコープ付きで与える
- **`match` の網羅性検査**: パターンマッチは全ケース網羅を前提とする
- **セミコロンなし**: ブロック内の要素は通常改行で区切る

プログラムはトップレベルの宣言と式の列からなる。

## 2. 字句構造

### 2.1 キーワード

以下のキーワードは予約語であり、識別子として使用できない。

`alias`, `bind`, `effect`, `else`, `export`, `fn`, `for`, `if`, `impl`, `let`, `match`, `mut`, `own`, `resume`, `return`, `trait`, `type`, `with`

### 2.2 演算子と記号

| 種類 | 記号 |
| --- | --- |
| 算術 | `+` `-` `*` `/` `%` |
| 比較 | `==` `!=` `<` `>` `<=` `>=` |
| 論理 | `&&` `\|\|` `!` |
| 型・制約・参照 | `&` `\|` |
| 代入 | `=` |
| メンバーアクセス | `.` |
| 関数・マッチ | `->` `=>` |
| 注釈・継承 | `:` |
| 束縛 | `<-` |
| ワイルドカード | `_` |
| 区切り | `,` |
| グループ化 | `(` `)` `{` `}` `[` `]` |
| 範囲 | `..` |

`::`、`;`、`...` は現行文法では定義されない。

### 2.3 リテラル

- 整数リテラル: `42`, `0xFF`, `0b1010`, `1_000_000`
- 浮動小数点リテラル: `3.14`, `1e10`, `1.2e-3`
- 文字列リテラル: `"hello"`, `"""multi\nline"""`
- 文字リテラル: `'a'`, `'\n'`
- ブールリテラル: `true`, `false`
- ユニットリテラル: `()`
- リストリテラル: `[1, 2, 3]`

文字列では `\n`, `\t`, `\\`, `\"`, `\u{XXXX}` などのエスケープを使える。

`null` リテラルは存在しない。値の有無は `Option<T>` のような型で表現する。

### 2.4 コメント

```protorun
// 単一行コメント

/*
 * 複数行コメント
 */

/// ドキュメントコメント
```

## 3. 型システム

### 3.1 基本型

基本型として `Int`, `Float`, `Double`, `Bool`, `Char`, `String`, `Unit` を持つ。

### 3.2 複合型

- 配列型: `[T]`
- タプル型: `(T1, T2, ...)`
- 関数型: `fn (...) -> R`
- ジェネリック型: `Option<T>`, `Result<T, E>` など

関数型は入出力の型を表す。効果依存は関数定義時の effect パラメータで宣言する。

### 3.3 型参照と所有権修飾子

型参照は型名とジェネリック引数からなる。

```protorun
T
Result<Int, String>
own FileHandle
&State
&mut Buffer
```

`own`, `&`, `&mut` は文法に含まれる。詳細な所有権意味論はこの文書では扱わない。

### 3.4 ジェネリクスと制約

ジェネリックパラメータは `<T, U, ...>` で導入する。

制約は `:` と `where` で記述する。

```protorun
T: Show
T: Eq & Ord
where T: Show, U: Ord
```

## 4. 宣言

### 4.1 宣言の種類

宣言は次の 2 種類である。

- `let` 宣言
- `impl` 宣言

`export` は `let` および `impl` の前に付与できる。

### 4.2 `let` 宣言

```ebnf
LetDecl ::= ("export")? "let" ("mut")? LetPattern (":" Type)? ("=" Expression)?
```

`let` は新しい束縛を導入する。右辺には通常の値だけでなく、関数式、型定義式、トレイト定義式、効果定義式、型エイリアス定義式を置ける。

```protorun
let x = 42
let mut counter = 0
let add = fn (a: Int, b: Int) -> Int => a + b
let Point = type { x: Float, y: Float }
let Show = trait { let show: fn(self) -> String }
let Logger = effect { let log: fn(message: String) -> Unit }
let UserId = alias Int
```

意味論上の要点:

- `mut` がない束縛は不変
- `let mut` の左辺は単一識別子でなければならない
- 型注釈を省略した場合は右辺から型推論する
- 右辺省略形はシグネチャ宣言として解釈されうる

### 4.3 `let` に使えるパターン

```ebnf
LetPattern ::= LetIdentifierPattern
             | LetTuplePattern
             | LetRecordPattern

LetIdentifierPattern ::= Identifier
LetTuplePattern ::= "(" (LetPattern ("," LetPattern)*)? ")"
LetRecordPattern ::= TypeRef "{" (LetRecordFieldPattern ("," LetRecordFieldPattern)*)? ("," "..")? "}"
LetRecordFieldPattern ::= Identifier (":" LetPattern)?
```

`let` の左辺は、識別子・タプル・レコードの分解束縛に限定される。`match` で使える全パターンが許可されるわけではない。

### 4.4 型定義

型定義は `type` 定義式を `let` で束縛して導入する。

```protorun
let Point = type { x: Float, y: Float }
let Option = type<T> { Some(T) | None }
let Shape = type {
  Circle(Float) |
  Rectangle { width: Float, height: Float } |
  Point
}
```

- フィールド定義の列ならレコード型
- ヴァリアント定義の列ならヴァリアント型
- ヴァリアントの区切りは `|`

### 4.5 型エイリアス

```protorun
let UserId = alias Int
let StringMap = alias<T> Map<String, T>
```

型エイリアスは既存の型に別名を付ける。新しい構造や意味論は導入しない。

### 4.6 トレイト定義

トレイト定義は `trait` 定義式を `let` で束縛して導入する。

```protorun
let Show = trait {
  let show: fn(self) -> String
}

let Ord = trait: Eq {
  let compare: fn(self, other: Self) -> Int
  let equals = fn(self, other: Self) -> Bool => self.compare(other) == 0
}
```

- トレイト本体の要素は `LetDecl`
- 右辺なし `let` はシグネチャ、右辺あり `let` はデフォルト実装
- 継承は `trait: SuperTrait` による単一継承のみ

### 4.7 `impl` 宣言

```ebnf
ImplDecl ::= ("export")? "impl" GenericParams? TypeRef ("for" TypeRef)? WhereClause? "{" ImplItem* "}"
ImplItem ::= LetDecl
```

`impl` は次のどちらにも使う。

- `impl Trait for Type { ... }` : トレイト実装
- `impl Effect for Type { ... }` : 効果実装

```protorun
impl Show for Int {
  let show = fn (self) -> String => self.toString()
}

impl<T> Show for Option<T> where T: Show {
  let show = fn (self) -> String => match self {
    Option.Some(v) => v.show(),
    Option.None => "None"
  }
}
```

## 5. 文

### 5.1 `return` 文

現行仕様で文として定義されるのは `return` のみである。

```ebnf
ReturnStatement ::= "return" Expression?
```

- `return expr` は `expr` を返して現在の関数を終了する
- `return` は `Unit` を返す
- 関数本体の最後の式による暗黙の返り値より優先して評価を打ち切る

`return` は関数内部でのみ使用できる。

## 6. 式

### 6.1 ブロック式

```ebnf
BlockExpr ::= "{" BlockItem* "}"
BlockItem ::= Declaration | Statement | Expression
```

ブロックはローカルスコープを導入する。最後の要素が式ならその値がブロック全体の値になる。空ブロック、または最後の要素が宣言や `return` の場合は `Unit` となる。

### 6.2 条件式

```ebnf
IfExpr ::= "if" Expression BlockExpr ("else" (IfExpr | BlockExpr))?
```

`if` は式であり、各分岐の本体は常にブロック式である。

### 6.3 パターンマッチ

```ebnf
MatchExpr ::= "match" Expression "{" (MatchArm ("," MatchArm)*)? "}"
MatchArm ::= Pattern ("if" Expression)? "=>" Expression
```

`match` は被検査値をパターンと照合し、最初に一致したアームの右辺を評価する。ガード `if` を持てる。コンパイラは網羅性を検査する。

### 6.4 コレクション内包表記

```ebnf
CollectionComprehensionExpr ::= ListComprehension
ListComprehension ::= "[" Expression "for" Pattern "<-" Expression ("if" Expression)? "]"
```

リストの変換・抽出を表す。現行文法ではリスト内包表記のみを定義する。

### 6.5 `bind` 式

```ebnf
BindExpr ::= "bind" "{" (Pattern "<-" Expression)* Expression "}"
```

`bind` はモナド的な逐次計算を平坦に記述するための式である。各束縛は改行区切りで書かれる想定である。

### 6.6 関数式

```ebnf
FunctionHeader ::= "fn" GenericParams? ParamList? EffectParamList? ImplicitParamList? ("->" ReturnType)?
FunctionExpr ::= FunctionHeader "=>" Expression

ParamList ::= "(" (Param ("," Param)*)? ")"
EffectParamList ::= "(" (EffectParam ("," EffectParam)*)? ")"
ImplicitParamList ::= "(" "with" Param ("," Param)* ")"

Param ::= Identifier ":" Type
EffectParam ::= "effect" Identifier ":" TypeRef
```

関数式は無名関数を表す。通常パラメータ、effect パラメータ、implicit パラメータを持てる。関数本体は単一の式であり、複数要素を記述したい場合はブロック式を用いる。

effect パラメータは、その関数がどの効果実装に依存するかを宣言する。

### 6.7 呼び出し・メンバーアクセス・代入

```ebnf
CallExpr ::= Expression "(" ArgList? ")"
ArgList ::= Expression ("," Expression)*

MemberAccessExpr ::= Expression "." Identifier

AssignmentExpr ::= LValue "=" Expression
LValue ::= IdentifierExpr | MemberAccessExpr
```

- 呼び出しは関数適用を表す
- メンバーアクセスはフィールド・メソッド・モジュール要素の参照に使う
- 代入は可変束縛または代入可能なメンバーにのみ許される

### 6.8 `with` 式

```ebnf
WithExpr ::= "with" WithBinding ("," WithBinding)* BlockExpr
WithBinding ::= Identifier "=" Expression (":" TypeRef)?
```

`with` は指定したスコープ内で効果ハンドラ実装を有効にする。各束縛は `alias = instance` または `alias = instance: EffectType` の形を取る。ブロックの評価結果が `with` 式の値となる。

### 6.9 タプル・グループ化・範囲

```ebnf
TupleExpr ::= "(" Expression "," ExpressionList ")"
           | "(" ")"

GroupedExpr ::= "(" Expression ")"
RangeExpr ::= Expression ".." Expression
```

- `()` は `Unit` 値
- 2 要素以上の丸括弧付きカンマ列はタプル
- 単一要素の `(expr)` はグループ化であり、1 要素タプル構文はない

### 6.10 レコード構築

```ebnf
RecordExpr ::= TypeRef "{" (RecordFieldInit ("," RecordFieldInit)*)? "}"
RecordFieldInit ::= Identifier ":" Expression
```

`TypeRef { field: value, ... }` はレコード値を構築する。

### 6.11 部分適用

`Expression(args...)` において `_` を含む形は部分適用式として扱われ、新しい関数値を生成する。

```protorun
let add_one = add(1, _)
let add_ten = add(_, 10)
```

### 6.12 定義式

以下は式であり、通常 `let` の右辺に置かれる。

- `type ...`
- `trait ...`
- `effect ...`
- `alias ...`

このため、型・トレイト・効果の定義も言語上は束縛される値として導入される。

### 6.13 パターン

```ebnf
Pattern ::= LiteralPattern
          | IdentifierPattern
          | TuplePattern
          | ConstructorPattern
          | RecordPattern
          | WildcardPattern

LiteralPattern ::= LiteralExpr
IdentifierPattern ::= ("ref")? ("mut")? Identifier
TuplePattern ::= "(" (Pattern ("," Pattern)*)? ")"
ConstructorPattern ::= QualifiedIdentifier ("(" (Pattern ("," Pattern)*)? ")")?
RecordPattern ::= TypeRef "{" (RecordFieldPattern ("," RecordFieldPattern)*)? ("," "..")? "}"
RecordFieldPattern ::= Identifier (":" Pattern)?
WildcardPattern ::= "_"
```

パターンは主に `match` で使う。`let` で使えるのは制限付きの `LetPattern` のみである。

## 7. 代数的効果

### 7.1 効果インターフェース

効果は `effect` 定義式で操作シグネチャの集合を定義する。

```protorun
let State = effect<S> {
  let get: fn() -> S
  let put: fn(value: S) -> Unit
}

let Logger = effect {
  let log: fn(message: String) -> Unit
}
```

効果定義の本体要素は `LetDecl` であり、通常は右辺なしシグネチャを記述する。

### 7.2 効果実装

効果実装は `impl Effect for Type` で記述する。

```protorun
impl State<Int> for Counter {
  let get = fn (self) -> Int => self.count
  let put = fn (self, value: Int) -> Unit => {
    self.count = value
  }
}
```

実装対象の型インスタンスが状態や設定を保持し、実装関数は `self` を通じてそれにアクセスする。

### 7.3 effect パラメータ

関数は effect パラメータによって依存する効果を宣言する。

```protorun
let counter = fn (effect log: Logger, effect state: State<Int>) -> Int => {
  let current = state.get()
  log.log("count")
  state.put(current + 1)
  state.get()
}
```

`alias.operation(...)` は単なるメソッド呼び出しではなく、対応する効果操作を起動する。

### 7.4 `with` によるハンドラ提供

effect パラメータに対して具体的な実装インスタンスを与えるには `with` を使う。

```protorun
with log = logger, state = counterState {
  counter()
}
```

型注釈 `: EffectType` は省略可能で、省略時は型推論される。複数の効果を同時に提供できる。

### 7.5 継続制御

効果操作が呼ばれると、計算は中断され、ハンドラ実装に制御が移る。ハンドラは継続を以下のいずれかの形で扱える。

1. **暗黙的再開**  
   `resume` を受け取らない場合、ハンドラ本体の評価結果で暗黙的に 1 回再開する。

2. **明示的再開**  
   `resume` パラメータを受け取り、任意のタイミングで継続を呼ぶ。

3. **再開しない**  
   継続を呼ばずに値を返し、`with` 式全体をその値で終了させる。例外的な早期脱出に使える。

```protorun
impl Logger for ConsoleLogger {
  let log = fn (self, message: String, resume: (Unit) -> S) -> Unit => {
    println(message)
    resume(())
  }
}
```

### 7.6 仕様上の留保

以下は現行文書でも未確定または検討中である。

- RAII や自動解放と効果の連携
- 一部の効果関連ライフサイクル規約
- 効果を関数型へどう完全に反映するか

## 8. モジュールシステム

モジュールシステムは公開範囲と名前空間を提供する。

**注意**: `module` 構文自体は現行仕様でも変更の可能性がある。

### 8.1 モジュール定義

```protorun
module Math {
  export let add = fn (a: Int, b: Int) -> Int => a + b
  let helper = fn () -> Int => 42
  export let Point = type { x: Float, y: Float }
}
```

- `export` 付き宣言はモジュール外から参照可能
- `export` なし宣言はモジュール内に閉じる
- モジュールはネスト可能

### 8.2 インポート

```protorun
import Math.add
import Math.{add, Point}
import Math as M
import Math.add as addition
```

選択的インポート、複数インポート、別名インポートを持つ。ワイルドカード形式は記述例があるが、明示的インポートが推奨される。

### 8.3 階層構造

モジュールは階層化できる。

```protorun
module Graphics {
  export module TwoD {
    export let drawRect = fn (x: Int, y: Int) -> Unit => ()
  }
}
```

ファイルシステムとの対応付けも想定されているが、正確な規則は今後の仕様整理対象である。

## 9. EBNF文法

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
             | AliasDefinitionExpr

TypeDefinitionExpr ::= "type" GenericParams? "{" (FieldDefinitionList | VariantDefinitionList)? "}"

FieldDefinitionList ::= FieldDefinition ("," FieldDefinition)*
FieldDefinition ::= Identifier ":" Type

VariantDefinitionList ::= VariantDefinition ("|" VariantDefinition)*
VariantDefinition ::= Identifier ("(" TypeList? ")")?
                    | Identifier "{" (FieldDefinition ("," FieldDefinition)*)? "}"

TraitDefinitionExpr ::= "trait" GenericParams? (":" TypeRef)? "{" TraitItem* "}"
TraitItem ::= LetDecl

EffectDefinitionExpr ::= "effect" GenericParams? "{" EffectItem* "}"
EffectItem ::= LetDecl

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

ExpressionList ::= Expression ("," Expression)*

ArrayType ::= "[" Type "]"

ReturnType ::= Type | "Unit"

Statement ::= ReturnStatement

ReturnStatement ::= "return" Expression?

LiteralExpr ::= IntLiteral | FloatLiteral | StringLiteral | BoolLiteral | UnitLiteral
              | ListLiteral

ListLiteral ::= "[" (Expression ("," Expression)*)? "]"

TupleExpr ::= "(" Expression "," ExpressionList ")"
            | "(" ")"

GroupedExpr ::= "(" Expression ")"

IdentifierExpr ::= Identifier

BlockExpr ::= "{" BlockItem* "}"
BlockItem ::= Declaration | Statement | Expression

IfExpr ::= "if" Expression BlockExpr ("else" (IfExpr | BlockExpr))?

MatchExpr ::= "match" Expression "{" (MatchArm ("," MatchArm)*)? "}"
MatchArm ::= Pattern ("if" Expression)? "=>" Expression

CollectionComprehensionExpr ::= ListComprehension
ListComprehension ::= "[" Expression "for" Pattern "<-" Expression ("if" Expression)? "]"

BindExpr ::= "bind" "{" (Pattern "<-" Expression)* Expression "}"

FunctionHeader ::= "fn" GenericParams? ParamList? EffectParamList? ImplicitParamList? ("->" ReturnType)?

FunctionExpr ::= FunctionHeader "=>" Expression

CallExpr ::= Expression "(" ArgList? ")"
ArgList ::= Expression ("," Expression)*

MemberAccessExpr ::= Expression "." Identifier

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

Operator ::= "+" | "-" | "*" | "/" | "%" | "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&" | "||" | "!"

WhereClause ::= "where" WherePredicate ("," WherePredicate)*
WherePredicate ::= TypeRef ":" TypeConstraint
```

## 10. 補足

- モジュール定義構文は暫定であり、今後変更される可能性がある
- 所有権の詳細意味論は別途詰める必要がある
- 一部の式 (`CollectionComprehensionExpr`, `BindExpr`, 効果と関数型の統合詳細など) は将来調整されうる
