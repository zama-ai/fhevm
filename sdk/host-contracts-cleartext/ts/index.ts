import { abi as aclAbi, template as aclTemplate } from './artifacts/ACL.js';
import { abi as fhevmExecutorAbi, template as fhevmExecutorTemplate } from './artifacts/CleartextFHEVMExecutor.js';
import { abi as kmsVerifierAbi, template as kmsVerifierTemplate } from './artifacts/CleartextKMSVerifier.js';
import { abi as inputVerifierAbi, template as inputVerifierTemplate } from './artifacts/CleartextInputVerifier.js';
import { abi as hcuLimitAbi, template as hcuLimitTemplate } from './artifacts/HCULimit.js';
import { abi as protocolConfigAbi, template as protocolConfigTemplate } from './artifacts/ProtocolConfig.js';
import { abi as kmsGenerationAbi, template as kmsGenerationTemplate } from './artifacts/KMSGeneration.js';
import {
  abi as cleartextArithmeticAbi,
  template as cleartextArithmeticTemplate,
} from './artifacts/CleartextArithmetic.js';
import { abi as cleartextDbAbi, template as cleartextDbTemplate } from './artifacts/CleartextDB.js';
import { template as pauserSetTemplate } from './artifacts/PauserSet.js';
import { abi as aclOwnerAbi } from './artifacts/ACLOwner.js';
import type {
  AddressReplacement,
  ContractUpgradeSpec,
  DeployedImplementation,
  HexString,
  UpgradeTarget,
} from './private.js';
import type {
  AbstractEthereumProvider,
  AbstractEthereumSigner,
  AbstractEthereumUtils,
  CleartextAddresses,
  DeployReturnType,
  FhevmAddressesV12,
  FhevmAddressesV14,
} from './public.js';
import {
  assertDeployedAddress,
  assertNoCodeAt,
  patchTemplateBytecode,
  assertNoCodeAtTargets,
  deployImplementation,
} from './utils.js';
import { deployACLProxy, deployEmptyUUPSProxy, deployEmptyUUPSProxyACL, deployERC1967Proxy } from './proxies.js';
import { setupACLOwner, toACLOwnerOps } from './aclOwner.js';

////////////////////////////////////////////////////////////////////////////////

// Public API surface: re-export the abstract adapter interfaces and shared types so consumers can
// import them from the package entry.
export type {
  AbstractEthereumProvider,
  AbstractEthereumSigner,
  AbstractEthereumUtils,
  DeployParameters,
  DeployReturnType,
  EncodeCallParameters,
  FhevmAddressesV12,
  FhevmAddressesV14,
} from './public.js';

////////////////////////////////////////////////////////////////////////////////

export type FhevmAddressAllocationV12 = {
  readonly fhevmAddresses: FhevmAddressesV12;
  readonly nextStartNonce: bigint;
};

export type FhevmAddressAllocationV14 = {
  readonly fhevmAddresses: FhevmAddressesV14;
  readonly nextStartNonce: bigint;
};

export function precomputeFhevmAddressesV12(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly from: `0x${string}`;
  readonly startNonce: bigint;
}): FhevmAddressAllocationV12 {
  return {
    fhevmAddresses: {
      aclAddress: parameters.ethUtils.getContractAddress({ from: parameters.from, nonce: parameters.startNonce + 1n }),
      fhevmExecutorAddress: parameters.ethUtils.getContractAddress({
        from: parameters.from,
        nonce: parameters.startNonce + 3n,
      }),
      kmsVerifierAddress: parameters.ethUtils.getContractAddress({
        from: parameters.from,
        nonce: parameters.startNonce + 4n,
      }),
      inputVerifierAddress: parameters.ethUtils.getContractAddress({
        from: parameters.from,
        nonce: parameters.startNonce + 5n,
      }),
      hcuLimitAddress: parameters.ethUtils.getContractAddress({
        from: parameters.from,
        nonce: parameters.startNonce + 6n,
      }),
    },
    nextStartNonce: parameters.startNonce + 7n,
  };
}

export function precomputeFhevmAddressesV14(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly from: `0x${string}`;
  readonly startNonce: bigint;
}): FhevmAddressAllocationV14 {
  const v12 = precomputeFhevmAddressesV12(parameters);
  return {
    fhevmAddresses: {
      ...v12.fhevmAddresses,
      protocolConfigAddress: parameters.ethUtils.getContractAddress({
        from: parameters.from,
        nonce: v12.nextStartNonce + 0n,
      }),
      kmsGenerationAddress: parameters.ethUtils.getContractAddress({
        from: parameters.from,
        nonce: v12.nextStartNonce + 1n,
      }),
    },
    nextStartNonce: v12.nextStartNonce + 2n,
  };
}

