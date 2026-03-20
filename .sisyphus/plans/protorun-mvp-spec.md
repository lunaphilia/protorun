# Protorun MVP仕様完成計画

## TL;DR

> **Quick Summary**: Protorun言語のMVP仕様を完成させる。EBNF文法と散文仕様の11箇所の矛盾を解消し、ループ構文・エントリポイント・数値型・文字列補間・イテレータプロトコル・最小標準ライブラリなど、実用言語として動作するために必要な全ギャップを埋める。所有権システムはMVPから除外。
>
> **Deliverables**:
> - 統一されたEBNF文法 (12-grammar.md)
> - 全散文仕様の文法整合性 (01〜09, 11章)
> - 新規仕様: ループ構文、エントリポイント、数値型、文字列補間、イテレータ、UFCS
> - 新規章: 10-standard-library.md (最小prelude)
> - 延期章の整理: 07-ownership.md に将来フェーズ明記
> - 全サンプルコード更新 (11-examples.md)
>
> **Estimated Effort**: Large
> **Parallel Execution**: YES - 4 waves
> **Critical Path**: Task 1 → Task 2 → Tasks 3-8 (parallel) → Tasks 9-13 (parallel) → Tasks 14-16 (parallel) → Final Verification

---

## Context

### Original Request
Protorunを実用的なプログラミング言語にするために不足している機能・文法を調査し、仕様を詰める。

### Interview Summary
**Key Discussions**:
- MVP仕様完成をスコープとする（全ギャップ解消）
- ループはEffektスタイル（break/continueを代数的効果として実装）
- EBNFを正規仕様とするが、明らかなEBNFバグ（戻り値型`:` → `->`等）は修正
- ジェネリクスは`[T]`、インデックスはトレイトメソッド（Scala方式）
- エントリポイントはトップレベル式（スクリプト風）、効果は明示的with必須
- 文字列補間はPython風 `f"...{expr}..."`
- 数値型はサイズ付き完全サポート（i8〜i128, u8〜u128, f32/f64）
- 所有権(ch7)・`?`演算子・リストパターン/スプレッドはMVPから除外
- 可変フィールドは`mut`に統一（`mutable`廃止）
- 演算子オーバーロードはMVP組み込みのみ

**Research Findings**:
- Oracle: 11箇所の文法矛盾、所有権×効果の設計リスク（延期で回避）
- Librarian: Koka/Effekt/OCaml5/Unison/Scala3調査 — Effektのbreak/continue効果を推奨

### Metis Review
**Identified Gaps** (addressed):
- 文法矛盾は当初の5箇所ではなく11箇所
- 関数戻り値型の`:` vs `->` — `->` に決定（EBNF修正）
- 可変フィールド`mutable`の所有権延期後の扱い — `mut`に統一
- トップレベル効果ハンドラ提供方法 — 明示的with必須

---

## Work Objectives

### Core Objective
Protorun言語仕様を、パーサー実装（Nom/Rust）が開始できるレベルの一貫性と完全性に引き上げる。

### Concrete Deliverables
- `docs/language-spec/12-grammar.md` — 統一・拡張されたEBNF文法
- `docs/language-spec/02-lexical-structure.md` — キーワード・リテラル更新
- `docs/language-spec/03-type-system.md` — 数値型・関数型更新
- `docs/language-spec/04-declarations.md` — 構文統一
- `docs/language-spec/05-statements.md` — ループ文追加
- `docs/language-spec/06-expressions.md` — 構文統一・新機能追加
- `docs/language-spec/07-ownership.md` — 将来フェーズ明記
- `docs/language-spec/08-algebraic-effects.md` — 構文統一・mutable修正
- `docs/language-spec/09-modules.md` — 構文統一
- `docs/language-spec/10-standard-library.md` — 新規作成（最小prelude）
- `docs/language-spec/11-examples.md` — 全面更新
- `docs/language-spec.md` — 目次・更新履歴更新

### Definition of Done
- [ ] EBNF文法と散文仕様に矛盾がゼロ
- [ ] 全サンプルコードがEBNF文法に準拠
- [ ] 言語のHello Worldプログラムが仕様通りに記述可能
- [ ] 削除・延期された機能(所有権・?演算子・リストパターン)の参照が全て整理済み

### Must Have
- 全11箇所の文法矛盾解消
- ループ構文（while/for/loop + break/continue効果）の完全仕様
- エントリポイント（トップレベル式）の仕様
- 数値型の完全仕様
- 文字列補間の仕様
- イテレータプロトコルの仕様
- 最小prelude標準ライブラリ仕様
- UFCS/メソッド解決規則の仕様
- クロージャキャプチャ意味論（GCベース）

### Must NOT Have (Guardrails)
- 所有権・借用・ライフタイムの仕様（ch7は将来明記のみ）
- `?` 演算子の仕様
- リストパターン・スプレッド演算子の仕様
- ユーザー定義演算子オーバーロード
- 並行処理・async仕様
- FFI仕様
- アノテーション/属性システム
- 新しい概念の発明（既存の設計哲学に沿った拡張のみ）

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO（仕様ドキュメントのみ）
- **Automated tests**: None（仕様文書の整合性チェック）
- **Framework**: N/A

### QA Policy
各タスクのQAは文書間の整合性検証。具体的にはGrep/Read toolで矛盾チェック。
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Foundation — EBNF is source of truth):
├── Task 1: EBNF文法の修正・拡張 (12-grammar.md) [deep]
├── Task 2: 設計方針文書の作成 (.sisyphus/drafts/design-decisions.md) [writing]

