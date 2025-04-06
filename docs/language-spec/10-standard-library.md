# 7. 標準ライブラリ

## 7.1 標準ライブラリの概念と目的

標準ライブラリは、Protorun言語に組み込まれた基本的な機能とデータ構造を提供するコンポーネントです。この標準ライブラリは以下の目的で設計されています：

1. **基本機能の提供**: 一般的なプログラミングタスクに必要な基本的な機能を提供します。
2. **一貫性の確保**: 言語全体で一貫したAPIとデータ構造を提供します。
3. **効率性の最適化**: 頻繁に使用される操作を効率的に実装します。
4. **安全性の保証**: 型安全性とメモリ安全性を保証する方法でAPIを設計します。
5. **拡張性の確保**: ユーザーが独自の機能を追加できるように拡張可能な設計を提供します。

標準ライブラリは、言語の核となる部分であり、すべてのProtorunプログラムで利用可能です。これにより、開発者は基本的な機能を再実装する必要なく、より高レベルの問題に集中できます。

## 7.2 コアデータ構造

標準ライブラリは、プログラミングに不可欠な基本的なデータ構造を提供します。

```
// リスト
enum List<T> {
  Cons(head: T, tail: List<T>),
  Nil
}

// List モジュール (コンパニオンオブジェクト的な役割)
module List {
  // リスト操作 (例)
  // 注意: 以下の関数定義は例であり、実際の標準ライブラリの実装とは異なる場合があります。
  // また、List<T> のメソッドとして実装される可能性もあります。
  export fn empty<T>(): List<T> = List.Nil
  export fn cons<T>(head: T, tail: List<T>): List<T> = List.Cons(head, tail)
  // export fn append<T>(list1: List<T>, list2: List<T>): List<T> // 例: appendの実装は省略
  // map や filter などの高階関数は、List<T> 型のメソッドとして
  // または List モジュール内の関数として提供される可能性があります。
}

// マップ (トレイトとして定義 - 実装は標準ライブラリで提供)
trait Map<K, V> {
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
  export fn empty<K, V>(): Map<K, V>
}


// セット (トレイトとして定義 - 実装は標準ライブラリで提供)
trait Set<T> {
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
  export fn empty<T>(): Set<T>
}

// オプション型
enum Option<T> {
  Some(T),
  None
}

// 結果型
enum Result<T, E> {
  Ok(T),
  Err(E)
}

// Result型の拡張メソッド (implブロックを使用)
impl<T, E> Result<T, E> {
  // 既存のメソッド
  fn map<U>(self, f: (T) -> U): Result<U, E> = match self {
    Result.Ok(value) => Result.Ok(f(value)),
    Result.Err(error) => Result.Err(error)
  }

  fn flatMap<U>(self, f: (T) -> Result<U, E>): Result<U, E> = match self {
    Result.Ok(value) => f(value),
    Result.Err(error) => Result.Err(error)
  }

  // 新しいユーティリティメソッド
  fn mapErr<F>(self, f: (E) -> F): Result<T, F> = match self {
    Result.Ok(value) => Result.Ok(value),
    Result.Err(error) => Result.Err(f(error))
  }

  fn flatMapErr<F>(self, f: (E) -> Result<T, F>): Result<T, F> = match self {
    Result.Ok(value) => Result.Ok(value),
    Result.Err(error) => f(error)
  }

  // 注意: panic は言語のコア機能として別途定義される必要があります
  fn unwrap(self): T = match self {
    Result.Ok(value) => value,
    Result.Err(_) => panic("Result.unwrap called on an Err value")
  }

  fn unwrapOr(self, default: T): T = match self {
    Result.Ok(value) => value,
    Result.Err(_) => default
  }

  fn unwrapOrElse(self, f: (E) -> T): T = match self {
    Result.Ok(value) => value,
    Result.Err(error) => f(error)
  }

  fn isOk(self): Bool = match self {
    Result.Ok(_) => true,
    Result.Err(_) => false
  }

  fn isErr(self): Bool = match self {
    Result.Ok(_) => false,
    Result.Err(_) => true
  }

  // 注意: panic は言語のコア機能として別途定義される必要があります
  fn unwrapErr(self): E = match self {
    Result.Ok(_) => panic("Result.unwrapErr called on an Ok value"),
    Result.Err(error) => error
  }
}

// Result モジュール (ユーティリティ関数)
module Result {
  // 複数のResultを結合する (実装は List の機能に依存するため、シグネチャのみ示す)
  export fn all<T, E>(results: List<Result<T, E>>): Result<List<T>, E>
  // {
  //   // 実装例 (List.fold, List.reverse, List.cons が必要)
  //   results.fold(Result.Ok(List.empty<T>()), (accResult, currentResult) => {
  //     bind {
  //       acc <- accResult
  //       current <- currentResult
  //       Result.Ok(List.cons(current, acc)) // 逆順になるので最後に reverse が必要
  //     }
  //   }).map(list => list.reverse()) // List.reverse が必要
  // }

  // 最初に成功したResultを返す (実装は List の機能に依存するため、シグネチャのみ示す)
  export fn any<T, E>(results: List<Result<T, E>>): Result<T, List<E>>
  // {
  //   // 実装例 (List.fold, List.reverse, List.cons が必要)
  //   results.fold(Result.Err(List.empty<E>()), (accResult, currentResult) => {
  //     match (accResult, currentResult) {
  //       (Result.Ok(v), _) => Result.Ok(v), // 既に成功が見つかっていればそれを返す
  //       (_, Result.Ok(v)) => Result.Ok(v), // 現在のものが成功ならそれを返す
  //       (Result.Err(errs), Result.Err(err)) => Result.Err(List.cons(err, errs)), // 両方エラーならエラーリストに追加 (逆順)
  //     }
  //   }).mapErr(errs => errs.reverse()) // List.reverse が必要
  // }
}
```

