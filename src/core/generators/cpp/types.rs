use crate::core::ast::FieldType;

pub fn gen_type_def(t: &FieldType) -> String {
    match t {
        FieldType::COMPLEX(complex_types) => String::from(complex_types.str()),
        FieldType::PRIMITIVE(primitive_types) => {
            match primitive_types {
                crate::core::ast::PrimitiveTypes::U8 => String::from("uint8_t"),
                crate::core::ast::PrimitiveTypes::U16 => String::from("uint16_t"),
                crate::core::ast::PrimitiveTypes::U32 => String::from("uint32_t"),
                crate::core::ast::PrimitiveTypes::I8 => String::from("int8_t"),
                crate::core::ast::PrimitiveTypes::I16 => String::from("int16_t"),
                crate::core::ast::PrimitiveTypes::I32 => String::from("int32_t"),
                crate::core::ast::PrimitiveTypes::F32 => String::from("float"),
                crate::core::ast::PrimitiveTypes::Bool => String::from("bool"),
            }
        },
    }
    
}