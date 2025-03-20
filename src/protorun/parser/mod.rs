// Protorun言語の構文解析器

use super::ast::{
    BinaryOperator, Decl, Expr, Parameter, Program, Span, Stmt, Type, UnaryOperator,
};
use super::error::{Error, Result};
use super::lexer::{Lexer, Token, TokenKind};

/// パーサー
pub struct Parser {
    /// トークン
    tokens: Vec<Token>,
    /// 現在のトークンのインデックス
    current: usize,
    /// ファイル名
    filename: Option<String>,
}

impl Parser {
    /// 新しいパーサーを作成
    pub fn new(tokens: Vec<Token>, filename: Option<String>) -> Self {
        Self {
            tokens,
            current: 0,
            filename,
        }
    }

    /// 入力文字列からパーサーを作成
    pub fn from_str(input: &str, filename: Option<String>) -> Result<Self> {
        let mut lexer = Lexer::new(input, filename.clone());
        let tokens = lexer.tokenize()?;
        Ok(Self::new(tokens, filename))
    }

    /// 現在のトークンを取得
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// 次のトークンへ進む
    fn advance(&mut self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.current_token()
    }

    /// 現在のトークンが指定された種類かチェック
    fn check(&self, kind: &TokenKind) -> bool {
        match self.current_token() {
            Some(token) => &token.kind == kind,
            None => false,
        }
    }

    /// 現在のトークンが指定された種類なら次へ進む
    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// 指定された種類のトークンを期待し、次へ進む
    fn expect(&mut self, kind: &TokenKind, message: &str) -> Result<&Token> {
        if let Some(token) = self.current_token() {
            // TokenKind::Identifierの場合は、名前の一致を無視して型だけチェック
            let match_token = match (&token.kind, kind) {
                (TokenKind::Identifier(_), TokenKind::Identifier(_)) => true,
                _ => &token.kind == kind,
            };
            
            if match_token {
                self.advance();
                return Ok(&self.tokens[self.current - 1]);
            }

            return Err(Error::syntax(
                format!("{}: {:?}の代わりに{:?}を受け取りました", message, kind, token.kind),
                Some(token.span.clone()),
                self.filename.clone(),
            ));
        }

        Err(Error::syntax(
            format!("{}: 入力が予期せず終了しました", message),
            None,
            self.filename.clone(),
        ))
    }

    // プログラム全体をパース
    pub fn parse_program(&mut self) -> Result<Program> {
        let mut declarations = Vec::new();
        let mut statements = Vec::new();

        while let Some(token) = self.current_token() {
            if token.kind == TokenKind::Eof {
                break;
            }

            match token.kind {
                TokenKind::Fn => {
                    let decl = self.parse_function_declaration()?;
                    declarations.push(decl);
                }
                _ => {
                    let stmt = self.parse_statement()?;
                    statements.push(stmt);
                }
            }
        }

        Ok(Program {
            declarations,
            statements,
        })
    }

    // 関数宣言をパース
    fn parse_function_declaration(&mut self) -> Result<Decl> {
        let fn_token = self.expect(&TokenKind::Fn, "関数宣言が期待されます")?;
        let start_pos = fn_token.span.start;
        let start_line = fn_token.span.line;
        let start_column = fn_token.span.column;

        // 関数名
        let name_token = self.expect(&TokenKind::Identifier("".to_string()), "関数名が期待されます")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };

        // パラメータリスト
        self.expect(&TokenKind::LeftParen, "パラメータリストの開始('(')が期待されます")?;
        let parameters = self.parse_parameters()?;
        self.expect(&TokenKind::RightParen, "パラメータリストの終了(')')が期待されます")?;