```
## 7.3 I/O操作

標準ライブラリは、ファイルシステムやコンソールとの対話など、I/O操作のための効果を提供します。

```
// I/O効果
effect IO {
  // ファイル操作
  fn readFile(path: String): Result<String, IOError>
  fn writeFile(path: String, content: String): Result<Unit, IOError>
  fn fileExists(path: String): Bool
  fn deleteFile(path: String): Result<Unit, IOError>

  // コンソール操作
  fn println(message: String): Unit
  fn print(message: String): Unit
  fn readLine(): String

  // 環境変数
  fn getEnv(name: String): Option<String>
  fn setEnv(name: String, value: String): Result<Unit, IOError>
}

// I/O効果のハンドラ
handler IOHandler: IO { // "for" を ":" に修正
  // ファイル操作の実装
  fn readFile(path: String): Result<String, IOError> = {
    // プラットフォーム固有の実装
  }

  fn writeFile(path: String, content: String): Result<Unit, IOError> = {
    // プラットフォーム固有の実装
  }

  // コンソール操作の実装
  fn println(message: String): Unit = {
    // プラットフォーム固有の実装
  }

  fn readLine(): String = {
    // プラットフォーム固有の実装
  }

  // 他のI/O操作の実装...
}

// 使用例
fn processFile(path: String): Result<String, IOError> & IO = { // "with" を "&" に修正 (効果の宣言)
  if IO.fileExists(path) {
    let content = IO.readFile(path)?
    let processed = processContent(content)
    IO.writeFile(path + ".processed", processed)?
    IO.println(s"ファイル $path を処理しました")
    Result.Ok(processed)
  } else {
    IO.println(s"ファイル $path が見つかりません")
    Result.Err(IOError.FileNotFound(path))
  }
}
```

## 7.4 並行処理

標準ライブラリは、並行処理と非同期プログラミングのためのサポートを提供します。

```
// 非同期効果
effect Async {
  // タスク管理
  fn spawn<T>(task: () -> T): Task<T>
  fn await<T>(task: Task<T>): T
  fn sleep(duration: Duration): Unit

  // 並行制御
  fn withTimeout<T>(duration: Duration, task: () -> T): Result<T, TimeoutError>
  fn race<T>(tasks: List<() -> T>): T
  fn all<T>(tasks: List<() -> T>): List<T>
}

