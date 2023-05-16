---
comments: true
---

# 与数字钱包进行交互

当智能合约写好后，下一个步骤就是如何设计用户界面来与智能合约进行交互。在用户界面要调用链上的智能合约，必须通过数字钱包软件，正面的示例是演示的通过`Anchor`这个钱包来发送交易。实现的功能是调用`eosio.token`这个账号里的智能合约的`transfer`Action。

## 下载Anchor钱包

桌面版本通过正面的链接下载选择最新版本下载：

```
https://github.com/greymass/anchor/tags
```

手机版本通过应用商店搜索`Anchor Wallet`进行下载

下载后导入EOS的账号的私钥即可使用，桌面版本也支持ledger硬件钱包

## 网页代码

下面这个html代码只依赖于`anchor.min.js`，可以在本地打开直接运行，完整的代码可以从下面的链接中找到，也可以直接尝试下面这个例子，需要事先安装好anchor钱包并且已经导入了账号私钥：


```
https://github.com/learnforpractice/pscdk-book/tree/main/examples/frontend
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
<script src="../../assets/javascripts/anchor.min.js"></script>
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
    <title>简单的HTML界面</title>
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

简单解释下：

当网页初始化后，会调用`new anchor.Anchor`来创建一个钱包实例。然后调用`wallet.init`进行初始化，注意`init`是一个异步函数

```javascript
var wallet = new anchor.Anchor
wallet.init()
```

点`Login`先连接钱包客户端，会弹出一个对话框，点`Launch Anchor`打开桌面钱包或者手机上打开`Anchor`软件进行扫码进行授权。

```javascript
await wallet.login()
```

点`Logout`取消授权

```javascript
await wallet.logout()
```

注意`login`和`logout`两个函数都是异步的


输入合法的转账账号和转账的数额，然后点`Transfer`即会在手机或者桌面软件上弹出授权对话框进行授权。请确保界面上描述的Action准确无误后才能确认授权。正面是相关的代码，调用了`eosio.token`的`transfer`这个Action。

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

总结：

上面的例子中`anchor.min.js`是用的用到了正面的库经过webpack后生成的代码

```
https://github.com/greymass/ual-anchor
```

上面的例子只是演示了一个最简单的例子，在实际的例子中，可能要用到`vuejs`,`react`,`svelte`等应用框架，这些框架都可以直接使用`ual-anchor`这个库。实现也可以参考一下的示例：

```
https://github.com/greymass/ual-anchor-demo
```
