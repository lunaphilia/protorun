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
fn test_decl_let() { // 関数名を変更
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
    
    let value = Expr::IntLiteral(42, span_expr.clone());
    let pattern = Pattern::Identifier("x".to_string(), span_expr); // name を Pattern に変更

    let decl = Decl::Let { // Stmt を Decl に変更
        pattern,
        type_annotation: None,
        value,
        span: span_let.clone(),
    };

    match decl {
        Decl::Let { pattern, type_annotation, value, span } => { // Stmt を Decl に変更
            // パターンのチェックを追加
            match pattern {
                Pattern::Identifier(name, _) => assert_eq!(name, "x"),
                _ => panic!("期待される識別子パターンではありません"),
            }
            assert_eq!(type_annotation, None);
            assert_eq!(span, span_let);

            match value {
                Expr::IntLiteral(val, _) => assert_eq!(val, 42),
                _ => panic!("期待される整数リテラルではありません"),
            }
        }
        _ => panic!("期待されるlet宣言ではありません"), // メッセージ変更
    }
}

#[test]
fn test_decl_var() { // var宣言のテストを追加
    let span_var = Span {
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

    let value = Expr::IntLiteral(0, span_expr);

    let decl = Decl::Var {
        name: "count".to_string(),
        type_annotation: None,
        value,
        span: span_var.clone(),
    };

    match decl {
        Decl::Var { name, type_annotation, value, span } => {
            assert_eq!(name, "count");
            assert_eq!(type_annotation, None);
            assert_eq!(span, span_var);

            match value {
                Expr::IntLiteral(val, _) => assert_eq!(val, 0),
                _ => panic!("期待される整数リテラルではありません"),
            }
        }
        _ => panic!("期待されるvar宣言ではありません"),
    }
}


#[test]
fn test_decl_function_as_let_lambda() { // 関数名を変更し、let + lambda で表現
    let span_let = Span { // let 全体のスパン
        start: 0,
        end: 20,
        line: 1,
        column: 1,
    };
    let span_pattern = Span { // パターン 'foo' のスパン
        start: 4,
        end: 7,
        line: 1,
        column: 5,
    };
    let span_lambda = Span { // ラムダ式全体のスパン
        start: 10,
        end: 20,
        line: 1,
        column: 11,
    };
    let span_param = Span { // パラメータ 'x' のスパン
        start: 13, // 'fn (x) = x' の 'x'
        end: 14,
        line: 1,
        column: 14,
    };
    let span_body = Span { // 本体 'x' のスパン
        start: 19, // 'fn (x) = x' の最後の 'x'
        end: 20,
        line: 1,
        column: 20,
    };

    let parameter = Parameter {
        name: "x".to_string(),
        type_annotation: None,
        span: span_param,
    };

    let body = Box::new(Expr::Identifier("x".to_string(), span_body));

    let function_expr = Expr::FunctionExpr { // Renamed from LambdaExpr
        parameters: Some(vec![parameter]), // Option<Vec<Parameter>> に変更
        effect_parameters: None, // 追加
        implicit_parameters: None, // 追加
        body,
        span: span_lambda.clone(),
    };

    let pattern = Pattern::Identifier("foo".to_string(), span_pattern);

    let decl = Decl::Let {
        pattern,
        type_annotation: None,
        value: function_expr, // Use the renamed variable
        span: span_let.clone(),
    };

    match decl {
        Decl::Let { pattern, type_annotation, value, span } => {
            assert_eq!(span, span_let);
            assert_eq!(type_annotation, None);

            match pattern {
                Pattern::Identifier(name, _) => assert_eq!(name, "foo"),
                _ => panic!("期待される識別子パターンではありません"),
            }

            match value {
                Expr::FunctionExpr { parameters, effect_parameters, implicit_parameters, body, span: lambda_span } => { // Renamed from LambdaExpr
                    assert_eq!(lambda_span, span_lambda);
                    assert!(parameters.is_some());
                    assert_eq!(parameters.as_ref().unwrap().len(), 1);
                    assert_eq!(parameters.as_ref().unwrap()[0].name, "x");
                    assert!(effect_parameters.is_none());
                    assert!(implicit_parameters.is_none());

                    match *body {
                        Expr::Identifier(name, _) => assert_eq!(name, "x"),
                        _ => panic!("期待される識別子ではありません"),
                    }
                },
                _ => panic!("期待される関数式ではありません"), // Message updated
            }
        },
        Decl::Var { .. } => panic!("期待される let 宣言ではありません (Var)"),
        Decl::HandlerDecl(_) => panic!("期待される let 宣言ではありません (HandlerDecl)"), // Added HandlerDecl arm
    }
}

#[test]
fn test_export_decl() {
    let span = Span {
        start: 0,
        end: 10,
        line: 1,
        column: 1,
    };

    // 個別エクスポートのテスト
    let single_export = ExportDecl::Single {
        name: "add".to_string(),
        span: span.clone(),
    };

    match single_export {
        ExportDecl::Single { name, span: export_span } => {
            assert_eq!(name, "add");
            assert_eq!(export_span, span);
        }
        _ => panic!("期待される個別エクスポートではありません"),
    }

    // グループエクスポートのテスト
    let group_export = ExportDecl::Group {
        names: vec!["add".to_string(), "subtract".to_string()],
        span: span.clone(),
    };

    match group_export {
        ExportDecl::Group { names, span: export_span } => {
            assert_eq!(names.len(), 2);
            assert_eq!(names[0], "add");
            assert_eq!(names[1], "subtract");
            assert_eq!(export_span, span);
        }
        _ => panic!("期待されるグループエクスポートではありません"),
    }
}

#[test]
fn test_import_decl() {
    let span = Span {
        start: 0,
        end: 15,
        line: 1,
        column: 1,
    };

    // 選択的インポートのテスト
    let selective_import = ImportDecl::Selective {
        module_path: "math".to_string(),
        imports: vec![
            ImportItem {
                name: "add".to_string(),
                alias: None,
                span: span.clone(),
            },
            ImportItem {
                name: "subtract".to_string(),
                alias: Some("sub".to_string()),
                span: span.clone(),
            },
        ],
        span: span.clone(),
    };

    match selective_import {
        ImportDecl::Selective { module_path, imports, span: import_span } => {
            assert_eq!(module_path, "math");
            assert_eq!(imports.len(), 2);
            assert_eq!(imports[0].name, "add");
            assert_eq!(imports[0].alias, None);
            assert_eq!(imports[1].name, "subtract");
            assert_eq!(imports[1].alias, Some("sub".to_string()));
            assert_eq!(import_span, span);
        }
        _ => panic!("期待される選択的インポートではありません"),
    }

    // モジュール全体のインポートのテスト
    let module_import = ImportDecl::Module {
        module_path: "math".to_string(),
        alias: "Math".to_string(),
        span: span.clone(),
    };

    match module_import {
        ImportDecl::Module { module_path, alias, span: import_span } => {
            assert_eq!(module_path, "math");
            assert_eq!(alias, "Math");
            assert_eq!(import_span, span);
        }
        _ => panic!("期待されるモジュールインポートではありません"),
    }
}

#[test]
fn test_module() {
    let span = Span {
        start: 0,
        end: 50,
        line: 1,
        column: 1,
    };

    let module = Module {
        path: "math".to_string(),
        exports: vec![
            ExportDecl::Single {
                name: "add".to_string(),
                span: span.clone(),
            },
        ],
        imports: vec![
            ImportDecl::Module {
                module_path: "core".to_string(),
                alias: "Core".to_string(),
                span: span.clone(),
            },
        ],
        declarations: Vec::new(),
        type_declarations: Vec::new(),
        trait_declarations: Vec::new(),
        impl_declarations: Vec::new(),
        // statements: Vec::new(), // 削除
        expressions: Vec::new(), // 追加
        span: span.clone(),
    };

    assert_eq!(module.path, "math");
    assert_eq!(module.exports.len(), 1);
    assert_eq!(module.imports.len(), 1);
}

#[test]
fn test_program() {
    let span = Span {
        start: 0,
        end: 100,
        line: 1,
        column: 1,
    };

    let math_module = Module {
        path: "math".to_string(),
        exports: vec![
            ExportDecl::Single {
                name: "add".to_string(),
                span: span.clone(),
            },
        ],
        imports: Vec::new(),
        declarations: Vec::new(),
        type_declarations: Vec::new(),
        trait_declarations: Vec::new(),
        impl_declarations: Vec::new(),
        // statements: Vec::new(), // 削除
        expressions: Vec::new(), // 追加
        span: span.clone(),
    };

    let program = Program {
        modules: vec![math_module],
        declarations: Vec::new(),
        type_declarations: Vec::new(),
        trait_declarations: Vec::new(),
        impl_declarations: Vec::new(),
        // statements: Vec::new(), // 削除
        expressions: Vec::new(), // 追加
    };

    assert_eq!(program.modules.len(), 1);
    assert_eq!(program.modules[0].path, "math");
    assert_eq!(program.modules[0].exports.len(), 1);
}
