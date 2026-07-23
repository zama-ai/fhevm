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

import type { Bytes32 } from '../../core/types/primitives.js';
import { bytesToHex, hexToBytes } from '../../core/base/bytes.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { publicDecryptCertificate } from '../actions/publicDecryptCertificate.js';
import {
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from '../internal/generated/confidentialToken/programAddress.js';
import { getSettleInstructionAsync } from './internal/generated/confidentialBatcher/instructions/settle.js';
import { fetchBatch } from './internal/generated/confidentialBatcher/accounts/batch.js';
import { EVENT_AUTHORITY_SEED, burnedAmountLineage } from './internal/batcherPdas.js';
import { fetchSolanaPublicDecryptProof, type SolanaProofServiceConfig } from './internal/proofService.js';
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

/** What `settleBatch` still needs beyond the certificate/proof legs and the keeper signer. */
export type SolanaVaultSettleOptions = {
  readonly rpc: Rpc<SolanaRpcApi>;
  readonly rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
  /** Decrypt runtime (auth) for the certificate leg. */
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
};

/**
 * Settles a dispatched batch. The caller supplies only the batcher's roots (plus infra handles and
 * the off-chain ALT address); settle resolves everything batch-specific itself:
 *
 * - the batch (its current batch, or `batchIndex` when pinned) and its born-public burned handle,
 *   read from `Batch`;
 * - the full settle account set, derived from the roots, the batch, and the burned handle; and
 * - the burned lineage's live MMR peaks and leaf count, read from its `EncryptedValue` account.
 *
 * Two off-chain legs then feed the on-chain instruction: the burned lineage's MMR inclusion proof
 * from the solana-proof-service (verified against the live peaks), and the KMS burn certificate from
 * the relayer, requested with that proof. The certified cleartext is a 32-byte `uint256`; settle's
 * on-chain argument is a `u64`, so its low 8 bytes are taken big-endian and its high 24 asserted zero
 * ({@link settleTotalFromCleartext}). The instruction is sent as an ALT-aware v0 transaction — 34
 * accounts overflow a legacy packet — keeping the fee payer and `redemption_record` static.
 */
export async function settleBatch(
  chain: FhevmSolanaChain,
  proofConfig: SolanaProofServiceConfig,
  keeper: TransactionSigner,
  options: SolanaVaultSettleOptions,
): Promise<Signature> {
  const { rpc, roots } = options;

  // Resolve the batch and its born-public burned handle from chain state.
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

  const accounts = await deriveSettleAccounts(roots, addresses, burnedTotalHandle as Bytes32);

  // The burned lineage carries the cert's ACL value key; cross-check its derived PDA against the
  // settle account so a roots/derivation mismatch fails here, not at on-chain verify.
  const burned = await burnedAmountLineage(roots.joinConfidentialMint, addresses.batchJoinTokenAccount);
  if (burned.encryptedValueAddress !== accounts.batchBurnedAmountValue) {
    throw new Error(
      `derived burned-amount lineage ${burned.encryptedValueAddress} disagrees with the settle account ${accounts.batchBurnedAmountValue}`,
    );
  }

  // Read the burned lineage's live MMR state (the peaks the proof is verified against).
  const lineage = await getEncryptedValueState(rpc, accounts.batchBurnedAmountValue);

  // Leg 1: the settle burns to a born-public handle, so its proof is a public-decrypt leaf. The
  // service resolves the leaf from (encryptedValue, burnedTotalHandle); the SDK never supplies a
  // leaf index, and the resolved index comes back on the proof.
  const proof = await fetchSolanaPublicDecryptProof(proofConfig, burned.encryptedValueAddress, burnedTotalHandle);
  if (proof.leafCount !== lineage.leafCount) {
    throw new Error(
      `proof-service leaf count ${proof.leafCount} does not match the on-chain lineage leaf count ${lineage.leafCount}`,
    );
  }

  // Leg 2: KMS burn certificate, verified against the live peaks with the leg-1 proof.
  const claim = await publicDecryptCertificate(
    { chain, runtime: options.runtime },
    {
      handle: bytesToHex(burnedTotalHandle),
      contextId: options.contextId,
      aclValueKey: burned.aclValueKey,
      proofSlot: proof.leafCount,
      encryptedValueAccount: base58.decode(burned.encryptedValueAddress),
      peaks: lineage.peaks,
      leafCount: lineage.leafCount,
      mmrProofBytes: proof.mmrProofBytes,
    },
  );

  const cleartextTotal = settleTotalFromCleartext(hexToBytes(claim.abiEncodedCleartext));
  const signatures = claim.signatures.map((signature, index) => {
    const bytes = hexToBytes(signature);
    if (bytes.length !== 65) throw new Error(`certificate signature[${index}] must be 65 bytes`);
    return bytes;
  });

  // The ALT holds every settle account except the fee payer and redemption_record. redemption_record
  // is seeded by the burned handle and only exists after dispatch, so it cannot live in a table
  // frozen at open_batch — assert it against the derived set rather than trusting the caller.
  const lookupTableAddresses = settleAccountsToLookupTableAddresses(accounts);
  if (lookupTableAddresses.includes(accounts.redemptionRecord)) {
    throw new Error(
      `settle lookup table must not contain redemption_record (${accounts.redemptionRecord}); it must remain a static account`,
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
    redemptionRecord: accounts.redemptionRecord,
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
    payoutComputeSigner: accounts.payoutComputeSigner,
    payoutTotalSupplyAuthority: accounts.payoutTotalSupplyAuthority,
    batchPayoutBalanceValue: accounts.batchPayoutBalanceValue,
    payoutTotalSupplyValue: accounts.payoutTotalSupplyValue,
    zamaEventAuthority,
    confidentialTokenEventAuthority,
    cleartextTotal,
    signatures,
    extraData: hexToBytes(claim.extraData),
    leafIndex: claim.inclusionProof.leafIndex,
    siblings: [...claim.inclusionProof.siblings],
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
