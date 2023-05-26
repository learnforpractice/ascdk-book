# require_recipient Function

The `require_recipient` function is declared in `vmapi/on_chain/eosio.rs` in the `rust-chain` package as follows:

```rust
pub fn require_recipient(name: Name)
```

The `require_recipient` function is used to notify other contracts that the current contract has called a specific action. If the notified contract has the same action, that action will be executed.

The following code snippets demonstrate how to send a notification from one contract to another:

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
            chain_println!("++++++++in receiver, name:", name);
        }
    }
}
```

The receiver contract is deployed in the `hello` account. In the code snippet below, the `#[chain(action="test", notify)]` line differs from a normal action code. The `notify` parameter indicates that this action is intended to receive notifications from other contracts. The `self.receiver` and `self.first_receiver` values are different for the action that receives notifications. In this example, `self.first_receiver` corresponds to the `alice` account, and `self.receiver` corresponds to `hello`. You can check the output of `chain_println!(self.receiver, self.first_receiver);` during the test execution to verify this.

```rust
#[chain(action="test", notify)]
pub fn test(&self, name: String) {
    chain_println!(self.receiver, self.first_receiver);
    chain_println!("++++++++in receiver, name:", name);
}
```

The test code is as follows:

```python
@chain_test
def test_notify(tester):
    deploy_contracts(tester)
    args = {'name': 'alice'}
    r = tester.push_action('alice', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

Compilation:

```bash
cd examples/notify

pushd sender
rust-contract build
popd

pushd receiver
rust-contract build
popd
```

Testing:

```bash
ipyeos -m pytest -s -x test.py -k test_notify
```

Output:

```
[(alice,test)->alice]: CONSOLE OUTPUT BEGIN =====================
alice alice
++++++++in sender, name is: alice

[(alice,test)->alice]: CONSOLE OUTPUT END   =====================
debug 2023-05-26T02:42:02.693 thread-0  apply_context.cpp:40          print_debug          ] 
[(alice,test)->hello]: CONSOLE OUTPUT BEGIN =====================
hello alice
++++++++in receiver, name: alice

[(alice,test)->hello]: CONSOLE OUTPUT END   =====================
```

In the output, you can see that the `test` action is called in the `alice` contract. The `require_recipient` function notifies the `hello` contract, and the `test` action in the `hello` contract is executed. The console output shows the values of `self.receiver` and `self.first_receiver`, as well as the name received as a parameter.

[Example Code](https://github.com/learnforpractice/rscdk-book/tree/master/examples/notify)
