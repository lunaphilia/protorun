# Protorun 仕様ドキュメント包括整理

## TL;DR

> **Quick Summary**: Protorun言語仕様の全12章を10章に再編成し、EBNFとの不整合を修正、不要ファイルを削除、未完了の設計変更（fn除去・TraitItem統一）を完了する包括的ドキュメント整理。
> 
> **Deliverables**:
> - 再編成された10章の仕様ドキュメント（新ファイル名・番号体系）
> - EBNF準拠に修正された全コード例・散文
> - 更新された目次（language-spec.md）とAGENTS.md
> - 不要ファイルの完全削除（design/, taskdef/, 旧ownership章, 更新履歴）
> 
> **Estimated Effort**: Large
> **Parallel Execution**: YES - 4 waves
> **Critical Path**: T2(EBNF更新) → T3-T10(章再編成) → T11-T14(統合) → F1-F4(検証)

---

## Context

### Original Request
仕様ドキュメントを整理したい。内容の矛盾・不整合の解消、構成・章立ての見直し、不要ファイルの整理、記述の品質向上を包括的に実施。

### Interview Summary
**Key Discussions**:
- 更新履歴（language-spec.md内）: 不要 → 削除
- taskdef/, docs/design/: 不要 → 削除（git履歴に残る）
- 07-ownership.md: MVP不要 → 削除して番号詰め
- 品質向上の優先順位: EBNFとの整合性が最優先
- 未完了設計変更（fn除去、TraitItem統一）: 今回のスコープに含める
- bind/resume: EBNFが正。散文側を修正
- 章再構成: 提案B（しっかり再編成）を採用

**Research Findings**:
- 06-expressions.md が875行で最大 — 04/08と大量重複
- 05-statements.md がreturnとループを混在
- TOCが10章の実態と乖離（I/O・並行処理→実際はprelude）
- 約50箇所のクロスリファレンスがリナンバリングで壊れる
- 02-lexical-structure.md のキーワードテーブルが最大のリスク（~30参照）
- EBNF不整合: 09のreturn type、06のmap記法、11のindex記法、12の散文

### Metis Review
**Identified Gaps** (addressed):
- クロスリファレンスの網羅的更新（~50箇所）→ 専用タスク(T13)で対応
- 02-lexical-structure.md キーワードテーブルの隠れた依存 → 専用タスク(T6)で対応
- bind/resume のEBNF不在 → EBNF準拠原則で散文側を修正
- export のEBNF未定義 → 今回スコープ外（機能追加に該当）
- 章間のコンテンツ移動でのロスト防止 → 各タスクに明示的な移動元・移動先を指定

---

## Work Objectives

### Core Objective
Protorun言語仕様を12章→10章に再編成し、EBNFを唯一の正とする整合性を全章で確立する。

### Concrete Deliverables
- `docs/language-spec/01-introduction.md` — そのまま（変更なし）
- `docs/language-spec/02-lexical-structure.md` — キーワードテーブル更新、fn除去反映
- `docs/language-spec/03-type-system.md` — fn除去反映、参照更新
- `docs/language-spec/04-declarations-and-statements.md` — 旧04+05統合＋旧06から宣言移動
- `docs/language-spec/05-expressions.md` — 旧06スリム化（純粋な式のみ）
- `docs/language-spec/06-algebraic-effects.md` — 旧08＋効果関連集約
- `docs/language-spec/07-modules.md` — 旧09、構文修正
- `docs/language-spec/08-standard-library.md` — 旧10、セクション修正
- `docs/language-spec/09-examples.md` — 旧11、構文修正
- `docs/language-spec/10-grammar.md` — 旧12、EBNF更新＋散文修正
- `docs/language-spec.md` — 新TOC（更新履歴削除）
- `AGENTS.md` — 新ファイル構成反映

### Definition of Done
- [ ] `grep -rn "fn(" docs/language-spec/ | grep -v "12-grammar\|10-grammar"` → FunctionHeader/FunctionExprでのfnが0件
- [ ] `grep -rn "04-declarations\.md\|05-statements\|06-expressions\|07-ownership\|08-algebraic-effects\|09-modules\|10-standard-library\|11-examples\|12-grammar" docs/ AGENTS.md` → 0件
- [ ] 全10章のファイルが正しい番号・名前で存在
- [ ] language-spec.md のTOCが全章の実際のセクション構成と一致

### Must Have
- EBNF(10-grammar.md)が唯一のソースオブトゥルース
- fn除去がFunctionHeader/FunctionExprの全出現に反映
- TraitItemがLetDeclベースに統一
- 章間の重複コンテンツが解消（1つの概念は1つの章で説明）
- 全コード例がEBNF準拠

### Must NOT Have (Guardrails)
- 新しい言語機能の追加（export EBNF定義等は対象外）
- 所有権システム関連のコンテンツ（07章は完全削除）
- 英語への翻訳（日本語仕様のまま）
- EBNFに反するコード例の残存
- 章間の重複説明の残存（参照リンクに置き換え）
- `<T>` ジェネリクス構文（`[T]` のみ）
- `: Type` 戻り型構文（`-> Type` のみ）
- `if cond { ... }` 構文（`if cond then { ... }` のみ）
- `->` マップリテラル区切り（`:` のみ）
- `list[0]` インデックスアクセス（`.get(0)` のみ）

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (ドキュメントのみのプロジェクト)
- **Automated tests**: None (ドキュメント整理のため)
- **Framework**: N/A

### QA Policy
Every task MUST include agent-executed QA scenarios.
- **ドキュメント整合性**: Bash (grep) — 禁止パターン検索、クロスリファレンス検証
- **ファイル存在確認**: Bash (ls/find) — 正しいファイル名・構成の確認
- **コンテンツ検証**: Bash (grep/wc) — セクション存在確認、行数確認

Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Foundation — EBNF source of truth + file cleanup):
├── Task 2: Update EBNF grammar — fn removal, TraitItem, prose fixes [deep]
└── Task 1: Delete unnecessary files (AFTER Task 2 reads design/taskdef) [quick]

Wave 2 (Chapter Restructuring + Content Fixes — MAX PARALLEL):
├── Task 3:  Merge old 04+05 → new 04-declarations-and-statements (depends: 2) [deep]
├── Task 4:  Slim old 06 → new 05-expressions (depends: 2) [deep]
├── Task 5:  Consolidate old 08 + effects → new 06-algebraic-effects (depends: 2) [deep]
├── Task 6:  Fix 02-lexical-structure — keyword table + fn removal (depends: 2) [unspecified-high]
├── Task 7:  Fix 03-type-system — fn removal + refs (depends: 2) [quick]
├── Task 8:  Fix old 09 → new 07-modules (depends: 2) [quick]
├── Task 9:  Fix old 10 → new 08-standard-library (depends: 2) [quick]
└── Task 10: Fix old 11 → new 09-examples (depends: 2) [unspecified-high]

