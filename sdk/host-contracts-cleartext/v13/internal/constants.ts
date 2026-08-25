// Constants shared across internal/ tooling. Anything duplicated by two or more scripts belongs here —
// the values below were each defined in two or three places before, which is how they drift.
//
// This module is deliberately dependency-free: paths and literals only, no imports from generateTemplates
// or its siblings. Everything in internal/ may import it, so a dependency of its own would risk a cycle.
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

/** This package's root directory, holding internal/, test/, scripts/ and pkg/. */
export const PACKAGE_ROOT_ABS_PATH = join(dirname(fileURLToPath(import.meta.url)), '..');

/** The pkg/ directory — the published payload, and the only part of the tree that ships (rule 9). */
export const PKG_DIR_ABS_PATH = join(PACKAGE_ROOT_ABS_PATH, 'pkg');

/**
 * The previous generation, consumed by the upgrade path and its e2e. A sibling directory rather than an
 * npm dependency, so the fixture can be rebuilt from source (RULES.md rule 20).
 */
export const PREVIOUS_GENERATION_DIR_ABS_PATH = resolve(PACKAGE_ROOT_ABS_PATH, '..', 'v12');

/**
 * The npm name **every** generation publishes under. Generations differ only by version — v12 is 0.12.x,
 * v13 is 0.13.x — so the name alone never identifies which one you have.
 */
const PACKAGE_NAME = '@fhevm/host-contracts-cleartext';

/**
 * Directory the previous generation's fixture is extracted into, and the specifier the upgrade e2e
 * imports it by.
 *
 * It cannot be PACKAGE_NAME: the e2e needs both generations resolvable at once, and npm keys
 * `node_modules` by name, so one would overwrite the other. Node resolves a bare specifier by walking
 * directory paths and never consults the manifest's `name`, so an aliased directory resolves fine while
 * the package inside still calls itself PACKAGE_NAME. Verified against a two-generation fixture.
 */
export const PREVIOUS_GENERATION_FIXTURE_ALIAS = `${PACKAGE_NAME}-v12`;

/**
 * The mnemonic the **local stack is deployed from** — the deployer and admin accounts, anvil's funded
 * set, and the addresses baked into pkg/forge/.
 *
 * Not to be confused with `FHEVM_MNEMONIC` in pkg/ts/constants.ts, which derives the KMS and coprocessor
 * *signer* pools. Two different mnemonics with two different jobs; swapping them produces a stack whose
 * addresses look right and whose signatures never verify.
 */
export const MNEMONIC =
  'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';

/**
 * Account index of the deployer within MNEMONIC (HD path `m/44'/60'/0'/0/5`).
 *
 * Load-bearing: every stack address is `CREATE(deployer, nonce)`, so this index together with a start
 * nonce of 0 is what makes the deploy land on the addresses ZamaConfig.sol compiles into consumers
 * (RULES.md rules 15 and 17). Changing it moves the entire stack.
 */
export const DEPLOYER_ADDRESS_INDEX = 5;

/**
 * The Solidity import prefix the contracts read their address set through — the left side of a Foundry
 * remapping, as in `fhevm-config-0.13.0/=internal/placeholders/`. Not a directory: what it maps *to*
 * varies by who is compiling, which is the whole point (RULES.md rule 11).
 *
 * Version-pinned, so two protocol generations can coexist in one project without their address sets
 * colliding — which also means it moves when the protocol minor does (README step 5).
 *
 * Only the TS copy lives here. `remappings.txt` and `pkg/src/addresses/FHEVMHostAddresses.sol` carry the
 * same string and cannot import a constant; they stay the authoritative pair, and a mismatch surfaces as
 * a compile error there or, for this script, as surviving placeholder markers.
 */
export const FHEVM_CONFIG_REMAPPING_PREFIX = 'fhevm-config-0.13.0/';

