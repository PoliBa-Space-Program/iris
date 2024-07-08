use std::collections::HashMap;

use super::{enumeration::Enum, structure::Struct};

/**
 * It represents an entire file with a name and defined structs.
 */
pub struct Package {
    pub version: Option<String>,
    pub name: Option<String>,
    pub structs: HashMap<String, Struct>,
    pub enums: HashMap<String, Enum>
}

impl Package {
    pub fn is_some(&self) -> bool {
        self.version.is_some() && self.name.is_some()
    }
}