Wave 3 (Integration — file ops + cross-references):
├── Task 11: Rename/move files to final numbering (depends: 3-10) [quick]
├── Task 12: Rewrite language-spec.md TOC (depends: 11) [quick]
├── Task 13: Update all cross-references across all files (depends: 11) [unspecified-high]
└── Task 14: Update AGENTS.md (depends: 11) [quick]

Wave FINAL (Verification — 4 parallel reviews):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review — grep for forbidden patterns (unspecified-high)
├── Task F3: Real manual QA — verify every chapter (unspecified-high)
└── Task F4: Scope fidelity check (deep)
-> Present results -> Get explicit user okay

Critical Path: T2 → T3-T10 → T11 → T13 → F1-F4 → user okay
Parallel Speedup: ~65% faster than sequential
Max Concurrent: 8 (Wave 2)
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| T2   | —         | T1, T3-T10 | 1    |
| T1   | T2        | —      | 1    |
| T3   | T2        | T11    | 2    |
| T4   | T2        | T11    | 2    |
| T5   | T2        | T11    | 2    |
| T6   | T2        | T11    | 2    |
| T7   | T2        | T11    | 2    |
| T8   | T2        | T11    | 2    |
| T9   | T2        | T11    | 2    |
| T10  | T2        | T11    | 2    |
| T11  | T3-T10    | T12-T14| 3    |
| T12  | T11       | F1-F4  | 3    |
| T13  | T11       | F1-F4  | 3    |
| T14  | T11       | F1-F4  | 3    |
| F1   | T12-T14   | —      | F    |
| F2   | T12-T14   | —      | F    |
| F3   | T12-T14   | —      | F    |
| F4   | T12-T14   | —      | F    |

### Agent Dispatch Summary

- **Wave 1**: **2** (sequential) — T2 → `deep`, then T1 → `quick`
- **Wave 2**: **8** — T3 → `deep`, T4 → `deep`, T5 → `deep`, T6 → `unspecified-high`, T7 → `quick`, T8 → `quick`, T9 → `quick`, T10 → `unspecified-high`
- **Wave 3**: **4** — T11 → `quick`, T12 → `quick`, T13 → `unspecified-high`, T14 → `quick`
- **FINAL**: **4** — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

- [x] 1. Delete unnecessary files

  **What to do**:
  - Delete `docs/design/function-signature-unification-plan.md`
  - Delete `docs/design/let-decl-unification-plan.md`
  - Delete `docs/design/type-keyword-unification-plan.md`
  - Delete `docs/design/` directory itself
  - Delete `taskdef/function-type-syntax-update.md`
  - Delete `taskdef/` directory itself
  - Delete `docs/language-spec/07-ownership.md`

  **Must NOT do**:
  - Delete any other spec files
  - Modify any remaining files (content fixes are separate tasks)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple file deletions only, no content decisions
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO (must run AFTER Task 2)
  - **Parallel Group**: Wave 1 (after Task 2 completes)
  - **Blocks**: Nothing directly (other tasks reference content, not these files)
  - **Blocked By**: Task 2 (T2 must read design/taskdef files before T1 deletes them)

  **References**:
  **Pattern References**:
  - N/A — straightforward deletions

  **Acceptance Criteria**:
  ```
  Scenario: All target files deleted
    Tool: Bash
    Steps:
      1. Run: rm docs/design/function-signature-unification-plan.md docs/design/let-decl-unification-plan.md docs/design/type-keyword-unification-plan.md
      2. Run: rmdir docs/design/
      3. Run: rm taskdef/function-type-syntax-update.md
      4. Run: rmdir taskdef/
      5. Run: rm docs/language-spec/07-ownership.md
      6. Run: ls docs/design/ 2>&1
      7. Run: ls taskdef/ 2>&1
      8. Run: ls docs/language-spec/07-ownership.md 2>&1
    Expected Result: Steps 6-8 all return "No such file or directory"
    Evidence: .sisyphus/evidence/task-1-files-deleted.txt

  Scenario: No other files accidentally deleted
    Tool: Bash
    Steps:
      1. Run: ls docs/language-spec/*.md | wc -l
    Expected Result: 11 files remain (01-06, 08-12 — 07 deleted)
    Evidence: .sisyphus/evidence/task-1-remaining-files.txt
  ```

  **Commit**: YES (group 1)
  - Message: `chore: remove obsolete design plans, taskdef, and ownership chapter`
  - Files: `docs/design/*, taskdef/*, docs/language-spec/07-ownership.md`

