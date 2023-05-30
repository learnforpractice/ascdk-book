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