Wave 2 (Core specs — depend on EBNF, MAX PARALLEL):
├── Task 3: 字句構造の更新 (02-lexical-structure.md) [quick]
├── Task 4: 型システムの更新 (03-type-system.md) [unspecified-high]
├── Task 5: 宣言の更新 (04-declarations.md) [unspecified-high]
├── Task 6: 文の拡張 — ループ構文追加 (05-statements.md) [deep]
├── Task 7: 式の更新 (06-expressions.md) [unspecified-high]
├── Task 8: 代数的効果の更新 (08-algebraic-effects.md) [unspecified-high]

Wave 3 (Dependent specs + new chapters):
├── Task 9: 所有権章の延期整理 (07-ownership.md) [quick]
├── Task 10: モジュールの更新 (09-modules.md) [quick]
├── Task 11: 標準ライブラリ新規作成 (10-standard-library.md) [deep]
├── Task 12: サンプルプログラム全面更新 (11-examples.md) [unspecified-high]
├── Task 13: 序論の更新 (01-introduction.md) [quick]

Wave 4 (Integration + finalization):
├── Task 14: 目次・更新履歴更新 (language-spec.md) [quick]
├── Task 15: 全体整合性最終チェック [deep]

Critical Path: Task 1 → Task 6 → Task 11 → Task 12 → Task 15
Max Concurrent: 6 (Wave 2)
```

### Dependency Matrix
| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 | — | 3-8, 10-15 |
| 2 | — | 3-8 |
| 3 | 1 | 12, 15 |
| 4 | 1 | 11, 12, 15 |
| 5 | 1 | 12, 15 |
| 6 | 1 | 8, 11, 12, 15 |
| 7 | 1 | 12, 15 |
| 8 | 1, 6 | 11, 12, 15 |
| 9 | 1 | 12, 15 |
| 10 | 1 | 15 |
| 11 | 4, 6, 8 | 12, 15 |
| 12 | 3-11 | 15 |
| 13 | 1 | 15 |
| 14 | 12, 13 | 15 |
| 15 | ALL | — |

### Agent Dispatch Summary
- **Wave 1**: 2 tasks — T1 → `deep`, T2 → `writing`
- **Wave 2**: 6 tasks — T3 → `quick`, T4-T5 → `unspecified-high`, T6 → `deep`, T7-T8 → `unspecified-high`
- **Wave 3**: 5 tasks — T9-T10,T13 → `quick`, T11 → `deep`, T12 → `unspecified-high`
- **Wave 4**: 2 tasks — T14 → `quick`, T15 → `deep`
- **FINAL**: 4 parallel reviews → oracle, unspecified-high, unspecified-high, deep

---

## TODOs

- [x] 1. EBNF文法の修正・拡張 (12-grammar.md)

  **What to do**:
  - 11箇所の矛盾を解消:
    - #6: `FunctionHeader` の戻り値型を `(":" ReturnType)?` → `("->" ReturnType)?` に修正
    - #8: `IfExpr` に `elif` 対応を追加: `IfExpr ::= "if" Expression "then" Expression ("elif" Expression "then" Expression)* ("else" Expression)?`
    - #9: `FloatLiteral` の二重定義（L83, L85）を1つに統合
    - #10: `ModuleDecl`, `ImportDecl` をEBNFに追加（ch9の構文を形式化）
    - #11: `SetLiteral ::= "#{" (Expression ("," Expression)*)? "}"` を追加
  - 新しい構文規則を追加:
    - `WhileExpr ::= "while" Expression "then" Expression` (Effektスタイルループ)
    - `ForExpr ::= "for" Pattern "in" Expression "then" Expression` (イテレータベースループ)
    - `LoopExpr ::= "loop" Expression` (無限ループ)
    - `BreakExpr ::= "break" Expression?`
    - `ContinueExpr ::= "continue"`
    - `StringInterpolation ::= "f" "\"" (StringContent | "{" Expression "}")* "\""` (文字列補間)
    - `RangeExpr ::= Expression ".." Expression` / `Expression "..=" Expression`
  - Expression リストに新しい式を追加
  - 数値型サフィックス定義が既にあることを確認し、本文との紐付けを強化
  - 可変フィールドの構文: `FieldDefinition ::= "mut"? Identifier ":" Type`

  **Must NOT do**:
  - 所有権関連の構文（`own`, `&`, `&mut`, ライフタイム `'a`）を追加しない
  - `?` 演算子を追加しない
  - リストパターン構文を追加しない

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: EBNF文法の自己整合性が重要。複数の規則間の依存関係を理解して修正する必要がある
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 2)
  - **Parallel Group**: Wave 1
  - **Blocks**: Tasks 3-15
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `docs/language-spec/12-grammar.md` — 現在のEBNF文法全体。全ての規則がここに定義されている

  **API/Type References**:
  - `docs/language-spec/09-modules.md:20-37` — module/import/exportの散文構文。EBNFに形式化する元

  **External References**:
  - Effekt言語のwhile/for/loop/break/continue構文: https://effekt-lang.org/docs/

  **WHY Each Reference Matters**:
  - 12-grammar.md: 修正対象そのもの。全規則の依存関係を把握して無矛盾に修正する必要がある
  - 09-modules.md: module/importのEBNF規則がないため、散文から形式化する際の入力元

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: EBNF二重定義の解消
    Tool: Bash (grep)
    Steps:
      1. grep -c "^FloatLiteral" docs/language-spec/12-grammar.md
      2. Assert output is exactly "1"
    Expected Result: FloatLiteralの定義が1箇所のみ
    Evidence: .sisyphus/evidence/task-1-float-dedup.txt

  Scenario: 新規構文規則の存在確認
    Tool: Bash (grep)
    Steps:
      1. grep "WhileExpr" docs/language-spec/12-grammar.md
      2. grep "ForExpr" docs/language-spec/12-grammar.md
      3. grep "LoopExpr" docs/language-spec/12-grammar.md
      4. grep "StringInterpolation" docs/language-spec/12-grammar.md
      5. grep "SetLiteral" docs/language-spec/12-grammar.md
      6. grep "ModuleDecl\|ImportDecl" docs/language-spec/12-grammar.md
      7. Assert all greps return at least 1 match
    Expected Result: 全ての新規構文規則が定義されている
    Evidence: .sisyphus/evidence/task-1-new-rules.txt

  Scenario: 戻り値型が -> に統一されている
    Tool: Bash (grep)
    Steps:
      1. grep "FunctionHeader" docs/language-spec/12-grammar.md
      2. Assert output contains "->" and does NOT contain '(":" ReturnType)'
    Expected Result: FunctionHeaderの戻り値型が -> を使用
    Evidence: .sisyphus/evidence/task-1-return-arrow.txt

  Scenario: 禁止構文が含まれていない
    Tool: Bash (grep)
    Steps:
      1. grep -c "own\b" docs/language-spec/12-grammar.md (所有権キーワード)
      2. grep -c "'[a-z]" docs/language-spec/12-grammar.md (ライフタイム)
      3. Assert both outputs are "0"
    Expected Result: 所有権関連構文がEBNFに存在しない
    Evidence: .sisyphus/evidence/task-1-no-ownership.txt
  ```

  **Commit**: YES (group with Task 2)
  - Message: `docs: unify and extend EBNF grammar for MVP spec`
  - Files: `docs/language-spec/12-grammar.md`

- [x] 2. 設計方針文書の作成

  **What to do**:
  - `.sisyphus/drafts/design-decisions.md` に全17項目の設計決定を正式に記録
  - 各決定の根拠・代替案・トレードオフを簡潔に記載
  - 他のタスク実行者が参照できるよう、構文例を含める
  - MVPスコープ（IN/OUT）を明確にリスト化

  **Must NOT do**:
  - 実装に関する決定を含めない（これは仕様文書のみ）

  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: 文書作成タスク
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 1)
  - **Parallel Group**: Wave 1
  - **Blocks**: Tasks 3-8 (参照情報として)
  - **Blocked By**: None

  **References**:
  - `.sisyphus/drafts/protorun-gap-analysis.md` — インタビューで確定した全決定事項の記録

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: 全17決定事項の記録確認
    Tool: Bash (grep)
    Steps:
      1. grep -c "##" .sisyphus/drafts/design-decisions.md
      2. Assert at least 17 decision sections exist
    Expected Result: 17以上のセクションが存在
    Evidence: .sisyphus/evidence/task-2-decisions.txt
  ```

  **Commit**: YES (group with Task 1)
  - Message: `docs: record MVP design decisions`
  - Files: `.sisyphus/drafts/design-decisions.md`

