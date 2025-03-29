# 5. 代数的効果

## 5.1 代数的効果の概念と目的

代数的効果は、副作用を型安全に表現し制御するためのProtorun言語の中核機能です。この機能は以下の目的で設計されています：

1. **型安全な副作用**: 関数が持つ可能性のある副作用を型レベルで追跡し、安全に制御します。
2. **効果の分離**: 純粋な計算ロジックと副作用を明確に分離します。
3. **合成可能性**: 異なる効果を持つ計算を安全に合成できるようにします。
4. **局所的な効果**: 効果の影響範囲を明示的に制限し、プログラムの理解と推論を容易にします。
5. **継続ベースの制御フロー**: 例外処理や非決定性計算などの高度な制御フローパターンを表現します。

代数的効果は、モナドのような他の副作用制御メカニズムと比較して、より直感的で合成しやすいアプローチを提供します。効果ハンドラを通じて効果の実装を提供することで、効果の使用と実装を分離し、コードの再利用性と保守性を向上させます。

## 5.2 効果の定義

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

効果の定義は、関数のシグネチャの集合として表現されます。各効果操作は、その効果が提供する機能を表します。効果はパラメータ化することができ、特定の型に対して効果を定義できます。また、所有権システムと統合されており、効果操作は所有権の移動や借用を明示的に表現できます。

## 5.3 効果ハンドラの定義

効果ハンドラはtraitに対するimplと同様の構文で定義できます。効果ハンドラには3つの実装スタイルがあります：

### 5.3.1 暗黙的な継続（デフォルト）

