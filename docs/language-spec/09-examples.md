# 9. 例：サンプルプログラム

## 9.1 サンプルプログラムの目的と概要

サンプルプログラムは、Protorun言語の機能と使用パターンを実際のコード例を通じて示すことを目的としています。これらの例は以下の役割を果たします：

1. **言語機能の実演**: 言語の主要な機能を実際のコードで示します。
2. **パターンの例示**: 一般的なプログラミングパターンをProtorun言語でどのように実装するかを示します。
3. **ベストプラクティス**: 言語の推奨される使用方法とスタイルを示します。
4. **学習リソース**: 言語を学ぶ開発者のための具体的な参考例を提供します。
5. **機能の組み合わせ**: 異なる言語機能がどのように連携するかを示します。

以下のサンプルプログラムは、Protorun言語の様々な側面を示すために選ばれています。

## 9.2 簡単な計算機

この例は、代数的データ型、パターンマッチング、代数的効果（例外処理）を使用した簡単な計算機の実装を示しています。

```
// 計算機の実装
enum Expr {
  Number(Int),
  Add(Expr, Expr),
  Subtract(Expr, Expr),
  Multiply(Expr, Expr),
  Divide(Expr, Expr)
}

// 例外効果
effect Exception<E> {
  fn raise<T>(error: E): T
}

// 例外ハンドラ
handler ExceptionHandler<E> for Exception<E> {
  fn raise<T>(error: E): noresume T = {
    Result.Err(error)
  }
}

// 式の評価
fn evaluate(expr: Expr): Result<Int, String> with Exception<String> = {
  match expr {
    Expr.Number(value) => Result.Ok(value),
    
    Expr.Add(left, right) => {
      let l = evaluate(left)?
      let r = evaluate(right)?
      Result.Ok(l + r)
    },
    
    Expr.Subtract(left, right) => {
      let l = evaluate(left)?
      let r = evaluate(right)?
      Result.Ok(l - r)
    },
    
    Expr.Multiply(left, right) => {
      let l = evaluate(left)?
      let r = evaluate(right)?
      Result.Ok(l * r)
    },
    
    Expr.Divide(left, right) => {
      let l = evaluate(left)?
      let r = evaluate(right)?
      
      if r == 0 {
        Exception.raise("ゼロ除算エラー")
      } else {
        Result.Ok(l / r)
      }
    }
  }
}

// 使用例
fn main(): Unit with Console = {
  let expr = Expr.Add(
    Expr.Number(10),
    Expr.Multiply(
      Expr.Number(2),
      Expr.Number(3)
    )
  )
  
  let result = runWithException(() => evaluate(expr))
  
  match result {
    Result.Ok(value) => Console.log(s"結果: $value"),
    Result.Err(error) => Console.log(s"エラー: $error")
  }
}
```

この例では、以下の言語機能を示しています：

1. **代数的データ型（enum）**: 式の構造を表現するための階層的なデータ型
2. **パターンマッチング**: 式の種類に基づいた処理の分岐
3. **代数的効果**: 例外処理のための型安全なメカニズム
4. **特殊な継続制御**: `noresume`キーワードを使用した継続を呼び出さない効果ハンドラ
5. **エラー処理**: `Result`型と`?`演算子を使用したエラー伝播

## 9.3 状態を持つカウンター

この例は、状態効果を使用したカウンターの実装を示しています。

```
// カウンターの実装
fn makeCounter(initial: Int): () -> Int with State<Int> = {
  () => {
    let current = State.get()
    State.modify(count => count + 1)
    current
  }
}

// 使用例
fn main(): Unit with Console = {
  // 状態ハンドラを定義
  handler StateHandler<S> for State<S> {
    var state: S
    
    fn get(): S = state
    
    fn set(newState: S): Unit = {
      state = newState
    }
    
    fn modify(f: (S) -> S): Unit = {
      state = f(state)
    }
  }
  
  // カウンターを作成して使用
  with State<Int> handled by StateHandler(state = 0) {
    let counter = makeCounter(0)
    
    Console.log(s"1回目: ${counter()}")  // 0
    Console.log(s"2回目: ${counter()}")  // 1
    Console.log(s"3回目: ${counter()}")  // 2
  }
}
```

この例では、以下の言語機能を示しています：

1. **状態効果**: 明示的な状態の取得と更新
2. **クロージャ**: 状態を捕捉する関数
3. **効果ハンドラ**: 状態効果の実装
4. **ジェネリクス**: 型パラメータを使用した汎用的な状態ハンドラ
5. **効果スコープ**: `with`式を使用した効果の局所的な適用

