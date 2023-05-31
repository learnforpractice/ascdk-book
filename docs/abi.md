# Detailed Explanation of ABI Types

# Built-in ABI Types

Here are the built-in ABI types, a total of 31:

- Basic types: name, bytes, string
- Numeric types: bool, int8, uint8, int16, uint16, int32, uint32, int64, uint64, int128, uint128, varint32, varuint32, float32, float64, float128
- Time-related: time_point, time_point_sec, block_timestamp_type
- Cryptographic function related: checksum160, checksum256, checksum512, public_key, signature
- Token related: symbol, symbol_code, asset, extended_asset

The most commonly used are the following:

```
name bytes string bool uint64 checksum256
public_key signature symbol asset extended_asset
```

# Correspondence Table of Built-in Data Types in ABI and AssemblyScript

The following table shows the correspondence between built-in types in ABI and types in AssemblyScript. If it is not built-in, it is defined in the relevant module in the `asm-chain` nodejs package and can be referenced using the structure as follows:

```ts
import {
    Name,
    Asset,
    Symbol,
    SymbolCode,

    VarUint32,
    printString,
    Checksum160,
    Checksum256,
    Checksum512,

    PublicKey,
    Signature,
    TimePoint,
    TimePointSec,
    ExtendedAsset,
    I128,
    U128,
} from "asm-chain";
```

Relationship table:

| ABI Type | AssemblyScript Type | Module |
|:--------:|:-------------------:|:-----------------:|
| bool | bool | Built-in |
| int8 | i8 | Built-in |
| uint8 | u8 | Built-in |
| int16 | i16 | Built-in |
| int32 | i32 | Built-in |
| uint32 | u32 | Built-in |
| int64 | i64 | Built-in |
| uint64 | u64 | Built-in |
| int128 | I128 | Built-in |
| uint128 | I128 | Built-in |
| varint32 | | |
| varuint32 | VarUint32 | varint |
| float64 | f64 | Built-in |
| float128 | Float128 | float128 |
| time_point | TimePoint | time |
| time_point_sec | TimePointSec | time |
| name | Name | name |
| bytes | u8[] | Built-in |
| string | string | Built-in |
| checksum160 | Checksum160 | crypto |
| checksum256 | Checksum256/u256 | crypto |
| checksum512 | Checksum512 | crypto |
| public_key | PublicKey | crypto |
| signature | Signature | crypto |
| symbol | Symbol | asset |
| symbol_code | SymbolCode | asset |
| asset | Asset | asset |
| extended_asset | ExtendedAsset | asset |

# Special ABI Types

## optional

## variant

## binaryextension
