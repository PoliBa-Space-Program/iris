#[derive(PartialEq, Debug, Clone)]
pub enum TokenTypes {
    Colon,
    SemiColon,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,

    Identifier,
    UInt,
    SemanticVersion,

    Version,
    Struct,
    Enum,
    Package,

    EndOfStream
}