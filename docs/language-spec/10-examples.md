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
  fn raise<T>(error: E): T // noresume はハンドラ側で指定
}

// 例外ハンドラ
handler ExceptionHandler<E>: Exception<E> { // "for" を ":" に修正
  fn raise<T>(error: E): noresume Result<T, E> = { // 戻り値型を Result に修正
    Result.Err(error)
  }
}

// 式の評価
fn evaluate(expr: Expr): Result<Int, String> & Exception<String> = { // "with" を "&" に修正
  match expr {
    Expr.Number(value) => Result.Ok(value),

    Expr.Add(left, right) => {
      let l = evaluate(left)? // ?演算子は Exception 効果を伝播させる (ハンドラで Result.Err に変換される)
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
        Exception.raise("ゼロ除算エラー") // ハンドラによって Result.Err に変換される
      } else {
        Result.Ok(l / r)
      }
    }
  }
}

// runWithException ヘルパー関数 (07-algebraic-effects.md の例を参考)
fn runWithException<T, E>(action: () -> T & Exception<E>): Result<T, E> = {
  handler ExceptionHandler: Exception<E> {
    fn raise<R>(error: E): noresume Result<R, E> = {
      Result.Err(error)
    }
  }
  with ExceptionHandler: Exception<E> {
    Result.Ok(action()) // action() が raise を呼ぶと Err(error) が返る
  }
}


