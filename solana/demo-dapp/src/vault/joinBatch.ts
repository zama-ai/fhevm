import { findAssociatedTokenPda } from '@solana-program/token';
import { findEventAuthorityPda } from '@fhevm/solana-zama-host';
import { INSTRUCTIONS_SYSVAR_ADDRESS, prepareTransientStore } from '@fhevm/sdk/solana';
import {
  address,
  getSignatureFromTransaction,
  type Address,
  type Blockhash,
  type Signature,
  type TransactionSigner,
} from '@solana/kit';
import { base58 } from '@scure/base';
import { hexToBytes } from '@fhevm/sdk/base';
import { bytes32HexToHandle, isSolanaHostChainId } from '@fhevm/sdk/solana';
import type { FhevmSolanaChain } from '@fhevm/sdk/solana';
import type { Bytes32Hex } from '@fhevm/sdk/types';
import type { SolanaInputProof } from '@fhevm/sdk/solana';
import {
  getJoinInstructionAsync,
  type JoinAsyncInput,
} from './internal/generated/confidentialBatcher/instructions/join.js';
import { batchApp, tokenApp, withDenyRecords, type DenyListParameters } from './internal/denyRecords.js';
import { findBatchAuthorityPda } from './internal/generated/confidentialBatcher/pdas/index.js';
import { joinStoreAddress, tokenStoreAddress } from './internal/encryptedStores.js';
import { findTokenAccountPda, ZAMA_HOST_PROGRAM_ADDRESS, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import type { DemoClient } from '../demoClient';

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
export type SolanaVaultJoinParameters = Pick<
  JoinAsyncInput,
  'joinMintHcuBlockMeter' | 'joinMintHcuTrustedAppRecord' | 'batchHcuBlockMeter' | 'batchHcuTrustedAppRecord'
> &
  DenyListParameters & {
  readonly inputProof: SolanaInputProof;

  readonly inputIndex: number;
  /** Joining user; the transfer authority over their confidential balance. */
  readonly user: TransactionSigner;
  readonly batcher: Address;
  readonly batch: Address;
  /** Confidential mint the batcher joins with (`batcher.join_confidential_mint`). */
  readonly joinConfidentialMint: Address;
  /** SPL mint wrapped by `joinConfidentialMint`. Freeze checks the join owners' ATAs on this mint. */
  readonly joinUnderlyingMint: Address;
  /** Token program that owns `joinUnderlyingMint` (`Tokenkeg` or Token-2022). */
  readonly tokenProgram: Address;
  /** Called after signing (which simulates first) and immediately before submission, for persistent recovery journals. */
  readonly onTransactionSigned?:
    | ((transaction: {
        readonly signature: Signature;
        readonly blockhash: Blockhash;
        readonly lastValidBlockHeight: bigint;
      }) => void | Promise<void>)
    | undefined;
};

/** Builds, signs, sends, and confirms one batch join; `client.payer` pays JoinRecord growth and transientStore rent. */
export async function joinBatch(
  fhevm: { readonly solanaChain: FhevmSolanaChain; readonly aclProgramAddress: Bytes32Hex },
  client: DemoClient,
  parameters: SolanaVaultJoinParameters,
): Promise<Signature> {
  const { inputProof, inputIndex, user, joinConfidentialMint } = parameters;
  const zamaHostProgramAddress = address(base58.encode(hexToBytes(fhevm.aclProgramAddress)));
  if (zamaHostProgramAddress !== ZAMA_HOST_PROGRAM_ADDRESS) {
    throw new Error('configured ACL program does not match the host compiled into confidential-token');
  }
  const handles = inputProof.handles.map((handle) => bytes32HexToHandle(handle.bytes32Hex));
  if (!Number.isInteger(inputIndex) || inputIndex < 0 || inputIndex > 255 || inputIndex >= handles.length) {
    throw new Error(`inputIndex ${inputIndex} is outside the submitted proof`);
  }
  const inputHandle = handles[inputIndex];
  if (inputHandle === undefined) throw new Error(`inputIndex ${inputIndex} is outside the submitted proof`);
  if (inputHandle.fheType !== 'euint64') throw new Error('join amount must be euint64');
  if (!isSolanaHostChainId(inputProof.chainId)) throw new Error('join requires a Solana chain id');
  if (inputProof.chainId !== fhevm.solanaChain.id)
    throw new Error('input proof chain id does not match the client chain');
  if (base58.encode(hexToBytes(inputProof.aclContractAddress)) !== zamaHostProgramAddress) {
    throw new Error('input proof ACL does not match the configured Zama host program');
  }
  if (base58.encode(hexToBytes(inputProof.userAddress)) !== user.address) {
    throw new Error('input proof user does not match the joining user');
  }
  // The token program re-checks this binding in-execution (`assert_amount_attestation_binding`).
  if (base58.encode(hexToBytes(inputProof.contractAddress)) !== CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS) {
    throw new Error('input proof contract does not match the confidential-token program');
  }
  const signatures = inputProof.signatures.map((signature, index) => {
    const bytes = hexToBytes(signature);
    if (bytes.length !== 65) throw new Error(`input proof signature[${index}] must be 65 bytes`);
    return bytes;
  });

  const [batchAuthority] = await findBatchAuthorityPda({ batch: parameters.batch });
  const userTokenAccount = (await findTokenAccountPda({ mint: joinConfidentialMint, owner: user.address }))[0];
  const batchJoinTokenAccount = (await findTokenAccountPda({ mint: joinConfidentialMint, owner: batchAuthority }))[0];
  const joinStore = await joinStoreAddress(parameters.batch, user.address);
  const transientStore = await prepareTransientStore({ payer: client.payer, host: zamaHostProgramAddress });
  const joinInstruction = await getJoinInstructionAsync({
    user,
    payer: client.payer,
    batcher: parameters.batcher,
    batch: parameters.batch,
    joinConfidentialMint,
    joinUnderlyingMint: parameters.joinUnderlyingMint,
    userAta: (await findAssociatedTokenPda({
      owner: user.address,
      tokenProgram: parameters.tokenProgram,
      mint: parameters.joinUnderlyingMint,
    }))[0],
    batchAuthorityAta: (await findAssociatedTokenPda({
      owner: batchAuthority,
      tokenProgram: parameters.tokenProgram,
      mint: parameters.joinUnderlyingMint,
    }))[0],
    userTokenAccount,
    batchJoinTokenAccount,
    userBalanceStore: await tokenStoreAddress(joinConfidentialMint, userTokenAccount),
    batchBalanceStore: await tokenStoreAddress(joinConfidentialMint, batchJoinTokenAccount),
    joinStore,
    transientStore: transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
    confidentialTokenEventAuthority: (await findEventAuthorityPda({ programAddress: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS }))[0],
    inputHandle: hexToBytes(inputHandle.bytes32Hex),
    ctHandles: handles.map((handle) => hexToBytes(handle.bytes32Hex)),
    handleIndex: inputIndex,
    userAddress: hexToBytes(inputProof.userAddress),
    contractAddress: hexToBytes(inputProof.contractAddress),
    contractChainId: inputProof.chainId,
    extraData: hexToBytes(inputProof.extraData),
    signatures,
    joinMintHcuBlockMeter: parameters.joinMintHcuBlockMeter,
    joinMintHcuTrustedAppRecord: parameters.joinMintHcuTrustedAppRecord,
    batchHcuBlockMeter: parameters.batchHcuBlockMeter,
    batchHcuTrustedAppRecord: parameters.batchHcuTrustedAppRecord,
  });
  const instruction = await withDenyRecords(joinInstruction, parameters.denyListEnabled, [
    tokenApp(joinConfidentialMint),
    batchApp(parameters.batch),
  ]);

  const signed = await client.signFheTransaction(transientStore, [instruction]);
  const { message, transaction } = signed.context;
  const signature = getSignatureFromTransaction(transaction);
  await parameters.onTransactionSigned?.({
    signature,
    blockhash: message.lifetimeConstraint.blockhash,
    lastValidBlockHeight: message.lifetimeConstraint.lastValidBlockHeight,
  });
  await client.sendSignedTransaction(transaction);
  return signature;
}
