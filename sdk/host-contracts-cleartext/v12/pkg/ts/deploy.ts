import { deployACLProxy, deployEmptyUUPSProxy, deployEmptyUUPSProxyACL, deployERC1967Proxy } from './proxies.js';
import { abi as aclAbi, template as aclTemplate } from './artifacts/ACL.js';
import { abi as fhevmExecutorAbi, template as fhevmExecutorTemplate } from './artifacts/CleartextFHEVMExecutor.js';
import { abi as kmsVerifierAbi, template as kmsVerifierTemplate } from './artifacts/CleartextKMSVerifier.js';
import { abi as hcuLimitAbi, template as hcuLimitTemplate } from './artifacts/HCULimit.js';
import { abi as inputVerifierAbi, template as inputVerifierTemplate } from './artifacts/CleartextInputVerifier.js';
import { abi as aclOwnerAbi } from './artifacts/ACLOwner.js';
import {
  abi as cleartextArithmeticAbi,
  template as cleartextArithmeticTemplate,
} from './artifacts/CleartextArithmetic.js';
import { abi as cleartextDbAbi, template as cleartextDbTemplate } from './artifacts/CleartextDB.js';
import type {
  AbstractEthereumProvider,
  AbstractEthereumSigner,
  AbstractEthereumUtils,
  BootstrapConfig,
  CleartextAddresses,
  Deployed,
  DeployReturnType,
  InputVerifierInitConfig,
  FhevmAddresses,
  HCULimitInitConfig,
  KMSVerifierInitConfig,
} from './types/public.js';
import {
  assertDeployedAddress,
  assertNoCodeAt,
  assertNoCodeAtTargets,
  buildHostAddressReplacements,
  deployImplementations,
  sendStep,
} from './utils.js';
import { setupACLOwner, toACLOwnerOps } from './aclOwner.js';
import type { ContractUpgradeSpec, DeployedImplementation, UpgradeTarget } from './types/private.js';
import { deployPauserSet } from './pauserSet.js';
import { precomputeAddresses } from './addresses.js';
import { DEFAULT_BOOTSTRAP_CONFIG } from './constants.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Deploy a fresh v13 host-contract stack from scratch.
 *
 * End to end: deploy the 7 empty proxies + PauserSet, install a standing `ACLOwner` (owned by
 * `admin`), then atomically materialize all 7 proxies in a single `ACLOwner.upgrade(...)` transaction.
 * The `deployer` funds/sends the permissionless deployments; `admin` owns `ACLOwner` and signs the
 * one owner-gated upgrade transaction.
 */
