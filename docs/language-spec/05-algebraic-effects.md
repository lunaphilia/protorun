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

暗黙的な継続スタイルでは、効果ハンドラの実装は通常の関数と同様に記述できます。継続の呼び出しは自動的に行われ、戻り値は継続に渡されます。これにより、効果ハンドラの実装が簡潔になり、多くの一般的なケースで十分です。

### 5.3.2 明示的な継続

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

明示的な継続スタイルでは、効果ハンドラの実装は継続を明示的に受け取り、呼び出す必要があります。これにより、継続の呼び出しタイミングや方法をより細かく制御できます。例えば、非同期処理や複雑な制御フローを実装する場合に有用です。

### 5.3.3 特殊な継続制御

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

特殊な継続制御スタイルでは、継続を呼び出さない（`noresume`）または複数回呼び出す（`multiresume`）ことを明示的に示すことができます。これにより、例外処理や非決定性計算などの高度な制御フローパターンを表現できます。

## 5.4 ライフサイクル管理を持つ効果

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

ライフサイクル管理を持つ効果は、リソースの獲得と解放を自動的に管理するための特別な種類の効果です。`with lifecycle`キーワードを使用して定義され、リソースのライフサイクルを追跡し、スコープ終了時に自動的に解放することを保証します。これにより、リソースリークを防ぎ、安全なリソース管理を実現します。

## 5.5 効果の使用

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

効果を使用する関数は、関数シグネチャに`with`キーワードを使用してその効果を宣言します。これにより、関数が持つ可能性のある副作用が型レベルで追跡されます。複数の効果を持つ関数は、`&`演算子を使用して効果を組み合わせることができます。また、`with`式を使用して効果のスコープを明示的に制限することもできます。

## 5.6 効果ハンドラの使用

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

効果ハンドラは、`with ... handled by ...`構文を使用して適用されます。これにより、特定のスコープ内での効果の実装を提供します。効果ハンドラはインラインで定義することもできますし、事前に定義したハンドラを使用することもできます。効果ハンドラは、効果の使用と実装を分離し、同じ効果に対して異なる実装を提供することを可能にします。

## 5.7 効果の合成

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
```

代数的効果の主要な利点の一つは、異なる効果を持つ計算を簡単に合成できることです。効果ハンドラを組み合わせることで、複雑な副作用を持つプログラムを構築できます。効果ハンドラはネストすることができ、内側のハンドラから外側のハンドラへと効果が伝播します。これにより、モジュラーで再利用可能な方法で副作用を管理できます。

## 5.8 代数的効果の設計上の考慮事項

代数的効果を設計する際には、以下の点を考慮することが重要です：

1. **効果の粒度**: 効果は適切な粒度で設計し、関連する操作をグループ化します。
2. **効果の合成**: 効果は合成可能に設計し、異なる効果を組み合わせて使用できるようにします。
3. **効果の局所性**: 効果の影響範囲を明示的に制限し、プログラムの理解と推論を容易にします。
4. **効果の型安全性**: 効果は型レベルで追跡され、適切なハンドラが提供されていることを保証します。
5. **効果の推論**: 効果推論を活用して、明示的な効果注釈を最小限に抑えます。

代数的効果は、副作用を型安全に管理するための強力なツールであり、関数型プログラミングの純粋性と命令型プログラミングの表現力を組み合わせることができます。