/**
 * Nonce the deploy sequence starts from — 0, i.e. an account that has sent no transaction.
 *
 * Part of the address derivation, not a detail: `CREATE(deployer, nonce)` means a deployer that has
 * already sent anything produces a different stack. Emitted into pkg/forge/LocalHostAddresses.sol so a
 * consumer can assert the precondition rather than discover it.
 */
export const DEPLOYER_START_NONCE = 0n;

/**
 * The three addresses `library-solidity/config/ZamaConfig.sol` returns from `_getLocalConfig()`, the
 * branch it takes on chain id 31337 (RULES.md rules 15 and 17).
 *
 * They are not ours to choose. `ZamaConfig` is a library dApps inherit, so these literals are compiled
 * into consumer bytecode and cannot be reconfigured afterwards — a default local deploy landing anywhere
 * else leaves every such dApp calling addresses that hold no code. Anything deriving or baking in the
 * local address set checks against these, which is what turns a wrong deployer index or start nonce into
 * a loud failure instead of a plausible-looking artifact.
 *
 * Note `CoprocessorAddress` in ZamaConfig **is** the FHEVMExecutor address; the two names describe one
 * contract.
 *
 * Being a transcription, this constant can drift from the file it copies — and every other check compares
 * against *it*, so nothing downstream would notice. `npm run check:zama-config`
 * (internal/checkZamaLocalConfig.ts) closes that loop by parsing `_getLocalConfig()` and comparing, and
 * runs as part of `npm run build`.
 */
export const ZAMA_LOCAL_CONFIG = {
  aclAddress: '0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D',
  fhevmExecutorAddress: '0xe3a9105a3a932253A70F126eb1E3b589C643dD24',
  kmsVerifierAddress: '0x901F8942346f7AB3a01F6D7613119Bca447Bb030',
} as const;

/**
 * Every address the contracts read from the `fhevm-config-<version>/addresses.sol` the build compiles
 * against — the placeholder markers, the patch-site baseline, and the generated forge constants are all
 * keyed by these names, so this list is the schema all three agree on.
 *
 * Order is not significant, but adding a name here is only half the job: the config file, the contracts
 * that import it and `test/templates.test.ts`'s fixtures all have to gain it too (README step 4).
 */
export const ADDRESS_NAMES = [
  'ACL_ADDRESS',
  'FHEVM_EXECUTOR_ADDRESS',
  'KMS_VERIFIER_ADDRESS',
  'INPUT_VERIFIER_ADDRESS',
  'HCU_LIMIT_ADDRESS',
  'PROTOCOL_CONFIG_ADDRESS',
  'KMS_GENERATION_ADDRESS',
  'PAUSER_SET_ADDRESS',
  'CLEARTEXT_ARITHMETIC_ADDRESS',
  'CLEARTEXT_DB_ADDRESS',
] as const;

export type AddressName = (typeof ADDRESS_NAMES)[number];

/**
 * Nonce offsets of the **host protocol block** — the contracts whose position within the deploy is fixed
 * by the protocol's own deploy order, and which therefore only move when that order changes.
 *
 * Offsets 0 and 2 carry no named address: they are the two empty-proxy implementations each proxy is
 * constructed over (see UNNAMED_NONCE_CONTRACTS in the generator). That is why the numbering has gaps and
 * cannot simply be an index.
 *
 * A deliberate duplicate of the ordering in `pkg/ts/addresses.ts`. `internal/` cannot import `pkg/ts`:
 * `internal/tsconfig.json` sets `rootDir: "."`, so the import fails with TS6059 and TS6307, and
 * `test/tsconfig.json` sweeps `../internal/**` so it fails there too. The alternatives were a child
 * process — readable only as a separate file, which then has to be excluded from every tsconfig and from
 * eslint — or copying the numbers. This is the smaller cost.
 */
