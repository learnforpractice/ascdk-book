---
comments: true
---

# Functions Related to Cryptography

The cryptographic related functions are defined in the `crypto.ts` in the `asm-chain` package, which can be imported as follows:

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

sha256 hash function

```ts
function sha256(data: u8[]): Checksum256
```

Used to check if the hash256 value is normal, if incorrect, it will throw an exception directly

```ts
function assertSha256(data: u8[], hash: Checksum256): void
```

## sha1

sha1 hash function

```ts
function sha1(data: u8[]): Checksum160
```

Used to check if the sha1 hash value is normal, if incorrect, it will throw an exception directly

```ts
function assertSha1(data: u8[], hash: Checksum160): void
```


## sha512

sha512 hash function

```ts
function sha512(data: u8[]): Checksum512
```

Used to check if the hash512 value is normal, if incorrect, it will throw an exception directly

```ts
function assertSha512(data: u8[], hash: Checksum512): void
```

## ripemd160

ripemd160 hash function

```ts
function ripemd160(data: u8[]): Checksum160
```

Used to check if the hash value of ripemd160 is normal, if incorrect, it will throw an exception directly

```ts
function assertRipemd160(data: u8[], hash: Checksum160): void
```

## recoverKey

Used to recover the public key from digest and signature

```ts
function recoverKey(digest: Checksum256, sig: Signature): PublicKey
```

Check if the signature is normal, if abnormal, it will throw an exception

```ts
function assertRecoverKey(digest: Checksum256, sig: Signature, pub: PublicKey): void
```

## Example:

[Complete Example Code](https://github.com/learnforpractice/ascdk-book/tree/master/examples/cryptotest)

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

Test code:

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

Compilation:

```bash
cd examples/cryptotest
yarn
yarn build
```

Testing:

```
ipyeos -m pytest -s -x test.py -k test_hash
ipyeos -m pytest -s -x test.py -k test_recover_key
```

In this example code, it demonstrates the usage of common hash functions as well as `recoverKey` and `assertRecoverKey`. The usage of hash functions is pretty straightforward, here is an explanation for the `recoverKey` test code: `recoverKey` accepts two parameters, `digest` and `signature`. The digest is the result of running sha256 on a binary data. In the above code, the hash calculation is done on `hello, world` through sha256, as shown in the following code:

```python
msg = b'hello, world'
h = hashlib.sha256()
h.update(msg)
sig = eos.sign_digest(h.hexdigest(), priv)
```

`eos.sign_digest` is used to sign the data.

In the smart contract, the corresponding code is:

```rust
let digest = sha256(&msg);
```

Here is an explanation for `testrecover`:

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

The principle of `recoverKey` is the same as the verification of signatures in Transactions by nodes, which is to sign the digest and then verify it with a public key. In actual smart contract applications, if you want to determine whether a piece of binary data is signed with a specific private key in a smart contract, you can use the above method. The process is as follows:

- The contract saves the public key corresponding to a user's private key
- The user signs the data with their own private key
- The user sends the data and its corresponding signature to the smart contract
- The smart contract can call `RecoverKey` to restore the public key from the user's data and its signature
- The smart contract reads the user's public key saved on the chain, and compares it with the public key restored by calling `RecoverKey`. If they match, it can be confirmed that the data is signed by the corresponding user.