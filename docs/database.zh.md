---
comments: true
---

# 数据库的操作

链上数据存储和读取是智能合约的一个重要功能。EOS链实现了一个内存数据库，支持以表的方式来存储数据，其中，每一个表的每一项数据都有唯一的主索引，称之为primary key，类型为uint64，表中存储的原始数据为任意长度的二进制数据，在智能合约调用存储数据的功能时，会将类的数据序列化后存进表中，在读取的时候又会通过反序列化的方式将原始数据转成类对象。并且还支持uint64, Uint128, Uint256, Float64, Float128类型的二重索引表，可以把二重索引表看作数据长度固定的特殊的表。主索引表和二重索引表可以配合起来使用，以实现多重索引的功能。二重索引表可以有多个。二重索引表的值是可以重复的，但是主索引表的主索引必须是唯一的。

下面结合示例来讲解下EOS的链上的内存数据库的使用。

## store/find/update

存储，查找，更新三个功能是数据库最基本的功能了，下面的代码演示了如何通过这三个功能进行链上的计数。

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

解释一下上面的代码：

- `#[chain(primary)]`指定了一个主索引成员变量为account, 类型为`Name`，需要注意的是，如果主索引为非`u64`类型，则需要实现`rust_chain::db::PrimaryValueInterface`trait的`get_primary`方法，如`Name`结构实现的代码如下：
```rust
impl PrimaryValueInterface for Name {
    fn get_primary(&self) -> u64 {
        return self.value();
    }
}
```
- `counter`模块通过`#[rust_chain::contract]`引用了`rust_chain`这个包中的`contract`宏，这个宏会根据模块中的`chain`属性来生成额外的数据库操作相关和处理action相关的代码。
- `#[chain(table="counter")]`这行代码利用了`chain`属性来定义一个表，表的名称是`counter`，是一个`name`结构，`table`关键字指引编译器生成表相关的代码，生成的代码会对`rust-chain`代码中的`MultiIndex`结构相关的代码进行封装，以方便开发者进行调用
- `#[chain(action = "inc")]`表示`inc_count`方法是一个`action`，会通过包含在Transaction中的Action结构来触发
- `Counter::new_table(self.receiver)`指定创建一个表，`self.receiver`指定的是当前合约的账号名称，表示表是存储在当前合约账号。
- `let it = db.find(account.value());`用于查找主索引为`1`所在的值，返回的值是`Iterator`类型
- `let Some(mut value) = it.get_value()`用于获取`Iterator`中的值，如果值不存在，则调用`db.store(&value, payer);`来保存一个新值到数据库中，否则将count加1后调用`db.update(&it, &value, payer);`来更新数据库中的数据。其中的payer用于指定哪个账号支付RAM资源，并且需要在Transaction中已经用该账号的`active`权限签名。

编译：

```bash
cd examples/counter
rust-contract build
```

测试：

```bash
ipyeos -m pytest -s -x test.py -k test_counter
```

运行的测试代码如下：

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
                                                                                                    
## Remove

下面的代码演示了如何去删除数据库中的一项数据。

```rust
#[chain(action = "testremove")]
pub fn test_remove(&self, account: Name) {
    let db = Counter::new_table(self.receiver);
    let it = db.find(account.value());
    check(it.is_ok(), "key not found");
    db.remove(&it);
}
```

上面的代码先调用`let it = db.find(account.value());`方法来查找指定的数据，然后再调用`remove`删除，调用`it.is_ok()`以检查指定的索引所在的数据存不存在。

**注意：**

这里的`remove`并不需要调用`store`或者`update`所指定的`payer`账号的权限即可删除数据，所以，在实际的应用中，需要通过调用`rust_chain.require_auth`来确保指定账号的权限才可以删除数据，例如：

```rust
require_auth(name!("hello"))
```

测试代码：

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

这里，先调用`inc`这个action来保证数据库中有存储数据，然后调用`testremove`来删除指定的数据，并且通过`get_table_rows`来确定数据是否已经添加或者被修改或者被删除，相关的`get_table_rows`的用法将在下面介绍。

编译：

```bash
cd examples/counter
rust-contract build .
```