- [x] 3. 字句構造の更新 (02-lexical-structure.md)

  **What to do**:
  - キーワードテーブルを更新: `while`, `for`, `loop`, `break`, `continue`, `in` を追加。`own` を削除
  - ブールリテラルが `True`/`False` であることを明記（EBNF準拠）
  - 数値型を正式追加: i8/i16/i32/i64/i128, u8/u16/u32/u64/u128, f32/f64。`Int` = i64、`Float` = f64のエイリアス
  - 整数・浮動小数点リテラルのサフィックス（i32, f64等）を本文で説明
  - 文字列補間リテラル `f"..."` を追加
  - 演算子テーブルに `..` (範囲)、`..=` (閉範囲) を正式追加
  - `->` の用途説明を更新（関数型定義 + 関数戻り値型）
  - `=` の用途に「関数本体の区切り」を追加
  - `then` キーワードを追加（if式で使用）
  - セットリテラル `#{...}` を追加

  **Must NOT do**:
  - `&`, `&mut`, `own`, `ref` を所有権コンテキストで説明しない（延期済み）
  - `?` 演算子を含めない

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4-8)
  - **Blocks**: Tasks 12, 15
  - **Blocked By**: Task 1

  **References**:
  - `docs/language-spec/02-lexical-structure.md` — 現在のキーワード・演算子・リテラル定義
  - `docs/language-spec/12-grammar.md` — Task 1で更新されたEBNF（正規仕様）

  **Acceptance Criteria**:
  ```
  Scenario: 新キーワードの存在確認
    Tool: Bash (grep)
    Steps:
      1. grep "while\|for\|loop\|break\|continue\|in\|then" docs/language-spec/02-lexical-structure.md のキーワードテーブル内
      2. Assert all keywords present
    Expected Result: 7つの新キーワードが全てテーブルに存在
    Evidence: .sisyphus/evidence/task-3-keywords.txt
  ```

  **Commit**: YES (group Wave 2)
  - Message: `docs: update lexical structure for MVP`
  - Files: `docs/language-spec/02-lexical-structure.md`

