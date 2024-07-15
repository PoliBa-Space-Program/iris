pub struct AST {
    pub packages: Vec<Package>
}

pub struct Package {
    pub name: String,
    pub version: String,
    pub structs: Vec<Struct>,
    pub enums: Vec<Enum>
}

pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>
}

pub struct StructField {
    pub name: String,
    pub t: FieldType,
    pub array: Option<u32>
}

pub enum FieldType {
    ENUM(Enum),
    STRUCT(Struct),
    PRIMITIVE(PrimitiveTypes)
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
    pub variants: Vec<EnumVariant>
}

pub struct EnumVariant {
    pub name: String,
    pub value: u32
}