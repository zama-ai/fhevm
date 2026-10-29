# v1.0.0 — cleartext FHEVM host contracts, generation v1.0

This folder holds the Solidity sources of the **cleartext** FHEVM host-contract stack, protocol generation
**v1.0** (`@fhevm/host-contracts-cleartext@1.0.0-0`), so the js-sdk local mock test suite can deploy it on anvil
next to `v0.12.0/`, `v0.13.0/` and `v0.16.0/`. This file covers everything needed to compile and deploy it. You
do not need to have seen fhevm-mocks.

Nothing outside this folder has been changed yet. No foundry profile, no deploy script and no test-runner
entry exists for it. Section 7 lists that wiring.

Paths written `fhevm-mocks:<path>` are files in `zama-ai/fhevm-mocks` at commit `c71685c`, under
`host-contracts-cleartext/v1.0/` unless they start with `/`. You can read any of them with
`git -C <fhevm-mocks checkout> show c71685c:host-contracts-cleartext/v1.0/<path>`.

---

## 1. Provenance

| | |
|---|---|
| copied from | `zama-ai/fhevm-mocks`, branch `devex/alexb/v1.0/new`, commit `c71685cb2712028107b997959cbe4f2f67e325b9` |
| sync history | Initial copy at `8b91ae5`. Re-synced at `585442e`: `cleartext/CleartextPaddedInputter.sol`, `cleartext/shared/LibCleartextPad.sol` and `cleartext/shared/CleartextArithmeticBase.sol` changed (fhevm-mocks `e7008bd` pad block index little-endian, tech-spec#553; `82819de` pad decryption responses signed per share, tech-spec#547; `277da6f` docs; `585442e` the cleartext coprocessor re-checks a pad package's chain and label). Re-synced at `c71685c`: `cleartext/CleartextPaddedInputter.sol` changed (every node's pad decryption response built in one view call, `getCleartextPadResponsesMaterial`; `cleartextDecryptPadPayload` added; `cleartextEncryptPadPayload` and `padDecryptResponseDigest` removed; `PadDecryptRequestPayload.prf` renamed `prfId`). No other file under `pkg/src` changed. |
| package | `host-contracts-cleartext/v1.0/pkg/src/` (npm `@fhevm/host-contracts-cleartext` `1.0.0-0`) |
| vendored upstream | `zama-ai/fhevm` `host-contracts/contracts` @ `10702a23009147252950a69059968b8a20599549`, labelled `v1.0.0-0-g10702a230` (fhevm `main`, no upstream tag yet). Source: `fhevm-mocks:pkg/package.json` → `fhevm.vendoredFrom` |
| local patches on the vendored copy | `ACL.sol`, `ProtocolConfig.sol`, `ProtocolConfigReplica.sol`, `FHEVMExecutor.sol`, `PaddedInputter.sol` (same field; see commit `b0d98bf` in fhevm-mocks for the reasons) |
| older pin | The generation was first vendored at fhevm tag **v0.15.0-3**, then v0.15.0-6, then moved to the commit above. Several comments still say "v0.15.0-3", for example `cleartext/shared/LibFhevmCoprocessorConfig.sol` and `fhevm-mocks:README.md` §1. The committed pin is the commit above. |

`sha.txt` records the same facts. Its first line is the upstream fhevm commit, the same convention as
`../v0.12.0/sha.txt` and `../v0.16.0/sha.txt`.

## 2. What is in the folder (60 files + this notice)

| path | files | origin (`fhevm-mocks:pkg/src/…`) | notes |
|---|---|---|---|
| `host-contracts/contracts/` | 30 | `contracts/` | Vendored upstream host contracts, byte-identical to the source. The 3 LayerZero-dependent bridge sources are left out (see below). |
| `host-contracts/addresses/FHEVMHostAddresses.sol` | 1 | `addresses/FHEVMHostAddresses.sol` | The `*Add` aliases every contract imports. **One import rewritten** (see below). |
| `config/addresses.sol` | 1 | *new*, the file `fhevm-config-1.0.0/addresses.sol` resolves to | The 12 real `*_ADDRESS` constants. Committed with the canonical local set (§4). It is **regenerated per deployer/nonce**. |
| `cleartext/` (+ `shared/`, `shared/interfaces/`) | 27 | `cleartext/` | The cleartext implementations and their libraries. Relative imports rewritten. |
| `erc1967/ERC1967Proxy.sol` | 1 | `erc1967/ERC1967Proxy.sol` | A thin wrapper over OZ 5.1.0 `ERC1967Proxy`. Unchanged. |
| `upgrade/ACLOwner.sol` | 1 | `upgrade/ACLOwner.sol` | The standing owner of the stack (new in this deploy flow). Unchanged. |
| `sha.txt` | 1 | — | Provenance. |

### Adaptations to the source

All edits are path-only. No Solidity logic was touched.

1. **Layout.** The layout follows the siblings: upstream contracts go under `host-contracts/contracts/`, the address aliases under
   `host-contracts/addresses/`, and the cleartext variants under `cleartext/`. Every relative import in `cleartext/`
   was rewritten to match (24 lines, 11 files):
   - `cleartext/*.sol`: `"../contracts/…"` → `"../host-contracts/contracts/…"`, `"../addresses/…"` → `"../host-contracts/addresses/…"`
   - `cleartext/shared/*.sol`: `"../../contracts/…"` → `"../../host-contracts/contracts/…"`

   Imports inside `host-contracts/contracts/` (`"../addresses/FHEVMHostAddresses.sol"`) already resolve and are
   untouched.
2. **The config remapping replaced by a relative import.** Upstream fhevm-mocks has
   `import {...} from "fhevm-config-1.0.0/addresses.sol";` in `FHEVMHostAddresses.sol`, resolved by a remapping
   (`fhevm-mocks:remappings.txt`: `fhevm-config-1.0.0/=internal/placeholders/`, overridden at deploy time with
   `FOUNDRY_REMAPPINGS=fhevm-config-1.0.0/=internal/.deploy-config/`). Here it is
   `import {...} from "../../config/addresses.sol";`.
   **Why:** `contracts/foundry.toml` compiles all of `src/` in every profile. An import that only a new
   remapping can resolve would break `forge build` for every existing profile until `foundry.toml` were edited.
   With the relative import the folder compiles with the current config, as the siblings do. They commit a
   real-address `FHEVMHostAddresses.sol`, and their deploy regenerates it.
   *Alternative, not done:* revert that one import to `fhevm-config-1.0.0/addresses.sol` and add
   `"fhevm-config-1.0.0/=./src/v1.0.0/config/"` to **every** profile's `remappings` (default, v12, v13, v16 and the new one).
3. **Left out:** `contracts/bridge/{ConfidentialBridge,HandlesSender,HandlesReceiver}.sol`. They import
   `@layerzerolabs/*`, which this repo does not have. fhevm-mocks never compiles them either (`skip` in
   `fhevm-mocks:foundry.toml`), and a cleartext stack never bridges. `ACL` only needs
   `bridge/interfaces/IConfidentialBridge.sol` (kept), and `confidentialBridgeAdd` is a literal `address(0)`.
   `bridge/BridgeEvents.sol` and `bridge/interfaces/IDstApp.sol` are dependency-free and kept.
4. **Not copied** (non-Solidity or not part of the product): `pkg/forge/**` (forge-only deploy and test tooling with
   pre-compiled bytecode), `pkg/ts`, `pkg/abi`, `pkg/templates`, `internal/placeholders/addresses.sol`
   (marker addresses for bytecode templating; not needed when compiling against real addresses),
   `create2-deploy/` (testnet CREATE2 path), tests.
5. `cleartext/CleartextForge{ACL,Arithmetic,FHEVMExecutor,HCULimit}.sol` are kept because they are part of
   `pkg/src`. They import `forge-std/Vm.sol` and call cheatcodes, so they are only for an in-process
   forge stack (`fhevm-mocks:pkg/forge/src/LibForgeFhevmStack.sol`). **Do not deploy them on anvil.** The anvil
   path uses the non-Forge variants.

## 3. Compiling

**No config change is needed to compile.** These commands were run (forge 1.5.1), with output sent to a temp dir
so the repo's `out/` and `cache/` stay untouched:

```sh
cd sdk/js-sdk/contracts
# only the new folder, with this repo's foundry.toml, per profile
for p in default v12 v13 v16; do
  FOUNDRY_PROFILE=$p forge build --contracts src/v1.0.0 --out "$TMP/out-$p" --cache-path "$TMP/cache-$p"
done                                   # → "Compiler run successful" (solc 0.8.33 auto-detected), 120 files
# the whole tree (all generations + v1.0.0)
FOUNDRY_PROFILE=default forge build --out "$TMP/full" --cache-path "$TMP/fullcache"   # → successful, 218 files
FOUNDRY_PROFILE=v16     forge build --out "$TMP/full16" --cache-path "$TMP/fullcache16"  # → successful
```

The source's own settings (`fhevm-mocks:foundry.toml`) are solc `0.8.24`, `evm_version = cancun`,
`optimizer = true`, `optimizer_runs = 800`, `via_ir = false`, `bytecode_hash = "none"`. This repo has the same
optimizer and `via_ir` and auto-detects solc. The folder also compiles with `--use 0.8.24 --evm-version cancun`
(verified). Nothing here needs `via_ir`.
Dependencies: `@openzeppelin/contracts` and `@openzeppelin/contracts-upgradeable` **5.1.0** (same as the source
pins), and `forge-std` (only for the `CleartextForge*` files). All resolve through the existing remappings.

Code size with solc 0.8.33 and 800 runs: the largest are `CleartextKMSVerifier` at 23,094 B, `CleartextFHEVMExecutor`
at 22,834 B and `CleartextHCULimit` at 20,511 B, all under EIP-170. The test anvil also runs with
`--disable-code-size-limit`.

### Optional profile (convention only)

Imports are relative, so no remapping is required. To match the siblings, add a profile. The name is your choice,
for example `v1`, and §7 assumes `v1`:

```toml
[profile.v1]
remappings = [ <the 7 shared lines of [profile.v16]>,
    "@fhevm/host-contracts/=./src/v1.0.0/host-contracts/",
    "@fhevm/cleartext/=./src/v1.0.0/cleartext/",
    "@fhevm/host-contracts-1.0.0/=./src/v1.0.0/host-contracts/",
    "@fhevm/cleartext-1.0.0/=./src/v1.0.0/cleartext/",
]
```

and add `{ access = "write", path = "src/v1.0.0/config" }` to `[profile.default].fs_permissions`. The
address writer (§4) needs it.

## 4. The address config: `config/addresses.sol`

Most contracts **compile in** the addresses of their sister contracts:

- `ACL` bakes the executor, `PauserSet` and the bridge (0).
- `FHEVMExecutor` bakes ACL, HCULimit, InputVerifier and PaddedInputter.
- `CleartextFHEVMExecutor` bakes CleartextArithmetic and PaddedInputter.
- `CleartextArithmetic` bakes CleartextDB.
- `CleartextKMSVerifier` and `CleartextPaddedInputter` bake ACL, KMSVerifier and ProtocolConfig.
- `EmptyUUPSProxy` and `PauserSet` bake ACL, via `ACLOwnable`.

So the order is fixed: compute the addresses, write `config/addresses.sol`, compile, then deploy. In fhevm-mocks this is
`fhevm-mocks:scripts/deploy.sh` (steps 1–3), `pkg/forge/script/ComputeAddresses.s.sol` and
`pkg/forge/script/FhevmDeployScript.s.sol`.

Every address is `CREATE(deployer, N + offset)`, where `N` is the deployer's nonce when the deploy starts:

| offset | contract created | constant in `config/addresses.sol` | canonical address (deployer `0x8B8f…0DF4`, N = 0) |
|---|---|---|---|
| +0 | `EmptyUUPSProxyACL` (impl. of the ACL proxy) | — | `0x34e3eD8472e409dbF8FDf933cA996DC75e4Be126` |
| +1 | `ERC1967Proxy` → **ACL** | `ACL_ADDRESS` | `0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D` |
| +2 | `EmptyUUPSProxy` (shared impl. of every other proxy) | — | `0xCD3ab3bd6bcc0c0bf3E27912a92043e817B1cf69` |
| +3 | `ERC1967Proxy` → **FHEVMExecutor** | `FHEVM_EXECUTOR_ADDRESS` | `0xe3a9105a3a932253A70F126eb1E3b589C643dD24` |
| +4 | `ERC1967Proxy` → **KMSVerifier** | `KMS_VERIFIER_ADDRESS` | `0x901F8942346f7AB3a01F6D7613119Bca447Bb030` |
| +5 | `ERC1967Proxy` → **InputVerifier** | `INPUT_VERIFIER_ADDRESS` | `0x36772142b74871f255CbD7A3e89B401d3e45825f` |
| +6 | `ERC1967Proxy` → **HCULimit** | `HCU_LIMIT_ADDRESS` | `0x233ff88A48c172d29F675403e6A8e302b0F032D9` |
| +7 | `ERC1967Proxy` → **ProtocolConfig** | `PROTOCOL_CONFIG_ADDRESS` | `0x44aA028fd264C76BF4A8f8B4d8A5272f6AE25CAc` |
| +8 | `ERC1967Proxy` → **KMSGeneration** | `KMS_GENERATION_ADDRESS` | `0x216be43148dB537BeddBC268163deb1a802b5553` |
| +9 | `ERC1967Proxy` → **PaddedInputter** (RFC-030, new in v1.0) | `PADDED_INPUTTER_ADDRESS` | `0xded0D2a71268DC12622BdD1b55d68a1CB5662327` |
| +10 | `ERC1967Proxy` → **CleartextArithmetic** | `CLEARTEXT_ARITHMETIC_ADDRESS` | `0x6933Afcf0F4bCE1A611baD0A6FaafF0337a7ba1E` |
| +11 | `ERC1967Proxy` → **CleartextDB** | `CLEARTEXT_DB_ADDRESS` | `0x590e3330386Fa042843773541aaBb3a45EC3164D` |
| +12 | **PauserSet** (immutable, no proxy) | `PAUSER_SET_ADDRESS` | `0xcF5De2DbC4Ea85426db47845537A90ec4e6316E2` |
| +13 | `ERC1967Proxy` → **ProtocolConfigReplica** | `PROTOCOL_CONFIG_REPLICA_ADDRESS` | `0x492e52843AE2eCF9daae6B951E12C3614Ee115C1` |
| +14 | **ACLOwner** (no contract references it, so it is not in the file) | — | `0xc8D844BcF85739dCCC1DDB972Ba35A3e39402F3F` |

Sources:

- `fhevm-mocks:/cleartext-config.json` → `localhost.generations["1.0.0"].nonces` (+0..+13; `PROXY_COUNT = 11`,
  `ADDRESSED_NONCE_COUNT = 14`).
- `fhevm-mocks:pkg/forge/src/_internal/LocalHostAddresses.sol`.
- `fhevm-mocks:pkg/ts/addresses.ts` (`HOST_NONCE_OFFSET`).
- +14 comes from `fhevm-mocks:scripts/deploy.sh` (`EXPECTED_ACL_OWNER`). It was recomputed with `cast compute-address` and observed in the simulation (§6).

The canonical deployer is account index **5** of the mnemonic
`adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer`
(`m/44'/60'/0'/0/5` → `0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4`), at start nonce **0**. This is the
same deployer and mnemonic as `scripts/fhevm-deploy.sh` (`deployer_mnemonic_index=5`). ACL, FHEVMExecutor and
KMSVerifier therefore land on the three addresses `library-solidity/config/ZamaConfig.sol` `_getLocalConfig()` returns
for chain 31337, and InputVerifier and ProtocolConfig land where `test/chains/localcleartext.ts` expects them.

The committed `config/addresses.sol` holds exactly this canonical set. **Regenerate it whenever the deployer or the
start nonce differs.** For example, the multi-chain tests set `DEPLOYER_MNEMONIC=FIRST_ANVIL_MNEMONIC` for the legacy
slot (`test/infra/config.ts`). After the deploy, restore the canonical file the way `test/infra/anvil/anvils.ts` does for the
siblings' `FHEVMHostAddresses.sol`. The file must declare all 12 constants with exactly these names. The
`WriteAddresses` contract in Appendix A writes it. Never edit `host-contracts/addresses/FHEVMHostAddresses.sol`:
it is a fixed alias file, unlike the siblings' generated `FHEVMHostAddresses.sol`.

`forge script` recompiles changed sources before running, so writing `config/addresses.sol` and then running
the deploy script in a second `forge script` invocation is enough. This is the same mechanism the siblings rely on.
fhevm-mocks guards against a stale build by checking that no placeholder marker survives. Here the deploy script's
per-CREATE `require(address == expected)` checks do the same job.

## 5. Deployment procedure (local anvil)

Preconditions:

- anvil runs with `--chain-id 31337 --disable-code-size-limit`, as `test/scripts/localcleartext-run-tests.sh` starts it.
- The deployer is funded (`anvil_setBalance`, as `fhevm-deploy.sh` does) and **has sent nothing** (nonce N = 0 for
  the canonical addresses).
- No other transaction is sent from the deployer between computing the addresses and the end of nonce +13.
- Under `forge script --broadcast` every call is a transaction and consumes a nonce. Use `--slow`, as the siblings do.

Only one key is needed. fhevm-mocks deploys everything from the deployer, and the `ACLOwner` owner (the "admin") is
also the deployer (`fhevm-mocks:pkg/forge/script/FhevmDeployScript.s.sol`). The siblings' second key
(`EMPTY_UUPS_MNEMONIC`, index 100) is **not** used.

### Transaction list (all from the deployer; nonce = N + k)

| k | transaction | arguments |
|---|---|---|
| 0 | create `EmptyUUPSProxyACL` | — |
| 1 | create `ERC1967Proxy(impl = <k0>, data = EmptyUUPSProxyACL.initialize(deployer))` | the ACL's initial owner is the deployer; must equal `ACL_ADDRESS` |
| 2 | create `EmptyUUPSProxy` | — (bakes `ACL_ADDRESS` for `onlyACLOwner` upgrades) |
| 3…11 | create `ERC1967Proxy(impl = <k2>, data = EmptyUUPSProxy.initialize())`, 9 times, in this order: FHEVMExecutor, KMSVerifier, InputVerifier, HCULimit, ProtocolConfig, KMSGeneration, PaddedInputter, CleartextArithmetic, CleartextDB | each must equal its constant |
| 12 | create `PauserSet` | no args; must equal `PAUSER_SET_ADDRESS` |
| 13 | create `ERC1967Proxy(impl = <k2>, EmptyUUPSProxy.initialize())` → ProtocolConfigReplica | must equal `PROTOCOL_CONFIG_REPLICA_ADDRESS` |
| 14 | create `ACLOwner(initialOwner = deployer, aclAddress = ACL_ADDRESS)` | address is free |
| 15 | `PauserSet.addPauser(<ACLOwner>)` | **must** happen while the deployer still owns ACL (`onlyACLOwner`). Add any extra pausers here too (see §8) |
| 16 | `ACL.transferOwnership(<ACLOwner>)` | Ownable2Step offer |
| 17 | `ACLOwner.acceptACLOwnership()` | ACL owner is now `ACLOwner`; `ACLOwner.owner()` is the deployer |
| 18…28 | create 11 implementations: `CleartextACL`, `CleartextFHEVMExecutor`, `CleartextKMSVerifier`, `CleartextInputVerifier`, `CleartextHCULimit`, `ProtocolConfig`, `KMSGeneration`, `CleartextPaddedInputter`, `CleartextArithmetic`, `CleartextDB`, `ProtocolConfigReplica` | addresses free; note `ProtocolConfig` and `KMSGeneration` are the **vendored** contracts: v1.0 has no cleartext variant of them, unlike `../v0.16.0/cleartext/CleartextProtocolConfig.sol` / `CleartextKMSGeneration.sol` |
| 29 | `ACLOwner.upgrade(Op[11])`, one atomic tx (≈3.4 M gas); each `Op = (proxy, implementation, initData)`, executed as `proxy.upgradeToAndCall(implementation, initData)` | see the table below |

The deployer ends at nonce N + 30. That was observed in the simulation.

### The 11 `ACLOwner.upgrade` ops (order as in `FhevmDeployScript._materialize`)

| # | proxy | implementation | initData |
|---|---|---|---|
| 0 | ACL | `CleartextACL` | `ACL.initializeFromEmptyProxy()` (keeps the current owner) |
| 1 | FHEVMExecutor | `CleartextFHEVMExecutor` | `FHEVMExecutor.initializeFromEmptyProxy()` |
| 2 | KMSVerifier | `CleartextKMSVerifier` | `KMSVerifier.initializeFromEmptyProxy(address verifyingContractSource = DECRYPTION_ADDRESS, uint64 chainIDSource = GATEWAY_CHAIN_ID)` (no signers: since v13 they live in ProtocolConfig) |
| 3 | InputVerifier | `CleartextInputVerifier` | `InputVerifier.initializeFromEmptyProxy(address verifyingContractSource = INPUT_VERIFICATION_ADDRESS, uint64 chainIDSource = GATEWAY_CHAIN_ID, address[] initialSigners = coprocessorSigners, uint256 initialThreshold = COPROCESSOR_THRESHOLD)` |
| 4 | HCULimit | `CleartextHCULimit` | `HCULimit.initializeFromEmptyProxy(uint48 hcuCapPerBlock = 281474976710655, uint48 maxHCUDepthPerTx = 5000000, uint48 maxHCUPerTx = 20000000)` |
| 5 | ProtocolConfig | `ProtocolConfig` | `ProtocolConfig.initializeFromEmptyProxy(KmsNodeParams[] nodes, KmsThresholds thresholds, string softwareVersion = "0.0.0-cleartext", PcrValues[] pcrValues = [])` (activates KMS context `0x07…01` and epoch `0x08…01`) |
| 6 | KMSGeneration | `KMSGeneration` | `KMSGeneration.initializeFromEmptyProxy()` |
| 7 | PaddedInputter | `CleartextPaddedInputter` | `PaddedInputter.initializeFromEmptyProxy(uint16[] initialPrfInstances = [1])` (RFC-030 PRF instance `CLEARTEXT_PAD_PRF_INSTANCE = 1`, never 0) |
| 8 | CleartextArithmetic | `CleartextArithmetic` | `CleartextArithmetic.initializeFromEmptyProxy()` |
| 9 | CleartextDB | `CleartextDB` | `CleartextDB.initializeFromEmptyProxy(address initialWriter = CLEARTEXT_ARITHMETIC_ADDRESS)` |
| 10 | ProtocolConfigReplica | `ProtocolConfigReplica` | `ProtocolConfigReplica.initializeFromCanonical(uint256 kmsContextId = KMS_CONTEXT_COUNTER_BASE + 1 = 0x07<<248 + 1, uint256 epochId = EPOCH_COUNTER_BASE + 1 = 0x08<<248 + 1, KmsNodeParams[] nodes, KmsThresholds thresholds)` with the **same** nodes and thresholds as op 5 |

Encode each initializer against the **declaring** contract (`ACL`, `FHEVMExecutor`, `HCULimit`, `PaddedInputter`,
…), not the cleartext subclass. solc cannot form a function pointer to an inherited member through the derived type.
The selector is the same.

### Bootstrap values (`fhevm-mocks:pkg/forge/src/_internal/LocalHostBootstrap.sol`, generated from `/cleartext-config.json`)

| value | fhevm-mocks default | js-sdk siblings today (`scripts/fhevm-deploy.sh`) |
|---|---|---|
| gateway chain id (`GATEWAY_CHAIN_ID`) | `100733346448153` (`uint48(keccak256("fhevm.cheat.chainId cleartext gateway"))`) | **`654321`**, also hard-coded in `test/chains/localcleartext.ts` (`gateway.id`) and `test/infra/config.ts` |
| `DECRYPTION_ADDRESS` (KMSVerifier EIP-712 verifyingContract, domain name `"Decryption"`) | `0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721` | same |
| `INPUT_VERIFICATION_ADDRESS` (InputVerifier EIP-712 verifyingContract, domain name `"InputVerification"`) | `0x6189F6c0c3E40B4a3c72ec86262295D78d845297` | same |
| coprocessor signers (`m/44'/60'/0'/2/i` of `test test test test test test test future home engine virtual motion`, i = 0..3) | `0x6727b56B5D1E990C75fE5dCd572D084fc5DC35cb`, `0xF1CDC3ebe04A063EAbfaf5c3129494F6049Ee229`, `0x46A03377bcBE895407ee4842b63f3249bc8327BD`, `0xFd0ea258B3D2b153DD76CAC0E2270574Da8f18E8` | same pool (4 signers) |
| coprocessor threshold | `4` | `1` |
| KMS signers (`m/44'/60'/0'/3/i`, i = 0..3) | `0x649c125E183640141C2AC329673A22dF9FA81b3C`, `0xDB05A2EdF583E3b1644A59434a163aD178595EAf`, `0x03D684498fB1D52286D09Dd42e720c89b55C63c3`, `0x8252dD4D1350BCd34f419aBD05f8cfA146d71C5D` | same pool (4 nodes) |
| KMS tx senders (`m/44'/60'/0'/4/i`) | `0x4c41862B9eb691CA197E97a53219E5d860d9dDbB`, `0x822D412944cE9CD7C92f96F162307cF4c9f88F15`, `0xA47E4a9AeaB93C7c2aC8fd5c4E4eC5cB70DD5F8f`, `0xed3C9f5a6d4D8c00A375B8c79dbDF0f69BAAe547` | same |
| `KmsNodeParams[i]` other fields | `ipAddress "127.0.0.<i+1>"`, `storageUrl "s3://kms-bucket-<i+1>"`, `partyId i+1`, `mpcIdentity "kms-core-<i+1>-cleartext-core-<i+1>"`, `caCert 0x`, `storagePrefix "cleartext-<i+1>"` | — |
| KMS thresholds `{publicDecryption, userDecryption, kmsGen, mpc}` | all `4` (= node count) | all `1` |
| KMS `softwareVersion` / `pcrValues` | `"0.0.0-cleartext"` / `[]` | — |
| HCU | cap/block `281474976710655`, depth/tx `5000000`, per tx `20000000` | same |
| PaddedInputter PRF instances | `[1]` | n/a (v0.16.0 has no PaddedInputter proxy) |

**Recommendation for the js-sdk mock suite:** keep `GATEWAY_CHAIN_ID = 654321`, or the EIP-712 domains will not
match `test/chains/localcleartext.ts` and every signature will fail. Thresholds of 1 or 4 both work. The js-sdk
cleartext relayer (`src/core/modules/relayer/cleartext/signers.ts`) derives 20 keys per pool from the same mnemonic
and paths and reads the signer set on-chain. Both parameter sets were simulated successfully (§6).

## 6. How this procedure was verified (no anvil)

The reference script in **Appendix A** was run as an in-memory `forge script`, with no `--rpc-url` and no
network, from the deployer key above. It ran in a scratch Foundry project whose `src/v1.0.0` is byte-identical to
this folder and whose remappings match this repo's `foundry.toml`.

- `DeployAndVerify` with the fhevm-mocks defaults: success. Every CREATE landed on its expected address,
  ACLOwner = `0xc8D844BcF85739dCCC1DDB972Ba35A3e39402F3F`, the deployer ended at nonce 30, and every check in
  `DeployV1.verify` passed.
- The same run with `GATEWAY_CHAIN_ID = 654321`, `COPROCESSOR_THRESHOLD = 1` and KMS thresholds `1`: success.
- `WriteAddresses` for the canonical deployer produced exactly the 12 constants committed in `config/addresses.sol`.
  For the `FIRST_ANVIL_MNEMONIC` deployer (index 5), the regenerated config followed by `DeployAndVerify` succeeded
  (ACL at `0xB2F34cBa704F4F460bFa6718Da9ed1B8CfbDA200`).

Run it the same way once wired:
`DEPLOYER_PRIVATE_KEY=$(cast wallet private-key --mnemonic "<mnemonic>" --mnemonic-index 5) forge script scripts/v1.0.0/DeployCleartextFHEVMHost.s.sol:DeployAndVerify`.

## 7. Wiring checklist (for the next step; none of it is done)

Use the v0.16.0 path as the template: `scripts/fhevm-deploy.sh` → `scripts/v0.16.0/DeployCleartextFHEVMHost.s.sol`
→ `scripts/v0.16.0/libraries/DeployLib.sol`.

1. `contracts/foundry.toml`:
   - add `[profile.v1]` (optional, §3);
   - add `fs_permissions` write on `src/v1.0.0/config`.
2. `contracts/scripts/v1.0.0/`:
   - Appendix A as `DeployCleartextFHEVMHost.s.sol`.
   - Appendix B as `LocalHostBootstrap.sol`. Set `GATEWAY_CHAIN_ID = 654321` and pick thresholds (§5). Thresholds are
     `COPROCESSOR_THRESHOLD` there, and `_kmsThresholds()` plus the `== 4` check in `verify` in the script.
   - Optionally read the mnemonic, index and thresholds from the same env vars `FhevmConfigLib` uses (`../v0.16.0/libraries/`).
   - Provide equivalents of the contracts `fhevm-deploy.sh` calls:
     - `PrintFhevmSigners`: JSON `{deployer:{address}, emptyUupsDeployer:{address:0x0…0}}`.
     - `WriteFHEVMHostAddressesDotSol`: here `WriteAddresses`, which writes **`src/v1.0.0/config/addresses.sol`**.
     - `PrintFHEVMHostAddressesDotSol` (dry run).
     - `Deploy`.
     - `Verify`, which needs `ACL_OWNER_ADDRESS = CREATE(deployer, N+14)`.
3. `contracts/scripts/fhevm-lib.sh`:
   - `fhevm_assert_foundry_profile`: accept `v1`.
   - `fhevm_host_addresses_file v1`: return `…/src/v1.0.0/config/addresses.sol`, the file to regenerate.
4. `contracts/scripts/fhevm-deploy.sh`: map `v1 → host_contracts_version="v1.0.0"`, and consider `LATEST_PROFILE_ALIAS`.
5. `test/scripts/localcleartext-run-tests.sh`: accept `v1` in both `case "$PROFILE"` checks.
6. `test/chains/`: a `localcleartext_v1` entry in `chain-defaults.json` and a chain definition if one is needed. The
   addresses match `localcleartext.ts` for the canonical deployer; v1.0 adds `paddedInputter` at `0xded0…2327`.
7. `test/infra/config.ts` and `test/infra/anvil/anvils.ts`: update the slot and `profileVersionDir` if v1.0 becomes a slot.
   anvils.ts restores the regenerated address file after deploy, which here is `src/v1.0.0/config/addresses.sol`.
8. `scripts/fhetest-deploy.sh` (FHETest) runs after the stack. It sends nothing as the ACL owner, so it is unaffected
   by ACLOwner owning the ACL, but check it with the new profile.

## 8. Differences from the closest sibling deploy (`v0.16.0`, `scripts/v0.16.0/libraries/DeployLib.sol`)

| | v0.16.0 (current) | v1.0.0 |
|---|---|---|
| pinned nonces | 0..9 | 0..13 (+14 ACLOwner) |
| PaddedInputter | abstract, inherited by FHEVMExecutor (no own address) | **own proxy at +9**, `CleartextPaddedInputter`, init `[1]` |
| `0xded0D2a71268DC12622BdD1b55d68a1CB5662327` (nonce 9) | **PauserSet** | **PaddedInputter**. PauserSet moves to nonce 12 (`0xcF5De2Db…16E2`) |
| CleartextArithmetic / CleartextDB | not separate contracts | own proxies at +10 / +11; CleartextDB writer = CleartextArithmetic |
| ProtocolConfigReplica | not deployed (source present) | proxy at +13, `initializeFromCanonical(...)` |
| ProtocolConfig / KMSGeneration impl | `CleartextProtocolConfig` / `CleartextKMSGeneration` | vendored `ProtocolConfig` / `KMSGeneration` |
| upgrade mechanism | deployer EOA calls `upgradeToAndCall` per proxy | standing `ACLOwner`; one atomic `ACLOwner.upgrade(ops)` |
| final ACL owner | deployer EOA | **ACLOwner** (`ACLOwner.owner()` = deployer). Later owner-gated calls (`addPauser`, `defineNewKmsContextAndEpoch`, HCU setters…) must go through `ACLOwner.execute(target, data)` |
| pausers | `NUM_PAUSERS=2` from the pausers mnemonic index 2 | ACLOwner, plus optional `PAUSER_ADDRESS_0` in fhevm-mocks. Add the env pausers at k = 15, **before** the ownership transfer, or later via `ACLOwner.execute` |
| empty-proxy deployer | `EMPTY_UUPS_MNEMONIC` declared (unused by DeployLib) | not used |
| address file regenerated | `src/v0.16.0/host-contracts/addresses/FHEVMHostAddresses.sol` (lowercase `*Add`) | `src/v1.0.0/config/addresses.sol` (`*_ADDRESS`; `FHEVMHostAddresses.sol` aliases them) |
| cleartext markers | — | `IS_CLEARTEXT()` on CleartextACL, CleartextHCULimit, CleartextArithmetic; `CleartextACL.CLEARTEXT_PROTOCOL_VERSION() == 1000000` |

## 9. Post-deploy checks

These are implemented in `DeployV1.verify` in Appendix A. The full list is in `fhevm-mocks:pkg/forge/script/VerifyFhevmDeploy.s.sol`.

- `cast call <ACL> "IS_CLEARTEXT()(bool)"` returns `true`. The same holds for HCULimit and CleartextArithmetic.
- `cast call <ACL> "CLEARTEXT_PROTOCOL_VERSION()(uint256)"` returns `1000000`.
- `getVersion()`, as in `fhevm-mocks:pkg/ts/versions.ts`. Cleartext subclasses report their base name:

  | contract | version string |
  |---|---|
  | ACL | `ACL v0.6.0` |
  | FHEVMExecutor | `FHEVMExecutor v0.7.0` |
  | KMSVerifier | `KMSVerifier v0.5.0` |
  | InputVerifier | `InputVerifier v0.3.0` |
  | HCULimit | `HCULimit v0.4.0` |
  | ProtocolConfig | `ProtocolConfig v0.3.0` |
  | KMSGeneration | `KMSGeneration v0.3.0` |
  | PaddedInputter | `PaddedInputter v0.1.0` |
  | CleartextArithmetic | `CleartextArithmetic v0.6.0` |
  | CleartextDB | `CleartextDB v0.1.0` |
  | PauserSet | `PauserSet v0.1.0` |
  | ProtocolConfigReplica | `ProtocolConfigReplica v0.3.0` |

- Wiring:
  - `ACL.getFHEVMExecutorAddress()` is the executor.
  - `ACL.getPauserSetAddress()` is PauserSet.
  - `ACL.getConfidentialBridgeAddress()` is `0`.
  - `FHEVMExecutor.getACLAddress()`, `getHCULimitAddress()` and `getInputVerifierAddress()` return their contracts.
  - `HCULimit.getFHEVMExecutorAddress()` is the executor.
- Ownership:
  - `ACL.owner()` is the ACLOwner.
  - `ACLOwner.ACL_ADDRESS()` is the ACL.
  - `PauserSet.isPauser(ACLOwner)` is true.
- Bootstrap:
  - `InputVerifier.getCoprocessorSigners()` and `getThreshold()` return the configured values.
  - `ProtocolConfig.getKmsSigners()` returns the 4 KMS signers.
  - `ProtocolConfig.getCurrentKmsContextId()` and `ProtocolConfigReplica.getCurrentKmsContextId()` both return `0x0700…01`.
  - `PaddedInputter.isSupportedPrfInstance(1)` is true.
  - `HCULimit.getMaxHCUPerTx()` returns `20000000`.
- EIP-712:
  - `KMSVerifier.eip712Domain()` → `("Decryption", chainId = GATEWAY_CHAIN_ID, verifyingContract = DECRYPTION_ADDRESS)`.
  - `InputVerifier.eip712Domain()` → `("InputVerification", GATEWAY_CHAIN_ID, INPUT_VERIFICATION_ADDRESS)`.

## 10. Caveats and open questions

- **Gateway chain id.** fhevm-mocks defaults to `100733346448153`; the js-sdk tests assume `654321`. Pick one
  consistently (§5).
- **SDK support of v1.0 behaviour is not verified.** Two cases are untested: the js-sdk resolving and using a
  standalone `PaddedInputter` proxy (in v0.16.0 the pad flow is inside FHEVMExecutor), and the version strings above
  (e.g. `KMSVerifier v0.5.0`, `ACL v0.6.0`). Check `src/core/host-contracts/` and the pad-based encryption
  (`encryptWithPad`, `fetchPadShares`) code paths before relying on them.
- **The provenance label differs from the task brief.** The committed pin is fhevm `10702a23` (`v1.0.0-0-g10702a230`) with 5
  declared local patches, not tag v0.15.0-3. Stale "v0.15.0-3" comments remain in some sources.
- **Compiler.** This repo auto-selects the newest installed solc (0.8.33 here). fhevm-mocks pins 0.8.24/cancun.
  Both compile, but the bytecode differs from fhevm-mocks' published artifacts. The deploy is only verified
  against itself (as with the siblings).
- **`config/addresses.sol` is regenerated in place.** Like the siblings' `FHEVMHostAddresses.sol`, a deploy with a
  non-canonical deployer dirties it and must restore it afterwards.
- **`CleartextForge*` contracts** are compiled but must not be deployed on anvil (§2.5).
- **ACLOwner** is non-upgradeable. Its owner, the deployer, holds all upgrade authority via `ACLOwner.execute` / `upgrade`.

---

## Appendix A: reference deploy script (verified in-memory, §6)

Intended path: `contracts/scripts/v1.0.0/DeployCleartextFHEVMHost.s.sol`. It is adapted from
`fhevm-mocks:pkg/forge/script/FhevmDeployScript.s.sol`, `ComputeAddresses.s.sol` and `VerifyFhevmDeploy.s.sol`.
`PROXY_COUNT` and the `getVersion` strings are copied from `pkg/forge/src/_internal/LocalHostAddresses.sol` and
`pkg/ts/versions.ts`.

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// Reference deploy for src/v1.0.0. Adapted from fhevm-mocks@8b91ae5
// host-contracts-cleartext/v1.0/pkg/forge/script/FhevmDeployScript.s.sol (+ VerifyFhevmDeploy.s.sol checks).
// Bootstrap values come from ./LocalHostBootstrap.sol (copied verbatim from
// host-contracts-cleartext/v1.0/pkg/forge/src/_internal/LocalHostBootstrap.sol).

import {Script, console} from "forge-std/Script.sol";
import {ERC1967Proxy} from "../../src/v1.0.0/erc1967/ERC1967Proxy.sol";
import {EmptyUUPSProxyACL} from "../../src/v1.0.0/host-contracts/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {EmptyUUPSProxy} from "../../src/v1.0.0/host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ACL} from "../../src/v1.0.0/host-contracts/contracts/ACL.sol";
import {FHEVMExecutor} from "../../src/v1.0.0/host-contracts/contracts/FHEVMExecutor.sol";
import {KMSVerifier} from "../../src/v1.0.0/host-contracts/contracts/KMSVerifier.sol";
import {InputVerifier} from "../../src/v1.0.0/host-contracts/contracts/InputVerifier.sol";
import {HCULimit} from "../../src/v1.0.0/host-contracts/contracts/HCULimit.sol";
import {ProtocolConfig} from "../../src/v1.0.0/host-contracts/contracts/ProtocolConfig.sol";
import {ProtocolConfigReplica} from "../../src/v1.0.0/host-contracts/contracts/ProtocolConfigReplica.sol";
import {KMSGeneration} from "../../src/v1.0.0/host-contracts/contracts/KMSGeneration.sol";
import {PaddedInputter} from "../../src/v1.0.0/host-contracts/contracts/PaddedInputter.sol";
import {PauserSet} from "../../src/v1.0.0/host-contracts/contracts/immutable/PauserSet.sol";
import {EPOCH_COUNTER_BASE, KMS_CONTEXT_COUNTER_BASE} from "../../src/v1.0.0/host-contracts/contracts/shared/Constants.sol";
import {KmsNodeParams, KmsThresholds, PcrValues} from "../../src/v1.0.0/host-contracts/contracts/shared/Structs.sol";
import {CleartextACL} from "../../src/v1.0.0/cleartext/CleartextACL.sol";
import {CleartextFHEVMExecutor} from "../../src/v1.0.0/cleartext/CleartextFHEVMExecutor.sol";
import {CleartextKMSVerifier} from "../../src/v1.0.0/cleartext/CleartextKMSVerifier.sol";
import {CleartextInputVerifier} from "../../src/v1.0.0/cleartext/CleartextInputVerifier.sol";
import {CleartextHCULimit} from "../../src/v1.0.0/cleartext/CleartextHCULimit.sol";
import {CleartextPaddedInputter} from "../../src/v1.0.0/cleartext/CleartextPaddedInputter.sol";
import {CleartextArithmetic} from "../../src/v1.0.0/cleartext/CleartextArithmetic.sol";
import {CleartextDB} from "../../src/v1.0.0/cleartext/CleartextDB.sol";
import {ACLOwner} from "../../src/v1.0.0/upgrade/ACLOwner.sol";
import {LocalHostBootstrap} from "./LocalHostBootstrap.sol";
import {
    aclAdd,
    fhevmExecutorAdd,
    kmsVerifierAdd,
    inputVerifierAdd,
    hcuLimitAdd,
    protocolConfigAdd,
    kmsGenerationAdd,
    paddedInputterAdd,
    protocolConfigReplicaAdd,
    pauserSetAdd,
    cleartextArithmeticAdd,
    cleartextDbAdd
} from "../../src/v1.0.0/host-contracts/addresses/FHEVMHostAddresses.sol";

