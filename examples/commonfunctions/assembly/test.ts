import {
    Name,
    PermissionLevel,
    Contract,

    requireAuth,
    requireAuth2,
    hasAuth,
    isAccount,
    currentTime,

    print,
    check
} from "asm-chain";

@contract
class MyContract extends Contract {
    constructor(receiver: Name, firstReceiver: Name, action: Name) {
        super(receiver, firstReceiver, action);
    }

    @action("test")
    test(): void {
        let ret = isAccount(Name.fromString("noexits"));
        print(`+++isAccount(noexits): ${ret}\n`);
        ret = isAccount(this.receiver);
        print(`+++isAccount(this.receiver): ${ret}\n`);

        print(`hasAuth: ${hasAuth(this.receiver)}\n`);
        requireAuth(this.receiver);
        requireAuth2(new PermissionLevel(this.receiver, Name.fromString("active")));
        print(`current time: ${currentTime()}\n`);
    }
}
