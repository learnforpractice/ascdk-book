---
comments: true
---

# 常用智能合约函数

## is_account

声明：

```rust
pub fn is_account(name: Name) -> bool
```

说明：

用来判断账号存不存在

## has_auth

声明：

```rust
pub fn has_auth(name: Name) -> bool
```

说明：

用来判断是否有指定账号的`active`权限，也就是Transaction是否有用指定账号的`active`权限所对应的私钥进行签名。用于签名的私钥最少有一个，也可能二个以上。

## require_auth/require_auth2

声明：

```rust
pub fn require_auth(name: Name)
pub fn require_auth2(account: Name, permission: Name)
```

说明：

这两个函数在账号不存在或者没有检测到有指定账号的权限时都会抛出异常，不同的是`require_auth`为检测是否存在`active`权限，而`require_auth2`可以检测指定的权限。

## current_time

声明：

```rust
pub fn current_time() -> TimePoint
```

说明:

用于获取Transaction所在的区块的时间

## check

声明：

```rust
pub fn check(test: bool, msg: &str)
```

说明：

如果test为false，则会抛出异常，所有在Transaction中的执行过的action以及本action已经执行的对数据库的操作都将被回滚，Transaction将不会上链。这和以太坊中的`revert`机制有比较大的区别。其结果是导致EOS网络相对比较脆弱，因为出异常的Transaction被回滚后不消耗资源，也就是不造成成本，导致网络比较容易被攻击。但是在正常的合约中，该函数在智能合约中使用也比较频繁，可参考[token](https://github.com/uuosio/rscdk/blob/main/examples/token/lib.rs)中相关的代码


## 示例代码：

```python
from chain.action import has_auth, require_auth, require_auth2, is_account
from chain.contract import Contract

@contract(main=True)
class MyContract(Contract):

    def __init__(self):
        super().__init__()

    @action('test')
    def test(self):
        has_auth(n"hello")

        require_auth(n"hello")
        require_auth2(n"hello", n"active")

        print(is_account(n"hello"))
        print(is_account(n"hello"))
        return
```

编译：
```
python-contract build common_example.codon
```

测试代码：

```python
def test_common():
    t = init_test('common_example')
    ret = t.push_action('hello', 'test', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

测试：

```
ipyeos -m pytest -s -x test.py -k test_common
```

输出：
```
[(hello,test)->hello]: CONSOLE OUTPUT BEGIN =====================
True
True

[(hello,test)->hello]: CONSOLE OUTPUT END   =====================
```
