import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { polygonAmoy } from '../chains/definitions/polygonAmoy.js';
import { getCurrentKmsContextAndEpoch } from '../host-contracts/getCurrentKmsContextAndEpoch-p.js';
import { invalidateVersionCache } from '../host-contracts/HostContractVersion-p.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { resolveFhevmClientFrozenContext } from './resolveFhevmClientFrozenContext-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/frozenContext/resolveFhevmClientFrozenContext-p.test.ts
////////////////////////////////////////////////////////////////////////////////

const ACL_ADDRESS = polygonAmoy.fhevm.contracts.acl.address as ChecksummedAddress;
const INPUT_VERIFIER_ADDRESS = polygonAmoy.fhevm.contracts.inputVerifier.address as ChecksummedAddress;
const KMS_VERIFIER_ADDRESS = polygonAmoy.fhevm.contracts.kmsVerifier.address as ChecksummedAddress;
const PROTOCOL_CONFIG_ADDRESS = polygonAmoy.fhevm.contracts.protocolConfig?.address as ChecksummedAddress;

// getVersion() answers per address. getCurrentKmsContextAndEpoch() answers (7, 3).
function makeClient(protocolConfigVersion: string) {
  const versionsByAddress = new Map<string, string>([
    [ACL_ADDRESS, 'ACL v0.3.0'],
    [INPUT_VERIFIER_ADDRESS, 'InputVerifier v0.3.0'],
    [KMS_VERIFIER_ADDRESS, 'KMSVerifier v0.4.0'],
    [PROTOCOL_CONFIG_ADDRESS, protocolConfigVersion],
  ]);
  const readContract = vi.fn(
    async (_trustedClient: unknown, parameters: { readonly address: string; readonly functionName: string }) => {
      if (parameters.functionName === 'getCurrentKmsContextAndEpoch') {
        return [7n, 3n];
      }
      const version = versionsByAddress.get(parameters.address);
      if (version === undefined) {
        throw new Error(`No mocked version for ${parameters.address}`);
      }
      return version;
    },
  );

  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum: { readContract } as unknown as EthereumModule,
    relayer: {} as RelayerModule,
    config: {},
  });

  return createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
    chain: polygonAmoy,
    client: {},
    runtime,
  });
}

async function readContextAndEpoch(protocolConfigVersion: string) {
  const client = makeClient(protocolConfigVersion);
  await resolveFhevmClientFrozenContext(client);
  return getCurrentKmsContextAndEpoch(client, { protocolConfigAddress: PROTOCOL_CONFIG_ADDRESS });
}

beforeEach(() => {
  invalidateVersionCache({ includeInflight: true });
});

describe('ProtocolConfigReplica at the ProtocolConfig address', () => {
  it('accepts a replica and reads its context and epoch', async () => {
    await expect(readContextAndEpoch('ProtocolConfigReplica v0.4.0')).resolves.toEqual({ contextId: 7n, epochId: 3n });
  });

  it('rejects another host contract', async () => {
    await expect(resolveFhevmClientFrozenContext(makeClient('ACL v0.3.0'))).rejects.toThrow(
      "Invalid contract name. Expecting 'ProtocolConfig' or 'ProtocolConfigReplica', got ACL.",
    );
  });
});
