import { findAssociatedTokenPda, TOKEN_PROGRAM_ADDRESS } from '@solana-program/token';
import type { EncryptionBits } from '@fhevm/sdk/types';
import type { Bytes32Hex } from '@fhevm/sdk/types';
import type { SolanaInputProof } from '@fhevm/sdk/solana';
import { toSolanaZkProof } from '@fhevm/sdk/solana';
import { asBytes32Hex, asBytes65Hex, bytesToHex } from '@fhevm/sdk/base';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const sendAndConfirm = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  sendAndConfirmTransactionFactory: () => sendAndConfirm,
}));

import {
  AccountRole,
  address,
  generateKeyPairSigner,
  type Address,
  type Transaction,
  type TransactionSigner,
} from '@solana/kit';
import { base58 } from '@scure/base';

import { confidentialTransfer, type SolanaConfidentialTransferParameters } from './confidentialTransfer.js';
import { findDenyScopeRecordPda, ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, parseConfidentialTransferInstruction } from '@fhevm/confidential-token';
import { messageOf, testDemoClient } from '../../testDemoClient';
import { tokenApp } from '../internal/hostPolicy.js';
import { hcuSlots, testHostPolicy } from '../testHostPolicy.js';

const CHAIN_ID = 72057594037940281n;
const ACL = `0x${'11'.repeat(32)}` as Bytes32Hex;
const CANONICAL_ACL = asBytes32Hex(bytesToHex(base58.decode(ZAMA_HOST_PROGRAM_ADDRESS)));
const SIGNATURE = asBytes65Hex(`0x${'44'.repeat(65)}`);

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
): SolanaInputProof {
  const local = toSolanaZkProof({
    chainId: overrides.chainId ?? CHAIN_ID,
    aclContractAddress: overrides.acl ?? CANONICAL_ACL,
    contractAddress: bytesToHex(base58.decode(contract)),
    userAddress: bytesToHex(base58.decode(owner)),
    ciphertextWithZkProof: new Uint8Array([1]),
    encryptionBits: overrides.bits ?? [64],
  });
  return {
    handles: local.getInputHandles(),
    chainId: local.chainId,
    aclContractAddress: local.aclContractAddress,
    contractAddress: local.contractAddress,
    userAddress: local.userAddress,
    signatures: [SIGNATURE],
    extraData: '0x00' as never,
  };
}

