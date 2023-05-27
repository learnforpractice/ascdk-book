---
comments: true
---

# Database Operations

Chain data storage and retrieval are crucial functions of smart contracts. The EOS chain implements a memory database, supporting data storage in table form. Each entry in a table has a unique primary index, also known as a primary key, of the uint64 type. The raw data stored in the table are binary data of any length. When the smart contract invokes the data storage function, it serializes class data and stores it in the table. When reading, it deserializes the raw data back into class objects. It also supports secondary index tables of types uint64, Uint128, Uint256, Float64, Float128, which can be considered special tables with fixed data length. Primary index tables and secondary index tables can work together to implement multi-indexing. A table can have multiple secondary index tables. Secondary index table values can be duplicated, but the primary index of the primary index table must be unique.

Below, we will use an example to explain the use of the EOS chain's in-memory database.

## store/find/update

Storage, search, and update are the most basic functions of a database. The following code demonstrates how to use these three functions to perform chain counting.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(warnings))]

#[rust_chain::contract]
mod counter {
    use rust_chain::{
        Name,
        chain_println,
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
    }
}
```

Explanation of the code above:

- `#[chain(primary)]` specifies a primary index member variable as account, of type `Name`. Note that if the primary index is not of `u64` type, you need to implement the `get_primary` method of the `rust_chain::db::PrimaryValueInterface` trait, as the `Name` structure has implemented:
```rust
impl PrimaryValueInterface for Name {
    fn get_primary(&self) -> u64 {
        return self.value();
    }
}
```
- The `counter` module uses the `#[rust_chain::contract]` to reference the `contract` macro from the `rust_chain` package. This macro generates additional code related to database operations and action processing based on the `chain` attributes in the module.
- `#[chain(table="counter")]` uses the `chain` attribute to define a table named `counter`, which is a `name` structure. The `table` keyword instructs the compiler to generate table-related code, wrapping the `MultiIndex` structure related code in `rust-chain` to make it easier for developers to call.
- `#[chain(action = "inc")]` indicates that the `inc_count` method is an `action`, triggered by the Action structure included in the Transaction.
- `Counter::new_table(self.receiver)` creates a table, where `self.receiver` specifies the name of the current contract account, indicating that the table is stored in the current contract account.
- `let it = db.find(account.value());` is used to find the value located at the primary index, and the returned value is of type `Iterator`.
- `let Some(mut value) = it.get_value()` is used to obtain the value in `Iterator`. If the value doesn't exist, `db.store(&value, payer);` is used to save a new value to the database. Otherwise, after incrementing count by 1, `db.update(&it, &value, payer);` is used to update the data in the database. The payer specifies which account pays for the RAM resources and needs to have signed with the `active` permission of that account in the Transaction.

Compilation:

```bash
cd examples/counter
rust-contract build
```

Testing:

```bash
ipyeos -m pytest -s -x test.py -k test_counter
```

Test code to run is as follows:

```python
@chain_test
def test_counter(tester: ChainTester):
    deploy_contract(tester, 'counter')
    args = {}
    
    r = tester.push_action('hello', 'inc', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()

    r = tester.push_action('hello', 'inc', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

## Removal

The code below demonstrates how to delete a piece of data from the database.

```rust
#[chain(action = "testremove")]
pub fn test_remove(&self, account: Name) {
    let db = Counter::new_table(self.receiver);
    let it = db.find(account.value());
    check(it.is_ok(), "key not found");
    db.remove(&it);
}
```

The code above first calls the `let it = db.find(account.value());` method to find the specified data, then calls `remove` to delete it, and checks if the specified index's data exists by calling `it.is_ok()`.

**Note:**

Here, `remove` doesn't need the `payer` account's permission specified by `store` or `update` to delete data. Therefore, in actual applications, you need to ensure the specified account's permission by calling `rust_chain.require_auth`, for example:

```rust
require_auth(name!("hello"))
```

Test code:

```python
@chain_test
def test_remove(tester: ChainTester):
    deploy_contract(tester, 'counter')
    args = {'account': 'alice'}
    
    r = tester.push_action('hello', 'inc', args, {'hello': 'active'})
    tester.produce_block()
    r = tester.get_table_rows(True, 'hello', '', 'counter', '', '', 10)
    logger.info("+++++++++table rows: %s", r)

    r = tester.push_action('hello', 'inc', args, {'hello': 'active'})
    tester.produce_block()
    r = tester.get_table_rows(True, 'hello', '', 'counter', '', '', 10)
    logger.info("+++++++++table rows: %s", r)

    r = tester.push_action('hello', 'testremove', args, {'hello': 'active'})
    tester.produce_block()
    r = tester.get_table_rows(True, 'hello', '', 'counter', '', '', 10)
    logger.info("+++++++++table rows: %s", r)
