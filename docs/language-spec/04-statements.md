# 4. 文

## 4.1 文の概要

Protorun言語における「文 (Statement)」は、プログラムの実行フローを制御するための構文要素です。現在の仕様では、文として分類されるのは **`return` 文のみ**です。

文は関数本体やブロック式 `{...}` の内部など、特定のコンテキストで使用されます。

- **return文 (`ReturnStatement`)**: 現在の関数から値を返します。（[4.2 return文](#42-return文-return-statement)参照）

以前の仕様では、式を単独で記述する「式文」や、変数束縛のための `let` 文、`var` 文も文として扱われることがありましたが、以下のように整理されました。

-   **式**: 式は評価されて値を生成します。副作用のためだけに式を実行する場合（例: `println("Hello")`）、それはブロック式 `{...}` の要素 (`BlockItem::Expression`) として扱われます。詳細は [5. 式](05-expressions.md) の章を参照してください。
-   **`let`/`var`**: これらは新しい名前をスコープに導入する「宣言」として分類されます。詳細は [3. 宣言](03-declarations.md) の章を参照してください。

Protorunでは、文や宣言の終わりを示すためにセミコロンは**不要**です。通常、**改行**が要素の区切りとして解釈されます。

```protorun
// 式文の例
println("Hello") // println呼び出しは式文

// 複数行にわたる式も改行で区切られる
let temp = 1 + 2 + // これは let 宣言
           3 + 4
println(temp) // これはトップレベルの式
```

複数の宣言や式を一つのまとまりとして扱いたい場合は、**ブロック式** `{ ... }` を使用します。ブロック式はそれ自体も式であり、値を返すことができます。詳細は [5. 式](05-expressions.md#52-ブロック式) の章で説明します。

この章では、`return` 文について詳しく説明します。

## 4.2 return文 (Return Statement)

**構文:**

```ebnf
ReturnStatement ::= 'return' Expression?
```
(式の詳細な文法は [5. 式](05-expressions.md) を参照)

**意味:**

`return` 文は、現在の関数 (`fn`) の実行を即座に終了し、制御を関数の呼び出し元に戻します。

- `Expression` が指定されている場合、その式が評価され、その結果が関数の返り値として呼び出し元に渡されます。`return` される値の型は、関数の宣言された返り値型と一致している必要があります。
- `Expression` が省略されている場合、`Unit` 値が返り値として渡されます。これは、返り値型が `Unit` である関数で使用されます。

`return` 文は、関数の途中の任意の場所で使用でき、早期リターン（Early Return）を実現します。

**関数本体の最後の式との関係:**

Protorunの関数本体（通常はブロック式 `{...}`）では、ブロック内の最後の式が評価され、その結果が関数の暗黙的な返り値となります。`return` 文は、この暗黙的なリターンよりも優先され、関数の実行をその場で終了させます。

```protorun
fn check(x: Int): String {
  if x < 0 {
    return "Negative" // 早期リターン
  }
  // x >= 0 の場合、以下のブロックの最後の式が返る
  {
    println("Processing positive number...")
    "Non-negative" // この文字列が暗黙的な返り値となる
  }
}

fn process(): Unit { // または fn process() { ... } と書いても同じ
  println("Start processing")
  if should_stop_early() {
     return // 式を省略すると Unit を返す (早期リターン)
  }
  println("Continue processing")
  // 関数の最後に到達した場合、暗黙的に Unit が返る
}
```

**具体例:**

```protorun
// 条件に合致したら早期に値を返す
fn find_first(items: List<Int>, target: Int): Option<Int> {
  for item in items {
    if item == target {
      return Some(item) // 見つかったら即座に Some(item) を返す
    }
  }
  None // ループで見つからなければ、最後の式として None が返る
}

// Unit を返す関数での早期リターン
fn print_positive(x: Int): Unit {
  if x <= 0 {
    return // x が正でない場合は何もせずに関数を終了
  }
  println(x)
}

// ネストした関数やラムダ式からの return は、
// それを直接含む最も内側の fn で定義された関数から抜けることに注意
fn outer(): Int {
  let inner_lambda = |y: Int| -> Int {
    if y == 0 {
      // return 100 // これは outer 関数から return するのではなく、
                   // ラムダ式からの return (もし許可されるなら) か、
                   // あるいはコンパイルエラーになる可能性がある (言語仕様による)
                   // Protorun の詳細仕様では、ラムダ式からの非ローカルリターンは
                   // 効果などで明示的に扱う必要があるかもしれない。
                   // ここでは単純な fn からの return を示す。
    }
    y * 2
  }
  if some_condition() {
    return 0 // outer 関数から 0 を返す
  }
  inner_lambda(5) // outer 関数の暗黙のリターン値
}
```

**注意点:**

- `return` 文は `fn` で定義された関数の内部でのみ使用できます。トップレベルやモジュールレベルで直接使用することはできません。
- ラムダ式やクロージャ内での `return` の挙動（それが定義された関数から抜けるのか、ラムダ式自体から抜けるのか）は言語仕様で明確に定義される必要があります。多くの場合、非ローカルリターン（定義された関数から抜ける）には特別な構文や効果が必要となることがあります。Protorunにおける正確な仕様は別途確認が必要です。（現時点では、`fn` からの直接的なリターンを想定しています。）
