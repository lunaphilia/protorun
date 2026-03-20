# 5. 式

## 5.1 式の概要と設計原則

Protorun は式ベースの言語です。多くの構文要素は値を返す `Expression` として扱われます。

- 一貫性: 似た構文は同じ規則で評価される
- 合成性: 式を他の式の中に自然に埋め込める
- 表現力: 複雑な処理を簡潔に記述できる
- 参照透過性: 純粋な式は同じ入力に対して同じ結果を返す

式は値を返します。副作用を持つ式も存在しますが、値の流れを明示しやすい構造を優先します。

---

## 5.2 ブロック式 (`BlockExpr`)

ブロック式は 0 個以上の `BlockItem` を `{}` で囲んだ式です。

```ebnf
BlockExpr ::= "{" BlockItem* "}"
BlockItem ::= Declaration | Statement | Expression
```

- 最後の要素が式なら、その評価結果がブロック全体の値
- 最後が宣言または文、または空ブロックなら `Unit`
- ブロック内で宣言した識別子のスコープはブロック内に限定

```protorun
let a = 10

let b = {
  let a = 20
  a + 5
}

let logged = {
  Console.println("side effect")
  42
}
```

---

## 5.3 条件式 (`IfExpr`)

`if` は式です。構文は `if cond then expr` で、`elif` と `else` を任意で連結できます。

```ebnf
IfExpr ::= "if" Expression "then" Expression
           ("elif" Expression "then" Expression)*
           ("else" Expression)?
```

分岐本体には単一式もブロック式も置けます。複数行を使う場合はブロック式を使います。

```protorun
let sign = if x > 0 then {
  "positive"
} elif x < 0 then {
  "negative"
} else {
  "zero"
}
```

---

## 5.4 パターンマッチ (`MatchExpr`)

`match` は `scrutinee` を複数パターンに照合し、最初に一致した分岐の式を評価します。

```ebnf
MatchExpr ::= "match" Expression "{" MatchCase* "}"
MatchCase ::= Pattern ("if" Expression)? "=>" Expression
```

```protorun
let desc = match value {
  0 => "zero"
  n if n < 0 => "negative"
  _ => "other"
}

let render = match result {
  Result.Ok(v) => {
    log_ok(v)
    f"ok: {v}"
  }
  Result.Err(e) => {
    log_error(e)
    f"err: {e}"
  }
}
```

### 5.4.1 パターン

主要なパターンは次の通りです。

- リテラルパターン: `0`, `"text"`, `True`
- 識別子パターン: `x`
- タプルパターン: `(x, y)`
- コンストラクタパターン: `Option.Some(v)`, `Result.Err(e)`
- ワイルドカード: `_`

```protorun
let message = match pair {
  (0, y) => f"first is zero, second is {y}"
  (x, 0) => f"first is {x}, second is zero"
  _ => "neither"
}
```

---

## 5.5 リテラル式 (`LiteralExpr`, `StringInterpolation`)

### 5.5.1 基本リテラル

- 数値: `1`, `42`, `3.14`
- 文字列: `"hello"`
- 真偽値: `True`, `False`
- Unit: `()`

### 5.5.2 文字列補間

```ebnf
StringInterpolation ::= "f" "\"" (StringContent | "{" Expression "}")* "\""
```

```protorun
let name = "World"
let greeting = f"Hello, {name}!"
let detail = f"2 * 21 = {2 * 21}"
```

### 5.5.3 コレクションリテラル

```ebnf
ListLiteral ::= "[" (Expression ("," Expression)*)? "]"
MapLiteral ::= "{" (MapEntry ("," MapEntry)*)? "}"
MapEntry ::= Expression ":" Expression
SetLiteral ::= "#{" (Expression ("," Expression)*)? "}"
```

```protorun
let numbers = [1, 2, 3]
let scores = {"apple": 1, "banana": 2}
let point_set = #{(0, 0), (1, 1)}
```

---

## 5.6 関数呼び出し式 (`CallExpr`)

呼び出しは `callee(arg1, arg2, ...)` で表現します。

```ebnf
CallExpr ::= Expression "(" (Expression ("," Expression)*)? ")"
```

```protorun
let sum = add(10, 20)
let empty = make_list()
```

---

## 5.7 メンバーアクセス式 (`MemberAccessExpr`) と UFCS

ドット `.` でメンバーへアクセスします。`x.f(y)` は UFCS として `f(x, y)` と同等に扱えます。

```ebnf
MemberAccessExpr ::= Expression "." Identifier
```

```protorun
let upper = "hello".to_upper()
let also_upper = to_upper("hello")

let first = list.get(0)
```

インデックスアクセスは `[]` ではなく、`get`/`set` などのメソッドを使います。

---

## 5.8 単項・二項演算式

```ebnf
UnaryOperatorExpr ::= ("-" | "!") Expression
BinaryOperatorExpr ::= Expression Operator Expression
```

```protorun
let a = -x
let b = !flag

let c = x + y * 2
let d = x > 0 && y > 0
```

演算子優先順位に従って評価され、必要に応じて `GroupedExpr` で明示します。

---

## 5.9 代入式 (`AssignmentExpr`)

代入は可変束縛に対して新しい値を設定する式です。

```ebnf
AssignmentExpr ::= Identifier "=" Expression
```

```protorun
let mut count = 0
count = count + 1
```

---

## 5.10 範囲式 (`RangeExpr`)

```ebnf
RangeExpr ::= Expression ".." Expression
            | Expression "..=" Expression
```

```protorun
let r1 = 1..10
let r2 = 1..=10
```

`a..b` は終端を含まず、`a..=b` は終端を含みます。

---

## 5.11 グループ化式 (`GroupedExpr`)

```ebnf
GroupedExpr ::= "(" Expression ")"
```

```protorun
let result = (2 + 3) * 4
let same = (5)
```

`(expr)` はグループ化です。タプルは `(a, b)` のようにカンマを含む形で記述します。

---

## 5.12 リスト内包表記 (`ListComprehension`)

```ebnf
ListComprehension ::= "[" Expression "for" Pattern "<-" Expression ("if" Expression)? "]"
```

```protorun
let doubled_evens = [x * 2 for x <- numbers if x % 2 == 0]
let pairs = [(x, y) for x <- xs if x > 0 for y <- ys if y > 0]
```

内包表記はフィルタと変換の組み合わせを簡潔に表現します。

---

## 5.13 レコード構築式 (`RecordExpr`)

`TypeName { field: value, ... }` の形式でレコード値を構築します。

```ebnf
RecordExpr ::= TypeRef "{" (RecordFieldInit ("," RecordFieldInit)*)? "}"
RecordFieldInit ::= Identifier ":" Expression
```

```protorun
let p = Point { x: 10.0, y: 20.0 }
let user = User { name: "Ada", age: 36 }
```

---

## 5.14 非採用/将来検討

- `bind` 式は現行 EBNF に含まれないため本章から削除しました。将来の再導入は別途仕様化します。
