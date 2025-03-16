# Protorun言語仕様

## 1. 序論

Protorun（プロトラン）は、関数型プログラミング、強力な型システム、メモリ安全性を統合した新しいプログラミング言語です。Scala、Haskell、Rustなどの言語から優れた特性を取り入れつつ、より統合的なアプローチを目指しています。

### 1.1 設計理念

Protorunは以下の設計理念に基づいています：

- **安全性**: 強力な型システムと所有権モデルによるメモリ安全性
- **表現力**: 関数型プログラミングパラダイムと代数的データ型
- **制御性**: 代数的効果による副作用の型安全な制御
- **シンプルさ**: 学習しやすく、読みやすい構文と強力な型推論
- **効率性**: 高性能な実行と予測可能なリソース使用

### 1.2 コア機能の優先順位

Protorunの設計では、以下の機能を優先しています：

1. **代数的効果システム**: 副作用を型レベルで追跡し制御するメカニズム
2. **所有権モデル**: メモリ安全性を静的に保証するメカニズム
3. **強力な型システム**: 静的型付けと型推論による安全性
4. **代数的データ型**: データのモデリングのための表現力豊かな型

## 2. 字句構造

### 2.1 キーワード

```
fn       let      var      if       else     match    case
return   for      while    in       trait    impl     type
effect   handle   resume   try      catch    throw    with
module   import   pub      enum     resource do       as
```

### 2.2 演算子

```
+  -  *  /  %  =  ==  !=  <  >  <=  >=  &&  ||  !  &  |  ^  ~  <<  >>
->  =>  ::  .  ,  ;  :  ..  ...  +=  -=  *=  /=  %=  &=  |=  ^=
|>  |>* >>>  >>>*
```

### 2.3 リテラル

```
整数: 42, 0xFF, 0b1010, 1_000_000
浮動小数点: 3.14, 1e10, 1.2e-3
文字列: "hello", """複数行文字列"""
文字: 'a', '\n', '\u{1F600}'
ブール: true, false
単位: ()
```

注意: Protorun言語では`null`リテラルは存在しません。値の存在/不在はOption型で表現します。

### 2.4 コメント

```
// 単一行コメント

/*
 * 複数行
 * コメント
 */

/// ドキュメントコメント（関数や型の直前に記述）
```

## 3. 型システム

### 3.1 基本型

```
Int, Float, Double, Bool, Char, String, Unit
```

すべての型はnon-nullableです。nullは言語から排除されています。

### 3.2 複合型

```
// 配列型
[T]

// タプル型
(T1, T2, ..., Tn)

// 関数型
(T1, T2, ..., Tn) -> R
(T1, T2, ..., Tn) -> R with E

// オプション型（値の存在/不在を表現）
Option<T>

// 結果型（成功/失敗を表現）
Result<T, E>
```

### 3.3 型定義

```
// レコード型（構造体）
type Person = {
  name: String,
  age: Int
}

// 代数的データ型（enum構文）
enum Option<T> {
  Some(T),
  None
}

enum Result<T, E> {
  Ok(T),
  Err(E)
}

// 型エイリアス
type StringMap<T> = Map<String, T>
```

### 3.4 トレイト（インターフェース）

```
// トレイト定義
trait Show {
  fn show(self): String
}

// トレイト継承（単一継承のみ）
trait Eq {
  fn equals(self, other: Self): Bool
}

trait Ord extends Eq {
  fn compare(self, other: Self): Int
  
  // デフォルト実装
  fn equals(self, other: Self): Bool = self.compare(other) == 0
}

// トレイト実装
impl Show for Int {
  fn show(self): String = self.toString()
}

// ジェネリックな実装
impl<T: Show> Show for Option<T> {
  fn show(self): String = match self {
    Option.Some(v) => s"Some(${v.show()})",
    Option.None => "None"
  }
}
```

### 3.5 リソース型

```
// リソース型の定義
resource type File {
  // 内部フィールド
  handle: FileHandle,
  path: String,
  
  // 獲得関数（コンストラクタ）
  fn open(path: String): Result<File, IOError> = {
    // ファイルを開く実装
  }
  
  // 解放関数（デストラクタ）
  fn close(self): Unit = {
    // ファイルを閉じる実装
  }
  
  // 通常のメソッド
  fn read(self: &Self): Result<String, IOError> = {
    // ファイルを読み込む実装
  }
  
  fn write(self: &mut Self, content: String): Result<Unit, IOError> = {
    // ファイルに書き込む実装
  }
}
```

