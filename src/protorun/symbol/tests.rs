// シンボルテーブルのテスト

use super::*;
use crate::protorun::ast::Span;

#[test]
fn test_symbol_addition() {
    let mut table = SymbolTable::new(ScopeKind::Global);
    let symbol = Symbol {
        name: "x".to_string(),
        kind: SymbolKind::Variable,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
        is_mutable: false,
    };
    
    assert!(table.add_symbol(symbol.clone()));
    assert!(!table.add_symbol(symbol)); // 2回目は失敗するはず
}

#[test]
fn test_symbol_lookup() {
    let mut table = SymbolTable::new(ScopeKind::Global);
    let symbol = Symbol {
        name: "x".to_string(),
        kind: SymbolKind::Variable,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
        is_mutable: false,
    };
    
    table.add_symbol(symbol);
    
    let found = table.lookup_symbol("x");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "x");
    
    assert!(table.lookup_symbol("y").is_none());
}

#[test]
fn test_nested_scopes() {
    let mut global = SymbolTable::new(ScopeKind::Global);
    let global_symbol = Symbol {
        name: "global_var".to_string(),
        kind: SymbolKind::Variable,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
        is_mutable: false,
    };
    global.add_symbol(global_symbol);
    
    let global_rc = Rc::new(RefCell::new(global));
    let mut function = SymbolTable::with_parent(ScopeKind::Function, global_rc.clone());
    let function_symbol = Symbol {
        name: "function_var".to_string(),
        kind: SymbolKind::Variable,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
        is_mutable: false,
    };
    function.add_symbol(function_symbol);
    
    // 関数スコープからグローバル変数を検索
    let global_var = function.lookup_symbol_recursive("global_var");
    assert!(global_var.is_some());
    assert_eq!(global_var.unwrap().name, "global_var");
    
    // グローバルスコープから関数変数を検索（見つからないはず）
    assert!(global_rc.borrow().lookup_symbol_recursive("function_var").is_none());
}
