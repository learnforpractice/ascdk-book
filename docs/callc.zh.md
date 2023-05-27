---
comments: true
---

# Rust代码里调用C/C++代码

这一章介绍下如何在Rust代码中调用C/C++的代码，完整的示例代码可以从下面的链接中获取：

[完整代码](https://github.com/learnforpractice/rscdk-book/tree/master/examples/testcallcpp)

首先，用下面的命令安装`eoscdt`用于编译c或者是c++代码:

```bash
python3 -m pip install -U eoscdt
```

如果你的平台是 Windows 或 MacOSX M1/M2，你也可以下载一个包含ipyeos工具的镜像

```bash
docker pull ghcr.io/uuosio/scdk:latest
```

在`scdk`这个docker镜像中，已经包含了如下的工具：

```
ipyeos
gscdk
pscdk
eoscdt
pyeoskit
```

然后通过下面的命令在docker中运行`bash`：

```bash
docker run --entrypoint bash -it --rm -v "$(pwd)":/develop -t ghcr.io/uuosio/scdk
```

再在bash中执行接下来的编译`say_hello`库的命令。

下面以编译`say_hello`函数为例，演示如何从Rust代码中调用C/C++中的代码：

先看[say_hello/say_hello.cpp](https://github.com/learnforpractice/rscdk-book/blob/master/examples/testcallcpp/say_hello/say_hello.cpp)中的代码：

```cpp
#include <stdint.h>
#include <eosio/eosio.hpp>

using namespace eosio;

extern "C" void say_hello(const char *name, size_t size) {
    print("hello ", std::string(name, size));
}
```

这里需要注意的是，因为是C++文件，则需在函数前面加上`extern "C"`，否则会在链接过程中因找不到函数而出错。

再看[say_hello/CMakeLists.txt](https://github.com/learnforpractice/rscdk-book/blob/master/examples/testcallcpp/say_hello/CMakeLists.txt)中的关键内容：

```cmake
add_library(say_hello
    say_hello.cpp
)

target_include_directories(say_hello PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}
)
```

这里把`say_hello`相关的代码编译成一个`say_hello`这个库。

接下来编译这个库：

```bash
mkdir -p say_hello/build
cd say_hello/build
cmake -DCMAKE_TOOLCHAIN_FILE=`cdt-get-dir`/CDTWasmToolchain.cmake ..
make
```

会在`say_hello/build`目录下生成`libsay_hello.a`这个库文件。

接下来看下[lib.rs](https://github.com/learnforpractice/rscdk-book/blob/master/examples/testcallcpp/lib.rs)中的代码：

```rs
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(warnings))]

#[rust_chain::contract]
mod hello {
    extern "C" {
        fn say_hello(name: *const u8, size: usize);
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
                say_hello(name.as_ptr(), name.len());
            }
        }
    }
}
```

其中：

```rs
extern "C" {
    fn say_hello(name: *const u8, size: usize);
}
```

声明一个定义在C/C++中的`say_hello`这个函数，注意这里与C++代码中的`const char *`类型对应的类型为`*const u8`

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
use std::process::Command;

fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")
        .unwrap_or_else(|_| "unknown".to_string());
    println!("Target architecture: {}", target_arch);

    if target_arch == "wasm32" {
        let output = Command::new("cdt-get-root-dir")
        .output()
        .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();
        println!("{}", stdout);

        println!("cargo:rustc-link-search=./say_hello/build");
        println!("cargo:rustc-link-lib=static=say_hello");

        println!("cargo:rustc-link-search={}/{}", stdout.trim(), "lib");
        println!("cargo:rustc-link-lib=static=c++");
    }
}
```

在上面的代码里，如下的代码即是指定在编译的目标系统为wasm32时，链接`libsay_hello.a`

```rust
println!("cargo:rustc-link-search=./say_hello/build");
println!("cargo:rustc-link-lib=static=say_hello");  
```

如下的代码是链接`libc++.a`这个库，代码的路径是通过运行`cdt-get-root-dir`这个命令来获取的：

```rust
println!("cargo:rustc-link-search={}/{}", stdout.trim(), "lib");
println!("cargo:rustc-link-lib=static=c++");
```

最后，运行下面的命令来生成合约文件：

```bash
rust-contract build
```

再运行下面的命令进行测试：

```bash
ipyeos -m pytest -s -x test.py -k test_hello
```

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

