import { TOKEN_PROGRAM_ADDRESS } from '@solana-program/token';
import type { EncryptionBits } from '@fhevm/sdk/types';
import type { Bytes32Hex } from '@fhevm/sdk/types';
import type { SolanaInputProof } from '@fhevm/sdk/solana';
import { toSolanaZkProof } from '@fhevm/sdk/solana';
import { bytesToHex } from '@fhevm/sdk/base';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const sendAndConfirm = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  sendAndConfirmTransactionFactory: () => sendAndConfirm,
}));
import {
  address,
  generateKeyPairSigner,
  getSignatureFromTransaction,
  getTransactionMessageLoadedAccountsDataSizeLimit,
  isSolanaError,
  SOLANA_ERROR__FAILED_TO_SIGN_TRANSACTION,
  SOLANA_ERROR__TRANSACTION__FAILED_WHEN_SIMULATING_TO_ESTIMATE_RESOURCE_LIMITS,
  type Address,
  type Transaction,
  type TransactionPartialSigner,
  type TransactionSigner,
} from '@solana/kit';
import { base58 } from '@scure/base';

import { joinBatch, type SolanaVaultJoinParameters } from './joinBatch.js';
import { getJoinInstructionDataDecoder } from './internal/generated/confidentialBatcher/instructions/join.js';
import { findDenyScopeRecordPda, ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { LOADED_ACCOUNTS_DATA_SIZE_LIMIT } from '@fhevm/solana-zama-host/client';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import { encodedSize, TEST_BLOCKHASH, messageOf, testDemoClient } from '../testDemoClient';

const CHAIN_ID = 72057594037940281n;
const CANONICAL_ACL = bytesToHex(base58.decode(ZAMA_HOST_PROGRAM_ADDRESS));
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
  overrides: { acl?: Bytes32Hex; chainId?: bigint; bits?: readonly EncryptionBits[]; signatures?: number } = {},
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
    signatures: Array.from({ length: overrides.signatures ?? 1 }, () => SIGNATURE as never),
    extraData: '0x00' as never,
  };
}

