use std::collections::{HashMap, HashSet};

use super::{ast::{self, ComplexTypes, FieldType, Package, PrimitiveTypes, StructField}, token_types::TokenTypes, tokenizer::{Token, Tokenizer}};

pub struct Parser {
    tokenizer: Tokenizer,
    pub ast: ast::AST,
    index: usize,
    curly_brackets: u32,
    in_struct: Option<String>,
    in_enum: Option<String>
}

impl Parser {
    pub fn new(src: String) -> Parser {
        let mut parser = Parser {
            tokenizer: Tokenizer::new(src),
            ast: ast::AST {
                packages: Vec::new()
            },
            index: 0,
            curly_brackets: 0,
            in_struct: None,
            in_enum: None
        };

        parser.tokenizer.tokenize();

        parser
    }

    pub fn print(&self) {
        println!("{:?}", self.tokenizer.structs);
        println!("{:?}", self.tokenizer.enums);

        for p in &self.ast.packages {
            println!("{:?} {:?}", p.name, p.version);
            
            for s in p.structs.values() {
                println!("{:?}:", s.name);
                for f in s.fields.values() {
                    println!("{:?} {:?} [{:?}]", f.name, f.t, f.array);
                }
            }
    
            for e in p.enums.values() {
                println!("{:?}:", e.name);
                for v in &e.variants_order {
                    println!("{:?} {:?}", v.name, v.value)
                }
            }
        }
    }

    /// Exit the program with an error
    fn error(&self, msg: &str, code: u32, row: u32, col: u32) {
        panic!("Parser:{}:{} Error E{}: {}", row, col, code, msg);
    }

    pub fn next(&mut self) -> &Token {
        self.index += 1;
        self.tokenizer.tokens.get(self.index).unwrap()
    }

    /// Create the AST used for code generation
    pub fn generate_ast(&mut self) {
        self.ast.packages.push(Package {
            name: None,
            version: None,
            structs: HashMap::new(),
            enums: HashMap::new()
        });
        
        while self.index < self.tokenizer.tokens.len() {
            let token = self.tokenizer.tokens.get(self.index).unwrap();
            match token.t {
                TokenTypes::CloseCurlyBracket => {
                    if self.curly_brackets == 0 {
                        self.error("Unexpected closed curly bracket `}`.", 1, token.row, token.col);
                    }
                    
                    self.curly_brackets -= 1;
                    self.in_struct = None;
                    self.in_enum = None;
                },
                TokenTypes::Version => self.version(),
                TokenTypes::Package => self.package(),
                TokenTypes::Struct => self.structure(),
                TokenTypes::Enum => self.enumeration(),
                TokenTypes::Identifier => {
                    if self.in_struct != None {
                        self.struct_field();
                    }
                    else if self.in_enum != None {
                        self.enum_variant();
                    }
                    else {
                        let token = self.tokenizer.tokens.get(self.index).unwrap();
                        self.error("Unexpected token.", 1, token.row, token.col);
                    }
                },
                TokenTypes::EndOfStream => break,
                _ => self.error("Unexpected token.", 1, token.row, token.col)
            }

            self.index += 1;
        }

        if self.curly_brackets > 0 {
            self.error("Opened curly brackets not closed.", 1, 0, 0);
        }
    }

    /// Read the version declaration
    fn version(&mut self) {
        if self.ast.packages.last().unwrap().version != None {
            let token = self.tokenizer.tokens.get(self.index).unwrap();
            self.error("Version already declared.", 1, token.row, token.col);
        }
        self.index += 1;

        let token = self.tokenizer.tokens.get(self.index).unwrap();
        if token.t == TokenTypes::SemanticVersion {
            self.ast.packages.last_mut().unwrap().version = Some(token.value.clone().unwrap());
            
            self.index += 1;

            let token = self.tokenizer.tokens.get(self.index).unwrap();
            if token.t != TokenTypes::SemiColon {
                self.error("Expected semicolon.", 1, token.row, token.col);
            }
        }
        else {
            self.error("Expected semantic version after keyword `version`.", 1, token.row, token.col);
        }
    }

