# 3. 型システム

## 3.1 型システムの概要と目的

Protorun言語の型システムは、静的型付け、強力な型推論、代数的データ型、トレイト（型クラス）、および所有権を組み合わせた包括的なシステムです。この型システムは以下の目的で設計されています：

1. **安全性**: コンパイル時に型エラーを検出し、実行時エラーを防止します
2. **表現力**: 複雑な概念を型安全に表現するための豊富な機能を提供します
3. **抽象化**: コードの再利用と拡張を促進する抽象化メカニズムを提供します
4. **効率性**: 型消去と特殊化により、効率的なコード生成を可能にします
5. **推論可能性**: 明示的な型注釈を最小限に抑えつつ、型安全性を確保します

Protorun言語の型システムは、Hindley-Milner型推論をベースに、効果型、所有権型、トレイト制約を統合した拡張システムです。これにより、表現力と安全性のバランスを取りながら、使いやすさを実現しています。

## 3.2 基本型

```
Int, Float, Double, Bool, Char, String, Unit
```

Protorun言語の基本型は、以下の原則に基づいて設計されています：

1. **完全性**: プログラミングに必要な基本的なデータ型をすべて提供します
2. **一貫性**: 各型は明確な意味と操作を持ち、予測可能な動作をします
3. **安全性**: すべての型はnon-nullableであり、nullによる実行時エラーを防止します

特に、nullの排除は重要な設計決定です。これは、Tony Hoareが「10億ドルの間違い」と呼んだnullポインタ例外を防ぐためです。値の存在/不在はOption型で明示的に表現され、型システムによって安全に処理されます。これにより、実行時のnullチェックの必要性が減少し、コードの信頼性が向上します。

## 3.3 複合型

```
// 配列型
[T]

// タプル型
(T1, T2, ..., Tn)

// 関数型
(T1, T2, ..., Tn) -> R
(T1, T2, ..., Tn) -> R & E

// オプション型（値の存在/不在を表現）
Option<T>

// 結果型（成功/失敗を表現）
Result<T, E>
```

Protorun言語の複合型は、以下の原則に基づいて設計されています：

1. **合成性**: 基本型から複雑な型を構築するための明確なメカニズムを提供します
2. **型安全性**: 複合型の操作は型チェックされ、型の整合性が保証されます
3. **表現力**: 様々なデータ構造と計算パターンを表現するための豊富な型を提供します

特に注目すべき点：

- **関数型の効果注釈**: 関数型には効果注釈（`& E`）を含めることができます。これにより、関数が持つ可能性のある副作用を型レベルで追跡し、型安全な効果制御を実現します。

- **Option型とResult型**: nullの代わりにOption型を使用し、例外の代わりにResult型を使用することで、エラー処理を型安全かつ明示的に行うことができます。これにより、エラーハンドリングの漏れを防ぎ、コードの堅牢性が向上します。

## 3.4 型定義

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

Protorun言語の型定義機能は、以下の原則に基づいて設計されています：

1. **抽象化**: 複雑なデータ構造を抽象化し、意味のある名前で表現できます
2. **型安全性**: 定義された型は型チェックされ、型の整合性が保証されます
3. **表現力**: レコード型と代数的データ型により、様々なデータモデルを表現できます

特に重要な機能：

- **レコード型**: 名前付きフィールドを持つ構造体を定義できます。これにより、関連するデータをグループ化し、意味のある単位として扱うことができます。

- **代数的データ型（enum）**: 複数の異なるケースを持つデータ型を定義できます。これは、パターンマッチングと組み合わせて使用することで、型安全な条件分岐を実現します。代数的データ型は、ドメインモデルを正確に表現し、不正な状態を型レベルで排除するのに役立ちます。

- **型エイリアス**: 既存の型に新しい名前を付けることができます。これにより、コードの意図を明確にし、長い型名を簡潔に表現できます。

## 3.5 トレイト（インターフェース）

