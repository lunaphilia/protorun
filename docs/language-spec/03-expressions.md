# 4. 式と文

## 4.1 式と文の概要と設計原則

Protorun言語は、式ベースの言語設計を採用しています。これは、ほとんどの構文要素が値を返す式として実装されていることを意味します。この設計は以下の原則に基づいています：

1. **一貫性**: 同様の構文要素は同様の方法で扱われるべきです
2. **合成性**: 式は他の式の中に自然に組み込めるべきです
3. **表現力**: 複雑な計算を簡潔かつ読みやすく表現できるべきです
4. **参照透過性**: 純粋な式は、同じ入力に対して常に同じ結果を返すべきです

式ベースの設計は、関数型プログラミングの原則に沿っており、コードの推論と最適化を容易にします。また、副作用を明示的に扱うことで、プログラムの理解と保守が容易になります。

## 4.2 変数宣言

```
// 不変変数（デフォルト）
let x = 42
let y: Int = 42

// 可変変数
var counter = 0
var name: String = "John"
```

Protorun言語の変数宣言は、以下の原則に基づいて設計されています：

1. **不変性の優先**: デフォルトでは変数は不変（`let`）であり、これにより予測可能性と安全性が向上します
2. **明示的な可変性**: 可変変数は`var`キーワードで明示的に宣言し、意図を明確にします
3. **型推論**: 型注釈は省略可能であり、コンパイラが文脈から型を推論します
4. **静的型付け**: すべての変数は明確な型を持ち、型安全性が保証されます

不変変数をデフォルトにする設計決定は、関数型プログラミングの原則に基づいています。不変性により、コードの理解と推論が容易になり、並行処理における安全性も向上します。一方で、可変性が必要な場合には`var`キーワードを使用することで、その意図を明示的に示すことができます。

型推論により、冗長な型注釈を省略できますが、必要に応じて明示的に型を指定することも可能です。これにより、コードの簡潔さと明確さのバランスを取ることができます。

## 4.3 関数定義

```
// 基本的な関数定義
fn add(a: Int, b: Int): Int = {
  a + b
}

// 単一式の場合は中括弧を省略可能
fn square(x: Int): Int = x * x

// 効果付き関数
fn log(message: String): Unit & Console = {
  Console.log(message)
}

// ジェネリック関数
fn identity<T>(x: T): T = x

// 複数のパラメータリスト（カリー化）
fn add(a: Int)(b: Int): Int = a + b
```

Protorun言語の関数定義は、以下の原則に基づいて設計されています：

1. **一等の値**: 関数は一等の値であり、変数に代入したり、他の関数に渡したりできます
2. **型安全性**: 関数のパラメータと戻り値の型は静的に型チェックされます
3. **効果の明示**: 関数が持つ副作用は型シグネチャに明示的に記述されます
4. **多相性**: ジェネリックパラメータにより、型に依存しない汎用的なコードを書くことができます
5. **簡潔性**: 単一式の関数は中括弧を省略でき、コードの簡潔さを向上させます

特に重要な特徴：

- **効果注釈**: 関数が持つ副作用を`&`演算子で明示することで、関数の振る舞いが明確になり、予期しない副作用を防止します。これは、純粋関数型プログラミングの原則と実用性のバランスを取るための設計決定です。

- **ジェネリック関数**: 型パラメータを使用することで、様々な型に対して動作する汎用的な関数を定義できます。これにより、コードの再利用性が向上し、型安全性を維持しながら抽象化が可能になります。

- **カリー化**: 複数のパラメータリストをサポートすることで、部分適用や関数合成などの関数型プログラミングパターンを自然に表現できます。これにより、より表現力豊かなコードを書くことができます。

## 4.4 制御構造

