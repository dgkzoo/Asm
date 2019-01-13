/**
 * ## アセンブラ
 *
 * ## author
 * dgkzoo
 */
use symbol_table::SymbolTable;
use parser::Parser;

pub struct Assembler {
}

impl Assembler {
    pub fn create_symbol_tble(&self, _filepath:String) -> SymbolTable{
        let st = SymbolTable:: new();
        return st;
    }

    pub fn exec(&self, filepath:String, st:SymbolTable) {
        let parser = Parser::new(filepath.to_string());
    }
}