## 4. 式と文

### 4.1 変数宣言

```
// 不変変数（デフォルト）
let x = 42
let y: Int = 42

// 可変変数
var counter = 0
var name: String = "John"
```

### 4.2 関数定義

```
// 基本的な関数定義
fn add(a: Int, b: Int): Int = {
  a + b
}

// 単一式の場合は中括弧を省略可能
fn square(x: Int): Int = x * x

// 効果付き関数
fn log(message: String): Unit with Console = {
  Console.log(message)
}

// ジェネリック関数
fn identity<T>(x: T): T = x

// 複数のパラメータリスト（カリー化）
fn add(a: Int)(b: Int): Int = a + b
```

### 4.3 制御構造

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

// for式
for {
  x <- xs
  y <- ys
  if condition
} yield x + y

// while文
while condition {
  // ループ本体
}

// with式（効果スコープ）
with Console {
  Console.log("このスコープ内でConsole効果を使用可能")
}

// with式（効果ハンドラを指定）
with Console handled by consoleLogger {
  Console.log("このスコープ内のConsole効果はconsoleLoggerでハンドル")
}

// do式（モナド的な連鎖のための構文糖）
do {
  user <- findUser(userId)
  email <- getUserEmail(user)
  validEmail <- validateEmail(email)
  validEmail
}
```

### 4.4 パターンマッチング

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

### 4.5 関数合成

```
// パイプライン演算子
infix operator |> : 0
fn |><A, B>(a: A, f: (A) -> B): B = f(a)

// 効果を持つパイプライン演算子
infix operator |>* : 0
fn |>*<A, B, E>(a: A, f: (A) -> B with E): B with E = f(a)

// パイプラインの使用例
fn processData(data: String): Result<ProcessedData, ProcessError> with Logger = {
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
  f: (A) -> B with E1,
  g: (B) -> C with E2
): (A) -> C with E1 & E2 = {
  (a: A) => g(f(a))
}

// 関数合成の使用例
fn processUser(userId: String): UserStats with IO & Logger = {
  // 関数を合成
  let process = fetchUser >>>* validateUser >>>* enrichUserData >>>* logUserAccess >>> calculateUserStats
  
  // 合成関数を適用
  process(userId)
}
```

## 5. 所有権システム

### 5.1 所有権

```
// 所有権の移動
fn take(v: Vec<Int>): Int = {
  // vの所有権がここに移動
  v.length
}

let vec = [1, 2, 3]
take(vec)
// ここでvecは使用不可
```

### 5.2 借用

```
// 不変借用
fn peek(v: &Vec<Int>): Int = {
  // vの参照のみを使用
  v.length
}

// 可変借用
fn modify(v: &mut Vec<Int>): Unit = {
  // vの可変参照を使用
  v.push(42)
}

let mut vec = [1, 2, 3]
peek(&vec)    // 不変借用
modify(&mut vec)  // 可変借用
```

### 5.3 ライフタイム

```
// ライフタイム注釈
fn longest<'a>(x: &'a str, y: &'a str): &'a str = {
  if x.length > y.length { x } else { y }
}

// 構造体でのライフタイム
type Ref<'a, T> = {
  value: &'a T
}
```

### 5.4 リソース管理

```
// リソース型を使用したリソース管理
fn processFile(path: String): Result<String, IOError> = {
  // ファイルを開く（スコープ終了時に自動的に閉じられる）
  let file = File.open(path)?
  
  // ファイルの内容を読み取る
  let content = file.read()?
  
  // 内容を処理
  let processed = processContent(content)
  
  Result.Ok(processed)
} // ここでfileのreleaseメソッド（close）が自動的に呼び出される
```

## 6. 代数的効果

### 6.1 効果の定義

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
  fn openFile(path: String): Result<File, IOError>
  
  // ファイルの所有権を消費する操作
  fn closeFile(file: File): Result<Unit, IOError>
  
  // ファイルの借用を使用する操作
  fn readFile(file: &File): Result<String, IOError>
  
  // ファイルの可変借用を使用する操作
  fn writeFile(file: &mut File, content: String): Result<Unit, IOError>
}
```

