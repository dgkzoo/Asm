/**
 * ## アセンブラ・メイン
 *
 * ## author
 * dgkzoo
 */
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
    let filename = &args[1];
    println!("fileName = {}", filename);

    let _assembler = Assembler {};
}
