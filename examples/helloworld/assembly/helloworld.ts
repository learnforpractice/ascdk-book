import {
    Name,
    Contract,
    print,
} from "asm-chain";

@contract
class MyContract extends Contract {
    constructor(receiver: Name, firstReceiver: Name, action: Name) {
        super(receiver, firstReceiver, action);
    }

    @action("sayhello")
    say_hello(): void {
        print("++++++++hello, world\n");
    }
}
