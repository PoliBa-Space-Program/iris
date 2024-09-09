use crate::core::ast::{FieldType, Package, Struct};

use super::field::gen_pack_arg;
use super::types::gen_pack_format;

pub fn gen_code(strc: &Struct, package: &Package) -> String {
    let mut out = String::new();
    let mut struct_format = String::new();
    struct_format.push_str(format!(">").as_str());
    for f in &strc.fields_order {
        struct_format.push_str(gen_pack_format(&strc.fields.get(f).unwrap().t, &package).as_str());
        for _ in 1..strc.fields.get(f).unwrap().array.unwrap_or_default() {
            struct_format.push_str(gen_pack_format(&strc.fields.get(f).unwrap().t, &package).as_str());
        }
    }

    out.push_str(format!("            class {}:\n", strc.name).as_str());
    out.push_str(format!("                NAME_HASH = {}\n", strc.fnv_1a()).as_str());
    out.push_str(format!("                BYTES_LENGTH = {} + 4\n", strc.size(package)).as_str());

    out.push_str("                def __init__(self");
    for f in &strc.fields_order {
        out.push_str(format!(", {}", f).as_str());
    }
    out.push_str("):\n");
    for f in &strc.fields_order {
        out.push_str(format!("                    self.{} = {}\n", f, f).as_str());
    }

    out.push_str("                def encode(self) -> bytes:\n");
    out.push_str(format!("                    return struct.pack('>I{}B', self.NAME_HASH, *self.to_be_bytes())\n", strc.size(package)).as_str());

    out.push_str("                def to_be_bytes(self) -> bytes:\n");
    out.push_str(format!("                    return struct.pack('{}'", struct_format).as_str());
    for f in &strc.fields_order {
        out.push_str(", ");
        out.push_str(gen_pack_arg(strc.fields.get(f).unwrap()).as_str());
    }
    out.push_str(")\n");

    out.push_str("                @staticmethod\n");
    out.push_str("                def decode(raw: bytes):\n");
    out.push_str(format!("                    data = struct.unpack('>I{}B', raw)\n", strc.size(package)).as_str());
    out.push_str(format!("                    return Iris.Packages.{}.{}.from_be_bytes(bytes(data[1:]))\n", package.name.as_ref().unwrap(), strc.name).as_str());

    out.push_str("                @staticmethod\n");
    out.push_str("                def from_be_bytes(raw: bytes):\n");
    out.push_str(format!("                    data = struct.unpack('{}', raw)\n", struct_format).as_str());
    out.push_str(format!("                    return Iris.Packages.{}.{}(\n", package.name.as_ref().unwrap(), strc.name).as_str());
    let mut data_index = 0;
    for f in &strc.fields_order {
        let f = strc.fields.get(f).unwrap();
        out.push_str(format!("                        {}=", f.name).as_str());
        match f.array {
            Some(n) => {
                match &f.t {
                    FieldType::COMPLEX(c) => {
                        match c {
                            crate::core::ast::ComplexTypes::Struct(s) => {
                                out.push_str(format!("[Iris.Packages.{}.{}.from_be_bytes(bytes(data[i:i+{}])) for i in range({}, {}, {})]", package.name.as_ref().unwrap(), s, f.t.size(package), data_index, f.size(package), f.t.size(package)).as_str());
                                data_index += f.size(package);
                            },
                            crate::core::ast::ComplexTypes::Enum(_) => {
                                out.push_str(format!("data[{}:{}]", data_index, data_index + n).as_str());
                                data_index += f.size(package);
                            },
                            crate::core::ast::ComplexTypes::Unknown(_) => todo!(),
                        }
                    },
                    FieldType::PRIMITIVE(_) => {
                        out.push_str(format!("data[{}:{}]", data_index, data_index + n).as_str());
                        data_index += n;
                    }
                }
            },
            None => {
                match &f.t {
                    FieldType::COMPLEX(c) => {
                        match c {
                            crate::core::ast::ComplexTypes::Struct(s) => {
                                out.push_str(format!("Iris.Packages.{}.{}.from_be_bytes(bytes(data[{}:{}]))", package.name.as_ref().unwrap(), s, data_index, data_index + strc.size(package)).as_str());
                                data_index += f.size(package);
                            },
                            crate::core::ast::ComplexTypes::Enum(_) => {
                                out.push_str(format!("data[{}]", data_index).as_str());
                                data_index += 1;
                            },
                            crate::core::ast::ComplexTypes::Unknown(_) => todo!(),
                        }
                    },
                    FieldType::PRIMITIVE(_) => {
                        out.push_str(format!("data[{}]", data_index).as_str());
                        data_index += 1;
                    }
                }
            }
        }
        out.push_str(",\n");
    }
    out.push_str("                    )\n");

    out
}