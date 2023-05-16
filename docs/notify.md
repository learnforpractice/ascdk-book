---
comments: true
---

# The `require_recipient` function

The function is declared in `action.codon` as follows:

```python
def require_recipient(account: Name):
```

The `require_recipient` function is utilized to notify other contracts. If the `account` contract has the same action, this action will be called.

The following code in `sender.codon` and `receiver.codon` demonstrates how to send a notification from one contract to another.

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

Note that the definition of the `sayhello` function in `receiver.codon` is slightly different from that in `sender.codon`. The `notify=True` in the `action` decorator of the `sayhello` function in `receiver.codon` is used to specify that this action is used to receive notifications and can only be triggered by calling `require_recipient`.

The following is the test code:

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

Compile:
```bash
python-contract build notify/receiver.codon
python-contract build notify/sender.codon
```

Test:

```bash
ipyeos -m pytest -s -x test.py -k test_notify
```

Output:

```
[(hello,sayhello)->hello]: CONSOLE OUTPUT BEGIN =====================
hello, world

[(hello,sayhello)->hello]: CONSOLE OUTPUT END   =====================
debug 2023-03-28T13:08:01.110 thread-0  apply_context.cpp:30          print_debug          ] 
[(hello,sayhello)->alice]: CONSOLE OUTPUT BEGIN =====================
hello, world from notify

[(hello,sayhello)->alice]: CONSOLE OUTPUT END   =====================
```