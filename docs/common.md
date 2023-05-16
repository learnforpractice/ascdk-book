---
comments: true
---

# Common Smart Contract Functions

## is_account

Declaration:

```python
def is_account(account: Name ) -> bool:
    ...
```

Explanation:

Used to determine if an account exists.

## has_auth

Declaration:

```python
def has_auth(account: Name) -> bool:
    ...
```

Explanation:

Used to determine if the 'active' permission of the specified account is present, i.e. whether the Transaction is signed with the private key corresponding to the 'active' permission of the specified account. There is at least one corresponding private key, and possibly multiple.

## require_auth/require_auth2

Declaration:

```python
def require_auth(account: Name):
    ...

def require_auth2(account: Name, permission: Name):
    ...
```

Explanation:

Both of these functions throw an exception when the account does not exist or the specified account's permission is not detected. The difference is that `require_auth` checks for the existence of the 'active' permission, while `require_auth2` can check for a specified permission.

## publication_time/current_time

## check

Declaration:

```python
def check(test: bool, msg: str) -> None:
```

Explanation:

If test is False, an exception will be thrown. This function is frequently used in smart contracts, and can be referenced in the code of `token.codon`.

## Example Code:

```python
from chain.action import has_auth, require_auth, require_auth2, is_account
from chain.contract import Contract

@contract(main=True)
class MyContract(Contract):

    def __init__(self):
        super().__init__()

    @action('test')
    def test(self):
        has_auth(n"hello")

        require_auth(n"hello")
        require_auth2(n"hello", n"active")

        print(is_account(n"hello"))
        print(is_account(n"hello"))
        return
```

Compilation:

```bash
python-contract build common_example.codon
```

Test code:

```python
def test_common():
    t = init_test('common_example')
    ret = t.push_action('hello', 'test', {}, {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
```

Test:

```bash
ipyeos -m pytest -s -x test.py -k test_common
```

Output:
```
[(hello,test)->hello]: CONSOLE OUTPUT BEGIN =====================
True
True

[(hello,test)->hello]: CONSOLE OUTPUT END   =====================
```