```
// if式
if condition {
  // then部
} else {
  // else部
}

// match式
match value {
  pattern1 => expression1,
  pattern2 if guard => expression2,
  _ => defaultExpression
}

// コレクションリテラル内包表記
[x * 2 for x <- numbers if x % 2 == 0]
{k.toUpperCase() -> v * 2 for (k, v) <- map if v > 0}
#{x * x for x <- range(1, 10)}

// bind式（モナド連鎖）
bind {
  user <- findUser(userId)
  email <- getUserEmail(user)
  validEmail <- validateEmail(email)
  validEmail
}

// with式（効果スコープ）
with Console {
  Console.log("このスコープ内でConsole効果を使用可能")
}

// with式（効果ハンドラを指定）
with ConsoleHandler: Console {
  Console.log("このスコープ内のConsole効果はConsoleHandlerでハンドル")
}

// with式（効果の暗黙的な提供）
with DatabaseHandler: Database {
  // Database効果が暗黙的に利用可能になる
  processUserData("user123")
}

// with式（複数の効果の暗黙的な提供）
with DatabaseHandler: Database, LoggerHandler: Logger {
  // Database効果とLogger効果が暗黙的に利用可能になる
  processUserOrder("user123", "order456")
}

// 効果のスコープ化
with scoped Logger {
  // このスコープ内でのみ有効な効果の実装
  fn log(message: String): Unit = {
    println(s"[LOG] $message")
  }
  
  // 効果を使用するコード
  Logger.log("Starting process...")
  // 処理...
  Logger.log("Process complete")
}
```

Protorun言語の制御構造は、以下の原則に基づいて設計されています：

1. **式ベース**: ほとんどの制御構造は値を返す式として実装されています
2. **型安全性**: 制御構造は型チェックされ、型の整合性が保証されます
3. **表現力**: 様々な計算パターンを自然に表現できる豊富な制御構造を提供します
4. **効果の制御**: 効果のスコープと実装を明示的に制御する構造を提供します

特に重要な特徴：

- **if式とmatch式の二重性**: 両方の構造を維持することで、単純な条件分岐（if）と複雑なパターンマッチング（match）の両方に最適化された構文を提供します。これは、使いやすさと表現力のバランスを取るための設計決定です。

- **コレクションリテラル内包表記**: コレクション操作を簡潔に表現するための構文です。Pythonの内包表記からインスピレーションを得ており、コレクションの種類（リスト、マップ、セット）に応じた構文を提供します。

- **bind式**: モナド的な連鎖を表現するための構文です。Option、Result、Futureなどのモナド的な型の連鎖に最適化されています。Haskellのdoノーテーションからインスピレーションを得ていますが、より明示的な名前を使用しています。

- **with式**: 効果のスコープとハンドラを制御するための構文です。これにより、効果の影響範囲を明示的に制限し、効果の実装を提供することができます。これは、代数的効果システムの中核となる機能です。with式はブロック内の最後の式の評価結果を返します。

- **効果のスコープ化**: 効果の実装を局所的に提供するための構文です。これにより、効果の実装を必要な場所に限定し、グローバルな状態の変更を避けることができます。これは、効果システムの柔軟性と安全性を向上させるための設計決定です。

### 4.4.1 コレクションリテラル内包表記

コレクションリテラル内包表記は、コレクションの変換、フィルタリング、結合などの操作を簡潔に表現するための構文です。この構文は、Pythonの内包表記からインスピレーションを得ています。

```
// リスト内包表記
[expression for pattern <- iterable if condition]

// 例：偶数の2倍
[x * 2 for x <- numbers if x % 2 == 0]

// 複数のイテレータ
[(x, y) for x <- xs for y <- ys if x + y > 5]

// パターンマッチング
[(name, age) for Person(name, age) <- people if age >= 18]

// マップ内包表記
{keyExpr -> valueExpr for pattern <- iterable if condition}

// 例：キーと値の変換
{k.toUpperCase() -> v * 2 for (k, v) <- originalMap if v > 0}

// キーと値の入れ替え
{v -> k for (k, v) <- originalMap}

// セット内包表記
#{expression for pattern <- iterable if condition}

// 例：平方数のセット
#{x * x for x <- range(1, 10)}

// 文字列の最初の文字のセット
#{word[0] for word <- words}
```

