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

    ///
    /// 初期化
    /// 定義済みシンボルをMapに入れとく
    ///
    pub fn init(&mut self) {
        self.symbol_map.insert("SP".to_string(), 0x0000);
        self.symbol_map.insert("LCL".to_string(), 0x0001);
        self.symbol_map.insert("ARG".to_string(), 0x0002);
        self.symbol_map.insert("THIS".to_string(), 0x0003);
        self.symbol_map.insert("THAT".to_string(), 0x0004);
        self.symbol_map.insert("R0".to_string(), 0x0000);
        self.symbol_map.insert("R1".to_string(), 0x0001);
        self.symbol_map.insert("R2".to_string(), 0x0002);
        self.symbol_map.insert("R3".to_string(), 0x0003);
        self.symbol_map.insert("R4".to_string(), 0x0004);
        self.symbol_map.insert("R5".to_string(), 0x0005);
        self.symbol_map.insert("R6".to_string(), 0x0006);
        self.symbol_map.insert("R7".to_string(), 0x0007);
        self.symbol_map.insert("R8".to_string(), 0x0008);
        self.symbol_map.insert("R9".to_string(), 0x0009);
        self.symbol_map.insert("R10".to_string(), 0x000A);
        self.symbol_map.insert("R11".to_string(), 0x000B);
        self.symbol_map.insert("R12".to_string(), 0x000C);
        self.symbol_map.insert("R13".to_string(), 0x000D);
        self.symbol_map.insert("R14".to_string(), 0x000E);
        self.symbol_map.insert("R15".to_string(), 0x000F);
        self.symbol_map.insert("SCREEN".to_string(), 0x4000);
        self.symbol_map.insert("KBD".to_string(), 0x6000);
    }

    ///
    /// 指定のシンボルが登録済みか？
    ///
    pub fn contains(&self, symbol: &String) -> bool {
        return self.symbol_map.contains_key(symbol);
    }

    ///
    /// シンボルを登録する
    ///
    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.symbol_map.insert(symbol, address);
    }

    ///
    /// 指定のシンボルのアドレスを返す
    ///
    pub fn get_address(&self, symbol: String) -> &u16 {
        return self.symbol_map.get(&symbol).unwrap();
    }
}
