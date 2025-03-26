// リテラル値パーサーのテスト

use super::*;
use crate::protorun::ast::Expr;

#[test]
fn test_parse_int_literal() {
    let input = "42";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::IntLiteral(value, _) => assert_eq!(value, 42),
        _ => panic!("期待される整数リテラルではありません"),
    }
}

#[test]
fn test_parse_negative_int_literal() {
    let input = "-42";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::UnaryOp { operator, expr, .. } => {
            assert_eq!(operator, crate::protorun::ast::UnaryOperator::Neg);
            match *expr {
                Expr::IntLiteral(value, _) => assert_eq!(value, 42),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待される単項演算ではありません"),
    }
}

#[test]
fn test_parse_string_literal() {
    let input = r#""Hello, world!""#;
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::StringLiteral(value, _) => assert_eq!(value, "Hello, world!"),
        _ => panic!("期待される文字列リテラルではありません"),
    }
}

#[test]
fn test_parse_string_literal_with_escapes() {
    let input = r#""Hello,\nworld!""#;
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::StringLiteral(value, _) => assert_eq!(value, "Hello,\nworld!"),
        _ => panic!("期待される文字列リテラルではありません"),
    }
}

#[test]
fn test_parse_bool_literal() {
    // true
    {
        let input = "true";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BoolLiteral(value, _) => assert_eq!(value, true),
            _ => panic!("期待される真偽値リテラルではありません"),
        }
    }
    
    // false
    {
        let input = "false";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BoolLiteral(value, _) => assert_eq!(value, false),
            _ => panic!("期待される真偽値リテラルではありません"),
        }
    }
}

#[test]
fn test_parse_unit_literal() {
    let input = "()";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::UnitLiteral(_) => (),
        _ => panic!("期待されるユニットリテラルではありません"),
    }
}

#[test]
fn test_parse_list_literal() {
    let input = "[1, 2, 3]";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::ListLiteral { elements, .. } => {
            assert_eq!(elements.len(), 3);
            
            match &elements[0] {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 1),
                _ => panic!("期待される整数リテラルではありません"),
            }
            
            match &elements[1] {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 2),
                _ => panic!("期待される整数リテラルではありません"),
            }
            
            match &elements[2] {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 3),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待されるリストリテラルではありません"),
    }
}

#[test]
fn test_parse_empty_list_literal() {
    let input = "[]";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::ListLiteral { elements, .. } => {
            assert_eq!(elements.len(), 0);
        },
        _ => panic!("期待される空のリストリテラルではありません"),
    }
}

#[test]
fn test_parse_map_literal() {
    let input = "{\"key\" -> 42, \"another\" -> 100}";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::MapLiteral { entries, .. } => {
            assert_eq!(entries.len(), 2);
            
            match &entries[0].0 {
                Expr::StringLiteral(key, _) => assert_eq!(key, "key"),
                _ => panic!("期待される文字列リテラルではありません"),
            }
            
            match &entries[0].1 {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 42),
                _ => panic!("期待される整数リテラルではありません"),
            }
            
            match &entries[1].0 {
                Expr::StringLiteral(key, _) => assert_eq!(key, "another"),
                _ => panic!("期待される文字列リテラルではありません"),
            }
            
            match &entries[1].1 {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 100),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待されるマップリテラルではありません"),
    }
}

#[test]
fn test_parse_empty_map_literal() {
    let input = "{}";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::MapLiteral { entries, .. } => {
            assert_eq!(entries.len(), 0);
        },
        _ => panic!("期待される空のマップリテラルではありません"),
    }
}

#[test]
fn test_parse_set_literal() {
    let input = "#{1, 2, 3}";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::SetLiteral { elements, .. } => {
            assert_eq!(elements.len(), 3);
            
            match &elements[0] {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 1),
                _ => panic!("期待される整数リテラルではありません"),
            }
            
            match &elements[1] {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 2),
                _ => panic!("期待される整数リテラルではありません"),
            }
            
            match &elements[2] {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 3),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待されるセットリテラルではありません"),
    }
}

#[test]
fn test_parse_empty_set_literal() {
    let input = "#{}";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::SetLiteral { elements, .. } => {
            assert_eq!(elements.len(), 0);
        },
        _ => panic!("期待される空のセットリテラルではありません"),
    }
}
