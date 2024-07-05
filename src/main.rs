use std::env;
use std::fs;
use std::collections::HashMap;
use std::io::Write;
use regex::Regex;



const VERSION: &str = "0.1.0";


enum Types {
    U8,
    I8,
    BOOL,
    U16,
    I16,
    U32,
    I32,
    F32
}

impl Types {
    fn from_str(s: &str) -> Result<Types, &str> {
        match s {
            "u8" => Ok(Types::U8),
            "i8" => Ok(Types::I8),
            "bool" => Ok(Types::BOOL),
            "u16" => Ok(Types::U16),
            "i16" => Ok(Types::I16),
            "u32" => Ok(Types::U32),
            "i32" => Ok(Types::I32),
            "f32" => Ok(Types::F32),
            _ => Err("No compatible type found.")
        }
    }

    fn size(&self) -> u32 {
        match self {
            Types::U8 | Types::I8 | Types::BOOL => 1,
            Types::U16 | Types::I16 => 2,
            Types::U32 | Types::I32 | Types::F32 => 4
        }
    }

    fn to_string(&self) -> String {
        match self {
            Types::U8 => String::from("u8"),
            Types::I8 => String::from("i8"),
            Types::BOOL => String::from("bool"),
            Types::U16 => String::from("u16"),
            Types::I16 => String::from("i16"),
            Types::U32 => String::from("u32"),
            Types::I32 => String::from("i32"),
            Types::F32 => String::from("f32")
        }
    }
}

struct Field {
    name: String,
    wire_type: Types,
    array: Option<u32>,
    size: u32
}

impl Field {
    fn gen_declaration(&self) -> String {
        let mut out = String::new();

        out.push_str(format!("pub {}: ", self.name).as_str());
        out.push_str(match self.array {
            Some(n) => {
                format!("[{}; {}],\n", self.wire_type.to_string(), n)
            },
            None => {
                format!("{},\n", self.wire_type.to_string())
            }
        }.as_str());

        out
    }

    fn gen_encode(&self) -> String {
        let mut out = String::new();

        match self.array {
            Some(_n) => {
                out.push_str(format!("for i in self.{} {{\n", self.name).as_str());
                out.push_str("for x in i.to_be_bytes() {\n");
                out.push_str("data[index] = x;\n");
                out.push_str("index += 1;\n");
                out.push_str("}\n");
                out.push_str("}\n");
            },
            None => {
                out.push_str(format!("for x in self.{}.to_be_bytes() {{\n", self.name).as_str());
                out.push_str("data[index] = x;\n");
                out.push_str("index += 1;\n");
                out.push_str("}\n");
            }
        }

        out
    }

    fn gen_default(&self) -> String {
        let mut out = String::new();

        out.push_str(format!("{}: ", self.name).as_str());
        out.push_str(match self.array {
            Some(n) => format!("[{}; {}]", match self.wire_type {
                Types::U8 | Types::U16 | Types::U32 | Types::I8 | Types::I16 | Types::I32 => "0",
                Types::BOOL => "false",
                Types::F32 => "0.0"
            }, n),
            None => format!("{}", match self.wire_type {
                Types::U8 | Types::U16 | Types::U32 | Types::I8 | Types::I16 | Types::I32 => "0",
                Types::BOOL => "false",
                Types::F32 => "0.0"
            })
        }.as_str());
        out.push_str(",\n");

        out
    }

    fn gen_from_bytes(&self) -> String {
        let mut out = String::new();

        match self.array {
            Some(n) => {
                out.push_str(format!("for i in 0..{} {{\n", n).as_str());
                out.push_str(format!("out.{}[i] = {}::from_be_bytes(data[index..index+{}].try_into().unwrap());\n", self.name, self.wire_type.to_string(), self.wire_type.size()).as_str());
                out.push_str(format!("index += {};\n", self.wire_type.size()).as_str());
                out.push_str("}\n");
            },
            None => {
                out.push_str(format!("out.{} = {}::from_be_bytes(data[index..index+{}].try_into().unwrap());\n", self.name, self.wire_type.to_string(), self.wire_type.size()).as_str());
                out.push_str(format!("index += {};\n", self.wire_type.size()).as_str());
            }
        }

        out
    }
}

struct Struct {
    name: String,
    fields: HashMap<String, Field>,
    fields_order: Vec<String>,
    size: u32
}

