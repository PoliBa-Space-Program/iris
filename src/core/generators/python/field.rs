use crate::core::ast::{Package, PrimitiveTypes, ComplexTypes, FieldType, StructField};

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

