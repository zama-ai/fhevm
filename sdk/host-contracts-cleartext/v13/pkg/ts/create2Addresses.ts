import { template as aclOwnerTemplate } from './artifacts/ACLOwner.js';
import { template as emptyProxyTemplate } from './artifacts/EmptyUUPSProxy.js';
import { template as emptyProxyAclTemplate } from './artifacts/EmptyUUPSProxyACL.js';
import { template as erc1967ProxyTemplate } from './artifacts/ERC1967Proxy.js';
import { template as pauserSetTemplate } from './artifacts/PauserSet.js';
// The initializer each proxy is constructed with belongs to the EMPTY implementation it sits over, not to
// the real contract that will eventually be behind it. `ACL`'s own initializer is
// `initializeFromEmptyProxy`, called later during materialization — using it here would encode the wrong
// selector into the proxy's init code, and therefore into its address.
import { abi as emptyProxyAclAbi } from './artifacts/EmptyUUPSProxyACL.js';
import { abi as emptyProxyAbi } from './artifacts/EmptyUUPSProxy.js';
import type { Create2Addresses, Create2Parameters } from './types/public.js';
import { patchTemplateBytecode } from './utils.js';

/**
 * The canonical deterministic-deployment proxy, at the same address on every chain.
 *
 * Not configurable, deliberately: it is the one input that must never vary per operator or per chain, and
 * every address this module returns is a function of it. Note that inheriting the ADDRESS is not the same
 * as trusting the CONTRACT — a different contract could squat it on some testnet, which is why the deploy
 * coordinator gates on the factory's runtime code hash before sending anything. That check needs a chain;
 * this module is pure, so it cannot make it.
 */
export const CREATE2_FACTORY = '0x4e59b44847b379578588920cA78FbF26c0B4956C';

/** The salt namespace. Mixed with the version, so two generations never collide on the same role name. */
const SALT_PREFIX = 'fhevm.cleartext';

/**
 * Role names, byte-for-byte as the deploy scripts spell them.
 *
 * These strings are INSIDE the salt, therefore inside every address. A typo here does not fail — it
 * produces a different, perfectly valid address that nothing else in the system agrees with. They are
 * duplicated from `create2-deploy/script/FhevmCreate2Base.s.sol`'s `R_*` constants because Solidity and
 * TypeScript cannot share a constant; `test/create2-roles.test.ts` compares the two lists.
 */
export const CREATE2_ROLES = {
  implEmptyProxyAcl: 'IMPL_EMPTY_UUPS_PROXY_ACL',
  implEmptyProxy: 'IMPL_EMPTY_UUPS_PROXY',
  acl: 'ACL_ADDRESS',
  fhevmExecutor: 'FHEVM_EXECUTOR_ADDRESS',
  kmsVerifier: 'KMS_VERIFIER_ADDRESS',
  inputVerifier: 'INPUT_VERIFIER_ADDRESS',
  hcuLimit: 'HCU_LIMIT_ADDRESS',
  protocolConfig: 'PROTOCOL_CONFIG_ADDRESS',
  kmsGeneration: 'KMS_GENERATION_ADDRESS',
  cleartextArithmetic: 'CLEARTEXT_ARITHMETIC_ADDRESS',
  cleartextDb: 'CLEARTEXT_DB_ADDRESS',
  pauserSet: 'PAUSER_SET_ADDRESS',
  aclOwner: 'ACL_OWNER',
} as const;

/** `0x`-stripped concatenation. Both halves are already hex; only one `0x` may survive. */
function concatHex(...parts: readonly string[]): `0x${string}` {
  return `0x${parts.map((p) => (p.startsWith('0x') ? p.slice(2) : p)).join('')}`;
}

/**
 * Predicts every address a CREATE2 deploy of this generation will land on.
 *
 * The deterministic-deployment counterpart to `precomputeAddresses`, and a strictly stronger guarantee.
 * `precomputeAddresses` derives `CREATE(deployer, nonce)`, so it depends on the deployer's live nonce and
 * is invalidated by any transaction that moves it. These addresses depend on nothing but the factory, the
 * salt inputs and the init code: no chain access, no deployer nonce, no ordering. They are the same before
 * the first transaction and after the last.
 *
 * ## What it does not return
 *
 * The nine implementation addresses. Their init code bakes in the COMPLETE address set — that is what
 * makes the stack's wiring immutable — so predicting them needs the output of this function fed back into
 * a rebuild, which is the deploy coordinator's three-pass pipeline and not something a pure function can
 * do. Everything a consumer needs to talk to a stack is here; the implementations only matter to whoever
 * is deploying it.
 *
 * ## Why two passes are enough
 *
 * Only two templates carry a baked-in address at all — `EmptyUUPSProxy` and `PauserSet`, both just the
 * ACL — and the ACL proxy's own init code references no address, only the empty implementation it sits
 * over. So the chain resolves in order with no fixpoint:
 *
 *   1. the empty ACL implementation, whose init code is constant
 *   2. the ACL proxy over it, carrying `initialize(deployer)`
 *   3. the shared empty implementation, which bakes the ACL address from step 2
 *   4. every other proxy over step 3 — all sharing ONE init code, distinguished purely by salt
 *   5. `PauserSet` (bakes the ACL) and `ACLOwner` (takes it as a constructor argument)
 *
 * ## The deployer is part of the address
 *
 * `deployer` is baked into the ACL proxy's `initialize(address)` call and into `ACLOwner`'s constructor,
 * so it changes those two addresses. It is the DEPLOYER, not the final admin: `PauserSet.addPauser` is
 * `onlyACLOwner`, so the early steps are only sendable by whoever this names, and a multisig admin cannot
 * sign mid-run. Passing the admin here produces a stack that cannot be bootstrapped.
 *
 * @example
 * ```ts
 * const predicted = await precomputeCreate2Addresses({
 *   ethUtils,
 *   version: '0.13',
 *   deploymentId: 'mainnet-1',
 *   deployer: '0x…',
 * });
 * ```
 */
