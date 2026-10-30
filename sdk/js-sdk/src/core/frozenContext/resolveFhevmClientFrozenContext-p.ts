import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import type { HostContractVersion } from '../types/hostContract.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { addressToChecksummedAddress, asAddress } from '../base/address.js';
import { executeWithBatching } from '../base/promise.js';
import { assertIsHostContractVersionOf, getHostContractVersion } from '../host-contracts/HostContractVersion-p.js';
import { pubKeyCrsVersionForChain } from '../runtime/ProtocolVersionResolver-p.js';
import { createFhevmClientFrozenContext } from './fhevmClientFrozenContext-p.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves the version basis of a client into an immutable
 * {@link FhevmClientFrozenContext}: the PubKey/CRS version, derived purely from
 * the chain (`pubKeyCrsVersionForChain`).
 *
 * TFHE / TKMS are not part of this basis: this SDK release ships a single
 * module for each, so {@link FhevmClientFrozenContext.tfheVersion} /
 * `.tkmsVersion` always report that canonical version directly, with nothing
 * to resolve or store here.
 *
 * It also reads `getVersion()` on the configured host contracts (ACL,
 * InputVerifier, KMSVerifier and, if configured, ProtocolConfig), in one batched
 * call, to check that each address points at the expected contract. The
 * versions themselves are not stored: the SDK does not branch on them.
 *
 * @internal
 */
export async function resolveFhevmClientFrozenContext(fhevm: FhevmBase<FhevmChain>): Promise<FhevmClientFrozenContext> {
  const aclAddress: ChecksummedAddress = addressToChecksummedAddress(
    asAddress(fhevm.chain.fhevm.contracts.acl.address),
  );
  const inputVerifierAddress: ChecksummedAddress = addressToChecksummedAddress(
    asAddress(fhevm.chain.fhevm.contracts.inputVerifier.address),
  );
  const kmsVerifierAddress: ChecksummedAddress = addressToChecksummedAddress(
    asAddress(fhevm.chain.fhevm.contracts.kmsVerifier.address),
  );
  // ProtocolConfig only exists on v0.14+ deployments — absent on older chains.
  const protocolConfigAddress: ChecksummedAddress | undefined =
    fhevm.chain.fhevm.contracts.protocolConfig === undefined
      ? undefined
      : addressToChecksummedAddress(asAddress(fhevm.chain.fhevm.contracts.protocolConfig.address));

  // Identity check only: each `getVersion()` result must name the expected contract.
  const [aclVersion, inputVerifierVersion, kmsVerifierVersion, protocolConfigVersion] =
    (await executeWithBatching<unknown>(
      [
        () => getHostContractVersion(fhevm, { address: aclAddress }),
        () => getHostContractVersion(fhevm, { address: inputVerifierAddress }),
        () => getHostContractVersion(fhevm, { address: kmsVerifierAddress }),
        ...(protocolConfigAddress === undefined
          ? []
          : [() => getHostContractVersion(fhevm, { address: protocolConfigAddress })]),
      ],
      fhevm.options.batchRpcCalls,
    )) as [HostContractVersion, HostContractVersion, HostContractVersion, HostContractVersion | undefined];

  assertIsHostContractVersionOf(aclVersion, 'ACL');
  assertIsHostContractVersionOf(inputVerifierVersion, 'InputVerifier');
  assertIsHostContractVersionOf(kmsVerifierVersion, 'KMSVerifier');
  if (protocolConfigVersion !== undefined) {
    assertIsHostContractVersionOf(protocolConfigVersion, 'ProtocolConfig', 'ProtocolConfigReplica');
  }

  return createFhevmClientFrozenContext({
    pubKeyCrsVersion: pubKeyCrsVersionForChain(fhevm.chain),
  });
}
