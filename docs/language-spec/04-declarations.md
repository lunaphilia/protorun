# 4. 宣言

## 4.1 宣言の概要

Protorun言語における宣言は、プログラムの構成要素（変数束縛、関数、型、トレイト、効果、ハンドラなど）を定義するための構文です。宣言は、新しい名前をスコープに導入し、それに紐づく実体（値、型、振る舞いなど）を定義します。

Protorunの宣言は、主に `let` キーワードを用いた束縛宣言に統一されています。これにより、言語の構文的な一貫性が高まります。トレイト実装 (`impl`) のみが例外的に専用のキーワードを持ちます。

トップレベルのプログラムは、これらの宣言と式の並びで構成されます。

この章では、Protorunで利用可能な主要な宣言について説明します。具体的には以下のものが含まれます：

- **束縛宣言 (`let`)**: イミュータブルまたはミュータブルな束縛を導入します。`mut` キーワードの有無で可変性を制御します。右辺には通常の式に加え、関数式、型定義式、トレイト定義式なども指定できます。
- **トレイト実装 (`impl`)**: 型に対してトレイトの振る舞いを実装します。
- **エクスポート宣言 (`export`)**: モジュールから外部へ公開する要素を指定します。`let` (不変・可変問わず) および `impl` 宣言の前に `export` キーワードを付与できます。（[9. モジュール](09-modules.md) で詳述されます）

(効果とハンドラの詳細については [8. 代数的効果](08-algebraic-effects.md) を参照してください。)

## 4.2 束縛宣言 (let)

### 4.2.1 概要と設計原則

Protorunでは、`let` キーワードを用いて名前付きの束縛を宣言します。これには、値（変数）、関数、型、トレイト、効果、ハンドラ、型エイリアスなどが含まれます。`mut` キーワードを付与することで、可変な束縛を宣言できます。

```protorun
// 不変束縛（値）
let x = 42
let y: Int = 42

// 可変束縛（値）
let mut counter = 0
let mut name: String = "John"

// 関数定義 (関数式を束縛)
let add = fn (a: Int, b: Int): Int => a + b

// 型定義 (型定義式を束縛)
let Point = type { x: Float, y: Float }
let Option = enum<T> { Some(T), None }

// 型エイリアス定義 (型エイリアス定義式を束縛)
let UserId = alias Int
```

束縛宣言の設計は、以下の原則に基づいています：

1.  **統一された構文**: すべての束縛宣言が `let ("mut")? LetPattern = Expression` の形式に従い、言語の学習コストと複雑さを低減します。型定義なども式の一種として扱われます。
2.  **不変性の優先**: デフォルトでは束縛は不変 (`let` のみ) であり、これにより予測可能性と安全性が向上します。これは関数型プログラミングの原則に沿った設計です。不変性により、コードの理解と推論が容易になり、並行処理における安全性も向上します。
3.  **明示的な可変性**: 値の可変性が必要な場合は `mut` キーワードを `let` の後に付与して明示的に宣言します (`let mut`)。これにより、状態が変化する箇所をコード上で明確に識別できます。
4.  **型推論**: 多くの場合、初期化式から値束縛の型が推論されるため、型注釈 (`: Type`) は省略可能です。これによりコードが簡潔になります。
5.  **静的型付け**: 型注釈を省略した場合でも、すべての束縛はコンパイル時に明確な型を持ちます。これにより型安全性が保証され、実行時エラーを削減します。必要に応じて明示的に型を指定することも可能です。
6.  **限定された分解束縛**: `let` 宣言の左辺 (`LetPattern`) では、識別子、タプル、レコードのパターンによる分解束縛が可能です。`match` 式で使える全てのパターンが許可されるわけではありません。
7.  **`let mut` の制約**: `let mut` を使用する場合、左辺のパターンは単一の識別子 (`LetIdentifierPattern`) でなければなりません。この制約は意味論解析でチェックされます。

### 4.2.2 構文と意味

`let` 宣言は、新しい名前束縛を現在のスコープに導入します。`mut` キーワードの有無によって、束縛が不変か可変かが決まります。

**構文:**

```ebnf
LetDecl ::= ("export")? "let" ("mut")? LetPattern (":" Type)? "=" Expression

LetPattern ::= LetIdentifierPattern
             | LetTuplePattern
             | LetRecordPattern

LetIdentifierPattern ::= Identifier
LetTuplePattern ::= "(" (LetPattern ("," LetPattern)*)? ")"
LetRecordPattern ::= TypeRef "{" (LetRecordFieldPattern ("," LetRecordFieldPattern)*)? ("," "..")? "}"
LetRecordFieldPattern ::= Identifier (":" LetPattern)?
```
([12. 文法](12-grammar.md) も参照)

