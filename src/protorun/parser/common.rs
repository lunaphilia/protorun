// Protorun言語のパーサー共通ユーティリティ

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, multispace1, none_of},
    combinator::{cut, map, map_res, opt, recognize, value},
    error::{context, ErrorKind, VerboseError},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    Finish, IResult,
};

use std::rc::Rc;
use std::cell::RefCell;
use crate::protorun::ast::Span;
use crate::protorun::error::{Error, Result};
use crate::protorun::symbol::{Symbol, SymbolTable, ScopeKind, SymbolKind};

/// パーサーの結果型
pub type ParseResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

/// パーサーのコンテキスト情報
pub struct ParserContext<'a> {
    /// 元の入力文字列
    pub original_input: &'a str,
    /// ファイル名
    pub filename: Option<String>,
    /// 現在のシンボルテーブル
    pub symbol_table: Rc<RefCell<SymbolTable>>,
}

impl<'a> ParserContext<'a> {
    /// 新しいパーサーコンテキストを作成
    pub fn new(input: &'a str, filename: Option<String>) -> Self {
        let global_scope = Rc::new(RefCell::new(SymbolTable::new(ScopeKind::Global)));
        Self {
            original_input: input,
            filename,
            symbol_table: global_scope,
        }
    }
    
    /// 新しいスコープを開始
    pub fn enter_scope(&self, scope_kind: ScopeKind) {
        let current = self.symbol_table.clone();
        let new_scope = Rc::new(RefCell::new(SymbolTable::with_parent(scope_kind, current)));
        // 内部可変性を使用して symbol_table を更新
        // 安全性: symbol_tableはRc<RefCell<>>で包まれているため、内部可変性を持つ
        // このメソッドの呼び出し中に他のスレッドがsymbol_tableにアクセスすることはない
        let self_ptr = self as *const Self as *mut Self;
        unsafe {
            (*self_ptr).symbol_table = new_scope;
        }
    }
    
    /// 現在のスコープを終了し、親スコープに戻る
    pub fn exit_scope(&self) {
        let parent = {
            let current = self.symbol_table.borrow();
            current.parent()
        };
        
        if let Some(parent_scope) = parent {
            // 内部可変性を使用して symbol_table を更新
            let self_ptr = self as *const Self as *mut Self;
            unsafe {
                (*self_ptr).symbol_table = parent_scope;
            }
        }
    }
    
    /// シンボルを追加
    pub fn add_symbol(&self, symbol: Symbol) -> bool {
        self.symbol_table.borrow_mut().add_symbol(symbol)
    }
    
    /// シンボルを検索
    pub fn lookup_symbol(&self, name: &str) -> Option<Symbol> {
        self.symbol_table.borrow().lookup_symbol_recursive(name)
    }
    
    /// シンボルの使用をマークするメソッド
    pub fn mark_symbol_used(&self, name: &str) -> bool {
        self.symbol_table.borrow_mut().mark_symbol_used(name)
    }
    
    /// 未使用シンボルを検出するメソッド
    pub fn find_unused_symbols(&self) -> Vec<Symbol> {
        self.symbol_table.borrow().find_unused_symbols()
            .into_iter()
            .cloned()
            .collect()
    }
    
    /// 特定の種類のシンボルを検索するメソッド
    pub fn find_symbols_by_kind(&self, kind: SymbolKind) -> Vec<Symbol> {
        self.symbol_table.borrow().find_symbols_by_kind(kind)
            .into_iter()
            .cloned()
            .collect()
    }
    
    /// 現在のスコープの種類を取得するメソッド
    pub fn current_scope_kind(&self) -> ScopeKind {
        self.symbol_table.borrow().scope_kind()
    }
    
    /// スコープのネスト深度を取得するメソッド
    pub fn scope_depth(&self) -> usize {
        let mut depth = 0;
        let mut current_opt = Some(self.symbol_table.clone());
        
        while let Some(current) = current_opt {
            if let Some(parent) = current.borrow().parent() {
                depth += 1;
                current_opt = Some(parent);
            } else {
                break;
            }
        }
        
        depth
    }
    
