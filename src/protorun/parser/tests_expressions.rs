// 式パーサーのテスト

use super::*;
// BlockItem をインポート
use crate::protorun::ast::{BinaryOperator, Expr, UnaryOperator, ComprehensionKind, Pattern as AstPattern, LiteralValue, BlockItem};

#[test]
fn test_parse_block_expr() {
    let input = "{ let x = 10 \n x }"; // セミコロンを削除し、改行を追加
    let mut parser = Parser::new(None);

    let expr = parser.parse_expression(input).unwrap();

    match expr {
        // ブロック式全体が返されることを期待
        Expr::BlockExpr { items, .. } => { // final_expr を削除
            // 最後の要素が Expression(Identifier("x")) であることを確認
            assert!(items.len() > 0);
            match items.last().unwrap() {
                BlockItem::Expression(expr) => {
                     match expr {
                         Expr::Identifier(name, _) => assert_eq!(name, "x"),
                         _ => panic!("Block final item is not Identifier"),
                     }
                },
                _ => panic!("Block final item is not Expression"),
            }
            // items の残りの要素（let 宣言）を確認
            assert_eq!(items.len(), 2); // let と x があるはず
            match &items[0] {
                BlockItem::Declaration(decl) => {
                    match decl {
                        crate::protorun::ast::Decl::Let { pattern, value, .. } => {
                             match pattern {
                                crate::protorun::ast::Pattern::Identifier(name, _) => assert_eq!(name, "x"),
                                _ => panic!("Expected identifier pattern"),
                             }
                             match value {
                                Expr::IntLiteral(val, _) => assert_eq!(*val, 10),
                                _ => panic!("Expected IntLiteral"),
                             }
                        },
                        _ => panic!("Expected Let declaration"),
                    }
                },
                _ => panic!("Expected Declaration item"),
            }
        }
        _ => panic!("Expected BlockExpr"),
    }
}

#[test]
fn test_parse_nested_block_expr() {
    let input = "{ let x = 10 \n { let y = 20 \n x + y } }"; // セミコロンを削除し、改行を追加
    let mut parser = Parser::new(None);

    let expr = parser.parse_expression(input).unwrap();
    // アサーション修正: BlockExpr { items: [Let x, BlockExpr { items: [Let y, Expr(BinaryOp)] }] } を期待
    match expr {
        Expr::BlockExpr { items: outer_items, .. } => { // final_expr を削除
            assert_eq!(outer_items.len(), 2); // let x と 内側ブロック
            match &outer_items[1] { // 最後の要素が内側ブロック
                BlockItem::Expression(outer_final_expr) => {
                     match outer_final_expr {
                         Expr::BlockExpr { items: inner_items, .. } => { // final_expr を削除
                            assert_eq!(inner_items.len(), 2); // let y と x + y
                            match &inner_items[1] { // 最後の要素が二項演算
                                BlockItem::Expression(inner_final_expr) => {
                                     match inner_final_expr {
                                         Expr::BinaryOp { operator, .. } => assert_eq!(*operator, BinaryOperator::Add), // x + y
                                         _ => panic!("Inner block final item is not BinaryOp"),
                                     }
                                },
                                _ => panic!("Inner block final item is not Expression"),
                            }
                        },
                        _ => panic!("Outer block final item is not BlockExpr"),
                    }
                },
                 _ => panic!("Outer block final item is not Expression"),
            }
        },
        _ => panic!("Expected outer BlockExpr"),
    }
}

#[test]
fn test_parse_function_call() {
    let input = "add(10, 20)";
    let mut parser = Parser::new(None);
    
    let expr = parser.parse_expression(input).unwrap();
    
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
    let mut parser = Parser::new(None);
    
    let expr = parser.parse_expression(input).unwrap();
    
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
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Add),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 減算
    {
        let input = "5 - 3";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Sub),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 乗算
    {
        let input = "4 * 2";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Mul),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 除算
    {
        let input = "10 / 2";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Div),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 剰余
    {
        let input = "10 % 3";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
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
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
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
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
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
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Eq),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 非等価
    {
        let input = "x != y";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Neq),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // より小さい
    {
        let input = "x < y";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Lt),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // より大きい
    {
        let input = "x > y";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Gt),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 以下
    {
        let input = "x <= y";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Lte),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 以上
    {
        let input = "x >= y";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Gte),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
}

