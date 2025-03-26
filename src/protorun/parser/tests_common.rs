// 共通ユーティリティのテスト

use super::*;
use crate::protorun::ast::{Expr, Span};
use crate::protorun::error::Error;
use crate::protorun::symbol::{Symbol, SymbolTable, ScopeKind, SymbolKind};

#[test]
fn test_parse_error_unexpected_token() {
    let input = "let x = ;";
    let mut parser = Parser::new(None);
    let result = parser.parse_program(input);
    
    assert!(result.is_err());
    
    match result {
        Ok(_) => panic!("エラーが期待されます"),
        Err(err) => {
            match err.kind {
                crate::protorun::error::ErrorKind::Syntax(_) => (), // 期待される構文エラー
                _ => panic!("期待される構文エラーではありません"),
            }
        }
    }
}

#[test]
fn test_parser_context_scope_management() {
    let input = "{ let x = 10; { let y = 20; } }";
    let mut ctx = ParserContext::new(input, None);
    
    // 初期スコープはグローバル
    assert_eq!(ctx.current_scope_kind(), ScopeKind::Global);
    assert_eq!(ctx.scope_depth(), 0);
    
    // ブロックスコープに入る
    ctx.enter_scope(ScopeKind::Block);
    assert_eq!(ctx.current_scope_kind(), ScopeKind::Block);
    assert_eq!(ctx.scope_depth(), 1);
    
    // シンボルを追加
    let symbol_x = Symbol {
        name: "x".to_string(),
        kind: SymbolKind::Variable,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 0, line: 0, column: 0 },
        is_mutable: false,
        type_info: None,
        is_used: false,
    };
    assert!(ctx.add_symbol(symbol_x.clone()));
    
    // ネストされたブロックスコープに入る
    ctx.enter_scope(ScopeKind::Block);
    assert_eq!(ctx.current_scope_kind(), ScopeKind::Block);
    assert_eq!(ctx.scope_depth(), 2);
    
    // 別のシンボルを追加
    let symbol_y = Symbol {
        name: "y".to_string(),
        kind: SymbolKind::Variable,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 0, line: 0, column: 0 },
        is_mutable: false,
        type_info: None,
        is_used: false,
    };
    assert!(ctx.add_symbol(symbol_y.clone()));
    
    // 内側のスコープから外側のシンボルを検索
    let lookup_x = ctx.lookup_symbol("x");
    assert!(lookup_x.is_some());
    assert_eq!(lookup_x.unwrap().name, "x");
    
    // 内側のスコープから内側のシンボルを検索
    let lookup_y = ctx.lookup_symbol("y");
    assert!(lookup_y.is_some());
    assert_eq!(lookup_y.unwrap().name, "y");
    
    // 内側のスコープを抜ける
    ctx.exit_scope();
    assert_eq!(ctx.current_scope_kind(), ScopeKind::Block);
    assert_eq!(ctx.scope_depth(), 1);
    
    // 外側のスコープから内側のシンボルを検索（見つからない）
    let lookup_y_after_exit = ctx.lookup_symbol("y");
    assert!(lookup_y_after_exit.is_none());
    
    // 外側のスコープから外側のシンボルを検索
    let lookup_x_after_exit = ctx.lookup_symbol("x");
    assert!(lookup_x_after_exit.is_some());
    assert_eq!(lookup_x_after_exit.unwrap().name, "x");
    
    // 外側のスコープを抜ける
    ctx.exit_scope();
    assert_eq!(ctx.current_scope_kind(), ScopeKind::Global);
    assert_eq!(ctx.scope_depth(), 0);
}

#[test]
fn test_parser_context_symbol_usage() {
    let input = "let x = 10; let y = x + 5;";
    let mut ctx = ParserContext::new(input, None);
    
    // シンボルを追加
    let symbol_x = Symbol {
        name: "x".to_string(),
        kind: SymbolKind::Variable,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 0, line: 0, column: 0 },
        is_mutable: false,
        type_info: None,
        is_used: false,
    };
    assert!(ctx.add_symbol(symbol_x.clone()));
    
    let symbol_y = Symbol {
        name: "y".to_string(),
        kind: SymbolKind::Variable,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 0, line: 0, column: 0 },
        is_mutable: false,
        type_info: None,
        is_used: false,
    };
    assert!(ctx.add_symbol(symbol_y.clone()));
    
    // シンボルの使用をマーク
    assert!(ctx.mark_symbol_used("x"));
    
    // 未使用シンボルを検出
    let unused_symbols = ctx.find_unused_symbols();
    assert_eq!(unused_symbols.len(), 1);
    assert_eq!(unused_symbols[0].name, "y");
    
    // 残りのシンボルも使用済みにマーク
    assert!(ctx.mark_symbol_used("y"));
    
    // 未使用シンボルを再度検出
    let unused_symbols_after = ctx.find_unused_symbols();
    assert_eq!(unused_symbols_after.len(), 0);
}

#[test]
fn test_parser_context_symbol_by_kind() {
    let input = "let x = 10; fn f() = 42;";
    let mut ctx = ParserContext::new(input, None);
    
    // 変数シンボルを追加
    let symbol_x = Symbol {
        name: "x".to_string(),
        kind: SymbolKind::Variable,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 0, line: 0, column: 0 },
        is_mutable: false,
        type_info: None,
        is_used: false,
    };
    assert!(ctx.add_symbol(symbol_x.clone()));
    
    // 関数シンボルを追加
    let symbol_f = Symbol {
        name: "f".to_string(),
        kind: SymbolKind::Function,
        type_annotation: None,
        declaration_span: Span { start: 0, end: 0, line: 0, column: 0 },
        is_mutable: false,
        type_info: None,
        is_used: false,
    };
    assert!(ctx.add_symbol(symbol_f.clone()));
    
    // 変数シンボルを検索
    let variables = ctx.find_symbols_by_kind(SymbolKind::Variable);
    assert_eq!(variables.len(), 1);
    assert_eq!(variables[0].name, "x");
    
    // 関数シンボルを検索
    let functions = ctx.find_symbols_by_kind(SymbolKind::Function);
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "f");
}

