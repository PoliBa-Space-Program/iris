use super::{ast::{self, Package}, token_types::TokenTypes, tokenizer::Tokenizer};

pub struct Parser {
    tokenizer: Tokenizer,
    ast: ast::AST,
    index: usize
}

impl Parser {
    pub fn new(src: String) -> Parser {
        let mut parser = Parser {
            tokenizer: Tokenizer::new(src),
            ast: ast::AST {
                packages: Vec::new()
            },
            index: 0
        };

        parser.tokenizer.tokenize();

        parser
    }

    /// Exit the program with an error
    fn error(&self, msg: &str, code: u32, row: usize, col: usize) {
        panic!("Parser:{}:{} Error E{}: {}", row, col, code, msg);
    }

    pub fn generate_ast(&mut self) {
        self.ast.packages.push(Package {
            name: String::new(),
            version: String::new(),
            structs: Vec::new(),
            enums: Vec::new()
        });
        
        while self.index < self.tokenizer.tokens.len() {
            match self.tokenizer.tokens.get(self.index).unwrap().t {
                TokenTypes::Version => self.version(),
                TokenTypes::Package => self.package(),
                TokenTypes::Struct => self.structure(),
                TokenTypes::Enum => self.enumeration(),
                TokenTypes::EndOfStream => {},
                _ => {}
            }

            self.index += 1;
        }
    }

    fn version(&mut self) {
        self.index += 1;

        let token = self.tokenizer.tokens.get(self.index).unwrap();
        if token.t == TokenTypes::SemanticVersion {
            self.ast.packages.last_mut().unwrap().version = token.value.clone().unwrap();
            
            self.index += 1;

            let token = self.tokenizer.tokens.get(self.index).unwrap();
            if token.t != TokenTypes::SemiColon {
                self.error("Expected line feed.", 1, token.row, token.col);
            }
        }
        else {
            self.error("Expected semantic version after keyword `version`.", 1, token.row, token.col);
        }
    }

    fn package(&mut self) {
        self.index += 1;

        let token = self.tokenizer.tokens.get(self.index).unwrap();
        if token.t == TokenTypes::Identifier {
            self.ast.packages.last_mut().unwrap().version = token.value.clone().unwrap();
        }
        else {
            self.error("Expected identifier after keyword `package`.", 1, token.row, token.col);
        }
    }

    fn structure(&mut self) {
        self.index += 1;

        let token = self.tokenizer.tokens.get(self.index).unwrap();
        if token.t == TokenTypes::Identifier {
            self.index += 1;
            let name = token.value.as_ref().unwrap().clone();

            let token = self.tokenizer.tokens.get(self.index).unwrap();
            if token.t == TokenTypes::Colon {
                self.ast.packages.last_mut().unwrap().structs.push(ast::Struct {
                    name,
                    fields: Vec::new()
                });
            }
            else {
                self.error("Expected `:` after the identifier of struct.", 1, token.row, token.col);
            }
        }
        else {
            self.error("Expected identifier after keyword `struct`.", 1, token.row, token.col);
        }
    }

    fn enumeration(&mut self) {
        self.index += 1;

        let token = self.tokenizer.tokens.get(self.index).unwrap();
        if token.t == TokenTypes::Identifier {
            self.index += 1;
            let name = token.value.as_ref().unwrap().clone();

            let token = self.tokenizer.tokens.get(self.index).unwrap();
            if token.t == TokenTypes::Colon {
                self.ast.packages.last_mut().unwrap().enums.push(ast::Enum {
                    name,
                    variants: Vec::new()
                });
            }
            else {
                self.error("Expected `:` after the identifier of an enum.", 1, token.row, token.col);
            }
        }
        else {
            self.error("Expected identifier after keyword `enum`.", 1, token.row, token.col);
        }
    }
}