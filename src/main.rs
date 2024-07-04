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
    name: String,
    wire_type: WireTypes,
    array: Option<u32>,
    size: u32
}

impl Field {
    fn gen_declaration(&self) -> String {
        let mut out = String::new();

        out.push_str(format!("pub {}: ", self.name).as_str());
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
                WireTypes::U8 | WireTypes::U16 | WireTypes::U32 | WireTypes::I8 | WireTypes::I16 | WireTypes::I32 => "0",
                WireTypes::BOOL => "false",
                WireTypes::F32 => "0.0"
            }, n),
            None => format!("{}", match self.wire_type {
                WireTypes::U8 | WireTypes::U16 | WireTypes::U32 | WireTypes::I8 | WireTypes::I16 | WireTypes::I32 => "0",
                WireTypes::BOOL => "false",
                WireTypes::F32 => "0.0"
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
                out.push_str(format!("out.{}[i] = utils::from_be_bytes_{}(&data[index..index+4]).unwrap();\n", self.name, WireTypes::wire_type_to_string(&self.wire_type)).as_str());
                out.push_str("index += 4;\n");
                out.push_str("}\n");
            },
            None => {
                out.push_str(format!("out.{} = utils::from_be_bytes_{}(&data[index..index+4]).unwrap();\n", self.name, WireTypes::wire_type_to_string(&self.wire_type)).as_str());
                out.push_str("index += 4;\n");
            }
        }

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

        out.push_str(format!("pub struct {} {{\n", self.name).as_str());
        for f in self.fields.iter() {
            out.push_str(f.gen_declaration().as_str());
        }
        out.push_str("}\n");

        out.push_str(format!("impl {} {{\n", self.name).as_str());

        out.push_str(format!("pub const NAME_HASH: u32 = {};\n", self.fnv_1a()).as_str());
        out.push_str(format!("pub const BYTES_LENGTH: usize = {} + 4;\n", self.size).as_str());
        
        out.push_str(format!("pub fn encode(&self) -> [u8; {}] {{\n", self.size + 4).as_str());

        out.push_str(format!("let mut data: [u8; {}] = [0; {}];", self.size + 4, self.size + 4).as_str());
        out.push_str(format!("let mut index = 0;").as_str());

        out.push_str(format!("for x in u32::to_be_bytes({}) {{\n", self.fnv_1a()).as_str());
        out.push_str("data[index] = x;\n");
        out.push_str("index += 1;\n");
        out.push_str("}\n");

        for f in self.fields.iter() {
            out.push_str(f.gen_encode().as_str());
        }

        out.push_str("data\n");

        out.push_str("}\n");

        out.push_str(format!("pub fn to_be_bytes(&self) -> [u8; {}] {{\n", self.size + 4).as_str());
        out.push_str("self.encode()\n");
        out.push_str("}\n");

        out.push_str(format!("pub fn decode(data: &[u8]) -> {} {{\n", self.name).as_str());
        
        out.push_str(format!("let mut out = {} {{\n", self.name).as_str());
        for f in self.fields.iter() {
            out.push_str(f.gen_default().as_str());
        }
        out.push_str("};\n");

        out.push_str("let mut index = 4;\n");

        for f in self.fields.iter() {
            out.push_str(f.gen_from_bytes().as_str());
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

        out.push_str("use super::utils;\n");

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
        else if let Some(_caps) = blank_line_regex.captures(line) {
            
        }
        else {
            panic!("Invalid syntax at line {}", i + 1);
        }
    }

    
    let mut dist = fs::File::create("dist.rs").expect("Error while creating the file.");
    dist.write_all(b"#![no_std]\n");
    dist.write_all(b"pub mod iris {\n");
    
    dist.write_all(b"pub mod utils {\n");
    dist.write_all(b"pub fn from_be_bytes_u8(a: &[u8]) -> Result<u8, &str> {match a.len() { 1 => Ok(a[0]), _ => Err(\"The length must be 1.\") }}
        pub fn from_be_bytes_u16(a: &[u8]) -> Result<u16, &str> {match a.len() { 2 => Ok(((a[0] as u16) << 8) | ((a[1] as u16) << 0)), _ => Err(\"The length must be 2.\") }}
        pub fn from_be_bytes_u32(a: &[u8]) -> Result<u32, &str> {match a.len() { 4 => Ok(((a[0] as u32) << 24) | ((a[1] as u32) << 16) | ((a[2] as u32) << 8) | ((a[3] as u32) << 0)), _ => Err(\"The length must be 4.\") }}
        pub fn from_be_bytes_i8(a: &[u8]) -> Result<i8, &str> {match a.len() { 1 => Ok(a[0] as i8), _ => Err(\"The length must be 1.\") }}
        pub fn from_be_bytes_i16(a: &[u8]) -> Result<i16, &str> {match a.len() { 2 => Ok(((a[0] as i16) << 8) | ((a[1] as i16) << 0)), _ => Err(\"The length must be 2.\") }}
        pub fn from_be_bytes_i32(a: &[u8]) -> Result<i32, &str> {match a.len() { 4 => Ok(((a[0] as i32) << 24) | ((a[1] as i32) << 16) | ((a[2] as i32) << 8) | ((a[3] as i32) << 0)), _ => Err(\"The length must be 4.\") }}
        pub fn from_be_bytes_f32(a: &[u8]) -> Result<f32, &str> {match a.len() { 4 => Ok((((a[0] as u32) << 24) | ((a[1] as u32) << 16) | ((a[2] as u32) << 8) | ((a[3] as u32) << 0)) as f32), _ => Err(\"The length must be 4.\") }}
        pub fn from_be_bytes_bool(a: &[u8]) -> Result<bool, &str> {match a.len() { 1 => Ok(a[0] != 0), _ => Err(\"The length must be 1.\") }}\n");
    dist.write_all(b"}\n");

    dist.write_all(package.gen_code().as_bytes());

    dist.write(b"pub enum DecodeRes {\n");
    for s in package.structs.values() {
        dist.write_all(format!("{}_{}({}::{}),\n", package.name.clone().unwrap(), s.name, package.name.clone().unwrap(), s.name).as_bytes());
    }
    dist.write_all(b"}\n");

    dist.write_all(b"pub fn decode(data: &[u8]) -> Result<DecodeRes, &str> {\n");
    
    dist.write_all(b"let struct_name_hash = ((data[0] as u32) << 24) | ((data[1] as u32) << 16) | ((data[2] as u32) << 8) | ((data[3] as u32) << 0);\n");
    
    dist.write_all(b"match struct_name_hash {\n");
    for s in package.structs.values() {
        dist.write_all(format!("{}::{}::NAME_HASH if data.len() == {}::{}::BYTES_LENGTH => Ok(DecodeRes::{}_{}({}::{}::decode(&data))),\n", package.name.clone().unwrap(), s.name, package.name.clone().unwrap(), s.name, package.name.clone().unwrap(), s.name, package.name.clone().unwrap(), s.name).as_bytes());
    }
    dist.write_all(b"_ => Err(\"Unknown data.\")\n");
    dist.write_all(b"}\n");
    
    dist.write_all(b"}\n");

    dist.write_all(b"}\n");
}