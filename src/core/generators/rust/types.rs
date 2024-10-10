use crate::core::ast::{ComplexTypes, FieldType, Package, PrimitiveTypes};

use super::field::gen_default;

pub fn gen_default_value(t: &FieldType, package: &Package) -> String {
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
                        out.push_str(gen_default(f, package).as_str());
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