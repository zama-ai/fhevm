import { createSolanaFheTransaction } from '@fhevm/sdk/solana';
import { publicProof, type ProofService } from './internal/publicProof.js';
import {
  getBase64EncodedWireTransaction,
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
import { getSettleInstructionAsync } from './internal/generated/confidentialBatcher/instructions/settle.js';
import { fetchBatch } from './internal/generated/confidentialBatcher/accounts/batch.js';
import { settleTotalFromCleartext } from './internal/cleartext.js';
import { buildAndSignSettleTransaction } from './internal/settleMessage.js';
import {
  deriveBatchAddresses,
  deriveSettleAccounts,
  settleAccountsToLookupTableAddresses,
  type BatchAddresses,
  type VaultDemoRoots,
} from './derive.js';
import { getCurrentBatch } from './reads.js';

const ZERO_HANDLE = new Uint8Array(32);

/** What `settleBatch` needs beyond the certificate phase and the keeper signer. */
export type SolanaVaultSettleOptions = {
  readonly rpc: Rpc<SolanaRpcApi>;
  readonly proofService: ProofService;
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
 * Settles the batch's pinned burn handle with a KMS certificate and public-access proof.
 * The Connector fetches its own proof for the certificate request. After that request finishes,
 * this caller fetches a fresh listener proof and verifies it against the on-chain state peaks.
 * The resulting settle instruction uses the batch's lookup table to fit the transaction packet.
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

  // The KMS burn certificate. The relayer request names the handle and the account, nothing else.
  const claim = await publicDecryptCertificate(
    { chain, runtime: options.runtime },
    {
      handle: bytesToHex(burnedTotalHandle),
      contextId: options.contextId,
      encryptedStore: base58.decode(accounts.batchBurnedAmountStore),
      options: options.certificateOptions,
    },
  );

  const cleartextTotal = settleTotalFromCleartext(hexToBytes(claim.abiEncodedCleartext));
  const inclusionProof = await publicProof(
    rpc,
    options.proofService,
    accounts.batchBurnedAmountStore,
    burnedTotalHandle,
  );

  const signatures = claim.signatures.map((signature, index) => {
    const bytes = hexToBytes(signature);
    if (bytes.length !== 65) throw new Error(`certificate signature[${index}] must be 65 bytes`);
    return bytes;
  });

  // Provisioning and settlement use the same ordered account list, including the event
  // authorities and batch PDAs that the generated instruction can also derive.
  const lookupTableAddresses = settleAccountsToLookupTableAddresses(accounts);
  if (!lookupTableAddresses.includes(accounts.pendingBurn)) {
    throw new Error(
      `settle lookup table must contain pending_burn (${accounts.pendingBurn}); it is known at open_batch`,
    );
  }

  const fhe = await createSolanaFheTransaction({ payer: keeper });
  const settleInstruction = await getSettleInstructionAsync({
    ...fhe.accounts,
    payer: keeper,
    ...accounts,
    cleartextTotal,
    signatures,
    extraData: hexToBytes(claim.extraData),
    leafIndex: inclusionProof.leafIndex,
    siblings: [...inclusionProof.siblings],
    authorityFundingLamports: options.authorityFundingLamports,
  });

  const { value: latestBlockhash } = await rpc.getLatestBlockhash({ commitment: 'confirmed' }).send();
  const transaction = await buildAndSignSettleTransaction({
    instructions: fhe.wrap([settleInstruction]),
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
