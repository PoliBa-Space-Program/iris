#![no_std]
pub mod iris {
    pub mod utils {
        pub fn from_be_bytes_u8(a: &[u8; 1]) -> u8 {
            a[0]
        }
        pub fn from_be_bytes_u16(a: &[u8; 2]) -> u16 {
            ((a[0] as u16) << 8) | ((a[1] as u16) << 0)
        }
        pub fn from_be_bytes_u32(a: &[u8; 4]) -> u32 {
            ((a[0] as u32) << 32)
                | ((a[1] as u32) << 16)
                | ((a[2] as u32) << 8)
                | ((a[3] as u32) << 0)
        }
        pub fn from_be_bytes_i8(a: &[u8; 1]) -> i8 {
            a[0] as i8
        }
        pub fn from_be_bytes_i16(a: &[u8; 2]) -> i16 {
            ((a[0] as i16) << 8) | ((a[1] as i16) << 0)
        }
        pub fn from_be_bytes_i32(a: &[u8; 4]) -> i32 {
            ((a[0] as i32) << 32)
                | ((a[1] as i32) << 16)
                | ((a[2] as i32) << 8)
                | ((a[3] as i32) << 0)
        }
        pub fn from_be_bytes_f32(a: &[u8; 4]) -> f32 {
            (((a[0] as u32) << 32)
                | ((a[1] as u32) << 16)
                | ((a[2] as u32) << 8)
                | ((a[3] as u32) << 0)) as f32
        }
        pub fn from_be_bytes_bool(a: &[u8; 1]) -> bool {
            a[0] != 0
        }
    }
    pub mod Telemetry {
        pub struct FlightData {
            max_altitude: f32,
            max_velocity: f32,
            computer_id: u32,
            current_altitude: f32,
            mesured_temperatures: [f32; 4],
        }
        impl FlightData {
            pub const NAME_HASH: u32 = 61427819;
            pub const LENGTH_BYTES: u32 = 32 + 4;
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
            pub fn decode(&self, data: &[u8]) -> FlightData {
                let mut out = FlightData {
                    max_altitude: 0.0,
                    max_velocity: 0.0,
                    computer_id: 0,
                    current_altitude: 0.0,
                    mesured_temperatures: [0.0; 4],
                };
                let mut index = 4;
                out.max_altitude = utils::from_be_bytes_f32(&data[index..index + 4]);
                index += 4;
                out.max_velocity = utils::from_be_bytes_f32(&data[index..index + 4]);
                index += 4;
                out.computer_id = utils::from_be_bytes_u32(&data[index..index + 4]);
                index += 4;
                out.current_altitude = utils::from_be_bytes_f32(&data[index..index + 4]);
                index += 4;
                for i in 0..4 {
                    out.mesured_temperatures[i] = utils::from_be_bytes_f32(&data[index..index + 4]);
                    index += 4;
                }
                out
            }
        }
    }
    enum DecodeRes {
        Telemetry_FlightData(Telemetry::FlightData),
    }
    pub fn decode(data: &[u8]) -> Result<DecodeRes, &str> {
        let struct_name_hash = ((data[0] as u32) << 32)
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
