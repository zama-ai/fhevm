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

/** Account index of the KMS signer within MNEMONIC. */
export const KMS_SIGNER_ADDRESS_INDEX = 8;

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
 * Nonce offset, relative to DEPLOYER_START_NONCE, at which each named address is created.
 *
 * A deliberate duplicate of the ordering in `pkg/ts/addresses.ts`. `internal/` cannot import `pkg/ts`:
 * `internal/tsconfig.json` sets `rootDir: "."`, so the import fails with TS6059 and TS6307, and
 * `test/tsconfig.json` sweeps `../internal/**` so it fails there too. The alternatives were a child
 * process — readable only as a separate file, which then has to be excluded from every tsconfig and from
 * eslint — or copying ten numbers. This is the smaller cost.
 *
 * The same layout is duplicated a third time in `pkg/forge/script/ComputeAddresses.s.sol`, which cannot
 * import TypeScript at all. So: change the deploy order and it must change in three places. What catches
 * a missed one is the ZAMA_LOCAL_CONFIG assertion in generateLocalHostBytecode.ts — a wrong offset moves
 * the addresses and the assertion fires at generation time.
 *
 * Offsets 0 and 2 carry no named address: they are the two empty-proxy implementations each proxy is
 * constructed over (see UNNAMED_NONCE_CONTRACTS in the generator).
 */
export const NONCE_OFFSET: Readonly<Record<AddressName, bigint>> = {
  ACL_ADDRESS: 1n,
  FHEVM_EXECUTOR_ADDRESS: 3n,
  KMS_VERIFIER_ADDRESS: 4n,
  INPUT_VERIFIER_ADDRESS: 5n,
  HCU_LIMIT_ADDRESS: 6n,
  PROTOCOL_CONFIG_ADDRESS: 7n,
  KMS_GENERATION_ADDRESS: 8n,
  CLEARTEXT_ARITHMETIC_ADDRESS: 9n,
  CLEARTEXT_DB_ADDRESS: 10n,
  PAUSER_SET_ADDRESS: 11n,
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

/**
 * The bootstrap defaults `ts/deploy.ts` applies when no config is supplied — kept here so the Solidity
 * mirror in pkg/forge/ is generated from one place rather than transcribed.
 *
 * These are echoes of `ts/constants.ts`, which is the source of truth for the TypeScript path. The
 * duplication is unavoidable (internal/ cannot import the payload — see localHostAddresses) and is
 * checked by `test/templates.test.ts`.
 */
export const GATEWAY_CHAIN_ID = 654321n;
export const INPUT_VERIFICATION_ADDRESS = '0x6189F6c0c3E40B4a3c72ec86262295D78d845297';
export const DECRYPTION_ADDRESS = '0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721';
export const COPROCESSOR_COUNT = 4;
export const COPROCESSOR_THRESHOLD = 4;
export const KMS_NODE_COUNT = 4;
export const HCU_CAP_PER_BLOCK = 281474976710655n;
export const MAX_HCU_DEPTH_PER_TX = 5000000n;
export const MAX_HCU_PER_TX = 20000000n;
