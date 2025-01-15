use crate::core::ast::StructField;

use super::types::gen_type_def;

pub fn gen_prop_declaration(field: &StructField) -> String {
    let mut out = String::new();

    out.push_str(format!("{} {}", gen_type_def(&field.t), field.name).as_str());
    match field.array {
        Some(n) => out.push_str(format!("[{}]", n).as_str()),
        None => {}
    }
    out.push_str(";\n");

    out
}

pub fn gen_arg_declaration(field: &StructField) -> String {
    let mut out = String::new();

    out.push_str(format!("{} ", gen_type_def(&field.t)).as_str());
    match field.array {
        Some(_n) => out.push_str("*"),
        None => {}
    }
    out.push_str(field.name.as_str());

    out
}

pub fn gen_prop_init(field: &StructField) -> String {
    let mut out = String::new();

    match field.array {
        Some(n) => {
            out.push_str(format!("for (int i = 0; i < {}; i++) {{ this->{}[i] = {}[i]; }}\n", n, field.name, field.name).as_str());
        },
        None => {
            out.push_str(format!("this->{} = {};\n", field.name, field.name).as_str());
        }
    }

    out
}

pub fn gen_to_be_bytes_conv(field: &StructField) -> String {
    let mut out = String::new();

    match field.array {
        Some(n) => {
            out.push_str(format!("for (int j = 0; j < {}; j++) {{\n", n).as_str());
            match &field.t {
                crate::core::ast::FieldType::COMPLEX(complex_types) => {
                    out.push_str(format!("this->{}[j].to_be_bytes(buffer + i);\n", field.name).as_str());
                    out.push_str(format!("i += this->{}[j].BYTES_LENGTH", field.name).as_str());
                    match complex_types {
                        crate::core::ast::ComplexTypes::Struct(_) => out.push_str(" - 4"),
                        crate::core::ast::ComplexTypes::Enum(_) => {},
                        crate::core::ast::ComplexTypes::Unknown(_) => todo!(),
                    };
                    out.push_str(";\n");
                },
                crate::core::ast::FieldType::PRIMITIVE(_primitive_types) => {
                    out.push_str(format!("iris::to_be_bytes(this->{}[j], buffer + i);\n", field.name).as_str());
                    out.push_str(format!("i += sizeof(this->{}[j]);\n", field.name).as_str());
                },
            }
            out.push_str("}\n");
        },
        None => {
            match &field.t {
                crate::core::ast::FieldType::COMPLEX(complex_types) => {
                    out.push_str(format!("this->{}.to_be_bytes(buffer + i);\n", field.name).as_str());
                    out.push_str(format!("i += this->{}.BYTES_LENGTH", field.name).as_str());
                    match complex_types {
                        crate::core::ast::ComplexTypes::Struct(_) => out.push_str(" - 4"),
                        crate::core::ast::ComplexTypes::Enum(_) => {},
                        crate::core::ast::ComplexTypes::Unknown(_) => todo!(),
                    };
                    out.push_str(";\n");
                },
                crate::core::ast::FieldType::PRIMITIVE(_primitive_types) => {
                    out.push_str(format!("iris::to_be_bytes(this->{}, buffer + i);\n", field.name).as_str());
                    out.push_str(format!("i += sizeof(this->{});\n", field.name).as_str());
                },
            }
        }
    }

    out
}

pub fn gen_from_be_bytes_conv(field: &StructField) -> String {
    let mut out = String::new();

    match field.array {
        Some(n) => {
            out.push_str(format!("for (int j = 0; j < {}; j++) {{\n", n).as_str());
            match &field.t {
                crate::core::ast::FieldType::COMPLEX(complex_types) => {
                    out.push_str(format!("out.{}[j] = {}::from_be_bytes(raw + i);\n", field.name, complex_types.str()).as_str());
                    out.push_str(format!("i += {}::BYTES_LENGTH", complex_types.str()).as_str());
                    match complex_types {
                        crate::core::ast::ComplexTypes::Struct(_) => out.push_str(" - 4"),
                        crate::core::ast::ComplexTypes::Enum(_) => {},
                        crate::core::ast::ComplexTypes::Unknown(_) => todo!(),
                    };
                    out.push_str(";\n");
                },
                crate::core::ast::FieldType::PRIMITIVE(_primitive_types) => {
                    out.push_str(format!("out.{}[j] = iris::from_be_bytes<{}>(raw + i);\n", field.name, gen_type_def(&field.t)).as_str());
                    out.push_str(format!("i += sizeof({});\n", gen_type_def(&field.t)).as_str());
                },
            }
            out.push_str("}\n");
        },
        None => {
            match &field.t {
                crate::core::ast::FieldType::COMPLEX(complex_types) => {
                    out.push_str(format!("out.{} = {}::from_be_bytes(raw + i);\n", field.name, complex_types.str()).as_str());
                    out.push_str(format!("i += {}::BYTES_LENGTH", complex_types.str()).as_str());
                    match complex_types {
                        crate::core::ast::ComplexTypes::Struct(_) => out.push_str(" - 4"),
                        crate::core::ast::ComplexTypes::Enum(_) => {},
                        crate::core::ast::ComplexTypes::Unknown(_) => todo!(),
                    };
                    out.push_str(";\n");
                },
                crate::core::ast::FieldType::PRIMITIVE(_primitive_types) => {
                    out.push_str(format!("out.{} = iris::from_be_bytes<{}>(raw + i);\n", field.name, gen_type_def(&field.t)).as_str());
                    out.push_str(format!("i += sizeof({});\n", gen_type_def(&field.t)).as_str());
                },
            }
        }
    }

    out
}