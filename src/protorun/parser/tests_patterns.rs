// パターンマッチングパーサーのテスト

use super::*;
use crate::protorun::ast::{Pattern as AstPattern, LiteralValue};

#[test]
fn test_parse_literal_pattern() {
    // 整数リテラルパターン
    {
        let input = "match x { 42 => true }";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            crate::protorun::ast::Expr::MatchExpr { cases, .. } => {
                assert_eq!(cases.len(), 1);
                
                match &cases[0].0 {
                    AstPattern::Literal(LiteralValue::Int(value), _) => assert_eq!(*value, 42),
                    _ => panic!("期待される整数リテラルパターンではありません"),
                }
            },
            _ => panic!("期待されるmatch式ではありません"),
        }
    }
    
    // 文字列リテラルパターン
    {
        let input = "match x { \"hello\" => true }";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            crate::protorun::ast::Expr::MatchExpr { cases, .. } => {
                assert_eq!(cases.len(), 1);
                
                match &cases[0].0 {
                    AstPattern::Literal(LiteralValue::String(value), _) => assert_eq!(value, "hello"),
                    _ => panic!("期待される文字列リテラルパターンではありません"),
                }
            },
            _ => panic!("期待されるmatch式ではありません"),
        }
    }
    
    // 真偽値リテラルパターン
    {
        let input = "match x { true => 1, false => 0 }";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            crate::protorun::ast::Expr::MatchExpr { cases, .. } => {
                assert_eq!(cases.len(), 2);
                
                match &cases[0].0 {
                    AstPattern::Literal(LiteralValue::Bool(value), _) => assert_eq!(*value, true),
                    _ => panic!("期待される真偽値リテラルパターンではありません"),
                }
                
                match &cases[1].0 {
                    AstPattern::Literal(LiteralValue::Bool(value), _) => assert_eq!(*value, false),
                    _ => panic!("期待される真偽値リテラルパターンではありません"),
                }
            },
            _ => panic!("期待されるmatch式ではありません"),
        }
    }
}

#[test]
fn test_parse_wildcard_pattern() {
    let input = "match x { _ => 42 }";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        crate::protorun::ast::Expr::MatchExpr { cases, .. } => {
            assert_eq!(cases.len(), 1);
            
            match &cases[0].0 {
                AstPattern::Wildcard(_) => (),
                _ => panic!("期待されるワイルドカードパターンではありません"),
            }
        },
        _ => panic!("期待されるmatch式ではありません"),
    }
}

#[test]
fn test_parse_identifier_pattern() {
    let input = "match x { n => n * 2 }";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        crate::protorun::ast::Expr::MatchExpr { cases, .. } => {
            assert_eq!(cases.len(), 1);
            
            match &cases[0].0 {
                AstPattern::Identifier(name, _) => assert_eq!(name, "n"),
                _ => panic!("期待される識別子パターンではありません"),
            }
        },
        _ => panic!("期待されるmatch式ではありません"),
    }
}

#[test]
fn test_parse_tuple_pattern() {
    let input = "match point { (x, y) => x + y }";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        crate::protorun::ast::Expr::MatchExpr { cases, .. } => {
            assert_eq!(cases.len(), 1);
            
            match &cases[0].0 {
                AstPattern::Tuple(patterns, _) => {
                    assert_eq!(patterns.len(), 2);
                    
                    match &patterns[0] {
                        AstPattern::Identifier(name, _) => assert_eq!(name, "x"),
                        _ => panic!("期待される識別子パターンではありません"),
                    }
                    
                    match &patterns[1] {
                        AstPattern::Identifier(name, _) => assert_eq!(name, "y"),
                        _ => panic!("期待される識別子パターンではありません"),
                    }
                },
                _ => panic!("期待されるタプルパターンではありません"),
            }
        },
        _ => panic!("期待されるmatch式ではありません"),
    }
}

#[test]
fn test_parse_constructor_pattern() {
    let input = "match opt { Some(value) => value, None => 0 }";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        crate::protorun::ast::Expr::MatchExpr { cases, .. } => {
            assert_eq!(cases.len(), 2);
            
            // Some(value)パターン
            match &cases[0].0 {
                AstPattern::Constructor { name, arguments, .. } => {
                    assert_eq!(name, "Some");
                    assert_eq!(arguments.len(), 1);
                    
                    match &arguments[0] {
                        AstPattern::Identifier(name, _) => assert_eq!(name, "value"),
                        _ => panic!("期待される識別子パターンではありません"),
                    }
                },
                _ => panic!("期待されるコンストラクタパターンではありません"),
            }
            
            // Noneパターン
            match &cases[1].0 {
                AstPattern::Identifier(name, _) => assert_eq!(name, "None"),
                _ => panic!("期待される識別子パターンではありません"),
            }
        },
        _ => panic!("期待されるmatch式ではありません"),
    }
}

#[test]
fn test_parse_nested_pattern() {
    let input = "match data { Some((x, y)) => x + y, None => 0 }";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        crate::protorun::ast::Expr::MatchExpr { cases, .. } => {
            assert_eq!(cases.len(), 2);
            
            // Some((x, y))パターン
            match &cases[0].0 {
                AstPattern::Constructor { name, arguments, .. } => {
                    assert_eq!(name, "Some");
                    assert_eq!(arguments.len(), 1);
                    
                    match &arguments[0] {
                        AstPattern::Tuple(patterns, _) => {
                            assert_eq!(patterns.len(), 2);
                            
                            match &patterns[0] {
                                AstPattern::Identifier(name, _) => assert_eq!(name, "x"),
                                _ => panic!("期待される識別子パターンではありません"),
                            }
                            
                            match &patterns[1] {
                                AstPattern::Identifier(name, _) => assert_eq!(name, "y"),
                                _ => panic!("期待される識別子パターンではありません"),
                            }
                        },
                        _ => panic!("期待されるタプルパターンではありません"),
                    }
                },
                _ => panic!("期待されるコンストラクタパターンではありません"),
            }
        },
        _ => panic!("期待されるmatch式ではありません"),
    }
}

#[test]
fn test_parse_pattern_with_guard() {
    let input = "match x { n if n > 0 => n * 2, _ => 0 }";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        crate::protorun::ast::Expr::MatchExpr { cases, .. } => {
            assert_eq!(cases.len(), 2);
            
            // n if n > 0 => n * 2
            match &cases[0] {
                (pattern, guard, expr) => {
                    match pattern {
                        AstPattern::Identifier(name, _) => assert_eq!(name, "n"),
                        _ => panic!("期待される識別子パターンではありません"),
                    }
                    
                    assert!(guard.is_some());
                    match guard.as_ref().unwrap() {
                        crate::protorun::ast::Expr::BinaryOp { operator, .. } => {
                            assert_eq!(*operator, crate::protorun::ast::BinaryOperator::Gt);
                        },
                        _ => panic!("期待される二項演算ではありません"),
                    }
                    
                    match expr {
                        crate::protorun::ast::Expr::BinaryOp { operator, .. } => {
                            assert_eq!(*operator, crate::protorun::ast::BinaryOperator::Mul);
                        },
                        _ => panic!("期待される二項演算ではありません"),
                    }
                }
            }
        },
        _ => panic!("期待されるmatch式ではありません"),
    }
}
