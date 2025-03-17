# 6. 代数的効果

## 6.1 効果の定義

```
// 基本的な効果定義
effect Console {
  fn log(message: String): Unit
  fn readLine(): String
}

// パラメータ化された効果
effect State<S> {
  fn get(): S
  fn set(newState: S): Unit
  fn modify(f: (S) -> S): Unit
}

// 所有権を考慮した効果定義
effect FileSystem {
  // ファイルの所有権を返す操作
  fn openFile(path: String): Result<own File, IOError>
  
  // ファイルの所有権を消費する操作
  fn closeFile(file: own File): Result<Unit, IOError>
  
  // ファイルの借用を使用する操作
  fn readFile(file: &File): Result<String, IOError>
  
  // ファイルの可変借用を使用する操作
  fn writeFile(file: &mut File, content: String): Result<Unit, IOError>
}
```

## 6.2 効果ハンドラの定義

効果ハンドラはtraitに対するimplと同様の構文で定義できます。効果ハンドラには3つの実装スタイルがあります：

### 6.2.1 暗黙的な継続（デフォルト）

```
// 暗黙的な継続を使用した効果ハンドラ
handler ConsoleHandler for Console {
  fn log(message: String): Unit = {
    println(message)
    // 暗黙的にresumeが呼び出される
  }
  
  fn readLine(): String = {
    readLine()
    // 戻り値が自動的に継続に渡される
  }
}
```

### 6.2.2 明示的な継続

```
// 明示的な継続を使用した効果ハンドラ
handler ExplicitConsoleHandler for Console {
  fn log(message: String, resume: () -> Unit): Unit = {
    println(message)
    resume()  // 明示的に継続を呼び出す
  }
  
  fn readLine(resume: (String) -> Unit): Unit = {
    let input = readLine()
    resume(input)  // 明示的に値を継続に渡す
  }
}
```

### 6.2.3 特殊な継続制御

```
// 継続を呼び出さない効果ハンドラ
handler ExceptionHandler<E> for Exception<E> {
  fn raise<T>(error: E): noresume T = {
    // noresume型は継続を呼び出さないことを示す
    Result.Err(error)
  }
}

// 継続を複数回呼び出す効果ハンドラ
handler ChoiceHandler for Choice {
  fn choose<T>(options: [T]): multiresume T = {
    // multiresume型は複数回の継続呼び出しを示す
    for option in options {
      // 各選択肢に対して継続を呼び出す
      resume(option) match {
        Result.Ok(value) => return value,  // 成功したら結果を返す
        Result.Err(_) => continue          // 失敗したら次の選択肢を試す
      }
    }
    throw new NoValidChoiceError()
  }
}
```

## 6.3 ライフサイクル管理を持つ効果

```
// ライフサイクル管理を持つ効果（旧リソース効果）
effect ResourceManager<R> with lifecycle {
  // 獲得操作（自動的にリソースのライフサイクル管理が行われる）
  fn open<E>(acquireFn: () -> Result<R, E>): Result<R, E> with cleanup
  
  // 解放操作の定義
  fn cleanup(resource: R): Unit
  
  // 通常の効果操作
  fn use<T>(resource: &R, operation: (r: &R) -> T): T
}

// ライフサイクル管理効果のハンドラ
handler ResourceManagerHandler<R> for ResourceManager<R> {
  // アクティブなリソースを追跡
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
    // リソースを解放
    if resource is managed type {
      resource.close()
    }
    activeResources.remove(resource)
  }
  
  fn use<T>(resource: &R, operation: (r: &R) -> T): T = {
    operation(resource)
  }
  
  // ハンドラの終了時に未解放のリソースを自動的に解放
  fn finalize(): Unit = {
    for resource in activeResources {
      if resource is managed type {
        resource.close()
      }
    }
  }
}

// 使用例
fn processFile(path: String): Result<String, IOError> with ResourceManager<File> = {
  // openはリソースを獲得し、スコープ終了時に自動的に解放される
  let file = ResourceManager.open(() => File.open(path))?
  
  // リソースの使用
  let content = ResourceManager.use(&file, f => f.readToString())?
  
  Result.Ok(content)
} // fileは自動的に解放される（cleanup関数が呼び出される）
```

## 6.4 コンテキスト型

