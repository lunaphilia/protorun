// ASTモジュールのテスト

use super::*;

#[test]
fn test_span_creation() {
    let span = Span {
        start: 0,
        end: 5,
        line: 1,
        column: 1,
    };
    
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 5);
    assert_eq!(span.line, 1);
    assert_eq!(span.column, 1);
}

#[test]
fn test_expr_int_literal() {
    let span = Span {
        start: 0,
        end: 2,
        line: 1,
        column: 1,
    };
    
    let expr = Expr::IntLiteral(42, span.clone());
    
    match expr {
        Expr::IntLiteral(value, expr_span) => {
            assert_eq!(value, 42);
            assert_eq!(expr_span, span);
        }
        _ => panic!("期待される整数リテラルではありません"),
    }
}

#[test]
fn test_expr_float_literal() {
    let span = Span {
        start: 0,
        end: 4,
        line: 1,
        column: 1,
    };
    
    let expr = Expr::FloatLiteral(3.14, span.clone());
    
    match expr {
        Expr::FloatLiteral(value, expr_span) => {
            assert_eq!(value, 3.14);
            assert_eq!(expr_span, span);
        }
        _ => panic!("期待される浮動小数点リテラルではありません"),
    }
}

#[test]
fn test_expr_bool_literal() {
    let span = Span {
        start: 0,
        end: 4,
        line: 1,
        column: 1,
    };
    
    let expr_true = Expr::BoolLiteral(true, span.clone());
    let expr_false = Expr::BoolLiteral(false, span.clone());
    
    match expr_true {
        Expr::BoolLiteral(value, expr_span) => {
            assert_eq!(value, true);
            assert_eq!(expr_span, span);
        }
        _ => panic!("期待される真偽値リテラルではありません"),
    }
    
    match expr_false {
        Expr::BoolLiteral(value, expr_span) => {
            assert_eq!(value, false);
            assert_eq!(expr_span, span);
        }
        _ => panic!("期待される真偽値リテラルではありません"),
    }
}

#[test]
fn test_expr_string_literal() {
    let span = Span {
        start: 0,
        end: 10,
        line: 1,
        column: 1,
    };
    
    let expr = Expr::StringLiteral("Hello".to_string(), span.clone());
    
    match expr {
        Expr::StringLiteral(value, expr_span) => {
            assert_eq!(value, "Hello");
            assert_eq!(expr_span, span);
        }
        _ => panic!("期待される文字列リテラルではありません"),
    }
}

#[test]
fn test_expr_identifier() {
    let span = Span {
        start: 0,
        end: 5,
        line: 1,
        column: 1,
    };
    
    let expr = Expr::Identifier("x".to_string(), span.clone());
    
    match expr {
        Expr::Identifier(name, expr_span) => {
            assert_eq!(name, "x");
            assert_eq!(expr_span, span);
        }
        _ => panic!("期待される識別子ではありません"),
    }
}

#[test]
fn test_expr_binary_op() {
    let span1 = Span {
        start: 0,
        end: 1,
        line: 1,
        column: 1,
    };
    
    let span2 = Span {
        start: 2,
        end: 3,
        line: 1,
        column: 3,
    };
    
    let span_op = Span {
        start: 0,
        end: 3,
        line: 1,
        column: 1,
    };
    
    let left = Box::new(Expr::IntLiteral(10, span1));
    let right = Box::new(Expr::IntLiteral(20, span2));
    
    let expr = Expr::BinaryOp {
        left,
        operator: BinaryOperator::Add,
        right,
        span: span_op.clone(),
    };
    
    match expr {
        Expr::BinaryOp { left, operator, right, span } => {
            assert_eq!(operator, BinaryOperator::Add);
            assert_eq!(span, span_op);
            
            match *left {
                Expr::IntLiteral(value, _) => assert_eq!(value, 10),
                _ => panic!("期待される整数リテラルではありません"),
            }
            
            match *right {
                Expr::IntLiteral(value, _) => assert_eq!(value, 20),
                _ => panic!("期待される整数リテラルではありません"),
            }
        }
        _ => panic!("期待される二項演算ではありません"),
    }
}

#[test]
fn test_expr_unary_op() {
    let span = Span {
        start: 0,
        end: 2,
        line: 1,
        column: 1,
    };
    
    let span_op = Span {
        start: 0,
        end: 2,
        line: 1,
        column: 1,
    };
    
    let expr_inner = Box::new(Expr::IntLiteral(42, span));
    
    let expr = Expr::UnaryOp {
        operator: UnaryOperator::Neg,
        expr: expr_inner,
        span: span_op.clone(),
    };
    
    match expr {
        Expr::UnaryOp { operator, expr, span } => {
            assert_eq!(operator, UnaryOperator::Neg);
            assert_eq!(span, span_op);
            
            match *expr {
                Expr::IntLiteral(value, _) => assert_eq!(value, 42),
                _ => panic!("期待される整数リテラルではありません"),
            }
        }
        _ => panic!("期待される単項演算ではありません"),
    }
}

