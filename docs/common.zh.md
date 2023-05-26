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

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(warnings))]

#[rust_chain::contract]
#[allow(dead_code)]
mod commonfunctions {
    use rust_chain::{
        Name,
        has_auth,
        require_auth,
        require_auth2,
        is_account,

        name,
        chain_println,
    };

    #[chain(main)]
    pub struct Contract {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl Contract {
        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action = "test")]
        pub fn test(&self) {
            has_auth(name!("hello"));

            require_auth(name!("hello"));
            require_auth2(name!("hello"), name!("active"));
    
            chain_println!(is_account(name!("hello")));
            chain_println!(is_account(name!("noexists")));
        }
    }
}
```

编译：

```bash
cd examples/commonfunctions
rust-contract build
```

测试代码：

```python
@chain_test
def test_commonfunctions(tester):
    deploy_contract(tester, 'commonfunctions')
    args = {}
    r = tester.push_action('hello', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

测试：

```
ipyeos -m pytest -s -x test.py -k test_commonfunctions
```

输出：
```
[(hello,test)->hello]: CONSOLE OUTPUT BEGIN =====================
true
false

[(hello,test)->hello]: CONSOLE OUTPUT END   =====================
```