```

Here, the `inc` action is called first to ensure that data is stored in the database. Then, `testremove` is called to delete the specified data. The `get_table_rows` function is used to confirm whether the data has been added, modified, or deleted. The usage of `get_table_rows` will be introduced later.

Compilation:

```bash
cd examples/counter
rust-contract build .
```

Testing:

```bash
ipyeos -m pytest -s -x test.py -k test_remove
```

Output:

```
INFO     test:test.py:90 +++++++++table rows: {'rows': [{'account': 'alice', 'count': 1}], 'more': False, 
INFO     test:test.py:95 +++++++++table rows: {'rows': [{'account': 'alice', 'count': 2}], 'more': False, 'next_key': ''}
INFO     test:test.py:100 +++++++++table rows: {'rows': [], 'more': False, 'next_key': ''}
```

## lower_bound/upper_bound

These two methods are also used to find elements in the table. Unlike the `find` method, these two functions are used for fuzzy searches. The `lower_bound` method returns the `Iterator` of the first element `>=` the specified `id`, and the `upper_bound` method returns the `Iterator` of the first element `>` the specified `id`. The usage is as follows:

```rust
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
```

Test code:

```python
@chain_test
def test_bound(tester: ChainTester):
    deploy_contract(tester, 'counter')
    args = {}
    r = tester.push_action('hello', 'testbound', args, {'hello': 'active'})
```

Compilation:

```bash
cd examples/counter
rust-contract build
```

Running the test:

```bash
ipyeos -m pytest -s -x test.py -k test_bound
```

Output:

```
+++++db.lower_bound(1) return primary key: 1
+++++db.upper_bound(3) return primary key: 5
```

## Using the API to Query the Table

The above examples are all about how to operate the on-chain database's tables through smart contracts. In fact, the off-chain `get_table_rows` API interface provided by EOS can also be used to query the tables on the chain. In the `ipyeos`'s `ChainTester` class and `pyeoskit`'s `ChainApiAsync` and `ChainApi` classes, the `get_table_rows` interface is provided to facilitate table query operations.

In Python code, the definition of `get_table_rows` is as follows:

```python
def get_table_rows(self, _json, code, scope, table,
                                lower_bound, upper_bound,
                                limit,
                                key_type='',
                                index_position='', 
                                reverse = False,
                                show_payer = False):
    """ Fetch smart contract data from an account. 
    key_type: "i64"|"i128"|"i256"|"float64"|"float128"|"sha256"|"ripemd160"
    index_position: "2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"|"10"
    """
```

Explanation of this interface's parameters:

- `_json`: True to return data in JSON format, False to return raw data represented in hexadecimal
- `code`: The account where the table is located
- `scope`: Usually set as an empty string, when there are the same `code` and `table`, different `scope` can be used to distinguish different tables
- `table`: The name of the data table to be queried
- `lower_bound`: The starting primary key for the query, either string type or numerical type. When it's a string type, it can represent a `name` type. If it starts with a hexadecimal string '0x', it represents a numerical type. If it's empty, it means starting the query from the beginning.
- `upper_bound`: The ending primary key for the query, either string type or numerical type. When it's a string type, it can represent a `name` type. If it starts with a hexadecimal string '0x', it represents a numerical type. If it's empty, it means there's no upper limit set, and all values `>=` `lower_bound` will be returned.
- `limit`: Used to limit the number of returned values
- `key_type`: Used to specify the type of index, by default it's a 64-bit unsigned integer type
- `index_position`: Used to specify the relative position of the index. Empty or '1' indicates the main index, from '2' and above indicates the position of the secondary index
- `reverse`: Specifies whether to return data in reverse order
- `show_payer`: Specifies whether to show the account paying for RAM resources

To query the table through `get_table_rows`, the structure of the table must be visible in the ABI description. In the `db_example1` example, the generated `test.abi` includes the following information, which is the description of the table:

```json
"tables": [
    {
        "name": "counter",
       "type": "Counter",
        "index_type": "i64",
        "key_names": [],
        "key_types": []
    }
]
```

Test code:

```python
@chain_test
def test_offchain_find(tester):
    deploy_contract(tester, 'counter')

    r = tester.push_action('hello', 'testbound', b'', {'hello': 'active'})
    tester.produce_block()

    r = tester.get_table_rows(False, 'hello', '', 'counter', '', '', 10)
    logger.info("+++++++rows: %s", r)

    r = tester.get_table_rows(True, 'hello', '', 'counter', '', '', 10)
    logger.info("+++++++rows: %s", r)

    r = tester.get_table_rows(True, 'hello', '', 'counter', '1', '3', 10)
    logger.info("+++++++rows: %s", r)
```

Running the test code:

```bash
ipyeos -m pytest -s -x test.py -k test_offchain_find
```

Output:

```
INFO     test:test.py:118 +++++++rows: {'rows': ['01000000000000000100000000000000', '03000000000000000100000000000000', '05000000000000000100000000000000'], 'more': False, 'next_key': ''}
INFO     test:test.py:121 +++++++rows: {'rows': [{'account': '............1', 'count': 1}, {'account': '............3', 'count': 1}, {'account': '............5', 'count': 1}], 'more': False, 'next_key': ''}
INFO     test:test.py:124 +++++++rows: {'rows': [{'account': '............1', 'count': 1}, {'account': '............3', 'count': 1}], 'more': False, 'next_key': ''}
```

Note that the `account` here is of the `name` structure, which converts the numerical value to a string, so the output may appear somewhat odd.

## Storage, Querying, and Updating of Secondary Indexes

First, let's look at the following example:

[Example Code](https://github.com/learnforpractice/rscdk-book/tree/master/examples/secondaryindex)

```rust
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
    }
}
```

In this example, two secondary indexes are defined:

```rust
#[chain(secondary)]
b: u64,
#[chain(secondary)]
c: u128,
```

The `test1` action calls the `store` method to store 3 sets of data. The `test2` action demonstrates the use of `lower_bound` to search for secondary indexes, as well as the use of the generated `update_b` method to update secondary index data.

Test code:

```python
@chain_test
def test_secondary(tester):
    deploy_contract(tester, 'secondaryindex')

    args = {}
    r = tester.push_action('hello', 'test1', args, {'hello': 'active'})
    tester.produce_block()
    r = tester.get_table_rows(True, 'hello', '', 'mydata', '', '', 10)
    logger.info("+++++++rows: %s", r)

    args = {
        'b': 222
    }
    r = tester.push_action('hello', 'test2', args, {'hello': 'active'})
    tester.produce_block()
    r = tester.get_table_rows(True, 'hello', '', 'mydata', '', '', 10)
    logger.info("+++++++rows: %s", r)
