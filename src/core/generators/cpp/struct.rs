use crate::core::ast::{Package, Struct};

use super::field::{gen_arg_declaration, gen_from_be_bytes_conv, gen_prop_declaration, gen_prop_init, gen_to_be_bytes_conv};

pub fn gen_code(strc: &Struct, package: &Package) -> String {
    let mut out = String::new();

    out.push_str(format!("class {} {{\n", strc.name).as_str());
    out.push_str("public:\n");
    out.push_str(format!("static inline unsigned int NAME_HASH() {{ return {}; }}\n", strc.fnv_1a()).as_str());
    out.push_str(format!("static inline size_t BYTES_LENGTH() {{ return {} + 4; }}\n", strc.size(package)).as_str());
    out.push_str(format!("iris::byte DATA_BUFFER[{} + 4] = {{0}};\n", strc.size(package)).as_str());
    for f in strc.fields.values() {
        out.push_str(gen_prop_declaration(f).as_str());
    }
    out.push_str(format!("{}() {{}}\n", strc.name).as_str());
    out.push_str(format!("{}(", strc.name).as_str());
    let mut counter = 1;
    for f in strc.fields.values() {
        out.push_str(gen_arg_declaration(f).as_str());
        if counter < strc.fields.len() {
            out.push_str(",");
            counter += 1;
        }
    }
    out.push_str(") {\n");
    for f in strc.fields.values() {
        out.push_str(gen_prop_init(f).as_str());
    }
    out.push_str("}\n");

    out.push_str("iris::byte *encode() {\n");
    out.push_str("iris::to_be_bytes(this->NAME_HASH(), this->DATA_BUFFER);\n");
    out.push_str("this->to_be_bytes();\n");
    out.push_str("return this->DATA_BUFFER;\n");
    out.push_str("}\n");

    out.push_str("inline iris::byte *to_be_bytes() {\n");
    out.push_str("return this->to_be_bytes(this->DATA_BUFFER + 4);\n");
    out.push_str("}\n");

    out.push_str("iris::byte *to_be_bytes(iris::byte *buffer) {\n");
    out.push_str("int i = 0;\n");
    for s in &strc.fields_order {
        out.push_str(gen_to_be_bytes_conv(strc.fields.get(s).unwrap()).as_str());
    }
    out.push_str("return buffer;\n");
    out.push_str("}\n");

    out.push_str(format!("static {} decode(iris::byte *raw) {{\n", strc.name).as_str());
    out.push_str(format!("return {}::from_be_bytes(raw + 4);\n", strc.name).as_str());
    out.push_str("}\n");

    out.push_str(format!("static {} from_be_bytes(iris::byte *raw) {{\n", strc.name).as_str());
    out.push_str(format!("{} out = {}();\n", strc.name, strc.name).as_str());
    out.push_str("int i = 0;\n");
    for s in &strc.fields_order {
        out.push_str(gen_from_be_bytes_conv(strc.fields.get(s).unwrap()).as_str());
    }
    out.push_str("return out;\n");
    out.push_str("}\n");

    out.push_str("};\n");

    out
}