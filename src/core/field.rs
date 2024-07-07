use super::{package::Package, types::Types};

/*
 * Field of a struct.
 */
pub struct Field {
    pub name: String,
    pub type_name: String,
    pub array: Option<u32>
}

impl Field {
    pub fn size(&self, package: &Package) -> u32 {
        Types::from_str(self.type_name.as_str(), package).unwrap().size(package) * self.array.unwrap_or(1)
    }
}