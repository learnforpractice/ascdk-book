---
comments: true
---

# Database Operations

On-chain data storage and retrieval is a critical feature of smart contracts. EOS implements an in-memory database that allows data to be stored in a table format. Each item in a table has a unique primary index, called a primary key, which is of type `u64`. The raw data stored in the table can be binary data of any length. When the storage function of a smart contract is called, the data is serialized and stored in the table. When reading, the stored data is deserialized back as a class object. Additionally, EOS supports secondary index tables of types `u64`, `u128`, `u256`, `double`, and `long double`, which can be considered special tables with a fixed data length. Primary and secondary index tables can be used together to implement multiple indexes. There can be multiple secondary index tables, and the values in these tables can be repeated, but the primary index in the primary index table must be unique.

The following example demonstrates the usage of EOS's in-memory database.

## Store

Storage is the simplest function of the database, and the following code demonstrates this functionality.

```python
# db_example1.codon

from chain.database import primary
from chain.contract import Contract

@table("mytable")
class A(object):
    a: primary[u64]
    b: str
    def __init__(self, a: u64, b: str):
        self.a = primary[u64](a)
        self.b = b

@contract(main=True)
class MyContract(Contract):

    def __init__(self):
        super().__init__()

    @action('teststore')
    def test_store(self):
        print('db_test')
        item = A(123u64, 'hello, world')
        table = A.new_table(n'hello', n'')
        table.store(item, n'hello')
```

Compile:

```
python-contract build db_example/db_example1.codon
```

```bash
ipyeos -m pytest -s -x test.py -k test_store
```

The test code you are running is as follows:

```python
def test_store():
    t = init_db_test('db_example1')
    ret = t.push_action('hello', 'teststore', "", {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

**Note**:

In this example, if there is already data with the primary key `123u64` in the table, an exception will be thrown when the function is called.

To modify the above test case to the following code:

```python
def test_example1():
    t = init_db_test('db_example1')
    ret = t.push_action('hello', 'test', "", {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])

    # will raise exception
    ret = t.push_action('hello', 'test', "", {'hello': 'active'})
    t.produce_block()
```

When running the test with the same command, if the `push_action` is called for the second time, the function will throw an exception similar to the following:

```
could not insert object, most likely a uniqueness constraint was violated
```

To avoid such exceptions, the `update` method must be used when updating data in the table. Before calling `store`, it is necessary to check whether the primary index already exists in the table. If it does, `store` method cannot be called, and `update` method must be used instead. The following example demonstrates how to use it:
                                                                                                    
## find/update

This section demonstrates the lookup and update functionality of the database.

```python
# db_example1.codon

...

@contract(main=True)
class MyContract(Contract):

...

    @action('testupdate')
    def test_update(self, value: str):
        print('db_test')
        table = A.new_table(n'hello', n'')
        key = 123u64
        it = table.find(key)
        if it.is_ok():
            print('+++++update value:', value)
            item = A(key, value)
            table.update(it, item, n'hello')
        else:
            print('+++++store value:', value)
            item = A(key, value)
            table.store(item, n'hello')
```

The following is the test code:

```python
def test_update():
    t = init_db_test('db_example1')
    ret = t.push_action('hello', 'testupdate', {'value': 'hello, bob'}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])

    ret = t.push_action('hello', 'testupdate', {'value': 'hello, alice'}, {'hello': 'active'})
    t.produce_block()
```

Compile using the following command:

```
python-contract build db_example/db_example1.codon
```

Execute the test code with the following command:

```bash
ipyeos -m pytest -s -x test.py -k test_update
```

When calling
```python
t.push_action('hello', 'testupdate', {'value': 'hello, bob'}, {'hello': 'active'})
```

it will output:

```
+++++store value: hello, bob
```

When calling `testupdate` action again:
```python
t.push_action('hello', 'testupdate', {'value': 'hello, alice'}, {'hello': 'active'})
```

it will output:

```
+++++update value: hello, alice
```

As you can see, the above code is a bit complicated. First, it needs to call `find` to determine whether the value corresponding to the primary index exists, and then decide whether to call `store` or `update`. It should be noted that, during the update process, **the value of the primary index cannot be changed**, otherwise an exception will be thrown.

You can try to modify the update code to:

```python
item = A(key+1u64, value)
table.update(it, item, n'hello')
```

You will see an exception thrown in the smart contract.
                                                                                                    
## Remove

The following code demonstrates how to remove an item from the database.

```python
# db_example/db_example1.codon

