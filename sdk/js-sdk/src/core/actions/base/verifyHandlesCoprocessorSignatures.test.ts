import type { EthereumModule } from '../../modules/ethereum/types.js';
import type { RelayerModule } from '../../modules/relayer/types.js';
import type { InputHandle } from '../../types/encryptedTypes-p.js';
import type { Bytes65Hex, BytesHex, ChecksummedAddress, Uint64BigInt } from '../../types/primitives.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../../ethers/internal/ethers-p.js';
import { sepolia } from '../../chains/definitions/sepolia.js';
import { createCoreFhevm } from '../../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../../runtime/CoreFhevmRuntime-p.js';
import { DuplicateSignerError, UnknownSignerError } from '../../errors/SignersError.js';

// Recovered signer set is derived from the EIP-712 signatures; mock it so each
// test controls the recovered addresses directly.
const { recoverSignersMock } = vi.hoisted(() => ({ recoverSignersMock: vi.fn() }));
vi.mock('../../utils-p/runtime/recoverSigners.js', () => ({
  recoverSigners: recoverSignersMock,
}));

import { verifyHandlesCoprocessorSignatures } from './verifyHandlesCoprocessorSignatures.js';

////////////////////////////////////////////////////////////////////////////////

const A = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266' as ChecksummedAddress;
const B = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8' as ChecksummedAddress;
const C = '0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC' as ChecksummedAddress;
const D = '0x90F79bf6EB2c4f870365E785982E1f101E93b906' as ChecksummedAddress;
const USER = '0x976EA74026E726554dB657fA54763abd0C3a0aa9' as ChecksummedAddress;
const CONTRACT = '0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65' as ChecksummedAddress;

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

function makeFhevm(
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
    inputHandles: [{ bytes32Hex: `0x${'ab'.repeat(32)}` } as unknown as InputHandle],
    userAddress: USER,
    contractAddress: CONTRACT,
    chainId: 11155111n as Uint64BigInt,
    extraData: '0x' as BytesHex,
  };
}

////////////////////////////////////////////////////////////////////////////////

describe('actions/base verifyHandlesCoprocessorSignatures — stale signer refetch', () => {
  beforeEach(() => {
    recoverSignersMock.mockReset();
  });

  it('passes without refetching when the cached signer set is already valid', async () => {
    recoverSignersMock.mockResolvedValue([A, B, C]);
    const { readContract } = makeReadContract([{ threshold: 3, signers: [A, B, C, D] }]);
    const fhevm = makeFhevm(readContract);

    await expect(verifyHandlesCoprocessorSignatures(fhevm, makeParameters())).resolves.toBeUndefined();
    expect(readContract).toHaveBeenCalledTimes(2);
  });

  it('force-refreshes and recovers on an unknown signer (rotated-in coprocessor)', async () => {
    recoverSignersMock.mockResolvedValue([A, B, D]);
    const { readContract } = makeReadContract([
      { threshold: 3, signers: [A, B, C] }, // stale: D missing
      { threshold: 3, signers: [A, B, C, D] }, // fresh: D present
    ]);
    const fhevm = makeFhevm(readContract);

    await expect(verifyHandlesCoprocessorSignatures(fhevm, makeParameters())).resolves.toBeUndefined();
    expect(readContract).toHaveBeenCalledTimes(4);
  });

  it('does NOT refetch on a duplicate recovered signer', async () => {
    recoverSignersMock.mockResolvedValue([A, A]);
    const { readContract } = makeReadContract([{ threshold: 1, signers: [A, B, C] }]);
    const fhevm = makeFhevm(readContract);

    await expect(verifyHandlesCoprocessorSignatures(fhevm, makeParameters())).rejects.toBeInstanceOf(
      DuplicateSignerError,
    );
    expect(readContract).toHaveBeenCalledTimes(2);
  });

  it('retries exactly once, surfacing the error when the fresh set is still invalid', async () => {
    recoverSignersMock.mockResolvedValue([A, B, D]);
    const { readContract } = makeReadContract([
      { threshold: 3, signers: [A, B, C] },
      { threshold: 3, signers: [A, B, C] }, // D still missing
    ]);
    const fhevm = makeFhevm(readContract);

    await expect(verifyHandlesCoprocessorSignatures(fhevm, makeParameters())).rejects.toBeInstanceOf(
      UnknownSignerError,
    );
    expect(readContract).toHaveBeenCalledTimes(4);
  });
});
