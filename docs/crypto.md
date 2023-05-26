---
comments: true
---

# Cryptography Related Functions

Cryptography related functions are defined in `crypto.rs` in the `rust-chain` package, they can be imported as shown below:

```rust
use rust_chain::{
    sha256,
};
```

Or just import the crypto module:

```rust
use rust_chain::crypto;
```

## sha256

SHA-256 hash function

```rust
pub fn sha256(data: &[u8]) -> Checksum256
```

Used to check if the SHA-256 hash value is normal, will throw an exception if incorrect.

```rust
pub fn assert_sha256(data: &[u8], hash: &Checksum256)
```

## sha1

SHA-1 hash function

```rust
pub fn sha1( data: &[u8]) -> Checksum160
```

Used to check if the SHA-1 hash value is normal, will throw an exception if incorrect.

```rust
pub fn assert_sha1(data: &[u8], hash: &Checksum160)
```

## sha512

SHA-512 hash function

```rust
pub fn sha512( data: &[u8]) -> Checksum512
```

Used to check if the SHA-512 hash value is normal, will throw an exception if incorrect.

```rust
pub fn assert_sha512(data: &[u8], hash: &Checksum512)
```

## ripemd160

RIPEMD-160 hash function

```rust
pub fn ripemd160(data: &[u8]) -> Checksum160
```

Used to check if the RIPEMD-160 hash value is normal, will throw an exception if incorrect.

```rust
pub fn assert_ripemd160(data: &[u8], hash: &Checksum160)
```

## recover_key

Used to recover the public key from the digest and signature.

```rust
pub fn recover_key( digest: &Checksum256 , sig: &Signature) -> PublicKey
```

Check if the signature is normal, will throw an exception if incorrect.

```rust
pub fn assert_recover_key(digest: &Checksum256, sig: &Signature, pubkey: &PublicKey)
```

## Example:

[Full Example Code](https://github.com/learnforpractice/rscdk-book/tree/master/examples/cryptotest)

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

Test code:

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

Compile:

```bash
cd examples/cryptotest
rust-contract build
```

Test:

```
ipyeos -m pytest -s -x test.py -k test_hash
ipyeos -m pytest -s -x test.py -k test_recover_key
```

In this example code, we demonstrate the usage of commonly used hash functions and `recover_key` and `assert_recover_key` functions. The usage of hash functions is quite straightforward. Here we explain the test code for `recover_key`.

`recover_key` accepts two parameters, `digest` and `signature`. The digest is the result of applying SHA-256 to some binary data. In the above code, we compute the SHA-256 hash of `hello, world`:

```python
msg = b'hello, world'
h = hashlib.sha256()
h.update(msg)
sig = eos.sign_digest(h.hexdigest(), priv)
```

In the smart contract, the corresponding code is:

```rust
let digest = sha256(&msg);
```

Explanation for `testrecover`:

```rust
#[chain(action="testrecover")]
pub fn test_recover(&self, msg: Vec<u8>, sig: Signature, pub_key: PublicKey) {
    chain_println!("++++++msg:", msg);
    let digest = sha256(&msg);
    let _pubkey = recover_key(&digest, &sig);
    // Check if the public key is correct
    check(_pubkey == pub_key, "_pubkey == k1"); 

    // The function is equivalent to the above two lines of code
    assert_recover_key(&digest, &sig, &pub_key); 
}
```

The principle of `recover_key` is the same as the node verifying the validity of the signature in the Transaction. It works by signing the digest and then verifying it with the public key.

In practical smart contract applications, if you want to determine in the smart contract whether a piece of binary data has been signed with a specific private key, you can use the method described above. The process is as follows:

- The contract saves the public key corresponding to a user's private key.
- The user signs the data with their private key.
- The user sends the data and the corresponding signature to the smart contract.
- The smart contract can call `RecoverKey` to recover the public key from the user's data and the signature of the data.
- The smart contract reads the user's public key stored on the chain and compares it with the public key recovered by calling `RecoverKey`. If they are the same, it can be determined that the data is signed by the corresponding user.