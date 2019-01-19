///
/// ## アセンブラ・メイン
///
/// ## author
/// dgkzoo
///
use std::env;
extern crate asm;
use asm::assembler::Assembler;

fn main() {
    // 引数チェック
    if env::args().len() != 2 {
        println!("引数が不正です。*.asmファイルを１つ指定してください");
        return;
    }

    // asmファイル
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    //    let filepath = "./test/counter/counter.asm";
    println!("filepath = {}", filepath);

    let mut assembler = Assembler::new();
    assembler.exec(filepath.to_string());
}
