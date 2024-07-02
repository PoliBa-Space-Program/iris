# iris
Serialization format based on Protocol Buffers.
This format was created in order to serialize and make easier the interpretation of data received by the flight computer of a rocket.
Instead of just implemanting Protocol Buffers for a specific platform, I decided to make some changes in order to reduce the amount of computation required.
The main needs are:
 - Low bandwidth usage
 - Data serialization
 - Easy to encode and decode
 - Fast to encode and decode
 - Run on a STM32


## Specification

### Version
The version is used to manage compatibility.
The version must be declared at the top of the file.

### Package
The name of the package, it will be used when generating the classes.
The name of the package must be placed after the version.

### Field number
The max number of field is 2^32.

### Field order
The fields are encoded in the same order as they are declared. So the first field will be the first found in the encoded bytes.

### Arrays
Only 1-D arrays are supported, to create an N-dimensions array it's necessary to create a separated struct containing a 1-D array.
All the arrays need to have known size at compile time. An array cannot be greater than 16KB (aribtrary limit set to avoid filling all the RAM of an embedded system).
Because this serialization format is meant to run on embedded sysstems with limited resources, we can't use the heap (we could but the trouble in most cases is not paid off). For this reason no dynamic data structures, but the size must be known at compile time.

### Default values
By default every type is associated with a default value:
| Type | Default |
| --- | --- |
| B8, B16, B32 | null bytes |
| LEN | empty array |

### Supported types
The supported data types are:
 - int (8-bit, 16-bit, 32-bit)
 - uint (8-bit, 16-bit, 32-bit)
 - float (32-bit)
 - bool (8-bit)
 - byte (8-bit) -> alias u8
 - enum (32-bit) -> alias i32

The wire type are:
| ID | Name	| Used For |
| --- | --- | --- |
| 0 | LEN | i8[], u8[], i16[], u16[], i32[], u32[], bool[], f32[] |
| 1 | B8 | i8, u8 |
| 2 | B16 | i16, u16 |
| 3 | B32 | f32, i32, u32 |


## File format regex
### Version
```
version +[0-9]+\.[0-9]+\.[0-9]+
```
### Package
```
package +[_a-zA-Z][_a-zA-Z0-9]*
```
### Struct
```
struct +[_a-zA-Z][_a-zA-Z0-9]*:
```
### Field
```
 {4}(optional +)?(i8|i16|i32|u8|u16|u32|f32|bool|byte|[_a-zA-Z][_a-zA-Z0-9]*)(\[[0-9]*\])? +[_a-zA-Z][_a-zA-Z0-9]*: +[0-9]+
```


## Encoding
An encoded struct is composed by:
 - struct id: 32-bit hash of the struct name
 - field: field value
```
B8: fixed 8-bit type
B16: fixed 16-bit type
B32: fixed 32-bit type
LEN: variable length of bytes
```

## Generated code

### Type conversion


```
version 0.1.0

package Telemetry

struct FlightData:
    f32 max_altitude
    f32 max_velocity
    u32 computer_id
    f32 current_altitude
    f32[4] mesured_temperatures
```
```
#![no_std]
pub mod utils {
    pub fn from_be_bytes_u8(a: &[u8; 1]) -> u8 {
        a[0]
    }
    pub fn from_be_bytes_u16(a: &[u8; 2]) -> u16 {
        ((a[0] as u16) << 8) | 
        ((a[1] as u16) << 0)
    }
    pub fn from_be_bytes_u32(a: &[u8; 4]) -> u32 {
        ((a[0] as u32) << 32) | 
        ((a[1] as u32) << 16) |
        ((a[2] as u32) << 8) |
        ((a[3] as u32) << 0)
    }
    pub fn from_be_bytes_i8(a: &[u8; 1]) -> i8 {
        a[0] as i8
    }
    pub fn from_be_bytes_i16(a: &[u8; 2]) -> i16 {
        ((a[0] as i16) << 8) | 
        ((a[1] as i16) << 0)
    }
    pub fn from_be_bytes_i32(a: &[u8; 4]) -> i32 {
        ((a[0] as i32) << 32) | 
        ((a[1] as i32) << 16) |
        ((a[2] as i32) << 8) |
        ((a[3] as i32) << 0)
    }
    pub fn from_be_bytes_f32(a: &[u8; 4]) -> f32 {
        (((a[0] as u32) << 32) | 
        ((a[1] as u32) << 16) |
        ((a[2] as u32) << 8) |
        ((a[3] as u32) << 0)) as f32
    }
    pub fn from_be_bytes_bool(a: &[u8; 1]) -> bool {
        a[0] != 0
    }
}

pub mod iris {
    pub mod Telemetry {
        pub struct FlightData {
            max_altitude: f32,
            max_velocity: f32,
            computer_id: u32,
            current_altitude: f32,
            mesured_temperatures: [f32; 4]
        }

        impl FlightData {
            pub const NAME_HASH: u32 = 6161343;
            pub const LENGTH_BYTES: u32 = 32 + 4;

            pub fn to_be_bytes(&self) -> [u8; LENGTH_BYTES] {
                self.encode()
            }
            pub fn encode(&self) -> [u8; LENGTH_BYTES] {
                let mut data: [u8; self.LENGTH_BYTES] = [0; self.LENGTH_BYTES];
                let mut index = 0;

                for x in self.NAME_HASH.to_be_bytes() {
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
                        index += 1
                    }
                }

                data
            }
            pub fn decode(&self, data: &[u8]) -> FlightData {

            }
        }
    }

    enum DecodeRes {
        Telemetry_FlightData(Telemetry::FlightData)
    }

    pub fn decode(data: &[u8]) -> Result<DecodeRes, &str> {
        let struct_name_hash = ((data[0] as u32) << 32) | ((data[1] as u32) << 16) | ((data[2] as u32) << 8) | ((data[3] as u32) << 0);
                
        match struct_name_hash {
            Telemetry::FlightData::NAME_HASH if data.len() == Telemetry::FlightData::BYTES_LENGTH => Ok(DecodeRes::Telemetry_FlightData(Telemetry::FlightData::decode(&data))),
            _ => Err("Unknown data.")
        }
    }
}
```