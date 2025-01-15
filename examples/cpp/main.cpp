#include "iris.hpp"

int main() {
    iris::Telemetry::Battery batteries1[2] = {
        iris::Telemetry::Battery(43.65),
        iris::Telemetry::Battery(88.32)
    };

    iris::Telemetry::Battery batteries2[2] = {
        iris::Telemetry::Battery(32.88),
        iris::Telemetry::Battery(123.312)
    };
    
    iris::Telemetry::Computer computers[2] = {
        iris::Telemetry::Computer(123, batteries1, iris::Telemetry::Status::SLEEP),
        iris::Telemetry::Computer(1337, batteries2, iris::Telemetry::Status::FLIGHT)
    };
    
    iris::Telemetry::Data data = iris::Telemetry::Data(computers);

    iris::byte *en = data.encode();

    iris::Telemetry::Data de = iris::decode<iris::Telemetry::Data>(en, data.BYTES_LENGTH);

    return 0;
}