import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { InputHandle } from '../types/encryptedTypes-p.js';
import type { Bytes65Hex, BytesHex, ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { DuplicateSignerError, ThresholdSignerError, UnknownSignerError } from '../errors/SignersError.js';

// The signer set is recovered from the EIP-712 signatures via on-chain-free
// crypto. Mock it so each test controls exactly which addresses come back,
// independent of the (irrelevant) signature bytes.
const { recoverSignersMock } = vi.hoisted(() => ({ recoverSignersMock: vi.fn() }));
vi.mock('../utils-p/runtime/recoverSigners.js', () => ({
  recoverSigners: recoverSignersMock,
}));

import { verifyHandlesCoprocessorSignatures } from './verifyHandlesCoprocessorSignatures.js';

////////////////////////////////////////////////////////////////////////////////
// Valid EIP-55 checksummed addresses (hardhat default accounts).
////////////////////////////////////////////////////////////////////////////////

const A = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266' as ChecksummedAddress;
const B = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8' as ChecksummedAddress;
const C = '0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC' as ChecksummedAddress;
const D = '0x90F79bf6EB2c4f870365E785982E1f101E93b906' as ChecksummedAddress;
const USER = '0x976EA74026E726554dB657fA54763abd0C3a0aa9' as ChecksummedAddress;
const CONTRACT = '0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65' as ChecksummedAddress;

////////////////////////////////////////////////////////////////////////////////

/**
 * Mock `readContract` that serves the InputVerifier `getThreshold()` and
 * `getCoprocessorSigners()` calls. Each call to the read path consumes one
 * "round" (one threshold + one signers entry), so index 0 is the initial
 * (possibly stale) read and index 1 is what a forced refresh would fetch.
 */
function makeReadContract(rounds: ReadonlyArray<{ threshold: number; signers: ChecksummedAddress[] }>): {
  readContract: EthereumModule['readContract'];
} {
  const thresholds = rounds.map((r) => r.threshold);
  const signers = rounds.map((r) => [...r.signers]);

  const readContract = vi.fn(async (_trustedClient: unknown, parameters: { functionName: string }) => {
    if (parameters.functionName === 'getThreshold') {
      const t = thresholds.shift();
      if (t === undefined) {
        throw new Error('No more mocked thresholds');
      }
      return t;
    }
    if (parameters.functionName === 'getCoprocessorSigners') {
      const s = signers.shift();
      if (s === undefined) {
        throw new Error('No more mocked signers');
      }
      return s;
    }
    throw new Error(`Unexpected functionName ${parameters.functionName}`);
  }) as unknown as EthereumModule['readContract'];

  return { readContract };
}

function makeContext(
  readContract: EthereumModule['readContract'],
): ReturnType<typeof createCoreFhevm<typeof sepolia, ReturnType<typeof createFhevmRuntime>, object>> {
  const ethereum = { readContract } as unknown as EthereumModule;

  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum,
    relayer: {} as RelayerModule,
    config: {},
  });

  return createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
    chain: sepolia,
    client: {},
    runtime,
  });
}

function makeParameters() {
  return {
    coprocessorSignatures: [`0x${'11'.repeat(65)}`] as Bytes65Hex[],
    handles: [{ bytes32Hex: `0x${'ab'.repeat(32)}` } as unknown as InputHandle],
    userAddress: USER,
    contractAddress: CONTRACT,
    chainId: 11155111n as Uint64BigInt,
    extraData: '0x' as BytesHex,
  };
}

////////////////////////////////////////////////////////////////////////////////

