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
    let mut parser = Parser::new(Some(filename));
    let program = parser.parse_program(&content)?;

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

        let mut parser = Parser::new(None);
        match parser.parse_program(input) {
            Ok(program) => {
                println!("解析成功！");
                print_program(&program);
            }
            Err(e) => {
                println!("解析エラー: {}", e);
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

    // println!("文数: {}", program.statements.len()); // 削除
    // for (i, stmt) in program.statements.iter().enumerate() { // 削除
    //     println!("文 #{}: {}", i + 1, stmt_to_string(stmt)); // 削除
    // }
    println!("トップレベル式数: {}", program.expressions.len()); // 追加
    for (i, expr) in program.expressions.iter().enumerate() { // 追加
        println!("式 #{}: {}", i + 1, expr_to_string(expr)); // 追加
    }
}

// 宣言を文字列に変換
fn decl_to_string(decl: &Decl) -> String {
    match decl {
        Decl::Let { pattern, type_annotation, value, .. } => {
            let type_str = if let Some(t) = type_annotation {
                format!(": {}", type_to_string(t))
            } else {
                String::new()
            };
            format!("let {}{} = {}", pattern_to_string(pattern), type_str, expr_to_string(value))
        },
        Decl::Var { name, type_annotation, value, .. } => {
            let type_str = if let Some(t) = type_annotation {
                format!(": {}", type_to_string(t))
            } else {
                String::new()
            };
            format!("var {}{} = {}", name, type_str, expr_to_string(value))
        },
        // TODO: 他の Decl バリアントも追加する必要があるかもしれない
    }
}

// 文を文字列に変換 (Return のみ)
fn stmt_to_string(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Return { value, .. } => {
            if let Some(expr) = value {
                format!("return {}", expr_to_string(expr))
            } else {
                "return".to_string()
            }
        }
        // Stmt::Expr のアームを削除
    }
}

