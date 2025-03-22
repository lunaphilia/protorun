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
        type_info: None,
        is_used: false,
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
        type_info: None,
        is_used: false,
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
        type_info: None,
        is_used: false,
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
        type_info: None,
        is_used: false,
    };
    function.add_symbol(function_symbol);
    
    // 関数スコープからグローバル変数を検索
    let global_var = function.lookup_symbol_recursive("global_var");
    assert!(global_var.is_some());
    assert_eq!(global_var.unwrap().name, "global_var");
    
    // グローバルスコープから関数変数を検索（見つからないはず）
    assert!(global_rc.borrow().lookup_symbol_recursive("function_var").is_none());
}

#[test]
fn test_type_symbol_registration() {
    let mut table = SymbolTable::new(ScopeKind::Global);
    let type_symbol = Symbol {
        name: "MyStruct".to_string(),
        kind: SymbolKind::Type,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
        is_mutable: false,
        type_info: Some(TypeInfo {
            kind: TypeKind::Struct,
            type_parameters: vec!["T".to_string()],
        }),
        is_used: false,
    };
    
    assert!(table.add_symbol(type_symbol.clone()));
    
    let found = table.lookup_symbol("MyStruct");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.kind, SymbolKind::Type);
    assert!(found.type_info.is_some());
    let type_info = found.type_info.as_ref().unwrap();
    assert_eq!(type_info.kind, TypeKind::Struct);
    assert_eq!(type_info.type_parameters, vec!["T".to_string()]);
}

#[test]
fn test_symbol_usage_tracking() {
    let mut table = SymbolTable::new(ScopeKind::Global);
    let symbol = Symbol {
        name: "x".to_string(),
        kind: SymbolKind::Variable,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
        is_mutable: false,
        type_info: None,
        is_used: false,
    };
    
    table.add_symbol(symbol);
    
    // 使用前は未使用
    let unused = table.find_unused_symbols();
    assert_eq!(unused.len(), 1);
    assert_eq!(unused[0].name, "x");
    
    // 使用をマーク
    assert!(table.mark_symbol_used("x"));
    
    // 使用後は未使用リストに含まれない
    let unused = table.find_unused_symbols();
    assert_eq!(unused.len(), 0);
}

#[test]
fn test_find_symbols_by_kind() {
    let mut table = SymbolTable::new(ScopeKind::Global);
    
    // 変数シンボルを追加
    let var_symbol = Symbol {
        name: "x".to_string(),
        kind: SymbolKind::Variable,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
        is_mutable: false,
        type_info: None,
        is_used: false,
    };
    table.add_symbol(var_symbol);
    
    // 型シンボルを追加
    let type_symbol = Symbol {
        name: "MyType".to_string(),
        kind: SymbolKind::Type,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
        is_mutable: false,
        type_info: Some(TypeInfo {
            kind: TypeKind::Struct,
            type_parameters: Vec::new(),
        }),
        is_used: false,
    };
    table.add_symbol(type_symbol);
    
    // 変数シンボルを検索
    let vars = table.find_symbols_by_kind(SymbolKind::Variable);
    assert_eq!(vars.len(), 1);
    assert_eq!(vars[0].name, "x");
    
    // 型シンボルを検索
    let types = table.find_symbols_by_kind(SymbolKind::Type);
    assert_eq!(types.len(), 1);
    assert_eq!(types[0].name, "MyType");
}