- [x] 2. Update EBNF grammar — fn removal, TraitItem unification, prose fixes

  **What to do**:
  - **FunctionHeader**: Remove `fn` keyword
    - Current: `FunctionHeader ::= "fn" GenericParams? ParamList ("->" ReturnType)?`
    - New: `FunctionHeader ::= GenericParams? ParamList ("->" ReturnType)?`
  - **TraitItem**: Unify with LetDecl
    - Current: `TraitItem ::= "effect"? Identifier "=" FunctionHeader ("=>" Expression)?`
    - New: `TraitItem ::= LetDecl`
  - **ImplItem**: Unify with LetDecl
    - Current: `ImplItem ::= "effect"? Identifier "=" FunctionDefinitionExpr`
    - New: `ImplItem ::= LetDecl`
  - **Keyword list**: Remove `fn` from Keyword production
    - Current line includes `"fn"` — remove it
  - **Prose fixes in 12.3 section**:
    - L293: `if cond { ... }` → `if cond then { ... } else { ... }`
    - L295: `fn[GenericParams]?(Params)?: ReturnType = Expr` → Update to reflect fn removal
    - L307: Review `BindExpr`, `PartialApplicationExpr` mentions — if not in EBNF, mark clearly as "将来検討" or remove
  - **Cross-reference updates**: L327-329 still reference old chapter numbers (07-ownership, 08-algebraic-effects, 09-modules) — update to placeholder comments noting these will be renamed in T11

  **Must NOT do**:
  - Add new grammar productions (e.g., export in LetDecl)
  - Change the body separator (`=` for FunctionDefinitionExpr — this stays)
  - Modify any other chapter files (those are Wave 2 tasks)

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Grammar changes require careful consistency analysis; fn removal affects multiple productions
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 1)
  - **Blocks**: Tasks 3-10 (all depend on updated EBNF as source of truth)
  - **Blocked By**: None

  **References**:
  **Pattern References**:
  - `docs/language-spec/12-grammar.md:215-216` — Current FunctionDefinitionExpr + FunctionHeader definitions
  - `docs/language-spec/12-grammar.md:224-228` — Current TraitItem, EffectItem definitions
  - `docs/language-spec/12-grammar.md:236-239` — Current ImplDecl, ImplItem definitions
  - `docs/language-spec/12-grammar.md:247-250` — Current Keyword list
  - `docs/language-spec/12-grammar.md:286-307` — Prose section 12.3.5 with issues

  **API/Type References**:
  - `docs/design/function-signature-unification-plan.md:58-67` — TraitItem/EffectItem target state (file will be deleted by T1 but read this FIRST for context)
  - `taskdef/function-type-syntax-update.md:11-16` — fn removal scope (read FIRST before T1 deletes)

  **WHY Each Reference Matters**:
  - grammar.md:215-216: These are the exact lines to modify for FunctionHeader fn removal
  - grammar.md:224-228: TraitItem needs complete replacement; EffectItem already correct
  - grammar.md:236-239: ImplItem needs alignment with LetDecl pattern
  - grammar.md:247-250: fn must be removed from keyword list after removal from grammar
  - grammar.md:286-307: Prose must match updated EBNF productions
  - design plan and taskdef: Read these first as they document the intended target state and rationale

  **QA Scenarios**:
  ```
  Scenario: fn removed from FunctionHeader
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "FunctionHeader" docs/language-spec/12-grammar.md
      2. Assert line contains GenericParams? ParamList but NOT "fn"
    Expected Result: FunctionHeader definition has no "fn" keyword
    Evidence: .sisyphus/evidence/task-2-fn-removed.txt

  Scenario: TraitItem uses LetDecl
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "TraitItem" docs/language-spec/12-grammar.md
      2. Assert definition is "TraitItem ::= LetDecl"
    Expected Result: TraitItem unified with LetDecl
    Evidence: .sisyphus/evidence/task-2-traititem.txt

  Scenario: ImplItem uses LetDecl
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "ImplItem" docs/language-spec/12-grammar.md
      2. Assert definition is "ImplItem ::= LetDecl"
    Expected Result: ImplItem unified with LetDecl
    Evidence: .sisyphus/evidence/task-2-implitem.txt

  Scenario: fn removed from Keyword list
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "Keyword" docs/language-spec/12-grammar.md
      2. Assert "fn" does not appear in the Keyword production
    Expected Result: "fn" absent from keyword list
    Evidence: .sisyphus/evidence/task-2-keyword.txt

  Scenario: Prose section 12.3.5 corrected
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "if cond {" docs/language-spec/12-grammar.md
    Expected Result: 0 matches — all corrected to "if ... then ..."
    Evidence: .sisyphus/evidence/task-2-prose-if.txt
  ```

  **Commit**: YES (group 2)
  - Message: `docs: update EBNF grammar — remove fn from FunctionHeader, unify TraitItem/ImplItem with LetDecl`
  - Files: `docs/language-spec/12-grammar.md`

- [x] 3. Create new 04-declarations-and-statements.md — merge old 04 + 05 + declaration content from 06

  **What to do**:
  - Create `docs/language-spec/04-declarations-and-statements.md` (新規ファイル)
  - **旧04-declarations.md** から全コンテンツを取り込み:
    - 4.1 宣言の概要、4.2 変数宣言、4.3 関数定義、4.4 型定義、4.5 効果/ハンドラ型定義、4.6 トレイト定義
  - **旧05-statements.md** から取り込み:
    - return文の説明
    - ループ構文（while/for/loop）— ただしbreak/continueの効果としての説明は06-algebraic-effects(T5)に委譲、ここではループ構文の基本形のみ
  - **旧06-expressions.md** から宣言関連コンテンツを移動:
    - TypeDefinitionExpr, TraitDefinitionExpr, EffectDefinitionExpr, HandlerDefinitionExpr の説明
    - AliasDefinitionExpr の説明
    - FunctionDefinitionExpr の説明
    - これらは「定義式は宣言的に使われる」として宣言章に配置
  - **fn除去の反映**: 全コード例からFunctionHeader/FunctionExprの`fn`を削除。T2で更新されたEBNFに準拠
  - **TraitItem/ImplItem変更の反映**: trait/effect内のメンバー定義をLetDeclベースの新構文に更新
  - 章番号を「4.」で統一（4.1〜4.N）
  - 旧04, 旧05のファイルはT11で削除

  **Must NOT do**:
  - 効果ハンドリングの詳細説明（→ T5の06章へ）
  - break/continueの代数的効果としての説明（→ T5の06章へ）
  - パターンマッチングの詳細（→ T4の05章へ）
  - 式としての制御構造の詳細（if/match — → T4の05章へ）

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 3つのソースファイルからコンテンツを統合し、一貫した章として再構成する複雑なタスク
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4-10)
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **References**:
  **Pattern References**:
  - `docs/language-spec/04-declarations.md` — 全体がソース（旧宣言章、全428行）
  - `docs/language-spec/05-statements.md` — 全体がソース（旧文章、全242行）
  - `docs/language-spec/06-expressions.md:350-550` — 定義式セクション（TypeDefinitionExpr〜HandlerDefinitionExpr）の移動元
  - `docs/language-spec/06-expressions.md:670-780` — FunctionDefinitionExpr説明の移動元

  **API/Type References**:
  - `docs/language-spec/12-grammar.md:47` — LetDecl定義（T2更新後）
  - `docs/language-spec/12-grammar.md:200-231` — 定義式の文法ルール（TypeDefinitionExpr〜HandlerDefinitionExpr）
  - `docs/language-spec/12-grammar.md:215-216` — FunctionDefinitionExpr + FunctionHeader（T2更新後、fn除去済み）

  **WHY Each Reference Matters**:
  - 04-declarations.md: 宣言セクションのベースコンテンツ。構造とスタイルを維持
  - 05-statements.md: return文とループ構文を取り込む。break/continueの効果説明は06章(T5)に委譲
  - 06-expressions.md:350-550: 定義式の説明を移動。06章に残すと重複が解消されない
  - 06-expressions.md:670-780: 関数定義の詳細説明も移動
  - 12-grammar.md: T2更新後のEBNFがソースオブトゥルース。全コード例をこれに準拠させる

  **QA Scenarios**:
  ```
  Scenario: New file exists with merged content
    Tool: Bash
    Steps:
      1. Run: ls docs/language-spec/04-declarations-and-statements.md
      2. Run: grep -c "##" docs/language-spec/04-declarations-and-statements.md
    Expected Result: File exists, has 8+ section headings (declarations + statements + definition exprs)
    Evidence: .sisyphus/evidence/task-3-file-exists.txt

  Scenario: fn keyword removed from all code examples
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "= fn(" docs/language-spec/04-declarations-and-statements.md
    Expected Result: 0 matches
    Evidence: .sisyphus/evidence/task-3-fn-removed.txt

  Scenario: No effect handling details in this chapter
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "with.*handler\|resume\|継続" docs/language-spec/04-declarations-and-statements.md
    Expected Result: 0 matches (or only cross-reference links to chapter 6)
    Evidence: .sisyphus/evidence/task-3-no-effects.txt

  Scenario: TraitItem examples use new LetDecl syntax
    Tool: Bash (grep)
    Steps:
      1. Run: grep -A2 "trait" docs/language-spec/04-declarations-and-statements.md | grep "let"
    Expected Result: Trait member definitions use "let name: FunctionType" format
    Evidence: .sisyphus/evidence/task-3-traititem-syntax.txt
  ```

  **Commit**: YES (group 3)
  - Message: `docs: restructure spec chapters — merge declarations+statements, slim expressions, consolidate effects`
  - Files: `docs/language-spec/04-declarations-and-statements.md`

