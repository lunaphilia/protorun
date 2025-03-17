# 4. 式と文

## 4.1 変数宣言

```
// 不変変数（デフォルト）
let x = 42
let y: Int = 42

// 可変変数
var counter = 0
var name: String = "John"
```

## 4.2 関数定義

```
// 基本的な関数定義
fn add(a: Int, b: Int): Int = {
  a + b
}

// 単一式の場合は中括弧を省略可能
fn square(x: Int): Int = x * x

// 効果付き関数
fn log(message: String): Unit with Console = {
  Console.log(message)
}

// ジェネリック関数
fn identity<T>(x: T): T = x

// 複数のパラメータリスト（カリー化）
fn add(a: Int)(b: Int): Int = a + b
```

## 4.3 制御構造

```
// if式
if condition {
  // then部
} else {
  // else部
}

// match式
match value {
  pattern1 => expression1,
  pattern2 if guard => expression2,
  _ => defaultExpression
}

// for式
for {
  x <- xs
  y <- ys
  if condition
} yield x + y

// while文
while condition {
  // ループ本体
}

// with式（効果スコープ）
with Console {
  Console.log("このスコープ内でConsole効果を使用可能")
}

// with式（効果ハンドラを指定）
with Console handled by ConsoleHandler {
  Console.log("このスコープ内のConsole効果はConsoleHandlerでハンドル")
}

// do式（モナド的な連鎖のための構文糖）
do {
  user <- findUser(userId)
  email <- getUserEmail(user)
  validEmail <- validateEmail(email)
  validEmail
}

// 効果のスコープ化（旧ローカライゼーション）
with scoped effect Logger {
  // このスコープ内でのみ有効な効果の実装
  fn log(message: String): Unit = {
    println(s"[LOG] $message")
  }
  
  // 効果を使用するコード
  Logger.log("Starting process...")
  // 処理...
  Logger.log("Process complete")
}
```

## 4.4 パターンマッチング

```
// 値のパターンマッチング
match x {
  0 => "ゼロ",
  n if n < 0 => "負の数",
  _ => "その他"
}

// 構造のパターンマッチング
match opt {
  Option.Some(value) => s"値: $value",
  Option.None => "値なし"
}

// タプルのパターンマッチング
match pair {
  (0, y) => s"最初の要素はゼロ、2番目は$y",
  (x, 0) => s"最初の要素は$x、2番目はゼロ",
  (x, y) => s"($x, $y)"
}
```

## 4.5 関数合成

```
// パイプライン演算子
infix operator |> : 0
fn |><A, B>(a: A, f: (A) -> B): B = f(a)

// 効果を持つパイプライン演算子
infix operator |>* : 0
fn |>*<A, B, E>(a: A, f: (A) -> B with E): B with E = f(a)

// パイプラインの使用例
fn processData(data: String): Result<ProcessedData, ProcessError> with Logger = {
  data
    |> parse              // 純粋関数
    |>* validate          // Exception効果
    |>* enrich            // IO効果
    |>* log               // Logger効果
    |> finalize           // 純粋関数
}

// 関数合成演算子
infix operator >>> : 1
fn >>><A, B, C>(f: (A) -> B, g: (B) -> C): (A) -> C = {
  (a: A) => g(f(a))
}

// 効果を持つ関数合成演算子
infix operator >>>* : 1
fn >>>*<A, B, C, E1, E2>(
  f: (A) -> B with E1,
  g: (B) -> C with E2
): (A) -> C with E1 & E2 = {
  (a: A) => g(f(a))
}

// 関数合成の使用例
fn processUser(userId: String): UserStats with IO & Logger = {
  // 関数を合成
  let process = fetchUser >>>* validateUser >>>* enrichUserData >>>* logUserAccess >>> calculateUserStats
  
  // 合成関数を適用
  process(userId)
}
