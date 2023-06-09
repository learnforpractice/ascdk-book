import {
    Name,
    Contract,

    check,
    print,
    requireAuth,
} from "asm-chain";

@table("counter")
class Counter {
    public account: Name|null;
    public count: u64;
    constructor(account: Name|null=null, count: u64=0) {
        this.count = count;
        this.account = account;
    }

    @primary
    get primary(): u64 {
        return this.account!.N;
    }
}

@contract
class MyContract extends Contract {
    constructor(receiver: Name, firstReceiver: Name, action: Name) {
        super(receiver, firstReceiver, action);
    }

    @action("inc")
    inc(account: Name): void {
        requireAuth(account);

        let mi = Counter.new(account);
        let it = mi.find(account.N);
        let count: u64 = 0;
        let payer: Name = account;

        if (it.isOk()) {
            let counter = mi.get(it)
            counter.count += 1;
            mi.update(it, counter, payer);
            count = counter.count;
        } else {
            let counter = new Counter(account, 1);
            mi.store(counter, payer);
            count = 1;
        }
        print(`++++++++count:${count}`);
    }

    @action("testremove")
    testRemove(account: Name): void {
        requireAuth(account);
        let mi = Counter.new(account);
        let it = mi.find(account.N);
        check(it.isOk(), "account not found");
        mi.remove(it);
    }

    @action("testbound")
    testBound(): void {
        let table = Counter.new(this.receiver);
        let payer = this.receiver;

        let value = new Counter(new Name(1), 1);
        table.store(value, payer);

        value = new Counter(new Name(3), 1);
        table.store(value, payer);

        value = new Counter(new Name(5), 1);
        table.store(value, payer);

        let it = table.lowerBound(1);
        check(it.isOk() && it.primary == 1, "bad value");
        print(`+++++db.lower_bound(1) return primary key: ${it.primary}\n`);

        it = table.upperBound(3);
        check(it.isOk() && it.primary == 5, "bad value");
        print(`+++++db.lower_bound(3) return primary key: ${it.primary}\n`);
    }
}
