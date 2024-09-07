use crate::core::ast::{ComplexTypes, FieldType, Package, PrimitiveTypes};

pub fn gen_pack_format(t: &FieldType, package: &Package) -> String {
    match t {
        FieldType::PRIMITIVE(p) => {
            match p {
                PrimitiveTypes::U8 => String::from("B"),
                PrimitiveTypes::I8 => String::from("b"),
                PrimitiveTypes::U16 => String::from("H"),
                PrimitiveTypes::I16 => String::from("h"),
                PrimitiveTypes::U32 => String::from("I"),
                PrimitiveTypes::I32 => String::from("i"),
                PrimitiveTypes::Bool => String::from("?"),
                PrimitiveTypes::F32 => String::from("f")
            }
        },
        FieldType::COMPLEX(c) => {
            match c {
                ComplexTypes::Struct(s) => format!("{}B", t.size(package)),
                ComplexTypes::Enum(e) => String::from("I"),
                ComplexTypes::Unknown(_u) => panic!("Can't generate pack format value for unknown type.")
            }
        }
    }
}