// 使用例
fn main(): Unit & Console = { // "with" を "&" に修正
  let expr = Expr.Add(
    Expr.Number(10),
    Expr.Multiply(
      Expr.Number(2),
      Expr.Number(3)
    )
  )

  // evaluate は Result<Int, String> & Exception<String> を返す
  // runWithException で Exception 効果をハンドルし、Result<Int, String> に変換
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
5. **エラー処理**: `Result`型と`?`演算子を使用したエラー伝播（効果ハンドラとの連携）

## 8.3 状態を持つカウンター

この例は、状態効果を使用したカウンターの実装を示しています。

```
// 状態効果 (07-algebraic-effects.md より)
effect State<S> {
  fn get(): S
  fn set(newState: S): Unit
  fn modify(f: (S) -> S): Unit
}

// カウンターの実装
fn makeCounter(): () -> Int & State<Int> = { // initial は不要 (状態はハンドラで管理)
  () => {
    let current = State.get()
    State.modify(count => count + 1)
    current
  }
}

// runWithState ヘルパー関数 (07-algebraic-effects.md の例を参考)
fn runWithState<S, T>(initialState: S, action: () -> T & State<S>): (T, S) = {
  var state = initialState

  handler StateHandler: State<S> {
    fn get(): S = state
    fn set(newState: S): Unit = { state = newState }
    fn modify(f: (S) -> S): Unit = { state = f(state) }
  }

  let result = with StateHandler: State<S> {
    action()
  }
  (result, state) // 最終的な状態も返す
}


// 使用例
fn main(): Unit & Console = { // "with" を "&" に修正
  let counter = makeCounter()

  // runWithState を使ってカウンターを実行し、最終状態は無視
  let (result1, _) = runWithState(0, () => {
    let r1 = counter()
    Console.log(s"1回目: ${r1}") // 0
    Result.Ok(r1) // runWithState に結果を返す (型を合わせるため Result を使用)
  })

  let (result2, _) = runWithState(1, () => { // 初期状態を 1 に設定
    let r2 = counter()
    Console.log(s"2回目: ${r2}") // 1
    Result.Ok(r2)
  })

  let (result3, _) = runWithState(2, () => { // 初期状態を 2 に設定
    let r3 = counter()
    Console.log(s"3回目: ${r3}") // 2
    Result.Ok(r3)
  })

  // または、一つのハンドラスコープで実行
  let (finalResult, finalState) = runWithState(0, () => {
      let c = makeCounter()
      Console.log(s"A: ${c()}") // 0
      Console.log(s"B: ${c()}") // 1
      Console.log(s"C: ${c()}") // 2
      Result.Ok("Done") // 最後の結果
  })
  Console.log(s"最終状態: $finalState") // 3
}
```

この例では、以下の言語機能を示しています：

1. **状態効果**: 明示的な状態の取得と更新
2. **クロージャ**: 状態を捕捉する関数
3. **効果ハンドラ**: 状態効果の実装
4. **ジェネリクス**: 型パラメータを使用した汎用的な状態ハンドラ
5. **効果スコープ**: `with`式を使用した効果の局所的な適用

## 8.4 ファイル処理（ライフサイクル管理効果を使用）

この例は、ライフサイクル管理効果を使用したファイル処理の実装を示しています。ライフサイクル管理効果の詳細については、[7.4 ライフサイクル管理効果](07-algebraic-effects.md#74-ライフサイクル管理効果)を参照してください。

```
// FileSystem 効果 (05-algebraic-effects.md より)
effect FileSystem: LifecycleEffect<File> { // File 型は別途定義が必要
  fn acquire(): File // LifecycleEffect から継承
  fn release(resource: File): Unit // LifecycleEffect から継承
  fn read(): String // FileSystem 固有の操作 (例)
  fn write(content: String): Unit // FileSystem 固有の操作 (例)
}

// ファイル処理の実装（ライフサイクル管理効果を使用）
fn processFile(path: String): Result<String, IOError> & FileSystem = { // IOError 型は別途定義が必要
  // ファイルを開く（スコープ終了時に自動的に閉じられる）
  let file = FileSystem.acquire() // acquire は File を返す (ハンドラで実装)

  // ファイルから読み込む (read は FileSystem 効果の操作)
  let content = FileSystem.read() // read は String を返す (ハンドラで実装)

  // 処理された内容を別のファイルに書き込む (write は FileSystem 効果の操作)
  let processed = content.toUpperCase()
  FileSystem.write(processed) // write は Unit を返す (ハンドラで実装)

  Result.Ok(processed)
} // file は自動的に解放される（FileSystem.release(file) が呼び出される）
```

この例では、以下の言語機能を示しています：

1. **ライフサイクル管理効果**: リソースの獲得と解放を自動的に管理
2. **効果ハンドラ**: リソース管理効果の実装（別途定義が必要）
3. **エラー処理**: `Result`型を使用したエラー表現（IOErrorなど）
4. **効果シグネチャ**: 関数が `FileSystem` 効果を持つことを示す

## 8.5 暗黙的パラメータを使用したデータベース操作

この例は、暗黙的パラメータと効果システムを使用したデータベース操作の実装を示しています。暗黙的パラメータの詳細については、[7.8 暗黙的パラメータと効果システム](07-algebraic-effects.md#78-暗黙的パラメータと効果システム)を参照してください。

```
// ユーザーデータ型
type User = {
  id: String,
  name: String,
  email: String
}

// Database 効果 (05-algebraic-effects.md より)
effect Database: LifecycleEffect<Connection> { // Connection 型は別途定義が必要
  fn acquire(): Connection
  fn release(resource: Connection): Unit
  fn query(sql: String): Result<QueryResult, DbError> // QueryResult, DbError は別途定義が必要
  fn execute(sql: String): Result<Unit, DbError>
}


// ユーザーリポジトリ
fn getUserById(userId: String)(with db: Database): Result<User, DbError> = {
  let result = db.query(s"SELECT * FROM users WHERE id = $userId")?

  if result.isEmpty() { // QueryResult に isEmpty メソッドが必要
    Result.Err(DbError.NotFound(s"ユーザーが見つかりません: $userId"))
  } else {
    let row = result.first() // QueryResult に first メソッドが必要
    Result.Ok(User {
      id: row.getString("id"), // row に getString メソッドが必要
      name: row.getString("name"),
      email: row.getString("email")
    })
  }
}

fn updateUser(user: User)(with db: Database): Result<Unit, DbError> = {
  db.execute(s"UPDATE users SET name = '${user.name}', email = '${user.email}' WHERE id = '${user.id}'")?
  Result.Ok(())
}

// 使用例 (ハンドラ定義は省略)
// handler DatabaseHandler: Database { ... }
// fn main(): Unit & Console = {
//   with DatabaseHandler: Database {
//     let userResult = getUserById("user123")
//     match userResult {
//       Result.Ok(user) => {
//         Console.log(s"ユーザー名: ${user.name}")
//         let updatedUser = { ...user, name: "新しい名前" } // レコード更新構文が必要
//         updateUser(updatedUser) match {
//           Result.Ok(_) => Console.log("更新成功"),
//           Result.Err(e) => Console.log(s"更新エラー: $e")
//         }
//       },
//       Result.Err(e) => Console.log(s"取得エラー: $e")
//     }
//   }
// }

```

この例では、以下の言語機能を示しています：

1. **暗黙的パラメータ**: `Database` 効果を暗黙的に関数に渡す
2. **効果システムとの連携**: 効果を依存性として注入する
3. **レコード型**: ユーザーデータの表現
4. **エラー処理**: `Result` 型によるデータベースエラーの処理
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
  fn log(level: LogLevel, message: String): Unit // LogLevel は別途定義が必要
}

// ビジネスロジック
fn processUserData(userId: String): Result<UserData, Error> & DbAccess & Logging = { // UserData, Error は別途定義が必要
  Logging.log(LogLevel.Info, s"ユーザーデータの処理開始: $userId")

  // データベースからユーザー情報を取得
  let userResult = DbAccess.query(s"SELECT * FROM users WHERE id = '$userId'")?

  if userResult.isEmpty() {
    Logging.log(LogLevel.Warning, s"ユーザーが見つかりません: $userId")
    return Result.Err(Error.NotFound(s"ユーザーが見つかりません: $userId"))
  }

  let userData = parseUserData(userResult.first()) // parseUserData は別途定義が必要

  // 処理ログを記録
  DbAccess.execute(s"INSERT INTO logs (user_id, action) VALUES ('$userId', 'data_processed')")?

  Logging.log(LogLevel.Info, s"ユーザーデータの処理完了: $userId")
  Result.Ok(userData)
}

// 暗黙的パラメータを使用して効果を実装するヘルパー関数
fn runWithDbAndLogger<T>(action: () -> T & DbAccess & Logging)(with db: Database, logger: Logger): Result<T, Error> = {
  // 効果ハンドラを定義
  handler DbHandler: DbAccess {
    fn query(sql: String): Result<QueryResult, DbError> = {
      db.query(sql) // 暗黙的に渡された db を使用
    }

    fn execute(sql: String): Result<Unit, DbError> = {
      db.execute(sql) // 暗黙的に渡された db を使用
    }
  }

  handler LogHandler: Logging {
    fn log(level: LogLevel, message: String): Unit = {
      logger.log(level, message) // 暗黙的に渡された logger を使用
    }
  }

  // 効果ハンドラを適用
  with DbHandler: DbAccess {
    with LogHandler: Logging {
      // action() を実行し、結果を Result.Ok でラップ
      // action() 内で発生した効果 (DbAccess, Logging) はここでハンドルされる
      // action() が Result.Err を返した場合、それがそのままこの with 式の結果となる
      // (try/catch は不要)
      Result.Ok(action())
    }
  }
}

// 使用例
fn main(): Unit & Console = { // 効果を Console に変更
  // データベースハンドラとロガーハンドラを定義 (実際の接続やファイル設定など)
  handler DatabaseHandler: Database { /* 実装... */ }
  handler LoggerHandler: Logger { /* 実装... */ }

  // ハンドラを適用して、暗黙的パラメータを提供
  with DatabaseHandler: Database {
    with LoggerHandler: Logger {
      // runWithDbAndLogger を呼び出し、暗黙的に db と logger を渡す
      runWithDbAndLogger(() => {
        processUserData("user123") // DbAccess と Logging 効果を使用
      }) match {
        Result.Ok(userDataResult) => { // runWithDbAndLogger は Result<Result<UserData, Error>, Error> を返す可能性があるので注意
             match userDataResult {
                 Result.Ok(userData) => Console.log(s"ユーザーデータ: $userData"),
                 Result.Err(processError) => Console.log(s"処理エラー: $processError")
             }
        },
        Result.Err(handlerError) => Console.log(s"ハンドラエラー: $handlerError") // runWithDbAndLogger 自体のエラー (通常は発生しないはず)
      }
    }
  }
}
```

この例では、以下の言語機能を示しています：

1. **代数的効果と暗黙的パラメータの連携**: 効果ハンドラが暗黙的パラメータを使用して実装される
2. **複数の効果**: 複数の効果を組み合わせたビジネスロジック
3. **効果ハンドラの合成**: 複数のハンドラを組み合わせた処理
4. **エラー処理**: `Result` 型によるエラー処理
5. **依存性の注入**: `runWithDbAndLogger` 関数が依存性（`db`, `logger`）を受け取り、それを使用して効果ハンドラを構成する

これらのサンプルプログラムは、Protorun言語の主要な機能と使用パターンを示しています。実際のアプリケーション開発では、これらのパターンを組み合わせて、より複雑で実用的なプログラムを構築することができます。