#[test]
fn test_parser_context_calculate_span() {
    let input = "let x = 10;\nlet y = 20;";
    let ctx = ParserContext::new(input, None);
    
    // 最初の行の位置情報
    let span1 = ctx.calculate_span(&input[10..]);
    assert_eq!(span1.line, 1);
    assert_eq!(span1.column, 10);
    
    // 2行目の位置情報
    let span2 = ctx.calculate_span(&input[20..]);
    assert_eq!(span2.line, 2);
    assert_eq!(span2.column, 8);
}

#[test]
fn test_ws_comments() {
    // 空白のスキップ
    {
        let input = "  \t\n  42";
        let result = common::ws_comments(literals::int_literal)(input);
        assert!(result.is_ok());
        
        let (remaining, value) = result.unwrap();
        assert_eq!(value, 42);
        assert_eq!(remaining, "");
    }
    
    // コメントのスキップ
    {
        let input = "// This is a comment\n42";
        let result = common::ws_comments(literals::int_literal)(input);
        assert!(result.is_ok());
        
        let (remaining, value) = result.unwrap();
        assert_eq!(value, 42);
        assert_eq!(remaining, "");
    }
    
    // 空白とコメントの混在
    {
        let input = "  // This is a comment\n  // Another comment\n  42";
        let result = common::ws_comments(literals::int_literal)(input);
        assert!(result.is_ok());
        
        let (remaining, value) = result.unwrap();
        assert_eq!(value, 42);
        assert_eq!(remaining, "");
    }
}

#[test]
fn test_identifier() {
    // 基本的な識別子
    {
        let input = "variable";
        let result = common::identifier(input);
        assert!(result.is_ok());
        
        let (remaining, value) = result.unwrap();
        assert_eq!(value, "variable");
        assert_eq!(remaining, "");
    }
    
    // アンダースコアを含む識別子
    {
        let input = "my_variable";
        let result = common::identifier(input);
        assert!(result.is_ok());
        
        let (remaining, value) = result.unwrap();
        assert_eq!(value, "my_variable");
        assert_eq!(remaining, "");
    }
    
    // 数字を含む識別子
    {
        let input = "variable123";
        let result = common::identifier(input);
        assert!(result.is_ok());
        
        let (remaining, value) = result.unwrap();
        assert_eq!(value, "variable123");
        assert_eq!(remaining, "");
    }
    
    // アンダースコアで始まる識別子
    {
        let input = "_variable";
        let result = common::identifier(input);
        assert!(result.is_ok());
        
        let (remaining, value) = result.unwrap();
        assert_eq!(value, "_variable");
        assert_eq!(remaining, "");
    }
    
    // 数字で始まる識別子（無効）
    {
        let input = "123variable";
        let result = common::identifier(input);
        assert!(result.is_err());
    }
}

#[test]
fn test_keyword() {
    // キーワードの認識
    {
        let input = "let x = 42";
        let result = common::keyword("let")(input);
        assert!(result.is_ok());
        
        let (remaining, value) = result.unwrap();
        assert_eq!(value, "let");
        assert_eq!(remaining, "x = 42");
    }
    
    // 空白とコメントを含むキーワード
    {
        let input = "  // comment\n  let x = 42";
        let result = common::keyword("let")(input);
        assert!(result.is_ok());
        
        let (remaining, value) = result.unwrap();
        assert_eq!(value, "let");
        assert_eq!(remaining, "x = 42");
    }
    
    // 不一致のキーワード
    {
        let input = "var x = 42";
        let result = common::keyword("let")(input);
        assert!(result.is_err());
    }
}

#[test]
fn test_delimited_list() {
    // カンマ区切りのリスト
    {
        let input = "(1, 2, 3)";
        let result = common::delimited_list(
            '(',
            literals::int_literal,
            ',',
            ')'
        )(input);
        assert!(result.is_ok());
        
        let (remaining, values) = result.unwrap();
        assert_eq!(values, vec![1, 2, 3]);
        assert_eq!(remaining, "");
    }
    
    // 空のリスト
    {
        let input = "()";
        let result = common::delimited_list(
            '(',
            literals::int_literal,
            ',',
            ')'
        )(input);
        assert!(result.is_ok());
        
        let (remaining, values) = result.unwrap();
        assert_eq!(values, Vec::<i64>::new());
        assert_eq!(remaining, "");
    }
    
    // 空白を含むリスト
    {
        let input = "( 1 , 2 , 3 )";
        let result = common::delimited_list(
            '(',
            literals::int_literal,
            ',',
            ')'
        )(input);
        assert!(result.is_ok());
        
        let (remaining, values) = result.unwrap();
        assert_eq!(values, vec![1, 2, 3]);
        assert_eq!(remaining, "");
    }
    
    // コメントを含むリスト
    {
        let input = "(1, // first item\n 2, // second item\n 3 // third item\n)";
        let result = common::delimited_list(
            '(',
            literals::int_literal,
            ',',
            ')'
        )(input);
        assert!(result.is_ok());
        
        let (remaining, values) = result.unwrap();
        assert_eq!(values, vec![1, 2, 3]);
        assert_eq!(remaining, "");
    }
}
