// 共通ユーティリティのテスト

use super::*;

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
