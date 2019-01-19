///
/// ## シンボルテーブル
///
/// ## author
/// dgkzoo
///
use std::collections::HashMap;

pub struct SymbolTable {
    symbol_map: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbol_map: HashMap::new(),
        }
    }

    pub fn contains(&self, symbol: &String) -> bool {
        return self.symbol_map.contains_key(symbol);
    }

    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.symbol_map.insert(symbol, address);
    }

    pub fn get_address(&self, symbol: String) -> &u16 {
        return self.symbol_map.get(&symbol).unwrap();
    }
}