### 6.2 効果の使用

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

### 6.3 効果ハンドラ

```
// 効果ハンドラ
fn runConsole<T>(action: () -> T with Console): T = {
  handle action() {
    Console.log(message) => {
      println(message)
      resume()
    }
    
    Console.readLine() => {
      let input = readLine()
      resume(input)
    }
  }
}

// 状態効果のハンドラ
fn runState<S, T>(initialState: S, action: () -> T with State<S>): (T, S) = {
  var state = initialState
  
  let result = handle action() {
    State.get() => resume(state),
    
    State.set(newState) => {
      state = newState
      resume()
    },
    
    State.modify(f) => {
      state = f(state)
      resume()
    }
  }
  
  (result, state)
}

// 所有権を考慮した効果ハンドラ
fn runFileSystem<T>(action: () -> T with FileSystem): T = {
  handle action() {
    FileSystem.openFile(path) => {
      let file = RealFileSystem.open(path)
      // fileの所有権をresumeに渡す
      resume(file)
    },
    
    FileSystem.closeFile(file) => {
      // fileの所有権を消費
      RealFileSystem.close(file)
      resume()
    },
    
    FileSystem.readFile(file) => {
      // fileの不変借用を使用
      let content = RealFileSystem.read(file)
      resume(content)
    },
    
    FileSystem.writeFile(file, content) => {
      // fileの可変借用を使用
      RealFileSystem.write(file, content)
      resume()
    }
  }
}
```

### 6.4 効果の合成

```
// 複数の効果を扱う
fn program(): Int = {
  runConsole(() => {
    runState(0, () => {
      counter()
    })._1
  })
}

// 効果ハンドラを指定したスコープ
fn main(): Unit = {
  // Console効果のハンドラを指定
  with Console handled by consoleLogger {
    // このスコープ内のConsole効果はconsoleLoggerでハンドルされる
    Console.log("アプリケーション開始")
    
    // State効果のハンドラを指定
    with State<AppConfig> handled by (action => runState(defaultConfig, action)) {
      // このスコープ内のState効果はrunStateでハンドルされる
      let config = State.get()
      Console.log(s"設定: $config")
      
      // アプリケーションロジック
      runApplication()
    }
    
    Console.log("アプリケーション終了")
  }
}
```

### 6.5 リソース効果

```
// リソース効果
effect Resource<R> {
  fn acquire<E>(acquire: () -> Result<R, E>, release: (R) -> Unit): Result<R, E>
}

// リソースハンドラ
fn withResource<R, E, T>(action: () -> T with Resource<R>): T = {
  handle action() {
    Resource.acquire(acquire, release) => {
      match acquire() {
        Result.Ok(resource) => {
          try {
            // resourceの所有権を継続に渡す
            let result = resume(Result.Ok(resource))
            // resourceの所有権が戻ってくる（リソースの解放責任）
            release(resource)
            result
          } catch (e) {
            // 例外発生時もリソースを解放
            release(resource)
            throw e
          }
        },
        Result.Err(error) => resume(Result.Err(error))
      }
    }
  }
}

// 使用例
fn processFile(path: String): Result<String, IOError> with Resource<File> = {
  // リソース獲得（所有権の移動）
  let fileResult = Resource.acquire(
    () => File.open(path),
    file => file.close()  // 所有権を消費する関数を渡す
  )
  
  match fileResult {
    Result.Ok(file) => {
      // fileの所有権を使用
      let content = file.read()?
      Result.Ok(content)
    },
    Result.Err(error) => Result.Err(error)
  }
}
```

## 6. 代数的効果

### 6.1 効果の定義

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
}
```

### 6.2 効果の使用

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
```

### 6.3 効果ハンドラ

```
// 効果ハンドラ
fn runConsole<T>(action: () -> T with Console): T = {
  handle action() {
    Console.log(message) => {
      println(message)
      resume()
    }
    
    Console.readLine() => {
      let input = readLine()
      resume(input)
    }
  }
}

// 状態効果のハンドラ
fn runState<S, T>(initialState: S, action: () -> T with State<S>): (T, S) = {
  var state = initialState
  
  let result = handle action() {
    State.get() => resume(state)
    
    State.set(newState) => {
      state = newState
      resume()
    }
  }
  
  (result, state)
}
```

