---
comments: true
---

# 发布智能合约

```python
from pyeoskit import eosapi
from pyeoskit import wallet

#设置节点
eosapi.set_node('https://eos.greymass.com')

#导入合约账号的active权限对应的私钥，可以同时导入多个私钥
wallet.import_key('mywallet', 'your private key')

account_name = 'myaccount'
contract_name = 'test'

#读取wasm文件
with open(f'{contract_name}.wasm', 'rb') as f:
    code = f.read()

#读取ABI文件
with open(f'{contract_name}.abi', 'rb') as f:
    abi = f.read()

#发布
eosapi.deploy_contract(account_name, code, abi)
```

如何你有active权限对应的私钥保存在Ledger硬件钱包里，可以用`indices`参数指定ledger中的私钥所在的索引：

```python
eosapi.deploy_contract(account_name, code, abi， indices=[0,])
```

另外，如果你的wasm代码过大而导致发布失败，可以尝试在调用deploy_contract的时候加上参数：`compress=True`

```python
eosapi.deploy_contract(account_name, code, abi， compress=True)
```
