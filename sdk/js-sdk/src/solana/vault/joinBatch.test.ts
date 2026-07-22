import type { EncryptionBits } from '../../core/types/fheType.js';
import type { Bytes32Hex } from '../../core/types/primitives.js';
import type { SolanaZkProof } from '../../core/types/zkProof-p.js';
import { toSolanaZkProof } from '../../core/coprocessor/SolanaZkProof-p.js';
import { bytesToHex } from '../../core/base/bytes.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const sendAndConfirm = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  sendAndConfirmTransactionFactory: () => sendAndConfirm,
}));

import {
  address,
  decompileTransactionMessage,
  generateKeyPairSigner,
  getBase64Encoder,
  getCompiledTransactionMessageDecoder,
  getTransactionDecoder,
  type Address,
  type TransactionSigner,
} from '@solana/kit';
import { base58 } from '@scure/base';

import { joinBatch, type SolanaVaultJoinParameters } from './joinBatch.js';
import { findComputeSignerPda } from '../internal/generated/confidentialToken/pdas/computeSigner.js';
import { getJoinInstructionDataDecoder } from './internal/generated/confidentialBatcher/instructions/join.js';

const CHAIN_ID = (1n << 63n) | 12345n;
const CANONICAL_ACL = bytesToHex(base58.decode('6AtbvED1rfX68aCT1tYgU1aeu4kFksPDxZG9gtB1Fgtu'));
const SIGNATURE = `0x${'44'.repeat(65)}` as const;

function key(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}
function signer(a: Address): TransactionSigner {
  return { address: a, signTransactions: async () => [] } as unknown as TransactionSigner;
}

function proof(
  owner: Address,
  contract: Address,
  overrides: { acl?: Bytes32Hex; chainId?: bigint; bits?: readonly EncryptionBits[] } = {},
): SolanaZkProof {
  return toSolanaZkProof({
    chainId: overrides.chainId ?? CHAIN_ID,
    aclContractAddress: overrides.acl ?? CANONICAL_ACL,
    contractAddress: bytesToHex(base58.decode(contract)),
    userAddress: bytesToHex(base58.decode(owner)),
    ciphertextWithZkProof: new Uint8Array([1]),
    encryptionBits: overrides.bits ?? [64],
  });
}

async function parameters(overrides: Partial<SolanaVaultJoinParameters> = {}): Promise<SolanaVaultJoinParameters> {
  const joinConfidentialMint = key(2);
  const user = signer(key(1));
  const [computeSigner] = await findComputeSignerPda({ mint: joinConfidentialMint });
  const inputProof = proof(user.address, computeSigner);
  return {
    rpc: {} as SolanaVaultJoinParameters['rpc'],
    rpcSubscriptions: {} as SolanaVaultJoinParameters['rpcSubscriptions'],
    inputProof,
    inputProofResult: {
      handles: inputProof.getInputHandles(),
      signatures: [SIGNATURE] as never,
      extraData: '0x00' as never,
    },
    inputIndex: 0,
    user,
    payer: signer(key(3)),
    batcher: key(4),
    batch: key(5),
    joinConfidentialMint,
    userBalanceValue: key(6),
    batchBalanceValue: key(7),
    userTransferredValue: key(8),
    pendingJoinValue: key(9),
    hostConfig: key(10),
    ...overrides,
  } satisfies SolanaVaultJoinParameters;
}

const context = { solanaChain: { id: CHAIN_ID } as never, aclProgramAddress: CANONICAL_ACL as never };

describe('joinBatch (attested arm)', () => {
  beforeEach(() => sendAndConfirm.mockReset().mockResolvedValue(undefined));

  it('builds, simulates, sends, and encodes the coprocessor attestation into the join instruction', async () => {
    const user = await generateKeyPairSigner();
    const joinConfidentialMint = key(2);
    const [computeSigner] = await findComputeSignerPda({ mint: joinConfidentialMint });
    const inputProof = proof(user.address, computeSigner);
    const simulate = vi.fn().mockReturnValue({ send: vi.fn().mockResolvedValue({ value: { err: null } }) });
    const params = await parameters({
      user,
      payer: user,
      joinConfidentialMint,
      inputProof,
      inputProofResult: {
        handles: inputProof.getInputHandles(),
        signatures: [SIGNATURE] as never,
        extraData: '0x00' as never,
      },
      rpc: {
        getLatestBlockhash: vi.fn().mockReturnValue({
          send: vi.fn().mockResolvedValue({ value: { blockhash: key(20), lastValidBlockHeight: 1_000n } }),
        }),
        simulateTransaction: simulate,
      } as unknown as SolanaVaultJoinParameters['rpc'],
    });

    await expect(joinBatch(context, params)).resolves.toEqual(expect.any(String));
    expect(sendAndConfirm).toHaveBeenCalledOnce();

    const wire = simulate.mock.calls[0]![0] as string;
    const transaction = getTransactionDecoder().decode(getBase64Encoder().encode(wire));
    const compiled = getCompiledTransactionMessageDecoder().decode(transaction.messageBytes);
    const message = decompileTransactionMessage(compiled);
    // [0] = SetComputeUnitLimit, [1] = join.
    const data = getJoinInstructionDataDecoder().decode(message.instructions[1]!.data!);
    expect(data.handleIndex).toBe(0);
    expect(data.contractChainId).toBe(CHAIN_ID);
    expect(Array.from(data.inputHandle)).toEqual(Array.from(inputProof.getInputHandles()[0]!.bytes32));
    expect(data.signatures).toHaveLength(1);
  });

  it.each([
    [
      'a non-u64 input',
      async () => {
        const p = await parameters();
        const bad = proof(p.user.address, (await findComputeSignerPda({ mint: p.joinConfidentialMint }))[0], {
          bits: [8],
        });
        return { ...p, inputProof: bad, inputProofResult: { ...p.inputProofResult, handles: bad.getInputHandles() } };
      },
      'must be euint64',
    ],
    [
      'a different owner',
      async () => ({ ...(await parameters()), user: signer(key(99)) }),
      'does not match the joining user',
    ],
    [
      'a malformed attestation signature',
      async () => {
        const p = await parameters();
        return { ...p, inputProofResult: { ...p.inputProofResult, signatures: [`0x44` as never] } };
      },
      'must be 65 bytes',
    ],
  ])('rejects %s before any RPC call', async (_name, mutate, message) => {
    const getLatestBlockhash = vi.fn();
    const simulateTransaction = vi.fn();
    const base = await mutate();
    const params = {
      ...base,
      rpc: { getLatestBlockhash, simulateTransaction } as unknown as SolanaVaultJoinParameters['rpc'],
    };
    await expect(joinBatch(context, params)).rejects.toThrow(message);
    expect(getLatestBlockhash).not.toHaveBeenCalled();
    expect(simulateTransaction).not.toHaveBeenCalled();
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });
});
