# @fhevm/hardhat-plugin

Hardhat plugin for testing FHEVM contracts against the on-chain cleartext engine
(`@fhevm/host-contracts-cleartext`). It provides `hre.fhevm`: encrypted-input building, user/public
decryption, and automatic standing-up of the cleartext host stack on the in-process Hardhat network,
a `hardhat node`, or anvil. Against an external node it adopts a stack that is already there (e.g.
one prepared with the package's `npm run deploy:local`) instead of redeploying.

```ts
const input = await hre.fhevm.createEncryptedInput(contractAddress, user.address).add32(42).encrypt();
await counter.increment(input.handles[0], input.inputProof);
const clear = await hre.fhevm.userDecryptEuint(FhevmType.euint32, handle, contractAddress, user);
```

## What this plugin is — and is not

This is a thin consumer of `@fhevm/host-contracts-cleartext` (see the package's `DESIGN.md`). The
split of responsibilities:

- **The package owns deployment.** The plugin calls the package's `deployAt`, which places the stack
  (bytecode patching, proxy-state seeding, initializer order). Nothing in the plugin deploys or
  places host-contract code.
- **The plugin supplies the three things the package cannot know:**
  - *How* to reach this node — `src/engine/stack/adapters.ts`, implementing the package's abstract
    provider/signer over an EIP-1193 dev node (`src/engine/node.ts`).
  - *Where* the stack must live — `src/engine/stack/addresses.ts` (three addresses are pinned by
    `ZamaConfig`, compiled into the contract under test).
  - *What* to initialize it with — `src/engine/stack/config.ts` (mock signer keys, gateway identity,
    HCU limits).
- **The FHE protocol layer** (`src/engine/fhe/`) plays the off-chain gateway/KMS role: it builds
  input proofs and signs EIP-712 payloads that the REAL on-chain verifiers check.

## Layout

```
src/index.ts               Hardhat glue (extendEnvironment) — all HH coupling lives here
src/api.ts                 hre.fhevm implementation (Hardhat-free)
src/engine/node.ts         EIP-1193 + dev-node cheat codes (anvil / in-process hardhat)
src/engine/stack/          adapters + addresses + bootstrap config + ABI read-back
src/engine/fhe/            encrypt / decrypt / handle / fhetype — the mock off-chain actors
```

## Build order

The plugin imports `@fhevm/host-contracts-cleartext/ts`, whose compiled output (`ts/_cjs`, `ts/_esm`)
is gitignored. On a fresh clone, build the package first:

```sh
cd ../host-contracts-cleartext && npm install && npm run build:templates && npm run build
```
