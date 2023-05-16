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
                                                                                                    
# ABI中的内置数据类型和Python中的数据类型的对应关系表

下面的这张表显示了ABI中的内置类型和Python中的类型的对应关系.

需要说明一下的是，所以这些和Python对应的类型虽然有些属于不同的模块，但是都已经事先import到global的环境中了，所以开发者在使用的过程中，Python中对应的这些ABI类型可以直接用，不用再去用例如下面的方式来import:

```python
from chain.crypto import PublicKey
```

当然，重复import也不会有什么影响

关系表：

|         ABI 类型     |   Python 类型       |      所属模块    |
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
|       varint32       |      VarInt32      |   structs |
|       varuint32      |      VarUint32     |   structs |
|        float32       |     float32        |  内置     |
|        float64       |       float        |  内置     |
|       float128       |      Float128      |  structs  |
|      time_point      |      TimePoint     |  structs  |
|    time_point_sec    |    TimePointSec    |  structs  |
| block_timestamp_type | BlockTimestampType |  structs  |
|         name         |        Name        |  name  |
|         bytes        |        bytes       |  内置  |
|        string        |        str         |  内置  |
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
