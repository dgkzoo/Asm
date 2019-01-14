///
/// ## Parser
///
/// ## author
/// dgkzoo
///
use std::fs;
use std::io::{BufReader, BufRead};

pub struct Parser {
    filepath:String,
    curent_line:String,
}

impl Parser {
    pub fn new(filepath:String) -> Parser {
        Parser {
            filepath: filepath,
            curent_line: "".to_string(),
        }
    }

    ///
    /// １ファイルをアセンブルする
    ///
    pub fn advance(&self) -> bool {
        let file = fs::File::open(self.filepath.to_string()).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            println!("{}", line.unwrap()); // 改行は含まない
        }

        return true;
    }
}