- `let`: 宣言を開始するキーワード。
- `("mut")?`: **任意**の `mut` キーワード。存在する場合、可変束縛となります。
- `LetPattern`: 束縛する名前（または複数の名前）を指定するパターン。以下の形式が許可されます。
    - `LetIdentifierPattern`: 単一の識別子 (例: `x`)。
    - `LetTuplePattern`: タプルの分解束縛 (例: `(a, b)`)。
    - `LetRecordPattern`: レコードの分解束縛 (例: `Point { x, y }`)。
    `match` 式で使える `LiteralPattern` や `ConstructorPattern` などは `let` 宣言では使用できません。
- `(: Type)?`: **任意**の型注釈。値束縛の場合に利用でき、省略された場合は右辺の `Expression` から推論されます。定義式の場合は通常、型注釈は不要です。
- `= Expression`: 束縛する対象を評価または解釈する式。これには、通常の計算式、関数式、そして型定義式、トレイト定義式なども含まれます。`let` 宣言では**右辺の式が必須**です。

**意味:**

`let` 宣言は以下の動作を行います。

1.  右辺の `Expression` を評価または解釈します。
2.  結果の値、関数、型、トレイトなどを左辺の `LetPattern` に束縛します。値の場合はパターンマッチングが行われます。
3.  束縛された名前は、宣言された時点から現在のスコープの終わりまで有効です。
4.  `mut` キーワードがない場合、束縛された名前は**不変**であり、後から別の値や定義を再代入することはできません。
5.  `mut` キーワードがある場合 (`let mut`)、束縛された名前は**可変**であり、後から代入演算子 (`=`) を使って同じ型の別の式の結果を代入することができます。ただし、`let mut` の左辺は単一の識別子でなければなりません（意味論チェック）。

**`let mut` と定義式:**
構文上は `let mut` の右辺に型定義式 (`type { ... }`) などを記述できますが、これは主に構文の一貫性を保つための措置です。`let mut` で定義式を束縛した場合の正確な意味論や、再代入の可否については現在検討中です。現時点では、`let mut` は主に可変の値（変数）の束縛に使用することを想定しています。将来的に、定義式の束縛に対して制限が加えられる可能性があります。

**具体例:**

```protorun
// 値の束縛 (不変)
let message = "Hello"
let count: Int = 100

// パターンによる値の分解束縛
let point_value = (10, 20)
let (x, y) = point_value

// レコードの値の分解束縛
let person_value = Person { name: "Alice", age: 30 }
let { name: person_name, age } = person_value

// 関数定義 (関数式を束縛)
let square = fn x => x * x
let identity = fn <T> (x: T): T => x

// 型定義 (型定義式を束縛)
let Person = type { name: String, age: Int }
let Result = enum<T, E> { Ok(T), Err(E) }

// トレイト定義 (トレイト定義式を束縛)
let Show = trait { fn show(self): String }
let Ord = trait<T: Eq> { fn compare(self, other: T): Int }

// 型エイリアス定義 (型エイリアス定義式を束縛)
let UserId = alias Int
let StringMap = alias<T> Map<String, T>

// スコープ
fn example_scope() {
  let outer = "Outer scope"
  {
    let inner = "Inner scope"
    println(outer) // OK
    println(inner) // OK
  }
  println(outer) // OK
  // println(inner) // エラー: inner はスコープ外
}
```

**不変性の利点:**

`let` による不変束縛をデフォルトとすることで、プログラムの特定の部分で値や定義が変わらないことが保証されます。これにより、コードの可読性、予測可能性、そして特に並行処理における安全性が向上します。状態の変化を追跡する必要がある箇所が減り、プログラムの理解やデバッグが容易になります。

**`let` による関数定義:**

