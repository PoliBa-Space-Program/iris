use crate::core::{enumeration::Enum, package::Package};

pub fn gen_code(enmn: &Enum, _package: &Package) -> String {
    let mut out = String::new();

    out.push_str("#[derive(Copy, Clone)]\n");
    out.push_str(format!("pub enum {} {{\n", enmn.name).as_str());
    for name in &enmn.variants_order {
        if let Some(v) = enmn.variants.get(name) {
            out.push_str(format!("{} = {},\n", v.name, v.value).as_str());
        }
    }
    out.push_str("}\n");

    out.push_str(format!("impl {} {{\n", enmn.name).as_str());
    {
        out.push_str("pub fn to_be_bytes(&self) -> [u8; 4] {\n");
        out.push_str("(match self {\n");
        for name in &enmn.variants_order {
            if let Some(v) = enmn.variants.get(name) {
                out.push_str(format!("{}::{} => {},\n", enmn.name, v.name, v.value).as_str());
            }
        }
        out.push_str("} as u32).to_be_bytes()\n");
        out.push_str("}\n");
    }
    {
        out.push_str(format!("pub fn from_be_bytes(data: [u8; 4]) -> {} {{\n", enmn.name).as_str());
        out.push_str("match u32::from_be_bytes(data) {\n");
        for name in &enmn.variants_order {
            if let Some(v) = enmn.variants.get(name) {
                out.push_str(format!("{} => {}::{},\n", v.value, enmn.name, v.name).as_str());
            }
        }
        out.push_str("_ => panic!(\"No variant found.\")");
        out.push_str("}\n");
        out.push_str("}\n");
    }
    out.push_str("}\n");

    out
}