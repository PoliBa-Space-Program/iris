use std::collections::HashMap;

use super::{field::Field, package::Package};

/**
 * It's like a normal C-like struct but without methods.
 */
pub struct Struct {
    pub name: String,
    pub fields: HashMap<String, Field>,
    pub fields_order: Vec<String>
}

impl Struct {
    /// Simple, non-cryptographic hashing algorithm.
    /// This hashing function is used to create an unique id for every struct based on the given name.
    /// Reference: https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV-1a_hash
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

    pub fn size(&self, package: &Package) -> u32 {
        let mut counter = 0;

        for f in self.fields.values() {
            counter += f.size(package);
        }

        counter
    }
}