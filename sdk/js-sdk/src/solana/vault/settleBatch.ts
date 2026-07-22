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

import { bytesToHex, hexToBytes } from '../../core/base/bytes.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { publicDecryptCertificate } from '../actions/publicDecryptCertificate.js';
import {
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from '../internal/generated/confidentialToken/programAddress.js';
import { getSettleInstructionAsync } from './internal/generated/confidentialBatcher/instructions/settle.js';
import {
  burnRedemptionAddress,
  burnedAmountLineage,
  findBatchAuthorityPda,
  tokenAccountAddress,
} from './internal/batcherPdas.js';
import { fetchSolanaMmrProof, type SolanaProofServiceConfig } from './internal/proofService.js';
import { settleTotalFromCleartext } from './internal/cleartext.js';
import { buildAndSignSettleTransaction } from './internal/settleMessage.js';

/** Accounts for `settleBatch` that are not derivable from the batch alone. */
export type SolanaVaultSettleAccounts = {
  readonly batcher: Address;
  readonly batch: Address;
  /** `batcher.join_confidential_mint`. */
  readonly joinConfidentialMint: Address;
  readonly joinUnderlyingMint: Address;
  readonly joinMintVaultUnderlying: Address;
  readonly joinMintVaultAuthority: Address;
  readonly hostConfig: Address;
  /** KMS context for the id the certificate commits to. */
  readonly kmsContext: Address;
  readonly vault: Address;
  readonly vaultAuthority: Address;
  readonly vaultTokenAccount: Address;
  readonly payoutConfidentialMint: Address;
  readonly payoutUnderlyingMint: Address;
  readonly batchPayoutTokenAccount: Address;
  readonly payoutMintVaultUnderlying: Address;
  readonly payoutMintVaultAuthority: Address;
  readonly payoutComputeSigner: Address;
  readonly payoutTotalSupplyAuthority: Address;
  readonly batchPayoutBalanceValue: Address;
  readonly payoutTotalSupplyValue: Address;
};

export type SolanaVaultSettleParameters = {
  readonly rpc: Rpc<SolanaRpcApi>;
  readonly rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
  /** Certificate leg context: relayer chain config + decrypt runtime (auth). */
  readonly chain: FhevmSolanaChain;
  readonly runtime: FhevmRuntime;
  /** Vault-module config for the proof-service leg. */
  readonly proofService: SolanaProofServiceConfig;
  /** Pays the batch authority funding; the static fee payer (never loaded from the lookup table). */
  readonly payer: TransactionSigner;
  readonly accounts: SolanaVaultSettleAccounts;
  /** The batch's born-public burned total handle (`Batch.burned_total_handle` / `BatchDispatched`). */
  readonly burnedTotalHandle: Uint8Array;
  /** 32-byte context id the certificate commits to (the host's current KMS context). */
  readonly contextId: Uint8Array;
  /** Live MMR peaks of the burned lineage, read from its `EncryptedValue` account. */
  readonly peaks: readonly Uint8Array[];
  /** Live leaf count of the burned lineage, read from its `EncryptedValue` account. */
  readonly leafCount: bigint;
  /** The settle lookup table created at `open_batch`. */
  readonly lookupTableAddress: Address;
  /** The addresses that table holds — every settle account except the fee payer and `redemption_record`. */
  readonly lookupTableAddresses: readonly Address[];
  readonly authorityFundingLamports: bigint;
  readonly computeUnitLimit?: number | undefined;
};

/**
 * Settles a dispatched batch. Two off-chain legs feed the on-chain instruction:
 *
 * 1. the burned lineage's MMR inclusion proof from the solana-proof-service (verified against live
 *    on-chain peaks), and
 * 2. the KMS burn certificate from the relayer, requested with the proof from leg 1.
 *
 * The certified cleartext comes back as a 32-byte `uint256`; settle's on-chain argument is a `u64`,
 * so its low 8 bytes are extracted big-endian and its high 24 bytes asserted zero
 * ({@link settleTotalFromCleartext}). The instruction is then sent as an ALT-aware v0 transaction —
 * 34 accounts overflow a legacy packet — keeping the fee payer and `redemption_record` static.
 */
export async function settleBatch(parameters: SolanaVaultSettleParameters): Promise<Signature> {
  const { accounts } = parameters;
  const [batchAuthority] = await findBatchAuthorityPda({ batch: accounts.batch });
  const batchJoinTokenAccount = await tokenAccountAddress(accounts.joinConfidentialMint, batchAuthority);
  const burned = await burnedAmountLineage(accounts.joinConfidentialMint, batchJoinTokenAccount);

  // Leg 1: MMR proof (the batch burned lineage always holds one leaf → depth-0 proof).
  const proof = await fetchSolanaMmrProof(parameters.proofService, burned.encryptedValueAddress, 0n);
  // The proof is verified against the service's own live peaks; cross-check that those peaks
  // describe the same lineage state the caller read for the certificate, so a proof-service/account
  // mismatch fails fast here instead of at on-chain verify.
  if (proof.leafCount !== parameters.leafCount) {
    throw new Error(
      `proof-service leaf count ${proof.leafCount} does not match the lineage leaf count ${parameters.leafCount} supplied for verification`,
    );
  }

  // Leg 2: KMS burn certificate, verified against the live peaks with the leg-1 proof.
  const claim = await publicDecryptCertificate(
    { chain: parameters.chain, runtime: parameters.runtime },
    {
      handle: bytesToHex(parameters.burnedTotalHandle),
      contextId: parameters.contextId,
      aclValueKey: burned.aclValueKey,
      proofSlot: proof.proofSlot,
      encryptedValueAccount: base58.decode(burned.encryptedValueAddress),
      peaks: parameters.peaks,
      leafCount: parameters.leafCount,
      mmrProofBytes: proof.mmrProofBytes,
    },
  );

  const cleartextTotal = settleTotalFromCleartext(hexToBytes(claim.abiEncodedCleartext));
  const signatures = claim.signatures.map((signature, index) => {
    const bytes = hexToBytes(signature);
    if (bytes.length !== 65) throw new Error(`certificate signature[${index}] must be 65 bytes`);
    return bytes;
  });
  const redemptionRecord = await burnRedemptionAddress(accounts.joinConfidentialMint, parameters.burnedTotalHandle);
  // redemption_record is seeded by the burned handle and only exists after dispatch, so it cannot
  // live in a lookup table frozen at open_batch — it must stay a static account. If the caller put
  // it in the table, they built the table wrong; fail loudly rather than emit an invalid v0 message.
  if (parameters.lookupTableAddresses.includes(redemptionRecord)) {
    throw new Error(
      `lookupTableAddresses must not contain the settle redemption_record (${redemptionRecord}); it is underivable at open_batch and must remain a static account`,
    );
  }
  const eventAuthoritySeed = new TextEncoder().encode('__event_authority');
  const [zamaEventAuthority] = await getProgramDerivedAddress({
    programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
    seeds: [eventAuthoritySeed],
  });
  const [confidentialTokenEventAuthority] = await getProgramDerivedAddress({
    programAddress: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    seeds: [eventAuthoritySeed],
  });

  const settleInstruction = await getSettleInstructionAsync({
    payer: parameters.payer,
    batcher: accounts.batcher,
    batch: accounts.batch,
    joinConfidentialMint: accounts.joinConfidentialMint,
    batchJoinTokenAccount,
    joinUnderlyingMint: accounts.joinUnderlyingMint,
    joinMintVaultUnderlying: accounts.joinMintVaultUnderlying,
    joinMintVaultAuthority: accounts.joinMintVaultAuthority,
    batchBurnedAmountValue: burned.encryptedValueAddress,
    redemptionRecord,
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
    authorityFundingLamports: parameters.authorityFundingLamports,
  });

  const { value: latestBlockhash } = await parameters.rpc.getLatestBlockhash({ commitment: 'confirmed' }).send();
  const transaction = await buildAndSignSettleTransaction({
    settleInstruction,
    feePayer: parameters.payer,
    latestBlockhash,
    computeUnitLimit: parameters.computeUnitLimit ?? 1_000_000,
    lookupTableAddress: parameters.lookupTableAddress,
    lookupTableAddresses: parameters.lookupTableAddresses,
  });

  const wireTransaction = getBase64EncodedWireTransaction(transaction);
  const simulation = await parameters.rpc
    .simulateTransaction(wireTransaction, { commitment: 'confirmed', encoding: 'base64', sigVerify: true })
    .send();
  if (simulation.value.err !== null) {
    const err = JSON.stringify(simulation.value.err, (_key, value: unknown) =>
      typeof value === 'bigint' ? value.toString() : value,
    );
    const logs = simulation.value.logs?.join('\n') ?? '';
    throw new Error(logs.length > 0 ? `settle simulation failed: ${err}\n${logs}` : `settle simulation failed: ${err}`);
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
