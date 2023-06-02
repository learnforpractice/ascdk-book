---
comments: true
---

# Database Operations

On-chain data storage and retrieval is a key feature of smart contracts. The EOS chain has implemented a memory database that supports data storage in the form of tables. Each item in each table has a unique primary key, which is of the uint64 type. The raw data stored in the table is binary data of arbitrary length. When the smart contract calls the function to store data, the data of the class is serialized and stored in the table. When reading, the original data is converted into class objects through deserialization. It also supports secondary index tables of u64, u128, u256, f64, and Float128 types. Secondary index tables can be seen as special tables with fixed data length. Primary index tables and secondary index tables can be used together to achieve multi-index functions. A secondary index table can have multiple, and the values in the secondary index table can be duplicated, but the primary key of the primary index table must be unique.

Below, let's discuss the use of EOS's on-chain memory database using an example.

## store/find/update

Storage, lookup, and update are the most basic functions of a database. The following code demonstrates how to perform on-chain counting through these three functions.

[Full code](https://github.com/learnforpractice/ascdk-book/tree/master/examples/counter)

```ts
import {
    Name,
    Contract,
    print,
} from "asm-chain";

@table("counter")
class Counter {
    public key: u64;
    public count: u64;
    constructor(count: u64=0) {
        this.count = count;
        this.key = Name.fromString("counter").N;
    }

    @primary
    get primary(): u64 {
        return this.key;
    }
}

@contract
class MyContract extends Contract {
    constructor(receiver: Name, firstReceiver: Name, action: Name) {
        super(receiver, firstReceiver, action);
    }

    @action("inc")
    inc(): void {
        let mi = Counter.new(this.receiver);
        let it = mi.find(Name.fromString("counter").N);
        let count: u64 = 0;
        let payer: Name = this.receiver;

        if (it.isOk()) {
            let counter = mi.get(it)
            counter.count += 1;
            mi.update(it, counter, payer);
            count = counter.count;
        } else {
            let counter = new Counter(1);
            mi.store(counter, payer);
            count = 1;
        }
        print(`++++++++count:${count}`);
    }
}
```

Let's explain the above code:

- `@primary` specifies a primary index member variable as key, of `u64` type.
- `@table("counter")` This line of code defines a table named `counter`, which is a `name` structure. The `table` decorator guides the compiler to generate table-related code. The generated code encapsulates the `MultiIndex` structure related code in `asm-chain`, which makes it easier for developers to call.
- `@action("inc")` indicates that the `inc` method is an `action` that will be triggered by the Action structure included in the Transaction.
- `let mi = Counter.new(this.receiver);` indicates to create a table. `this.receiver` specifies the current contract's account name, indicating that the table is stored in the current contract account.
- `let it = mi.find(Name.fromString("counter").N);` is used to find the value where the primary index is located. The returned value is of the `PrimaryIterator` type.
- `let counter = mi.get(it)` is used to get the value in `PrimaryIterator`. If the value does not exist, `mi.store(counter, payer);` is called to save a new value to the database. Otherwise, after increasing the count by 1, `mi.update(it, counter, payer);` is called to update the data in the database. The payer specifies which account pays for RAM resources, and it is required to be signed with the `active` permission of this account in the Transaction.

Compilation:

```bash
cd examples/counter
yarn
yarn build
```

Testing:

```bash
ipyeos -m pytest -s -x test.py -k test_inc
```

The test code being run is as follows:

```python
@chain_test
def test_inc(tester):
    deploy_contract(tester, 'counter')
    args = {'account': 'hello'}
    r = tester.push_action('hello', 'inc', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
    ret = tester.get_table_rows(True, 'hello', '', 'counter', '', '', 10)
    logger.info("+++++++rows: %s", ret)

    r = tester.push_action('hello', 'inc', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
    ret = tester.get_table_rows(True, 'hello', '', 'counter', '', '', 10)
    logger.info("+++++++rows: %s", ret)
```

## Remove

The following code demonstrates how to delete an item of data from the database.

```ts
@action("testremove")
testRemove(account: Name): void {
    requireAuth(account);
    let mi = Counter.new(account);
    let it = mi.find(account.N);
    check(it.isOk(), "account not found");
    mi.remove(it);
}
```

The above code first calls the `let it = mi.find(account.N);` method to find the specified data, then calls `remove` to delete it. It calls `it.isOk()` to check whether the data at the specified index exists.

**Note:**

Here, `remove` does not need the permission of the `payer` account specified by `store` or `update` to delete data. Therefore, in actual applications, you need to call `asm_chain.requireAuth` to ensure the permission of the specified account before you can delete data, for example:

```ts
requireAuth(account);
```

Test code:

```python
@chain_test
def test_remove(tester):
    deploy_contract(tester, 'counter')
    args = {'account': 'hello'}
    r = tester.push_action('hello', 'inc', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
    ret = tester.get_table_rows(True, 'hello', '', 'counter', '', '', 10)
    logger.info("+++++++rows: %s", ret)

    r = tester.push_action('hello', 'inc', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()

    ret = tester.get_table_rows(True, 'hello', '', 'counter', '', '', 10)
    logger.info("+++++++rows: %s", ret)

    args = {'account': 'hello'}
    r = tester.push_action('hello', 'testremove', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
    ret = tester.get_table_rows(True, 'hello', '', 'counter', '', '', 10)
    logger.info("+++++++rows: %s", ret)
```

Here, first, the `inc` action is called to ensure that there is data stored in the database, then `testremove` is called to delete the specified data, and `get_table_rows` is used to determine whether data has been added, modified, or deleted. The usage of `get_table_rows` will be introduced below.

Compilation:

```bash
cd examples/counter
yarn
yarn build
```

Testing:

```bash
ipyeos -m pytest -s -x test.py -k test_remove
```

Output:

```
INFO     test:test.py:93 +++++++rows: {'rows': [{'account': 'hello', 'count': 1}], 'more': False, 'next_key': ''}
INFO     test:test.py:100 +++++++rows: {'rows': [{'account': 'hello', 'count': 2}], 'more': False, 'next_key': ''}
INFO     test:test.py:107 +++++++rows: {'rows': [], 'more': False, 'next_key': ''}
```

## lowerBound/upperBound

These two methods are also used to find elements in the table. Unlike the `find` method, these two functions are used for fuzzy searches. The `lowerBound` method returns the `PrimaryIterator` of the first element `>=` the specified `id`, and the `upperBound` method returns the `PrimaryIterator` of the first element `>` the specified `id`. Let's look at the usage:

```ts
@action("testbound")
testBound(): void {
    let table = Counter.new(this.receiver);
    let payer = this.receiver;

    let value = new Counter(new Name(1), 1);
    table.store(value, payer);

    value = new Counter(new Name(3), 1);
    table.store(value, payer);

    value = new Counter(new Name(5), 1);
    table.store(value, payer);

    let it = table.lowerBound(1);
    check(it.isOk() && it.primary == 1, "bad value");
    print(`+++++db.lower_bound(1) return primary key: ${it.primary}\n`);

    it = table.upperBound(3);
    check(it.isOk() && it.primary == 5, "bad value");
    print(`+++++db.lower_bound(5) return primary key: ${it.primary}\n`);
}

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
yarn
yarn build
```

Run the test:

```bash
ipyeos -m pytest -s -x test.py -k test_bound
```

Output:

```
+++++db.lower_bound(1) return primary key: 1
+++++db.upper_bound(3) return primary key: 5
```

## Querying the table using the API

The above examples are all about how to operate the on-chain database tables through smart contracts. In fact, the `get_table_rows` API provided by EOS can also be used to query the on-chain tables. Both `ChainTester` in `ipyeos` and `ChainApiAsync` and `ChainApi` in `pyeoskit` provide the `get_table_rows` interface to facilitate querying operations on the table.

In Python code, the definition of `get_table_rows` is as follows:


```python
def get_table_rows(self, _json, code, scope, table,
                                lower_bound, upper_bound,
                                limit,
                                key_type='',
                                index_position='', 
                                encode_type='',
                                reverse = False,
                                show_payer = False):
    """ Fetch smart contract data from an account. 
    key_type: "i64"|"i128"|"i256"|"float64"|"float128"|"sha256"|"ripemd160"
    index_position: "2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"|"10"
    encode_type: "dec" or "hex", default to "dec"
    """
```

Let's break down the parameters for this function:

- `_json`: If True, it returns JSON-formatted database records, if False, it returns the raw data in hexadecimal format.
- `code`: Represents the account where the table is located.
- `scope`: Generally set as an empty string. When `code` and `table` are the same, different `scope` can be used to differentiate different tables.
- `table`: The name of the table to be queried.
- `lower_bound`: The starting value of the primary or secondary index, specified by `key_type`. It can be numeric, a string of numbers, or a hexadecimal string.
- `upper_bound`: The ending value of the primary or secondary index, specified by `key_type`. It can be numeric, a string of numbers, or a hexadecimal string. If empty, it means no upper limit has been set. If a non-empty value is set, the results will return all values that are `>=lower_bound` and `<=upper_bound`.
- `limit`: Limits the number of returned values. If the queried records exceed the limit, `more` will be set to `true` in the returned values, and `next_key` will point to the next valid index.
- `key_type`: The values can be: `"name"`, `"i64"`, `"i128"`, `"i256"`, `"float64"`, `"float128"`, `"sha256"`, `"ripemd160"`. For the primary index (i.e., `index_position` is `1`), the value can only be `"name"` or `"i64"`. For secondary index (i.e., `index_position >= 2`), the value could be any of the listed types. The encoding method of `lower_bound` and `upper_bound` under each value will be explained separately below.
- `index_position`: Specifies the relative position of the index. If it's empty or `1`, it denotes the primary index. Any number above `2` denotes the position of the secondary index.
- `encode_type`: It's either `"dec"` or `"hex"`, defaulting to `"dec"`. It specifies the encoding format of `lower_bound`, `upper_bound`, and the return value `next_key`.
- `reverse`: Specifies whether the data should be returned in reverse order.
- `show_payer`: Specifies whether to display the RAM resource paying account.

Detailed explanation for `key_type`:

- "name" is a `name` type string.
- "i64" can be a numeric type or a string of numbers, such as 123 or "123".
- "i128" can be a numeric type, a string of numbers, or a hexadecimal string, such as: 123, "123", "0xaabb", "aabb".
- "i256" when the value of `encode_type` is `"dec"` or an empty string `""`, the encoding format is: a hexadecimal string, represented in **little-endian mode**, 64 characters in length. For example: `fb54b91bfed2fe7fe39a92d999d002c550f0fa8360ec998f4bb65b00c86282f5` will be converted into two `uint128_t` type values in little-endian mode: `50f0fa8360ec998f4bb65b00c86282f5` and `fb54b91bfed2fe7fe39a92d999d002c5`. When the value of `encode_type` is `"hex"`, it uses the same encoding method as the `"sha256"` type, which is big-endian mode.
- "float64": The value is a floating-point string, like `"123.456"`.
- "float128": When the value of `encode_type` is `"dec"` or an empty string `""`, the value is a floating-point string, like `"123.456"`, and the range it represents can only be within the range allowed by `float64`. When the value of `encode_type` is `"hex"`, `encode_type` represents the data as a hexadecimal string in little-endian mode.
- "sha256": A hexadecimal string represented in **big-endian mode**, 64 characters long, will be converted into two `uint128_t` type values in little-endian mode: such as `f58262c8005bb64b8f99ec6083faf050c502d099d9929ae37ffed2fe1bb954fb` will be converted into `50f0fa8360ec998f4bb65b00c86282f5` and `fb54b91bfed2fe7fe39a92d999d002c5`. Refer to the [keytype_converter](https://github.com/AntelopeIO/leap/blob/db132c5fd44e0b1c492e46e3f51e185cd5c59ed0/plugins/chain_plugin/include/eosio/chain_plugin/chain_plugin.hpp#L900) structure's code for more details.
- "ripemd160": A hexadecimal string, 64 characters long, big-endian mode, will be converted into two `uint128_t` type values in little-endian mode: such as `83a83a3876c64c33f66f33c54f1869edef5b5d4a000000000000000000000000` will be converted into `ed69184fc5336ff6334cc676383aa883` and `0000000000000000000000004a5d5bef`. Refer to the [keytype_converter](https://github.com/AntelopeIO/leap/blob/db132c5fd44e0b1c492e46e3f51e185cd5c59ed0/plugins/chain_plugin/include/eosio/chain_plugin/chain_plugin.hpp#L918) structure's code for more details.

The `get_table_rows` function's parameters are quite complex, here's a summary:

- If `lower_bound` and `upper_bound` are empty, it means the query has no range limit.
- When the value of `key_type` is `"i256"` and `"float128"`, the encoding method of `lower_bound` and `upper_bound` is also affected by `encode_type`.

To query a table through `get_table_rows`, the structure of the table must be visible in the description of ABI. In the `db_example1` example, the generated `test.abi` contains the following information, which is a description of the table:

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

Run the test code:

```bash
ipyeos -m pytest -s -x test.py -k test_offchain_find
```

Output:

```
INFO     test:test.py:125 +++++++rows: {'rows': ['01000000000000000100000000000000', '03000000000000000100000000000000', '05000000000000000100000000000000'], 'more': False, 'next_key': ''}
INFO     test:test.py:128 +++++++rows: {'rows': [{'account': '............1', 'count': 1}, {'account': '............3', 'count': 1}, {'account': '............5', 'count': 1}], 'more': False, 'next_key': ''}
INFO     test:test.py:131 +++++++rows: {'rows': [{'account': '............1', 'count': 1}, {'account': '............3', 'count': 1}], 'more': False, 'next_key': ''}
```

Note that here the `account` is a `name` structure and will convert the value into a string, so the output may look a bit strange.

## Storage, Query, and Update of Secondary Indexes

Please refer to the following example first:

[Example Code](https://github.com/learnforpractice/ascdk-book/tree/master/examples/secondaryindex)
```ts
import {
    Name,
    Table,
    U128,
    U256,
    printString,
    printHex,
    check,
    Contract,
    print,
} from "asm-chain";

@table("mydata")
class MyData extends Table {
    constructor(
        public a: u64=0,
        public b: u64=0,
        public c: U128=new U128()
    ) {
        super();
    }

    @primary
    get getPrimary(): u64 {
        return this.a;
    }

    @secondary
    get bvalue(): u64 {
        return this.b;
    }

    @secondary
    set bvalue(value: u64) {
        this.b = value;
    }

    @secondary
    get cvalue(): U128 {
        return this.c;
    }

    @secondary
    set cvalue(value: U128) {
        this.c = value;
    }
}

@contract
class MyContract extends Contract{
    @action("test")
    testSecondary(): void {
        let mi = MyData.new(this.receiver);

        let value = new MyData(1, 2, new U128(3));
        mi.store(value, this.receiver);

        value = new MyData(11, 22, new U128(33));
        mi.store(value, this.receiver);

        value = new MyData(111, 222, new U128(333));
        mi.store(value, this.receiver);


        let idx = mi.bvalueDB;    
        let idxIt = idx.find(2);
        printString(`+++++++++idx64.find: ${idxIt.i}, ${idxIt.primary}\n`);
        check(idxIt.primary == 1, "bad value");

        let ret = idx.lowerBound(2);
        check(ret.primary == 1, "bad value");

        ret = idx.upperBound(22);
        check(ret.primary == 111, "bad value");
    }

    @action("testupdate")
    testSecondaryUpdate(): void {
        let mi = MyData.new(this.receiver);
        let idx = mi.bvalueDB;
        let idxIt = idx.find(222);
        check(idxIt.isOk(), "value 222 not found");
        check(idxIt.primary == 111, "bad primary value");
        mi.updateBvalue(idxIt, 223, this.receiver);
        let ret = idx.find(22);
        check(ret.isOk(), "bad scondary value");
    }

    @action("testremove")
    testSecondaryRemove(): void {
        let table = MyData.new(this.receiver);
        let idx = table.bvalueDB;
        let idxIt = idx.find(222);
        check(idxIt.isOk(), "value 222 not found");
        check(idxIt.primary == 111, "bad primary value");
        let primaryIt = table.find(idxIt.primary);
        check(primaryIt.isOk(), "bad primary value");
        table.remove(primaryIt);
    }

}
```

In this example, two secondary indexes are defined:

```ts
@secondary
get bvalue(): u64 {
    return this.b;
}

@secondary
set bvalue(value: u64) {
    this.b = value;
}

@secondary
get cvalue(): U128 {
    return this.c;
}

@secondary
set cvalue(value: U128) {
    this.c = value;
}
```

- The `test` action calls the `store` method to store three sets of data and demonstrates how to use `lowerBound` to find the secondary index.
- The `testupdate` action demonstrates how to call the generated `updateBvalue` method to update data of the secondary index. `updateBvalue` is a generated method, the pattern is `update` + method name of the secondary index.

Test code:

```python
@chain_test
def test_secondary_update(tester: ChainTester):
    deploy_contract(tester, 'secondaryindex')
    args = {}
    r = tester.push_action('hello', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()

    r = tester.push_action('hello', 'testupdate', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

Compile:

```bash
cd examples/secondaryindex
yarn
yarn build
```

Run the test:

```bash
ipyeos -m pytest -s -x test.py -k test_secondary_update
```

Output:
```
INFO     test:test.py:85 {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
INFO     test:test.py:92 {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 223, 'c': '333'}], 'more': False, 'next_key': ''}
```

From the output of:

```
{'a': 111, 'b': 223, 'c': '333'}
```

We can see that 222 has been changed to 223, and other values remain the same.

## Deletion of Secondary Indexes

```ts
@action("testremove")
testSecondaryRemove(): void {
    let table = MyData.new(this.receiver);
    let idx = table.bvalueDB;
    let idxIt = idx.find(222);
    check(idxIt.isOk(), "value 222 not found");
    check(idxIt.primary == 111, "bad primary value");
    let primaryIt = table.find(idxIt.primary);
    check(primaryIt.isOk(), "bad primary value");
    table.remove(primaryIt);
}
```

Explanation of the above code:

- `let idxIt = idx.find(222);` Search for secondary index.
- `let primaryIt = table.find(idxIt.primary);` Get the primary index through `idxIt.primary`, and then return the `PrimaryIterator` of the primary index.
- `table.remove(primaryIt)` Deletes the element in the table, including the primary index and all secondary indexes.

From the above example, we can see that the deletion of a secondary index first involves finding the primary index through the secondary index and then deleting all data including the secondary index through the primary index.

Test code:

```python
@chain_test
def test_secondary_remove(tester: ChainTester):
    deploy_contract(tester, 'secondaryindex')
    args = {}
    r = tester.push_action('hello', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
    ret = tester.get_table_rows(True, 'hello', '', 'mydata', '', '', 10)
    logger.info(ret)

    r = tester.push_action('hello', 'testremove', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
    ret = tester.get_table_rows(True, 'hello', '', 'mydata', '', '', 10)
    logger.info(ret)
```

Compile:

```bash
cd examples/secondaryindex
yarn
yarn build
```

Run the test:

```bash
ipyeos -m pytest -s -x test.py -k test_secondary_remove
```

Output:

```
INFO     test:test.py:102 {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
INFO     test:test.py:108 {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}], 'more': False, 'next_key': ''}
```

Comparing the return values of the two get_table_rows calls, you will find that the data set `{'a': 111, 'b': 222, 'c': '333'}` has been deleted.

## Using API for Secondary Index Query on Tables

The above example defined two secondary indexes, the types of which are `u64` and `u128` respectively. The `get_table_rows` API also supports finding corresponding values through secondary indexes.

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

**Note** that when querying `c`, since the type is `u128`, for values beyond the range of `u64` type, hexadecimal can be used to represent data, such as `0x14d` in the above code, which is `333` in decimal.

Run the test case:

```bash
ipyeos -m pytest -s -x test.py -k test_offchain_find
```

The running result of the above test code is as follows:

```
INFO     test:test.py:117 +++++++rows: {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
INFO     test:test.py:120 +++++++rows: {'rows': [{'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
INFO     test:test.py:123 +++++++rows: {'rows': [{'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
```

## Summary

The data storage function in EOS is quite comprehensive, and the feature of secondary index tables makes data lookup very flexible. This chapter has detailed the addition, deletion, modification, and query of database tables. There is a lot of content in this chapter, so it will take some time to digest. You can try to make some modifications based on the example and try running it to increase your understanding of the knowledge points in this chapter.

[Example code 1](https://github.com/learnforpractice/ascdk-book/tree/master/examples/counter)
[Example code 2](https://github.com/learnforpractice/ascdk-book/tree/master/examples/secondaryindex)