```

Compile:

```bash
cd examples/secondaryindex
rust-contract build
```

Run the test:

```bash
ipyeos -m pytest -s -x test.py -k test_secondary
```

Output:

```
INFO     test:test.py:78 +++++++rows: {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
[(hello,test2)->hello]: CONSOLE OUTPUT BEGIN =====================
+++b: 222
++++primary value 111 secondary value: 222

[(hello,test2)->hello]: CONSOLE OUTPUT END   =====================
INFO     test:test.py:86 +++++++rows: {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 223, 'c': '333'}], 'more': False, 'next_key': ''}
```

From the output:

```
{'a': 111, 'b': 223, 'c': '333'}
```

It can be seen that 222 has been changed to 223, while other values remain unchanged.
  
## Deletion of Secondary Indexes

```rust
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
```

The code above can be explained as follows:

- `let it = idx.find(b);` searches for the secondary index.
- `let primary_it = db.find(it.primary);` gets the primary index through `it.primary`, and returns the iterator of the primary index.
- `db.remove(&primary_it);` removes the element from the table, including the primary index and all secondary indexes.

From the example above, it can be seen that the deletion of secondary indexes first finds the primary index through the secondary index, and then deletes it through the primary index.

Test code:

```python
@chain_test
def test_remove(tester):
    deploy_contract(tester, 'secondaryindex')

    args = {}
    r = tester.push_action('hello', 'test1', args, {'hello': 'active'})
    tester.produce_block()
    r = tester.get_table_rows(True, 'hello', '', 'mydata', '', '', 10)
    logger.info("+++++++rows: %s", r)

    args = {
        'b': 222
    }
    r = tester.push_action('hello', 'test3', args, {'hello': 'active'})
    tester.produce_block()
    r = tester.get_table_rows(True, 'hello', '', 'mydata', '', '', 10)
    logger.info("+++++++rows: %s", r)