// パターンを文字列に変換
fn pattern_to_string(pattern: &protorun::ast::Pattern) -> String {
    match pattern {
        protorun::ast::Pattern::Literal(value, _) => {
            match value {
                protorun::ast::LiteralValue::Int(i) => i.to_string(),
                protorun::ast::LiteralValue::Float(f) => f.to_string(),
                protorun::ast::LiteralValue::Bool(b) => b.to_string(),
                protorun::ast::LiteralValue::String(s) => format!("\"{}\"", s),
                protorun::ast::LiteralValue::Unit => "()".to_string(),
            }
        },
        protorun::ast::Pattern::Identifier(name, _) => name.clone(),
        protorun::ast::Pattern::Tuple(patterns, _) => {
            let patterns_str: Vec<String> = patterns.iter()
                .map(pattern_to_string)
                .collect();
            format!("({})", patterns_str.join(", "))
        },
        protorun::ast::Pattern::Constructor { name, arguments, .. } => {
            if arguments.is_empty() {
                name.clone()
            } else {
                let args_str: Vec<String> = arguments.iter()
                    .map(pattern_to_string)
                    .collect();
                format!("{}({})", name, args_str.join(", "))
            }
        },
        protorun::ast::Pattern::Wildcard(_) => "_".to_string(),
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
        Expr::ListLiteral { elements, .. } => {
            let elements_str: Vec<String> = elements.iter()
                .map(expr_to_string)
                .collect();
            format!("[{}]", elements_str.join(", "))
        },
        Expr::MapLiteral { entries, .. } => {
            let entries_str: Vec<String> = entries.iter()
                .map(|(key, value)| format!("{} -> {}", expr_to_string(key), expr_to_string(value)))
                .collect();
            format!("{{{}}}", entries_str.join(", "))
        },
        Expr::SetLiteral { elements, .. } => {
            let elements_str: Vec<String> = elements.iter()
                .map(expr_to_string)
                .collect();
            format!("#{{{}}}", elements_str.join(", "))
        },
        Expr::TupleLiteral { elements, .. } => { // Added arm for TupleLiteral
            let elements_str: Vec<String> = elements.iter()
                .map(expr_to_string)
                .collect();
            // UnitLiteral is handled separately, so elements will have >= 2 items here
            format!("({})", elements_str.join(", "))
        },
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
        Expr::MemberAccess { object, member, .. } => {
            format!("{}.{}", expr_to_string(object), member)
        },
        Expr::ParenExpr(expr, _) => {
            format!("({})", expr_to_string(expr))
        },
        Expr::IfExpr { condition, then_branch, else_branch, .. } => {
            let else_str = if let Some(else_expr) = else_branch {
                format!(" else {}", expr_to_string(else_expr))
            } else {
                String::new()
            };
            
            format!("if {} {}{}",
                expr_to_string(condition),
                expr_to_string(then_branch),
                else_str
            )
        },
        Expr::MatchExpr { scrutinee, cases, .. } => {
            let cases_str: Vec<String> = cases.iter()
                .map(|(pattern, guard, expr)| {
                    let guard_str = if let Some(g) = guard {
                        format!(" if {}", expr_to_string(g))
                    } else {
                        String::new()
                    };
                    
                    format!("{}{} => {}",
                        pattern_to_string(pattern),
                        guard_str,
                        expr_to_string(expr)
                    )
                })
                .collect();
            
            format!("match {} {{ {} }}",
                expr_to_string(scrutinee),
                cases_str.join(", ")
            )
        },
        Expr::CollectionComprehension { kind, output_expr, input_expr, pattern, condition, .. } => {
            let condition_str = if let Some(cond) = condition {
                format!(" if {}", expr_to_string(cond))
            } else {
                String::new()
            };
            
            let (prefix, suffix) = match kind {
                protorun::ast::ComprehensionKind::List => ("[", "]"),
                protorun::ast::ComprehensionKind::Map => ("{", "}"),
                protorun::ast::ComprehensionKind::Set => ("#{", "}"),
            };
            
            format!("{}{} for {} <- {}{}{}",
                prefix,
                expr_to_string(output_expr),
                pattern_to_string(pattern),
                expr_to_string(input_expr),
                condition_str,
                suffix
            )
        },
        Expr::BindExpr { bindings, final_expr, .. } => {
            let bindings_str: Vec<String> = bindings.iter()
                .map(|(pattern, expr)| {
                    format!("{} <- {}",
                        pattern_to_string(pattern),
                        expr_to_string(expr)
                    )
                })
                .collect();
            
            format!("bind {{ {}; {} }}",
                bindings_str.join("; "),
                expr_to_string(final_expr)
            )
        },
        Expr::WithExpr { handler, effect_type, body, .. } => {
            // handler は Box<Expr> なので直接 expr_to_string を呼ぶ
            let handler_str = expr_to_string(handler);

            let effect_str = if let Some(effect) = effect_type {
                format!(": {}", type_to_string(effect))
            } else {
                String::new()
            };
            
            format!("with {}{}{}",
                handler_str,
                effect_str,
                expr_to_string(body)
            )
        },
        // FunctionExpr の表示処理
        Expr::FunctionExpr { parameters, effect_parameters, implicit_parameters, body, .. } => { // Renamed from LambdaExpr
            let params_str = parameters.as_ref().map_or("".to_string(), |params| { // パラメータなしの場合は空文字列
                let p_strs: Vec<String> = params.iter()
                    .map(|p| {
                        if let Some(t) = &p.type_annotation {
                            format!("{}: {}", p.name, type_to_string(t))
                        } else {
                            p.name.clone()
                        }
                    })
                    .collect();
                format!("({})", p_strs.join(", "))
            });
            // Effect/Implicit パラメータの表示 (簡易版)
            let effect_params_str = effect_parameters.as_ref().map_or("".to_string(), |params| {
                let p_strs: Vec<String> = params.iter().map(|p| format!("effect {}: {}", p.name, type_to_string(&p.effect_type))).collect();
                format!("({})", p_strs.join(", "))
            });
             let implicit_params_str = implicit_parameters.as_ref().map_or("".to_string(), |params| {
                let p_strs: Vec<String> = params.iter().map(|p| format!("with {}: {}", p.name, p.type_annotation.as_ref().map_or("< inferred >".to_string(), |t| type_to_string(t)))).collect();
                format!("({})", p_strs.join(", "))
            });

            format!("fn {}{}{} => {}", // Changed = to =>
                params_str,
                effect_params_str,
                implicit_params_str,
                expr_to_string(body)
            )
        },
        Expr::BlockExpr { items, .. } => {
            // ブロックの内容を簡易的に表示
            let items_str: Vec<String> = items.iter().map(|item| {
                match item {
                    protorun::ast::BlockItem::Declaration(decl) => decl_to_string(decl),
                    protorun::ast::BlockItem::Statement(stmt) => stmt_to_string(stmt),
                    protorun::ast::BlockItem::Expression(expr) => expr_to_string(expr), // Expression を追加
                }
            }).collect();
            // final_expr はないので、items を結合するだけ
            format!("{{ {} }}", items_str.join(" ")) // 区切りは空白にする
        },
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
        },
        protorun::ast::Type::Array { element_type, .. } => {
            format!("[{}]", type_to_string(element_type))
        },
        // protorun::ast::Type::Tuple の処理を削除
        protorun::ast::Type::Generic { base_type, type_arguments, .. } => {
            let args: Vec<String> = type_arguments.iter()
                .map(type_to_string)
                .collect();
            
            format!("{}<{}>", base_type, args.join(", "))
        },
        protorun::ast::Type::Reference { is_mutable, referenced_type, .. } => {
            if *is_mutable {
                format!("&mut {}", type_to_string(referenced_type))
            } else {
                format!("&{}", type_to_string(referenced_type))
            }
        },
        protorun::ast::Type::Owned { owned_type, .. } => {
            format!("own {}", type_to_string(owned_type))
        },
        protorun::ast::Type::WithEffect { base_type, effect_type, .. } => {
            format!("{} & {}", type_to_string(base_type), type_to_string(effect_type))
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
