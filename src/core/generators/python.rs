/**
 * Code generation for Python.
 */

use package::gen_code;

use super::code_gen::CodeGen;

pub mod types;
pub mod field;
pub mod r#struct;
pub mod enumeration;
pub mod package;

pub struct Python { }

impl CodeGen for Python {
    fn gen_code(&self, package: &crate::core::ast::Package) -> String {
        let mut out = String::new();

        out.push_str("import enum\n");
        out.push_str("import struct\n");
        out.push_str("import itertools\n");

        out.push_str("class Iris:\n");
        out.push_str("    class Packages:\n");
        
        out.push_str(gen_code(&package).as_str());

        out.push_str("    @staticmethod\n");
        out.push_str("    def decode(raw: bytes):\n");
        out.push_str("        name_hash = struct.unpack('>I', raw[0:4])[0]\n");
        out.push_str("        match name_hash:\n");
        for s in package.structs.values() {
            out.push_str(format!("            case Iris.Packages.{}.{}.NAME_HASH if len(raw) == Iris.Packages.{}.{}.BYTES_LENGTH:\n", package.name.clone().unwrap(), s.name, package.name.clone().unwrap(), s.name).as_str());
            out.push_str(format!("                return Iris.Packages.{}.{}.decode(raw)\n", package.name.clone().unwrap(), s.name).as_str());
        }
        out.push_str("            case _:\n");
        out.push_str("                raise 'Unknown data.'\n");

        out
    }
}