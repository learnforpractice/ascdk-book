#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(warnings))]

#[rust_chain::contract]
mod secondaryindex {
    use rust_chain::{
        Name,
        chain_println,
        check,
    };

    #[chain(table="mydata")]
    pub struct MyData {
        #[chain(primary)]
        a: u64,
        #[chain(secondary)]
        b: u64,
        #[chain(secondary)]
        c: u128,
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

        #[chain(action = "test1")]
        pub fn test1(&self) {
            let db = MyData::new_table(self.receiver);

            let data = MyData{a: 1, b: 2, c: 3};
            db.store(&data, self.receiver);

            let data = MyData{a: 11, b: 22, c: 33};
            db.store(&data, self.receiver);

            let data = MyData{a: 111, b: 222, c: 333};
            db.store(&data, self.receiver);
            chain_println!("++++test1 done!");
        }

        #[chain(action = "test2")]
        pub fn test2(&self, b: u64) {
            chain_println!("+++b:", b);
            let db = MyData::new_table(self.receiver);
            let idx = db.get_idx_by_b();
            let (it_secondary, mut secondary_value) = idx.lower_bound(b);
            if it_secondary.is_ok() {
                chain_println!("++++primary value", it_secondary.primary, "secondary value:", secondary_value);
                // update secondary value
                let payer = self.receiver;
                secondary_value += 1;
                db.update_b(&it_secondary, secondary_value, payer);
            }
        }

        #[chain(action = "test3")]
        pub fn test3(&self, b: u64) {
            chain_println!("+++b:", b);
            let db = MyData::new_table(self.receiver);
            let idx = db.get_idx_by_b();
            let it = idx.find(b);
            check(it.is_ok(), "b not found");
            
            let primary_it = db.find(it.primary);
            check(primary_it.is_ok(), "primary key not found");
            db.remove(&primary_it);

            let it = idx.find(b);
            check(!it.is_ok(), "b shoud not exit now");
        }
    }
}
