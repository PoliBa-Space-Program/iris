use std::collections::{HashMap, HashSet};

pub struct AST {
    pub packages: Vec<Package>
}

pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub structs: HashMap<String, Struct>,
    pub enums: HashMap<String, Enum>
}

pub struct Struct {
    pub name: String,
    pub fields: HashMap<String, StructField>,
    pub fields_order: Vec<String>
}

pub struct StructField {
    pub name: String,
    pub t: FieldType,
    pub array: Option<u32>
}

impl StructField {
    pub fn new(name: String, field_type: String, array: Option<u32>) -> StructField {
        StructField {
            name,
            t: FieldType::new(field_type),
            array
        }
    }
}

pub enum FieldType {
    COMPLEX(String),
    PRIMITIVE(PrimitiveTypes)
}

impl FieldType {
    pub fn new(field_type: String) -> FieldType {
        match field_type.as_str() {
            "u8" => FieldType::PRIMITIVE(PrimitiveTypes::U8),
            "u16" => FieldType::PRIMITIVE(PrimitiveTypes::U16),
            "u32" => FieldType::PRIMITIVE(PrimitiveTypes::U32),
            "i8" => FieldType::PRIMITIVE(PrimitiveTypes::I8),
            "i16" => FieldType::PRIMITIVE(PrimitiveTypes::I16),
            "i32" => FieldType::PRIMITIVE(PrimitiveTypes::I32),
            "f32" => FieldType::PRIMITIVE(PrimitiveTypes::F32),
            "bool" => FieldType::PRIMITIVE(PrimitiveTypes::Bool),
            _ => FieldType::COMPLEX(field_type)
        }
    }
}

pub enum PrimitiveTypes {
    U8,
    U16,
    U32,
    I8,
    I16,
    I32,
    F32,
    Bool
}

pub struct Enum {
    pub name: String,
    pub variants: HashSet<String>,
    pub variants_order: Vec<EnumVariant>
}

pub struct EnumVariant {
    pub name: String,
    pub value: u32
}