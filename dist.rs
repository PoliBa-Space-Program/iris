#![no_std]
pub mod iris {
    pub mod Telemetry {
        pub struct FlightData {
            pub max_altitude: f32,
            pub max_velocity: f32,
            pub computer_id: u32,
            pub current_altitude: f32,
            pub mesured_temperatures: [f32; 4],
        }
        impl FlightData {
            pub const NAME_HASH: u32 = 61427819;
            pub const BYTES_LENGTH: usize = 32 + 4;
            pub fn encode(&self) -> [u8; 36] {
                let mut data: [u8; 36] = [0; 36];
                let mut index = 0;
                for x in u32::to_be_bytes(61427819) {
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
                for x in self.computer_id.to_be_bytes() {
                    data[index] = x;
                    index += 1;
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
            pub fn to_be_bytes(&self) -> [u8; 36] {
                self.encode()
            }
            pub fn decode(data: &[u8]) -> FlightData {
                let mut out = FlightData {
                    max_altitude: 0.0,
                    max_velocity: 0.0,
                    computer_id: 0,
                    current_altitude: 0.0,
                    mesured_temperatures: [0.0; 4],
                };
                let mut index = 4;
                out.max_altitude = f32::from_be_bytes(data[index..index + 4].try_into().unwrap());
                index += 4;
                out.max_velocity = f32::from_be_bytes(data[index..index + 4].try_into().unwrap());
                index += 4;
                out.computer_id = u32::from_be_bytes(data[index..index + 4].try_into().unwrap());
                index += 4;
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
    }
    pub enum DecodeRes {
        Telemetry_FlightData(Telemetry::FlightData),
    }
    pub fn decode(data: &[u8]) -> Result<DecodeRes, &str> {
        let struct_name_hash = ((data[0] as u32) << 24)
            | ((data[1] as u32) << 16)
            | ((data[2] as u32) << 8)
            | ((data[3] as u32) << 0);
        match struct_name_hash {
            Telemetry::FlightData::NAME_HASH
                if data.len() == Telemetry::FlightData::BYTES_LENGTH =>
            {
                Ok(DecodeRes::Telemetry_FlightData(
                    Telemetry::FlightData::decode(&data),
                ))
            }
            _ => Err("Unknown data."),
        }
    }
}
