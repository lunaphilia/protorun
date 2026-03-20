# Learnings — protorun-mvp-spec

## 2026-03-20 Session start

### 確定済み設計決定 (17項目)
1. **スコープ**: MVP仕様完成（全ギャップ解消）
2. **ループ設計**: Effektスタイル — while/forは組み込み構文、break/continueは代数的効果
3. **文法正規化**: EBNFを正規仕様とするが、明白なバグ(#6戻り値型等)はEBNFを修正
4. **ジェネリクス**: Scala方式 — `[T]`を維持、インデックスはトレイトメソッド`list.get(0)`
5. **エントリポイント**: トップレベル式（スクリプト風、明示的mainなし）
6. **`?` 演算子**: MVP仕様から省く
7. **所有権システム**: ch7全体を将来フェーズに延期（GCベース言語に）
8. **文字列補間**: Python風 `f"...{expr}..."` (fプレフィックス)
9. **数値型**: サイズ付き完全サポート: i8〜i128, u8〜u128, f32/f64。Int=i64, Float=f64エイリアス
10. **メソッド/インデックス**: トレイトメソッド方式 — UFCS: `x.f(y)`はトレイト→フリー関数順
11. **標準ライブラリ**: 最小prelude — Option,Result,List,Map,Set,String,Iterator,Show,Eq,Ord,Console効果
12. **リストパターン/スプレッド**: サンプルから削除（文法には追加しない）
13. **演算子オーバーロード**: MVPは組み込み型のみ
14. **追加要望**: 特になし
15. **関数戻り値型**: `->` に統一（EBNFの`:` を修正）
16. **可変フィールド**: `mut` に統一（`let mutable`廃止）
17. **トップレベル効果**: 明示的with必須（暗黙ハンドラなし）

### EBNFと散文の11箇所矛盾
| # | 対象 | EBNF | 散文 | 解決方針 |
|---|------|------|------|---------|
| 1 | if構文 | `if cond then expr` | `if cond { block }` | EBNF準拠 |
| 2 | ブール | `True`/`False` | `true`/`false` | EBNF準拠 |
| 3 | ジェネリクス | `[T]` (GenericArgs) | `<T>` | EBNF準拠 |
| 4 | 関数本体 | `= Expression` | `=> Expression` | EBNF準拠 |
| 5 | マップリテラル | `key : value` | `key -> value` | EBNF準拠 |
| 6 | 戻り値型 | `(":" ReturnType)?` | `-> ReturnType` | 散文準拠(EBNF修正) |
| 7 | FunctionExpr名 | `FunctionDefinitionExpr` | `FunctionExpr` | EBNF準拠 |
| 8 | elif | 未定義 | 使用あり | EBNF追加 |
| 9 | FloatLiteral | 二重定義 | N/A | EBNF修正(統合) |
| 10 | module/import | 未定義 | ch9に記載 | EBNF追加 |
| 11 | セットリテラル | 未定義 | `#{...}` 使用 | EBNF追加 |

### 新規追加すべき文法規則
- `WhileExpr`, `ForExpr`, `LoopExpr`, `BreakExpr`, `ContinueExpr`
- `StringInterpolation` (`f"..."`)
- `RangeExpr` (`..`, `..=`)
- `SetLiteral` (`#{...}`)
- `ModuleDecl`, `ImportDecl`
- `FieldDefinition` に `"mut"?` を追加

### コードベース構造
- 言語: 仕様ドキュメントのみ（実装なし）
- 仕様ファイル: `docs/language-spec/*.md`
- パーサー実装予定: Nom (Rust)
- 実装方針: インタープリタから段階的

### EBNFの現在の問題点（確認済み）
- L83とL85に `FloatLiteral` が二重定義
- `FunctionHeader` L195: `(":" ReturnType)?` → `("->" ReturnType)?` に修正必要
- GenericArgs が `[...]` だが散文コードは全て `<T>` を使用
- `if then` 構文だが散文は `if { }` を使用
- `BoolLiteral`: `True`/`False`だが散文は`true`/`false`
- `MapEntry`: `key ":" value`だが散文は `"key" -> value`

- 2026-03-20: Final spec consistency pass fixed remaining ownership-like reference notation in `03-type-system.md`, removed lingering `?` operator usage in `07-ownership.md`, and added missing `WithExpr`/`WithBinding` grammar productions in `12-grammar.md` to complete Hello World traceability.
