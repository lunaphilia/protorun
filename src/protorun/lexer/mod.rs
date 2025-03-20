// Protorun言語の字句解析器

use super::ast::Span;
use super::error::{Error, Result};

/// トークンの種類
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // キーワード
    Let,
    Fn,
    If,
    Else,
    True,
    False,
    Return,
    
    // 識別子
    Identifier(String),
    
    // リテラル
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    
    // 記号
    LeftParen,       // (
    RightParen,      // )
    LeftBrace,       // {
    RightBrace,      // }
    LeftBracket,     // [
    RightBracket,    // ]
    Comma,           // ,
    Dot,             // .
    Colon,           // :
    Semicolon,       // ;
    Equal,           // =
    
    // 演算子
    Plus,            // +
    Minus,           // -
    Star,            // *
    Slash,           // /
    Percent,         // %
    
    // 比較演算子
    EqualEqual,      // ==
    NotEqual,        // !=
    Less,            // <
    Greater,         // >
    LessEqual,       // <=
    GreaterEqual,    // >=
    
    // 論理演算子
    And,             // &&
    Or,              // ||
    Bang,            // !
    
    // 特殊
    Eof,             // 入力の終端
}

/// トークン
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// トークンの種類
    pub kind: TokenKind,
    /// 位置情報
    pub span: Span,
}

impl Token {
    /// 新しいトークンを作成
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// レキサー
pub struct Lexer<'a> {
    /// 入力テキスト
    input: &'a str,
    /// 入力テキストをCharsイテレータに変換
    chars: std::str::Chars<'a>,
    /// 文字の位置（文字数）
    position: usize,
    /// バイト位置
    byte_position: usize,
    /// 現在の行番号（1始まり）
    line: usize,
    /// 現在の列番号（1始まり）
    column: usize,
    /// 現在の文字
    current_char: Option<char>,
    /// ファイル名
    filename: Option<String>,
}

impl<'a> Lexer<'a> {
    /// 新しいレキサーを作成
    pub fn new(input: &'a str, filename: Option<String>) -> Self {
        let mut lexer = Self {
            input,
            chars: input.chars(),
            position: 0,
            byte_position: 0,
            line: 1,
            column: 1,
            current_char: None,
            filename,
        };
        
        // 最初の文字を設定
        lexer.read_char();
        
        lexer
    }
    
    /// 次の文字を読み込み、現在の文字を更新
    fn read_char(&mut self) {
        self.current_char = self.chars.next();
        
        if let Some(c) = self.current_char {
            // バイト位置を更新（UTF-8エンコーディングでは文字のバイト数は可変）
            self.byte_position += c.len_utf8();
            
            // 行番号と列番号を更新
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            
            self.position += 1;
        }
    }
    
    /// 現在の位置から始まるスパンを作成
    fn make_span(&self, start_pos: usize, start_line: usize, start_column: usize) -> Span {
        Span {
            start: start_pos,
            end: self.position,
            line: start_line,
            column: start_column,
        }
    }
    
