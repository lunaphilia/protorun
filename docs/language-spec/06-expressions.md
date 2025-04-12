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

Protorunは、条件分岐や繰り返し、効果ハンドリングなどのための多様な制御構造を提供します。これらはすべて式として設計されており、値を返すことができます。

```protorun
// if式 (すべての分岐でブロック式が必須)
if condition1 {
  expression1 // ブロック式
} elif condition2 {
  expression2
} else {
  expression3
}

// 単一の値を返す場合もブロック式を使用
let result = if x > 0 { 1 } elif x < 0 { -1 } else { 0 }

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

// with式（ハンドラインスタンスの提供）
with alias1 = handlerInstance1: EffectType1,
     alias2 = handlerInstance2: EffectType2, ...
{
  // ブロック内で alias1 や alias2 に対応する効果操作を呼び出す
  // ...
}
```

Protorun言語の制御構造は、以下の原則に基づいて設計されています：

1. **式ベース**: ほとんどの制御構造は値を返す式として実装されています
2. **型安全性**: 制御構造は型チェックされ、型の整合性が保証されます
3. **表現力**: 様々な計算パターンを自然に表現できる豊富な制御構造を提供します
4. **効果の制御**: 効果のスコープと実装を明示的に制御する構造を提供します

特に重要な特徴：

- **if式**:
  - **構文**: `if condition1 { then_branch1 } [elif condition2 { then_branch2 }]* [else { else_branch }]?`
  - 条件に基づいて評価する式を選択します。`if` キーワードで始まり、条件式、そして中括弧 `{}` で囲まれたブロック式（`then` 節）が続きます。
  - オプションで、0個以上の `elif` 節（`elif` キーワード、条件式、ブロック式のペア）と、最後の `else` 節（`else` キーワード、ブロック式）を持つことができます。
  - **ブロック式必須**: `if`, `elif`, `else` の各分岐の本体は、**常に中括弧 `{}` で囲まれたブロック式**でなければなりません。単一の式を返す場合でも `{ expression }` のように記述します。これにより構文の一貫性を保ちます。
  - **利点**:
    - 単純な条件分岐や、論理演算子 (`&&`, `||`, `!`) を使った複合条件を直感的かつ簡潔に表現できます。
    - `if ... elif ... else` チェーンにより、段階的な条件評価を自然に記述できます。
    - 多くのプログラマーにとって馴染み深く、学習コストが低い構文です。
  - **設計思想**: Protorunでは、より複雑なパターンマッチングに適した `match` 式も提供しますが、単純な条件分岐には `if` 式の方が読みやすく適切であると考え、両方の構文を採用しています。これにより、状況に応じて最適な表現を選択できます。`if` 式の構文をブロック式に統一することで、単一式の場合の `then` キーワードなどを不要にし、構文の複雑さを低減しています。