- [x] 4. 型システムの更新 (03-type-system.md)

  **What to do**:
  - 基本型セクション (3.2) を拡張: サイズ付き整数型(i8〜i128, u8〜u128)、浮動小数点型(f32, f64)を追加。`Int` = i64、`Float` = f64のエイリアスを明記
  - 関数型 (3.3) の表記を更新: `(T1, T2) -> R` 形式。`fn`キーワードは型表現に含まれないことを再確認
  - ジェネリクス表記を `[T]` に統一（散文の `<T>` を全て `[T]` に置換）
  - イテレータプロトコルの型を記述: `Iterator[T]` トレイト、`Iterable[T]` トレイト
  - 文字列補間型: `f"..."` は `String` 型に評価されることを記述
  - 範囲型: `Range[T]`, `RangeInclusive[T]` を追加
  - 所有権関連の記述を削除または将来フェーズとしてマーク（`own`, `&`, `&mut` の型コンテキスト）

  **Must NOT do**:
  - 所有権型（参照型 `&T`, `&mut T`）を現在仕様として記述しない

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2
  - **Blocks**: Tasks 11, 12, 15
  - **Blocked By**: Task 1

  **References**:
  - `docs/language-spec/03-type-system.md` — 現在の型システム定義
  - `docs/language-spec/12-grammar.md` — 更新されたEBNF

  **Acceptance Criteria**:
  ```
  Scenario: ジェネリクス表記の統一
    Tool: Bash (grep)
    Steps:
      1. grep -n "<T>" docs/language-spec/03-type-system.md でコードブロック内の <T> を検索
      2. Assert 0 matches in code blocks (全て [T] に置換済み)
    Expected Result: コードブロック内に <T> 表記が存在しない
    Evidence: .sisyphus/evidence/task-4-generics.txt
  ```

  **Commit**: YES (group Wave 2)
  - Message: `docs: update type system for MVP (sized numerics, [T] generics)`
  - Files: `docs/language-spec/03-type-system.md`

- [x] 5. 宣言の更新 (04-declarations.md)

  **What to do**:
  - 全コード例をEBNF準拠に更新: `=>` → `=`、`<T>` → `[T]`、`-> RetType` は維持(EBNF修正済み)
  - 可変フィールド構文を `let mutable` → `mut` に変更: `type Counter { mut count: Int }` のように
  - 関数定義例を更新: `let add = fn(a: Int, b: Int) -> Int = a + b`
  - トレイト定義・実装のコード例をEBNF準拠に更新
  - 効果・ハンドラ定義のコード例をEBNF準拠に更新
  - 所有権関連の記述（借用パラメータ等）を削除または将来マーク

  **Must NOT do**:
  - 新しい宣言形式を発明しない（EBNFに定義されたもののみ）

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2
  - **Blocks**: Tasks 12, 15
  - **Blocked By**: Task 1

  **References**:
  - `docs/language-spec/04-declarations.md` — 現在の宣言仕様（428行）
  - `docs/language-spec/12-grammar.md` — 更新されたEBNF

  **Acceptance Criteria**:
  ```
  Scenario: mutable キーワード廃止確認
    Tool: Bash (grep)
    Steps:
      1. grep -c "let mutable" docs/language-spec/04-declarations.md
      2. Assert output is "0"
    Expected Result: "let mutable" が存在しない
    Evidence: .sisyphus/evidence/task-5-no-mutable.txt
  ```

  **Commit**: YES (group Wave 2)
  - Message: `docs: update declarations for MVP grammar`
  - Files: `docs/language-spec/04-declarations.md`

- [x] 6. 文の拡張 — ループ構文追加 (05-statements.md)

  **What to do**:
  - ループ構文を新セクションとして追加:
    - `while` 式: `while condition then body` — 条件が True の間 body を繰り返す。式として値を返す（最後の反復の値、または Unit）
    - `for` 式: `for pattern in iterable then body` — Iterable トレイトを実装する値を反復。sugar: `iterable.iter().forEach(fn(pattern) = body)`
    - `loop` 式: `loop body` — 無限ループ。break で脱出
  - break/continue を代数的効果として定義:
    - `break` と `continue` は予約キーワードだが、意味論的にはループ構文が提供する効果ハンドラによって処理される
    - `break expr` — ループから脱出し、expr をループ式全体の値とする
    - `continue` — 現在の反復をスキップし、次の反復に進む
  - break/continue の効果としての意味論を説明: while/for/loop は暗黙的にbreak/continue効果のハンドラを提供
  - return文の既存仕様はそのまま維持
  - 評価順序とスコープ規則を明記

  **Must NOT do**:
  - ループ以外の新しい文を追加しない
  - break/continue を通常のキーワード文として定義しない（効果ベースであることを維持）

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Effektスタイルのbreak/continue効果の意味論を正確に定義する必要がある
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2
  - **Blocks**: Tasks 8, 11, 12, 15
  - **Blocked By**: Task 1

  **References**:
  - `docs/language-spec/05-statements.md` — 現在の文仕様（returnのみ）
  - `docs/language-spec/12-grammar.md` — WhileExpr/ForExpr/LoopExpr の文法定義
  - Effekt言語ドキュメント — break/continue as effects パターン

  **Acceptance Criteria**:
  ```
  Scenario: ループ構文の仕様存在確認
    Tool: Bash (grep)
    Steps:
      1. grep -c "while" docs/language-spec/05-statements.md
      2. grep -c "for.*in" docs/language-spec/05-statements.md
      3. grep -c "loop" docs/language-spec/05-statements.md
      4. grep -c "break" docs/language-spec/05-statements.md
      5. grep -c "continue" docs/language-spec/05-statements.md
      6. Assert all counts > 0
    Expected Result: while/for/loop/break/continue の全てが仕様に記載
    Evidence: .sisyphus/evidence/task-6-loop-spec.txt

  Scenario: break/continueの効果ベース意味論の記述
    Tool: Bash (grep)
    Steps:
      1. grep -i "effect\|効果\|ハンドラ" docs/language-spec/05-statements.md で効果関連の記述を検索
      2. Assert matches exist near break/continue sections
    Expected Result: break/continueが代数的効果として説明されている
    Evidence: .sisyphus/evidence/task-6-break-effects.txt
  ```

  **Commit**: YES (group Wave 2)
  - Message: `docs: add loop constructs with break/continue as effects`
  - Files: `docs/language-spec/05-statements.md`

