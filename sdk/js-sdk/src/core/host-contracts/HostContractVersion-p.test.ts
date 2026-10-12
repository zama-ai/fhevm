import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { getHostContractVersion, invalidateVersionCache } from './HostContractVersion-p.js';

const ACL_ADDRESS = sepolia.fhevm.contracts.acl.address as ChecksummedAddress;
const KMS_VERIFIER_ADDRESS = sepolia.fhevm.contracts.kmsVerifier.address as ChecksummedAddress;
const PROTOCOL_CONFIG_ADDRESS = sepolia.fhevm.contracts.protocolConfig?.address as ChecksummedAddress;

function makeClient(
  readContract: EthereumModule['readContract'],
): ReturnType<typeof createCoreFhevm<typeof sepolia, ReturnType<typeof createFhevmRuntime>, object>> {
  const ethereum = {
    readContract,
  } as unknown as EthereumModule;

  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum,
    relayer: {} as RelayerModule,
    config: {},
  });

  return createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
    chain: sepolia,
    client: {},
    runtime,
  });
}

function makeReadContract(
  versionsByAddress: ReadonlyMap<ChecksummedAddress, string[]>,
): EthereumModule['readContract'] {
  return vi.fn(async (_trustedClient, parameters) => {
    const versions = versionsByAddress.get(parameters.address);
    const version = versions?.shift();
    if (version === undefined) {
      throw new Error(`No mocked version for ${parameters.address}`);
    }
    return version;
  });
}

beforeEach(() => {
  invalidateVersionCache({ includeInflight: true });
});

describe('HostContractVersion cache', () => {
  it('invalidates one cached host contract version entry', async () => {
    const readContract = makeReadContract(
      new Map<ChecksummedAddress, string[]>([
        [ACL_ADDRESS, ['ACL v0.1.0', 'ACL v0.2.0']],
        [KMS_VERIFIER_ADDRESS, ['KMSVerifier v0.1.0', 'KMSVerifier v0.2.0']],
      ]),
    );
    const client = makeClient(readContract);

    await expect(getHostContractVersion(client, { address: ACL_ADDRESS })).resolves.toMatchObject({
      version: 'ACL v0.1.0',
    });
    await expect(getHostContractVersion(client, { address: KMS_VERIFIER_ADDRESS })).resolves.toMatchObject({
      version: 'KMSVerifier v0.1.0',
    });
    expect(readContract).toHaveBeenCalledTimes(2);

    invalidateVersionCache(client, { address: ACL_ADDRESS });

    await expect(getHostContractVersion(client, { address: ACL_ADDRESS })).resolves.toMatchObject({
      version: 'ACL v0.2.0',
    });
    await expect(getHostContractVersion(client, { address: KMS_VERIFIER_ADDRESS })).resolves.toMatchObject({
      version: 'KMSVerifier v0.1.0',
    });
    expect(readContract).toHaveBeenCalledTimes(3);
  });

  it('invalidates all cached host contract version entries', async () => {
    const readContract = makeReadContract(
      new Map<ChecksummedAddress, string[]>([
        [ACL_ADDRESS, ['ACL v0.1.0', 'ACL v0.2.0']],
        [KMS_VERIFIER_ADDRESS, ['KMSVerifier v0.1.0', 'KMSVerifier v0.2.0']],
      ]),
    );
    const client = makeClient(readContract);

    await expect(getHostContractVersion(client, { address: ACL_ADDRESS })).resolves.toMatchObject({
      version: 'ACL v0.1.0',
    });
    await expect(getHostContractVersion(client, { address: KMS_VERIFIER_ADDRESS })).resolves.toMatchObject({
      version: 'KMSVerifier v0.1.0',
    });
    expect(readContract).toHaveBeenCalledTimes(2);

    invalidateVersionCache();

    await expect(getHostContractVersion(client, { address: ACL_ADDRESS })).resolves.toMatchObject({
      version: 'ACL v0.2.0',
    });
    await expect(getHostContractVersion(client, { address: KMS_VERIFIER_ADDRESS })).resolves.toMatchObject({
      version: 'KMSVerifier v0.2.0',
    });
    expect(readContract).toHaveBeenCalledTimes(4);
  });
});

describe('HostContractVersion parsing', () => {
  it('parses a ProtocolConfigReplica version', async () => {
    const readContract = makeReadContract(
      new Map<ChecksummedAddress, string[]>([[PROTOCOL_CONFIG_ADDRESS, ['ProtocolConfigReplica v0.1.0']]]),
    );
    const client = makeClient(readContract);

    await expect(getHostContractVersion(client, { address: PROTOCOL_CONFIG_ADDRESS })).resolves.toEqual({
      version: 'ProtocolConfigReplica v0.1.0',
      contractName: 'ProtocolConfigReplica',
      major: 0,
      minor: 1,
      patch: 0,
    });
  });

  it('rejects an unknown contract name', async () => {
    const readContract = makeReadContract(
      new Map<ChecksummedAddress, string[]>([[PROTOCOL_CONFIG_ADDRESS, ['ProtocolConfigMirror v0.1.0']]]),
    );
    const client = makeClient(readContract);

    await expect(getHostContractVersion(client, { address: PROTOCOL_CONFIG_ADDRESS })).rejects.toThrow(
      'Invalid version format: "ProtocolConfigMirror v0.1.0".',
    );
  });
});
