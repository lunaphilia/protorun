# 4. 宣言と文

## 4.1 概要

この章では、Protorun の宣言 (`let`, `impl`) と、文として扱う `return` を定義します。
`while` / `for` / `loop` / `break` / `continue` は文法上は式ですが、ここでは基本形のみを整理します。

宣言は新しい名前をスコープへ導入し、値・関数・型・トレイト・効果・ハンドラを定義します。
詳細な制御式（`if` / `match`）や効果処理の実行モデルはそれぞれ 5 章・6 章を参照してください。

## 4.2 変数宣言 (LetDecl)

`LetDecl` は宣言の基本形です。

```ebnf
LetDecl ::= "let" "mut"? IrrefutablePattern (":" Type)? ("=" Expression)?
```

- `let` は不変束縛を導入します。
- `let mut` は可変束縛を導入します。
- 左辺は `IrrefutablePattern` であり、失敗しないパターンのみ許可されます。
- `let mut` の左辺は単一識別子に制限されます（意味論で検証）。

```protorun
let x = 42
let y: Int = 10

let mut counter = 0
counter = counter + 1

let (a, b) = (1, 2)
let Point { x: px, y: py } = point
```

## 4.3 関数定義 (FunctionDefinitionExpr)

関数定義は `let` と `FunctionDefinitionExpr` を組み合わせて記述します。

```ebnf
FunctionDefinitionExpr ::= FunctionHeader "=" Expression
FunctionHeader ::= GenericParams? ParamList ("->" ReturnType)?
```

`FunctionHeader` に `fn` キーワードは存在しません。

```protorun
let add = (a: Int, b: Int) -> Int = a + b
let square = (x: Int) -> Int = x * x
let identity = [T](x: T) -> T = x

let factorial = (n: Int) -> Int = {
  if n <= 1 then { 1 } else { n * factorial(n - 1) }
}
```

## 4.4 型定義 (TypeDefinitionExpr)

`type` はレコード型またはバリアント型を定義します。

```ebnf
TypeDefinitionExpr ::= "type" GenericParams? "{" (FieldDefinitionList | VariantDefinitionList)? "}"
```

```protorun
// レコード型
let Person = type {
  name: String,
  age: Int
}

// バリアント型
let Option = type[T] {
  Some(T),
  None
}
```

## 4.5 型エイリアス (AliasDefinitionExpr)

既存の型へ別名を与えます。

```ebnf
AliasDefinitionExpr ::= "alias" GenericParams? Type
```

```protorun
let UserId = alias Int
let StringMap = alias[T] Map[String, T]
let Callback = alias (Int) -> String
```

## 4.6 トレイト定義 (TraitDefinitionExpr)

トレイトは振る舞いのインターフェースです。

```ebnf
TraitDefinitionExpr ::= "trait" GenericParams? (":" TypeRef)? "{" TraitItem* "}"
TraitItem ::= LetDecl
```

`TraitItem` は `LetDecl` で統一されます。

- シグネチャ: `let method: (params) -> ReturnType;`
- デフォルト実装: `let method = (params) -> ReturnType = body`

```protorun
let Show = trait {
  let show: (self) -> String;
}

let Ord = trait: Show {
  let compare: (self, other: Self) -> Int;
  let showDebug = (self) -> String = f"Debug: {self.show()}";
}
```

## 4.7 効果インターフェース定義 (EffectDefinitionExpr)

効果インターフェースは操作シグネチャを定義します。

```ebnf
EffectDefinitionExpr ::= "effect" GenericParams? "{" EffectItem* "}"
EffectItem ::= LetDecl
```

`EffectItem` も `LetDecl` です。

```protorun
let State = [S] effect {
  let get: () -> S;
  let put: (value: S) -> Unit;
}

let Console = effect {
  let log: (message: String) -> Unit;
}
```

## 4.8 ハンドラ型定義 (HandlerDefinitionExpr)

ハンドラ定義は、ある効果を特定の型に対して実装する定義式です。

```ebnf
HandlerDefinitionExpr ::= "handler" GenericParams? TypeRef "for" TypeRef WhereClause? "{" HandlerItem* "}"
HandlerItem ::= LetDecl
```

```protorun
let CounterHandler = handler State[Int] for Counter {
  let get = (self: Counter) -> Int = self.count;
  let put = (self: Counter, value: Int) -> Unit = {
    self.count = value
  };
}
```

効果処理の詳細は 6 章および 8 章を参照してください。

## 4.9 トレイト実装 (ImplDecl)

`impl` はトレイトを型へ適用する宣言です。

```ebnf
ImplDecl ::= "impl" GenericParams? TypeRef "for" TypeRef WhereClause? "{" ImplItem* "}"
ImplItem ::= LetDecl
```

`ImplItem` は `LetDecl` で記述します。

```protorun
impl Show for Int {
  let show = (self: Int) -> String = self.toString();
}

impl[T] Show for Option[T] where T: Show {
  let show = (self: Option[T]) -> String = match self {
    Option.Some(v) => f"Some({v.show()})",
    Option.None => "None"
  };
}
```

## 4.10 return 文

`return` は現在の関数評価を終了して呼び出し元へ制御を戻します。

```ebnf
ReturnStatement ::= "return" Expression?
```

- `return expr` は `expr` を返します。
- `return` は `Unit` を返します。
- 関数本体末尾の式による暗黙リターンより優先されます。

```protorun
let classify = (x: Int) -> String = {
  if x < 0 then { return "negative" }
  "non-negative"
}

let printIfPositive = (x: Int) -> Unit = {
  if x <= 0 then { return }
  println(x)
}
```

## 4.11 ループ構文

`while` / `for` / `loop` は反復を表す式です。ここでは基本形のみ示します。

```ebnf
WhileExpr ::= "while" Expression "then" Expression
ForExpr ::= "for" Pattern "in" Expression "then" Expression
LoopExpr ::= "loop" Expression
BreakExpr ::= "break" Expression?
ContinueExpr ::= "continue"
```

```protorun
let mut i = 0
while i < 3 then {
  println(i)
  i = i + 1
}

for n in numbers then {
  if n == 0 then { continue }
  if n < 0 then { break n }
  println(n)
}

let result = loop {
  let input = readLine()
  if input == "quit" then { break 0 }
}
```

`break` / `continue` の代数的効果としての詳細な意味論は 6 章を参照してください。
