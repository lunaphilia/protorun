# Task 9: Create docs/language-spec/08-standard-library.md

## Status: ✓ COMPLETED

### What was done:
1. ✓ Read original 10-standard-library.md (278 lines)
2. ✓ Created new 08-standard-library.md with:
   - Section numbers changed from 10.x to 8.x
   - Title changed to "# 8. 標準ライブラリ"
   - All 4 `fn` keywords removed from code examples
   - Section structure preserved (actual prelude content, not "I/O操作"/"並行処理")

### Verification:
- ✓ File created: docs/language-spec/08-standard-library.md (278 lines)
- ✓ All section headers numbered 8.x (8.1 through 8.7)
- ✓ No `fn(` keywords remain in file
- ✓ Actual sections match prelude content:
  - 8.1 概要
  - 8.2 コアデータ型
  - 8.3 コアトレイト
  - 8.4 数値トレイト
  - 8.5 コア効果
  - 8.6 基本関数
  - 8.7 デシュガー規則

### Changes made:
- Line 236: `fn(self, message: String)` → `(self, message: String)`
- Line 239: `fn(self, message: String)` → `(self, message: String)`
- Line 242: `fn(self)` → `(self)`
- Line 278: `fn(x)` → `(x)`

### Evidence files created:
- .sisyphus/evidence/task-9-sections.txt
- .sisyphus/evidence/task-9-fn-removed.txt
