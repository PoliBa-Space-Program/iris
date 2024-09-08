use crate::core::ast::{Package, PrimitiveTypes, ComplexTypes, FieldType, StructField};

pub fn gen_pack_format(field: &StructField, package: &Package) -> String {
    let mut out = String::new();

    match &field.t {
        FieldType::COMPLEX(c) => {
            match c {
                ComplexTypes::Struct(_) => out.push_str(format!("{}B", field.size(package)).as_str()),
                ComplexTypes::Enum(_) => out.push_str(format!("{}I", field.array.unwrap_or(1)).as_str()),
                ComplexTypes::Unknown(_) => panic!("Can't generate pack format value for unknown type."),
            }
        },
        FieldType::PRIMITIVE(p) => {
            if let Some(n) = field.array {
                out.push_str(format!("{}", n).as_str());
            }
            match p {
                PrimitiveTypes::U8 => out.push_str("B"),
                PrimitiveTypes::I8 => out.push_str("b"),
                PrimitiveTypes::U16 => out.push_str("H"),
                PrimitiveTypes::I16 => out.push_str("h"),
                PrimitiveTypes::U32 => out.push_str("I"),
                PrimitiveTypes::I32 => out.push_str("i"),
                PrimitiveTypes::Bool => out.push_str("?"),
                PrimitiveTypes::F32 => out.push_str("f")
            }
        }
    }

    out
}

pub fn gen_pack_arg(field: &StructField) -> String {
    let mut out = String::new();

    match field.array {
        Some(_) => {
            out.push_str("*itertools.chain.from_iterable([i");
            match field.t {
                FieldType::COMPLEX(_) => out.push_str(".to_be_bytes()"),
                FieldType::PRIMITIVE(_) => {}
            }
            out.push_str(format!(" for i in self.{}])", field.name).as_str());
        },
        None => {
            match &field.t {
                FieldType::COMPLEX(c) => {
                    match c {
                        ComplexTypes::Struct(_) => out.push_str(format!("*self.{}.to_be_bytes()", field.name).as_str()),
                        ComplexTypes::Enum(_) => out.push_str(format!("self.{}", field.name).as_str()),
                        ComplexTypes::Unknown(_) => panic!("Can't generate pack argument for unknown type.")
                    }
                },
                FieldType::PRIMITIVE(_) => out.push_str(format!("self.{}", field.name).as_str())
            }
        }
    };

    out
}