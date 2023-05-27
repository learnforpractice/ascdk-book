---
comments: true
---

# HelloWorld

## 第一个智能合约

以下展示了一个最简单的智能合约代码和测试代码

[完整示例](https://github.com/learnforpractice/rscdk-book/tree/master/examples/helloworld)

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

编译：

```bash
cd examples/helloworld
rust-contract build
```

运行测试代码：

```
ipyeos -m pytest -s -x test.py -k test_sayhello
```

输出：

```
[(hello,sayhello)->hello]: CONSOLE OUTPUT BEGIN =====================
hello,world!

[(hello,sayhello)->hello]: CONSOLE OUTPUT END   =====================
```

## 创建一个初始项目

可以用`rust-contract init`命令来创建一个初始项目，例如下面的代码创建了一个`mycontract`的初始项目：

```bash
rust-contract init mycontract
```

创建完后可以用下面的命令编译合约：

```bash
cd mycontract
./build.sh
```

执行成功后会在`target`目录生成`mycontract.wasm`和`mycontract.abi`这两个文件

可以运行下面的命令进行测试：

```bash
./test.sh
```

会以绿色字体输出以下的的文字信息：

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

需要注意的是上面的输出是调试信息，如果是在主网上运行,`chain_println!`函数输出的内容是看不到的，如果是运行在测试网，则在运行nodeos命令的时候要加上参数`--contracts-console`才能在返回中看调试输出。

在上面测试代码中，则是直接通过下面的这行代码来输出调试信息：

```python
chaintester.chain_config['contracts_console'] = True
```

另外，在发布版本的代码中，为了提高程序运行的性能，也不应该包含`chain_println!`代码。
