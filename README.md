# iris
Serialization format that can run everywhere.
This format was created in order to serialize and make easier the interpretation of data received by the flight computer of a rocket.

The first choice was [Protocol Buffers](https://protobuf.dev/) (used in [gRPC](https://grpc.io/)), but an implementation for emebedded systems was needed. In particular in Rust no std.

Also, [serde](https://github.com/serde-rs/serde) was not an option because only available for Rust but I use also C++, Python and other languages depending on what is needed.

The main needs are:
 - Low bandwidth
 - Easy to encode and decode
 - Fast to encode and decode
 - Run on embedded systems


## Index
 - [Specification](#specification)
   - [Version](#version)
   - [Package](#package)
   - [Field number](#field-number)
   - [Field order](#field-order)
   - [Arrays](#arrays)
   - [Supported types](#supported-types)
   - [Comments](#comments)
 - [File format regex](#file-format-regex)
   - [Version](#version-1)
   - [Package](#package-1)
   - [Struct](#struct)
   - [Struct field](#struct-field)
 - [Encoding](#encoding)
 - [Usage](#usage)
 - [Examples](#examples)

## Specification

### Version
The version is used to manage compatibility.
The version must be declared on top of the file.
```
version 1.3.13
```

### Package
The name of the package, it will be used when generating the classes.
The name of the package must be placed after the version.
```
package TheGreatesPackage_ever
```

### Field number
The max number of field in a single struct is 2^32.

### Field order
The fields are encoded in the same order as they are declared. The first field will be the first found in the encoded bytes.

### Arrays
Only 1-D arrays are supported.
All the arrays need to have known size at compile time.
Because this serialization format is meant to run on embedded systems with limited resources, we can't use the heap (we could but the trouble in most cases is not paid off). For this reason no dynamic data structures, but the size must be known at compile time.
```
struct A:
    u32[11] array
```


### Supported types
The supported data types are:
| Size | Types |
| --- | --- |
| 1 | i8, u8, bool |
| 2 | i16, u16 |
| 4 | f32, i32, u32 |


### Comments
To create a single line comment use `#`.
```
version 23.1.3 # This is a comment

# And this is another comment
package Something
```

## File format regex
### Version
```
^(?<version>version) +(?<number>(?<major>[0-9]+)\.(?<minor>[0-9]+)\.(?<patch>[0-9]+)) *(#.*)?$
```
### Package
```
^(?<package>package) +(?<name>[_a-zA-Z][_a-zA-Z0-9]*) *(#.*)?$
```
### Struct
```
^(?<struct>struct) +(?<name>[_a-zA-Z][_a-zA-Z0-9]*): *(#.*)?$
```
### Struct field
```
^ {4}(?<type>i8|i16|i32|u8|u16|u32|f32|bool)(?<array>\[[0-9]+\])? +(?<name>[_a-zA-Z][_a-zA-Z0-9]*) *(#.*)?$
```
### Enum
```
^(?<enum>enum) +(?<name>[_a-zA-Z][_a-zA-Z0-9]*): *(#.*)?$
```
### Enum field
```
^ {4}(?<name>[_a-zA-Z][_a-zA-Z0-9]*): [+-]?[0-9]+ *(#.*)?$
```


## Encoding
An encoded struct is composed by:
 - struct id: 32-bit hash of the struct name
 - field: MSB field value

The hashing function used is [fnv-1a](https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV-1a_hash).


## Usage
Clone the repository, make sure to have installed the latest version of rustc and cargo.

In the folder open a terminal and type:
```
cargo run file.iris
```
By default it will generate Rust code.

Using the flag `--lang` or `-l` you can specify the language to use. The supported languages are:
| Language | Accepted values |
| --- | --- |
| Rust | rust, rs |
| Python | python, py |
| C++ | c++, cpp |
| C | c |

Example:
```
cargo run file.iris --lang cpp
```
It will crate `iris.hpp` file in current path.


Using the flag `--out` or `-o` you can specify the output foler where the file will be created.
```
cargo run file.iris --out ./foo/aaaa/folder
```
The file `iris.rs` will be created in `./foo/aaaa/folder`.


## Examples
Check the `examples` folder to see how to use the generated code.

## To Do
Next things to do:
 - enums
 - imports
 - N-dimensions arrays
 - Extend code generation to other languages (Python, C++, C)