#![cfg_attr(not(feature = "std"), no_std)]

#[rust_chain::contract]
#[allow(dead_code)]
mod hello {
    use rust_chain::{
        Name,
        chain_println,
    };

    #[chain(table="counter")]
    pub struct Counter {
        #[chain(primary)]
        key: u64,
        count: u64
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

        #[chain(action = "inc")]
        pub fn inc_count(&self) {
            let db = Counter::new_table(self.receiver);
            let it = db.find(1u64);
            if let Some(mut value) = it.get_value() {
                value.count += 1;
                db.update(&it, &value, self.receiver);
                chain_println!("count is", value.count);
            } else {
                db.store(&Counter{key: 1, count: 1}, self.receiver);
                chain_println!("count is", 1);
            }
        }
    }
}

#[cfg(feature="std")]
#[no_mangle]
fn native_apply(receiver: u64, first_receiver: u64, action: u64) {
    crate::hello::native_apply(receiver, first_receiver, action);
}

#[cfg(test)]
mod tests {

    use rust_chain::ChainTester;
    use rust_chain::serializer::Packer as _;

    fn deploy_contract(tester: &mut ChainTester) {
        let ref wasm_file = format!("./target/hello.wasm");
        let ref abi_file = format!("./target/hello.abi");
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
