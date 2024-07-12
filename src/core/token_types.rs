#[derive(PartialEq, Debug)]
pub enum TokenTypes {
    Unknown,

    Colon,
    OpenSquareBracket,
    CloseSquareBracket,
    Indentation,

    Identifier,
    Int,
    Float,
    SemanticVersion,

    Version,
    Struct,
    Enum,
    Package,
    U8,
    U16,
    U32,
    I8,
    I16,
    I32,
    F32,
    Bool,

    EndOfStream
}