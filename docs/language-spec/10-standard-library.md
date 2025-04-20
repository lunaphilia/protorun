# 10. 標準ライブラリ

## 10.1 標準ライブラリの概念と目的

標準ライブラリは、Protorun言語に組み込まれた基本的な機能とデータ構造を提供するコンポーネントです。この標準ライブラリは以下の目的で設計されています：

1. **基本機能の提供**: 一般的なプログラミングタスクに必要な基本的な機能を提供します。
2. **一貫性の確保**: 言語全体で一貫したAPIとデータ構造を提供します。
3. **効率性の最適化**: 頻繁に使用される操作を効率的に実装します。
4. **安全性の保証**: 型安全性とメモリ安全性を保証する方法でAPIを設計します。
5. **拡張性の確保**: ユーザーが独自の機能を追加できるように拡張可能な設計を提供します。

標準ライブラリは、言語の核となる部分であり、すべてのProtorunプログラムで利用可能です。これにより、開発者は基本的な機能を再実装する必要なく、より高レベルの問題に集中できます。

## 10.2 コアデータ構造

標準ライブラリは、プログラミングに不可欠な基本的なデータ構造を提供します。

```protorun
// リスト (ヴァリアント型)
let List = type<T> {
  Cons(head: T, tail: List<T>),
  Nil
}

// List モジュール (コンパニオンオブジェクト的な役割)
module List {
  // リスト操作 (例)
  // 注意: 以下の関数定義は例であり、実際の標準ライブラリの実装とは異なる場合があります。
  // また、List<T> のメソッドとして実装される可能性もあります。
  export let empty = fn <T>(): List<T> => List.Nil
  export let cons = fn <T>(head: T, tail: List<T>): List<T> => List.Cons(head, tail)
  // export let append = fn <T>(list1: List<T>, list2: List<T>): List<T> => { /* ... */ } // 例: appendの実装は省略
  // map や filter などの高階関数は、List<T> 型のメソッドとして
  // または List モジュール内の関数として提供される可能性があります。
}

// マップ (トレイトとして定義 - 実装は標準ライブラリで提供)
let Map = trait<K, V> {
  fn get(key: K): Option<V>
  fn put(key: K, value: V): Map<K, V> // Note: May need to be mutable or return new map
  fn remove(key: K): Map<K, V>      // Note: May need to be mutable or return new map
  fn contains(key: K): Bool
  fn keys(): List<K>
  fn values(): List<V>
  fn entries(): List<(K, V)> // タプル値は維持
  fn isEmpty(): Bool
  fn size(): Int
}
// Map モジュール (コンパニオンオブジェクト的な役割)
module Map {
  // マップ構築関数など (例)
  export let empty = fn <K, V>(): Map<K, V> => { /* ... */ }
}


// セット (トレイトとして定義 - 実装は標準ライブラリで提供)
let Set = trait<T> {
  fn contains(value: T): Bool
  fn add(value: T): Set<T>      // Note: May need to be mutable or return new set
  fn remove(value: T): Set<T>   // Note: May need to be mutable or return new set
  fn union(other: Set<T>): Set<T>
  fn intersection(other: Set<T>): Set<T>
  fn difference(other: Set<T>): Set<T>
  fn isEmpty(): Bool
  fn size(): Int
  fn toList(): List<T>
}
// Set モジュール (コンパニオンオブジェクト的な役割)
module Set {
  // セット構築関数など (例)
  export let empty = fn <T>(): Set<T> => { /* ... */ }
}

// オプション型 (ヴァリアント型)
let Option = type<T> {
  Some(T),
  None
}

// 結果型 (ヴァリアント型)
let Result = type<T, E> {
  Ok(T),
  Err(E)
}

// Result型の拡張メソッド (implブロックを使用)
impl<T, E> Result<T, E> {
  // 既存のメソッド
  let map = fn <U>(self, f: (T) -> U): Result<U, E> => match self {
    Result.Ok(value) => Result.Ok(f(value)),
    Result.Err(error) => Result.Err(error)
  }

  let flatMap = fn <U>(self, f: (T) -> Result<U, E>): Result<U, E> => match self {
    Result.Ok(value) => f(value),
    Result.Err(error) => Result.Err(error)
  }

  // 新しいユーティリティメソッド
  let mapErr = fn <F>(self, f: (E) -> F): Result<T, F> => match self {
    Result.Ok(value) => Result.Ok(value),
    Result.Err(error) => Result.Err(f(error))
  }

  let flatMapErr = fn <F>(self, f: (E) -> Result<T, F>): Result<T, F> => match self {
    Result.Ok(value) => Result.Ok(value),
    Result.Err(error) => f(error)
  }

  // 注意: panic は言語のコア機能として別途定義される必要があります
  let unwrap = fn (self): T => match self {
    Result.Ok(value) => value,
    Result.Err(_) => panic("Result.unwrap called on an Err value")
  }

  let unwrapOr = fn (self, default: T): T => match self {
    Result.Ok(value) => value,
    Result.Err(_) => default
  }

  let unwrapOrElse = fn (self, f: (E) -> T): T => match self {
    Result.Ok(value) => value,
    Result.Err(error) => f(error)
  }

  let isOk = fn (self): Bool => match self {
    Result.Ok(_) => true,
    Result.Err(_) => false
  }

  let isErr = fn (self): Bool => match self {
    Result.Ok(_) => false,
    Result.Err(_) => true
  }

  // 注意: panic は言語のコア機能として別途定義される必要があります
  let unwrapErr = fn (self): E => match self {
    Result.Ok(_) => panic("Result.unwrapErr called on an Ok value"),
    Result.Err(error) => error
  }
}

// Result モジュール (ユーティリティ関数)
module Result {
  // 複数のResultを結合する (実装は List の機能に依存するため、シグネチャのみ示す)
  export let all = fn <T, E>(results: List<Result<T, E>>): Result<List<T>, E> => {
    // 実装例 (List.fold, List.reverse, List.cons が必要)
    // results.fold(Result.Ok(List.empty<T>()), (accResult, currentResult) => {
    //   bind {
    //     acc <- accResult
    //     current <- currentResult
    //     Result.Ok(List.cons(current, acc)) // 逆順になるので最後に reverse が必要
    //   }
    // }).map(list => list.reverse()) // List.reverse が必要
    /* ... 実装 ... */
  }

  // 最初に成功したResultを返す (実装は List の機能に依存するため、シグネチャのみ示す)
  export let any = fn <T, E>(results: List<Result<T, E>>): Result<T, List<E>> => {
    // 実装例 (List.fold, List.reverse, List.cons が必要)
    // results.fold(Result.Err(List.empty<E>()), (accResult, currentResult) => {
    //   match (accResult, currentResult) {
    //     (Result.Ok(v), _) => Result.Ok(v), // 既に成功が見つかっていればそれを返す
    //     (_, Result.Ok(v)) => Result.Ok(v), // 現在のものが成功ならそれを返す
    //     (Result.Err(errs), Result.Err(err)) => Result.Err(List.cons(err, errs)) // 両方エラーならエラーリストに追加 (逆順)
    //   }
    // }).mapErr(errs => errs.reverse()) // List.reverse が必要
    /* ... 実装 ... */
  }
}
```

