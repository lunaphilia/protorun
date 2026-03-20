- 04章統合時は `LetDecl` を中心に据えると、`TraitItem` / `EffectItem` / `HandlerItem` / `ImplItem` の記法説明を一貫して整理できる。
- 関数定義例は `FunctionHeader ::= GenericParams? ParamList ("->" ReturnType)?` に合わせ、`fn` を完全除去して `(params) -> Ret = body` へ統一する。
- 章境界の維持には「本章は基本形のみ、詳細は別章参照」という書き方が有効で、重複説明と仕様衝突を防げる。
- 代数的効果章の再編では、`with` / `break` / `continue` の EBNF を本文に再掲すると、旧 05 章と旧 08 章由来のセマンティクスを一箇所へ統合しやすい。
- `EffectItem ::= LetDecl` と `HandlerItem ::= LetDecl` を本文で明示した上で、`let get: () -> S;` と `let get = () -> S = ...` を対で置くと、宣言と実装の対応が読み手に伝わりやすい。

## Task 10 (2026-03-20): 09-examples.md 新規作成

### 作業概要
旧 `11-examples.md`（430行）を `09-examples.md` にコピーし、EBNF準拠チェックと修正を実施。

### 適用した修正
1. **セクション番号**: `11.x` → `9.x`、タイトル `# 11.` → `# 9.`
2. **fn除去**: 20箇所以上の `fn(...)` を `(...)` に変換（ラムダ・named function共）
   - `fn[T](...)` → `[T](...)` （ジェネリクス付き）
   - `fn(x) = ...` → `(x) = ...` （無名関数）
3. **インデックスアクセス**: `scores[name]` → `scores.get(name)` （1箇所）
4. **章参照**: `[12. 文法](12-grammar.md)` → `[10. 文法](10-grammar.md)`
5. **説明文更新**: `fn キーワードを使用` → `(params) -> ReturnType = body 形式`、`map[key]` → `map.get(key)`

### 元ファイルで既に正しかった点
- ジェネリクス: 全て `[T]` 記法（`<T>` なし）
- Bool: `True`/`False` 大文字
- 文字列補間: `f"..."` 形式（`s"..."` なし）
- if/while/for: `then` キーワード付き
- マップリテラル: `{ key : value }` 形式（`->` なし）
- 戻り型: `-> TypeName` 形式（`: TypeName` なし）

### 検証結果（全パターンでマッチ0件）
- `\[name\]|\[key\]|\[0\]` → 0件
- `= fn(` → 0件
- `<[A-Z][a-z]*>` → 0件
- `): [A-Z]` → 0件
- ` true| false` → 0件
- `s"` → 0件
