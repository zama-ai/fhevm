import { deployACLProxy, deployEmptyUUPSProxy, deployEmptyUUPSProxyACL, deployERC1967Proxy } from './proxies.js';
import { abi as aclAbi, template as aclTemplate } from './artifacts/ACL.js';
import { abi as fhevmExecutorAbi, template as fhevmExecutorTemplate } from './artifacts/CleartextFHEVMExecutor.js';
import { abi as kmsVerifierAbi, template as kmsVerifierTemplate } from './artifacts/CleartextKMSVerifier.js';
import { abi as hcuLimitAbi, template as hcuLimitTemplate } from './artifacts/HCULimit.js';
import { abi as inputVerifierAbi, template as inputVerifierTemplate } from './artifacts/CleartextInputVerifier.js';
import { abi as protocolConfigAbi, template as protocolConfigTemplate } from './artifacts/ProtocolConfig.js';
import { abi as kmsGenerationAbi, template as kmsGenerationTemplate } from './artifacts/KMSGeneration.js';
import { abi as aclOwnerAbi } from './artifacts/ACLOwner.js';
import {
  abi as cleartextArithmeticAbi,
  template as cleartextArithmeticTemplate,
} from './artifacts/CleartextArithmetic.js';
import { abi as cleartextDbAbi, template as cleartextDbTemplate } from './artifacts/CleartextDB.js';
import { template as pauserSetTemplate } from './artifacts/PauserSet.js';
import type {
  AbstractEthereumProvider,
  AbstractEthereumSigner,
  AbstractEthereumUtils,
  BootstrapConfigV13,
  CleartextAddresses,
  DeployedV13,
  DeployReturnType,
  InputVerifierInitConfig,
  FhevmAddressesV12,
  FhevmAddressesV13,
  HCULimitInitConfig,
  KMSVerifierInitConfig,
} from './types/public.js';
import {
  assertDeployedAddress,
  assertNoCodeAt,
  assertNoCodeAtTargets,
  buildHostAddressReplacementsV13,
  deployImplementations,
  patchTemplateBytecode,
} from './utils.js';
import { setupACLOwner, toACLOwnerOps } from './aclOwner.js';
import type { ContractUpgradeSpec, DeployedImplementation, UpgradeTarget } from './types/private.js';

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
  readonly precomputed: {
    readonly fhevmAddresses: FhevmAddressesV13;
    readonly cleartextAddresses: CleartextAddresses;
    readonly pauserSetAddress: string;
  };
  readonly config: BootstrapConfigV13;
}): Promise<DeployedV13> {
  const { fhevmAddresses, cleartextAddresses } = parameters.precomputed;

  // 1. Deploy the 7 core empty proxies, then the 2 cleartext-infra proxies (on the shared impl).
  const { emptyUUPSProxyAddress } = await deployEmptyProxiesV13({
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
    precomputedPauserSetAddress: parameters.precomputed.pauserSetAddress,
  });

  // 3. Install the standing ACLOwner (owned by `admin`) and hand it ACL ownership.
  const { aclOwnerAddress } = await setupACLOwner({
    deployer: parameters.deployer,
    currentAclOwner: parameters.deployer,
    admin: parameters.admin,
    aclAddress: fhevmAddresses.aclAddress,
  });

  // 4. Deploy the 9 real implementations (permissionless) — bootstrap specs, empty→real.
  const { implementations } = await buildBootstrapPlanV13({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    precomputedAddresses: fhevmAddresses,
    cleartextAddresses,
    config: bootstrapUpgradeConfigV13({
      pauserSetAddress: parameters.precomputed.pauserSetAddress,
      cleartextAddresses,
      config: parameters.config,
    }),
  });

  // 5. Materialize all 9 atomically via the standing ACLOwner.
  await parameters.admin.writeContract({
    address: aclOwnerAddress,
    abi: aclOwnerAbi,
    functionName: 'upgrade',
    args: [toACLOwnerOps(implementations)],
  });

  return {
    fhevmAddresses,
    cleartextAddresses,
    pauserSetAddress: parameters.precomputed.pauserSetAddress,
    aclOwnerAddress,
  };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Phase 1 for a fresh v13 stack: deploys all 9 real implementations (patched with v13 host addresses)
 * and encodes their `upgradeToAndCall` calldata. Sends no owner-gated transaction.
 */
async function buildBootstrapPlanV13(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly precomputedAddresses: FhevmAddressesV13;
  readonly cleartextAddresses: CleartextAddresses;
  readonly config: UpgradeConfigV13;
}): Promise<{ readonly implementations: readonly DeployedImplementation[] }> {
  const addressReplacements = buildHostAddressReplacementsV13({
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
      contractName: 'ProtocolConfig',
      proxyAddress: addr.protocolConfigAddress,
      template: protocolConfigTemplate,
      abi: protocolConfigAbi,
      spec: parameters.config.protocolConfig,
    },
    {
      contractName: 'KMSGeneration',
      proxyAddress: addr.kmsGenerationAddress,
      template: kmsGenerationTemplate,
      abi: kmsGenerationAbi,
      spec: parameters.config.kmsGeneration,
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

async function deployEmptyProxiesV13(parameters: {
  readonly ethProvider: AbstractEthereumProvider;
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly precomputedFhevmAddresses: FhevmAddressesV13;
}): Promise<{ emptyUUPSProxyAddress: DeployReturnType }> {
  const { emptyUUPSProxyAddress } = await deployEmptyProxiesV12(parameters);

  const targetsV13: ReadonlyArray<{ readonly contractName: string; readonly address: string }> = [
    { contractName: 'ProtocolConfig', address: parameters.precomputedFhevmAddresses.protocolConfigAddress },
    { contractName: 'KMSGeneration', address: parameters.precomputedFhevmAddresses.kmsGenerationAddress },
  ];

  // Assert none of the target host addresses are already occupied before deploying anything.
  await assertNoCodeAtTargets({
    ethProvider: parameters.ethProvider,
    targets: targetsV13,
  });

  // step 1: deploy ProtocolConfig ERC1967Proxy (startNonce + 0)
  const protocolConfigProxyAddress = await deployERC1967Proxy({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    emptyUUPSProxyAddress: emptyUUPSProxyAddress.contractAddress,
  });
  console.log(`ProtocolConfig = ${protocolConfigProxyAddress.contractAddress}`);
  assertDeployedAddress({
    contractName: 'ProtocolConfig',
    expectedAddress: parameters.precomputedFhevmAddresses.protocolConfigAddress,
    actualAddress: protocolConfigProxyAddress.contractAddress,
  });

  // step 2: deploy KMSGeneration ERC1967Proxy (startNonce + 1)
  const kmsGenerationProxyAddress = await deployERC1967Proxy({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    emptyUUPSProxyAddress: emptyUUPSProxyAddress.contractAddress,
  });
  console.log(`KMSGeneration = ${kmsGenerationProxyAddress.contractAddress}`);
  assertDeployedAddress({
    contractName: 'KMSGeneration',
    expectedAddress: parameters.precomputedFhevmAddresses.kmsGenerationAddress,
    actualAddress: kmsGenerationProxyAddress.contractAddress,
  });

  return { emptyUUPSProxyAddress };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Deploys the two cleartext-infra ERC1967 proxies (`CleartextArithmetic`, `CleartextDB`) on the
 * shared `EmptyUUPSProxy` implementation. Called after `deployEmptyProxiesV13`, before PauserSet, so
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

async function deployEmptyProxiesV12(parameters: {
  readonly ethProvider: AbstractEthereumProvider;
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly precomputedFhevmAddresses: FhevmAddressesV12;
}): Promise<{ emptyUUPSProxyAddress: DeployReturnType }> {
  const targetsV12: ReadonlyArray<{ readonly contractName: string; readonly address: string }> = [
    { contractName: 'ACL', address: parameters.precomputedFhevmAddresses.aclAddress },
    { contractName: 'FHEVMExecutor', address: parameters.precomputedFhevmAddresses.fhevmExecutorAddress },
    { contractName: 'KMSVerifier', address: parameters.precomputedFhevmAddresses.kmsVerifierAddress },
    { contractName: 'InputVerifier', address: parameters.precomputedFhevmAddresses.inputVerifierAddress },
    { contractName: 'HCULimit', address: parameters.precomputedFhevmAddresses.hcuLimitAddress },
  ];

  // Assert none of the target host addresses are already occupied before deploying anything.
  await assertNoCodeAtTargets({
    ethProvider: parameters.ethProvider,
    targets: targetsV12,
  });

  // Step 1: deploy EmptyUUPSProxyACL (startNonce + 0)
  const emptyUUPSProxyACLAddress = await deployEmptyUUPSProxyACL({
    deployer: parameters.deployer,
  });
  console.log(`EmptyUUPSProxyACL = ${emptyUUPSProxyACLAddress.contractAddress}`);

  // step 2: deploy ACL ERC1967Proxy (startNonce + 1)
  const aclProxyAddress = await deployACLProxy({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    emptyUUPSProxyACLAddress: emptyUUPSProxyACLAddress.contractAddress,
  });
  console.log(`ACL = ${aclProxyAddress.contractAddress}`);

  assertDeployedAddress({
    contractName: 'ACL',
    expectedAddress: parameters.precomputedFhevmAddresses.aclAddress,
    actualAddress: aclProxyAddress.contractAddress,
  });

  // step 3: deploy shared EmptyUUPSProxy implementation (startNonce + 2)
  const emptyUUPSProxyAddress = await deployEmptyUUPSProxy({
    deployer: parameters.deployer,
    aclAddress: parameters.precomputedFhevmAddresses.aclAddress,
  });
  console.log(`EmptyUUPSProxy = ${emptyUUPSProxyAddress.contractAddress}`);

  // step 4: deploy FHEVMExecutor ERC1967Proxy (startNonce + 3)
  const fhevmExecutorProxyAddress = await deployERC1967Proxy({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    emptyUUPSProxyAddress: emptyUUPSProxyAddress.contractAddress,
  });
  console.log(`FHEVMExecutor = ${fhevmExecutorProxyAddress.contractAddress}`);
  assertDeployedAddress({
    contractName: 'FHEVMExecutor',
    expectedAddress: parameters.precomputedFhevmAddresses.fhevmExecutorAddress,
    actualAddress: fhevmExecutorProxyAddress.contractAddress,
  });

  // step 5: deploy KMSVerifier ERC1967Proxy (startNonce + 4)
  const kmsVerifierProxyAddress = await deployERC1967Proxy({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    emptyUUPSProxyAddress: emptyUUPSProxyAddress.contractAddress,
  });
  console.log(`KMSVerifier = ${kmsVerifierProxyAddress.contractAddress}`);
  assertDeployedAddress({
    contractName: 'KMSVerifier',
    expectedAddress: parameters.precomputedFhevmAddresses.kmsVerifierAddress,
    actualAddress: kmsVerifierProxyAddress.contractAddress,
  });

  // step 6: deploy InputVerifier ERC1967Proxy (startNonce + 5)
  const inputVerifierProxyAddress = await deployERC1967Proxy({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    emptyUUPSProxyAddress: emptyUUPSProxyAddress.contractAddress,
  });
  console.log(`InputVerifier = ${inputVerifierProxyAddress.contractAddress}`);
  assertDeployedAddress({
    contractName: 'InputVerifier',
    expectedAddress: parameters.precomputedFhevmAddresses.inputVerifierAddress,
    actualAddress: inputVerifierProxyAddress.contractAddress,
  });

  // step 7: deploy HCULimit ERC1967Proxy (startNonce + 6)
  const hcuLimitProxyAddress = await deployERC1967Proxy({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    emptyUUPSProxyAddress: emptyUUPSProxyAddress.contractAddress,
  });
  console.log(`HCULimit = ${hcuLimitProxyAddress.contractAddress}`);
  assertDeployedAddress({
    contractName: 'HCULimit',
    expectedAddress: parameters.precomputedFhevmAddresses.hcuLimitAddress,
    actualAddress: hcuLimitProxyAddress.contractAddress,
  });

  return { emptyUUPSProxyAddress };
}

////////////////////////////////////////////////////////////////////////////////

/*
  PauserSet.sol
*/
/**
 * Deploys the `PauserSet` contract.
 *
 * `PauserSet` is a non-proxy (immutable) contract with no constructor args and no initializer. It
 * bakes in the ACL address (via `FHEVMHostAddresses.sol`) to gate `addPauser`/`removePauser` on the
 * ACL owner, so that address is patched into the bytecode before deployment. Deploy it before the
 * `upgrade(...)` step and feed the returned address in as `UpgradeConfig.pauserSetAddress`, since the
 * other host contracts reference it too.
 */
async function deployPauserSet(parameters: {
  readonly deployer: AbstractEthereumSigner;
  readonly aclAddress: string;
}): Promise<DeployReturnType> {
  const bytecode = patchTemplateBytecode({
    template: pauserSetTemplate,
    field: 'bytecode',
    replacements: [{ referenceName: 'ACL_ADDRESS', replacement: parameters.aclAddress }],
  });
  return await parameters.deployer.deploy({ bytecode });
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
/** @internal — intermediate config built from `BootstrapConfigV13`; not part of the public API. */
type UpgradeConfigV13 = {
  readonly pauserSetAddress: string;
  readonly acl: ContractUpgradeSpec;
  readonly fhevmExecutor: ContractUpgradeSpec;
  readonly kmsVerifier: ContractUpgradeSpec;
  readonly inputVerifier: ContractUpgradeSpec;
  readonly hcuLimit: ContractUpgradeSpec;
  readonly protocolConfig: ContractUpgradeSpec;
  readonly kmsGeneration: ContractUpgradeSpec;
  readonly cleartextArithmetic: ContractUpgradeSpec;
  readonly cleartextDb: ContractUpgradeSpec;
};

/** Maps the typed bootstrap config to a full `UpgradeConfigV13` of `initializeFromEmptyProxy` specs. */
function bootstrapUpgradeConfigV13(parameters: {
  readonly pauserSetAddress: string;
  readonly cleartextAddresses: CleartextAddresses;
  readonly config: BootstrapConfigV13;
}): UpgradeConfigV13 {
  const { config } = parameters;
  const bootstrap = (initArgs: readonly unknown[]): ContractUpgradeSpec => ({
    initFn: 'initializeFromEmptyProxy',
    initArgs,
  });
  return {
    pauserSetAddress: parameters.pauserSetAddress,
    acl: bootstrap([]),
    fhevmExecutor: bootstrap([]),
    kmsVerifier: bootstrap(kmsVerifierInitArgsV13(config.kmsVerifier)),
    inputVerifier: bootstrap(eip712VerifierInitArgs(config.inputVerifier)),
    hcuLimit: bootstrap(hcuLimitInitArgs(config.hcuLimit)),
    protocolConfig: bootstrap([config.protocolConfig.initialKmsNodes, config.protocolConfig.initialThresholds]),
    kmsGeneration: bootstrap([]),
    cleartextArithmetic: bootstrap([]),
    // CleartextDB.initializeFromEmptyProxy(initialWriter) — seed CleartextArithmetic as the writer.
    cleartextDb: bootstrap([parameters.cleartextAddresses.cleartextArithmeticAddress]),
  };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Builds the arguments for v13 `KMSVerifier.initializeFromEmptyProxy`
 * `(address verifyingContractSource, uint64 chainIDSource)`. In v13 the KMS signer set moved to
 * `ProtocolConfig`, so — unlike v12 — no signers/threshold are passed here.
 */
function kmsVerifierInitArgsV13(config: KMSVerifierInitConfig): readonly unknown[] {
  return [config.verifyingContractSource, config.chainIDSource];
}

////////////////////////////////////////////////////////////////////////////////

/** Builds the `initializeFromEmptyProxy` arguments for InputVerifier bootstrap, type-safely. */
function eip712VerifierInitArgs(config: InputVerifierInitConfig): readonly unknown[] {
  return [config.verifyingContractSource, config.chainIDSource, config.initialSigners, config.initialThreshold];
}

////////////////////////////////////////////////////////////////////////////////

/** Builds the `initializeFromEmptyProxy` arguments for HCULimit bootstrap, type-safely. */
function hcuLimitInitArgs(config: HCULimitInitConfig): readonly unknown[] {
  return [config.hcuCapPerBlock, config.maxHCUDepthPerTx, config.maxHCUPerTx];
}
