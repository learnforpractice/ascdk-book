#![cfg_attr(not(feature = "std"), no_std)]

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

#[cfg(feature="std")]
#[no_mangle]
fn native_apply(receiver: u64, first_receiver: u64, action: u64) {
    crate::commonfunctions::native_apply(receiver, first_receiver, action);
}

#[cfg(test)]
mod tests {

    use rust_chain::ChainTester;
    use rust_chain::serializer::Packer as _;

    fn deploy_contract(tester: &mut ChainTester) {
        let ref wasm_file = format!("./target/commonfunctions.wasm");
        let ref abi_file = format!("./target/commonfunctions.abi");
        tester.deploy_contract("hello", wasm_file, abi_file).unwrap();
    }

    fn update_auth(tester: &mut ChainTester) {
        let updateauth_args = r#"{
            "account": "hello",
            "permission": "active",
            "parent": "owner",
            "auth": {
                "threshold": 1,
                "keys": [
                    {
                        "key": "EOS6AjF6hvF7GSuSd4sCgfPKq5uWaXvGM2aQtEUCwmEHygQaqxBSV",
                        "weight": 1
                    }
                ],
                "accounts": [{"permission":{"actor": "hello", "permission": "eosio.code"}, "weight":1}],
                "waits": []
            }
        }"#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;

        tester.push_action("eosio", "updateauth", updateauth_args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_inc() {
        let mut tester = ChainTester::new();
        //uncomment the following line to enable contract debugging.
        // tester.enable_debug_contract("hello", true).unwrap();

        deploy_contract(&mut tester);
        update_auth(&mut tester);
    
        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "inc", "".into(), permissions).unwrap();
        tester.produce_block();

        tester.push_action("hello", "inc", "".into(), permissions).unwrap();
        tester.produce_block();
    }
}