#[test]
fn test_expr_function_call() {
    let span_func = Span {
        start: 0,
        end: 3,
        line: 1,
        column: 1,
    };
    
    let span_arg = Span {
        start: 4,
        end: 5,
        line: 1,
        column: 5,
    };
    
    let span_call = Span {
        start: 0,
        end: 6,
        line: 1,
        column: 1,
    };
    
    let func = Box::new(Expr::Identifier("foo".to_string(), span_func));
    let arg = Expr::IntLiteral(42, span_arg);
    
    let expr = Expr::FunctionCall {
        function: func,
        arguments: vec![arg],
        span: span_call.clone(),
    };
    
    match expr {
        Expr::FunctionCall { function, arguments, span } => {
            assert_eq!(span, span_call);
            assert_eq!(arguments.len(), 1);
            
            match *function {
                Expr::Identifier(name, _) => assert_eq!(name, "foo"),
                _ => panic!("期待される識別子ではありません"),
            }
            
            match &arguments[0] {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 42),
                _ => panic!("期待される整数リテラルではありません"),
            }
        }
        _ => panic!("期待される関数呼び出しではありません"),
    }
}

#[test]
fn test_stmt_let() {
    let span_let = Span {
        start: 0,
        end: 10,
        line: 1,
        column: 1,
    };
    
    let span_expr = Span {
        start: 6,
        end: 8,
        line: 1,
        column: 7,
    };
    
    let value = Expr::IntLiteral(42, span_expr);
    
    let stmt = Stmt::Let {
        name: "x".to_string(),
        type_annotation: None,
        value,
        span: span_let.clone(),
    };
    
    match stmt {
        Stmt::Let { name, type_annotation, value, span } => {
            assert_eq!(name, "x");
            assert_eq!(type_annotation, None);
            assert_eq!(span, span_let);
            
            match value {
                Expr::IntLiteral(val, _) => assert_eq!(val, 42),
                _ => panic!("期待される整数リテラルではありません"),
            }
        }
        _ => panic!("期待されるlet文ではありません"),
    }
}

#[test]
fn test_decl_function() {
    let span_func = Span {
        start: 0,
        end: 20,
        line: 1,
        column: 1,
    };
    
    let span_param = Span {
        start: 4,
        end: 5,
        line: 1,
        column: 5,
    };
    
    let span_body = Span {
        start: 15,
        end: 16,
        line: 1,
        column: 16,
    };
    
    let parameter = Parameter {
        name: "x".to_string(),
        type_annotation: None,
        span: span_param,
    };
    
    let body = Expr::Identifier("x".to_string(), span_body);
    
    let decl = Decl::Function {
        name: "foo".to_string(),
        parameters: vec![parameter],
        return_type: None,
        body,
        span: span_func.clone(),
    };
    
    match decl {
        Decl::Function { name, parameters, return_type, body, span } => {
            assert_eq!(name, "foo");
            assert_eq!(parameters.len(), 1);
            assert_eq!(return_type, None);
            assert_eq!(span, span_func);
            
            assert_eq!(parameters[0].name, "x");
            
            match body {
                Expr::Identifier(name, _) => assert_eq!(name, "x"),
                _ => panic!("期待される識別子ではありません"),
            }
        }
    }
}

#[test]
fn test_program() {
    let span_func = Span {
        start: 0,
        end: 15,
        line: 1,
        column: 1,
    };
    
    let span_stmt = Span {
        start: 16,
        end: 25,
        line: 2,
        column: 1,
    };
    
    let decl = Decl::Function {
        name: "foo".to_string(),
        parameters: vec![],
        return_type: None,
        body: Expr::IntLiteral(42, span_func.clone()),
        span: span_func,
    };
    
    let stmt = Stmt::Let {
        name: "x".to_string(),
        type_annotation: None,
        value: Expr::IntLiteral(10, span_stmt.clone()),
        span: span_stmt,
    };
    
    let program = Program {
        declarations: vec![decl],
        statements: vec![stmt],
    };
    
    assert_eq!(program.declarations.len(), 1);
    assert_eq!(program.statements.len(), 1);
    
    match &program.declarations[0] {
        Decl::Function { name, .. } => assert_eq!(name, "foo"),
    }
    
    match &program.statements[0] {
        Stmt::Let { name, .. } => assert_eq!(name, "x"),
        _ => panic!("期待されるlet文ではありません"),
    }
}
