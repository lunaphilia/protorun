# 9. 例：サンプルプログラム

## 9.1 簡単な計算機

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

## 9.2 状態を持つカウンター

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

## 9.3 ファイル処理（ライフサイクル管理効果を使用）

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
