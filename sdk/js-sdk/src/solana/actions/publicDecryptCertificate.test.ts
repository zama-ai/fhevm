import { afterEach, describe, expect, it, vi } from 'vitest';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { RelayerAsyncRequest } from '../../core/modules/relayer/module/RelayerAsyncRequest.js';
import { buildSolanaUserDecryptMmrProofExtraData } from '../../core/coprocessor/SolanaUserDecrypt-p.js';
import { bytesToHex } from '../../core/base/bytes.js';
import {
  mmrLeafNode,
  publicDecryptLeafCommitment,
  type MmrProof,
} from '../proof.js';
import {
  publicDecryptCertificate,
  type SolanaPublicDecryptCertificateParameters,
} from './publicDecryptCertificate.js';

const handle = new Uint8Array(32);
handle[22] = 0x80;
const account = new Uint8Array(32).fill(4);
const contextId = new Uint8Array(32).fill(5);
const aclValueKey = new Uint8Array(32).fill(6);
const proof: MmrProof = { leafIndex: 0n, siblings: [] };

function u32LE(value: number): Uint8Array {
  const out = new Uint8Array(4);
  new DataView(out.buffer).setUint32(0, value, true);
  return out;
}

function u64LE(value: bigint): Uint8Array {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setBigUint64(0, value, true);
  return out;
}

function concat(...parts: readonly Uint8Array[]): Uint8Array {
  const out = new Uint8Array(parts.reduce((length, part) => length + part.length, 0));
  let offset = 0;
  for (const part of parts) {
    out.set(part, offset);
    offset += part.length;
  }
  return out;
}

const proofBlob = (mode = 0x02, includedProof = proof) =>
  concat(new Uint8Array([mode]), u64LE(includedProof.leafIndex), u32LE(includedProof.siblings.length), ...includedProof.siblings);

const parameters = (): SolanaPublicDecryptCertificateParameters => ({
  handle,
  contextId,
  aclValueKey,
  proofSlot: 1n,
  encryptedValueAccount: account,
  peaks: [mmrLeafNode(publicDecryptLeafCommitment(account, 0n, handle))],
  leafCount: 1n,
  mmrProofBytes: proofBlob(),
  options: { fetchRetries: 1 },
});

const context = {
  chain: {
    id: 0x8000000000000000n,
    fhevm: { relayerUrl: 'https://relayer.example.com', acl: { domainKeys: [] } },
  },
  runtime: { config: { auth: { type: 'ApiKeyHeader', value: 'test' } } } as FhevmRuntime,
};

const requestExtraData = () =>
  bytesToHex(buildSolanaUserDecryptMmrProofExtraData(contextId, aclValueKey, 1n, proofBlob()));
const signature = 'ab'.repeat(65);
const successResult = () => ({ decryptedValue: '00', signatures: [signature], extraData: requestExtraData() });

describe('publicDecryptCertificate', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it('verifies the canonical proof, follows the queued relayer path, and returns an untrusted claim', async () => {
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
      inclusionProof: proof,
    });
  });

  it('rejects a non-public proof mode before the network', async () => {
    await expect(publicDecryptCertificate(context, { ...parameters(), mmrProofBytes: proofBlob(0x01) })).rejects.toThrow(
      'must use mode 0x02',
    );
  });

  it('rejects a malformed proof blob instead of accepting a separate proof', async () => {
    await expect(
      publicDecryptCertificate(context, { ...parameters(), mmrProofBytes: concat(proofBlob(), new Uint8Array([0])) }),
    ).rejects.toThrow('trailing byte');
  });

  it('rejects invalid inclusion', async () => {
    const input = parameters();
    await expect(
      publicDecryptCertificate(context, { ...input, peaks: [new Uint8Array(32)] }),
    ).rejects.toThrow('failed client-side verification');
  });

  it('rejects a proof slot that is not the pinned leaf count', async () => {
    await expect(publicDecryptCertificate(context, { ...parameters(), proofSlot: 0n })).rejects.toThrow(
      'proof slot must equal the pinned leaf count',
    );
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
