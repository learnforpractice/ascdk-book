---
comments: true
---

# Using Inline Action in Smart Contracts

An action can also be initiated within a smart contract, such action is called an inline action. It's important to note that actions are asynchronous, meaning that the contract code corresponding to the inline action will only be called after the entire code has been executed. If the contract being called does not define the related action or if there is no deployed contract in the account, the call will have no effect but no exceptions will be thrown. These empty inline actions are not without purpose, they can be used as on-chain logs for applications to query.

The following example uses an inline action to make an EOS transfer to illustrate the use of inline actions.

```ts
import {
    Name,
    Contract,
    Asset,
    Symbol,
    Action,
    PermissionLevel,
    print,
    printString,
} from "asm-chain";

@packer
class Transfer {
    constructor(
        public from: Name,
        public to: Name,
        public quantity: Asset,
        public memo: string){
    }
}

@contract
class MyContract extends Contract {
    constructor(receiver: Name, firstReceiver: Name, action: Name) {
        super(receiver, firstReceiver, action);
    }

    @action("test")
    test(): void {
        let transfer = new Transfer(
            this.receiver,
            Name.fromString("alice"),
            new Asset(10000, new Symbol("EOS", 4)),
            "hello"
        );

        let a = Action.new(
            Name.fromString("eosio.token"),
            Name.fromString("transfer"),
            new PermissionLevel(this.receiver, Name.fromString("active")),
            transfer,
        );
        a.send();
        printString(`Done!`);
    }
}
```

The code above implements a function to transfer `1.0000 EOS` from the `hello` account to the `alice` account. The `hello` and `alice` accounts are both created when the test is started and can be used directly.

Test code:

```python
@chain_test
def test_hello(tester):
    deploy_contract(tester, 'test')

    logger.info("balance of hello before transfer: %s",  tester.get_balance('hello'))
    logger.info("balance of alice before transfer: %s",  tester.get_balance('alice'))

    args = {}
    r = tester.push_action('hello', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()

    logger.info("balance of hello after transfer: %s",  tester.get_balance('hello'))
    logger.info("balance of alice after transfer: %s",  tester.get_balance('alice'))
```

Compile:

```bash
cd examples/inlineaction
yarn
yarn build
```

Run the test:

```bash
ipyeos -m pytest -s -x test.py -k test_inline_action
```

Output:

```
INFO     test:test.py:74 balance of hello before transfer: 50000000000
INFO     test:test.py:75 balance of alice before transfer: 50000000000
INFO     test:test.py:81 balance of hello after transfer: 49999990000
INFO     test:test.py:82 balance of alice after transfer: 50000010000
```

Please note that in order to call an inline action within a contract, you need to add the virtual permission `eosio.code` to the `active` permission of the account. In the test code, the function below is used to add the `eosio.code` virtual permission to the `active` permission.

```python
def update_auth(chain, account):
    a = {
        "account": account,
        "permission": "active",
        "parent": "owner",
        "auth": {
            "threshold": 1,
            "keys": [
                {
                    "key": 'EOS6AjF6hvF7GSuSd4sCgfPKq5uWaXvGM2aQtEUCwmEHygQaqxBSV',
                    "weight": 1
                }
            ],
            "accounts": [{"permission":{"actor":account,"permission": 'eosio.code'}, "weight":1}],
            "waits": []
        }
    }
    chain.push_action('eosio', 'updateauth', a, {account:'active'})
```

In summary:

In EOS, in addition to including actions in a Transaction to call contract code, an Action can also be initiated in the contract code to call other contract code. This kind of Action is called an Inline Action. To allow contract code to use Inline Actions, the `eosio.code` virtual permission must be added to the `active` permission of the contract account.

[Complete Example](https://github.com/learnforpractice/ascdk-book/tree/master/examples/inlineaction)