内包表記は、以下のような高階関数の組み合わせに変換されます：

```
// リスト内包表記
[x * 2 for x <- numbers if x % 2 == 0]

// 変換後
numbers.filter(x => x % 2 == 0).map(x => x * 2)

// 複数のイテレータを持つ内包表記
[(x, y) for x <- xs for y <- ys if x + y > 5]

// 変換後
xs.flatMap(x => 
  ys.filter(y => x + y > 5)
    .map(y => (x, y))
)
```

内包表記の利点は、コレクション操作を宣言的かつ読みやすく表現できることです。特に複数のコレクションを組み合わせる場合や、フィルタリングと変換を組み合わせる場合に有用です。

### 4.4.2 bind式

bind式は、モナド的な計算の連鎖を簡潔に表現するための構文です。Option、Result、Future、Eitherなどのモナド的な型の連鎖に最適化されています。

```
// 基本形式
bind {
  pattern1 <- expression1
  pattern2 <- expression2
  if condition
  finalExpression
}

// 例：ユーザー情報の取得と検証
bind {
  user <- findUser(userId)
  email <- getUserEmail(user)
  validEmail <- validateEmail(email)
  validEmail
}

// 例：エラーハンドリング
bind {
  data <- fetchData()
  parsed <- parseData(data)
  if parsed.isValid
  processData(parsed)
}
```

bind式は、以下のような`flatMap`と`map`の連鎖に変換されます：

```
// bind式
bind {
  user <- findUser(userId)
  email <- getUserEmail(user)
  validEmail <- validateEmail(email)
  validEmail
}

// 変換後
findUser(userId).flatMap(user => 
  getUserEmail(user).flatMap(email => 
    validateEmail(email).map(validEmail => 
      validEmail
    )
  )
)
```

bind式の利点は、ネストしたflatMap/map呼び出しを平坦で読みやすい形式で表現できることです。特に、早期リターンパターン（エラーが発生した場合に処理を中断する）を自然に表現できます。

bind式は、以下のような型を持つ値に対して使用できます：

1. **Option<T>**: 値が存在するかどうかを表す型
2. **Result<T, E>**: 成功または失敗を表す型
3. **Future<T>**: 非同期計算の結果を表す型
4. **Either<L, R>**: 2つの可能な型のうちの1つを表す型
5. **カスタムモナド型**: `flatMap`と`map`メソッドを持つ任意の型

bind式を使用するには、対象の型が以下のメソッドを提供している必要があります：

```
trait Monad<T> {
  fn flatMap<U>(f: (T) -> Monad<U>): Monad<U>
  fn map<U>(f: (T) -> U): Monad<U>
}
```

### 4.4.3 with式の返り値と用途

with式は式として設計されており、ブロック内の最後の式の評価結果を返します。この返り値は他の式と同様に使用できます：

### 4.4.4 暗黙的パラメータ

Protorun言語は、暗黙的パラメータをサポートしています。これにより、スコープ内で利用可能な値を明示的に渡すことなく、関数に提供することができます。

