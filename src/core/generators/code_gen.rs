use crate::core::ast::Package;

use super::{cpp, python, rust};


pub enum Langs {
    RUST(rust::Rust),
    PYTHON(python::Python),
    CPP(cpp::CPP)
}

impl Langs {
    pub fn from_string(s: &str) -> Langs {
        match s {
            "rust" | "rs" => Langs::RUST(rust::Rust {  }),
            "python" | "py" => Langs::PYTHON(python::Python {  }),
            "c++" | "cpp" => Langs::CPP(cpp::CPP {  }),
            _ => panic!("Error, specified language is not supported.")
        }
    }

    pub fn ext(&self) -> &str {
        match self {
            Langs::RUST(_) => "rs",
            Langs::PYTHON(_) => "py",
            Langs::CPP(_) => "hpp"
        }
    }
}

pub trait CodeGen {
    fn gen_code(&self, package: &Package) -> String;
}