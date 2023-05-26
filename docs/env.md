# Setting Up the Development Environment

## Installing Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

Activate the new PATH environment variable:

```bash
source $HOME/.cargo/env
```

## Installing Nightly Rust

Due to issues with compiling contracts using newer versions of Rust nightly, it is necessary to install the `nightly-2023-02-07` version of Rust:

```
rustup install nightly-2023-02-07
rustup component add rust-src --toolchain nightly-2023-02-07
```

Then, on Linux platform, execute the following command:
```bash
ln -s ~/.rustup/toolchains/nightly-2023-02-07-x86_64-unknown-linux-gnu ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu
```

If you are on macOSX platform, execute the following command:
```bash
ln -s ~/.rustup/toolchains/nightly-2023-02-07-x86_64-apple-darwin ~/.rustup/toolchains/nightly-x86_64-apple-darwin
```

If you have already installed the default nightly version beforehand, you will need to rename or delete the original directory. Otherwise, it will cause the creation of the symbolic link to fail.

The additional setup mentioned above will be improved in future versions to allow compiling Rust contracts using the stable version.

## Installing Binaryen

* Install version >= 99 of `binaryen`:
  * [Debian/Ubuntu](https://tracker.debian.org/pkg/binaryen): `apt-get install binaryen`
  * [Homebrew](https://formulae.brew.sh/formula/binaryen): `brew install binaryen`
  * [Arch Linux](https://archlinux.org/packages/community/x86_64/binaryen/): `pacman -S binaryen`
  * Windows: [Binary releases can be downloaded](https://github.com/WebAssembly/binaryen/releases)

## Creating a Virtual Python Environment for Testing

```bash
python3 -m venv ~/env
source ~/env/bin/activate
python3 -m pip install --upgrade pip
```

The next time you want to use the test environment, simply run the following command again.

```bash
source ~/env/bin/activate
```

## Installing the EOS Testing Framework

Install ipyeos:

```bash
python3 -m pip install ipyeos
```

If your platform is Windows or MacOSX M1/M2, you can also download an image that includes the ipyeos tool:

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

On macOS, the recommended software for installing and running Docker is [OrbStack](https://orbstack.dev/download). For other platforms, you can use [Docker Desktop](https://www.docker.com/products/docker-desktop).

## Installing the Rust Smart Contract Builder

```bash
python3 -m pip install rust-contracts-builder
```

## Installing the EOS Python Toolkit

```bash
python3 -m pip install pyeoskit
```

pyeoskit is used for deploying contracts to the mainnet or testnet.

## Checking the Environment

Create a new Rust contract project:

```bash
rust-contract init hello
```

Build:

```bash
cd hello
rust-contract build
```

Test:

```bash
ipyeos -m pytest -s -x test.py -k test_hello
```

If your platform does not support running ipyeos directly, such as on Windows or macOS M1/M2, or on other platforms using the ARM instruction set, you can use Docker to run this command:

```bash
docker run --entrypoint ipyeos -it --rm -v "$(pwd)":/develop -w /develop -t ghcr.io/uuosio/scdk -m pytest -s -x test.py -k test_hello
```

Alternatively, you can run `cargo test` to run the tests:

```bash
cargo test
```

When running `cargo test`, the tests defined in `lib.rs` will be executed:

```rust
#[test]
fn test_inc() {
    let mut tester = ChainTester::new();
    //uncomment the following line to enable contract debugging.
    // tester.enable_debug_contract("hello", true).unwrap();

    deploy_contract(&mut tester);
    update_auth(&mut tester);

    let permissions = r#"
    {
        "hello": "active"
    }
    "#;
    tester.push_action("hello", "inc", "".into(), permissions).unwrap();
    tester.produce_block();

    tester.push_action("hello", "inc", "".into(), permissions).unwrap();
    tester.produce_block();
}
```

Note that before running `cargo test`, you must first execute the `eosdebugger` application available in `ipyeos`. The Rust test code needs to connect to `eosdebugger` to run the tests.

If your platform does not support running `eosdebugger` directly, such as on Windows or on platforms using the ARM instruction set, you can use Docker to run this command:

```bash
docker run -it --rm -p 9090:9090 -p 9092:9092 -p 9093:9093 -t ghcr.io/uuosio/scdk
```

After starting `eosdebugger`, run `cargo test`, and you will see the following output in the `eosdebugger` console:

```bash
[(hello,inc)->hello]: CONSOLE OUTPUT BEGIN =====================
count is 1

[(hello,inc)->hello]: CONSOLE OUTPUT END   =====================
debug 2023-05-24T09:18:36.315 ipyeos    controller.cpp:2498           clear_expired_input_ ] removed 0 expired transactions of the 50 input dedup list, pending block time 2018-06-01T12:00:04.000
debug 2023-05-24T09:18:36.319 ipyeos    apply_context.cpp:40          print_debug          ] 
[(hello,inc)->hello]: CONSOLE OUTPUT BEGIN =====================
count is 2

[(hello,inc)->hello]: CONSOLE OUTPUT END   =====================
```
