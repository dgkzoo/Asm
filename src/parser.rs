/**
 * ## Parser
 *
 * ## author
 * dgkzoo
 */
use std::fs::File;

pub struct Parser {
    file:File,
}

impl Parser {
    pub fn new(filepath:String) -> Parser {
        Parser {
            file:File::open(filepath).expect("file not found")
        }
    }

    /**
     * ファイルにコマンドが存在するか返す
     */
    pub fn has_more_commands() -> bool {
        return true;
    }
}
