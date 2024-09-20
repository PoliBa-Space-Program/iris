use crate::core::ast::FieldType;

pub fn gen_type_def(t: &FieldType) -> String {
    match t {
        FieldType::COMPLEX(complex_types) => String::from(complex_types.str()),
        FieldType::PRIMITIVE(primitive_types) => {
            match primitive_types {
                crate::core::ast::PrimitiveTypes::U8 => String::from("unsigned char"),
                crate::core::ast::PrimitiveTypes::U16 => String::from("unsigned short"),
                crate::core::ast::PrimitiveTypes::U32 => String::from("unsigned int"),
                crate::core::ast::PrimitiveTypes::I8 => String::from("signed char"),
                crate::core::ast::PrimitiveTypes::I16 => String::from("short"),
                crate::core::ast::PrimitiveTypes::I32 => String::from("int"),
                crate::core::ast::PrimitiveTypes::F32 => String::from("float"),
                crate::core::ast::PrimitiveTypes::Bool => String::from("bool"),
            }
        },
    }
    
}