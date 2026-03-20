# Task 8: Create 07-modules.md with Syntax Corrections

## Completion Status: ✅ COMPLETE

### What Was Done
1. **File Created**: `docs/language-spec/07-modules.md` (184 lines)
2. **Source**: Copied from `docs/language-spec/09-modules.md` with corrections
3. **Section Numbering**: Updated from 9.x to 7.x throughout

### Syntax Corrections Applied

#### Return Type Syntax
- **OLD**: `fn(a: Int, b: Int): Int = a + b`
- **NEW**: `(a: Int, b: Int) -> Int = a + b`

All instances corrected:
- Line 22: `export let add = (a: Int, b: Int) -> Int = a + b;`
- Line 23: `export let subtract = (a: Int, b: Int) -> Int = a - b;`
- Line 26: `let helper = () -> Int = 42;`
- Line 91: `export let drawRect = (x: Int, y: Int, width: Int, height: Int, color: Color) -> Unit = { ... };`
- Line 98: `export let drawCube = (x: Int, y: Int, z: Int, size: Int, color: Color) -> Unit = { ... };`
- Line 126: `export let drawLine = (...) -> Unit = { ... };`
- Line 132: `export let drawSphere = (...) -> Unit = { ... };`
- Line 139: `let main = () -> Unit = { ... };`
- Line 159: `let greet = (name: String) -> String = f"Hello, {name}!"`

#### Function Keyword Removal
- Removed all `fn` keywords from function definitions
- All functions now use direct parameter list syntax

### Verification Results

✅ **Return Type Check**: `grep -n "): [A-Z]"` → No matches (PASS)
✅ **fn= Check**: `grep -n "= fn("` → No matches (PASS)
✅ **fn( Check**: `grep -n "fn("` → No matches (PASS)
✅ **Section Numbers**: All subsections use 7.x format (7.1-7.7)
✅ **File Size**: 184 lines (same as original)

### Evidence Files Created
- `.sisyphus/evidence/task-8-return-type.txt`
- `.sisyphus/evidence/task-8-fn-removed.txt`

### Grammar Compliance
All syntax now matches the canonical EBNF in `12-grammar.md`:
- Line 215: `FunctionDefinitionExpr ::= FunctionHeader "=" Expression`
- Line 216: `FunctionHeader ::= GenericParams? ParamList ("->" ReturnType)?`
