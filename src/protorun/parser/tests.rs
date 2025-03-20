// パーサーモジュールのテスト

use super::*;
use crate::protorun::ast::{BinaryOperator, Expr, Stmt, UnaryOperator};
use crate::protorun::error::ErrorKind;

#[test]
fn test_parse_expr_statement() {
    let input = "42;";
    let mut parser = Parser::from_str(input, None).unwrap();
    let program = parser.parse_program().unwrap();
    
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
fn test_parse_block_expr() {
    let input = "{ let x = 10; x }";
    let mut parser = Parser::from_str(input, None).unwrap();
    
    let expr = parser.parse_expression().unwrap();
    
    match expr {
        Expr::Identifier(name, _) => assert_eq!(name, "x"),
        _ => panic!("ブロック式の最後の式がIdentifierではありません"),
    }
}

#[test]
fn test_parse_nested_block_expr() {
    let input = "{ let x = 10; { let y = 20; x + y } }";
    let mut parser = Parser::from_str(input, None).unwrap();
    
    let expr = parser.parse_expression().unwrap();
    
    match expr {
        Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Add),
        _ => panic!("ネストされたブロック式の最後の式がBinaryOpではありません"),
    }
}

#[test]
fn test_parse_function_call() {
    let input = "add(10, 20)";
    let mut parser = Parser::from_str(input, None).unwrap();
    
    let expr = parser.parse_expression().unwrap();
    
    match expr {
        Expr::FunctionCall { function, arguments, .. } => {
            match &*function {
                Expr::Identifier(name, _) => assert_eq!(name, "add"),
                _ => panic!("期待される関数名識別子ではありません"),
            }
            
            assert_eq!(arguments.len(), 2);
            
            match &arguments[0] {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 10),
                _ => panic!("期待される整数リテラルではありません"),
            }
            
            match &arguments[1] {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 20),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待される関数呼び出しではありません"),
    }
}

#[test]
fn test_parse_nested_function_call() {
    let input = "add(multiply(10, 2), 20)";
    let mut parser = Parser::from_str(input, None).unwrap();
    
    let expr = parser.parse_expression().unwrap();
    
    match expr {
        Expr::FunctionCall { function, arguments, .. } => {
            match &*function {
                Expr::Identifier(name, _) => assert_eq!(name, "add"),
                _ => panic!("期待される関数名識別子ではありません"),
            }
            
            assert_eq!(arguments.len(), 2);
            
            match &arguments[0] {
                Expr::FunctionCall { function, arguments, .. } => {
                    match &**function {
                        Expr::Identifier(name, _) => assert_eq!(name, "multiply"),
                        _ => panic!("期待される関数名識別子ではありません"),
                    }
                    
                    assert_eq!(arguments.len(), 2);
                    
                    match &arguments[0] {
                        Expr::IntLiteral(value, _) => assert_eq!(*value, 10),
                        _ => panic!("期待される整数リテラルではありません"),
                    }
                    
                    match &arguments[1] {
                        Expr::IntLiteral(value, _) => assert_eq!(*value, 2),
                        _ => panic!("期待される整数リテラルではありません"),
                    }
                },
                _ => panic!("期待される関数呼び出しではありません"),
            }
            
            match &arguments[1] {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 20),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待される関数呼び出しではありません"),
    }
}

#[test]
fn test_parse_arithmetic_expressions() {
    // 加算
    {
        let input = "1 + 2";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Add),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 減算
    {
        let input = "5 - 3";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Sub),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 乗算
    {
        let input = "4 * 2";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Mul),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 除算
    {
        let input = "10 / 2";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Div),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 剰余
    {
        let input = "10 % 3";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Mod),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
}

#[test]
fn test_parse_unary_expressions() {
    // 負の数
    {
        let input = "-42";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::UnaryOp { operator, expr, .. } => {
                assert_eq!(operator, UnaryOperator::Neg);
                
                match *expr {
                    Expr::IntLiteral(value, _) => assert_eq!(value, 42),
                    _ => panic!("期待される整数リテラルではありません"),
                }
            },
            _ => panic!("期待される単項演算ではありません"),
        }
    }
    
    // 論理否定
    {
        let input = "!true";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::UnaryOp { operator, expr, .. } => {
                assert_eq!(operator, UnaryOperator::Not);
                
                match *expr {
                    Expr::BoolLiteral(value, _) => assert_eq!(value, true),
                    _ => panic!("期待される真偽値リテラルではありません"),
                }
            },
            _ => panic!("期待される単項演算ではありません"),
        }
    }
}

#[test]
fn test_parse_comparison_expressions() {
    // 等価
    {
        let input = "x == y";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Eq),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 非等価
    {
        let input = "x != y";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Neq),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // より小さい
    {
        let input = "x < y";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Lt),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // より大きい
    {
        let input = "x > y";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Gt),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 以下
    {
        let input = "x <= y";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Lte),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 以上
    {
        let input = "x >= y";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Gte),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
}

