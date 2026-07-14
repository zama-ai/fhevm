import { describe, expect, test } from 'bun:test';
import { runSolanaPublicDecrypt, type PublicDecryptDependencies } from './public-decrypt';

const hex32 = (byte: string) => `0x${byte.repeat(64)}`;
const environment = (): Record<string, string> => ({
  PD_CONTRACTS_CHAIN_ID: '9223372036854788153',
  PD_RELAYER_URL: 'http://127.0.0.1:3000',
  PD_HANDLE: hex32('1'),
  PD_CONTEXT_ID: hex32('2'),
  PD_ACL_VALUE_KEY: hex32('3'),
  PD_MMR_PROOF_SLOT: '1',
  PD_MMR_ENCRYPTED_VALUE_ACCOUNT: hex32('4'),
  PD_MMR_PEAKS: `${hex32('5')},${hex32('6')}`,
  PD_MMR_LEAF_COUNT: '1',
  PD_MMR_PROOF_BYTES: '0x02000000000000000000000000',
});
const claim = {
  handle: hex32('1'),
  abiEncodedCleartext: '000000000000002a',
  signatures: ['ab'.repeat(65)],
  extraData: '0x03',
};

describe('solana-public-decrypt', () => {
  test('passes explicit witness inputs to the SDK without a separately decoded proof', async () => {
    let received: unknown;
    const dependencies: PublicDecryptDependencies = {
      publicDecryptCertificate: async (input) => {
        received = input;
        return claim;
      },
    };
    await runSolanaPublicDecrypt(environment(), dependencies);
    expect(received).toMatchObject({
      chainId: 9223372036854788153n,
      relayerUrl: 'http://127.0.0.1:3000',
      request: {
        handle: hex32('1'),
        proofSlot: 1n,
        leafCount: 1n,
        peaks: [
          Uint8Array.from(Buffer.from('5'.repeat(64), 'hex')),
          Uint8Array.from(Buffer.from('6'.repeat(64), 'hex')),
        ],
      },
    });
    expect((received as { request: object }).request).not.toHaveProperty('proof');
  });

  test('requires every explicit public-decrypt input', async () => {
    for (const name of Object.keys(environment())) {
      const input: Record<string, string | undefined> = environment();
      delete input[name];
      await expect(
        runSolanaPublicDecrypt(input, { publicDecryptCertificate: async () => claim }),
      ).rejects.toThrow(`missing env ${name}`);
    }
  });

  test('preserves SDK terminal errors', async () => {
    const terminal = Object.assign(new Error('public-decrypt failed'), { status: 'failed' });
    let thrown: unknown;
    try {
      await runSolanaPublicDecrypt(environment(), {
        publicDecryptCertificate: async () => Promise.reject(terminal),
      });
    } catch (error) {
      thrown = error;
    }
    expect(thrown).toBe(terminal);
  });
});
