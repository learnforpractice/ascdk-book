---
comments: true
---

# HelloWorld

## 第一个智能合约

以下展示了一个最简单的智能合约代码和测试代码

```python
# helloworld.codon

from chain.contract import Contract

@contract(main=True)
class MyContract(Contract):

    def __init__(self):
        super().__init__()

    @action('sayhello')
    def say_hello(self):
        print("Hello, World!")
```

测试代码：

```python
# helloworldtest.py

import os
from ipyeos import chaintester
from ipyeos.chaintester import ChainTester
from ipyeos import log

chaintester.chain_config['contracts_console'] = True

logger = log.get_logger(__name__)

dir_name = os.path.dirname(os.path.abspath(__file__))

def init_test(contract_name):
    t = ChainTester(True)
    wasm_file = os.path.join(dir_name, f'{contract_name}.wasm')
    with open(wasm_file, 'rb') as f:
        code = f.read()

    abi_file = os.path.join(dir_name, f'{contract_name}.abi')
    with open(abi_file, 'r') as f:
        abi = f.read()

    t.deploy_contract('hello', code, abi)
    t.produce_block()
    return t

def test():
    t = init_test('helloworld')
    ret = t.push_action('hello', 'sayhello', "", {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```


编译：

```
python-contract build helloworld.codon
```


运行测试代码：
```
ipyeos -m pytest -s -x testhelloworld.py -k test
```

输出：

```
Hello, World!
```

## 创建一个初始项目

可以用`python-contract init`命令来创建一个初始项目，例如下面的代码创建了一个`mycontract`的初始项目：

```
python-contract init mycontract
```

创建完后可以用下面的命令编译合约：

```
cd mycontract
./build.sh
```

执行成功后会生成`mycontract.wasm`和`mycontract.abi`这两个文件

可以运行下面的命令进行测试：

```
./test.sh
```

会以绿色字体输出以下的的文字信息：

```
[(hello,sayhello)->hello]: CONSOLE OUTPUT BEGIN =====================
hello  alice

[(hello,sayhello)->hello]: CONSOLE OUTPUT END   =====================
```

需要注意的是上面的输出是调用信息，如果是在主网上运行,`print`函数输出的内容是看不到的，如果是运行在测试网，则在运行nodeos命令的时候要加上参数`--contracts-console`才能在返回中看调试输出。

在上面测试代码中，则是直接通过下面的这行代码来输出调试信息：

```python
chaintester.chain_config['contracts_console'] = True
```

另外，在发布版本的代码中，为了提高程序运行的性能，也不应该包含print代码。
