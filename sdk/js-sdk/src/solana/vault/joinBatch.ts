import {
  address,
  appendTransactionMessageInstructions,
  assertIsFullySignedTransaction,
  assertIsTransactionWithBlockhashLifetime,
  assertIsTransactionWithinSizeLimit,
  createTransactionMessage,
  getBase64EncodedWireTransaction,
  getProgramDerivedAddress,
  getSignatureFromTransaction,
  pipe,
  sendAndConfirmTransactionFactory,
  setTransactionMessageComputeUnitLimit,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type Address,
  type Rpc,
  type RpcSubscriptions,
  type Signature,
  type SolanaRpcApi,
  type SolanaRpcSubscriptionsApi,
  type TransactionSigner,
} from '@solana/kit';
import { base58 } from '@scure/base';

import { hexToBytes } from '../../core/base/bytes.js';
import { assertHandleArrayEquals } from '../../core/handle/FhevmHandle.js';
import type { SolanaZkProof } from '../../core/types/zkProof-p.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { Bytes32Hex } from '../../core/types/primitives.js';
import type { SolanaSubmitInputProofResult } from '../actions/submitInputProof.js';
import { getJoinInstructionAsync } from './internal/generated/confidentialBatcher/instructions/join.js';
import { findComputeSignerPda } from '../internal/generated/confidentialToken/pdas/computeSigner.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../internal/generated/confidentialToken/programAddress.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '../internal/generated/confidentialToken/programAddress.js';
import { EVENT_AUTHORITY_SEED, findBatchAuthorityPda, tokenAccountAddress } from './internal/batcherPdas.js';

/**
 * Joins a batch with a coprocessor-attested confidential amount of the batcher's join token. This
 * is the SAME action for both directions — a deposit batcher joins with confidential underlying, a
 * redeem batcher with confidential shares — because the direction is a property of the on-chain
 * `Batcher` account, not the SDK call.
 *
 * It uses the **attested** `confidential_transfer` arm (a fresh coprocessor input proof), NOT the
 * from-value arm: the join amount is a new encrypted input the user authorizes, so the same proof
 * plumbing and binding checks as {@link confidentialTransfer} apply and are copied here.
 */
export type SolanaVaultJoinParameters = {
  readonly rpc: Rpc<SolanaRpcApi>;
  readonly rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
  readonly inputProof: SolanaZkProof;
  readonly inputProofResult: SolanaSubmitInputProofResult;
  readonly inputIndex: number;
  /** Joining user; the transfer authority over their confidential balance. */
  readonly user: TransactionSigner;
  /** Pays join-record, transfer-output, and eval ACL rent. */
  readonly payer: TransactionSigner;
  readonly batcher: Address;
  readonly batch: Address;
  /** Confidential mint the batcher joins with (`batcher.join_confidential_mint`). */
  readonly joinConfidentialMint: Address;
  /** User's stable balance lineage on the join mint. */
  readonly userBalanceValue: Address;
  /** Batch account's stable balance lineage on the join mint. */
  readonly batchBalanceValue: Address;
  /** User's stable transferred-amount lineage (the batcher eval's operand). */
  readonly userTransferredValue: Address;
  /** User's joined lineage for this batch (`pending_join_value`; see `pendingJoinLineage`). */
  readonly pendingJoinValue: Address;
  readonly hostConfig: Address;
  readonly computeUnitLimit?: number | undefined;
};

async function eventAuthority(programAddress: Address): Promise<Address> {
  return (await getProgramDerivedAddress({ programAddress, seeds: [EVENT_AUTHORITY_SEED] }))[0];
}

