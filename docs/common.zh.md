---
comments: true
---

# 常用智能合约函数

## isAccount

声明：

```ts
function isAccount(name: Name): bool
```

说明：

用来判断账号存不存在

## hasAuth

声明：

```ts
function hasAuth(name: Name): bool
```

说明：

用来判断是否有指定账号的`active`权限，也就是Transaction是否有用指定账号的`active`权限所对应的私钥进行签名。用于签名的私钥最少有一个，也可能二个以上。

## requireAuth/requireAuth2

声明：

```ts
function requireAuth(name: Name): void
function hasAuth(name: Name): bool
function requireAuth2(permissionLevel: PermissionLevel): void
```

说明：

这两个函数在账号不存在或者没有检测到有指定账号的权限时都会抛出异常，不同的是`requireAuth`为检测是否存在`active`权限，而`requireAuth2`可以检测指定的权限。其中，`requireAuth`函数在合约的开发中使用非常频繁，用于确保action的执行有某个账号的`active`权限。

## currentTime

声明：

```ts
function currentTime(): u64
```

说明:

用于获取Transaction所在的区块的时间，单位为微秒(1秒等于1000,000微秒)

## check

声明：

```ts
function check(test: bool, msg: string): void
```

说明：

如果test为false，则会抛出异常，所有在Transaction中的执行过的action以及本action已经执行的对数据库的操作都将被回滚，Transaction将不会上链。这和以太坊中的`revert`机制有比较大的区别。其结果是导致EOS网络相对比较脆弱，因为出异常的Transaction被回滚后不消耗资源，也就是不造成成本，导致网络比较容易被攻击。但是在正常的合约中，该函数在智能合约中使用也比较频繁，可参考[eosio.token.contract.ts](https://github.com/uuosio/ascdk/blob/master/examples/eosio.token/eosio.token.contract.ts)中相关的代码


## 示例代码：

```ts
import {
    Name,
    PermissionLevel,
    Contract,

    requireAuth,
    requireAuth2,
    hasAuth,
    isAccount,
    print,

    check
} from "asm-chain";

@contract
class MyContract extends Contract {
    constructor(receiver: Name, firstReceiver: Name, action: Name) {
        super(receiver, firstReceiver, action);
    }

    @action("test")
    test(): void {
        let ret = isAccount(Name.fromString("noexits"));
        print(`+++isAccount(noexits): ${ret}\n`);
        ret = isAccount(this.receiver);
        print(`+++isAccount(this.receiver): ${ret}\n`);

        print(`hasAuth: ${hasAuth(this.receiver)}`);
        requireAuth(this.receiver);
        requireAuth2(new PermissionLevel(this.receiver, Name.fromString("active")));
    }
}
```

编译：

```bash
cd examples/commonfunctions
yarn
yarn build
```

测试代码：

```python
@chain_test
def test_hello(tester):
    deploy_contract(tester, 'test')
    args = {}
    r = tester.push_action('hello', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

测试：

```bash
ipyeos -m pytest -s -x test.py -k test_hello
```

输出：

```
[(hello,test)->hello]: CONSOLE OUTPUT BEGIN =====================
+++isAccount(noexits): false
+++isAccount(this.receiver): true
hasAuth: true
current time: 1527854403000000

[(hello,test)->hello]: CONSOLE OUTPUT END   =====================
```

[完整示例代码](https://github.com/learnforpractice/ascdk-book/tree/master/examples/commonfunctions)
