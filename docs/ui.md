---
comments: true
---

# Interact with Digital Wallets

After the smart contract is written, the next step is to design a user interface to interact with the smart contract. To call the on-chain smart contract from the user interface, you must use digital wallet software. The following demo will show you how to call the `transfer` Action of the smart contract in the `eosio.token` account.

## Download Anchor Wallet

Download the desktop version from the link below, and choose the latest version:

```
https://github.com/greymass/anchor/tags
```

For mobile versions, search for "Anchor Wallet" in the app store to download.

After downloading, import the private key of your EOS account to use the wallet. The desktop version also supports Ledger hardware wallets.

## Web Page Code

The following HTML code only relies on anchor.min.js and can be run directly when opened locally. The complete code can be found in the link below, or you can try the following example directly. Make sure you have already installed the Anchor wallet and imported the account private key:

```
https://github.com/learnforpractice/ascdk-book/tree/master/examples/frontend
```

<style>
    form {
        background-color: #f0f0f0;
        padding: 20px;
        border-radius: 10px;
        box-shadow: 0 2px 5px rgba(0, 0, 0, 0.5);
    }
    form button {
        color: #ffffff; /* You can change this color to your desired foreground color */
        background-color: #007bff;
        padding: 5px 10px;
        border: none;
        border-radius: 5px;
        cursor: pointer;
    }

    form button:hover {
        background-color: #0056b3;
    }
    form label {
        display: inline-block;
        width: 100px; /* Adjust this value to set the desired width for the labels */
        text-align: left;
        margin-right: 10px;
    }
</style>
<script src="../assets/javascripts/anchor.min.js"></script>
<script>
const eos = {
    chainId: 'aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906',
    rpcEndpoints: [{
    protocol: 'https',
    host: 'eos.greymass.com',
    port: '443',
    }]
}

var wallet = new anchor.Anchor([eos], {
    // Required: The app name, required by anchor-link. Short string identifying the app
    appName: 'my-example-dapp',
    // Optional: a @greymass/eosio APIClient from eosjs for both your use and to use internally in UAL
    // client = new APIClient({ provider }),
    // Optional: a JsonRpc instance from eosjs for your use
    // rpc: new JsonRpc(),
    // Optional: The callback service URL to use, defaults to https://cb.anchor.link
    service: 'https://cb.anchor.link',
    // Optional: A flag to disable the Greymass Fuel integration, defaults to false (enabled)
    // disableGreymassFuel: false,
    // Optional: An account name on a Fuel enabled network to specify as the referrer for transactions
    // fuelReferrer: 'teamgreymass',
    // Optional: A flag to enable the Anchor Link UI request status, defaults to true (enabled)
    // requestStatus: true,  
    // Optional: Whether or not to verify the signatures during user login, defaults to false (disabled)
    // verifyProofs: false,
});

(async () => {
    await wallet.init();
})();

window.wallet = wallet;
</script>
<form>
    <button type="button" id="login">Login</button>
    <button type="button" id="logout">Logout</button><br><br>
    <label for="account">To Account:</label>
    <input type="text" id="account" name="account"><br><br>
    <label for="amount">Quantity:</label>
    <input type="number" id="amount" name="amount" value="0.0001"><label>EOS</label><br><br>
    <label for="memo">Memo:</label>
    <input type="text" id="memo" name="memo" value="hello world"><br><br>
    <button type="button" id="transfer">Transfer</button>
</form>

<script>
    document.getElementById("transfer").addEventListener("click", async function() {
        if (wallet.users.length == 0) {
            alert("please login first!");
            return;
        }
        let account = document.getElementById("account").value;
        let amount = document.getElementById("amount").value;
        let memo = document.getElementById("memo").value;

        console.log("Account: " + account);
        console.log("Amount: " + amount);
        console.log("Memo: " + memo);

        amount = parseFloat(amount).toFixed(4);
        let user = wallet.users[0];
        let args = {
            action: {
                account: 'eosio.token',
                name: 'transfer',
                authorization: [user.session.auth],
                data: {
                    from: user.session.auth.actor,
                    to: account,
                    quantity: `${amount} EOS`,
                    memo: memo,
                },
            },
        }
        var ret = await user.session.transact(args);
        console.log(ret);
        alert(JSON.stringify(ret.processed));
    });

    document.getElementById("login").addEventListener("click", async function() {
        if (wallet.users.length == 0) {
            var ret = await wallet.login();
            console.log("++++++:", ret);
        }
    });

    document.getElementById("logout").addEventListener("click", async function() {
        if (wallet.users.length != 0) {
            await wallet.logout();
        }
    });
