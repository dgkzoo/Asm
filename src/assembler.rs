use parser;
use parser::Parser;
use std::fs;

///
/// ## アセンブラ
///
/// ## author
/// dgkzoo
///
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use symbol_table::SymbolTable;

pub struct Assembler {}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {}
    }

    ///
    /// シンボルテーブルを作成する
    ///
    fn create_symbol_tble(&self, _filepath: String) -> SymbolTable {
        let st = SymbolTable::new();
        return st;
    }

    ///
    /// アセンブルの実行
    ///
    pub fn exec(&self, filepath: String) {
        let infilepath = filepath.to_string();
        let inpath = Path::new(&infilepath);
        let outfilepath = String::from(
            inpath
                .with_file_name(inpath.file_stem().unwrap())
                .to_str()
                .unwrap(),
        );

        let st = self.create_symbol_tble(infilepath.to_string());
        self.assemble(infilepath.to_string(), outfilepath);
    }

    fn assemble(&self, filepath: String, outfilepath: String) {
        let parser = Parser::new();

        let infile = fs::File::open(filepath.to_string()).unwrap();
        let mut out_buf = BufWriter::new(fs::File::create(outfilepath).unwrap());

        let reader = BufReader::new(infile);
        for line in reader.lines() {
            // コメント、空白行などを除去
            let line = parser.get_valid_line(line.unwrap());
            if line.is_empty() {
                continue;
            }

            // コマンドタイプの取得
            let command_type = parser.get_command_type(line.to_string());

            // シンボルの取得
            let mut symbol: String = "".to_string();
            if command_type == parser::A_COMMAND || command_type == parser::L_COMMAND {
                symbol = parser.get_symbol(line.to_string());
            }

            // dest=comp;jmp の取得
            let dest = parser.get_dest(line.to_string());
            let comp = parser.get_comp(line.to_string());
            let jmp = parser.get_jmp(line.to_string());

            let out_code = format!(
                "{} com:{} dest:{} comp:{} jmp:{} sym:{}\n",
                line.to_string(),
                command_type,
                dest.to_string(),
                comp.to_string(),
                jmp.to_string(),
                symbol.to_string()
            );

            print!("{}", out_code);
            out_buf.write(out_code.as_bytes()).unwrap();
        }
    }
}
