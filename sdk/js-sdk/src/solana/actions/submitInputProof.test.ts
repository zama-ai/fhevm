import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { InputHandle } from '../../core/types/encryptedTypes-p.js';
import type { EncryptionBits } from '../../core/types/fheType.js';
import type { Bytes32Hex } from '../../core/types/primitives.js';
import type { SolanaZkProof } from '../../core/types/zkProof-p.js';
import { hexToBytes32 } from '../../core/base/bytes.js';
import { fetchCoprocessorSignatures } from '../../core/modules/relayer/module/fetchCoprocessorSignatures.js';
import { toSolanaZkProof } from '../../core/coprocessor/SolanaZkProof-p.js';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import { base58 } from '@scure/base';

import { submitInputProof } from './submitInputProof.js';

const CHAIN_ID = (1n << 63n) | 12345n;
const ACL = `0x${'11'.repeat(32)}` as Bytes32Hex;
const CONTRACT = `0x${'22'.repeat(32)}` as Bytes32Hex;
const USER = `0x${'33'.repeat(32)}` as Bytes32Hex;
const SIGNATURE = `0x${'44'.repeat(65)}` as const;

function proof(ciphertext = 1, encryptionBits: readonly EncryptionBits[] = [64]): SolanaZkProof {
  return toSolanaZkProof({
    chainId: CHAIN_ID,
    aclContractAddress: ACL,
    contractAddress: CONTRACT,
    userAddress: USER,
    ciphertextWithZkProof: new Uint8Array([ciphertext]),
    encryptionBits,
  });
}

function response(status: number, body: unknown): Response {
  return new Response(JSON.stringify(body), { status, headers: { 'retry-after': '1' } });
}

function queued(requestId = 'post-request'): Response {
  return response(202, { status: 'queued', requestId, result: { jobId: 'job-1' } });
}

function pending(requestId = 'get-request'): Response {
  return response(202, { status: 'queued', requestId });
}

function succeeded(handles: readonly InputHandle[]): Response {
  return response(200, {
    status: 'succeeded',
    requestId: 'done-request',
    result: {
      accepted: true,
      handles: handles.map((handle) => handle.bytes32Hex),
      signatures: [SIGNATURE],
      extraData: '0x00',
    },
  });
}

function context(): { readonly runtime: FhevmRuntime; readonly solanaChain: ReturnType<typeof solanaChain> } {
  const runtime = {
    config: { auth: { type: 'ApiKeyHeader', value: 'test-key' } },
    relayer: { fetchCoprocessorSignatures },
  } as unknown as FhevmRuntime;
  return { runtime, solanaChain: solanaChain() };
}

function solanaChain() {
  return {
    id: CHAIN_ID,
    fhevm: { relayerUrl: 'https://relayer.example.com', acl: { domainKeys: [ACL] } },
  } as const;
}

async function settle<T>(promise: Promise<T>, queuedGetCount = 0): Promise<T> {
  for (let i = 0; i <= queuedGetCount; i++) {
    await vi.advanceTimersByTimeAsync(1_000);
  }
  return promise;
}

async function expectRejected(promise: Promise<unknown>, message: string): Promise<void> {
  const expectation = expect(promise).rejects.toThrow(message);
  await vi.advanceTimersByTimeAsync(1_000);
  await expectation;
}

describe('submitInputProof', () => {
  const originalFetch = globalThis.fetch;

  beforeEach(() => vi.useFakeTimers());

  afterEach(() => {
    globalThis.fetch = originalFetch;
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it('submits base58 identities and returns the matching typed result', async () => {
    const inputProof = proof();
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(queued())
      .mockResolvedValueOnce(succeeded(inputProof.getInputHandles()));
    globalThis.fetch = fetchMock;

    const result = await settle(submitInputProof(context(), { inputProof }));

    expect(result.handles.map((handle) => handle.bytes32Hex)).toEqual(
      inputProof.getInputHandles().map((handle) => handle.bytes32Hex),
    );
    expect(result.signatures).toEqual([SIGNATURE]);
    expect(result.extraData).toBe('0x00');

    const [, init] = fetchMock.mock.calls[0] as [string, RequestInit];
    expect(new Headers(init.headers).get('x-api-key')).toBe('test-key');
    expect(JSON.parse(init.body as string)).toEqual({
      ciphertextWithInputVerification: '01',
      contractAddress: base58.encode(hexToBytes32(CONTRACT)),
      contractChainId: '0x8000000000003039',
      extraData: '0x00',
      userAddress: base58.encode(hexToBytes32(USER)),
    });
  });

  it('uses the existing queued GET path until the request succeeds', async () => {
    const inputProof = proof();
    globalThis.fetch = vi
      .fn()
      .mockResolvedValueOnce(queued())
      .mockResolvedValueOnce(pending())
      .mockResolvedValueOnce(succeeded(inputProof.getInputHandles()));

    const result = await settle(submitInputProof(context(), { inputProof }), 1);

    expect(result.handles[0]?.bytes32Hex).toBe(inputProof.getInputHandles()[0]?.bytes32Hex);
    expect(globalThis.fetch).toHaveBeenCalledTimes(3);
  });

  it('rejects a malformed terminal response', async () => {
    const inputProof = proof();
    globalThis.fetch = vi
      .fn()
      .mockResolvedValueOnce(queued())
      .mockResolvedValueOnce(
        response(200, {
          status: 'succeeded',
          requestId: 'done-request',
          result: { accepted: true, handles: [123], signatures: [SIGNATURE], extraData: '0x00' },
        }),
      );

    await expectRejected(submitInputProof(context(), { inputProof }), 'does not match the expected schema');
  });

  it('rejects returned handle cardinality mismatches', async () => {
    const inputProof = proof();
    globalThis.fetch = vi.fn().mockResolvedValueOnce(queued()).mockResolvedValueOnce(succeeded([]));

    await expectRejected(submitInputProof(context(), { inputProof }), 'Unexpected handles list sizes');
  });

  it('rejects returned handle order mismatches', async () => {
    const inputProof = proof(1, [8, 16]);
    globalThis.fetch = vi
      .fn()
      .mockResolvedValueOnce(queued())
      .mockResolvedValueOnce(succeeded([...inputProof.getInputHandles()].reverse()));

    await expectRejected(submitInputProof(context(), { inputProof }), 'Unexpected handle[0]');
  });

  it('rejects returned handle value mismatches', async () => {
    const inputProof = proof();
    globalThis.fetch = vi
      .fn()
      .mockResolvedValueOnce(queued())
      .mockResolvedValueOnce(succeeded(proof(2).getInputHandles()));

    await expectRejected(submitInputProof(context(), { inputProof }), 'Unexpected handle[0]');
  });

  it('surfaces terminal relayer errors', async () => {
    const inputProof = proof();
    globalThis.fetch = vi.fn().mockResolvedValueOnce(
      response(400, {
        status: 'failed',
        requestId: 'failed-request',
        error: { label: 'request_error', message: 'proof rejected' },
      }),
    );

    await expect(submitInputProof(context(), { inputProof })).rejects.toThrow('proof rejected');
  });
});
