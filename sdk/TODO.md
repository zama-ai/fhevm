- solve the @fhevm/sdk issue : "dom" lib
  to solve the issue the SDK should add :

```ts
type BufferSource = ArrayBufferView | ArrayBuffer;
type RequestInfo = string;
type Response = unknown;
declare namespace WebAssembly {
  interface Module {}
  interface Memory {
    readonly buffer: ArrayBuffer;
  }
  interface Table {}
}
```

- solve the @fhevm/sdk issue : export bug (chains etc.)
- OK add 2600+ tests of HH v2
- create new branch
- integrate fhevm/sdk
- PROGRESS create a simple GUIDE
- OK remove tarball tests from v12 and v13
- OK move all v12 and v13 old tarball tests to test-consumer/<cjs|esm>/.
- PROGRESS remove harness word, remove payload word
- check-lint policy to fhevm-npm
- remove check-lint from subpackages
- add check-lint policy at the root
- hardcoded const mnemonic = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer'; is forbidden, import centralized constansts
- cleanup /Users/alex/src/me/zama-ai/fhevm/sdk/scripts
- OK move /Users/alex/src/me/zama-ai/fhevm/sdk/hardhat/v2/fhevm-hardhat-template code to inner pkg/ folder
- add fhevm-npm auto check polygon addresses everywhere
- add a script that tests all links in pacakge.sjomn (README broken)
- OK when possible in fhevm-npm use forge config --json when needed instead of manually parsing configs or guessing
