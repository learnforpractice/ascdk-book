---
comments: true
---

# ABI类型详解

# 内置的ABI类型

以下是内置的ABI类型，一共31个

- 基本类型：name bytes string
- 数值类型： bool int8 uint8 int16 uint16 int32 uint32 int64 uint64 int128 uint128 varint32 varuint32 float32 float64 float128
- 时间相关：time_point time_point_sec block_timestamp_type
- 密码函数相关：checksum160 checksum256 checksum512 public_key signature
- Token相关：symbol symbol_code asset extended_asset

比较常用的有下面这些：

```
name bytes string bool uint64 checksum256
public_key signature symbol asset extended_asset
```
                                                                                                    
# ABI中的内置数据类型和Rust中的数据类型的对应关系表

下面的这张表显示了ABI中的内置类型和Rust中的类型的对应关系.其中的所属模块，如果是非内置，则是在`rust-chain`这个rust package中相关的模块中定义的，另外，这些结构都已经导出到`lib.rs`，所以可以以下面的方式来引用相应的结构体：

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

关系表：

|         ABI 类型     |   Rust 类型       |      所属模块    |
|:--------------------:|:------------------:|:------------------:|
|         bool         |        bool        |   内置    |
|         int8         |         i8         |   内置    |
|         uint8        |         u8         |   内置    |
|         int16        |         i16        |   内置    |
|         int32        |         i32        |   内置    |
|        uint32        |         u32        |   内置    |
|         int64        |         i64        |   内置    |
|        uint64        |         u64        |   内置    |
|        int128        |        i128        |   内置    |
|        uint128       |        u128        |   内置    |
|       varint32       |                    |          |
|       varuint32      |      VarUint32     |   varint |
|        float32       |     float32        |  内置     |
|        float64       |       float        |  内置     |
|       float128       |      Float128      |  structs  |
|      time_point      |      TimePoint     |  structs  |
|    time_point_sec    |    TimePointSec    |  structs  |
| block_timestamp_type | BlockTimestampType |  structs  |
|         name         |        Name        |  name  |
|         bytes        |        Vec<u8>     |  内置  |
|        string        |        String      |  内置  |
|      checksum160     |     Checksum160    |  structs  |
|      checksum256     |   Checksum256/u256 |  structs  |
|      checksum512     |     Checksum512    |  structs  |
|      public_key      |      PublicKey     |  structs  |
|       signature      |      Signature     |  structs  |
|        symbol        |       Symbol       | asset   |
|      symbol_code     |     SymbolCode     | asset   |
|         asset        |        Asset       | asset   |
|    extended_asset    |    ExtendedAsset   | asset   |
                                                                                                    
# 特别的ABI类型

## optional

## variant

## binaryextension