- **match式**:
  - **構文**: `match scrutinee { [pattern [if guard]? => branch],* }`
  - 値 (`scrutinee`) を複数のパターンと比較し、最初に一致したパターンに対応する式 (`branch`) を評価します。
  - **利点**:
    - 代数的データ型（enum）の分解と条件分岐を強力に統合できます。
    - コンパイラによる網羅性チェックにより、すべてのケースが処理されているか検証できます。
    - パターンに加えてガード条件 (`if guard`) を指定でき、より複雑な条件分岐が可能です。
    - タプルパターンなどを用いて、複数の値に基づく分岐を簡潔に記述できます。
    - 関数型プログラミングのパラダイムと自然に調和します。
  - **設計思想**: `match` 式は、特に代数的データ型を扱う場合や、網羅性が重要な場合に威力を発揮します。`if` 式とは相補的な役割を果たし、言語全体の表現力を高めます。
  - `=>` の右辺 (`branch`) には、単一の式、または `{}` で囲まれたブロック式のいずれかを記述できます。複数行の文を実行したい場合はブロック式を使用する必要があります。（詳細は [6.4 パターンマッチング](#64-パターンマッチング) を参照）

- **コレクションリテラル内包表記**: コレクション操作を簡潔に表現するための構文です。Pythonの内包表記からインスピレーションを得ており、コレクションの種類（リスト、マップ、セット）に応じた構文を提供します。

- **bind式**: モナド的な計算の連鎖を簡潔に表現するための構文です。（詳細は [6.3.2 bind式](#632-bind式) を参照）

- **with式**: 特定のスコープ内で効果ハンドラインスタンスを有効にするための式です。`with alias1 = handlerInstance1: EffectType1, alias2 = handlerInstance2: EffectType2, ... { bodyExpr }` の形式を取ります。カンマ区切りで複数のハンドラを同時に指定でき、ネストを回避できます。`bodyExpr` の評価結果が `with` 式全体の値となります。（詳細は [6.3.5 with式](#635-with式) を参照）

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

### 6.3.3 関数式 (Function Expressions)

関数式（以前のラムダ式）は、無名関数をその場で定義するための構文です。`fn` キーワードで始まり、パラメータリスト（複数種類あり、すべてオプション）、オプションの戻り値型、そして `=>` の後に続く関数本体（式）で構成されます。

**構文:**

```ebnf
FunctionExpr ::= "fn" ParamList? EffectParamList? ImplicitParamList? (":" ReturnType)? "=>" Expression
ParamList ::= "(" (Param ("," Param)*)? ")"
EffectParamList ::= "(" (EffectParam ("," EffectParam)*)? ")"
ImplicitParamList ::= "(" "with" Param ("," Param)* ")"
Param ::= Identifier (":" Type)?
EffectParam ::= "effect" Identifier ":" TypeRef
ReturnType ::= Type | "Unit" // (Type の定義は他を参照)
```

- `fn`: 関数式の開始を示すキーワード。
- `ParamList?`: 通常のパラメータリスト（オプション）。`()` で囲み、カンマ区切りで `identifier (: Type)?` を記述します。
- `EffectParamList?`: Effect パラメータリスト（オプション）。`()` で囲み、カンマ区切りで `effect identifier: TypeRef` を記述します。関数が依存する効果インターフェースを指定します。
- `ImplicitParamList?`: Implicit パラメータリスト（オプション）。`(with ...)` で囲み、カンマ区切りで `identifier (: Type)?` を記述します。コンテキストから暗黙的に渡される値を指定します（Scala の implicit parameter list に類似）。
- `(":" ReturnType)?`: 戻り値の型注釈（オプション）。コロン `:` に続けて戻り値の型 (`ReturnType`) を記述します。
- `=>`: パラメータリスト/型注釈と関数本体を区切るキーワード。
- `Expression`: 関数本体。`=>` の後に直接続きます。単一の式である必要があります。複数の文を実行したい場合はブロック式 `{...}` を使用します。

**具体例:**

```protorun
// 通常のパラメータのみ
let add = fn (a: Int, b: Int): Int => a + b
let square = fn x => x * x // 型推論

// Effect パラメータを持つ関数式
let logOperation = fn (data: Data) (effect logger: Logger) => {
  logger.log(s"Processing $data")
  process(data)
}

// Implicit パラメータを持つ関数式
let greet = fn (name: String) (with context: Context) => {
  s"${context.greeting}, $name!"
}

// 複数のパラメータリストを持つ関数式
let complexCalc = fn (x: Int) (effect state: State<Int>) (with config: Config) => {
  let current = state.get()
  state.set(current + x * config.multiplier)
  state.get()
}

// パラメータなしの関数式
let getMeaning = fn => 42

// ブロック式を本体に持つ関数式
let process = fn (input: String) => {
  let trimmed = input.trim()
  println(s"Processing: $trimmed")
  trimmed.toUpperCase() // ブロックの最後の式が返り値
}
```

**特徴:**

- **統一された関数定義**: `let` 束縛と組み合わせることで、名前付き関数も無名関数も同じ `fn ... => ...` 形式で表現されます ([4.2.2 `let` による関数定義](04-declarations.md#let-による関数定義) を参照)。
- **パラメータリストの柔軟性**: 通常、Effect、Implicit の3種類のパラメータリストを任意の順序（ただし、各種類は1回まで）で記述できます（※注: 現在のパーサー実装では `ParamList? EffectParamList? ImplicitParamList?` の順序のみサポート）。これにより、カリー化や依存性の注入を表現豊かに行えます。
- **式ベース**: 関数式の本体は常に単一の式です。

### 6.3.4 with式

`with` 式は、特定のスコープ内で効果ハンドラインスタンスを適用するために使用されます。`with alias = handlerInstance: EffectType, ... { bodyExpr }` の形式を取り、`bodyExpr` の評価結果を返します。

```
// with式の返り値を変数に代入
let result = with log = ConsoleHandler {}: Console {
  log.log("計算を開始します")
  let x = complexCalculation()
  log.log("計算結果: " + x.toString())
  x  // この値がwith式の返り値となる
}

// with式の返り値を関数の引数として使用
processResult(with st = StateHandler<Int> { state: 0 }: State<Int> {
  let current = st.get()
  st.modify(c => c + 1)
  current * 2  // この値がwith式の返り値となる
})
```

with式の返り値を使用することには、以下のような実用的な価値があります：

1. **効果の局所化と結果の取得**: 効果の使用を特定のスコープに限定しながら、その結果を外部で利用できます。

   ```
   // リソース管理と結果の取得 (ハンドラがリソースを管理する想定)
   let fileContents = with fs = LocalFileHandler { basePath: "/data" }: FileSystem {
     let handle = fs.open("data.txt", FileMode.Read)?
     let content = fs.read(&handle)?
     fs.close(handle)?
     processData(content) // 処理結果を返す
     // fs インスタンスの破棄時にリソースが解放される (RAII連携、詳細は8.7節)
   }
   // fileContents には処理済みデータが格納される
   ```

2. **合成性の向上**: `with` 式を他の式（`if`, `match` など）と自然に組み合わせることができます。

   ```
   // 条件分岐での使用
   let result = if condition {
     with log = ConsoleHandler {}: Console {
       log.log("条件が真の場合の処理")
       computeForTrue()
     }
   } else {
     with log = FileLogger { path: "/log/false.log" }: Console {
       log.log("条件が偽の場合の処理")
       computeForFalse()
     }
   }
   ```

3. **効果の組み合わせと結果の合成**: 拡張された `with` 構文により、複数の効果を組み合わせ、その結果を合成できます。

   ```
   // 複数の効果と結果の合成
   let combinedResult = with log = ConsoleHandler {}: Console,
                            st = StateHandler<Int> { state: 0 }: State<Int>
                       {
                         log.log("最初の処理")
                         let result1 = computeFirst()
                         st.set(result1)
                         log.log("次の処理")
                         let result2 = computeSecond(st.get())
                         combineResults(result1, result2) // ブロックの結果
                       }
   ```

`with` 式が値を返す式として設計されていることは、Protorun の式ベースの設計原則に沿っており、言語全体の一貫性と表現力を向上させます。これにより、効果の制御と計算の結果を自然に組み合わせることができ、より簡潔で読みやすいコードを書くことが可能になります。詳細は [8.5 効果ハンドラインスタンスの提供 (`with` 構文)](08-algebraic-effects.md#85-効果ハンドラインスタンスの提供-with-構文) を参照してください。

## 6.4 パターンマッチング

`match` 式は、与えられた値 (`scrutinee`) を複数のパターンと比較し、最初に一致したパターンに対応する式 (`branch`) を評価します。`if` 式と比較して、特に代数的データ型（enum）の分解や網羅的なケース分析に適しています。

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

1. **網羅性チェック**: パターンが対象の型のすべての可能な値を網羅していることをコンパイル時に検証します。これにより、パターンマッチングの漏れによるバグを防止します。
2. **型安全性**: パターンマッチングは型チェックされ、型の整合性が保証されます。
3. **分解と束縛**: 複合データ構造を分解し、その構成要素に名前を束縛することができます。これにより、分解されたデータに簡単にアクセスできます。
4. **ガード条件**: パターンに加えて、追加の条件（ガード）を指定できます (`pattern if guard => ...`)。これにより、より細かい条件分岐が可能になります。

パターンマッチングは、代数的データ型と組み合わせて使用することで、その真価を発揮します。これにより、データの構造と処理を密接に結びつけ、型安全な条件分岐を実現できます。これは、ML系言語やHaskellからインスピレーションを得た機能であり、`if` 式よりも表現力が高く、特に関数型プログラミングスタイルに適しています。

## 6.5 効果操作呼び出し式

関数内で Effect パラメータのエイリアスを使って効果操作を呼び出す構文 `alias.operation(...)` も式の一種です。

```protorun
let example = fn (effect log: Console): Int = {
  log.log("開始") // 効果操作呼び出し式 (Unit を返す)
  let result = calculate()
  log.log("終了") // 効果操作呼び出し式
  result
}
```

**意味:**

`alias.operation(...)` という式は、見た目はメソッド呼び出しに似ていますが、定義により **代数的効果のメカニズム** を起動します。

1. 現在の計算が中断されます。
2. 継続（残りの計算）がキャプチャされます。
3. `with` 文によって `alias` に束縛されているアクティブなハンドラインスタンスの `operation` メソッド実装に制御が移ります（継続も渡されます）。
4. ハンドラメソッドが継続を処理し、最終的な結果を返します。この結果が `alias.operation(...)` 式全体の値となります。

この呼び出しは、通常の関数呼び出しとは異なり、非局所的な制御フロー（中断、再開、大域脱出など）を引き起こす可能性がある点が重要です。詳細は [8. 代数的効果](08-algebraic-effects.md) を参照してください。

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

## 6.7 タプルリテラルとグループ化

Protorunは、複数の値を一つのまとまりとして扱うためのタプルを提供します。タプルリテラルは括弧 `()` を用いて記述されますが、グループ化のための括弧との区別に注意が必要です。

**タプルリテラル:**

- **要素数0 (ユニット):** `()` は要素数が0個のタプルリテラルであり、`Unit` 型の唯一の値です。値を返さない式（例: `println("...")` の呼び出し）や、最後の要素が宣言であるブロック式などは、この `Unit` 値を返します。
- **要素数2以上:** `(式1, 式2, ...)` は、カンマ区切りで2つ以上の式を括弧で囲むことで、要素数2以上のタプルリテラルを生成します。各要素の型からタプルの型（例: `(Int, String, Bool)`）が推論されます。

```protorun
let unit_val = ()             // Unit 型
let pair = (10, "hello")      // (Int, String) 型
let triple = (true, 2.5, 'x') // (Bool, Float, Char) 型
```

**グループ化:**

- 括弧 `()` で**単一の式**を囲んだ場合、それはタプルリテラルではなく、**式のグループ化**として扱われます。これは、演算子の優先順位を制御したり、式の区切りを明確にするために使用されます。グループ化された式は、内部の式を評価した結果の値と型を持ちます。

```protorun
let result = (2 + 3) * 4 // (2 + 3) は Int 型の 5 に評価される。結果は 20
let simple_group = (5)     // これは Int 型の 5 であり、タプルではない
```

**要素数1のタプル:**

- Protorun言語の現在の仕様では、**要素数1のタプルを生成するためのリテラル構文は存在しません**。`(式)` は常にグループ化として解釈されます。これはSwift言語と同様の仕様であり、実用上問題になることは稀であると考えられています。もし要素が1つのコンテナが必要な場合は、構造体 (struct) などを定義することを検討してください。

## 6.8 部分適用式

部分適用式は、関数の一部の引数を事前に適用し、残りの引数を取る新しい関数を生成するための構文です。プレースホルダー `_` を使用して、後で適用される引数を示します。

**構文:**

```ebnf
PartialApplicationExpr ::= Expression "(" ((Expression | "_") ("," (Expression | "_"))*)? ")"
```

- `Expression`: 部分適用される関数を表す式。
- `( ... )`: 引数リスト。
- `Expression`: 事前に適用される引数。
- `_`: 後で適用される引数を示すプレースホルダー。

**意味:**

部分適用式は以下の動作を行います。

1. 左辺の `Expression` を評価し、関数 `f` を取得します。
2. 引数リスト内の `Expression` を評価し、事前に適用される引数の値を取得します。
3. 関数 `f`、事前に適用された引数の値、およびプレースホルダー `_` の位置情報を含む新しい関数（クロージャ）を生成します。
4. この新しい関数が部分適用式全体の値となります。

**具体例:**

```protorun
// 2引数関数
let add = fn (a: Int, b: Int): Int = a + b

// 部分適用: 最初の引数に 1 を適用
let add_one = add(1, _) // add_one は Int -> Int 型の関数

// 部分適用された関数の呼び出し
let result = add_one(5) // result は 6

// 部分適用: 2番目の引数に 10 を適用
let add_ten = add(_, 10) // add_ten は Int -> Int 型の関数
let result2 = add_ten(3) // result2 は 13

// 複数のプレースホルダー
let multiply = fn (a: Int, b: Int, c: Int): Int = a * b * c
let multiply_by_two = multiply(_, 2, _) // multiply_by_two は (Int, Int) -> Int 型の関数
let result3 = multiply_by_two(3, 4) // result3 は 24 (3 * 2 * 4)
```

**特徴:**

- **カリー化の促進**: 関数を段階的に適用することが容易になります。
- **コードの再利用**: 特定の引数が固定された関数を簡単に作成できます。
- **高階関数との連携**: 高階関数に渡す関数をその場で生成するのに便利です。

部分適用は、関数型プログラミングにおける重要なテクニックであり、コードの抽象化と再利用性を高めます。