        // 戻り値の型（オプション）
        let return_type = if self.match_token(&TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        // 関数本体
        self.expect(&TokenKind::Equal, "関数本体の開始('=')が期待されます")?;
        let body = self.parse_expression()?;

        // セミコロン（オプション）
        self.match_token(&TokenKind::Semicolon);

        let end_pos = if let Some(last_token) = self.tokens.get(self.current - 1) {
            last_token.span.end
        } else {
            self.tokens.last().map_or(0, |t| t.span.end)
        };

        let span = Span {
            start: start_pos,
            end: end_pos,
            line: start_line,
            column: start_column,
        };

        Ok(Decl::Function {
            name,
            parameters,
            return_type,
            body,
            span,
        })
    }

    // パラメータリストをパース
    fn parse_parameters(&mut self) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();

        // パラメータがない場合はすぐに戻る
        if self.check(&TokenKind::RightParen) {
            return Ok(parameters);
        }

        loop {
            let param_token = self.expect(&TokenKind::Identifier("".to_string()), "パラメータ名が期待されます")?;
            let param_name = match &param_token.kind {
                TokenKind::Identifier(name) => name.clone(),
                _ => unreachable!(),
            };

            let span = param_token.span.clone();

            // 型注釈（オプション）
            let type_annotation = if self.match_token(&TokenKind::Colon) {
                Some(self.parse_type()?)
            } else {
                None
            };

            parameters.push(Parameter {
                name: param_name,
                type_annotation,
                span,
            });

            // コンマがなければパラメータリストの終了
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }

