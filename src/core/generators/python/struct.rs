use crate::core::ast::{Package, Struct};

use super::field::gen_pack_format;

pub fn gen_code(strc: &Struct, package: &Package) -> String {
    let mut out = String::new();

    out.push_str(format!("            class {}:\n", strc.name).as_str());
    out.push_str(format!("                NAME_HASH = {}\n", strc.fnv_1a()).as_str());
    out.push_str(format!("                BYTES_LENGTH = {} + 4\n", strc.size(package)).as_str());

    out.push_str("                def __init__(self");
    for f in &strc.fields_order {
        out.push_str(format!(", {}", f).as_str());
    }
    out.push_str("):\n");

    out.push_str("                def encode(self) -> bytes:\n");
    out.push_str(format!("                    return struct.pack('>I{}B', self.NAME_HASH, *self.to_be_bytes())\n", strc.size(package)).as_str());

    out.push_str("                def to_be_bytes(self) -> bytes:\n");
    out.push_str("                    return struct.pack('>");
    for f in &strc.fields_order {
        out.push_str(gen_pack_format(strc.fields.get(f).unwrap()).as_str());
    }
    out.push_str("'");
    for f in &strc.fields_order {
        out.push_str(", ");
        
    }
    out.push_str(")\n");

    out
}