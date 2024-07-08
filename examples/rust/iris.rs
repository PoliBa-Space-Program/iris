#![no_std]
pub mod iris {
    pub mod Telemetry {
        #[derive(Copy, Clone)]
        pub enum Status {
            ACTIVE = 0,
            IDLE = 1,
            SLEEP = 2,
            LAND = 3,
            FLIGHT = 4,
        }
        impl Status {
            pub fn to_be_bytes(&self) -> [u8; 4] {
                (match self {
                    Status::ACTIVE => 0,
                    Status::IDLE => 1,
                    Status::SLEEP => 2,
                    Status::LAND => 3,
                    Status::FLIGHT => 4,
                } as u32)
                    .to_be_bytes()
            }
            pub fn from_be_bytes(data: [u8; 4]) -> Status {
                match u32::from_be_bytes(data) {
                    0 => Status::ACTIVE,
                    1 => Status::IDLE,
                    2 => Status::SLEEP,
                    3 => Status::LAND,
                    4 => Status::FLIGHT,
                    _ => panic!("No variant found."),
                }
            }
        }
        #[derive(Copy, Clone)]
        pub struct Data {
            pub computer_id: u32,
            pub status: Status,
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
                for x in self.computer_id.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                for x in self.status.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                data
            }
            pub fn decode(data: &[u8]) -> Data {
                Data::from_be_bytes(data[4..data.len()].try_into().unwrap())
            }
            pub fn from_be_bytes(data: [u8; Data::BYTES_LENGTH - 4]) -> Data {
                let mut out = Data {
                    computer_id: 0,
                    status: 0,
                };
                let mut index = 0;
                out.computer_id = u32::from_be_bytes(data[index..index + 4].try_into().unwrap());
                index += 4;
                out.status = Status::from_be_bytes(data[index..index + 4].try_into().unwrap());
                index += 4;
                out
            }
        }
    }
    pub enum Structs {
        Telemetry_Data(Telemetry::Data),
    }
    pub fn decode(data: &[u8]) -> Result<Structs, &str> {
        let struct_name_hash = u32::from_be_bytes(data[0..4].try_into().unwrap());
        match struct_name_hash {
            Telemetry::Data::NAME_HASH if data.len() == Telemetry::Data::BYTES_LENGTH => {
                Ok(Structs::Telemetry_Data(Telemetry::Data::decode(&data)))
            }
            _ => Err("Unknown data."),
        }
    }
}
