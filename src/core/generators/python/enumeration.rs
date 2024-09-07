use crate::core::ast::{Enum, Package};

pub fn gen_code(enmn: &Enum, _package: &Package) -> String {
    let mut out = String::new();

    out.push_str(format!("            class {}(enum.IntEnum):\n", enmn.name).as_str());
    for variant in &enmn.variants_order {
        out.push_str(format!("                {} = {}\n", variant.name, variant.value).as_str());
    }
    
    out.push_str("                def to_be_bytes(self) -> bytes:\n");
    out.push_str("                    return struct.pack('>I', self)\n");
    
    out.push_str("                @staticmethod\n");
    out.push_str("                def from_be_bytes(raw: bytes):\n");
    out.push_str("                    match struct.unpack('>I', raw)[0]:\n");
    for variant in &enmn.variants_order {
        out.push_str(format!("                        case {}:\n", variant.value).as_str());
        out.push_str(format!("                            return Iris.Packages.{}.{}.{}\n", _package.name.as_ref().unwrap(), enmn.name, variant.name).as_str());
    }
    out.push_str("                        case _:\n");
    out.push_str("                            raise 'No variant found.'\n");

    out
}