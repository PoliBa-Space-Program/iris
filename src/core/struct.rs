use std::collections::HashMap;

use super::field::Field;

pub struct Struct {
    pub name: String,
    pub fields: HashMap<String, Field>,
    pub fields_order: Vec<String>,
    pub size: u32
}

impl Struct {
    /*
    * Simple, non-cryptographic hashing algorithm.
    * Reference: https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV-1a_hash
    */
    fn fnv_1a(&self) -> u32 {
        let s = self.name.to_string();

        let mut hash = 2166136261;
        let fnv_prime = 16777619;

        for i in s.bytes() {
            hash ^= i as u32;
            hash = hash.wrapping_mul(fnv_prime);
        }

        hash
    }

    pub fn gen_code(&self) -> String {
        let mut out = String::new();

        out.push_str(format!("pub struct {} {{\n", self.name).as_str());
        for f in &self.fields_order {
            out.push_str(self.fields.get(f).unwrap().gen_declaration().as_str());
        }
        out.push_str("}\n");

        out.push_str(format!("impl {} {{\n", self.name).as_str());

        out.push_str(format!("pub const NAME_HASH: u32 = {};\n", self.fnv_1a()).as_str());
        out.push_str(format!("pub const BYTES_LENGTH: usize = {} + 4;\n", self.size).as_str());
        
        out.push_str(format!("pub fn encode(&self) -> [u8; {}::BYTES_LENGTH] {{\n", self.name).as_str());

        out.push_str(format!("let mut data: [u8; {}::BYTES_LENGTH] = [0; {}::BYTES_LENGTH];", self.name, self.name).as_str());
        out.push_str(format!("let mut index = 0;").as_str());

        out.push_str(format!("for x in u32::to_be_bytes({}::NAME_HASH) {{\n", self.name).as_str());
        out.push_str("data[index] = x;\n");
        out.push_str("index += 1;\n");
        out.push_str("}\n");

        for f in &self.fields_order {
            out.push_str(self.fields.get(f).unwrap().gen_encode().as_str());
        }

        out.push_str("data\n");

        out.push_str("}\n");

        out.push_str(format!("pub fn to_be_bytes(&self) -> [u8; {}::BYTES_LENGTH] {{\n", self.name).as_str());
        out.push_str("self.encode()\n");
        out.push_str("}\n");

        out.push_str(format!("pub fn decode(data: &[u8]) -> {} {{\n", self.name).as_str());
        
        out.push_str(format!("let mut out = {} {{\n", self.name).as_str());
        for f in &self.fields_order {
            out.push_str(self.fields.get(f).unwrap().gen_default().as_str());
        }
        out.push_str("};\n");

        out.push_str("let mut index = 4;\n");

        for f in &self.fields_order {
            out.push_str(self.fields.get(f).unwrap().gen_from_bytes().as_str());
        }

        out.push_str("out\n");

        out.push_str("}\n");

        out.push_str("}\n");

        out
    }
}