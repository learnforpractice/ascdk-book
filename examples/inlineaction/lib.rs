#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(warnings))]

#[rust_chain::contract]
mod inlineaction {
    use rust_chain::{
        Name,
        Action,
        PermissionLevel,    
        name,
        chain_println,
        serializer::Packer,
        Asset,
        Symbol,
    };

    #[chain(packer)]
    struct Transfer {
        from: Name,
        to: Name,
        quantity: Asset,
        memo: String
    }

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

        #[chain(action = "testaction")]
        pub fn test_action(&self) {
            let transfer = Transfer{
                from: name!("hello"),
                to: name!("alice"),
                quantity: Asset::new(10000, Symbol::new("EOS", 4)),
                memo: String::from("hello, world")
            };
            let perm = PermissionLevel::new(name!("hello"), name!("active"));
            let action = Action::new(name!("eosio.token"), name!("transfer"), perm, &transfer);
            action.send();
        }
    }
}

#[cfg(feature="std")]
#[no_mangle]
fn native_apply(receiver: u64, first_receiver: u64, action: u64) {
    crate::inlineaction::native_apply(receiver, first_receiver, action);
}

#[cfg(test)]
mod tests {

    use rust_chain::ChainTester;
    use rust_chain::serializer::Packer;
    use rust_chain::Encoder;

    fn deploy_contract(tester: &mut ChainTester) {
        let ref wasm_file = format!("./target/inlineaction.wasm");
        let ref abi_file = format!("./target/inlineaction.abi");
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

        let old_balance_hello = tester.get_balance("hello");
        let old_balance_alice = tester.get_balance("alice");

        let args = crate::inlineaction::testaction{};
        tester.push_action("hello", "testaction", Encoder::pack(&args).into(), permissions).unwrap();
        tester.produce_block();

        assert!(tester.get_balance("hello") == old_balance_hello - 10000);
        assert!(tester.get_balance("alice") == old_balance_alice + 10000);

        println!("{}", tester.get_balance("hello"));
    }
}
