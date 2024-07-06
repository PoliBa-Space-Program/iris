use crate::core::package::Package;

pub trait CodeGen {
    fn gen_code(&self, package: &Package) -> String;
}