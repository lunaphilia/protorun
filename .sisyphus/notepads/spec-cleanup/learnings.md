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

## Task 13: Old chapter file reference updates (2026-03-20)

### Files modified:
1. `docs/language-spec/10-grammar.md`
   - Section headers updated: `## 12.x` / `### 12.x.y` → `## 10.x` / `### 10.x.y` (all 11 headings)
   - Line 269: `[9. モジュール](09-modules.md)` → `[7. モジュール](07-modules.md)`
   - Line 327: `[8. 代数的効果](08-algebraic-effects.md)` → `[6. 代数的効果](06-algebraic-effects.md)`
   - Line 328: Removed file link `[7. 所有権](07-ownership.md)`, kept note as "将来フェーズで導入予定（現在はMVPスコープ外）"
   - Line 329: `[9. モジュール](09-modules.md)` → `[7. モジュール](07-modules.md)`

2. `docs/language-spec/06-algebraic-effects.md`
   - Line 7: `[4. 宣言](04-declarations.md)` → `[4. 宣言と文](04-declarations-and-statements.md)`
   - Line 7: `[12. 文法](12-grammar.md)` → `[10. 文法](10-grammar.md)`

### Notes:
- AGENTS.md was already updated to new file names before this task ran.
- 09-examples.md already correctly referenced `[10. 文法](10-grammar.md)`.
- Section number patterns `12.x` in 10-grammar.md were internal numbering from old chapter 12 system.

## Task 14: F3 Manual QA 指摘のEBNF整合修正 (2026-03-20)

### 修正方針
- EBNF定義本体は変更せず、散文・例・章内参照のみを EBNF に一致させる。
- `TraitItem` / `EffectItem` / `HandlerItem` / `ImplItem` が `LetDecl` に従うため、コード例の終端セミコロンを除去する。
- `MatchArmList ::= MatchArm (NEWLINE MatchArm)*` に合わせ、`match` アームはカンマ区切りではなく改行区切りへ統一する。

### 今回の具体修正
1. `docs/language-spec.md` の Chapter 10 サブセクション番号を `12.x` から `10.x` へ修正。
2. `docs/language-spec/10-grammar.md` の本文参照 `EBNF（12.2）` を `EBNF（10.2）` へ修正。
3. `docs/language-spec/02-lexical-structure.md` で octal/raw string 非対応注記を対応済み表現へ更新し、`protorun` コード例のセミコロンを除去。
4. `docs/language-spec/04-declarations-and-statements.md` で `ImplDecl` の `("for" TypeRef)?` を反映し、`match` アームのカンマおよび `let` 終端セミコロンを除去。
5. `docs/language-spec/03-type-system.md` の壊れたアンカーリンクを章リンク + テキスト参照に変更。
6. `docs/language-spec/06-algebraic-effects.md` / `docs/language-spec/07-modules.md` の `let` / `import` 例から終端セミコロンを除去。
7. `docs/language-spec/09-examples.md` で型なしラムダに型注釈を付与し、`match` アームのカンマ除去、record位置パターンを recordフィールドパターンへ修正。

### 検証メモ
- `docs/language-spec.md` の `12.1|12.2|12.3|12.4` は 0 件。
- `docs/language-spec/10-grammar.md` の `12.2` は 0 件。
- Markdown 向け LSP が未設定のため `lsp_diagnostics` は `.md` で実行不可（No LSP server configured for extension: .md）。
