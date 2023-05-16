---
comments: true
---

# 数据库的操作

链上数据存储和读取是智能合约的一个重要功能。EOS链实现了一个内存数据库，支持以表的方式来存储数据，其中，每一个表的每一项数据都有唯一的主索引，称之为primary key，类型为u64，表中存储的原始数据为任意长度的二进制数据，在智能合约调用存储数据的功能时，会将类的数据序列化后存进表中，在读取的时候又会通过反序列化的方式将原始数据转成类对象。并且还支持u64, u128, u256, double, long double类型的二级索引表，可以把二级索引表看作数据长度固定的特殊的表。主索引表和二级索引表可以配合起来使用，以实现多重索引的功能。二级索引表可以有多个。二级索引表的值是可以重复的，但是主索引表的主索引必须是唯一的。

下面结合示例来讲解下EOS的链上的内存数据库的使用。

## store

存储功能是数据库最简单的功能了，下面的代码即演示了该功能。

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

编译：

```
python-contract build db_example/db_example1.codon
```

```bash
ipyeos -m pytest -s -x test.py -k test_store
```
运行的测试代码如下：

```python
def test_store():
    t = init_db_test('db_example1')
    ret = t.push_action('hello', 'teststore', "", {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

注意在这个示例中，如果表中已经存在以`123u64`为key的数据，那么该函数会抛出异常。

如将上面的测试用例修改成下面的代码：

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

用同样的命令运行测试，在第二次调用`push_action`时，该函数就会抛出像下面的异常：

```
could not insert object, most likely a uniqueness constraint was violated
```

为了不抛出异常，在要更新表中的数据时，则要用到`update`方法。
在调用`store`之前要先对表中是否存在主索引进行判断，如果已经存在，则不能调用`store`方法，而必须调用`update`方法。
以下的示例展示了用法：
                                                                                                    
## find/update

这一节演示了数据库的查找和更新功能。

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

以下为测试代码：

```python
def test_update():
    t = init_db_test('db_example1')
    ret = t.push_action('hello', 'testupdate', {'value': 'hello, bob'}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])

    ret = t.push_action('hello', 'testupdate', {'value': 'hello, alice'}, {'hello': 'active'})
    t.produce_block()