    /// Read the declaration of the package name
    fn package(&mut self) {
        if self.ast.packages.last().unwrap().name != None {
            let token = self.tokenizer.tokens.get(self.index).unwrap();
            self.error("Package name already declared.", 1, token.row, token.col);
        }
        self.index += 1;

        let token = self.tokenizer.tokens.get(self.index).unwrap();
        if token.t == TokenTypes::Identifier {
            self.ast.packages.last_mut().unwrap().name = Some(token.value.clone().unwrap());
        
            self.index += 1;

            let token = self.tokenizer.tokens.get(self.index).unwrap();
            if token.t != TokenTypes::SemiColon {
                self.error("Expected semicolon.", 1, token.row, token.col);
            }
        }
        else {
            self.error("Expected identifier after keyword `package`.", 1, token.row, token.col);
        }
    }

    /// Create a node representing a struct
    fn structure(&mut self) {
        let token = self.tokenizer.tokens.get(self.index).unwrap();
        if self.curly_brackets > 0 {
            self.error("Curly bracket not closed.", 1, token.row, token.col);
        }
        self.index += 1;

        let token = self.tokenizer.tokens.get(self.index).unwrap();
        if token.t == TokenTypes::Identifier {
            self.index += 1;
            let name = token.value.as_ref().unwrap().clone();

            let token = self.tokenizer.tokens.get(self.index).unwrap();
            if token.t == TokenTypes::OpenCurlyBracket {
                if self.ast.packages.last().unwrap().structs.contains_key(&name) || self.ast.packages.last().unwrap().enums.contains_key(&name) {
                    self.error("Name already used.", 1, token.row, token.col);
                }
                else {
                    self.ast.packages.last_mut().unwrap().structs.insert(name.clone(), ast::Struct {
                        name: name.clone(),
                        fields: HashMap::new(),
                        fields_order: Vec::new()
                    });
                }
                self.curly_brackets += 1;
                self.in_struct = Some(name);
            }
            else {
                self.error("Expected `{` after the identifier of struct.", 1, token.row, token.col);
            }
        }
        else {
            self.error("Expected identifier after keyword `struct`.", 1, token.row, token.col);
        }
    }

    /// Create a node representing an enum 
    fn enumeration(&mut self) {
        let token = self.tokenizer.tokens.get(self.index).unwrap();
        if self.curly_brackets > 0 {
            self.error("Curly bracket not closed.", 1, token.row, token.col);
        }
        self.index += 1;

        let token = self.tokenizer.tokens.get(self.index).unwrap();
        if token.t == TokenTypes::Identifier {
            self.index += 1;
            let name = token.value.as_ref().unwrap().clone();

            let token = self.tokenizer.tokens.get(self.index).unwrap();
            if token.t == TokenTypes::OpenCurlyBracket {
                if self.ast.packages.last().unwrap().structs.contains_key(&name) || self.ast.packages.last().unwrap().enums.contains_key(&name) {
                    self.error("Name already used.", 1, token.row, token.col);
                }
                else {
                    self.ast.packages.last_mut().unwrap().enums.insert(name.clone(), ast::Enum {
                        name: name.clone(),
                        variants: HashSet::new(),
                        variants_order: Vec::new()
                    });
                }
                self.curly_brackets += 1;
                self.in_enum = Some(name);
            }
            else {
                self.error("Expected `{` after the identifier of an enum.", 1, token.row, token.col);
            }
        }
        else {
            self.error("Expected identifier after keyword `enum`.", 1, token.row, token.col);
        }
    }

