---
comments: true
---

# HelloWorld

## The First Smart Contract

The following code shows the simplest smart contract and its testing code:

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

Testing code:

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


Compile:

```
python-contract build helloworld.codon
```


To run the test code:

```
ipyeos -m pytest -s -x testhelloworld.py -k test
```

Output:

```
Hello, World!
```

## Create an Initial Project

You can create an initial project using the `python-contract init` command. For example, the following code creates an initial project named `mycontract`:

```
python-contract init mycontract
```

After creating the project, you can compile the contract using the following command:

```
cd mycontract
./build.sh
```

After a successful execution, `mycontract.wasm` and `mycontract.abi` files will be generated.

You can run the following command for testing:

```
./test.sh
```

The following information will be output:

```
[(hello,sayhello)->hello]: CONSOLE OUTPUT BEGIN =====================
hello  alice

[(hello,sayhello)->hello]: CONSOLE OUTPUT END   =====================
```

After confirming that the test was successful, you can proceed with smart contract development based on this existing project.