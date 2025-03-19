# 8. 例：サンプルプログラム

## 8.1 サンプルプログラムの目的と概要

サンプルプログラムは、Protorun言語の機能と使用パターンを実際のコード例を通じて示すことを目的としています。これらの例は以下の役割を果たします：

1. **言語機能の実演**: 言語の主要な機能を実際のコードで示します。
2. **パターンの例示**: 一般的なプログラミングパターンをProtorun言語でどのように実装するかを示します。
3. **ベストプラクティス**: 言語の推奨される使用方法とスタイルを示します。
4. **学習リソース**: 言語を学ぶ開発者のための具体的な参考例を提供します。
5. **機能の組み合わせ**: 異なる言語機能がどのように連携するかを示します。

以下のサンプルプログラムは、Protorun言語の様々な側面を示すために選ばれています。

## 8.2 簡単な計算機

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

## 8.3 状態を持つカウンター

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

## 8.4 ファイル処理（ライフサイクル管理効果を使用）

この例は、ライフサイクル管理効果を使用したファイル処理の実装を示しています。ライフサイクル管理効果の詳細については、[5.4 ライフサイクル管理効果](05-algebraic-effects.md#54-ライフサイクル管理効果)を参照してください。

```
// ファイル処理の実装（ライフサイクル管理効果を使用）
fn processFile(path: String): Result<String, IOError> & FileSystem = {
  // ファイルを開く（スコープ終了時に自動的に閉じられる）
  let file = FileSystem.acquire()
  
  // ファイルから読み込む
  let content = FileSystem.read()
  
  // 処理された内容を別のファイルに書き込む
  let processed = content.toUpperCase()
  FileSystem.write(processed)
  
  Result.Ok(processed)
} // fileは自動的に解放される（FileSystem.release(file)が呼び出される）
```

この例では、以下の言語機能を示しています：

1. **ライフサイクル管理効果**: リソースの獲得と解放を自動的に管理
2. **借用**: リソースの安全な参照
3. **効果ハンドラ**: リソース管理効果の実装
4. **エラー処理**: `Result`型と`?`演算子を使用したエラー伝播
5. **型チェック**: `is`演算子を使用した型チェック

## 8.5 暗黙的パラメータを使用したデータベース操作

この例は、暗黙的パラメータと効果システムを使用したデータベース操作の実装を示しています。暗黙的パラメータの詳細については、[5.8 暗黙的パラメータと効果システム](05-algebraic-effects.md#58-暗黙的パラメータと効果システム)を参照してください。

```
// ユーザーデータ型
type User = {
  id: String,
  name: String,
  email: String
}

// ユーザーリポジトリ
fn getUserById(userId: String)(with db: Database): Result<User, DbError> = {
  let result = db.query(s"SELECT * FROM users WHERE id = $userId")?
  
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

fn updateUser(user: User)(with db: Database): Result<Unit, DbError> = {
  db.execute(s"UPDATE users SET name = '${user.name}', email = '${user.email}' WHERE id = '${user.id}'")?
  Result.Ok(())
}
```

この例では、以下の言語機能を示しています：

1. **コンテキスト型**: 暗黙的なコンテキスト渡しとリソースライフサイクル管理
2. **トランザクション**: データベーストランザクションの実装
3. **レコード型**: ユーザーデータの表現
4. **レコード更新**: スプレッド構文を使用したレコードの更新
5. **文字列補間**: SQL文の構築

## 8.6 代数的効果と暗黙的パラメータの連携

この例は、代数的効果と暗黙的パラメータがどのように連携できるかを示しています。

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
fn processUserData(userId: String): Result<UserData, Error> & DbAccess & Logging = {
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

// 暗黙的パラメータを使用して効果を実装
fn runWithDatabase<T>(action: () -> T & DbAccess & Logging)(with db: Database, logger: Logger): Result<T, Error> = {
  // 効果ハンドラを定義
  handler DbHandler: DbAccess {
    fn query(sql: String): Result<QueryResult, DbError> = {
      db.query(sql)
    }
    
    fn execute(sql: String): Result<Unit, DbError> = {
      db.execute(sql)
    }
  }
  
  handler LogHandler: Logging {
    fn log(level: LogLevel, message: String): Unit = {
      logger.log(level, message)
    }
  }
  
  // 効果ハンドラを適用
  with DbHandler: DbAccess {
    with LogHandler: Logging {
      try {
        Result.Ok(action())
      } catch (e) {
        Result.Err(e)
      }
    }
  }
}

// 使用例
fn main(): Result<Unit, Error> & Console = {
  // データベースハンドラとロガーハンドラを定義
  handler DatabaseHandler: Database {
    // 実装...
  }
  
  handler LoggerHandler: Logger {
    // 実装...
  }
  
  // ハンドラを適用
  with DatabaseHandler: Database {
    with LoggerHandler: Logger {
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
  }
}
```

この例では、以下の言語機能を示しています：

1. **代数的効果とコンテキスト型の連携**: 効果ハンドラがコンテキスト型を使用して実装される
2. **複数の効果**: 複数の効果を組み合わせたビジネスロジック
3. **効果ハンドラの合成**: 複数のハンドラを組み合わせた処理
4. **エラー処理**: 複数のエラー型の統合
5. **コンテキストの提供**: 複数のコンテキスト型の同時提供

これらのサンプルプログラムは、Protorun言語の主要な機能と使用パターンを示しています。実際のアプリケーション開発では、これらのパターンを組み合わせて、より複雑で実用的なプログラムを構築することができます。
