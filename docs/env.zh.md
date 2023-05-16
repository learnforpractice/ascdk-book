Here is the translation of the provided Markdown text to Chinese:

```markdown
# 设置开发环境

## 安装 Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

激活新的 PATH 环境变量。

```bash
source $HOME/.cargo/env
```

## 安装 Nightly Rust

```bash
rustup toolchain install nightly --component rust-src
```

## 安装 binaryen

* 安装版本 >= 99 的 `binaryen`:
  * [Debian/Ubuntu](https://tracker.debian.org/pkg/binaryen): `apt-get install binaryen`
  * [Homebrew](https://formulae.brew.sh/formula/binaryen): `brew install binaryen`
  * [Arch Linux](https://archlinux.org/packages/community/x86_64/binaryen/): `pacman -S binaryen`
  * Windows: [二进制发布版本可以下载](https://github.com/WebAssembly/binaryen/releases)

## 为测试创建一个虚拟 Python 环境
```bash
python3 -m venv ~/env
source ~/env/bin/activate
python3 -m pip install --upgrade pip
```

下次你想使用测试环境时，只需再次运行以下命令。

```
source ~/env/bin/activate
```

## 安装 Eos 测试框架

安装 ipyeos：

```bash
python3 -m pip install ipyeos
```

运行 eosdebugger

```bash
eosdebugger
```

如果你的平台是 Windows 或 MacOSX M1/M2，你也可以下载一个容器镜像，并从 Docker 中运行 eosdebugger：

```bash
docker pull ghcr.io/uuosio/ipyeos:latest
```

```bash
docker run -it --rm -p 9090:9090 -p 9092:9092 -p 9093:9093 -t ghcr.io/uuosio/ipyeos
```

在 macOS 上安装和运行 Docker 的推荐软件是 [OrbStack](https://orbstack.dev/download)。对于其他平台，可以使用 [Docker Desktop](https://www.docker.com/products/docker-desktop)。

## 安装 Rust 智能合约构建器

```bash
python3 -m pip install rust-contracts-builder
```

## 安装 EOS 的 Python 工具包

```bash
python3 -m pip install pyeoskit
```

pyeoskit 用于部署合约。

## 检查环境

创建一个新的 rust 合约项目：

```bash
rust-contract init hello
```

构建

```bash
cd hello
./build.sh
```

测试

```bash
cargo test
```

如果 `eosdebugger` 正在运行，它将输出如下信息：

```bash
debug 2023-02-20T07:03:09.852 ipyeos    controller.cpp:2406           clear_expired_input_ ] removed 0 expired transactions of the 41 input dedup list
debug 2023-02-20T07:03:09.861 ipyeos    controller.cpp:2406           clear_expired_input_ ] removed 0 expired transactions of the 47 input dedup list
debug 2023-02-20T07:03:09.887 ipyeos    controller.cpp:2406           clear_expired_input_ ] removed 0 expired transactions of the 49 input dedup list
debug 2023-02-20T07:03:09.891 ipyeos    apply_context.cpp:28          print_debug          ]
[(hello,inc)->hello]: CONSOLE OUTPUT BEGIN =====================
count is 1

[(hello,inc)->hello]: CONSOLE OUTPUT END   =====================
debug 2023-02-20T07:03:09.894 ipyeos    controller.cpp:2406           clear_expired_input_ ] removed 0 expired transactions of the 50 input dedup list
debug 2023-02-20T07:03:09.897 ipyeos    apply_context.cpp:28          print_debug          ]
[(hello,inc)->hello]: CONSOLE OUTPUT BEGIN =====================
count is 2

[(hello,inc)->hello]: CONSOLE OUTPUT END   =====================
debug 2023-02-20T07:03:09.899 ipyeos    controller.cpp:2406           clear_expired_input_ ] removed 0 expired transactions of the 51 input dedup list
Listening for new connection...
```

另外，你可以使用下面的命令来运行测试，无需 `eosdebugger` 在后台运行。
```bash
ipyeos -m pytest -s -x test.py -k test_hello
```

也可以在docker中运行上面的命令：

```bash
docker run --entrypoint ipyeos -it -v$(pwd):/develop -w /develop -t ghcr.io/uuosio/ipyeos -m pytest -s -x test.py
```

如果你看到以下的输出，那就意味着所有东西都已经成功安装。

```
test.py debug 2022-07-04T04:01:58.496 ipyeos    apply_context.cpp:36          print_debug          ] 
[(hello,inc)->hello]: CONSOLE OUTPUT BEGIN =====================
count is 1

[(hello,inc)->hello]: CONSOLE OUTPUT END   =====================
debug 2022-07-04T04:01:58.498 ipyeos    apply_context.cpp:36          print_debug          ] 
[(hello,inc)->hello]: CONSOLE OUTPUT BEGIN =====================
count is 2

[(hello,inc)->hello]: CONSOLE OUTPUT END   =====================
.

============================== 1 passed in 0.90s ===============================
```