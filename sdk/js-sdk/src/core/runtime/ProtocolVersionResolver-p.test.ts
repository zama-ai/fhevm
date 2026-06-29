import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { HostContractVersion } from '../types/hostContract.js';
import type { ChecksummedAddress, UintNumber } from '../types/primitives.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { baseActions } from '../clients/decorators/base.js';
import { invalidateVersionCache } from '../host-contracts/HostContractVersion-p.js';
import { createFhevmRuntime } from './CoreFhevmRuntime-p.js';
import { createCoreFhevm, getResolvedProtocolVersion } from './CoreFhevm-p.js';
import {
  protocolContextFromAclVersion,
  protocolVersionFromAclVersion,
  pubKeyCrsVersionFromProtocolVersion,
  resolveProtocolContext,
  resolveProtocolVersion,
} from './ProtocolVersionResolver-p.js';

const ACL_ADDRESS = sepolia.fhevm.contracts.acl.address as ChecksummedAddress;
const KNOWN_EXACT_ACL_PROTOCOL_CASES = [
  { acl: [0, 2, 0], protocolVersion: '0.11.0', pubKeyCrsVersion: '1.5.1' },
  { acl: [0, 3, 0], protocolVersion: '0.12.0', pubKeyCrsVersion: '1.5.4' },
  { acl: [0, 4, 0], protocolVersion: '0.13.0', pubKeyCrsVersion: '1.6.1' },
  { acl: [0, 5, 0], protocolVersion: '0.14.0', pubKeyCrsVersion: '1.6.1' },
] as const;

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

function makeAclVersion(major: number, minor: number, patch: number): HostContractVersion<'ACL'> {
  return {
    version: `ACL v${major}.${minor}.${patch}`,
    contractName: 'ACL',
    major: major as UintNumber,
    minor: minor as UintNumber,
    patch: patch as UintNumber,
  };
}

function makeChain(relayerUrl: string): typeof sepolia {
  return {
    ...sepolia,
    fhevm: {
      ...sepolia.fhevm,
      relayerUrl,
    },
  };
}

