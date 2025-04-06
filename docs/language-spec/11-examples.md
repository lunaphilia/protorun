# 11. 例：サンプルプログラム

## 11.1 サンプルプログラムの目的と概要

サンプルプログラムは、Protorun言語の機能と使用パターンを実際のコード例を通じて示すことを目的としています。これらの例は以下の役割を果たします：

1. **言語機能の実演**: 言語の主要な機能を実際のコードで示します。
2. **パターンの例示**: 一般的なプログラミングパターンをProtorun言語でどのように実装するかを示します。
3. **ベストプラクティス**: 言語の推奨される使用方法とスタイルを示します。
4. **学習リソース**: 言語を学ぶ開発者のための具体的な参考例を提供します。
5. **機能の組み合わせ**: 異なる言語機能がどのように連携するかを示します。

以下のサンプルプログラムは、Protorun言語の様々な側面を示すために選ばれています。

## 11.2 簡単な計算機

この例は、代数的データ型、パターンマッチング、代数的効果（例外処理）を使用した簡単な計算機の実装を示しています。

```protorun
// 計算機の実装
enum Expr {
  Number(Int),
  Add(Expr, Expr),
  Subtract(Expr, Expr),
  Multiply(Expr, Expr),
  Divide(Expr, Expr)
}

// 例外効果インターフェース
effect Exception<E> {
  fn raise<T>(error: E): T // T は任意の型 (脱出するため)
}

// 例外ハンドラ型
handler ExceptionHandler<E>: Exception<E> {
  // フィールドなし
  fn raise<T>(error: E): noresume Result<T, E> = { // noresume で継続を破棄
    Result.Err(error)
  }
}

// 式の評価 (Effect パラメータを使用)
fn evaluate(expr: Expr)(effect exc: Exception<String>): Result<Int, String> = {
  match expr {
    Expr.Number(value) => Result.Ok(value),

    Expr.Add(left, right) => {
      // 再帰呼び出しも Effect パラメータを暗黙的に引き継ぐ (要仕様確認)
      // あるいは明示的に渡す: evaluate(left)(effect exc = exc)?
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
        // Effect パラメータを使って効果操作を呼び出す
        exc.raise("ゼロ除算エラー")
      } else {
        Result.Ok(l / r)
      }
    }
  }
}

// runWithException ヘルパー関数 (新しい構文を使用)
fn runWithException<T, E>(action: (effect exc: Exception<E>) -> T): Result<T, E> = {
  // ハンドラインスタンスを生成
  let handlerInstance = ExceptionHandler<E> {}
  // with 式でハンドラを注入
  with exc = handlerInstance: Exception<E> {
    // action を呼び出す。action 内の exc.raise は handlerInstance で処理される
    // action が正常終了すれば Ok、raise されれば Err が with 式の結果となる
    Result.Ok(action(effect exc = exc)) // action に effect パラメータを渡す (構文要検討)
  }
}


// 使用例
fn main()(effect console: Console): Unit = {
  let expr = Expr.Add(
    Expr.Number(10),
    Expr.Multiply(
      Expr.Number(2),
      Expr.Number(3)
    )
  )

  // evaluate は Result<Int, String> を返す (Exception 効果は runWithException で処理される)
  let result = runWithException((effect exc) => evaluate(expr)(effect exc = exc)) // ラムダ式で Effect パラメータを受け取る

  match result {
    Result.Ok(value) => console.log(s"結果: $value"),
    Result.Err(error) => console.log(s"エラー: $error")
  }

  // ゼロ除算の例
  let expr_div_zero = Expr.Divide(Expr.Number(5), Expr.Number(0))
  let result_div_zero = runWithException((effect exc) => evaluate(expr_div_zero)(effect exc = exc))

  match result_div_zero {
    Result.Ok(value) => console.log(s"結果: $value"), // ここは通らない
    Result.Err(error) => console.log(s"エラー: $error") // "エラー: ゼロ除算エラー"
  }
}

// main 関数を実行するためのトップレベルハンドラ (仮)
with console = ConsoleHandler {}: Console {
  main()
}
```

この例では、以下の言語機能を示しています：