## 10.3 I/O操作

標準ライブラリは、ファイルシステムやコンソールとの対話など、I/O操作のための効果を提供します。

```protorun
// I/O効果
let IO = effect {
  fn readFile(path: String): Result<String, IOError>
  fn writeFile(path: String, content: String): Result<Unit, IOError>
  fn fileExists(path: String): Bool
  fn deleteFile(path: String): Result<Unit, IOError>

  fn println(message: String): Unit
  fn print(message: String): Unit
  fn readLine(): String

  fn getEnv(name: String): Option<String>
  fn setEnv(name: String, value: String): Result<Unit, IOError>
}

// 仮のIOエラー型 (ヴァリアント型)
let IOError = type { FileNotFound(String), PermissionDenied, Other(String) }

// IOハンドラの実装例 (特定のプラットフォーム向け)
let IOHandler = type { /* プラットフォーム依存の状態など */ }
let IOHandlerImpl = handler IO for IOHandler {
  let readFile = fn (self, path: String): Result<String, IOError> => {
    // プラットフォーム固有の実装
    /* ... */
  }

  let writeFile = fn (self, path: String, content: String): Result<Unit, IOError> => {
    // プラットフォーム固有の実装
    /* ... */
  }

  let fileExists = fn (self, path: String): Bool => {
    // プラットフォーム固有の実装
    /* ... */
  }

  let deleteFile = fn (self, path: String): Result<Unit, IOError> => {
    // プラットフォーム固有の実装
    /* ... */
  }

  let println = fn (self, message: String): Unit => {
    // プラットフォーム固有の実装
    /* ... */
  }

  let print = fn (self, message: String): Unit => {
    // プラットフォーム固有の実装
    /* ... */
  }

  let readLine = fn (self): String => {
    // プラットフォーム固有の実装
    /* ... */
  }

  let getEnv = fn (self, name: String): Option<String> => {
    // プラットフォーム固有の実装
    /* ... */
  }

  let setEnv = fn (self, name: String, value: String): Result<Unit, IOError> => {
    // プラットフォーム固有の実装
    /* ... */
  }
}

// 使用例
let processFile = fn (path: String) (effect io: IO): Result<String, IOError> => {
  if io.fileExists(path) {
    let content = io.readFile(path)?
    let processed = processContent(content) // processContent は別途定義
    io.writeFile(path + ".processed", processed)?
    io.println(s"ファイル $path を処理しました")
    Result.Ok(processed)
  } else {
    io.println(s"ファイル $path が見つかりません")
    Result.Err(IOError.FileNotFound(path))
  }
}

// 実行例
let main = fn () => {
  let ioHandler = IOHandler { /* ... */ }
  with io = ioHandler {
    processFile("my_data.txt") match {
      Result.Ok(result) => io.println(s"処理成功: ${result}"),
      Result.Err(err) => io.println(s"処理失敗: ${err}") // エラー表示方法要検討
    }
  }
}
```