    /// 入力文字列と残りの文字列からSpan情報を計算
    pub fn calculate_span(&self, remaining: &'a str) -> Span {
        let consumed = self.original_input.len() - remaining.len();
        let start = consumed.saturating_sub(1);
        let end = consumed;
        
        // 行と列の計算
        let prefix = &self.original_input[..start];
        let line = 1 + prefix.chars().filter(|&c| c == '\n').count();
        let column = if let Some(last_newline) = prefix.rfind('\n') {
            prefix.len() - last_newline
        } else {
            1 + prefix.len()
        };
        
        Span {
            start,
            end,
            line,
            column,
        }
    }
}

/// 構文エラーをProtorunのエラーに変換
pub fn to_syntax_error<'a>(input: &'a str, error: VerboseError<&'a str>, filename: Option<String>) -> Error {
    // エラーメッセージの生成
    let message = if error.errors.is_empty() {
        "構文解析エラー".to_string()
    } else {
        let (input_slice, kind) = &error.errors[0];
        match kind {
            nom::error::VerboseErrorKind::Nom(ErrorKind::Tag) => {
                format!("期待されるキーワードが見つかりません: '{}'", input_slice.chars().take(10).collect::<String>())
            },
            nom::error::VerboseErrorKind::Nom(ErrorKind::Char) => {
                format!("期待される文字が見つかりません: '{}'", input_slice.chars().take(1).collect::<String>())
            },
            nom::error::VerboseErrorKind::Nom(ErrorKind::Eof) => "式が期待されます".to_string(),
            nom::error::VerboseErrorKind::Context(ctx) => format!("{}: 構文解析エラー", ctx),
            _ => format!("構文解析エラー: {:?}", kind),
        }
    };

    // エラーの位置情報
    let pos = input.len().saturating_sub(input.trim_start().len());
    let span = Span {
        start: pos,
        end: pos + 1,
        line: 1 + input[..pos].chars().filter(|&c| c == '\n').count(),
        column: 1 + input[..pos].chars().rev().take_while(|&c| c != '\n').count(),
    };

    Error::syntax(message, Some(span), filename)
}

/// 行コメントをスキップ
pub fn skip_comment(input: &str) -> ParseResult<&str> {
    preceded(
        tag("//"),
        terminated(
            take_while1(|c| c != '\n'),
            alt((value((), char('\n')), value((), nom::combinator::eof)))
        )
    )(input)
}

/// 空白とコメントをスキップ（コメント対応版）
pub fn ws_comments<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> ParseResult<'a, O>
where
    F: FnMut(&'a str) -> ParseResult<'a, O>,
{
    delimited(
        many0(alt((
            value((), multispace1),
            value((), skip_comment),
        ))),
        inner,
        many0(alt((
            value((), multispace1),
            value((), skip_comment),
        )))
    )
}

/// 識別子をパース
pub fn identifier(input: &str) -> ParseResult<&str> {
    recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        )
    )(input)
}

/// 識別子文字列をパース
pub fn identifier_string(input: &str) -> ParseResult<String> {
    map(identifier, |s: &str| s.to_string())(input)
}

/// キーワードをパース
pub fn keyword<'a>(kw: &'static str) -> impl FnMut(&'a str) -> ParseResult<'a, &'a str> {
    ws_comments(tag(kw))
}

/// 区切られたリストをパース
pub fn delimited_list<'a, F, O>(
    open: char,
    parser: F,
    separator: char,
    close: char,
) -> impl FnMut(&'a str) -> ParseResult<'a, Vec<O>>
where
    F: FnMut(&'a str) -> ParseResult<'a, O>,
{
    delimited(
        ws_comments(char(open)),
        separated_list0(
            ws_comments(char(separator)),
            parser
        ),
        cut(ws_comments(char(close)))
    )
}

/// コンテキスト付きのパーサー
pub fn with_context<'a, F, O>(
    ctx: &'static str,
    parser: F
) -> impl FnMut(&'a str) -> ParseResult<'a, O>
where
    F: FnMut(&'a str) -> ParseResult<'a, O>,
{
    context(ctx, parser)
}
