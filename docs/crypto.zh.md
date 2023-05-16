---
comments: true
---

# 密码学相关函数

密码学相关的函数在`crypto.codon`中定义，可以通过像下面的方式导入：

```python
from chain.crypto import sha256
```

或者只导入crypto模块：

```python
from chain import crypto
```

然后通过像`crypto.sha256`的方式来调用内部函数。

## sha256

sha256算法hash函数

```python
def sha256(data: bytes) -> Checksum256:
```

用于检测hash256值是否正常，不正确会直接抛出异常

```python
def assert_sha256(data: bytes, hash: Checksum256):
```

## sha1

sha1算法hash函数

```python
def sha1(data: bytes) -> Checksum160:
```

用于检测sha1 hash值是否正常，不正确会直接抛出异常

```python
def assert_sha1(data: bytes, hash: Checksum160):
```


## sha512

sha512算法hash函数

```python
def sha512(data: bytes) -> Checksum512:
```

用于检测hash512值是否正常，不正确会直接抛出异常

```python
def assert_sha512(data: bytes, hash: Checksum512):
```

## ripemd160

ripemd160算法hash函数

```python
def ripemd160(data: bytes) -> Checksum160:
```

用于检测ripemd160算法的hash值是否正常，不正确会直接抛出异常

```python
def assert_ripemd160(data: bytes, hash: Checksum160):
```

## recover_key

用于从digest和signture中恢复出公钥

```python
def recover_key(digest: Checksum256, sig: Signature) -> PublicKey:
```

检查签名是否正常，不正常会抛出异常

```python
def assert_recover_key(digest: Checksum256, sig: Signature, pub: PublicKey):
```

## 示例：

```python
# crypto_example.codon
from chain.contract import Contract
from chain.crypto import sha256, assert_sha256, sha512, assert_sha512, sha1, assert_sha1, ripemd160, assert_ripemd160
from chain.crypto import recover_key, assert_recover_key
from chain.crypto import Signature, Checksum256, PublicKey

@contract(main=True)
class MyContract(Contract):

    def __init__(self):
        super().__init__()

    @action('testcrypto')
    def test_crypto(self):
        assert_sha256(b"hello", sha256(b"hello"))
        assert_sha1(b"hello", sha1(b"hello"))
        assert_sha512(b"hello", sha512(b"hello"))
        assert_ripemd160(b"hello", ripemd160(b"hello"))

    @action('testrecover')
    def test_recover(self, msg: bytes, digest: Checksum256, sig: Signature, k1: PublicKey):
        _digest = sha256(msg)
        assert _digest == digest
        _pubkey = recover_key(digest, sig)
        assert _pubkey == k1, "_pubkey == k1"
        assert_recover_key(digest, sig, k1)
        print('done!')
```

测试代码：

```python
def test_crypto():
    t = init_test('crypto_example')
    args = {}
    ret = t.push_action('hello', 'testcrypto', args, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])

def test_recover():
    t = init_test('crypto_example')

    msg = b'hello,world'
    # key pair
    public_key = 'EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV'
    private_key = '5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3'

    h = hashlib.sha256()
    h.update(msg)
    digest = h.hexdigest()
    logger.info('++++digest: %s', digest)

    #sign with private key
    sig = eosapi.sign_digest(digest, private_key)
    logger.info('++++signature: %s', sig)
    args = {
        "msg": msg.hex(),
        "digest": digest,
        "sig": sig,
        "k1": public_key,
    }
    ret = t.push_action('hello', 'testrecover', args, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

编译：

```
python-contract build crypto_example.codon
```

测试：

```
ipyeos -m pytest -s -x test.py -k test_crypto
ipyeos -m pytest -s -x test.py -k test_recover
```

在这个示例代码中，分别演示了常用的hash函数的用法以及`recover_key`和`assert_recover_key`的用法。hash函数的用法比较简单，这里解释一下recover_key的测试代码：
recover_key接受二个参数，分别是`digest`和`signature`，digest是对一个二进制数据进行sha256运行的结果。在上面的代码中是通过对`hello,world`进行sha256算法的hash计算。

```python
h = hashlib.sha256()
h.update(b'hello,world')
digest = h.hexdigest()
```

运算出的结果作为参数传给action.

下面是对`testrecover`的解释：

```python
@action('testrecover')
def test_recover(self, msg: bytes, digest: Checksum256, sig: Signature, k1: PublicKey):
    _digest = sha256(msg)
    assert _digest == digest #判断digest是否对msg进行hash256算法的hash结果
    _pubkey = recover_key(digest, sig)
    assert _pubkey == k1, "_pubkey == k1" #判断public key是否正确

    assert_recover_key(digest, sig, k1) #作用相当于上面两行代码
    print('done!')
```

在发送的Transaction中也是需要包含用户对Transaction的签名的，以表示用户授权了这个Transaction。然后在智能合约，就可以调用的`require_auth`函数来判断Transaction是否进行过特定用户的授权。

在实际的智能合约的应用中，如果要在智能合约里判断某段二进制数据是否是用特定的私钥进行的签名也可以用上面的办法。过程如下：

- 首先用户用自己的私钥对数据进行签名
- 用户将数据，签名，公钥（注意这里不是私钥）传给智能合约
- 智能合约即可判断数据是否是用特别的私钥签的名，并进行相应的操作。
