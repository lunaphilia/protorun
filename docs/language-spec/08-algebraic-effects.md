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

効果インターフェースは、特定の計算効果が提供する操作のシグネチャの集合を定義します。**`let` 宣言**と **`effect` キーワードを用いた効果定義式**で定義します。

```protorun
// 基本的な効果インターフェース定義
let Console = effect {
  let log: (message: String) -> Unit
  let readLine: () -> String
}

// パラメータ化された効果インターフェース
let State = effect<S> {
  let get: () -> S
  let set: (newState: S) -> Unit
  let modify: (f: (S) -> S) -> Unit // 関数を受け取る操作も可能
}

// 所有権を考慮した効果インターフェース定義
let FileSystem = effect {
  // FileHandle の所有権を返す操作
  let open: (path: String, mode: FileMode) -> Result<own FileHandle, IOError>
  // FileHandle の所有権を消費する操作
  let close: (handle: own FileHandle) -> Result<Unit, IOError>
  // FileHandle の借用を使用する操作
  let read: (handle: &FileHandle) -> Result<String, IOError>
  // FileHandle の可変借用を使用する操作
  let write: (handle: &mut FileHandle, content: String) -> Result<Unit, IOError>
}

// 仮の型定義
let FileHandle = type { /* ... */ } // レコード型
let FileMode = type { Read, Write, ReadWrite } // ヴァリアント型
let IOError = type { /* ... */ } // レコード型
```

* `let EffectName = effect<GenericParams>? { ... }` は、`EffectName` という名前の効果インターフェースを定義します。
* 効果定義式 (`effect { ... }`) の内部には、その効果が提供する操作を右辺なし `LetDecl` (`let name: fn(...) -> ...`) の形式で記述します。
* 効果インターフェースは型パラメータを持つことができます (`State<S>`)。
* 操作シグネチャの型注釈 (`fn(...) -> ...`) は、所有権 (`own`, `&`, `&mut`) を含む通常の関数型と同じルールに従います。
* **重要:** 効果インターフェースの操作定義には、通常、ハンドラ固有の設定データ（例: ファイルパス、データベース接続文字列）は含めません。これらはハンドラ自身が保持します。

