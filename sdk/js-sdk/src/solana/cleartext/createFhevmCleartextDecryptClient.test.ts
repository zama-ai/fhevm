import * as authorization from './authorization.js';
import * as storeValues from './storeValues.js';
import * as revokePermits from '../actions/revokePermits.js';
import * as zamaHost from '@fhevm/solana-zama-host';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { createSolanaRpc } from '@solana/kit';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaDecryptTrust } from '../clients/decorators/permitDecrypt.js';
import { asBytes32Hex, bytesToHex } from '../../core/base/bytes.js';
import { buildHandle } from '../../core/handle/FhevmHandle.js';
import { setFhevmRuntimeConfig } from '../internal/config.js';
import { PERMIT_TRANSPORT_KEY_LEN, solanaPermitWalletFromSecretKey } from '../permit/index.js';
import { createFhevmCleartextDecryptClient } from './createFhevmCleartextDecryptClient.js';

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
    programs: { host: { address: asBytes32Hex(`0x${'22'.repeat(32)}`) } },
  },
} as const satisfies FhevmSolanaChain;
const decryptionContract = new Uint8Array(20).fill(0xdc);
const kmsSigner = new Uint8Array(20).fill(0x5e);
const trust: SolanaDecryptTrust = {
  kmsSigners: [{ partyId: 1, address: bytesToHex(kmsSigner) }],
  kmsContextId: asBytes32Hex(`0x${'00'.repeat(32)}`),
  kmsEpochId: asBytes32Hex(`0x${'00'.repeat(32)}`),
  fheParameter: 'test',
  gatewayEip712Domain: {
    name: 'Decryption',
    version: '1',
    chainId: 7n,
    verifyingContract: bytesToHex(decryptionContract),
  },
};

afterEach(() => vi.restoreAllMocks());

describe('createFhevmCleartextDecryptClient', () => {
  it('signs a permit and answers a user decryption without the KMS WASM', async () => {
    setFhevmRuntimeConfig({});
    vi.spyOn(revokePermits, 'fetchSolanaPermitInvalidation').mockResolvedValue(0n);
    vi.spyOn(zamaHost, 'fetchHostConfig').mockResolvedValue({
      data: { chainId: chain.id, gatewayChainId: 7n, decryptionContract },
    } as unknown as Awaited<ReturnType<typeof zamaHost.fetchHostConfig>>);
    vi.spyOn(zamaHost, 'fetchKmsContext').mockResolvedValue({
      data: { destroyed: false, signers: [kmsSigner] },
    } as unknown as Awaited<ReturnType<typeof zamaHost.fetchKmsContext>>);
    vi.spyOn(authorization, 'solanaRelayerDelegationRefusal').mockResolvedValue(undefined);
    vi.spyOn(authorization, 'judgeSolanaUserDecryption').mockResolvedValue({ authorized: true });
    vi.spyOn(storeValues, 'fetchCleartextStoreValue').mockResolvedValue(Uint8Array.of(42));
    const client = createFhevmCleartextDecryptClient({
      chain,
      rpc: createSolanaRpc('http://127.0.0.1:1'),
      trust,
      readMerkleProofs: () => Promise.reject(new Error('the leaf record is not read here')),
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
