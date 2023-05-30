import { ChainTester } from "chaintester"

it('test hello', async () => {
    let tester = new ChainTester();
    await tester.init();
    try {
        let ret = await tester.deployContract("hello", "./assembly/target/counter.wasm", "./assembly/target/counter.abi");
        expect(ret.except).toBeUndefined();

        ret = await tester.pushAction("hello", "inc", {}, {"hello": "active"});
        expect(ret.except).toBeUndefined();
        await tester.produceBlock();
        ret = await tester.pushAction("hello", "inc", {}, {"hello": "active"});
        expect(ret.except).toBeUndefined();
    } finally {
        await tester.free();
    }
})
