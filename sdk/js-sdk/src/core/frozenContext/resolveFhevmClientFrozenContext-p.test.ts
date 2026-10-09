import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { polygonAmoy } from '../chains/definitions/polygonAmoy.js';
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

function makeClient(protocolConfigVersion: string) {
  const versionsByAddress = new Map<string, string>([
    [ACL_ADDRESS, 'ACL v0.3.0'],
    [INPUT_VERIFIER_ADDRESS, 'InputVerifier v0.3.0'],
    [KMS_VERIFIER_ADDRESS, 'KMSVerifier v0.4.0'],
    [PROTOCOL_CONFIG_ADDRESS, protocolConfigVersion],
  ]);
  const readContract = vi.fn(async (_trustedClient: unknown, parameters: { readonly address: string }) => {
    const version = versionsByAddress.get(parameters.address);
    if (version === undefined) {
      throw new Error(`No mocked version for ${parameters.address}`);
    }
    return version;
  });

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

beforeEach(() => {
  invalidateVersionCache({ includeInflight: true });
});

describe('resolveFhevmClientFrozenContext', () => {
  it('stores a canonical ProtocolConfig version under ProtocolConfig', async () => {
    const fhevmContext = await resolveFhevmClientFrozenContext(makeClient('ProtocolConfig v0.2.0'));

    expect(fhevmContext.hostContractVersion('ProtocolConfig').version).toBe('ProtocolConfig v0.2.0');
    expect(fhevmContext.hasHostContractVersion('ProtocolConfigReplica')).toBe(false);
  });

  it('accepts a ProtocolConfigReplica at the ProtocolConfig address', async () => {
    const fhevmContext = await resolveFhevmClientFrozenContext(makeClient('ProtocolConfigReplica v0.1.0'));

    expect(fhevmContext.hostContractVersion('ProtocolConfigReplica')).toMatchObject({
      version: 'ProtocolConfigReplica v0.1.0',
      contractName: 'ProtocolConfigReplica',
    });
    expect(fhevmContext.hasHostContractVersion('ProtocolConfig')).toBe(false);
  });

  it('rejects another host contract at the ProtocolConfig address', async () => {
    await expect(resolveFhevmClientFrozenContext(makeClient('ACL v0.3.0'))).rejects.toThrow(
      "Invalid contract name. Expecting 'ProtocolConfig', got ACL.",
    );
  });
});
