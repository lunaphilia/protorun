mod protorun;

use protorun::ast::{Program, Decl, Stmt, Expr};
use protorun::error::Result;
use protorun::parser::Parser;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("使用法: {} <ファイル名> または --repl", args[0]);
        process::exit(1);
    }

    let result = if args[1] == "--repl" {
        run_repl()
    } else {
        let path = Path::new(&args[1]);
        run_file(path)
    };

    if let Err(err) = result {
        eprintln!("エラー: {}", err);
        process::exit(1);
    }
}

// ファイルを解析して実行する
fn run_file(path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)
        .map_err(|e| protorun::error::Error::other(
            format!("ファイルの読み込みエラー: {}", e),
            None,
            Some(path.to_string_lossy().to_string()),
        ))?;

    let filename = path.to_string_lossy().to_string();
    let mut parser = Parser::from_str(&content, Some(filename))?;
    let program = parser.parse_program()?;

    println!("解析成功！");
    print_program(&program);

    Ok(())
}

// REPLモード
fn run_repl() -> Result<()> {
    println!("Protorun REPL（対話モード）");
    println!("終了するには 'exit' と入力してください。");

    let mut input = String::new();
    
    loop {
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        input.clear();
        if std::io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let input = input.trim();
        if input == "exit" {
            break;
        }

        match Parser::from_str(input, None) {
            Ok(mut parser) => {
                match parser.parse_program() {
                    Ok(program) => {
                        println!("解析成功！");
                        print_program(&program);
                    }
                    Err(e) => {
                        println!("解析エラー: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("字句解析エラー: {}", e);
            }
        }
    }

    Ok(())
}

// プログラムの内容を表示する
fn print_program(program: &Program) {
    println!("宣言数: {}", program.declarations.len());
    for (i, decl) in program.declarations.iter().enumerate() {
        println!("宣言 #{}: {}", i + 1, decl_to_string(decl));
    }

    println!("文数: {}", program.statements.len());
    for (i, stmt) in program.statements.iter().enumerate() {
        println!("文 #{}: {}", i + 1, stmt_to_string(stmt));
    }
}

// 宣言を文字列に変換
fn decl_to_string(decl: &Decl) -> String {
    match decl {
        Decl::Function { name, parameters, return_type, .. } => {
            let params: Vec<String> = parameters.iter()
                .map(|p| {
                    if let Some(t) = &p.type_annotation {
                        format!("{}: {}", p.name, type_to_string(t))
                    } else {
                        p.name.clone()
                    }
                })
                .collect();
            
            let ret_type = if let Some(t) = return_type {
                format!(": {}", type_to_string(t))
            } else {
                String::new()
            };
            
            format!("fn {}({}){}", name, params.join(", "), ret_type)
        }
    }
}

// 文を文字列に変換
fn stmt_to_string(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Let { name, type_annotation, value, .. } => {
            let type_str = if let Some(t) = type_annotation {
                format!(": {}", type_to_string(t))
            } else {
                String::new()
            };
            
            format!("let {}{} = {}", name, type_str, expr_to_string(value))
        }
        Stmt::Expr { expr, .. } => {
            expr_to_string(expr)
        }
    }
}

// 式を文字列に変換
fn expr_to_string(expr: &Expr) -> String {
    match expr {
        Expr::IntLiteral(value, _) => value.to_string(),
        Expr::FloatLiteral(value, _) => value.to_string(),
        Expr::BoolLiteral(value, _) => value.to_string(),
        Expr::StringLiteral(value, _) => format!("\"{}\"", value),
        Expr::UnitLiteral(_) => "()".to_string(),
        Expr::Identifier(name, _) => name.clone(),
        Expr::BinaryOp { left, operator, right, .. } => {
            format!("({} {} {})", 
                expr_to_string(left), 
                op_to_string(operator), 
                expr_to_string(right)
            )
        }
        Expr::UnaryOp { operator, expr, .. } => {
            format!("{}({})", 
                unary_op_to_string(operator), 
                expr_to_string(expr)
            )
        }
        Expr::FunctionCall { function, arguments, .. } => {
            let args: Vec<String> = arguments.iter()
                .map(expr_to_string)
                .collect();
            
            format!("{}({})", expr_to_string(function), args.join(", "))
        }
        Expr::ParenExpr(expr, _) => {
            format!("({})", expr_to_string(expr))
        }
    }
}

// 型を文字列に変換
fn type_to_string(typ: &protorun::ast::Type) -> String {
    match typ {
        protorun::ast::Type::Simple { name, .. } => name.clone(),
        protorun::ast::Type::Function { parameters, return_type, .. } => {
            let params: Vec<String> = parameters.iter()
                .map(type_to_string)
                .collect();
            
            format!("({}) -> {}", 
                params.join(", "), 
                type_to_string(return_type)
            )
        }
    }
}

// 二項演算子を文字列に変換
fn op_to_string(op: &protorun::ast::BinaryOperator) -> &'static str {
    match op {
        protorun::ast::BinaryOperator::Add => "+",
        protorun::ast::BinaryOperator::Sub => "-",
        protorun::ast::BinaryOperator::Mul => "*",
        protorun::ast::BinaryOperator::Div => "/",
        protorun::ast::BinaryOperator::Mod => "%",
        protorun::ast::BinaryOperator::Eq => "==",
        protorun::ast::BinaryOperator::Neq => "!=",
        protorun::ast::BinaryOperator::Lt => "<",
        protorun::ast::BinaryOperator::Gt => ">",
        protorun::ast::BinaryOperator::Lte => "<=",
        protorun::ast::BinaryOperator::Gte => ">=",
        protorun::ast::BinaryOperator::And => "&&",
        protorun::ast::BinaryOperator::Or => "||",
    }
}

// 単項演算子を文字列に変換
fn unary_op_to_string(op: &protorun::ast::UnaryOperator) -> &'static str {
    match op {
        protorun::ast::UnaryOperator::Neg => "-",
        protorun::ast::UnaryOperator::Not => "!",
    }
}
