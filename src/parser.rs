///
/// ## Parser
///
/// ## author
/// dgkzoo
///
// use std::fs;
// use std::io::{Read, BufReader, BufRead};
pub const A_COMMAND: &str = "A";
pub const C_COMMAND: &str = "C";
pub const L_COMMAND: &str = "L";

///
/// パーサ
///
pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    ///
    /// 空白行、コメントの場合、ブランク文字列を返す
    ///
    pub fn get_valid_line(&self, line: &mut String) -> String {
        let mut line = line.trim().to_string();

        if line.is_empty() {
            return "".to_string();
        }
        if line.starts_with("//") {
            return "".to_string();
        }

        if line.contains("//") {
            let comment_line = line;
            let sep: Vec<&str> = comment_line.split("//").collect();
            line = sep.get(0).unwrap().to_string();
        }

        return line.trim().to_string();
    }

    ///
    /// コマンドタイプを返す
    ///
    pub fn get_command_type(&self, line: String) -> &str {
        if line.starts_with("@") {
            return A_COMMAND;
        }
        if line.contains("=") {
            return C_COMMAND;
        }
        if line.contains(";") {
            return C_COMMAND;
        }

        return L_COMMAND;
    }

    ///
    /// シンボルを抽出する
    ///
    pub fn get_symbol(&self, line: String) -> String {
        let line = line.trim().to_string();
        if line.contains("@") {
            return line.replace("@", "");
        }
        if line.contains("(") {
            return line.replace("(|)", "");
        }

        return line;
    }

    ///
    /// C命令のdestニーモニックを返す
    ///
    pub fn get_dest(&self, line: String) -> String {
        let line = line.trim().to_string();
        let sep: Vec<&str> = line.split("=").collect();
        if sep.len() == 2 {
            return sep.get(0).unwrap().to_string();
        }

        return "".to_string();
    }

    ///
    /// C命令のcompニーモニックを返す
    ///
    pub fn get_comp(&self, line: String) -> String {
        let line = line.trim().to_string();
        let sep: Vec<&str> = line.split("=").collect();
        if sep.len() == 2 {
            return sep.get(1).unwrap().to_string();
        }

        let comp_sep: Vec<&str> = line.split(";").collect();
        if comp_sep.len() == 2 {
            return comp_sep.get(0).unwrap().to_string();
        }

        return "".to_string();
    }

    ///
    /// C命令のjumpニーモニックを返す
    ///
    pub fn get_jmp(&self, line: String) -> String {
        let line = line.trim().to_string();
        let sep: Vec<&str> = line.split(";").collect();
        if sep.len() == 2 {
            return sep.get(1).unwrap().to_string();
        }

        return "".to_string();
    }
}