const HOST_NONCE_OFFSET = {
  ACL_ADDRESS: 1n,
  FHEVM_EXECUTOR_ADDRESS: 3n,
  KMS_VERIFIER_ADDRESS: 4n,
  INPUT_VERIFIER_ADDRESS: 5n,
  HCU_LIMIT_ADDRESS: 6n,
  PROTOCOL_CONFIG_ADDRESS: 7n,
  KMS_GENERATION_ADDRESS: 8n,
} as const;

/**
 * How many nonces the host block consumes — one past its highest offset, so it counts the unnamed
 * empty-proxy implementations at 0 and 2 as well.
 *
 * Derived, not written down: it is what the trailing block below is positioned against, so a literal here
 * would be a second thing to remember when the host block grows.
 */
const HOST_NONCE_COUNT: bigint =
  Object.values(HOST_NONCE_OFFSET).reduce((highest, offset) => (offset > highest ? offset : highest), 0n) + 1n;

/**
 * Nonce offset, relative to DEPLOYER_START_NONCE, at which each named address is created.
 *
 * Two groups, because the two have different reasons for their values:
 *
 *   - the **host block** (above) is pinned by the protocol's deploy order;
 *   - the **trailing block** (here) is simply appended after it, in this order. Its offsets are not
 *     chosen — they are `HOST_NONCE_COUNT + k` — so a new host contract must shift them, and deriving
 *     them is what makes that automatic instead of a three-number hand-edit.
 *
 * This mirrors the structure `pkg/ts/addresses.ts` already has, where each layer takes the previous
 * layer's `nextStartNonce` and the absolute numbers are never written down. A flat table here was a lossy
 * transcription of that: it recorded the results and dropped the dependency between them.
 *
 * Note the split is **positional, not by provenance**: `PauserSet` is a real host contract (vendored,
 * `src/contracts/immutable/PauserSet.sol`), but it is deployed after the cleartext-only pair, so it
 * belongs to the trailing block. Grouping it with the host contracts because of what it *is* would
 * reintroduce exactly the hardcoded offset this split removes.
 *
 * The same layout is duplicated a third time in `pkg/forge/script/ComputeAddresses.s.sol`, which cannot
 * import TypeScript at all and still spells out `nonce + 9/10/11`. So: change the deploy order and it must
 * change in three places. What catches a missed one differs per group — the ZAMA_LOCAL_CONFIG assertion in
 * generateLocalHostBytecode.ts only covers offsets 1, 3 and 4, so it says nothing about the trailing
 * block; `test/ts/precompute-addresses.test.ts` is what pins all ten against the payload's own layout.
 */
export const NONCE_OFFSET: Readonly<Record<AddressName, bigint>> = {
  ...HOST_NONCE_OFFSET,
  CLEARTEXT_ARITHMETIC_ADDRESS: HOST_NONCE_COUNT,
  CLEARTEXT_DB_ADDRESS: HOST_NONCE_COUNT + 1n,
  PAUSER_SET_ADDRESS: HOST_NONCE_COUNT + 2n,
};

/**
 * How many nonces the address-critical part of the deploy consumes: offsets 0-11, i.e. the ten named
 * contracts plus the two empty-proxy implementations. `pkg/ts/addresses.ts` calls this `nextStartNonce`.
 *
 * Derived from NONCE_OFFSET rather than written out, so there is no second number to keep in step.
 * Nothing past this point is pinned — ACLOwner at 12, then the implementations — because no bytecode
 * refers to those addresses.
 */
export const ADDRESSED_NONCE_COUNT: bigint =
  ADDRESS_NAMES.reduce((highest, name) => (NONCE_OFFSET[name] > highest ? NONCE_OFFSET[name] : highest), 0n) + 1n;

/**
 * What is created at each nonce that holds no named address.
 *
 * Lives beside NONCE_OFFSET because it is the same knowledge — the gaps at 0 and 2 are only explicable
 * together with the offsets that skip them. Both generators that render the deploy layout read it.
 */