    /// 空白とコメントをスキップ
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.current_char {
                // 空白文字をスキップ
                Some(c) if c.is_whitespace() => {
                    self.read_char();
                },
                
                // 行コメント（//）をスキップ
                Some('/') if self.peek_char() == Some('/') => {
                    // 最初のスラッシュは既に読み込まれているので、2つ目のスラッシュを読み込む
                    self.read_char();
                    
                    // 行末までスキップ
                    while let Some(c) = self.current_char {
                        if c == '\n' {
                            break;
                        }
                        self.read_char();
                    }
                    
                    // 改行文字が見つかった場合は、それもスキップ
                    if self.current_char == Some('\n') {
                        self.read_char();
                    }
                },
                
                // それ以外の文字の場合は終了
                _ => break,
            }
        }
    }
    
    /// 次の文字を先読みする
    fn peek_char(&self) -> Option<char> {
        // Charsイテレータのcloneは、現在のイテレータの状態をコピーします
        let mut chars_clone = self.chars.clone();
        chars_clone.next()
    }
    
    /// 識別子またはキーワードをトークン化
    fn read_identifier(&mut self) -> Result<Token> {
        // 現在の文字は既に識別子の最初の文字
        let start_pos = self.position - 1;
        let start_line = self.line;
        let start_column = self.column - 1;
        
        // 一文字目（現在処理中の文字）
        let mut identifier = if let Some(c) = self.input.chars().nth(start_pos) {
            c.to_string()
        } else {
            String::new()
        };
        
        // 次の文字へ進む
        self.read_char();
        
        // 残りの文字を読み込む
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.read_char();
            } else {
                break;
            }
        }
        
        let span = self.make_span(start_pos, start_line, start_column);
        
        // キーワードかどうかをチェック
        let kind = match identifier.as_str() {
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "return" => TokenKind::Return,
            _ => TokenKind::Identifier(identifier),
        };
        
        Ok(Token::new(kind, span))
    }
    
    /// 数値リテラルをトークン化
    fn read_number(&mut self) -> Result<Token> {
        // 現在の文字は既に数値の最初の桁
        let start_pos = self.position - 1;
        let start_line = self.line;
        let start_column = self.column - 1;
        let mut is_float = false;
        
        // 一桁目（現在処理中の数字）
        let mut number_str = if let Some(c) = self.input.chars().nth(start_pos) {
            c.to_string()
        } else {
            String::new()
        };
        
        // 次の文字へ進む
        self.read_char();
        
        // 残りの数字を読み込む
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                number_str.push(c);
                self.read_char();
            } else if c == '.' && !is_float {
                number_str.push(c);
                is_float = true;
                self.read_char();
            } else {
                break;
            }
        }
        
        let span = self.make_span(start_pos, start_line, start_column);
        
        if is_float {
            match number_str.parse::<f64>() {
                Ok(value) => Ok(Token::new(TokenKind::FloatLiteral(value), span)),
                Err(_) => Err(Error::lexical(
                    format!("無効な浮動小数点数: {}", number_str),
                    Some(span.clone()),
                    self.filename.clone(),
                )),
            }
        } else {
            match number_str.parse::<i64>() {
                Ok(value) => Ok(Token::new(TokenKind::IntLiteral(value), span)),
                Err(_) => Err(Error::lexical(
                    format!("無効な整数: {}", number_str),
                    Some(span.clone()),
                    self.filename.clone(),
                )),
            }
        }
    }
    
    /// 文字列リテラルをトークン化
    fn read_string(&mut self) -> Result<Token> {
        let start_pos = self.position - 1;
        let start_line = self.line;
        let start_column = self.column - 1;
        let mut string_content = String::new();
        
        // 閉じるダブルクォートまで読み込む
        self.read_char(); // 開始のダブルクォートをスキップ
        
        while let Some(c) = self.current_char {
            if c == '"' {
                self.read_char(); // 終了のダブルクォートをスキップ
                break;
            } else if c == '\\' {
                // エスケープシーケンスの処理
                self.read_char();
                match self.current_char {
                    Some('n') => string_content.push('\n'),
                    Some('r') => string_content.push('\r'),
                    Some('t') => string_content.push('\t'),
                    Some('\\') => string_content.push('\\'),
                    Some('"') => string_content.push('"'),
                    Some(c) => {
                        string_content.push('\\');
                        string_content.push(c);
                    },
                    None => {
                        let span = self.make_span(start_pos, start_line, start_column);
                        return Err(Error::lexical(
                            "文字列リテラルが終了する前に入力が終了しました",
                            Some(span),
                            self.filename.clone(),
                        ));
                    }
                }
            } else {
                string_content.push(c);
            }
            
            self.read_char();
        }
        
        let span = self.make_span(start_pos, start_line, start_column);
        Ok(Token::new(TokenKind::StringLiteral(string_content), span))
    }
    
    /// 次のトークンを取得
    pub fn next_token(&mut self) -> Result<Token> {
        // 空白とコメントをスキップ
        self.skip_whitespace_and_comments();
        
        // 現在の文字に基づいてトークンを作成
        let token = match self.current_char {
            None => {
                // 入力の終端
                let span = Span {
                    start: self.position,
                    end: self.position,
                    line: self.line,
                    column: self.column,
                };
                Ok(Token::new(TokenKind::Eof, span))
            },
            
            // 識別子
            Some(c) if c.is_alphabetic() || c == '_' => {
                self.read_identifier()
            },
            
            // 数値
            Some(c) if c.is_digit(10) => {
                self.read_number()
            },
            
            // 文字列
            Some('"') => {
                self.read_string()
            },
            
            // 記号と演算子
            Some(c) => {
                let start_pos = self.position - 1;
                let start_line = self.line;
                let start_column = self.column - 1;
                
                // 次の文字を読み込み、現在の文字を更新
                self.read_char();
                
                let kind = match c {
                    // 単一文字の記号
                    '(' => TokenKind::LeftParen,
                    ')' => TokenKind::RightParen,
                    '{' => TokenKind::LeftBrace,
                    '}' => TokenKind::RightBrace,
                    '[' => TokenKind::LeftBracket,
                    ']' => TokenKind::RightBracket,
                    ',' => TokenKind::Comma,
                    '.' => TokenKind::Dot,
                    ':' => TokenKind::Colon,
                    ';' => TokenKind::Semicolon,
                    
                    // 単一または複数文字の演算子
                    '=' => {
                        if self.current_char == Some('=') {
                            self.read_char();
                            TokenKind::EqualEqual
                        } else {
                            TokenKind::Equal
                        }
                    },
                    '+' => TokenKind::Plus,
                    '-' => TokenKind::Minus,
                    '*' => TokenKind::Star,
                    '/' => TokenKind::Slash,
                    '%' => TokenKind::Percent,
                    '!' => {
                        if self.current_char == Some('=') {
                            self.read_char();
                            TokenKind::NotEqual
                        } else {
                            TokenKind::Bang
                        }
                    },
                    '<' => {
                        if self.current_char == Some('=') {
                            self.read_char();
                            TokenKind::LessEqual
                        } else {
                            TokenKind::Less
                        }
                    },
                    '>' => {
                        if self.current_char == Some('=') {
                            self.read_char();
                            TokenKind::GreaterEqual
                        } else {
                            TokenKind::Greater
                        }
                    },
                    '&' => {
                        if self.current_char == Some('&') {
                            self.read_char();
                            TokenKind::And
                        } else {
                            return Err(Error::lexical(
                                format!("予期しない文字: {}", c),
                                Some(Span {
                                    start: start_pos,
                                    end: self.position,
                                    line: start_line,
                                    column: start_column,
                                }),
                                self.filename.clone(),
                            ));
                        }
                    },
                    '|' => {
                        if self.current_char == Some('|') {
                            self.read_char();
                            TokenKind::Or
                        } else {
                            return Err(Error::lexical(
                                format!("予期しない文字: {}", c),
                                Some(Span {
                                    start: start_pos,
                                    end: self.position,
                                    line: start_line,
                                    column: start_column,
                                }),
                                self.filename.clone(),
                            ));
                        }
                    },
                    
                    // 未知の文字
                    _ => {
                        return Err(Error::lexical(
                            format!("予期しない文字: {}", c),
                            Some(Span {
                                start: start_pos,
                                end: self.position,
                                line: start_line,
                                column: start_column,
                            }),
                            self.filename.clone(),
                        ));
                    }
                };
                
                let span = self.make_span(start_pos, start_line, start_column);
                Ok(Token::new(kind, span))
            }
        };
        
        token
    }
    
    /// すべてのトークンを取得
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenize_simple_expression() {
        let input = "let x = 42 + 3.14;";
        let mut lexer = Lexer::new(input, None);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 8); // let, x, =, 42, +, 3.14, ;, EOF
        
        assert_eq!(tokens[0].kind, TokenKind::Let);
        
        match &tokens[1].kind {
            TokenKind::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("期待される識別子ではありません"),
        }
        
        assert_eq!(tokens[2].kind, TokenKind::Equal);
        
        match tokens[3].kind {
            TokenKind::IntLiteral(value) => assert_eq!(value, 42),
            _ => panic!("期待される整数リテラルではありません"),
        }
        
        assert_eq!(tokens[4].kind, TokenKind::Plus);
        
        match tokens[5].kind {
            TokenKind::FloatLiteral(value) => assert_eq!(value, 3.14),
            _ => panic!("期待される浮動小数点リテラルではありません"),
        }
        
        assert_eq!(tokens[6].kind, TokenKind::Semicolon);
        assert_eq!(tokens[7].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_function_declaration() {
        let input = "fn add(x: Int, y: Int): Int = x + y;";
        let mut lexer = Lexer::new(input, None);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::Fn);
        
        match &tokens[1].kind {
            TokenKind::Identifier(name) => assert_eq!(name, "add"),
            _ => panic!("期待される識別子ではありません"),
        }
        
        assert_eq!(tokens[2].kind, TokenKind::LeftParen);
        
        match &tokens[3].kind {
            TokenKind::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("期待される識別子ではありません"),
        }
        
        assert_eq!(tokens[4].kind, TokenKind::Colon);
        
        match &tokens[5].kind {
            TokenKind::Identifier(name) => assert_eq!(name, "Int"),
            _ => panic!("期待される識別子ではありません"),
        }
        
        assert_eq!(tokens[6].kind, TokenKind::Comma);
        
        // 他のトークンも同様に確認...
    }

    // コメントに関するテスト
    #[test]
    fn test_skip_line_comments() {
        let input = "// これは行コメントです\nlet x = 10;";
        let mut lexer = Lexer::new(input, None);
        let tokens = lexer.tokenize().unwrap();
        
        // コメントはスキップされるので、最初のトークンはletになるはず
        assert_eq!(tokens[0].kind, TokenKind::Let);
        
        match &tokens[1].kind {
            TokenKind::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("期待される識別子ではありません"),
        }
    }

    #[test]
    fn test_multiple_comments() {
        let input = "// コメント1\n// コメント2\nlet x = 42;";
        let mut lexer = Lexer::new(input, None);
        let tokens = lexer.tokenize().unwrap();
        
        // 両方のコメントがスキップされる
        assert_eq!(tokens[0].kind, TokenKind::Let);
    }

    #[test]
    fn test_comment_after_code() {
        let input = "let x = 10; // 変数の定義\nlet y = 20;";
        let mut lexer = Lexer::new(input, None);
        let tokens = lexer.tokenize().unwrap();
        
        // コードとコメントが混在している場合の処理
        assert_eq!(tokens[0].kind, TokenKind::Let);
        
        match &tokens[1].kind {
            TokenKind::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("期待される識別子ではありません"),
        }
        
        assert_eq!(tokens[4].kind, TokenKind::Semicolon);
        
        // コメントの後のコードが正しく解析される
        assert_eq!(tokens[5].kind, TokenKind::Let);
        
        match &tokens[6].kind {
            TokenKind::Identifier(name) => assert_eq!(name, "y"),
            _ => panic!("期待される識別子ではありません"),
        }
    }

    #[test]
    fn test_comment_at_end_of_file() {
        let input = "let x = 10; // 最後のコメント";
        let mut lexer = Lexer::new(input, None);
        let tokens = lexer.tokenize().unwrap();
        
        // ファイル末尾のコメントが正しく処理される
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn test_japanese_comments() {
        let input = "// 日本語のコメント\nlet x = 42;";
        let mut lexer = Lexer::new(input, None);
        let tokens = lexer.tokenize().unwrap();
        
        // 日本語を含むコメントが正しく処理される
        assert_eq!(tokens[0].kind, TokenKind::Let);
    }

    #[test]
    fn test_comments_with_code_inside() {
        let input = "// let x = 10;\nlet y = 20;";
        let mut lexer = Lexer::new(input, None);
        let tokens = lexer.tokenize().unwrap();
        
        // コメント内のコードは無視される
        assert_eq!(tokens[0].kind, TokenKind::Let);
        
        match &tokens[1].kind {
            TokenKind::Identifier(name) => assert_eq!(name, "y"),
            _ => panic!("期待される識別子ではありません"),
        }
    }
}
