use std::{collections::HashMap, fs, io::Write, path::Path};

use super::ast::{ComplexTypes, FieldType, Package, PrimitiveTypes, Struct, StructField, AST};

use serde::{Deserialize, Serialize};
use serde_json::ser::to_string_pretty;


#[derive(Deserialize, Serialize)]
pub struct KronosCodeGen {
    pub version: String,
    pub package: String,
    pub data: HashMap<String, HashMap<String, String>>,
    pub rust_default: HashMap<String, String>
}

impl KronosCodeGen {
    fn gen_default_value(&self, t: &FieldType, package: &Package) -> String {
        match t {
            FieldType::PRIMITIVE(p) => {
                match p {
                    PrimitiveTypes::U8 | PrimitiveTypes::U16 | PrimitiveTypes::U32 | PrimitiveTypes::I8 | PrimitiveTypes::I16 | PrimitiveTypes::I32 => String::from("0"),
                    PrimitiveTypes::Bool => String::from("false"),
                    PrimitiveTypes::F32 => String::from("0.0")
                }
            },
            FieldType::COMPLEX(c) => {
                match c {
                    ComplexTypes::Struct(s) => {
                        let mut out = String::new();
    
                        out.push_str(format!("{} {{\n", s).as_str());
                        for f in package.structs.get(s).unwrap().fields.values() {
                            out.push_str(self.gen_default(f, package).as_str());
                        }
                        out.push_str("}\n");
    
                        out
                    },
                    ComplexTypes::Enum(e) => format!("{}::{}", e, package.enums.get(e).unwrap().variants.keys().next().unwrap()),
                    ComplexTypes::Unknown(_u) => panic!("Can't generate default value for unknow type.")
                }
            }
        }
    }

    fn gen_default(&self, field: &StructField, package: &Package) -> String {
        let mut out = String::new();
    
        out.push_str(format!("{}: ", field.name).as_str());
        out.push_str(match field.array {
            Some(n) => format!("[{}; {}]", self.gen_default_value(&field.t, package), n),
            None => format!("{}", self.gen_default_value(&field.t, package))
        }.as_str());
        out.push_str(",\n");
    
        out
    }

    fn expand_property(&mut self, s: &Struct, name: String, ast: &AST, class: String) {
        for f in s.fields.values() {
            let mut new_name = name.clone();
            new_name.push_str(&f.name);
            if let Some(size) = f.array {
                new_name.push_str(&format!("[{}]", size));
            }        
    
            match &f.t {
                super::ast::FieldType::COMPLEX(complex_types) => {
                    match complex_types {
                        super::ast::ComplexTypes::Struct(strct) => {
                            new_name.push('.');
                            self.expand_property(ast.package.structs.get(strct).unwrap(), new_name, ast, class.clone());
                        },
                        super::ast::ComplexTypes::Enum(_) => {
                            self.data.get_mut(&class).unwrap().insert(new_name, complex_types.str().to_string());
                        },
                        super::ast::ComplexTypes::Unknown(_) => {},
                    }
                },
                super::ast::FieldType::PRIMITIVE(primitive_types) => {
                    self.data.get_mut(&class).unwrap().insert(new_name, primitive_types.str().to_string());
                },
            };
        }
    }
    
    // Create file kronos-code-gen.json
    pub fn generate(&mut self, path: String, ast: &AST) {
        self.version = ast.package.version.clone().unwrap();
        self.package = ast.package.name.clone().unwrap();

        for s in ast.package.structs.values() {
            self.data.insert(s.name.clone(), HashMap::new());
            for f in s.fields.values() {
                let mut data_plot = String::from("data.");
                data_plot.push_str(&f.name);
                if let Some(size) = f.array {
                    data_plot.push_str(&format!("[{}]", size));
                }
    
                match &f.t {
                    super::ast::FieldType::COMPLEX(complex_types) => {
                        match complex_types {
                            super::ast::ComplexTypes::Struct(strct) => {
                                data_plot.push('.');
                                self.expand_property(ast.package.structs.get(strct).unwrap(), data_plot, ast, s.name.clone());
                            },
                            super::ast::ComplexTypes::Enum(_) => {
                                self.data.get_mut(&s.name).unwrap().insert(data_plot, complex_types.str().to_string());
                            },
                            super::ast::ComplexTypes::Unknown(_) => {},
                        }
                    },
                    super::ast::FieldType::PRIMITIVE(primitive_types) => {
                        self.data.get_mut(&s.name).unwrap().insert(data_plot, primitive_types.str().to_string());
                    },
                } 
            }

            let mut default_init = format!("let mut data = {} {{\n", s.name);
            for f in &s.fields_order {
                default_init.push_str(&self.gen_default(s.fields.get(f).unwrap(), &ast.package));
            }
            default_init.push_str("};\n");

            self.rust_default.insert(s.name.clone(), default_init);
        }
    
        let mut out = fs::File::create(
            Path::new(&path).join("kronos-code-gen.json")
        ).unwrap();
    
        out.write_all(to_string_pretty(&self).unwrap().as_bytes()).unwrap();
    }
}