# 8. 代数的効果

## 8.1 代数的効果の概念と目的

代数的効果は、計算効果（副作用や特殊な制御フロー）を型安全に表現し、モジュール化して扱うための Protorun 言語の中核機能です。この機能は以下の目的で設計されています：

1. **型安全な効果**: 関数が持つ可能性のある計算効果を型レベルで追跡し、安全に制御します。
2. **効果の分離**: 純粋な計算ロジックと計算効果（副作用の実装や制御フローの操作）を明確に分離します。
3. **合成可能性**: 異なる効果を持つ計算を安全に合成できるようにします。
4. **局所的な効果ハンドリング**: 効果の影響範囲（どのハンドラで処理されるか）を明示的に指定し、プログラムの理解と推論を容易にします。
5. **継続ベースの制御フロー**: 例外処理、非同期処理、非決定性計算、状態管理などの高度な制御フローパターンを統一的に表現します。

代数的効果は、モナドのような他の計算効果管理メカニズムと比較して、より直接的で合成しやすいアプローチを提供することを目指します。効果ハンドラを通じて効果の実装を提供することで、効果の使用（インターフェース）と実装（ハンドラ）を分離し、コードの再利用性、テスト容易性、保守性を向上させます。

## 8.2 効果インターフェースの定義 (`effect`)

効果インターフェースは、特定の計算効果が提供する操作のシグネチャの集合を定義します。`effect` キーワードを用いて定義します。

```protorun
// 基本的な効果インターフェース定義
effect Console {
  fn log(message: String): Unit
  fn readLine(): String
}

// パラメータ化された効果インターフェース
effect State<S> {
  fn get(): S
  fn set(newState: S): Unit
  fn modify(f: (S) -> S): Unit // 関数を受け取る操作も可能
}

// 所有権を考慮した効果インターフェース定義
effect FileSystem {
  // FileHandle の所有権を返す操作
  fn open(path: String, mode: FileMode): Result<own FileHandle, IOError>
  // FileHandle の所有権を消費する操作
  fn close(handle: own FileHandle): Result<Unit, IOError>
  // FileHandle の借用を使用する操作
  fn read(handle: &FileHandle): Result<String, IOError>
  // FileHandle の可変借用を使用する操作
  fn write(handle: &mut FileHandle, content: String): Result<Unit, IOError>
}

// 仮の型定義
type FileHandle { /* ... */ }
type FileMode { Read, Write, ReadWrite }
type IOError { /* ... */ }
```

* `effect EffectName { ... }` は、`EffectName` という名前の効果インターフェースを定義します。
* 内部には、その効果が提供する操作（関数シグネチャ）を記述します。
* 効果インターフェースは型パラメータを持つことができます (`State<S>`)。
* 操作シグネチャは、所有権 (`own`, `&`, `&mut`) を含む通常の関数シグネチャと同じルールに従います。
* **重要:** 効果インターフェースの操作定義には、通常、ハンドラ固有の設定データ（例: ファイルパス、データベース接続文字列）は含めません。これらはハンドラ自身が保持します。

## 8.3 ハンドラ型の定義 (`handler`)

ハンドラ型は、特定の効果インターフェース (`EffectType`) の操作を実装し、かつ自身の状態や設定データを保持できる新しい型を定義します。`handler` キーワードを用いて定義します。

```protorun
// ハンドラ型定義の基本構文
handler HandlerName: EffectType {
  // フィールド定義 (通常の type と同じ構文)
  let field1: Type1
  let mutable field2: Type2
  // ...

  // 効果操作の実装 (let 束縛 + 関数式)
  let operation1 = fn (args...): ReturnType => {
    // self を使ってフィールドにアクセス可能
    // 継続 (resume) を扱える (後述)
    // ... 実装 ...
  }
  // ... 他の操作の実装 ...
}
```

