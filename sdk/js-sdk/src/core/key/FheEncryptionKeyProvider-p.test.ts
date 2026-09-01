import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import { describe, expect, it, vi } from 'vitest';
import { fetchFheEncryptionKeyBytes } from '../actions/chain/fetchFheEncryptionKeyBytes.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { createCoreFhevm, setFrozenContext } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { createFhevmCleartextClient as createEthersCleartextClient } from '../../ethers/cleartext/createFhevmCleartextClient.js';
import { createFhevmCleartextClient as createViemCleartextClient } from '../../viem/cleartext/createFhevmCleartextClient.js';
import { computeFheEncryptionKeyDigests } from './authenticateFheEncryptionKeyBytes.js';
import { assertCleartextFheEncryptionKeyOptions } from './FheEncryptionKeyProvider-p.js';

describe('Core FHE encryption-key provider integration', () => {
  it('fails KMSGeneration mode closed without a native host client', async () => {
    const fixture = coreFixture(false);

    await expect(fetchFheEncryptionKeyBytes(fixture.client as never)).rejects.toBeInstanceOf(FhevmConfigError);
    expect(fixture.fetchKeyBytes).not.toHaveBeenCalled();
    expect(fixture.readContract).not.toHaveBeenCalled();
  });

  it('uses the real Core trusted-client path before admitting relayer bytes', async () => {
    const fixture = coreFixture(true);

    await expect(fetchFheEncryptionKeyBytes(fixture.client as never)).resolves.toEqual(fixture.key);
    expect(fixture.getChainId).toHaveBeenCalledTimes(6);
    expect(fixture.readContract).toHaveBeenCalledTimes(10);
    expect(fixture.fetchKeyBytes).toHaveBeenCalledOnce();
  });

  it('rejects real-key fields at the cleartext runtime boundary', () => {
    expect(() => assertCleartextFheEncryptionKeyOptions({ fheEncryptionKeyTrust: undefined })).toThrow(
      FhevmConfigError,
    );
    expect(() => assertCleartextFheEncryptionKeyOptions({ fheEncryptionKey: undefined })).toThrow(FhevmConfigError);
    expect(() => assertCleartextFheEncryptionKeyOptions({ moduleVersions: 'auto' })).not.toThrow();
  });

  it.each([
    [
      'ethers',
      (options: unknown) =>
        createEthersCleartextClient({ chain: sepolia, provider: {} as never, options: options as never }),
    ],
    [
      'viem',
      (options: unknown) =>
        createViemCleartextClient({ chain: sepolia, publicClient: {} as never, options: options as never }),
    ],
  ] as const)('rejects forbidden fields on function objects through the %s cleartext factory', (_, createClient) => {
    for (const field of ['fheEncryptionKeyTrust', 'fheEncryptionKey'] as const) {
      const options = Object.assign(() => undefined, { [field]: undefined });
      expect(() => createClient(options)).toThrow(FhevmConfigError);
    }
  });
});

function coreFixture(withClient: boolean) {
  const ownerToken = Symbol('provider-integration');
  const key = keyBytes();
  const digests = computeFheEncryptionKeyDigests(key);
  const fetchKeyBytes = vi.fn(async () => key);
  const getChainId = vi.fn(async () => BigInt(sepolia.id));
  const readContract = vi.fn(async (parameters: { readonly functionName: string }) => {
    switch (parameters.functionName) {
      case 'getActiveKeyId':
        return 101n;
      case 'getActiveCrsId':
        return 202n;
      case 'getKeyMaterials':
        return [[], [{ keyType: 1, digest: digests.publicKeyDigest }]];
      case 'getCrsMaterials':
        return [[], digests.crsDigest];
      default:
        throw new Error(`Unexpected read: ${parameters.functionName}`);
    }
  });
  const runtime = createFhevmRuntime(ownerToken, {
    config: {},
    ethereum: {
      getChainId: (_client: unknown) => getChainId(),
      readContract: (_client: unknown, parameters: { readonly functionName: string }) => readContract(parameters),
    } as never,
    relayer: { fetchFheEncryptionKeyBytes: fetchKeyBytes } as never,
  });
  const client = createCoreFhevm(ownerToken, {
    chain: sepolia,
    runtime,
    ...(withClient ? { client: { transport: 'trusted' } } : {}),
  });
  setFrozenContext(client, createFhevmClientFrozenContext({}));
  return { client, fetchKeyBytes, getChainId, key, readContract };
}

function keyBytes(): FheEncryptionKeyBytes {
  return {
    publicKeyBytes: { id: '101', bytes: new Uint8Array([1, 2, 3, 4]) },
    crsBytes: { id: '202', capacity: 2048, bytes: new Uint8Array([5, 6, 7, 8]) },
    metadata: { chainId: sepolia.id, relayerUrl: sepolia.fhevm.relayerUrl },
  };
}
