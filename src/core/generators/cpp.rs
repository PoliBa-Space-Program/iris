/**
 * Code generation for C++ (no STL).
 */
use package::gen_code;

use super::code_gen::CodeGen;

pub mod types;
pub mod field;
pub mod r#struct;
pub mod enumeration;
pub mod package;

pub struct CPP {  }


impl CodeGen for CPP {
    fn gen_code(&self, package: &crate::core::ast::Package) -> String {
        let mut out = String::new();

        out.push_str("#pragma once\n");
        out.push_str("namespace iris {\n");
        
        out.push_str("typedef uint8_t byte;\n");
        out.push_str("auto is_le = []() { int a = 1; return 1 == (int)*((byte*)(&a)) ? true : false; };\n");
        out.push_str("template <typename T>\n");
        out.push_str("void to_be_bytes(T data, byte *buffer) {\n");
        out.push_str("for (int i = 0; i < sizeof(data); i++) {\n");
        out.push_str("buffer[i] = *((byte*)(&data) + (is_le ? sizeof(data) - 1 - i : i));\n");
        out.push_str("}\n");
        out.push_str("}\n");
        out.push_str("template <typename T>\n");
        out.push_str("T from_be_bytes(byte *buffer) {\n");
        out.push_str("T data;\n");
        out.push_str("for (int i = 0; i < sizeof(T); i++) {\n");
        out.push_str("*((iris::byte*)(&data) + (is_le ? sizeof(T) - 1 - i : i)) = buffer[i];\n");
        out.push_str("}\n");
        out.push_str("return data;\n");
        out.push_str("}\n");

        out.push_str(gen_code(&package).as_str());

        out.push_str("template <typename T>\n");
        out.push_str("T decode(byte *raw, size_t len) {\n");
        out.push_str("uint32_t struct_name_hash = from_be_bytes<uint32_t>(raw);\n");
        let mut first = true;
        for s in package.structs.keys() {
            if first {
                out.push_str("if");
                first = false;
            }
            else {
                out.push_str("else if");
            }
            out.push_str(format!(" (struct_name_hash == {}::{}::NAME_HASH && len == {}::{}::BYTES_LENGTH) {{ return T::decode(raw); }}\n", package.name.clone().unwrap(), s, package.name.clone().unwrap(), s).as_str());
        }
        if package.structs.len() > 0 {
            out.push_str("else { throw 1; }\n");            
        }
        out.push_str("}\n");

        out.push_str("enum Structs {\n");
        for s in package.structs.keys() {
            out.push_str(format!("{}_{},\n", package.name.clone().unwrap(), s).as_str());
        }
        out.push_str("};\n");
        out.push_str("Structs check_type(byte *raw, size_t len) {\n");
        out.push_str("uint32_t struct_name_hash = from_be_bytes<uint32_t>(raw);\n");
        let mut first = true;
        for s in package.structs.keys() {
            if first {
                out.push_str("if");
                first = false;
            }
            else {
                out.push_str("else if");
            }
            out.push_str(format!(" (struct_name_hash == {}::{}::NAME_HASH && len == {}::{}::BYTES_LENGTH) {{ return Structs::{}_{}; }}\n", package.name.clone().unwrap(), s, package.name.clone().unwrap(), s, package.name.clone().unwrap(), s).as_str());
        }
        if package.structs.len() > 0 {
            out.push_str("else { throw 1; }\n");            
        }
        out.push_str("}\n");

        out.push_str("}\n");

        out
    }
}