</script>

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>发送Token示例</title>
    <style>
        body {
            margin: 0;
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            background-color: #f0f0f0;
        }
        form {
            background-color: #ffffff;
            padding: 20px;
            border-radius: 5px;
            box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
        }
    </style>
    <script src="./anchor.min.js"></script>
    <script>
    const eos = {
      chainId: 'aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906',
      rpcEndpoints: [{
        protocol: 'https',
        host: 'eos.greymass.com',
        port: '443',
      }]
    }
    
    var wallet = new anchor.Anchor([eos], {
      // Required: The app name, required by anchor-link. Short string identifying the app
      appName: 'my-example-dapp',
      // Optional: a @greymass/eosio APIClient from eosjs for both your use and to use internally in UAL
      // client = new APIClient({ provider }),
      // Optional: a JsonRpc instance from eosjs for your use
      // rpc: new JsonRpc(),
      // Optional: The callback service URL to use, defaults to https://cb.anchor.link
      service: 'https://cb.anchor.link',
      // Optional: A flag to disable the Greymass Fuel integration, defaults to false (enabled)
      // disableGreymassFuel: false,
      // Optional: An account name on a Fuel enabled network to specify as the referrer for transactions
      // fuelReferrer: 'teamgreymass',
      // Optional: A flag to enable the Anchor Link UI request status, defaults to true (enabled)
      // requestStatus: true,  
      // Optional: Whether or not to verify the signatures during user login, defaults to false (disabled)
      // verifyProofs: false,
    });

    (async () => {
        await wallet.init();
    })();

    window.wallet = wallet;
    </script>
</head>
<body>
    <form>
        <button type="button" id="login">Login</button>
        <button type="button" id="logout">Logout</button><br><br>
        <label for="account">To Account:</label>
        <input type="text" id="account" name="account"><br><br>
        <label for="amount">Quantity:</label>
        <input type="number" id="amount" name="amount" value="0.0001"><label>EOS</label><br><br>
        <label for="memo">Memo:</label>
        <input type="text" id="memo" name="memo" value="hello world"><br><br>
        <button type="button" id="transfer">Transfer</button>
    </form>

    <script>
        document.getElementById("transfer").addEventListener("click", async function() {
            if (wallet.users.length == 0) {
                alert("please login first!");
                return;
            }
            let account = document.getElementById("account").value;
            let amount = document.getElementById("amount").value;
            let memo = document.getElementById("memo").value;

            console.log("Account: " + account);
            console.log("Amount: " + amount);
            console.log("Memo: " + memo);

            amount = parseFloat(amount).toFixed(4);
            let user = wallet.users[0];
            let args = {
                action: {
                    account: 'eosio.token',
                    name: 'transfer',
                    authorization: [user.session.auth],
                    data: {
                        from: user.session.auth.actor,
                        to: account,
                        quantity: `${amount} EOS`,
                        memo: memo,
                    },
                },
            }
            var ret = await user.session.transact(args);
            console.log(ret);
            alert(JSON.stringify(ret.processed));
        });

        document.getElementById("login").addEventListener("click", async function() {
            if (wallet.users.length == 0) {
                var ret = await wallet.login();
                console.log("++++++:", ret);
            }
        });

        document.getElementById("logout").addEventListener("click", async function() {
            if (wallet.users.length != 0) {
                await wallet.logout();
            }
        });
    </script>
</body>
</html>
```

A simple explanation:

When the web page is initializing, it will call `new anchor.Anchor` to create a wallet instance. Then it calls `wallet.init` for initialization, note that `init` is an asynchronous function.

```javascript
var wallet = new anchor.Anchor
wallet.init()
```

Click `Login` to connect to the wallet client first. A dialog box will pop up, click `Launch Anchor` to open the desktop wallet or open the `Anchor` app on your phone to scan the QR code for authorization.

```javascript
await wallet.login()
```

Click `Logout` to cancel the authorization.

```javascript
await wallet.logout()
```

Note that both `login` and `logout` functions are asynchronous.

Enter a valid transfer account and transfer amount, then click `Transfer` to prompt the authorization dialog on your phone or desktop software. Please ensure the Action described on the interface is accurate before confirming authorization. Below is the related code, calling the `transfer` Action of `eosio.token`.

```javascript
let user = wallet.users[0];
let args = {
    action: {
        account: 'eosio.token',
        name: 'transfer',
        authorization: [user.session.auth],
        data: {
            from: user.session.auth.actor,
            to: account,
            quantity: `${amount} EOS`,
            memo: memo,
        },
    },
}
var ret = await user.session.transact(args);
```

Summary:

In the above example, `anchor.min.js` is the front-end library code generated by webpack from ual-anchor library.

```
https://github.com/greymass/ual-anchor
```

The above example only demonstrates a simple case. In actual examples, you may need to use application frameworks like `vuejs`, `react`, `svelte`, etc. These frameworks can directly use the `ual-anchor` library. You can refer to the following examples for implementation:

```
https://github.com/greymass/ual-anchor-demo
```
