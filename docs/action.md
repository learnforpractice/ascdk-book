---
comments: true
---

# Use of Inline Action in Smart Contracts

In smart contracts, one can also initiate an action, known as an inline action. It is important to note that actions are asynchronous, which means that the corresponding contract code of the inline action will be called only after the entire code has been executed. If the called contract does not define the corresponding action or there is no deployed contract in the account, the call will have no effect but no exception will be thrown either. These empty inline actions can be used as on-chain logs for querying by applications.

Here is the complete code of the Action class in `action.codon`:

```python
@packer
class Action(object):
    account: Name
    name: Name
    authorization: List[PermissionLevel]
    data: bytes

    def __init__(self, account: Name, name: Name, data: bytes=bytes()):
        self.account = account
        self.name = name
        self.authorization = [PermissionLevel(account, n'active')]
        self.data = data

    def __init__(self, account: Name, name: Name, permission_account: Name, data: bytes=bytes()):
        self.account = account
        self.name = name
        self.authorization = [PermissionLevel(permission_account, n'active')]
        self.data = data

    def __init__(self, account: Name, name: Name, permission_account: Name, permission_name: Name, data: bytes=bytes()):
        self.account = account
        self.name = name
        self.authorization = [PermissionLevel(permission_account, permission_name)]
        self.data = data

    def __init__(self, account: Name, name: Name, authorization: List[PermissionLevel], data: bytes=bytes()):
        self.account = account
        self.name = name
        self.authorization = authorization
        self.data = data

    def send(self):
        raw = pack(self)
        send_inline(raw.ptr, u32(raw.len))

    def send(self, data: T, T: type):
        self.data = pack(data)
        raw = pack(self)
        send_inline(raw.ptr, u32(raw.len))
```

The class has three `__init__` methods. Please use as per your requirements. The most commonly used should be the following initialization function:

```python
def __init__(self, account: Name, name: Name, data: bytes=bytes())
```

This function defaults to the `active` permission of the account.

The following initialization function specifies the account of the permission, and also defaults to the `active` permission of the account:

```python
def __init__(self, account: Name, name: Name, permission_account: Name, data: bytes=bytes())
```

If other permissions are used, the following initialization function can be used:

```python
def __init__(self, account: Name, name: Name, permission_account: Name, permission_name: Name, data: bytes=bytes()):
```

If multiple permissions are used by the account, then use this initialization function.

```python
def __init__(self, account: Name, name: Name, authorization: List[PermissionLevel], data: bytes=bytes()):
```

Example:

```python
# action_example.codon
from packer import pack
from chain.action import Action, PermissionLevel
from chain.contract import Contract

@packer
class Person:
    name: str
    height: u64
    def __init__(self, name: str, height: u64):
        self.name = name
        self.height = height

@contract(main=True)
class MyContract(Contract):

    def __init__(self)
        super().__init__()

    @action('test')
    def test(self):
        a = Action(n'hello', n'test2')
        print('++++send test2 action')
        a.send("1 alice")

        a = Action(n'hello', n'test2', n'hello')
        print('++++send test2 action')
        a.send("2 alice")

        a = Action(n'hello', n'test2', n'hello', n'active')
        print('++++send test2 action')
        a.send("3 alice")

        a = Action(n'hello', n'test2', [PermissionLevel(n"hello", n"active")])
        print('++++send test2 action')
        a.send("4 alice")

        a = Action(n'hello', n'test3')
        print('++++send test3 action')
        a.send(Person("alice", 175u64))
        return

    @action('test2')
    def test2(self, name: str):
        print('++++=name:', name)

    @action('test3')
    def test3(self, name: str, height: u64):
        print('++++=name:', name, 'height:', height)
```

Test code:

```python
def test_action():
    t = init_test('action_example')
    ret = t.push_action('hello', 'test', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s", ret['elapsed'])
```

Compile:

```
python-contract build action_example.codon
```

Run the test:

```
ipyeos -m pytest -s -x test.py -k test_action
```

Output:

```
debug 2023-03-28T12:35:48.175 thread-0  apply_context.cpp:30          print_debug          ] 
[(hello,test)->hello]: CONSOLE OUTPUT BEGIN =====================
++++send test2 action
++++send test2 action
++++send test2 action
++++send test2 action
++++send test3 action

[(hello,test)->hello]: CONSOLE OUTPUT END   =====================
debug 2023-03-28T12:35:48.175 thread-0  apply_context.cpp:30          print_debug          ] 
[(hello,test2)->hello]: CONSOLE OUTPUT BEGIN =====================
++++=name: 1 alice

[(hello,test2)->hello]: CONSOLE OUTPUT END   =====================
debug 2023-03-28T12:35:48.175 thread-0  apply_context.cpp:30          print_debug          ] 
[(hello,test2)->hello]: CONSOLE OUTPUT BEGIN =====================
++++=name: 2 alice

[(hello,test2)->hello]: CONSOLE OUTPUT END   =====================
debug 2023-03-28T12:35:48.175 thread-0  apply_context.cpp:30          print_debug          ] 
[(hello,test2)->hello]: CONSOLE OUTPUT BEGIN =====================
++++=name: 3 alice

[(hello,test2)->hello]: CONSOLE OUTPUT END   =====================
debug 2023-03-28T12:35:48.175 thread-0  apply_context.cpp:30          print_debug          ] 
[(hello,test2)->hello]: CONSOLE OUTPUT BEGIN =====================
++++=name: 4 alice

[(hello,test2)->hello]: CONSOLE OUTPUT END   =====================
debug 2023-03-28T12:35:48.175 thread-0  apply_context.cpp:30          print_debug          ] 
[(hello,test3)->hello]: CONSOLE OUTPUT BEGIN =====================
++++=name: alice height: 175

[(hello,test3)->hello]: CONSOLE OUTPUT END   =====================
debug 2023-03-28T12:35:48.177 thread-0  controller.cpp:2444           clear_expired_input_ ] removed 0 expired transactions of the 50 input dedup list, pending block time 2018-06-01T12:00:04.000
```

As you can see, it first calls the `test` action specified in the Transaction, and then calls the `test2` Action. However, the `test2` action is not specified in the Transaction, but is initiated in the smart contract. In addition, it demonstrates how to send an action with multiple parameters through `test3`.

It should be noted that in order to call inline action in the contract, you need to add the `eosio.code` virtual permission to the `active` permission of the account. In the test code, the `eosio.code` virtual permission is added to the `active` permission through the following function.

```python
def update_auth(chain, account):
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
    chain.push_action('eosio', 'updateauth', a, {account:'active'})
```
