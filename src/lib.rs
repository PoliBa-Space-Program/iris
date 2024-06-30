pub mod iris {
    use std::collections::HashMap;

    const VERSION: &str = "0.1.0";

    struct Package {
        version: Option<String>,
        name: Option<String>,
        structs: HashMap<String, Struct>
    }

    impl Package {
        fn is_some(&self) -> bool {
            self.version.is_some() && self.name.is_some()
        }
    
    
        fn gen_code(&self) -> String {
            let mut out = String::new();
            let mut opened_parenthesis: Vec<&str> = Vec::new();
    
            out.push_str(format!("mod pub {} {{", self.name.as_ref().unwrap()).as_str());
            opened_parenthesis.push("{");
    
            for (n, s) in self.structs.iter() {
                
            }
    
            out
        }
    
        fn type_size(&self, wire_type: &str) -> Result<u32, &str> {
            match wire_type {
                "u8" | "i8" | "bool" => Ok(1),
                "u16" | "i16" => Ok(2),
                "u32" | "i32" | "f32" => Ok(4),
                _ => {
                    match self.structs.get(wire_type) {
                        Some(s) => Ok(s.size),
                        None => Err("Unknown type.")
                    }
                }
            }
        }
    }

    struct Struct {
        name: String,
        fields: Vec<Field>,
        size: u32
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
        
        fn gen_code(&self) -> String {
            let mut out = String::new();
            let mut opened_parenthesis: Vec<&str> = Vec::new();
    
            out.push_str(format!("mod pub {} {{", self.name).as_str());
            opened_parenthesis.push("{");
    
            out.push_str(format!("const {}_NAME_HASH: u32 = {};", self.name.to_uppercase(), self.fnv_1a()).as_str());
            out.push_str(format!("const {}_LENGTH_BYTES: u32 = {};", self.name.to_uppercase(), 12).as_str());
    
            out
        }
    }

    struct Field {
        name: String,
        wire_type: String,
        array: Option<u32>,
        size: u32
    }
}