impl Struct {
    /*
    * Simple, non-cryptographic hashing algorithm.
    * Reference: https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV-1a_hash
    */
    fn fnv_1a(&self) -> u32 {
        let s = self.name.to_string();

        let mut hash = 2166136261;
        let fnv_prime = 16777619;

        for i in s.bytes() {
            hash ^= i as u32;
            hash = hash.wrapping_mul(fnv_prime);
        }

        hash
    }

    fn gen_code(&self) -> String {
        let mut out = String::new();

        out.push_str(format!("pub struct {} {{\n", self.name).as_str());
        for f in &self.fields_order {
            out.push_str(self.fields.get(f).unwrap().gen_declaration().as_str());
        }
        out.push_str("}\n");

        out.push_str(format!("impl {} {{\n", self.name).as_str());

        out.push_str(format!("pub const NAME_HASH: u32 = {};\n", self.fnv_1a()).as_str());
        out.push_str(format!("pub const BYTES_LENGTH: usize = {} + 4;\n", self.size).as_str());
        
        out.push_str(format!("pub fn encode(&self) -> [u8; {}::BYTES_LENGTH] {{\n", self.name).as_str());

        out.push_str(format!("let mut data: [u8; {}::BYTES_LENGTH] = [0; {}::BYTES_LENGTH];", self.name, self.name).as_str());
        out.push_str(format!("let mut index = 0;").as_str());

        out.push_str(format!("for x in u32::to_be_bytes({}::NAME_HASH) {{\n", self.name).as_str());
        out.push_str("data[index] = x;\n");
        out.push_str("index += 1;\n");
        out.push_str("}\n");

        for f in &self.fields_order {
            out.push_str(self.fields.get(f).unwrap().gen_encode().as_str());
        }

        out.push_str("data\n");

        out.push_str("}\n");

        out.push_str(format!("pub fn to_be_bytes(&self) -> [u8; {}::BYTES_LENGTH] {{\n", self.name).as_str());
        out.push_str("self.encode()\n");
        out.push_str("}\n");

        out.push_str(format!("pub fn decode(data: &[u8]) -> {} {{\n", self.name).as_str());
        
        out.push_str(format!("let mut out = {} {{\n", self.name).as_str());
        for f in &self.fields_order {
            out.push_str(self.fields.get(f).unwrap().gen_default().as_str());
        }
        out.push_str("};\n");

        out.push_str("let mut index = 4;\n");

        for f in &self.fields_order {
            out.push_str(self.fields.get(f).unwrap().gen_from_bytes().as_str());
        }

        out.push_str("out\n");

        out.push_str("}\n");

        out.push_str("}\n");

        out
    }
}

struct Package {
    version: Option<String>,
    name: Option<String>,
    structs: HashMap<String, Struct>
}

impl Package {
    fn is_some(&self) -> bool {
        self.version.is_some() && self.name.is_some()
    }


    fn gen_code(&self) -> String {
        let mut out = String::new();

        out.push_str(format!("pub mod {} {{\n", self.name.as_ref().unwrap()).as_str());

        for s in self.structs.values() {
            out.push_str(s.gen_code().as_str());
        }

        out.push_str("}\n");

        out
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();    
    
    if args.len() != 2 {
        panic!("Excpected file path.");
    }
    
    let file_path = &args[1];
    let cnt: String = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(err) => panic!("{}", err)
    };


    let version_regex = Regex::new(r"^(?<version>version) +(?<number>[0-9]+\.[0-9]+\.[0-9]+) *(#.*)?$").unwrap();
    let package_regex = Regex::new(r"^(?<package>package) +(?<name>[_a-zA-Z][_a-zA-Z0-9]*) *(#.*)?$").unwrap();
    let struct_regex = Regex::new(r"^(?<struct>struct) +(?<name>[_a-zA-Z][_a-zA-Z0-9]*): *(#.*)?$").unwrap();
    let field_regex = Regex::new(r"^ {4}(?<type>i8|i16|i32|u8|u16|u32|f32|bool)(?<array>\[[0-9]+\])? +(?<name>[_a-zA-Z][_a-zA-Z0-9]*) *(#.*)?$").unwrap();
    let blank_line_regex = Regex::new(r"^\s*(#.*)?$").unwrap();

    let mut package = Package {
        version: None,
        name: None,
        structs: HashMap::new()
    };

