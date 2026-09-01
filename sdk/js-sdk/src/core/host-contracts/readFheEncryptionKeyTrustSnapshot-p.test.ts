import { describe, expect, it, vi } from 'vitest';
import { sepolia } from '../chains/definitions/sepolia.js';
import { getConfiguredFheEncryptionKeyTrust } from '../chains/configuredFheEncryptionKeyTrust-p.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { readFheEncryptionKeyTrustSnapshot } from './readFheEncryptionKeyTrustSnapshot-p.js';

const PUBLIC_KEY_DIGEST = `0x${'11'.repeat(32)}` as const;
const CRS_DIGEST = `0x${'22'.repeat(32)}` as const;

describe('readFheEncryptionKeyTrustSnapshot', () => {
  it('reads and confirms one fresh KMSGeneration snapshot', async () => {
    const readContract = contractReader();

    await expect(
      readFheEncryptionKeyTrustSnapshot(configuredTrust(), {
        getChainId: async () => BigInt(sepolia.id),
        readContract,
      }),
    ).resolves.toEqual({
      publicKeyId: 101n,
      crsId: 202n,
      publicKeyDigest: PUBLIC_KEY_DIGEST,
      crsDigest: CRS_DIGEST,
    });

    expect(readContract).toHaveBeenCalledTimes(6);
    for (const [parameters] of readContract.mock.calls) {
      expect(parameters.blockTag).toBe('finalized');
    }
  });

  it('fails closed if the active identity rotates during acquisition', async () => {
    let keyIdReads = 0;
    const readContract = contractReader((functionName) => {
      if (functionName === 'getActiveKeyId') {
        keyIdReads += 1;
        return keyIdReads === 1 ? 101n : 102n;
      }
      return undefined;
    });

    await expect(
      readFheEncryptionKeyTrustSnapshot(configuredTrust(), {
        getChainId: async () => BigInt(sepolia.id),
        readContract,
      }),
    ).rejects.toBeInstanceOf(FhevmConfigError);
  });

  it('checks the connected chain before reading KMSGeneration', async () => {
    const readContract = contractReader();

    await expect(
      readFheEncryptionKeyTrustSnapshot(configuredTrust(), {
        getChainId: async () => 80_002n,
        readContract,
      }),
    ).rejects.toBeInstanceOf(FhevmConfigError);
    expect(readContract).not.toHaveBeenCalled();
  });

  it('reports malformed chain-configured digests with host-chain context', async () => {
    const trust = configuredTrust();
    const readContract = contractReader((functionName) => {
      if (functionName === 'getKeyMaterials') {
        return { keyDigests: [{ keyType: 1, digest: '0x1234' }] };
      }
      return undefined;
    });

    const error: unknown = await readFheEncryptionKeyTrustSnapshot(trust, {
      getChainId: async () => BigInt(sepolia.id),
      readContract,
    }).then(
      () => undefined,
      (cause: unknown) => cause,
    );

    expect(error).toBeInstanceOf(FhevmConfigError);
    if (!(error instanceof FhevmConfigError)) {
      throw new Error('Expected FhevmConfigError.');
    }
    expect(error.message).toContain(`chain-configured trust material on chain ${trust.chainId.toString()}`);
    expect(error.message).toContain(trust.kmsGenerationAddress);
    expect(error.message).not.toContain('fheEncryptionKeyTrust');
    expect(error.cause).toBeInstanceOf(FhevmConfigError);
  });
});

function configuredTrust() {
  const trust = getConfiguredFheEncryptionKeyTrust(sepolia);
  if (trust === undefined) {
    throw new Error('Sepolia KMSGeneration config is missing.');
  }
  return trust;
}

function contractReader(override?: (functionName: string) => unknown) {
  return vi.fn(async (parameters: { readonly blockTag?: string | undefined; readonly functionName: string }) => {
    const overridden = override?.(parameters.functionName);
    if (overridden !== undefined) {
      return overridden;
    }
    switch (parameters.functionName) {
      case 'getActiveKeyId':
        return 101n;
      case 'getActiveCrsId':
        return 202n;
      case 'getKeyMaterials':
        return { keyDigests: [{ keyType: 1, digest: PUBLIC_KEY_DIGEST }] };
      case 'getCrsMaterials':
        return { crsDigest: CRS_DIGEST };
      default:
        throw new Error(`Unexpected read: ${parameters.functionName}`);
    }
  });
}
