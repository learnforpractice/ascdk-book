---
comments: true
---

# Prerequisite Knowledge

## How to Use This Document

This document uses example code to introduce relevant knowledge points related to smart contracts. The knowledge points involved in smart contracts are relatively few. In order to master the use of smart contracts more quickly, readers need to run the examples while reading the content of this document.

## What is a Smart Contract

Smart contracts are code that can be executed on a blockchain. A blockchain is a distributed database, also known as a distributed ledger, maintained by a group of nodes (computers) with the same functionality. Each node stores a copy of this distributed database. These nodes collectively form a blockchain network. On-chain execution means that smart contracts are executed on every node. The main function of smart contracts is to perform operations such as adding, deleting, and modifying data in the database, and the node software ensures the consistency of the execution results on each node.

## What is an AssemblyScript Smart Contract

An AssemblyScript smart contract is a code that can be executed on-chain, written in the AssemblyScript language (a language syntax compatible with TypeScript). Using the EOS network as an example, the code for an AssemblyScript smart contract would be compiled into a binary file called WebAssembly, and it can be published and executed on-chain, thus achieving a certain operational effect.

## What is EOS

EOS is a blockchain network based on the Delegated Proof of Stake (DPOS) consensus algorithm. The main network was officially launched on June 8, 2018. The EOS mainnet is controlled by 21 Block Producers (BP) who are elected by voting and are responsible for packaging transactions into blocks.

## Account

On the EOS blockchain, each transaction entity is represented by an account. The account name is a `name` structure, which will be discussed in the next section. The account structure in C++ code is relatively complex.

The following is a brief overview of the information contained in an account, based on the information returned by EOS's `get_account` RPC interface with the following code:

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
  "owner": "testaccount",
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

Let's briefly introduce the meanings of the main fields:

- `account_name`: Account name, the rules will be discussed in the next section
- `privileged`: `true` indicates that the account is a privileged account, such as `eosio` being a privileged account. `false` means it's a regular account
- `last_code_update`: The last update time of the smart contract in the account
- `created`: Account creation time
- `core_liquid_balance`: Account's available balance
- `ram_quota`: Total memory allocated to the account. Since the EOS database is an in-memory database, all on-chain data must be stored in memory, and memory is limited, so memory is allocated as a resource to accounts.
- `net_weight`: The weight of the network resources allocated to the account
- `cpu_weight`: The weight of the CPU resources allocated to the account
- `net_limit`: Account's network resource usage
- `cpu_limit`: CPU resource usage
- `ram_usage`: Already used memory
- `permissions`: Account permissions; account permissions contain one or more public keys or account information, each public key and account permission occupies a certain weight, when sending a transaction, the private key corresponding to the public key must be used to sign the transaction, and the weight must be greater or equal to the `threshold` for the transaction to be recognized by BP. When the permissions of the account contain not public key information but designate inherited permissions from a certain account, the public key information will be extracted from the permission information of this account during signing, which is implemented through the algorithm of the C++ program. EOS's RPC interface also has a `get_required_keys` interface to obtain the public key information for signing.
- `total_resources`: This specifies the information of resources allocated to the account, such as NET, CPU, RAM, etc.

## Name Structure
`name` is one of the most basic data structures in EOS, represented at the low level by a 64-bit unsigned integer (uint64_t).

The definition in C++ is as follows:

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

However, when used at the application layer, it is represented as a string, and the string can only contain these characters: ".12345abcdefghijklmnopqrstuvwxyz". There are a total of 32 characters, each representing one of the numbers 0-31. These strings can be considered as base-32 data, with each 5 bits in the `uint64_t` converted to one of the characters above. Since `uint64_t` has a maximum of 64 bits, the first 60 bits can represent 12 characters, with the character range represented by the regular expression `[.1-5a-z]`. The highest 4 bits can only be represented by 16 characters, with the range of these characters represented by the regular expression `[.1-5a-j]`.

In practice, such as when creating an account, common mistakes include treating '6' to '9', '0', and uppercase letters as valid characters, and not limiting the length to 12 characters.

In summary:

- In EOS, the value of `name` is actually a `uint64_t` type at the low level, and is represented as a string when used at the application layer. This string can have up to 13 characters.
- The range of the 13th character is smaller than the range that the first 12 characters can represent.
- When using the `name` structure to represent an account name, there can be up to 12 characters.
- In addition, the `name` structure is also used to represent some other types, as shown in the following C++ code:

[libraries/chain/include/eosio/chain/types.hpp](https://github.com/EOSIO/eos/blob/5082391c60b0fa5e68157c385cd402bf25aea934/libraries/chain/include/eosio/chain/types.hpp#L133)

```c++
   using account_name     = name;
   using action_name      = name;
   using scope_name       = name;
   using permission_name  = name;
   using table_name       = name;
```

In this C++ code, the `name` structure is also used to represent action names, table names, and so on. Note that unlike account names, when representing these names as strings, there can be up to 13 characters. However, for convenience, it is customary to use up to 12 characters to represent these names.

## Transaction Structure

The basic data structure on EOS is called a transaction (Transaction), and BPs are responsible for packaging transactions collected over a period of time into a block. Smart contract developers must fully understand the Transaction data structure.


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

Let's briefly explain the more important fields:

- `expiration`, sets the timeout for the transaction to be added to the chain; if the timeout is exceeded, the transaction will be rejected from being included in the block.
- `ref_block_num`, `ref_block_prefix` these two member variables are designed to prevent transactions from being re-included in blocks on forked chains.
- `actions`, this is an array structure of actions; the concept of action is very important. Each action corresponds to a smart contract function on the chain. When a BP includes a transaction in a block, the corresponding smart contract function is called based on the action. This will be explained in detail in the following section.
- `context_free_actions`, this is also an array of actions. The difference is that when the smart contract function corresponding to an action is called, the execution of the code is prohibited from calling APIs related to the on-chain database.

## Action Structure

The Action structure is contained within the Transaction structure. An action structure in C++ code is defined as follows:

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

In which, the definition of [permission_level](https://github.com/EOSIO/eos/blob/5082391c60b0fa5e68157c385cd402bf25aea934/libraries/chain/include/eosio/chain/action.hpp#L12) is as follows:

```C++
struct permission_level {
    account_name    actor;
    permission_name permission;
};
```
The meanings of the member variables in the structure are explained as follows:

- `account` is used to specify the account name of the smart contract to be called
- `name` is the name of the action being called
- `authorization` is an array of permissions
- `data` is the serialized raw data contained in the action; when called by the smart contract, it will be deserialized into a specific data structure

## ABI (Application Binary Interface)

When developing smart contracts, during the compilation process of the smart contract code, under normal circumstances, an ABI file (.abi) will be generated for each smart contract binary code (.wasm). However, it should be noted that this file is not required for calling smart contracts on the chain. Its purpose is to help developers obtain relevant action information and construct the corresponding Transaction data structure for easy interaction with the blockchain.

The content of an ABI file is in JSON format, like this:

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

- `version` is used to specify the ABI version
- `structs` is used to define data structures and will be used in both `actions` and `tables`
- `actions` describes the actions in the smart contract, with each action corresponding to a smart contract function
- `tables` describes table information, so your web application can query the on-chain database information using the `get_table_rows` RPC API
