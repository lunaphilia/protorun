# 8. 標準ライブラリ

## 8.1 概要

Protorun言語の標準ライブラリ（prelude）は、言語の基本的な機能を提供する最小限のモジュール集合です。
preludeに含まれる型、トレイト、効果は自動的にインポートされます。

## 8.2 コアデータ型

### 8.2.1 Option[T]
値の存在/不在を表現する型。

```protorun
let Option = type[T] {
  Some(T),
  None
}
```

主なメソッド:
- `let isSome: (self) -> Bool`
- `let isNone: (self) -> Bool`
- `let unwrap: (self) -> T` — Noneの場合はpanicする
- `let unwrapOr: (self, default: T) -> T`
- `let map: [U](self, f: (T) -> U) -> Option[U]`
- `let flatMap: [U](self, f: (T) -> Option[U]) -> Option[U]`

### 8.2.2 Result[T, E]
成功/失敗を表現する型。

```protorun
let Result = type[T, E] {
  Ok(T),
  Err(E)
}
```

主なメソッド:
- `let isOk: (self) -> Bool`
- `let isErr: (self) -> Bool`
- `let unwrap: (self) -> T` — Errの場合はpanicする
- `let unwrapOr: (self, default: T) -> T`
- `let map: [U](self, f: (T) -> U) -> Result[U, E]`
- `let flatMap: [U](self, f: (T) -> Result[U, E]) -> Result[U, E]`

### 8.2.3 List[T]
可変長リスト型。

```protorun
let List = type[T] { /* 組み込み実装 */ }
```

主なメソッド:
- `let length: (self) -> Int`
- `let get: (self, index: Int) -> Option[T]`
- `let set: (mut self, index: Int, value: T) -> Unit`
- `let push: (mut self, value: T) -> Unit`
- `let pop: (mut self) -> Option[T]`
- `let map: [U](self, f: (T) -> U) -> List[U]`
- `let filter: (self, predicate: (T) -> Bool) -> List[T]`
- `let foldLeft: [R](self, initial: R, f: (R, T) -> R) -> R`
- `let iter: (self) -> Iterator[T]`

### 8.2.4 Map[K, V]
キーと値のマッピング型。

```protorun
let Map = type[K, V] { /* 組み込み実装 */ }
```

主なメソッド:
- `let get: (self, key: K) -> Option[V]`
- `let set: (mut self, key: K, value: V) -> Unit`
- `let remove: (mut self, key: K) -> Option[V]`
- `let contains: (self, key: K) -> Bool`
- `let keys: (self) -> List[K]`
- `let values: (self) -> List[V]`
- `let iter: (self) -> Iterator[(K, V)]`

### 8.2.5 Set[T]
重複なし集合型。

```protorun
let Set = type[T] { /* 組み込み実装 */ }
```

主なメソッド:
- `let contains: (self, value: T) -> Bool`
- `let insert: (mut self, value: T) -> Bool`
- `let remove: (mut self, value: T) -> Bool`
- `let size: (self) -> Int`
- `let iter: (self) -> Iterator[T]`

### 8.2.6 String
テキスト型。

主なメソッド:
- `let length: (self) -> Int`
- `let concat: (self, other: String) -> String`
- `let contains: (self, substr: String) -> Bool`
- `let startsWith: (self, prefix: String) -> Bool`
- `let endsWith: (self, suffix: String) -> Bool`
- `let trim: (self) -> String`
- `let toUpperCase: (self) -> String`
- `let toLowerCase: (self) -> String`
- `let split: (self, delimiter: String) -> List[String]`

### 8.2.7 Range[T] と RangeInclusive[T]
範囲型。

```protorun
let Range = type[T] {
  start: T,
  end: T
}

let RangeInclusive = type[T] {
  start: T,
  end: T
}
```

- `Range[T]` は `Iterable[T]` を実装する（半開区間 [start, end)）
- `RangeInclusive[T]` は `Iterable[T]` を実装する（閉区間 [start, end]）

## 8.3 コアトレイト

### 8.3.1 Show
文字列表現を提供するトレイト。

```protorun
let Show = trait {
  let show: (self) -> String
}
```

組み込み型の実装: `Int`, `Float`, `Bool`, `Char`, `String`, `Unit` は全て `Show` を実装する。

