use crate::core::ast::Package;

use super::{enumeration, r#struct};

pub fn gen_code(package: &Package) -> String {
    let mut out = String::new();

    out.push_str(format!("        class {}:\n", package.name.as_ref().unwrap()).as_str());

    for e in package.enums.values() {
        out.push_str(enumeration::gen_code(e, package).as_str());
    }

    for s in package.structs.values() {
        out.push_str(r#struct.gen_code(s, package).as_str());
    }

    out
}