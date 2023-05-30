---
comments: true
---

# requireRecipient Function

The function is declared in `action.ts` in `asm-chain` as follows:

```ts
function requireRecipient(name: Name): void
```

The `requireRecipient` function is used to notify other contracts that this contract has called a certain action, which is the action where `requireRecipient` is called. If the notified contract has the same action, then this action will be called.

The following `sender` and `receiver` code demonstrates how to send a notification from one contract to another.

```ts
// sender
import {
    print,
    requireAuth,
    requireRecipient,

    Name,
    Contract,
} from "asm-chain";

@contract
class MyContract extends Contract {
    @action("sayhello")
    sayHello(name: Name): void {
        print(`hello ${name}!`);
        requireAuth(name);
        requireRecipient(Name.fromString('hello'));
    }
}
```

```ts
//receiver.ts
import {
    Name,
    Contract,

    print,
} from "asm-chain";

@contract
class MyContract extends Contract {
    @action("sayhello", notify)
    sayHello(name: Name): void {
        print(`notify: hello ${name}!`);
    }
}
```

Let's explain the code:

- The `sender` contract code is deployed in the `alice` contract account. The `sayhello` action in it calls the `requireRecipient` function to notify the `hello` account that it has called the `sayhello` action.
- The `receiver` contract code is deployed in the `hello` account. From the following code, the line `#[chain(action="test", notify)]` is different from the normal action code. The `notify` parameter indicates that this action is used to receive notifications from other contracts. For an action that receives notifications, its `self.receiver` and `self.first_receiver` are not the same. In this case, `self.first_receiver` is the `alice` account and `self.receiver` is `hello`. You can view the output of `chain_println!(self.receiver, self.first_receiver);` during the test run to know this.

```ts
@action("sayhello", notify)
sayHello(name: Name): void {
    print(`notify: hello ${name}!`);
}
```

Below is the test code:

```python
@chain_test
def test_notify(tester):
    deploy_contracts(tester)
    args = {'name': 'alice'}
    r = tester.push_action('alice', 'sayhello', args, {'alice': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
```

Compilation:

```bash
cd examples/notify
yarn
yarn build
```

Testing:

```bash
ipyeos -m pytest -s -x test.py -k test_notify
```

Output:

```
[(alice,sayhello)->alice]: CONSOLE OUTPUT BEGIN =====================
hello alice!
[(alice,sayhello)->alice]: CONSOLE OUTPUT END   =====================
[(alice,sayhello)->hello]: CONSOLE OUTPUT BEGIN =====================
notify: hello alice!
[(alice,sayhello)->hello]: CONSOLE OUTPUT END   =====================
```

[Example code](https://github.com/learnforpractice/ascdk-book/tree/master/examples/notify)
