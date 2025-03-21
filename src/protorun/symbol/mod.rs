// Protorun言語のシンボルテーブル

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::protorun::ast::{Span, Type};
use crate::protorun::error::{Error, Result};

/// シンボルの種類
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
    Type,
    Parameter,
}

/// スコープの種類
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind {
    Global,
    Module,
    Function,
    Block,
    Loop,
}

/// シンボル情報
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    /// シンボル名
    pub name: String,
    /// シンボルの種類
    pub kind: SymbolKind,
    /// 型注釈（オプション）
    pub type_annotation: Option<Type>,
    /// 宣言位置
    pub declaration_span: Span,
    /// 可変かどうか
    pub is_mutable: bool,
}

/// シンボルテーブル
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// シンボルのマップ
    symbols: HashMap<String, Symbol>,
    /// 親スコープ
    parent: Option<Rc<RefCell<SymbolTable>>>,
    /// スコープの種類
    scope_kind: ScopeKind,
}

impl SymbolTable {
    /// 新しいシンボルテーブルを作成
    pub fn new(scope_kind: ScopeKind) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: None,
            scope_kind,
        }
    }
    
    /// 親スコープを持つシンボルテーブルを作成
    pub fn with_parent(scope_kind: ScopeKind, parent: Rc<RefCell<SymbolTable>>) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Some(parent),
            scope_kind,
        }
    }
    
    /// シンボルを追加
    pub fn add_symbol(&mut self, symbol: Symbol) -> bool {
        if self.symbols.contains_key(&symbol.name) {
            return false; // 既に存在する場合は追加失敗
        }
        self.symbols.insert(symbol.name.clone(), symbol);
        true
    }
    
    /// 現在のスコープでシンボルを検索
    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
    
    /// 現在のスコープから親スコープへ遡ってシンボルを検索
    pub fn lookup_symbol_recursive(&self, name: &str) -> Option<Symbol> {
        if let Some(symbol) = self.symbols.get(name) {
            return Some(symbol.clone());
        }
        
        if let Some(parent) = &self.parent {
            return parent.borrow().lookup_symbol_recursive(name);
        }
        
        None
    }
    
    /// スコープの種類を取得
    pub fn scope_kind(&self) -> ScopeKind {
        self.scope_kind.clone()
    }
    
    /// 親スコープを取得
    pub fn parent(&self) -> Option<Rc<RefCell<SymbolTable>>> {
        self.parent.clone()
    }
}

#[cfg(test)]
mod tests;
