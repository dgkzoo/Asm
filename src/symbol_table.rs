/**
 * ## シンボルテーブル
 *
 * ## author
 * dgkzoo
 */
use std::collections::HashMap;

pub struct SymbolTable {
    pub symbol_map:HashMap<String, String>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { symbol_map:HashMap::new() }
    }
}