- [x] 4. Create new 05-expressions.md — slim old 06 to pure expressions only

  **What to do**:
  - Create `docs/language-spec/05-expressions.md` (新規ファイル)
  - **旧06-expressions.md** から以下のコンテンツのみを残す:
    - 式の概要と設計原則
    - ブロック式 (`BlockExpr`)
    - 制御構造: `if`/`elif`/`else`、`match`
    - パターンマッチング詳細
    - リテラル式（数値、文字列、コレクション）
    - 関数呼び出し式 (`CallExpr`)
    - メンバーアクセス式 (`MemberAccessExpr`)
    - 二項/単項演算式
    - 代入式 (`AssignmentExpr`)
    - レンジ式 (`RangeExpr`)
    - グループ化式 (`GroupedExpr`)
    - 文字列補間 (`StringInterpolation`)
    - リスト内包表記 (`ListComprehension`)
    - レコード構築式 (`RecordExpr`)
  - **削除（他章へ移動済み）**:
    - TypeDefinitionExpr, TraitDefinitionExpr, EffectDefinitionExpr, HandlerDefinitionExpr → T3(04章)
    - AliasDefinitionExpr, FunctionDefinitionExpr → T3(04章)
    - WithExpr, 効果操作呼び出し → T5(06章)
    - bind式 → EBNFにないため削除 or 「将来検討」と明記
    - 関数合成 → EBNFにないため削除 or 「将来検討」と明記
  - **fn除去の反映**: 残るコード例でfnキーワードを削除
  - **マップリテラル構文修正**: `->` → `:` に全修正（旧06 L97, L181, L184, L187）
  - 章番号を「5.」で統一

  **Must NOT do**:
  - 定義式の説明を残す（T3の04章に移動済み）
  - 効果関連の説明を残す（T5の06章に移動済み）
  - 新しい式の追加

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 875行の巨大ファイルから適切にコンテンツを選別・除去する判断が必要
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3, 5-10)
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **References**:
  **Pattern References**:
  - `docs/language-spec/06-expressions.md` — 全体がソース（875行）。以下のセクションを残す:
    - 6.1 式の概要（冒頭〜L50付近）
    - 6.2 ブロック式（L50〜L90付近）
    - 6.3 制御構造 — if/match のみ（L90〜L250付近、ただしbind/withは除外）
    - 6.4 パターンマッチング（L250〜L350付近）
    - 6.7 メンバーアクセス式（L780〜末尾）
  - `docs/language-spec/12-grammar.md:49-76` — Expression定義の全プロダクション（何が式として定義されているかの正確なリスト）
  - `docs/language-spec/12-grammar.md:131-133` — MapLiteral, MapEntry定義（`:` 区切り）

  **WHY Each Reference Matters**:
  - 06-expressions.md: ソースファイル。セクションごとに残す/移動を判断する
  - grammar.md:49-76: Expression定義が正。ここにないものは式章から除去
  - grammar.md:131-133: MapEntry定義が `:` であることを確認し、コード例を修正

  **QA Scenarios**:
  ```
  Scenario: File is significantly smaller than original
    Tool: Bash
    Steps:
      1. Run: wc -l docs/language-spec/05-expressions.md
    Expected Result: 400行以下（元の875行から大幅削減）
    Evidence: .sisyphus/evidence/task-4-linecount.txt

  Scenario: No definition expressions remain
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "TypeDefinitionExpr\|TraitDefinitionExpr\|EffectDefinitionExpr\|HandlerDefinitionExpr\|AliasDefinitionExpr\|FunctionDefinitionExpr" docs/language-spec/05-expressions.md
    Expected Result: 0 matches (or only brief cross-references to chapter 4)
    Evidence: .sisyphus/evidence/task-4-no-definitions.txt

  Scenario: No effect handling content remains
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "WithExpr\|効果操作\|handler\|resume" docs/language-spec/05-expressions.md
    Expected Result: 0 matches (or only cross-references to chapter 6)
    Evidence: .sisyphus/evidence/task-4-no-effects.txt

  Scenario: Map literal syntax corrected
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n '"->' docs/language-spec/05-expressions.md
      2. Run: grep -n "-> " docs/language-spec/05-expressions.md | grep -v "ReturnType\|戻り値\|->"
    Expected Result: No `->` used as map entry separator
    Evidence: .sisyphus/evidence/task-4-map-syntax.txt

  Scenario: fn removed from code examples
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "= fn(" docs/language-spec/05-expressions.md
    Expected Result: 0 matches
    Evidence: .sisyphus/evidence/task-4-fn-removed.txt
  ```

  **Commit**: YES (group 3)
  - Message: (same as T3 — grouped commit)
  - Files: `docs/language-spec/05-expressions.md`

