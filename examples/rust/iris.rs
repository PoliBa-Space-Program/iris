#![no_std]
pub mod iris {
    pub mod packages {
        pub mod Structs {
            #[derive(Copy, Clone)]
            pub enum A {
                a = 0,
            }
            impl A {
                pub fn to_be_bytes(&self) -> [u8; 4] {
                    (match self {
                        A::a => 0,
                    } as u32)
                        .to_be_bytes()
                }
                pub fn from_be_bytes(data: [u8; 4]) -> A {
                    match u32::from_be_bytes(data) {
                        0 => A::a,
                        _ => panic!("No variant found."),
                    }
                }
            }
            #[derive(Copy, Clone)]
            pub struct Data {}
            impl Data {
                pub const NAME_HASH: u32 = 1062369733;
                pub const BYTES_LENGTH: usize = 0 + 4;
                pub fn encode(&self) -> [u8; Data::BYTES_LENGTH] {
                    let mut data: [u8; Data::BYTES_LENGTH] = [0; Data::BYTES_LENGTH];
                    let mut index = 0;
                    for x in u32::to_be_bytes(Data::NAME_HASH) {
                        data[index] = x;
                        index += 1;
                    }
                    for x in self.to_be_bytes() {
                        data[index] = x;
                        index += 1;
                    }
                    data
                }
                pub fn to_be_bytes(&self) -> [u8; Data::BYTES_LENGTH - 4] {
                    let mut data: [u8; Data::BYTES_LENGTH - 4] = [0; Data::BYTES_LENGTH - 4];
                    let mut index = 0;
                    data
                }
                pub fn decode(data: &[u8]) -> Data {
                    Data::from_be_bytes(data[4..data.len()].try_into().unwrap())
                }
                pub fn from_be_bytes(data: [u8; Data::BYTES_LENGTH - 4]) -> Data {
                    let mut out = Data {};
                    let mut index = 0;
                    out
                }
            }
        }
    }
    pub enum Structs {
        Structs_Data(packages::Structs::Data),
    }
    pub fn decode(data: &[u8]) -> Result<Structs, &str> {
        let struct_name_hash = u32::from_be_bytes(data[0..4].try_into().unwrap());
        match struct_name_hash {
            packages::Structs::Data::NAME_HASH
                if data.len() == packages::Structs::Data::BYTES_LENGTH =>
            {
                Ok(Structs::Structs_Data(packages::Structs::Data::decode(
                    &data,
                )))
            }
            _ => Err("Unknown data."),
        }
    }
}