    let mut in_struct = false;
    let mut struct_name = "";
    for (i, line) in cnt.lines().enumerate() {
        if let Some(caps) = version_regex.captures(line) {
            if package.version.is_some() {
                panic!("Version already declared.");
            }
            
            let version = caps.name("number").unwrap().as_str();
            if version != VERSION {
                panic!("The current version is {VERSION}, but the file uses {version}.");
            }

            package.version = Some(version.to_string());
        }
        else if let Some(caps) = package_regex.captures(line) {
            if package.version.is_none() {
                panic!("Version not found. Before the name declaration you must place the version used.");
            }
            if package.name.is_some() {
                panic!("Error at line {}. Package name already defined.", i + 1);
            }

            let name = caps.name("name").unwrap().as_str();
            package.name = Some(name.to_string());
        }
        else if let Some(caps) = struct_regex.captures(line) {
            if !package.is_some() {
                panic!("Version and/or package name are/is missing.");
            }

            let name = caps.name("name").unwrap().as_str();
            if package.structs.contains_key(name) {
                panic!("Error at line {}. A struct with the name '{name}' already exists.", i + 1);
            }

            in_struct = true;
            struct_name = name;
            package.structs.insert(name.to_string(), Struct {
                name: name.to_string(),
                fields: HashMap::new(),
                fields_order: Vec::new(),
                size: 0
            });
        }
        else if let Some(caps) = field_regex.captures(line) {
            if !package.is_some() {
                panic!("Version and/or package name are/is missing.");
            }
            if !in_struct {
                panic!("Syntax error at line {}. Field declared outside a struct.", i + 1);
            }

            let var_type = caps.name("type").unwrap().as_str();
            let array_size = match caps.name("array") {
                Some(g) => {
                    let index_stmt = g.as_str();
                    let index_str = &index_stmt[1..index_stmt.len() - 1];
                    match index_str.parse() {
                        Ok(n) => Some(n),
                        Err(_) => panic!("Error at line {}. Invalid number for the size.", i + 1)
                    }
                },
                None => None
            };
            let name = caps.name("name").unwrap().as_str();

            match package.structs.get_mut(struct_name) {
                Some(s) => {
                    if s.fields.contains_key(&name.to_string()) {
                        panic!("Error at line {}. A field named `{}` already exists in the struct.", i + 1, name);   
                    }
                    else {
                        s.fields.insert(name.to_string(), Field {
                            name: name.to_string(),
                            wire_type: Types::from_str(var_type).unwrap(),
                            array: array_size,
                            size: Types::from_str(var_type).unwrap().size() * array_size.unwrap_or(1)
                        });
                        s.size += Types::from_str(var_type).unwrap().size() * array_size.unwrap_or(1);
                        s.fields_order.push(name.to_string());
                    }
                },
                None => panic!("Error at line {}. Struct named `{}` not found.", i + 1, struct_name)
            }
        }
        else if let Some(_caps) = blank_line_regex.captures(line) {
            
        }
        else {
            panic!("Error at line {}. Invalid syntax.", i + 1);
        }
    }

    
    let mut dist = fs::File::create("dist.rs").expect("Error while creating the file.");
    dist.write_all(b"#![no_std]\n");
    dist.write_all(b"pub mod iris {\n");

    dist.write_all(package.gen_code().as_bytes());

    dist.write(b"pub enum Structs {\n");
    for s in package.structs.values() {
        dist.write_all(format!("{}_{}({}::{}),\n", package.name.clone().unwrap(), s.name, package.name.clone().unwrap(), s.name).as_bytes());
    }
    dist.write_all(b"}\n");

    dist.write_all(b"pub fn decode(data: &[u8]) -> Result<Structs, &str> {\n");
    
    dist.write_all(b"let struct_name_hash = u32::from_be_bytes(data[0..4].try_into().unwrap());\n");
    
    dist.write_all(b"match struct_name_hash {\n");
    for s in package.structs.values() {
        dist.write_all(format!("{}::{}::NAME_HASH if data.len() == {}::{}::BYTES_LENGTH => Ok(Structs::{}_{}({}::{}::decode(&data))),\n", package.name.clone().unwrap(), s.name, package.name.clone().unwrap(), s.name, package.name.clone().unwrap(), s.name, package.name.clone().unwrap(), s.name).as_bytes());
    }
    dist.write_all(b"_ => Err(\"Unknown data.\")\n");
    dist.write_all(b"}\n");
    
    dist.write_all(b"}\n");

    dist.write_all(b"}\n");
}