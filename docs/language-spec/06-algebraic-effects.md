# 6. 代数的効果

## 6.1 概念と目的

代数的効果は、計算本体と副作用・制御フローを分離し、合成可能な形で扱うための仕組みです。Protorun では、効果インターフェース (`effect`)、ハンドラ (`handler`)、適用スコープ (`with`) を通じて、効果の発生点と処理点を明示します。

この章は主に**使い方とセマンティクス**を扱います。基本的な宣言形は [4. 宣言と文](04-declarations-and-statements.md) と [10. 文法](10-grammar.md) を参照してください。

## 6.2 効果インターフェースの定義 (EffectDefinitionExpr)

効果インターフェースは、操作シグネチャの集合です。`EffectDefinitionExpr` は `EffectItem ::= LetDecl` で構成され、操作は `let` 宣言で記述します。

```protorun
let Console = effect {
  let log: (message: String) -> Unit;
  let readLine: () -> String;
}

let State = [S] effect {
  let get: () -> S;
  let put: (S) -> Unit;
}
```

- `effect` 本体の各項目は `let operation: (params) -> ReturnType;` の形式を取ります。
- 効果定義は値に束縛され、関数側は `effect` パラメータを通じてそのインターフェースに依存します。
- ジェネリクスは常に `[T]` 形式です。

## 6.3 ハンドラ型の定義 (HandlerDefinitionExpr)

ハンドラは、効果インターフェースを特定の対象型に対して実装する式です。`HandlerDefinitionExpr` も `HandlerItem ::= LetDecl` で構成されます。

```protorun
let StateHandler = [S](initial: S) handler State[S] for Unit {
  let get = () -> S = initial;
  let put = (s: S) -> Unit = ();
}

let ConsoleHandler = handler Console for Unit {
  let log = (message: String) -> Unit = println(message);
  let readLine = () -> String = input();
}
```

- ハンドラ項目は `let name = (params) -> ReturnType = expr` の形で記述します。
- 本章の関数例は、すべて現行の関数式構文で統一します。

## 6.4 Effect パラメータ

関数は `effect` パラメータで必要な効果依存を明示します。これは型安全な依存宣言です。

```protorun
let greet = (name: String, effect console: Console) -> Unit = {
  console.log(f"Hello, {name}")
}

let tick = (effect state: State[Int], effect console: Console) -> Int = {
  let n = state.get()
  state.put(n + 1)
  console.log(f"count = {n + 1}")
  n + 1
}
```

## 6.5 効果ハンドリング (WithExpr)

`with` 式は、効果依存に対して具体的なハンドラ値を束縛する式です。

代表形は `with handler = HandlerExpr { ... }` です。

```ebnf
WithExpr ::= "with" WithBinding ("," WithBinding)* BlockExpr
WithBinding ::= Identifier "=" Expression (":" TypeRef)?
```

```protorun
let result = with state = StateHandler[Int](0), console = ConsoleHandler {
  tick()
}

let result2 = with state = StateHandler[Int](10): State[Int] {
  state.put(state.get() + 5)
  state.get()
}
```

- 束縛は `alias = expr` で記述し、必要なら `: TypeRef` で型を固定できます。
- 効果は `with` ブロックの静的スコープで有効になります。
- トップレベルで効果操作を行う場合も `with` による明示的提供が必要です。

## 6.6 効果操作の呼び出し

`alias.operation(args...)` は通常のメンバー呼び出し構文ですが、効果呼び出しとして評価されます。

評価の流れ:

1. `alias` と引数を評価する。
2. 対応する `with` 束縛から有効なハンドラ実装を選ぶ。
3. 呼び出し位置の継続を捕捉し、ハンドラ項目へ制御を移す。
4. ハンドラが継続を再開または破棄し、式全体の値を確定する。

このため、効果操作呼び出しは非局所的な制御フローを生み得ます。

## 6.7 継続制御

ハンドラは継続を次のいずれかとして扱えます。

- 暗黙再開: ハンドラ項目の本体評価後に継続へ値を渡して 1 回再開する。
- 明示再開: 継続を受け取り、必要なタイミングで再開する。
- 非再開: 継続を破棄し、`with` 式全体の結果を直接返す。

```protorun
let Abort = [E] effect {
  let raise: (E) -> Nothing;
}

let AbortToResult = [E, T] handler Abort[E] for Unit {
  let raise = (e: E) -> Result[T, E] = Result.Err(e);
}
```

非再開パスは、例外変換や早期脱出を表現するために使われます。

## 6.8 break/continue の代数的効果セマンティクス

`break` と `continue` は制御文であると同時に、代数的効果として意味づけられます。

```ebnf
BreakExpr ::= "break" Expression?
ContinueExpr ::= "continue"
```

- `while` / `for` / `loop` は暗黙的に `BreakEffect` と `ContinueEffect` のハンドラを提供します。
- `break expr` は現在反復の残り継続を破棄し、最内ループ式を `expr` で終了させます。
- `break` は `Unit` を返す `break ()` の省略形です。
- `continue` は現在反復の残り継続を破棄し、次の反復へ遷移します。

```protorun
let firstOdd = loop {
  let n = nextNumber()
  if n % 2 == 0 then {
    continue
  }
  break n
}
```

このモデルにより、ループ脱出と反復スキップが継続制御と整合的に扱われます。

## 6.9 ライフサイクル管理効果

リソース獲得・使用・解放は効果としてモデル化できます。`with` スコープの境界は、リソース寿命を局所化する設計点になります。

```protorun
let ManagedFile = effect {
  let open: (String) -> Result[File, IOError];
  let close: (File) -> Result[Unit, IOError];
}

let UseFile = (path: String, effect fs: ManagedFile) -> Result[String, IOError] = {
  let f = fs.open(path)
  // ...
}
```

最終的な自動解放戦略（`with` 終了時のフック等）は、実装段階で詳細化します。

## 6.10 依存性注入としてのEffectパラメータ

`effect` パラメータと `with` は、型付き DI の構文です。

- 呼び出し側は `with` で実装を選択する。
- 被呼び出し側は抽象インターフェース（効果）だけに依存する。
- 同一効果に対する差し替え（本番/テスト）を局所的に行える。

```protorun
let Clock = effect {
  let nowMillis: () -> Int;
}

let measure = (effect clock: Clock, work: () -> Unit) -> Int = {
  let t0 = clock.nowMillis()
  work()
  clock.nowMillis() - t0
}
```

## 6.11 設計上の考慮事項

1. 効果インターフェースは責務単位で分割し、操作粒度を揃える。
2. ハンドラ実装は `with` 境界で交換可能に設計し、隠れ依存を作らない。
3. 継続を破棄する操作（早期脱出）は戻り型を明示して読みやすくする。
4. ループ制御効果（`break`/`continue`）は他効果と混在するため、局所的なスコープを保つ。
5. 言語実装では継続捕捉コストを最適化し、効果呼び出しのオーバーヘッドを抑える。
