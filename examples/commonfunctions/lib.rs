#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(warnings))]

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
