---
comments: true
---

# 数据库的操作

链上数据存储和读取是智能合约的一个重要功能。EOS链实现了一个内存数据库，支持以表的方式来存储数据，其中，每一个表的每一项数据都有唯一的主索引，称之为primary key，类型为uint64，表中存储的原始数据为任意长度的二进制数据，在智能合约调用存储数据的功能时，会将类的数据序列化后存进表中，在读取的时候又会通过反序列化的方式将原始数据转成类对象。并且还支持uint64, Uint128, Uint256, Float64, Float128类型的二重索引表，可以把二重索引表看作数据长度固定的特殊的表。主索引表和二重索引表可以配合起来使用，以实现多重索引的功能。二重索引表可以有多个。二重索引表的值是可以重复的，但是主索引表的主索引必须是唯一的。

下面结合示例来讲解下EOS的链上的内存数据库的使用。

## Store

存储功能是数据库最简单的功能了，下面的代码即演示了该功能。

[db_example1](https://github.com/learnforpractice/gscdk-book/tree/master/examples/db_example1)

```go
package main

import (
	"github.com/uuosio/chain"
)

// table mytable
type A struct {
	a uint64 //primary
	b string
}

// contract test
type MyContract struct {
	Receiver      chain.Name
	FirstReceiver chain.Name
	Action        chain.Name
}

func NewContract(receiver, firstReceiver, action chain.Name) *MyContract {
	return &MyContract{receiver, firstReceiver, action}
}

// action teststore
func (c *MyContract) TestStore(name string) {
	code := c.Receiver
	payer := c.Receiver
	mytable := NewATable(code)
	data := &A{123, "hello, world"}
	mytable.Store(data, payer)
}
```

解释一下上面的代码：

- `// table mytable`这行注释指引编译器生成表相关的代码，如NewATable即是生成的代码，生成的代码保存在generated.go这个文件里。
- `// contract test`这行注释表示`MyContract`是一个智能合约类，同样会指引编译器生成额外的代码
- `// action teststore`表示`TestStore`方法是一个`action`，会通过包含在Transaction中的Action结构来触发
- `NewATable(code)`指定创建一个表，表保存在`code`指定的账号里，在这个测试例子里是`hello`这个账号。
- `mytable.Store(data, payer)`这行代码即将数据保存到链上的数据库中。其中的payer用于指定哪个账号支付RAM资源，并且需要在Transaction中已经用账号的`active`权限签名。

编译：

```bash
cd examples/db_example1
go-contract build .
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

注意在这个示例中，如果表中已经存在以`123`类型为`uint64`的主索引的数据，那么该函数会抛出异常。

如将上面的测试用例修改成下面的代码：

```python
def test_example1():
    t = init_db_test('db_example1')
    ret = t.push_action('hello', 'teststore', "", {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])

    # will raise exception
    ret = t.push_action('hello', 'teststore', "", {'hello': 'active'})
    t.produce_block()
```

用同样的命令运行测试，在第二次调用`push_action`时，该函数就会抛出像下面的异常：

```
could not insert object, most likely a uniqueness constraint was violated
```

为了不抛出异常，在要更新表中的数据时，则要用到`Update`方法。
在调用`Store`之前要先对表中是否存在主索引进行判断，如果已经存在，则不能调用`Store`方法，而必须调用`Update`方法。
以下的示例展示了用法：
                                                                                                    
## Find/Update

这一节演示了数据库的查找和更新功能。

```go
// db_example1

// action testupdate
func (c *MyContract) TestUpdate() {
	code := c.Receiver
	payer := c.Receiver
	mytable := NewATable(code)
	it, data := mytable.GetByKey(123)
	chain.Check(it.IsOk(), "bad key")
	chain.Println("+++++++old value:", data.b)
	data.b = "goodbye world"
	mytable.Update(it, data, payer)
	chain.Println("done!")
}

```

以下为测试代码：

```python
@chain_test
def test_update(tester):
    deploy_contract(tester, 'db_example1')

    r = tester.push_action('hello', 'teststore', b'', {'hello': 'active'})
    tester.produce_block()

    r = tester.push_action('hello', 'testupdate', b'', {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

编译：

```bash
cd examples/db_example1
go-contract build .
```

用下面的命令来运行测试代码：

```bash
ipyeos -m pytest -s -x test.py -k test_update
```

在调用:

```python
r = tester.push_action('hello', 'testupdate', b'', {'hello': 'active'})
```

会输出：

```
+++++++old value: hello, world
```

可以看出，上面的代码稍微有点复杂，首先要调用`GetByKey`获取`Iterator`和存储的值，通过`it.IsOk()`判断和主索引对应的值存不存在，再调用`Update`更新数据。其中的payer用于指定哪个账号支持RAM资源，并且需要在Transaction中已经用账号的`active`权限签名。需要注意的是，在更新的过程中，**主索引的值是不能变的**，否则会抛出异常。

可以试着将update的代码修改成：

```go
data.a = 1
data.b = "goodbye world"
```

你将会看到到智能合约里抛出的有如下指示的异常:

```
mi.Update: Can not change primary key during update
```
                                                                                                    
## Remove

下面的代码演示了如何去删除数据库中的一项数据。

```go
// db_example1
// action testremove
func (c *MyContract) TestRemove() {
	code := c.Receiver
	mytable := NewATable(code)
	it := mytable.Find(123)
	chain.Check(it.IsOk(), "key 123 does not exists!")

	mytable.Remove(it)

	it = mytable.Find(123)
	chain.Check(!it.IsOk(), "something went wrong")
	chain.Println("+++++done!")
}
```

上面的代码先调用`mytable.Find(123)`方法来查找指定的数据，然后再调用`Remove`删除，调用`it.IsOk()`以检查指定的索引所在的数据存不存在。

**注意：**

这里的`Remove`并不需要调用`Store`或者`Update`所指定的payer账号的权限即可删除数据，所以，在实际的应用中，需要通过调用`chain.RequireAuth`来确保指定账号的权限才可以删除数据，例如：
```go
	chain.RequireAuth(chain.NewName("hello"))
```

测试代码：

```python
@chain_test
def test_remove(tester):
    deploy_contract(tester, 'db_example1')

    r = tester.push_action('hello', 'teststore', b'', {'hello': 'active'})
    tester.produce_block()

    r = tester.push_action('hello', 'testremove', b'', {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

编译：

```bash
cd examples/db_example1
go-contract build .
```

测试：

```bash
ipyeos -m pytest -s -x test.py -k test_remove
```
                                                                                                    
## Lowerbound/Upperbound

这两个方法也是用来查找表中的元素的，不同于`find`方法，这两个函数用于模糊查找。其中，`lowerbound`方法返回`>=`指定`id`的`Iterator`，`upperbound`方法返回`>`指定`id`的`Iterator`，下面来看下用法：

```go
// examples/db_example1

// action testbound
func (c *MyContract) TestBound() {
	code := c.Receiver
	payer := c.Receiver

	mytable := NewATable(code)
	mytable.Store(&A{1, "1"}, payer)
	mytable.Store(&A{2, "2"}, payer)
	mytable.Store(&A{5, "3"}, payer)

	it := mytable.Lowerbound(1)
	chain.Check(it.IsOk() && it.GetPrimary() == 1, "bad Lowerbound value")

	it = mytable.Upperbound(2)
	chain.Check(it.IsOk() && it.GetPrimary() == 5, "bad Upperbound value")
}
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
++++testbound done!
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
    deploy_contract(tester, 'db_example1')

    r = tester.push_action('hello', 'testbound', b'', {'hello': 'active'})
    tester.produce_block()

    r = tester.get_table_rows(False, 'hello', '', 'mytable', '', '', 10)
    logger.info("+++++++rows: %s", r)

    r = tester.get_table_rows(True, 'hello', '', 'mytable', '', '', 10)
    logger.info("+++++++rows: %s", r)

    r = tester.get_table_rows(True, 'hello', '', 'mytable', '1', '2', 10)
    logger.info("+++++++rows: %s", r)
```

输出：

```
+++++++rows: {'rows': ['01000000000000000131', '02000000000000000132', '05000000000000000133'], 'more': False, 'next_key': ''}
+++++++rows: {'rows': [{'a': 1, 'b': '1'}, {'a': 2, 'b': '2'}, {'a': 5, 'b': '3'}], 'more': False, 'next_key': ''}
+++++++rows: {'rows': [{'a': 1, 'b': '1'}, {'a': 2, 'b': '2'}], 'more': False, 'next_key': ''}
```
                                                                                                    
## 二重索引的操作

请先看下面的例子：

[db_example2](https://github.com/learnforpractice/gscdk-book/tree/master/examples/db_example2)

```go
// db_example2
package main

import (
	"github.com/uuosio/chain"
)

// table mytable
type A struct {
	a uint64        //primary
	b uint64        //secondary
	c chain.Uint128 //secondary
	d string
}

// contract db_example2
type MyContract struct {
	Receiver      chain.Name
	FirstReceiver chain.Name
	Action        chain.Name
}

func NewContract(receiver, firstReceiver, action chain.Name) *MyContract {
	return &MyContract{receiver, firstReceiver, action}
}

// action teststore
func (c *MyContract) TestStore() {
	code := c.Receiver
	payer := c.Receiver
	mytable := NewATable(code)
	data := &A{1, 2, chain.NewUint128(3, 0), "1"}
	mytable.Store(data, payer)
	chain.Println("++++++++teststore done!")
}
```

在这个例子中，定义了两个二重索引：

```go
b uint64        //secondary
c chain.Uint128 //secondary
```

测试代码：

```python
# test.py
@chain_test
def test_store(tester):
    deploy_contract(tester, 'db_example2')
    r = tester.push_action('hello', 'teststore', b'', {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

编译：

```bash
cd examples/db_example2
go-contract build .
```

运行测试：

```bash
ipyeos -m pytest -s -x test.py -k test_store
```

总结：对比主索引的例子，如果一个表中包含二重索引，那么存储所调用的方法是一样的，都是调用的`Store`这个方法

                                                                                                    
## 二重索引的的更新

在实际的应用中，有时候需要更新二重索引。请先看下面的代码

```go
// db_example2

// action testupdate
func (c *MyContract) TestUpdate() {
	code := c.Receiver
	payer := c.Receiver
	mytable := NewATable(code)

    idxb := mytable.GetIdxTableByb()
	secondaryIt := idxb.Find(2)
	chain.Check(secondaryIt.IsOk(), "secondary index 2 not found")
	mytable.Updateb(secondaryIt, 3, payer)

	secondaryIt = idxb.Find(3)
	chain.Check(secondaryIt.IsOk() && secondaryIt.Primary == 1, "secondary index 3 not found")
	chain.Println("+++++++test update done!")
}
```

注意上面代码中的这段代码：

```go
idxb := mytable.GetIdxTableByb()
secondaryIt := idxb.Find(2)
chain.Check(secondaryIt.IsOk(), "secondary index 2 not found")
mytable.Updateb(secondaryIt, 3, payer)

secondaryIt = idxb.Find(3)
chain.Check(secondaryIt.IsOk() && secondaryIt.Primary == 1, "secondary index 3 not found")
chain.Println("+++++++test update done!")
```

简述下过程：

- `idxb := mytable.GetIdxTableByb()` 获取`b`的二重索引，`GetIdxTableByb`是一个自动生成的函数，代码可以在`generated.go`中找到
- `secondaryIt := idxb.Find(2)`查找二重索引的类型为`uint64`的值`2`，返回的值`secondaryIt`为`SecondaryIterator`类型
- **`mytable.Updateb(secondaryIt, uint64(3), payer)`** 这行代码即是实现了更新的功能，更新`b`的值为`3`，`Updateb`为自动生成的函数，定义在`generated.go`中
- `secondaryIt = idxb.Find(3)`查找新的二重索引
- `chain.Check(secondaryIt.IsOk() && secondaryIt.Primary == 1, "secondary index 3 not found")` 用于确认二重索引是否更新成功，注意，这里还会判断主索引是否为`1`
                                                                                                    
## 二重索引的查询

二重索引同样支持通过`Find`, `Lowerbound`, `Upperbound`的方式来查询表，以下是示例，在这个示例中，显示了如何查询`b`, `c`这两个二重索引中的值

```go
// action testbound
func (c *MyContract) TestBound() {
	code := c.Receiver
	payer := c.Receiver

	mytable := NewATable(code)
	data := &A{1, 2, chain.NewUint128(3, 0), "1"}
	mytable.Store(data, payer)
	data = &A{11, 22, chain.NewUint128(33, 0), "11"}
	mytable.Store(data, payer)
	data = &A{111, 222, chain.NewUint128(333, 0), "111"}
	mytable.Store(data, payer)

	{
		idxb := mytable.GetIdxTableByb()
		secondaryIt := idxb.Find(2)
		chain.Check(secondaryIt.IsOk() && secondaryIt.Primary == 1, "key 2 not found")
	}

	{
		idxb := mytable.GetIdxTableByb()
		secondaryIt, secondaryValue := idxb.Lowerbound(2)
		chain.Check(secondaryIt.IsOk() && secondaryIt.Primary == 1 && secondaryValue == 2, "bad Lowerbound value")

		secondaryIt, secondaryValue = idxb.Upperbound(22)
		chain.Check(secondaryIt.IsOk() && secondaryIt.Primary == 111 && secondaryValue == 222, "bad Upperbound value")
	}

	{
		idxc := mytable.GetIdxTableByc()
		secondaryIt, secondaryValue := idxc.Lowerbound(chain.NewUint128(3, 0))
		chain.Check(secondaryIt.IsOk() && secondaryIt.Primary == 1 && secondaryValue == chain.NewUint128(3, 0), "bad Lowerbound value")

		secondaryIt, secondaryValue = idxc.Upperbound(chain.NewUint128(33, 0))
		chain.Check(secondaryIt.IsOk() && secondaryIt.Primary == 111 && secondaryValue == chain.NewUint128(333, 0), "bad Upperbound value")
		chain.Println("++++testbound done!")
	}
}
```

测试的例子如下：

```python
@chain_test
def test_bound(tester):
    deploy_contract(tester, 'db_example2')

    r = tester.push_action('hello', 'testbound', b'', {'hello': 'active'})
    tester.produce_block()
```

在示例的目录下用下面的命令来运行测试用例：

```bash
ipyeos -m pytest -s -x test.py -k test_bound
```

## 二重索引的删除

```go
// action testremove
func (c *MyContract) TestRemove() {
	code := c.Receiver
	mytable := NewATable(code)

	idxb := mytable.GetIdxTableByb()
	secondaryIt := idxb.Find(2)

	it := mytable.Find(secondaryIt.Primary)
	chain.Check(it.IsOk(), "key does not exists!")

	mytable.Remove(it)

	secondaryIt = idxb.Find(2)
	chain.Check(!secondaryIt.IsOk(), "something went wrong")
	chain.Println("+++++testremove done!")
}
```

解释一下上面的代码：

- `secondaryIt := idxb.Find(2)` 查找二重索引
- `it := mytable.Find(secondaryIt.Primary)` 通过`SecondaryIterator`获取主索引，再通过主索引返回庆索引的`Iterator`
- `mytable.Remove(it)` 删除表中的元素，包含主索引和所有二重索引

从上面的例子中可以看出，二重索引的删除是先通过二重索引找到主索引：，再通过主索引来删除的


```python
# test.py
@chain_test
def test_remove(tester):
    deploy_contract(tester, 'db_example2')

    r = tester.push_action('hello', 'teststore', b'', {'hello': 'active'})
    tester.produce_block()

    r = tester.push_action('hello', 'testremove', b'', {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```


编译：

```bash
cd examples/db_example2
go-contract build .
```

运行测试：

```bash
ipyeos -m pytest -s -x test.py -k test_remove
```

## 利用API来对表进行二重索引查询

在例子`db_example2`中，定义了两个二重索引，类型分别为`uint64`,`chain.Uint128`，`get_table_rows`API还支持通过二重索引来查找对应的值

```python
@chain_test
def test_offchain_find(tester):
    deploy_contract(tester, 'db_example2')

    r = tester.push_action('hello', 'testbound', b'', {'hello': 'active'})
    tester.produce_block()

    r = tester.get_table_rows(True, 'hello', '', 'mytable', '1', '', 10, key_type="i64", index_position="2")
    logger.info("+++++++rows: %s", r)

    r = tester.get_table_rows(True, 'hello', '', 'mytable', '3', '', 10, key_type="i128", index_position="3")
    logger.info("+++++++rows: %s", r)
```

运行测试用例：

```bash
ipyeos -m pytest -s -x test.py -k test_offchain_find
```

上面的测试代码的运行结果如下：

```
{'rows': [{'a': 1, 'b': 2, 'c': '3', 'd': '1'}, {'a': 11, 'b': 22, 'c': '33', 'd': '11'}, {'a': 111, 'b': 222, 'c': '333', 'd': '111'}], 'more': False, 'next_key': ''}
{'rows': [{'a': 1, 'b': 2, 'c': '3', 'd': '1'}, {'a': 11, 'b': 22, 'c': '33', 'd': '11'}, {'a': 111, 'b': 222, 'c': '333', 'd': '111'}], 'more': False, 'next_key': ''}
```
                                                                                                    

## 总结

EOS中的数据存储功能是比较完善的，并且有二重索引表的功能，使数据的查找变得非常的灵活。本章详细讲解了数据库表的增，删，改，查的代码。本章的内容较多，需要花点时间好好消化。可以在示例的基础上作些改动，并且尝试运行以增加对这章知识点的理解。
