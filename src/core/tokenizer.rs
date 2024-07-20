use std::collections::HashSet;

use super::token_types::TokenTypes;


pub struct Token {
    pub t: TokenTypes,
    pub value: Option<String>,
    pub row: u32,
    pub col: u32
}

pub struct Tokenizer {
    src: String,
    pub tokens: Vec<Token>,
    pub structs: HashSet<String>,
    pub enums: HashSet<String>,
    row: u32,
    col: u32,
    pos: u32,
    current_c: Option<char>
}

impl Tokenizer {
    /// Constructor
    pub fn new(src: String) -> Tokenizer {
        Tokenizer { 
            src: src.clone(),
            tokens: Vec::new(),
            structs: HashSet::new(),
            enums: HashSet::new(),
            row: 1,
            col: 1,
            pos: 0,
            current_c: Some(src.chars().nth(0).unwrap())
        }
    }

    pub fn print(&self) {
        for t in &self.tokens {
            println!("{:?} {:?}", t.t, t.value);
        }
    }

    /// Divide the source file into tokens and save them in a vector
    pub fn tokenize(&mut self) {
        let mut token = self.get_next_token();
        self.tokens.push(token);

        while self.tokens.last().unwrap().t != TokenTypes::EndOfStream {
            token = self.get_next_token();
            self.tokens.push(token);
        }
    }

    /// Get the next token in the input
    pub fn get_next_token(&mut self) -> Token {
        while let Some(c) = self.current_c {
            if c.is_ascii_whitespace() {
                self.skip_whitespace();
                continue;
            }
            if c == '#' {
                self.advance();
                self.skip_comment();
                continue;
            }
            if c.is_ascii_alphabetic() {
                return self.id();
            }
            if c.is_ascii_digit() {
                return self.number();
            }
            if c  == ':' {
                self.advance();
                return Token { t: TokenTypes::Colon, value: None, row: self.row, col: self.col };
            }
            if c == ';' {
                self.advance();
                return Token { t: TokenTypes::SemiColon, value: None, row: self.row, col: self.col };
            }
            if c == '[' {
                self.advance();
                return Token { t: TokenTypes::OpenSquareBracket, value: None, row: self.row, col: self.col };
            }
            if c == ']' {
                self.advance();
                return Token { t: TokenTypes::CloseSquareBracket, value: None, row: self.row, col: self.col };
            }
            if c == '{' {
                self.advance();
                return Token { t: TokenTypes::OpenCurlyBracket, value: None, row: self.row, col: self.col };
            }
            if c == '}' {
                self.advance();
                return Token { t: TokenTypes::CloseCurlyBracket, value: None, row: self.row, col: self.col };
            }
            
            self.error("Syntax error, unknown token.", 1);
        }

        return Token { t: TokenTypes::EndOfStream, value: None, row: self.row, col: self.col };
    }

    /// Exit the program with an error
    fn error(&self, msg: &str, code: u32) {
        panic!("Tokenizer:{}:{} Error E{}: {}", self.row, self.col, code, msg);
    }

    /// Advance to the next character and set the current character 
    fn advance(&mut self) {
        if self.current_c == Some('\n') {
            self.row += 1;
            self.col = 0;
        }

        self.pos += 1;
        if self.pos >= self.src.len() as u32 {
            self.current_c = None;
        }
        else {
            self.current_c = Some(self.src.chars().nth(self.pos as usize).unwrap());
            self.col += 1;
        }
    }

    /// Go to the next position and return the character
    fn peek(&self, n: u32) -> Option<char> {
        self.src.chars().nth((self.pos + n) as usize)
    }

    /// Skip whitespaces until the next token
    fn skip_whitespace(&mut self) {
        while self.current_c.unwrap().is_ascii_whitespace() {
            self.advance();
        }
    }

    /// Skip the comment
    fn skip_comment(&mut self) {
        while self.current_c.unwrap_or_default() != '\n' {
            self.advance();
        }
    }

    /// Return an unsigned integer or semantic version token consumed from the input
    fn number(&mut self) -> Token {
        let num_row = self.row;
        let num_col = self.col;

        let mut buf = String::new();
        while self.current_c.unwrap().is_ascii_digit() {
            buf.push(self.current_c.unwrap());
            self.advance();
        }

        if self.current_c.unwrap() == '.' {
            buf.push(self.current_c.unwrap());
            self.advance();

            while self.current_c.unwrap().is_ascii_digit() {
                buf.push(self.current_c.unwrap());
                self.advance();
            }

            if self.current_c.unwrap() == '.' {
                buf.push(self.current_c.unwrap());
                self.advance();

                while self.current_c.unwrap().is_ascii_digit() {
                    buf.push(self.current_c.unwrap());
                    self.advance();
                }
            }

            return Token {
                t: TokenTypes::SemanticVersion,
                value: Some(buf),
                row: num_row,
                col: num_col
            };
        }
        
        return Token {
            t: TokenTypes::UInt,
            value: Some(buf),
            row: num_row,
            col: num_col
        };
    }

    /// Return an identifier or keyword token
    fn id(&mut self) -> Token {
        let num_row = self.row;
        let num_col = self.col;

        let mut buf = String::new();
        while self.current_c.unwrap_or_default().is_ascii_alphanumeric() {
            buf.push(self.current_c.unwrap());
            self.advance();
        }

        match buf.as_str() {
            "version" => Token { t: TokenTypes::Version, value: None, row: num_row, col: num_col },
            "package" => Token { t: TokenTypes::Package, value: None, row: num_row, col: num_col },
            "struct" => Token { t: TokenTypes::Struct, value: None, row: num_row, col: num_col },
            "enum" => Token { t: TokenTypes::Enum, value: None, row: num_row, col: num_col },
            _ => {
                if self.tokens.len() > 0 {
                    if self.tokens.last().unwrap().t == TokenTypes::Struct {
                        self.structs.insert(buf.clone());
                    }
                    else if self.tokens.last().unwrap().t == TokenTypes::Enum {
                        self.enums.insert(buf.clone());
                    }
                }

                Token { t: TokenTypes::Identifier, value: Some(buf), row: num_row, col: num_col }
            }
        }
    }
}