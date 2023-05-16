---
comments: true
---

# 和标准Python的差异

## Python智能合约代码和标准Python代码的差异

一般Python都被称作一种动态语言，特点是编译时生成的是比较高级的虚拟机指令(byte code)，执行Python程序的时候是还需通过专门的虚拟机程序解释执行的。好处是代码非常灵活，支持对代码的动态修改，缺点是执行速度慢。最早的Python智能合约即是通过这种方式执行的。后来为了解决Python代码执行速度慢的问题，出现了许多解决方案，其中就有像Codon这里的将Python代码直接编译成可以直接执行的机器码(binary code)这样的工具。Python智能合约后面也采用了Codon编译器，从而在保留写Python的代码的易用性的同时，又能保证执行速度。但是由于编译的方式不同，生成的目标文件不相同，被执行的方式和标准的Python程序不同，所以用Codon编译的Python代码在在兼容性上和标准的Python代码有一些差异。下面来介绍一下这些差异，避免在写智能合约的时候产生疑惑。

## 类的方法支持重载

例如：

```python
class Action(object):
...
    def __init__(self, account: Name, name: Name, data: bytes=bytes()):
        ...

    def __init__(self, account: Name, name: Name, permission_account: Name, data: bytes=bytes()):
        ...
```

但是要注意的是，模块的函数暂时不支持函数重载

## Codon里的int类型是64位有符号整数

这和Python里的无限整数不同，一个原因是Codon是静态类型语言，代码会直接编译成机器码(binary code)，而不是虚拟机指令(byte code)。另外一个原因是出于效率的考虑。

## Codon的数值类型

除了int类型外，Codon中还有这几种数值类型：`byte`, `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `u64`, `u128`, `u256`,分别用来表示64位的无符号整数，128位的无符号整数，256位的无符号整数

如果要将其它数据类型转换成`byte`，可以用例如下面的方法：

```python
a = byte(123)
b = byte(int(123i8))
c = byte(int(123u8))
d = byte(int(123i32))
e = byte(int(123u32))
f = byte(int(123u64))
```

## 字符串类型

Codon 目前使用的是 ASCII 字符串，不像 Python 的 unicode 字符串。

## 字典类型

Codon 的字典类型不保留插入顺序，不像Python那样从3.6开始支持。

## Optional类型

Codon的编译器提供了对Optional类型的支持

```python
class A:
    value: u64
    def __init__(self, value):
        self.value = value
a = Optional(A(123u64))
```

判断Optional类型是否为None:

```python
if a is None:
    do_something()
```

获取Optional类型中的值:

```python
from internal.types.optional import unwrap
a1 = unwrap(a)
```

还有种更简单的办法：

```python
a1: A = a
```

这里，编译器在编译的时候会判断是否为Optional类型，是则会调用unwrap(a)，unwrap函数在a为None的时候会抛出异常。

Optional中的类的方法或者成员变量可以直接访问：

```python
print(a.value)
```

实际上调用的是：
```python
print(unwrap(a).value)
```

## Union类型

Union类型是Codon的内置类型，在Python中，并没有对Union提供内置类型支持，所以要注意用法的不同。
在Codon中，Union的数据结构并不定义在codon的代码里，是编译器内部实现的，但是可以用下面的类的定义来表示：
```python
class Union:
    tag: byte
    value: object
```

用法：

```python
a = Union[int, str](123)
print(a == 123)
```
输出：True

如果类型不对，则会直接抛出异常，如下面的代码会抛出异常：

```python
a = Union[int, str](123)
print(a == "ABC")
```

要注意的是，Codon里的Union，如果union里的类型只是顺序不同，那么视为同一个种类型。

如下面的代码输出为：True

```python
a = Union[int, str](123)
b = Union[str, int]("abc")
print(isinstance(a, Union[str, int]))
```

## 总结
最后，特别需要说明一下的是，编译智能合约代码后生成的可执行文件的指令是WebAssembly指令，文件的扩展名为.wasm。在现实中，并没有能直接执行WebAssembly指令的CPU存在，在执行的时候，大多数情况也是需要通过专门的虚拟机程序来解释执行的，但是执行的是binary code而已。这和Python的虚拟机执行byte code的方式类似，但是WebAssebmly的指令由于是binary code，比Python的虚拟机指令更加低级，安全性也更好，可以比较方便的翻译成系统的机器码，所以可以比较方便的以JIT或者AOT的方式先编译成所在的系统指令后再执行，从而优化执行的速度。在EOS区块链中，智能合约的代码既可以通过`eosvm`以JIT的方式来执行的，也可以通过对指令解释执行的方式来执行，甚至还可以先编译成系统可直接执行的机器码来运行。而Python的虚拟机指令则很难被完全的以JIT和AOT的方式来执行。

一句话来作下总结：Python智能合约采用Codon这种静态编译器，牺牲了一部分的和标准Python代码的兼容性为代价，以获得运行速度和安全性上的大幅的提升。