export async function precomputeCreate2Addresses(parameters: Create2Parameters): Promise<Create2Addresses> {
  const { ethUtils, version, deploymentId, deployer } = parameters;
  const factory = parameters.factory ?? CREATE2_FACTORY;

  /** `keccak256(abi.encode(prefix, version, deploymentId, role))`. */
  const saltFor = (role: string): `0x${string}` =>
    ethUtils.keccak256({
      bytes: ethUtils.encodeAbiParameters({
        types: ['string', 'string', 'string', 'string'],
        values: [SALT_PREFIX, version, deploymentId, role],
      }),
    });

  const predict = (role: string, initCode: string): `0x${string}` =>
    ethUtils.getCreate2Address({
      from: factory,
      salt: saltFor(role),
      initCodeHash: ethUtils.keccak256({ bytes: initCode }),
    });

  /** An ERC-1967 proxy over `implementation`, constructed with `initData`. */
  const proxyInitCode = (implementation: string, initData: string): `0x${string}` =>
    concatHex(
      erc1967ProxyTemplate.bytecode,
      ethUtils.encodeAbiParameters({ types: ['address', 'bytes'], values: [implementation, initData] }),
    );

  // --- 1. the empty ACL implementation: constant init code, no baked address, no constructor args ---
  const implEmptyProxyAcl = predict(CREATE2_ROLES.implEmptyProxyAcl, emptyProxyAclTemplate.bytecode);

  // --- 2. the ACL proxy over it ---
  const aclInitData = await ethUtils.encodeCall({
    abi: emptyProxyAclAbi,
    functionName: 'initialize',
    args: [deployer],
  });
  const aclAddress = predict(CREATE2_ROLES.acl, proxyInitCode(implEmptyProxyAcl, aclInitData));

  // --- 3. the shared empty implementation, which bakes the ACL address ---
  const withAcl = (template: typeof emptyProxyTemplate): `0x${string}` =>
    patchTemplateBytecode({
      template,
      field: 'bytecode',
      replacements: [{ referenceName: CREATE2_ROLES.acl, replacement: aclAddress }],
    });
  const implEmptyProxy = predict(CREATE2_ROLES.implEmptyProxy, withAcl(emptyProxyTemplate));

  // --- 4. every other proxy: ONE init code, distinguished purely by salt ---
  //
  // Worth being explicit about, because it looks like a bug: eight proxies with byte-identical init code
  // land on eight different addresses. That is the whole reason the role name is in the salt.
  const emptyInitData = await ethUtils.encodeCall({ abi: emptyProxyAbi, functionName: 'initialize', args: [] });
  const sharedProxyInitCode = proxyInitCode(implEmptyProxy, emptyInitData);
  const sharedProxy = (role: string): `0x${string}` => predict(role, sharedProxyInitCode);

  // --- 5. the two non-proxies ---
  const pauserSetAddress = predict(CREATE2_ROLES.pauserSet, withAcl(pauserSetTemplate));
  const aclOwnerAddress = predict(
    CREATE2_ROLES.aclOwner,
    concatHex(
      aclOwnerTemplate.bytecode,
      ethUtils.encodeAbiParameters({ types: ['address', 'address'], values: [deployer, aclAddress] }),
    ),
  );

  return {
    fhevmAddresses: {
      aclAddress,
      fhevmExecutorAddress: sharedProxy(CREATE2_ROLES.fhevmExecutor),
      kmsVerifierAddress: sharedProxy(CREATE2_ROLES.kmsVerifier),
      inputVerifierAddress: sharedProxy(CREATE2_ROLES.inputVerifier),
      hcuLimitAddress: sharedProxy(CREATE2_ROLES.hcuLimit),
      protocolConfigAddress: sharedProxy(CREATE2_ROLES.protocolConfig),
      kmsGenerationAddress: sharedProxy(CREATE2_ROLES.kmsGeneration),
    },
    cleartextAddresses: {
      cleartextArithmeticAddress: sharedProxy(CREATE2_ROLES.cleartextArithmetic),
      cleartextDbAddress: sharedProxy(CREATE2_ROLES.cleartextDb),
    },
    pauserSetAddress,
    aclOwnerAddress,
    emptyImplementations: { acl: implEmptyProxyAcl, shared: implEmptyProxy },
    factory,
  };
}
