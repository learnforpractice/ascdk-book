---
comments: true
---

# 设置开发环境

## 安装 Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

激活新的 PATH 环境变量：

```bash
source $HOME/.cargo/env
```

对于Windows平台，可前往下面的的官方下载地址根据提示下载：

```
https://www.rust-lang.org/tools/install
```

## 安装 binaryen

* 安装版本 >= 99 的 `binaryen`:
  * [Debian/Ubuntu](https://tracker.debian.org/pkg/binaryen): `apt-get install binaryen`
  * [Homebrew](https://formulae.brew.sh/formula/binaryen): `brew install binaryen`
  * [Arch Linux](https://archlinux.org/packages/community/x86_64/binaryen/): `pacman -S binaryen`
  * Windows: [二进制发布版本可以下载](https://github.com/WebAssembly/binaryen/releases)

## 为测试创建一个虚拟Python环境
```bash
python3 -m venv ~/env
source ~/env/bin/activate
python3 -m pip install --upgrade pip
```

下次你想使用测试环境时，只需再次运行以下命令。

```
source ~/env/bin/activate
```

## 安装EOS测试框架

安装 ipyeos：

```bash
python3 -m pip install -U ipyeos
```

如果你的平台是 Windows 或 MacOSX M1/M2，你还需要下载一个包含ipyeos工具的镜像

```bash
docker pull ghcr.io/uuosio/ipyeos:latest
```

在 macOS 上安装和运行 Docker 的推荐软件是 [OrbStack](https://orbstack.dev/download)。对于其他平台，可以使用 [Docker Desktop](https://www.docker.com/products/docker-desktop)。

## 安装 Rust 智能合约构建器

```bash
python3 -m pip install -U rust-contracts-builder
```

## 安装 EOS 的 Python 工具包

```bash
python3 -m pip install -U pyeoskit
```

pyeoskit 用于部署合约到主网或者测试网。

## 检查环境

创建一个新的 rust 合约项目：

```bash
rust-contract init hello
```

构建

```bash
cd hello
rust-contract build
```

测试:

```bash
ipyeos -m pytest -s -x test.py -k test_hello
```

在Windows平台或者或 MacOSX M1/M2，运行上面的命令将自动启动docker来运行测试。


另外，你也可以运行`cargo test`来运行测试：

```bash
cargo test
```

这时，运行的是`lib.rs`里面的测试代码：

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

需要注意的是，执行`cargo test`之前，必须先执行`eosdebugger`这个在`ipyeos`中的应用，rust测试代码需要连接到`eosdebugger`来运行测试。

在Windows平台或者使用ARM指令集的CPU的平台，运行该命令将自动通过docker来运行eosdebugger.

启动后，运行`cargo test`，即可以运行`eosdebugger`中的控制台中看到如下的输出：

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
