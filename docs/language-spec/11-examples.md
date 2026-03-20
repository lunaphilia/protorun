# 11. 例：サンプルプログラム

## 11.1 サンプルプログラムの目的と概要

サンプルプログラムは、Protorun言語の機能と使用パターンを実際のコード例を通じて示すことを目的としています。これらの例は以下の役割を果たします：

1. **言語機能の実演**: 言語の主要な機能を実際のコードで示します。
2. **パターンの例示**: 一般的なプログラミングパターンをProtorun言語でどのように実装するかを示します。
3. **ベストプラクティス**: 言語の推奨される使用方法とスタイルを示します。
4. **学習リソース**: 言語を学ぶ開発者のための具体的な参考例を提供します。
5. **機能の組み合わせ**: 異なる言語機能がどのように連携するかを示します。

以下のサンプルプログラムは、Protorun言語の様々な側面を示すために選ばれています。これらの例は、[12. 文法](12-grammar.md)で定義された構文のみを使用しています。

## 11.2 Hello World

この例は、Console効果を使用した最もシンプルなProtorunプログラムを示しています。

```protorun
// Hello World — トップレベル式 + Console効果
let ConsoleLogger = type {}
let ConsoleLoggerHandler = handler Console for ConsoleLogger {
  let println = fn(self, message: String) -> Unit = { /* 組み込み */ }
}

with console = ConsoleLogger {} {
  console.println("Hello, World!")
}
```

この例では、以下の言語機能を示しています：

1. **効果（Effect）**: `Console` 効果を宣言し、外部との入出力を型安全に扱います。
2. **ハンドラ**: `handler` キーワードを使用した効果の実装を定義します。
3. **`with` 式**: 効果ハンドラをスコープに導入します。
4. **メソッド呼び出し**: `console.println(...)` 形式のメソッド呼び出し。

## 11.3 オプション型の定義と使用

この例は、代数的データ型（ヴァリアント型）とパターンマッチングを使用した、オプション型の実装を示しています。

```protorun
// オプション型の定義
let Option = type[T] {
  Some(T),
  None
}

// オプション値を操作する関数
let map = fn[T, U](option: Option[T], f: (T) -> U) -> Option[U] = {
  match option {
    Option.Some(value) => Option.Some(f(value)),
    Option.None => Option.None
  }
}

let getOrElse = fn[T](option: Option[T], default: T) -> T = {
  match option {
    Option.Some(value) => value,
    Option.None => default
  }
}

// 使用例
let main = fn() -> Unit = {
  let maybeNumber = Option.Some(42)
  let noNumber = Option.None

  let doubled = map(maybeNumber, fn(x) = x * 2)
  let result1 = getOrElse(doubled, 0)

  let result2 = getOrElse(noNumber, 0)

  result1
}
```

この例では、以下の言語機能を示しています：

1. **ヴァリアント型**: `type[T] { Some(T), None }` 構文を使用した代数的データ型の定義
2. **ジェネリクス**: 型パラメータ `[T]` を使用した汎用的な型の定義
3. **パターンマッチング**: `match` 式を使用した値の分解と条件分岐
4. **関数定義**: `fn` キーワードを使用した関数の定義
5. **高階関数**: 関数を引数として受け取る `map` 関数

## 11.4 リスト処理

この例は、標準ライブラリのリストメソッドを使用した基本的なリスト操作を示しています。

```protorun
// リスト操作の基本 — 標準ライブラリメソッドを使用
let numbers = [1, 2, 3, 4, 5]

// 長さの取得
let len = numbers.length()

// リストの要素を変換（map）
let doubled = numbers.map(fn(x) = x * 2)

// リストのフィルタリング
let evens = numbers.filter(fn(x) = x % 2 == 0)

// リストの畳み込み（foldLeft）
let sum = numbers.foldLeft(0, fn(acc, x) = acc + x)

// 複数の操作をメソッドチェーンで組み合わせ
let result = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
  .filter(fn(x) = x % 2 == 0)
  .map(fn(x) = x * x)
  .foldLeft(0, fn(acc, x) = acc + x)
```

この例では、以下の言語機能を示しています：

