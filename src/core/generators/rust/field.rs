use crate::core::{field::Field, package::Package, types::Types};

use super::types::gen_default_value;

pub fn gen_declaration(field: &Field) -> String {
    let mut out = String::new();

    out.push_str(format!("pub {}: ", field.name).as_str());
    out.push_str(match field.array {
        Some(n) => {
            format!("[{}; {}],\n", field.type_name, n)
        },
        None => {
            format!("{},\n", field.type_name)
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

pub fn gen_default(field: &Field, package: &Package) -> String {
    let mut out = String::new();

    out.push_str(format!("{}: ", field.name).as_str());
    out.push_str(match field.array {
        Some(n) => format!("[{}; {}]", gen_default_value(&Types::from_str(field.type_name.as_str(), package).unwrap(), package), n),
        None => format!("{}", gen_default_value(&Types::from_str(field.type_name.as_str(), package).unwrap(), package))
    }.as_str());
    out.push_str(",\n");

    out
}

pub fn gen_from_bytes(field: &Field, package: &Package) -> String {
    let mut out = String::new();

    match field.array {
        Some(n) => {
            out.push_str(format!("for i in 0..{} {{\n", n).as_str());
            out.push_str(format!("out.{}[i] = {}::from_be_bytes(data[index..index+{}].try_into().unwrap());\n", field.name, field.type_name, field.size(package) / n).as_str());
            out.push_str(format!("index += {};\n", field.size(package) / n).as_str());
            out.push_str("}\n");
        },
        None => {
            out.push_str(format!("out.{} = {}::from_be_bytes(data[index..index+{}].try_into().unwrap());\n", field.name, field.type_name, field.size(package)).as_str());
            out.push_str(format!("index += {};\n", field.size(package)).as_str());
        }
    }

    out
}