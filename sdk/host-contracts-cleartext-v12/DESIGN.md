# Design principles — `@fhevm/host-contracts-cleartext`

A cleartext (mock) implementation of the FHEVM host-contract stack for **one protocol version**.
Same contracts, same interfaces, same wire formats as the real protocol — only the FHE math is
replaced by plaintext arithmetic, so a dev node can run the whole protocol without a coprocessor,
KMS, or gateway.

## 1. One package per protocol version

Each FHEVM protocol release gets its own package version (`v0.12`, `v0.13`, `v0.14`, …), differing
**only** by the genuine protocol deltas of that release (e.g. v0.14 adds `ProtocolConfig` KMS node
params and the ConfidentialBridge address). Everything else — file layout, TS API shape, template
mechanism, type names — is kept deliberately identical across versions, with **v0.13 as the
structural source of truth**.

> **This directory is the v0.12 package.** `-v12` and `-v13` sit in this repo only for ease of
> development; they are destined for the corresponding previous release branches of the fhevm repo,
> while `host-contracts-cleartext` (no suffix) tracks the current protocol version (v0.14).

## 2. One package, three consumers

| Consumer | What it uses the package for |
|---|---|
| **js-sdk** (`@fhevm/sdk`) | Mocks the *entire protocol* so the SDK itself can be tested end-to-end: encrypt → input proof → on-chain verification → decrypt, against real contract code. |
| **`sdk/hardhat-plugin`** | FHEVM testing in Hardhat projects (in-process network, `hardhat node`, or anvil). |
| **`sdk/forge-fhevm`** | FHEVM testing in Foundry projects, consuming the package's Solidity sources via remappings (vendors nothing). |

The consumers are thin by design: they adapt the package to their environment, they do not
reimplement any of it.

## 3. The package owns deployment — consumers never deploy

Deployment is the hardest part of the stack (proxy bootstrapping, initializer ordering, ownership
seeding, cross-referenced addresses), so it exists **exactly once**, here. Three entry points, one
per kind of consumer:

- **`ts/deploy.ts`** — CREATE-based, for any real chain: deploy empty proxies, `upgradeToAndCall`
  each to its implementation with `initializeFromEmptyProxy(...)`. Addresses fall out of the deployer
  nonce (`precomputeAddresses`).
- **`ts/deployAt.ts`** — fixed-address placement, for dev nodes driven from TypeScript (anvil,
  hardhat in-process): patch the template bytecode for the caller's address map, `setCodeAt` each
  contract, seed the proxy storage slots (`Initializable`, ACL's `Ownable` owner) via `setStorageAt`,
  then run the same ordered initializer list as `deploy` (`bootstrapInitCalls` — one definition,
  both paths).
- **`src/deploy/FhevmStack.sol`** — fixed-address deployment for Foundry consumers, who cannot call
  the TS library: real ERC-1967 proxies and initializers, with `deployCodeTo` as the only cheat
  (running a proxy constructor at a chosen address). The addresses come from the consumer's
  `fhevm-config` remapping, so the same compile-time constants drive both the contracts and their
  placement.

For a standing local node (anvil or `hardhat node`), the same fixed-address deployment ships in two
invocable forms, both using the ZamaConfig-pinned addresses and the standard mock identity: Foundry
projects run `forge script` on a one-line subclass of **`src/deploy/DeployFhevmStackLocal.s.sol`**
(everything goes through `vm.rpc`, so no `--broadcast` and no key), and npm-side tooling runs
**`npm run deploy:local`** here (wraps `ts/deployAt`). Either way, every consumer — forge scripts,
Hardhat projects, other tooling — finds the same ready stack on the node, and the hardhat plugin
adopts it instead of redeploying.

The fixed-address paths exist because `ZamaConfig._getLocalConfig()` (chainid 31337) hardcodes the
ACL / FHEVMExecutor / KMSVerifier addresses **into the user's compiled contracts**. CREATE can never
land there; only cheat-code placement can. Consumers supply three things the package cannot know:
*how* to reach the node (adapters / forge's `vm`), *where* the stack must live (the address map),
and *what* to initialize it with (signers/config). Nothing else.

## 4. No runtime dependency on viem or ethers

All chain access goes through the package's abstract interfaces — `AbstractEthereumProvider`
(`setCodeAt`, `setStorageAt`, `getCodeAt` — dev-node cheat codes) and `AbstractEthereumSigner`
(`deploy`, `writeContract`, `getAddress`). Any eth library, or a raw EIP-1193 provider, can
implement them. viem appears only as a devDependency, in tests.

## 5. Templates, not artifacts

`templates/*.json` ship the compiled bytecode with **placeholder addresses** plus
`addressReferences` byte offsets; `abi/*.json` ship the interfaces. Patching the offsets yields
bytecode for *any* address map without recompiling — which is what makes `deployAt` (and forge
consumers via the `fhevm-config` remapping) possible. Known trap: solc's constant-pool ordering is
value-dependent, so a single-referenced address constant can move (currently only `HCULimit`);
`test/templates.test.ts` pins the affected set and asserts address-equivalence there,
byte-identity everywhere else.

## 6. Real protocol, cleartext math

Everything except the FHE computation is the genuine article: input-proof layout
(`[numHandles][numSigners][handles][sigs][extraData]` — the extraData doubles as the cleartext
channel), handle metadata checks, EIP-712 domains for input verification and user/public decryption
(including the host-chain-id override for user-decrypt), KMS signature thresholds, ACL permission
checks, HCU accounting. This is what makes the mock a valid test target for the js-sdk: the SDK's
proofs and signatures are verified by the same on-chain code paths as in production.

## 7. Contract architecture in one paragraph

The ownership root is the **ACL** (everything else resolves its owner through `ACL.owner()` via
`ACLOwnable`); all upgradeable contracts are UUPS empty-proxies bootstrapped with
`initializeFromEmptyProxy` (guarded by `onlyFromEmptyProxy`). The executor is split for EIP-170:
**`CleartextFHEVMExecutor`** stays thin and delegates all arithmetic to an external
**`CleartextArithmetic`** contract, which is the *sole writer* of the shared **`CleartextDB`**
plaintext store (write-ACL mirrors `PauserSet`). `Cleartext*` variants exist only for the contracts
that must observe or expose cleartext (Executor, InputVerifier, KMSVerifier); the rest (ACL,
HCULimit, ProtocolConfig, KMSGeneration, PauserSet) are the plain host contracts.

## Historical plans

The original design/working plans live in [`plans/`](plans/) (`FIRST_PLAN.md` — abstract provider +
template mechanism; `CLEARTEXT_EXECUTOR_SPLIT_PLAN.md` — the Arithmetic/DB split; `TS_LIB_PLAN.md` —
atomic upgrades via `ACLOwner`).
