pub struct Code {}

impl Code {
    pub fn new() -> Code {
        Code {}
    }

    pub fn dest(&self, mnemonic: String) -> u16 {
        match mnemonic.as_str() {
            "" => 0,
            "0" => 0,
            "M" => 1,
            "D" => 2,
            "MD" => 3,
            "A" => 4,
            "AM" => 5,
            "AD" => 6,
            "AMD" => 7,
            _ => panic!("Invalid dest mnemonic: {}", mnemonic),
        }
    }

    pub fn comp(&self, mnemonic: String) -> u16 {
        match mnemonic.as_str() {
            "" => 0,
            "0" => 0b0101010,
            "1" => 0b0111111,
            "-1" => 0b0111010,
            "D" => 0b0001100,
            "A" => 0b0110000,
            "M" => 0b1110000,
            "!D" => 0b0001101,
            "!A" => 0b0110001,
            "!M" => 0b1110001,
            "-D" => 0b0001111,
            "-A" => 0b0110011,
            "-M" => 0b1110011,
            "D+1" => 0b0011111,
            "A+1" => 0b0110111,
            "M+1" => 0b1110111,
            "D-1" => 0b0001110,
            "A-1" => 0b0110010,
            "M-1" => 0b1110010,
            "D+A" => 0b0000010,
            "D+M" => 0b1000010,
            "D-A" => 0b0010011,
            "D-M" => 0b1010011,
            "A-D" => 0b0000111,
            "M-D" => 0b1000111,
            "D&A" => 0b0000000,
            "D&M" => 0b1000000,
            "D|A" => 0b0010101,
            "D|M" => 0b1010101,
            _ => panic!("Invalid comp mnemonic: {}", mnemonic),
        }
    }

    pub fn jump(&self, mnemonic: String) -> u16 {
        match mnemonic.as_str() {
            "" => 0,
            "JGT" => 1,
            "JEQ" => 2,
            "JGE" => 3,
            "JLT" => 4,
            "JNE" => 5,
            "JLE" => 6,
            "JMP" => 7,
            _ => panic!("Invalid jump mnemonic: {}", mnemonic),
        }
    }
}