interface ICleartextMarkers {
    function IS_CLEARTEXT() external view returns (bool);
    function CLEARTEXT_PROTOCOL_VERSION() external view returns (uint256);
}

interface IVersioned {
    function getVersion() external view returns (string memory);
}

interface IEIP712Domain {
    function eip712Domain()
        external
        view
        returns (bytes1, string memory, string memory, uint256, address, bytes32, uint256[] memory);
}

uint256 constant PROXY_COUNT = 11;

library DeployV1 {
    // Must be called inside vm.startBroadcast(deployerKey), deployer at nonce 0 (or at the nonce the
    // addresses.sol was generated for). Nonces +0..+13 are pinned; +14 is ACLOwner (free).
    function deploy(address deployer) internal returns (ACLOwner aclOwner) {
        // +0 EmptyUUPSProxyACL, +1 ACL proxy
        EmptyUUPSProxyACL aclImpl = new EmptyUUPSProxyACL();
        ERC1967Proxy aclProxy =
            new ERC1967Proxy(address(aclImpl), abi.encodeCall(EmptyUUPSProxyACL.initialize, (deployer)));
        require(address(aclProxy) == aclAdd, "ACL proxy address mismatch");
        // +2 shared EmptyUUPSProxy implementation
        EmptyUUPSProxy sharedImpl = new EmptyUUPSProxy();
        _emptyProxy(sharedImpl, fhevmExecutorAdd); // +3
        _emptyProxy(sharedImpl, kmsVerifierAdd); // +4
        _emptyProxy(sharedImpl, inputVerifierAdd); // +5
        _emptyProxy(sharedImpl, hcuLimitAdd); // +6
        _emptyProxy(sharedImpl, protocolConfigAdd); // +7
        _emptyProxy(sharedImpl, kmsGenerationAdd); // +8
        _emptyProxy(sharedImpl, paddedInputterAdd); // +9
        _emptyProxy(sharedImpl, cleartextArithmeticAdd); // +10
        _emptyProxy(sharedImpl, cleartextDbAdd); // +11
        PauserSet ps = new PauserSet(); // +12
        require(address(ps) == pauserSetAdd, "PauserSet address mismatch");
        _emptyProxy(sharedImpl, protocolConfigReplicaAdd); // +13

        // +14 ACLOwner (owner = deployer), then pausers, then ACL ownership -> ACLOwner (two-step)
        aclOwner = new ACLOwner(deployer, aclAdd);
        PauserSet(pauserSetAdd).addPauser(address(aclOwner));
        ACL(aclAdd).transferOwnership(address(aclOwner));
        aclOwner.acceptACLOwnership();

        // implementations (free nonces) + one atomic ACLOwner.upgrade
        ACLOwner.Op[] memory ops = new ACLOwner.Op[](PROXY_COUNT);
        ops[0] = ACLOwner.Op(aclAdd, address(new CleartextACL()), abi.encodeCall(ACL.initializeFromEmptyProxy, ()));
        ops[1] = ACLOwner.Op(
            fhevmExecutorAdd,
            address(new CleartextFHEVMExecutor()),
            abi.encodeCall(FHEVMExecutor.initializeFromEmptyProxy, ())
        );
        ops[2] = ACLOwner.Op(kmsVerifierAdd, address(new CleartextKMSVerifier()), _kmsVerifierInit());
        ops[3] = ACLOwner.Op(inputVerifierAdd, address(new CleartextInputVerifier()), _inputVerifierInit());
        ops[4] = ACLOwner.Op(hcuLimitAdd, address(new CleartextHCULimit()), _hcuLimitInit());
        ops[5] = ACLOwner.Op(protocolConfigAdd, address(new ProtocolConfig()), _protocolConfigInit());
        ops[6] = ACLOwner.Op(
            kmsGenerationAdd, address(new KMSGeneration()), abi.encodeCall(KMSGeneration.initializeFromEmptyProxy, ())
        );
        ops[7] = ACLOwner.Op(paddedInputterAdd, address(new CleartextPaddedInputter()), _paddedInputterInit());
        ops[8] = ACLOwner.Op(
            cleartextArithmeticAdd,
            address(new CleartextArithmetic()),
            abi.encodeCall(CleartextArithmetic.initializeFromEmptyProxy, ())
        );
        ops[9] = ACLOwner.Op(
            cleartextDbAdd,
            address(new CleartextDB()),
            abi.encodeCall(CleartextDB.initializeFromEmptyProxy, (cleartextArithmeticAdd))
        );
        ops[10] = ACLOwner.Op(
            protocolConfigReplicaAdd,
            address(new ProtocolConfigReplica()),
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (KMS_CONTEXT_COUNTER_BASE + 1, EPOCH_COUNTER_BASE + 1, _kmsNodes(), _kmsThresholds())
            )
        );
        aclOwner.upgrade(ops);
    }

    function _emptyProxy(EmptyUUPSProxy sharedImpl, address expected) private {
        ERC1967Proxy proxy = new ERC1967Proxy(address(sharedImpl), abi.encodeCall(EmptyUUPSProxy.initialize, ()));
        require(address(proxy) == expected, "proxy address mismatch");
    }

    function _paddedInputterInit() private pure returns (bytes memory) {
        return abi.encodeCall(PaddedInputter.initializeFromEmptyProxy, (LocalHostBootstrap.padPrfInstances()));
    }

    function _kmsVerifierInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            KMSVerifier.initializeFromEmptyProxy,
            (LocalHostBootstrap.DECRYPTION_ADDRESS, LocalHostBootstrap.GATEWAY_CHAIN_ID)
        );
    }

    function _inputVerifierInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            InputVerifier.initializeFromEmptyProxy,
            (
                LocalHostBootstrap.INPUT_VERIFICATION_ADDRESS,
                LocalHostBootstrap.GATEWAY_CHAIN_ID,
                LocalHostBootstrap.coprocessorSigners(),
                LocalHostBootstrap.COPROCESSOR_THRESHOLD
            )
        );
    }

    function _hcuLimitInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            HCULimit.initializeFromEmptyProxy,
            (
                LocalHostBootstrap.HCU_CAP_PER_BLOCK,
                LocalHostBootstrap.MAX_HCU_DEPTH_PER_TX,
                LocalHostBootstrap.MAX_HCU_PER_TX
            )
        );
    }

    function _protocolConfigInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            ProtocolConfig.initializeFromEmptyProxy,
            (_kmsNodes(), _kmsThresholds(), LocalHostBootstrap.KMS_SOFTWARE_VERSION, new PcrValues[](0))
        );
    }

    function _kmsNodes() internal pure returns (KmsNodeParams[] memory nodes) {
        address[] memory signers = LocalHostBootstrap.kmsSigners();
        address[] memory txSenders = LocalHostBootstrap.kmsTxSenders();
        string[] memory ips = LocalHostBootstrap.kmsIpAddresses();
        string[] memory urls = LocalHostBootstrap.kmsStorageUrls();
        string[] memory identities = LocalHostBootstrap.kmsMpcIdentities();
        string[] memory prefixes = LocalHostBootstrap.kmsStoragePrefixes();
        int32[] memory partyIds = LocalHostBootstrap.kmsPartyIds();
        nodes = new KmsNodeParams[](LocalHostBootstrap.KMS_NODE_COUNT);
        for (uint256 i = 0; i < nodes.length; i++) {
            nodes[i] = KmsNodeParams({
                txSenderAddress: txSenders[i],
                signerAddress: signers[i],
                ipAddress: ips[i],
                storageUrl: urls[i],
                partyId: partyIds[i],
                mpcIdentity: identities[i],
                caCert: LocalHostBootstrap.KMS_NODE_CA_CERT,
                storagePrefix: prefixes[i]
            });
        }
    }

    function _kmsThresholds() internal pure returns (KmsThresholds memory) {
        uint256 c = LocalHostBootstrap.KMS_NODE_COUNT;
        return KmsThresholds({publicDecryption: c, userDecryption: c, kmsGen: c, mpc: c});
    }

    function _eq(string memory a, string memory b, string memory what) private pure {
        require(keccak256(bytes(a)) == keccak256(bytes(b)), string.concat("version mismatch: ", what, " = ", a));
    }

    function verify(address expectedAclOwner) internal view {
        // markers
        require(ICleartextMarkers(aclAdd).IS_CLEARTEXT(), "ACL.IS_CLEARTEXT");
        require(ICleartextMarkers(aclAdd).CLEARTEXT_PROTOCOL_VERSION() == 1_000_000, "CLEARTEXT_PROTOCOL_VERSION");
        require(ICleartextMarkers(hcuLimitAdd).IS_CLEARTEXT(), "HCULimit.IS_CLEARTEXT");
        require(ICleartextMarkers(cleartextArithmeticAdd).IS_CLEARTEXT(), "CleartextArithmetic.IS_CLEARTEXT");
        // versions
        _eq(IVersioned(aclAdd).getVersion(), "ACL v0.6.0", "ACL");
        _eq(IVersioned(fhevmExecutorAdd).getVersion(), "FHEVMExecutor v0.7.0", "FHEVMExecutor");
        _eq(IVersioned(kmsVerifierAdd).getVersion(), "KMSVerifier v0.5.0", "KMSVerifier");
        _eq(IVersioned(inputVerifierAdd).getVersion(), "InputVerifier v0.3.0", "InputVerifier");
        _eq(IVersioned(hcuLimitAdd).getVersion(), "HCULimit v0.4.0", "HCULimit");
        _eq(IVersioned(protocolConfigAdd).getVersion(), "ProtocolConfig v0.3.0", "ProtocolConfig");
        _eq(IVersioned(kmsGenerationAdd).getVersion(), "KMSGeneration v0.3.0", "KMSGeneration");
        _eq(IVersioned(paddedInputterAdd).getVersion(), "PaddedInputter v0.1.0", "PaddedInputter");
        _eq(IVersioned(cleartextArithmeticAdd).getVersion(), "CleartextArithmetic v0.6.0", "CleartextArithmetic");
        _eq(IVersioned(cleartextDbAdd).getVersion(), "CleartextDB v0.1.0", "CleartextDB");
        _eq(IVersioned(pauserSetAdd).getVersion(), "PauserSet v0.1.0", "PauserSet");
        _eq(
            IVersioned(protocolConfigReplicaAdd).getVersion(), "ProtocolConfigReplica v0.3.0", "ProtocolConfigReplica"
        );
        // wiring
        require(ACL(aclAdd).getFHEVMExecutorAddress() == fhevmExecutorAdd, "ACL.getFHEVMExecutorAddress");
        require(ACL(aclAdd).getPauserSetAddress() == pauserSetAdd, "ACL.getPauserSetAddress");
        require(ACL(aclAdd).getConfidentialBridgeAddress() == address(0), "ACL.getConfidentialBridgeAddress");
        require(FHEVMExecutor(fhevmExecutorAdd).getACLAddress() == aclAdd, "FHEVMExecutor.getACLAddress");
        require(FHEVMExecutor(fhevmExecutorAdd).getHCULimitAddress() == hcuLimitAdd, "FHEVMExecutor.getHCULimitAddress");
        require(
            FHEVMExecutor(fhevmExecutorAdd).getInputVerifierAddress() == inputVerifierAdd,
            "FHEVMExecutor.getInputVerifierAddress"
        );
        require(HCULimit(hcuLimitAdd).getFHEVMExecutorAddress() == fhevmExecutorAdd, "HCULimit.getFHEVMExecutorAddress");
        // ownership
        require(ACL(aclAdd).owner() == expectedAclOwner, "ACL.owner != ACLOwner");
        require(ACLOwner(expectedAclOwner).ACL_ADDRESS() == aclAdd, "ACLOwner.ACL_ADDRESS");
        require(PauserSet(pauserSetAdd).isPauser(expectedAclOwner), "ACLOwner not a pauser");
        // bootstrap
        address[] memory cop = InputVerifier(inputVerifierAdd).getCoprocessorSigners();
        require(cop.length == 4 && cop[0] == LocalHostBootstrap.coprocessorSigners()[0], "coprocessor signers");
        require(InputVerifier(inputVerifierAdd).getThreshold() == LocalHostBootstrap.COPROCESSOR_THRESHOLD, "cop thr");
        address[] memory kms = ProtocolConfig(protocolConfigAdd).getKmsSigners();
        require(kms.length == 4 && kms[3] == LocalHostBootstrap.kmsSigners()[3], "kms signers");
        require(
            ProtocolConfig(protocolConfigAdd).getCurrentKmsContextId() == LocalHostBootstrap.INITIAL_KMS_CONTEXT_ID,
            "kms context id"
        );
        require(ProtocolConfig(protocolConfigAdd).getPublicDecryptionThreshold() == 4, "public decryption threshold");
        require(
            ProtocolConfigReplica(protocolConfigReplicaAdd).getCurrentKmsContextId()
                == LocalHostBootstrap.INITIAL_KMS_CONTEXT_ID,
            "replica context id"
        );
        require(
            PaddedInputter(paddedInputterAdd).isSupportedPrfInstance(LocalHostBootstrap.PAD_PRF_INSTANCE), "prf"
        );
        require(HCULimit(hcuLimitAdd).getMaxHCUPerTx() == LocalHostBootstrap.MAX_HCU_PER_TX, "maxHCUPerTx");
        (, string memory n1,, uint256 c1, address v1,,) = IEIP712Domain(kmsVerifierAdd).eip712Domain();
        require(keccak256(bytes(n1)) == keccak256("Decryption"), "KMSVerifier domain name");
        require(c1 == LocalHostBootstrap.GATEWAY_CHAIN_ID && v1 == LocalHostBootstrap.DECRYPTION_ADDRESS, "kmsv dom");
        (, string memory n2,, uint256 c2, address v2,,) = IEIP712Domain(inputVerifierAdd).eip712Domain();
        require(keccak256(bytes(n2)) == keccak256("InputVerification"), "InputVerifier domain name");
        require(
            c2 == LocalHostBootstrap.GATEWAY_CHAIN_ID && v2 == LocalHostBootstrap.INPUT_VERIFICATION_ADDRESS, "iv dom"
        );
    }
}