- [x] 5. Create new 06-algebraic-effects.md — consolidate all effect material

  **What to do**:
  - Create `docs/language-spec/06-algebraic-effects.md` (新規ファイル)
  - **旧08-algebraic-effects.md** から全コンテンツを取り込み（ベース）:
    - 8.1 概念と目的、8.2 効果インターフェース、8.3 ハンドラ型、8.4 Effectパラメータ、8.5 with構文、8.6 継続制御、8.7 ライフサイクル管理、8.8 依存性注入、8.9 設計上の考慮事項
  - **旧06-expressions.md** から効果関連を移動:
    - WithExpr の詳細説明
    - 効果操作呼び出し式の説明
    - bind式の説明（EBNFにない場合は「将来検討」と明記するか削除）
  - **旧05-statements.md** から効果関連を移動:
    - break/continue の代数的効果としての説明
    - ループと効果の相互作用の説明
  - **fn除去の反映**: 全コード例からfnキーワードを削除
  - **TraitItem/EffectItem変更の反映**: effect定義内のメンバーをLetDeclベースの新構文に更新
  - 章番号を「6.」で統一
  - 旧08のファイルはT11で削除

  **Must NOT do**:
  - 宣言としてのeffect/handler定義の基本構文説明（→ T3の04章）
  - ループ構文そのものの説明（→ T3の04章）
  - マッチ式やif式の説明（→ T4の05章）

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 複数ソースから効果関連コンテンツを集約。重複排除と一貫性確保が必要
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3-4, 6-10)
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **References**:
  **Pattern References**:
  - `docs/language-spec/08-algebraic-effects.md` — 全体がベースソース（476行）
  - `docs/language-spec/06-expressions.md:540-680` — WithExpr, 効果操作呼び出しの詳細
  - `docs/language-spec/05-statements.md:50-242` — ループ/break/continueの効果説明

  **API/Type References**:
  - `docs/language-spec/12-grammar.md:175-176` — WithExpr定義
  - `docs/language-spec/12-grammar.md:227-231` — EffectDefinitionExpr, HandlerDefinitionExpr, EffectItem, HandlerItem
  - `docs/language-spec/12-grammar.md:144-145` — BreakExpr, ContinueExpr

  **WHY Each Reference Matters**:
  - 08-algebraic-effects.md: ベースコンテンツ。既存の構成を維持しつつ追加素材を統合
  - 06-expressions.md:540-680: with式と効果操作の説明を移動。05章(T4)には残さない
  - 05-statements.md:50-242: break/continueの効果セマンティクスは効果章に属する
  - grammar.md: 全構文をEBNF準拠に更新するための参照

  **QA Scenarios**:
  ```
  Scenario: All effect material consolidated
    Tool: Bash (grep)
    Steps:
      1. Run: grep -c "##" docs/language-spec/06-algebraic-effects.md
      2. Run: grep -n "with.*handler\|WithExpr\|break.*効果\|continue.*効果" docs/language-spec/06-algebraic-effects.md | head -5
    Expected Result: 10+ section headings; contains WithExpr and break/continue effect explanations
    Evidence: .sisyphus/evidence/task-5-effects-consolidated.txt

  Scenario: fn removed from code examples
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "= fn(" docs/language-spec/06-algebraic-effects.md
    Expected Result: 0 matches
    Evidence: .sisyphus/evidence/task-5-fn-removed.txt

  Scenario: EffectItem examples use LetDecl syntax
    Tool: Bash (grep)
    Steps:
      1. Run: grep -B2 -A2 "effect.*{" docs/language-spec/06-algebraic-effects.md | head -20
    Expected Result: Effect members use "let name: FunctionType" format
    Evidence: .sisyphus/evidence/task-5-effectitem-syntax.txt
  ```

  **Commit**: YES (group 3)
  - Message: (same as T3 — grouped commit)
  - Files: `docs/language-spec/06-algebraic-effects.md`

- [x] 6. Fix 02-lexical-structure.md — keyword table update, fn removal, chapter reference preparation

  **What to do**:
  - **キーワードテーブル更新**:
    - `fn` キーワードの行を削除（文法から除去されたため）
    - `bind` の行: EBNFに `BindExpr` がないため「将来検討」と明記するか行を削除
    - `resume` の行: 文法ルール参照を確認し、正確な参照先に更新
    - `handler` の行: 参照先を旧06章から新04章（定義式）と新06章（効果）に更新
    - 各行の章参照を新番号に更新（旧04→新04、旧06→新05、旧08→新06、旧09→新07 等）
  - **fn除去の反映**: 章内のコード例やfn関連の説明を更新
  - **演算子テーブル**: マップリテラルの `->` 記法への言及があれば `:` に修正

  **Must NOT do**:
  - 新しいキーワードの追加
  - 演算子の追加/削除

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: キーワードテーブルは~30行の章参照を含む高密度ファイル。全行のレビューが必要
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3-5, 7-10)
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **References**:
  **Pattern References**:
  - `docs/language-spec/02-lexical-structure.md:18-45` — キーワードテーブル（各行に章参照あり）
  - `docs/language-spec/02-lexical-structure.md:140` — 演算子の説明
  - `docs/language-spec/02-lexical-structure.md:256` — その他の参照

  **API/Type References**:
  - `docs/language-spec/12-grammar.md:247-250` — 更新後のKeyword production（T2でfn除去済み）

  **WHY Each Reference Matters**:
  - 02:18-45: ここが最大のリスク。全行を1つずつレビューし、旧章番号→新章番号に更新
  - 12-grammar.md: fnがKeywordから除去されていることを確認し、テーブルと一致させる

  **QA Scenarios**:
  ```
  Scenario: fn keyword row removed or updated
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "| fn |" docs/language-spec/02-lexical-structure.md
    Expected Result: fn行が存在しない、またはfnが除去された旨の説明に更新
    Evidence: .sisyphus/evidence/task-6-fn-keyword.txt

  Scenario: No references to old chapter numbers
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "05-statements\|06-expressions\|07-ownership\|08-algebraic\|09-modules\|10-standard\|11-examples\|12-grammar" docs/language-spec/02-lexical-structure.md
    Expected Result: 0 matches
    Evidence: .sisyphus/evidence/task-6-old-refs.txt
  ```

  **Commit**: YES (group 4)
  - Message: `docs: fix EBNF inconsistencies across all chapters — fn removal, syntax corrections`
  - Files: `docs/language-spec/02-lexical-structure.md`

- [x] 7. Fix 03-type-system.md — fn removal, chapter reference update

  **What to do**:
  - **fn除去の反映**: 関数型の説明・コード例から `fn` キーワードを削除
    - 旧: `fn(Int, String) -> Bool` → 新: `(Int, String) -> Bool`
    - FunctionType の説明を更新
  - **タプル型のセマンティクス明記**: `(Type)` が `Type` と同一であることを明記（taskdef要件）
  - **章参照更新**: 他章への参照を新番号に更新
    - L82, L84, L86, L96 付近の参照

  **Must NOT do**:
  - 新しい型の追加
  - 型システムの設計変更

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 119行の小さなファイル。fn除去と参照更新のみ
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3-6, 8-10)
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **References**:
  **Pattern References**:
  - `docs/language-spec/03-type-system.md` — 全体（119行）
  - `docs/language-spec/03-type-system.md:82-96` — 他章への参照箇所

  **API/Type References**:
  - `docs/language-spec/12-grammar.md:36-45` — FunctionType定義（fnなし）

  **WHY Each Reference Matters**:
  - 03-type-system.md: 小さいファイルだが関数型の説明がfn除去の影響を受ける
  - grammar.md:36-45: FunctionTypeの正確な定義。コード例をこれに合わせる

  **QA Scenarios**:
  ```
  Scenario: fn removed from function type examples
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "fn(" docs/language-spec/03-type-system.md
    Expected Result: 0 matches（FunctionType表記にfnが残っていない）
    Evidence: .sisyphus/evidence/task-7-fn-removed.txt

  Scenario: (Type) semantics documented
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "(Type)\|単一.*タプル\|1要素" docs/language-spec/03-type-system.md
    Expected Result: 1+ matches — (Type) = Type の説明が存在
    Evidence: .sisyphus/evidence/task-7-tuple-semantics.txt
  ```

  **Commit**: YES (group 4)
  - Message: (same as T6 — grouped commit)
  - Files: `docs/language-spec/03-type-system.md`