describe('verifyHandlesCoprocessorSignatures — stale signer refetch', () => {
  beforeEach(() => {
    recoverSignersMock.mockReset();
  });

  it('passes without refetching when the cached signer set is already valid', async () => {
    recoverSignersMock.mockResolvedValue([A, B, C]);
    const { readContract } = makeReadContract([{ threshold: 3, signers: [A, B, C, D] }]);
    const context = makeContext(readContract);

    await expect(verifyHandlesCoprocessorSignatures(context, makeParameters())).resolves.toBeUndefined();

    // One round only: getThreshold + getCoprocessorSigners. No forced refresh.
    expect(readContract).toHaveBeenCalledTimes(2);
  });

  it('reuses the TTL cache across calls when verification keeps succeeding', async () => {
    recoverSignersMock.mockResolvedValue([A, B, C]);
    const { readContract } = makeReadContract([{ threshold: 3, signers: [A, B, C, D] }]);
    const context = makeContext(readContract);

    await expect(verifyHandlesCoprocessorSignatures(context, makeParameters())).resolves.toBeUndefined();
    await expect(verifyHandlesCoprocessorSignatures(context, makeParameters())).resolves.toBeUndefined();

    // Second verification hits the cache — still only the initial 2 reads.
    expect(readContract).toHaveBeenCalledTimes(2);
  });

  it('force-refreshes and recovers on an unknown signer (rotated-in coprocessor)', async () => {
    // Recovered set includes D, which the stale on-chain snapshot does not know.
    recoverSignersMock.mockResolvedValue([A, B, D]);
    const { readContract } = makeReadContract([
      { threshold: 3, signers: [A, B, C] }, // stale: D missing -> UnknownSignerError
      { threshold: 3, signers: [A, B, C, D] }, // fresh: D present -> passes
    ]);
    const context = makeContext(readContract);

    await expect(verifyHandlesCoprocessorSignatures(context, makeParameters())).resolves.toBeUndefined();

    // 2 reads for the stale round + 2 for the forced refresh.
    expect(readContract).toHaveBeenCalledTimes(4);
  });

  it('force-refreshes and recovers when the cached threshold is stale', async () => {
    recoverSignersMock.mockResolvedValue([A]);
    const { readContract } = makeReadContract([
      { threshold: 3, signers: [A, B, C] }, // stale: 1 < 3 -> ThresholdSignerError
      { threshold: 1, signers: [A, B, C] }, // fresh: 1 >= 1 -> passes
    ]);
    const context = makeContext(readContract);

    await expect(verifyHandlesCoprocessorSignatures(context, makeParameters())).resolves.toBeUndefined();

    expect(readContract).toHaveBeenCalledTimes(4);
  });

  it('retries exactly once, surfacing the error when the fresh set is still invalid', async () => {
    recoverSignersMock.mockResolvedValue([A, B, D]);
    const { readContract } = makeReadContract([
      { threshold: 3, signers: [A, B, C] }, // stale: D missing
      { threshold: 3, signers: [A, B, C] }, // fresh: D still missing -> rethrow
    ]);
    const context = makeContext(readContract);

    await expect(verifyHandlesCoprocessorSignatures(context, makeParameters())).rejects.toBeInstanceOf(
      UnknownSignerError,
    );

    // Exactly one refresh — not an unbounded loop.
    expect(readContract).toHaveBeenCalledTimes(4);
  });

  it('does NOT refetch on a duplicate recovered signer (malicious-relayer signal)', async () => {
    // A appears twice in the recovered set — a relayer-side signal that a fresh
    // on-chain read cannot fix, so it must throw without a refresh.
    recoverSignersMock.mockResolvedValue([A, A]);
    const { readContract } = makeReadContract([{ threshold: 1, signers: [A, B, C] }]);
    const context = makeContext(readContract);

    await expect(verifyHandlesCoprocessorSignatures(context, makeParameters())).rejects.toBeInstanceOf(
      DuplicateSignerError,
    );

    // Only the initial read — no forced refresh.
    expect(readContract).toHaveBeenCalledTimes(2);
  });

  it('reports a ThresholdSignerError after the refresh also fails the threshold', async () => {
    recoverSignersMock.mockResolvedValue([A]);
    const { readContract } = makeReadContract([
      { threshold: 3, signers: [A, B, C] },
      { threshold: 2, signers: [A, B, C] }, // still 1 < 2
    ]);
    const context = makeContext(readContract);

    await expect(verifyHandlesCoprocessorSignatures(context, makeParameters())).rejects.toBeInstanceOf(
      ThresholdSignerError,
    );
    expect(readContract).toHaveBeenCalledTimes(4);
  });
});
