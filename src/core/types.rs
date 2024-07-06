/*
 * Supported types:
 * Ux -> unsigned int x-bits
 * Ix -> signed int x-bits
 * BOOL -> boolean
 * F32 -> float
 * F64 -> double
 * LEN -> other structs
 */

pub enum Types {
    U8,
    I8,
    BOOL,
    U16,
    I16,
    U32,
    I32,
    F32
}

impl Types {
    pub fn from_str(s: &str) -> Result<Types, &str> {
        match s {
            "u8" => Ok(Types::U8),
            "i8" => Ok(Types::I8),
            "bool" => Ok(Types::BOOL),
            "u16" => Ok(Types::U16),
            "i16" => Ok(Types::I16),
            "u32" => Ok(Types::U32),
            "i32" => Ok(Types::I32),
            "f32" => Ok(Types::F32),
            _ => Err("No compatible type found.")
        }
    }

    /// Returns type size in bytes.
    pub fn size(&self) -> u32 {
        match self {
            Types::U8 | Types::I8 | Types::BOOL => 1,
            Types::U16 | Types::I16 => 2,
            Types::U32 | Types::I32 | Types::F32 => 4
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Types::U8 => String::from("u8"),
            Types::I8 => String::from("i8"),
            Types::BOOL => String::from("bool"),
            Types::U16 => String::from("u16"),
            Types::I16 => String::from("i16"),
            Types::U32 => String::from("u32"),
            Types::I32 => String::from("i32"),
            Types::F32 => String::from("f32")
        }
    }
}