- [x] 7. 式の更新 (06-expressions.md)

  **What to do**:
  - 全コード例をEBNF準拠に更新: `=>` → `=`、`<T>` → `[T]`、`true`/`false` → `True`/`False`、`if cond {` → `if cond then`
  - 関数式の構文更新: `FunctionExpr ::= FunctionHeader "=" Expression`。戻り値型は `->`
  - 文字列補間式セクション追加: `f"Hello {name}, age {age + 1}"`。Show トレイトの to_string メソッドにデシュガーされる
  - 範囲式セクション追加: `1..10`, `1..=10`。Range/RangeInclusive 型を返す
  - with式の構文をEBNF準拠に更新
  - bind式の構文をEBNF準拠に更新
  - 内包表記の構文をEBNF準拠に更新
  - 部分適用式の構文をEBNF準拠に更新
  - メソッド呼び出しとUFCS規則の新セクション追加:
    - `x.f(y)` の解決順序: 1) 型の固有メソッド 2) スコープ内トレイトメソッド
    - 曖昧時の明示指定: `Type.f(x, y)` / `Trait.f(x, y)`
  - インデックスアクセスの説明: `list.get(0)` / `list.set(0, val)` （`[]`はジェネリクス専用）
  - 所有権関連の式（`&expr`, `&mut expr`, `*ref`）を削除または将来マーク
  - `?` 演算子の使用例を削除

  **Must NOT do**:
  - リストパターン/スプレッド演算子を追加しない

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2
  - **Blocks**: Tasks 12, 15
  - **Blocked By**: Task 1

  **References**:
  - `docs/language-spec/06-expressions.md` — 現在の式仕様（760行、最大のファイル）
  - `docs/language-spec/12-grammar.md` — 更新されたEBNF

  **Acceptance Criteria**:
  ```
  Scenario: ブールリテラルの統一確認
    Tool: Bash (grep)
    Steps:
      1. grep -Pn '\btrue\b|\bfalse\b' docs/language-spec/06-expressions.md
      2. Expected output: 0 lines (no lowercase booleans)
      3. If any output, each is a FAIL
    Expected Result: 0 matches — 全て True/False に統一済み
    Failure Indicators: grep が1行以上出力した場合は未修正箇所あり
    Evidence: .sisyphus/evidence/task-7-bool-check.txt

  Scenario: 関数本体区切りの統一確認（matchアーム除外）
    Tool: Bash (grep)
    Steps:
      1. grep -n "=> " docs/language-spec/06-expressions.md | grep -v "match\|MatchArm\|pattern.*=>\|Pattern.*=>" で match アーム以外の => を検索
      2. Expected output: 0 lines
    Expected Result: match アーム以外に => が存在しない（全て = に統一）
    Failure Indicators: match コンテキスト外で => が残っている
    Evidence: .sisyphus/evidence/task-7-arrow-check.txt

  Scenario: ?演算子の排除確認
    Tool: Bash (grep)
    Steps:
      1. grep -Pn '\?\s*$|\?\s*;|\)\?' docs/language-spec/06-expressions.md
      2. Expected output: 0 lines
    Expected Result: ? 演算子の使用がゼロ
    Evidence: .sisyphus/evidence/task-7-no-question.txt
  ```

  **Commit**: YES (group Wave 2)
  - Message: `docs: update expressions for MVP grammar`
  - Files: `docs/language-spec/06-expressions.md`

- [x] 8. 代数的効果の更新 (08-algebraic-effects.md)

  **What to do**:
  - 全コード例をEBNF準拠に更新（ジェネリクス`[T]`、関数`=`、ブール`True/False`等）
  - `let mutable count: Int` → `mut count: Int` に全て変更
  - 所有権関連の効果例（`own FileHandle`, `&FileHandle`, `&mut FileHandle`）を簡略化: 所有権修飾子を削除し、通常の値渡しに変更
  - break/continue効果との関係を追加: ループ構文がハンドラを暗黙提供する旨の相互参照
  - `?` 演算子を使った例を、明示的な match 式に書き換え
  - with式の構文をEBNF準拠に更新
  - トップレベルでの効果使用に関する注意を追加: 明示的withが必要

  **Must NOT do**:
  - 所有権関連の効果パターンを新たに定義しない
  - 並行処理効果を追加しない

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2
  - **Blocks**: Tasks 11, 12, 15
  - **Blocked By**: Tasks 1, 6

  **References**:
  - `docs/language-spec/08-algebraic-effects.md` — 現在の代数的効果仕様（441行）
  - `docs/language-spec/05-statements.md` — Task 6で追加されるループ/break/continue効果定義

  **Acceptance Criteria**:
  ```
  Scenario: mutable/所有権構文の排除
    Tool: Bash (grep)
    Steps:
      1. grep -c "let mutable" docs/language-spec/08-algebraic-effects.md
      2. grep -c "own FileHandle\|&FileHandle\|&mut FileHandle" docs/language-spec/08-algebraic-effects.md
      3. Assert both are "0"
    Expected Result: 旧可変構文と所有権構文が排除されている
    Evidence: .sisyphus/evidence/task-8-no-ownership.txt
  ```

  **Commit**: YES (group Wave 2)
  - Message: `docs: update algebraic effects for MVP (remove ownership)`
  - Files: `docs/language-spec/08-algebraic-effects.md`