```

Compile:

```bash
cd examples/secondaryindex
go-contract build .
```

Run the test:

```bash
ipyeos -m pytest -s -x test.py -k test_remove
```

Output:

```
INFO     test:test.py:96 +++++++rows: {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
[(hello,test3)->hello]: CONSOLE OUTPUT BEGIN =====================
+++b: 222
[(hello,test3)->hello]: CONSOLE OUTPUT END   =====================
INFO     test:test.py:104 +++++++rows: {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}], 'more': False, 'next_key': ''}
```

Comparing the return values of the two `get_table_rows`, it can be seen that the data set `{'a': 111, 'b': 222, 'c': '333'}` has been deleted.

## Use API to Query Tables with Secondary Indexes

In the examples above, two secondary indexes were defined, of types `u64` and `u128` respectively. The `get_table_rows` API also supports finding corresponding values through secondary indexes.

```python
@chain_test
def test_offchain_find(tester: ChainTester):
    deploy_contract(tester, 'secondaryindex')

    args = {}
    r = tester.push_action('hello', 'test1', args, {'hello': 'active'})
    r = tester.get_table_rows(True, 'hello', '', 'mydata', '1', '', 10, key_type="i64", index_position="1")
    logger.info("+++++++rows: %s", r)

    r = tester.get_table_rows(True, 'hello', '', 'mydata', '11', '', 10, key_type="i64", index_position="2")
    logger.info("+++++++rows: %s", r)
    # 0x14d == 333
    r = tester.get_table_rows(True, 'hello', '', 'mydata', '0x14d', '', 10, key_type="i128", index_position="3")
    logger.info("+++++++rows: %s", r)
```

**Note**, when querying `c`, because its type is `u128`, for values beyond the range of `u64` type, you can use hexadecimal to represent the data. For example, the hexadecimal `0x14d` is `333` in decimal.

Run the test case:

```bash
ipyeos -m pytest -s -x test.py -k test_offchain_find
```

The output of the test code above is as follows:
```
INFO     test:test.py:113 +++++++rows: {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
INFO     test:test.py:116 +++++++rows: {'rows': [{'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
INFO     test:test.py:119 +++++++rows: {'rows': [{'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
```

## Conclusion

The data storage function in EOS is quite robust, and it has the feature of secondary index tables, which makes data search very flexible. This chapter provides a detailed explanation of the code for adding, deleting, updating, and querying database tables. This chapter contains a lot of content and will take some time to digest. You can make some changes based on the examples and try to run them to enhance your understanding of the knowledge points in this chapter.

[Example Code 1](https://github.com/learnforpractice/rscdk-book/tree/master/examples/counter)
[Example Code 2](https://github.com/learnforpractice/rscdk-book/tree/master/examples/secondaryindex)