効果定義式の詳細な構文は [6. 式](06-expressions.md#694-効果定義式-effect) を参照してください。

## 8.3 効果ハンドラの実装 (`handler`)

効果ハンドラは、特定の効果インターフェース (`EffectName`) の操作を、特定の型 (`TargetType`) に対して実装します。**`let` 宣言**と **`handler` キーワードを用いたハンドラ定義式**で定義します。ハンドラ実装自体は通常状態を持たず、操作対象のデータ (`self`) は実装対象の型 (`TargetType`) のインスタンスとなります。

```protorun
// 効果ハンドラ実装の基本構文
let HandlerImplName = handler<GenericParams>? EffectName<EffectArgs> for TargetType<TargetArgs> {
  // 効果操作の実装 (let 束縛 + 関数式)
  // 関数式内では 'self' を通じて TargetType のインスタンスにアクセス可能
  let operation1 = fn (self, args...) -> ReturnType => {
    // self を使って TargetType のフィールドやメソッドにアクセス
    // 継続 (resume) を扱える (後述)
    // ... 実装 ...
  }
  // ... 他の操作の実装 ...
}
```

* `let HandlerImplName = handler EffectName for TargetType { ... }` は、型 `TargetType` が効果インターフェース `EffectName` を実装する方法を定義し、そのハンドラ定義を `HandlerImplName` という名前で束縛します。
* **状態の保持:** ハンドラ定義式自体は状態を持ちません。状態や設定は、実装対象の型 `TargetType` がフィールドとして保持します。
* **効果操作の実装:** `EffectName` で定義された各操作に対応する実装を、`let operationName = fn (self, ...) -> ... => ...` の形式で記述します。最初の引数 `self` は `TargetType` のインスタンス（操作対象のデータ）を受け取ります。
* **継続制御:** ハンドラ関数式の本体は、代数的効果システムの核となる **継続 (`resume`)** を扱う特別な能力を持ちます。これにより、計算の中断、再開、破棄などを制御できます（詳細は 8.6 節）。

```protorun
// 状態を持つ型の定義
let Counter = type {
  let mutable count: Int
}

// 効果インターフェース (再掲)
let State = effect<S> {
  let get: () -> S
  let set: (newState: S) -> Unit
}

// Counter 型に対して State<Int> 効果を実装
let CounterStateHandler = handler State<Int> for Counter {
  let get = fn (self) -> Int => self.count
  let set = fn (self, newState: Int) -> Unit => { self.count = newState }
}

// 状態を持たない効果の例
let Logger = effect {
  let log: (message: String) -> Unit
}
// 状態を持たない型 (例: Unit 型や空の構造体) に対して実装
let ConsoleLogger = type {} // ダミーの型
let ConsoleLoggerHandler = handler Logger for ConsoleLogger {
  let log = fn (self, message: String) -> Unit => { println(message) }
}

// 設定を持つ型の例
let FileSystemConfig = type {
  let basePath: String
}
let FileSystem = effect { // 再掲
  let open: (path: String, mode: FileMode) -> Result<own FileHandle, IOError>
  // ...
}
let FileSystemHandler = handler FileSystem for FileSystemConfig {
  let open = fn (self, path: String, mode: FileMode) -> Result<own FileHandle, IOError> => {
    let fullPath = self.basePath + "/" + path // self は FileSystemConfig インスタンス
    // ... fullPath を使ってファイルを開く処理 ...
  }
  // ...
}

// インスタンス生成 (通常の型インスタンス生成)
let counterInstance = Counter { count: 0 }
let loggerInstance = ConsoleLogger {}
let fsConfigInstance = FileSystemConfig { basePath: "/tmp/data" }
// これらのインスタンスを `with` 構文で使用する (後述)
```

このように、`handler` 定義式は特定の型 (`TargetType`) に対して効果インターフェース (`EffectName`) の操作を実装します。実際の状態や設定は `TargetType` 型のインスタンスが保持し、ハンドラ実装は `self` を通じてそれにアクセスします。

ハンドラ定義式の詳細な構文は [6. 式](06-expressions.md#695-ハンドラ定義式-handler) を参照してください。

## 8.4 Effect パラメータによる効果の宣言

関数が特定の効果操作を実行する可能性があること、および対応するハンドラ実装に依存することを宣言するために、**Effect パラメータ** 構文を使用します。

```protorun
// 関数定義: 'logger' という名前で Logger 効果の実装に依存することを宣言
let greet = fn (name: String)(effect logger: Logger) -> Unit => {
  // パラメータ名を使って効果操作を呼び出す
  logger.log(s"こんにちは、${name}さん！")
}

// 複数の Effect パラメータを持つ関数
// 'log' という名前で Logger 効果、'state' という名前で State<Int> 効果に依存
let counter = fn (effect log: Logger, effect state: State<Int>) -> Int => {
  let current = state.get()
  log.log(s"現在の値: $current")
  state.set(current + 1)
  state.get() // 新しい値を返す
}

// ファイル操作の例 (異なる実装に依存)
// 'localFs' と 'remoteFs' という名前で FileSystem の異なる実装に依存
let processFiles = fn (effect localFs: FileSystem, effect remoteFs: FileSystem) -> Result<Unit, Error> => {
  let handle1 = localFs.open("local.txt", FileMode.Read)? // localFs に束縛されたハンドラ実装が使われる
  let content = localFs.read(&handle1)? // read 操作も定義されていると仮定
  localFs.close(handle1)? // close 操作も定義されていると仮定

  let handle2 = remoteFs.open("remote/data.zip", FileMode.Write)? // remoteFs に束縛されたハンドラ実装が使われる
  remoteFs.write(&mut handle2, content)? // write 操作も定義されていると仮定
  remoteFs.close(handle2)?

  Result.Ok(())
}
```

**Effect パラメータ (`effect alias: EffectType`) の意味:**

1. **効果発生の可能性:** この関数は、`alias` という名前を通じて `EffectType` で定義された操作を実行する可能性があります。
2. **ハンドラ依存性:** この関数を呼び出す際には、`alias` という名前に対して `EffectType` を実装するハンドラを持つインスタンスが `with` 構文によって提供されている必要があります。
3. **効果操作の呼び出し:** 関数内では、`alias.operation(...)` という構文で効果操作を呼び出します。この呼び出しは、対応する `with` 構文で提供されたハンドラ実装を通じて **代数的効果のメカニズム（中断と継続処理）** を起動します。

この Effect パラメータ構文により、関数が必要とする名前付きの効果実装への依存性が明確になります。

## 8.5 効果ハンドラの提供 (`with` 構文)

関数が必要とする Effect パラメータに対して、具体的なハンドラ実装を持つインスタンスを提供（注入）するために `with` 構文を使用します。`with` 構文は、特定のスコープ内でハンドラを有効にします。

**`with` 構文:**

```protorun
// 型注釈を省略した場合 (型推論される)
with alias1 = instance1,
     alias2 = instance2,
     ...
{
  // このブロック内で、指定されたエイリアスに対応するハンドラが有効になる
  // instance1 は alias1 に対応する EffectType を実装するハンドラを持つ必要がある
  // ... Effect パラメータを持つ関数を呼び出すコード ...
}

// 型注釈を明示した場合
with alias1 = instance1: EffectType1,
     alias2 = instance2: EffectType2,
     ...
{
  // このブロック内で、指定されたエイリアスに対応するハンドラが有効になる
  // instance1 は EffectType1 を実装するハンドラを持つ必要がある
  // ... Effect パラメータを持つ関数を呼び出すコード ...
}
```

* `alias = instance`:
  * `instance`: 提供するインスタンス（値）。
  * `alias`: このインスタンスをハンドラとして束縛する名前（エイリアス）。関数側の Effect パラメータのエイリアスと対応します。
* `: EffectType`: **省略可能な型注釈**。このインスタンスをどの効果インターフェース (`EffectType`) のハンドラとして使用するかを明示的に指定します。省略された場合、`alias` に対応する Effect パラメータと `instance` の型から型推論によって決定されます。インスタンスが複数の効果インターフェースを実装している場合など、曖昧さを排除するために明示的な注釈が役立ちます。
* カンマ区切りで複数のハンドラを同時に指定できます。これによりネストが深くなるのを防ぎます。
* **スコープ:** `with` ブロック `{ ... }` 内で、指定されたエイリアスに対応する Effect パラメータを持つ関数が呼び出されると、束縛されたインスタンス (`instance`) に関連付けられたハンドラ実装（型注釈があればそれに従う）が効果処理のために使用されます。

**使用例:**

```protorun
// 型定義 (再掲)
let Counter = type { let mutable count: Int }
let ConsoleLogger = type {}
let FileSystemConfig = type { let basePath: String }
let S3Config = type { let bucket: String, let region: String }

// ハンドラ実装 (再掲)
let CounterStateHandler = handler State<Int> for Counter { /* ... */ }
let ConsoleLoggerHandler = handler Logger for ConsoleLogger { /* ... */ }
let FileSystemHandler = handler FileSystem for FileSystemConfig { /* ... */ }
// S3 用のハンドラ実装 (仮)
let S3FileSystemHandler = handler FileSystem for S3Config { /* ... */ }

// 関数定義 (再掲)
let counter = fn (effect log: Logger, effect state: State<Int>) -> Int => { /* ... */ }
let processFiles = fn (effect localFs: FileSystem, effect remoteFs: FileSystem) -> Result<Unit, Error> => { /* ... */ }

// インスタンス生成
let logger = ConsoleLogger {}
let counterState = Counter { count: 0 }
let localFsConfig = FileSystemConfig { basePath: "/tmp/data" }
let s3FsConfig = S3Config { bucket: "my-app-bucket", region: "us-east-1" }

// with を使ってインスタンスを提供・注入 (型注釈なし、推論される)
with log = logger,
     state = counterState
{
  let val1 = counter()
  let val2 = counter()
  println(s"最終値: ${val2}")
}

// 別の例: ファイルシステム (型注釈を明示)
with localFs = localFsConfig: FileSystem, // FileSystem として使用することを明示
     remoteFs = s3FsConfig: FileSystem // FileSystem として使用することを明示
{
  processFiles()?
}

// 型注釈が必要になるかもしれない例 (仮)
// もし `myInstance` が `EffectA` と `EffectB` の両方を実装していた場合
// let foo = fn (effect a: EffectA) => { ... }
// let bar = fn (effect b: EffectB) => { ... }
// with fx = myInstance: EffectA { // EffectA として使うことを明示
//   foo()
// }
// with fx = myInstance: EffectB { // EffectB として使うことを明示
//   bar()
// }
```

この `with` 構文により、効果を使うコード（関数）と効果を実装するコード（ハンドラ）を分離し、合成点（`with` ブロック）で依存性を注入するという、疎結合な設計が可能になります。型推論により簡潔に記述できますが、必要に応じて型注釈で明示性を提供することもできます。

## 8.6 継続制御

代数的効果の核心は、ハンドラが **継続 (`resume`)** を制御できる点にあります。効果操作 (`alias.operation()`) が呼び出されると、計算は中断され、制御が `with` で提供されたインスタンスに対応するハンドラのメソッド実装に移ります。このとき、ハンドラは中断箇所からの「残りの計算」を表す継続を受け取ります。

ハンドラのメソッド実装は、この継続をどう扱うかを決定できます。

### 8.6.1 暗黙的な継続呼び出し (デフォルト)

ハンドラ関数が継続 (`resume`) をパラメータとして受け取らない場合、デフォルトの動作として、関数本体の評価が完了した後に暗黙的に継続が **1回だけ** 呼び出され、関数本体の評価結果が継続に渡されます。

```protorun
// Counter 型は State<Int> を実装すると仮定
let CounterStateHandler = handler State<Int> for Counter {
  // get は resume パラメータを受け取らない
  let get = fn (self) -> Int => self.count
  // => 実行後、暗黙的に resume(self.count) が呼び出される

  // set は resume パラメータを受け取らない
  let set = fn (self, newState: Int) -> Unit => {
    self.count = newState
    // => 実行後、暗黙的に resume(()) が呼び出される
  }
}
```

このスタイルは、単純な状態変更や副作用の実行に適しており、ハンドラの実装を簡潔にします。ハンドラ関数自体の戻り値型 `R` は、継続に渡される値の型となります。

### 8.6.2 明示的な継続呼び出し

継続をより細かく制御したい場合、ハンドラ関数は継続を明示的にパラメータとして受け取ることができます。継続の型は `(R) -> S` の形式で、`R` は継続に渡す値の型、`S` は `with` 式全体の最終的な結果の型を表します。

```protorun
// ConsoleLogger 型は Logger を実装すると仮定
let ConsoleLoggerHandler = handler Logger for ConsoleLogger {
  // log 操作は Unit を返す計算の継続を受け取る
  // ハンドラ関数自体の戻り値は Unit (継続呼び出し後の値ではない)
  let log = fn (self, message: String, resume: (Unit) -> S) -> Unit => {
    println(message)
    resume(()) // 明示的に継続を呼び出す。結果は S 型。
  }
}

// Reader 効果の例
let Reader = effect<Env> { let ask: () -> Env }
// EnvProvider<Env> 型は Reader<Env> を実装すると仮定
let EnvProvider = type<Env> { environment: Env }
let ReaderHandler = handler Reader<Env> for EnvProvider<Env> {
  // ask 操作は Env を受け取る計算の継続を受け取る
  // ハンドラ関数自体の戻り値は Unit
  let ask = fn (self, resume: (Env) -> S) -> Unit => {
    let env = self.environment
    resume(env) // 環境の値を継続に渡して再開。結果は S 型。
  }
}
```

明示的な継続スタイルは、継続の呼び出しタイミングを制御したり、継続に渡す値を操作したりする場合に有用です。ハンドラ関数自体の戻り値型は、`with` 式全体の型 `S` とは通常異なります。

### 8.6.3 継続を呼び出さない (大域脱出)

ハンドラ関数が継続 `resume` を呼び出さずに値を返す場合、その値が `with` 式全体の最終的な結果となります。この場合、ハンドラ関数の戻り値型は `with` 式が期待する型 `S` と一致する必要があります。残りの計算は破棄されます。これは例外処理などで利用されます。

```protorun
// 効果インターフェース
let Exception = effect<E> { let raise: fn<T>(error: E) -> T } // T は任意の型

// ハンドラ実装 (状態は持たない例)
let ExceptionHandler = type<E> {} // ダミー型
let ExceptionHandlerImpl = handler Exception<E> for ExceptionHandler<E> {
  // raise が呼ばれたら継続を破棄し、Result.Err を返す。
  // このハンドラ関数自体の戻り値型は Result<T, E> であり、
  // これは with 式全体の期待する型と一致する必要がある。
  let raise = fn <T>(self, error: E, resume: (Nothing) -> Result<T, E>) -> Result<T, E> => {
    // resume の引数型が Nothing なのは、この継続が呼び出せないことを示す慣習。
    // 実際に resume() を呼び出さずに Result<T, E> 型の値を返す。
    Result.Err(error)
  }
}

// 使用例
let runWithException = fn <T, E>(action: (effect exc: Exception<E>) -> T) -> Result<T, E> => {
  let handlerInstance = ExceptionHandler<E> {}
  // この with 式は Result<T, E> 型の値を返す
  with exc = handlerInstance {
    // action() 内で exc.raise が呼ばれると、ハンドラの raise が実行され、
    // resume が呼び出されずに Result.Err(error) が with 式の結果となる。
    // action() が正常終了した場合、その結果 T が Result.Ok で包まれて返る。
    let result: T = action() // action の戻り値型は T
    Result.Ok(result) // 正常終了時の結果も Result<T, E> 型
  }
}
```
継続を呼び出さないことを示すために、`resume` パラメータの引数型として `Nothing` 型（または他のボトム型）を使用することが推奨されますが、必須ではありません。重要なのは、ハンドラ関数が `resume` を呼び出さずに、`with` 式が期待する型 `S` の値を返すことです。

継続の扱いは、ハンドラ関数のシグネチャ（`resume` パラメータの有無と型）と、関数本体での `resume` の呼び出し方によって決まります。

## 8.7 ライフサイクル管理効果

リソース管理（獲得と解放）は一般的な計算効果であり、代数的効果でうまくモデル化できます。Protorun は、これを支援するための規約やビルトイン機能を提供する可能性があります。

**アプローチ：ハンドラがリソースを管理**

新しい設計モデルでは、ハンドラインスタンスが状態を持つため、リソース（ファイルハンドル、データベース接続など）をハンドラインスタンスのフィールドとして保持し、効果操作を通じてそのリソースを操作するのが自然です。

```protorun
// 効果インターフェース (リソース獲得・解放操作を含む)
let ManagedResource = effect<R> {
  let acquire: fn() -> Result<R, Error> // リソースを獲得 (あるいは参照を返す？)
  let useResource: fn(resource: &R, /* ... */) -> Result<Unit, Error>
  let release: fn(resource: R) -> Result<Unit, Error> // 明示的な解放？
}

// ハンドラが実装される型 (リソースと状態を保持)
let ResourceManager = type<R> {
  let resourceConfig: ResourceConfig // リソース生成に必要な設定
  let mutable resourceInstance: Option<R> = None // 保持するリソースインスタンス
}
// ハンドラ実装
let ResourceManagerHandler = handler ManagedResource<R> for ResourceManager<R> {
  let acquire = fn (self) -> Result<R, Error> => {
    if self.resourceInstance.isSome() {
      // 既に獲得済みの場合のエラー処理など
    }
    // self.resourceConfig を使ってリソースを生成
    let newResource = createResource(self.resourceConfig)?
    self.resourceInstance = Some(newResource)
    // R を返すか、あるいは内部で保持するだけか？ 要検討
    Ok(newResource) // 仮: R を返す
  }

  let useResource = fn (resource: &R, /* ... */) -> Result<Unit, Error> => {
    // resource を使った処理
  }

  let release = fn (resource: R) -> Result<Unit, Error> => {
    // リソースを解放する処理
    destroyResource(resource)?
    self.resourceInstance = None
    Ok(())
  }
}
```

**自動解放 (RAII) との連携:**

C++ の RAII や Rust の `Drop` トレイトのように、スコープを抜けたら自動的にリソースが解放される仕組みと代数的効果をどう連携させるかは重要な設計課題です。

* **`with` ブロックと連動:** `with alias = instance: EffectType { ... }` または `with alias = instance { ... }` ブロックを抜ける際に、`instance` が持つリソース（例えば `release` メソッドを持つフィールドや、`Drop` のような特別なトレイトを実装している場合）を自動的に解放する、というルールを導入することが考えられます。
* **`LifecycleEffect` のような規約:** 特定のインターフェース（例: `acquire`/`release` を持つ `LifecycleEffect`）をハンドラが実装していれば、`with` が自動解放を試みる、という方法も考えられます。

この領域は、所有権システムとの連携も含め、さらなる詳細な設計が必要です。

## 8.8 Effect パラメータと依存性注入 (再掲)

Effect パラメータ (`effect alias: EffectType`) と `with` 構文 (`with alias = instance: EffectType` または `with alias = instance`) は、依存性注入パターンを型安全かつ明示的に実現するための主要なメカニズムとなります。

これにより、関数は抽象的な効果インターフェースにのみ依存し、具体的な実装（ハンドラ）は呼び出し側（`with` ブロック）で注入されるため、疎結合でテストしやすいコードを書くことができます。

## 8.9 設計上の考慮事項 (更新)

代数的効果とハンドラを設計する際には、以下の点を考慮することが重要です：

1. **効果の粒度**: 効果インターフェースは適切な粒度で設計し、関連する操作をグループ化します。
2. **ハンドラの責務**: ハンドラ実装は効果の操作ロジックを担当します。必要な状態や設定は、ハンドラが実装される型 (`TargetType`) のインスタンスが管理します。
3. **データと効果の分離**: 効果インターフェースの操作シグネチャ (`let name: fn(...) -> ...`) には、ハンドラ固有のデータを含めず、抽象性を保ちます。データはハンドラが実装される型のインスタンス (`self`) が保持するか、操作の引数として渡されるべきかを慎重に検討します（通常は `self` が保持する方がカプセル化に適しています）。
4. **継続制御の選択**: ハンドラの各操作実装において、継続をどう扱うか（暗黙的、明示的、呼び出さない）を、関数実装のシグネチャ (`fn (...) -> ... => ...` における `resume` パラメータの有無と型）と `resume` の呼び出し方によって適切に表現します。
5. **合成可能性**: 異なる効果を持つインスタンスを `with` 構文で容易に組み合わせられるように設計します。
6. **型安全性**: Effect パラメータと `with` 構文（およびオプションの型注釈）により、ハンドラ実装を持つインスタンスの依存関係と提供が型レベルでチェックされることを保証します。
7. **パフォーマンス**: （言語実装の課題として）効果処理メカニズム（中断、継続キャプチャ、ハンドラ呼び出し）のオーバーヘッドを最小限に抑える最適化が重要です。

代数的効果は、計算効果を構造化し、型安全に管理するための強力なツールであり、関数型プログラミングの純粋性と命令型プログラミングの表現力や状態管理を組み合わせるための有望なアプローチです。