export async function deploy(parameters: {
  readonly ethProvider: AbstractEthereumProvider;
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly admin: AbstractEthereumSigner;
  // Deterministic deploy addresses. If omitted, they are derived from `deployer`'s live nonce (read
  // via a view call) — assuming `deployer` sends exactly this function's transactions, in order, next.
  // That assumption is a hard requirement on the signer, not a preference: see the nonce section of
  // `AbstractEthereumSigner` in types/public.ts for what an adapter must do to satisfy it.
  readonly precomputed?:
    | {
        readonly fhevmAddresses: FhevmAddresses;
        readonly cleartextAddresses: CleartextAddresses;
        readonly pauserSetAddress: string;
      }
    | undefined;
  readonly config?: BootstrapConfig | undefined;
}): Promise<Deployed> {
  const precomputed = parameters.precomputed ?? (await precomputeFromDeployerNonce(parameters));
  const { fhevmAddresses, cleartextAddresses } = precomputed;
  const config = parameters.config ?? DEFAULT_BOOTSTRAP_CONFIG;

  // 1. Deploy the 7 core empty proxies, then the 2 cleartext-infra proxies (on the shared impl).
  const { emptyUUPSProxyAddress } = await deployEmptyProxies({
    ethProvider: parameters.ethProvider,
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    precomputedFhevmAddresses: fhevmAddresses,
  });
  await deployCleartextEmptyProxies({
    ethProvider: parameters.ethProvider,
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    precomputedCleartextAddresses: cleartextAddresses,
    emptyUUPSProxyAddress: emptyUUPSProxyAddress.contractAddress,
  });

  // 2. Deploy PauserSet.
  await deployPauserSetContract({
    ethProvider: parameters.ethProvider,
    ethUtils: parameters.ethUtils,
    pauserSetDeployer: parameters.deployer,
    aclAddress: fhevmAddresses.aclAddress,
    precomputedPauserSetAddress: precomputed.pauserSetAddress,
  });

  // 3. Install the standing ACLOwner (owned by `admin`) and hand it ACL ownership.
  const { aclOwnerAddress } = await setupACLOwner({
    deployer: parameters.deployer,
    currentAclOwner: parameters.deployer,
    admin: parameters.admin,
    aclAddress: fhevmAddresses.aclAddress,
    pauserSetAddress: precomputed.pauserSetAddress,
  });

  // 4. Deploy the 9 real implementations (permissionless) — bootstrap specs, empty→real.
  const { implementations } = await buildBootstrapPlan({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    precomputedAddresses: fhevmAddresses,
    cleartextAddresses,
    config: bootstrapUpgradeConfig({
      pauserSetAddress: precomputed.pauserSetAddress,
      cleartextAddresses,
      config,
    }),
  });

  // 5. Materialize all 9 atomically via the standing ACLOwner.
  await sendStep({
    label: 'ACLOwner.upgrade',
    send: () =>
      parameters.admin.writeContract({
        address: aclOwnerAddress,
        abi: aclOwnerAbi,
        functionName: 'upgrade',
        args: [toACLOwnerOps(implementations)],
      }),
  });

  return {
    fhevmAddresses,
    cleartextAddresses,
    pauserSetAddress: precomputed.pauserSetAddress,
    aclOwnerAddress,
  };
}

/**
 * Derive the deploy addresses from `deployer`'s live nonce, read via a view call (`getTransactionCount`
 * at the latest block). Assumes `deployer` will send exactly `deploy`'s transactions, in order, with
 * nothing else interleaved — the same contract that holds when a caller passes an explicit `startNonce`.
 */
async function precomputeFromDeployerNonce(parameters: {
  readonly ethProvider: AbstractEthereumProvider;
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
}): Promise<{
  readonly fhevmAddresses: FhevmAddresses;
  readonly cleartextAddresses: CleartextAddresses;
  readonly pauserSetAddress: string;
}> {
  const from = (await parameters.deployer.getAddress()) as `0x${string}`;
  const startNonce = BigInt(await parameters.ethProvider.getTransactionCount({ address: from }));
  return precomputeAddresses({ ethUtils: parameters.ethUtils, from, startNonce });
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Phase 1 for a fresh v13 stack: deploys all 7 real implementations (patched with v13 host addresses)
 * and encodes their `upgradeToAndCall` calldata. Sends no owner-gated transaction.
 */
async function buildBootstrapPlan(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly precomputedAddresses: FhevmAddresses;
  readonly cleartextAddresses: CleartextAddresses;
  readonly config: UpgradeConfig;
}): Promise<{ readonly implementations: readonly DeployedImplementation[] }> {
  const addressReplacements = buildHostAddressReplacements({
    fhevmAddresses: parameters.precomputedAddresses,
    cleartextAddresses: parameters.cleartextAddresses,
    pauserSetAddress: parameters.config.pauserSetAddress,
  });

  const addr = parameters.precomputedAddresses;
  const targets: readonly UpgradeTarget[] = [
    {
      contractName: 'ACL',
      proxyAddress: addr.aclAddress,
      template: aclTemplate,
      abi: aclAbi,
      spec: parameters.config.acl,
    },
    {
      contractName: 'FHEVMExecutor',
      proxyAddress: addr.fhevmExecutorAddress,
      template: fhevmExecutorTemplate,
      abi: fhevmExecutorAbi,
      spec: parameters.config.fhevmExecutor,
    },
    {
      contractName: 'KMSVerifier',
      proxyAddress: addr.kmsVerifierAddress,
      template: kmsVerifierTemplate,
      abi: kmsVerifierAbi,
      spec: parameters.config.kmsVerifier,
    },
    {
      contractName: 'InputVerifier',
      proxyAddress: addr.inputVerifierAddress,
      template: inputVerifierTemplate,
      abi: inputVerifierAbi,
      spec: parameters.config.inputVerifier,
    },
    {
      contractName: 'HCULimit',
      proxyAddress: addr.hcuLimitAddress,
      template: hcuLimitTemplate,
      abi: hcuLimitAbi,
      spec: parameters.config.hcuLimit,
    },
    {
      contractName: 'CleartextArithmetic',
      proxyAddress: parameters.cleartextAddresses.cleartextArithmeticAddress,
      template: cleartextArithmeticTemplate,
      abi: cleartextArithmeticAbi,
      spec: parameters.config.cleartextArithmetic,
    },
    {
      contractName: 'CleartextDB',
      proxyAddress: parameters.cleartextAddresses.cleartextDbAddress,
      template: cleartextDbTemplate,
      abi: cleartextDbAbi,
      spec: parameters.config.cleartextDb,
    },
  ];

  return { implementations: await deployImplementations({ ...parameters, addressReplacements, targets }) };
}