#[test]
fn test_parse_parenthesized_expr() {
    let input = "(1 + 2) * 3";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
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
fn test_parse_complex_expression() {
    let input = "1 + 2 * 3 + 4";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
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
fn test_parse_if_expr() {
    // 基本的なif式
    {
        let input = "if x > 0 { 42 } else { -42 }";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::IfExpr { condition, then_branch, else_branch, .. } => {
                match *condition {
                    Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Gt),
                    _ => panic!("期待される二項演算ではありません"),
                }

                // then_branch が BlockExpr { items: [Expression(IntLiteral(42))] } であることを確認
                match *then_branch {
                    Expr::BlockExpr{ items, .. } => { // final_expr を削除
                        assert_eq!(items.len(), 1);
                        match &items[0] {
                            BlockItem::Expression(expr) => {
                                match expr {
                                    Expr::IntLiteral(v, _) => assert_eq!(*v, 42),
                                    _ => panic!("then_branch の item が IntLiteral(42) ではありません"),
                                }
                            },
                            _ => panic!("then_branch の item が Expression ではありません"),
                        }
                    },
                    _ => panic!("then_branch が BlockExpr ではありません"),
                }

                // else_branch が Some(BlockExpr { items: [Expression(UnaryOp { op: Neg, expr: IntLiteral(42) })] }) であることを確認
                assert!(else_branch.is_some());
                match *else_branch.unwrap() {
                    Expr::BlockExpr{ items, .. } => { // final_expr を削除
                         assert_eq!(items.len(), 1);
                         match &items[0] {
                             BlockItem::Expression(expr) => {
                                 match expr {
                                     Expr::UnaryOp{ operator, expr: inner_expr, .. } => {
                                         assert_eq!(*operator, UnaryOperator::Neg);
                                         match &**inner_expr { // Box なので参照外し
                                             Expr::IntLiteral(v, _) => assert_eq!(*v, 42),
                                             _ => panic!("else_branch の UnaryOp の内部が IntLiteral(42) ではありません"),
                                         }
                                     },
                                     _ => panic!("else_branch の item が UnaryOp ではありません"),
                                 }
                             },
                             _ => panic!("else_branch の item が Expression ではありません"),
                         }
                    },
                    _ => panic!("else_branch が BlockExpr ではありません"),
                }
            },
            _ => panic!("期待されるif式ではありません"),
        }
    }
    
    // else部がないif式
    {
        let input = "if x > 0 { 42 }";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();

        match expr {
            Expr::IfExpr { condition, then_branch, else_branch, .. } => {
                match *condition {
                    Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Gt),
                    _ => panic!("期待される二項演算ではありません"),
                }

                // then_branch が BlockExpr { items: [Expression(IntLiteral(42))] } であることを確認
                match *then_branch {
                     Expr::BlockExpr{ items, .. } => { // final_expr を削除
                        assert_eq!(items.len(), 1);
                        match &items[0] {
                            BlockItem::Expression(expr) => {
                                match expr {
                                    Expr::IntLiteral(v, _) => assert_eq!(*v, 42),
                                    _ => panic!("then_branch の item が IntLiteral(42) ではありません"),
                                }
                            },
                            _ => panic!("then_branch の item が Expression ではありません"),
                        }
                    },
                    _ => panic!("then_branch が BlockExpr ではありません"),
                }

                assert!(else_branch.is_none());
            },
            _ => panic!("期待されるif式ではありません"),
        }
    }
    
    // ネストされたif式
    {
        let input = "if x > 0 { 42 } else if x < 0 { -42 } else { 0 }";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();

        match expr {
            Expr::IfExpr { condition, then_branch, else_branch, .. } => {
                match *condition {
                    Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Gt),
                    _ => panic!("期待される二項演算ではありません"),
                }

                // then_branch が BlockExpr { items: [Expression(IntLiteral(42))] } であることを確認
                match *then_branch {
                     Expr::BlockExpr{ items, .. } => { // final_expr を削除
                        assert_eq!(items.len(), 1);
                        match &items[0] {
                            BlockItem::Expression(expr) => {
                                match expr {
                                    Expr::IntLiteral(v, _) => assert_eq!(*v, 42),
                                    _ => panic!("then_branch の item が IntLiteral(42) ではありません"),
                                }
                            },
                            _ => panic!("then_branch の item が Expression ではありません"),
                        }
                    },
                    _ => panic!("then_branch が BlockExpr ではありません"),
                }

                match else_branch {
                    Some(else_expr) => {
                        match *else_expr {
                            Expr::IfExpr { .. } => (), // ネストされたif式, 中身のチェックは省略
                            _ => panic!("期待されるif式ではありません"),
                        }
                    },
                    None => panic!("else部が期待されます"),
                }
            },
            _ => panic!("期待されるif式ではありません"),
        }
    }
}

