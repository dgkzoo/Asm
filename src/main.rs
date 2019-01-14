///
/// ## アセンブラ・メイン
///
/// ## author
/// dgkzoo
///
use std::env;
extern crate asm;
use asm::assembler::Assembler;
use asm::symbol_table::SymbolTable;

fn main() {
    // 引数チェック
    if env::args().len() != 2 {
        println!("引数が不正です。*.asmファイルを１つ指定してください");
        return;
    }

    // asmファイル
    let args: Vec<String> = env::args().collect();
//    let filepath = &args[1];
    let filepath = "./test/add/Add.asm";
    println!("filepath = {}", filepath);

    let assembler = Assembler::new();
    let st = assembler.create_symbol_tble(filepath.to_string());
    assembler.exec(filepath.to_string(), st);
}