関数は `let` 束縛と関数式 ([6.3.3 関数式](06-expressions.md#633-関数式)) を組み合わせて定義します。これにより、関数も他の値と同様に扱われ、言語の一貫性が保たれます。詳細な構文は [6. 式](06-expressions.md) を参照してください。

```protorun
let add = fn (a: Int, b: Int): Int => a + b
let square = fn x => x * x
let identity = fn <T> (x: T): T => x
```

**暗黙的な再帰:**

`let` で束縛された関数式は、自身の名前を再帰的に参照できます。特別なキーワード (`rec` など) は不要です。

```protorun
let factorial = fn (n: Int): Int => {
  if n <= 1 {
    1
  } else {
    n * factorial(n - 1) // 再帰呼び出し
  }
}
```

ただし、シャドウイングには注意が必要です。同じスコープ内で同名の束縛を再宣言すると、意図しない動作を引き起こす可能性があります。

### 4.2.3 可変性 (`mut`)

Protorunでは不変性がデフォルトであり推奨されますが、アルゴリズムの実装（例: ループカウンター、状態の蓄積）や特定のデータ構造の効率的な更新など、可変性が必要となる場面もあります。`let mut` はそのような場合に、状態変化を伴うことを明示的に示すために使用されます。

```protorun
// 可変変数の宣言と初期化
let mut counter = 0
let mut name: String = "Bob"

// 再代入
counter = counter + 1
name = "Charlie"

// counter = "error" // エラー: 型が異なる値は代入できない
```

可変変数の使用は、プログラムのどの部分で状態が変化しうるかを理解する上で重要になるため、そのスコープを可能な限り小さく保つことが推奨されます。

## 4.3 型定義 (type / enum)

Protorun言語では、`let` 宣言と**型定義式** (`type` または `enum` キーワードで始まる式) を組み合わせて、新しい型を定義します。これにより、コードの抽象化レベルを高め、データ構造を明確に表現することが可能になります。

型定義式の詳細な構文については [6. 式](06-expressions.md) の章を参照してください。

### 4.3.1 レコード型定義 (構造体)

レコード型は、名前付きフィールドを持つ複合データ型を定義します。`let` と `type` キーワードを用いた型定義式で定義します。

**宣言 (束縛):**

```protorun
let TypeName = type<GenericParams>? { /* フィールド定義 */ }
```

**具体例:**

```protorun
let Person = type {
  name: String,
  age: Int
}

let Pair = type<A, B> {
  first: A,
  second: B
}
```

レコード型は、アプリケーションのドメインモデル（例: ユーザー、製品、注文など）を表現するのに適しています。

### 4.3.2 代数的データ型定義 (enum)

代数的データ型（ADT）は、複数の異なる可能性のある構造（ヴァリアント）を一つの型として定義する方法です。`let` と `enum` キーワードを用いた型定義式を使用します。

**宣言 (束縛):**

```protorun
let TypeName = enum<GenericParams>? { /* ヴァリアント定義 */ }
```

**具体例:**

```protorun
let Option = enum<T> {
  Some(T),
  None
}

let Result = enum<T, E> {
  Ok(T),
  Err(E)
}

let Shape = enum {
  Circle(radius: Float),
  Rectangle(width: Float, height: Float),
  Point
}
```

代数的データ型は、特にエラーハンドリング (`Result`)、オプションの値 (`Option`)、状態機械、ツリー構造などの表現に強力です。

## 4.4 型エイリアス定義 (alias)

型エイリアスは、既存の型に新しい名前（エイリアス）を付ける機能です。`let` 宣言と `alias` キーワードを用いた**型エイリアス定義式**を組み合わせて定義します。

型エイリアス定義式の詳細な構文については [6. 式](06-expressions.md) の章を参照してください。

**宣言 (束縛):**

```protorun
let AliasName = alias<GenericParams>? ExistingType<GenericParams>
```

**具体例:**

```protorun
let UserId = alias Int
let StringMap = alias<T> Map<String, T>
let PointTuple = alias (Float, Float)
let Callback = alias fn (Int): String
```

型エイリアスは、特にジェネリック型や関数型など、型シグネチャが長くなりがちな場合にコードを整理し、理解しやすくするのに有効です。

## 4.5 効果とハンドラ定義 (effect / handler)

**効果定義式** (`effect`) と**ハンドラ定義式** (`handler`) は、代数的効果システムの中核となる宣言であり、`let` を用いて定義されます。

これらの定義式の詳細な構文については [6. 式](06-expressions.md) および [8. 代数的効果](08-algebraic-effects.md) の章を参照してください。

### 4.5.1 効果インターフェース定義 (effect)

効果インターフェースは、計算効果が提供する操作のシグネチャ（インターフェース）を定義します。`let` と `effect` キーワードを用いた効果定義式で定義します。

**宣言 (束縛):**

```protorun
let EffectName = effect<GenericParams>? { /* 操作シグネチャ */ }
```

**具体例:**

```protorun
let State = effect<S> {
  fn get(): S
  fn put(value: S): Unit
}

let Console = effect {
  fn log(message: String): Unit
}
```

### 4.5.2 ハンドラ定義 (handler)

ハンドラ定義は、特定の型に対して、特定の効果インターフェースの操作を実装する方法を定義します。`let` と `handler` キーワードを用いたハンドラ定義式で定義します。

**宣言 (束縛):**

```protorun
let HandlerName = handler<GenericParams>? EffectName<EffectArgs> for TargetType<TargetArgs> { /* 操作実装 */ }
```

**具体例:**

```protorun
let CounterStateHandler = handler State<Int> for CounterState {
  let get = fn (self): Int => self.count
  let put = fn (self, value: Int): Unit => {
    resume_with(Unit, CounterState { count: value })
  }
}
```

## 4.6 トレイト定義 (trait) と実装 (impl)

### 4.6.1 トレイト定義 (trait)

トレイトは、型の振る舞いを定義するインターフェースです。`let` と `trait` キーワードを用いた**トレイト定義式**を用いて定義します。

トレイト定義式の詳細な構文については [6. 式](06-expressions.md) の章を参照してください。

**宣言 (束縛):**

```protorun
let TraitName = trait<GenericParams>? (: SuperTrait<SuperArgs>)? { /* メソッドシグネチャ / デフォルト実装 */ }
```

**具体例:**

```protorun
let Show = trait {
  fn show(self): String
}

let Eq = trait {
  fn equals(self, other: Self): Bool
}

let Ord = trait: Eq {
  fn compare(self, other: Self): Int
  fn equals(self, other: Self): Bool = self.compare(other) == 0 // デフォルト実装
}

let Add = trait<Rhs = Self, Output = Self> {
    fn add(self, rhs: Rhs): Output
}
```

### 4.6.2 トレイト実装 (impl)

トレイト実装は、特定の型に対して特定のトレイトが定義するメソッドを実装します。これは `let` 宣言の例外であり、`impl` キーワードを使用します。

**構文:**

```ebnf
ImplDeclaration ::= 'impl' <GenericParams>? TraitName<TraitArgs> for TypeName<TypeArgs> <WhereClause>? '{'
                      ImplItem*
                   '}'
ImplItem ::= LetDecl // 主に関数定義
```

- `impl`: 実装を開始するキーワード。
- `<GenericParams>?`: 実装自体がジェネリックな場合の型パラメータ。
- `TraitName<TraitArgs>`: 実装するトレイト。
- `TypeName<TypeArgs>`: 実装対象の型。
- `<WhereClause>?`: 型パラメータに対する追加の制約（例: `where T: Show`）。
- `ImplItem*`: トレイトメソッドの実装。通常は `let method_name = fn ...` の形式。

**具体例:**

```protorun
impl Show for Int {
  let show = fn (self): String => self.toString()
}

impl<T> Show for Option<T> where T: Show {
  let show = fn (self): String => match self {
    Option.Some(v) => s"Some(${v.show()})",
    Option.None => "None"
  }
}

impl Add for Int {
    let add = fn (self, rhs: Int): Int => self + rhs
}
```

**トレイトシステムの設計原則:**

Protorun言語のトレイトシステムは、以下の原則に基づいて設計されています：

1.  **インターフェース抽象化**: 型の振る舞いを抽象化し、多相的なコードを書くことができます。
2.  **型安全な多相性**: トレイト制約により、ジェネリックコードの型安全性を保証します。
3.  **コード再利用**: デフォルト実装により、共通の機能を再利用できます。
4.  **単一継承**: 複雑さを抑えるために、トレイトの継承は単一継承のみをサポートします。

トレイトシステムは、Haskellの型クラスとRustのトレイトからインスピレーションを得ています。特に：

- **単一継承の選択**: 多重継承の複雑さ（ダイヤモンド問題など）を避けるために、単一継承のみをサポートしています。これにより、型システムの複雑さを抑えつつ、必要な抽象化を実現しています。
- **デフォルト実装**: トレイトメソッドにデフォルト実装を提供できます。これにより、トレイトを実装する型の実装負担を軽減し、コードの再利用性を高めています。
- **ジェネリックな実装**: 型パラメータにトレイト制約を付けることで、型安全なジェネリックコードを書くことができます。これにより、型の抽象化と具体的な実装の分離が可能になります。
