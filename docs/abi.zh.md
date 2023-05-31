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
                                                                                                    
# ABI中的内置数据类型和 AssemblyScript 中的数据类型的对应关系表

下面的这张表显示了ABI中的内置类型和 AssemblyScript 中的类型的对应关系.其中的所属模块，如果是非内置，则是在`asm-chain`这个 nodejs package中相关的模块中定义的，可以以下面的方式来引用相应的结构体：

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

关系表：

|         ABI 类型     |   AssemblyScript 类型       |      所属模块    |
|:--------------------:|:------------------:|:------------------:|
|         bool         |        bool        |   内置    |
|         int8         |         i8         |   内置    |
|         uint8        |         u8         |   内置    |
|         int16        |         i16        |   内置    |
|         int32        |         i32        |   内置    |
|        uint32        |         u32        |   内置    |
|         int64        |         i64        |   内置    |
|        uint64        |         u64        |   内置    |
|        int128        |        I128        |   内置    |
|        uint128       |        I128        |   内置    |
|       varint32       |                    |          |
|       varuint32      |      VarUint32     |   varint |
|        float64       |       f64        |  内置     |
|       float128       |      Float128      |  float128  |
|      time_point      |      TimePoint     |  time  |
|    time_point_sec    |    TimePointSec    |  time  |
|         name         |        Name        |  name  |
|         bytes        |        u8[]     |  内置  |
|        string        |        string      |  内置  |
|      checksum160     |     Checksum160    |  crypto  |
|      checksum256     |   Checksum256/u256 |  crypto  |
|      checksum512     |     Checksum512    |  crypto  |
|      public_key      |      PublicKey     |  crypto  |
|       signature      |      Signature     |  crypto  |
|        symbol        |       Symbol       | asset   |
|      symbol_code     |     SymbolCode     | asset   |
|         asset        |        Asset       | asset   |
|    extended_asset    |    ExtendedAsset   | asset   |
                                                                                                    
# 特别的ABI类型

## optional

## variant

## binaryextension
