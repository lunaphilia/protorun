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

## 2. 字句構造

### 2.1 キーワード

```
fn       let      var      if       else     match    case
return   for      while    in       trait    impl     type
effect   handle   resume   try      catch    throw    with
module   import   pub      sealed   object   extends  as
```

### 2.2 演算子

```
+  -  *  /  %  =  ==  !=  <  >  <=  >=  &&  ||  !  &  |  ^  ~  <<  >>
->  =>  ::  .  ,  ;  :  ?  ??  ..  ...  +=  -=  *=  /=  %=  &=  |=  ^=
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

### 3.2 複合型

```
// 配列型
[T]

// タプル型
(T1, T2, ..., Tn)

// 関数型
(T1, T2, ..., Tn) -> R
(T1, T2, ..., Tn) -> R with E

// オプション型
Option<T>

// 結果型
Result<T, E>
```

### 3.3 型定義

```
// レコード型（構造体）
type Person = {
  name: String,
  age: Int
}

// 代数的データ型（シールドトレイト + ケースクラス/オブジェクト）
sealed trait Option<T>
object Option {
  case class Some<T>(value: T) extends Option<T>
  case object None extends Option<Nothing>
}

// 型エイリアス
type StringMap<T> = Map<String, T>
```

### 3.4 トレイト（インターフェース）

```
trait Show<T> {
  fn show(value: T): String
}

trait Eq<T> {
  fn equals(a: T, b: T): Bool
}

// トレイト実装
impl Show<Int> {
  fn show(value: Int): String = value.toString()
}

// ジェネリックな実装
impl<T: Show<T>> Show<Option<T>> {
  fn show(value: Option<T>): String = match value {
    Option.Some(v) => s"Some(${Show.show(v)})",
    Option.None => "None"
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
