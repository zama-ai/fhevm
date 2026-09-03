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
- centralize MNEMONIC -> ZamaConfig local + all constants related to it + use same paradigm as cleartext-config.json
- create new branch
- integrate fhevm/sdk
- PROGRESS create a simple GUIDE
- PROGRESS remove harness word, remove payload word
- hardcoded const mnemonic = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer'; is forbidden, import centralized constants
- cleanup /Users/alex/src/me/zama-ai/fhevm/sdk/scripts
- add fhevm-npm auto check polygon addresses everywhere
- add a script that tests all links in package.json (README broken)
- hardhat/v3 plugin `internal/network.ts` classifies public chains from the vendored `fhevm-chains.ts` face; in the future it should import `@fhevm/sdk/chains` instead, once `@fhevm/sdk/chains` is itself auto-generated from `fhevm-chains.config.json` (same source, one copy less)
- Elias bun bug