- [x] 8. Create new 07-modules.md from old 09 — fix return type syntax

  **What to do**:
  - Copy `docs/language-spec/09-modules.md` → `docs/language-spec/07-modules.md`
  - **戻り型構文修正**: 全関数定義の `: Type` を `-> Type` に修正
    - L22, L23, L26, L91, L98, L139, L159 付近（7箇所以上）
    - 例: `fn(a: Int, b: Int): Int = a + b` → `(a: Int, b: Int) -> Int = a + b`
    - 注: fn除去も同時に反映
  - **章番号更新**: セクション番号を「7.」に変更
  - 旧09のファイルはT11で削除

  **Must NOT do**:
  - モジュールシステムの仕様変更
  - export構文のEBNF追加

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 184行。機械的な構文置換が主
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3-7, 9-10)
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **References**:
  **Pattern References**:
  - `docs/language-spec/09-modules.md` — 全体がソース（184行）

  **API/Type References**:
  - `docs/language-spec/12-grammar.md:215-216` — FunctionDefinitionExpr（T2更新後、fn除去済み）
  - `docs/language-spec/12-grammar.md:241-243` — ModuleDecl, ImportDecl

  **WHY Each Reference Matters**:
  - 09-modules.md: ソースファイル。全コード例の構文を修正
  - grammar.md: 正確な構文をここから参照

  **QA Scenarios**:
  ```
  Scenario: No colon return types remain
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "): [A-Z]" docs/language-spec/07-modules.md
    Expected Result: 0 matches
    Evidence: .sisyphus/evidence/task-8-return-type.txt

  Scenario: fn removed from function definitions
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "= fn(" docs/language-spec/07-modules.md
    Expected Result: 0 matches
    Evidence: .sisyphus/evidence/task-8-fn-removed.txt
  ```

  **Commit**: YES (group 4)
  - Message: (same as T6 — grouped commit)
  - Files: `docs/language-spec/07-modules.md`

- [x] 9. Create new 08-standard-library.md from old 10 — fix section structure

  **What to do**:
  - Copy `docs/language-spec/10-standard-library.md` → `docs/language-spec/08-standard-library.md`
  - **セクション構成修正**: TOCでは「10.2 I/O操作」「10.3 並行処理」となっているが、実際のファイル内容はprelude（コアデータ構造、トレイト、効果、基本関数、デシュガールール）。実態に合わせたセクション見出しに整理
  - **fn除去の反映**: コード例からfnキーワードを削除
  - **章番号更新**: セクション番号を「8.」に変更
  - 旧10のファイルはT11で削除

  **Must NOT do**:
  - 新しい標準ライブラリ機能の追加
  - 機能の削除

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 278行。セクション見出しの整理とfn除去が主
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3-8, 10)
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **References**:
  **Pattern References**:
  - `docs/language-spec/10-standard-library.md` — 全体がソース（278行）
  - `docs/language-spec.md:57-60` — 旧TOCの10章エントリ（実態と乖離している箇所）

  **WHY Each Reference Matters**:
  - 10-standard-library.md: 実際のセクション構成を確認し、見出しを正確に反映
  - language-spec.md: TOCとの乖離を把握

  **QA Scenarios**:
  ```
  Scenario: Section headings match actual content
    Tool: Bash (grep)
    Steps:
      1. Run: grep "^##" docs/language-spec/08-standard-library.md
    Expected Result: Headings reflect actual prelude content (not "I/O操作" or "並行処理")
    Evidence: .sisyphus/evidence/task-9-sections.txt

  Scenario: fn removed from code examples
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "= fn(" docs/language-spec/08-standard-library.md
    Expected Result: 0 matches
    Evidence: .sisyphus/evidence/task-9-fn-removed.txt
  ```

  **Commit**: YES (group 4)
  - Message: (same as T6 — grouped commit)
  - Files: `docs/language-spec/08-standard-library.md`

- [x] 10. Create new 09-examples.md from old 11 — fix syntax errors

  **What to do**:
  - Copy `docs/language-spec/11-examples.md` → `docs/language-spec/09-examples.md`
  - **構文修正**:
    - L297付近: `scores[name]` → `scores.get(name)` — インデックスアクセスはメソッド呼び出しに
    - 全コード例: fn除去を反映
    - 全コード例: 他のEBNF不整合（もしあれば）を修正
  - **章番号更新**: セクション番号を「9.」に変更
  - **章参照更新**: 他章への参照を新番号に更新（特にL13の12-grammar参照）
  - 旧11のファイルはT11で削除

  **Must NOT do**:
  - サンプルプログラムの追加/削除
  - ロジックの変更（構文修正のみ）

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 430行のサンプルコード。全例文のEBNF準拠チェックが必要
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3-9)
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **References**:
  **Pattern References**:
  - `docs/language-spec/11-examples.md` — 全体がソース（430行）
  - `docs/language-spec/11-examples.md:297` — `scores[name]` の問題箇所

  **API/Type References**:
  - `docs/language-spec/12-grammar.md` — 全EBNF定義（T2更新後）。全コード例の準拠チェック用

  **WHY Each Reference Matters**:
  - 11-examples.md: 全サンプルコードを1つずつEBNFと照合する必要がある
  - grammar.md: ソースオブトゥルース。特にfn除去後の新構文を確認

  **QA Scenarios**:
  ```
  Scenario: scores[name] index access fixed
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "\[name\]\|\[key\]\|\[0\]" docs/language-spec/09-examples.md
    Expected Result: 0 matches（ブラケットインデックスなし）
    Evidence: .sisyphus/evidence/task-10-index-access.txt

  Scenario: fn removed from all examples
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "= fn(" docs/language-spec/09-examples.md
    Expected Result: 0 matches
    Evidence: .sisyphus/evidence/task-10-fn-removed.txt

  Scenario: No angle bracket generics
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "<[A-Z][a-z]*>" docs/language-spec/09-examples.md
    Expected Result: 0 matches
    Evidence: .sisyphus/evidence/task-10-generics.txt

  Scenario: No colon return types
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "): [A-Z]" docs/language-spec/09-examples.md
    Expected Result: 0 matches
    Evidence: .sisyphus/evidence/task-10-return-types.txt
  ```

  **Commit**: YES (group 4)
  - Message: (same as T6 — grouped commit)
  - Files: `docs/language-spec/09-examples.md`