### 6.4 効果の合成

```
// 複数の効果を扱う
fn program(): Int = {
  runConsole(() => {
    runState(0, () => {
      counter()
    })._1
  })
}
```

## 7. モジュールシステム

### 7.1 モジュール定義

```
module Math {
  // 公開関数
  pub fn add(a: Int, b: Int): Int = a + b
  
  // 非公開関数
  fn helper(): Int = 42
}
```

### 7.2 インポート

```
// モジュールのインポート
import Math
import Math.add
import Math.{add, subtract}
import Math.*
```

## 8. 標準ライブラリ

### 8.1 コアデータ構造

```
// リスト
sealed trait List<T>
object List {
  case class Cons<T>(head: T, tail: List<T>) extends List<T>
  case object Nil extends List<Nothing>
}

// マップ
trait Map<K, V> {
  fn get(key: K): Option<V>
  fn put(key: K, value: V): Map<K, V>
  fn remove(key: K): Map<K, V>
}

// セット
trait Set<T> {
  fn contains(value: T): Bool
  fn add(value: T): Set<T>
  fn remove(value: T): Set<T>
}
```

### 8.2 I/O操作

```
effect IO {
  fn readFile(path: String): Result<String, IOError>
  fn writeFile(path: String, content: String): Result<Unit, IOError>
  fn println(message: String): Unit
  fn readLine(): String
}
```

### 8.3 並行処理

```
effect Async {
  fn spawn<T>(task: () -> T): Task<T>
  fn await<T>(task: Task<T>): T
  fn sleep(duration: Duration): Unit
}
```

## 9. 例：サンプルプログラム

### 9.1 簡単な計算機

```
// 計算機の実装
sealed trait Expr
object Expr {
  case class Number(value: Int) extends Expr
  case class Add(left: Expr, right: Expr) extends Expr
  case class Subtract(left: Expr, right: Expr) extends Expr
  case class Multiply(left: Expr, right: Expr) extends Expr
  case class Divide(left: Expr, right: Expr) extends Expr
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
  
  let result = runException(() => evaluate(expr))
  
  match result {
    Result.Ok(value) => Console.log(s"結果: $value"),
    Result.Err(error) => Console.log(s"エラー: $error")
  }
}
```

### 9.2 状態を持つカウンター

```
// カウンターの実装
fn makeCounter(initial: Int): () -> Int with State<Int> = {
  () => {
    let current = State.get()
    State.set(current + 1)
    current
  }
}

// 使用例
fn main(): Unit with Console = {
  let counter = makeCounter(0)
  
  runState(0, () => {
    Console.log(s"1回目: ${counter()}")  // 0
    Console.log(s"2回目: ${counter()}")  // 1
    Console.log(s"3回目: ${counter()}")  // 2
  })
}
```

### 9.3 ファイル処理

```
// ファイル処理の実装
fn processFile(path: String): Result<String, IOError> with IO = {
  let content = IO.readFile(path)?
  let processed = content.toUpperCase()
  IO.writeFile(path + ".processed", processed)?
  Result.Ok(processed)
}

// 使用例
fn main(): Unit with Console & IO = {
  match processFile("input.txt") {
    Result.Ok(content) => Console.log(s"処理完了: $content"),
    Result.Err(error) => Console.log(s"エラー: $error")
  }
}
```

## 10. 文法（EBNF）

以下に、Protorun言語の主要な構文をEBNF形式で示します。

