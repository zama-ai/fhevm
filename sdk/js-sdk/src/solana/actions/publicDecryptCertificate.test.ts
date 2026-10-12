import { afterEach, describe, expect, it, vi } from 'vitest';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { SolanaDecryptionRunError } from '../decryption/retry.js';
import { bytesToHex } from '../../core/base/bytes.js';
import {
  publicDecryptCertificate,
  solanaPublicDecryptContextId,
  solanaPublicDecryptExtraData,
  type SolanaPublicDecryptBatch,
} from './publicDecryptCertificate.js';
import { asBytes32Hex } from '../../core/base/bytes.js';

const handle = new Uint8Array(32);
handle[22] = 0x01;
const account = new Uint8Array(32).fill(4);
const contextId = new Uint8Array(32).fill(5);
const epochId = new Uint8Array(32).fill(6);

const parameters = (): SolanaPublicDecryptBatch => ({
  entries: [{ handle, encryptedStore: account }],
  contextId,
  epochId,
  options: { fetchRetries: 1 },
});

const context = {
  chain: {
    id: 0x0100000000000000n,
    fhevm: {
      relayerUrl: 'https://relayer.example.com',
      decryptionRelayerUrl: 'https://decryption.relayer.example.com',
      programs: { host: { address: asBytes32Hex(`0x${'22'.repeat(32)}`) } },
    },
  },
  runtime: { config: { auth: { type: 'ApiKeyHeader', value: 'test' } } } as FhevmRuntime,
};

const requestExtraData = () => solanaPublicDecryptExtraData(contextId, epochId);

describe('solanaPublicDecryptExtraData', () => {
  it('is the v2 KMS routing of the context and epoch, with no store in it', () => {
    expect(solanaPublicDecryptExtraData(contextId, epochId)).toBe(`0x02${'05'.repeat(32)}${'06'.repeat(32)}`);
  });

  it('refuses a field of the wrong width before anything is sent', async () => {
    expect(() => solanaPublicDecryptExtraData(new Uint8Array(31), epochId)).toThrow('contextId must be 32 bytes');
    expect(() => solanaPublicDecryptExtraData(contextId, new Uint8Array(33))).toThrow('epochId must be 32 bytes');
    await expect(
      publicDecryptCertificate(context, { ...parameters(), entries: [{ handle, encryptedStore: new Uint8Array(33) }] }),
    ).rejects.toThrow('encryptedStore must be 32 bytes');
  });
});

describe('solanaPublicDecryptContextId', () => {
  it('reads the context id back from the v2 routing', () => {
    expect(solanaPublicDecryptContextId({ extraData: requestExtraData() })).toEqual(contextId);
  });

  it.each([
    ['version 0', '0x00'],
    ['version 1', `0x01${'05'.repeat(32)}`],
  ])('rejects %s routing', (_name, extraData) => {
    expect(() => solanaPublicDecryptContextId({ extraData })).toThrow('must be the v2 KMS routing');
  });
});

const signature = 'ab'.repeat(65);
const successResult = () => ({ decryptedValue: '00', signatures: [signature], extraData: requestExtraData() });

/** A relayer answer: the success envelope around `result`, or an error body with its status. */
const succeeded = (result: unknown) =>
  new Response(JSON.stringify({ status: 'succeeded', requestId: 'r1', result }), { status: 200 });
const refused = (status: number, code: string) =>
  new Response(JSON.stringify({ code, message: `${code} for this request`, requestId: 'r1' }), { status });

describe('publicDecryptCertificate', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it('posts the Connector body to the decryption relayer once and returns an untrusted claim over the batch', async () => {
    const fetchMock = vi.fn().mockResolvedValue(succeeded(successResult()));
    global.fetch = fetchMock;

    const otherHandle = handle.slice();
    otherHandle[0] = 0x07;
    const otherAccount = new Uint8Array(32).fill(8);
    const claim = await publicDecryptCertificate(context, {
      ...parameters(),
      entries: [
        { handle, encryptedStore: account },
        { handle: otherHandle, encryptedStore: otherAccount },
      ],
    });

    expect(fetchMock).toHaveBeenCalledTimes(1);
    const [url, init] = fetchMock.mock.calls[0] as [string, RequestInit];
    expect(url).toBe('https://decryption.relayer.example.com/v4/public-decrypt');
    expect((init.headers as Record<string, string>)['x-api-key']).toBe('test');
    expect(JSON.parse(String(init.body))).toStrictEqual({
      scheme: 'solana-public-decrypt-v1',
      ctHandles: [bytesToHex(handle), bytesToHex(otherHandle)],
      extraData: requestExtraData(),
      encryptedStores: [bytesToHex(account), bytesToHex(otherAccount)],
    });
    expect(claim).toEqual({
      handles: [bytesToHex(handle), bytesToHex(otherHandle)],
      abiEncodedCleartext: '00',
      signatures: [signature],
      extraData: requestExtraData(),
    });
  });

  // A public leaf the coprocessors have not indexed yet is denied for now: the same bytes, later.
  it('submits the same body again after an ACL denial, and answers once it passes', async () => {
    vi.useFakeTimers();
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(refused(403, 'acl_denied'))
      .mockResolvedValue(succeeded(successResult()));
    global.fetch = fetchMock;

    const pending = publicDecryptCertificate(context, parameters());
    await vi.runAllTimersAsync();
    await expect(pending).resolves.toMatchObject({ abiEncodedCleartext: '00' });
    expect(fetchMock).toHaveBeenCalledTimes(2);
    const bodies = fetchMock.mock.calls.map(([, init]) => (init as RequestInit).body);
    expect(bodies[1]).toBe(bodies[0]);
  });

  it('gives up at once on a refusal nothing repairs, with the code the relayer sent', async () => {
    const fetchMock = vi.fn().mockResolvedValue(refused(410, 'kms_context_destroyed'));
    global.fetch = fetchMock;

    const error = await publicDecryptCertificate(context, parameters()).catch((caught: unknown) => caught);
    expect(error).toBeInstanceOf(SolanaDecryptionRunError);
    expect((error as SolanaDecryptionRunError).rejection).toEqual({
      code: 'kms_context_destroyed',
      message: 'kms_context_destroyed for this request',
    });
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });

  it('surfaces an answer that is neither envelope as the infrastructure error it is', async () => {
    global.fetch = vi.fn().mockResolvedValue(new Response('<html>bad gateway</html>', { status: 502 }));
    await expect(publicDecryptCertificate(context, parameters())).rejects.toThrow(
      'answered HTTP 502 without an error body',
    );
  });

  it.each([
    [{ ...successResult(), decryptedValue: '' }, 'cleartext must be nonempty'],
    [{ ...successResult(), decryptedValue: '0' }, 'cleartext must be nonempty'],
    [{ ...successResult(), decryptedValue: 'zz' }, 'cleartext must be nonempty'],
    [{ ...successResult(), signatures: [] }, 'at least one signature'],
    [{ ...successResult(), signatures: ['ab'] }, 'valid 65-byte hex'],
    [{ ...successResult(), signatures: ['a'] }, 'got 1 hex characters'],
    [{ ...successResult(), signatures: ['zz'.repeat(65)] }, 'valid 65-byte hex'],
    [{ ...successResult(), extraData: '0x00' }, 'extraData does not match'],
    [{ ...successResult(), extraData: `0x01${'05'.repeat(32)}` }, 'extraData does not match'],
  ])('rejects malformed certificate material %#', async (result, message) => {
    global.fetch = vi.fn().mockResolvedValue(succeeded(result));
    await expect(publicDecryptCertificate(context, parameters())).rejects.toThrow(message);
  });
});
