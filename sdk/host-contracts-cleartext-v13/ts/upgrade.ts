import { abi as aclAbi, template as aclTemplate } from './artifacts/ACL.js';
import { abi as fhevmExecutorAbi, template as fhevmExecutorTemplate } from './artifacts/CleartextFHEVMExecutor.js';
import { abi as kmsVerifierAbi, template as kmsVerifierTemplate } from './artifacts/CleartextKMSVerifier.js';
import { abi as hcuLimitAbi, template as hcuLimitTemplate } from './artifacts/HCULimit.js';
import { abi as protocolConfigAbi, template as protocolConfigTemplate } from './artifacts/ProtocolConfig.js';
import { abi as kmsGenerationAbi, template as kmsGenerationTemplate } from './artifacts/KMSGeneration.js';
import {
  abi as cleartextArithmeticAbi,
  template as cleartextArithmeticTemplate,
} from './artifacts/CleartextArithmetic.js';
import { abi as aclOwnerAbi } from './artifacts/ACLOwner.js';
import type { ContractUpgradeSpec, DeployedImplementation, UpgradeTarget } from './types/private.js';
import type {
  AbstractEthereumProvider,
  AbstractEthereumSigner,
  AbstractEthereumUtils,
  CleartextAddresses,
  FhevmAddressesV12,
  FhevmAddressesV13,
  UpdateV12ToV13MigrationConfig,
} from './types/public.js';
import { buildHostAddressReplacementsV13, deployImplementations } from './utils.js';
import { deployEmptyUUPSProxy, deployERC1967Proxy } from './proxies.js';
import { toACLOwnerOps } from './aclOwner.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Update an already-deployed v12 stack to v13.
 *
 * Deploys the two new proxies (`ProtocolConfig`, `KMSGeneration`) — at wherever `deployer`'s nonce
 * lands, since a live v12 stack's layout is fixed — then, in one atomic `ACLOwner.upgrade(...)`:
 *   - materializes `ProtocolConfig` via `initializeFromMigration` (seeding the migrated KMS context),
 *   - materializes `KMSGeneration` via `initializeFromEmptyProxy`,
 *   - re-points + version-bumps the four changed v12 contracts (ACL/FHEVMExecutor `reinitializeV4`,
 *     HCULimit/KMSVerifier `reinitializeV3` — all no-arg).
 * `InputVerifier` is untouched (its v13 bytecode is identical and its version did not bump).
 *
 * All v13 implementations are patched with the ACTUAL addresses: the existing v12 proxies + existing
 * PauserSet + the two newly deployed proxies.
 *
 * @dev Requires the live stack's ACL owner to already be the `ACLOwner` at `aclOwnerAddress`, and
 *      `admin` to be that `ACLOwner`'s owner. If the stack is EOA-owned, install one first
 *      (`setupACLOwner`).
 */
export async function updateV12ToV13(parameters: {
  readonly ethProvider: AbstractEthereumProvider;
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly admin: AbstractEthereumSigner;
  readonly aclOwnerAddress: string;
  readonly existing: FhevmAddressesV12 & { readonly pauserSetAddress: string };
  // Live cleartext-infra proxies of the running v12 stack. The new v13 `CleartextFHEVMExecutor` impl
  // bakes `cleartextArithmeticAdd`, so it must be patched with the real proxy address (not the
  // placeholder) or the post-upgrade cleartext round-trip would call a dead address.
  readonly cleartext: CleartextAddresses;
  readonly migration: UpdateV12ToV13MigrationConfig;
}): Promise<{ readonly protocolConfigAddress: string; readonly kmsGenerationAddress: string }> {
  const { pauserSetAddress, ...existingV12 } = parameters.existing;

  // 1. Deploy a fresh shared EmptyUUPSProxy impl (patched with the existing ACL address), then the two
  //    new v13 proxies pointing at it.
  const emptyImpl = await deployEmptyUUPSProxy({
    deployer: parameters.deployer,
    aclAddress: existingV12.aclAddress,
  });
  const protocolConfigProxy = await deployERC1967Proxy({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    emptyUUPSProxyAddress: emptyImpl.contractAddress,
  });
  const kmsGenerationProxy = await deployERC1967Proxy({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    emptyUUPSProxyAddress: emptyImpl.contractAddress,
  });

  const fhevmV13: FhevmAddressesV13 = {
    ...existingV12,
    protocolConfigAddress: protocolConfigProxy.contractAddress,
    kmsGenerationAddress: kmsGenerationProxy.contractAddress,
  };

  // 2. Phase 1: deploy the v13 implementations (permissionless).
  const { implementations } = await buildUpdateV12ToV13Plan({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    fhevmAddresses: fhevmV13,
    cleartextAddresses: parameters.cleartext,
    pauserSetAddress,
    migration: parameters.migration,
  });

  // 3. One atomic ACLOwner.upgrade: 2 materializations + 4 reinitializations.
  await parameters.admin.writeContract({
    address: parameters.aclOwnerAddress,
    abi: aclOwnerAbi,
    functionName: 'upgrade',
    args: [toACLOwnerOps(implementations)],
  });

  return {
    protocolConfigAddress: fhevmV13.protocolConfigAddress,
    kmsGenerationAddress: fhevmV13.kmsGenerationAddress,
  };
}

