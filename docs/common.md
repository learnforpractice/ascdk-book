---
comments: true
---

# Common Smart Contract Functions

## is_account

Declaration:

```rust
pub fn is_account(name: Name) -> bool
```

Description:

Used to determine whether an account exists.

## has_auth

Declaration:

```rust
pub fn has_auth(name: Name) -> bool
```

Description:

Used to determine whether it has the `active` authority of the specified account, that is, whether the Transaction has been signed with the private key corresponding to the `active` authority of the specified account. The private key used for signing may be at least one or more.

## require_auth/require_auth2

Declaration:

```rust
pub fn require_auth(name: Name)
pub fn require_auth2(account: Name, permission: Name)
```

Description:

These two functions will throw an exception when the account does not exist or the authority of the specified account is not detected. The difference is that `require_auth` checks for the existence of `active` authority, while `require_auth2` can check for specified authority.

## current_time

Declaration:

```rust
pub fn current_time() -> TimePoint
```

Description:

Used to get the time of the block where the Transaction is located.

## check

Declaration:

```rust
pub fn check(test: bool, msg: &str)
```

Description:

If test is false, it will throw an exception, all actions executed in the Transaction and the operations on the database that have been executed in this action will be rolled back, and the Transaction will not be on the chain. This is quite different from the `revert` mechanism in Ethereum. The result is that the EOS network is relatively fragile because the Transaction that throws an exception is rolled back and does not consume resources, that is, it does not cause costs, making the network easier to attack. However, in normal contracts, this function is also frequently used in smart contracts. For reference, see the related code in [token](https://github.com/uuosio/rscdk/blob/main/examples/token/lib.rs).

## Sample Code:

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(warnings))]

#[rust_chain::contract]
#[allow(dead_code)]
mod commonfunctions {
    use rust_chain::{
        Name,
        has_auth,
        require_auth,
        require_auth2,
        is_account,

        name,
        chain_println,
    };

    #[chain(main)]
    pub struct Contract {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl Contract {
        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action = "test")]
        pub fn test(&self) {
            has_auth(name!("hello"));

            require_auth(name!("hello"));
            require_auth2(name!("hello"), name!("active"));
    
            chain_println!(is_account(name!("hello")));
            chain_println!(is_account(name!("noexists")));
        }
    }
}
```

Compile:

```bash
cd examples/commonfunctions
rust-contract build
```

Test Code:

```python
@chain_test
def test_commonfunctions(tester):
    deploy_contract(tester, 'commonfunctions')
    args = {}
    r = tester.push_action('hello', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

Test:

```
ipyeos -m pytest -s -x test.py -k test_commonfunctions
```

Output:
```
[(hello,test)->hello]: CONSOLE OUTPUT BEGIN =====================
true
false

[(hello,test)->hello]: CONSOLE OUTPUT END   =====================
```
This output shows that the `is_account` function returned `true` when checking the existence of the "hello" account and `false` for the "noexists" account.

It is important to understand the use of these functions as they are frequently used in Rust smart contracts. They provide various checks and controls for contract interactions, ensuring the correctness of transactions and contract actions.

This tutorial provides a good overview of how to use these basic functions, but always remember that the use of such functions should be adapted to the specific requirements of your smart contract.

[Example Code](https://github.com/learnforpractice/rscdk-book/tree/master/examples/commonfunctions)