#[test]
fn test_parse_match_expr() {
    let input = "match x { 
        0 => 42, 
        n if n > 0 => n * 2, 
        _ => -1 
    }";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::MatchExpr { scrutinee, cases, .. } => {
            match *scrutinee {
                Expr::Identifier(ref name, _) => assert_eq!(name, "x"),
                _ => panic!("期待される識別子ではありません"),
            }
            
            assert_eq!(cases.len(), 3);
            
            // 最初のケース: 0 => 42
            match &cases[0] {
                (pattern, guard, expr) => {
                    match pattern {
                        AstPattern::Literal(LiteralValue::Int(value), _) => assert_eq!(*value, 0),
                        _ => panic!("期待されるリテラルパターンではありません"),
                    }
                    
                    assert!(guard.is_none());
                    
                    match expr {
                        Expr::IntLiteral(value, _) => assert_eq!(*value, 42),
                        _ => panic!("期待される整数リテラルではありません"),
                    }
                }
            }
            
            // 2番目のケース: n if n > 0 => n * 2
            match &cases[1] {
                (pattern, guard, expr) => {
                    match pattern {
                        AstPattern::Identifier(name, _) => assert_eq!(name, "n"),
                        _ => panic!("期待される識別子パターンではありません"),
                    }
                    
                    assert!(guard.is_some());
                    
                    match expr {
                        Expr::BinaryOp { operator, .. } => assert_eq!(*operator, BinaryOperator::Mul),
                        _ => panic!("期待される二項演算ではありません"),
                    }
                }
            }
            
            // 3番目のケース: _ => -1
            match &cases[2] {
                (pattern, guard, expr) => {
                    match pattern {
                        AstPattern::Wildcard(_) => (),
                        _ => panic!("期待されるワイルドカードパターンではありません"),
                    }
                    
                    assert!(guard.is_none());
                    
                    match expr {
                        Expr::UnaryOp { operator, expr, .. } => {
                            assert_eq!(*operator, UnaryOperator::Neg);
                            match &**expr {
                                Expr::IntLiteral(value, _) => assert_eq!(*value, 1),
                                _ => panic!("期待される整数リテラルではありません"),
                            }
                        },
                        _ => panic!("期待される単項演算ではありません"),
                    }
                }
            }
        },
        _ => panic!("期待されるmatch式ではありません"),
    }
}

#[test]
fn test_parse_list_comprehension() {
    let input = "[x * 2 for x <- numbers if x > 0]";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::CollectionComprehension { kind, output_expr, input_expr, pattern, condition, .. } => {
            assert_eq!(kind, ComprehensionKind::List);
            
            match *output_expr {
                Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Mul),
                _ => panic!("期待される二項演算ではありません"),
            }
            
            match *input_expr {
                Expr::Identifier(ref name, _) => assert_eq!(name, "numbers"),
                _ => panic!("期待される識別子ではありません"),
            }
            
            match pattern {
                AstPattern::Identifier(ref name, _) => assert_eq!(name, "x"),
                _ => panic!("期待される識別子パターンではありません"),
            }
            
            assert!(condition.is_some());
            match *condition.unwrap() {
                Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Gt),
                _ => panic!("期待される二項演算ではありません"),
            }
        },
        _ => panic!("期待されるコレクション内包表記ではありません"),
    }
}

#[test]
fn test_parse_map_comprehension() {
    let input = "{k -> v * 2 for (k, v) <- entries}";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::CollectionComprehension { kind, output_expr, input_expr, pattern, .. } => {
            assert_eq!(kind, ComprehensionKind::Map);
            
            // 出力式はキーと値のペアを表すタプル式
            match *output_expr {
                Expr::ParenExpr(_, _) => (),
                _ => panic!("期待されるタプル式ではありません"),
            }
            
            match *input_expr {
                Expr::Identifier(ref name, _) => assert_eq!(name, "entries"),
                _ => panic!("期待される識別子ではありません"),
            }
            
            match pattern {
                AstPattern::Tuple(patterns, _) => {
                    assert_eq!(patterns.len(), 2);
                    
                    match &patterns[0] {
                        AstPattern::Identifier(name, _) => assert_eq!(name, "k"),
                        _ => panic!("期待される識別子パターンではありません"),
                    }
                    
                    match &patterns[1] {
                        AstPattern::Identifier(name, _) => assert_eq!(name, "v"),
                        _ => panic!("期待される識別子パターンではありません"),
                    }
                },
                _ => panic!("期待されるタプルパターンではありません"),
            }
        },
        _ => panic!("期待されるコレクション内包表記ではありません"),
    }
}

