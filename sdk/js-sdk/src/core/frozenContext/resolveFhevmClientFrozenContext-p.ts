import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import type { HostContractVersion } from '../types/hostContract.js';
import type { FhevmClientFrozenContext } from './fhevmClientFrozenContext-p.js';
import { addressToChecksummedAddress, asAddress } from '../base/address.js';
import { executeWithBatching } from '../base/promise.js';
import { assertIsHostContractVersionOf, getHostContractVersion } from '../host-contracts/HostContractVersion-p.js';
import { hyperWasmResolveTfheModuleVersion, hyperWasmResolveTkmsModuleVersion } from '../runtime/HyperWasmSolver-p.js';
import { protocolContextFromAclVersion } from '../runtime/ProtocolVersionResolver-p.js';
import { createFhevmClientFrozenContext } from './createFhevmClientFrozenContext-p.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves the **complete** version basis of a client into an immutable
 * {@link FhevmClientFrozenContext}, captured in a single pass:
 *
 * - the host-contract versions — ACL, InputVerifier, KMSVerifier,
 * - the derived protocol + PubKey/CRS versions, and
 * - both the TFHE and TKMS module versions.
 *
 * ### One capture, every tier
 *
 * The full basis is resolved regardless of the client's current tier — a base or
 * encrypt-only client resolves the decrypt-side versions too. This is deliberate:
 * it captures every version at a **single moment**, so a later `extend()` (e.g.
 * encrypt → encrypt+decrypt) reuses the already-resolved values instead of
 * reading the chain a second time. A second, later read could straddle a contract
 * upgrade and mix versions from two moments into one basis — the exact drift this
 * context exists to prevent — and capturing everything up front is what lets us
 * skip block-number pinning (and its reorg exposure) entirely.
 *
 * Resolving the full basis is cheap: over a base-only resolution it adds just the
 * InputVerifier / KMSVerifier `getVersion()` reads (TTL-cached, and batched into
 * the same call here); protocol / PubKey-CRS / TFHE / TKMS are pure table
 * lookups. It does **not** load any WASM — module *loading* stays per-tier, so a
 * base client resolves the TFHE/TKMS *versions* without ever pulling in their
 * WASM binaries.
 *
 * ### Independent of client init state
 *
 * Every value is read fresh from chain (`getHostContractVersion`) or derived
 * purely (`protocolContextFromAclVersion`, `hyperWasmResolve*ModuleVersion`); the
 * resolver never consults the client's init-time memoized `getResolved*` values.
 *
 * @internal
 */
export async function resolveFhevmClientFrozenContext(
  fhevm: FhevmBase<FhevmChain>,
): Promise<FhevmClientFrozenContext> {
  const aclAddress: ChecksummedAddress = addressToChecksummedAddress(
    asAddress(fhevm.chain.fhevm.contracts.acl.address),
  );
  const inputVerifierAddress: ChecksummedAddress = addressToChecksummedAddress(
    asAddress(fhevm.chain.fhevm.contracts.inputVerifier.address),
  );
  const kmsVerifierAddress: ChecksummedAddress = addressToChecksummedAddress(
    asAddress(fhevm.chain.fhevm.contracts.kmsVerifier.address),
  );

  // Design note — reads are intentionally NOT pinned to a block number.
  // All protocol contracts (ACL, KMSVerifier, InputVerifier, ProtocolConfig, …)
  // are upgraded together in a single atomic transaction that bumps every
  // `getVersion()` in one step, so this batch either all observes the pre-upgrade
  // versions or all observes the post-upgrade ones — never a mixed, half-applied
  // state. Capturing the whole basis in one call is what keeps it coherent.
  const [aclVersion, inputVerifierVersion, kmsVerifierVersion] = (await executeWithBatching<unknown>(
    [
      () => getHostContractVersion(fhevm, { address: aclAddress }),
      () => getHostContractVersion(fhevm, { address: inputVerifierAddress }),
      () => getHostContractVersion(fhevm, { address: kmsVerifierAddress }),
    ],
    fhevm.options.batchRpcCalls,
  )) as [HostContractVersion, HostContractVersion, HostContractVersion];

  assertIsHostContractVersionOf(aclVersion, 'ACL');
  assertIsHostContractVersionOf(inputVerifierVersion, 'InputVerifier');
  assertIsHostContractVersionOf(kmsVerifierVersion, 'KMSVerifier');

  const protocolContext = protocolContextFromAclVersion(fhevm.chain, aclVersion);

  const tfheVersion = hyperWasmResolveTfheModuleVersion(fhevm, protocolContext);
  const tkmsVersion = hyperWasmResolveTkmsModuleVersion(fhevm, protocolContext);

  return createFhevmClientFrozenContext({
    hostContractVersions: {
      ACL: aclVersion,
      InputVerifier: inputVerifierVersion,
      KMSVerifier: kmsVerifierVersion,
    },
    protocolVersion: protocolContext.protocolVersion,
    pubKeyCrsVersion: protocolContext.pubKeyCrsVersion,
    tfheVersion,
    tkmsVersion,
  });
}
