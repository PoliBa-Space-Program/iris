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

impl Package {
    pub fn add_struct_field(&mut self, struct_name: &String, field: StructField) {
        let s = self.structs.get_mut(struct_name).unwrap();
        s.fields_order.push(field.name.clone());
        s.fields.insert(field.name.clone(), field);
    }

    pub fn add_enum_variant(&mut self, enum_name: &String, variant: EnumVariant) {
        let e = self.enums.get_mut(enum_name).unwrap();
        e.variants.insert(variant.name.clone());
        e.variants_order.push(variant);
    }
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

    pub fn size(&self, pkg: &Package) -> u32 {
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
    pub array: Option<u32>
}

impl StructField {
    pub fn size(&self, pkg: &Package) -> u32 {
        self.t.size(pkg) * self.array.unwrap_or(1)
    }
}

#[derive(Debug, Clone)]
pub enum FieldType {
    COMPLEX(ComplexTypes),
    PRIMITIVE(PrimitiveTypes)
}

impl FieldType {
    pub fn str(&self) -> &str {
        match self {
            FieldType::PRIMITIVE(p) => p.str(),
            FieldType::COMPLEX(c) => c.str()
        }
    }

    pub fn size(&self, pkg: &Package) -> u32 {
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
    pub fn str(&self) -> &str {
        match self {
            ComplexTypes::Struct(s) => s.as_str(),
            ComplexTypes::Enum(e) => e.as_str(),
            ComplexTypes::Unknown(u) => u.as_str()
        }
    }

    pub fn size(&self, pkg: &Package) -> u32 {
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
    pub fn str(&self) -> &str {
        match self {
            PrimitiveTypes::U8 => "u8",
            PrimitiveTypes::U16 => "u16",
            PrimitiveTypes::U32 => "u32",
            PrimitiveTypes::I8 => "i8",
            PrimitiveTypes::I16 => "i16",
            PrimitiveTypes::I32 => "i32",
            PrimitiveTypes::F32 => "f32",
            PrimitiveTypes::Bool => "bool"
        }
    }

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

    pub fn size(&self) -> u32 {
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
    pub fn size(&self) -> u32 {
        4
    }
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub value: u32
}