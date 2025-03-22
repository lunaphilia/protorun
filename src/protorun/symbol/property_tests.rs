// シンボルテーブルのプロパティベーステスト

#[cfg(test)]
mod property_tests {
    use super::super::*;
    use crate::protorun::ast::Span;
    use proptest::prelude::*;
    
    proptest! {
        // シンボル名の追加と検索をテスト
        #[test]
        fn test_symbol_addition_and_lookup(name in "[a-zA-Z][a-zA-Z0-9_]*") {
            let mut table = SymbolTable::new(ScopeKind::Global);
            let symbol = Symbol {
                name: name.clone(),
                kind: SymbolKind::Variable,
                type_annotation: None,
                declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
                is_mutable: false,
                type_info: None,
                is_used: false,
            };
            
            assert!(table.add_symbol(symbol));
            
            let found = table.lookup_symbol(&name);
            assert!(found.is_some());
            assert_eq!(found.unwrap().name, name);
        }
        
        // 重複シンボルの検出をテスト
        #[test]
        fn test_duplicate_symbol_detection(name in "[a-zA-Z][a-zA-Z0-9_]*") {
            let mut table = SymbolTable::new(ScopeKind::Global);
            let symbol1 = Symbol {
                name: name.clone(),
                kind: SymbolKind::Variable,
                type_annotation: None,
                declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
                is_mutable: false,
                type_info: None,
                is_used: false,
            };
            
            let symbol2 = Symbol {
                name: name.clone(),
                kind: SymbolKind::Variable,
                type_annotation: None,
                declaration_span: Span { start: 2, end: 3, line: 1, column: 2 },
                is_mutable: true,
                type_info: None,
                is_used: false,
            };
            
            assert!(table.add_symbol(symbol1));
            assert!(!table.add_symbol(symbol2)); // 2回目は失敗するはず
        }
        
        // ネストされたスコープでの名前解決をテスト
        #[test]
        fn test_nested_scopes_name_resolution(
            global_name in "[a-zA-Z][a-zA-Z0-9_]*",
            local_name in "[a-zA-Z][a-zA-Z0-9_]*"
        ) {
            // 名前が異なることを確認（同じ場合はスキップ）
            prop_assume!(global_name != local_name);
            
            // グローバルスコープを作成
            let mut global = SymbolTable::new(ScopeKind::Global);
            let global_symbol = Symbol {
                name: global_name.clone(),
                kind: SymbolKind::Variable,
                type_annotation: None,
                declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
                is_mutable: false,
                type_info: None,
                is_used: false,
            };
            global.add_symbol(global_symbol);
            
            let global_rc = Rc::new(RefCell::new(global));
            
            // ローカルスコープを作成
            let mut local = SymbolTable::with_parent(ScopeKind::Function, global_rc.clone());
            let local_symbol = Symbol {
                name: local_name.clone(),
                kind: SymbolKind::Variable,
                type_annotation: None,
                declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
                is_mutable: false,
                type_info: None,
                is_used: false,
            };
            local.add_symbol(local_symbol);
            
            // グローバル変数をローカルスコープから検索
            let global_var = local.lookup_symbol_recursive(&global_name);
            assert!(global_var.is_some());
            assert_eq!(global_var.unwrap().name, global_name);
            
            // ローカル変数をローカルスコープから検索
            let local_var = local.lookup_symbol_recursive(&local_name);
            assert!(local_var.is_some());
            assert_eq!(local_var.unwrap().name, local_name);
            
            // ローカル変数をグローバルスコープから検索（見つからないはず）
            let not_found = global_rc.borrow().lookup_symbol_recursive(&local_name);
            assert!(not_found.is_none());
        }
        
        // シンボルの使用状態の追跡をテスト
        #[test]
        fn test_symbol_usage_tracking(name in "[a-zA-Z][a-zA-Z0-9_]*") {
            let mut table = SymbolTable::new(ScopeKind::Global);
            let symbol = Symbol {
                name: name.clone(),
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
            assert_eq!(unused[0].name, name);
            
            // 使用をマーク
            assert!(table.mark_symbol_used(&name));
            
            // 使用後は未使用リストに含まれない
            let unused = table.find_unused_symbols();
            assert_eq!(unused.len(), 0);
        }
        
        // 複数のシンボルを含むテーブルをテスト
        #[test]
        fn test_multiple_symbols(
            names in prop::collection::vec("[a-zA-Z][a-zA-Z0-9_]*", 1..10)
        ) {
            // 重複を排除
            let names: Vec<String> = names.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();
            
            let mut table = SymbolTable::new(ScopeKind::Global);
            
            // シンボルを追加
            for name in &names {
                let symbol = Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Variable,
                    type_annotation: None,
                    declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
                    is_mutable: false,
                    type_info: None,
                    is_used: false,
                };
                assert!(table.add_symbol(symbol));
            }
            
            // すべてのシンボルを検索
            for name in &names {
                let found = table.lookup_symbol(name);
                assert!(found.is_some());
                assert_eq!(found.unwrap().name, *name);
            }
            
            // 存在しないシンボルを検索
            let not_found = table.lookup_symbol("nonexistent");
            assert!(not_found.is_none());
            
            // 未使用シンボルを確認
            let unused = table.find_unused_symbols();
            assert_eq!(unused.len(), names.len());
            
            // 一部のシンボルを使用済みにマーク
            if !names.is_empty() {
                assert!(table.mark_symbol_used(&names[0]));
                
                // 未使用シンボルを再確認
                let unused = table.find_unused_symbols();
                assert_eq!(unused.len(), names.len() - 1);
            }
        }
        
        // 型定義シンボルのテスト
        #[test]
        fn test_type_symbol(
            name in "[A-Z][a-zA-Z0-9_]*",
            type_params in prop::collection::vec("[A-Z]", 0..3)
        ) {
            let mut table = SymbolTable::new(ScopeKind::Global);
            
            // 型定義シンボルを作成
            let type_symbol = Symbol {
                name: name.clone(),
                kind: SymbolKind::Type,
                type_annotation: None,
                declaration_span: Span { start: 0, end: 1, line: 1, column: 1 },
                is_mutable: false,
                type_info: Some(TypeInfo {
                    kind: TypeKind::Struct,
                    type_parameters: type_params.clone(),
                    fields: None,
                    variants: None,
                    super_trait: None,
                    aliased_type: None,
                }),
                is_used: false,
            };
            
            // シンボルを追加
            assert!(table.add_symbol(type_symbol));
            
            // シンボルを検索
            let found = table.lookup_symbol(&name);
            assert!(found.is_some());
            
            let found = found.unwrap();
            assert_eq!(found.kind, SymbolKind::Type);
            assert!(found.type_info.is_some());
            
            let type_info = found.type_info.as_ref().unwrap();
            assert_eq!(type_info.kind, TypeKind::Struct);
            assert_eq!(type_info.type_parameters, type_params);
        }
    }
}