// 非同期効果のハンドラ
handler AsyncHandler: Async { // "for" を ":" に修正
  // タスク管理の実装
  fn spawn<T>(task: () -> T, resume: (Task<T>) -> Unit): Unit = {
    // プラットフォーム固有の実装
    let taskHandle = createTask(task)
    resume(taskHandle)
  }

  fn await<T>(task: Task<T>, resume: (T) -> Unit): Unit = {
    // プラットフォーム固有の実装
    task.onComplete(result => resume(result))
  }

  // 他の非同期操作の実装...
}

// 使用例
fn fetchData(url: String): Result<String, NetworkError> & Async & IO = { // "with" を "&" に修正
  IO.println(s"データを取得中: $url")

  let task = Async.spawn(() => {
    // ネットワークリクエストの実行
    networkRequest(url)
  })

  // タイムアウト付きで結果を待機
  Async.withTimeout(Duration.ofSeconds(10), () => {
    let result = Async.await(task)?
    IO.println(s"データ取得完了: ${result.length} バイト")
    Result.Ok(result)
  }) match {
    Result.Ok(data) => Result.Ok(data),
    Result.Err(_) => {
      IO.println("タイムアウトしました")
      Result.Err(NetworkError.Timeout)
    }
  }
}
```

並行処理は代数的効果として実装され、非同期プログラミングを型安全かつ直感的に行うことができます。これにより、コールバックの複雑さを避けつつ、効率的な並行処理を実現できます。

## 7.5 コレクション操作

標準ライブラリは、コレクションに対する豊富な操作を提供します。

```
// リスト操作
module List {
  // 変換操作
  export fn map<T, U>(list: List<T>, f: (T) -> U): List<U>
  export fn flatMap<T, U>(list: List<T>, f: (T) -> List<U>): List<U>
  export fn filter<T>(list: List<T>, predicate: (T) -> Bool): List<T>

  // 集約操作
  export fn fold<T, U>(list: List<T>, initial: U, f: (U, T) -> U): U
  export fn reduce<T>(list: List<T>, f: (T, T) -> T): Option<T>
  export fn sum<T: Num>(list: List<T>): T // Num トレイトが必要

  // 検索操作
  export fn find<T>(list: List<T>, predicate: (T) -> Bool): Option<T>
  export fn contains<T: Eq>(list: List<T>, value: T): Bool // Eq トレイトが必要
  export fn indexOf<T: Eq>(list: List<T>, value: T): Option<Int> // Eq トレイトが必要

  // 構造操作
  export fn append<T>(list1: List<T>, list2: List<T>): List<T>
  export fn reverse<T>(list: List<T>): List<T>
  export fn take<T>(list: List<T>, n: Int): List<T>
  export fn drop<T>(list: List<T>, n: Int): List<T>
}

// マップ操作
module Map {
  // 変換操作
  export fn mapValues<K, V, W>(map: Map<K, V>, f: (V) -> W): Map<K, W>
  export fn filterKeys<K, V>(map: Map<K, V>, predicate: (K) -> Bool): Map<K, V>
  export fn filterValues<K, V>(map: Map<K, V>, predicate: (V) -> Bool): Map<K, V>

  // 集約操作
  export fn foldEntries<K, V, U>(map: Map<K, V>, initial: U, f: (U, K, V) -> U): U

  // 構造操作
  export fn merge<K, V>(map1: Map<K, V>, map2: Map<K, V>): Map<K, V> // 衝突解決戦略が必要な場合がある
  export fn withDefault<K, V>(map: Map<K, V>, defaultValue: V): (K) -> V
}