```

编译：

```
python-contract build db_example/db_example1.codon
```

用下面的命令来运行测试代码：

```bash
ipyeos -m pytest -s -x test.py -k test_update
```

在调用
```python
t.push_action('hello', 'testupdate', {'value': 'hello, bob'}, {'hello': 'active'})
```

会输出：

```
+++++store value: hello, bob
```

再次调用 `testupdate`：

```python
t.push_action('hello', 'testupdate', {'value': 'hello, alice'}, {'hello': 'active'})
```

会输出：

```
+++++update value: hello, alice
```

可以看出，上面的代码稍微有点复杂，首先要调用`find`判断和主索引对应的值存不存在，再决定是调用`store`还是`update`。需要注意的是，在更新的过程中，**主索引的值是不能变的**，否则会抛出异常。

可以试着将update的代码修改成：

```python
item = A(key+1u64, value)
table.update(it, item, n'hello')
```

你将会看到到智能合约里抛出的异常
                                                                                                    
## remove

下面的代码演示了如何去删除数据库中的一项数据。

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

测试代码：

```python
def test_remove():
    t = init_db_test('db_example1')
    ret = t.push_action('hello', 'testremove', "", {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

编译：

```
python-contract build db_example/db_example1.codon
```

测试：

```bash
ipyeos -m pytest -s -x test.py -k test_remove
```


上面的代码先调用`store`方法来存储索引为`123u64`的数据，然后再再调用`remove`删除，调用`assert`以检查结果。如果一切正常，程序将不会抛出任何的异常。
                                                                                                    
## lowerbound/upperbound

这两个方法也是用来查找给中的元素的，不同于`find`方法，这两个函数用于模糊查找。其中，`lowerbound`方法返回`>=`指定`id`的`Iterator`，`upperbound`方法返回`>`指定`id`的`Iterator`，下面来看下用法：

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

测试代码：
```python
def test_bound():
    t = init_db_test('db_example1)
    ret = t.push_action('hello', 'testbound', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

编译：

```
python-contract build db_example/db_example1.codon
```

运行测试：

```bash
ipyeos -m pytest -s -x test.py -k test_bound
```

输出：

```
+++++: 1 alice
+++++: 3 bob
```
                                                                                                    
## 利用API来对表进行主索引查询

上面的例子都是讲的如果通过智能合约来操作链上的数据库的表，实际上，通过EOS提供的链下的`get_table_rows`的API的接口，也同样可以对链上的表进行查询工作。
在测试代码中，get_table_rows的定义如下

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

首先，要通过`get_table_rows`来查询表，表的结构必须在ABI的描述中可见，可以通过下面的代码来让表在生成相应的ABI文件中有描述：

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

这里，通过`table`这个内置的`decorator`来让编译器在ABI中加入表的结构。
给类加了这个`table`，编译器会自动给类添加`get_primary`以及`new_table`函数：

同时，类的成员变量也要满足相应的要求：
首先，必须声明一个主索引变量，类型必须是`database.primary`, `primary`类的实现如下：

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

`primary`类是一个模版类，如果`primary`的`value`的类型不为`u64`，则类型必须实现`get_primary`方法，`primary`类还有一个`__call__`方法，以方便获取`value`。在后面会讲到的多重索引中，还会用到二级索引，二级索引的类型必须是`database.secondary`

编译：

```bash
python-contract build db_example/db_example5.codon
```

你将在生成的`db_example5.abi`中看到下面的描述：
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

再看下测试代码：

```python
def test_example5():
    t = init_db_test('db_example5')
    ret = t.push_action('hello', 'test', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
    rows = t.get_table_rows(True, 'hello', '', 'mytable', 1, '', 10)
    logger.info('++++++=rows: %s', rows)
```

运行测试：

```bash
ipyeos -m pytest -s -x test.py -k test_example5
```

输出：

```
++++++=rows: {'rows': [{'a': 1, 'b': 'alice'}, {'a': 3, 'b': 'bob'}, {'a': 5, 'b': 'john'}], 'more': False, 'next_key': ''}
```
                                                                                                    
## 二级索引的操作

请先看下面的例子：

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

在这个例子中，定义了两个二级索引：

```python
b: secondary[u64]
c: secondary[u128]
```

在代码里，通过`get_idx_table_by_b`和`get_idx_table_by_c`来获取二级索引的表，返回的对象类型分别为`IdxTable64`和`IdxTable128`。
二级索引的表和主索引的表有类似的方法名称，也可以执行二级索引的查找的功能。

测试代码：

```python
# test.py
def test_example7():
    t = init_db_test('db_example7')
    ret = t.push_action('hello', 'test', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

编译：

```
python-contract build db_example/db_example7.codon
```

运行测试：

```bash
ipyeos -m pytest -s -x test.py -k test_example7
```

输出：

```
++++++it.primary: 1
++++++it.primary: 1
```
                                                                                                    
## 二级索引的的更新

在实际的应用中，有时候需要更新二级索引。请先看下面的代码

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

注意上面代码中的这段代码：

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

简述下过程：

- `it_sec = idx_table_b.find(2u64)`查找二级索引的值`2u64`，返回的`SecondaryIterator`类型的`it_sec`。
- **`table.update_b(it_sec, 22u64, payer)`** 这行代码即是实现了更新的功能，更新`b`的值为`22u64`
- `it_sec = idx_table_b.find(22u64)`查找新的二级索引
- `assert assert it_sec.is_ok()`用于确认二级索引是否更新成功
- `assert it_sec.primary == 1u64`用于确认主索引是否正确

而`update_b`是由编译器生成的代码，生成的代码如下：

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

从代码看出，更新二级索引的时候，还会更新主索引的对应的值
                                                                                                    
## 二级索引的删除

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

在这个例子中，首先调用`store`存储主索引为`1u64`，以及第一个二级索引的值为`2u64`的对象A，然后查询`2u64`，确认`it_sec.primary == 1u64`，然后调用`remove`删除主索引为`1u64`的数据，最后再次查询`2u64`，确认元素已经被删除。

## 

```python
# test.py
def test_remove_secondary():
    t = init_db_test('db_example8')
    ret = t.push_action('hello', 'testremove', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```


编译：

```
python-contract build db_example/db_example8.codon
```

运行测试：

```bash
ipyeos -m pytest -s -x test.py -k test_remove_secondary
```

## 利用API来对表进行二重索引查询

在例子`db_example8.codon`中，定义了两个二级索引，类型分别为`u64`,`u128`，`get_table_rows`API还支持通过二级索引来查找对应的值

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

下面对代码作下解释

通过二级索引`b`来查找表中的值：

```python
rows = t.get_table_rows(True, 'hello', '', 'mytable', 22, '', 10, 'i64', '2')
```

这里的`i64`即是`b`的索引类型，`2`是索引对应的序号，注意一下这里不是从`1`开始算起的。


通过二级索引`c`来查找表中的值：

```python
rows = t.get_table_rows(True, 'hello', '', 'mytable', '3', '', 10, 'i128', '3')
```

这里的`i128`即是`c`的索引类型，注意这里lowerbound参数的值`3`是二级索引的值，由于u128已经超过了64位整数的表示范围，所以用数字字符串表示，最后一个参数`3`是索引对应的序号。


上面的测试代码的运行结果如下：

```
++++++++++[{'a': 1, 'b': 22, 'c': '3'}, {'a': 111, 'b': 222, 'c': '333'}]
++++++++++[{'a': 1, 'b': 22, 'c': '3'}, {'a': 111, 'b': 222, 'c': '333'}]
```
                                                                                                    
## 数据库的实现原理

上面的代码演示了数据库的基本操作，但是实际上在编译的过程中，有些方法和类是由编译器生成的，下面的代码把这些由编译器生成的代码展示了出来。

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

这个例子展示了有二个二级索引的例子，只是把`table`改成了`packer`，编译器即不再会生成数据库相关的代码。
对比可知，编译器在编译的时候会生成`MultiIndexA`这个继承自`mi.codon`中定义的`MultiIndexBase`类，

这个类有以下几个方法：

- def store(self, item: A, payer: Name) -> Iterator[A]
- def update(self, it: Iterator[A], item: A, payer: Name)
- def remove(self, it: Iterator[A])
- def get_idx_table_by_b(self) -> IdxTable64:
- def get_idx_table_by_c(self) -> IdxTable128:
- def update_b(self, it: SecondaryIterator, b: u64, payer: Name) -> None:
- def update_c(self, it: SecondaryIterator, c: u128, payer: Name) -> None:


同时会为类`A`生成额外的方法：

- `get_primary` 获取主索引
- `get_idx_table_by_b`, 用于获取二级索引`b`的表，返回的类是`IdxTable64`
- `get_idx_table_by_c`，用于获取二级索引`c`的表，返回的类为`IdxTable128`。
- `new_table`

测试代码：

```python
def test_example6():
    t = init_db_test('db_example6')
    ret = t.push_action('hello', 'test', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

编译：

```
python-contract build db_example/db_example6.codon
```

运行测试：

```bash
ipyeos -m pytest -s -x test.py -k test_example6
```

输出：

```
++++++it.primary: 1
++++++it.primary: 1
```

                                                                                                    
## 总结

EOS中的数据存储功能是比较完善的，并且有二级索引表的功能，使数据的查找变得非常的灵活。本章详细讲解了数据库表的增，删，改，查的代码。本章的内容较多，需要花点时间好好消化。可以在示例的基础上作些发动，并且尝试运行以增加对这章知识点的理解。