        Ok(parameters)
    }

    // 型をパース
    fn parse_type(&mut self) -> Result<Type> {
        let type_token = self.expect(&TokenKind::Identifier("".to_string()), "型名が期待されます")?;
        let type_name = match &type_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };

        let span = type_token.span.clone();

        // 今は簡単な型のみサポート
        Ok(Type::Simple { name: type_name, span })
    }

    // 文をパース
    fn parse_statement(&mut self) -> Result<Stmt> {
        match self.current_token() {
            Some(token) => match token.kind {
                TokenKind::Let => self.parse_let_statement(),
                _ => {
                    let expr = self.parse_expression()?;
                    let span = expr.span();

                    // セミコロン（オプション）
                    self.match_token(&TokenKind::Semicolon);

                    Ok(Stmt::Expr { expr, span })
                }
            },
            None => Err(Error::syntax(
                "文が期待されます".to_string(),
                None,
                self.filename.clone(),
            )),
        }
    }

    // let文をパース
    fn parse_let_statement(&mut self) -> Result<Stmt> {
        let let_token = self.expect(&TokenKind::Let, "let文が期待されます")?;
        let start_pos = let_token.span.start;
        let start_line = let_token.span.line;
        let start_column = let_token.span.column;

        // 変数名
        let name_token = self.expect(&TokenKind::Identifier("".to_string()), "変数名が期待されます")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };

        // 型注釈（オプション）
        let type_annotation = if self.match_token(&TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        // 初期値
        self.expect(&TokenKind::Equal, "let文の代入演算子('=')が期待されます")?;
        let value = self.parse_expression()?;

        // セミコロン
        self.expect(&TokenKind::Semicolon, "let文の終了(';')が期待されます")?;

        let end_pos = if let Some(last_token) = self.tokens.get(self.current - 1) {
            last_token.span.end
        } else {
            self.tokens.last().map_or(0, |t| t.span.end)
        };

        let span = Span {
            start: start_pos,
            end: end_pos,
            line: start_line,
            column: start_column,
        };

        Ok(Stmt::Let {
            name,
            type_annotation,
            value,
            span,
        })
    }

    // 式をパース（優先順位に基づく）
    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_equality()
    }

    // 等価性の式をパース
    fn parse_equality(&mut self) -> Result<Expr> {
        let mut expr = self.parse_comparison()?;

        while let Some(token) = self.current_token() {
            let operator = match token.kind {
                TokenKind::EqualEqual => BinaryOperator::Eq,
                TokenKind::NotEqual => BinaryOperator::Neq,
                _ => break,
            };

            let op_span = token.span.clone();
            self.advance();

            let right = self.parse_comparison()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
                line: expr.span().line,
                column: expr.span().column,
            };

            expr = Expr::BinaryOp {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    // 比較の式をパース
    fn parse_comparison(&mut self) -> Result<Expr> {
        let mut expr = self.parse_term()?;

        while let Some(token) = self.current_token() {
            let operator = match token.kind {
                TokenKind::Less => BinaryOperator::Lt,
                TokenKind::Greater => BinaryOperator::Gt,
                TokenKind::LessEqual => BinaryOperator::Lte,
                TokenKind::GreaterEqual => BinaryOperator::Gte,
                _ => break,
            };

            let op_span = token.span.clone();
            self.advance();

            let right = self.parse_term()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
                line: expr.span().line,
                column: expr.span().column,
            };

            expr = Expr::BinaryOp {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    // 項をパース
    fn parse_term(&mut self) -> Result<Expr> {
        let mut expr = self.parse_factor()?;

        while let Some(token) = self.current_token() {
            let operator = match token.kind {
                TokenKind::Plus => BinaryOperator::Add,
                TokenKind::Minus => BinaryOperator::Sub,
                _ => break,
            };

            let op_span = token.span.clone();
            self.advance();

            let right = self.parse_factor()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
                line: expr.span().line,
                column: expr.span().column,
            };

            expr = Expr::BinaryOp {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    // 因子をパース
    fn parse_factor(&mut self) -> Result<Expr> {
        let mut expr = self.parse_unary()?;

        while let Some(token) = self.current_token() {
            let operator = match token.kind {
                TokenKind::Star => BinaryOperator::Mul,
                TokenKind::Slash => BinaryOperator::Div,
                TokenKind::Percent => BinaryOperator::Mod,
                _ => break,
            };

            let op_span = token.span.clone();
            self.advance();

            let right = self.parse_unary()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
                line: expr.span().line,
                column: expr.span().column,
            };

            expr = Expr::BinaryOp {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    // 単項式をパース
    fn parse_unary(&mut self) -> Result<Expr> {
        if let Some(token) = self.current_token() {
            let operator = match token.kind {
                TokenKind::Minus => UnaryOperator::Neg,
                TokenKind::Bang => UnaryOperator::Not,
                _ => return self.parse_call(),
            };

            let op_span = token.span.clone();
            let start_pos = op_span.start;
            let start_line = op_span.line;
            let start_column = op_span.column;
            self.advance();

            let expr = self.parse_unary()?;
            let span = Span {
                start: start_pos,
                end: expr.span().end,
                line: start_line,
                column: start_column,
            };

            return Ok(Expr::UnaryOp {
                operator,
                expr: Box::new(expr),
                span,
            });
        }

        self.parse_call()
    }

    // 関数呼び出しをパース
    fn parse_call(&mut self) -> Result<Expr> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&TokenKind::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    // 関数呼び出しの引数をパース
    fn finish_call(&mut self, function: Expr) -> Result<Expr> {
        let start_pos = function.span().start;
        let start_line = function.span().line;
        let start_column = function.span().column;
        let mut arguments = Vec::new();

        // 引数がない場合
        if !self.check(&TokenKind::RightParen) {
            loop {
                arguments.push(self.parse_expression()?);
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
        }

        let close_paren = self.expect(&TokenKind::RightParen, "関数呼び出しの終了(')')が期待されます")?;
        let end_pos = close_paren.span.end;

        let span = Span {
            start: start_pos,
            end: end_pos,
            line: start_line,
            column: start_column,
        };

        Ok(Expr::FunctionCall {
            function: Box::new(function),
            arguments,
            span,
        })
    }

    // 基本式をパース
    fn parse_primary(&mut self) -> Result<Expr> {
        if let Some(token) = self.current_token().cloned() {
            let expr = match &token.kind {
                TokenKind::True => {
                    self.advance();
                    Expr::BoolLiteral(true, token.span)
                }
                TokenKind::False => {
                    self.advance();
                    Expr::BoolLiteral(false, token.span)
                }
                TokenKind::IntLiteral(value) => {
                    let value = *value;
                    self.advance();
                    Expr::IntLiteral(value, token.span)
                }
                TokenKind::FloatLiteral(value) => {
                    let value = *value;
                    self.advance();
                    Expr::FloatLiteral(value, token.span)
                }
                TokenKind::StringLiteral(value) => {
                    let value = value.clone();
                    self.advance();
                    Expr::StringLiteral(value, token.span)
                }
                TokenKind::Identifier(name) => {
                    let name = name.clone();
                    self.advance();
                    Expr::Identifier(name, token.span)
                }
                TokenKind::LeftParen => {
                    let start_pos = token.span.start;
                    let start_line = token.span.line;
                    let start_column = token.span.column;
                    self.advance();
                    let expr = self.parse_expression()?;
                    let close_paren = self.expect(&TokenKind::RightParen, "式の終了(')')が期待されます")?;
                    let span = Span {
                        start: start_pos,
                        end: close_paren.span.end,
                        line: start_line,
                        column: start_column,
                    };
                    Expr::ParenExpr(Box::new(expr), span)
                }
                TokenKind::LeftBrace => {
                    let start_pos = token.span.start;
                    let start_line = token.span.line;
                    let start_column = token.span.column;
                    self.advance();
                    
                    // ブロック内の文をパース
                    let mut statements = Vec::new();
                    let mut last_expr = None;
                    
                    // 複数の文と最後の式をパースする
                    while !self.check(&TokenKind::RightBrace) && !self.check(&TokenKind::Eof) {
                        if let Some(token) = self.current_token() {
                            if token.kind == TokenKind::RightBrace {
                                break;
                            }
                            
                            if self.check(&TokenKind::Let) {
                                let stmt = self.parse_statement()?;
                                statements.push(stmt);
                            } else {
                                // セミコロンで終わっていれば文、そうでなければ式
                                let expr = self.parse_expression()?;
                                
                                if self.match_token(&TokenKind::Semicolon) {
                                    let span = expr.span();
                                    statements.push(Stmt::Expr { expr, span });
                                } else {
                                    last_expr = Some(expr);
                                    break; // ブロックの最後の式
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    
                    let close_brace = self.expect(&TokenKind::RightBrace, "ブロックの終了('}')が期待されます")?;
                    
                    let span = Span {
                        start: start_pos,
                        end: close_brace.span.end,
                        line: start_line,
                        column: start_column,
                    };
                    
                    // Blockエクスプレッションを未実装の場合は、下記のようなプレースホルダー実装を使う
                    // 本来はここにExpr::Block { statements, last_expr, span }のようなものが必要
                    // 暫定的にlast_expr があれば返し、なければ UnitLiteral を返す
                    if let Some(expr) = last_expr {
                        expr
                    } else {
                        Expr::UnitLiteral(span)
                    }
                }
                _ => {
                    return Err(Error::syntax(
                        format!("式が期待されます: {:?}", token.kind),
                        Some(token.span.clone()),
                        self.filename.clone(),
                    ));
                }
            };

            Ok(expr)
        } else {
            Err(Error::syntax(
                "式が期待されます（入力が予期せず終了しました）".to_string(),
                None,
                self.filename.clone(),
            ))
        }
    }
}

// ASTノードのspan取得用のヘルパートレイト
trait Spannable {
    fn span(&self) -> Span;
}

impl Spannable for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::IntLiteral(_, span) => span.clone(),
            Expr::FloatLiteral(_, span) => span.clone(),
            Expr::BoolLiteral(_, span) => span.clone(),
            Expr::StringLiteral(_, span) => span.clone(),
            Expr::UnitLiteral(span) => span.clone(),
            Expr::Identifier(_, span) => span.clone(),
            Expr::BinaryOp { span, .. } => span.clone(),
            Expr::UnaryOp { span, .. } => span.clone(),
            Expr::FunctionCall { span, .. } => span.clone(),
            Expr::ParenExpr(_, span) => span.clone(),
        }
    }
}

#[cfg(test)]
mod tests;
