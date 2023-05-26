---
comments: true
---

# HelloWorld

## First Smart Contract

Below is the code for the simplest smart contract and its test code.

[Complete Example](https://github.com/learnforpractice/rscdk-book/tree/master/examples/helloworld)

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(warnings))]

#[rust_chain::contract]
mod helloworld {
    use rust_chain::{
        Name,
        chain_println,
    };

    #[chain(main)]
    #[allow(dead_code)]
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

        #[chain(action = "sayhello")]
        pub fn say_hello(&self) {
            chain_println!("hello,world!");
        }
    }
}
```

Test code:

```python
import os
import sys
import json
import struct
import pytest

test_dir = os.path.dirname(__file__)
sys.path.append(os.path.join(test_dir, '..'))

from ipyeos import log
from ipyeos import eos
from ipyeos import chaintester
from ipyeos.chaintester import ChainTester

chaintester.chain_config['contracts_console'] = True

logger = log.get_logger(__name__)

def init_tester():
    tester = chaintester.ChainTester()
    return tester

def chain_test(fn):
    def call():
        tester = init_tester()
        ret = fn(tester)
        tester.free()
        return ret
    return call

class NewChainTester():
    def __init__(self):
        self.tester = None

    def __enter__(self):
        self.tester = init_tester()
        return self.tester

    def __exit__(self, type, value, traceback):
        self.tester.free()

test_dir = os.path.dirname(__file__)
def deploy_contract(tester, package_name):
    with open(f'{test_dir}/target/{package_name}.wasm', 'rb') as f:
        code = f.read()
    with open(f'{test_dir}/target/{package_name}.abi', 'rb') as f:
        abi = f.read()
    tester.deploy_contract('hello', code, abi)

@chain_test
def test_sayhello(tester):
    deploy_contract(tester, 'helloworld')
    ret = tester.push_action('hello', 'sayhello', "", {'hello': 'active'})
    tester.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

Compile:

```bash
cd examples/helloworld
rust-contract build
```

Run test code:

```
ipyeos -m pytest -s -x test.py -k test_sayhello
```

Output:

```
[(hello,sayhello)->hello]: CONSOLE OUTPUT BEGIN =====================
hello,world!

[(hello,sayhello)->hello]: CONSOLE OUTPUT END   =====================
```

## Creating an Initial Project

You can use the `rust-contract init` command to create an initial project. For example, the following command creates an initial project named `mycontract`:

```bash
rust-contract init mycontract
```

After creating the project, you can compile the contract using the following command:

```bash
cd mycontract
./build.sh
```

After a successful execution, the `target` directory will contain the `mycontract.wasm` and `mycontract.abi` files.

You can use the following command to run the tests:

```bash
./test.sh
```

The output will show the following text in green font:

```
debug 2023-05-26T01:43:59.121 thread-0  apply_context.cpp:40          print_debug          ] 
[(hello,inc)->hello]: CONSOLE OUTPUT BEGIN =====================
count is 1

[(hello,inc)->hello]: CONSOLE OUTPUT END   =====================
debug 2023-05-26T01:43:59.123 thread-0  controller.cpp:2499           clear_expired_input_ ] removed 0 expired transactions of the 50 input dedup list, pending block time 2018-06-01T12:00:03.500
debug 2023-05-26T01:43:59.125 thread-0  apply_context.cpp:40          print_debug          ] 
[(hello,inc)->hello]: CONSOLE OUTPUT BEGIN =====================
count is 2

[(hello,inc)->hello]: CONSOLE OUTPUT END   =====================
```

Please note that the above output is for debugging purposes. If running on the mainnet, the output from the `chain_println!` function will not be visible. To see the debug output in a testnet environment, you need to include the `--contracts-console` parameter when running the `nodeos` command.

In the test code provided, debug information is directly outputted with the following line of code:

```python
chaintester.chain_config['contracts_console'] = True
```

Furthermore, in the released version of the code, `chain_println!` statements should not be included to improve the performance of the program.
