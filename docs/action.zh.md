---
comments: true
---

# 内联Action在智能合约的使用

在智能合约中也可以发起一个action，这样的action称之为内联action(inline action)。需要注意的是，action是异步的，也就是说，只有在整个代码执行完后，内联action对应的合约代码才会被调用，如果被调用的合约没有定义相关的action或者账号中没有部属合约，那么调用将没有影响，但也不会有异常抛出。像这些空的内联action也不是没有任何作用，例如可以当作链上的日志，以供应用程序来查询。

下面通过利用inline action来进行EOS转账的列子来说明inline action的用法。

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

在上面的代码中，实现了从`hello`这个账号转账`1.0000 EOS`到`alice`这个账号的功能。`hello`和`alice`这两个账号都是在启动测试的时候创建好的，可以直接使用。

测试代码：

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

编译：

```bash
cd examples/inlineaction
rust-contract build
```

运行测试：

```bash
ipyeos -m pytest -s -x test.py -k test_inline_action
```

输出：

```
INFO     test:test.py:74 balance of hello before transfer: 50000000000
INFO     test:test.py:75 balance of alice before transfer: 50000000000
INFO     test:test.py:81 balance of hello after transfer: 49999990000
INFO     test:test.py:82 balance of alice after transfer: 50000010000
```

需要注意的是，为了在合约中能够调用inline action，需要在账号的`active`权限中添加`eosio.code`这个虚拟权限,在测试代码中，通过下面的函数来将`eosio.code`这个虚拟权限添加到`active`权限中。

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

总结：

在EOS中，除了可以通过在Transaction里包含action来调用合约的代码之外，在合约的代码里，也可以发起一个Action来调用合约的代码，这样的Action称之为Inline Action. 要允许合约代码使用Inline Action，还必须在合约账号的`active`权限中添加`eosio.code`这个虚拟权限。

[完整示例](https://github.com/learnforpractice/rscdk-book/tree/master/examples/inlineaction)
