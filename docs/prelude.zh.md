---
comments: true
---

# 预备知识

## 如何使用本文档

本文档采用通过示例代码来介绍相关智能合约的相关的知识点。智能合约涉及到的知识点相对比较少，为了更快速的掌握智能合约的用法，需要读者在阅读本文档的同时，运行和修改示例以加深对内容的理解。

## 什么是智能合约

智能合约是可以在链上执行的代码。区块链是一个分布式的数据库，或者称之为分布式账本。由一群有着相同功能的节点（计算机）负责维护。每个节点都保存了这个分布式数据库的一个副本。这些节点共同组成了一个区块链网络。所谓链上执行，即智能合约在每个节点都会被执行。智能合约的主要功能是对数据库进行增删改查等操作，并且由节点软件来确保每个节点的执行结果的一致性。

## 什么是 AssemblyScript 智能合约

AssemblyScript 智能合约是用 AssemblyScript 语言(一种语法和 Typescript 兼容的语言)写的可在链上执行的代码。以EOS网络为例，AssemblyScript智能合约的代码会被编译成叫 Webassebmly 的二进制文件，并且可以发布到链上并被执行，从而达成某种操作效果。

## 什么是EOS
EOS是一个基于代理权益证明(DPOS - Delegated Proof of Stake)共识算法的区块链网络。主网于2018年的6月8号正式上线。EOS的主网由21个块生产者(Block Producer)控制，块生产者简称为BP，由投票产生，负责将交易(Transaction)打包到区块中。

## 账号(Account)
不同于以太坊的通过地址（公钥）来进行交易，EOS区块链的交易的实体是比地址更复杂的账号。账号的名称是一个name结构，在下一节会讲到。账号在C++代码里表示的结构比较复杂。

下面是通过EOS的`get_account`RPC接口返回的信息来分集一下包含在账号里的信息：

```python
from pyeoskit import eosapi
eosapi.set_node("https://eos.greymass.com")
eosapi.get_account("testaccount")
```

```json
{
 "account_name": "testaccount",
 "head_block_num": 301601062,
 "head_block_time": "2023-03-28T06:19:09.500",
 "privileged": false,
 "last_code_update": "1970-01-01T00:00:00.000",
 "created": "2018-06-13T04:43:18.000",
 "core_liquid_balance": "0.0001 EOS",
 "ram_quota": 3052,
 "net_weight": 0,
 "cpu_weight": 0,
 "net_limit": {
  "used": 0,
  "available": 0,
  "max": 0,
  "last_usage_update_time": "2018-06-13T04:43:18.000",
  "current_used": 0
 },
 "cpu_limit": {
  "used": 0,
  "available": 0,
  "max": 0,
  "last_usage_update_time": "2018-06-13T04:43:18.000",
  "current_used": 0
 },
 "ram_usage": 2996,
 "permissions": [
  {
   "perm_name": "active",
   "parent": "owner",
   "required_auth": {
    "threshold": 1,
    "keys": [
     {
      "key": "EOS5eCkKszJt22****Y2YampuDDD8q95w2mF",
      "weight": 1
     }
    ],
    "accounts": [],
    "waits": []
   },
   "linked_actions": []
  },
  {
   "perm_name": "owner",
   "parent": "",
   "required_auth": {
    "threshold": 1,
    "keys": [
     {
      "key": "EOS5eCkKszJ****q95w2mF",
      "weight": 1
     }
    ],
    "accounts": [],
    "waits": []
   },
   "linked_actions": []
  }
 ],
 "total_resources": {
  "owner": "helloworld11",
  "net_weight": "0.0000 EOS",
  "cpu_weight": "0.0000 EOS",
  "ram_bytes": 3052
 },
 "self_delegated_bandwidth": null,
 "refund_request": null,
 "voter_info": null,
 "rex_info": null,
 "subjective_cpu_bill_limit": {
  "used": 0,
  "available": 0,
  "max": 0,
  "last_usage_update_time": "2000-01-01T00:00:00.000",
  "current_used": 0
 },
 "eosio_any_linked_actions": []
}
```

简单介绍一下主要的字段的意思：

