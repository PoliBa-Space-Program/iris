from iris import Iris

data = Iris.Packages.Telemetry.Data(
    [
        Iris.Packages.Telemetry.Computer(
            123,
            [
                Iris.Packages.Telemetry.Battery(43.65),
                Iris.Packages.Telemetry.Battery(88.32)
            ],
            Iris.Packages.Telemetry.Status.IDLE
        ),
        Iris.Packages.Telemetry.Computer(
            1337,
            [
                Iris.Packages.Telemetry.Battery(32.88),
                Iris.Packages.Telemetry.Battery(123.312)
            ],
            Iris.Packages.Telemetry.Status.FLIGHT
        )
    ]
)

en = data.encode()

de = Iris.decode(en)