测试：

```bash
ipyeos -m pytest -s -x test.py -k test_remove
```
输出：

```
NFO     test:test.py:90 +++++++++table rows: {'rows': [{'account': 'alice', 'count': 1}], 'more': False, 
INFO     test:test.py:95 +++++++++table rows: {'rows': [{'account': 'alice', 'count': 2}], 'more': False, 'next_key': ''}
INFO     test:test.py:100 +++++++++table rows: {'rows': [], 'more': False, 'next_key': ''}
```
                                                                                                    
## lower_bound/upper_bound

这两个方法也是用来查找表中的元素的，不同于`find`方法，这两个函数用于模糊查找。其中，`lower_bound`方法返回`>=`指定`id`第一个元素的`Iterator`，`upper_bound`方法返回`>`指定`id`的第一个元素的`Iterator`，下面来看下用法：

```rust

```

测试代码：

```python
@chain_test
def test_bound(tester):
    deploy_contract(tester, 'db_example1')

    r = tester.push_action('hello', 'testbound', b'', {'hello': 'active'})
    tester.produce_block()
```

编译：

```bash
cd examples/db_example1
go-contract build .
```

运行测试：

```bash
ipyeos -m pytest -s -x test.py -k test_bound
```

输出：

```
+++++db.lower_bound(1) return primary key: 1
+++++db.upper_bound(3) return primary key: 5
```
                                                                                                    
## 利用API来对表进行查询

上面的例子都是讲的如何通过智能合约来操作链上的数据库的表，实际上，通过EOS提供的链下的`get_table_rows`的API的接口，也同样可以对链上的表进行查询工作。在`ipyeos`的`ChainTester`这个类中和`pyeoskit`的`ChainApiAsync`和`ChainApi`这两个类，都提供了`get_table_rows`接口，以方便对表进行查询操作

在Python代码中，`get_table_rows`的定义如下

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

解释下这个接口的参数：

- `_json`: True 返回json格式的数据，False返回16进制表示的原始数据
- `code`： 表所在的账号，
- `scope`： 一般设置为空字符串，当有相同的`code`，`table`时，不同的`scope`可以用来区别不同的表
- `table`： 要查询的数据表名
- `lower_bound`：查询起始主键，字符串类型或者数值类型，字符串类型时可以表示一个`name`类型，如果是以`0x`开头的十六进制字符串，则表示一个数值类型,为空时表示从起始位置开始查询
- `upper_bound`：查询结束主键，字符串类型或者数值类型，字符串类型时可以表示一个`name`类型，如果是以`0x`开头的十六进制字符串，则表示一个数值类型, 为空时表示没有设置上限，将返回>=`lower_bound`的所有值
- `limit`：用于限制返回的值的个数
- `key_type`：用于指定索引的类型,默认为64位的无符号整数类型
- `index_position`：用于指定索引的相对位置，为空或者为`1`表示主索引，从`2`以上表示二重索引的位置
- `reverse`：指定是否要返回倒序表示的数据
- `show_payer`：指定是否显示RAM资源的付费账号

要通过`get_table_rows`来查询表，表的结构必须在ABI的描述中可见，在`db_example1`这个例子中，生成的`test.abi`中，包含如下信息即是对表的描述：

