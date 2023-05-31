---
comments: true
---

# 密码学相关函数

密码学相关的函数在`asm-chain`package中的`crypto.ts`中定义，可以通过像下面的方式导入：

```ts
import {
    ripemd160,
    sha1,
    sha256,
    sha512,

    assertRipemd160,
    assertSha1,
    assertSha256,
    assertSha512,

    Signature,
    PublicKey,

    recoverKey,
    assertRecoverKey,
} from "asm-chain";
```

## sha256

sha256算法hash函数

```ts
function sha256(data: u8[]): Checksum256
```

用于检测hash256值是否正常，不正确会直接抛出异常

```ts
function assertSha256(data: u8[], hash: Checksum256): void
```

## sha1

sha1算法hash函数

```ts
function sha1(data: u8[]): Checksum160
```

用于检测sha1 hash值是否正常，不正确会直接抛出异常

```ts
function assertSha1(data: u8[], hash: Checksum160): void
```


## sha512

sha512算法hash函数

```ts
function sha512(data: u8[]): Checksum512
```

用于检测hash512值是否正常，不正确会直接抛出异常

```ts
function assertSha512(data: u8[], hash: Checksum512): void
```

## ripemd160

ripemd160算法hash函数

```ts
function ripemd160(data: u8[]): Checksum160
```

用于检测ripemd160算法的hash值是否正常，不正确会直接抛出异常

```ts
function assertRipemd160(data: u8[], hash: Checksum160): void
```

## recoverKey

用于从digest和signture中恢复出公钥

```ts
function recoverKey(digest: Checksum256, sig: Signature): PublicKey
```

检查签名是否正常，不正常会抛出异常

```ts
function assertRecoverKey(digest: Checksum256, sig: Signature, pub: PublicKey): void
```

## 示例：

[完整示例代码](https://github.com/learnforpractice/ascdk-book/tree/master/examples/cryptotest)

```ts
import {
    Name,
    Contract,

    ripemd160,
    sha1,
    sha256,
    sha512,

    assertRipemd160,
    assertSha1,
    assertSha256,
    assertSha512,

    Signature,
    PublicKey,
    recoverKey,
    assertRecoverKey,

    print,
    check,
} from "asm-chain";

@contract
class MyContract extends Contract {
    constructor(receiver: Name, firstReceiver: Name, action: Name) {
        super(receiver, firstReceiver, action);
    }

    @action("testhash")
    testhash(data: u8[]): void {
        print(`+++++++${data}\n`)
        assertRipemd160(data, ripemd160(data));
        assertSha1(data, sha1(data));
        assertSha256(data, sha256(data));
        assertSha512(data, sha512(data));
        print("+++++done!");
    }

    @action("testrecover")
    test_recover(msg: u8[], sig: Signature, pub_key: PublicKey): void {
        let digest = sha256(msg);
        let _pub_key = recoverKey(digest, sig);
        check(pub_key == _pub_key, "invalid public key");

        assertRecoverKey(digest, sig, pub_key);
        print("++++++++test_recover done!");
    }
}
```

测试代码：

```python
@chain_test
def test_hash(tester):
    deploy_contract(tester, 'test')
    r = tester.push_action('hello', 'testhash', {}, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()

@chain_test
def test_recover_key(tester):
    deploy_contract(tester, 'test')
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
yarn
yarn build
```

测试：

```
ipyeos -m pytest -s -x test.py -k test_hash
ipyeos -m pytest -s -x test.py -k test_recover_key
```

在这个示例代码中，分别演示了常用的hash函数的用法以及`recoverKey`和`assertRecoverKey`的用法。hash函数的用法比较简单，这里解释一下`recoverKey`的测试代码：
`recoverKey`接受二个参数，分别是`digest`和`signature`，digest是对一个二进制数据进行sha256运行的结果。在上面的代码中是通过对`hello, world`进行sha256算法的hash计算，如下代码所示：

```python
msg = b'hello, world'
h = hashlib.sha256()
h.update(msg)
sig = eos.sign_digest(h.hexdigest(), priv)
```

其中的`eos.sign_digest`用来对数据进行签名。


下面是对`testrecover`的解释：

```ts
@action("testrecover")
test_recover(msg: u8[], sig: Signature, pub_key: PublicKey): void {
    let digest = sha256(msg);
    let _pub_key = recoverKey(digest, sig);
    check(pub_key == _pub_key, "invalid public key");

    assertRecoverKey(digest, sig, pub_key);
    print("++++++++test_recover done!");
}
```

`recoverKey`的原理和节点检验Transaction中的签名是否有效是一样的，就是通过对digest进行签名，然后再用公钥进行验证。
在实际的智能合约的应用中，如果要在智能合约里判断某段二进制数据是否是用特定的私钥进行的签名也可以用上面的办法。过程如下：

- 合约中保存用户一个私钥对应的公钥
- 用户用自己的私钥对数据进行签名
- 用户将数据，以及对应的签名传给智能合约
- 智能合约可以调用`RecoverKey`从用户数据，以及对数据的签名中还原出公钥
- 智能合约读取保存在链上的用户公钥，与通过调用`RecoverKey`还原出的公钥进行比较，相同即可以确定数据是对应的用户签的名
