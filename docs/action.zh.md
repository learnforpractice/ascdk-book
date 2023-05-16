---
comments: true
---

# 内联Action在智能合约的使用

在智能合约中也可以发起一个action，这样的action称之为内联action(inline action)。需要注意的是，action是异步的，也就是说，只有在整个代码执行完后，内联action对应的合约代码才会被调用，如果被调用的合约没有定义相关的action或者账号中没有部属合约，那么调用将没有影响，但也不会有异常抛出。像这些空的内联action也不是没有任何作用，例如可以当作链上的日志，以供应用程序来查询。

以下是Action类在`action.codon`中的完整代码：

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

该类有三个`__init__`函数，请根据需求来使用，使用比较多的应该是下面这个初始化函数：

```python
def __init__(self, account: Name, name: Name, data: bytes=bytes())
```

这个函数默认使用和account的`active`权限

下面这个初始化函数指定了权限的账号，也是默认使用账号的`active`权限

```python
def __init__(self, account: Name, name: Name, permission_account: Name, data: bytes=bytes())
```

如果使用的是其它权限，则可以使用下面的个初始化函数：

```python
def __init__(self, account: Name, name: Name, permission_account: Name, permission_name: Name, data: bytes=bytes()):
```

如果账号用了多个权限，则用下个这个初始化函数。

```python
def __init__(self, account: Name, name: Name, authorization: List[PermissionLevel], data: bytes=bytes()):
```

示例：

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

    def __init__(self):
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
                                                                                                    
测试代码：

```python
def test_action():
    t = init_test('action_example')
    ret = t.push_action('hello', 'test', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s", ret['elapsed'])
```

编译：
```
python-contract build action_example.codon
```

运行测试：

```
ipyeos -m pytest -s -x test.py -k test_action
```

输出：

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

可以看到，这里先调用了`test`这个在Transaction里指定了的action，然后调用了`test2`这个Action，但是`test2`这个action并没有在Transaction里指定，而是在智能合约里发起的。另外，还通过`test3`演示了如何发送带多个参数的action.


需要注意的是，为了在合约中能够调用inline action，需要在账号的`active`权限中添加`eosio.code`这个虚拟权限,在测试代码中，通过下面的函数来将`eosio.code`这个虚拟权限添加到`active`权限中。

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