1. **代数的データ型（enum）**: 式の構造を表現。
2. **パターンマッチング**: 式の種類に基づいた処理の分岐。
3. **代数的効果**: 例外処理のための型安全なメカニズム (`Exception<E>`)。
4. **ハンドラ型**: 効果インターフェースの実装 (`ExceptionHandler<E>`)。
5. **Effect パラメータ**: 関数が効果実装に依存することを宣言 (`effect exc: Exception<String>`)。
6. **効果操作呼び出し**: エイリアスを使った呼び出し (`exc.raise(...)`)。
7. **`with` 式**: ハンドラインスタンスの注入 (`with exc = ExceptionHandler<E> {} ...`)。
8. **継続制御**: `noresume` による大域脱出。
9. **エラー処理**: `Result` 型と `?` 演算子。

## 11.3 状態を持つカウンター

この例は、状態効果を使用したカウンターの実装を示しています。

```protorun
// 状態効果インターフェース (再掲)
effect State<S> {
  fn get(): S
  fn set(newState: S): Unit
  fn modify(f: (S) -> S): Unit
}

// 状態ハンドラ型 (再掲)
handler StateHandler<S>: State<S> {
  let mutable state: S // フィールド定義

  // 効果操作の実装
  fn get(): S = self.state
  fn set(newState: S): Unit = { self.state = newState }
  fn modify(f: (S) -> S): Unit = { self.state = f(self.state) }
}


// カウンターの実装 (Effect パラメータを使用)
// この関数自体は状態を持たず、State<Int> 効果に依存する
fn counterTick(effect state: State<Int>): Int = {
  let current = state.get()
  state.modify(count => count + 1)
  current // インクリメント前の値を返す
}

// runWithState ヘルパー関数 (新しい構文を使用)
fn runWithState<S, T>(initialState: S, action: (effect st: State<S>) -> T): (T, S) = {
  // ハンドラインスタンスを生成
  let handlerInstance = StateHandler<S> { state: initialState }
  // with 式でハンドラを注入
  let result = with st = handlerInstance: State<S> {
    action(effect st = st) // action に Effect パラメータを渡す (構文要検討)
  }
  // 最終的な状態をハンドラインスタンスから取得
  let finalState = handlerInstance.state // フィールドアクセス (可視性要検討)
  (result, finalState)
}


// 使用例
fn main()(effect console: Console): Unit = {

  // runWithState を使ってカウンターを実行
  let (result1, state1) = runWithState(0, (effect st) => {
    let r1 = counterTick(effect state = st)
    console.log(s"1回目: ${r1}") // 0
    r1 // ヘルパー関数に結果を返す
  })
  console.log(s"状態1: ${state1}") // 1

  // 別の初期状態で実行
  let (result2, state2) = runWithState(10, (effect st) => {
    let r2 = counterTick(effect state = st)
    console.log(s"2回目: ${r2}") // 10
    r2
  })
  console.log(s"状態2: ${state2}") // 11

  // 一つのハンドラスコープで複数回実行
  let (finalResult, finalState) = runWithState(0, (effect st) => {
      console.log(s"A: ${counterTick(effect state = st)}") // 0
      console.log(s"B: ${counterTick(effect state = st)}") // 1
      console.log(s"C: ${counterTick(effect state = st)}") // 2
      "Done" // 最後の結果
  })
  console.log(s"最終状態: ${finalState}") // 3
}

// main 関数を実行するためのトップレベルハンドラ (仮)
with console = ConsoleHandler {}: Console {
  main()
}
```

この例では、以下の言語機能を示しています：

1. **状態効果**: `State<S>` インターフェース。
2. **ハンドラ型**: 状態をフィールドとして持つ `StateHandler<S>`。
3. **Effect パラメータ**: `effect state: State<Int>`。
4. **効果操作呼び出し**: `state.get()`, `state.modify(...)`。
5. **`with` 式**: `StateHandler` インスタンスの注入。
6. **ハンドラインスタンスの状態**: `with` ブロックを抜けた後にハンドラの状態を取得（フィールド可視性による）。

## 11.4 ファイル処理（ライフサイクル管理効果を使用）

この例は、ライフサイクル管理効果を使用したファイル処理の実装を示しています。（ライフサイクル管理効果と RAII の連携はまだ詳細設計が必要です。ここでは基本的な考え方を示します。）

