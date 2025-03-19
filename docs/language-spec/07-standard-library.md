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
sealed trait List<T>
object List {
  case class Cons<T>(head: T, tail: List<T>) extends List<T>
  case object Nil extends List<Nothing>
  
  // リスト操作
  pub fn empty<T>(): List<T> = Nil
  pub fn cons<T>(head: T, tail: List<T>): List<T> = Cons(head, tail)
  pub fn map<T, U>(list: List<T>, f: (T) -> U): List<U> = match list {
    Nil => Nil,
    Cons(head, tail) => Cons(f(head), map(tail, f))
  }
  pub fn filter<T>(list: List<T>, predicate: (T) -> Bool): List<T> = match list {
    Nil => Nil,
    Cons(head, tail) => if predicate(head) {
      Cons(head, filter(tail, predicate))
    } else {
      filter(tail, predicate)
    }
  }
}

// マップ
trait Map<K, V> {
  fn get(key: K): Option<V>
  fn put(key: K, value: V): Map<K, V>
  fn remove(key: K): Map<K, V>
  fn contains(key: K): Bool
  fn keys(): List<K>
  fn values(): List<V>
  fn entries(): List<(K, V)>
  fn isEmpty(): Bool
  fn size(): Int
}

// セット
trait Set<T> {
  fn contains(value: T): Bool
  fn add(value: T): Set<T>
  fn remove(value: T): Set<T>
  fn union(other: Set<T>): Set<T>
  fn intersection(other: Set<T>): Set<T>
  fn difference(other: Set<T>): Set<T>
  fn isEmpty(): Bool
  fn size(): Int
  fn toList(): List<T>
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
```

これらのコアデータ構造は、型安全で効率的なプログラミングを可能にします。各データ構造は、関連する操作と共に提供され、関数型プログラミングのパターンをサポートします。

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
handler IOHandler for IO {
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
fn processFile(path: String): Result<String, IOError> with IO = {
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

I/O操作は代数的効果として実装され、型安全な方法で副作用を制御できます。これにより、純粋な関数と副作用を持つコードを明確に分離できます。

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
handler AsyncHandler for Async {
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
fn fetchData(url: String): Result<String, NetworkError> with Async & IO = {
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
  pub fn map<T, U>(list: List<T>, f: (T) -> U): List<U>
  pub fn flatMap<T, U>(list: List<T>, f: (T) -> List<U>): List<U>
  pub fn filter<T>(list: List<T>, predicate: (T) -> Bool): List<T>
  
  // 集約操作
  pub fn fold<T, U>(list: List<T>, initial: U, f: (U, T) -> U): U
  pub fn reduce<T>(list: List<T>, f: (T, T) -> T): Option<T>
  pub fn sum<T: Num>(list: List<T>): T
  
  // 検索操作
  pub fn find<T>(list: List<T>, predicate: (T) -> Bool): Option<T>
  pub fn contains<T: Eq>(list: List<T>, value: T): Bool
  pub fn indexOf<T: Eq>(list: List<T>, value: T): Option<Int>
  
  // 構造操作
  pub fn append<T>(list1: List<T>, list2: List<T>): List<T>
  pub fn reverse<T>(list: List<T>): List<T>
  pub fn take<T>(list: List<T>, n: Int): List<T>
  pub fn drop<T>(list: List<T>, n: Int): List<T>
}

// マップ操作
module Map {
  // 変換操作
  pub fn mapValues<K, V, W>(map: Map<K, V>, f: (V) -> W): Map<K, W>
  pub fn filterKeys<K, V>(map: Map<K, V>, predicate: (K) -> Bool): Map<K, V>
  pub fn filterValues<K, V>(map: Map<K, V>, predicate: (V) -> Bool): Map<K, V>
  
  // 集約操作
  pub fn foldEntries<K, V, U>(map: Map<K, V>, initial: U, f: (U, K, V) -> U): U
  
  // 構造操作
  pub fn merge<K, V>(map1: Map<K, V>, map2: Map<K, V>): Map<K, V>
  pub fn withDefault<K, V>(map: Map<K, V>, defaultValue: V): (K) -> V
}

// セット操作
module Set {
  // 集合演算
  pub fn union<T>(set1: Set<T>, set2: Set<T>): Set<T>
  pub fn intersection<T>(set1: Set<T>, set2: Set<T>): Set<T>
  pub fn difference<T>(set1: Set<T>, set2: Set<T>): Set<T>
  pub fn symmetricDifference<T>(set1: Set<T>, set2: Set<T>): Set<T>
  
  // 検査操作
  pub fn isSubset<T>(set1: Set<T>, set2: Set<T>): Bool
  pub fn isSuperset<T>(set1: Set<T>, set2: Set<T>): Bool
  pub fn isDisjoint<T>(set1: Set<T>, set2: Set<T>): Bool
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
  pub const PI: Float = 3.14159265358979323846
  pub const E: Float = 2.71828182845904523536
  
  // 基本関数
  pub fn sqrt(x: Float): Float
  pub fn pow(x: Float, y: Float): Float
  pub fn exp(x: Float): Float
  pub fn log(x: Float): Float
  pub fn log10(x: Float): Float
  
  // 三角関数
  pub fn sin(x: Float): Float
  pub fn cos(x: Float): Float
  pub fn tan(x: Float): Float
  pub fn asin(x: Float): Float
  pub fn acos(x: Float): Float
  pub fn atan(x: Float): Float
  pub fn atan2(y: Float, x: Float): Float
  
  // 丸め関数
  pub fn floor(x: Float): Float
  pub fn ceil(x: Float): Float
  pub fn round(x: Float): Float
  pub fn truncate(x: Float): Float
}
```

数値計算機能は、科学計算や数学的操作を効率的に行うための基盤を提供します。

## 7.7 文字列操作

標準ライブラリは、文字列操作のための豊富な機能を提供します。

```
// 文字列操作
module String {
  // 基本操作
  pub fn length(s: String): Int
  pub fn isEmpty(s: String): Bool
  pub fn charAt(s: String, index: Int): Option<Char>
  pub fn substring(s: String, start: Int, end: Int): String
  
  // 変換操作
  pub fn toUpperCase(s: String): String
  pub fn toLowerCase(s: String): String
  pub fn trim(s: String): String
  pub fn replace(s: String, oldStr: String, newStr: String): String
  
  // 検索操作
  pub fn contains(s: String, substr: String): Bool
  pub fn startsWith(s: String, prefix: String): Bool
  pub fn endsWith(s: String, suffix: String): Bool
  pub fn indexOf(s: String, substr: String): Option<Int>
  pub fn lastIndexOf(s: String, substr: String): Option<Int>
  
  // 分割と結合
  pub fn split(s: String, delimiter: String): List<String>
  pub fn join(strings: List<String>, delimiter: String): String
  
  // 文字列補間
  pub fn format(template: String, args: Map<String, String>): String
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