/// Step 1: write src/v1.0.0/config/addresses.sol for DEPLOYER_PRIVATE_KEY at its CURRENT nonce (read from
/// --rpc-url). Path from FHEVM_CONFIG_ADDRESSES_FILE (needs a write fs_permission). Same output shape as
/// fhevm-mocks pkg/forge/script/ComputeAddresses.s.sol.
contract WriteAddresses is Script {
    function run() external {
        address d = vm.addr(vm.envUint("DEPLOYER_PRIVATE_KEY"));
        uint64 n = vm.getNonce(d);
        string memory c = string.concat(
            "// SPDX-License-Identifier: BSD-3-Clause-Clear\n\npragma solidity ^0.8.24;\n\n",
            "// Auto-generated - do not edit by hand. Valid ONLY for deployer ",
            vm.toString(d),
            " at start nonce ",
            vm.toString(uint256(n)),
            ".\n"
        );
        c = string.concat(c, _line("ACL_ADDRESS", d, n + 1), _line("FHEVM_EXECUTOR_ADDRESS", d, n + 3));
        c = string.concat(c, _line("KMS_VERIFIER_ADDRESS", d, n + 4), _line("INPUT_VERIFIER_ADDRESS", d, n + 5));
        c = string.concat(c, _line("HCU_LIMIT_ADDRESS", d, n + 6), _line("PROTOCOL_CONFIG_ADDRESS", d, n + 7));
        c = string.concat(c, _line("KMS_GENERATION_ADDRESS", d, n + 8), _line("PADDED_INPUTTER_ADDRESS", d, n + 9));
        c = string.concat(c, _line("CLEARTEXT_ARITHMETIC_ADDRESS", d, n + 10), _line("CLEARTEXT_DB_ADDRESS", d, n + 11));
        c = string.concat(
            c, _line("PAUSER_SET_ADDRESS", d, n + 12), _line("PROTOCOL_CONFIG_REPLICA_ADDRESS", d, n + 13)
        );
        vm.writeFile(vm.envString("FHEVM_CONFIG_ADDRESSES_FILE"), c);
    }

    function _line(string memory name, address d, uint64 nonce) private pure returns (string memory) {
        return string.concat("\naddress constant ", name, " = address(", vm.toString(vm.computeCreateAddress(d, nonce)), ");\n");
    }
}