```protorun
// FileSystem 効果インターフェース (再掲)
effect FileSystem {
  fn open(path: String, mode: FileMode): Result<own FileHandle, IOError>
  fn close(handle: own FileHandle): Result<Unit, IOError>
  fn read(handle: &FileHandle): Result<String, IOError>
  fn write(handle: &mut FileHandle, content: String): Result<Unit, IOError>
}
// 仮の型定義
type FileHandle { id: Int } // 簡単のため ID のみ
type FileMode { Read, Write }
type IOError { message: String }

// ファイルシステムハンドラ型 (リソースを管理する可能性)
// 簡単のため、ここでは状態を持たないハンドラとする
handler SimpleFileHandler: FileSystem {
  // フィールドなし

  fn open(path: String, mode: FileMode): Result<own FileHandle, IOError> = {
    println(s"Simulating open: ${path}, mode: ${mode}")
    // 実際のファイルオープン処理...
    Ok(FileHandle { id: 123 }) // 仮のハンドル
  }
  fn close(handle: own FileHandle): Result<Unit, IOError> = {
    println(s"Simulating close: handle ${handle.id}")
    // 実際のファイルクローズ処理...
    Ok(())
  }
  fn read(handle: &FileHandle): Result<String, IOError> = {
    println(s"Simulating read: handle ${handle.id}")
    Ok("Simulated file content")
  }
  fn write(handle: &mut FileHandle, content: String): Result<Unit, IOError> = {
    println(s"Simulating write: handle ${handle.id}, content: '${content}'")
    Ok(())
  }
}

// ファイル処理の実装 (Effect パラメータを使用)
fn processFile(path: String)(effect fs: FileSystem): Result<String, IOError> = {
  // ハンドラにファイルを開かせ、ハンドルを取得
  let handle = fs.open(path, FileMode.Read)?

  // try-finally のような構造が必要か？ (RAII連携待ち)
  // 現状では明示的に close を呼ぶ必要がある
  let contentResult = fs.read(&handle) // read の結果を一旦変数に

  // close を呼ぶ (エラーがあってもなくても)
  let closeResult = fs.close(handle)

  // read の結果と close の結果を組み合わせる
  match (contentResult, closeResult) {
    (Result.Ok(content), Result.Ok(_)) => Result.Ok(content.toUpperCase()),
    (Result.Err(readErr), _) => Result.Err(readErr), // read エラー優先
    (_, Result.Err(closeErr)) => Result.Err(closeErr) // close エラー
  }
}

// 使用例
fn main()(effect console: Console): Unit = {
  // ハンドラインスタンスを生成
  let fsHandler = SimpleFileHandler {}

  // with でハンドラを注入して実行
  let result = with fs = fsHandler: FileSystem {
    processFile("my_data.txt")(effect fs = fs) // Effect パラメータを渡す
  }

  match result {
    Result.Ok(processedContent) => console.log(s"処理結果: ${processedContent}"),
    Result.Err(ioError) => console.log(s"ファイルエラー: ${ioError.message}")
  }
}

// main 関数を実行するためのトップレベルハンドラ (仮)
with console = ConsoleHandler {}: Console {
  main()
}
```

この例では、以下の言語機能を示しています：

1. **効果インターフェース**: ファイル操作を定義 (`FileSystem`)。
2. **ハンドラ型**: ファイル操作を実装 (`SimpleFileHandler`)。
3. **Effect パラメータ**: ファイルシステム実装への依存性 (`effect fs: FileSystem`)。
4. **`with` 式**: ハンドラの注入。
5. **リソース管理の課題**: 現状では明示的な `close` が必要。RAII との連携が望まれる点を示唆。

## 11.5 依存性注入としての Effect パラメータ

この例は、Effect パラメータが依存性注入のメカニズムとしてどのように機能するかを示します。特に、データベースアクセスのような外部サービスへの依存を抽象化する例です。