* `handler HandlerName: EffectType { ... }` は、`HandlerName` という名前の新しい型を定義します。この型は `EffectType` インターフェースを実装します。
* **フィールド定義:** ハンドラが状態や設定を保持するために、内部にフィールドを定義できます。フィールド定義の構文は、通常の `type` 宣言（レコード形式など）と同じです。
* **効果操作の実装:** `EffectType` で定義された各操作に対応する実装を、`let operationName = fn (...) => ...` の形式で記述します。関数式内では `self` を通じてインスタンスのフィールドにアクセスできます。
* **継続制御:** ハンドラ関数式の本体は、代数的効果システムの核となる **継続 (`resume`)** を扱う特別な能力を持ちます。これにより、計算の中断、再開、破棄などを制御できます（詳細は 8.6 節）。

**ハンドラインスタンスの生成:**

ハンドラ型のインスタンスは、通常の `type` と同じ **レコードリテラル構文** を使って生成・初期化します。

```protorun
// 状態を持たないハンドラ型の例
handler ConsoleHandler: Console {
  // フィールドなし

  let log = fn (message: String): Unit => { println(message) }
  let readLine = fn (): String => { readLineFromStdio() } // 仮の関数
}
// インスタンス生成 (フィールドがないので空のレコードリテラル)
let consoleHandlerInstance = ConsoleHandler {}

// 状態を持つハンドラ型の例
handler StateHandler<S>: State<S> {
  let mutable state: S // フィールド定義

  // 効果操作の実装
  let get = fn (): S => self.state
  let set = fn (newState: S): Unit => { self.state = newState }
  let modify = fn (f: (S) -> S): Unit => { self.state = f(self.state) }
}
// インスタンス生成 (フィールドを初期化)
let stateHandlerInstance = StateHandler<Int> { state: 0 }

// 設定を持つハンドラ型の例
handler LocalFileHandler: FileSystem {
  let basePath: String // 設定フィールド

  let open = fn (path: String, mode: FileMode): Result<own FileHandle, IOError> => {
    let fullPath = self.basePath + "/" + path // フィールドを使用
    // ... fullPath を使ってファイルを開く処理 ...
  }
  // ... 他の操作の実装 (close, read, write) ...
}
// インスタンス生成 (設定を渡して初期化)
let localFsHandlerInstance = LocalFileHandler { basePath: "/tmp/data" }
```

このように、`handler` は状態と振る舞い（効果実装）をカプセル化した型を定義し、そのインスタンスが実際の効果処理を担当します。

## 8.4 Effect パラメータによる効果の宣言

関数が特定の効果操作を実行する可能性があること、および対応するハンドラ実装に依存することを宣言するために、**Effect パラメータ** 構文を使用します。

```protorun
// 関数定義: 'console' という名前で Console 効果の実装に依存することを宣言
fn greet(name: String)(effect console: Console): Unit {
  // パラメータ名を使って効果操作を呼び出す
  console.log(s"こんにちは、${name}さん！")
}

// 複数の Effect パラメータを持つ関数
// 'log' という名前で Console 効果、'state' という名前で State<Int> 効果に依存
fn counter(effect log: Console, effect state: State<Int>): Int {
  let current = state.get()
  log.log(s"現在の値: $current")
  state.set(current + 1)
  state.get() // 新しい値を返す
}

// ファイル操作の例 (異なる実装に依存)
// 'localFs' と 'remoteFs' という名前で FileEffect の異なる実装に依存
fn processFiles(effect localFs: FileEffect, effect remoteFs: FileEffect): Result<Unit, Error> {
  let handle1 = localFs.open("local.txt", FileMode.Read)? // localFs に束縛されたハンドラが使われる
  let content = localFs.read(&handle1)?
  localFs.close(handle1)?

  let handle2 = remoteFs.open("remote/data.zip", FileMode.Write)? // remoteFs に束縛されたハンドラが使われる
  remoteFs.write(&mut handle2, content)?
  remoteFs.close(handle2)?

  Result.Ok(())
}
```

**Effect パラメータ (`effect alias: EffectType`) の意味:**

1. **効果発生の可能性:** この関数は、`alias` という名前を通じて `EffectType` で定義された操作を実行する可能性があります。
2. **ハンドラ依存性:** この関数を呼び出す際には、`alias` という名前に対して `EffectType` を実装するハンドラインスタンスが提供されている必要があります。
3. **効果操作の呼び出し:** 関数内では、`alias.operation(...)` という構文で効果操作を呼び出します。この呼び出しは、定義により **代数的効果のメカニズム（中断と継続処理）** を起動します。