/// Step 3: broadcast deploy. DEPLOYER_PRIVATE_KEY must be at the nonce config/addresses.sol was written for
/// (every CREATE below is checked against it and reverts on mismatch).
contract Deploy is Script {
    function run() external {
        uint256 key = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address deployer = vm.addr(key);
        vm.startBroadcast(key);
        ACLOwner aclOwner = DeployV1.deploy(deployer);
        vm.stopBroadcast();
        console.log("ACLOwner:", address(aclOwner));
    }
}

/// Read-only checks against a deployed stack. ACL_OWNER_ADDRESS = CREATE(deployer, 14).
contract Verify is Script {
    function run() external view {
        DeployV1.verify(vm.envAddress("ACL_OWNER_ADDRESS"));
        console.log("v1.0.0 cleartext stack verified");
    }
}

/// In-memory self-test (no RPC): deploy then verify in one simulated run.
contract DeployAndVerify is Script {
    function run() external {
        uint256 key = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address deployer = vm.addr(key);
        vm.startBroadcast(key);
        ACLOwner aclOwner = DeployV1.deploy(deployer);
        vm.stopBroadcast();
        DeployV1.verify(address(aclOwner));
        console.log("ACLOwner:", address(aclOwner));
        console.log("deployer nonce after deploy:", vm.getNonce(deployer));
    }
}
```

## Appendix B: `LocalHostBootstrap.sol` (verbatim copy of `fhevm-mocks:pkg/forge/src/_internal/LocalHostBootstrap.sol`)

Intended path: `contracts/scripts/v1.0.0/LocalHostBootstrap.sol`. For the js-sdk suite, change `GATEWAY_CHAIN_ID` to `654321`. Optionally set `COPROCESSOR_THRESHOLD = 1` and the KMS thresholds to 1 (see §5 and §7).

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// AUTOGENERATED by `npm run generate:local-host-bytecode`.
// DO NOT EDIT — your changes will be overwritten. See internal/generateLocalHostBytecode.ts.
//
// The bootstrap arguments ts/deploy.ts applies when no config is supplied — the Solidity mirror of
// DEFAULT_BOOTSTRAP_CONFIG. The signer pools are derived from FHEVM_MNEMONIC at the same HD paths the
// js-sdk cleartext relayer derives its keys from, so a stack deployed with these is one the SDK can sign
// for. Registering any other signer leaves the relayer with no key for the address the chain reports.

library LocalHostBootstrap {
    /// @dev The gateway chain id every EIP-712 proof is bound to. Not a real chain: in cleartext mode it
    ///      only has to be a stable agreed-upon value, so it is derived from a namespaced string the same
    ///      way the two addresses below are, and truncated to uint48 so it stays a plausible chain id.
    ///        GATEWAY_CHAIN_ID = uint48(uint256(keccak256("fhevm.cheat.chainId cleartext gateway")))
    uint64 internal constant GATEWAY_CHAIN_ID = 100733346448153;

    /// @dev The EIP-712 verifyingContract each proof is bound to. Not deployed anywhere: on a real
    ///      network these are gateway contracts, and in cleartext mode they only need to be a stable
    ///      agreed-upon value, so each is derived from a namespaced string:
    ///        INPUT_VERIFICATION_ADDRESS =
    ///          address(uint160(uint256(keccak256("fhevm.cheat.address cleartext input verification"))))
    ///        DECRYPTION_ADDRESS =
    ///          address(uint160(uint256(keccak256("fhevm.cheat.address cleartext decryption"))))
    ///      Both re-derived and checked against ts/constants.ts.
    address internal constant INPUT_VERIFICATION_ADDRESS = 0x6189F6c0c3E40B4a3c72ec86262295D78d845297;
    address internal constant DECRYPTION_ADDRESS = 0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721;

    uint256 internal constant COPROCESSOR_THRESHOLD = 4;
    uint256 internal constant KMS_NODE_COUNT = 4;

    uint48 internal constant HCU_CAP_PER_BLOCK = 281474976710655;
    uint48 internal constant MAX_HCU_DEPTH_PER_TX = 5000000;
    uint48 internal constant MAX_HCU_PER_TX = 20000000;

    /// @dev The KMS context and epoch the canonical ProtocolConfig activates at deploy (`initializeFromEmptyProxy`):
    ///      the first of each, `KMS_CONTEXT_COUNTER_BASE + 1` and `EPOCH_COUNTER_BASE + 1` (Constants.sol: the
    ///      0x07 and 0x08 type tags, shifted by 248). ProtocolConfigReplica is seeded with the same ids, the same
    ///      KMS nodes and the same thresholds (`initializeFromCanonical`), so on one chain both answer alike.
    ///      Hex literals rather than `(uint256(7) << 248) + 1`, so scripts/anvil-lib.sh can read them as-is.
    uint256 internal constant INITIAL_KMS_CONTEXT_ID =
        0x0700000000000000000000000000000000000000000000000000000000000001;
    uint256 internal constant INITIAL_KMS_EPOCH_ID = 0x0800000000000000000000000000000000000000000000000000000000000001;

    /// @dev The RFC-030 PRF instance every deploy path registers on PaddedInputter (cleartext-config.json
    ///      `CLEARTEXT_PAD_PRF_INSTANCE`). Never 0: an unset `prf` must be refused, not taken as the default.
    uint16 internal constant PAD_PRF_INSTANCE = 1;

    /// @dev `PaddedInputter.initializeFromEmptyProxy`'s list: the default instance alone.
    function padPrfInstances() internal pure returns (uint16[] memory instances) {
        instances = new uint16[](1);
        instances[0] = PAD_PRF_INSTANCE;
    }

    /// @dev m/44'/60'/0'/2/i
    function coprocessorSigners() internal pure returns (address[] memory out) {
        out = new address[](4);
        out[0] = 0x6727b56B5D1E990C75fE5dCd572D084fc5DC35cb;
        out[1] = 0xF1CDC3ebe04A063EAbfaf5c3129494F6049Ee229;
        out[2] = 0x46A03377bcBE895407ee4842b63f3249bc8327BD;
        out[3] = 0xFd0ea258B3D2b153DD76CAC0E2270574Da8f18E8;
    }

    /// @dev m/44'/60'/0'/3/i
    function kmsSigners() internal pure returns (address[] memory out) {
        out = new address[](4);
        out[0] = 0x649c125E183640141C2AC329673A22dF9FA81b3C;
        out[1] = 0xDB05A2EdF583E3b1644A59434a163aD178595EAf;
        out[2] = 0x03D684498fB1D52286D09Dd42e720c89b55C63c3;
        out[3] = 0x8252dD4D1350BCd34f419aBD05f8cfA146d71C5D;
    }

    /// @dev m/44'/60'/0'/4/i
    function kmsTxSenders() internal pure returns (address[] memory out) {
        out = new address[](4);
        out[0] = 0x4c41862B9eb691CA197E97a53219E5d860d9dDbB;
        out[1] = 0x822D412944cE9CD7C92f96F162307cF4c9f88F15;
        out[2] = 0xA47E4a9AeaB93C7c2aC8fd5c4E4eC5cB70DD5F8f;
        out[3] = 0xed3C9f5a6d4D8c00A375B8c79dbDF0f69BAAe547;
    }

    function kmsIpAddresses() internal pure returns (string[] memory out) {
        out = new string[](4);
        out[0] = "127.0.0.1";
        out[1] = "127.0.0.2";
        out[2] = "127.0.0.3";
        out[3] = "127.0.0.4";
    }

    function kmsStorageUrls() internal pure returns (string[] memory out) {
        out = new string[](4);
        out[0] = "s3://kms-bucket-1";
        out[1] = "s3://kms-bucket-2";
        out[2] = "s3://kms-bucket-3";
        out[3] = "s3://kms-bucket-4";
    }

    function kmsMpcIdentities() internal pure returns (string[] memory out) {
        out = new string[](4);
        out[0] = "kms-core-1-cleartext-core-1";
        out[1] = "kms-core-2-cleartext-core-2";
        out[2] = "kms-core-3-cleartext-core-3";
        out[3] = "kms-core-4-cleartext-core-4";
    }

    function kmsStoragePrefixes() internal pure returns (string[] memory out) {
        out = new string[](4);
        out[0] = "cleartext-1";
        out[1] = "cleartext-2";
        out[2] = "cleartext-3";
        out[3] = "cleartext-4";
    }

    string internal constant KMS_SOFTWARE_VERSION = "0.0.0-cleartext";

    /// @dev Hex in the config, bytes here; every node shares it (a cleartext stack has none).
    bytes internal constant KMS_NODE_CA_CERT = hex"";

    /// @dev `KmsNodeParams.partyId` is one-based: node i is party i + 1.
    function kmsPartyIds() internal pure returns (int32[] memory out) {
        out = new int32[](KMS_NODE_COUNT);
        for (uint256 i = 0; i < out.length; i++) {
            out[i] = int32(uint32(i + 1));
        }
    }
}
```
