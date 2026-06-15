import type { Provider } from 'ethers';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import { formatError } from './formatError';

// Asserts the contract at `address` is the expected proxy by checking its getVersion() prefix.
// Defaults to the local network provider; pass `provider` to check a contract on another chain
// (e.g. the canonical ProtocolConfig read over a remote RPC).
export async function assertContractMatchesVersionPrefix(
  hre: HardhatRuntimeEnvironment,
  address: string,
  versionPrefix: string,
  provider: Provider = hre.ethers.provider,
): Promise<void> {
  const contract = new hre.ethers.Contract(address, ['function getVersion() view returns (string)'], provider);

  let version: string;
  try {
    version = await contract.getVersion();
  } catch (err) {
    throw new Error(
      `Contract at ${address} does not expose getVersion(); it is not a ${versionPrefix} proxy. (${formatError(err)})`,
    );
  }

  if (!version.startsWith(versionPrefix)) {
    throw new Error(`Contract at ${address} reports version "${version}"; expected "${versionPrefix} v…".`);
  }
}
