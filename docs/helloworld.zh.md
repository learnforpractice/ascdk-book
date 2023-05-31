---
comments: true
---

# HelloWorld

## 第一个智能合约

以下展示了一个最简单的智能合约代码和测试代码

[完整示例](https://github.com/learnforpractice/rscdk-book/tree/master/examples/helloworld)

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

测试代码：

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

编译：

```bash
cd examples/helloworld
yarn
yarn build
```

运行测试代码：

```bash
ipyeos -m pytest -s -x test.py -k test_hello
```

或者运行:
```bash
yarn pytest
```

输出：

```
[(hello,sayhello)->hello]: CONSOLE OUTPUT BEGIN =====================
hello,world!

[(hello,sayhello)->hello]: CONSOLE OUTPUT END   =====================
```

需要注意的是上面的输出是调试信息，如果是在主网上运行,`print`函数输出的内容是看不到的，如果是运行在测试网，则在运行nodeos命令的时候要加上参数`--contracts-console`才能在输出中看调试信息。

在上面测试代码中，则是直接通过下面的这行代码来输出调试信息：

```python
chaintester.chain_config['contracts_console'] = True
```

另外，在发布版本的代码中，为了提高程序运行的性能，也不应该包含`print`相关的代码。
