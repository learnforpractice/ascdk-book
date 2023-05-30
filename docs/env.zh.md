---
comments: true
---

# 设置开发环境

AssemblyScript智能合约的环境设置比较简单，只需要安装`nodejs`和`ipyeos`即可，与使用react或者vue.js开发网页无异。nodejs可以从下面的链接中下载并安装：

```
https://nodejs.org/
```

接下来安装`ipyeos`，这是一个python包:

```bash
python3 -m pip install -U ipyeos
```

如果你的平台是 Windows 或 MacOSX M1/M2，你还需要下载一个包含ipyeos工具的镜像

```bash
docker pull ghcr.io/uuosio/ipyeos:latest
```

在 macOS 上安装和运行 Docker 的推荐软件是 [OrbStack](https://orbstack.dev/download)。对于其他平台，可以使用 [Docker Desktop](https://www.docker.com/products/docker-desktop)。

## 测试开发环境

可以用下面的方式测试你的开发环境是否设置成功：

首先，从下面的链接下载示例代码：

```
https://github.com/uuosio/as-template
```


然后，使用`cd`命令进入目录，并执行下面的命令进行编译：

```bash
cd as-template
yarn
yarn build
```

如果在`assembly/target`目录下生成`counter.wasm`和`counter.abi`两个文件，表明编译成功。

接下来运行下面的命令运行python测试脚本进行测试：

```bash
yarn pytest
```

将会看到如下的输出：

```
[(hello,inc)->hello]: CONSOLE OUTPUT BEGIN =====================
++++++++count:1
[(hello,inc)->hello]: CONSOLE OUTPUT END   =====================
[(hello,inc)->hello]: CONSOLE OUTPUT BEGIN =====================
++++++++count:2
[(hello,inc)->hello]: CONSOLE OUTPUT END   =====================
```

当然，也可以用typescript来写测试脚本，测试代码在`tests/test.spec.ts`这个文件里。

测试前需要在终端中运行`eosdebugger`这个在`ipyeos`中的工具：

```bash
eosdebugger
```

当看到类似下面的输出时：
```
2023-05-30 16:03:21,259 INFO wasyncore 486 Serving on http://127.0.0.1:9093
```

表示运行成功。

然后用下面的命令运行测试：

```bash
yarn test
```

你将在`eosdebugger`中看到同样的输出


测试代码链接：

[as-template](https://github.com/uuosio/as-template)