async function parameters(overrides: Partial<SolanaVaultJoinParameters> = {}): Promise<SolanaVaultJoinParameters> {
  const user = signer(key(1));
  return {
    inputProof: proof(user.address, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
    inputIndex: 0,
    user,
    batcher: key(4),
    batch: key(5),
    joinConfidentialMint: key(2),
    joinUnderlyingMint: key(11),
    tokenProgram: TOKEN_PROGRAM_ADDRESS,
    ...overrides,
  } satisfies SolanaVaultJoinParameters;
}

const context = { solanaChain: { id: CHAIN_ID } as never, aclProgramAddress: CANONICAL_ACL as never };

/** A user who also pays and signs for real, with the demo client over a scripted RPC. */
async function sendable(overrides: Partial<SolanaVaultJoinParameters> = {}, signatures = 1) {
  const user = await generateKeyPairSigner();
  const inputProof = proof(user.address, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, { signatures });
  return { inputProof, ...testDemoClient(user), params: await parameters({ user, inputProof, ...overrides }) };
}

/** The last transaction handed to Kit's send-and-confirm. */
const sent = (): Transaction => sendAndConfirm.mock.lastCall![0] as Transaction;


describe('joinBatch (attested arm)', () => {
  beforeEach(() => sendAndConfirm.mockReset().mockResolvedValue(undefined));
  afterEach(() => vi.restoreAllMocks());

  it('journals the signed transaction before sending it, and encodes the attestation into the join', async () => {
    let finishJournal!: () => void;
    const journal = new Promise<void>((resolve) => {
      finishJournal = resolve;
    });
    const onTransactionSigned = vi.fn(() => journal);
    const { client, inputProof, params, rpcMethods } = await sendable({ onTransactionSigned });

    const pending = joinBatch(context, client, params);
    await vi.waitFor(() => expect(onTransactionSigned).toHaveBeenCalledOnce());
    expect(sendAndConfirm).not.toHaveBeenCalled();
    finishJournal();
    const signature = await pending;
    expect(sendAndConfirm).toHaveBeenCalledOnce();
    const transaction = sent();
    // Kit's own preflight is skipped because the estimate already simulated; the wait is at finalized (DD-070).
    expect(sendAndConfirm).toHaveBeenCalledWith(transaction, { commitment: 'finalized', skipPreflight: true });
    expect(getSignatureFromTransaction(transaction)).toBe(signature);
    expect(onTransactionSigned).toHaveBeenCalledWith({ signature, blockhash: TEST_BLOCKHASH, lastValidBlockHeight: 1_000n });
    // One simulation, Kit's resource estimate, before signing.
    expect(rpcMethods).toEqual(['getLatestBlockhash', 'simulateTransaction']);

    // open_transient_store, join, close_transient_store. v1 carries the compute limit in the
    // message, so there is no compute-budget instruction.
    const message = messageOf(transaction);
    expect(message.version).toBe(1);
    // Not the simulated 100,000 bytes: a join balance store that grows before the join lands must still fit.
    expect(getTransactionMessageLoadedAccountsDataSizeLimit(message)).toBe(LOADED_ACCOUNTS_DATA_SIZE_LIMIT);
    expect([...message.instructions].map((instruction) => instruction.programAddress)).toEqual([
      ZAMA_HOST_PROGRAM_ADDRESS,
      CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
      ZAMA_HOST_PROGRAM_ADDRESS,
    ]);
    const data = getJoinInstructionDataDecoder().decode(message.instructions[1]!.data!);
    expect(data.handleIndex).toBe(0);
    expect(data.contractChainId).toBe(CHAIN_ID);
    expect(Array.from(data.inputHandle)).toEqual(Array.from(inputProof.handles[0]!.bytes32));
    expect(data.signatures).toHaveLength(1);
  });

  it('appends, under the deny list, the join mint then the batch deny record', async () => {
    const submittedJoinAccounts = async (denyListEnabled: boolean): Promise<Address[]> => {
      const { client, params } = await sendable({ denyListEnabled });
      await joinBatch(context, client, params);
      const join = messageOf(sent()).instructions[1]!;
      return Array.from(join.accounts ?? [], (account) => account.address);
    };
    const plain = await submittedJoinAccounts(false);
    const params = await parameters();
    const [joinMintRecord] = await findDenyScopeRecordPda({ appProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, scope: params.joinConfidentialMint });
    const [batchRecord] = await findDenyScopeRecordPda({ appProgram: CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, scope: params.batch });
    // The attested transfer runs as the join mint; the contribution runs as the batch.
    expect((await submittedJoinAccounts(true)).slice(plain.length)).toEqual([joinMintRecord, batchRecord]);
  });

  // The largest join: the host's maximum coprocessor threshold (MAX_COPROCESSOR_SIGNERS = 8) with
  // the deny records and both HCU witnesses of each app. A v1 transaction is at most 4,096 bytes
  // and 64 account keys (solana-message v1::MAX_TRANSACTION_SIZE and MAX_ADDRESSES).
  it('fits one v1 transaction at the maximum coprocessor threshold with every witness', async () => {
    const { client, params } = await sendable(
      {
        denyListEnabled: true,
        joinMintHcuBlockMeter: key(30),
        joinMintHcuTrustedAppRecord: key(31),
        batchHcuBlockMeter: key(32),
        batchHcuTrustedAppRecord: key(33),
      },
      8,
    );
    await joinBatch(context, client, params);
    const size = encodedSize(sent());
    expect(size.version).toBe(1);
    expect(size.bytes).toBeLessThanOrEqual(4096);
    expect(size.addresses).toBeLessThanOrEqual(64);
  });

  it('does not submit when persistent transaction journaling fails', async () => {
    const { client, params } = await sendable({
      onTransactionSigned: async () => {
        throw new Error('journal unavailable');
      },
    });
    await expect(joinBatch(context, client, params)).rejects.toThrow('journal unavailable');
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });

  it('neither signs, journals nor sends when the estimate simulation fails', async () => {
    const user = (await generateKeyPairSigner()) as TransactionPartialSigner;
    const signTransactions = vi.fn(user.signTransactions.bind(user));
    const tracked = { address: user.address, signTransactions } as TransactionSigner;
    const onTransactionSigned = vi.fn();
    const { client, failNextSimulation } = testDemoClient(tracked);
    failNextSimulation({ err: { InstructionError: [1, { Custom: 6_001 }] }, logs: ['Program log: rejected'] });
    const params = await parameters({
      user: tracked,
      inputProof: proof(tracked.address, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
      onTransactionSigned,
    });

    const error = await joinBatch(context, client, params).catch((caught: unknown) => caught);
    expect(isSolanaError(error, SOLANA_ERROR__FAILED_TO_SIGN_TRANSACTION)).toBe(true);
    const cause = (error as Error).cause;
    expect(isSolanaError(cause, SOLANA_ERROR__TRANSACTION__FAILED_WHEN_SIMULATING_TO_ESTIMATE_RESOURCE_LIMITS)).toBe(true);
    expect(cause).toMatchObject({ context: { logs: ['Program log: rejected'] } });
    expect(signTransactions).not.toHaveBeenCalled();
    expect(onTransactionSigned).not.toHaveBeenCalled();
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });

  it.each([
    [
      'a non-u64 input',
      async () => {
        const p = await parameters();
        return { ...p, inputProof: proof(p.user.address, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, { bits: [8] }) };
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
        return { ...p, inputProof: { ...p.inputProof, signatures: [`0x44` as never] } };
      },
      'must be 65 bytes',
    ],
  ])('rejects %s before any RPC call', async (_name, mutate, message) => {
    const { client, rpcMethods } = testDemoClient(signer(key(3)));
    await expect(joinBatch(context, client, await mutate())).rejects.toThrow(message);
    expect(rpcMethods).toEqual([]);
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });
});
