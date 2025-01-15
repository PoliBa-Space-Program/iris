#pragma once
#include <cstdint>
#include <cstddef>
namespace iris
{
    typedef uint8_t byte;
    auto is_le = []()
    { int a = 1; return 1 == (int)*((byte*)(&a)) ? true : false; };
    template <typename T>
    void to_be_bytes(T data, byte *buffer)
    {
        for (int i = 0; i < sizeof(data); i++)
        {
            buffer[i] = *((byte *)(&data) + (is_le ? sizeof(data) - 1 - i : i));
        }
    }
    template <typename T>
    T from_be_bytes(byte *buffer)
    {
        T data;
        for (int i = 0; i < sizeof(T); i++)
        {
            *((iris::byte *)(&data) + (is_le ? sizeof(T) - 1 - i : i)) = buffer[i];
        }
        return data;
    }
    namespace Telemetry
    {
        class Status
        {
        public:
            enum Value : uint32_t
            {
                SLEEP = 0,
                FLIGHT = 2,
                ACTIVE = 1,
                IDLE = 3,
            };
            static const size_t BYTES_LENGTH = 4;
            iris::byte DATA_BUFFER[4] = {0};
            Value value;
            Status() {}
            Status(uint32_t value)
            {
                this->value = Value(value);
            }
            inline iris::byte *to_be_bytes()
            {
                return this->to_be_bytes(this->DATA_BUFFER);
            }
            iris::byte *to_be_bytes(iris::byte *buffer)
            {
                iris::to_be_bytes(this->value, buffer);
                return buffer;
            }
            static Status from_be_bytes(iris::byte *raw)
            {
                return Status(iris::from_be_bytes<uint32_t>(raw));
            }
        };
        class Battery
        {
        public:
            static const uint32_t NAME_HASH = 2215305518;
            static const size_t BYTES_LENGTH = 4 + 4;
            iris::byte DATA_BUFFER[4 + 4] = {0};
            float charge;
            Battery() {}
            Battery(float charge)
            {
                this->charge = charge;
            }
            iris::byte *encode()
            {
                iris::to_be_bytes(this->NAME_HASH, this->DATA_BUFFER);
                this->to_be_bytes();
                return this->DATA_BUFFER;
            }
            inline iris::byte *to_be_bytes()
            {
                return this->to_be_bytes(this->DATA_BUFFER + 4);
            }
            iris::byte *to_be_bytes(iris::byte *buffer)
            {
                int i = 0;
                iris::to_be_bytes(this->charge, buffer + i);
                i += sizeof(this->charge);
                return buffer;
            }
            static Battery decode(iris::byte *raw)
            {
                return Battery::from_be_bytes(raw + 4);
            }
            static Battery from_be_bytes(iris::byte *raw)
            {
                Battery out = Battery();
                int i = 0;
                out.charge = iris::from_be_bytes<float>(raw + i);
                i += sizeof(float);
                return out;
            }
        };
        class Computer
        {
        public:
            static const uint32_t NAME_HASH = 3613607352;
            static const size_t BYTES_LENGTH = 16 + 4;
            iris::byte DATA_BUFFER[16 + 4] = {0};
            Battery batteries[2];
            Status status;
            uint32_t id;
            Computer() {}
            Computer(uint32_t id, Battery *batteries, Status status)
            {
                for (int i = 0; i < 2; i++)
                {
                    this->batteries[i] = batteries[i];
                }
                this->status = status;
                this->id = id;
            }
            iris::byte *encode()
            {
                iris::to_be_bytes(this->NAME_HASH, this->DATA_BUFFER);
                this->to_be_bytes();
                return this->DATA_BUFFER;
            }
            inline iris::byte *to_be_bytes()
            {
                return this->to_be_bytes(this->DATA_BUFFER + 4);
            }
            iris::byte *to_be_bytes(iris::byte *buffer)
            {
                int i = 0;
                iris::to_be_bytes(this->id, buffer + i);
                i += sizeof(this->id);
                for (int j = 0; j < 2; j++)
                {
                    this->batteries[j].to_be_bytes(buffer + i);
                    i += this->batteries[j].BYTES_LENGTH - 4;
                }
                this->status.to_be_bytes(buffer + i);
                i += this->status.BYTES_LENGTH;
                return buffer;
            }
            static Computer decode(iris::byte *raw)
            {
                return Computer::from_be_bytes(raw + 4);
            }
            static Computer from_be_bytes(iris::byte *raw)
            {
                Computer out = Computer();
                int i = 0;
                out.id = iris::from_be_bytes<uint32_t>(raw + i);
                i += sizeof(uint32_t);
                for (int j = 0; j < 2; j++)
                {
                    out.batteries[j] = Battery::from_be_bytes(raw + i);
                    i += Battery::BYTES_LENGTH - 4;
                }
                out.status = Status::from_be_bytes(raw + i);
                i += Status::BYTES_LENGTH;
                return out;
            }
        };
        class Data
        {
        public:
            static const uint32_t NAME_HASH = 1062369733;
            static const size_t BYTES_LENGTH = 32 + 4;
            iris::byte DATA_BUFFER[32 + 4] = {0};
            Computer computers[2];
            Data() {}
            Data(Computer *computers)
            {
                for (int i = 0; i < 2; i++)
                {
                    this->computers[i] = computers[i];
                }
            }
            iris::byte *encode()
            {
                iris::to_be_bytes(this->NAME_HASH, this->DATA_BUFFER);
                this->to_be_bytes();
                return this->DATA_BUFFER;
            }
            inline iris::byte *to_be_bytes()
            {
                return this->to_be_bytes(this->DATA_BUFFER + 4);
            }
            iris::byte *to_be_bytes(iris::byte *buffer)
            {
                int i = 0;
                for (int j = 0; j < 2; j++)
                {
                    this->computers[j].to_be_bytes(buffer + i);
                    i += this->computers[j].BYTES_LENGTH - 4;
                }
                return buffer;
            }
            static Data decode(iris::byte *raw)
            {
                return Data::from_be_bytes(raw + 4);
            }
            static Data from_be_bytes(iris::byte *raw)
            {
                Data out = Data();
                int i = 0;
                for (int j = 0; j < 2; j++)
                {
                    out.computers[j] = Computer::from_be_bytes(raw + i);
                    i += Computer::BYTES_LENGTH - 4;
                }
                return out;
            }
        };
    }
    template <typename T>
    T decode(byte *raw, size_t len)
    {
        uint32_t struct_name_hash = from_be_bytes<uint32_t>(raw);
        if (struct_name_hash == Telemetry::Data::NAME_HASH && len == Telemetry::Data::BYTES_LENGTH)
        {
            return T::decode(raw);
        }
        else if (struct_name_hash == Telemetry::Computer::NAME_HASH && len == Telemetry::Computer::BYTES_LENGTH)
        {
            return T::decode(raw);
        }
        else if (struct_name_hash == Telemetry::Battery::NAME_HASH && len == Telemetry::Battery::BYTES_LENGTH)
        {
            return T::decode(raw);
        }
        else
        {
            throw 1;
        }
    }
    enum Structs
    {
        Telemetry_Data,
        Telemetry_Computer,
        Telemetry_Battery,
    };
    Structs check_type(byte *raw, size_t len)
    {
        uint32_t struct_name_hash = from_be_bytes<uint32_t>(raw);
        if (struct_name_hash == Telemetry::Data::NAME_HASH && len == Telemetry::Data::BYTES_LENGTH)
        {
            return Structs::Telemetry_Data;
        }
        else if (struct_name_hash == Telemetry::Computer::NAME_HASH && len == Telemetry::Computer::BYTES_LENGTH)
        {
            return Structs::Telemetry_Computer;
        }
        else if (struct_name_hash == Telemetry::Battery::NAME_HASH && len == Telemetry::Battery::BYTES_LENGTH)
        {
            return Structs::Telemetry_Battery;
        }
        else
        {
            throw 1;
        }
    }
}