暗黙的パラメータと効果システムの詳細については、[5.8 暗黙的パラメータと効果システム](05-algebraic-effects.md#58-暗黙的パラメータと効果システム)を参照してください。

```
// with式の返り値を変数に代入
let result = with Console {
  Console.log("計算を開始します")
  let x = complexCalculation()
  Console.log("計算結果: " + x.toString())
  x  // この値がwith式の返り値となる
}

// with式の返り値を関数の引数として使用
processResult(with State<Counter> {
  let current = State.get()
  State.modify(c => c.increment())
  current.value * 2  // この値がwith式の返り値となる
})
```

with式の返り値を使用することには、以下のような実用的な価値があります：

1. **効果の局所化と結果の取得**: 効果の使用を特定のスコープに限定しながら、その結果を外部で利用できます。

```
// リソース管理と結果の取得
let fileContents = with ResourceManager<File> {
  // ファイルを開く（スコープ終了時に自動的に閉じられる）
  let file = ResourceManager.open(() => File.open("data.txt"))?
  
  // ファイルの内容を読み込み、処理した結果を返す
  ResourceManager.use(&file, f => f.readToString())?
    |> processData
}
// fileContentsには処理済みのファイル内容が格納され、
// ファイルは自動的に閉じられている
```

2. **合成性の向上**: with式を他の式と自然に組み合わせることができます。

```
// 条件分岐での使用
let result = if condition {
  with Console {
    Console.log("条件が真の場合の処理")
    computeForTrue()
  }
} else {
  with Logger {
    Logger.log("条件が偽の場合の処理")
    computeForFalse()
  }
}
```

3. **効果の組み合わせと結果の合成**: 異なる効果を持つ複数のwith式から返された値を合成できます。

```
// 複数の効果と結果の合成
let combinedResult = {
  let result1 = with Console {
    Console.log("最初の処理")
    computeFirst()
  }
  
  let result2 = with State<AppState> {
    let state = State.get()
    computeSecond(state)
  }
  
  // 2つの結果を合成
  combineResults(result1, result2)
}
```

4. **リソース管理の安全性**: リソースの安全な管理と結果の取得を一つの式で表現できます。

```
// 複数のリソースを使用した計算
let result = with ResourceManager<Connection> {
  let conn = ResourceManager.open(() => Database.connect(url))?
  
  with Transaction(conn): Transaction {
    // トランザクション内の処理
    let data = executeQuery(conn, query)?
    processData(data)
  }  // トランザクションは自動的にコミットまたはロールバック
}  // 接続は自動的に閉じられる
```

with式が値を返す式として設計されていることは、Protorunの式ベースの設計原則に沿っており、言語全体の一貫性と表現力を向上させます。これにより、効果の制御と計算の結果を自然に組み合わせることができ、より簡潔で読みやすいコードを書くことが可能になります。

## 4.5 パターンマッチング

```
// 値のパターンマッチング
match x {
  0 => "ゼロ",
  n if n < 0 => "負の数",
  _ => "その他"
}

// 構造のパターンマッチング
match opt {
  Option.Some(value) => s"値: $value",
  Option.None => "値なし"
}

// タプルのパターンマッチング
match pair {
  (0, y) => s"最初の要素はゼロ、2番目は$y",
  (x, 0) => s"最初の要素は$x、2番目はゼロ",
  (x, y) => s"($x, $y)"
}
```

Protorun言語のパターンマッチングは、以下の原則に基づいて設計されています：

1. **網羅性チェック**: パターンが対象の型のすべての可能な値を網羅していることをコンパイル時に検証します
2. **型安全性**: パターンマッチングは型チェックされ、型の整合性が保証されます
3. **分解と束縛**: 複合データ構造を分解し、その構成要素に名前を束縛することができます
4. **ガード条件**: パターンに加えて、追加の条件（ガード）を指定できます

パターンマッチングは、代数的データ型と組み合わせて使用することで、その真価を発揮します。これにより、データの構造と処理を密接に結びつけ、型安全な条件分岐を実現できます。

特に重要な特徴：

- **網羅性チェック**: コンパイラは、パターンがすべての可能なケースを網羅していることを検証します。これにより、パターンマッチングの漏れによるバグを防止します。

- **ガード条件**: パターンに加えて、追加の条件を指定できます。これにより、より細かい条件分岐が可能になります。

- **変数束縛**: パターンマッチングの過程で、データ構造の構成要素に名前を束縛できます。これにより、分解されたデータに簡単にアクセスできます。

パターンマッチングは、命令型プログラミングの条件分岐（if-else）よりも表現力が高く、特に代数的データ型を扱う際に威力を発揮します。これは、ML系言語やHaskellからインスピレーションを得た機能です。

## 4.6 関数合成

```
// パイプライン演算子
infix operator |> : 0
fn |><A, B>(a: A, f: (A) -> B): B = f(a)

// 効果を持つパイプライン演算子
infix operator |>* : 0
fn |>*<A, B, E>(a: A, f: (A) -> B & E): B & E = f(a)

// パイプラインの使用例
fn processData(data: String): Result<ProcessedData, ProcessError> & Logger = {
  data
    |> parse              // 純粋関数
    |>* validate          // Exception効果
    |>* enrich            // IO効果
    |>* log               // Logger効果
    |> finalize           // 純粋関数
}

// 関数合成演算子
infix operator >>> : 1
fn >>><A, B, C>(f: (A) -> B, g: (B) -> C): (A) -> C = {
  (a: A) => g(f(a))
}

// 効果を持つ関数合成演算子
infix operator >>>* : 1
fn >>>*<A, B, C, E1, E2>(
  f: (A) -> B & E1,
  g: (B) -> C & E2
): (A) -> C & E1 & E2 = {
  (a: A) => g(f(a))
}

// 関数合成の使用例
fn processUser(userId: String): UserStats & IO & Logger = {
  // 関数を合成
  let process = fetchUser >>>* validateUser >>>* enrichUserData >>>* logUserAccess >>> calculateUserStats
  
  // 合成関数を適用
  process(userId)
}
```

Protorun言語の関数合成機能は、以下の原則に基づいて設計されています：

1. **合成可能性**: 関数を組み合わせて新しい関数を作成できます
2. **読みやすさ**: データの流れを左から右へと読みやすく表現できます
3. **型安全性**: 関数合成は型チェックされ、型の整合性が保証されます
4. **効果の追跡**: 効果を持つ関数の合成では、効果も適切に合成されます

関数合成は、関数型プログラミングの中核的な概念であり、複雑な処理を小さな関数の組み合わせとして表現することを可能にします。これにより、コードの再利用性と保守性が向上します。

特に重要な特徴：

- **パイプライン演算子（`|>`）**: データを関数に渡す操作を左から右へと読みやすく表現します。これは、F#やElmのパイプライン演算子からインスピレーションを得ています。

- **効果を持つパイプライン演算子（`|>*`）**: 効果を持つ関数に対するパイプライン操作を表現します。これにより、効果を持つ関数と純粋関数を明確に区別しながら、一貫したパイプラインスタイルでコードを書くことができます。

- **関数合成演算子（`>>>`）**: 関数を合成して新しい関数を作成します。これは、Haskellの関数合成演算子からインスピレーションを得ています。

- **効果を持つ関数合成演算子（`>>>*`）**: 効果を持つ関数を合成します。合成された関数は、両方の関数の効果を持ちます。これにより、効果の追跡と合成が型レベルで行われます。

これらの演算子により、データ変換パイプラインや処理フローを簡潔かつ読みやすく表現できます。また、効果システムと統合されているため、副作用を持つ関数の合成も型安全に行うことができます。

## 4.7 メンバーアクセス式

メンバーアクセス式は、レコード型やモジュールなどの構造化されたデータのメンバー（フィールドや関数）にアクセスするために使用されます。ドット (`.`) 演算子を使用します。

```
// レコードのフィールドアクセス
let point = { x: 10.0, y: 20.0 }
let xCoord = point.x // xCoord は 10.0

// モジュールの関数アクセス
import Math
let sum = Math.add(5, 3) // sum は 8

// メソッド呼び出し（メンバーアクセスと関数呼び出しの組み合わせ）
let message = "hello world"
let upperMessage = message.toUpperCase() // "HELLO WORLD"
```

メンバーアクセスは左結合性を持つ後置演算子として扱われ、関数呼び出し (`()`) と同じ優先順位を持ちます。これにより、メソッドチェーン (`obj.method1().method2()`) のような記述が可能になります。