- [x] 9. 所有権章の延期整理 (07-ownership.md)

  **What to do**:
  - ファイル冒頭に明確な延期通知を追加: 「この章の内容は将来フェーズの設計案であり、MVP仕様には含まれません」
  - 既存内容は設計参考として保持するが、現在の仕様として扱わないよう明記
  - 他章からの参照を整理（各章で所有権を参照している箇所のリスト）

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: Tasks 12, 15
  - **Blocked By**: Task 1

  **References**:
  - `docs/language-spec/07-ownership.md` — 現在の所有権仕様

  **Acceptance Criteria**:
  ```
  Scenario: 延期通知の存在
    Tool: Bash (grep)
    Steps:
      1. head -10 docs/language-spec/07-ownership.md を確認
      2. Assert "将来フェーズ" or "MVP仕様には含まれません" が存在
    Expected Result: 延期通知がファイル冒頭に存在
    Evidence: .sisyphus/evidence/task-9-deferred.txt
  ```

  **Commit**: YES (group Wave 3)
  - Message: `docs: mark ownership chapter as deferred`
  - Files: `docs/language-spec/07-ownership.md`

- [x] 10. モジュールの更新 (09-modules.md)

  **What to do**:
  - 全コード例をEBNF準拠に更新（ジェネリクス`[T]`、関数`=`等）
  - トップレベル式がエントリポイントであることの説明を追加
  - モジュールとファイルシステムの関係を明確化
  - エントリポイントファイル（main.pr等）の扱いを定義

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: Task 15
  - **Blocked By**: Task 1

  **References**:
  - `docs/language-spec/09-modules.md` — 現在のモジュール仕様

  **Acceptance Criteria**:
  ```
  Scenario: エントリポイントの記述存在
    Tool: Bash (grep)
    Steps:
      1. grep -i "エントリポイント\|entry point\|トップレベル" docs/language-spec/09-modules.md
      2. Assert at least 1 match
    Expected Result: エントリポイント規則が記述されている
    Evidence: .sisyphus/evidence/task-10-entrypoint.txt
  ```

  **Commit**: YES (group Wave 3)
  - Message: `docs: update modules with entry point rules`
  - Files: `docs/language-spec/09-modules.md`

- [x] 11. 標準ライブラリ新規作成 (10-standard-library.md)

  **What to do**:
  - ファイルを新規作成。以下のprelude仕様を含める:
  - **コアデータ型**: Option[T], Result[T, E], List[T], Map[K, V], Set[T], String, Range[T], RangeInclusive[T]
  - **コアトレイト**:
    - `Show` — 文字列表現 (`let show: (self) -> String`)
    - `Eq` — 等価比較 (`let equals: (self, other: Self) -> Bool`)
    - `Ord: Eq` — 順序比較 (`let compare: (self, other: Self) -> Int`)
    - `Iterator[T]` — イテレータプロトコル (`let next: (mut self) -> Option[T]`)
    - `Iterable[T]` — イテレータ変換 (`let iter: (self) -> Iterator[T]`)
    - `Hash` — ハッシュ計算
  - **数値トレイト**: `Num`, `Add`, `Sub`, `Mul`, `Div` (組み込みのみ、MVP)
  - **コア効果**:
    - `Console` 効果 (`let println: (message: String) -> Unit`, `let readLine: () -> String`)
  - **基本関数**: `assert`, `panic`
  - 各型・トレイトはシグネチャのみ定義（実装はインタープリタ組み込み）
  - 言語構文がどのprelude要素にデシュガーされるかを明記:
    - `for x in list then ...` → `list.iter()` + `Iterator.next()`
    - `f"hello {x}"` → `String.concat(["hello ", x.show()])`
    - `1..10` → `Range { start: 1, end: 10 }`
    - `==` → `Eq.equals()`
    - `<`, `>`, `<=`, `>=` → `Ord.compare()`

  **Must NOT do**:
  - I/O、ファイルシステム、ネットワーク等の高レベルライブラリを含めない
  - 並行処理プリミティブを含めない

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: デシュガー規則と型定義の整合性が重要
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: Tasks 12, 15
  - **Blocked By**: Tasks 4, 6, 8

  **References**:
  - `docs/language-spec/03-type-system.md` — Task 4で更新された型定義
  - `docs/language-spec/05-statements.md` — Task 6で追加されたループ定義
  - `docs/language-spec/08-algebraic-effects.md` — Task 8で更新された効果定義
  - `docs/language-spec/12-grammar.md` — 更新されたEBNF

  **Acceptance Criteria**:
  ```
  Scenario: ファイル存在とコア要素
    Tool: Bash (grep)
    Steps:
      1. test -f docs/language-spec/10-standard-library.md
      2. grep "Option" docs/language-spec/10-standard-library.md
      3. grep "Iterator" docs/language-spec/10-standard-library.md
      4. grep "Console" docs/language-spec/10-standard-library.md
      5. grep "デシュガー\|desugar" docs/language-spec/10-standard-library.md
      6. Assert all pass
    Expected Result: ファイルが存在し、コア要素とデシュガー規則を含む
    Evidence: .sisyphus/evidence/task-11-stdlib.txt
  ```

  **Commit**: YES (group Wave 3)
  - Message: `docs: add minimal standard library specification`
  - Files: `docs/language-spec/10-standard-library.md`