## 10.4 並行処理

標準ライブラリは、並行処理と非同期プログラミングのためのサポートを提供します。

```protorun
// 仮の型
let Duration = type { /* ... */ }
module Duration { export let ofSeconds = fn (Int): Duration => { /* ... */ } }
let Task = type<T> { /* ... */ } // 非同期タスクを表す型
let TimeoutError = type {} // レコード型 (空)
let NetworkError = type { Timeout, ConnectionFailed, Other(String) } // ヴァリアント型

// 非同期効果
let Async = effect {
  fn spawn<T>(task: () -> T): Task<T>
  fn await<T>(task: Task<T>): T
  fn sleep(duration: Duration): Unit

  fn withTimeout<T>(duration: Duration, task: () -> T): Result<T, TimeoutError>
  fn race<T>(tasks: List<() -> T>): T
  fn all<T>(tasks: List<() -> T>): List<T>
}

// Asyncハンドラの実装例
let AsyncHandler = type { /* スレッドプールなどの状態 */ }
let AsyncHandlerImpl = handler Async for AsyncHandler {
  let spawn = fn <T> (self, task: () -> T, resume: (Task<T>) -> Unit): Unit => {
    let taskHandle = createTask(task) // createTask は内部関数と仮定
    resume(taskHandle)
  }

  let await = fn <T> (self, task: Task<T>, resume: (T) -> Unit): Unit => {
    // task.onComplete は Task<T> のメソッドと仮定
    task.onComplete(result => resume(result))
  }

  let sleep = fn (self, duration: Duration): Unit => {
    // プラットフォーム固有の実装
    /* ... */
  }

  let withTimeout = fn <T> (self, duration: Duration, task: () -> T): Result<T, TimeoutError> => {
    // 実装
    /* ... */
  }

  let race = fn <T> (self, tasks: List<() -> T>): T => {
    // 実装
    /* ... */
  }

  let all = fn <T> (self, tasks: List<() -> T>): List<T> => {
    // 実装
    /* ... */
  }
}

// 使用例
let fetchData = fn (url: String) (effect async: Async, effect io: IO): Result<String, NetworkError> => {
  io.println(s"データを取得中: $url")

  let task = async.spawn(() => {
    networkRequest(url) // networkRequest は外部関数と仮定
  })

  async.withTimeout(Duration.ofSeconds(10), () => {
    let result = async.await(task)? // await が Result を返すか、例外効果を使うか要検討
    io.println(s"データ取得完了: ${result.length} バイト")
    Result.Ok(result)
  }) match {
    Result.Ok(dataResult) => dataResult, // dataResult は Result<String, NetworkError>
    Result.Err(_) => {
      io.println("タイムアウトしました")
      Result.Err(NetworkError.Timeout)
    }
  }
}

// 実行例
let mainAsync = fn () => {
  let asyncHandler = AsyncHandler { /* ... */ }
  let ioHandler = IOHandler { /* ... */ }
  with async = asyncHandler, io = ioHandler {
    fetchData("http://example.com") match {
      Result.Ok(data) => io.println("成功"),
      Result.Err(err) => io.println("失敗")
    }
  }
}
```