// セット操作
module Set {
  // 集合演算
  export fn union<T>(set1: Set<T>, set2: Set<T>): Set<T>
  export fn intersection<T>(set1: Set<T>, set2: Set<T>): Set<T>
  export fn difference<T>(set1: Set<T>, set2: Set<T>): Set<T>
  export fn symmetricDifference<T>(set1: Set<T>, set2: Set<T>): Set<T>

  // 検査操作
  export fn isSubset<T>(set1: Set<T>, set2: Set<T>): Bool
  export fn isSuperset<T>(set1: Set<T>, set2: Set<T>): Bool
  export fn isDisjoint<T>(set1: Set<T>, set2: Set<T>): Bool
}
```

これらのコレクション操作は、関数型プログラミングのパターンに基づいており、データの変換と操作を簡潔かつ表現力豊かに行うことができます。

## 7.6 数値計算

標準ライブラリは、数値計算のための機能を提供します。

```
// 数値型クラス
trait Num<T> {
  fn add(self, other: T): T
  fn subtract(self, other: T): T
  fn multiply(self, other: T): T
  fn divide(self, other: T): T
  fn negate(self): T
  fn abs(self): T
  fn zero(): T
  fn one(): T
}

// 数学関数
module Math {
  // 定数
  export const PI: Float = 3.14159265358979323846
  export const E: Float = 2.71828182845904523536

  // 基本関数
  export fn sqrt(x: Float): Float
  export fn pow(x: Float, y: Float): Float
  export fn exp(x: Float): Float
  export fn log(x: Float): Float
  export fn log10(x: Float): Float

  // 三角関数
  export fn sin(x: Float): Float
  export fn cos(x: Float): Float
  export fn tan(x: Float): Float
  export fn asin(x: Float): Float
  export fn acos(x: Float): Float
  export fn atan(x: Float): Float
  export fn atan2(y: Float, x: Float): Float

  // 丸め関数
  export fn floor(x: Float): Float
  export fn ceil(x: Float): Float
  export fn round(x: Float): Float
  export fn truncate(x: Float): Float
}
```

数値計算機能は、科学計算や数学的操作を効率的に行うための基盤を提供します。

## 7.7 文字列操作

標準ライブラリは、文字列操作のための豊富な機能を提供します。

```
// 文字列操作
module String {
  // 基本操作
  export fn length(s: String): Int
  export fn isEmpty(s: String): Bool
  export fn charAt(s: String, index: Int): Option<Char>
  export fn substring(s: String, start: Int, end: Int): String

  // 変換操作
  export fn toUpperCase(s: String): String
  export fn toLowerCase(s: String): String
  export fn trim(s: String): String
  export fn replace(s: String, oldStr: String, newStr: String): String

  // 検索操作
  export fn contains(s: String, substr: String): Bool
  export fn startsWith(s: String, prefix: String): Bool
  export fn endsWith(s: String, suffix: String): Bool
  export fn indexOf(s: String, substr: String): Option<Int>
  export fn lastIndexOf(s: String, substr: String): Option<Int>

  // 分割と結合
  export fn split(s: String, delimiter: String): List<String>
  export fn join(strings: List<String>, delimiter: String): String

  // 文字列補間 (言語機能として提供される可能性あり)
  // export fn format(template: String, args: Map<String, String>): String
}
```

文字列操作機能は、テキスト処理やユーザーインターフェースの構築に不可欠です。

## 7.8 標準ライブラリの設計上の考慮事項

標準ライブラリを設計する際には、以下の点を考慮することが重要です：

1. **一貫性**: APIは一貫した命名規則とパターンに従うべきです。
2. **効率性**: 頻繁に使用される操作は効率的に実装すべきです。
3. **型安全性**: APIは型安全性を保証し、コンパイル時にエラーを検出すべきです。
4. **拡張性**: ライブラリは将来の拡張を考慮して設計すべきです。
5. **使いやすさ**: APIは直感的で使いやすく、十分なドキュメントを提供すべきです。

標準ライブラリは、言語の使いやすさと表現力に大きく影響します。適切に設計された標準ライブラリは、開発者の生産性を向上させ、安全で効率的なプログラムの作成を支援します。