1. **リストリテラル**: `[1, 2, 3, 4, 5]` のようなリストの構築
2. **メソッドチェーン**: `list.filter(...).map(...).foldLeft(...)` による操作の連鎖
3. **高階関数**: `map`, `filter`, `foldLeft` への関数引数
4. **無名関数**: `fn(x) = ...` 形式の無名関数

## 11.5 バイナリツリー

この例は、バイナリツリーのデータ構造と操作を示しています。

```protorun
// バイナリツリーの定義
let Tree = type[T] {
  Leaf,
  Node(T, Tree[T], Tree[T])
}

// ツリーに要素を挿入
let insert = fn[T](tree: Tree[T], value: T, compare: (T, T) -> Int) -> Tree[T] = {
  match tree {
    Tree.Leaf => Tree.Node(value, Tree.Leaf, Tree.Leaf),
    Tree.Node(data, left, right) => {
      let cmp = compare(value, data)
      if cmp < 0 then {
        Tree.Node(data, insert(left, value, compare), right)
      } elif cmp > 0 then {
        Tree.Node(data, left, insert(right, value, compare))
      } else {
        tree
      }
    }
  }
}

// ツリーの要素を検索
let contains = fn[T](tree: Tree[T], value: T, compare: (T, T) -> Int) -> Bool = {
  match tree {
    Tree.Leaf => False,
    Tree.Node(data, left, right) => {
      let cmp = compare(value, data)
      if cmp < 0 then {
        contains(left, value, compare)
      } elif cmp > 0 then {
        contains(right, value, compare)
      } else {
        True
      }
    }
  }
}

// ツリーを走査してリストに変換（中順走査）
let toList = fn[T](tree: Tree[T]) -> List[T] = {
  match tree {
    Tree.Leaf => [],
    Tree.Node(data, left, right) => {
      let leftList = toList(left)
      let rightList = toList(right)
      leftList.concat([data]).concat(rightList)
    }
  }
}

// 使用例
let main = fn() -> Unit = {
  let compareInt = fn(a: Int, b: Int) -> Int = a - b

  let tree = Tree.Leaf
  let tree1 = insert(tree, 5, compareInt)
  let tree2 = insert(tree1, 3, compareInt)
  let tree3 = insert(tree2, 7, compareInt)
  let tree4 = insert(tree3, 1, compareInt)

  let found = contains(tree4, 3, compareInt)
  let sorted = toList(tree4)

  sorted
}
```

この例では、以下の言語機能を示しています：

1. **再帰的データ構造**: `Tree[T]` 型自身を含むヴァリアント型の定義
2. **高階関数**: 比較関数を引数として受け取るパターン
3. **ネストしたパターンマッチング**: `Tree.Node(data, left, right)` による深い分解
4. **リストのメソッド呼び出し**: `concat` によるリストの結合

## 11.6 Result型とエラー処理

この例は、Result型を使用したエラー処理のパターンを示しています。

```protorun
// Result型の定義
let Result = type[T, E] {
  Ok(T),
  Err(E)
}

// Resultを操作する関数
let mapResult = fn[T, U, E](result: Result[T, E], f: (T) -> U) -> Result[U, E] = {
  match result {
    Result.Ok(value) => Result.Ok(f(value)),
    Result.Err(error) => Result.Err(error)
  }
}

let flatMap = fn[T, U, E](result: Result[T, E], f: (T) -> Result[U, E]) -> Result[U, E] = {
  match result {
    Result.Ok(value) => f(value),
    Result.Err(error) => Result.Err(error)
  }
}

// 数値演算の例
let divide = fn(a: Int, b: Int) -> Result[Int, String] = {
  if b == 0 then {
    Result.Err("Division by zero")
  } else {
    Result.Ok(a / b)
  }
}

let safeSqrt = fn(x: Int) -> Result[Int, String] = {
  if x < 0 then {
    Result.Err("Negative number")
  } else {
    Result.Ok(x)
  }
}

// 複数の演算を連鎖
let compute = fn(a: Int, b: Int, c: Int) -> Result[Int, String] = {
  flatMap(
    divide(a, b),
    fn(quotient) = flatMap(
      safeSqrt(quotient),
      fn(sqrt) = Result.Ok(sqrt + c)
    )
  )
}

// 使用例
let main = fn() -> Unit = {
  let result1 = compute(100, 10, 5)
  let result2 = compute(100, 0, 5)
  let result3 = compute(-100, 10, 5)

  match result1 {
    Result.Ok(value) => value,
    Result.Err(msg) => 0
  }
}
```

