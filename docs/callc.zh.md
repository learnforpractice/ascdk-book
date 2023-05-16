---
comments: true
---

# Codon代码里调用C/C++代码

首先，用下面的命令安装`eoscdt`用于编译c或者是c++代码:

```bash
python3 -m pip install -U eoscdt
```

下面以编译`say_hello`函数为例，演示如何编译代码：

如果源文件是c代码，例如：

say_hello.c

```c
void prints(const char *s);

void say_hello(const char *s) {
	prints(s);
}
```

则用下面的命令编译：

```bash
cdt-cc -c -o say_hello.o say_hello.c
```


如果源文件是c++代码，例如：

say_hello.cpp

```cpp
extern "C" void prints(const char *s);

extern "C" void say_hello(const char *s) {
	prints(s);
}
```

则用下面的命令编译：

```bash
cdt-cpp -c -o say_hello.o say_hello.cpp
```

注意，如果是C++文件，则需在函数前面加上`extern "C"`，否则会在链接时出错。

接下来看下如何在codon中使用`say_hello`这个函数：

test.codon

```python
from chain.contract import Contract

from C import say_hello(cobj);

@contract(main=True)
class MyContract(Contract):

    @action("sayhello")
    def say_hello(self):
        say_hello("hello, world".c_str())
```

这里的

```python
from C import say_hello(cobj);
```

即告诉codon编译器要链接`say_hello`这个c函数。所有的C/C++里的指针类型都对应Codon里的`cobj`类型

下面的这行代码即是调用c函数，c_str返回的值是`cobj`类型, 相当于C/C++里的`const char *`类型

```python
say_hello("abc".c_str())
```

接下来用下面的命令来编译：

```bash
python-contract build --linker-flags="say_hello.o" test.codon
```

这里的`--linker-flags="say_hello.o`即是告诉编译器要链接`say_hello.o`这个obj文件

接下来用下面的代码来测试：

test.py

```python
import os
from ipyeos import eos
from ipyeos import chaintester
from ipyeos.chaintester import ChainTester
from ipyeos import log
from pyeoskit import eosapi

chaintester.chain_config['contracts_console'] = True
eos.set_log_level("default", 3)

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
    eos.set_log_level("default", 1)
    return t

def test_say_hello():
    t = init_test('test')
    ret = t.push_action('hello', 'sayhello', "", {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

运行测试：

```bash
ipyeos -m pytest -s -x test.py -k test_say_hello
```

会有下面的输出：

```
hello, world
```

[完整代码链接](https://github.com/learnforpractice/pscdk-book/tree/main/examples/callc)
