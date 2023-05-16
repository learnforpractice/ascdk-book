---
comments: true
---

# Explanation of ABI types

## Built-in ABI types

Here are the built-in ABI types, a total of 31:

- Basic types: name bytes string
- Numerical types: bool int8 uint8 int16 uint16 int32 uint32 int64 uint64 int128 uint128 varint32 varuint32 float32 float64 float128
- Time-related: time_point time_point_sec block_timestamp_type
- Password-related functions: checksum160 checksum256 checksum512 public_key signature
- Token-related: symbol symbol_code asset extended_asset

The following are the most commonly used:

```
name bytes string bool uint64 checksum256
public_key signature symbol asset extended_asset
```

## Correspondence table of built-in data types in ABI and data types in Python

The table below shows the correspondence between the built-in types in ABI and the types in Python.

It should be noted that although some of these types corresponding to Python belong to different modules, they have all been imported into the global environment in advance. Therefore, developers can directly use these ABI types corresponding to Python without needing to import them in the following way:

```python
from chain.crypto import PublicKey
```

Of course, repeated imports will not have any impact.

|         ABI Type     |   Python Type       |      Module    |
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
|       varint32       |      VarInt32      |   Structs |
|       varuint32      |      VarUint32     |   Structs |
|        float32       |     float32        |  Built-in     |
|        float64       |       float        |  Built-in     |
|       float128       |      Float128      |  Structs  |
|      time_point      |      TimePoint     |  Structs  |
|    time_point_sec    |    TimePointSec    |  Structs  |
| block_timestamp_type | BlockTimestampType |  Structs  |
|         name         |        Name        |  Name  |
|         bytes        |        bytes       |  Built-in  |
|        string        |        str         |  Built-in  |
|      checksum160     |     Checksum160    |  Crypto  |
|      checksum256     |   Checksum256/u256 |  Crypto  |
|      checksum512     |     Checksum512    |  Crypto  |
|      public_key      |      PublicKey     |  Crypto  |
|       signature      |      Signature     |  Crypto  |
|        symbol        |       Symbol       | Asset   |
|      symbol_code     |     SymbolCode     | Asset   |
|         asset        |        Asset       | Asset   |
|    extended_asset    |    ExtendedAsset   | Asset   |

## Special ABI types

### Optional

### Variant

### Binaryextension
