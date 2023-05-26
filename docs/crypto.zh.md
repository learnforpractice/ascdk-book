---
comments: true
---

# 密码学相关函数

密码学相关的函数在`rust-chain`package中的`crypto.rs`中定义，可以通过像下面的方式导入：

```rust
use rust_chain::{
    sha256,
};
```

或者只导入crypto模块：

```rust
use rust_chain::crypto;
```

## sha256

sha256算法hash函数

```rust
pub fn sha256(data: &[u8]) -> Checksum256
```

用于检测hash256值是否正常，不正确会直接抛出异常

```rust
pub fn assert_sha256(data: &[u8], hash: &Checksum256)
```

## sha1

sha1算法hash函数

```rust
pub fn sha1( data: &[u8]) -> Checksum160
```

用于检测sha1 hash值是否正常，不正确会直接抛出异常

```rust
pub fn assert_sha1(data: &[u8], hash: &Checksum160)
```


## sha512

sha512算法hash函数

```rust
pub fn sha512( data: &[u8]) -> Checksum512
```

用于检测hash512值是否正常，不正确会直接抛出异常

```rust
pub fn assert_sha512(data: &[u8], hash: &Checksum512)
```

## ripemd160

ripemd160算法hash函数

```rust
pub fn ripemd160(data: &[u8]) -> Checksum160
```

用于检测ripemd160算法的hash值是否正常，不正确会直接抛出异常

```rust
pub fn assert_ripemd160(data: &[u8], hash: &Checksum160)
```

## recover_key

用于从digest和signture中恢复出公钥

```rust
pub fn recover_key( digest: &Checksum256 , sig: &Signature) -> PublicKey
```

检查签名是否正常，不正常会抛出异常

```rust
pub fn assert_recover_key(digest: &Checksum256, sig: &Signature, pubkey: &PublicKey)
```

## 示例：

[完整示例代码](https://github.com/learnforpractice/rscdk-book/tree/master/examples/cryptotest)

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[rust_chain::contract]
#[allow(dead_code)]
mod cryptotest {
    use rust_chain::{
        Name,

        PublicKey,
        Signature,

        sha256,
        assert_sha256,
        sha1,
        assert_sha1,
        sha512,
        assert_sha512,
        ripemd160,
        assert_ripemd160,

        recover_key,
        assert_recover_key,

        check,
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
            let data: Vec<u8> =  vec![1, 2, 3, 4, 5, 6, 7];
            let ret = sha256(&data);
            assert_sha256(&data, &ret);

            let ret = sha1(&data);
            assert_sha1(&data, &ret);

            let ret = sha512(&data);
            assert_sha512(&data, &ret);

            let ret = ripemd160(&data);
            assert_ripemd160(&data, &ret);
            chain_println!("done!");
        }

        #[chain(action="testrecover")]
        pub fn test_recover(&self, msg: Vec<u8>, sig: Signature, pub_key: PublicKey) {
            chain_println!("++++++msg:", msg);
            let digest = sha256(&msg);
            let _pubkey = recover_key(&digest, &sig);
            check(_pubkey == pub_key, "_pubkey == k1");
            assert_recover_key(&digest, &sig, &pub_key);
        }
    }
}
```

测试代码：

```python
@chain_test
def test_hash(tester):
    deploy_contract(tester, 'cryptotest')
    r = tester.push_action('hello', 'testhash', {}, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()

@chain_test
def test_recover_key(tester):
    deploy_contract(tester, 'cryptotest')
    key = eos.create_key()
    pub = key['public']
    priv = key['private']

    msg = b'hello, world'
    h = hashlib.sha256()
    h.update(msg)
    sig = eos.sign_digest(h.hexdigest(), priv)

    args = {
        'msg': msg.hex(),
        'sig': sig,
        'pub_key': pub,
    }
    r = tester.push_action('hello', 'testrecover', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

编译：

```bash
cd examples/cryptotest
rust-contract build
```

测试：

```
ipyeos -m pytest -s -x test.py -k test_hash
ipyeos -m pytest -s -x test.py -k test_recover_key
```

在这个示例代码中，分别演示了常用的hash函数的用法以及`recover_key`和`assert_recover_key`的用法。hash函数的用法比较简单，这里解释一下`recover_key`的测试代码：
`recover_key`接受二个参数，分别是`digest`和`signature`，digest是对一个二进制数据进行sha256运行的结果。在上面的代码中是通过对`hello, world`进行sha256算法的hash计算，如下代码所示：

```python
msg = b'hello, world'
h = hashlib.sha256()
h.update(msg)
sig = eos.sign_digest(h.hexdigest(), priv)
```

其中的`eos.sign_digest`用来对数据进行签名。

在智能合约里，对应的代码是：

```rust
let digest = sha256(&msg);
```

下面是对`testrecover`的解释：

```rust
#[chain(action="testrecover")]
pub fn test_recover(&self, msg: Vec<u8>, sig: Signature, pub_key: PublicKey) {
    chain_println!("++++++msg:", msg);
    let digest = sha256(&msg);
    let _pubkey = recover_key(&digest, &sig);
    check(_pubkey == pub_key, "_pubkey == k1"); //#判断public key是否正确
    assert_recover_key(&digest, &sig, &pub_key); //作用相当于上面两行代码
}
```

`recover_key`的原理和节点检验Transaction中的签名是否有效是一样的，就是通过对digest进行签名，然后再用公钥进行验证。
在实际的智能合约的应用中，如果要在智能合约里判断某段二进制数据是否是用特定的私钥进行的签名也可以用上面的办法。过程如下：

- 合约中保存用户一个私钥对应的公钥
- 用户用自己的私钥对数据进行签名
- 用户将数据，以及对应的签名传给智能合约
- 智能合约可以调用`RecoverKey`从用户数据，以及对数据的签名中还原出公钥
- 智能合约读取保存在链上的用户公钥，与通过调用`RecoverKey`还原出的公钥进行比较，相同即可以确定数据是对应的用户签的名
