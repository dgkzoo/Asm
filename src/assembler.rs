///
/// ## アセンブラ
///
/// ## author
/// dgkzoo
///
use std::io::{BufReader, BufRead};
use std::fs;
use symbol_table::SymbolTable;
use parser;
use parser::Parser;

pub struct Assembler {
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {}
    }

    ///
    /// シンボルテーブルを作成する
    /// 
    pub fn create_symbol_tble(&self, _filepath:String) -> SymbolTable{
        let st = SymbolTable:: new();
        return st;
    }

    ///
    /// アセンブルの実行
    /// 
    pub fn exec(&self, filepath:String, st:SymbolTable) {
        let parser = Parser::new();

        let file = fs::File::open(filepath.to_string()).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            // コメント、空白行などを除去
            let line = parser.get_valid_line(line.unwrap());
            if line.is_empty() {
                continue;
            }

            // コマンドタイプの取得
            let command_type = parser.get_command_type(line.to_string());

            // シンボルの取得
            let mut symbol:String = "".to_string();
            if command_type == parser::A_COMMAND || command_type == parser::L_COMMAND {
                symbol = parser.get_symbol(line.to_string());
            }

            let dest = parser.get_dest(line.to_string());
            println!("{} com:{} dest:{} sym:{}", line.to_string(), command_type, dest.to_string(), symbol.to_string());
        }
    }
}