#[test]
fn test_parse_bind_expr() {
    // bind 式内のセミコロンは必要
    let input = "bind { \n x <- getX(); \n y <- getY(); \n x + y \n }"; // 改行追加
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::BindExpr { bindings, final_expr, .. } => {
            assert_eq!(bindings.len(), 2);
            
            // 最初のバインド: x <- getX()
            match &bindings[0] {
                (pattern, expr) => {
                    match pattern {
                        AstPattern::Identifier(name, _) => assert_eq!(name, "x"),
                        _ => panic!("期待される識別子パターンではありません"),
                    }
                    
                    match expr {
                        Expr::FunctionCall { .. } => (),
                        _ => panic!("期待される関数呼び出しではありません"),
                    }
                }
            }
            
            // 2番目のバインド: y <- getY()
            match &bindings[1] {
                (pattern, expr) => {
                    match pattern {
                        AstPattern::Identifier(name, _) => assert_eq!(name, "y"),
                        _ => panic!("期待される識別子パターンではありません"),
                    }
                    
                    match expr {
                        Expr::FunctionCall { .. } => (),
                        _ => panic!("期待される関数呼び出しではありません"),
                    }
                }
            }
            
            // 最終式: x + y
            match *final_expr {
                Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Add),
                _ => panic!("期待される二項演算ではありません"),
            }
        },
        _ => panic!("期待されるbind式ではありません"),
    }
}