- `account_name`: 账号名，规则在下一节中会讲到
- `privileged`: `true`表示账号是特权账号，如`eosio`即是特权账号。`false`则表示普通账号
- `last_code_update`: 账号中的智能合约的最后一次更新时间
- `created`: 账号的创建时间,
- `core_liquid_balance`: 账号的可用余额,
- `ram_quota`: 账号分配的总内存，由于EOS的数据库是内存数据库，所有的链上数据都是要放到内存中，而内存是有限的，所以将内存作为一种资源来分配给账号。
- `net_weight`: 账号分配到的网络资源的权重
- `cpu_weight`: 账号分配到的CPU资源的权重
- `net_limit`：账号的网络资源的使用情况
- `cpu_limit`CPU资源的使用情况
- `ram_usage`: 已经使用的内存
- `permissions`: 账号的权限，账号的权限包含一个或者多个公钥或者账号的信息，每个公钥和账号的权限又占一定的权重(weight)，在发送交易(transaction)时，必须用公钥对应的私钥对交易进行签名，并且权重要大于等于`threshold`时，这个交易才能被BP认可。当账号的权限里包含的不是公钥信息，而是指定继承自某个账号的权限信息时，在签名的时候，会从这个账号的权限信息里提取出公钥信息，这通过C++程序的算法来实现。EOS的RPC接口中也有一个`get_required_keys`接口来获取签名的公钥信息。
- `total_resources`：这里指定账号分配的NET，CPU， RAM等资源的信息
                                                                                                    
## Name 结构
name是EOS中的一个最基本的数据结构，在底层用一个64位的无符号整数(uint64_t)表示。

在C++中的定义如下：

