use crate::core::{package::Package, r#struct::Struct};

use super::field::{gen_declaration, gen_default, gen_encode, gen_from_bytes};

pub fn gen_code(strc: &Struct, package: &Package) -> String {
    let mut out = String::new();

    out.push_str(format!("pub struct {} {{\n", strc.name).as_str());
    for f in &strc.fields_order {
        out.push_str(gen_declaration(strc.fields.get(f).unwrap()).as_str());
    }
    out.push_str("}\n");

    out.push_str(format!("impl {} {{\n", strc.name).as_str());

    out.push_str(format!("pub const NAME_HASH: u32 = {};\n", strc.fnv_1a()).as_str());
    out.push_str(format!("pub const BYTES_LENGTH: usize = {} + 4;\n", strc.size(package)).as_str());
    
    out.push_str(format!("pub fn encode(&self) -> [u8; {}::BYTES_LENGTH] {{\n", strc.name).as_str());

    out.push_str(format!("let mut data: [u8; {}::BYTES_LENGTH] = [0; {}::BYTES_LENGTH];", strc.name, strc.name).as_str());
    out.push_str(format!("let mut index = 0;").as_str());

    out.push_str(format!("for x in u32::to_be_bytes({}::NAME_HASH) {{\n", strc.name).as_str());
    out.push_str("data[index] = x;\n");
    out.push_str("index += 1;\n");
    out.push_str("}\n");

    for f in &strc.fields_order {
        out.push_str(gen_encode(strc.fields.get(f).unwrap()).as_str());
    }

    out.push_str("data\n");

    out.push_str("}\n");

    out.push_str(format!("pub fn to_be_bytes(&self) -> [u8; {}::BYTES_LENGTH] {{\n", strc.name).as_str());
    out.push_str("self.encode()\n");
    out.push_str("}\n");

    out.push_str(format!("pub fn decode(data: &[u8]) -> {} {{\n", strc.name).as_str());
    
    out.push_str(format!("let mut out = {} {{\n", strc.name).as_str());
    for f in &strc.fields_order {
        out.push_str(gen_default(strc.fields.get(f).unwrap(), package).as_str());
    }
    out.push_str("};\n");

    out.push_str("let mut index = 4;\n");

    for f in &strc.fields_order {
        out.push_str(gen_from_bytes(strc.fields.get(f).unwrap(), package).as_str());
    }

    out.push_str("out\n");

    out.push_str("}\n");

    out.push_str("}\n");

    out
}