////////////////////////////////////////////////////////////////////////////////

export function precomputeAddresses(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly from: `0x${string}`;
  readonly startNonce: bigint;
}): {
  fhevmAddresses: FhevmAddressesV14;
  cleartextAddresses: CleartextAddresses;
  pauserSetAddress: string;
  nextStartNonce: bigint;
} {
  const { fhevmAddresses, nextStartNonce } = precomputeFhevmAddressesV14(parameters);
  // Cleartext infra proxies follow the v13 core, then PauserSet.
  const cleartextAddresses: CleartextAddresses = {
    cleartextArithmeticAddress: parameters.ethUtils.getContractAddress({
      from: parameters.from,
      nonce: nextStartNonce,
    }),
    cleartextDbAddress: parameters.ethUtils.getContractAddress({ from: parameters.from, nonce: nextStartNonce + 1n }),
  };
  return {
    fhevmAddresses,
    cleartextAddresses,
    pauserSetAddress: parameters.ethUtils.getContractAddress({ from: parameters.from, nonce: nextStartNonce + 2n }),
    nextStartNonce: nextStartNonce + 3n,
  };
}

////////////////////////////////////////////////////////////////////////////////

/** Bootstrap init values for a fresh v13 stack (`deploy`). One entry per proxy that takes init args;
 * ACL/FHEVMExecutor/KMSGeneration take none. */
export type BootstrapConfigV14 = {
  readonly kmsVerifier: { readonly verifyingContractSource: string; readonly chainIDSource: bigint };
  readonly inputVerifier: EIP712VerifierInitConfig;
  readonly hcuLimit: HCULimitInitConfig;
  readonly protocolConfig: {
    readonly initialKmsNodeParams: readonly KmsNodeParams[];
    readonly initialThresholds: KmsThresholds;
    /** v14: KMS Core software version recorded on the initial epoch. */
    readonly softwareVersion: string;
    /** v14: enclave PCR measurements for the initial epoch. Empty for the cleartext stack. */
    readonly pcrValues: readonly PcrValues[];
  };
};

