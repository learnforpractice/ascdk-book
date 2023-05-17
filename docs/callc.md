---
comments: true
---

# Calling C/C++ Code in Rust

First, use the following command to install `eoscdt` for compiling C or C++ code:

```bash
python3 -m pip install -U eoscdt
```

## Compile C/C++ Code to a Library and Link it to Rust Code

Next, let's take compiling the `say_hello` function as an example to demonstrate how to compile code:

If the source file is C code, for example:

say_hello.c

```c
void prints(const char *s);

void say_hello(const char *s) {
	prints(s);
}
```

Then use the following command to compile:

```bash
cdt-cc -c -o say_hello.o say_hello.c
```

If the source file is C++ code, for example:

say_hello.cpp

```cpp
extern "C" void prints(const char *s);

extern "C" void say_hello(const char *s) {
	prints(s);
}
```

Then use the following command to compile:

```bash
cdt-cpp -c -o say_hello.o say_hello.cpp
```

Please note that if it is a C++ file, you need to add `extern "C"` in front of the function, otherwise an error will occur in the linking process below.

After successful compilation, package the `.o` file into a library file ending with `.a`:

```bash
cdt-ar rcs libsay_hello.a say_hello.o
```

Next, let's see how to use the `say_hello` function in Rust code:

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

Where:

```rs
extern "C" {
    fn say_hello(name: *const u8);
}
```

It declares a function defined in C. Please note that the type corresponding to `const char *` in C++ code here is `*const u8`.

The following code shows how to call the `say_hello` function:

```rs
#[chain(action="test")]
pub fn test(&self, name: String) {
    unsafe {
        say_hello(name.as_ptr());
    }
}
```

Because it is a C function, it must be called within an `unsafe` code block.

Next, let's see how to link to the `libsay_hello.a` library.

Create a new file `build.rs`, and add the following content:

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

In the above code, the following code specifies linking to `libsay_hello.a` when the target system of compilation is wasm32:

```rust
println!("cargo:rustc-link-search=./");
println!("cargo:rustc-link-lib=static=say_hello");    
```

The complete example code can be found at the following link:
[Example Code Link 1](https://github.com/uuosio/rscdk/tree/main/tests/testcallcpp)

Use the following command to build and test the code:

build:

```
./build.sh
```

test:

```
./test.sh
```

## Build C++ Library With Cmake and link to Rust Code

Additionally, you can use CMake to compile C++ code and generate library files. Examples can be found at the following link:

[Example Code Link 2](https://github.com/uuosio/rscdk/tree/main/tests/testcallcpp2)

The content of `build.sh` is as follows:

```bash
mkdir -p say_hello/build
pushd say_hello/build
cmake -DCMAKE_TOOLCHAIN_FILE=`cdt-get-dir`/CDTWasmToolchain.cmake ..
make
popd
rust-contract build
```

The code to compile C++ code into a library file is as follows:

```bash
cmake -DCMAKE_TOOLCHAIN_FILE=`cdt-get-dir`/CDTWasmToolchain.cmake ..
```

Here, you need to specify the CDT toolchain file to compile C++ code into wasm32 library files.

Run `./build.sh` to compile C++ code and Rust code.

Run `./test.sh` to test.

## Appendix

here's a mapping of Rust types to C types. 

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
