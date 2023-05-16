---
comments: true
---

# Cryptographic functions

Cryptographic functions are defined in `crypto.codon`. They can be imported like the following code:

```python
from chain.crypto import sha256
```

Or just import the `crypto` module:

```python
from chain import crypto
```

Then, call internal functions such as `crypto.sha256` using the appropriate format.

## sha256

Hash function using the sha256 algorithm:

```python
def sha256(data: bytes) -> Checksum256:
```

Used for checking if the hash256 value is correct. If not, an exception will be raised:

```python
def assert_sha256(data: bytes, hash: Checksum256):
```

## sha1

Hash function using the sha1 algorithm:

```python
def sha1(data: bytes) -> Checksum160:
```

Used for checking if the sha1 hash value is correct. If not, an exception will be raised:

```python
def assert_sha1(data: bytes, hash: Checksum160):
```

## sha512

Hash function using the sha512 algorithm:

```python
def sha512(data: bytes) -> Checksum512:
```

Used for checking if the hash512 value is correct. If not, an exception will be raised:

```python
def assert_sha512(data: bytes, hash: Checksum512):
```

## ripemd160

Hash function using the ripemd160 algorithm:

```python
def ripemd160(data: bytes) -> Checksum160:
```

Used for checking if the ripemd160 algorithm hash value is correct. If not, an exception will be raised:

```python
def assert_ripemd160(data: bytes, hash: Checksum160):
```

## recover_key

Used to recover the public key from digest and signature:

```python
def recover_key(digest: Checksum256, sig: Signature) -> PublicKey:
```

Checks if the signature is correct. If not, an exception will be raised:

```python
def assert_recover_key(digest: Checksum256, sig: Signature, pub: PublicKey):
```

## Example:

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

Testing code:

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

Compilation:

```
python-contract build crypto_example.codon
```

Testing:

```
ipyeos -m pytest -s -x test.py -k test_crypto
ipyeos -m pytest -s -x test.py -k test_recover
```

In this example code, the usage of commonly used hash functions as well as the usage of `recover_key` and `assert_recover_key` are demonstrated separately. The usage of hash functions is relatively simple; here is an explanation of the test code for `recover_key`:
`recover_key` takes two parameters, namely `digest` and `signature`. The `digest` is the result of running the sha256 algorithm on a binary data. In the above code, the hash calculation was performed on `hello,world` using the sha256 algorithm.

```python
h = hashlib.sha256()
h.update(b'hello,world')
digest = h.hexdigest()
```

The computed result is passed as a parameter to the action.

Here is an explanation of `testrecover`:

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

In the sent transaction, the user's signature on the transaction is also required to indicate that the user has authorized the transaction. Then, in the smart contract, the `require_auth` function can be called to determine whether the transaction has been authorized by a specific user.

In actual smart contract applications, the above method can also be used to determine whether a certain section of binary data in the smart contract is signed using a specific private key. The process is as follows:

- Firstly, the user signs the data using his own private key
- The user passes the data, signature, and public key (note that this is not a private key) to the smart contract
- The smart contract can then determine whether the data is signed using a particular private key and perform corresponding operations.
