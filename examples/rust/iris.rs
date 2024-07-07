#![no_std]
pub mod iris {
    pub mod Telemetry {
        pub struct FlightData {
            pub max_altitude: f32,
            pub max_velocity: f32,
            pub computers: [FlightComputer; 5],
            pub current_altitude: f32,
            pub mesured_temperatures: [f32; 4],
        }
        impl FlightData {
            pub const NAME_HASH: u32 = 61427819;
            pub const BYTES_LENGTH: usize = 48 + 4;
            pub fn encode(&self) -> [u8; FlightData::BYTES_LENGTH] {
                let mut data: [u8; FlightData::BYTES_LENGTH] = [0; FlightData::BYTES_LENGTH];
                let mut index = 0;
                for x in u32::to_be_bytes(FlightData::NAME_HASH) {
                    data[index] = x;
                    index += 1;
                }
                for x in self.max_altitude.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                for x in self.max_velocity.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                for i in self.computers {
                    for x in i.to_be_bytes() {
                        data[index] = x;
                        index += 1;
                    }
                }
                for x in self.current_altitude.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                for i in self.mesured_temperatures {
                    for x in i.to_be_bytes() {
                        data[index] = x;
                        index += 1;
                    }
                }
                data
            }
            pub fn to_be_bytes(&self) -> [u8; FlightData::BYTES_LENGTH] {
                self.encode()
            }
            pub fn decode(data: &[u8]) -> FlightData {
                let mut out = FlightData {
                    max_altitude: 0.0,
                    max_velocity: 0.0,
                    computers: [FlightComputer { id: 0 }; 5],
                    current_altitude: 0.0,
                    mesured_temperatures: [0.0; 4],
                };
                let mut index = 4;
                out.max_altitude = f32::from_be_bytes(data[index..index + 4].try_into().unwrap());
                index += 4;
                out.max_velocity = f32::from_be_bytes(data[index..index + 4].try_into().unwrap());
                index += 4;
                for i in 0..5 {
                    out.computers[i] =
                        FlightComputer::from_be_bytes(data[index..index + 4].try_into().unwrap());
                    index += 4;
                }
                out.current_altitude =
                    f32::from_be_bytes(data[index..index + 4].try_into().unwrap());
                index += 4;
                for i in 0..4 {
                    out.mesured_temperatures[i] =
                        f32::from_be_bytes(data[index..index + 4].try_into().unwrap());
                    index += 4;
                }
                out
            }
        }
        pub struct FlightComputer {
            pub id: u32,
        }
        impl FlightComputer {
            pub const NAME_HASH: u32 = 3212217306;
            pub const BYTES_LENGTH: usize = 4 + 4;
            pub fn encode(&self) -> [u8; FlightComputer::BYTES_LENGTH] {
                let mut data: [u8; FlightComputer::BYTES_LENGTH] =
                    [0; FlightComputer::BYTES_LENGTH];
                let mut index = 0;
                for x in u32::to_be_bytes(FlightComputer::NAME_HASH) {
                    data[index] = x;
                    index += 1;
                }
                for x in self.id.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                data
            }
            pub fn to_be_bytes(&self) -> [u8; FlightComputer::BYTES_LENGTH] {
                self.encode()
            }
            pub fn decode(data: &[u8]) -> FlightComputer {
                let mut out = FlightComputer { id: 0 };
                let mut index = 4;
                out.id = u32::from_be_bytes(data[index..index + 4].try_into().unwrap());
                index += 4;
                out
            }
        }
    }
    pub enum Structs {
        Telemetry_FlightData(Telemetry::FlightData),
        Telemetry_FlightComputer(Telemetry::FlightComputer),
    }
    pub fn decode(data: &[u8]) -> Result<Structs, &str> {
        let struct_name_hash = u32::from_be_bytes(data[0..4].try_into().unwrap());
        match struct_name_hash {
            Telemetry::FlightData::NAME_HASH
                if data.len() == Telemetry::FlightData::BYTES_LENGTH =>
            {
                Ok(Structs::Telemetry_FlightData(
                    Telemetry::FlightData::decode(&data),
                ))
            }
            Telemetry::FlightComputer::NAME_HASH
                if data.len() == Telemetry::FlightComputer::BYTES_LENGTH =>
            {
                Ok(Structs::Telemetry_FlightComputer(
                    Telemetry::FlightComputer::decode(&data),
                ))
            }
            _ => Err("Unknown data."),
        }
    }
}
