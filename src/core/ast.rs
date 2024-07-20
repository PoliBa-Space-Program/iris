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

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: HashMap<String, StructField>,
    pub fields_order: Vec<String>
}

impl Struct {
    /// Simple, non-cryptographic hashing algorithm.
    /// This hashing function is used to create an unique id for every struct based on the given name.
    /// Reference: https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV-1a_hash
    pub fn fnv_1a(&self) -> u32 {
        let s = self.name.to_string();

        let mut hash = 2166136261;
        let fnv_prime = 16777619;

        for i in s.bytes() {
            hash ^= i as u32;
            hash = hash.wrapping_mul(fnv_prime);
        }

        hash
    }

    pub fn size(&self, pkg: &Package) -> usize {
        let mut counter = 0;
        for f in self.fields.values() {
            counter += f.size(pkg);
        }

        counter
    }
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub t: FieldType,
    pub array: Option<usize>
}

impl StructField {
    pub fn new(name: String, field_type: String, array: Option<usize>) -> StructField {
        StructField {
            name,
            t: FieldType::new(field_type),
            array
        }
    }

    pub fn size(&self, pkg: &Package) -> usize {
        self.t.size(pkg) * self.array.unwrap_or(1)
    }
}

#[derive(Debug, Clone)]
pub enum FieldType {
    COMPLEX(ComplexTypes),
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
            _ => FieldType::COMPLEX(ComplexTypes::Unknown(field_type))
        }
    }

    pub fn size(&self, pkg: &Package) -> usize {
        match self {
            FieldType::PRIMITIVE(p) => p.size(),
            FieldType::COMPLEX(c) => c.size(pkg)
        }
    }
}

#[derive(Debug, Clone)]
pub enum ComplexTypes {
    Struct(String),
    Enum(String),
    Unknown(String)
}

impl ComplexTypes {
    pub fn size(&self, pkg: &Package) -> usize {
        match self {
            ComplexTypes::Struct(s) => pkg.structs.get(s).unwrap().size(pkg), 
            ComplexTypes::Enum(e) => pkg.enums.get(e).unwrap().size(),
            ComplexTypes::Unknown(_) => panic!("Can't return size of unknown data types.")
        }
    }
}

#[derive(Debug, Clone)]
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

impl PrimitiveTypes {
    pub fn new(s: String) -> PrimitiveTypes {
        match s.as_str() {
            "u8" => PrimitiveTypes::U8,
            "u16" => PrimitiveTypes::U16,
            "u32" => PrimitiveTypes::U32,
            "i8" => PrimitiveTypes::I8,
            "i16" => PrimitiveTypes::I16,
            "i32" => PrimitiveTypes::I32,
            "f32" => PrimitiveTypes::F32,
            "bool" => PrimitiveTypes::Bool,
            _ => panic!("Unknown primitive type.")
        }
    }

    pub fn size(&self) -> usize {
        match self {
            PrimitiveTypes::U8 | PrimitiveTypes::I8 | PrimitiveTypes::Bool => 1,
            PrimitiveTypes::U16 | PrimitiveTypes::I16 => 2,
            PrimitiveTypes::U32 | PrimitiveTypes::I32 | PrimitiveTypes::F32 => 4
        }
    }
}


#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub variants: HashSet<String>,
    pub variants_order: Vec<EnumVariant>
}

impl Enum {
    pub fn size(&self) -> usize {
        4
    }
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub value: u32
}