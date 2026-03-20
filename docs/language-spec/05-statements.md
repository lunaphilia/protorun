# 5. 文

## 5.1 文の概要

Protorun言語における「文 (Statement)」は、プログラムの実行フローを制御するための構文要素です。現在の仕様で `Statement` として分類されるのは **`return` 文のみ**です。

一方で、ループ制御は文ではなく式として提供されます。

- **return文 (`ReturnStatement`)**: 現在の関数から値を返します。（[5.2 return文](#52-return文-return-statement)参照）
- **while式 / for式 / loop式**: 反復処理を行う式です。（[5.3 ループ式](#53-ループ式)参照）
- **break式 / continue式**: ループの制御を行う式です。（[5.4 break と continue](#54-break-と-continue)参照）

なお、式（[6. 式](06-expressions.md) 参照）や宣言（[4. 宣言](04-declarations.md) 参照）は、Protorunにおいて「文」とは区別されます。`while` / `for` / `loop` / `break` / `continue` は文法上 `Expression` に含まれます。

Protorunでは、文や宣言の終わりを示すためにセミコロンは不要です。通常、改行が要素の区切りとして解釈されます。

## 5.2 return文 (Return Statement)

**構文:**

```ebnf
ReturnStatement ::= "return" Expression?
```

（式の詳細な文法は [6. 式](06-expressions.md) を参照）

**意味:**

`return` 文は、現在の関数 (`fn`) の実行を即座に終了し、制御を関数の呼び出し元に戻します。

- `Expression` が指定されている場合、その式が評価され、その結果が関数の返り値として呼び出し元に渡されます。`return` される値の型は、関数の宣言された返り値型と一致している必要があります。
- `Expression` が省略されている場合、`Unit` 値が返り値として渡されます。これは、返り値型が `Unit` である関数で使用されます。

`return` 文は、関数の途中の任意の場所で使用でき、早期リターン（Early Return）を実現します。

**関数本体の最後の式との関係:**

Protorunの関数本体（通常はブロック式 `{...}`）では、ブロック内の最後の式が評価され、その結果が関数の暗黙的な返り値となります。`return` 文は、この暗黙的なリターンよりも優先され、関数の実行をその場で終了させます。

```protorun
let check = fn(x: Int) -> String = {
  if x < 0 then {
    return "Negative"
  }
  {
    println("Processing positive number...")
    "Non-negative"
  }
}

let process = fn() -> Unit = {
  println("Start processing")
  if should_stop_early() then {
    return
  }
  println("Continue processing")
}
```

**具体例:**

```protorun
let find_first = fn(items: List[Int], target: Int) -> Option[Int] = {
  for item in items then {
    if item == target then {
      return Option.Some(item)
    }
  }
  Option.None
}

let print_positive = fn(x: Int) -> Unit = {
  if x <= 0 then {
    return
  }
  println(x)
}

let outer = fn() -> Int = {
  let inner_lambda = fn(y: Int) -> Int = {
    if y == 0 then {
      100
    } else {
      y * 2
    }
  }
  if some_condition() then {
    return 0
  }
  inner_lambda(5)
}
```

**注意点:**

- `return` 文は `fn` で定義された関数の内部でのみ使用できます。トップレベルやモジュールレベルで直接使用することはできません。
- ラムダ式やクロージャ内での `return` の挙動（それが定義された関数から抜けるのか、ラムダ式自体から抜けるのか）は言語仕様で明確に定義される必要があります。多くの場合、非ローカルリターンには特別な構文や効果が必要です。

## 5.3 ループ式

### 5.3.1 while式

**構文:**

```ebnf
WhileExpr ::= "while" Expression "then" Expression
```

**意味:**

- 条件式 (`Expression`) を評価し、`True` であれば本体 (`Expression`) を評価します。
- 本体の評価後、再び条件を評価します（繰り返し）。
- 条件が `False` になったとき、`while` 式全体の値は `Unit` です。
- `break expr` によって脱出した場合、`while` 式全体の値は `expr` の値です。

```protorun
let mut i = 0
while i < 10 then {
  println(f"i = {i}")
  i = i + 1
}
```

### 5.3.2 for式

**構文:**

```ebnf
ForExpr ::= "for" Pattern "in" Expression "then" Expression
```

**意味:**

- `in` の右辺の式を評価し、`Iterable[T]` を実装する値を取得します。
- `.iter()` を呼び出してイテレータを取得します。
- イテレータの `next()` が `Option.Some(value)` を返す間、`value` を `Pattern` に束縛して本体を評価します。
- `next()` が `Option.None` を返したとき、`for` 式全体の値は `Unit` です。
- `break expr` によって脱出した場合、`for` 式全体の値は `expr` の値です。

**デシュガー:**

```protorun
for x in list then body
// ↓ 等価
let mut iter = list.iter()
loop {
  match iter.next() {
    Option.Some(x) => body
    Option.None => break
  }
}
```

```protorun
let numbers = [1, 2, 3, 4, 5]
for n in numbers then {
  println(f"n = {n}")
}
```

### 5.3.3 loop式

**構文:**

```ebnf
LoopExpr ::= "loop" Expression
```

**意味:**

- 本体 (`Expression`) を無限に繰り返します。
- `break expr` によってのみ脱出できます。
- `loop` 式全体の値は `break expr` の `expr` の値です。
- `break` なしの `loop` は発散（diverge）します。

```protorun
let result = loop {
  let input = readLine()
  if input == "quit" then {
    break 0
  } else {
    println(f"You said: {input}")
  }
}
```

## 5.4 break と continue

### 5.4.1 break式

**構文:**

```ebnf
BreakExpr ::= "break" Expression?
```

**意味:**

- 最も内側のループ（`while`, `for`, `loop`）から脱出します。
- `Expression` が指定された場合、その値がループ式全体の値となります。
- `Expression` が省略された場合、`Unit` がループ式全体の値となります。

**代数的効果としての意味論:**

`break` は代数的効果として実装されます。`while`, `for`, `loop` は暗黙的に `Break` 効果のハンドラを提供します。`break expr` が評価されると現在の継続は破棄され、ループ式全体の値として `expr` が返されます。

```protorun
let found = loop {
  let item = getNextItem()
  if item.isTarget() then {
    break item
  }
}
```

### 5.4.2 continue式

**構文:**

```ebnf
ContinueExpr ::= "continue"
```

**意味:**

- 最も内側のループの現在の反復をスキップし、次の反復に進みます。
- `while` の場合は条件の再評価に戻ります。
- `for` の場合は次の要素の処理に進みます。
- `loop` の場合はループ先頭に戻ります。

**代数的効果としての意味論:**

`continue` は代数的効果として実装されます。`while`, `for`, `loop` は暗黙的に `Continue` 効果のハンドラを提供します。`continue` が評価されると現在の反復の残りの計算は破棄され、次の反復が開始されます。

```protorun
for n in numbers then {
  if n % 2 == 0 then {
    continue
  }
  println(f"奇数: {n}")
}
```
