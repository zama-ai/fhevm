import * as authorization from './authorization.js';
import * as storeValues from './storeValues.js';
import * as revokePermits from '../actions/revokePermits.js';
import * as zamaHost from '@fhevm/solana-zama-host';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { createSolanaRpc } from '@solana/kit';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import * as hostKms from '../actions/hostKms.js';
import { asBytes32, asBytes32Hex, hexToBytes } from '../../core/base/bytes.js';
import type { Handle } from '../../core/types/encryptedTypes-p.js';
import { bytes32ToHandle, FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION } from '../../core/handle/FhevmHandle.js';
import { setFhevmRuntimeConfig } from '../internal/config.js';
import { PERMIT_TRANSPORT_KEY_LEN, solanaPermitWalletFromSecretKey } from '../permit/index.js';
import { createFhevmCleartextDecryptClient } from './createFhevmCleartextDecryptClient.js';

/**
 * Test helper, as main's handle tests have it: a computed handle
 * hash21 (bytes 0..20) | 0xFF (byte 21) | chainId (bytes 22..29) | fheTypeId (30) | version (31).
 */
function buildHandle({
  chainId,
  hash21,
  fheTypeId,
}: {
  chainId: number | bigint;
  hash21: string;
  fheTypeId: number;
}): Handle {
  const bytes = new Uint8Array(32);
  bytes.set(hexToBytes(hash21), 0);
  bytes[21] = 0xff;
  new DataView(bytes.buffer).setBigUint64(22, BigInt(chainId));
  bytes[30] = fheTypeId;
  bytes[31] = FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION;
  return bytes32ToHandle(asBytes32(bytes));
}

// EVM's cleartext path decrypts with no TKMS WASM; this client must not load it either.
vi.mock('../../core/modules/decrypt/module/init-p.js', async (importOriginal) => ({
  ...(await importOriginal<typeof import('../../core/modules/decrypt/module/init-p.js')>()),
  initTkmsModule: () => {
    throw new Error('the cleartext client loaded the KMS WASM');
  },
}));

const chain = {
  id: 72057594037940281n,
  fhevm: {
    relayerUrl: 'https://relayer.example.test',
    decryptionRelayerUrl: 'https://decryption.relayer.example.test',
    programs: { host: { address: asBytes32Hex(`0x${'22'.repeat(32)}`) } },
  },
} as const satisfies FhevmSolanaChain;
const decryptionContract = new Uint8Array(20).fill(0xdc);
const kmsSigner = new Uint8Array(20).fill(0x5e);

afterEach(() => vi.restoreAllMocks());

describe('createFhevmCleartextDecryptClient', () => {
  it('signs a permit and answers a user decryption without the KMS WASM', async () => {
    setFhevmRuntimeConfig({});
    vi.spyOn(revokePermits, 'fetchSolanaPermitInvalidation').mockResolvedValue(0n);
    vi.spyOn(hostKms, 'readActiveKmsRouting').mockResolvedValue({
      contextId: new Uint8Array(32).fill(0x33),
      epochId: new Uint8Array(32).fill(0x44),
    });
    vi.spyOn(zamaHost, 'fetchHostConfig').mockResolvedValue({
      data: { chainId: chain.id, gatewayChainId: 7n, decryptionContract },
    } as unknown as Awaited<ReturnType<typeof zamaHost.fetchHostConfig>>);
    vi.spyOn(zamaHost, 'fetchKmsContext').mockResolvedValue({
      data: { destroyed: false, signers: [kmsSigner] },
    } as unknown as Awaited<ReturnType<typeof zamaHost.fetchKmsContext>>);
    vi.spyOn(authorization, 'judgeSolanaUserDecryption').mockResolvedValue({ authorized: true });
    vi.spyOn(storeValues, 'fetchCleartextStoreValue').mockResolvedValue(Uint8Array.of(42));
    const client = createFhevmCleartextDecryptClient({
      chain,
      rpc: createSolanaRpc('http://127.0.0.1:1'),
      readLeaves: () => Promise.reject(new Error('the leaf record is not read here')),
    });

    const session = await client.signPermit({
      wallet: solanaPermitWalletFromSecretKey(new Uint8Array(32).fill(7)),
      durationSeconds: 3_600n,
    });
    const value = await client.decryptValue({
      session,
      entry: {
        handle: buildHandle({ chainId: chain.id, hash21: `0x${'a1'.repeat(21)}`, fheTypeId: 2 }).bytes32,
        encryptedStore: new Uint8Array(32).fill(0xea),
      },
    });

    expect(session.signedPermit.fields.transportKey).toHaveLength(PERMIT_TRANSPORT_KEY_LEN);
    expect(value).toMatchObject({ value: 42 });
  });
});