この Effect パラメータ構文により、関数が必要とする名前付きの効果実装への依存性が明確になり、従来の `& EffectType` という戻り値型への注釈は不要になります。

## 8.5 効果ハンドラインスタンスの提供 (`with` 構文)

関数が必要とする Effect パラメータに対して、具体的なハンドラインスタンスを提供（注入）するために `with` 構文を使用します。`with` 構文は、特定のスコープ内でハンドラを有効にします。

**`with` 構文:**

```protorun
with alias1 = handlerInstance1: EffectType1,
     alias2 = handlerInstance2: EffectType2,
     ...
{
  // このブロック内で、指定されたエイリアスに対応するハンドラが有効になる
  // ... Effect パラメータを持つ関数を呼び出すコード ...
}
```

* `alias = handlerInstance: EffectType`:
  * `handlerInstance`: 提供するハンドラ型のインスタンス（値）。`HandlerType { field: value }` のようにその場で生成することも、既存の変数を使うこともできます。
  * `alias`: このハンドラインスタンスを束縛する名前（エイリアス）。関数側の Effect パラメータのエイリアスと対応します。
  * `EffectType`: このハンドラが実装する効果インターフェース。型チェックに使われます。
* カンマ区切りで複数のハンドラを同時に指定できます。これによりネストが深くなるのを防ぎます。
* **スコープ:** `with` ブロック `{ ... }` 内で、指定されたエイリアスに対応する Effect パラメータを持つ関数が呼び出されると、束縛されたハンドラインスタンスが効果処理のために使用されます。

**使用例:**

```protorun
// ハンドラ型定義 (再掲)
handler ConsoleHandler: Console { /* ... */ }
handler StateHandler<S>: State<S> { let mutable state: S }

// 関数定義 (再掲)
fn counter(effect log: Console, effect state: State<Int>): Int { /* ... */ }

// with を使ってハンドラインスタンスを提供・注入
with log = ConsoleHandler {}, // ConsoleHandler インスタンスを 'log' に束縛
     state = StateHandler<Int> { state: 0 } // StateHandler インスタンスを 'state' に束縛
     : Console, State<Int> // 型を明示 (省略可能か？要検討)
{
  let val1 = counter() // counter は log と state ハンドラを使って実行される
  let val2 = counter()
  println(s"最終値: ${val2}")
}

// 別の例: ファイルシステム
handler LocalFileHandler: FileSystem { let basePath: String }
handler S3FileHandler: FileSystem { let bucket: String, let region: String }
fn processFiles(effect localFs: FileEffect, effect remoteFs: FileEffect): Result<Unit, Error> { /* ... */ }

let localConfig = "/tmp/data"
let s3Config = ("my-app-bucket", "us-east-1")

with localFs = LocalFileHandler { basePath: localConfig },
     remoteFs = S3FileHandler { bucket: s3Config.0, region: s3Config.1 }
     : FileEffect // 同じ EffectType に複数のハンドラを異なるエイリアスで束縛
{
  processFiles()?
}
```

この `with` 構文により、効果を使うコード（関数）と効果を実装するコード（ハンドラ）を分離し、合成点（`with` ブロック）で依存性を注入するという、疎結合な設計が可能になります。

## 8.6 継続制御

代数的効果の核心は、ハンドラが **継続 (`resume`)** を制御できる点にあります。効果操作 (`alias.operation()`) が呼び出されると、計算は中断され、制御が対応するハンドラのメソッド実装に移ります。このとき、ハンドラは中断箇所からの「残りの計算」を表す継続を受け取ります。

ハンドラのメソッド実装は、この継続をどう扱うかを決定できます。

### 8.6.1 暗黙的な継続（デフォルト）

ハンドラ関数式の本体で継続を明示的に扱わない場合、デフォルトの動作として、関数式の評価が完了した後に暗黙的に継続が **1回だけ** 呼び出され、関数式の評価結果が継続に渡されます。