/** Result of `deploy`: the full v14 address set plus the standing admin. */
export type DeployedV14 = {
  readonly fhevmAddresses: FhevmAddressesV14;
  readonly cleartextAddresses: CleartextAddresses;
  readonly pauserSetAddress: string;
  readonly aclOwnerAddress: string;
};

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
    readonly fhevmAddresses: FhevmAddressesV14;
    readonly cleartextAddresses: CleartextAddresses;
    readonly pauserSetAddress: string;
  };
  readonly config: BootstrapConfigV14;
}): Promise<DeployedV14> {
  const { fhevmAddresses, cleartextAddresses } = parameters.precomputed;

  // 1. Deploy the 7 core empty proxies, then the 2 cleartext-infra proxies (on the shared impl).
  const { emptyUUPSProxyAddress } = await deployEmptyProxiesV14({
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
  const { implementations } = await buildBootstrapPlanV14({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    precomputedAddresses: fhevmAddresses,
    cleartextAddresses,
    config: bootstrapUpgradeConfigV14({
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

/** Maps the typed bootstrap config to a full `UpgradeConfigV14` of `initializeFromEmptyProxy` specs. */
function bootstrapUpgradeConfigV14(parameters: {
  readonly pauserSetAddress: string;
  readonly cleartextAddresses: CleartextAddresses;
  readonly config: BootstrapConfigV14;
}): UpgradeConfigV14 {
  const { config } = parameters;
  const bootstrap = (initArgs: readonly unknown[]): ContractUpgradeSpec => ({
    initFn: 'initializeFromEmptyProxy',
    initArgs,
  });
  return {
    pauserSetAddress: parameters.pauserSetAddress,
    acl: bootstrap([]),
    fhevmExecutor: bootstrap([]),
    kmsVerifier: bootstrap(kmsVerifierInitArgsV14(config.kmsVerifier)),
    inputVerifier: bootstrap(eip712VerifierInitArgs(config.inputVerifier)),
    hcuLimit: bootstrap(hcuLimitInitArgs(config.hcuLimit)),
    // v14 `initializeFromEmptyProxy(KmsNodeParams[], KmsThresholds, string, PcrValues[])` — two more
    // args than v13, which took only (KmsNode[], KmsThresholds).
    protocolConfig: bootstrap([
      config.protocolConfig.initialKmsNodeParams,
      config.protocolConfig.initialThresholds,
      config.protocolConfig.softwareVersion,
      config.protocolConfig.pcrValues,
    ]),
    kmsGeneration: bootstrap([]),
    cleartextArithmetic: bootstrap([]),
    // CleartextDB.initializeFromEmptyProxy(initialWriter) — seed CleartextArithmetic as the writer.
    cleartextDb: bootstrap([parameters.cleartextAddresses.cleartextArithmeticAddress]),
  };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Isolated PauserSet deployment step. `PauserSet` is deployed by its own dedicated
 * `pauserSetDeployer` (not the main host deployer), so its address is a plain CREATE address of that
 * signer at `startNonce` (default `0n`, i.e. a fresh deployer's first transaction).
 *
 * Asserts the target slot is empty, deploys, and asserts it landed at the expected address.
 */
export async function deployPauserSetContract(parameters: {
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

export async function deployEmptyProxiesV12(parameters: {
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

export async function deployEmptyProxiesV14(parameters: {
  readonly ethProvider: AbstractEthereumProvider;
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly precomputedFhevmAddresses: FhevmAddressesV14;
}): Promise<{ emptyUUPSProxyAddress: DeployReturnType }> {
  const { emptyUUPSProxyAddress } = await deployEmptyProxiesV12(parameters);

  const targetsV14: ReadonlyArray<{ readonly contractName: string; readonly address: string }> = [
    { contractName: 'ProtocolConfig', address: parameters.precomputedFhevmAddresses.protocolConfigAddress },
    { contractName: 'KMSGeneration', address: parameters.precomputedFhevmAddresses.kmsGenerationAddress },
  ];

  // Assert none of the target host addresses are already occupied before deploying anything.
  await assertNoCodeAtTargets({
    ethProvider: parameters.ethProvider,
    targets: targetsV14,
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
 * shared `EmptyUUPSProxy` implementation. Called after `deployEmptyProxiesV14`, before PauserSet, so
 * their CREATE addresses match `precomputeAddresses`.
 */
export async function deployCleartextEmptyProxies(parameters: {
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
export async function deployPauserSet(parameters: {
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
export type UpgradeConfigV14 = {
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

/**
 * Bootstrap init values for `KMSVerifier.initializeFromEmptyProxy` /
 * `InputVerifier.initializeFromEmptyProxy` (identical signatures):
 * `(address verifyingContractSource, uint64 chainIDSource, address[] initialSigners, uint256 initialThreshold)`.
 */
export type EIP712VerifierInitConfig = {
  readonly verifyingContractSource: string;
  readonly chainIDSource: bigint;
  readonly initialSigners: readonly string[];
  readonly initialThreshold: bigint;
};

/**
 * Bootstrap init values for `HCULimit.initializeFromEmptyProxy`:
 * `(uint48 hcuCapPerBlock, uint48 maxHCUDepthPerTx, uint48 maxHCUPerTx)`.
 */
export type HCULimitInitConfig = {
  readonly hcuCapPerBlock: bigint;
  readonly maxHCUDepthPerTx: bigint;
  readonly maxHCUPerTx: bigint;
};

/**
 * A KMS node entry for `ProtocolConfig` (v14). Mirrors the on-chain `KmsNodeParams` struct; passed as
 * an object — viem matches the tuple by component name.
 *
 * v14 replaced v13's `KmsNode` with `KmsNodeParams`, adding the MPC connection metadata
 * (`partyId`, `mpcIdentity`, `caCert`, `storagePrefix`). The cleartext stack never talks to a real
 * KMS, so these can be any well-formed placeholder values.
 */
export type KmsNodeParams = {
  readonly txSenderAddress: string;
  readonly signerAddress: string;
  readonly ipAddress: string;
  readonly storageUrl: string;
  /** Solidity `int32`. */
  readonly partyId: number;
  readonly mpcIdentity: string;
  /** Solidity `bytes`. */
  readonly caCert: HexString;
  readonly storagePrefix: string;
};

/**
 * Nitro-enclave PCR measurements attested at KMS-context activation (v14). Mirrors the on-chain
 * `PcrValues` struct. Unused by the cleartext stack — pass empty byte strings.
 */
export type PcrValues = {
  /** Solidity `bytes`. */
  readonly pcr0: HexString;
  /** Solidity `bytes`. */
  readonly pcr1: HexString;
  /** Solidity `bytes`. */
  readonly pcr2: HexString;
};

/** The four KMS thresholds for `ProtocolConfig` (v14). Mirrors the on-chain `KmsThresholds` struct. */
export type KmsThresholds = {
  readonly publicDecryption: bigint;
  readonly userDecryption: bigint;
  readonly kmsGen: bigint;
  readonly mpc: bigint;
};

/** Builds `ContractUpgradeSpec.initArgs` for InputVerifier bootstrap, type-safely. */
export function eip712VerifierInitArgs(config: EIP712VerifierInitConfig): readonly unknown[] {
  return [config.verifyingContractSource, config.chainIDSource, config.initialSigners, config.initialThreshold];
}

/**
 * Builds `ContractUpgradeSpec.initArgs` for v13 `KMSVerifier.initializeFromEmptyProxy`
 * `(address verifyingContractSource, uint64 chainIDSource)`. In v13 the KMS signer set moved to
 * `ProtocolConfig`, so — unlike v12 — no signers/threshold are passed here.
 */
export function kmsVerifierInitArgsV14(config: {
  readonly verifyingContractSource: string;
  readonly chainIDSource: bigint;
}): readonly unknown[] {
  return [config.verifyingContractSource, config.chainIDSource];
}

/** Builds `ContractUpgradeSpec.initArgs` for HCULimit bootstrap, type-safely. */
export function hcuLimitInitArgs(config: HCULimitInitConfig): readonly unknown[] {
  return [config.hcuCapPerBlock, config.maxHCUDepthPerTx, config.maxHCUPerTx];
}

/** Deploys each target's implementation and encodes its `upgradeToAndCall` (Phase 1; sends nothing). */
async function deployImplementations(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly addressReplacements: readonly AddressReplacement[];
  readonly targets: readonly UpgradeTarget[];
}): Promise<readonly DeployedImplementation[]> {
  const implementations: DeployedImplementation[] = [];
  for (const target of parameters.targets) {
    implementations.push(
      await deployImplementation({
        ethUtils: parameters.ethUtils,
        deployer: parameters.deployer,
        contractName: target.contractName,
        proxyAddress: target.proxyAddress,
        template: target.template,
        abi: target.abi,
        addressReplacements: parameters.addressReplacements,
        spec: target.spec,
      }),
    );
  }
  return implementations;
}

/**
 * Phase 1 for a fresh v13 stack: deploys all 7 real implementations (patched with v13 host addresses)
 * and encodes their `upgradeToAndCall` calldata. Sends no owner-gated transaction.
 */
async function buildBootstrapPlanV14(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly precomputedAddresses: FhevmAddressesV14;
  readonly cleartextAddresses: CleartextAddresses;
  readonly config: UpgradeConfigV14;
}): Promise<{ readonly implementations: readonly DeployedImplementation[] }> {
  const addressReplacements = buildHostAddressReplacementsV14({
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

/**
 * The eight host addresses baked into every real implementation's bytecode (via `FHEVMHostAddresses.sol`).
 * References whose offsets are empty for a given template patch as no-ops, so passing all eight is safe.
 */
function buildHostAddressReplacementsV14(parameters: {
  readonly fhevmAddresses: FhevmAddressesV14;
  readonly pauserSetAddress: string;
  // Optional: only the fresh `deploy` materializes the cleartext-infra contracts. The (deferred)
  // v12→v13 update path omits them until the cleartext-v12 fixture lands (see plan Decision #4).
  readonly cleartextAddresses?: CleartextAddresses;
}): AddressReplacement[] {
  const replacements: AddressReplacement[] = [
    // v0.12.0
    { referenceName: 'ACL_ADDRESS', replacement: parameters.fhevmAddresses.aclAddress },
    { referenceName: 'FHEVM_EXECUTOR_ADDRESS', replacement: parameters.fhevmAddresses.fhevmExecutorAddress },
    { referenceName: 'KMS_VERIFIER_ADDRESS', replacement: parameters.fhevmAddresses.kmsVerifierAddress },
    { referenceName: 'INPUT_VERIFIER_ADDRESS', replacement: parameters.fhevmAddresses.inputVerifierAddress },
    { referenceName: 'HCU_LIMIT_ADDRESS', replacement: parameters.fhevmAddresses.hcuLimitAddress },
    // v0.13.0
    { referenceName: 'PROTOCOL_CONFIG_ADDRESS', replacement: parameters.fhevmAddresses.protocolConfigAddress },
    { referenceName: 'KMS_GENERATION_ADDRESS', replacement: parameters.fhevmAddresses.kmsGenerationAddress },
    { referenceName: 'PAUSER_SET_ADDRESS', replacement: parameters.pauserSetAddress },
  ];

  if (parameters.cleartextAddresses !== undefined) {
    replacements.push(
      {
        referenceName: 'CLEARTEXT_ARITHMETIC_ADDRESS',
        replacement: parameters.cleartextAddresses.cleartextArithmeticAddress,
      },
      { referenceName: 'CLEARTEXT_DB_ADDRESS', replacement: parameters.cleartextAddresses.cleartextDbAddress },
    );
  }

  return replacements;
}