## 9.4 ファイル処理（ライフサイクル管理効果を使用）

この例は、ライフサイクル管理効果を使用したファイル処理の実装を示しています。

```
// ファイル処理の実装（ライフサイクル管理効果を使用）
fn processFile(path: String): Result<String, IOError> with ResourceManager<File> = {
  // ファイルを開く（スコープ終了時に自動的に閉じられる）
  let file = ResourceManager.open(() => File.open(path))?
  
  // ファイルから読み込む
  let content = ResourceManager.use(&file, f => f.read())?
  
  // 処理された内容を別のファイルに書き込む
  let processed = content.toUpperCase()
  let outputFile = ResourceManager.open(() => File.open(path + ".processed"))?
  ResourceManager.use(&outputFile, f => f.write(processed))?
  
  Result.Ok(processed)
} // fileとoutputFileは自動的に解放される（cleanup関数が呼び出される）

// 使用例
fn main(): Unit with Console = {
  // ライフサイクル管理効果ハンドラを定義
  handler ResourceManagerHandler<R> for ResourceManager<R> {
    var activeResources = Set<R>()
    
    fn open<E>(acquireFn: () -> Result<R, E>): Result<R, E> = {
      match acquireFn() {
        Result.Ok(resource) => {
          activeResources.add(resource)
          Result.Ok(resource)
        },
        Result.Err(error) => Result.Err(error)
      }
    }
    
    fn cleanup(resource: R): Unit = {
      if resource is managed type {
        resource.close()
      }
      activeResources.remove(resource)
    }
    
    fn use<T>(resource: &R, operation: (r: &R) -> T): T = {
      operation(resource)
    }
    
    fn finalize(): Unit = {
      for resource in activeResources {
        if resource is managed type {
          resource.close()
        }
      }
    }
  }
  
  // ハンドラを適用して処理を実行
  with ResourceManager<File> handled by ResourceManagerHandler {
    match processFile("input.txt") {
      Result.Ok(content) => Console.log(s"処理完了: $content"),
      Result.Err(error) => Console.log(s"エラー: $error")
    }
  }
}
```

この例では、以下の言語機能を示しています：

1. **ライフサイクル管理効果**: リソースの獲得と解放を自動的に管理
2. **借用**: リソースの安全な参照
3. **効果ハンドラ**: リソース管理効果の実装
4. **エラー処理**: `Result`型と`?`演算子を使用したエラー伝播
5. **型チェック**: `is`演算子を使用した型チェック

## 9.5 コンテキスト型を使用したデータベース操作

この例は、コンテキスト型を使用したデータベース操作の実装を示しています。

```
// データベースコンテキスト型
context type Database {
  connection: DbConnection,
  
  // 初期化
  fn connect(config: DbConfig): Result<Database, DbError> = {
    let conn = DbConnection.open(config)?
    Result.Ok(Database { connection: conn })
  }
  
  // 解放（自動的に呼び出される）
  fn close(self): Unit = {
    self.connection.close()
  }
  
  // クエリメソッド
  fn query(self: &Self, sql: String): Result<QueryResult, DbError> = {
    self.connection.query(sql)
  }
  
  // トランザクションメソッド
  fn transaction<T, E>(self: &Self, action: () -> Result<T, E> with Database): Result<T, E | DbError> = {
    self.connection.beginTransaction()
    
    try {
      let result = action()
      
      match result {
        Result.Ok(_) => self.connection.commit(),
        Result.Err(_) => self.connection.rollback()
      }
      
      result
    } catch (e) {
      self.connection.rollback()
      throw e
    }
  }
}

// ユーザーデータ型
type User = {
  id: String,
  name: String,
  email: String
}

// ユーザーリポジトリ
fn getUserById(userId: String): Result<User, DbError> with Database = {
  let result = Database.query(s"SELECT * FROM users WHERE id = $userId")?
  
  if result.isEmpty() {
    Result.Err(DbError.NotFound(s"ユーザーが見つかりません: $userId"))
  } else {
    let row = result.first()
    Result.Ok(User {
      id: row.getString("id"),
      name: row.getString("name"),
      email: row.getString("email")
    })
  }
}

fn updateUser(user: User): Result<Unit, DbError> with Database = {
  Database.query(s"UPDATE users SET name = '${user.name}', email = '${user.email}' WHERE id = '${user.id}'")?
  Result.Ok(())
}

// 使用例
fn main(): Result<Unit, Error> with Console = {
  // データベース接続を確立
  let db = Database.connect(DbConfig {
    host: "localhost",
    port: 5432,
    username: "admin",
    password: "password",
    database: "myapp"
  })?
  
  // コンテキスト型として提供
  with db {
    // トランザクション内でユーザーを更新
    Database.transaction(() => {
      let user = getUserById("user123")?
      let updatedUser = { ...user, name: "新しい名前" }
      updateUser(updatedUser)?
      Result.Ok(())
    }) match {
      Result.Ok(_) => Console.log("ユーザーを更新しました"),
      Result.Err(error) => Console.log(s"エラー: $error")
    }
    
    Result.Ok(())
  } // dbは自動的に閉じられる
}
```

