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

        #[chain(action = "testhash")]
        pub fn test_hash(&self) {
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
