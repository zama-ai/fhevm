import { readFileSync } from 'node:fs';
import { afterEach, describe, expect, it, vi } from 'vitest';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { RelayerAsyncRequest } from '../../core/modules/relayer/module/RelayerAsyncRequest.js';
import { bytesToHex } from '../../core/base/bytes.js';
import { hexToBytes } from '../proof.js';
import {
  buildSolanaPublicDecryptExtraData,
  publicDecryptCertificate,
  type SolanaPublicDecryptCertificateParameters,
} from './publicDecryptCertificate.js';

const handle = new Uint8Array(32);
handle[22] = 0x80;
const account = new Uint8Array(32).fill(4);
const contextId = new Uint8Array(32).fill(5);

const parameters = (): SolanaPublicDecryptCertificateParameters => ({
  handle,
  contextId,
  encryptedValueAccount: account,
  options: { fetchRetries: 1 },
});

const context = {
  chain: {
    id: 0x8000000000000000n,
    fhevm: { relayerUrl: 'https://relayer.example.com' },
  },
  runtime: { config: { auth: { type: 'ApiKeyHeader', value: 'test' } } } as FhevmRuntime,
};

const requestExtraData = () => bytesToHex(buildSolanaPublicDecryptExtraData(contextId, account));

////////////////////////////////////////////////////////////////////////////////
// The committed carrier byte vectors, run against this encoder.
//
// The fixture is shared with the connector (`solana_extra_data_byte_vectors.rs` runs the same
// records against the Rust codec), and this runner is what keeps the two hand-mirrored layouts
// pinned to each other. The `malformed` section exercises parsing, which only Rust does.
////////////////////////////////////////////////////////////////////////////////

/* eslint-disable @typescript-eslint/naming-convention -- the fixture's own field names are snake_case */

interface ExtraDataVectors {
  readonly schema: string;
  readonly records: ReadonlyArray<{
    readonly name: string;
    readonly input: {
      readonly context_id_hex: string;
      readonly encrypted_value_account_hex: string;
    };
    readonly blob_hex: string;
  }>;
  readonly malformed: ReadonlyArray<{ readonly name: string }>;
}

/* eslint-enable @typescript-eslint/naming-convention */

describe('committed extraData byte vectors (solana/test-fixtures/user-decrypt)', () => {
  const extraData = JSON.parse(
    readFileSync(
      new URL('../../../../../solana/test-fixtures/user-decrypt/extra_data_v1.json', import.meta.url),
      'utf8',
    ),
  ) as ExtraDataVectors;

  it('recognizes the fixture schema and finds records to run', () => {
    expect(extraData.schema).toBe('zama-solana-public-decrypt-extra-data/v1');
    expect(extraData.records.length).toBeGreaterThan(0);
    expect(extraData.malformed.length).toBeGreaterThan(0);
  });

  it.each(extraData.records.map((record) => [record.name, record] as const))('extraData blob: %s', (_name, record) => {
    const blob = buildSolanaPublicDecryptExtraData(
      hexToBytes(`0x${record.input.context_id_hex}`),
      hexToBytes(`0x${record.input.encrypted_value_account_hex}`),
    );
    expect(blob).toHaveLength(65);
    expect(bytesToHex(blob)).toBe(`0x${record.blob_hex}`);
  });

  it('refuses a field of the wrong width before anything is sent', () => {
    expect(() => buildSolanaPublicDecryptExtraData(new Uint8Array(31), account)).toThrow('contextId must be 32 bytes');
    expect(() => buildSolanaPublicDecryptExtraData(contextId, new Uint8Array(33))).toThrow(
      'encryptedValueAccount must be 32 bytes',
    );
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
