#[derive(PartialEq, Debug)]
pub enum TokenTypes {
    Unknown,

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