```
// コンテキスト型の宣言（旧コンテキストリソース）
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
}

// 使用例
fn processUserData(userId: String): Result<UserData, Error> with Database = {
  // 暗黙的にコンテキストからDatabaseを使用
  let userData = Database.query(s"SELECT * FROM users WHERE id = $userId")?
  Result.Ok(parseUserData(userData))
}

fn main(): Result<Unit, Error> = {
  // データベース接続を確立
  let db = Database.connect(config)?
  
  // コンテキスト型として提供
  with db {
    // dbが暗黙的に利用可能になる
    let userData = processUserData("user123")?
    displayUserData(userData)
    
    Result.Ok(())
  } // dbは自動的に閉じられる
}
```

## 6.5 効果の使用

```
// 効果を使用する関数
fn greet(name: String): Unit with Console = {
  Console.log(s"こんにちは、${name}さん！")
}

// 複数の効果
fn counter(): Int with Console & State<Int> = {
  let current = State.get()
  Console.log(s"現在の値: $current")
  State.set(current + 1)
  State.get()
}

// 効果スコープを使用
fn processData(data: String): String = {
  // 通常のスコープ（効果なし）
  let length = data.length
  
  // Console効果のスコープ
  with Console {
    Console.log(s"データ処理: $length文字")
  }
  
  // State効果のスコープ
  with State<ProcessingState> {
    let state = State.get()
    // 状態に基づく処理
    State.set({ ...state, processed: true })
  }
  
  // 処理結果
  processResult(data)
}
```

## 6.6 効果ハンドラの使用

```
// 効果ハンドラを使用した関数
fn runWithConsole<T>(action: () -> T with Console): T = {
  with Console handled by ConsoleHandler {
    action()
  }
}

// 状態効果のハンドラを使用
fn runWithState<S, T>(initialState: S, action: () -> T with State<S>): (T, S) = {
  var state = initialState
  
  // 状態ハンドラを定義
  handler StateHandler for State<S> {
    fn get(): S = state
    
    fn set(newState: S): Unit = {
      state = newState
    }
    
    fn modify(f: (S) -> S): Unit = {
      state = f(state)
    }
  }
  
  // ハンドラを適用
  let result = with State<S> handled by StateHandler {
    action()
  }
  
  (result, state)
}

// 明示的な継続を使用するハンドラ
fn runWithExplicitConsole<T>(action: () -> T with Console): T = {
  // 明示的な継続を使用するハンドラを定義
  handler ExplicitConsoleHandler for Console {
    fn log(message: String, resume: () -> Unit): Unit = {
      println(message)
      resume()
    }
    
    fn readLine(resume: (String) -> Unit): Unit = {
      let input = readLine()
      resume(input)
    }
  }
  
  with Console handled by ExplicitConsoleHandler {
    action()
  }
}

// 継続を呼び出さないハンドラ（例外処理）
fn runWithException<T, E>(action: () -> T with Exception<E>): Result<T, E> = {
  // 継続を呼び出さないハンドラを定義
  handler ExceptionHandler for Exception<E> {
    fn raise<R>(error: E): noresume R = {
      return Result.Err(error)
    }
  }
  
  try {
    let result = with Exception<E> handled by ExceptionHandler {
      action()
    }
    Result.Ok(result)
  } catch (e) {
    Result.Err(e)
  }
}
```

## 6.7 効果の合成

```
// 複数の効果を扱う
fn program(): Int = {
  // 複数のハンドラを合成
  with Console handled by ConsoleHandler {
    with State<Int> handled by (initialState = 0) {
      counter()
    }
  }
}

// 効果ハンドラを指定したスコープ
fn main(): Unit = {
  // Console効果のハンドラを指定
  with Console handled by ConsoleHandler {
    // このスコープ内のConsole効果はConsoleHandlerでハンドルされる
    Console.log("アプリケーション開始")
    
    // State効果のハンドラを指定
    with State<AppConfig> handled by StateHandler(initialState = defaultConfig) {
      // このスコープ内のState効果はStateHandlerでハンドルされる
      let config = State.get()
      Console.log(s"設定: $config")
      
      // アプリケーションロジック
      runApplication()
    }
    
    Console.log("アプリケーション終了")
  }
}
