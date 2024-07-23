pub fn error(ctx: ErrorType, msg: &str, code: u32, row: u32, col: u32) {
    panic!("{}:{}:{} Error E{}: {}", ctx.str(), row, col, code, msg);
}

pub enum ErrorType {
    Tokenizer,
    Parser,
    CodeGenerator
}

impl ErrorType {
    pub fn str(&self) -> &str {
        match self {
            ErrorType::Tokenizer => "Tokenizer",
            ErrorType::Parser => "Parser",
            ErrorType::CodeGenerator => "Code generator"
        }
    }
}