#[test]
fn test_parse_with_expr() {
    // 式としてのハンドラ
    {
        let input = "with logger { \n log(\"Hello\") \n 42 \n }"; // セミコロン削除、改行追加
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();

        match expr {
            Expr::WithExpr { handler, effect_type, body, .. } => {
                // ハンドラが式 (Identifier) であることを確認
                match *handler { // handler は Box<Expr> なので参照外し
                     Expr::Identifier(name, _) => assert_eq!(name, "logger"),
                     _ => panic!("Handler expression is not Identifier"),
                }

                assert!(effect_type.is_none());

                // body が BlockExpr { items: [Expression(FunctionCall), Expression(IntLiteral)] } であることを確認
                match *body {
                     Expr::BlockExpr { items, .. } => { // final_expr を削除
                         assert_eq!(items.len(), 2); // log("Hello") と 42
                         match &items[0] {
                             BlockItem::Expression(expr) => {
                                 match expr {
                                     Expr::FunctionCall{ function, .. } => {
                                         match &**function {
                                             Expr::Identifier(name, _) => assert_eq!(name, "log"),
                                             _ => panic!("Expected log function call"),
                                         }
                                     },
                                     _ => panic!("First item is not FunctionCall"),
                                 }
                             },
                             _ => panic!("First item is not Expression"),
                         }
                         match &items[1] {
                             BlockItem::Expression(expr) => {
                                 match expr {
                                     Expr::IntLiteral(v, _) => assert_eq!(*v, 42),
                                     _ => panic!("Second item is not IntLiteral(42)"),
                                 }
                             },
                             _ => panic!("Second item is not Expression"),
                         }
                     },
                    _ => panic!("Body is not BlockExpr"),
                }
            },
            _ => panic!("期待されるwith式ではありません"),
        }
    }

    // 型としてのハンドラと効果型 (この構文は廃止されたのでコメントアウト)
    // {
    //     let input = "with Logger: IO { \n log(\"Hello\") \n 42 \n }"; // セミコロン削除、改行追加
    //     let mut parser = Parser::new(None);
    //     let expr = parser.parse_expression(input).unwrap();

    //     match expr {
    //         Expr::WithExpr { handler, effect_type, body, .. } => {
    //             // ハンドラが式 (Identifier("Logger")) としてパースされることを確認 (alt順序変更のため)
    //             match *handler {
    //                  Expr::Identifier(name, _) => assert_eq!(name, "Logger"),
    //                  _ => panic!("Handler expression is not Identifier"),
    //             }

    //             assert!(effect_type.is_some());
    //             match effect_type.unwrap() {
    //                 crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "IO"),
    //                 _ => panic!("期待される単純型ではありません"),
    //             }

    //             // body が BlockExpr { items: [Expression(FunctionCall), Expression(IntLiteral)] } であることを確認
    //             match *body {
    //                  Expr::BlockExpr { items, .. } => { // final_expr を削除
    //                      assert_eq!(items.len(), 2); // log("Hello") と 42
    //                      match &items[0] {
    //                          BlockItem::Expression(expr) => {
    //                              match expr {
    //                                  Expr::FunctionCall{ function, .. } => {
    //                                      match &**function {
    //                                          Expr::Identifier(name, _) => assert_eq!(name, "log"),
    //                                          _ => panic!("Expected log function call"),
    //                                      }
    //                                  },
    //                                  _ => panic!("First item is not FunctionCall"),
    //                              }
    //                          },
    //                          _ => panic!("First item is not Expression"),
    //                      }
    //                      match &items[1] {
    //                          BlockItem::Expression(expr) => {
    //                              match expr {
    //                                  Expr::IntLiteral(v, _) => assert_eq!(*v, 42),
    //                                  _ => panic!("Second item is not IntLiteral(42)"),
    //                              }
    //                          },
    //                          _ => panic!("Second item is not Expression"),
    //                      }
    //                  },
    //                 _ => panic!("Body is not BlockExpr"),
    //             }
    //         },
    //         _ => panic!("期待されるwith式ではありません"),
    //     }
    // }
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
fn test_parse_empty_collections() {
    // 空のリスト
    {
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
    
    // 空のマップ
    {
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
    
    // 空のセット
    {
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
}

#[test]
fn test_parse_nested_collections() {
    let input = "[[1, 2], [3, 4]]";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::ListLiteral { elements, .. } => {
            assert_eq!(elements.len(), 2);
            
            // 最初のネストされたリスト
            match &elements[0] {
                Expr::ListLiteral { elements: nested_elements, .. } => {
                    assert_eq!(nested_elements.len(), 2);
                    
                    match &nested_elements[0] {
                        Expr::IntLiteral(value, _) => assert_eq!(*value, 1),
                        _ => panic!("期待される整数リテラルではありません"),
                    }
                    
                    match &nested_elements[1] {
                        Expr::IntLiteral(value, _) => assert_eq!(*value, 2),
                        _ => panic!("期待される整数リテラルではありません"),
                    }
                },
                _ => panic!("期待されるリストリテラルではありません"),
            }
            
            // 2番目のネストされたリスト
            match &elements[1] {
                Expr::ListLiteral { elements: nested_elements, .. } => {
                    assert_eq!(nested_elements.len(), 2);
                    
                    match &nested_elements[0] {
                        Expr::IntLiteral(value, _) => assert_eq!(*value, 3),
                        _ => panic!("期待される整数リテラルではありません"),
                    }
                    
                    match &nested_elements[1] {
                        Expr::IntLiteral(value, _) => assert_eq!(*value, 4),
                        _ => panic!("期待される整数リテラルではありません"),
                    }
                },
                _ => panic!("期待されるリストリテラルではありません"),
            }
        },
        _ => panic!("期待されるリストリテラルではありません"),
    }
}

#[test]
fn test_parse_lambda_expr() {
    // 基本的なラムダ式
    {
        let input = "(x) => x + 1";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::LambdaExpr { parameters, body, .. } => {
                assert_eq!(parameters.len(), 1);
                
                assert_eq!(parameters[0].name, "x");
                assert!(parameters[0].type_annotation.is_none());
                
                match *body {
                    Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Add),
                    _ => panic!("期待される二項演算ではありません"),
                }
            },
            _ => panic!("期待されるラムダ式ではありません"),
        }
    }
    
    // 型注釈付きのパラメータを持つラムダ式
    {
        let input = "(x: Int) => x * 2";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::LambdaExpr { parameters, body, .. } => {
                assert_eq!(parameters.len(), 1);
                
                assert_eq!(parameters[0].name, "x");
                assert!(parameters[0].type_annotation.is_some());
                
                match &parameters[0].type_annotation {
                    Some(ty) => {
                        match ty {
                            crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "Int"),
                            _ => panic!("期待される単純型ではありません"),
                        }
                    },
                    None => panic!("型注釈が期待されます"),
                }
                
                match *body {
                    Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Mul),
                    _ => panic!("期待される二項演算ではありません"),
                }
            },
            _ => panic!("期待されるラムダ式ではありません"),
        }
    }
    
    // 複数のパラメータを持つラムダ式
    {
        let input = "(x: Int, y: Int) => x + y";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::LambdaExpr { parameters, body, .. } => {
                assert_eq!(parameters.len(), 2);
                
                assert_eq!(parameters[0].name, "x");
                assert_eq!(parameters[1].name, "y");
                
                match *body {
                    Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Add),
                    _ => panic!("期待される二項演算ではありません"),
                }
            },
            _ => panic!("期待されるラムダ式ではありません"),
        }
    }
    
    // 空のパラメータリストを持つラムダ式
    {
        let input = "() => 42";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::LambdaExpr { parameters, body, .. } => {
                assert_eq!(parameters.len(), 0);
                
                match *body {
                    Expr::IntLiteral(value, _) => assert_eq!(value, 42),
                    _ => panic!("期待される整数リテラルではありません"),
                }
            },
            _ => panic!("期待されるラムダ式ではありません"),
        }
    }
    
    // ブロック式を本体に持つラムダ式
    {
        let input = "(x) => { let y = x * 2 \n y + 1 }"; // セミコロンを削除し、改行を追加
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();

        match expr {
            Expr::LambdaExpr { parameters, body, .. } => {
                assert_eq!(parameters.len(), 1);
                assert_eq!(parameters[0].name, "x");

                // body が BlockExpr であることを確認
                match *body {
                    Expr::BlockExpr { items, .. } => { // final_expr を削除
                        assert_eq!(items.len(), 2); // let y = ... と y + 1
                        match &items[1] { // 最後の要素が BinaryOp
                            BlockItem::Expression(expr) => {
                                match expr {
                                     Expr::BinaryOp { operator, .. } => assert_eq!(*operator, BinaryOperator::Add), // y + 1
                                     _ => panic!("Lambda body final item is not BinaryOp"),
                                }
                            },
                            _ => panic!("Lambda body final item is not Expression"),
                        }
                    },
                    _ => panic!("Lambda body is not BlockExpr"),
                }
            },
            _ => panic!("期待されるラムダ式ではありません"),
        }
    }
}