#[test]
fn test_parse_parenthesized_expr() {
    let input = "(1 + 2) * 3";
    let mut parser = Parser::from_str(input, None).unwrap();
    let expr = parser.parse_expression().unwrap();
    
    match expr {
        Expr::BinaryOp { operator, left, right, .. } => {
            assert_eq!(operator, BinaryOperator::Mul);
            
            match &*left {
                Expr::ParenExpr(inner, _) => {
                    match &**inner {
                        Expr::BinaryOp { operator, .. } => assert_eq!(*operator, BinaryOperator::Add),
                        _ => panic!("カッコ内の式が期待される二項演算ではありません"),
                    }
                },
                _ => panic!("期待されるカッコ式ではありません"),
            }
            
            match &*right {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 3),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待される二項演算ではありません"),
    }
}

#[test]
fn test_parse_let_with_type_annotation() {
    let input = "let x: Int = 42;";
    let mut parser = Parser::from_str(input, None).unwrap();
    let program = parser.parse_program().unwrap();
    
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
fn test_parse_complex_expression() {
    let input = "1 + 2 * 3 + 4";
    let mut parser = Parser::from_str(input, None).unwrap();
    let expr = parser.parse_expression().unwrap();
    
    match expr {
        Expr::BinaryOp { operator, left, right, .. } => {
            assert_eq!(operator, BinaryOperator::Add);
            
            match &*left {
                Expr::BinaryOp { operator, left, right, .. } => {
                    assert_eq!(*operator, BinaryOperator::Add);
                    
                    match &**left {
                        Expr::IntLiteral(value, _) => assert_eq!(value, &1),
                        _ => panic!("期待される整数リテラルではありません"),
                    }
                    
                    match &**right {
                        Expr::BinaryOp { operator, left, right, .. } => {
                            assert_eq!(*operator, BinaryOperator::Mul);
                            
                            match &**left {
                                Expr::IntLiteral(value, _) => assert_eq!(value, &2),
                                _ => panic!("期待される整数リテラルではありません"),
                            }
                            
                            match &**right {
                                Expr::IntLiteral(value, _) => assert_eq!(value, &3),
                                _ => panic!("期待される整数リテラルではありません"),
                            }
                        },
                        _ => panic!("期待される二項演算ではありません"),
                    }
                },
                _ => panic!("期待される二項演算ではありません"),
            }
            
            match &*right {
                Expr::IntLiteral(value, _) => assert_eq!(value, &4),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待される二項演算ではありません"),
    }
}

#[test]
fn test_parse_error_unexpected_token() {
    let input = "let x = ;";
    let mut parser = Parser::from_str(input, None).unwrap();
    let result = parser.parse_program();
    
    assert!(result.is_err());
    
    match result {
        Ok(_) => panic!("エラーが期待されます"),
        Err(err) => {
            match err.kind {
                ErrorKind::Syntax(_) => (), // 期待される構文エラー
                _ => panic!("期待される構文エラーではありません"),
            }
        }
    }
}

#[test]
fn test_parse_error_unexpected_eof() {
    let input = "let x =";
    let mut parser = Parser::from_str(input, None).unwrap();
    let result = parser.parse_program();
    
    assert!(result.is_err());
    
    match result {
        Ok(_) => panic!("エラーが期待されます"),
        Err(err) => {
            match err.kind {
                ErrorKind::Syntax(msg) => assert!(msg.contains("式が期待されます")),
                _ => panic!("期待される構文エラーではありません"),
            }
        }
    }
}

#[test]
fn test_parse_string_literal() {
    let input = r#"let message = "Hello, world!";"#;
    let mut parser = Parser::from_str(input, None).unwrap();
    let program = parser.parse_program().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Let { name, value, .. } => {
            assert_eq!(name, "message");
            
            match value {
                Expr::StringLiteral(value, _) => assert_eq!(value, "Hello, world!"),
                _ => panic!("期待される文字列リテラルではありません"),
            }
        },
        _ => panic!("期待されるlet文ではありません"),
    }
}

#[test]
fn test_parse_bool_literal() {
    // true
    {
        let input = "true";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::BoolLiteral(value, _) => assert_eq!(value, true),
            _ => panic!("期待される真偽値リテラルではありません"),
        }
    }
    
    // false
    {
        let input = "false";
        let mut parser = Parser::from_str(input, None).unwrap();
        let expr = parser.parse_expression().unwrap();
        
        match expr {
            Expr::BoolLiteral(value, _) => assert_eq!(value, false),
            _ => panic!("期待される真偽値リテラルではありません"),
        }
    }
}
