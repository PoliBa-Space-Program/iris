/**
 * Code generation for Rust (no std).
 */

use package::gen_code;

use super::code_gen::CodeGen;


pub mod field;
pub mod r#struct;
pub mod package;


pub struct Rust {  }


impl CodeGen for Rust {
    fn gen_code(&self, package: &crate::core::package::Package) -> String {
        let mut out = String::new();

        out.push_str("#![no_std]\n");
        out.push_str("pub mod iris {\n");

        out.push_str(gen_code(&package).as_str());

        out.push_str("pub enum Structs {\n");
        for s in package.structs.values() {
            out.push_str(format!("{}_{}({}::{}),\n", package.name.clone().unwrap(), s.name, package.name.clone().unwrap(), s.name).as_str());
        }
        out.push_str("}\n");

        out.push_str("pub fn decode(data: &[u8]) -> Result<Structs, &str> {\n");
        
        out.push_str("let struct_name_hash = u32::from_be_bytes(data[0..4].try_into().unwrap());\n");
        
        out.push_str("match struct_name_hash {\n");
        for s in package.structs.values() {
            out.push_str(format!("{}::{}::NAME_HASH if data.len() == {}::{}::BYTES_LENGTH => Ok(Structs::{}_{}({}::{}::decode(&data))),\n", package.name.clone().unwrap(), s.name, package.name.clone().unwrap(), s.name, package.name.clone().unwrap(), s.name, package.name.clone().unwrap(), s.name).as_str());
        }
        out.push_str("_ => Err(\"Unknown data.\")\n");
        out.push_str("}\n");
        
        out.push_str("}\n");

        out.push_str("}\n");

        out
    }
}