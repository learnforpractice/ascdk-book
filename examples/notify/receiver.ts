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
