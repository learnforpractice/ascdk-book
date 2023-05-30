import {
    Name,
    Contract,

    ripemd160,
    sha1,
    sha256,
    sha512,

    assertRipemd160,
    assertSha1,
    assertSha256,
    assertSha512,

    Signature,
    PublicKey,
    recoverKey,
    assertRecoverKey,

    print,
    check,
} from "asm-chain";

@contract
class MyContract extends Contract {
    constructor(receiver: Name, firstReceiver: Name, action: Name) {
        super(receiver, firstReceiver, action);
    }

    @action("testhash")
    testhash(data: u8[]): void {
        print(`+++++++${data}\n`)
        assertRipemd160(data, ripemd160(data));
        assertSha1(data, sha1(data));
        assertSha256(data, sha256(data));
        assertSha512(data, sha512(data));
        print("+++++done!");
    }

    @action("testrecover")
    test_recover(msg: u8[], sig: Signature, pub_key: PublicKey): void {
        let digest = sha256(msg);
        let _pub_key = recoverKey(digest, sig);
        check(pub_key == _pub_key, "invalid public key");

        assertRecoverKey(digest, sig, pub_key);
        print("++++++++test_recover done!");
    }
}