    /// Add the field to the struct
    fn struct_field(&mut self) {
        let mut array: Option<u32> = None;
        let mut name: String = String::new();

        let var_type = self.tokenizer.tokens.get(self.index).unwrap();
        let field_type = match var_type.value.clone().unwrap().as_str() {
            "u8" | "u16" | "u32" | "i8" | "i16" | "i32" | "f32" | "bool" => FieldType::PRIMITIVE(PrimitiveTypes::new(var_type.value.clone().unwrap())),
            _ => {
                if self.tokenizer.structs.contains(&var_type.value.clone().unwrap()) {
                    FieldType::COMPLEX(ComplexTypes::Struct(var_type.value.clone().unwrap()))
                }
                else if self.tokenizer.enums.contains(&var_type.value.clone().unwrap()) {
                    FieldType::COMPLEX(ComplexTypes::Enum(var_type.value.clone().unwrap()))
                }
                else {
                    FieldType::COMPLEX(ComplexTypes::Unknown(var_type.value.clone().unwrap()))
                }
            }
        };

        self.index += 1;
        let token = self.tokenizer.tokens.get(self.index).unwrap();
        
        if token.t == TokenTypes::OpenSquareBracket {
            self.index += 1;
            let array_size = self.tokenizer.tokens.get(self.index).unwrap();
            if array_size.t == TokenTypes::UInt {
                array = match array_size.value.as_ref().unwrap().parse() {
                    Ok(v) => Some(v),
                    Err(_) => None
                };
                if array == None {
                    self.error("Invalid index.", 1, array_size.row, array_size.col);
                }
            }
            else {
                self.error("Expected unsigned integer.", 1, array_size.row, array_size.col);
            }

            self.index += 1;
            let token = self.tokenizer.tokens.get(self.index).unwrap();
            if token.t != TokenTypes::CloseSquareBracket {
                self.error("Expected `]` but found something else.", 1, token.row, token.col);
            }

            self.index += 1;
            let token = self.tokenizer.tokens.get(self.index).unwrap();
            if token.t == TokenTypes::Identifier {
                name = token.value.clone().unwrap();
            }
            else {
                self.error("Expected an identifier.", 1, token.row, token.col);
            }
        }
        else if token.t == TokenTypes::Identifier {
            name = token.value.clone().unwrap();
        }
        else {
            self.error("Unexpected token after identifier.", 1, token.row, token.col);
        }

        self.index += 1;
        let token = self.tokenizer.tokens.get(self.index).unwrap();
        if token.t != TokenTypes::SemiColon {
            self.error("Expected a semicolon `;`.", 1, token.row, token.col);
        }

        if self.ast.packages.last().unwrap().structs.get(self.in_struct.as_ref().unwrap()).unwrap().fields.contains_key(&name) {
            self.error("Field name already used.", 1, token.row, token.col);
        }

        self.ast.packages.last_mut().unwrap()
            .structs.get_mut(self.in_struct.as_ref().unwrap()).unwrap()
            .fields
            .insert(
                name.clone(), 
                StructField {
                    name: name.clone(), 
                    t: field_type, 
                    array
                }
            );
        self.ast.packages.last_mut().unwrap()
            .structs.get_mut(self.in_struct.as_ref().unwrap()).unwrap()
            .fields_order.push(name);
    }

    /// Add the variant to the enum
    fn enum_variant(&mut self) {
        let variant_value = self.ast.packages.last().unwrap().enums.get(self.in_enum.as_ref().unwrap()).unwrap().variants_order.len();
        let name = self.tokenizer.tokens.get(self.index).unwrap();

        if self.ast.packages.last().unwrap().enums.get(self.in_enum.as_ref().unwrap()).unwrap().variants.contains(&name.value.clone().unwrap()) {
            self.error("Variant name already used.", 1, name.row, name.col);
        }

        self.index += 1;
        let token = self.tokenizer.tokens.get(self.index).unwrap();
        if token.t == TokenTypes::SemiColon {
            self.ast.packages.last_mut().unwrap()
                .enums.get_mut(self.in_enum.as_ref().unwrap()).unwrap()
                .variants.insert(name.value.clone().unwrap());
            self.ast.packages.last_mut().unwrap()
                .enums.get_mut(self.in_enum.as_ref().unwrap()).unwrap()
                .variants_order.push(ast::EnumVariant {
                    name: name.value.clone().unwrap(),
                    value: variant_value as u32
                });
        }
        else {
            self.error("Expected a semicolon `;`.", 1, token.row, token.col);
        }
    }
}