この例では、以下の言語機能を示しています：

1. **複数の型パラメータ**: `Result[T, E]` のような複数のジェネリック型パラメータ
2. **エラー処理パターン**: Result型を使用した型安全なエラー処理
3. **関数の連鎖**: `flatMap` を使用した計算の合成
4. **ネストした関数呼び出し**: 複数のResult値を組み合わせる処理

## 11.7 マップとペアの操作

この例は、マップリテラルとペア型を使用したデータ操作を示しています。

```protorun
// ペア型の定義
let Pair = type[A, B] {
  MakePair(A, B)
}

// マップの操作
let main = fn() -> Unit = {
  let scores = {
    "Alice" : 95,
    "Bob" : 87,
    "Charlie" : 92
  }

  let person = Pair.MakePair("Alice", 25)

  match person {
    Pair.MakePair(name, age) => {
      let score = scores[name]
      score
    }
  }
}
```

この例では、以下の言語機能を示しています：

1. **マップリテラル**: `{ key : value }` 構文
2. **ペア型**: 2つの値を組み合わせた型
3. **マップアクセス**: `map[key]` による値の取得

## 11.8 型エイリアスと抽象化

この例は、型エイリアスを使用した型の抽象化を示しています。

```protorun
// 型エイリアスの定義
let UserId = alias Int
let UserName = alias String
let Email = alias String

// ユーザー型の定義
let User = type {
  id: UserId,
  name: UserName,
  email: Email
}

// ユーザー情報を表示する関数
let displayUser = fn(user: User) -> String = {
  let name = match user { User(id, name, email) => name }
  name
}

// 使用例
let main = fn() -> Unit = {
  let user = User {
    id: 1,
    name: "Alice",
    email: "alice@example.com"
  }

  let info = displayUser(user)
  info
}
```

この例では、以下の言語機能を示しています：

1. **型エイリアス**: `alias` キーワードを使用した型の別名定義
2. **レコード型**: フィールドを持つ構造体のような型
3. **レコード構築**: `TypeName { field: value }` 構文
4. **型の抽象化**: 基本型にドメイン固有の意味を与えるパターン

## 11.9 ループ

この例は、Protorunのループ構文を示しています。

```protorun
// while ループ
let mut i = 0
while i < 5 then {
  with console = ConsoleLogger {} {
    console.println(f"i = {i}")
  }
  i = i + 1
}

// for ループ — リストの反復
let numbers = [1, 2, 3, 4, 5]
for n in numbers then {
  with console = ConsoleLogger {} {
    console.println(f"n = {n}")
  }
}

// for ループ — 範囲の反復
for x in 1..=10 then {
  with console = ConsoleLogger {} {
    console.println(f"x = {x}")
  }
}

// loop + break — 値を返すループ
let result = loop {
  let x = 42
  if x > 10 then {
    break x
  }
}
```

この例では、以下の言語機能を示しています：

1. **`while` ループ**: `while condition then body` 構文
2. **`for` ループ**: `for pattern in iterable then body` 構文
3. **範囲式**: `1..=10` による範囲の生成
4. **`loop` 式**: 無限ループと `break` による値の返却
5. **可変束縛**: `let mut i = 0` と `i = i + 1` による変数の更新

## 11.10 文字列補間

この例は、Protorunの文字列補間機能を示しています。

```protorun
// 基本的な文字列補間
let name = "Protorun"
let version = 1
let greeting = f"Welcome to {name} v{version}!"

// 式の補間
let x = 42
let msg = f"The answer is {x * 2}"

// 複数の補間
let firstName = "Alice"
let lastName = "Smith"
let fullName = f"{firstName} {lastName}"

// Console効果を使用した出力
with console = ConsoleLogger {} {
  console.println(greeting)
  console.println(msg)
  console.println(f"Full name: {fullName}")
}
```

この例では、以下の言語機能を示しています：

1. **文字列補間**: `f"..."` リテラルと `{expr}` による値の埋め込み
2. **式の補間**: 文字列補間内での任意の式評価
3. **`Console` 効果**: 補間された文字列の出力