/** Builds, simulates, sends, and confirms one batch join. */
export async function joinBatch(
  fhevm: { readonly solanaChain: FhevmSolanaChain; readonly aclProgramAddress: Bytes32Hex },
  parameters: SolanaVaultJoinParameters,
): Promise<Signature> {
  const { inputProof, inputProofResult, inputIndex, user, joinConfidentialMint } = parameters;
  const zamaHostProgramAddress = address(base58.encode(hexToBytes(fhevm.aclProgramAddress)));
  if (zamaHostProgramAddress !== ZAMA_HOST_PROGRAM_ADDRESS) {
    throw new Error('configured ACL program does not match the host compiled into confidential-token');
  }
  const handles = inputProof.getInputHandles();
  assertHandleArrayEquals(inputProofResult.handles, handles, {
    actualName: 'input proof submission',
    expectedName: 'input proof',
  });
  if (!Number.isInteger(inputIndex) || inputIndex < 0 || inputIndex > 255 || inputIndex >= handles.length) {
    throw new Error(`inputIndex ${inputIndex} is outside the submitted proof`);
  }
  const inputHandle = handles[inputIndex];
  if (inputHandle === undefined) throw new Error(`inputIndex ${inputIndex} is outside the submitted proof`);
  if (inputProof.encryptionBits[inputIndex] !== 64) throw new Error('join amount must be euint64');
  if ((inputProof.chainId & (1n << 63n)) === 0n) throw new Error('join requires a Solana chain id');
  if (inputProof.chainId !== fhevm.solanaChain.id)
    throw new Error('input proof chain id does not match the client chain');
  if (base58.encode(hexToBytes(inputProof.aclContractAddress)) !== zamaHostProgramAddress) {
    throw new Error('input proof ACL does not match the configured Zama host program');
  }
  const [joinComputeSigner] = await findComputeSignerPda({ mint: joinConfidentialMint });
  if (base58.encode(hexToBytes(inputProof.userAddress)) !== user.address) {
    throw new Error('input proof user does not match the joining user');
  }
  if (base58.encode(hexToBytes(inputProof.contractAddress)) !== joinComputeSigner) {
    throw new Error('input proof contract does not match the join mint compute signer');
  }
  const signatures = inputProofResult.signatures.map((signature, index) => {
    const bytes = hexToBytes(signature);
    if (bytes.length !== 65) throw new Error(`input proof signature[${index}] must be 65 bytes`);
    return bytes;
  });

  const [batchAuthority] = await findBatchAuthorityPda({ batch: parameters.batch });
  const instruction = await getJoinInstructionAsync({
    user,
    payer: parameters.payer,
    batcher: parameters.batcher,
    batch: parameters.batch,
    joinConfidentialMint,
    joinComputeSigner,
    userTokenAccount: await tokenAccountAddress(joinConfidentialMint, user.address),
    batchJoinTokenAccount: await tokenAccountAddress(joinConfidentialMint, batchAuthority),
    userBalanceValue: parameters.userBalanceValue,
    batchBalanceValue: parameters.batchBalanceValue,
    userTransferredValue: parameters.userTransferredValue,
    pendingJoinValue: parameters.pendingJoinValue,
    zamaEventAuthority: await eventAuthority(zamaHostProgramAddress),
    hostConfig: parameters.hostConfig,
    confidentialTokenEventAuthority: await eventAuthority(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
    inputHandle: hexToBytes(inputHandle.bytes32Hex),
    ctHandles: handles.map((handle) => hexToBytes(handle.bytes32Hex)),
    handleIndex: inputIndex,
    userAddress: hexToBytes(inputProof.userAddress),
    contractAddress: hexToBytes(inputProof.contractAddress),
    contractChainId: inputProof.chainId,
    extraData: hexToBytes(inputProofResult.extraData),
    signatures,
  });

  const { value: latestBlockhash } = await parameters.rpc.getLatestBlockhash({ commitment: 'confirmed' }).send();
  const message = pipe(
    createTransactionMessage({ version: 0 }),
    (m) => setTransactionMessageFeePayerSigner(parameters.payer, m),
    (m) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, m),
    (m) => setTransactionMessageComputeUnitLimit(parameters.computeUnitLimit ?? 400_000, m),
    (m) => appendTransactionMessageInstructions([instruction], m),
  );
  const transaction = await signTransactionMessageWithSigners(message);
  assertIsFullySignedTransaction(transaction);
  assertIsTransactionWithBlockhashLifetime(transaction);
  assertIsTransactionWithinSizeLimit(transaction);
  const wireTransaction = getBase64EncodedWireTransaction(transaction);
  const simulation = await parameters.rpc
    .simulateTransaction(wireTransaction, { commitment: 'confirmed', encoding: 'base64', sigVerify: true })
    .send();
  if (simulation.value.err !== null) {
    const err = JSON.stringify(simulation.value.err, (_key, value: unknown) =>
      typeof value === 'bigint' ? value.toString() : value,
    );
    const logs = simulation.value.logs?.join('\n') ?? '';
    throw new Error(logs.length > 0 ? `join simulation failed: ${err}\n${logs}` : `join simulation failed: ${err}`);
  }
  await sendAndConfirmTransactionFactory({ rpc: parameters.rpc, rpcSubscriptions: parameters.rpcSubscriptions })(
    transaction,
    {
      commitment: 'confirmed',
      skipPreflight: true,
    },
  );
  return getSignatureFromTransaction(transaction);
}
