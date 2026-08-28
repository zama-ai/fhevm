// Constants shared across internal/ tooling. Anything duplicated by two or more scripts belongs here —
// the values below were each defined in two or three places before, which is how they drift.
//
// This module is deliberately dependency-free: paths and literals only, no imports from generateTemplates
// or its siblings. Everything in internal/ may import it, so a dependency of its own would risk a cycle.
import { findWorkspaceRootAbsPath } from '@fhevm/sdk-common-dev';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

/**
 * This package's root directory, holding internal/, test/, scripts/ and pkg/.
 *
 * Stays HERE and can never move into @fhevm/sdk-common-dev: it is derived from `import.meta.url`, so the
 * same expression evaluated inside the shared package would resolve to that package instead of this
 * one — and every path built from it would point at the wrong directory while still looking valid.
 * That is why the helpers below take it as an argument rather than inferring it.
 */
export const PACKAGE_ROOT_ABS_PATH = join(dirname(fileURLToPath(import.meta.url)), '..');

/** The sdk workspace root. Tarballs go to TARBALL_DIR_ABS_PATH in @fhevm/sdk-common-dev, not here. */
export const WORKSPACE_ROOT_ABS_PATH = findWorkspaceRootAbsPath(PACKAGE_ROOT_ABS_PATH);

/** The pkg/ directory — the published payload, and the only part of the tree that ships. */
export const PKG_DIR_ABS_PATH = join(PACKAGE_ROOT_ABS_PATH, 'pkg');

/**
 * The previous generation's DIRECTORY — for work that is not an import: running its scripts as child
 * processes, and cleaning its build output. `test/e2e/create2-upgrade.test.ts` is the only caller.
 *
 * TypeScript that merely wants v12's code must NOT come through here. v12 is a workspace member, so
 * it is imported like any dependency — `@fhevm/host-contracts-cleartext-dev-v12/pkg/ts/index.ts`,
 * resolved through the link npm created, with types and go-to-definition intact. The upgrade e2e used
 * to build v12, pack it and extract it under an alias in `test/ts/node_modules` purely to get an
 * importable copy; a workspace link does that already, and the whole apparatus is gone.
 *
 * A tarball is for testing the PUBLISH CONTRACT — `files` omissions, undeclared deps, stale output —
 * and it is worth its cost only when the packed artifact is what is on trial. Using one to obtain a
 * sibling's source code buys nothing and pays a build, a pack and an install for it.
 */
export const PREVIOUS_GENERATION_DIR_ABS_PATH = resolve(PACKAGE_ROOT_ABS_PATH, '..', 'v12');

// The local stack's deploy identity, defined in @fhevm/sdk-common-dev because every generation shares it.
// Re-exported so all of internal/ keeps one import point for constants.
export { DEPLOYER_ADDRESS_INDEX, MNEMONIC, ZAMA_LOCAL_CONFIG } from '@fhevm/sdk-common-dev';

/**
 * The Solidity import prefix the contracts read their address set through — the left side of a Foundry
 * remapping, as in `fhevm-config-0.13.0/=internal/placeholders/`. Not a directory: what it maps *to*
 * varies by who is compiling, which is the whole point.
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