/**
 * Phase 1 for a v12→v13 update: deploys the v13 implementations for the changed/new contracts
 * (patched with the actual live addresses) and encodes their `upgradeToAndCall` calldata. Sends no
 * owner-gated transaction. The two new proxies (`ProtocolConfig`, `KMSGeneration`) must already exist
 * at `fhevmAddresses`; `InputVerifier` is intentionally absent (its v13 bytecode is unchanged).
 * @internal — used by `updateV12ToV13`; not part of the public API.
 */
async function buildUpdateV12ToV13Plan(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly fhevmAddresses: FhevmAddressesV13;
  readonly cleartextAddresses: CleartextAddresses;
  readonly pauserSetAddress: string;
  readonly migration: UpdateV12ToV13MigrationConfig;
}): Promise<{ readonly implementations: readonly DeployedImplementation[] }> {
  const addressReplacements = buildHostAddressReplacementsV13({
    fhevmAddresses: parameters.fhevmAddresses,
    cleartextAddresses: parameters.cleartextAddresses,
    pauserSetAddress: parameters.pauserSetAddress,
  });

  const addr = parameters.fhevmAddresses;
  const noArgs = (initFn: string): ContractUpgradeSpec => ({ initFn, initArgs: [] });
  const targets: readonly UpgradeTarget[] = [
    {
      contractName: 'ProtocolConfig',
      proxyAddress: addr.protocolConfigAddress,
      template: protocolConfigTemplate,
      abi: protocolConfigAbi,
      spec: {
        initFn: 'initializeFromMigration',
        initArgs: [
          parameters.migration.existingContextId,
          parameters.migration.existingKmsNodes,
          parameters.migration.existingThresholds,
        ],
      },
    },
    {
      contractName: 'KMSGeneration',
      proxyAddress: addr.kmsGenerationAddress,
      template: kmsGenerationTemplate,
      abi: kmsGenerationAbi,
      spec: noArgs('initializeFromEmptyProxy'),
    },
    {
      contractName: 'ACL',
      proxyAddress: addr.aclAddress,
      template: aclTemplate,
      abi: aclAbi,
      spec: noArgs('reinitializeV4'),
    },
    {
      contractName: 'FHEVMExecutor',
      proxyAddress: addr.fhevmExecutorAddress,
      template: fhevmExecutorTemplate,
      abi: fhevmExecutorAbi,
      spec: noArgs('reinitializeV4'),
    },
    {
      contractName: 'HCULimit',
      proxyAddress: addr.hcuLimitAddress,
      template: hcuLimitTemplate,
      abi: hcuLimitAbi,
      spec: noArgs('reinitializeV3'),
    },
    {
      contractName: 'KMSVerifier',
      proxyAddress: addr.kmsVerifierAddress,
      template: kmsVerifierTemplate,
      abi: kmsVerifierAbi,
      spec: noArgs('reinitializeV3'),
    },
    // v13 added the `fheSum`/`fheIsIn` operators, whose cleartext `record*` hooks are new selectors on
    // `CleartextArithmetic`. The v12 arithmetic lacks them, so the upgrade must re-point this proxy at
    // the v13 implementation too (stateless bump via `reinitializeV2`).
    {
      contractName: 'CleartextArithmetic',
      proxyAddress: parameters.cleartextAddresses.cleartextArithmeticAddress,
      template: cleartextArithmeticTemplate,
      abi: cleartextArithmeticAbi,
      spec: noArgs('reinitializeV2'),
    },
  ];

  return { implementations: await deployImplementations({ ...parameters, addressReplacements, targets }) };
}
