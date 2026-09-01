import type { FheEncryptionKeyBytes, FheEncryptionKeyDigests } from '@fhevm/sdk/types';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { computeFheEncryptionKeyDigests } from '../../src/core/key/authenticateFheEncryptionKeyBytes.js';
import {
  createFheTestClientOptions,
  createFheTestEncryptClientOptions,
  createLocalstackFheEncryptionKeyTrust,
  resolveFheEncryptionKeyTrust,
} from './fheEncryptionKeyTrust.js';

const LOCALSTACK_VAULT_URL = 'http://localhost:9000/kms-public';
const LOCALSTACK_PUBLIC_KEY_ID = '0400000000000000000000000000000000000000000000000000000000000001';
const LOCALSTACK_CRS_ID = '0500000000000000000000000000000000000000000000000000000000000001';
const LATEST_PUBLIC_KEY_URL = `${LOCALSTACK_VAULT_URL}/PUB/PUB/PublicKey/${LOCALSTACK_PUBLIC_KEY_ID}`;
const LATEST_CRS_URL = `${LOCALSTACK_VAULT_URL}/PUB/PUB/CRS/${LOCALSTACK_CRS_ID}`;
const LEGACY_PUBLIC_KEY_URL = `${LOCALSTACK_VAULT_URL}/PUB/PublicKey/${LOCALSTACK_PUBLIC_KEY_ID}`;
const LEGACY_CRS_URL = `${LOCALSTACK_VAULT_URL}/PUB/CRS/${LOCALSTACK_CRS_ID}`;
const METADATA = { chainId: 12_345, relayerUrl: 'http://relayer.example' } as const;
const PUBLIC_KEY_BYTES = new Uint8Array([1, 2, 3, 4]);
const CRS_BYTES = new Uint8Array([5, 6, 7, 8]);