- [x] 12. サンプルプログラム全面更新 (11-examples.md)

  **What to do**:
  - 全サンプルをEBNF準拠に全面書き換え:
    - ジェネリクス: `<T>` → `[T]`
    - 関数本体: `=> expr` → `= expr`
    - if式: `if cond { block }` → `if cond then block`
    - ブール: `true`/`false` → `True`/`False`
    - マップ: `"key" -> value` → `"key" : value`
  - `?` 演算子を使ったコードを match 式に書き換え
  - `[head, ...tail]` リストパターンを使ったコードをコンストラクタパターンに書き換え
  - 所有権関連のコード例を削除
  - 新しいサンプルを追加:
    - Hello World（トップレベル式 + with Console）
    - ループ使用例（while/for/loop + break）
    - 文字列補間使用例
    - イテレータ使用例
  - 既存サンプルの説明文も新仕様に合わせて更新

  **Must NOT do**:
  - `?` 演算子、リストパターン、所有権構文を使わない
  - 散文仕様と矛盾するコード例を残さない

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: Task 15
  - **Blocked By**: Tasks 3-11

  **References**:
  - `docs/language-spec/11-examples.md` — 現在のサンプル（359行）
  - `docs/language-spec/12-grammar.md` — 更新されたEBNF
  - `docs/language-spec/10-standard-library.md` — Task 11で作成されるprelude

  **Acceptance Criteria**:
  ```
  Scenario: 禁止構文の完全排除
    Tool: Bash (grep)
    Steps:
      1. grep -n "<T>\|<U>\|<A>\|<B>\|<S>\|<E>\|<R>" docs/language-spec/11-examples.md のコードブロック内
      2. grep -n "=> " docs/language-spec/11-examples.md のコードブロック内（matchアーム除く）
      3. grep -n "\btrue\b\|\bfalse\b" docs/language-spec/11-examples.md
      4. grep -n "\.\.\." docs/language-spec/11-examples.md (スプレッド)
      5. Assert all counts are 0 (or only in match => context)
    Expected Result: 旧構文が全て排除されている
    Evidence: .sisyphus/evidence/task-12-clean-examples.txt

  Scenario: Hello Worldサンプルの存在
    Tool: Bash (grep)
    Steps:
      1. grep -i "Hello World\|hello world" docs/language-spec/11-examples.md
      2. Assert at least 1 match
    Expected Result: Hello Worldサンプルが含まれている
    Evidence: .sisyphus/evidence/task-12-hello-world.txt
  ```

  **Commit**: YES (group Wave 3)
  - Message: `docs: rewrite all examples for MVP grammar`
  - Files: `docs/language-spec/11-examples.md`

- [x] 13. 序論の更新 (01-introduction.md)

  **What to do**:
  - MVPの位置づけを反映: 所有権は将来フェーズであることを明記
  - コア機能の優先順位 (1.3) を更新: 代数的効果 → 強力な型システム → 代数的データ型。所有権モデルは「将来フェーズ」に移動
  - 言語の進化と将来計画 (1.4) を更新: 所有権システムの導入を将来計画に追加

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: Task 15
  - **Blocked By**: Task 1

  **References**:
  - `docs/language-spec/01-introduction.md` — 現在の序論

  **Acceptance Criteria**:
  ```
  Scenario: 所有権の将来フェーズ明記
    Tool: Bash (grep)
    Steps:
      1. grep -i "将来\|future\|フェーズ" docs/language-spec/01-introduction.md
      2. Assert matches near "所有権" or "ownership"
    Expected Result: 所有権が将来フェーズとして記載されている
    Evidence: .sisyphus/evidence/task-13-intro.txt
  ```

  **Commit**: YES (group Wave 3)
  - Message: `docs: update introduction for MVP scope`
  - Files: `docs/language-spec/01-introduction.md`

- [x] 14. 目次・更新履歴更新 (language-spec.md)

  **What to do**:
  - 更新履歴に今回のMVP仕様完成の記録を追加（日付、変更内容のサマリー）
  - 目次の第7章に「(将来フェーズ)」を追記
  - 第10章のリンクが正しいことを確認

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4
  - **Blocks**: Task 15
  - **Blocked By**: Tasks 12, 13

  **References**:
  - `docs/language-spec.md` — 目次ファイル

  **Acceptance Criteria**:
  ```
  Scenario: 更新履歴の存在
    Tool: Bash (grep)
    Steps:
      1. grep "2026\|MVP" docs/language-spec.md
      2. Assert at least 1 match
    Expected Result: 今回の更新が更新履歴に記載されている
    Evidence: .sisyphus/evidence/task-14-changelog.txt
  ```

  **Commit**: YES (group Wave 4)
  - Message: `docs: update TOC and changelog`
  - Files: `docs/language-spec.md`

