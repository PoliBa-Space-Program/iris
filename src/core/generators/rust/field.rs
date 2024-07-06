use crate::core::{field::Field, types::Types};

pub fn gen_declaration(field: &Field) -> String {
    let mut out = String::new();

    out.push_str(format!("pub {}: ", field.name).as_str());
    out.push_str(match field.array {
        Some(n) => {
            format!("[{}; {}],\n", field.f_type.to_string(), n)
        },
        None => {
            format!("{},\n", field.f_type.to_string())
        }
    }.as_str());

    out
}

pub fn gen_encode(field: &Field) -> String {
    let mut out = String::new();

    match field.array {
        Some(_n) => {
            out.push_str(format!("for i in self.{} {{\n", field.name).as_str());
            out.push_str("for x in i.to_be_bytes() {\n");
            out.push_str("data[index] = x;\n");
            out.push_str("index += 1;\n");
            out.push_str("}\n");
            out.push_str("}\n");
        },
        None => {
            out.push_str(format!("for x in self.{}.to_be_bytes() {{\n", field.name).as_str());
            out.push_str("data[index] = x;\n");
            out.push_str("index += 1;\n");
            out.push_str("}\n");
        }
    }

    out
}

pub fn gen_default(field: &Field) -> String {
    let mut out = String::new();

    out.push_str(format!("{}: ", field.name).as_str());
    out.push_str(match field.array {
        Some(n) => format!("[{}; {}]", match field.f_type {
            Types::U8 | Types::U16 | Types::U32 | Types::I8 | Types::I16 | Types::I32 => "0",
            Types::BOOL => "false",
            Types::F32 => "0.0"
        }, n),
        None => format!("{}", match field.f_type {
            Types::U8 | Types::U16 | Types::U32 | Types::I8 | Types::I16 | Types::I32 => "0",
            Types::BOOL => "false",
            Types::F32 => "0.0"
        })
    }.as_str());
    out.push_str(",\n");

    out
}

pub fn gen_from_bytes(field: &Field) -> String {
    let mut out = String::new();

    match field.array {
        Some(n) => {
            out.push_str(format!("for i in 0..{} {{\n", n).as_str());
            out.push_str(format!("out.{}[i] = {}::from_be_bytes(data[index..index+{}].try_into().unwrap());\n", field.name, field.f_type.to_string(), field.f_type.size()).as_str());
            out.push_str(format!("index += {};\n", field.f_type.size()).as_str());
            out.push_str("}\n");
        },
        None => {
            out.push_str(format!("out.{} = {}::from_be_bytes(data[index..index+{}].try_into().unwrap());\n", field.name, field.f_type.to_string(), field.f_type.size()).as_str());
            out.push_str(format!("index += {};\n", field.f_type.size()).as_str());
        }
    }

    out
}