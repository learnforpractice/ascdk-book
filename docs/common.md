---
comments: true
---

# Common Smart Contract Functions

## isAccount

Declaration:

```ts
function isAccount(name: Name): bool
```

Description:

Used to determine whether an account exists or not.

## hasAuth

Declaration:

```ts
function hasAuth(name: Name): bool
```

Description:

Used to determine whether it has the `active` permission of a specified account, that is, whether the Transaction has been signed with the private key corresponding to the `active` permission of the specified account. The private key used for signing may be at least one, or two or more.

## requireAuth/requireAuth2

Declaration:

```ts
function requireAuth(name: Name): void
function hasAuth(name: Name): bool
function requireAuth2(permissionLevel: PermissionLevel): void
```

Description:

These two functions will throw exceptions if the account does not exist or if the specified account's permission is not detected. The difference is that `requireAuth` checks for the existence of `active` permission, while `requireAuth2` can check for specified permissions. Among them, the `requireAuth` function is used very frequently in contract development to ensure that the execution of an action has the `active` permission of a certain account.

## currentTime

Declaration:

```ts
function currentTime(): u64
```

Description:

Used to get the time of the block where the Transaction is located, in microseconds (1 second equals 1,000,000 microseconds).

## check

Declaration:

```ts
function check(test: bool, msg: string): void
```

Description:

If test is false, it will throw an exception, all actions executed in the Transaction and the operations on the database that have been executed by this action will be rolled back, and the Transaction will not be chained. This is quite different from the `revert` mechanism in Ethereum. The result is that the EOS network is relatively vulnerable, because the rollback of the exceptional Transaction does not consume resources, that is, it does not incur costs, making the network more susceptible to attacks. But in normal contracts, this function is also frequently used in smart contracts, you can refer to the related code in [eosio.token.contract.ts](https://github.com/uuosio/ascdk/blob/master/examples/eosio.token/eosio.token.contract.ts)


## Example Code:

```ts
import {
    Name,
    PermissionLevel,
    Contract,

    requireAuth,
    requireAuth2,
    hasAuth,
    isAccount,
    print,

    check
} from "asm-chain";

@contract
class MyContract extends Contract {
    constructor(receiver: Name, firstReceiver: Name, action: Name) {
        super(receiver, firstReceiver, action);
    }

    @action("test")
    test(): void {
        let ret = isAccount(Name.fromString("noexits"));
        print(`+++isAccount(noexits): ${ret}\n`);
        ret = isAccount(this.receiver);
        print(`+++isAccount(this.receiver): ${ret}\n`);

        print(`hasAuth: ${hasAuth(this.receiver)}`);
        requireAuth(this.receiver);
        requireAuth2(new PermissionLevel(this.receiver, Name.fromString("active")));
    }
}
```

Compilation:

```bash
cd examples/commonfunctions
yarn
yarn build
```

Test code:

```python
@chain_test
def test_hello(tester):
    deploy_contract(tester, 'test')
    args = {}
    r = tester.push_action('hello', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

Testing:

```bash
ipyeos -m pytest -s -x test.py -k test_hello
```

Output:

```
[(hello,test)->hello]: CONSOLE OUTPUT BEGIN =====================
+++isAccount(noexits): false
+++isAccount(this.receiver): true
hasAuth: true
current time: 1527854403000000

[(hello,test)->hello]: CONSOLE OUTPUT END   =====================
```

[Full Example Code](https://github.com/learnforpractice/ascdk-book/tree/master/examples/commonfunctions)