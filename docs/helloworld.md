---
comments: true
---

# HelloWorld

## The First Smart Contract

Below is a simple example of a smart contract code and its corresponding test code.

[Full Example](https://github.com/learnforpractice/rscdk-book/tree/master/examples/helloworld)

```ts
import {
    Name,
    Contract,
    print,
} from "asm-chain";

@contract
class MyContract extends Contract {
    constructor(receiver: Name, firstReceiver: Name, action: Name) {
        super(receiver, firstReceiver, action);
    }

    @action("sayhello")
    say_hello(): void {
        print("++++++++hello, world\n");
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
from ipyeos import chaintester
from ipyeos.chaintester import ChainTester

chaintester.chain_config['contracts_console'] = True

logger = log.get_logger(__name__)

def update_auth(tester, account):
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
    tester.push_action('eosio', 'updateauth', a, {account:'active'})

def init_tester():
    tester = chaintester.ChainTester()
    update_auth(tester, 'hello')
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
    with open(f'{test_dir}/assembly/target/{package_name}.wasm', 'rb') as f:
        code = f.read()
    with open(f'{test_dir}/assembly/target/{package_name}.abi', 'rb') as f:
        abi = f.read()
    tester.deploy_contract('hello', code, abi)

@chain_test
def test_hello(tester):
    deploy_contract(tester, 'helloworld')
    args = {}
    r = tester.push_action('hello', 'sayhello', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

Compile:

```bash
cd examples/helloworld
yarn
yarn build
```

Run test code:

```bash
ipyeos -m pytest -s -x test.py -k test_hello
```

Or run:

```bash
yarn pytest
```

Output:

```
[(hello,sayhello)->hello]: CONSOLE OUTPUT BEGIN =====================
hello,world!

[(hello,sayhello)->hello]: CONSOLE OUTPUT END   =====================
```

Note that the above output is debugging information. If running on the mainnet, the output of the `print` function is not visible. If running on the testnet, you need to add the `--contracts-console` parameter when running the nodeos command to see the debug output.

In the above test code, the debug information is outputted directly by this line of code:

```python
chaintester.chain_config['contracts_console'] = True
```

Additionally, in the released version of the code, to improve the performance of program execution, you should not include `print` related code.