#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(warnings))]

#[rust_chain::contract]
mod counter {
    use rust_chain::{
        Name,
        chain_println,
        check,
    };
    
    #[chain(table="counter")]
    pub struct Counter {
        #[chain(primary)]
        account: Name,
        count: u64,
    }

    #[chain(main)]
    #[allow(dead_code)]
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
        pub fn inc_count(&self, account: Name) {
            let db = Counter::new_table(self.receiver);
            let it = db.find(account.value());
            let payer = self.receiver;
            if let Some(mut value) = it.get_value() {
                value.count += 1;
                db.update(&it, &value, payer);
                chain_println!("+++count:", value.count);
            } else {
                let value = Counter{account: account, count: 1};
                db.store(&value, payer);
                chain_println!("+++count:", value.count);
            }
        }

        #[chain(action = "testremove")]
        pub fn test_remove(&self, account: Name) {
            let db = Counter::new_table(self.receiver);
            let it = db.find(account.value());
            check(it.is_ok(), "key not found");
            db.remove(&it);
        }

        #[chain(action = "testbound")]
        pub fn test_bound(&self) {
            let payer = self.receiver;

            let db = Counter::new_table(self.receiver);
            let value = Counter{account: Name{n: 1}, count: 1};
            db.store(&value, payer);

            let value = Counter{account: Name{n: 3}, count: 1};
            db.store(&value, payer);

            let value = Counter{account: Name{n: 5}, count: 1};
            db.store(&value, payer);

            let it = db.lower_bound(1);
            check(it.is_ok() && it.get_primary() == Some(1), "bad value");
            chain_println!("+++++db.lower_bound(1) return primary key:", it.get_primary().unwrap());

            let it = db.upper_bound(3);
            check(it.is_ok() && it.get_primary() == Some(5), "bad value");
            chain_println!("+++++db.upper_bound(3) return primary key:", it.get_primary().unwrap());
        }
    }
}
