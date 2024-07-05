use std::collections::HashMap;

use super::r#struct::Struct;

pub struct Package {
    pub version: Option<String>,
    pub name: Option<String>,
    pub structs: HashMap<String, Struct>
}

impl Package {
    pub fn is_some(&self) -> bool {
        self.version.is_some() && self.name.is_some()
    }


    pub fn gen_code(&self) -> String {
        let mut out = String::new();

        out.push_str(format!("pub mod {} {{\n", self.name.as_ref().unwrap()).as_str());

        for s in self.structs.values() {
            out.push_str(s.gen_code().as_str());
        }

        out.push_str("}\n");

        out
    }
}