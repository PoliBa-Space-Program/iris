use std::collections::HashSet;

use crate::core::ast::Package;

use super::{enumeration, r#struct};

pub fn gen_code(package: &Package) -> String {
    let mut out = String::new();

    out.push_str(format!("namespace {} {{\n", package.name.clone().unwrap()).as_str());
    
    for e in package.enums.values() {
        out.push_str(enumeration::gen_code(e, package).as_str());
    }
    let mut skipped_structs: HashSet<String> = HashSet::new();
    let mut inserted: HashSet<String> = HashSet::new();
    for s in package.structs.values() {
        let mut no_struct_fields = true;
        for f in s.fields.values() {
            match &f.t {
                crate::core::ast::FieldType::COMPLEX(complex_types) => match complex_types {
                    crate::core::ast::ComplexTypes::Struct(_) => {
                        no_struct_fields = false;
                        break;
                    },
                    _ => {}
                },
                crate::core::ast::FieldType::PRIMITIVE(_primitive_types) => {},
            }
        }
        if no_struct_fields {
            out.push_str(r#struct::gen_code(s, package).as_str());
            inserted.insert(s.name.clone());
        }
        else {
            skipped_structs.insert(s.name.clone());
        }
    }
    while inserted.len() < package.structs.len() {
        for s in &skipped_structs {
            let mut no_struct_fields = true;
            for f in package.structs.get(s).unwrap().fields.values() {
                match &f.t {
                    crate::core::ast::FieldType::COMPLEX(complex_types) => match complex_types {
                        crate::core::ast::ComplexTypes::Struct(struct_name) => {
                            if !inserted.contains(struct_name) {
                                no_struct_fields = false;
                            }
                            break;
                        },
                        _ => {}
                    },
                    crate::core::ast::FieldType::PRIMITIVE(_primitive_types) => {}
                }
            }
            if no_struct_fields && !inserted.contains(s) {
                out.push_str(r#struct::gen_code(package.structs.get(s).unwrap(), package).as_str());
                inserted.insert(s.to_string());
            }
        }
    }

    out.push_str("}\n");

    out
}