import * as input from '../actions/encryptInput.js';
import * as submission from '../actions/submitInputProof.js';
import * as lifecycle from '../../core/runtime/CoreFhevm-p.js';
import { toSolanaZkProof } from '../../core/coprocessor/SolanaZkProof-p.js';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { createSolanaRpc } from '@solana/kit';
import { createFhevmEncryptClient } from './createFhevmEncryptClient.js';
import { setFhevmRuntimeConfig } from '../internal/config.js';
import type { FheEncryptionKeyBytes } from '../../core/types/fheEncryptionKey.js';

const chain = { id: 9223372036854788153n, fhevm: { relayerUrl: 'https://relayer.example.test' } };
const rpc = createSolanaRpc('http://localhost:8899');

afterEach(() => vi.restoreAllMocks());

describe('Solana encrypt client configuration', () => {
  it('uses the native RPC without advertising EVM capabilities', () => {
    setFhevmRuntimeConfig({});
    const client = createFhevmEncryptClient({ chain, rpc });
    expect(client.chain).toBe(chain);
    expect(client.rpc).toBe(rpc);
    expect(client.encryptValues).toBeTypeOf('function');
    expect(client.generateZkProof).toBeTypeOf('function');
    for (const member of ['ethereum', 'runtime', 'protocolVersion', 'options', 'extend'])
      expect(member in client).toBe(false);
  });
  it('rejects an injected key for another relayer before accessing its material', () => {
    const key = { metadata: { relayerUrl: 'https://another.example.test' } } as FheEncryptionKeyBytes;
    expect(() => createFhevmEncryptClient({ chain, rpc, options: { fheEncryptionKey: key } })).toThrow(
      'does not match',
    );
  });
  it('rejects unsupported options from an untyped caller', () => {
    // @ts-expect-error EVM RPC batching has no effect on Solana encryption.
    expect(() => createFhevmEncryptClient({ chain, rpc, options: { batchRpcCalls: true } })).toThrow('Unsupported');
  });
  it('composes proving and submission into typed values and a consumable attestation', async () => {
    const proof = toSolanaZkProof({
      chainId: chain.id,
      aclContractAddress: `0x${'11'.repeat(32)}` as never,
      contractAddress: `0x${'22'.repeat(32)}` as never,
      userAddress: `0x${'33'.repeat(32)}` as never,
      ciphertextWithZkProof: new Uint8Array([1]),
      encryptionBits: [64],
    });
    vi.spyOn(lifecycle, 'initPublicAction').mockResolvedValue({ tfheVersion: '1.6.2' } as never);
    vi.spyOn(input, 'encryptInput').mockResolvedValue(proof);
    const attestation = { handles: proof.getInputHandles(), signatures: [], extraData: '0x00' as never };
    const submit = vi.spyOn(submission, 'submitInputProof').mockResolvedValue(attestation);
    const client = createFhevmEncryptClient({ chain, rpc });
    const result = await client.encryptValues({
      contractAddress: proof.contractAddress,
      userAddress: proof.userAddress,
      values: [{ type: 'uint64', value: 42n }],
    });
    expect(submit).toHaveBeenCalledWith(expect.anything(), { inputProof: proof, options: undefined });
    expect(result.encryptedValues).toEqual(attestation.handles.map((handle) => handle.bytes32Hex));
    expect(result.inputProof).toMatchObject({
      ...attestation,
      chainId: chain.id,
      contractAddress: proof.contractAddress,
      userAddress: proof.userAddress,
    });
    expect('ciphertextWithZkProof' in result.inputProof).toBe(false);
    const scalar = await client.encryptValue({
      contractAddress: proof.contractAddress,
      userAddress: proof.userAddress,
      value: { type: 'uint64', value: 42n },
    });
    expect(scalar).toEqual({ encryptedValue: result.encryptedValues[0], inputProof: result.inputProof });
  });
});
