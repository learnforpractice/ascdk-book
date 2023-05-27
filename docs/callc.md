---
comments: true
---

# Calling C/C++ Code from Rust

This chapter will introduce how to call C/C++ code from Rust. You can get the complete example code from the link below:

[Complete code](https://github.com/learnforpractice/rscdk-book/tree/master/examples/testcallcpp)

First, use the following command to install `eoscdt` for compiling C or C++ code:

```bash
python3 -m pip install -U eoscdt
```

If your platform is Windows or MacOSX M1/M2, you can also download an image that includes the `eoscdt` tool:

```bash
docker pull ghcr.io/uuosio/scdk:latest
```

The `scdk` Docker image already includes the following tools:

```
ipyeos
gscdk
pscdk
eoscdt
pyeoskit
```

Then, run `bash` in Docker with the following command:

```bash
docker run --entrypoint bash -it --rm -v "$(pwd)":/develop -t ghcr.io/uuosio/scdk
```

Next, execute the following command in bash to compile the `say_hello` library.


Below, we will compile the `say_hello` function as an example to show how to call code from C/C++ in Rust:

First, take a look at the code in [say_hello/say_hello.cpp](https://github.com/learnforpractice/rscdk-book/blob/master/examples/testcallcpp/say_hello/say_hello.cpp):

```cpp
#include <stdint.h>
#include <eosio/eosio.hpp>

using namespace eosio;

extern "C" void say_hello(const char *name, size_t size) {
    print("hello ", std::string(name, size));
}
```

Here, note that since it is a C++ file, you need to add `extern "C"` before the function. Otherwise, there will be an error in the linking process because the function cannot be found.

Then, look at the key content in [say_hello/CMakeLists.txt](https://github.com/learnforpractice/rscdk-book/blob/master/examples/testcallcpp/say_hello/CMakeLists.txt):

```cmake
add_library(say_hello
    say_hello.cpp
)

target_include_directories(say_hello PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}
)
```

This compiles the `say_hello` related code into a library called `say_hello`.

Next, compile this library:

```bash
mkdir -p say_hello/build
cd say_hello/build
cmake -DCMAKE_TOOLCHAIN_FILE=`cdt-get-dir`/CDTWasmToolchain.cmake ..
make
```

This will generate the `libsay_hello.a` library file in the `say_hello/build` directory.

Next, take a look at the code in [lib.rs](https://github.com/learnforpractice/rscdk-book/blob/master/examples/testcallcpp/lib.rs):

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

Here:

```rs
extern "C" {
    fn say_hello(name: *const u8, size: usize);
}
```

It declares a `say_hello` function defined in C/C++. Note that the type corresponding to the `const char *` type in the C++ code is `*const u8`.

The following code shows how to call the `say_hello` function:

```rs
#[chain(action="test")]
pub fn test(&self, name: String) {
    unsafe {
        say_hello(name.as_ptr(), name.len());
    }
}
```

Since it's a C function, it must be called within an `unsafe` block.

Next, let's see how to link the `libsay_hello.a` library.

Create a new `build.rs` file and add the following content:

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

In the above code, the following code specifies that when the target system for compilation is wasm32, it links `libsay_hello.a`:

```rust
println!("cargo:rustc-link-search=./say_hello/build");
println!("cargo:rustc-link-lib=static=say_hello");  
```

The following code links the `libc++.a` library, and the code path is obtained by running the `cdt-get-root-dir` command:

```rust
println!("cargo:rustc-link-search={}/{}", stdout.trim(), "lib");
println!("cargo:rustc-link-lib=static=c++");
```

Finally, run the following command to generate the contract file:

```bash
rust-contract build
```

Then run the following command to perform tests:

```bash
ipyeos -m pytest -s -x test.py -k test_hello
```

# Appendix:

Rust type and C type comparison table:

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
