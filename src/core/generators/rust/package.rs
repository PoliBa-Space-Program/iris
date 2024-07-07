use crate::core::package::Package;

use super::r#struct;

pub fn gen_code(package: &Package) -> String {
    let mut out = String::new();

    out.push_str(format!("pub mod {} {{\n", package.name.as_ref().unwrap()).as_str());

    for s in package.structs.values() {
        out.push_str(r#struct::gen_code(s, package).as_str());
    }

    out.push_str("}\n");

    out
}