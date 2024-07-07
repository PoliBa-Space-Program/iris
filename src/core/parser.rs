use std::{collections::HashMap, fs, io::Write, path::Path};

use regex::Regex;

use crate::core::generators::code_gen::CodeGen;
use crate::core::{field::Field, package::Package, r#struct::Struct};
use crate::core::generators::rust;
use crate::core::generators::python;
use crate::core::generators::cpp;
use crate::core::generators::c;


enum Langs {
    RUST(rust::Rust),
    PYTHON(python::Python),
    CPP(cpp::Cpp),
    C(c::C)
}

impl Langs {
    fn from_string(s: &str) -> Langs {
        match s {
            "rust" | "rs" => Langs::RUST(rust::Rust {  }),
            "python" | "py" => Langs::PYTHON(python::Python {  }),
            "c++" | "cpp" => Langs::CPP(cpp::Cpp {  }),
            "c" => Langs::C(c::C {  }),
            _ => panic!("Error, specified language is not supported.")
        }
    }

    fn ext(&self) -> &str {
        match self {
            Langs::RUST(_) => "rs",
            Langs::PYTHON(_) => "py",
            Langs::CPP(_) => "hpp",
            Langs::C(_) => "h"
        }
    }
}

/*
 * The parser for semplicity read line by line the source code and execute the regex.
 * Subsequent decisions are made based on the matching regex for the given line of code.
 */
pub fn parse(file_path: &String, out_path: &String, lang: &String) {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    
    let cnt: String = fs::read_to_string(file_path).unwrap();
    if !Path::new(out_path).is_dir() {
        panic!("Invalid directory.")
    }
    let lang = Langs::from_string(lang);

    // Regex used to match the code
    let version_regex = Regex::new(r"^(?<version>version) +(?<number>(?<major>[0-9]+)\.(?<minor>[0-9]+)\.(?<patch>[0-9]+)) *(#.*)?$").unwrap();
    let package_regex = Regex::new(r"^(?<package>package) +(?<name>[_a-zA-Z][_a-zA-Z0-9]*) *(#.*)?$").unwrap();
    let struct_regex = Regex::new(r"^(?<struct>struct) +(?<name>[_a-zA-Z][_a-zA-Z0-9]*): *(#.*)?$").unwrap();
    let field_regex = Regex::new(r"^ {4}(?<type>i8|i16|i32|u8|u16|u32|f32|bool|[_a-zA-Z][_a-zA-Z0-9]*)(?<array>\[[0-9]+\])? +(?<name>[_a-zA-Z][_a-zA-Z0-9]*) *(#.*)?$").unwrap();
    let blank_line_regex = Regex::new(r"^\s*(#.*)?$").unwrap();

    let mut package = Package {
        version: None,
        name: None,
        structs: HashMap::new()
    };

    let mut in_struct = false;
    let mut struct_name = "";
    for (i, line) in cnt.lines().enumerate() {
        // If line is a version declaration
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
        // If line is declaring the name of the package
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
        // If line is declaring the name of a struct
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
                fields_order: Vec::new()
            });
        }
        // If the name is declaring the field of a struct
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
                            type_name: var_type.to_string(),
                            array: array_size
                        });
                        s.fields_order.push(name.to_string());
                    }
                },
                None => panic!("Error at line {}. Struct named `{}` not found.", i + 1, struct_name)
            }
        }
        else if let Some(_caps) = blank_line_regex.captures(line) {
            // Empty line
        }
        else {
            panic!("Error at line {}. Invalid syntax.", i + 1);
        }
    }


    // Generate output code
    let mut out = fs::File::create(
        Path::new(out_path.as_str()).join(format!("iris.{}", lang.ext()))
    ).unwrap();

    out.write_all(match lang {
        Langs::RUST(l) => l.gen_code(&package),
        Langs::PYTHON(_) => todo!("Python code generation is not yet supported."),
        Langs::CPP(_) => todo!("C++ code generation is not yet supported."),
        Langs::C(_) => todo!("C code generation is not yet supported.")
    }.as_bytes()).unwrap();
}