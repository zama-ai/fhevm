import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { HostContractVersion, HostContractVersionString } from '../types/hostContract.js';
import type { ChecksummedAddress, UintNumber } from '../types/primitives.js';
import { describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { polygonAmoy } from '../chains/definitions/polygonAmoy.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { getCurrentKmsContextAndEpoch } from './getCurrentKmsContextAndEpoch-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/host-contracts/getCurrentKmsContextAndEpoch-p.test.ts
////////////////////////////////////////////////////////////////////////////////

const PROTOCOL_CONFIG_ADDRESS = polygonAmoy.fhevm.contracts.protocolConfig?.address as ChecksummedAddress;

function hostContractVersion<name extends 'ProtocolConfig' | 'ProtocolConfigReplica'>(
  contractName: name,
  minor: number,
): HostContractVersion<name> {
  return {
    version: `${contractName} v0.${minor}.0` as HostContractVersionString,
    contractName,
    major: 0 as UintNumber,
    minor: minor as UintNumber,
    patch: 0 as UintNumber,
  };
}

// A fresh runtime per test gives a fresh cache key (runtime uid + address).
function makeClient() {
  const readContract = vi.fn(async () => [7n, 3n]);

  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum: { readContract } as unknown as EthereumModule,
    relayer: {} as RelayerModule,
    config: {},
  });

  const client = createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
    chain: polygonAmoy,
    client: {},
    runtime,
  });

  return { client, readContract };
}

describe('getCurrentKmsContextAndEpoch', () => {
  it('reads the context and epoch from a ProtocolConfigReplica v0.1.0', async () => {
    const { client, readContract } = makeClient();
    const fhevmContext = createFhevmClientFrozenContext({
      hostContractVersions: { ProtocolConfigReplica: hostContractVersion('ProtocolConfigReplica', 1) },
    });

    await expect(
      getCurrentKmsContextAndEpoch(client, { protocolConfigAddress: PROTOCOL_CONFIG_ADDRESS, fhevmContext }),
    ).resolves.toEqual({ contextId: 7n, epochId: 3n });
    expect(readContract).toHaveBeenCalledTimes(1);
  });

  it('reads the context and epoch from a ProtocolConfig v0.2.0', async () => {
    const { client } = makeClient();
    const fhevmContext = createFhevmClientFrozenContext({
      hostContractVersions: { ProtocolConfig: hostContractVersion('ProtocolConfig', 2) },
    });

    await expect(
      getCurrentKmsContextAndEpoch(client, { protocolConfigAddress: PROTOCOL_CONFIG_ADDRESS, fhevmContext }),
    ).resolves.toEqual({ contextId: 7n, epochId: 3n });
  });

  it('rejects a ProtocolConfig before v0.2.0', async () => {
    const { client, readContract } = makeClient();
    const fhevmContext = createFhevmClientFrozenContext({
      hostContractVersions: { ProtocolConfig: hostContractVersion('ProtocolConfig', 1) },
    });

    await expect(
      getCurrentKmsContextAndEpoch(client, { protocolConfigAddress: PROTOCOL_CONFIG_ADDRESS, fhevmContext }),
    ).rejects.toThrow('requires ProtocolConfig >= v0.2.0');
    expect(readContract).not.toHaveBeenCalled();
  });
});
