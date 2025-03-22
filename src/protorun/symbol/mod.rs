// Protorun言語のシンボルテーブル

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::protorun::ast::{Span, Type};
use crate::protorun::error::{Error, Result};
use crate::protorun::parser::common::ParserContext;

/// シンボルの種類
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
    Type,
    Parameter,
}

/// 型の種類
#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Struct,
    Enum,
    Trait,
    TypeAlias,
}

/// 型定義の詳細情報
#[derive(Debug, Clone, PartialEq)]
pub struct TypeInfo {
    /// 型の種類（enum, struct, trait など）
    pub kind: TypeKind,
    /// 型パラメータ（ジェネリック型の場合）
    pub type_parameters: Vec<String>,
    /// フィールド（レコード型の場合）
    pub fields: Option<Vec<(String, crate::protorun::ast::Type)>>,
    /// バリアント（enum型の場合）
    pub variants: Option<Vec<crate::protorun::ast::EnumVariant>>,
    /// 親トレイト（トレイトの場合）
    pub super_trait: Option<crate::protorun::ast::Type>,
    /// エイリアス先の型（型エイリアスの場合）
    pub aliased_type: Option<crate::protorun::ast::Type>,
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
    /// 型定義の詳細情報（型シンボルの場合）
    pub type_info: Option<TypeInfo>,
    /// 使用されているかどうか
    pub is_used: bool,
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
    
    /// シンボルの使用をマークするメソッド
    pub fn mark_symbol_used(&mut self, name: &str) -> bool {
        if let Some(symbol) = self.symbols.get_mut(name) {
            symbol.is_used = true;
            return true;
        }
        
        // 親スコープで再帰的に検索
        if let Some(parent) = &self.parent {
            return parent.borrow_mut().mark_symbol_used(name);
        }
        
        false
    }
    
    /// 未使用シンボルを検出するメソッド
    pub fn find_unused_symbols(&self) -> Vec<&Symbol> {
        self.symbols.values()
            .filter(|symbol| !symbol.is_used)
            .collect()
    }
    
    /// 特定の種類のシンボルを検索するメソッド
    pub fn find_symbols_by_kind(&self, kind: SymbolKind) -> Vec<&Symbol> {
        self.symbols.values()
            .filter(|symbol| symbol.kind == kind)
            .collect()
    }
    
    /// スコープ内のすべてのシンボルを取得するメソッド
    pub fn get_all_symbols(&self) -> Vec<&Symbol> {
        self.symbols.values().collect()
    }
}

/// 型定義のシンボル登録ヘルパー関数
pub fn register_type_symbol(
    ctx: &ParserContext,
    name: &str,
    kind: TypeKind,
    type_parameters: Vec<String>,
    span: Span
) -> bool {
    let symbol = Symbol {
        name: name.to_string(),
        kind: SymbolKind::Type,
        type_annotation: None,
        declaration_span: span,
        is_mutable: false,
        type_info: Some(TypeInfo {
            kind,
            type_parameters,
            fields: None,
            variants: None,
            super_trait: None,
            aliased_type: None,
        }),
        is_used: false,
    };
    
    ctx.add_symbol(symbol)
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod property_tests;