この例では、以下の言語機能を示しています：

1. **コンテキスト型**: 暗黙的なコンテキスト渡しとリソースライフサイクル管理
2. **トランザクション**: データベーストランザクションの実装
3. **レコード型**: ユーザーデータの表現
4. **レコード更新**: スプレッド構文を使用したレコードの更新
5. **文字列補間**: SQL文の構築

## 9.6 代数的効果とコンテキスト型の連携

この例は、代数的効果とコンテキスト型がどのように連携できるかを示しています。

```
// データベースアクセス効果
effect DbAccess {
  fn query(sql: String): Result<QueryResult, DbError>
  fn execute(sql: String): Result<Unit, DbError>
}

// ロギング効果
effect Logging {
  fn log(level: LogLevel, message: String): Unit
}

// ビジネスロジック
fn processUserData(userId: String): Result<UserData, Error> with DbAccess & Logging = {
  Logging.log(LogLevel.Info, s"ユーザーデータの処理開始: $userId")
  
  // データベースからユーザー情報を取得
  let userResult = DbAccess.query(s"SELECT * FROM users WHERE id = '$userId'")?
  
  if userResult.isEmpty() {
    Logging.log(LogLevel.Warning, s"ユーザーが見つかりません: $userId")
    return Result.Err(Error.NotFound(s"ユーザーが見つかりません: $userId"))
  }
  
  let userData = parseUserData(userResult.first())
  
  // 処理ログを記録
  DbAccess.execute(s"INSERT INTO logs (user_id, action) VALUES ('$userId', 'data_processed')")?
  
  Logging.log(LogLevel.Info, s"ユーザーデータの処理完了: $userId")
  Result.Ok(userData)
}

// コンテキスト型を使用して効果を実装
fn runWithDatabase<T>(action: () -> T with DbAccess & Logging): Result<T, Error> = {
  // データベースコンテキスト
  let db = Database.connect(config)?
  
  // ロガーコンテキスト
  let logger = Logger.init(logConfig)
  
  // 効果ハンドラを定義
  handler DbHandler for DbAccess {
    fn query(sql: String): Result<QueryResult, DbError> = {
      db.query(sql)
    }
    
    fn execute(sql: String): Result<Unit, DbError> = {
      db.execute(sql)
    }
  }
  
  handler LogHandler for Logging {
    fn log(level: LogLevel, message: String): Unit = {
      logger.log(level, message)
    }
  }
  
  // コンテキスト型と効果ハンドラを組み合わせて使用
  with db, logger {
    with DbAccess handled by DbHandler {
      with Logging handled by LogHandler {
        try {
          Result.Ok(action())
        } catch (e) {
          Result.Err(e)
        }
      }
    }
  }
}

// 使用例
fn main(): Result<Unit, Error> with Console = {
  runWithDatabase(() => {
    let userData = processUserData("user123")?
    Console.log(s"ユーザーデータ: $userData")
    Result.Ok(())
  }) match {
    Result.Ok(_) => Console.log("処理が完了しました"),
    Result.Err(error) => Console.log(s"エラー: $error")
  }
  
  Result.Ok(())
}
```

この例では、以下の言語機能を示しています：

1. **代数的効果とコンテキスト型の連携**: 効果ハンドラがコンテキスト型を使用して実装される
2. **複数の効果**: 複数の効果を組み合わせたビジネスロジック
3. **効果ハンドラの合成**: 複数のハンドラを組み合わせた処理
4. **エラー処理**: 複数のエラー型の統合
5. **コンテキストの提供**: 複数のコンテキスト型の同時提供

これらのサンプルプログラムは、Protorun言語の主要な機能と使用パターンを示しています。実際のアプリケーション開発では、これらのパターンを組み合わせて、より複雑で実用的なプログラムを構築することができます。
