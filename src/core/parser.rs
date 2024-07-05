use std::{collections::HashMap, fs, io::Write};

use regex::Regex;

use crate::core::{field::Field, package::Package, r#struct::Struct, types::Types};

pub fn parse(file_path: &String) {
    const VERSION: &str = "0.1.0";
    
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
                            f_type: Types::from_str(var_type).unwrap(),
                            array: array_size
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