describe('ProtocolVersionResolver', () => {
  beforeEach(() => {
    invalidateVersionCache({ includeInflight: true });
  });

  it('maps ACL host-contract versions to protocol versions', () => {
    expect(protocolVersionFromAclVersion(makeAclVersion(0, 2, 0))).toEqual({ version: '0.11.0', comparator: 'eq' });
    expect(protocolVersionFromAclVersion(makeAclVersion(0, 2, 1))).toEqual({ version: '0.11.0', comparator: 'eq' });
    expect(protocolVersionFromAclVersion(makeAclVersion(0, 3, 0))).toEqual({ version: '0.12.0', comparator: 'eq' });
    expect(protocolVersionFromAclVersion(makeAclVersion(0, 3, 1))).toEqual({ version: '0.12.0', comparator: 'eq' });
    expect(protocolVersionFromAclVersion(makeAclVersion(0, 4, 0))).toEqual({ version: '0.13.0', comparator: 'eq' });
    expect(protocolVersionFromAclVersion(makeAclVersion(0, 4, 1))).toEqual({ version: '0.13.0', comparator: 'eq' });
    expect(protocolVersionFromAclVersion(makeAclVersion(0, 5, 0))).toEqual({ version: '0.14.0', comparator: 'eq' });
    expect(protocolVersionFromAclVersion(makeAclVersion(0, 5, 1))).toEqual({ version: '0.14.0', comparator: 'eq' });
  });

  it('uses the highest known protocol version as lower bound for unknown newer ACL versions', () => {
    expect(protocolVersionFromAclVersion(makeAclVersion(0, 6, 0))).toEqual({ version: '0.14.0', comparator: 'gt' });
  });

  it('uses the lowest known protocol version as upper bound for unknown older ACL versions', () => {
    expect(protocolVersionFromAclVersion(makeAclVersion(0, 1, 0))).toEqual({ version: '0.11.0', comparator: 'lt' });
  });

  it('maps ACL host-contract versions to protocol context', () => {
    const localstackLikeChain = makeChain('http://localhost:3000');

    expect(protocolContextFromAclVersion(localstackLikeChain, makeAclVersion(0, 1, 0))).toEqual({
      protocolVersion: { version: '0.11.0', comparator: 'lt' },
      pubKeyCrsVersion: { version: '1.5.1', comparator: 'lt' },
    });
    expect(protocolContextFromAclVersion(localstackLikeChain, makeAclVersion(0, 2, 0))).toEqual({
      protocolVersion: { version: '0.11.0', comparator: 'eq' },
      pubKeyCrsVersion: { version: '1.5.1', comparator: 'eq' },
    });
    expect(protocolContextFromAclVersion(localstackLikeChain, makeAclVersion(0, 3, 0))).toEqual({
      protocolVersion: { version: '0.12.0', comparator: 'eq' },
      pubKeyCrsVersion: { version: '1.5.4', comparator: 'eq' },
    });
    expect(protocolContextFromAclVersion(localstackLikeChain, makeAclVersion(0, 4, 0))).toEqual({
      protocolVersion: { version: '0.13.0', comparator: 'eq' },
      pubKeyCrsVersion: { version: '1.6.1', comparator: 'eq' },
    });
    expect(protocolContextFromAclVersion(localstackLikeChain, makeAclVersion(0, 6, 0))).toEqual({
      protocolVersion: { version: '0.14.0', comparator: 'gt' },
      pubKeyCrsVersion: { version: '1.6.1', comparator: 'gt' },
    });
  });

  it('keeps the current public-chain PubKey/CRS version override separate from protocol resolution', () => {
    expect(protocolContextFromAclVersion(sepolia, makeAclVersion(0, 4, 0))).toEqual({
      protocolVersion: { version: '0.13.0', comparator: 'eq' },
      pubKeyCrsVersion: { version: '1.4.0-alpha.3', comparator: 'eq' },
    });
  });

  it('does not infer cleartext PubKey/CRS context from a localhost relayer URL', () => {
    expect(
      pubKeyCrsVersionFromProtocolVersion(makeChain('http://localhost:8545'), {
        version: '0.13.0',
        comparator: 'eq',
      }),
    ).toEqual({
      version: '1.6.1',
      comparator: 'eq',
    });
  });

  it('keeps exact ACL protocol mappings covered by exact generated PubKey/CRS intervals', () => {
    const localstackLikeChain = makeChain('http://localhost:3000');

    for (const { acl, protocolVersion: expectedProtocolVersion, pubKeyCrsVersion } of KNOWN_EXACT_ACL_PROTOCOL_CASES) {
      const aclVersion = makeAclVersion(acl[0], acl[1], acl[2]);
      const protocolVersion = protocolVersionFromAclVersion(aclVersion);

      expect(protocolVersion).toEqual({
        version: expectedProtocolVersion,
        comparator: 'eq',
      });
      expect(pubKeyCrsVersionFromProtocolVersion(localstackLikeChain, protocolVersion)).toEqual({
        version: pubKeyCrsVersion,
        comparator: 'eq',
      });
      expect(protocolContextFromAclVersion(localstackLikeChain, aclVersion)).toEqual({
        protocolVersion: {
          version: expectedProtocolVersion,
          comparator: 'eq',
        },
        pubKeyCrsVersion: {
          version: pubKeyCrsVersion,
          comparator: 'eq',
        },
      });
    }
  });

  it('resolves protocol version from the ACL host contract', async () => {
    const readContract = vi.fn(() => Promise.resolve('ACL v0.4.0')) satisfies EthereumModule['readContract'];
    const client = makeClient(readContract);

    expect(getResolvedProtocolVersion(client)).toBeUndefined();

    await expect(resolveProtocolVersion(client)).resolves.toEqual({ version: '0.13.0', comparator: 'eq' });

    expect(getResolvedProtocolVersion(client)).toBeUndefined();
    expect(readContract).toHaveBeenCalledWith(expect.anything(), {
      address: ACL_ADDRESS,
      abi: expect.any(Array) as unknown,
      args: [],
      functionName: 'getVersion',
    });
  });

  it('resolves protocol context from the ACL host contract', async () => {
    const readContract = vi.fn(() => Promise.resolve('ACL v0.4.0')) satisfies EthereumModule['readContract'];
    const client = makeClient(readContract);

    await expect(resolveProtocolContext(client)).resolves.toEqual({
      protocolVersion: { version: '0.13.0', comparator: 'eq' },
      pubKeyCrsVersion: { version: '1.4.0-alpha.3', comparator: 'eq' },
    });
  });

  it('resolves protocol version during base client init', async () => {
    const readContract = vi.fn(() => Promise.resolve('ACL v0.4.0')) satisfies EthereumModule['readContract'];
    const client = makeClient(readContract).extend(baseActions);

    expect(getResolvedProtocolVersion(client)).toBeUndefined();

    await client.ready;

    expect(client.protocolVersion).toEqual({ version: '0.13.0', comparator: 'eq' });
    expect(getResolvedProtocolVersion(client)).toEqual({ version: '0.13.0', comparator: 'eq' });
    expect(readContract).toHaveBeenCalledTimes(1);
  });
});