describe('localstack FHE encryption-key trust', () => {
  beforeEach(() => {
    vi.stubEnv('FHEVM_PUBLIC_KEY_DIGEST', undefined);
    vi.stubEnv('FHEVM_CRS_DIGEST', undefined);
  });

  afterEach(() => {
    vi.unstubAllEnvs();
  });

  it.each(['localstack', 'localstack_v11', 'localstack_v12', 'localstack_v13', 'localstack_v14'] as const)(
    'configures KMS-vault trust for %s',
    (chainName) => {
      expect(resolveFheEncryptionKeyTrust(chainName, {}, {})).toBeTypeOf('function');
    },
  );

  it.each([
    ['latest', 'PUB/PUB', LATEST_PUBLIC_KEY_URL, LATEST_CRS_URL, 2],
    ['legacy', 'PUB', LEGACY_PUBLIC_KEY_URL, LEGACY_CRS_URL, 4],
  ] as const)(
    'computes and memoizes %s trust from the independent KMS public vault',
    async (_, prefix, publicKeyUrl, crsUrl, calls) => {
      const fetcher = vaultFetcher(prefix);
      const trust = createLocalstackFheEncryptionKeyTrust(fetcher as unknown as typeof globalThis.fetch);

      const [first, second] = await Promise.all([trust(METADATA), trust(METADATA)]);

      expect(first).toEqual(computeDigests(PUBLIC_KEY_BYTES, CRS_BYTES));
      expect(second).toBe(first);
      expect(fetcher).toHaveBeenCalledTimes(calls);
      expect(fetcher).toHaveBeenCalledWith(publicKeyUrl);
      expect(fetcher).toHaveBeenCalledWith(crsUrl);
    },
  );

  it('fails closed on invalid vault responses and retries after a failed attempt', async () => {
    const fetcher = vaultFetcher('PUB/PUB');
    fetcher.mockResolvedValueOnce(new Response('', { status: 503 }));
    const trust = createLocalstackFheEncryptionKeyTrust(fetcher as unknown as typeof globalThis.fetch);

    await expect(trust(METADATA)).rejects.toThrow('HTTP 503');
    await expect(trust(METADATA)).resolves.toEqual(computeDigests(PUBLIC_KEY_BYTES, CRS_BYTES));
    expect(fetcher).toHaveBeenCalledTimes(6);
  });

  it('rejects empty vault material and unexpected chains', async () => {
    const emptyFetcher = vi.fn(async () => new Response());
    const emptyTrust = createLocalstackFheEncryptionKeyTrust(emptyFetcher as unknown as typeof globalThis.fetch);

    await expect(emptyTrust(METADATA)).rejects.toThrow('is empty');

    const fetcher = vaultFetcher('PUB/PUB');
    const trust = createLocalstackFheEncryptionKeyTrust(fetcher as unknown as typeof globalThis.fetch);
    await expect(trust({ ...METADATA, chainId: 1 })).rejects.toThrow('expected chain 12345');
    expect(fetcher).not.toHaveBeenCalled();
  });

  it('keeps explicit digest pins above localstack discovery and cleartext unconfigured', () => {
    const pins = {
      FHEVM_PUBLIC_KEY_DIGEST: `0x${'11'.repeat(32)}`,
      FHEVM_CRS_DIGEST: `0x${'22'.repeat(32)}`,
    };

    expect(resolveFheEncryptionKeyTrust('localstack_v11', pins, {})).toEqual({
      publicKeyDigest: pins.FHEVM_PUBLIC_KEY_DIGEST,
      crsDigest: pins.FHEVM_CRS_DIGEST,
    });
    expect(resolveFheEncryptionKeyTrust('localcleartext', pins, {})).toBeUndefined();
  });

  it.each(['mainnet', 'sepolia', 'testnet'] as const)(
    'does not pass application digest pins to the chain-configured KMSGeneration %s preset',
    (chainName) => {
      const pins = {
        FHEVM_PUBLIC_KEY_DIGEST: `0x${'11'.repeat(32)}`,
        FHEVM_CRS_DIGEST: `0x${'22'.repeat(32)}`,
      };

      expect(resolveFheEncryptionKeyTrust(chainName, pins, {})).toBeUndefined();
    },
  );

  it('allows test chains without KMSGeneration config to omit explicit digest pins', () => {
    expect(resolveFheEncryptionKeyTrust('devnet', {}, {})).toBeUndefined();
    expect(createFheTestEncryptClientOptions({ chainName: 'devnet', moduleVersions: 'auto' })).toEqual({
      moduleVersions: 'auto',
    });
  });

  it('omits real-key fields from shared cleartext client options', () => {
    const full = createFheTestClientOptions({ chainName: 'localcleartext', moduleVersions: 'auto' });
    const encrypt = createFheTestEncryptClientOptions({ chainName: 'localcleartext', moduleVersions: 'auto' });

    expect(full).toEqual({ moduleVersions: 'auto' });
    expect(encrypt).toEqual({ moduleVersions: 'auto' });
    expect(full).not.toHaveProperty('fheEncryptionKeyTrust');
    expect(encrypt).not.toHaveProperty('fheEncryptionKeyTrust');
  });
});

function vaultFetcher(prefix: 'PUB/PUB' | 'PUB') {
  return vi.fn(async (input: RequestInfo | URL) => {
    const url = input.toString();
    if (url === `${LOCALSTACK_VAULT_URL}/${prefix}/PublicKey/${LOCALSTACK_PUBLIC_KEY_ID}`) {
      return new Response(PUBLIC_KEY_BYTES);
    }
    if (url === `${LOCALSTACK_VAULT_URL}/${prefix}/CRS/${LOCALSTACK_CRS_ID}`) {
      return new Response(CRS_BYTES);
    }
    return new Response('', { status: 404 });
  });
}

function computeDigests(publicKeyBytes: Uint8Array, crsBytes: Uint8Array): FheEncryptionKeyDigests {
  const keyBytes: FheEncryptionKeyBytes = {
    publicKeyBytes: { id: 'expected-public-key', bytes: publicKeyBytes },
    crsBytes: { id: 'expected-crs', capacity: 2048, bytes: crsBytes },
    metadata: METADATA,
  };
  return computeFheEncryptionKeyDigests(keyBytes);
}