```
// トレイト定義
trait Show {
  fn show(self): String
}

// トレイト継承（単一継承のみ）
trait Eq {
  fn equals(self, other: Self): Bool
}

trait Ord: Eq {
  fn compare(self, other: Self): Int
  
  // デフォルト実装
  fn equals(self, other: Self): Bool = self.compare(other) == 0
}

// トレイト実装
impl Int: Show {
  fn show(self): String = self.toString()
}

// ジェネリックな実装
impl<T: Show> Option<T>: Show {
  fn show(self): String = match self {
    Option.Some(v) => s"Some(${v.show()})",
    Option.None => "None"
  }
}
```

Protorun言語のトレイトシステムは、以下の原則に基づいて設計されています：

1. **インターフェース抽象化**: 型の振る舞いを抽象化し、多相的なコードを書くことができます
2. **型安全な多相性**: トレイト制約により、ジェネリックコードの型安全性を保証します
3. **コード再利用**: デフォルト実装により、共通の機能を再利用できます
4. **単一継承**: 複雑さを抑えるために、トレイトの継承は単一継承のみをサポートします

トレイトシステムは、Haskellの型クラスとRustのトレイトからインスピレーションを得ています。特に：

- **単一継承の選択**: 多重継承の複雑さ（ダイヤモンド問題など）を避けるために、単一継承のみをサポートしています。これにより、型システムの複雑さを抑えつつ、必要な抽象化を実現しています。

- **デフォルト実装**: トレイトメソッドにデフォルト実装を提供できます。これにより、トレイトを実装する型の実装負担を軽減し、コードの再利用性を高めています。

- **ジェネリックな実装**: 型パラメータにトレイト制約を付けることで、型安全なジェネリックコードを書くことができます。これにより、型の抽象化と具体的な実装の分離が可能になります。

## 3.6 管理型（旧リソース型）

```
// 管理型の定義（旧リソース型）
managed File {
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

管理型（旧リソース型）は、以下の原則に基づいて設計されています：

1. **安全なリソース管理**: リソースの獲得と解放を自動的に管理し、リソースリークを防止します
2. **明示的なライフサイクル**: リソースのライフサイクル（獲得と解放）を明示的に定義します
3. **型安全性**: リソース操作の型安全性を保証し、不正な使用を防止します
4. **所有権との統合**: 所有権システムと統合し、リソースの安全な移動と借用を可能にします

管理型は、C++のRAIIパターンとRustの所有権システムからインスピレーションを得ています。特に：

- **自動リソース管理**: スコープベースのリソース管理により、リソースの解放忘れを防止します。これは、ガベージコレクションに依存せずに、予測可能なリソース管理を実現します。

- **明示的な解放関数**: 各管理型は明示的な解放関数（`close`など）を定義する必要があります。これにより、リソースの解放方法が明確になり、リソース管理の意図が明示的になります。

- **借用による安全な参照**: 所有権システムの借用メカニズムにより、リソースへの安全な参照が可能になります。これにより、リソースが使用中に解放されるといった問題を防止します。

管理型の名称は、以前は「リソース型」でしたが、その目的をより明確に表現するために「管理型」に変更されました。これは、この型がリソースそのものではなく、リソースの管理に焦点を当てていることを強調するためです。

### 3.6.1 管理型の暗黙的な使用

管理型は、明示的なパラメータとして使用するだけでなく、暗黙的なコンテキストとしても使用できます。これにより、依存性注入パターンを型安全かつ簡潔に実装できます。

```
// 管理型を暗黙的に使用する関数
fn processUserData(userId: String): Result<UserData, Error> with db: Database = {
  // dbが暗黙的に利用可能
  let userData = db.query(s"SELECT * FROM users WHERE id = $userId")?
  Result.Ok(parseUserData(userData))
}

