---
comments: true
---

# Detailed Explanation of ABI Types

# Built-in ABI Types

Here are the built-in ABI types, a total of 31:

- Basic types: name bytes string
- Numerical types: bool int8 uint8 int16 uint16 int32 uint32 int64 uint64 int128 uint128 varint32 varuint32 float32 float64 float128
- Time-related: time_point time_point_sec block_timestamp_type
- Cryptographic function related: checksum160 checksum256 checksum512 public_key signature
- Token-related: symbol symbol_code asset extended_asset

The ones commonly used are as follows:

```
name bytes string bool uint64 checksum256
public_key signature symbol asset extended_asset
```
                                                                                                    
# Table of Corresponding Relations between Built-in Data Types in ABI and Data Types in Rust

The table below shows the corresponding relations between built-in types in ABI and types in Rust. For the corresponding module, if it is not built-in, it is defined in the related module in the `rust-chain` Rust package. Moreover, these structures have been exported to `lib.rs`, so the corresponding structures can be referred to in the following way:

```rust
use rust_chain::{
    TimePoint,
    TimePointSec,
    Name,
    Checksum160,
    Checksum256,
    Checksum512,
    PublicKey,
    Signature,
    Symbol,
    SymbolCode,
    Asset,
    ExtendedAsset,
};
```

Relation Table:

|         ABI Type     |   Rust Type       |      Module    |
|:--------------------:|:------------------:|:------------------:|
|         bool         |        bool        |   Built-in    |
|         int8         |         i8         |   Built-in    |
|         uint8        |         u8         |   Built-in    |
|         int16        |         i16        |   Built-in    |
|         int32        |         i32        |   Built-in    |
|        uint32        |         u32        |   Built-in    |
|         int64        |         i64        |   Built-in    |
|        uint64        |         u64        |   Built-in    |
|        int128        |        i128        |   Built-in    |
|        uint128       |        u128        |   Built-in    |
|       varint32       |                    |                |
|       varuint32      |      VarUint32     |   varint     |
|        float32       |     float32        |  Built-in     |
|        float64       |       float        |  Built-in     |
|       float128       |      Float128      |  structs  |
|      time_point      |      TimePoint     |  structs  |
|    time_point_sec    |    TimePointSec    |  structs  |
| block_timestamp_type | BlockTimestampType |  structs  |
|         name         |        Name        |  name  |
|         bytes        |        Vec<u8>     |  Built-in  |
|        string        |        String      |  Built-in  |
|      checksum160     |     Checksum160    |  structs  |
|      checksum256     |   Checksum256/u256 |  structs  |
|      checksum512     |     Checksum512    |  structs  |
|      public_key      |      PublicKey     |  structs  |
|       signature      |      Signature     |  structs  |
|        symbol        |       Symbol       | asset   |
|      symbol_code     |     SymbolCode     | asset   |
|         asset        |        Asset       | asset   |
