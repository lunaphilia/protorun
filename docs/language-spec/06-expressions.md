# 6. 式

## 6.1 式の概要と設計原則

Protorun言語は、式ベースの言語設計を採用しています。これは、ほとんどの構文要素が値を返す式として実装されていることを意味します。この設計は以下の原則に基づいています：

1. **一貫性**: 同様の構文要素は同様の方法で扱われるべきです
2. **合成性**: 式は他の式の中に自然に組み込めるべきです
3. **表現力**: 複雑な計算を簡潔かつ読みやすく表現できるべきです
4. **参照透過性**: 純粋な式は、同じ入力に対して常に同じ結果を返すべきです

式ベースの設計は、関数型プログラミングの原則に沿っており、コードの推論と最適化を容易にします。また、副作用を明示的に扱うことで、プログラムの理解と保守が容易になります。

## 6.2 ブロック式

ブロック式は、0個以上の宣言、文 (`return`)、または式を中括弧 `{}` で囲んだものです。文法上は式の一種として扱われます。

```ebnf
BlockExpr ::= "{" (Declaration | Statement | Expression)* "}"
BlockItem ::= Declaration | Statement | Expression // AST上の表現
Statement ::= ReturnStatement // 現在の仕様では Return のみ
```

(Declaration, ReturnStatement, Expression の詳細は他の章を参照)

ブロック式は主に以下の目的で使用されます：

1. **要素のグループ化**: 複数の宣言、文、式を構文的に一つの単位としてまとめます。これは `if` 式の `then`/`else` 節や、`match` 式のケース、関数本体などで使用されます。
2. **スコープの導入**: ブロック内で宣言された変数 (`let` または `var`) は、そのブロック内でのみ有効なローカルスコープを持ちます。
3. **式の評価と値**: ブロック内の最後の要素が式 (`Expression`) である場合、その式の評価結果がブロック式全体の値となります。最後の要素が宣言や `return` 文である場合、またはブロックが空である場合、ブロック式の値は `Unit` となります。副作用のためだけに記述された式（最後の要素ではない式）の値は破棄されます。

```protorun
// 例1: if式でのブロック使用
if x > 0 {
  println("xは正です") // 文1
  x * 2             // 最後の式 (ブロックの値)
} else {
  println("xは0以下です") // 文1
  0                   // 最後の式 (ブロックの値)
}

// 例2: スコープ
let a = 10
let b = {
  let a = 20 // 内側のスコープの 'a'
  a + 5      // このブロックの値は 25
}
// ここでは外側の 'a' (10) が参照される
println(a) // 10
println(b) // 25

// 例3: Unitを返すブロック
{
  let temp = calculate()
  logResult(temp) // 副作用のための式
  // 最後の要素が式ではないため、このブロックは Unit を返す
}

// 例4: 最後の要素が宣言の場合 (Unit を返す)
{
  let x = 10
  let y = 20 // この宣言が最後なので Unit
}
```

ブロック内の要素は改行で区切られます。セミコロンは不要です。

## 6.3 制御構造

```protorun
// if式 (then/else には単一式またはブロック式が使用可能)
if condition
  expression1 // 単一式
else {
  statement1  // return 文
  expression2 // 最後の式
}

// match式
match value {
  pattern1 => expression1, // 単一式
  pattern2 if guard => { // ブロック式
     let temp = calculate(pattern2)
     temp + 1 // 最後の式
  },
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

// with式（ハンドラは常に式）
with consoleHandler { // consoleHandler はハンドラオブジェクトを持つ変数や式
  Console.log("このスコープ内のConsole効果は consoleHandler でハンドル")
}

// with式（効果の暗黙的な提供 - これはハンドラの実装に依存）
// with DatabaseHandler { // DatabaseHandler が Database 効果を提供する式である必要がある
//   processUserData("user123")
// }

// with式（複数のハンドラ式を提供 - カンマ区切りは現在サポートされていない）
// with databaseHandler, loggerHandler { ... }

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

- **if式**: 条件に基づいて評価する式を選択します。`then` 節と `else` 節には、単一の式、または `{}` で囲まれたブロック式のいずれかを記述できます。複数行の文を実行したい場合はブロック式を使用する必要があります。

- **match式**: 値をパターンと照合し、一致したパターンに対応する式を評価します。`=>` の右辺には、単一の式、または `{}` で囲まれたブロック式のいずれかを記述できます。複数行の文を実行したい場合はブロック式を使用する必要があります。

- **コレクションリテラル内包表記**: コレクション操作を簡潔に表現するための構文です。Pythonの内包表記からインスピレーションを得ており、コレクションの種類（リスト、マップ、セット）に応じた構文を提供します。

- **bind式**: モナド的な連鎖を表現するための構文です。Option、Result、Futureなどのモナド的な型の連鎖に最適化されています。Haskellのdoノーテーションからインスピレーションを得ていますが、より明示的な名前を使用しています。

- **with式**: 効果ハンドラ（式として評価される）を適用するスコープを定義します。`with handlerExpr { bodyExpr }` の形式を取ります。`handlerExpr` は効果ハンドラを提供する式である必要があります。オプションで `: EffectType` を指定して、ハンドルする効果の型を明示することもできます。`with` 式全体の値は `bodyExpr` の評価結果となります。

- **効果のスコープ化**: 効果の実装を局所的に提供するための構文です。これにより、効果の実装を必要な場所に限定し、グローバルな状態の変更を避けることができます。これは、効果システムの柔軟性と安全性を向上させるための設計決定です。

### 6.3.1 コレクションリテラル内包表記

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

### 6.3.2 bind式

bind式は、モナド的な計算の連鎖を簡潔に表現するための構文です。Option、Result、Future、Eitherなどのモナド的な型の連鎖に最適化されています。

```
// 基本形式 (各ステップは改行で区切ります)
bind {
  pattern1 <- expression1
  pattern2 <- expression2
  if condition // 条件もステップとして扱えます
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
  if parsed.isValid // ガード条件
  processData(parsed)
}
```

`bind` 式内の各ステップ (`pattern <- expression` や `if condition`) は、以前はセミコロンで区切られていましたが、現在は**改行**で区切られます。これにより、コードの見た目がよりすっきりします。

bind式は、以下のような`flatMap`と`map`の連鎖に変換されます：

```
// bind式 (セミコロンなし)
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