```
// 暗黙的な継続を使用した効果ハンドラ
handler ConsoleHandler: Console {
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

暗黙的な継続スタイルでは、効果ハンドラの実装は通常の関数と同様に記述できます。継続の呼び出しは自動的に行われ、戻り値は継続に渡されます。これにより、効果ハンドラの実装が簡潔になり、多くの一般的なケースで十分です。

### 5.3.2 明示的な継続

```
// 明示的な継続を使用した効果ハンドラ
handler ExplicitConsoleHandler: Console {
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

明示的な継続スタイルでは、効果ハンドラの実装は継続を明示的に受け取り、呼び出す必要があります。これにより、継続の呼び出しタイミングや方法をより細かく制御できます。例えば、非同期処理や複雑な制御フローを実装する場合に有用です。

### 5.3.3 特殊な継続制御

```
// 継続を呼び出さない効果ハンドラ
handler ExceptionHandler<E>: Exception<E> {
  fn raise<T>(error: E): noresume T = {
    // noresume型は継続を呼び出さないことを示す
    Result.Err(error)
  }
}

// 継続を複数回呼び出す効果ハンドラ
handler ChoiceHandler: Choice {
  fn choose<T>(options: [T]): multiresume T = {
    // multiresume型は複数回の継続呼び出しを示す
    for option in options {
      // 各選択肢に対して継続を呼び出す
      resume(option) match {
        Result.Ok(value) => return value,  // 成功したら結果を返す
        Result.Err(_) => continue          // 失敗したら次の選択肢を試す
      }
    }
    // 例外ではなくResult.Errを返す
    Result.Err(ChoiceError.NoValidChoice)
  }
}
```

特殊な継続制御スタイルでは、継続を呼び出さない（`noresume`）または複数回呼び出す（`multiresume`）ことを明示的に示すことができます。これにより、例外処理や非決定性計算などの高度な制御フローパターンを表現できます。

## 5.4 ライフサイクル管理効果

Protorun言語では、リソース管理と効果システムを統合するために、ビルトインの`LifecycleEffect<R>`型を提供しています。この型を継承する効果は、リソースのライフサイクル（獲得と解放）を自動的に管理することができます。

### 5.4.1 ライフサイクル管理効果の概念と目的

ライフサイクル管理効果は、以下の目的で設計されています：

1. **安全なリソース管理**: リソースの獲得と解放を自動的に管理し、リソースリークを防止します。
2. **明示的なライフサイクル**: リソースのライフサイクル（獲得と解放）を明示的に定義します。
3. **型安全性**: リソース操作の型安全性を保証し、不正な使用を防止します。
4. **効果システムとの統合**: 代数的効果システムと統合し、リソース管理を効果として表現します。
5. **合成可能性**: 他の効果と組み合わせて使用できるようにします。

ライフサイクル管理効果は、C++のRAIIパターン、Rustの所有権システム、そして代数的効果からインスピレーションを得ています。特に：

- **自動リソース管理**: スコープベースのリソース管理により、リソースの解放忘れを防止します。これは、ガベージコレクションに依存せずに、予測可能なリソース管理を実現します。

- **明示的な解放関数**: 各ライフサイクル管理効果は明示的な解放関数（`release`）を定義する必要があります。これにより、リソースの解放方法が明確になり、リソース管理の意図が明示的になります。

- **効果としての表現**: リソース管理を効果として表現することで、効果システムの型安全性と合成可能性を活用できます。これにより、リソース管理と他の効果を自然に組み合わせることができます。

### 5.4.2 ライフサイクル管理効果の定義

ライフサイクル管理効果は、`LifecycleEffect<R>`型を継承することで定義します。この型は、リソースの獲得と解放のための標準的なインターフェースを提供します。

```
// ビルトインのLifecycleEffect型
effect LifecycleEffect<R> {
  // リソース獲得操作（固定名）
  fn acquire(): R
  
  // リソース解放操作（固定名）
  fn release(resource: R): Unit
}
```

`LifecycleEffect<R>`を継承する効果は、`acquire`と`release`メソッドを実装する必要があります。`acquire`メソッドが呼び出されると、返されるリソースは自動的に追跡され、スコープ終了時に対応する`release`メソッドが自動的に呼び出されます。

以下は、データベース接続を管理するライフサイクル管理効果の例です：

```
// データベース効果の定義
effect Database: LifecycleEffect<Connection> {
  // LifecycleEffectから継承した操作を実装
  fn acquire(): Connection = {
    // 実際の接続処理
    RealDatabase.connect(dbUrl)
  }
  
  fn release(conn: Connection): Unit = {
    // 実際の切断処理
    conn.close()
  }
  
  // 追加の効果操作
  fn query(sql: String): Result<QueryResult, DbError>
  fn execute(sql: String): Result<Unit, DbError>
  fn transaction<T, E>(action: () -> Result<T, E> & Database): Result<T, E | DbError>
}
```

ライフサイクル管理効果は、リソース管理に関連する操作だけでなく、そのリソースを使用するための追加の操作も定義できます。例えば、`Database`効果は、データベース接続の獲得と解放に加えて、クエリの実行やトランザクションの管理などの操作も提供します。

### 5.4.3 ライフサイクル管理効果の使用

ライフサイクル管理効果を使用するには、まず効果を宣言し、`acquire`メソッドを呼び出してリソースを獲得します。リソースは、関数のスコープが終了すると自動的に解放されます。

```
// 使用例
fn processUserData(userId: String): Result<UserData, Error> & Database = {
  // リソース獲得（スコープ終了時に自動的に解放される）
  let conn = Database.acquire()
  
  // クエリ実行
  let userData = Database.query(s"SELECT * FROM users WHERE id = $userId")?
  
  Result.Ok(userData)
} // connは自動的に解放される（Database.release(conn)が呼び出される）
```

ライフサイクル管理効果を使用する関数は、その効果を型シグネチャに宣言する必要があります。これにより、関数が持つ可能性のある副作用が型レベルで追跡されます。

以下は、ファイル処理を行うライフサイクル管理効果の使用例です：

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

### 5.4.4 ライフサイクル管理効果のハンドラ

ライフサイクル管理効果のハンドラは、リソースの獲得と解放の実装を提供します。ハンドラは、効果の操作を実装し、リソースの適切な管理を保証します。

```
// 効果ハンドラの実装
handler DatabaseHandler: Database {
  // LifecycleEffectから継承した操作の実装
  fn acquire(): Connection = {
    // 実際の接続処理
    RealDatabase.connect(dbUrl)
  }
  
  fn release(conn: Connection): Unit = {
    // 実際の切断処理
    conn.close()
  }
  
  // 追加の効果操作の実装
  fn query(sql: String): Result<QueryResult, DbError> = {
    // 実装...
  }
  
  fn execute(sql: String): Result<Unit, DbError> = {
    // 実装...
  }
  
  fn transaction<T, E>(action: () -> Result<T, E> & Database): Result<T, E | DbError> = {
    let conn = acquire()
    let txResult = conn.beginTransaction()
    
    // トランザクション開始に失敗した場合はエラーを返す
    if txResult.isErr() {
      return Result.Err(txResult.unwrapErr())
    }
    
    // アクションを実行
    let result = action()
    
    // 結果に基づいてコミットまたはロールバック
    match result {
      Result.Ok(_) => {
        let commitResult = conn.commit()
        if commitResult.isErr() {
          // コミットに失敗した場合はエラーを返す
          return Result.Err(commitResult.unwrapErr())
        }
      },
      Result.Err(_) => {
        // エラーの場合はロールバック
        let _ = conn.rollback() // ロールバックの失敗は無視
      }
    }
    
    // アクションの結果を返す
    result
  }
}
```

ライフサイクル管理効果のハンドラは、以下の特徴を持ちます：

1. **リソースの獲得と解放**: ハンドラは、`acquire`メソッドでリソースを獲得し、`release`メソッドでリソースを解放します。
2. **自動解放**: `acquire`メソッドで獲得したリソースは、スコープ終了時に自動的に`release`メソッドで解放されます。
3. **追加の操作**: ハンドラは、リソース管理に加えて、そのリソースを使用するための追加の操作も実装できます。

以下は、ファイルシステムのライフサイクル管理効果ハンドラの例です：

```
// ライフサイクル管理効果ハンドラを定義
handler FileSystemHandler: FileSystem {
  // LifecycleEffectから継承した操作の実装
  fn acquire(): File = {
    // ファイルを開く
    File.open("input.txt")
  }
  
  fn release(file: File): Unit = {
    // ファイルを閉じる
    file.close()
  }
  
  // 追加の効果操作の実装
  fn read(): String = {
    // 実装...
    "ファイルの内容"
  }
  
  fn write(content: String): Unit = {
    // 実装...
  }
}
```

### 5.4.5 複数のライフサイクル効果の使用

複数のライフサイクル管理効果を同時に使用することもできます。各効果は独立して管理され、それぞれのリソースは適切なタイミングで解放されます。

```
// 複数のライフサイクル効果を使用する例
fn processData(): Result<String, Error> & Database & FileSystem = {
  // データベース接続を獲得
  let conn = Database.acquire()
  
  // ファイルを開く
  let file = FileSystem.acquire()
  
  // データベースからデータを取得
  let data = Database.query("SELECT * FROM data")?
  
  // ファイルに書き込み
  FileSystem.write(data.toString())
  
  // ファイルから読み込み
  let content = FileSystem.read()
  
  Result.Ok(content)
} // conn と file は自動的に解放される（逆順）
```

複数のリソースが獲得された場合、それらは獲得された順序の逆順で解放されます。これにより、リソース間の依存関係がある場合でも、安全に解放されることが保証されます。

### 5.4.6 ライフサイクル管理効果の利点

ライフサイクル管理効果を使用することには、以下のような利点があります：

1. **リソースリークの防止**: リソースの解放忘れを防止し、リソースリークを防ぎます。
2. **型安全性**: リソース管理を型レベルで追跡し、不正な使用を防止します。
3. **コードの簡潔さ**: 明示的な解放コードを書く必要がなく、コードの簡潔さが向上します。
4. **合成可能性**: 他の効果と組み合わせて使用できるため、複雑な処理を簡潔に表現できます。
5. **例外安全性**: 例外が発生した場合でも、リソースは適切に解放されます。

ライフサイクル管理効果は、Protorun言語のリソース管理の中核となる機能であり、安全で効率的なリソース管理を実現します。

## 5.5 効果の使用

```
// 効果を使用する関数
fn greet(name: String): Unit & Console = {
  Console.log(s"こんにちは、${name}さん！")
}

// 複数の効果
fn counter(): Int & Console & State<Int> = {
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

効果を使用する関数は、関数シグネチャに`&`演算子を使用してその効果を宣言します。これにより、関数が持つ可能性のある副作用が型レベルで追跡されます。複数の効果を持つ関数は、`&`演算子を使用して効果を組み合わせることができます。また、`with`式を使用して効果のスコープを明示的に制限することもできます。

## 5.6 効果ハンドラの使用

```
// 効果ハンドラを使用した関数
fn runWithConsole<T>(action: () -> T & Console): T = {
  with ConsoleHandler: Console {
    action()
  }
}

// 状態効果のハンドラを使用
fn runWithState<S, T>(initialState: S, action: () -> T & State<S>): (T, S) = {
  var state = initialState
  
  // 状態ハンドラを定義
  handler StateHandler: State<S> {
    fn get(): S = state
    
    fn set(newState: S): Unit = {
      state = newState
    }
    
    fn modify(f: (S) -> S): Unit = {
      state = f(state)
    }
  }
  
  // ハンドラを適用
  let result = with StateHandler: State<S> {
    action()
  }
  
  (result, state)
}

// 明示的な継続を使用するハンドラ
fn runWithExplicitConsole<T>(action: () -> T & Console): T = {
  // 明示的な継続を使用するハンドラを定義
  handler ExplicitConsoleHandler: Console {
    fn log(message: String, resume: () -> Unit): Unit = {
      println(message)
      resume()
    }
    
    fn readLine(resume: (String) -> Unit): Unit = {
      let input = readLine()
      resume(input)
    }
  }
  
  with ExplicitConsoleHandler: Console {
    action()
  }
}

// 継続を呼び出さないハンドラ（Result型ベースのエラー処理）
fn runWithException<T, E>(action: () -> T & Exception<E>): Result<T, E> = {
  // 継続を呼び出さないハンドラを定義
  handler ExceptionHandler: Exception<E> {
    fn raise<R>(error: E): noresume Result<R, E> = {
      return Result.Err(error)
    }
  }
  
  // ExceptionHandlerを適用し、結果をResult型で返す
  with ExceptionHandler: Exception<E> {
    Result.Ok(action())
  }
}
```

効果ハンドラは、`with ハンドラ: 効果`構文を使用して適用されます。これにより、特定のスコープ内での効果の実装を提供します。効果ハンドラはインラインで定義することもできますし、事前に定義したハンドラを使用することもできます。効果ハンドラは、効果の使用と実装を分離し、同じ効果に対して異なる実装を提供することを可能にします。

## 5.7 効果の合成

```
// 複数の効果を扱う
fn program(): Int = {
  // 複数のハンドラを合成
  with ConsoleHandler: Console {
    with StateHandler(initialState = 0): State<Int> {
      counter()
    }
  }
}

// 効果ハンドラを指定したスコープ
fn main(): Unit = {
  // Console効果のハンドラを指定
  with ConsoleHandler: Console {
    // このスコープ内のConsole効果はConsoleHandlerでハンドルされる
    Console.log("アプリケーション開始")
    
    // State効果のハンドラを指定
    with StateHandler(initialState = defaultConfig): State<AppConfig> {
      // このスコープ内のState効果はStateHandlerでハンドルされる
      let config = State.get()
      Console.log(s"設定: $config")
      
      // アプリケーションロジック
      runApplication()
    }
    
    Console.log("アプリケーション終了")
  }
}
```

代数的効果の主要な利点の一つは、異なる効果を持つ計算を簡単に合成できることです。効果ハンドラを組み合わせることで、複雑な副作用を持つプログラムを構築できます。効果ハンドラはネストすることができ、内側のハンドラから外側のハンドラへと効果が伝播します。これにより、モジュラーで再利用可能な方法で副作用を管理できます。

## 5.8 暗黙的パラメータと効果システム

Protorun言語では、暗黙的パラメータと効果システムを統合することで、依存性注入パターンを型安全に実現できます。この統合により、効果を暗黙的に渡すことができ、コードの簡潔さと型安全性を両立できます。

### 5.8.1 暗黙的パラメータの概念と目的

暗黙的パラメータは、以下の目的で設計されています：

1. **依存性注入**: 関数が必要とする依存関係を明示的に宣言し、暗黙的に提供できるようにします。
2. **コードの簡潔さ**: 依存関係を明示的に渡す必要がなく、コードの簡潔さが向上します。
3. **型安全性**: 暗黙的パラメータは型チェックされ、必要な依存関係が提供されていることが保証されます。
4. **効果システムとの統合**: 効果を暗黙的パラメータとして使用できるようにし、効果の使用と提供を簡潔に表現できます。

暗黙的パラメータは、Scalaのimplicit parameterからインスピレーションを得ていますが、効果システムと統合することで、より安全で使いやすい依存性注入メカニズムを提供します。

### 5.8.2 暗黙的パラメータの宣言

暗黙的パラメータは、関数シグネチャの後に`(with param: Type)`構文で宣言します。これにより、関数が必要とする依存関係を明示的に宣言できます。

```
// 暗黙的パラメータを宣言
fn processData(data: String)(with logger: Logger): Result<ProcessedData, Error> = {
  logger.log("データ処理開始")
  // 処理...
  logger.log("データ処理完了")
  Result.Ok(processedData)
}
```

複数の暗黙的パラメータを宣言する場合は、カンマで区切ります：

```
fn complexOperation(data: String)(with db: Database, logger: Logger): Result<Output, Error> = {
  // dbとloggerが暗黙的に利用可能
  logger.log("操作開始")
  let result = db.query(s"SELECT * FROM data WHERE id = '${data}'")?
  logger.log("操作完了")
  Result.Ok(processResult(result))
}
```

暗黙的パラメータは、関数の型シグネチャの一部となり、コンパイラによって型チェックされます。これにより、必要な依存関係が提供されていることが保証されます。

### 5.8.3 暗黙的パラメータの提供

暗黙的パラメータは、`with`式を使用して提供します。`with`式は、特定のスコープ内で暗黙的パラメータを利用可能にします。

```
// 暗黙的パラメータの提供
with LoggerHandler: Logger {
  // この呼び出しでは、スコープ内のLoggerが暗黙的に渡される
  processData("raw data")
}
```

複数の暗黙的パラメータを提供する場合は、複数の`with`式をネストするか、カンマで区切って一つの`with`式で提供できます：

```
// 複数の暗黙的パラメータの提供（ネスト）
with LoggerHandler: Logger {
  with DatabaseHandler: Database {
    // LoggerとDatabaseが暗黙的に利用可能
    complexOperation("data123")
  }
}

// 複数の暗黙的パラメータの提供（カンマ区切り）
with LoggerHandler: Logger, DatabaseHandler: Database {
  // LoggerとDatabaseが暗黙的に利用可能
  complexOperation("data123")
}
```

暗黙的パラメータの提供は、スコープベースです。つまり、`with`式のスコープ内でのみ、その暗黙的パラメータが利用可能になります。これにより、依存関係の影響範囲を明示的に制限できます。

### 5.8.4 ライフサイクル管理効果と暗黙的パラメータの統合

ライフサイクル管理効果は、暗黙的パラメータとしても使用できます。これにより、リソース管理と依存性注入を統合できます。

```
// 暗黙的パラメータを受け取る関数
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

// 使用例
with DatabaseHandler: Database {
  // Database効果が利用可能になる
  let conn = Database.acquire()
  
  // 暗黙的パラメータとして効果を渡す
  let user = getUserById("user123")?
  displayUser(user)
}
```

この例では、`getUserById`関数は`Database`効果を暗黙的パラメータとして受け取ります。`with DatabaseHandler: Database`式によって、`Database`効果が提供され、`getUserById`関数内で使用できるようになります。

ライフサイクル管理効果と暗黙的パラメータの統合には、以下のような利点があります：

1. **リソース管理と依存性注入の統合**: リソースの獲得と解放を自動的に管理しながら、そのリソースを暗黙的に提供できます。
2. **コードの簡潔さ**: リソースの獲得と解放、および依存関係の提供を簡潔に表現できます。
3. **型安全性**: リソース管理と依存性注入が型レベルで追跡され、不正な使用を防止します。

### 5.8.5 複数の効果と暗黙的パラメータの組み合わせ

複数の効果と暗黙的パラメータを組み合わせることで、より複雑な依存関係を表現できます。

```
// 複数の効果を暗黙的に使用する関数
fn processUserOrder(userId: String, orderId: String)(with db: Database, logger: Logger, client: HttpClient): Result<OrderDetails, Error> = {
  logger.log(LogLevel.Info, s"処理開始: ユーザー $userId, 注文 $orderId")
  
  let userData = db.query(s"SELECT * FROM users WHERE id = $userId")?
  let orderData = db.query(s"SELECT * FROM orders WHERE id = $orderId")?
  
  let additionalInfo = client.get(s"https://api.example.com/orders/$orderId/details")?
  
  logger.log(LogLevel.Info, "処理完了")
  Result.Ok(combineOrderDetails(userData, orderData, additionalInfo))
}

// 使用例
fn main(): Result<Unit, Error> & Console = {
  // 各効果のハンドラを適用
  with DatabaseHandler: Database {
    with LoggerHandler: Logger {
      with HttpClientHandler: HttpClient {
        let orderDetails = processUserOrder("user123", "order456")?
        displayOrderDetails(orderDetails)
        
        Result.Ok(())
      }
    }
  } // すべてのリソースは自動的に解放される
}
```

この例では、`processUserOrder`関数は`Database`、`Logger`、`HttpClient`の3つの効果を暗黙的パラメータとして受け取ります。それぞれの効果は、対応するハンドラによって提供されます。

### 5.8.6 効果ハンドラと暗黙的パラメータの連携

効果ハンドラ自体も暗黙的パラメータを受け取ることができます。これにより、効果ハンドラの実装を柔軟に変更できます。

```
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
      // action()を実行し、結果を返す
      action()
    }
  }
}

## 5.9 代数的効果の設計上の考慮事項

代数的効果を設計する際には、以下の点を考慮することが重要です：

1. **効果の粒度**: 効果は適切な粒度で設計し、関連する操作をグループ化します。
2. **効果の合成**: 効果は合成可能に設計し、異なる効果を組み合わせて使用できるようにします。
3. **効果の局所性**: 効果の影響範囲を明示的に制限し、プログラムの理解と推論を容易にします。
4. **効果の型安全性**: 効果は型レベルで追跡され、適切なハンドラが提供されていることを保証します。
5. **効果の推論**: 効果推論を活用して、明示的な効果注釈を最小限に抑えます。

代数的効果は、副作用を型安全に管理するための強力なツールであり、関数型プログラミングの純粋性と命令型プログラミングの表現力を組み合わせることができます。
