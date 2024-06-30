pub mod Telemetry {
    pub mod FlightData {
        const FLIGHTDATA_NAME_HASH: u32 = 61427819;
        const FLIGHTDATA_LENGTH_BYTES: u32 = 32 + 4;
        pub struct FlightData {
            max_altitude: f32,
            max_velocity: f32,
            computer_id: u32,
            current_altitude: f32,
            mesured_temperatures: [f32; 4],
        }
    }
    pub mod Avionics {
        const AVIONICS_NAME_HASH: u32 = 3975646145;
        const AVIONICS_LENGTH_BYTES: u32 = 56 + 4;
        pub struct Avionics {
            sensors_id: [u32; 14],
        }
    }
}
