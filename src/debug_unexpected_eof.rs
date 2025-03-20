// 予期せぬEOFのエラーメッセージを出力するデバッグプログラム
mod protorun;

fn main() {
    use protorun::parser::Parser;
    
    let input = "let x =";
    let mut parser = Parser::from_str(input, None).unwrap();
    let result = parser.parse_program();
    
    match result {
        Ok(_) => println!("成功（予期しない）"),
        Err(err) => println!("エラーメッセージ: {:?}", err),
    }
}