- [x] 11. Rename/delete files to final numbering scheme

  **What to do**:
  - **削除** (Wave 2で新ファイルが作成済みのため、旧ファイルは不要):
    - `docs/language-spec/04-declarations.md` (→ 新04-declarations-and-statements.md に統合済み)
    - `docs/language-spec/05-statements.md` (→ 新04に統合済み)
    - `docs/language-spec/06-expressions.md` (→ 新05-expressions.md に再構成済み)
    - `docs/language-spec/08-algebraic-effects.md` (→ 新06-algebraic-effects.md に再構成済み)
    - `docs/language-spec/09-modules.md` (→ 新07-modules.md にコピー済み)
    - `docs/language-spec/10-standard-library.md` (→ 新08-standard-library.md にコピー済み)
    - `docs/language-spec/11-examples.md` (→ 新09-examples.md にコピー済み)
  - **リネーム**:
    - `docs/language-spec/12-grammar.md` → `docs/language-spec/10-grammar.md`
  - **確認**: 最終的なファイル一覧:
    - 01-introduction.md, 02-lexical-structure.md, 03-type-system.md,
    - 04-declarations-and-statements.md, 05-expressions.md, 06-algebraic-effects.md,
    - 07-modules.md, 08-standard-library.md, 09-examples.md, 10-grammar.md

  **Must NOT do**:
  - ファイル内容の変更（内容はWave 2で完了済み）
  - 01-introduction.md の変更（変更不要）

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 機械的なファイル削除・リネーム操作のみ
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential start, then T12-T14 parallel)
  - **Blocks**: Tasks 12, 13, 14
  - **Blocked By**: Tasks 3-10

  **References**:
  N/A — 機械的操作

  **QA Scenarios**:
  ```
  Scenario: Exactly 10 chapter files exist
    Tool: Bash
    Steps:
      1. Run: ls docs/language-spec/*.md | grep -v "^docs/language-spec.md$" | sort
    Expected Result: Exactly 10 files: 01-introduction.md through 10-grammar.md
    Evidence: .sisyphus/evidence/task-11-file-list.txt

  Scenario: No old files remain
    Tool: Bash
    Steps:
      1. Run: ls docs/language-spec/04-declarations.md docs/language-spec/05-statements.md docs/language-spec/06-expressions.md docs/language-spec/07-ownership.md docs/language-spec/08-algebraic-effects.md docs/language-spec/09-modules.md docs/language-spec/10-standard-library.md docs/language-spec/11-examples.md docs/language-spec/12-grammar.md 2>&1
    Expected Result: All return "No such file or directory"
    Evidence: .sisyphus/evidence/task-11-old-files-gone.txt
  ```

  **Commit**: YES (group 5)
  - Message: `docs: renumber chapters to 01-10, update TOC, cross-references, and AGENTS.md`
  - Files: deleted files + renamed 10-grammar.md

- [x] 12. Rewrite language-spec.md — new TOC, remove changelog

  **What to do**:
  - **更新履歴セクション削除**: `## 更新履歴` 以降を全削除（L73-107）
  - **目次の完全書き換え**: 新10章構成に合わせてTOCを再生成
    - 各章のリンク先を新ファイル名に
    - 各章のサブセクション一覧を、実際のファイル内のセクション見出し（`##`）から正確に生成
  - 新TOC構成:
    1. [序論](language-spec/01-introduction.md)
    2. [字句構造](language-spec/02-lexical-structure.md)
    3. [型システム](language-spec/03-type-system.md)
    4. [宣言と文](language-spec/04-declarations-and-statements.md)
    5. [式](language-spec/05-expressions.md)
    6. [代数的効果](language-spec/06-algebraic-effects.md)
    7. [モジュールシステム](language-spec/07-modules.md)
    8. [標準ライブラリ](language-spec/08-standard-library.md)
    9. [サンプルプログラム](language-spec/09-examples.md)
    10. [文法（EBNF）](language-spec/10-grammar.md)

  **Must NOT do**:
  - 章の内容変更
  - 新しいセクションの追加

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: TOC書き換えと削除のみ
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 13, 14 — after T11)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 11

  **References**:
  **Pattern References**:
  - `docs/language-spec.md` — 現在のTOC＋更新履歴（107行）
  - 全新章ファイル (01-10) — サブセクション見出しの正確な一覧を取得

  **QA Scenarios**:
  ```
  Scenario: Changelog removed
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "更新履歴" docs/language-spec.md
    Expected Result: 0 matches
    Evidence: .sisyphus/evidence/task-12-changelog.txt

  Scenario: All 10 chapters in TOC
    Tool: Bash (grep)
    Steps:
      1. Run: grep -c "language-spec/" docs/language-spec.md
    Expected Result: 10 (one link per chapter)
    Evidence: .sisyphus/evidence/task-12-toc-count.txt

  Scenario: TOC links point to existing files
    Tool: Bash
    Steps:
      1. Run: grep -Eo 'language-spec/[^ )]+\.md' docs/language-spec.md | while read f; do test -f "docs/$f" && echo "OK: $f" || echo "MISSING: $f"; done
    Expected Result: All 10 return "OK"
    Evidence: .sisyphus/evidence/task-12-toc-links.txt
  ```

  **Commit**: YES (group 5)
  - Message: (same as T11 — grouped commit)
  - Files: `docs/language-spec.md`

- [x] 13. Update all cross-references across all chapter files

  **What to do**:
  - **全10章ファイルを走査**し、他章への参照（ファイル名リンク、章番号言及）を新番号に更新
  - 主な置換マッピング:
    - `04-declarations.md` → `04-declarations-and-statements.md`
    - `05-statements.md` → `04-declarations-and-statements.md` (統合先)
    - `06-expressions.md` → `05-expressions.md`
    - `07-ownership.md` → 削除（参照自体を除去）
    - `08-algebraic-effects.md` → `06-algebraic-effects.md`
    - `09-modules.md` → `07-modules.md`
    - `10-standard-library.md` → `08-standard-library.md`
    - `11-examples.md` → `09-examples.md`
    - `12-grammar.md` → `10-grammar.md`
  - **章番号の言及更新**: 「第8章」「8.」「[8. 代数的効果]」等の散文内参照も新番号に
  - **ownership参照の除去**: 07-ownershipへの参照はすべて削除（「将来フェーズ」の言及も除去）
  - **検証**: 修正後に旧ファイル名が残っていないことをgrepで確認

  **Must NOT do**:
  - 参照先の内容変更
  - 新しい参照の追加

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: ~50箇所のクロスリファレンスを全ファイルで網羅的に更新。漏れは致命的
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 12, 14 — after T11)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 11

  **References**:
  **Pattern References**:
  - 全10章ファイル — 各ファイル内のMarkdownリンクと散文内章番号参照
  - `docs/language-spec/02-lexical-structure.md` — 最多参照（~30箇所）
  - `docs/language-spec/10-grammar.md:327-329` — 旧章番号への参照（T2で部分対応済み）

  **QA Scenarios**:
  ```
  Scenario: Zero old filenames in any chapter
    Tool: Bash (grep)
    Steps:
      1. Run: grep -rn "04-declarations\.md\|05-statements\|06-expressions\|07-ownership\|08-algebraic-effects\|09-modules\|10-standard-library\|11-examples\|12-grammar" docs/language-spec/
    Expected Result: 0 matches
    Evidence: .sisyphus/evidence/task-13-old-refs.txt

  Scenario: No dangling ownership references
    Tool: Bash (grep)
    Steps:
      1. Run: grep -rn "所有権\|ownership\|07-ownership" docs/language-spec/
    Expected Result: 0 matches (or clearly marked as "将来フェーズ" without file reference)
    Evidence: .sisyphus/evidence/task-13-ownership-refs.txt
  ```

  **Commit**: YES (group 5)
  - Message: (same as T11 — grouped commit)
  - Files: all chapter files with cross-reference updates