```protorun
handler SimpleStateHandler<S>: State<S> {
  let mutable state: S
  let get = fn (): S => self.state // 戻り値 state が暗黙的に継続に渡される
  let set = fn (newState: S): Unit => {
    self.state = newState
    // 何も返さない (Unit) が、暗黙的に継続が呼び出される
  }
}
```

このスタイルは、単純な状態変更や副作用の実行に適しており、コードが簡潔になります。

### 8.6.2 明示的な継続

継続をより細かく制御したい場合、ハンドラ関数式は継続を明示的にパラメータとして受け取ることができます。継続の型は、残りの計算が期待する入力と出力を反映します。

```protorun
handler ExplicitConsoleHandler: Console {
  // log 操作は Unit を返す計算の継続を受け取る
  let log = fn (message: String, resume: () -> Unit): Unit => {
    println(message)
    resume() // 明示的に継続を呼び出す
  }

  // readLine 操作は String を受け取る計算の継続を受け取る
  let readLine = fn (resume: (String) -> Unit): Unit => {
    let input = readLineFromStdio() // 仮の関数
    resume(input) // 読み取った値を継続に渡して再開
  }
}
```

明示的な継続スタイルは、非同期処理や、継続の呼び出しタイミングを制御したい場合に有用です。

### 8.6.3 特殊な継続制御 (`noresume`, `multiresume`)

ハンドラ関数式は、継続の呼び出し方をさらに特殊化できます。これは、関数式のシグネチャ（戻り値型）で指定します。

* **継続を呼び出さない (`noresume`)**: 例外処理（大域脱出）のように、残りの計算を実行せずに処理を終了する場合に使います。関数式の戻り値型に `noresume` (またはそれに類する表現) を付けます。

    ```protorun
    effect Exception<E> { fn raise<T>(error: E): T } // T は任意の型

    handler ExceptionHandler<E>: Exception<E> {
      // raise が呼ばれたら継続を破棄し、Result.Err を返す
      let raise = fn <T>(error: E): noresume Result<T, E> => {
        Result.Err(error) // resume() を呼び出さない
      }
    }

    // 使用例
    fn runWithException<T, E>(action: (effect exc: Exception<E>) -> T): Result<T, E> = {
      with exc = ExceptionHandler<E>(): Exception<E> {
        // action() 内で exc.raise が呼ばれると、この with ブロック全体が Err を返す
        Result.Ok(action(effect exc = exc))
      }
    }
    ```

* **継続を複数回呼び出す (`multiresume`)**: 非決定性計算（バックトラック）のように、残りの計算を異なる状態で複数回試す場合に使います。関数式の戻り値型に `multiresume` (またはそれに類する表現) を付けます。

    ```protorun
    effect Choice { fn choose<T>(options: [T]): T }

    handler ChoiceHandler: Choice {
      // choose が呼ばれたら、各選択肢で継続を試す
      let choose = fn <T>(options: [T]): multiresume T => {
        for option in options {
          // 各選択肢で継続を呼び出し、結果を試す (Result<T, ResumeAgain> のような型が必要か？)
          let result = resume(option)
          // もし result が成功ならそれを返し、そうでなければ次の選択肢へ
          // ... (詳細なメカニズムは要検討) ...
        }
        // すべて失敗した場合の処理
      }
    }
    ```

    (`multiresume` の正確なセマンティクスと型付けは複雑であり、さらなる検討が必要です。)

これらの継続制御能力により、代数的効果は非常に表現力の高い計算効果管理メカニズムとなります。

## 8.7 ライフサイクル管理効果

リソース管理（獲得と解放）は一般的な計算効果であり、代数的効果でうまくモデル化できます。Protorun は、これを支援するための規約やビルトイン機能を提供する可能性があります。

**アプローチ：ハンドラがリソースを管理**

新しい設計モデルでは、ハンドラインスタンスが状態を持つため、リソース（ファイルハンドル、データベース接続など）をハンドラインスタンスのフィールドとして保持し、効果操作を通じてそのリソースを操作するのが自然です。

