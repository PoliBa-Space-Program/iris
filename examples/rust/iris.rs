#![no_std]
pub mod iris {
    pub mod Telemetry {
        #[derive(Copy, Clone)]
        pub struct Sensor {
            pub value: f32,
            pub active: bool,
        }
        impl Sensor {
            pub const NAME_HASH: u32 = 3385438363;
            pub const BYTES_LENGTH: usize = 5 + 4;
            pub fn encode(&self) -> [u8; Sensor::BYTES_LENGTH] {
                let mut data: [u8; Sensor::BYTES_LENGTH] = [0; Sensor::BYTES_LENGTH];
                let mut index = 0;
                for x in u32::to_be_bytes(Sensor::NAME_HASH) {
                    data[index] = x;
                    index += 1;
                }
                for x in self.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                data
            }
            pub fn to_be_bytes(&self) -> [u8; Sensor::BYTES_LENGTH - 4] {
                let mut data: [u8; Sensor::BYTES_LENGTH - 4] = [0; Sensor::BYTES_LENGTH - 4];
                let mut index = 0;
                for x in self.value.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                for x in (self.active as u8).to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                data
            }
            pub fn decode(data: &[u8]) -> Sensor {
                Sensor::from_be_bytes(data[4..data.len()].try_into().unwrap())
            }
            pub fn from_be_bytes(data: [u8; Sensor::BYTES_LENGTH - 4]) -> Sensor {
                let mut out = Sensor {
                    value: 0.0,
                    active: false,
                };
                let mut index = 0;
                out.value = f32::from_be_bytes(data[index..index + 4].try_into().unwrap());
                index += 4;
                out.active = u8::from_be_bytes(data[index..index + 1].try_into().unwrap()) != 0;
                index += 1;
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
        #[derive(Copy, Clone)]
        pub struct FlightComputer {
            pub sensor: Sensor,
            pub batteries: [Battery; 2],
        }
        impl FlightComputer {
            pub const NAME_HASH: u32 = 3212217306;
            pub const BYTES_LENGTH: usize = 13 + 4;
            pub fn encode(&self) -> [u8; FlightComputer::BYTES_LENGTH] {
                let mut data: [u8; FlightComputer::BYTES_LENGTH] =
                    [0; FlightComputer::BYTES_LENGTH];
                let mut index = 0;
                for x in u32::to_be_bytes(FlightComputer::NAME_HASH) {
                    data[index] = x;
                    index += 1;
                }
                for x in self.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                data
            }
            pub fn to_be_bytes(&self) -> [u8; FlightComputer::BYTES_LENGTH - 4] {
                let mut data: [u8; FlightComputer::BYTES_LENGTH - 4] =
                    [0; FlightComputer::BYTES_LENGTH - 4];
                let mut index = 0;
                for x in self.sensor.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                for i in self.batteries {
                    for x in i.to_be_bytes() {
                        data[index] = x;
                        index += 1;
                    }
                }
                data
            }
            pub fn decode(data: &[u8]) -> FlightComputer {
                FlightComputer::from_be_bytes(data[4..data.len()].try_into().unwrap())
            }
            pub fn from_be_bytes(data: [u8; FlightComputer::BYTES_LENGTH - 4]) -> FlightComputer {
                let mut out = FlightComputer {
                    sensor: Sensor {
                        value: 0.0,
                        active: false,
                    },
                    batteries: [Battery { charge: 0.0 }; 2],
                };
                let mut index = 0;
                out.sensor = Sensor::from_be_bytes(data[index..index + 5].try_into().unwrap());
                index += 5;
                for i in 0..2 {
                    out.batteries[i] =
                        Battery::from_be_bytes(data[index..index + 4].try_into().unwrap());
                    index += 4;
                }
                out
            }
        }
        #[derive(Copy, Clone)]
        pub struct FlightData {
            pub computers: [FlightComputer; 2],
        }
        impl FlightData {
            pub const NAME_HASH: u32 = 61427819;
            pub const BYTES_LENGTH: usize = 26 + 4;
            pub fn encode(&self) -> [u8; FlightData::BYTES_LENGTH] {
                let mut data: [u8; FlightData::BYTES_LENGTH] = [0; FlightData::BYTES_LENGTH];
                let mut index = 0;
                for x in u32::to_be_bytes(FlightData::NAME_HASH) {
                    data[index] = x;
                    index += 1;
                }
                for x in self.to_be_bytes() {
                    data[index] = x;
                    index += 1;
                }
                data
            }
            pub fn to_be_bytes(&self) -> [u8; FlightData::BYTES_LENGTH - 4] {
                let mut data: [u8; FlightData::BYTES_LENGTH - 4] =
                    [0; FlightData::BYTES_LENGTH - 4];
                let mut index = 0;
                for i in self.computers {
                    for x in i.to_be_bytes() {
                        data[index] = x;
                        index += 1;
                    }
                }
                data
            }
            pub fn decode(data: &[u8]) -> FlightData {
                FlightData::from_be_bytes(data[4..data.len()].try_into().unwrap())
            }
            pub fn from_be_bytes(data: [u8; FlightData::BYTES_LENGTH - 4]) -> FlightData {
                let mut out = FlightData {
                    computers: [FlightComputer {
                        sensor: Sensor {
                            value: 0.0,
                            active: false,
                        },
                        batteries: [Battery { charge: 0.0 }; 2],
                    }; 2],
                };
                let mut index = 0;
                for i in 0..2 {
                    out.computers[i] =
                        FlightComputer::from_be_bytes(data[index..index + 13].try_into().unwrap());
                    index += 13;
                }
                out
            }
        }
    }
    pub enum Structs {
        Telemetry_Sensor(Telemetry::Sensor),
        Telemetry_Battery(Telemetry::Battery),
        Telemetry_FlightComputer(Telemetry::FlightComputer),
        Telemetry_FlightData(Telemetry::FlightData),
    }
    pub fn decode(data: &[u8]) -> Result<Structs, &str> {
        let struct_name_hash = u32::from_be_bytes(data[0..4].try_into().unwrap());
        match struct_name_hash {
            Telemetry::Sensor::NAME_HASH if data.len() == Telemetry::Sensor::BYTES_LENGTH => {
                Ok(Structs::Telemetry_Sensor(Telemetry::Sensor::decode(&data)))
            }
            Telemetry::Battery::NAME_HASH if data.len() == Telemetry::Battery::BYTES_LENGTH => Ok(
                Structs::Telemetry_Battery(Telemetry::Battery::decode(&data)),
            ),
            Telemetry::FlightComputer::NAME_HASH
                if data.len() == Telemetry::FlightComputer::BYTES_LENGTH =>
            {
                Ok(Structs::Telemetry_FlightComputer(
                    Telemetry::FlightComputer::decode(&data),
                ))
            }
            Telemetry::FlightData::NAME_HASH
                if data.len() == Telemetry::FlightData::BYTES_LENGTH =>
            {
                Ok(Structs::Telemetry_FlightData(
                    Telemetry::FlightData::decode(&data),
                ))
            }
            _ => Err("Unknown data."),
        }
    }
}
