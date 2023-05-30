Here is the translation of the given Markdown text from Chinese to English:

---
comments: true
---

# Set up the Development Environment

Setting up the environment for AssemblyScript smart contracts is relatively simple. All you need to install are `nodejs` and `ipyeos`, just like developing a webpage using react or vue.js. You can download and install nodejs from the link below:

```
https://nodejs.org/
```

Next, install `ipyeos`, which is a python package:

```bash
python3 -m pip install -U ipyeos
```

If your platform is Windows or MacOSX M1/M2, you also need to download an image that includes the ipyeos tool:

```bash
docker pull ghcr.io/uuosio/ipyeos:latest
```

The recommended software for installing and running Docker on macOS is [OrbStack](https://orbstack.dev/download). For other platforms, you can use [Docker Desktop](https://www.docker.com/products/docker-desktop).

## Test the Development Environment

You can test if your development environment is set up successfully in the following way:

First, download the sample code from the link below:

```
https://github.com/uuosio/as-template
```

Then, use the `cd` command to enter the directory and run the following command to compile:

```bash
cd as-template
yarn
yarn build
```

If two files `counter.wasm` and `counter.abi` are generated in the `assembly/target` directory, it means the compilation was successful.

Next, run the following command to run the Python test script for testing:

```bash
yarn pytest
```

You will see the following output:

```
[(hello,inc)->hello]: CONSOLE OUTPUT BEGIN =====================
++++++++count:1
[(hello,inc)->hello]: CONSOLE OUTPUT END   =====================
[(hello,inc)->hello]: CONSOLE OUTPUT BEGIN =====================
++++++++count:2
[(hello,inc)->hello]: CONSOLE OUTPUT END   =====================
```

Additionally, you can also use typescript to write test scripts. The test code is in the `tests/test.spec.ts` file.

Before testing, you need to run the `eosdebugger` tool in `ipyeos` in the terminal:

```bash
eosdebugger
```

When you see an output similar to the following:
```
2023-05-30 16:03:21,259 INFO wasyncore 486 Serving on http://127.0.0.1:9093
```

It indicates successful operation.

Then use the following command to run the test:

```bash
yarn test
```

You will see the same output in `eosdebugger`.


Test code link:

[as-template](https://github.com/uuosio/as-template)
