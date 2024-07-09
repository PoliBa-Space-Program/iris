#![no_std]
pub mod iris {
    pub mod packages {
        pub mod Telemetry {
            #[derive(Copy, Clone)]
            pub enum Status {
                SLEEP = 0,
                ACTIVE = 1,
                FLIGHT = 2,
                IDLE = 3,
            }
            impl Status {
                pub fn to_be_bytes(&self) -> [u8; 4] {
                    (match self {
                        Status::SLEEP => 0,
                        Status::ACTIVE => 1,
                        Status::FLIGHT => 2,
                        Status::IDLE => 3,
                    } as u32)
                        .to_be_bytes()
                }
                pub fn from_be_bytes(data: [u8; 4]) -> Status {
                    match u32::from_be_bytes(data) {
                        0 => Status::SLEEP,
                        1 => Status::ACTIVE,
                        2 => Status::FLIGHT,
                        3 => Status::IDLE,
                        _ => panic!("No variant found."),
                    }
                }
            }
            #[derive(Copy, Clone)]
            pub struct Data {
                pub status: [Status; 2],
            }
            impl Data {
                pub const NAME_HASH: u32 = 1062369733;
                pub const BYTES_LENGTH: usize = 8 + 4;
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
                    for i in self.status {
                        for x in i.to_be_bytes() {
                            data[index] = x;
                            index += 1;
                        }
                    }
                    data
                }
                pub fn decode(data: &[u8]) -> Data {
                    Data::from_be_bytes(data[4..data.len()].try_into().unwrap())
                }
                pub fn from_be_bytes(data: [u8; Data::BYTES_LENGTH - 4]) -> Data {
                    let mut out = Data {
                        status: [Status::SLEEP; 2],
                    };
                    let mut index = 0;
                    for i in 0..2 {
                        out.status[i] =
                            Status::from_be_bytes(data[index..index + 4].try_into().unwrap());
                        index += 4;
                    }
                    out
                }
            }
        }
    }
    pub enum Structs {
        Telemetry_Data(packages::Telemetry::Data),
    }
    pub fn decode(data: &[u8]) -> Result<Structs, &str> {
        let struct_name_hash = u32::from_be_bytes(data[0..4].try_into().unwrap());
        match struct_name_hash {
            packages::Telemetry::Data::NAME_HASH
                if data.len() == packages::Telemetry::Data::BYTES_LENGTH =>
            {
                Ok(Structs::Telemetry_Data(packages::Telemetry::Data::decode(
                    &data,
                )))
            }
            _ => Err("Unknown data."),
        }
    }
}
