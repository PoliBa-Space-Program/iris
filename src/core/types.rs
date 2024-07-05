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
    fn str_to_wire_type(s: &str) -> Result<WireTypes, &str> {
        match s {
            "u8" => Ok(WireTypes::U8),
            "i8" => Ok(WireTypes::I8),
            "bool" => Ok(WireTypes::BOOL),
            "u16" => Ok(WireTypes::U16),
            "i16" => Ok(WireTypes::I16),
            "u32" => Ok(WireTypes::U32),
            "i32" => Ok(WireTypes::I32),
            "f32" => Ok(WireTypes::F32),
            _ => Err("No compatible wire type found.")
        }
    }

    fn size_wire_type(t: &WireTypes) -> u32 {
        match t {
            WireTypes::U8 | WireTypes::I8 | WireTypes::BOOL => 1,
            WireTypes::U16 | WireTypes::I16 => 2,
            WireTypes::U32 | WireTypes::I32 | WireTypes::F32 => 4
        }
    }

    fn to_string(t: &WireTypes) -> String {
        match t {
            WireTypes::U8 => String::from("u8"),
            WireTypes::I8 => String::from("i8"),
            WireTypes::BOOL => String::from("bool"),
            WireTypes::U16 => String::from("u16"),
            WireTypes::I16 => String::from("i16"),
            WireTypes::U32 => String::from("u32"),
            WireTypes::I32 => String::from("i32"),
            WireTypes::F32 => String::from("f32")
        }
    }
}