import {
    Name,
    Table,
    U128,
    U256,
    newSecondaryValue_u64,
    newSecondaryValue_f64,
    newSecondaryValue_U128,
    newSecondaryValue_U256,
    getSecondaryValue_Float128,
    printString,
    printHex,
    check,
    Float128,
    Contract,
    print,
    newSecondaryValue_Float128,
    Encoder,
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
    @action("test1")
    testmi1(): void {
        let mi = MyData.new(this.receiver);
        let value = new MyData(1, 2, new U128(3));//999999.99999
        mi.store(value, this.receiver);
        printHex(Encoder.pack(new U256(11)));
    }

    @action("test2")
    testmi2(): void {
        let mi = MyData.new(this.receiver);
        mi = MyData.new(this.receiver, this.receiver);

        let value = new MyData(1, 2, new U128(3));
        mi.store(value, this.receiver);

        value = new MyData(4, 5, new U128(6));
        mi.store(value, this.receiver);

        value = new MyData(7, 8, new U128(9));
        mi.store(value, this.receiver);

        let it = mi.find(4);
        check(it.isOk(), "value not found!");
        printString(`+++++++++++it.i:${it.i}\n`);
        value = it.getValue()!;
        printString(`+++++++++++it.i:${value.a}, ${value.b}, ${value.c}\n`);
        check(value.a == 4 && value.b == 5 && value.c == new U128(6), "bad value 1");

        print("++++++++++++++test IDX64++++++++++++++\n");
        {
            let idx = mi.bvalueDB;
            let idxIt = idx.findPrimary(7);
            printString(`++++++++${idxIt.i.i}, ${idxIt.value}\n`);
    
            {// 4, 5, 6
                // let idx64 = <IDX64>idx;
                let idxIt = idx.find(5);
                printString(`+++++++++idx64.find: ${idxIt.i}, ${idxIt.primary}\n`);
                check(idxIt.primary == 4, "bad value 6");
            }
    
            // 1 2 3
            // 4 5 6
            // 7 8 9
            {
                let secondary = newSecondaryValue_u64(2);
                let ret = idx.lowerBoundEx(secondary);
                check(ret.value == 2, "bad value 7");
                ret = idx.upperBoundEx(2);
                check(ret.value == 5, "bad value 8");
            }
        }

        print("++++++++++++++test IdxUpdate++++++++++++++");
        // 1 2 3 3.3 11
        {
            let idx = mi.bvalueDB;
            let idxIt = idx.find(2);
            printString(`+++++++++idx.find(2): ${idxIt.i}, ${idxIt.primary}\n`);
            check(idxIt.primary == 1, "bad value 9");
            mi.updateBvalue(idxIt, 22, this.receiver);
            let ret = idx.find(22);
            check(ret.isOk(), "bad scondary value 10");
        }
    }
}
