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
