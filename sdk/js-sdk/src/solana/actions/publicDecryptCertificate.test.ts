import { afterEach, describe, expect, it, vi } from 'vitest';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { RelayerAsyncRequest } from '../../core/modules/relayer/module/RelayerAsyncRequest.js';
import { bytesToHex } from '../../core/base/bytes.js';
import {
  publicDecryptCertificate,
  solanaPublicDecryptExtraData,
  type SolanaPublicDecryptCertificateParameters,
} from './publicDecryptCertificate.js';
import { asBytes32Hex } from '../../core/base/bytes.js';

const handle = new Uint8Array(32);
handle[22] = 0x01;
const account = new Uint8Array(32).fill(4);
const contextId = new Uint8Array(32).fill(5);

const parameters = (): SolanaPublicDecryptCertificateParameters => ({
  handle,
  contextId,
  encryptedStore: account,
  options: { fetchRetries: 1 },
});

const context = {
  chain: {
    id: 0x0100000000000000n,
    fhevm: {
      relayerUrl: 'https://relayer.example.com',
      programs: { host: { address: asBytes32Hex(`0x${'22'.repeat(32)}`) } },
    },
  },
  runtime: { config: { auth: { type: 'ApiKeyHeader', value: 'test' } } } as FhevmRuntime,
};

const requestExtraData = () => solanaPublicDecryptExtraData(contextId);

describe('solanaPublicDecryptExtraData', () => {
  it('is the v1 KMS routing of the context, with no store in it', () => {
    expect(solanaPublicDecryptExtraData(contextId)).toBe(`0x01${'05'.repeat(32)}`);
  });

  it('refuses a field of the wrong width before anything is sent', async () => {
    expect(() => solanaPublicDecryptExtraData(new Uint8Array(31))).toThrow('contextId must be 32 bytes');
    await expect(
      publicDecryptCertificate(context, { ...parameters(), encryptedStore: new Uint8Array(33) }),
    ).rejects.toThrow('encryptedStore must be 32 bytes');
  });
});

const signature = 'ab'.repeat(65);
const successResult = () => ({ decryptedValue: '00', signatures: [signature], extraData: requestExtraData() });

describe('publicDecryptCertificate', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it('follows the queued relayer path and returns an untrusted claim', async () => {
    vi.useFakeTimers();
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(
        new Response(JSON.stringify({ status: 'queued', requestId: 'r1', result: { jobId: 'j1' } }), {
          status: 202,
          headers: { 'Retry-After': '1' },
        }),
      )
      .mockResolvedValueOnce(
        new Response(JSON.stringify({ status: 'succeeded', requestId: 'r1', result: successResult() }), {
          status: 200,
        }),
      );
    global.fetch = fetchMock;

    const pending = publicDecryptCertificate(context, parameters());
    await vi.runAllTimersAsync();
    const claim = await pending;

    expect(fetchMock).toHaveBeenCalledTimes(2);
    expect(JSON.parse(String((fetchMock.mock.calls[0]?.[1] as RequestInit).body))).toEqual({
      ciphertextHandles: [bytesToHex(handle)],
      extraData: requestExtraData(),
      encryptedStores: [bytesToHex(account)],
    });
    expect(claim).toEqual({
      handle: bytesToHex(handle),
      abiEncodedCleartext: '00',
      signatures: [signature],
      extraData: requestExtraData(),
    });
  });

  it('retries only the typed KMS readiness timeout before queuing the certificate job', async () => {
    vi.useFakeTimers();
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(
        new Response(
          JSON.stringify({
            status: 'failed',
            error: { label: 'readiness_check_timed_out', message: 'KMS material is still indexing' },
          }),
          { status: 503, headers: { 'Retry-After': '1' } },
        ),
      )
      .mockResolvedValueOnce(
        new Response(JSON.stringify({ status: 'queued', requestId: 'r1', result: { jobId: 'j1' } }), {
          status: 202,
          headers: { 'Retry-After': '1' },
        }),
      )
      .mockResolvedValueOnce(
        new Response(JSON.stringify({ status: 'succeeded', requestId: 'r1', result: successResult() }), {
          status: 200,
        }),
      );
    global.fetch = fetchMock;

    const pending = publicDecryptCertificate(context, parameters());
    await vi.runAllTimersAsync();
    await expect(pending).resolves.toMatchObject({ abiEncodedCleartext: '00' });
    expect(fetchMock).toHaveBeenCalledTimes(3);
  });

  it('does not retry a non-readiness 503', async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      new Response(
        JSON.stringify({
          status: 'failed',
          error: { label: 'gateway_not_reachable', message: 'KMS gateway is unavailable' },
        }),
        { status: 503 },
      ),
    );
    global.fetch = fetchMock;

    await expect(publicDecryptCertificate(context, parameters())).rejects.toThrow('gateway_not_reachable');
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });

  it('uses the requested extraData when the relayer omits the optional response field', async () => {
    const result = successResult();
    vi.spyOn(RelayerAsyncRequest.prototype, 'run').mockResolvedValue({
      decryptedValue: result.decryptedValue,
      signatures: result.signatures,
    } as never);

    await expect(publicDecryptCertificate(context, parameters())).resolves.toMatchObject({
      extraData: requestExtraData(),
    });
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
  ])('rejects malformed certificate material %#', async (result, message) => {
    vi.spyOn(RelayerAsyncRequest.prototype, 'run').mockResolvedValue(result as never);
    await expect(publicDecryptCertificate(context, parameters())).rejects.toThrow(message);
  });

  it('preserves relayer terminal errors', async () => {
    const terminal = Object.assign(new Error('relayer request failed'), { status: 'failed' });
    vi.spyOn(RelayerAsyncRequest.prototype, 'run').mockRejectedValue(terminal);
    let thrown: unknown;
    try {
      await publicDecryptCertificate(context, parameters());
    } catch (error) {
      thrown = error;
    }
    expect(thrown).toBe(terminal);
  });
});