export const UNNAMED_NONCE_CONTRACTS: Readonly<Record<number, string>> = {
  0: 'EmptyUUPSProxyACL',
  2: 'EmptyUUPSProxy (shared implementation)',
};

/**
 * What is deployed at each *named* nonce, for the human-readable layout tables the generators emit.
 *
 * Written out rather than derived from the address name: `PAUSER_SET_ADDRESS` is not a proxy at all, and
 * "ERC1967Proxy (FHEVMExecutor)" is not recoverable from `FHEVM_EXECUTOR_ADDRESS` by any transformation
 * worth trusting. Keyed by AddressName, so a new address is a compile error here rather than a table that
 * quietly omits it.
 */
export const NONCE_LABEL: Readonly<Record<AddressName, string>> = {
  ACL_ADDRESS: 'ERC1967Proxy (ACL)',
  FHEVM_EXECUTOR_ADDRESS: 'ERC1967Proxy (FHEVMExecutor)',
  KMS_VERIFIER_ADDRESS: 'ERC1967Proxy (KMSVerifier)',
  INPUT_VERIFIER_ADDRESS: 'ERC1967Proxy (InputVerifier)',
  HCU_LIMIT_ADDRESS: 'ERC1967Proxy (HCULimit)',
  PROTOCOL_CONFIG_ADDRESS: 'ERC1967Proxy (ProtocolConfig)',
  KMS_GENERATION_ADDRESS: 'ERC1967Proxy (KMSGeneration)',
  CLEARTEXT_ARITHMETIC_ADDRESS: 'ERC1967Proxy (CleartextArithmetic)',
  CLEARTEXT_DB_ADDRESS: 'ERC1967Proxy (CleartextDB)',
  PAUSER_SET_ADDRESS: 'PauserSet',
};

/**
 * Solidity constant name per contract, used for the `pkg/forge/` bytecode declarations.
 *
 * Written out rather than derived, because camel-to-SNAKE on names
 * like `CleartextFHEVMExecutor`, `ERC1967Proxy` and `CleartextDB` mangles the acronyms — and these names
 * are a consumer-facing API, so they should be chosen, not computed. A contract missing an entry is a
 * generator error rather than a guessed name.
 */
export const CONSTANT_NAMES = {
  ACL: 'ACL',
  ACLOwner: 'ACL_OWNER',
  CleartextArithmetic: 'CLEARTEXT_ARITHMETIC',
  CleartextDB: 'CLEARTEXT_DB',
  CleartextFHEVMExecutor: 'CLEARTEXT_FHEVM_EXECUTOR',
  CleartextInputVerifier: 'CLEARTEXT_INPUT_VERIFIER',
  CleartextKMSVerifier: 'CLEARTEXT_KMS_VERIFIER',
  EmptyUUPSProxy: 'EMPTY_UUPS_PROXY',
  EmptyUUPSProxyACL: 'EMPTY_UUPS_PROXY_ACL',
  ERC1967Proxy: 'ERC1967_PROXY',
  HCULimit: 'HCU_LIMIT',
  KMSGeneration: 'KMS_GENERATION',
  PauserSet: 'PAUSER_SET',
  ProtocolConfig: 'PROTOCOL_CONFIG',
} as const;

/**
 * Every contract this package generates artifacts for. Derived from CONSTANT_NAMES so the two cannot
 * disagree: anything keyed by contract — the target list, the creation/runtime split — is typed by this
 * union, which makes a missing entry a compile error rather than a runtime throw.
 */
export type ContractName = keyof typeof CONSTANT_NAMES;

// The bootstrap defaults the deploy applies when no config is supplied used to be echoed here, as a
// hand-kept transcription of ts/constants.ts. They now live in `internal/cleartext-config.ts` — the single
// source of truth, copied byte-for-byte into the payload as pkg/ts/cleartext-config.ts — so the harness
// and `pkg/ts` read the same literals instead of two copies that could disagree. Import from there.
