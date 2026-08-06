import {
  appendTransactionMessageInstructions,
  assertIsFullySignedTransaction,
  assertIsTransactionWithBlockhashLifetime,
  assertIsTransactionWithinSizeLimit,
  compressTransactionMessageUsingAddressLookupTables,
  createTransactionMessage,
  pipe,
  setTransactionMessageComputeUnitLimit,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type Address,
  type Blockhash,
  type FullySignedTransaction,
  type Instruction,
  type Transaction,
  type TransactionSigner,
  type TransactionWithBlockhashLifetime,
  type TransactionWithinSizeLimit,
} from '@solana/kit';

/**
 * Builds and signs the ALT-aware v0 transaction that carries one `settle` instruction.
 *
 * Settle takes 34 accounts; a legacy transaction overflows the 1232-byte packet at every reachable
 * cert shape (measured in `batcher_mollusk.rs`), so a lookup table is mandatory. The table is
 * created at `open_batch` with every settle account derivable then — including `pending_burn`, which
 * is derived from the join mint and batch join token account. The fee payer stays static in the v0 message (Solana always
 * keeps it static) and is therefore NOT passed here as a table address; program ids and event-CPI
 * authorities outside the provisioned list also remain static.
 *
 * `compressTransactionMessageUsingAddressLookupTables` moves exactly the accounts present in
 * `lookupTableAddresses` into the table's lookups and leaves everything else static. The size
 * assertion runs against the resulting v0 wire form.
 */
export async function buildAndSignSettleTransaction(input: {
  readonly settleInstruction: Instruction;
  readonly feePayer: TransactionSigner;
  readonly latestBlockhash: { readonly blockhash: Blockhash; readonly lastValidBlockHeight: bigint };
  readonly computeUnitLimit: number;
  readonly lookupTableAddress: Address;
  /** The addresses the on-chain table holds — every settle account except the fee payer. */
  readonly lookupTableAddresses: readonly Address[];
}): Promise<Transaction & FullySignedTransaction & TransactionWithBlockhashLifetime & TransactionWithinSizeLimit> {
  const message = pipe(
    createTransactionMessage({ version: 0 }),
    (m) => setTransactionMessageFeePayerSigner(input.feePayer, m),
    (m) => setTransactionMessageLifetimeUsingBlockhash(input.latestBlockhash, m),
    (m) => setTransactionMessageComputeUnitLimit(input.computeUnitLimit, m),
    (m) => appendTransactionMessageInstructions([input.settleInstruction], m),
    (m) =>
      compressTransactionMessageUsingAddressLookupTables(m, {
        [input.lookupTableAddress]: [...input.lookupTableAddresses],
      }),
  );
  const transaction = await signTransactionMessageWithSigners(message);
  assertIsFullySignedTransaction(transaction);
  assertIsTransactionWithBlockhashLifetime(transaction);
  // Runs against the compressed v0 wire size — the whole reason settle needs the table.
  assertIsTransactionWithinSizeLimit(transaction);
  return transaction;
}
