use std::collections::HashMap;

pub struct Variant {
    pub name: String,
    pub value: u32
}

pub struct Enum {
    pub name: String,
    pub variants: HashMap<String, Variant>,
    pub variants_order: Vec<String>,
    pub counter: u32
}

impl Enum {
    pub fn size(&self) -> u32 {
        4
    }
}