@action('testremove')
def test_remove(self):
    print('test remove')
    item = A(123u64, 'hello, world')
    table = A.new_table(n'hello', n'')
    table.store(item, n'hello')

    it = table.find(123u64)
    assert it.is_ok()
    table.remove(it)

    it = table.find(123u64)
    assert not it.is_ok()
```

Test code:

```python
def test_remove():
    t = init_db_test('db_example1')
    ret = t.push_action('hello', 'testremove', "", {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

Compile using the following command:

```
python-contract build db_example/db_example1.codon
```

Test using the following command:

```bash
ipyeos -m pytest -s -x test.py -k test_remove
```

The above code first calls the `store` method to store the data with index `123u64` in the database, then calls `remove` to delete it, and uses `assert` to check the result. If everything is normal, the program will not throw any exceptions.

## Lowerbound/Upperbound

These two methods are also used to search for elements in the database. Unlike the `find` method, these two functions are used for fuzzy searching. Among them, the `lowerbound` method returns an `Iterator` whose `id` is `>=` the specified `id`, and the `upperbound` method returns an `Iterator` whose `id` is `>` the specified `id`. Let's take a look at the usage below:


```python
# db_example/db_example1.codon

...

@contract(main=True)
class MyContract(Contract):

...

    @action('testbound')
    def test_bound(self):
        print('db_test')
        table = A.new_table(n'hello', n'')
        payer = n'hello'

        value = A(1u64, "alice")
        table.store(value, payer)

        value = A(3u64, "bob")
        table.store(value, payer)

        value = A(5u64, "john")
        table.store(value, payer)

        it = table.lowerbound(1u64)
        value2: A = it.get_value()
        print("+++++:", value2.a, value2.b)
        assert value2.a == 1u64 and value2.b == 'alice'

        it = table.upperbound(1u64)
        value2: A = it.get_value()
        print("+++++:", value2.a, value2.b)
        assert value2.a == 3u64 and value2.b == 'bob'
```

Test code:

```python
def test_bound():
    t = init_db_test('db_example4')
    ret = t.push_action('hello', 'testbound', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

Compile using the following command:

```
python-contract build db_example/db_example1.codon
```

Run the test using the following command:

```bash
ipyeos -m pytest -s -x test.py -k test_bound
```

Output:

```
+++++: 1 alice
+++++: 3 bob
```

## Querying the Primary Index of a Table Using API

The above examples are all about how to operate the database table on the chain through the smart contract. In fact, by using the `get_table_rows` API provided by EOS off the chain, you can also query the table on the chain.

In the test code, the definition of `get_table_rows` is as follows:

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

First of all, to query a table using `get_table_rows`, the structure of the table must be visible in the ABI description. You can use the following code to describe the table in the corresponding generated ABI file:

```python
# db_example5.codon

from chain.database import primary
from chain.contract import Contract

@table("mytable")
class A(object):
    a: primary[u64]
    b: str
    def __init__(self, a: u64, b: str):
        self.a = primary[u64](a)
        self.b = b

@contract(main=True)
class MyContract(Contract):

    def __init__(self):
        super().__init__()

    @action('test')
    def test(self):
        print('db_test')
        table = A.new_table(n'hello', n'')
        payer = n'hello'

        value = A(1u64, "alice")
        table.store(value, payer)

        value = A(3u64, "bob")
        table.store(value, payer)

        value = A(5u64, "john")
        table.store(value, payer)

        it = table.lowerbound(1u64)
        value2: A = it.get_value()
        print("+++++:", value2.a, value2.b)
        assert value2.a() == 1u64 and value2.b == 'alice'

        it = table.upperbound(1u64)
        value2: A = it.get_value()
        print("+++++:", value2.a, value2.b)
        assert value2.a() == 3u64 and value2.b == 'bob'
```

Here, the `table` decorator is used to make the compiler include the structure of the table in the ABI.

After adding this `table` to the class, the compiler will automatically add the `get_primary` and `new_table` functions to the class.

At the same time, the member variables of the class must also meet certain requirements: first, a primary index variable must be declared, and the type must be `database.primary`. The implementation of the `primary` class is as follows:

```python
class primary[T](object):
    value: T
    def __init__(self, value: T):
        self.value = value

    def get_primary(self) -> u64:
        if isinstance(self.value, u64):
            return self.value
        return self.value.get_primary()

    def __pack__(self, enc: Encoder):
        self.value.__pack__(enc)

    def __unpack__(dec: Decoder) -> primary[T]:
        return primary[T](T.__unpack__(dec))

    def __call__(self) -> T:
        return self.value

    def __size__(self) -> int:
        return self.value.__size__()
```

The `primary` class is a template class. If the type of the `value` in `primary` is not of type `u64`, then the type must implement the `get_primary` method. The `primary` class also has a `__call__` method to facilitate access to `value`. In the following discussion of multiple indexes, binary indexes will also be used. The type of the binary index must be `database.secondary`.

Compilation:
```bash
python-contract build db_example/db_example5.codon
```

You will see the following description in the generated `db_example5.abi:`
```bash
"tables": [
        {
            "name": "mytable",
            "type": "A",
            "index_type": "i64",
            "key_names": [],
            "key_types": []
        }
    ]
```

Now consider the test code:
```python
def test_example5():
    t = init_db_test('db_example5')
    ret = t.push_action('hello', 'test', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
    rows = t.get_table_rows(True, 'hello', '', 'mytable', 1, '', 10)
    logger.info('++++++=rows: %s', rows)
```

Run the test:
```bash
ipyeos -m pytest -s -x test.py -k test_example5
```

Output:
```
++++++=rows: {'rows': [{'a': 1, 'b': 'alice'}, {'a': 3, 'b': 'bob'}, {'a': 5, 'b': 'john'}], 'more': False, 'next_key': ''}
```

## Operation of the binary index

First, consider the following example:

```python
# db_example7.codon

from chain.contract import Contract
from chain.database import primary, secondary
from chain.database import IdxTable64, IdxTable128, Iterator
from chain.name import Name

@table("mytable")
class A(object):
    a: database.primary[u64]
    b: secondary[u64]
    c: secondary[u128]

    def __init__(self, a: u64, b: u64, c: u128):
        self.a = primary[u64](a)
        self.b = secondary[u64](b)
        self.c = secondary[u128](c)

@contract(main=True)
class MyContract(Contract):

    def __init__(self):
        super().__init__()

    @action('test')
    def test(self):
        payer = n"hello"
        table = A.new_table(n"hello", n"")
        item = A(1u64, 2u64, 3u128)
        table.store(item, payer)

        idx_table_b = table.get_idx_table_by_b()
        it = idx_table_b.find(2u64)
        print("++++++it.primary:", it.primary)
        assert it.primary == 1u64

        idx_table_c = table.get_idx_table_by_c()
        it = idx_table_c.find(3u128)
        print("++++++it.primary:", it.primary)
        assert it.primary == 1u64
```

In this example, two binary indexes are defined:

```python
b: secondary[u64]
c: secondary[u128]
```

In the code, `get_idx_table_by_b` and `get_idx_table_by_c` are used to obtain the tables of the binary indexes, and the returned object types are `IdxTable64` and `IdxTable128`, respectively. The tables of binary indexes have similar method names as the tables of the primary index, and can also perform the function of binary index lookup.

Test code:

```python
# test.py
def test_example7():
    t = init_db_test('db_example7')
    ret = t.push_action('hello', 'test', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

Compile:

```
python-contract build db_example/db_example7.codon
```

Run the test:

```bash
ipyeos -m pytest -s -x test.py -k test_example7
```

Output:
```
++++++it.primary: 1
++++++it.primary: 1
```

## Updating the binary index

In practical applications, sometimes it is necessary to update the binary index. Please first look at the following code:

```python
# db_example8.codon
from chain.contract import Contract
from chain.database import primary, secondary
from chain.database import IdxTable64, IdxTable128, Iterator
from chain.name import Name

@table("mytable")
class A(object):
    a: database.primary[u64]
    b: secondary[u64]
    c: secondary[u128]

    def __init__(self, a: u64, b: u64, c: u128):
        self.a = primary[u64](a)
        self.b = secondary[u64](b)
        self.c = secondary[u128](c)

@contract(main=True)
class MyContract(Contract):

    def __init__(self):
        super().__init__()

    @action('test')
    def test(self):
        payer = n"hello"
        table = A.new_table(n"hello", n"")
        item = A(1u64, 2u64, 3u128)
        table.store(item, payer)
        item = A(111u64, 222u64, 333u128)
        table.store(item, payer)

        idx_table_b = table.get_idx_table_by_b()
        it_sec = idx_table_b.find(2u64)
        print("++++++it.primary:", it_sec.primary)
        assert it_sec.primary == 1u64
        
        table.update_b(it_sec, 22u64, payer)

        it_sec = idx_table_b.find(22u64)
        assert it_sec.is_ok()
        print("++++++it.primary:", it_sec.primary)
        assert it_sec.primary == 1u64
```

Note the following code in the above code:

```python
idx_table_b = table.get_idx_table_by_b()
it_sec = idx_table_b.find(2u64)
print("++++++it.primary:", it_sec.primary)
assert it_sec.primary == 1u64

table.update_b(it_sec, 22u64, payer)

it_sec = idx_table_b.find(22u64)
assert it_sec.is_ok()
print("++++++it.primary:", it_sec.primary)
assert it_sec.primary == 1u64
```

Brief description of the process:

- `it_sec = idx_table_b.find(2u64)`: Looks up the value `2u64` in the binary index and returns the `SecondaryIterator` type result `it_sec`.
- `table.update_b(it_sec, 22u64, payer)`: This line of code implements the update function and updates the value of `b` to `22u64`.
- `it_sec = idx_table_b.find(22u64)`: Looks up the new binary index.
- `assert assert it_sec.is_ok()`: Used to confirm whether the binary index has been updated successfully.
- `assert it_sec.primary == 1u64`: Used to confirm whether the primary index is correct.

The `update_b` code is generated by the compiler and is shown below:

```python
def update_b(self, it: SecondaryIterator, b: u64, payer: Name) -> None:
    # 更新`b`的二级索引
    self.idx_b.update(it, b, payer)
    # 查找主索引
    it_primary = self.table.find(it.primary)
    check(it_primary.is_ok(), "primary iterator not found")
    # 获取主索引对应的值
    value: A = it_primary.get_value()
    # 更新主索引对应的值
    value.b = secondary[u64](b)
    self.table.update(it_primary, value, payer)
```

From the code, it is apparent that when updating the binary index, the corresponding value in the primary index will also be updated.

## Deleting the secondary index

```python
@action('testremove')
def test_remove(self):
    payer = n"hello"
    table = A.new_table(n"hello", n"")
    item = A(1u64, 2u64, 3u128)
    table.store(item, payer)

    idx_table_b = table.get_idx_table_by_b()
    it_sec = idx_table_b.find(2u64)
    assert it_sec.primary == 1u64
    it = table.find(it_sec.primary)
    table.remove(it)

    it_sec = idx_table_b.find(2u64)
    assert not it_sec.is_ok()
    print('done!')
```

In this example, first call `store` to store an object A with the primary index of `1u64` and the first secondary index value of `2u64`. Then query `2u64` and confirm that `it_sec.primary == 1u64`. Next, call `remove` to delete the data with the primary index of `1u64`. Finally, query `2u64` again and confirm that the element has been deleted.

```python
# test.py
def test_remove_secondary():
    t = init_db_test('db_example8')
    ret = t.push_action('hello', 'testremove', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```


Compilation:

```bash
python-contract build db_example/db_example8.codon
```

Running the test:

```bash
ipyeos -m pytest -s -x test.py -k test_remove_secondary
```

## Using the API to perform secondary index queries on a table

In the `db_example8.codon` example, two binary indexes are defined, with the types `u64` and `u128`, respectively. The `get_table_rows` API also supports finding corresponding values through binary indexes.

```python
def test_example9():
    t = init_db_test('db_example8')
    ret = t.push_action('hello', 'test', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])

    # find by secondary u64
    rows = t.get_table_rows(True, 'hello', '', 'mytable', 22, '', 10, 'i64', '2')
    logger.info("++++++++++%s", rows['rows'])
    assert rows['rows'][0]['b'] == 22

    # find by secondary u128
    rows = t.get_table_rows(True, 'hello', '', 'mytable', '3', '', 10, 'i128', '3')
    logger.info("++++++++++%s", rows['rows'])
    assert rows['rows'][0]['c'] == '3'
```

Explanation of the code below:

To find the value in the table through the second index `b`:

```python
rows = t.get_table_rows(True, 'hello', '', 'mytable', 22, '', 10, 'i64', '2')
```

Here, `i64` is the index type of `b`, and `2` is zero-based index corresponding to the index.

To find the value in the table through the second index `c`:

```python
rows = t.get_table_rows(True, 'hello', '', 'mytable', '3', '', 10, 'i128', '3')
```

Here, `i128` is the index type of `c`. Note that the value `3` in the `lowerbound` parameter is the value of the binary index. Since `u128` has exceeded the range of 64-bit integers, a numeric string is used to represent it. Finally, the last parameter `3` is the corresponding index number.

The results of running the above test code are as follows:

```
++++++++++[{'a': 1, 'b': 22, 'c': '3'}, {'a': 111, 'b': 222, 'c': '333'}]
++++++++++[{'a': 1, 'b': 22, 'c': '3'}, {'a': 111, 'b': 222, 'c': '333'}]
```

## Implementation principles of the database

The above code demonstrates the basic operations of the database. However, during the compilation process, some methods and classes are generated by the compiler. The following code displays the code generated by the compiler.

```python
# db_example6.codon
from chain.contract import Contract
from chain.database import primary, secondary
from chain.database import IdxTable64, IdxTable128, Iterator
from chain.mi import MultiIndexBase
from chain.name import Name

@packer
class A(object):
    a: database.primary[u64]
    b: secondary[u64]
    c: secondary[u128]

    def __init__(self, a: u64, b: u64, c: u128):
        self.a = primary[u64](a)
        self.b = secondary[u64](b)
        self.c = secondary[u128](c)

    def get_primary(self) -> u64:
        return self.a()

class MultiIndexA(MultiIndexBase[A]):
    idx_b: IdxTable64
    idx_c: IdxTable128

    def __init__(self, code: Name, scope: Name, table: Name):
        MultiIndexBase[A].__init__(code, scope, table)
        idx_table_base = table.value & 0xfffffffffffffff0u64
        self.idx_b = IdxTable64(0, code, scope, Name(idx_table_base | u64(0)))
        self.idx_c = IdxTable128(1, code, scope, Name(idx_table_base | u64(1)))

    def store(self, item: A, payer: Name) -> Iterator[A]:
        id: u64 = item.get_primary()
        it = self.table.store(item, payer)
        self.idx_b.store(id, item.b(), payer)

        self.idx_c.store(id, item.c(), payer)

        return it

    def update(self, it: Iterator[A], item: A, payer: Name):
        self.table.update(it, item, payer)

        primary = item.get_primary()
        secondary = item.b()
        it_secondary, old_secondary = self.idx_b.find_by_primary(primary)
        if not secondary == old_secondary:
            self.idx_b.update(it_secondary, secondary, payer)

        secondary = item.c()
        it_secondary, old_secondary = self.idx_c.find_by_primary(primary)
        if not secondary == old_secondary:
            self.idx_c.update(it_secondary, secondary, payer)

    def remove(self, it: Iterator[A]):
        sec_it, _ = self.idx_b.find_by_primary(it.get_primary())
        self.idx_b.remove(sec_it)

        sec_it, _ = self.idx_c.find_by_primary(it.get_primary())
        self.idx_c.remove(sec_it)

        self.table.remove(it)

    def remove(self, primary: u64):
        it = self.table.find(primary)
        if it.is_ok():
            self.remove(it)

    def get_idx_table_by_b(self) -> IdxTable64:
        return self.idx_b

    def get_idx_table_by_c(self) -> IdxTable128:
        return self.idx_c

    def update_b(self, it: SecondaryIterator, b: u64, payer: Name) -> None:
        self.idx_b.update(it, b, payer)
        it_primary = self.table.find(it.primary)
        check(it_primary.is_ok(), "primary iterator not found")
        value: A = it_primary.get_value()
        value.b = secondary[u64](b)
        self.table.update(it_primary, value, payer)

    def update_c(self, it: SecondaryIterator, c: u128, payer: Name) -> None:
        self.idx_c.update(it, c, payer)
        it_primary = self.table.find(it.primary)
        check(it_primary.is_ok(), "primary iterator not found")
        value: A = it_primary.get_value()
        value.c = secondary[u128](c)
        self.table.update(it_primary, value, payer)

@extend
class A:
    def new_table(code: Name, scope: Name):
        return MultiIndexA(code, scope, n"mytable")

@contract(main=True)
class MyContract(Contract):

    def __init__(self):
        super().__init__()

    @action('test')
    def test(self):
        payer = n"hello"
        table = A.new_table(n"hello", n"")
        item = A(1u64, 2u64, 3u128)
        table.store(item, payer)

        idx_table_b = table.get_idx_table_by_b()
        it = idx_table_b.find(2u64)
        print("++++++it.primary:", it.primary)
        assert it.primary == 1u64

        idx_table_c = table.get_idx_table_by_c()
        it = idx_table_c.find(3u128)
        print("++++++it.primary:", it.primary)
        assert it.primary == 1u64
```

This example demonstrates a scenario where there are two binary indexes. Only the name `table` was changed to `packer`. In this case, the compiler will not generate any code related to the database.

By comparison, it is apparent that the compiler generates a class named `MultiIndexA`, which inherits from the `MultiIndexBase` class defined in `mi.codon`. This class has the following methods:

- def store(self, item: A, payer: Name) -> Iterator[A]
- def update(self, it: Iterator[A], item: A, payer: Name)
- def remove(self, it: Iterator[A])
- def get_idx_table_by_b(self) -> IdxTable64:
- def get_idx_table_by_c(self) -> IdxTable128:
- def update_b(self, it: SecondaryIterator, b: u64, payer: Name) -> None:
- def update_c(self, it: SecondaryIterator, c: u128, payer: Name) -> None:

In addition, the class `A` is generated, along with the following additional methods:

- `get_primary`: retrieves the primary index
- `get_idx_table_by_b`: retrieves the table indexed by `b`, returning an instance of the `IdxTable64` class
- `get_idx_table_by_c`: retrieves the table indexed by `c`, returning an instance of the `IdxTable128` class
- `new_table`

Test code:

```python
def test_example6():
    t = init_db_test('db_example6')
    ret = t.push_action('hello', 'test', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

Compilation:

```
python-contract build db_example/db_example6.codon
```

Running the test:

```bash
ipyeos -m pytest -s -x test.py -k test_example6
```

Output:

```
++++++it.primary: 1
++++++it.primary: 1
```

## Summary

The data storage functionality in EOS is relatively comprehensive, and the second-level index table function makes data querying very flexible. This chapter provides a detailed explanation of the code for table operations, including adding, deleting, modifying, and querying. This chapter contains a lot of content and requires some time to digest. You can try to modify the examples and run them to gain a better understanding of the content covered in this chapter.