- [x] 15. 全体整合性最終チェック

  **What to do**:
  - 全ファイル間の相互参照が有効であることを検証
  - EBNFの全規則に対応する散文仕様が存在することを確認
  - 散文仕様の全コードブロックがEBNFに準拠していることを確認
  - 削除機能（所有権、?演算子、リストパターン）の残存参照がゼロであることを確認
  - Hello World、フィボナッチ、効果ハンドラの3つのシナリオを仕様に沿ってトレース

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO (全タスク完了後)
  - **Parallel Group**: Wave 4 (after Task 14)
  - **Blocks**: Final Verification
  - **Blocked By**: ALL tasks

  **References**:
  - 全ファイル: `docs/language-spec/*.md`

  **Acceptance Criteria**:
  ```
  Scenario: 所有権構文の残存チェック（ch7除外）
    Tool: Bash (grep)
    Steps:
      1. grep -rn --include="*.md" --exclude="07-ownership.md" '\bown \|&mut \|&[A-Z]' docs/language-spec/
      2. Expected output: 0 lines
      3. If any output, each line is an orphan reference to ownership
    Expected Result: 0 matches — ch7以外に所有権構文が存在しない
    Failure Indicators: 1行以上の出力がある場合、未修正の所有権参照
    Evidence: .sisyphus/evidence/task-15-no-ownership-orphans.txt

  Scenario: ?演算子の残存チェック（全ファイル）
    Tool: Bash (grep)
    Steps:
      1. grep -rPn --include="*.md" --exclude="07-ownership.md" '\)\?|\?\s' docs/language-spec/
      2. Expected output: 0 lines (通常の疑問符テキストとは区別するため、括弧直後の?のみ検索)
    Expected Result: 0 matches — 演算子としての ? が存在しない
    Failure Indicators: `)?` パターンが見つかった場合は未修正の?演算子
    Evidence: .sisyphus/evidence/task-15-no-question-op.txt

  Scenario: リストパターンの残存チェック
    Tool: Bash (grep)
    Steps:
      1. grep -rPn --include="*.md" '\[head.*\.\.\.?tail\]|\[\.\.\.|\, \.\.\.|\.\.\.[a-z]' docs/language-spec/
      2. Expected output: 0 lines
    Expected Result: 0 matches — スプレッド/リストパターンが存在しない
    Failure Indicators: `...` を含むリストパターンが見つかった場合
    Evidence: .sisyphus/evidence/task-15-no-list-patterns.txt

  Scenario: Hello Worldトレース
    Tool: Read
    Steps:
      1. Read docs/language-spec/09-modules.md — エントリポイント規則を確認
      2. Read docs/language-spec/10-standard-library.md — Console効果の定義を確認
      3. Read docs/language-spec/08-algebraic-effects.md — with構文の仕様を確認
      4. Read docs/language-spec/12-grammar.md — WithExpr, CallExpr の文法規則を確認
      5. Trace: ファイルトップレベル → with console = ConsoleHandler {} then { console.println(f"Hello World") } → Console.println 解決 → 出力
      6. Assert: 各ステップに対応する仕様記述が存在し、未定義のステップがゼロ
    Expected Result: Hello World が仕様内で完全にトレース可能。全ステップに仕様の裏付けがある
    Failure Indicators: いずれかのステップで「この構文/意味論がどの仕様にも記述されていない」が発生
    Evidence: .sisyphus/evidence/task-15-hello-trace.md
  ```

  **Commit**: YES
  - Message: `docs: final consistency verification for MVP spec`
  - Files: (corrections found during check)

---

## Final Verification Wave

> 4 review agents run in PARALLEL. ALL must APPROVE.

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan. For each "Must Have": verify corresponding section exists in updated spec files. For each "Must NOT Have": search for forbidden content. Check evidence files. Compare deliverables.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | VERDICT`

- [x] F2. **Grammar Consistency Review** — `unspecified-high`
  For every EBNF rule in 12-grammar.md: grep all other spec files for corresponding syntax. Flag any prose that uses syntax contradicting EBNF. Check that every keyword in EBNF keyword list appears in 02-lexical-structure.md. Verify no orphan references to deleted features (ownership, `?`, list patterns).
  Output: `Rules checked [N] | Contradictions [N] | Orphans [N] | VERDICT`

- [x] F3. **Sample Code Validation** — `unspecified-high`
  Read every code block in 11-examples.md. Manually parse each against EBNF rules. Flag any construct not defined in grammar. Verify no `?` operator, no `[head, ...tail]`, no `<T>` generics, no `true`/`false`, no `=> expr`.
  Output: `Samples [N/N valid] | Invalid constructs [list] | VERDICT`

- [x] F4. **Completeness Check** — `deep`
  Can a Hello World be written? Can a fibonacci be written? Can an effect handler example work? Trace through spec to verify: entry point → top-level expression → with block → effect operation → handler resolution. Flag any undefined step.
  Output: `Scenarios [N/N traceable] | Gaps [list] | VERDICT`

---

## Commit Strategy

- **Wave 1**: `docs: unify EBNF grammar and establish design decisions`
- **Wave 2**: `docs: update core spec chapters for grammar consistency`
- **Wave 3**: `docs: add stdlib, update examples, defer ownership`
- **Wave 4**: `docs: finalize MVP spec with consistency verification`

---

## Success Criteria

### Verification Commands
```bash
# EBNF文法内の矛盾チェック（二重定義等）
grep -c "FloatLiteral" docs/language-spec/12-grammar.md  # Expected: 1 definition

# 削除機能の残存チェック
grep -r "?演算子\|\.?\b" docs/language-spec/  # Expected: 0 (or only in deferred notes)
grep -r "\[head.*\.\.\.tail\]" docs/language-spec/  # Expected: 0

# ジェネリクス表記統一チェック
grep -rn "<T>" docs/language-spec/*.md  # Expected: 0 in code blocks (all should be [T])
```

### Final Checklist
- [ ] EBNF文法が自己矛盾なし（二重定義・未定義参照ゼロ）
- [ ] 全散文仕様のコード例がEBNF準拠
- [ ] 11箇所の矛盾が全て解消
- [ ] ループ構文の完全仕様あり
- [ ] 第10章（標準ライブラリ）が存在
- [ ] 第7章（所有権）に将来フェーズ明記
- [ ] サンプルプログラムが新仕様で一貫
