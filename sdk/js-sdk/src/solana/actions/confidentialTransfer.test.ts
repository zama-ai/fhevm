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
  AccountRole,
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

import { confidentialTransfer, type SolanaConfidentialTransferParameters } from './confidentialTransfer.js';
import { findComputeSignerPda } from '../internal/generated/confidentialToken/pdas/computeSigner.js';

const CHAIN_ID = (1n << 63n) | 12345n;
const ACL = `0x${'11'.repeat(32)}` as Bytes32Hex;
const CANONICAL_ACL = bytesToHex(base58.decode('6AtbvED1rfX68aCT1tYgU1aeu4kFksPDxZG9gtB1Fgtu'));
const SIGNATURE = `0x${'44'.repeat(65)}` as const;

function key(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}

function signer(address: Address): TransactionSigner {
  return { address, signTransactions: async () => [] } as unknown as TransactionSigner;
}

function proof(
  owner: Address,
  contract: Address,
  overrides: {
    readonly acl?: Bytes32Hex;
    readonly chainId?: bigint;
    readonly bits?: readonly EncryptionBits[];
  } = {},
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

async function parameters(overrides: Partial<SolanaConfidentialTransferParameters> = {}) {
  const mint = key(2);
  const owner = signer(key(1));
  const [computeSigner] = await findComputeSignerPda({ mint });
  const inputProof = proof(owner.address, computeSigner);
  return {
    rpc: {} as SolanaConfidentialTransferParameters['rpc'],
    rpcSubscriptions: {} as SolanaConfidentialTransferParameters['rpcSubscriptions'],
    inputProof,
    inputProofResult: {
      handles: inputProof.getInputHandles(),
      signatures: [SIGNATURE],
      extraData: '0x00' as const,
    },
    inputIndex: 0,
    owner,
    feePayer: signer(key(3)),
    mint,
    fromAccount: key(4),
    toAccount: key(5),
    fromBalanceValue: key(6),
    toBalanceValue: key(7),
    hostConfig: key(8),
    ...overrides,
  } satisfies SolanaConfidentialTransferParameters;
}

const context = { solanaChain: { id: CHAIN_ID } as never, aclProgramAddress: CANONICAL_ACL };

describe('confidentialTransfer attestation binding', () => {
  beforeEach(() => sendAndConfirm.mockReset().mockResolvedValue(undefined));

  it('rejects a result that is not bound to the original proof', async () => {
    const params = await parameters();
    const other = proof(params.owner.address, (await findComputeSignerPda({ mint: params.mint }))[0], {
      bits: [8],
    });
    await expect(
      confidentialTransfer(context, {
        ...params,
        inputProofResult: { ...params.inputProofResult, handles: other.getInputHandles() },
      }),
    ).rejects.toThrow('Unexpected handle[0]');
  });

  it.each([
    [
      'an out-of-range selected index',
      async (params: Awaited<ReturnType<typeof parameters>>) => ({ ...params, inputIndex: 1 }),
      'outside the submitted proof',
    ],
    [
      'a non-u64 selected input',
      async (params: Awaited<ReturnType<typeof parameters>>) => {
        const inputProof = proof(params.owner.address, (await findComputeSignerPda({ mint: params.mint }))[0], {
          bits: [8],
        });
        return {
          ...params,
          inputProof,
          inputProofResult: { ...params.inputProofResult, handles: inputProof.getInputHandles() },
        };
      },
      'must be euint64',
    ],
    [
      'a non-Solana chain id',
      async (params: Awaited<ReturnType<typeof parameters>>) => {
        const inputProof = proof(params.owner.address, (await findComputeSignerPda({ mint: params.mint }))[0], {
          chainId: 12345n,
        });
        return {
          ...params,
          inputProof,
          inputProofResult: { ...params.inputProofResult, handles: inputProof.getInputHandles() },
        };
      },
      'requires a Solana chain id',
    ],
    [
      'a different client chain id',
      async (params: Awaited<ReturnType<typeof parameters>>) => params,
      'does not match the client chain',
    ],
    [
      'a different ACL program',
      async (params: Awaited<ReturnType<typeof parameters>>) => {
        const inputProof = proof(params.owner.address, (await findComputeSignerPda({ mint: params.mint }))[0], {
          acl: ACL,
        });
        return {
          ...params,
          inputProof,
          inputProofResult: { ...params.inputProofResult, handles: inputProof.getInputHandles() },
        };
      },
      'does not match the configured Zama host program',
    ],
    [
      'a different owner',
      async (params: Awaited<ReturnType<typeof parameters>>) => ({ ...params, owner: signer(key(9)) }),
      'does not match the transfer owner',
    ],
    [
      'a different mint domain',
      async (params: Awaited<ReturnType<typeof parameters>>) => ({ ...params, mint: key(10) }),
      'does not match the mint compute signer',
    ],
  ])('rejects %s', async (_name, mutate, message) => {
    const params = await mutate(await parameters());
    const actionContext =
      message === 'does not match the client chain'
        ? { ...context, solanaChain: { id: CHAIN_ID + 1n } as never }
        : context;
    await expect(confidentialTransfer(actionContext, params)).rejects.toThrow(message);
  });

  it.each(['distinct', 'same'])('simulates, sends, and confirms with %s owner and fee-payer signers', async (mode) => {
    const owner = await generateKeyPairSigner();
    const feePayer = mode === 'same' ? owner : await generateKeyPairSigner();
    const mint = key(2);
    const [computeSigner] = await findComputeSignerPda({ mint });
    const inputProof = proof(owner.address, computeSigner);
    const simulate = vi.fn().mockReturnValue({ send: vi.fn().mockResolvedValue({ value: { err: null } }) });
    const params = await parameters({
      owner,
      feePayer,
      mint,
      fromAccount: key(4),
      toAccount: mode === 'same' ? key(4) : key(5),
      fromBalanceValue: key(6),
      toBalanceValue: mode === 'same' ? key(6) : key(7),
      hcuBlockMeter: key(8),
      hcuTrustedAppRecord: key(9),
      ...(mode === 'same' ? {} : { denyRecords: [key(10), key(11)] }),
      inputProof,
      inputProofResult: {
        handles: inputProof.getInputHandles(),
        signatures: [SIGNATURE],
        extraData: '0x00',
      },
      rpc: {
        getLatestBlockhash: vi.fn().mockReturnValue({
          send: vi.fn().mockResolvedValue({ value: { blockhash: key(20), lastValidBlockHeight: 1_000n } }),
        }),
        simulateTransaction: simulate,
      } as unknown as SolanaConfidentialTransferParameters['rpc'],
    });

    await expect(confidentialTransfer(context, params)).resolves.toEqual(expect.any(String));
    expect(simulate).toHaveBeenCalledWith(expect.any(String), {
      commitment: 'confirmed',
      encoding: 'base64',
      sigVerify: true,
    });
    expect(sendAndConfirm).toHaveBeenCalledWith(expect.any(Object), {
      commitment: 'confirmed',
      skipPreflight: true,
    });
    const wire = simulate.mock.calls[0]![0] as string;
    const transaction = getTransactionDecoder().decode(getBase64Encoder().encode(wire));
    const compiled = getCompiledTransactionMessageDecoder().decode(transaction.messageBytes);
    const message = decompileTransactionMessage(compiled);
    expect(message.instructions).toHaveLength(2);
    expect([...message.instructions[0]!.data!]).toEqual([2, 128, 26, 6, 0]);
    if (mode === 'distinct') {
      expect(message.instructions[1]!.accounts?.slice(-2)).toEqual([
        { address: key(10), role: AccountRole.READONLY },
        { address: key(11), role: AccountRole.READONLY },
      ]);
    }
    expect(message.instructions[1]!.accounts?.[13]).toEqual({ address: key(8), role: AccountRole.WRITABLE });
    expect(message.instructions[1]!.accounts?.[14]).toEqual({ address: key(9), role: AccountRole.READONLY });
  });

  it('rejects deny records on the program self-transfer no-op path', async () => {
    const params = await parameters({
      toAccount: key(4),
      toBalanceValue: key(6),
      denyRecords: [key(10), key(11)],
    });
    await expect(confidentialTransfer(context, params)).rejects.toThrow('self-transfers cannot include deny records');
  });

  it.each(['0x44', `0x${'44'.repeat(66)}`])(
    'rejects a malformed attestation signature before RPC',
    async (signature) => {
      const getLatestBlockhash = vi.fn();
      const simulateTransaction = vi.fn();
      const defaults = await parameters({
        rpc: { getLatestBlockhash, simulateTransaction } as unknown as SolanaConfidentialTransferParameters['rpc'],
      });
      const params = {
        ...defaults,
        inputProofResult: {
          ...defaults.inputProofResult,
          signatures: [signature as never],
        },
      };

      await expect(confidentialTransfer(context, params)).rejects.toThrow('input proof signature[0] must be 65 bytes');
      expect(getLatestBlockhash).not.toHaveBeenCalled();
      expect(simulateTransaction).not.toHaveBeenCalled();
      expect(sendAndConfirm).not.toHaveBeenCalled();
    },
  );

  it('does not send a transaction whose simulation fails', async () => {
    const owner = await generateKeyPairSigner();
    const mint = key(2);
    const [computeSigner] = await findComputeSignerPda({ mint });
    const inputProof = proof(owner.address, computeSigner);
    const params = await parameters({
      owner,
      feePayer: owner,
      mint,
      inputProof,
      inputProofResult: { handles: inputProof.getInputHandles(), signatures: [SIGNATURE], extraData: '0x00' },
      rpc: {
        getLatestBlockhash: vi.fn().mockReturnValue({
          send: vi.fn().mockResolvedValue({ value: { blockhash: key(20), lastValidBlockHeight: 1_000n } }),
        }),
        simulateTransaction: vi.fn().mockReturnValue({
          send: vi.fn().mockResolvedValue({ value: { err: { InstructionError: [1, 'Custom'] } } }),
        }),
      } as unknown as SolanaConfidentialTransferParameters['rpc'],
    });

    await expect(confidentialTransfer(context, params)).rejects.toThrow('simulation failed');
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });

  it('rejects a host program that does not match the confidential-token deployment before RPC', async () => {
    const alternateHost = key(12);
    const alternateAcl = bytesToHex(base58.decode(alternateHost));
    const params = await parameters();
    await expect(confidentialTransfer({ ...context, aclProgramAddress: alternateAcl }, params)).rejects.toThrow(
      'does not match the host compiled into confidential-token',
    );
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });
});
