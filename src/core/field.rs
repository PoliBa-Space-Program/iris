use super::types::Types;

/*
 * Field of a struct.
 */
pub struct Field {
    pub name: String,
    pub f_type: Types,
    pub array: Option<u32>
}