use crate::core::ast::{Enum, Package};

pub fn gen_code(enmn: &Enum, _package: &Package) -> String {
    let mut out = String::new();

    out.push_str("#[derive(Copy, Clone)]\n");
    out.push_str(format!("pub enum {} {{\n", enmn.name).as_str());
    for variant in enmn.variants.values() {
        out.push_str(format!("{} = {},\n", variant.name, variant.value).as_str());
    }
    out.push_str("}\n");

    out.push_str(format!("impl {} {{\n", enmn.name).as_str());
    
    out.push_str("pub fn to_be_bytes(&self) -> [u8; 4] {\n");
    out.push_str("(match self {\n");
    for variant in enmn.variants.values() {
        out.push_str(format!("{}::{} => {},\n", enmn.name, variant.name, variant.value).as_str());
    }
    out.push_str("} as u32).to_be_bytes()\n");
    out.push_str("}\n");
    
    
    out.push_str(format!("pub fn from_be_bytes(data: [u8; 4]) -> {} {{\n", enmn.name).as_str());
    out.push_str("match u32::from_be_bytes(data) {\n");
    for variant in enmn.variants.values() {
        out.push_str(format!("{} => {}::{},\n", variant.value, enmn.name, variant.name).as_str());
    }
    out.push_str("_ => panic!(\"No variant found.\")");
    out.push_str("}\n");
    out.push_str("}\n");
    
    out.push_str("}\n");

    out
}