### 8.3.2 Eq
等価比較を提供するトレイト。

```protorun
let Eq = trait {
  let equals: (self, other: Self) -> Bool
}
```

`==` 演算子は `Eq.equals()` にデシュガーされる。
`!=` 演算子は `!Eq.equals()` にデシュガーされる。

### 8.3.3 Ord
順序比較を提供するトレイト。

```protorun
let Ord = trait: Eq {
  let compare: (self, other: Self) -> Int
  // compare が負 -> self < other
  // compare が 0 -> self == other
  // compare が正 -> self > other
}
```

`<`, `>`, `<=`, `>=` 演算子は `Ord.compare()` にデシュガーされる。

### 8.3.4 Iterator[T]
イテレータプロトコル。

```protorun
let Iterator = trait[T] {
  let next: (mut self) -> Option[T]
}
```

### 8.3.5 Iterable[T]
イテレータに変換できる型のトレイト。

```protorun
let Iterable = trait[T] {
  let iter: (self) -> Iterator[T]
}
```

`List[T]`, `Range[T]`, `RangeInclusive[T]`, `Set[T]`, `Map[K, V]` は全て `Iterable` を実装する。

### 8.3.6 Hash
ハッシュ計算を提供するトレイト（Map/Setのキーに必要）。

```protorun
let Hash = trait: Eq {
  let hash: (self) -> Int
}
```

## 8.4 数値トレイト

数値演算は組み込み型に対してのみ動作する（MVP）。

```protorun
let Add = trait[Rhs = Self, Output = Self] {
  let add: (self, rhs: Rhs) -> Output
}

let Sub = trait[Rhs = Self, Output = Self] {
  let sub: (self, rhs: Rhs) -> Output
}

let Mul = trait[Rhs = Self, Output = Self] {
  let mul: (self, rhs: Rhs) -> Output
}

let Div = trait[Rhs = Self, Output = Self] {
  let div: (self, rhs: Rhs) -> Output
}
```

`+`, `-`, `*`, `/` 演算子はそれぞれ対応するトレイトメソッドにデシュガーされる（組み込み型のみ）。

## 8.5 コア効果

### 8.5.1 Console 効果

```protorun
let Console = effect {
  let println: (message: String) -> Unit
  let print: (message: String) -> Unit
  let readLine: () -> String
}
```

標準的なハンドラ実装:

```protorun
let ConsoleLogger = type {}

let ConsoleLoggerHandler = handler Console for ConsoleLogger {
  let println = (self, message: String) -> Unit = {
    // 標準出力に改行付きで出力（組み込み実装）
  }
  let print = (self, message: String) -> Unit = {
    // 標準出力に出力（組み込み実装）
  }
  let readLine = (self) -> String = {
    // 標準入力から1行読み込み（組み込み実装）
  }
}
```

## 8.6 基本関数

```protorun
// プログラムを異常終了させる
let panic: (message: String) -> Unit

// 条件が False の場合にpanicする
let assert: (condition: Bool, message: String) -> Unit
```

## 8.7 デシュガー規則

言語構文がどのprelude要素にデシュガーされるかを以下に示す。

| 構文 | デシュガー先 |
|------|------------|
| `a == b` | `Eq.equals(a, b)` |
| `a != b` | `!Eq.equals(a, b)` |
| `a < b` | `Ord.compare(a, b) < 0` |
| `a > b` | `Ord.compare(a, b) > 0` |
| `a <= b` | `Ord.compare(a, b) <= 0` |
| `a >= b` | `Ord.compare(a, b) >= 0` |
| `a + b` | `Add.add(a, b)` |
| `a - b` | `Sub.sub(a, b)` |
| `a * b` | `Mul.mul(a, b)` |
| `a / b` | `Div.div(a, b)` |
| `f"hello {x}"` | `String.concat(["hello ", x.show()])` |
| `1..10` | `Range { start: 1, end: 10 }` |
| `1..=10` | `RangeInclusive { start: 1, end: 10 }` |
| `for x in expr then body` | `expr.iter()` + `Iterator.next()` ループ |
| `[x * 2 for x <- list if cond]` | `list.filter(cond).map((x) = x * 2)` |
