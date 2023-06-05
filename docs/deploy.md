---
comments: true
---

# Deploying a Smart Contract

```python
from pyeoskit import eosapi
from pyeoskit import wallet

# Set node
eosapi.set_node('https://eos.greymass.com')

# Import the private key corresponding to the active permission of the contract account,
# you can import multiple private keys simultaneously
wallet.import_key('mywallet', 'your private key')

account_name = 'myaccount'
contract_name = 'test'

# Read .wasm file
with open(f'{contract_name}.wasm', 'rb') as f:
    code = f.read()

# Read .abi file
with open(f'{contract_name}.abi', 'rb') as f:
    abi = f.read()

# Deploy
eosapi.deploy_contract(account_name, code, abi)

```

If you have the private key corresponding to the active permission saved in a Ledger Nano S wallet, you can use the `indices` parameter to specify the index of the private key in the wallet:

```python
eosapi.deploy_contract(account_name, code, abi， indices=[0,])
```

Additionally, if your WASM code is too large and causing deployment failures, you can try adding the parameter `compress=True` when calling `deploy_contract`:

```python
eosapi.deploy_contract(account_name, code, abi, compress=True)
```
