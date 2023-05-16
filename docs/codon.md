---
comments: true
---

# Differences between Codon and standard Python

Python is generally referred to as a dynamic language that generates relatively high-level virtual machine instructions (byte code) at compile time. When Python programs are executed, they are still interpreted by a specialized virtual machine program. The advantage is that the code is very flexible and supports dynamic modification of the code, but the disadvantage is that the execution speed is slow. The earliest Python smart contract was executed in this way. Later, in order to solve the problem of slow Python code execution speed, many solutions emerged, including tools like Codon that directly compile Python code into machine code that can be executed directly. The Python smart contract later adopted the Codon compiler, which not only preserves the ease of use of writing Python code, but also ensures execution speed. However, due to the different compilation methods, the generated target files are different, and the execution method is different from that of standard Python programs, so there are some differences in compatibility between Codon compiled Python code and standard Python code. Let's introduce these differences below to avoid confusion when writing smart contracts.

## Support for method overloading in classes

For example:

```python
class Action(object):
...
    def __init__(self, account: Name, name: Name, data: bytes=bytes()):
        ...

    def __init__(self, account: Name, name: Name, permission_account: Name, data: bytes=bytes()):
        ...
```

Note that module functions do not currently support function overloading.

## The `int` type in Codon is a 64-bit signed integer

This is different from infinite integers in Python. One reason is that Codon is a static type language, so the code is compiled directly into machine code (binary code), rather than virtual machine instructions (byte code). Another reason is for efficiency reasons.

## Numeric types in Codon

In addition to the `int` type, Codon also has these numeric types: `byte`, `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `u64`, `u128`, `u256`, which are used to represent 64-bit unsigned integers, 128-bit unsigned integers, and 256-bit unsigned integers respectively.

To convert other data types into `byte`, you can use methods like the ones below:

```python
a = byte(123)
b = byte(int(123i8))
c = byte(int(123u8))
d = byte(int(123i32))
e = byte(int(123u32))
f = byte(int(123u64))
```

## String type

Codon currently uses ASCII strings, unlike Python's unicode strings.

## Dictionary type

The dictionary type in Codon does not maintain the insertion order, unlike Python, which has supported this since version 3.6.

## Optional type

The Codon compiler provides support for the Optional type.

```python
class A:
    value: u64
    def __init__(self, value):
        self.value = value
a = Optional(A(123u64))
```

To check if the Optional type is None:

```python
if a is None:
    do_something()
```

To get the value in an Optional type:

```python
from internal.types.optional import unwrap
a1 = unwrap(a)
```

There is an even simpler way:

```python
a1: A = a
```

Here, during compilation, the compiler checks if the type is Optional, and if so, calls unwrap(a), which throws an exception if a is None.

Methods or member variables in an Optional class can be accessed directly:

```python
print(a.value)
```

In fact, it is equivalent to:

```python
print(unwrap(a).value)
```

## Union type

Union type is a built-in type in Codon. In Python, there is no built-in support for Union, so the usage is different. In Codon, the data structure of Union is not defined in Codon's code and is implemented internally by the compiler, but can be represented by the following class definition:

```python
class Union:
    tag: byte
    value: object
```

Usage:

```python
a = Union[int, str](123)
print(a == 123)
```

Output: True

If the type is incorrect, an exception will be thrown directly, as shown in the following code:

```python
a = Union[int, str](123)
print(a == "ABC")
```

Note that in Codon, if the types in the union are only different in their order, they are considered the same type.

The following code outputs: True

```python
a = Union[int, str](123)
b = Union[str, int]("abc")
print(isinstance(a, Union[str, int]))
```

## Conclusion

Finally, it is necessary to explain that the executable file generated after compiling the smart contract code in Python is in WebAssembly instructions, with a file extension of .wasm. In reality, there is no CPU that can execute WebAssembly instructions directly, and in most cases, they also need to be interpreted by a specialized virtual machine program during execution, but only binary code is executed. This is similar to Python's virtual machine executing byte code, but WebAssembly's instructions are more low-level than Python's virtual machine instructions, and are safer, so they can be easily translated into system machine code to optimize execution speed by JIT or AOT. In the EOS blockchain, smart contract code can be executed by `eosvm` using JIT, or by interpreting the instructions, or even by compiling into machine code that can be directly executed by the system. Python's virtual machine instructions are difficult to execute completely by JIT and AOT.

In conclusion, Python smart contracts use a static compiler like Codon, sacrificing some compatibility with standard Python code for the benefit of significantly improved execution speed and security.
