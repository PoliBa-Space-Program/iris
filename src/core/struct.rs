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
    pub fn fnv_1a(&self) -> u32 {
        let s = self.name.to_string();

        let mut hash = 2166136261;
        let fnv_prime = 16777619;

        for i in s.bytes() {
            hash ^= i as u32;
            hash = hash.wrapping_mul(fnv_prime);
        }

        hash
    }
}