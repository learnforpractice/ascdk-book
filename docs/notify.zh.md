---
comments: true
---

# requireRecipient 函数

函数在`asm-chain`中的`action.ts`中的声明如下：

```ts
function requireRecipient(name: Name): void
```

`requireRecipient`函数用来通知其它合约本合约调用了某个action，这个action即是调用`requireRecipient`所在的action. 如果被通知的合约有相同的action，那么这个action将被调用。

以下的`sender`, `receiver`的代码演示了如何从一个合约发送通知到另一个合约。

```ts
// sender
import {
    print,
    requireAuth,
    requireRecipient,

    Name,
    Contract,
} from "asm-chain";

@contract
class MyContract extends Contract {
    @action("sayhello")
    sayHello(name: Name): void {
        print(`hello ${name}!`);
        requireAuth(name);
        requireRecipient(Name.fromString('hello'));
    }
}
```

```ts
//receiver.ts
import {
    Name,
    Contract,

    print,
} from "asm-chain";

@contract
class MyContract extends Contract {
    @action("sayhello", notify)
    sayHello(name: Name): void {
        print(`notify: hello ${name}!`);
    }
}
```

解释下代码：

- `sender`合约代码部署在`alice`这个合约账号中，其中的`sayhello`action调用了`requireRecipient`这个函数来通知`hello`这个账号自己调用了`sayhello`这个action
- `receiver`合约代码部署在`hello`这个账号中，从其中如下所示的代码，`#[chain(action="sayhello", notify)]`这行代码与正常的action代码不同，其中的`notify`参数表示这个action是用来接收其它合约通知的。用于接收通知的action，其`self.receiver`和`self.first_receiver`是不相同的，在本例中,`self.first_receiver`为账号`alice`，`self.receiver`为`hello`，可以在运行测试时查看`chain_println!(self.receiver, self.first_receiver);`的输出即可知道。

```ts
@action("sayhello", notify)
sayHello(name: Name): void {
    print(`notify: hello ${name}!`);
}
```

以下是测试代码：

```python
@chain_test
def test_notify(tester):
    deploy_contracts(tester)
    args = {'name': 'alice'}
    r = tester.push_action('alice', 'sayhello', args, {'alice': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

编译：

```bash
cd examples/notify
yarn
yarn build
```

测试：

```bash
ipyeos -m pytest -s -x test.py -k test_notify
```

输出：

```
[(alice,sayhello)->alice]: CONSOLE OUTPUT BEGIN =====================
hello alice!
[(alice,sayhello)->alice]: CONSOLE OUTPUT END   =====================
[(alice,sayhello)->hello]: CONSOLE OUTPUT BEGIN =====================
notify: hello alice!
[(alice,sayhello)->hello]: CONSOLE OUTPUT END   =====================
```

[示例代码](https://github.com/learnforpractice/ascdk-book/tree/master/examples/notify)
