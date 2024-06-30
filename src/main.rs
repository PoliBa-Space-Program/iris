use std::env;
use std::fs;
use std::collections::HashMap;
use std::io::Write;
use regex::Regex;



const VERSION: &str = "0.1.0";


enum WireTypes {
    U8,
    I8,
    BOOL,
    U16,
    I16,
    U32,
    I32,
    F32
}

impl WireTypes {
    fn str_to_wire_type(s: &str) -> Result<WireTypes, &str> {
        match s {
            "u8" => Ok(WireTypes::U8),
            "i8" => Ok(WireTypes::I8),
            "bool" => Ok(WireTypes::BOOL),
            "u16" => Ok(WireTypes::U16),
            "i16" => Ok(WireTypes::I16),
            "u32" => Ok(WireTypes::U32),
            "i32" => Ok(WireTypes::I32),
            "f32" => Ok(WireTypes::F32),
            _ => Err("No compatible wire type found.")
        }
    }

    fn size_wire_type(t: &WireTypes) -> u32 {
        match t {
            WireTypes::U8 | WireTypes::I8 | WireTypes::BOOL => 1,
            WireTypes::U16 | WireTypes::I16 => 2,
            WireTypes::U32 | WireTypes::I32 | WireTypes::F32 => 4
        }
    }

    fn wire_type_to_string(t: &WireTypes) -> String {
        match t {
            WireTypes::U8 => String::from("u8"),
            WireTypes::I8 => String::from("i8"),
            WireTypes::BOOL => String::from("bool"),
            WireTypes::U16 => String::from("u16"),
            WireTypes::I16 => String::from("i16"),
            WireTypes::U32 => String::from("u32"),
            WireTypes::I32 => String::from("i32"),
            WireTypes::F32 => String::from("f32")
        }
    }
}

struct Field {
    number: u32,
    name: String,
    wire_type: WireTypes,
    array: Option<u32>,
    size: u32
}

impl Field {
    fn gen_declaration(&self) -> String {
        let mut out = String::new();

        out.push_str(format!("{}: ", self.name).as_str());
        out.push_str(match self.array {
            Some(n) => {
                format!("[{}; {}],\n", WireTypes::wire_type_to_string(&self.wire_type), n)
            },
            None => {
                format!("{},\n", WireTypes::wire_type_to_string(&self.wire_type))
            }
        }.as_str());

        out
    }
}

struct Struct {
    name: String,
    fields: Vec<Field>,
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
        let mut opened_parenthesis: Vec<&str> = Vec::new();

        out.push_str(format!("pub mod {} {{\n", self.name).as_str());
        opened_parenthesis.push("{");

        out.push_str(format!("const {}_NAME_HASH: u32 = {};\n", self.name.to_uppercase(), self.fnv_1a()).as_str());
        out.push_str(format!("const {}_LENGTH_BYTES: u32 = {} + 4;\n", self.name.to_uppercase(), self.size).as_str());

        out.push_str(format!("pub struct {} {{\n", self.name).as_str());
        opened_parenthesis.push("{");

        for f in self.fields.iter() {
            out.push_str(f.gen_declaration().as_str());
        }

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
        let mut opened_parenthesis: Vec<&str> = Vec::new();

        out.push_str(format!("pub mod {} {{\n", self.name.as_ref().unwrap()).as_str());
        opened_parenthesis.push("{");

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
                panic!("Duplicate version declaration.");
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

            let name = caps.name("name").unwrap().as_str();
            package.name = Some(name.to_string());
        }
        else if let Some(caps) = struct_regex.captures(line) {
            if !package.is_some() {
                panic!("Version and/or package name are/is missing.");
            }

            let name = caps.name("name").unwrap().as_str();
            if package.structs.contains_key(name) {
                panic!("Eror at line {}. A struct with the name '{name}' already exists.", i + 1);
            }

            in_struct = true;
            struct_name = name;
            package.structs.insert(name.to_string(), Struct {
                name: name.to_string(),
                fields: Vec::new(),
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
                    s.fields.push(Field {
                        number: s.fields.len() as u32,
                        name: name.to_string(),
                        wire_type: WireTypes::str_to_wire_type(var_type).unwrap(),
                        array: array_size,
                        size: WireTypes::size_wire_type(&WireTypes::str_to_wire_type(var_type).unwrap()) * array_size.unwrap_or(1)
                    });
                    s.size += s.fields.last().unwrap().size;
                },
                None => panic!("Error at line {}. Struct named `{}` not found.", i + 1, struct_name)
            }
        }
        else if let Some(caps) = blank_line_regex.captures(line) {
            
        }
        else {
            panic!("Invalid syntax at line {}", i + 1);
        }
    }

    
    let mut dist = fs::File::create("dist.rs").expect("Error while creating the file.");
    dist.write_all(package.gen_code().as_bytes());
}