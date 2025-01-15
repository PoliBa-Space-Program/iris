use std::{collections::HashMap, fs, io::Write, path::Path};

use super::ast::{Struct, AST};

use serde::{Deserialize, Serialize};
use serde_json::ser::to_string_pretty;


#[derive(Deserialize, Serialize)]
pub struct KronosCodeGen {
    pub package: String,
    pub data: HashMap<String, HashMap<String, String>>,
    pub size: HashMap<String, u32>
}

impl KronosCodeGen {
    fn expand_property(&mut self, s: &Struct, name: String, ast: &AST, class: String) {
        for f in s.fields.values() {
            for i in 0..f.array.unwrap_or(1) {
                let mut new_name = name.clone();
                new_name.push_str(&f.name);
                if f.array.is_some() {
                    new_name.push_str(&format!("[{}]", i));
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
    }
    
    // Create file kronos-code-gen.json
    pub fn generate(&mut self, path: String, ast: &AST) {
        self.package = ast.package.name.clone().unwrap();

        for s in ast.package.structs.values() {
            self.data.insert(s.name.clone(), HashMap::new());
            self.expand_property(s, String::new(), ast, s.name.clone());

            self.size.insert(s.name.clone(), s.size(&ast.package));
        }
    
        let mut out = fs::File::create(
            Path::new(&path).join("kronos-code-gen.json")
        ).unwrap();
    
        out.write_all(to_string_pretty(&self).unwrap().as_bytes()).unwrap();
    }
}