[libraries/chain/include/eosio/chain/name.hpp](https://github.com/EOSIO/eos/blob/5082391c60b0fa5e68157c385cd402bf25aea934/libraries/chain/include/eosio/chain/name.hpp#L42)

```c++
   struct name {
      uint64_t value = 0;
      bool empty()const { return 0 == value; }
      bool good()const  { return !empty();   }

      name( const char* str )   { set(str);           } 
      name( const string& str ) { set( str.c_str() ); }
...
   }
```

但是在应用层使用的时候都是以字符串的形式表示的,字符串也只能包含这些字符：".12345abcdefghijklmnopqrstuvwxyz"。字符一共32个，分别用来表示0～31这32个数字，可以把这些字符串看作是32进制的数据，`uint64_t`中，每5位转换成一个上面的字符，由于`uint64_t`最多只有64位，所以前60位可以表示12个字符，字符的范围用正则表达式表示为`[.1-5a-z]`，而最高的4位，只可以用16个字符来表示，这16个字符用正则表达式表示的范围为`[.1-5a-j]`，

在具体的使用的过程中，如在创建账号的时候，经常犯的错误就是把`6`到`9`，`0`以及大写的字母都作为有效的字符，还有就是没有把长度限制在12个字符之内。

总结一下：

- EOS中，name的值在底层其实是一个`uint64_t`类型，在应用层的时候才用字符串表示，这个字符串最多可以有13个字符
- 第13个字符的范围比前12个字符能表示的范围小。
- 在用name结构来表示账号（account）名时，最多只有12个字符。
- 另外，name结构也用来表示一些其它类型，见下面的C++代码：

[libraries/chain/include/eosio/chain/types.hpp](https://github.com/EOSIO/eos/blob/5082391c60b0fa5e68157c385cd402bf25aea934/libraries/chain/include/eosio/chain/types.hpp#L133)

```c++
   using account_name     = name;
   using action_name      = name;
   using scope_name       = name;
   using permission_name  = name;
   using table_name       = name;
```

在这个C++代码中，name结构也用来表示action， table的名称等等，注意的是，不同于账号(account)名，在用字符串表示这些名称时，最多可以有13个字符，但是一般为了方便，习惯上用的时候也是最多用12个字符来表示这些名称。
                                                                                                    
## Transaction 结构

EOS上基本数据结构称为交易（Transaction），由BP负责将一段时间内收集到的交易打包成一个区块。智能合约开发者必须充分了解Transaction的数据结构。


[libraries/chain/include/eosio/chain/transaction.hpp](https://github.com/EOSIO/eos/blob/5082391c60b0fa5e68157c385cd402bf25aea934/libraries/chain/include/eosio/chain/transaction.hpp#L30)
```c++
struct transaction_header {
      time_point_sec         expiration;   ///< the time at which a transaction expires
      uint16_t               ref_block_num       = 0U; ///< specifies a block num in the last 2^16 blocks.
      uint32_t               ref_block_prefix    = 0UL; ///< specifies the lower 32 bits of the blockid at get_ref_blocknum
      fc::unsigned_int       max_net_usage_words = 0UL; /// upper limit on total network bandwidth (in 8 byte words) billed for this transaction
      uint8_t                max_cpu_usage_ms    = 0; /// upper limit on the total CPU time billed for this transaction
      fc::unsigned_int       delay_sec           = 0UL; /// number of seconds to delay this transaction for during which it may be canceled.

...
   };

   struct transaction : public transaction_header {
      vector<action>         context_free_actions;
      vector<action>         actions;
      extensions_type        transaction_extensions;
...
   };
```

简单解释下比较重要的字段：

- `expiration`, 设置transaction上链的超时时间，超时将被拒绝加入区块中。
- `ref_block_num`, `ref_block_prefix` 这两个成员变量是用来防止transaction在fork链上被重新包含进区块而设计的。
- `actions`, 这是一个action的数组结构，action的概念非常重要，每一个action都对应一个链上的智能合约函数，BP在将Transaction包含进区块的时候，都会根据action来调用对应的智能合约函数，这将在下面的一节中详细解释。
- `context_free_actions `, 这同样是一个action的数组，不同的是，当和action相对应的智能合约函数被调用时，执行的代码被禁止调用和链上数据库相关的API。
                                                                                                    
## Action 结构

Action结构包含在Transaction结构中。一个action结构在C++代码中的定义如下：

[libraries/chain/include/eosio/chain/action.hpp](https://github.com/EOSIO/eos/blob/5082391c60b0fa5e68157c385cd402bf25aea934/libraries/chain/include/eosio/chain/action.hpp#L60)

```c++
   struct action {
      account_name               account;
      action_name                name;
      vector<permission_level>   authorization;
      bytes                      data;
...
   }
```

其中，[permission_level](https://github.com/EOSIO/eos/blob/5082391c60b0fa5e68157c385cd402bf25aea934/libraries/chain/include/eosio/chain/action.hpp#L12)的定义如下：

```C++
struct permission_level {
    account_name    actor;
    permission_name permission;
};
```
结构中的成员变量意义解释如下：

- `account`用来指定要被调用的智能合约的账号名
- `name`被调用的action的名称
- `authorization`权限数组
- `data`action所包含的已经被序列化后的原始数据，在被智能合约所调用的时候，会被反序列化成具体的数据结构

                                                                                                    
## ABI(Application Binary Interface)

在开发智能合约的时候，在编译智能合约代码的过程中，正常情况下，在生成的每个智能合约的二进制代码(.wasm)的同时，都会生成一个ABI文件(.abi)。但是要注意这个文件并不是调用链上的智能合约所必须的。它的作用是方便开发者能够获取相关的action的信息，以构造相应的Transaction数据结构，以方便和区块链进行交互。

一个ABI文件的内容是json格式的数据，像下面这个样子：

```json
{
    "version": "eosio::abi/1.2",
    "structs": [
        {
            "name": "A",
            "base": "",
            "fields": [
                {
                    "name": "a",
                    "type": "uint64"
                },
                {
                    "name": "b",
                    "type": "uint64"
                },
                {
                    "name": "c",
                    "type": "uint128"
                }
            ]
        },
        {
            "name": "test",
            "base": "",
            "fields": []
        }
    ],
    "actions": [
        {
            "name": "test",
            "type": "test",
            "ricardian_contract": ""
        }
    ],
    "tables": [
        {
            "name": "mytable",
            "type": "A",
            "index_type": "i64",
            "key_names": [],
            "key_types": []
        }
    ],
    "ricardian_clauses": []
}
```

- `version`用来指定ABI的版本
- `structs`用来指定数据结构，会在`actions`和`tables`这两个结构里被用到
- `actions`用来描述智能合约中的action的参数，每个action实际上都是对应一个智能合约函数
- `tables`用来描述表的信息，这样，你网页应用程序就可以通过`get_table_rows`这个RPC API来查询链上的数据库信息