```json
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

测试代码：

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

运行测试代码:

```bash
ipyeos -m pytest -s -x test.py -k test_offchain_find
```

输出：

```
INFO     test:test.py:118 +++++++rows: {'rows': ['01000000000000000100000000000000', '03000000000000000100000000000000', '05000000000000000100000000000000'], 'more': False, 'next_key': ''}
INFO     test:test.py:121 +++++++rows: {'rows': [{'account': '............1', 'count': 1}, {'account': '............3', 'count': 1}, {'account': '............5', 'count': 1}], 'more': False, 'next_key': ''}
INFO     test:test.py:124 +++++++rows: {'rows': [{'account': '............1', 'count': 1}, {'account': '............3', 'count': 1}], 'more': False, 'next_key': ''}
```

注意，这里的`account`由于是`name`结构，会将数值转换成字符串，所以输出看起来比较奇怪。
                                                                                                    
## 二重索引的存储，查询和更新

请先看下面的例子：

[示例代码](https://github.com/learnforpractice/rscdk-book/tree/master/examples/secondaryindex)

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

            let data = &MyData{a: 1, b: 2, c: 3};
            db.store(&data, self.receiver);

            let data = &MyData{a: 11, b: 22, c: 33};
            db.store(&data, self.receiver);

            let data = &MyData{a: 111, b: 222, c: 333};
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

在这个例子中，定义了两个二重索引：

```rust
#[chain(secondary)]
b: u64,
#[chain(secondary)]
c: u128,
```

`test1`action调用`store`方法存储了3组数据。
`test2`action演示了调用二重索引的`lower_bound`来查找二重索引，以及调用`update_b`这个生成的方法来更新二重索引的数据

测试代码：

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

编译：

```bash
cd examples/secondaryindex
rust-contract build
```

运行测试：

```bash
ipyeos -m pytest -s -x test.py -k test_secondary
```

输出：
```
INFO     test:test.py:78 +++++++rows: {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
[(hello,test2)->hello]: CONSOLE OUTPUT BEGIN =====================
+++b: 222
++++primary value 111 secondary value: 222

[(hello,test2)->hello]: CONSOLE OUTPUT END   =====================
INFO     test:test.py:86 +++++++rows: {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 223, 'c': '333'}], 'more': False, 'next_key': ''}
```

从输出中的:

```
{'a': 111, 'b': 223, 'c': '333'}
```

可以知道222已经被改成223了，其它的值保持不变
                   
## 二重索引的删除

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

解释一下上面的代码：

- `let it = idx.find(b);` 查找二重索引
- `let primary_it = db.find(it.primary);` 通过`it.primary`获取主索引，再通过主索引返回主索引的`Iterator`
- `db.remove(&primary_it);` 删除表中的元素，包含主索引和所有二重索引

从上面的例子中可以看出，二重索引的删除是先通过二重索引找到主索引：，再通过主索引来删除的

测试代码：

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

编译：

```bash
cd examples/secondaryindex
go-contract build .
```

运行测试：

```bash
ipyeos -m pytest -s -x test.py -k test_remove
```

输出：

```
INFO     test:test.py:96 +++++++rows: {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
[(hello,test3)->hello]: CONSOLE OUTPUT BEGIN =====================
+++b: 222
[(hello,test3)->hello]: CONSOLE OUTPUT END   =====================
INFO     test:test.py:104 +++++++rows: {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}], 'more': False, 'next_key': ''}
```

对比两次get_table_rows的返回值，会发现`{'a': 111, 'b': 222, 'c': '333'}`这组数据被删除了

## 利用API来对表进行二重索引查询

在上面的例子中定义了两个二重索引，类型分别为`u64`,`u128`，`get_table_rows`API还支持通过二重索引来查找对应的值

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

**注意**, 在查询`c`的时候，由于类型是`u128`，对于超出`u64`类型的范围时，可以用十六进制来表示数据，例如上面的`0x14d`的十进制数据为`333`

运行测试用例：

```bash
ipyeos -m pytest -s -x test.py -k test_offchain_find
```

上面的测试代码的运行结果如下：

```
INFO     test:test.py:113 +++++++rows: {'rows': [{'a': 1, 'b': 2, 'c': '3'}, {'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
INFO     test:test.py:116 +++++++rows: {'rows': [{'a': 11, 'b': 22, 'c': '33'}, {'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
INFO     test:test.py:119 +++++++rows: {'rows': [{'a': 111, 'b': 222, 'c': '333'}], 'more': False, 'next_key': ''}
```

## 总结

EOS中的数据存储功能是比较完善的，并且有二重索引表的功能，使数据的查找变得非常的灵活。本章详细讲解了数据库表的增，删，改，查的代码。本章的内容较多，需要花点时间好好消化。可以在示例的基础上作些改动，并且尝试运行以增加对这章知识点的理解。

[示例代码1](https://github.com/learnforpractice/rscdk-book/tree/master/examples/counter)
[示例代码2](https://github.com/learnforpractice/rscdk-book/tree/master/examples/secondaryindex)