#[test]
fn test_parse_member_access() {
    // 基本的なメンバーアクセス
    {
        let input = "obj.property";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::MemberAccess { object, member, .. } => {
                match *object {
                    Expr::Identifier(ref name, _) => assert_eq!(name, "obj"),
                    _ => panic!("期待される識別子ではありません"),
                }
                
                assert_eq!(member, "property");
            },
            _ => panic!("期待されるメンバーアクセスではありません"),
        }
    }
    
    // チェーンされたメンバーアクセス
    {
        let input = "obj.inner.property";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::MemberAccess { object, member, .. } => {
                assert_eq!(member, "property");
                
                match *object {
                    Expr::MemberAccess { object: ref inner_obj, member: ref inner_member, .. } => {
                        match **inner_obj {
                            Expr::Identifier(ref name, _) => assert_eq!(name, "obj"),
                            _ => panic!("期待される識別子ではありません"),
                        }
                        
                        assert_eq!(*inner_member, "inner");
                    },
                    _ => panic!("期待されるメンバーアクセスではありません"),
                }
            },
            _ => panic!("期待されるメンバーアクセスではありません"),
        }
    }
    
    // メンバーアクセス後の関数呼び出し
    {
        let input = "obj.method(42)";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::FunctionCall { function, arguments, .. } => {
                match *function {
                    Expr::MemberAccess { object, member, .. } => {
                        match *object {
                            Expr::Identifier(ref name, _) => assert_eq!(name, "obj"),
                            _ => panic!("期待される識別子ではありません"),
                        }
                        
                        assert_eq!(member, "method");
                    },
                    _ => panic!("期待されるメンバーアクセスではありません"),
                }
                
                assert_eq!(arguments.len(), 1);
                match &arguments[0] {
                    Expr::IntLiteral(value, _) => assert_eq!(*value, 42),
                    _ => panic!("期待される整数リテラルではありません"),
                }
            },
            _ => panic!("期待される関数呼び出しではありません"),
        }
    }
    
    // 複雑な式の結果に対するメンバーアクセス
    {
        let input = "(obj.get_inner()).property";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::MemberAccess { object, member, .. } => {
                assert_eq!(member, "property");
                
                match *object {
                    Expr::ParenExpr(ref inner, _) => {
                        match **inner {
                            Expr::FunctionCall { .. } => (), // 関数呼び出しの詳細は省略
                            _ => panic!("期待される関数呼び出しではありません"),
                        }
                    },
                    _ => panic!("期待される括弧式ではありません"),
                }
            },
            _ => panic!("期待されるメンバーアクセスではありません"),
        }
    }
}
