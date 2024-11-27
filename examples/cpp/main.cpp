#include "iris.hpp"

int main() {
    iris::packages::Telemetry::Battery batteries1[2] = {
        iris::packages::Telemetry::Battery(43.65),
        iris::packages::Telemetry::Battery(88.32)
    };

    iris::packages::Telemetry::Battery batteries2[2] = {
        iris::packages::Telemetry::Battery(32.88),
        iris::packages::Telemetry::Battery(123.312)
    };
    
    iris::packages::Telemetry::Computer computers[2] = {
        iris::packages::Telemetry::Computer(123, batteries1, iris::packages::Telemetry::Status::SLEEP),
        iris::packages::Telemetry::Computer(1337, batteries2, iris::packages::Telemetry::Status::FLIGHT)
    };
    
    iris::packages::Telemetry::Data data = iris::packages::Telemetry::Data(computers);

    iris::byte *en = data.encode();

    iris::packages::Telemetry::Data de = iris::decode<iris::packages::Telemetry::Data>(en, data.BYTES_LENGTH());

    return 0;
}