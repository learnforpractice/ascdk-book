---
comments: true
---

# Calling C/C++ Code in Codon

First, use the following command to install `eoscdt` for compiling C or C++ code:

```bash
python3 -m pip install -U eoscdt
```

Next, let's compile the `say_hello` function as an example to demonstrate how to compile code:

If the source file is in C, for example:

say_hello.c

```c
void prints(const char *s);

void say_hello(const char *s) {
	prints(s);
}
```

Then use the following command to compile:

```bash
cdt-cc -c -o say_hello.o say_hello.c
```


If the source file is C++ code, for example:

say_hello.cpp

```cpp
extern "C" void prints(const char *s);

extern "C" void say_hello(const char *s) {
	prints(s);
}
```

Then use the following command to compile:

```bash
cdt-cpp -c -o say_hello.o say_hello.cpp
```

Note that if it is a C++ file, you need to add `extern "C"` before the function, otherwise a link error will occur.

Next, let's see how to use the `say_hello` function in Codon:

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

Here,

```python
from C import say_hello(cobj);
```
tells the Codon compiler to link the say_hello C function. All pointer types in C/C++ correspond to the cobj type in Codon.

The following line of code calls the C function, the value returned by `c_str` is of `cobj` type, equivalent to `const char *` type in C/C++.

```python
say_hello("abc".c_str())
```

Next, use the following command to compile:

```bash
python-contract build --linker-flags="say_hello.o" test.codon
```

Here, `--linker-flags="say_hello.o"` tells the compiler to link the `say_hello.o` obj file.

Next, use the following code to test:

test.py:

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

Run the test:

```bash
ipyeos -m pytest -s -x test.py -k test_say_hello
```

You should see the following output:

```
hello, world
```

[Full Source Code](https://github.com/learnforpractice/pscdk-book/tree/main/examples/callc)