async function parameters(overrides: Partial<SolanaConfidentialTransferParameters> = {}) {
  const mint = key(2);
  const owner = signer(key(1));
  const inputProof = proof(owner.address, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
  return {
    inputProof,
    inputIndex: 0,
    owner,
    mint,
    underlyingMint: key(9),
    tokenProgram: TOKEN_PROGRAM_ADDRESS,
    fromAccount: key(4),
    toAccount: key(5),
    toOwner: key(10),
    fromStore: key(6),
    toStore: key(7),
    host: testHostPolicy(false),
    ...overrides,
  } satisfies SolanaConfidentialTransferParameters;
}

const context = { solanaChain: { id: CHAIN_ID } as never, aclProgramAddress: CANONICAL_ACL };

/** A client whose fee payer never signs: for inputs rejected before any RPC call. */
const idleClient = () => testDemoClient(signer(key(3)));

describe('confidentialTransfer attestation binding', () => {
  beforeEach(() => sendAndConfirm.mockReset().mockResolvedValue(undefined));
  afterEach(() => vi.restoreAllMocks());

  it('rejects malformed submitted handles before RPC', async () => {
    const params = await parameters();
    await expect(confidentialTransfer(context, idleClient().client, {
      ...params,
      inputProof: { ...params.inputProof, handles: ['0x44' as never] },
    })).rejects.toThrow();
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
        const inputProof = proof(params.owner.address, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, {
          bits: [8],
        });
        return {
          ...params,
          inputProof,
        };
      },
      'must be euint64',
    ],
    [
      'a non-Solana chain id',
      async (params: Awaited<ReturnType<typeof parameters>>) => {
        const inputProof = proof(params.owner.address, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, {
          chainId: 12345n,
        });
        return {
          ...params,
          inputProof,
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
        const inputProof = proof(params.owner.address, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, {
          acl: ACL,
        });
        return {
          ...params,
          inputProof,
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
      'a proof bound to another program',
      async (params: Awaited<ReturnType<typeof parameters>>) => {
        const inputProof = proof(params.owner.address, key(10));
        return {
          ...params,
          inputProof,
        };
      },
      'does not match the confidential-token program',
    ],
  ])('rejects %s', async (_name, mutate, message) => {
    const params = await mutate(await parameters());
    const actionContext =
      message === 'does not match the client chain'
        ? { ...context, solanaChain: { id: CHAIN_ID + 1n } as never }
        : context;
    await expect(confidentialTransfer(actionContext, idleClient().client, params)).rejects.toThrow(message);
  });

  it.each(['distinct', 'same'])('sends one v1 FHE transaction with %s owner and fee-payer signers', async (mode) => {
    const owner = await generateKeyPairSigner();
    const feePayer = mode === 'same' ? owner : await generateKeyPairSigner();
    const { client, rpcMethods } = testDemoClient(feePayer);
    const params = await parameters({
      owner,
      mint: key(2),
      fromAccount: key(4),
      toAccount: mode === 'same' ? key(4) : key(5),
      fromStore: key(6),
      toStore: mode === 'same' ? key(6) : key(7),
      host: testHostPolicy(true, true),
      inputProof: proof(owner.address, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
    });

    await expect(confidentialTransfer(context, client, params)).resolves.toEqual(expect.any(String));
    expect(rpcMethods).toEqual(['getLatestBlockhash', 'simulateTransaction']);
    expect(sendAndConfirm).toHaveBeenCalledOnce();
    const transaction = sendAndConfirm.mock.lastCall![0] as Transaction;
    expect(Object.keys(transaction.signatures).sort()).toEqual([...new Set([owner.address, feePayer.address])].sort());
    const message = messageOf(transaction);
    expect(message.version).toBe(1);
    // open_transient_store, confidential_transfer, close_transient_store.
    expect(message.instructions).toHaveLength(3);
    const [fromAta] = await findAssociatedTokenPda({
      owner: params.owner.address,
      tokenProgram: TOKEN_PROGRAM_ADDRESS,
      mint: params.underlyingMint,
    });
    const [toAta] = await findAssociatedTokenPda({
      owner: params.toOwner,
      tokenProgram: TOKEN_PROGRAM_ADDRESS,
      mint: params.underlyingMint,
    });
    expect(message.instructions[1]!.accounts?.[3]).toEqual({
      address: params.underlyingMint,
      role: AccountRole.READONLY,
    });
    expect(message.instructions[1]!.accounts?.[4]).toEqual({ address: fromAta, role: AccountRole.READONLY });
    expect(message.instructions[1]!.accounts?.[5]).toEqual({ address: toAta, role: AccountRole.READONLY });
    // The mint's HCU accounts and deny record, except on the program's no-op self-transfer.
    const transfer = message.instructions[1]!;
    const parsed = parseConfidentialTransferInstruction({ ...transfer, accounts: transfer.accounts!, data: transfer.data! });
    const { actual, expected } = await hcuSlots(parsed.accounts, { '': tokenApp(params.mint) });
    const [mintRecord] = await findDenyScopeRecordPda(tokenApp(params.mint));
    if (mode === 'distinct') {
      expect(actual).toEqual(expected);
      expect(transfer.accounts?.find((account) => account.address === expected.hcuBlockMeter)?.role).toBe(AccountRole.WRITABLE);
      expect(transfer.accounts?.at(-1)).toEqual({ address: mintRecord, role: AccountRole.READONLY });
    } else {
      expect(Object.values(actual)).toEqual([undefined, undefined]);
      expect(Array.from(transfer.accounts ?? [], (account) => account.address)).not.toContain(mintRecord);
    }
  });

  it.each(['0x44', `0x${'44'.repeat(66)}`])(
    'rejects a malformed attestation signature before RPC',
    async (signature) => {
      const { client, rpcMethods } = idleClient();
      const defaults = await parameters();
      const params = {
        ...defaults,
        inputProof: {
          ...defaults.inputProof,
          signatures: [signature as never],
        },
      };

      await expect(confidentialTransfer(context, client, params)).rejects.toThrow('input proof signature[0] must be 65 bytes');
      expect(rpcMethods).toEqual([]);
      expect(sendAndConfirm).not.toHaveBeenCalled();
    },
  );

  it('does not send a transaction whose estimate simulation fails', async () => {
    const owner = await generateKeyPairSigner();
    const { client, failNextSimulation } = testDemoClient(owner);
    failNextSimulation({ err: { InstructionError: [1, { Custom: 6_000 }] } });
    const params = await parameters({ owner, inputProof: proof(owner.address, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS) });

    await expect(confidentialTransfer(context, client, params)).rejects.toThrow('estimate its resource limits');
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });

  it('rejects a host program that does not match the confidential-token deployment before RPC', async () => {
    const alternateHost = key(12);
    const alternateAcl = asBytes32Hex(bytesToHex(base58.decode(alternateHost)));
    const params = await parameters();
    await expect(confidentialTransfer({ ...context, aclProgramAddress: alternateAcl }, idleClient().client, params)).rejects.toThrow(
      'does not match the host compiled into confidential-token',
    );
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });
});
