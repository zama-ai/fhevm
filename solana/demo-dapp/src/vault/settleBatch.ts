import {
  getBase64EncodedWireTransaction,
  getProgramDerivedAddress,
  getSignatureFromTransaction,
  sendAndConfirmTransactionFactory,
  type Address,
  type Rpc,
  type RpcSubscriptions,
  type Signature,
  type SolanaRpcApi,
  type SolanaRpcSubscriptionsApi,
  type TransactionSigner,
} from '@solana/kit';
import { base58 } from '@scure/base';

import { bytesToHex, hexToBytes } from '@sdk-src/core/base/bytes.js';
import type { FhevmSolanaChain } from '@sdk-src/core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '@sdk-src/core/types/coreFhevmRuntime.js';
import type { RelayerPublicDecryptOptions } from '@sdk-src/core/types/relayer.js';
import { publicDecryptCertificate } from '@sdk-src/solana/actions/publicDecryptCertificate.js';
import { buildPublicLeafProof, type MmrProof, type SolanaEncryptedValueAccountEvent } from '@sdk-src/solana/proof.js';
import {
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from './internal/generated/confidentialToken/programAddress.js';
import { getSettleInstructionAsync } from './internal/generated/confidentialBatcher/instructions/settle.js';
import { fetchBatch } from './internal/generated/confidentialBatcher/accounts/batch.js';
import { EVENT_AUTHORITY_SEED } from './internal/batcherPdas.js';
import { settleTotalFromCleartext } from './internal/cleartext.js';
import { buildAndSignSettleTransaction } from './internal/settleMessage.js';
import {
  deriveBatchAddresses,
  deriveSettleAccounts,
  settleAccountsToLookupTableAddresses,
  type BatchAddresses,
  type VaultDemoRoots,
} from './derive.js';
import { getCurrentBatch, getEncryptedValueState } from './reads.js';

const ZERO_HANDLE = new Uint8Array(32);

/** What `settleBatch` needs beyond the certificate phase and the keeper signer. */
export type SolanaVaultSettleOptions = {
  readonly rpc: Rpc<SolanaRpcApi>;
  readonly rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
  /** Decrypt runtime (auth) for the certificate phase. */
  readonly runtime: FhevmRuntime;
  /** The batcher's demo topology; every settle account is derived from these. */
  readonly roots: VaultDemoRoots;
  /** Which batch to settle; defaults to the batcher's current (most-recently-opened) batch. */
  readonly batchIndex?: bigint | undefined;
  /** 32-byte context id the certificate commits to (the host's current KMS context). */
  readonly contextId: Uint8Array;
  /**
   * The settle Address Lookup Table's address. It is created OFF-CHAIN at `open_batch` (the batcher
   * program creates no ALT — verified against `open_batch.rs`), so it is neither a PDA nor stored on
   * `Batcher`/`Batch`; it must be supplied. Its contents are derived, not supplied
   * ({@link settleAccountsToLookupTableAddresses}).
   */
  readonly lookupTableAddress: Address;
  readonly authorityFundingLamports: bigint;
  readonly computeUnitLimit?: number | undefined;
  /** Bounds and observes the relayer/KMS certificate request independently of the on-chain send. */
  readonly certificateOptions?: RelayerPublicDecryptOptions | undefined;
};

/**
 * The burned-amount value's whole leaf history, as the batcher wrote it.
 *
 * `dispatch` burns the batch's entire join balance once through `confidential_burn_from_value`,
 * which creates the `burned_amount` value publicly decryptable and allowed to the token account's
 * owner — the batch authority (`PersistentOutput::new_public(..., [owner])`). The host seals the
 * allow leaves before the public leaf (`fhe_execute.rs`), so the account holds exactly these two
 * leaves, and the public leaf is the second. The peaks this list implies are checked against the
 * live account before anything is sent, so a history the batcher did not write fails here rather
 * than on-chain.
 */
export function burnedAmountLeafHistory(
  burnedTotalHandle: Uint8Array,
  batchAuthority: Address,
): { readonly events: readonly SolanaEncryptedValueAccountEvent[]; readonly publicLeafIndex: bigint } {
  return {
    events: [
      { kind: 'allowed', handle: burnedTotalHandle, key: base58.decode(batchAuthority) },
      { kind: 'markedPublic', handle: burnedTotalHandle },
    ],
    publicLeafIndex: 1n,
  };
}

/**
 * The burned handle's public-leaf inclusion proof against the account's live peaks, from the
 * history in {@link burnedAmountLeafHistory}. Throws if the live account disagrees with it.
 */
export function buildBurnedAmountPublicProof(
  encryptedValueAccount: Address,
  live: { readonly leafCount: bigint; readonly peaks: readonly Uint8Array[] },
  burnedTotalHandle: Uint8Array,
  batchAuthority: Address,
): MmrProof {
  const history = burnedAmountLeafHistory(burnedTotalHandle, batchAuthority);
  return buildPublicLeafProof(base58.decode(encryptedValueAccount), live, history.events, history.publicLeafIndex);
}

/**
 * Settles a dispatched batch. The caller supplies only the batcher's roots (plus infra handles and
 * the off-chain ALT address); settle resolves everything batch-specific itself:
 *
 * - the batch (its current batch, or `batchIndex` when pinned) and its created-public burned handle,
 *   read from `Batch`;
 * - the full settle account set, derived from the roots, the batch, and the burned handle; and
 * - the burned value's live MMR peaks and leaf count, read from its `EncryptedValue` account.
 *
 * The KMS burn certificate is requested from the relayer for `(handle, account)` alone: the
 * Connector reads the account and fetches the public leaf's proof from the coprocessors itself
 * (RFC 035). The on-chain `settle` still verifies an inclusion proof, which this builds locally from
 * the two-leaf history the batcher wrote ({@link buildBurnedAmountPublicProof}). The certified
 * cleartext is a 32-byte `uint256`; settle's on-chain argument is a `u64`, so its low 8 bytes are
 * taken big-endian and its high 24 asserted zero ({@link settleTotalFromCleartext}). The instruction
 * is sent as an ALT-aware v0 transaction — 33 accounts overflow a legacy packet — keeping only the
 * fee payer (and program/event authorities outside the ALT address list) static.
 */
export async function settleBatch(
  chain: FhevmSolanaChain,
  keeper: TransactionSigner,
  options: SolanaVaultSettleOptions,
): Promise<Signature> {
  const { rpc, roots } = options;

  // Resolve the batch and its created-public burned handle from chain state.
  let addresses: BatchAddresses;
  let burnedTotalHandle: Uint8Array;
  if (options.batchIndex !== undefined) {
    addresses = await deriveBatchAddresses(roots, options.batchIndex);
    const batch = await fetchBatch(rpc, addresses.batch);
    burnedTotalHandle = new Uint8Array(batch.data.burnedTotalHandle);
  } else {
    const current = await getCurrentBatch(rpc, roots);
    addresses = current.addresses;
    burnedTotalHandle = new Uint8Array(current.state.burnedTotalHandle);
  }
  if (burnedTotalHandle.every((byte, i) => byte === ZERO_HANDLE[i])) {
    throw new Error(`batch ${addresses.batch} has no burned total handle yet; dispatch it before settling`);
  }

  const accounts = await deriveSettleAccounts(roots, addresses);

  // Read the burned value's live MMR state and build the public leaf's proof against it.
  const encryptedValueAccount = await getEncryptedValueState(rpc, accounts.batchBurnedAmountValue);
  const inclusionProof = buildBurnedAmountPublicProof(
    accounts.batchBurnedAmountValue,
    encryptedValueAccount,
    burnedTotalHandle,
    addresses.batchAuthority,
  );

  // The KMS burn certificate. The relayer request names the handle and the account, nothing else.
  const claim = await publicDecryptCertificate(
    { chain, runtime: options.runtime },
    {
      handle: bytesToHex(burnedTotalHandle),
      contextId: options.contextId,
      encryptedValueAccount: base58.decode(accounts.batchBurnedAmountValue),
      options: options.certificateOptions,
    },
  );

  const cleartextTotal = settleTotalFromCleartext(hexToBytes(claim.abiEncodedCleartext));
  const signatures = claim.signatures.map((signature, index) => {
    const bytes = hexToBytes(signature);
    if (bytes.length !== 65) throw new Error(`certificate signature[${index}] must be 65 bytes`);
    return bytes;
  });

  // The ALT holds every settle account except the fee payer (and program/event authorities the
  // builder resolves separately). `pendingBurn` seeds on the join mint and the batch's join token
  // account, both known at open_batch, so it is provisioned with the rest of the table.
  const lookupTableAddresses = settleAccountsToLookupTableAddresses(accounts);
  if (!lookupTableAddresses.includes(accounts.pendingBurn)) {
    throw new Error(
      `settle lookup table must contain pending_burn (${accounts.pendingBurn}); it is known at open_batch`,
    );
  }

  const [zamaEventAuthority] = await getProgramDerivedAddress({
    programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
    seeds: [EVENT_AUTHORITY_SEED],
  });
  const [confidentialTokenEventAuthority] = await getProgramDerivedAddress({
    programAddress: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    seeds: [EVENT_AUTHORITY_SEED],
  });

  const settleInstruction = await getSettleInstructionAsync({
    payer: keeper,
    batcher: accounts.batcher,
    batch: accounts.batch,
    joinConfidentialMint: accounts.joinConfidentialMint,
    batchJoinTokenAccount: accounts.batchJoinTokenAccount,
    joinUnderlyingMint: accounts.joinUnderlyingMint,
    joinMintVaultUnderlying: accounts.joinMintVaultUnderlying,
    joinMintVaultAuthority: accounts.joinMintVaultAuthority,
    batchBurnedAmountValue: accounts.batchBurnedAmountValue,
    pendingBurn: accounts.pendingBurn,
    hostConfig: accounts.hostConfig,
    kmsContext: accounts.kmsContext,
    vault: accounts.vault,
    vaultAuthority: accounts.vaultAuthority,
    vaultTokenAccount: accounts.vaultTokenAccount,
    payoutConfidentialMint: accounts.payoutConfidentialMint,
    payoutUnderlyingMint: accounts.payoutUnderlyingMint,
    batchPayoutTokenAccount: accounts.batchPayoutTokenAccount,
    payoutMintVaultUnderlying: accounts.payoutMintVaultUnderlying,
    payoutMintVaultAuthority: accounts.payoutMintVaultAuthority,
    payoutTotalSupplyAuthority: accounts.payoutTotalSupplyAuthority,
    batchPayoutBalanceValue: accounts.batchPayoutBalanceValue,
    payoutTotalSupplyValue: accounts.payoutTotalSupplyValue,
    zamaEventAuthority,
    confidentialTokenEventAuthority,
    cleartextTotal,
    signatures,
    extraData: hexToBytes(claim.extraData),
    leafIndex: inclusionProof.leafIndex,
    siblings: [...inclusionProof.siblings],
    authorityFundingLamports: options.authorityFundingLamports,
  });

  const { value: latestBlockhash } = await rpc.getLatestBlockhash({ commitment: 'confirmed' }).send();
  const transaction = await buildAndSignSettleTransaction({
    settleInstruction,
    feePayer: keeper,
    latestBlockhash,
    computeUnitLimit: options.computeUnitLimit ?? 1_000_000,
    lookupTableAddress: options.lookupTableAddress,
    lookupTableAddresses,
  });

  const wireTransaction = getBase64EncodedWireTransaction(transaction);
  const simulation = await rpc
    .simulateTransaction(wireTransaction, { commitment: 'confirmed', encoding: 'base64', sigVerify: true })
    .send();
  if (simulation.value.err !== null) {
    const err = JSON.stringify(simulation.value.err, (_key, value: unknown) =>
      typeof value === 'bigint' ? value.toString() : value,
    );
    const logs = simulation.value.logs?.join('\n') ?? '';
    throw new Error(logs.length > 0 ? `settle simulation failed: ${err}\n${logs}` : `settle simulation failed: ${err}`);
  }
  await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions: options.rpcSubscriptions })(transaction, {
    commitment: 'confirmed',
    skipPreflight: true,
  });
  return getSignatureFromTransaction(transaction);
}
