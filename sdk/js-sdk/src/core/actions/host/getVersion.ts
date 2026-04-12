import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import type { HostContractVersion } from '../../types/hostContract.js';
import { getVersion as getVersion_ } from '../../host-contracts/HostContractVersion-p.js';

export type GetVersionParameters = {
  readonly address: ChecksummedAddress;
  readonly forceRefresh?: boolean;
};

export type GetVersionReturnType = HostContractVersion;

/**
 * Reads the version of a host contract at the given address.
 *
 * Results are cached per address to avoid redundant RPC calls — concurrent
 * callers share the same in-flight request (deduplication), and subsequent
 * calls return the cached value instantly.
 *
 * Although host contract versions are generally immutable per deployment,
 * upgrades can occur in rare cases (e.g. proxy re-pointing). To account for
 * this, cached entries expire after 24 hours and are automatically refetched.
 *
 * @param parameters.address - The checksummed address of the host contract.
 * @param parameters.forceRefresh - If `true`, forces a fresh RPC call regardless
 *   of the cache state. Useful after a known contract upgrade.
 */
export async function getVersion(
  fhevm: Fhevm,
  parameters: GetVersionParameters,
): Promise<GetVersionReturnType> {
  return getVersion_(fhevm, parameters);
}
