use super::types::Types;

pub struct Field {
    pub name: String,
    pub f_type: Types,
    pub array: Option<u32>
}

impl Field {
    pub fn gen_declaration(&self) -> String {
        let mut out = String::new();

        out.push_str(format!("pub {}: ", self.name).as_str());
        out.push_str(match self.array {
            Some(n) => {
                format!("[{}; {}],\n", self.f_type.to_string(), n)
            },
            None => {
                format!("{},\n", self.f_type.to_string())
            }
        }.as_str());

        out
    }

    pub fn gen_encode(&self) -> String {
        let mut out = String::new();

        match self.array {
            Some(_n) => {
                out.push_str(format!("for i in self.{} {{\n", self.name).as_str());
                out.push_str("for x in i.to_be_bytes() {\n");
                out.push_str("data[index] = x;\n");
                out.push_str("index += 1;\n");
                out.push_str("}\n");
                out.push_str("}\n");
            },
            None => {
                out.push_str(format!("for x in self.{}.to_be_bytes() {{\n", self.name).as_str());
                out.push_str("data[index] = x;\n");
                out.push_str("index += 1;\n");
                out.push_str("}\n");
            }
        }

        out
    }

    pub fn gen_default(&self) -> String {
        let mut out = String::new();

        out.push_str(format!("{}: ", self.name).as_str());
        out.push_str(match self.array {
            Some(n) => format!("[{}; {}]", match self.f_type {
                Types::U8 | Types::U16 | Types::U32 | Types::I8 | Types::I16 | Types::I32 => "0",
                Types::BOOL => "false",
                Types::F32 => "0.0"
            }, n),
            None => format!("{}", match self.f_type {
                Types::U8 | Types::U16 | Types::U32 | Types::I8 | Types::I16 | Types::I32 => "0",
                Types::BOOL => "false",
                Types::F32 => "0.0"
            })
        }.as_str());
        out.push_str(",\n");

        out
    }

    pub fn gen_from_bytes(&self) -> String {
        let mut out = String::new();

        match self.array {
            Some(n) => {
                out.push_str(format!("for i in 0..{} {{\n", n).as_str());
                out.push_str(format!("out.{}[i] = {}::from_be_bytes(data[index..index+{}].try_into().unwrap());\n", self.name, self.f_type.to_string(), self.f_type.size()).as_str());
                out.push_str(format!("index += {};\n", self.f_type.size()).as_str());
                out.push_str("}\n");
            },
            None => {
                out.push_str(format!("out.{} = {}::from_be_bytes(data[index..index+{}].try_into().unwrap());\n", self.name, self.f_type.to_string(), self.f_type.size()).as_str());
                out.push_str(format!("index += {};\n", self.f_type.size()).as_str());
            }
        }

        out
    }
}