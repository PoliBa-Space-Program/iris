import enum
import struct
import itertools



class Iris:
    class Packages:
        class Telemetry:
            class Status(enum.IntEnum):
                SLEEP = 0
                ACTIVE = 1
                FLIGHT = 2
                IDLE = 3
                def to_be_bytes(self) -> bytes:
                    return struct.pack('>I', self)
                @staticmethod
                def from_be_bytes(raw: bytes):
                    match struct.unpack('>I', raw)[0]:
                        case 0:
                            return Iris.Packages.Telemetry.Status.SLEEP
                        case 1:
                            return Iris.Packages.Telemetry.Status.ACTIVE
                        case 2:
                            return Iris.Packages.Telemetry.Status.FLIGHT
                        case 3:
                            return Iris.Packages.Telemetry.Status.IDLE
                        case _:
                            raise 'No variant found.'
            class Computer:
                NAME_HASH = 3613607352
                BYTES_LENGTH = 16 + 4
                def __init__(self, id, batteries, status):
                    self.id = id
                    self.batteries = batteries
                    self.status = status
                def encode(self) -> bytes:
                    return struct.pack('>I16B', self.NAME_HASH, *self.to_be_bytes())
                def to_be_bytes(self) -> bytes:
                    return struct.pack('>I8BI', self.id, *itertools.chain.from_iterable([i.to_be_bytes() for i in self.batteries]), self.status)
                @staticmethod
                def decode(raw: bytes):
                    data = struct.unpack('>I16B', raw)
                    return Iris.Packages.Telemetry.Computer.from_be_bytes(bytes(data[1:]))
                @staticmethod
                def from_be_bytes(raw: bytes):
                    data = struct.unpack('>I8BI', raw)
                    return Iris.Packages.Telemetry.Computer(
                        id=data[0],
                        batteries=[Iris.Packages.Telemetry.Battery.from_be_bytes(bytes(data[i:i+4])) for i in range(1, 8, 4)],
                        status=data[9]
                    )
            class Data:
                NAME_HASH = 1062369733
                BYTES_LENGTH = 32 + 4
                def __init__(self, computers) -> None:
                    self.computers = computers
                def encode(self) -> bytes:
                    return struct.pack('>I32B', self.NAME_HASH, *self.to_be_bytes())
                def to_be_bytes(self) -> bytes:
                    return struct.pack('>32B', *itertools.chain.from_iterable([i.to_be_bytes() for i in self.computers]))
                @staticmethod
                def decode(raw: bytes):
                    data = struct.unpack('>I32B', raw)
                    return Iris.Packages.Telemetry.Data.from_be_bytes(bytes(data[1:]))
                @staticmethod
                def from_be_bytes(raw: bytes):
                    data = struct.unpack('>32B', raw)
                    return Iris.Packages.Telemetry.Data(
                        computers=[Iris.Packages.Telemetry.Computer.from_be_bytes(bytes(data[i:i+16])) for i in range(0, 32, 16)]
                    )
            class Battery:
                NAME_HASH = 2215305518
                BYTES_LENGTH = 4 + 4
                def __init__(self, charge) -> None:
                    self.charge = charge
                def encode(self) -> bytes:
                    return struct.pack('>I4B', self.NAME_HASH, *self.to_be_bytes())
                def to_be_bytes(self) -> bytes:
                    return struct.pack('>f', self.charge)
                @staticmethod
                def decode(raw: bytes):
                    data = struct.unpack('>I4B', raw)
                    return Iris.Packages.Telemetry.Battery.from_be_bytes(bytes(data[1:]))
                @staticmethod
                def from_be_bytes(raw: bytes):
                    data = struct.unpack('>f', raw)
                    return Iris.Packages.Telemetry.Battery(
                        charge=data[0]
                    )
    @staticmethod
    def decode(raw: bytes):
        name_hash = struct.unpack('>I', raw[0:4])[0]
        match name_hash:
            case Iris.Packages.Telemetry.Computer.NAME_HASH if len(raw) == Iris.Packages.Telemetry.Computer.BYTES_LENGTH:    
                return Iris.Packages.Telemetry.Computer.decode(raw)
            case Iris.Packages.Telemetry.Data.NAME_HASH if len(raw) == Iris.Packages.Telemetry.Data.BYTES_LENGTH:
                return Iris.Packages.Telemetry.Data.decode(raw)
            case Iris.Packages.Telemetry.Battery.NAME_HASH if len(raw) == Iris.Packages.Telemetry.Battery.BYTES_LENGTH:
                return Iris.Packages.Telemetry.Battery.decode(raw)
            case _:
                raise 'Unknown data.'
            

d = Iris.Packages.Telemetry.Data(
    computers=[
        Iris.Packages.Telemetry.Computer(
            id=123,
            batteries=[
                Iris.Packages.Telemetry.Battery(
                    charge=43.65
                ),
                Iris.Packages.Telemetry.Battery(
                    charge=88.32
                )
            ],
            status=Iris.Packages.Telemetry.Status.IDLE
        ),
        Iris.Packages.Telemetry.Computer(
            id=1337,
            batteries=[
                Iris.Packages.Telemetry.Battery(
                    charge=32.88
                ),
                Iris.Packages.Telemetry.Battery(
                    charge=123.312
                )
            ],
            status=Iris.Packages.Telemetry.Status.FLIGHT
        )
    ]
)

en = d.encode()
print([int(i) for i in en])

de = Iris.decode(en)

if isinstance(de, Iris.Packages.Telemetry.Data):
    print('TELEMETRY DATA')
    for c in de.computers:
        print('COMPUTER', c.id, c.status)
        print('BATTERIES:')
        for b in c.batteries:
            print(b.charge)
elif isinstance(de, Iris.Packages.Telemetry.Computer):
    pass
elif isinstance(de, Iris.Packages.Telemetry.Battery):
    pass
else:
    raise 'Error while decoding.'