並行処理は代数的効果として実装され、非同期プログラミングを型安全かつ直感的に行うことができます。これにより、コールバックの複雑さを避けつつ、効率的な並行処理を実現できます。

## 10.5 コレクション操作

標準ライブラリは、コレクションに対する豊富な操作を提供します。これらは多くの場合、対応する型のメソッドとして実装されるか、コンパニオンモジュール内の関数として提供されます。

```protorun
// リスト操作 (List モジュール内または List<T> のメソッドとして)
module List {
  // 変換操作
  export let map = fn <T, U>(list: List<T>, f: (T) -> U): List<U> => { /* ... */ }
  export let flatMap = fn <T, U>(list: List<T>, f: (T) -> List<U>): List<U> => { /* ... */ }
  export let filter = fn <T>(list: List<T>, predicate: (T) -> Bool): List<T> => { /* ... */ }

  // 集約操作
  export let fold = fn <T, U>(list: List<T>, initial: U, f: (U, T) -> U): U => { /* ... */ }
  export let reduce = fn <T>(list: List<T>, f: (T, T) -> T): Option<T> => { /* ... */ }
  export let sum = fn <T: Num>(list: List<T>): T => { /* ... */ } // Num トレイトが必要

  // 検索操作
  export let find = fn <T>(list: List<T>, predicate: (T) -> Bool): Option<T> => { /* ... */ }
  export let contains = fn <T: Eq>(list: List<T>, value: T): Bool => { /* ... */ } // Eq トレイトが必要
  export let indexOf = fn <T: Eq>(list: List<T>, value: T): Option<Int> => { /* ... */ } // Eq トレイトが必要

  // 構造操作
  export let append = fn <T>(list1: List<T>, list2: List<T>): List<T> => { /* ... */ }
  export let reverse = fn <T>(list: List<T>): List<T> => { /* ... */ }
  export let take = fn <T>(list: List<T>, n: Int): List<T> => { /* ... */ }
  export let drop = fn <T>(list: List<T>, n: Int): List<T> => { /* ... */ }
}

// マップ操作 (Map モジュール内または Map<K, V> のメソッドとして)
module Map {
  // 変換操作
  export let mapValues = fn <K, V, W>(map: Map<K, V>, f: (V) -> W): Map<K, W> => { /* ... */ }
  export let filterKeys = fn <K, V>(map: Map<K, V>, predicate: (K) -> Bool): Map<K, V> => { /* ... */ }
  export let filterValues = fn <K, V>(map: Map<K, V>, predicate: (V) -> Bool): Map<K, V> => { /* ... */ }

  // 集約操作
  export let foldEntries = fn <K, V, U>(map: Map<K, V>, initial: U, f: (U, K, V) -> U): U => { /* ... */ }

  // 構造操作
  export let merge = fn <K, V>(map1: Map<K, V>, map2: Map<K, V>): Map<K, V> => { /* ... */ } // 衝突解決戦略が必要な場合がある
  export let withDefault = fn <K, V>(map: Map<K, V>, defaultValue: V): ((K) -> V) => { /* ... */ }
}

// セット操作 (Set モジュール内または Set<T> のメソッドとして)
module Set {
  // 集合演算
  export let union = fn <T>(set1: Set<T>, set2: Set<T>): Set<T> => { /* ... */ }
  export let intersection = fn <T>(set1: Set<T>, set2: Set<T>): Set<T> => { /* ... */ }
  export let difference = fn <T>(set1: Set<T>, set2: Set<T>): Set<T> => { /* ... */ }
  export let symmetricDifference = fn <T>(set1: Set<T>, set2: Set<T>): Set<T> => { /* ... */ }

  // 検査操作
  export let isSubset = fn <T>(set1: Set<T>, set2: Set<T>): Bool => { /* ... */ }
  export let isSuperset = fn <T>(set1: Set<T>, set2: Set<T>): Bool => { /* ... */ }
  export let isDisjoint = fn <T>(set1: Set<T>, set2: Set<T>): Bool => { /* ... */ }
}
```

これらのコレクション操作は、関数型プログラミングのパターンに基づいており、データの変換と操作を簡潔かつ表現力豊かに行うことができます。

## 10.6 数値計算

