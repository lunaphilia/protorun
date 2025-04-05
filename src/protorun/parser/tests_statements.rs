// 文パーサーのテスト (現在は Return 文のみ)

use super::*;
use crate::protorun::ast::{Expr}; // Stmt を削除

#[test]
fn test_parse_top_level_expression() { // 関数名を変更
    let input = "42"; // セミコロンを削除 (トップレベル式には不要)
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();

    assert_eq!(program.declarations.len(), 0);
    assert_eq!(program.expressions.len(), 1); // statements -> expressions

    match &program.expressions[0] { // statements -> expressions
        // Stmt::Expr ではなく直接 Expr をチェック
        Expr::IntLiteral(value, _) => assert_eq!(*value, 42),
        _ => panic!("期待される整数リテラルではありません"),
    }
}

// test_parse_let_with_type_annotation を削除
// test_parse_var_statement を削除
// test_parse_var_with_type_annotation を削除
// test_parse_return_statement を削除 (トップレベル return はエラー)
// test_parse_complex_return_statement を削除 (トップレベル return はエラー)