- [x] 14. Update AGENTS.md — reflect new file structure

  **What to do**:
  - **Repository Structure セクション更新**:
    - `docs/language-spec/` の一覧を新10章に更新
    - `docs/design/` のエントリを削除
    - `taskdef/` のエントリを削除
  - **Syntax Rules テーブル更新**:
    - `fn` 関連の行: fn除去を反映（「Function body: `fn(x: Int) -> Int = x + 1`」→ `(x: Int) -> Int = x + 1`）
  - **Key Design Decisions 更新**:
    - `fn` キーワードに関する決定事項を更新
  - **Features NOT in MVP 更新**:
    - 所有権システムへの言及を維持（将来フェーズの注意喚起として）
    - ただし07-ownership.mdへの参照は削除
  - **Spec Editing Workflow 更新**:
    - `12-grammar.md` → `10-grammar.md` に更新

  **Must NOT do**:
  - Build/Test/Lint コマンドの変更
  - Git Conventions の変更

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: AGENTS.mdの特定セクションのみ更新
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 12, 13 — after T11)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 11

  **References**:
  **Pattern References**:
  - `AGENTS.md:11-26` — Repository Structure（ファイル一覧）
  - `AGENTS.md:66-68` — Syntax Rules テーブル
  - `AGENTS.md:149-151` — Spec Editing Workflow

  **QA Scenarios**:
  ```
  Scenario: Repository structure reflects new layout
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "language-spec/" AGENTS.md
    Expected Result: Lists 01-10 chapters with correct filenames; no design/ or taskdef/
    Evidence: .sisyphus/evidence/task-14-agents-structure.txt

  Scenario: No references to deleted files
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "07-ownership\|12-grammar\|design/\|taskdef/" AGENTS.md
    Expected Result: 0 matches for file paths (ownership concept mention OK, file reference NOT OK)
    Evidence: .sisyphus/evidence/task-14-old-refs.txt

  Scenario: fn removal reflected in syntax rules
    Tool: Bash (grep)
    Steps:
      1. Run: grep -n "fn(x:" AGENTS.md
    Expected Result: 0 matches — updated to new syntax without fn
    Evidence: .sisyphus/evidence/task-14-fn-syntax.txt
  ```

  **Commit**: YES (group 5)
  - Message: (same as T11 — grouped commit)
  - Files: `AGENTS.md`

---

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Present consolidated results to user and get explicit "okay" before completing.

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (grep for patterns, read files). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality Review** — `unspecified-high`
  Run comprehensive grep for ALL forbidden syntax patterns across docs/:
  - `<T>` generics (should be `[T]`)
  - `: Type` return types (should be `-> Type`) — excluding field declarations `name: Type`
  - `if ... {` without `then`
  - `->` in map literals (should be `:`)
  - `list[0]` index access (should be `.get()`)
  - `fn(` in FunctionHeader positions
  - `true`/`false` (should be `True`/`False`)
  - `s"..."` (should be `f"..."`)
  - References to old filenames (04-declarations.md, 05-statements, etc.)
  Output: `Patterns [N clean/N issues] | Files [N/N checked] | VERDICT`

- [ ] F3. **Real Manual QA** — `unspecified-high`
  Read EVERY chapter (01 through 10) sequentially. For each chapter:
  - Verify file exists with correct name
  - Verify section headings match TOC in language-spec.md
  - Verify all code examples compile against EBNF rules
  - Verify no dangling cross-references
  - Verify no duplicate content across chapters
  Save per-chapter report to `.sisyphus/evidence/final-qa/`.
  Output: `Chapters [N/N pass] | Cross-refs [N/N valid] | Duplicates [CLEAN/N issues] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect unaccounted changes.
  Output: `Tasks [N/N compliant] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

| Commit | Scope | Message | Files |
|--------|-------|---------|-------|
| 1 | T1 | `chore: remove obsolete design plans, taskdef, and ownership chapter` | docs/design/*, taskdef/*, docs/language-spec/07-ownership.md |
| 2 | T2 | `docs: update EBNF grammar — remove fn from FunctionHeader, unify TraitItem with LetDecl` | docs/language-spec/12-grammar.md |
| 3 | T3-T5 | `docs: restructure spec chapters — merge declarations+statements, slim expressions, consolidate effects` | docs/language-spec/04-*, 05-*, 06-* (new files) |
| 4 | T6-T10 | `docs: fix EBNF inconsistencies across all chapters — fn removal, syntax corrections` | docs/language-spec/02-*, 03-*, 09-*, 10-*, 11-* |
| 5 | T11-T14 | `docs: renumber chapters to 01-10, update TOC, cross-references, and AGENTS.md` | docs/language-spec/*, docs/language-spec.md, AGENTS.md |

---

## Success Criteria

### Verification Commands
```bash
# No old filenames referenced
grep -rn "04-declarations\.md\|05-statements\|06-expressions\|07-ownership\|08-algebraic-effects\|09-modules\|10-standard-library\|11-examples\|12-grammar" docs/ AGENTS.md
# Expected: 0 matches

# No fn in FunctionHeader positions (excluding grammar definition itself)
grep -rn "= fn(" docs/language-spec/ | grep -v "10-grammar"
# Expected: 0 matches

# No angle bracket generics
grep -rn "<[A-Z][a-z]*>" docs/language-spec/
# Expected: 0 matches

# No colon return types in function defs
grep -rn "fn(.*): [A-Z]" docs/language-spec/
# Expected: 0 matches

# All 10 chapter files exist
ls docs/language-spec/0[1-9]-*.md docs/language-spec/10-*.md
# Expected: 10 files listed

# No deleted files remain
ls docs/language-spec/07-ownership.md docs/language-spec/11-examples.md docs/language-spec/12-grammar.md 2>&1
# Expected: "No such file or directory" for all

# No design/taskdef remain
ls docs/design/ taskdef/ 2>&1
# Expected: "No such file or directory"
```

### Final Checklist
- [ ] All "Must Have" present
- [ ] All "Must NOT Have" absent
- [ ] All 10 chapters exist with correct numbering
- [ ] language-spec.md TOC matches actual chapter structure
- [ ] AGENTS.md reflects new file layout
- [ ] Zero forbidden syntax patterns in any chapter
