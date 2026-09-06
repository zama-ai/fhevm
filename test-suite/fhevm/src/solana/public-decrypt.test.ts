import { describe, expect, test } from 'bun:test';
import { certificateCleartext, runSolanaPublicDecrypt, type PublicDecryptDependencies } from './public-decrypt';

const hex32 = (byte: string) => `0x${byte.repeat(64)}`;
const environment = (): Record<string, string> => ({
  PD_CONTRACTS_CHAIN_ID: '9223372036854788153',
  PD_RELAYER_URL: 'http://127.0.0.1:3000',
  PD_HANDLE: hex32('1'),
  PD_CONTEXT_ID: hex32('2'),
  PD_ENCRYPTED_VALUE_ACCOUNT: hex32('4'),
});
const certificate = {
  handle: hex32('1'),
  abiEncodedCleartext: '000000000000002a',
  signatures: ['ab'.repeat(65)],
  extraData: '0x03',
};

describe('solana-public-decrypt', () => {
  test('names the handle and its account to the SDK, and nothing proof-shaped', async () => {
    let received: unknown;
    const dependencies: PublicDecryptDependencies = {
      publicDecryptCertificate: async (input) => {
        received = input;
        return certificate;
      },
    };
    await runSolanaPublicDecrypt(environment(), dependencies);
    expect(received).toEqual({
      chainId: 9223372036854788153n,
      relayerUrl: 'http://127.0.0.1:3000',
      apiKey: 'local',
      request: {
        handle: hex32('1'),
        contextId: Uint8Array.from(Buffer.from('2'.repeat(64), 'hex')),
        encryptedValueAccount: Uint8Array.from(Buffer.from('4'.repeat(64), 'hex')),
      },
    });
  });

  test('requires every explicit public-decrypt input', async () => {
    for (const name of Object.keys(environment())) {
      const input: Record<string, string | undefined> = environment();
      delete input[name];
      await expect(
        runSolanaPublicDecrypt(input, { publicDecryptCertificate: async () => certificate }),
      ).rejects.toThrow(`missing env ${name}`);
    }
  });

  test('rejects an account that is not 32 bytes', async () => {
    await expect(
      runSolanaPublicDecrypt(
        { ...environment(), PD_ENCRYPTED_VALUE_ACCOUNT: '0xabcd' },
        { publicDecryptCertificate: async () => certificate },
      ),
    ).rejects.toThrow('PD_ENCRYPTED_VALUE_ACCOUNT must be a 0x-prefixed 32-byte hex value');
  });

  test('interprets the certificate cleartext as unprefixed big-endian hex, never decimal', () => {
    // The relayer's ABI cleartext carries no 0x prefix; "46" is 70, not 46.
    expect(certificateCleartext({ abiEncodedCleartext: `${'0'.repeat(62)}46` })).toBe(70n);
    expect(certificateCleartext({ abiEncodedCleartext: `${'0'.repeat(62)}2a` })).toBe(42n);
    expect(certificateCleartext({ abiEncodedCleartext: `0x${'0'.repeat(62)}2a` })).toBe(42n);
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
