# 7. モジュールシステム

## 7.1 モジュール定義

```
module Math {
  // 公開関数
  pub fn add(a: Int, b: Int): Int = a + b
  
  // 非公開関数
  fn helper(): Int = 42
}
```

## 7.2 インポート

```
// モジュールのインポート
import Math
import Math.add
import Math.{add, subtract}
import Math.*
```

# 8. 標準ライブラリ

## 8.1 コアデータ構造

```
// リスト
sealed trait List<T>
object List {
  case class Cons<T>(head: T, tail: List<T>) extends List<T>
  case object Nil extends List<Nothing>
}

// マップ
trait Map<K, V> {
  fn get(key: K): Option<V>
  fn put(key: K, value: V): Map<K, V>
  fn remove(key: K): Map<K, V>
}

// セット
trait Set<T> {
  fn contains(value: T): Bool
  fn add(value: T): Set<T>
  fn remove(value: T): Set<T>
}
```

## 8.2 I/O操作

```
effect IO {
  fn readFile(path: String): Result<String, IOError>
  fn writeFile(path: String, content: String): Result<Unit, IOError>
  fn println(message: String): Unit
  fn readLine(): String
}
```

## 8.3 並行処理

```
effect Async {
  fn spawn<T>(task: () -> T): Task<T>
  fn await<T>(task: Task<T>): T
  fn sleep(duration: Duration): Unit
}
