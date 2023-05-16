---
comments: true
---

# Development Environment Setup

## Installing the Required Tools for Compilation and Testing

### Direct Installation of Development Toolkits

Install `ipyeos`, a package used for testing smart contracts or running a node:

```bash
python3 -m pip install ipyeos
```

Install `pyeoskit`, a tool used to interact with nodes, such as deploying smart contracts, etc:

```bash
python3 -m pip install pyeoskit
```

### Running in Docker

Currently, the development toolkit does not support Windows and Macbook M1/M2 platforms, and must be run using Docker on these platforms.

The recommended software for installing and running Docker on macOS is [OrbStack](https://orbstack.dev/download). For other platforms, you can use [Docker Desktop](https://www.docker.com/products/docker-desktop).

Download the Docker image using the following command:

```bash
docker pull ghcr.io/uuosio/pscdk:latest
```

Run the container:

```bash
docker run --entrypoint bash -it --rm -v "$(pwd)":/work -w /work -t ghcr.io/uuosio/pscdk
```

## Testing the Installation Environment

Create a test project using the following command:

```bash
python-contract init mytest
cd mytest
```

Compile the contract code:

```bash
python-contract build mytest.codon
```

Alternatively, you can run the `build.sh` script directly:

```bash
./build.sh
```

If there are no exceptions, the `mytest.wasm` WebAssembly binary file will be generated.

To test the installation environment, run:

```bash
ipyeos -m pytest -s -x test.py -k test_hello
```

Alternatively, you can run the test script `test.sh` directly:

```bash
./test.sh
```

You should see the output:

```
hello  alice
```
