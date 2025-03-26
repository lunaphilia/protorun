// 文パーサーのテスト

use super::*;
use crate::protorun::ast::{Stmt, Expr};

#[test]
fn test_parse_expr_statement() {
    let input = "42;";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.declarations.len(), 0);
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Expr { expr, .. } => {
            match expr {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 42),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待される式文ではありません"),
    }
}

#[test]
fn test_parse_let_with_type_annotation() {
    let input = "let x: Int = 42;";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Let { name, type_annotation, value, .. } => {
            assert_eq!(name, "x");
            
            match type_annotation {
                Some(ty) => {
                    match ty {
                        crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "Int"),
                        _ => panic!("期待される単純型ではありません"),
                    }
                },
                None => panic!("型注釈が期待されます"),
            }
            
            match value {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 42),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待されるlet文ではありません"),
    }
}

#[test]
fn test_parse_var_statement() {
    let input = "var x = 42;";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Var { name, type_annotation, value, .. } => {
            assert_eq!(name, "x");
            assert!(type_annotation.is_none());
            
            match value {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 42),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待されるvar文ではありません"),
    }
}

#[test]
fn test_parse_var_with_type_annotation() {
    let input = "var counter: Int = 0;";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Var { name, type_annotation, value, .. } => {
            assert_eq!(name, "counter");
            
            match type_annotation {
                Some(ty) => {
                    match ty {
                        crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "Int"),
                        _ => panic!("期待される単純型ではありません"),
                    }
                },
                None => panic!("型注釈が期待されます"),
            }
            
            match value {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 0),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待されるvar文ではありません"),
    }
}

#[test]
fn test_parse_return_statement() {
    // 値を返すreturn文
    {
        let input = "return 42;";
        let mut parser = Parser::new(None);
        let program = parser.parse_program(input).unwrap();
        
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::Return { value, .. } => {
                assert!(value.is_some());
                match value.as_ref().unwrap() {
                    Expr::IntLiteral(val, _) => assert_eq!(*val, 42),
                    _ => panic!("期待される整数リテラルではありません"),
                }
            },
            _ => panic!("期待されるreturn文ではありません"),
        }
    }
    
    // 値なしのreturn文
    {
        let input = "return;";
        let mut parser = Parser::new(None);
        let program = parser.parse_program(input).unwrap();
        
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::Return { value, .. } => {
                assert!(value.is_none());
            },
            _ => panic!("期待されるreturn文ではありません"),
        }
    }
}

#[test]
fn test_parse_complex_return_statement() {
    let input = "return x * y + z;";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Return { value, .. } => {
            assert!(value.is_some());
            
            match value.as_ref().unwrap() {
                Expr::BinaryOp { operator, .. } => assert_eq!(*operator, crate::protorun::ast::BinaryOperator::Add),
                _ => panic!("期待される二項演算ではありません"),
            }
        },
        _ => panic!("期待されるreturn文ではありません"),
    }
}
