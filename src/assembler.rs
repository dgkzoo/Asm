use code::Code;
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
use std::string::ToString;
use symbol_table::SymbolTable;

pub struct Assembler {
    ram_addr: u16,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler { ram_addr: 0x0010 }
    }

    ///
    /// シンボルテーブルを作成する
    ///
    fn create_symbol_tble(&self, _filepath: String) -> SymbolTable {
        let mut st = SymbolTable::new();
        st.init();
        return st;
    }

    ///
    /// アセンブルの実行
    ///
    pub fn exec(&mut self, filepath: String) {
        let infilepath = filepath.to_string();

        // 引数のパスから.asmファイルのリストを取得する
        let file_list = self.get_file_list(&infilepath);

        for asm_file_path in file_list {
            // 入出力のパス
            let inpath = Path::new(&asm_file_path);
            let mut outfilepath = String::from(
                inpath
                    .with_file_name(inpath.file_stem().unwrap())
                    .to_str()
                    .unwrap(),
            );
            outfilepath = outfilepath + ".code";

            // シンボルテーブルの作成
            let st = self.create_symbol_tble(infilepath.to_string());

            // アセンブルの実行
            self.assemble(st, asm_file_path.to_string(), outfilepath.to_string());
            println!(
                "assemble in:{} out:{}",
                asm_file_path.to_string(),
                outfilepath
            );
        }
    }

    ///
    /// パスから.asmファイルのリストを取得する
    /// ディレクトリの場合は配下のファイルのリストを返す
    ///
    fn get_file_list(&self, inpath_str: &str) -> Vec<String> {
        let mut vec: Vec<String> = vec![];
        let inpath = Path::new(inpath_str);

        // ファイル指定
        if !inpath.is_dir() {
            vec.push(String::from(inpath.to_str().unwrap()));
            return vec;
        }

        // ディレクトリ指定
        let paths = fs::read_dir(inpath_str).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let ext = path.extension().unwrap().to_str().unwrap();
            if "asm" != ext {
                continue;
            }

            let asm_path = path.display().to_string();
            vec.push(asm_path.to_string());
        }
        return vec;
    }

    ///
    /// アセンブル
    ///
    fn assemble(&mut self, mut st: SymbolTable, filepath: String, outfilepath: String) {
        let parser = Parser::new();
        let code = Code::new();

        let infile = fs::File::open(filepath.to_string()).unwrap();
        let mut out_buf = BufWriter::new(fs::File::create(outfilepath).unwrap());

        let reader = BufReader::new(infile);
        for line in reader.lines() {
            // コメント、空白行などを除去
            let line = parser.get_valid_line(&mut line.unwrap());
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

            let mut out_code = "".to_string();

            // A命令
            if command_type == parser::A_COMMAND {
                let mut address;

                // 数値はそのままアドレス
                if symbol.parse::<u16>().is_ok() {
                    address = symbol.parse::<u16>().unwrap();
                } else {
                    // シンボルテーブルに存在したらアドレスをもらう
                    if st.contains(&symbol) {
                        address = *st.get_address(symbol);
                    } else {
                        // シンボル登録
                        st.add_entry(symbol, self.ram_addr);
                        address = self.ram_addr;
                        self.ram_addr += 1;
                    }
                }

                out_code = format!("{:0>16b} //{}\n", address, line.to_string());
            }

            // C命令
            if command_type == parser::C_COMMAND {
                // dest=comp;jmp の取得
                let comp = parser.get_comp(line.to_string());
                let dest = parser.get_dest(line.to_string());
                let jump = parser.get_jmp(line.to_string());

                // dest=comp;jmp をコード化
                let comp_code = code.comp(comp.to_string());
                let dest_code = code.dest(dest.to_string());
                let jump_code = code.jump(jump.to_string());
                let code_val = (7 << 13) | (comp_code << 6) | (dest_code << 3) | jump_code;

                out_code = format!("{:0>16b} //{}\n", code_val, line.to_string());
            }

            // ファイルに出力
            //print!("{}", out_code);
            out_buf.write(out_code.as_bytes()).unwrap();
        }
    }
}