////////////////////////////////////////////////////////////////////////////////

async function deployEmptyProxies(parameters: {
  readonly ethProvider: AbstractEthereumProvider;
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly precomputedFhevmAddresses: FhevmAddresses;
}): Promise<{ emptyUUPSProxyAddress: DeployReturnType }> {
  const addr = parameters.precomputedFhevmAddresses;

  // Every host proxy except ACL, in deploy order. They are identical operations differing only by the
  // address each must land on, so they are a table rather than N copies of the same twelve lines: a
  // generation that adds or drops a host contract edits this list and nothing else.
  //
  // ACL is not in it because it is genuinely different — it gets its own empty implementation
  // (`EmptyUUPSProxyACL`), which every other proxy shares one of.
  //
  // ORDER IS LOAD-BEARING. Each address is `CREATE(deployer, startNonce + k)`, and the offsets in
  // `HOST_NONCE_OFFSET` (addresses.ts) are this list's positions. Reordering here silently moves every
  // subsequent address while the precomputed set keeps pointing at the old ones — which is what the
  // per-step `assertDeployedAddress` below turns into an immediate failure instead of a live stack
  // whose bytecode references dead addresses.
  const sharedImplProxies: ReadonlyArray<{ readonly contractName: string; readonly address: string }> = [
    { contractName: 'FHEVMExecutor', address: addr.fhevmExecutorAddress },
    { contractName: 'KMSVerifier', address: addr.kmsVerifierAddress },
    { contractName: 'InputVerifier', address: addr.inputVerifierAddress },
    { contractName: 'HCULimit', address: addr.hcuLimitAddress },
  ];

  // Assert no target host address is already occupied before deploying ANYTHING. Checked for the whole
  // set up front, not per contract: a half-deployed stack is far worse to recover from than a refusal,
  // and the deployer's nonce has already advanced by the time a later collision would be noticed.
  await assertNoCodeAtTargets({
    ethProvider: parameters.ethProvider,
    targets: [{ contractName: 'ACL', address: addr.aclAddress }, ...sharedImplProxies],
  });

  // nonce +0: EmptyUUPSProxyACL — the ACL proxy's own initial implementation.
  const emptyUUPSProxyACLAddress = await deployEmptyUUPSProxyACL({ deployer: parameters.deployer });
  console.log(`EmptyUUPSProxyACL = ${emptyUUPSProxyACLAddress.contractAddress}`);

  // nonce +1: the ACL proxy.
  const aclProxyAddress = await deployACLProxy({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    emptyUUPSProxyACLAddress: emptyUUPSProxyACLAddress.contractAddress,
  });
  console.log(`ACL = ${aclProxyAddress.contractAddress}`);
  assertDeployedAddress({
    contractName: 'ACL',
    expectedAddress: addr.aclAddress,
    actualAddress: aclProxyAddress.contractAddress,
  });

  // nonce +2: the one EmptyUUPSProxy implementation every remaining proxy is constructed over. It bakes
  // the ACL address, which is why it cannot be deployed before the ACL proxy exists.
  const emptyUUPSProxyAddress = await deployEmptyUUPSProxy({
    deployer: parameters.deployer,
    aclAddress: addr.aclAddress,
  });
  console.log(`EmptyUUPSProxy = ${emptyUUPSProxyAddress.contractAddress}`);

  // nonce +3 onward: one ERC1967Proxy per table entry, in table order.
  for (const target of sharedImplProxies) {
    // Sequential on purpose, not a concurrency oversight: these must occupy consecutive nonces in this
    // exact order, so `await` inside the loop is the requirement rather than a cost. Do not "optimize"
    // this into Promise.all — it would deploy the whole set at unpredictable nonces.
    const proxy = await deployERC1967Proxy({
      ethUtils: parameters.ethUtils,
      deployer: parameters.deployer,
      emptyUUPSProxyAddress: emptyUUPSProxyAddress.contractAddress,
    });
    console.log(`${target.contractName} = ${proxy.contractAddress}`);
    assertDeployedAddress({
      contractName: target.contractName,
      expectedAddress: target.address,
      actualAddress: proxy.contractAddress,
    });
  }

  return { emptyUUPSProxyAddress };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Deploys the two cleartext-infra ERC1967 proxies (`CleartextArithmetic`, `CleartextDB`) on the
 * shared `EmptyUUPSProxy` implementation. Called after `deployEmptyProxies`, before PauserSet, so
 * their CREATE addresses match `precomputeAddresses`.
 */
async function deployCleartextEmptyProxies(parameters: {
  readonly ethProvider: AbstractEthereumProvider;
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly precomputedCleartextAddresses: CleartextAddresses;
  readonly emptyUUPSProxyAddress: string;
}): Promise<void> {
  await assertNoCodeAtTargets({
    ethProvider: parameters.ethProvider,
    targets: [
      {
        contractName: 'CleartextArithmetic',
        address: parameters.precomputedCleartextAddresses.cleartextArithmeticAddress,
      },
      { contractName: 'CleartextDB', address: parameters.precomputedCleartextAddresses.cleartextDbAddress },
    ],
  });

  const cleartextArithmeticProxy = await deployERC1967Proxy({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    emptyUUPSProxyAddress: parameters.emptyUUPSProxyAddress,
  });
  console.log(`CleartextArithmetic = ${cleartextArithmeticProxy.contractAddress}`);
  assertDeployedAddress({
    contractName: 'CleartextArithmetic',
    expectedAddress: parameters.precomputedCleartextAddresses.cleartextArithmeticAddress,
    actualAddress: cleartextArithmeticProxy.contractAddress,
  });

  const cleartextDbProxy = await deployERC1967Proxy({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    emptyUUPSProxyAddress: parameters.emptyUUPSProxyAddress,
  });
  console.log(`CleartextDB = ${cleartextDbProxy.contractAddress}`);
  assertDeployedAddress({
    contractName: 'CleartextDB',
    expectedAddress: parameters.precomputedCleartextAddresses.cleartextDbAddress,
    actualAddress: cleartextDbProxy.contractAddress,
  });
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Isolated PauserSet deployment step. `PauserSet` is deployed by its own dedicated
 * `pauserSetDeployer` (not the main host deployer), so its address is a plain CREATE address of that
 * signer at `startNonce` (default `0n`, i.e. a fresh deployer's first transaction).
 *
 * Asserts the target slot is empty, deploys, and asserts it landed at the expected address.
 */
async function deployPauserSetContract(parameters: {
  readonly ethProvider: AbstractEthereumProvider;
  readonly ethUtils: AbstractEthereumUtils;
  readonly pauserSetDeployer: AbstractEthereumSigner;
  readonly precomputedPauserSetAddress: string;
  readonly aclAddress: string;
}): Promise<DeployReturnType> {
  await assertNoCodeAt({
    ethProvider: parameters.ethProvider,
    contractName: 'PauserSet',
    address: parameters.precomputedPauserSetAddress,
  });

  const pauserSetAddress = await deployPauserSet({
    deployer: parameters.pauserSetDeployer,
    aclAddress: parameters.aclAddress,
  });

  assertDeployedAddress({
    contractName: 'PauserSet',
    expectedAddress: parameters.precomputedPauserSetAddress,
    actualAddress: pauserSetAddress.contractAddress,
  });

  return pauserSetAddress;
}

////////////////////////////////////////////////////////////////////////////////
// Upgrade step: materialize each empty proxy into its real implementation.
//
// For every proxy this performs exactly two on-chain actions:
//   1. deploy the real implementation (its baked-in host addresses patched from `precomputedAddresses`)
//   2. proxy.upgradeToAndCall(newImplementation, initializeFromEmptyProxy(<init values>))
//
// It does NOT transfer ownership. The `signer` must already hold upgrade authority, i.e. be the
// current ACL owner (for the ACL proxy: the owner set at EmptyUUPSProxyACL.initialize; for every
// other proxy: whatever `ACL.owner()` returns, via ACLOwnable). Run the ownership transfer to the
// upgrader either before this (signer = upgrader) or after this (signer = deployer).
////////////////////////////////////////////////////////////////////////////////

/**
 * Per-contract upgrade specification. One entry per host proxy, in dependency-agnostic order.
 * `pauserSetAddress` is baked into every implementation's bytecode (see `buildHostAddressReplacements`).
 */
/** @internal — intermediate config built from `BootstrapConfig`; not part of the public API. */
type UpgradeConfig = {
  readonly pauserSetAddress: string;
  readonly acl: ContractUpgradeSpec;
  readonly fhevmExecutor: ContractUpgradeSpec;
  readonly kmsVerifier: ContractUpgradeSpec;
  readonly inputVerifier: ContractUpgradeSpec;
  readonly hcuLimit: ContractUpgradeSpec;
  readonly cleartextArithmetic: ContractUpgradeSpec;
  readonly cleartextDb: ContractUpgradeSpec;
};

/** Maps the typed bootstrap config to a full `UpgradeConfig` of `initializeFromEmptyProxy` specs. */
function bootstrapUpgradeConfig(parameters: {
  readonly pauserSetAddress: string;
  readonly cleartextAddresses: CleartextAddresses;
  readonly config: BootstrapConfig;
}): UpgradeConfig {
  const { config } = parameters;
  const bootstrap = (initArgs: readonly unknown[]): ContractUpgradeSpec => ({
    initFn: 'initializeFromEmptyProxy',
    initArgs,
  });
  return {
    pauserSetAddress: parameters.pauserSetAddress,
    acl: bootstrap([]),
    fhevmExecutor: bootstrap([]),
    kmsVerifier: bootstrap(kmsVerifierInitArgs(config.kmsVerifier)),
    inputVerifier: bootstrap(inputVerifierInitArgs(config.inputVerifier)),
    hcuLimit: bootstrap(hcuLimitInitArgs(config.hcuLimit)),
    cleartextArithmetic: bootstrap([]),
    // CleartextDB.initializeFromEmptyProxy(initialWriter) — seed CleartextArithmetic as the writer.
    cleartextDb: bootstrap([parameters.cleartextAddresses.cleartextArithmeticAddress]),
  };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Builds the arguments for `KMSVerifier.initializeFromEmptyProxy`
 * `(address verifyingContractSource, uint64 chainIDSource, address[] initialSigners, uint256 initialThreshold)`
 * — the same shape as InputVerifier, because in this generation the verifier carries its own signer set.
 */
function kmsVerifierInitArgs(config: KMSVerifierInitConfig): readonly unknown[] {
  return [config.verifyingContractSource, config.chainIDSource, config.initialSigners, config.initialThreshold];
}

////////////////////////////////////////////////////////////////////////////////

/** Builds the `initializeFromEmptyProxy` arguments for InputVerifier bootstrap, type-safely. */
function inputVerifierInitArgs(config: InputVerifierInitConfig): readonly unknown[] {
  return [config.verifyingContractSource, config.chainIDSource, config.initialSigners, config.initialThreshold];
}

////////////////////////////////////////////////////////////////////////////////

/** Builds the `initializeFromEmptyProxy` arguments for HCULimit bootstrap, type-safely. */
function hcuLimitInitArgs(config: HCULimitInitConfig): readonly unknown[] {
  return [config.hcuCapPerBlock, config.maxHCUDepthPerTx, config.maxHCUPerTx];
}