// 使用例
fn main(): Result<Unit, Error> = {
  let db = Database.connect(config)?
  
  // with式で暗黙的なコンテキストを提供
  with db {
    // dbが暗黙的に利用可能になる
    let userData = processUserData("user123")?
    displayUserData(userData)
    
    Result.Ok(())
  } // dbは自動的に閉じられる
}
```

管理型の暗黙的な使用には以下の特徴があります：

1. **暗黙的な依存関係**: 関数は`with`キーワードを使用して必要な管理型を宣言します。
2. **スコープベースの提供**: `with`式を使用して管理型をスコープ内で暗黙的に提供します。
3. **自動リソース管理**: スコープ終了時に管理型の解放メソッドが自動的に呼び出されます。
4. **型安全性**: コンパイラは必要な管理型が提供されていることを静的に検証します。

複数の管理型を同時に使用することもできます：

```
// 複数の管理型を暗黙的に使用する関数
fn processUserOrder(userId: String, orderId: String): Result<OrderDetails, Error> with db: Database, logger: Logger, client: HttpClient = {
  logger.info(s"処理開始: ユーザー $userId, 注文 $orderId")
  
  let userData = db.query(s"SELECT * FROM users WHERE id = $userId")?
  let orderData = db.query(s"SELECT * FROM orders WHERE id = $orderId")?
  
  let additionalInfo = client.get(s"https://api.example.com/orders/$orderId/details")?
  
  logger.info("処理完了")
  Result.Ok(combineOrderDetails(userData, orderData, additionalInfo))
}

// 使用例
fn main(): Result<Unit, Error> = {
  let db = Database.connect(dbConfig)?
  let logger = Logger.init(logConfig)
  let client = HttpClient.create(httpConfig)
  
  // 複数の管理型を提供
  with db, logger, client {
    let orderDetails = processUserOrder("user123", "order456")?
    displayOrderDetails(orderDetails)
    
    Result.Ok(())
  } // すべてのリソースは自動的に解放される
}
```

この機能は、Scalaのimplicit parameterからインスピレーションを得ていますが、リソース管理の自動化と組み合わせることで、より安全で使いやすい依存性注入メカニズムを提供します。

## 3.7 リソースパターン型クラス

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
  fn map<S>(resource: R, f: (R) -> S): S & Resource<S>
}

// 管理型に対する自動実装
impl<R: managed> R: Resource<R> {
  // デフォルト実装
  fn open<E>(acquireFn: () -> Result<R, E>): Result<R, E> = acquireFn()
  
  fn cleanup(resource: R): Unit = resource.close()
  
  fn use<T>(resource: &R, action: (r: &R) -> T): T = action(resource)
  
  fn map<S: managed>(resource: R, f: (R) -> S): S = {
    let result = f(resource)
    // resourceは自動的に解放される
    result
  }
}

// 使用例
fn withResource<R, T, E>(
  acquireFn: () -> Result<R, E>,
  action: (r: &R) -> T
): Result<T, E> & Resource<R> = {
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
```

リソースパターン型クラスは、以下の原則に基づいて設計されています：

1. **抽象化**: リソース管理パターンを抽象化し、再利用可能にします
2. **型安全性**: リソース操作の型安全性を保証し、不正な使用を防止します
3. **合成可能性**: リソース操作を合成し、複雑なリソース管理パターンを構築できます
4. **自動実装**: 管理型に対して自動的に実装を提供し、ボイラープレートコードを削減します

リソースパターン型クラスは、関数型プログラミングのモナドパターンとオブジェクト指向プログラミングのテンプレートメソッドパターンからインスピレーションを得ています。特に：

- **パターンの抽象化**: リソースの獲得、使用、解放という一般的なパターンを抽象化し、再利用可能にしています。これにより、リソース管理のボイラープレートコードを削減し、コードの可読性と保守性を向上させます。

- **自動実装**: 管理型に対して自動的に`Resource`トレイトの実装を提供することで、開発者の負担を軽減しています。これにより、新しい管理型を定義する際に、リソース管理のコードを再実装する必要がなくなります。

- **合成可能性**: `map`メソッドにより、リソースの変換と合成が可能になります。これにより、複数のリソースを組み合わせて使用する複雑なパターンを表現できます。

リソースパターン型クラスは、代数的効果システムと組み合わせて使用することで、より表現力豊かなリソース管理パターンを実現します。特に、ライフサイクル管理効果（`effect with lifecycle`）との統合により、リソース管理と効果システムの統合的なアプローチが可能になります。
