use crate::core::ast::{Enum, Package};

pub fn gen_code(enmn: &Enum, _package: &Package) -> String {
    let mut out = String::new();

    out.push_str(format!("class {} {{\n", enmn.name).as_str());
    out.push_str("public:\n");
    out.push_str("enum Value : uint32_t {\n");
    for v in enmn.variants.values() {
        out.push_str(format!("{} = {},\n", v.name, v.value).as_str());
    }
    out.push_str("};\n");
    out.push_str("static const size_t BYTES_LENGTH = 4;\n");
    out.push_str("iris::byte DATA_BUFFER[4] = {0};\n");
    out.push_str("Value value;\n");
    out.push_str(format!("{}() {{ }}\n", enmn.name).as_str());
    out.push_str(format!("{}(uint32_t value) {{\n", enmn.name).as_str());
    out.push_str("this->value = Value(value);\n");
    out.push_str("}\n");
    out.push_str("inline iris::byte *to_be_bytes() {\n");
    out.push_str("return this->to_be_bytes(this->DATA_BUFFER);\n");
    out.push_str("}\n");
    out.push_str("iris::byte *to_be_bytes(iris::byte *buffer) {\n");
    out.push_str("iris::to_be_bytes(this->value, buffer);\n");
    out.push_str("return buffer;\n");
    out.push_str("}\n");
    out.push_str("static Status from_be_bytes(iris::byte *raw) {\n");
    out.push_str("return Status(iris::from_be_bytes<uint32_t>(raw));\n");
    out.push_str("}\n");
    out.push_str("};\n");

    out
}