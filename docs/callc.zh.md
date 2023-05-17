---
comments: true
---

# Rust代码里调用C/C++代码

首先，用下面的命令安装`eoscdt`用于编译c或者是c++代码:

```bash
python3 -m pip install -U eoscdt
```

## 用命令行工具将C/C++代码编译成库并链接进Rust代码

下面以编译`say_hello`函数为例，演示如何编译代码：

如果源文件是c代码，例如：

say_hello.c

```c
void prints(const char *s);

void say_hello(const char *s) {
	prints(s);
}
```

则用下面的命令编译：

```bash
cdt-cc -c -o say_hello.o say_hello.c
```


如果源文件是c++代码，例如：

say_hello.cpp

```cpp
extern "C" void prints(const char *s);

extern "C" void say_hello(const char *s) {
	prints(s);
}
```

则用下面的命令编译：

```bash
cdt-cpp -c -o say_hello.o say_hello.cpp
```

这里需要注意的是，如果是C++文件，则需在函数前面加上`extern "C"`，否则会在下面的链接过程中出错。

编译成功后，把`.o`文件打包成以`.a`结尾的库文件：

```bash
cdt-ar rcs libsay_hello.a say_hello.o
```


接下来看下如何在rust代码中使用`say_hello`这个函数：

lib.rs

```rs
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(warnings))]

#[rust_chain::contract]
mod hello {
    extern "C" {
        fn say_hello(name: *const u8);
    }

    use rust_chain::{
        Name,
    };

    #[chain(main)]
    pub struct Hello {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl Hello {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action="test")]
        pub fn test(&self, name: String) {
            unsafe {
                say_hello(name.as_ptr());
            }
        }
    }
}
```

其中：

```rs
extern "C" {
    fn say_hello(name: *const u8);
}
```

声明一个定义在C中的函数，注意这里与C++代码中的`const char *`类型对应的类型为`*const u8`

下面的代码展示了如何调用`say_hello`这个函数：

```rs
#[chain(action="test")]
pub fn test(&self, name: String) {
    unsafe {
        say_hello(name.as_ptr());
    }
}
```

由于是一个C函数，所以必须在`unsafe`代码块里进行调用

下面来看如何链接`libsay_hello.a`这个库

新建文件`build.rs`,并且添加下面的内容：

```rs
use std::env;

fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")
        .unwrap_or_else(|_| "unknown".to_string());
    println!("Target architecture: {}", target_arch);

    if target_arch == "wasm32" {
        println!("cargo:rustc-link-search=./");
        println!("cargo:rustc-link-lib=static=say_hello");    
    }
}
```

在上面的代码里，如下的代码即是指定在编译的目标系统为wasm32时，链接`libsay_hello.a`

```rust
println!("cargo:rustc-link-search=./");
println!("cargo:rustc-link-lib=static=say_hello");    
```

完整的示例代码可以从下面的链接中找到：
[示例代码链接1](https://github.com/uuosio/rscdk/tree/main/tests/testcallcpp)

## 用Cmake工具编译C++代码为库文件并链接进Rust代码

当然，也可以用cmake来编译C++代码，并生成库文件，可以从下面的链接中找到示例：

[示例代码链接2](https://github.com/uuosio/rscdk/tree/main/tests/testcallcpp2)

其中的`build.sh`中的内容如下：

```bash
mkdir -p say_hello/build
pushd say_hello/build
cmake -DCMAKE_TOOLCHAIN_FILE=`cdt-get-dir`/CDTWasmToolchain.cmake ..
make
popd
rust-contract build
```

将C++代码编译成库文件的代码如下：

```bash
cmake -DCMAKE_TOOLCHAIN_FILE=`cdt-get-dir`/CDTWasmToolchain.cmake ..
```

这里需要指定CDT工具链文件来将C++代码编译成wasm32的库文件

运行`./build.sh`来编译C++代码和Rust代码

运行`./test.sh`来测试


# 附录:

Rust类型和C类型对照表：

| Rust                     | C                             |
|--------------------------|-------------------------------|
| bool                     | _Bool                         |
| char                     | char                          |
| i8                       | int8_t                        |
| i16                      | int16_t                       |
| i32                      | int32_t                       |
| i64                      | int64_t                       |
| isize                    | intptr_t                      |
| u8                       | uint8_t                       |
| u16                      | uint16_t                      |
| u32                      | uint32_t                      |
| u64                      | uint64_t                      |
| usize                    | uintptr_t                     |
| f32                      | float                         |
| f64                      | double                        |
| *const T                 | const T*                      |
| *mut T                   | T*                            |
| &T                       | const T*                      |
| &mut T                   | T*                            |
