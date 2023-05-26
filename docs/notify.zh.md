---
comments: true
---

# require_recipient函数

函数在`rust-chain`中的`vmapi/on_chain/eosio.rs`中的声明如下：

```rust
pub fn require_recipient(name: Name)
```

`require_recipient`函数用来通知其它合约本合约调用了某个action，这个action即是调用`require_recipient`所在的action. 如果被通知的合约有相同的action，那么这个action将被调用。

以下的`sender`, `receiver`的代码演示了如何从一个合约发送通知到另一个合约。

```rust
// sender
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(warnings))]

use rust_chain as chain;

#[chain::contract]
mod hello {
    use ::rust_chain::{
        Name,
        chain_println,
        require_recipient,
    };

    #[chain(packer)]
    pub struct SayHello {
        pub name: String
    }

    #[chain(main)]
    pub struct Hello {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl Hello {
        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action="test")]
        pub fn test(&self, name: String) {
            require_recipient(Name::new("hello"));
            chain_println!(self.receiver, self.first_receiver);
            chain_println!("++++++++in sender, name is:", name);
        }
    }
}
```

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(warnings))]

use rust_chain as chain;

#[chain::contract]
mod hello {
    use ::rust_chain::{
        Name,
        require_recipient,
        chain_println,
    };

    #[chain(packer)]
    pub struct SayHello {
        pub name: String
    }

    #[chain(main)]
    pub struct Hello {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl Hello {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action="test", notify)]
        pub fn test(&self, name: String) {
            chain_println!(self.receiver, self.first_receiver);
            chain_println!("++++++++int receiver, name:", name);
        }
    }
}
```

解释下代码：

- `sender`合约代码部署在`alice`这个合约账号中，其中的`test`action调用了`require_recipient`这个函数来通知`hello`这个账号自己调用了`test`这个action
- `receiver`合约代码部署在`hello`这个账号中，从其中如下所示的代码，`#[chain(action="test", notify)]`这行代码与正常的action代码不同，其中的`notify`参数表示这个action是用来接收其它合约通知的。用于接收通知的action，其`self.receiver`和`self.first_receiver`是不相同的，在本例中,`self.first_receiver`为账号`alice`，`self.receiver`为`hello`，可以在运行测试时查看`chain_println!(self.receiver, self.first_receiver);`的输出即可知道。

```rust
#[chain(action="test", notify)]
pub fn test(&self, name: String) {
    chain_println!(self.receiver, self.first_receiver);
    chain_println!("++++++++int receiver, name:", name);
}
```

以下是测试代码：

```python
@chain_test
def test_notify(tester):
    deploy_contracts(tester)
    args = {'name': 'alice'}
    r = tester.push_action('alice', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

编译：

```bash
cd examples/notify

pushd sender
rust-contract build
popd

pushd receiver
rust-contract build
popd
```

测试：

```bash
ipyeos -m pytest -s -x test.py -k test_notify
```

输出：

```
[(alice,test)->alice]: CONSOLE OUTPUT BEGIN =====================
alice alice
++++++++in sender, name is: alice

[(alice,test)->alice]: CONSOLE OUTPUT END   =====================
debug 2023-05-26T02:42:02.693 thread-0  apply_context.cpp:40          print_debug          ] 
[(alice,test)->hello]: CONSOLE OUTPUT BEGIN =====================
hello alice
++++++++int receiver, name: alice

[(alice,test)->hello]: CONSOLE OUTPUT END   =====================
```
