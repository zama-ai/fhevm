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
  AbstractEthereumSigner,
  AbstractEthereumUtils,
  CleartextAddresses,
  FhevmAddressesV14,
  UpdateV13ToV14MigrationConfig,
} from './types/public.js';
import { buildHostAddressReplacementsV14, deployImplementations } from './utils.js';
import { toACLOwnerOps } from './aclOwner.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Update an already-deployed v13 stack to v14.
 *
 * Unlike v12→v13, no new proxies are deployed — v14 has the same contract set as v13. The whole update
 * is one atomic `ACLOwner.upgrade(...)` that re-points + version-bumps the seven changed contracts:
 *   - `ProtocolConfig` via `reinitializeV2(kmsNodeParams, softwareVersion, pcrValues)` — v14 anchors
 *     each KMS context to its full node set + software version + PCR values, which the v13 contract
 *     never stored, so the operator supplies them (`migration`),
 *   - `KMSGeneration` via `reinitializeV2()` (backfills the completed key/CRS id arrays),
 *   - ACL/FHEVMExecutor `reinitializeV5`, HCULimit/KMSVerifier `reinitializeV4`,
 *     CleartextArithmetic `reinitializeV3` — all no-arg.
 * `InputVerifier` and `CleartextDB` are untouched (their v14 bytecode is identical and their versions
 * did not bump).
 *
 * All v14 implementations are patched with the ACTUAL addresses of the live stack.
 *
 * @dev Requires the live stack's ACL owner to already be the `ACLOwner` at `aclOwnerAddress`, and
 *      `admin` to be that `ACLOwner`'s owner. If the stack is EOA-owned, install one first
 *      (`setupACLOwner`).
 */
export async function updateV13ToV14(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly admin: AbstractEthereumSigner;
  readonly aclOwnerAddress: string;
  readonly existing: FhevmAddressesV14 & { readonly pauserSetAddress: string };
  // Live cleartext-infra proxies of the running v13 stack. The new v14 `CleartextFHEVMExecutor` impl
  // bakes `cleartextArithmeticAdd`/`cleartextDbAdd`, so they must be patched with the real proxy
  // addresses (not the placeholders) or the post-upgrade cleartext round-trip would call a dead address.
  readonly cleartext: CleartextAddresses;
  readonly migration: UpdateV13ToV14MigrationConfig;
}): Promise<void> {
  const { pauserSetAddress, ...fhevmAddresses } = parameters.existing;

  // 1. Phase 1: deploy the v14 implementations (permissionless).
  const { implementations } = await buildUpdateV13ToV14Plan({
    ethUtils: parameters.ethUtils,
    deployer: parameters.deployer,
    fhevmAddresses,
    cleartextAddresses: parameters.cleartext,
    pauserSetAddress,
    migration: parameters.migration,
  });

  // 2. One atomic ACLOwner.upgrade: 7 reinitializations.
  await parameters.admin.writeContract({
    address: parameters.aclOwnerAddress,
    abi: aclOwnerAbi,
    functionName: 'upgrade',
    args: [toACLOwnerOps(implementations)],
  });
}

/**
 * Phase 1 for a v13→v14 update: deploys the v14 implementations for the changed contracts (patched
 * with the actual live addresses) and encodes their `upgradeToAndCall` calldata. Sends no owner-gated
 * transaction. `InputVerifier` and `CleartextDB` are intentionally absent (their v14 bytecode is
 * unchanged).
 * @internal — used by `updateV13ToV14`; not part of the public API.
 */
async function buildUpdateV13ToV14Plan(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly fhevmAddresses: FhevmAddressesV14;
  readonly cleartextAddresses: CleartextAddresses;
  readonly pauserSetAddress: string;
  readonly migration: UpdateV13ToV14MigrationConfig;
}): Promise<{ readonly implementations: readonly DeployedImplementation[] }> {
  const addressReplacements = buildHostAddressReplacementsV14({
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
        initFn: 'reinitializeV2',
        initArgs: [
          parameters.migration.kmsNodeParams,
          parameters.migration.softwareVersion,
          parameters.migration.pcrValues,
        ],
      },
    },
    {
      contractName: 'KMSGeneration',
      proxyAddress: addr.kmsGenerationAddress,
      template: kmsGenerationTemplate,
      abi: kmsGenerationAbi,
      spec: noArgs('reinitializeV2'),
    },
    {
      contractName: 'ACL',
      proxyAddress: addr.aclAddress,
      template: aclTemplate,
      abi: aclAbi,
      spec: noArgs('reinitializeV5'),
    },
    {
      contractName: 'FHEVMExecutor',
      proxyAddress: addr.fhevmExecutorAddress,
      template: fhevmExecutorTemplate,
      abi: fhevmExecutorAbi,
      spec: noArgs('reinitializeV5'),
    },
    {
      contractName: 'HCULimit',
      proxyAddress: addr.hcuLimitAddress,
      template: hcuLimitTemplate,
      abi: hcuLimitAbi,
      spec: noArgs('reinitializeV4'),
    },
    {
      contractName: 'KMSVerifier',
      proxyAddress: addr.kmsVerifierAddress,
      template: kmsVerifierTemplate,
      abi: kmsVerifierAbi,
      spec: noArgs('reinitializeV4'),
    },
    // v14 added the `fheMulDiv` operator, whose cleartext compute hook is a new selector on
    // `CleartextArithmetic`. The v13 arithmetic lacks it, so the upgrade must re-point this proxy at
    // the v14 implementation too (stateless bump via `reinitializeV3`).
    {
      contractName: 'CleartextArithmetic',
      proxyAddress: parameters.cleartextAddresses.cleartextArithmeticAddress,
      template: cleartextArithmeticTemplate,
      abi: cleartextArithmeticAbi,
      spec: noArgs('reinitializeV3'),
    },
  ];

  return { implementations: await deployImplementations({ ...parameters, addressReplacements, targets }) };
}
