use iris::{decode, Telemetry::{Battery, Computer, Data, Status}, Structs};

mod iris;

fn main() {
    let data = Data {
        computers: [
            Computer {
                id: 123,
                batteries: [
                    Battery {
                        charge: 43.65
                    },
                    Battery {
                        charge: 88.32
                    }
                ],
                status: Status::IDLE
            },
            Computer {
                id: 1337,
                batteries: [
                    Battery {
                        charge: 32.88
                    },
                    Battery {
                        charge: 123.312
                    }
                ],
                status: Status::FLIGHT
            }
        ]
    };

    let en = data.encode();

    let de = decode(&en);
}