```protorun
// 効果インターフェース (リソース獲得・解放操作を含む)
effect ManagedResource<R> {
  fn acquire(): Result<R, Error> // リソースを獲得 (あるいは参照を返す？)
  fn useResource(resource: &R, /* ... */): Result<Unit, Error>
  fn release(resource: R): Result<Unit, Error> // 明示的な解放？
}

// ハンドラ型 (リソースと状態を保持)
handler ResourceManager<R>: ManagedResource<R> {
  let resourceConfig: ResourceConfig // リソース生成に必要な設定
  let mutable resourceInstance: Option<R> = None // 保持するリソースインスタンス

  fn acquire(): Result<R, Error> = {
    if self.resourceInstance.isSome() {
      // 既に獲得済みの場合のエラー処理など
    }
    // self.resourceConfig を使ってリソースを生成
    let newResource = createResource(self.resourceConfig)?
    self.resourceInstance = Some(newResource)
    // R を返すか、あるいは内部で保持するだけか？ 要検討
    Ok(newResource) // 仮: R を返す
  }

  fn useResource(resource: &R, /* ... */): Result<Unit, Error> = {
    // resource を使った処理
  }

  fn release(resource: R): Result<Unit, Error> = {
    // リソースを解放する処理
    destroyResource(resource)?
    self.resourceInstance = None
    Ok(())
  }
}
```

**自動解放 (RAII) との連携:**

C++ の RAII や Rust の `Drop` トレイトのように、スコープを抜けたら自動的にリソースが解放される仕組みと代数的効果をどう連携させるかは重要な設計課題です。

* **`with` ブロックと連動:** `with alias = handlerInstance: EffectType { ... }` ブロックを抜ける際に、`handlerInstance` が持つリソース（例えば `release` メソッドを持つフィールド）を自動的に解放する、というルールを導入することが考えられます。
* **`LifecycleEffect` のような規約:** 特定のインターフェース（例: `acquire`/`release` を持つ `LifecycleEffect`）をハンドラが実装していれば、`with` が自動解放を試みる、という方法も考えられます。

この領域は、所有権システムとの連携も含め、さらなる詳細な設計が必要です。

## 8.8 Effect パラメータと依存性注入 (再掲)

Effect パラメータ (`effect alias: EffectType`) と拡張された `with` 構文 (`with alias = Handler(): EffectType, ...`) は、依存性注入パターンを型安全かつ明示的に実現するための主要なメカニズムとなります。

これにより、関数は抽象的な効果インターフェースにのみ依存し、具体的な実装（ハンドラ）は呼び出し側（`with` ブロック）で注入されるため、疎結合でテストしやすいコードを書くことができます。

## 8.9 設計上の考慮事項 (更新)

代数的効果とハンドラを設計する際には、以下の点を考慮することが重要です：

1. **効果の粒度**: 効果インターフェースは適切な粒度で設計し、関連する操作をグループ化します。
2. **ハンドラの責務**: ハンドラは効果の実装と、必要な状態や設定の管理を担当します。状態を持つハンドラと持たないハンドラを適切に使い分けます。
3. **データと効果の分離**: 効果インターフェースの操作シグネチャには、ハンドラ固有のデータを含めず、抽象性を保ちます。データはハンドラインスタンスが保持するか、操作の引数として渡されるべきかを慎重に検討します（通常はハンドラが保持する方が疎結合）。
4. **継続制御の選択**: ハンドラの各操作実装において、継続をどう扱うか（暗黙的、明示的、`noresume`, `multiresume`）を適切に選択します。
5. **合成可能性**: 異なる効果やハンドラを `with` 構文で容易に組み合わせられるように設計します。
6. **型安全性**: Effect パラメータと `with` 構文により、ハンドラの依存関係と提供が型レベルでチェックされることを保証します。
7. **パフォーマンス**: （言語実装の課題として）効果処理メカニズム（中断、継続キャプチャ、ハンドラ呼び出し）のオーバーヘッドを最小限に抑える最適化が重要です。

代数的効果は、計算効果を構造化し、型安全に管理するための強力なツールであり、関数型プログラミングの純粋性と命令型プログラミングの表現力や状態管理を組み合わせるための有望なアプローチです。
