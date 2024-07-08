use crate::core::{package::Package, types::Types};

use super::field::gen_default;

pub fn gen_default_value(r#type: &Types, package: &Package) -> String {
    match r#type {
        Types::U8 | Types::U16 | Types::U32 | Types::I8 | Types::I16 | Types::I32 => String::from("0"),
        Types::BOOL => String::from("false"),
        Types::F32 => String::from("0.0"),
        Types::LEN(s) => {
            let mut out = String::new();

            out.push_str(format!("{} {{\n", s.name).as_str());
            for f in s.fields.values() {
                out.push_str(gen_default(f, package).as_str());
            }
            out.push_str("}\n");

            out
        },
        Types::ENUM(_e) => String::from("0")
    }
}