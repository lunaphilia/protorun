# 3. 型システム

## 3.1 基本型

```
Int, Float, Double, Bool, Char, String, Unit
```

すべての型はnon-nullableです。nullは言語から排除されています。

## 3.2 複合型

```
// 配列型
[T]

// タプル型
(T1, T2, ..., Tn)

// 関数型
(T1, T2, ..., Tn) -> R
(T1, T2, ..., Tn) -> R with E

// オプション型（値の存在/不在を表現）
Option<T>

// 結果型（成功/失敗を表現）
Result<T, E>
```

## 3.3 型定義

```
// レコード型（構造体）
type Person = {
  name: String,
  age: Int
}

// 代数的データ型（enum構文）
enum Option<T> {
  Some(T),
  None
}

enum Result<T, E> {
  Ok(T),
  Err(E)
}

// 型エイリアス
type StringMap<T> = Map<String, T>
```

## 3.4 トレイト（インターフェース）

```
// トレイト定義
trait Show {
  fn show(self): String
}

// トレイト継承（単一継承のみ）
trait Eq {
  fn equals(self, other: Self): Bool
}

trait Ord extends Eq {
  fn compare(self, other: Self): Int
  
  // デフォルト実装
  fn equals(self, other: Self): Bool = self.compare(other) == 0
}

// トレイト実装
impl Show for Int {
  fn show(self): String = self.toString()
}

// ジェネリックな実装
impl<T: Show> Show for Option<T> {
  fn show(self): String = match self {
    Option.Some(v) => s"Some(${v.show()})",
    Option.None => "None"
  }
}
```

## 3.5 リソース型

```
// 管理型の定義（旧リソース型）
managed type File {
  // 内部フィールド
  handle: FileHandle,
  path: String,
  
  // 獲得関数（コンストラクタ）
  fn open(path: String): Result<File, IOError> = {
    // ファイルを開く実装
  }
  
  // 解放関数（デストラクタ）
  fn close(self): Unit = {
    // ファイルを閉じる実装
  }
  
  // 通常のメソッド
  fn read(self: &Self): Result<String, IOError> = {
    // ファイルを読み込む実装
  }
  
  fn write(self: &mut Self, content: String): Result<Unit, IOError> = {
    // ファイルに書き込む実装
  }
}
```

## 3.6 リソースパターン型クラス

```
// 管理リソースパターンの型クラス
trait Resource<R> {
  // リソース獲得
  fn open<E>(acquireFn: () -> Result<R, E>): Result<R, E>
  
  // リソース解放
  fn cleanup(resource: R): Unit
  
  // リソース使用
  fn use<T>(resource: &R, action: (r: &R) -> T): T
  
  // リソース変換
  fn map<S>(resource: R, f: (R) -> S): S with Resource<S>
}

// 管理型に対する自動実装
impl<R: managed type> Resource<R> {
  // デフォルト実装
  fn open<E>(acquireFn: () -> Result<R, E>): Result<R, E> = acquireFn()
  
  fn cleanup(resource: R): Unit = resource.close()
  
  fn use<T>(resource: &R, action: (r: &R) -> T): T = action(resource)
  
  fn map<S: managed type>(resource: R, f: (R) -> S): S = {
    let result = f(resource)
    // resourceは自動的に解放される
    result
  }
}

// 使用例
fn withResource<R, T, E>(
  acquireFn: () -> Result<R, E>,
  action: (r: &R) -> T
): Result<T, E> with Resource<R> = {
  let resource = Resource.open(acquireFn)?
  try {
    let result = Resource.use(&resource, action)
    Resource.cleanup(resource)
    Result.Ok(result)
  } catch (e) {
    Resource.cleanup(resource)
    throw e
  }
}