```ebnf
Program ::= (Declaration | Statement)*

Declaration ::= FunctionDecl | TypeDecl | TraitDecl | ImplDecl | EffectDecl

FunctionDecl ::= "fn" Identifier GenericParams? ParamList (":" Type)? ("with" EffectType)? "=" Expression

TypeDecl ::= "type" Identifier GenericParams? "=" (RecordType | Type)
           | "sealed" "trait" Identifier GenericParams? ("{" TraitMember* "}")? ("extends" TypeRef)?

TraitDecl ::= "trait" Identifier GenericParams? ("{" TraitMember* "}")?

ImplDecl ::= "impl" GenericParams? TypeRef "for"? TypeRef "{" ImplMember* "}"

EffectDecl ::= "effect" Identifier GenericParams? "{" EffectOperation* "}"

RecordType ::= "{" (Identifier ":" Type ("," Identifier ":" Type)*)? "}"

TraitMember ::= FunctionDecl

ImplMember ::= FunctionDecl

EffectOperation ::= "fn" Identifier GenericParams? ParamList (":" Type)? ";"

ParamList ::= "(" (Param ("," Param)*)? ")"

Param ::= Identifier ":" Type

GenericParams ::= "<" (GenericParam ("," GenericParam)*)? ">"

GenericParam ::= Identifier (":" TypeConstraint)?

TypeConstraint ::= TypeRef (("&" | "|") TypeRef)*

Type ::= TypeRef
       | FunctionType
       | TupleType
       | ArrayType

TypeRef ::= Identifier GenericArgs?

GenericArgs ::= "<" (Type ("," Type)*)? ">"

FunctionType ::= "(" (Type ("," Type)*)? ")" "->" Type ("with" EffectType)?

TupleType ::= "(" Type ("," Type)+ ")"

ArrayType ::= "[" Type "]"

EffectType ::= TypeRef ("&" TypeRef)*

Statement ::= Expression ";"
            | "let" Pattern (":" Type)? "=" Expression ";"
            | "var" Identifier (":" Type)? "=" Expression ";"
            | "return" Expression? ";"

Expression ::= LiteralExpr
             | IdentifierExpr
             | BlockExpr
             | IfExpr
             | MatchExpr
             | ForExpr
             | LambdaExpr
             | CallExpr
             | MemberAccessExpr
             | BinaryExpr
             | UnaryExpr
             | HandleExpr

LiteralExpr ::= IntLiteral | FloatLiteral | StringLiteral | BoolLiteral | UnitLiteral

IdentifierExpr ::= Identifier

BlockExpr ::= "{" Statement* (Expression)? "}"

IfExpr ::= "if" Expression BlockExpr ("else" (IfExpr | BlockExpr))?

MatchExpr ::= "match" Expression "{" (Pattern ("if" Expression)? "=>" Expression ",")* "}"

ForExpr ::= "for" "{" (Pattern "<-" Expression ("if" Expression)?)* "}" "yield" Expression

LambdaExpr ::= ParamList "=>" Expression

CallExpr ::= Expression "(" (Expression ("," Expression)*)? ")"

MemberAccessExpr ::= Expression "." Identifier

BinaryExpr ::= Expression Operator Expression

UnaryExpr ::= Operator Expression

HandleExpr ::= "handle" Expression "{" (EffectCase)* "}"

EffectCase ::= QualifiedIdentifier ParamList "=>" BlockExpr

Pattern ::= LiteralPattern
          | IdentifierPattern
          | TuplePattern
          | ConstructorPattern
          | WildcardPattern

LiteralPattern ::= LiteralExpr

IdentifierPattern ::= Identifier

TuplePattern ::= "(" Pattern ("," Pattern)* ")"

ConstructorPattern ::= QualifiedIdentifier ("(" Pattern ("," Pattern)* ")")?

WildcardPattern ::= "_"

QualifiedIdentifier ::= (Identifier ".")* Identifier
```

## 11. 今後の展望

Protorun言語は現在開発中であり、以下の機能を今後追加する予定です：

- **依存型**: より表現力の高い型システム
- **線形型**: リソース使用の厳密な制御
- **並行処理モデル**: アクターモデルや軽量スレッド
- **メタプログラミング**: コンパイル時計算とコード生成
- **インクリメンタルコンパイル**: 高速な開発サイクル
- **言語サーバープロトコル**: IDE統合
- **パッケージマネージャ**: ライブラリエコシステム

## 12. 参考文献

- Scala言語仕様
- Haskell言語仕様
- Rust言語リファレンス
- "Types and Programming Languages" by Benjamin C. Pierce
- "Practical Foundations for Programming Languages" by Robert Harper
- "Programming with Algebraic Effects and Handlers" by Andrej Bauer and Matija Pretnar