```protorun
// ユーザーデータ型
type User = { id: String, name: String, email: String }
type DbError { NotFound(String), ConnectionError(String) }

// データベースアクセス効果インターフェース
effect Database {
  fn query(sql: String): Result<List<Map<String, String>>, DbError> // 簡単のため Map を使用
  fn execute(sql: String): Result<Unit, DbError>
}

// ユーザーリポジトリ関数 (Database 効果に依存)
fn getUserById(userId: String)(effect db: Database): Result<User, DbError> = {
  let result = db.query(s"SELECT * FROM users WHERE id = '$userId'")?

  if result.isEmpty() {
    Result.Err(DbError.NotFound(s"ユーザーが見つかりません: $userId"))
  } else {
    let row = result[0] // 最初の行を取得
    Result.Ok(User {
      id: row.get("id").getOrElse(""), // Map から値を取得 (Option を想定)
      name: row.get("name").getOrElse(""),
      email: row.get("email").getOrElse("")
    })
  }
}

fn updateUser(user: User)(effect db: Database): Result<Unit, DbError> = {
  db.execute(s"UPDATE users SET name = '${user.name}', email = '${user.email}' WHERE id = '${user.id}'")
}

// --- ハンドラ定義 (例) ---

// 実際のデータベースハンドラ型
handler PostgresDbHandler: Database {
  let connectionString: String
  // connectionPool: ConnectionPool // 内部状態としてプールを持つなど

  fn query(sql: String): Result<List<Map<String, String>>, DbError> = {
    // self.connectionString を使って Postgres に接続し、クエリ実行
    // ... 実際の DB アクセスロジック ...
  }
  fn execute(sql: String): Result<Unit, DbError> = {
    // ... 実際の DB アクセスロジック ...
  }
}

// テスト用モックハンドラ型
handler MockDbHandler: Database {
  let mutable users: Map<String, User> // テストデータを保持

  fn query(sql: String): Result<List<Map<String, String>>, DbError> = {
    // sql を簡易的にパースして users からデータを返す (テスト用)
    // ... モック実装 ...
  }
  fn execute(sql: String): Result<Unit, DbError> = {
    // users マップを更新する (テスト用)
    // ... モック実装 ...
  }
}

// --- 使用例 ---

fn main()(effect console: Console): Unit = {
  // --- 本番環境での実行 ---
  console.log("--- 本番 DB で実行 ---")
  let dbConnectionString = "postgres://user:pass@host:port/db"
  with db = PostgresDbHandler { connectionString: dbConnectionString }: Database {
    let userResult = getUserById("user123")(effect db = db)
    match userResult {
      Result.Ok(user) => console.log(s"取得ユーザー: ${user.name}"),
      Result.Err(e) => console.log(s"取得エラー: ${e}")
    }
  }

  // --- テスト環境での実行 ---
  console.log("\n--- モック DB で実行 ---")
  let initialUsers = Map.of([
    ("user123", User { id: "user123", name: "Alice", email: "alice@example.com" })
  ])
  with db = MockDbHandler { users: initialUsers }: Database {
    // ユーザー取得テスト
    let userResult = getUserById("user123")(effect db = db)
    match userResult {
      Result.Ok(user) => console.log(s"取得ユーザー (モック): ${user.name}"), // Alice
      Result.Err(e) => console.log(s"取得エラー (モック): ${e}")
    }

    // ユーザー更新テスト
    let updatedUser = User { id: "user123", name: "Alice Smith", email: "alice.smith@example.com" }
    let updateResult = updateUser(updatedUser)(effect db = db)
    match updateResult {
      Result.Ok(_) => console.log("更新成功 (モック)"),
      Result.Err(e) => console.log(s"更新エラー (モック): ${e}")
    }

    // 再度取得して確認
    let userResultAfterUpdate = getUserById("user123")(effect db = db)
    match userResultAfterUpdate {
      Result.Ok(user) => console.log(s"再取得ユーザー (モック): ${user.name}"), // Alice Smith
      Result.Err(e) => console.log(s"再取得エラー (モック): ${e}")
    }
  }
}

// main 関数を実行するためのトップレベルハンドラ (仮)
with console = ConsoleHandler {}: Console {
  main()
}
```

この例では、以下の言語機能を示しています：

1. **依存性注入**: `getUserById` や `updateUser` は抽象的な `Database` 効果に依存し、具体的な実装（`PostgresDbHandler` や `MockDbHandler`）は `with` で注入される。
2. **テスト容易性**: 同じビジネスロジック（`getUserById` など）を、本番用ハンドラとテスト用モックハンドラで差し替えて実行できる。
3. **ハンドラ型の状態**: `MockDbHandler` がテストデータを内部状態として保持する例。
4. **疎結合**: ビジネスロジックが特定のデータベース実装に依存しない。

Effect パラメータと `with` 式は、このような依存性の注入と抽象化を実現するための強力なツールとなります。
