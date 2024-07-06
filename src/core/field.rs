use super::types::Types;

pub struct Field {
    pub name: String,
    pub f_type: Types,
    pub array: Option<u32>
}