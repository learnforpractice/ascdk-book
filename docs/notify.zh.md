---
comments: true
---

# require_recipient函数

函数在`action.codon`中的声明如下：

```python
def require_recipient(account: Name):
```

`require_recipient`函数用来通知其它合约. 如果account合约有相同的action，那么这个action将被调用。

以下的`sender.codon`, `receiver.codon`的代码演示了如何从一个合约发送通知到另一个合约。

```python
# sender.codon
from chain.name import Name
from chain.contract import Contract
from chain.action import require_recipient

@contract(main=True)
class MyContract(Contract):

    @action('sayhello')
    def sayhello(self, receiver: Name):
        print('hello, world')
        require_recipient(receiver)
```

```python
# receiver.codon
from chain.name import Name
from chain.contract import Contract

@contract(main=True)
class MyContract(Contract):

    @action('sayhello', notify=True)
    def sayhello(self, receiver: Name):
        assert not self.receiver == self.first_receiver
        assert receiver == self.receiver
        print('hello, world from notify')
```

这里，要注意的是，`receiver.codon`中的`sayhello`函数和`sender.codon`中的`sayhello`函数的定义有些不同，`receiver.codon`中的`sayhello`的`action`decorator中多了`notify=True`，这是用来指定这个action是一个用来接收通知的action，只能通过调用`require_recipient`来触发。

以下是测试代码：

```python
def init_notify():
    t = ChainTester(True)
    update_auth(t, 'hello')

    wasm_file = os.path.join(dir_name, 'notify/sender.wasm')
    with open(wasm_file, 'rb') as f:
        code = f.read()
    abi_file = os.path.join(dir_name, 'notify/sender.abi')
    with open(abi_file, 'r') as f:
        abi = f.read()
    t.deploy_contract('hello', code, abi)
    t.produce_block()

    wasm_file = os.path.join(dir_name, 'notify/receiver.wasm')
    with open(wasm_file, 'rb') as f:
        code = f.read()
    abi_file = os.path.join(dir_name, 'notify/receiver.abi')
    with open(abi_file, 'r') as f:
        abi = f.read()
    t.deploy_contract('alice', code, abi)
    t.produce_block()
    return t

def test_notify():
    t = init_notify()
    args = {'receiver': 'alice'}
    ret = t.push_action('hello', 'sayhello', args, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

编译：
```bash
python-contract build notify/receiver.codon
python-contract build notify/sender.codon
```

测试：

```bash
ipyeos -m pytest -s -x test.py -k test_notify
```

输出：

```
[(hello,sayhello)->hello]: CONSOLE OUTPUT BEGIN =====================
hello, world

[(hello,sayhello)->hello]: CONSOLE OUTPUT END   =====================
debug 2023-03-28T13:08:01.110 thread-0  apply_context.cpp:30          print_debug          ] 
[(hello,sayhello)->alice]: CONSOLE OUTPUT BEGIN =====================
hello, world from notify

[(hello,sayhello)->alice]: CONSOLE OUTPUT END   =====================
```