標準ライブラリは、数値計算のための機能を提供します。

```protorun
// 数値型クラス (トレイト)
let Num = trait<T> {
  fn add(self, other: T): T
  fn subtract(self, other: T): T
  fn multiply(self, other: T): T
  fn divide(self, other: T): T
  fn negate(self): T
  fn abs(self): T
  fn zero(): T
  fn one(): T
}

// 数学関数 (Math モジュール)
module Math {
  // 定数
  export let PI: Float = 3.14159265358979323846
  export let E: Float = 2.71828182845904523536

  // 基本関数
  export let sqrt = fn (x: Float): Float => { /* ... */ }
  export let pow = fn (x: Float, y: Float): Float => { /* ... */ }
  export let exp = fn (x: Float): Float => { /* ... */ }
  export let log = fn (x: Float): Float => { /* ... */ }
  export let log10 = fn (x: Float): Float => { /* ... */ }

  // 三角関数
  export let sin = fn (x: Float): Float => { /* ... */ }
  export let cos = fn (x: Float): Float => { /* ... */ }
  export let tan = fn (x: Float): Float => { /* ... */ }
  export let asin = fn (x: Float): Float => { /* ... */ }
  export let acos = fn (x: Float): Float => { /* ... */ }
  export let atan = fn (x: Float): Float => { /* ... */ }
  export let atan2 = fn (y: Float, x: Float): Float => { /* ... */ }

  // 丸め関数
  export let floor = fn (x: Float): Float => { /* ... */ }
  export let ceil = fn (x: Float): Float => { /* ... */ }
  export let round = fn (x: Float): Float => { /* ... */ }
  export let truncate = fn (x: Float): Float => { /* ... */ }
}
```

数値計算機能は、科学計算や数学的操作を効率的に行うための基盤を提供します。

## 10.7 文字列操作

標準ライブラリは、文字列操作のための豊富な機能を提供します。これらは `String` 型のメソッドまたは `String` モジュール内の関数として提供される可能性があります。

```protorun
// 文字列操作 (String モジュールまたは String メソッド)
module String {
  // 基本操作
  export let length = fn (s: String): Int => { /* ... */ }
  export let isEmpty = fn (s: String): Bool => { /* ... */ }
  export let charAt = fn (s: String, index: Int): Option<Char> => { /* ... */ }
  export let substring = fn (s: String, start: Int, end: Int): String => { /* ... */ }

  // 変換操作
  export let toUpperCase = fn (s: String): String => { /* ... */ }
  export let toLowerCase = fn (s: String): String => { /* ... */ }
  export let trim = fn (s: String): String => { /* ... */ }
  export let replace = fn (s: String, oldStr: String, newStr: String): String => { /* ... */ }

  // 検索操作
  export let contains = fn (s: String, substr: String): Bool => { /* ... */ }
  export let startsWith = fn (s: String, prefix: String): Bool => { /* ... */ }
  export let endsWith = fn (s: String, suffix: String): Bool => { /* ... */ }
  export let indexOf = fn (s: String, substr: String): Option<Int> => { /* ... */ }
  export let lastIndexOf = fn (s: String, substr: String): Option<Int> => { /* ... */ }

  // 分割と結合
  export let split = fn (s: String, delimiter: String): List<String> => { /* ... */ }
  export let join = fn (strings: List<String>, delimiter: String): String => { /* ... */ }

  // 文字列補間 (言語機能として提供される可能性あり)
  // export let format = fn (template: String, args: Map<String, String>): String => { /* ... */ }
}
```

文字列操作機能は、テキスト処理やユーザーインターフェースの構築に不可欠です。

## 10.8 標準ライブラリの設計上の考慮事項

標準ライブラリを設計する際には、以下の点を考慮することが重要です：

1. **一貫性**: APIは一貫した命名規則とパターンに従うべきです。
2. **効率性**: 頻繁に使用される操作は効率的に実装すべきです。
3. **型安全性**: APIは型安全性を保証し、コンパイル時にエラーを検出すべきです。
4. **拡張性**: ライブラリは将来の拡張を考慮して設計すべきです。
5. **使いやすさ**: APIは直感的で使いやすく、十分なドキュメントを提供すべきです。

標準ライブラリは、言語の使いやすさと表現力に大きく影響します。適切に設計された標準ライブラリは、開発者の生産性を向上させ、安全で効率的なプログラムの作成を支援します。
