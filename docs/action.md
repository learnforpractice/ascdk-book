---
comments: true
---

# Use of Inline Actions in Smart Contracts

In smart contracts, an action can also be initiated, which is known as an inline action. It should be noted that actions are asynchronous, that is, the contract code corresponding to the inline action will only be called after the entire code is executed. If the called contract does not define the relevant action or there is no deployed contract in the account, the call will have no effect, but no exception will be thrown. Such empty inline actions are not without any effect, for example, they can be used as on-chain logs for application queries.

The following example illustrates the use of inline actions through EOS transfers.

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[rust_chain::contract]
mod inlineaction {
    use rust_chain::{
        Name,
        Action,
        PermissionLevel,    
        name,
        ACTIVE,
        chain_println,
        serializer::Packer,
        Asset,
        Symbol,
    };

    #[chain(packer)]
    struct Transfer {
        from: Name,
        to: Name,
        quantity: Asset,
        memo: String
    }

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

        #[chain(action = "testaction")]
        pub fn test_action(&self) {
            let transfer = Transfer{
                from: name!("hello"),
                to: name!("alice"),
                quantity: Asset::new(10000, Symbol::new("EOS", 4)),
                memo: String::from("hello, world")
            };
            let perm = PermissionLevel::new(name!("hello"), name!("active"));
            let action = Action::new(name!("eosio.token"), name!("transfer"), perm, &transfer);
            action.send();
        }
    }
}
```

In the above code, it implements the function of transferring `1.0000 EOS` from the account `hello` to the account `alice`. The accounts `hello` and `alice` are both created when starting the test and can be used directly.

Test code:

```python
@chain_test
def test_inline_action(tester: ChainTester):
    deploy_contract(tester, 'inlineaction')
    args = {}
    r = tester.push_action('hello', 'testaction', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
    logger.info("+++++++%s", tester.get_balance('hello'))
```

Compilation:

```bash
cd examples/inlineaction
rust-contract build
```

Run test:

```bash
ipyeos -m pytest -s -x test.py -k test_inline_action
```

Output:

```
INFO     test:test.py:74 balance of hello before transfer: 50000000000
INFO     test:test.py:75 balance of alice before transfer: 50000000000
INFO     test:test.py:81 balance of hello after transfer: 49999990000
INFO     test:test.py:82 balance of alice after transfer: 50000010000
```

Note that in order to be able to call inline actions in the contract, you need to add the virtual permission `eosio.code` to the account's `active` permission. In the test code, the virtual permission `eosio.code` is added to the `active` permission through the following function.

```python
def update_auth(chain, account):
    a = {
        "account": account,
        "permission": "active",
        "parent": "owner",
        "auth": {
            "threshold": 1,
            "keys": [
                {
                    "key": 'EOS6AjF6hvF7GSuSd4sCgfPKq5uWaXvGM2aQtEUCwmEHygQaqxBSV',
                    "weight": 1
                }
            ],
            "accounts": [{"permission":{"actor":account,"permission": 'eosio.code'}, "weight":1}],
            "waits": []
        }
    }
    chain.push_action('eosio', 'updateauth', a, {account:'active'})
```

In conclusion:

In EOS, besides being able to call contract code by including actions in a Transaction, contract code can also initiate an Action to call contract code. Such an Action is known as an Inline Action. To allow contract code to use Inline Actions, you must add the virtual permission `eosio.code` to the `active` permission of the contract account.

[Complete example](https://github.com/learnforpractice/rscdk-book/tree/master/examples/inlineaction)
