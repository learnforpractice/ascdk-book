import {
    Name,
    Table,
    U128,
    U256,
    printString,
    printHex,
    check,
    Contract,
    print,
} from "asm-chain";

@table("mydata")
class MyData extends Table {
    constructor(
        public a: u64=0,
        public b: u64=0,
        public c: U128=new U128()
    ) {
        super();
    }

    @primary
    get getPrimary(): u64 {
        return this.a;
    }

    @secondary
    get bvalue(): u64 {
        return this.b;
    }

    @secondary
    set bvalue(value: u64) {
        this.b = value;
    }

    @secondary
    get cvalue(): U128 {
        return this.c;
    }

    @secondary
    set cvalue(value: U128) {
        this.c = value;
    }
}

@contract
class MyContract extends Contract{

    @action("test")
    testSecondary(): void {
        let mi = MyData.new(this.receiver);

        let value = new MyData(1, 2, new U128(3));
        mi.store(value, this.receiver);

        value = new MyData(11, 22, new U128(33));
        mi.store(value, this.receiver);

        value = new MyData(111, 222, new U128(333));
        mi.store(value, this.receiver);


        let idx = mi.bvalueDB;    
        let idxIt = idx.find(2);
        printString(`+++++++++idx64.find: ${idxIt.i}, ${idxIt.primary}\n`);
        check(idxIt.primary == 1, "bad value");

        let ret = idx.lowerBound(2);
        check(ret.primary == 1, "bad value");

        ret = idx.upperBound(22);
        check(ret.primary == 111, "bad value");
    }

    @action("testupdate")
    testSecondaryUpdate(): void {
        let mi = MyData.new(this.receiver);
        let idx = mi.bvalueDB;
        let idxIt = idx.find(222);
        check(idxIt.isOk(), "value 222 not found");
        check(idxIt.primary == 111, "bad primary value");
        mi.updateBvalue(idxIt, 223, this.receiver);
        let ret = idx.find(22);
        check(ret.isOk(), "bad scondary value");
    }

    @action("testremove")
    testSecondaryRemove(): void {
        let table = MyData.new(this.receiver);
        let idx = table.bvalueDB;
        let idxIt = idx.find(222);
        check(idxIt.isOk(), "value 222 not found");
        check(idxIt.primary == 111, "bad primary value");
        let primaryIt = table.find(idxIt.primary);
        check(primaryIt.isOk(), "bad primary value");
        table.remove(primaryIt);
    }

}