### 6.3.3 ラムダ式 (Lambda Expressions)

ラムダ式（無名関数）は、関数をその場で定義するための簡潔な構文です。`=>` 演算子を使用して、パラメータリストと関数本体を区切ります。

```protorun
// 基本形
(param1: Type1, param2: Type2) => expression

// 型推論が可能な場合
let add = (a, b) => a + b

// 単一パラメータの場合 (カッコは省略可能)
let square = x => x * x

// 複数文を実行する場合 (ブロック式を使用)
let process = (input: String) => {
  let trimmed = input.trim()
  println(s"Processing: $trimmed")
  trimmed.toUpperCase() // ブロックの最後の式が返り値
}
```

ラムダ式の本体 (`=>` の右辺) は単一の `Expression` です。これにはリテラル、変数、関数呼び出し、演算、そしてブロック式 `{...}` など、任意の式を含めることができます。複数の文を実行したい場合は、ブロック式を使用する必要があります。

### 6.3.4 ラムダ式 (Lambda Expressions)

ラムダ式（無名関数）は、関数をその場で定義するための簡潔な構文です。`=>` 演算子を使用して、パラメータリストと関数本体を区切ります。

```protorun
// 基本形
(param1: Type1, param2: Type2) => expression

// 型推論が可能な場合
let add = (a, b) => a + b

// 単一パラメータの場合 (カッコは省略可能)
let square = x => x * x

// 複数文を実行する場合 (ブロック式を使用)
let process = (input: String) => {
  let trimmed = input.trim()
  println(s"Processing: $trimmed") // 副作用のための式
  trimmed.toUpperCase() // ブロックの最後の式が返り値
}
```

ラムダ式の本体 (`=>` の右辺) は単一の `Expression` です。これにはリテラル、変数、関数呼び出し、演算、そしてブロック式 `{...}` など、任意の式を含めることができます。複数の宣言や文を実行したい場合は、ブロック式を使用する必要があります。

### 6.3.5 with式の用途と返り値

`with` 式は、特定のスコープ内で効果ハンドラを適用するために使用されます。`with handlerExpr { bodyExpr }` の形式を取り、`bodyExpr` の評価結果を返します。

```protorun
// with式の返り値を変数に代入
let result = with consoleHandler { // consoleHandler はハンドラを提供する式
  Console.log("計算を開始します")
  let x = complexCalculation()
  Console.log("計算結果: " + x.toString())
  x  // この値がブロックの値となり、with式の返り値となる
}

// with式の返り値を関数の引数として使用
processResult(with stateHandler(0) { // stateHandler(0) がハンドラを提供する式
  let current = State.get()
  State.modify(c => c.increment())
  current.value * 2  // この値がブロックの値となり、with式の返り値となる
})
```

`with` 式が値を返す式として設計されていることは、Protorunの式ベースの設計原則に沿っており、言語全体の一貫性と表現力を向上させます。これにより、効果の制御と計算の結果を自然に組み合わせることができ、より簡潔で読みやすいコードを書くことが可能になります。

### 6.3.6 暗黙的パラメータ

Protorun言語は、暗黙的パラメータをサポートしています。これにより、スコープ内で利用可能な値を明示的に渡すことなく、関数に提供することができます。

暗黙的パラメータと効果システムの詳細については、[8.8 暗黙的パラメータと効果システム](08-algebraic-effects.md#88-暗黙的パラメータと効果システム)を参照してください。 <!-- Note: Assuming section numbers in 08-algebraic-effects.md will also be updated later if needed -->

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

## 6.4 パターンマッチング

`match` 式は、与えられた値を複数のパターンと比較し、最初に一致したパターンに対応する式を評価します。`=>` の右辺には、単一の式、または `{}` で囲まれたブロック式のいずれかを記述できます。

```protorun
// 値のパターンマッチング (右辺は単一式)
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

// 複数文を実行する場合 (右辺はブロック式)
match result {
    Result.Ok(value) => {
        logSuccess(value)
        processValue(value) // ブロックの最後の式が match 式の値
    },
    Result.Err(error) => {
        logError(error)
        handleError(error) // ブロックの最後の式が match 式の値
    }
}
```

`=>` の右辺は `Expression` であるため、単一の式を直接書くことも、複数の文を含むブロック式 `{...}` を書くことも可能です。複数行の文を実行したい場合はブロック式を使用する必要があります。

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

## 6.5 関数合成

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

## 6.6 メンバーアクセス式

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
