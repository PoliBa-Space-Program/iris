pub mod packages {
    pub mod Telemetry {
        #[derive(Copy, Clone)]
        pub enum Status {
            FLIGHT = 2,
            SLEEP = 0,
            IDLE = 3,
            ACTIVE = 1,
        }
        impl Status {
            pub fn to_be_bytes(&self) -> [u8; 4] {
                (match self {
                    Status::FLIGHT => 2,
                    Status::SLEEP => 0,
                    Status::IDLE => 3,
                    Status::ACTIVE => 1,
                } as u32)
                    .to_be_bytes()
            }
            pub fn from_be_bytes(data: [u8; 4]) -> Status {
                match u32::from_be_bytes(data) {
                    2 => Status::FLIGHT,
                    0 => Status::SLEEP,
                    3 => Status::IDLE,
                    1 => Status::ACTIVE,
                    _ => panic!("No variant found."),
                }
            }
        }
        #[derive(Copy, Clone)]
        pub struct Data {
            pub computers: [Computer; 2],
        }
        impl Data {
            pub const NAME_HASH: u32 = 1062369733;
            pub const BYTES_LENGTH: usize = 32 + 4;
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
                for i in self.computers {
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
                    computers: [Computer {
                        id: 0,
                        batteries: [Battery { charge: 0.0 }; 2],
                        status: Status::FLIGHT,
                    }; 2],
                };
                let mut index = 0;
                for i in 0..2 {
                    out.computers[i] =
                        Computer::from_be_bytes(data[index..index + 16].try_into().unwrap());
                    index += 16;
                }
                out
            }
        }
        #[derive(Copy, Clone)]
        pub struct Computer {
            pub id: u32,
            pub batteries: [Battery; 2],
            pub status: Status,
        }
        impl Computer {
            pub const NAME_HASH: u32 = 3613607352;
            pub const BYTES_LENGTH: usize = 16 + 4;
            pub fn encode(&self) -> [u8; Computer::BYTES_LENGTH] {
                let mut data: [u8; Computer::BYTES_LENGTH] = [0; Computer::BYTES_LENGTH];
                let mut index = 0;
                for x in u32::to_be_bytes(Computer::NAME_HASH) {
                    data[index] = x;
                    index += 1;
                }
                for x in self.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                data
            }
            pub fn to_be_bytes(&self) -> [u8; Computer::BYTES_LENGTH - 4] {
                let mut data: [u8; Computer::BYTES_LENGTH - 4] = [0; Computer::BYTES_LENGTH - 4];
                let mut index = 0;
                for x in self.id.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                for i in self.batteries {
                    for x in i.to_be_bytes() {
                        data[index] = x;
                        index += 1;
                    }
                }
                for x in self.status.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                data
            }
            pub fn decode(data: &[u8]) -> Computer {
                Computer::from_be_bytes(data[4..data.len()].try_into().unwrap())
            }
            pub fn from_be_bytes(data: [u8; Computer::BYTES_LENGTH - 4]) -> Computer {
                let mut out = Computer {
                    id: 0,
                    batteries: [Battery { charge: 0.0 }; 2],
                    status: Status::FLIGHT,
                };
                let mut index = 0;
                out.id = u32::from_be_bytes(data[index..index + 4].try_into().unwrap());
                index += 4;
                for i in 0..2 {
                    out.batteries[i] =
                        Battery::from_be_bytes(data[index..index + 4].try_into().unwrap());
                    index += 4;
                }
                out.status = Status::from_be_bytes(data[index..index + 4].try_into().unwrap());
                index += 4;
                out
            }
        }
        #[derive(Copy, Clone)]
        pub struct Battery {
            pub charge: f32,
        }
        impl Battery {
            pub const NAME_HASH: u32 = 2215305518;
            pub const BYTES_LENGTH: usize = 4 + 4;
            pub fn encode(&self) -> [u8; Battery::BYTES_LENGTH] {
                let mut data: [u8; Battery::BYTES_LENGTH] = [0; Battery::BYTES_LENGTH];
                let mut index = 0;
                for x in u32::to_be_bytes(Battery::NAME_HASH) {
                    data[index] = x;
                    index += 1;
                }
                for x in self.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                data
            }
            pub fn to_be_bytes(&self) -> [u8; Battery::BYTES_LENGTH - 4] {
                let mut data: [u8; Battery::BYTES_LENGTH - 4] = [0; Battery::BYTES_LENGTH - 4];
                let mut index = 0;
                for x in self.charge.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                data
            }
            pub fn decode(data: &[u8]) -> Battery {
                Battery::from_be_bytes(data[4..data.len()].try_into().unwrap())
            }
            pub fn from_be_bytes(data: [u8; Battery::BYTES_LENGTH - 4]) -> Battery {
                let mut out = Battery { charge: 0.0 };
                let mut index = 0;
                out.charge = f32::from_be_bytes(data[index..index + 4].try_into().unwrap());
                index += 4;
                out
            }
        }
    }
}
pub enum Structs {
    Telemetry_Data(packages::Telemetry::Data),
    Telemetry_Computer(packages::Telemetry::Computer),
    Telemetry_Battery(packages::Telemetry::Battery),
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
        packages::Telemetry::Computer::NAME_HASH
            if data.len() == packages::Telemetry::Computer::BYTES_LENGTH =>
        {
            Ok(Structs::Telemetry_Computer(
                packages::Telemetry::Computer::decode(&data),
            ))
        }
        packages::Telemetry::Battery::NAME_HASH
            if data.len() == packages::Telemetry::Battery::BYTES_LENGTH =>
        {
            Ok(Structs::Telemetry_Battery(
                packages::Telemetry::Battery::decode(&data),
            ))
        }
        _ => Err("Unknown data."),
    }
}
