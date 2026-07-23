import { getAddressEncoder, getProgramDerivedAddress, type Address } from '@solana/kit';

import type { Bytes32 } from '../../core/types/primitives.js';
import { findComputeSignerPda } from '../internal/generated/confidentialToken/pdas/computeSigner.js';
import { findTotalSupplyAuthorityPda } from '../internal/generated/confidentialToken/pdas/totalSupplyAuthority.js';
import { findVaultAuthorityPda as findMintVaultAuthorityPda } from '../internal/generated/confidentialToken/pdas/vaultAuthority.js';
import {
  findVaultAuthorityPda as findDemoVaultAuthorityPda,
  findVaultTokenAccountPda,
} from './internal/generated/demoVault/pdas/index.js';
import {
  findBatchAuthorityPda,
  findBatchJoinUnderlyingPda,
  findBatchPayoutUnderlyingPda,
  findJoinRecordPda,
} from './internal/generated/confidentialBatcher/pdas/index.js';
import {
  batchAddress,
  burnRedemptionAddress,
  encryptedValueAddress,
  tokenAccountAddress,
} from './internal/batcherPdas.js';

/**
 * The immutable roots of one batcher's demo topology — the addresses a real integrator (or the
 * demo-config JSON) knows up front. Everything batch-scoped is DERIVED from these plus a batch
 * index; nothing here is fetched. `join*`/`payout*` name the two directions' mints: a Deposit
 * batcher joins cUSDC and pays out cShares, a Redeem batcher the reverse.
 */
export interface VaultDemoRoots {
  readonly batcherProgram: Address;
  readonly tokenProgram: Address;
  readonly vaultProgram: Address;
  readonly hostProgram: Address;
  readonly batcher: Address;
  readonly vault: Address;
  /** Confidential mint the batch total is burned on (join leg). */
  readonly joinConfidentialMint: Address;
  /** Confidential mint claims pay out in (payout leg). */
  readonly payoutConfidentialMint: Address;
  /** SPL mint `joinConfidentialMint` wraps. */
  readonly joinUnderlyingMint: Address;
  /** SPL mint `payoutConfidentialMint` wraps. */
  readonly payoutUnderlyingMint: Address;
  readonly hostConfig: Address;
  readonly kmsContext: Address;
}

/** Every batch-scoped address derivable from the roots and a batch index — no chain reads. */
export interface BatchAddresses {
  /** The batch account PDA (seeds on the batcher's index). */
  readonly batch: Address;
  /** The per-batch signing authority (redemption recipient, vault actor, wrap owner). */
  readonly batchAuthority: Address;
  /** The batch authority's confidential token account on the join mint. */
  readonly batchJoinTokenAccount: Address;
  /** The batch authority's confidential token account on the payout mint. */
  readonly batchPayoutTokenAccount: Address;
  /** Plain SPL account receiving the redeemed batch total (join underlying). */
  readonly batchJoinUnderlying: Address;
  /** Plain SPL account receiving the vault leg's output (payout underlying). */
  readonly batchPayoutUnderlying: Address;
  /** The batch's burned-amount lineage on the join mint (the settle proof's `encrypted_value`). */
  readonly batchBurnedAmountValue: Address;
  /** The batch payout token account's confidential balance lineage. */
  readonly batchPayoutBalanceValue: Address;
}

/** Fixed 32-byte confidential-token field labels (padded to 32 with `_`), from `state/mod.rs`. */
const BALANCE_LABEL = new TextEncoder().encode('balance_________________________');
const TOTAL_SUPPLY_LABEL = new TextEncoder().encode('total_supply____________________');

/** Program ids for the classic SPL token + associated-token programs the confidential mints wrap. */
const SPL_TOKEN_PROGRAM_ADDRESS = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA' as Address;
const ASSOCIATED_TOKEN_PROGRAM_ADDRESS = 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL' as Address;

const addressEncoder = getAddressEncoder();
function encodeAddress(value: Address): Uint8Array {
  return new Uint8Array(addressEncoder.encode(value));
}

/**
 * The canonical associated token account for one owner and SPL mint on the classic token program
 * (`get_associated_token_address_with_program_id`). The confidential mint's underlying-token vault
 * is this ATA owned by the mint's vault authority.
 */
async function associatedTokenAddress(owner: Address, mint: Address): Promise<Address> {
  const [address] = await getProgramDerivedAddress({
    programAddress: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
    seeds: [encodeAddress(owner), encodeAddress(SPL_TOKEN_PROGRAM_ADDRESS), encodeAddress(mint)],
  });
  return address;
}

/**
 * Derives every batch-scoped address for a batcher and zero-based batch index. Pure: it seeds PDAs
 * and hashes labels, never touching the RPC. The join-side underlying accounts, the payout-side
 * confidential accounts, and both plain SPL underlyings all fall out of the batch account and the
 * roots' mints — the same derivations the on-chain `settle` handler resolves as `#[account(seeds)]`.
 */
export async function deriveBatchAddresses(roots: VaultDemoRoots, batchIndex: bigint): Promise<BatchAddresses> {
  const batch = await batchAddress(roots.batcher, batchIndex);
  const [batchAuthority] = await findBatchAuthorityPda({ batch });
  const batchJoinTokenAccount = await tokenAccountAddress(roots.joinConfidentialMint, batchAuthority);
  const batchPayoutTokenAccount = await tokenAccountAddress(roots.payoutConfidentialMint, batchAuthority);
  const [batchJoinUnderlying] = await findBatchJoinUnderlyingPda({ batch });
  const [batchPayoutUnderlying] = await findBatchPayoutUnderlyingPda({ batch });
  return {
    batch,
    batchAuthority,
    batchJoinTokenAccount,
    batchPayoutTokenAccount,
    batchJoinUnderlying,
    batchPayoutUnderlying,
    // Burned amount: acl domain = join mint, app account = batch join token account, label =
    // `burned_amount`. Payout balance: acl domain = payout mint, app account = batch payout token
    // account, label = `balance`.
    batchBurnedAmountValue: await encryptedValueAddress(
      roots.joinConfidentialMint,
      batchJoinTokenAccount,
      new TextEncoder().encode('burned_amount___________________'),
    ),
    batchPayoutBalanceValue: await encryptedValueAddress(
      roots.payoutConfidentialMint,
      batchPayoutTokenAccount,
      BALANCE_LABEL,
    ),
  };
}

/** The user's per-batch join record PDA — the companion derivation `deriveBatchAddresses` omits (it needs a user). */
export async function deriveJoinRecordAddress(batch: Address, user: Address): Promise<Address> {
  const [joinRecord] = await findJoinRecordPda({ batch, user });
  return joinRecord;
}

/**
 * The complete account set for one `settle`, derived from the roots, a batch's addresses, and the
 * batch's born-public burned handle. This is exactly the settle instruction's non-signer, non-fixed
 * accounts — the fee payer and the two event-CPI authorities are the only accounts `settleBatch`
 * still resolves itself. `redemptionRecord` is seeded by the burned handle (which only exists after
 * dispatch), so it is derivable only once the handle is known — hence the `burnedHandle` argument.
 */
export interface SolanaVaultSettleAccounts {
  readonly batcher: Address;
  readonly batch: Address;
  readonly joinConfidentialMint: Address;
  readonly batchJoinTokenAccount: Address;
  readonly joinUnderlyingMint: Address;
  readonly joinMintVaultUnderlying: Address;
  readonly joinMintVaultAuthority: Address;
  readonly batchBurnedAmountValue: Address;
  /** Per-handle redemption replay marker; created at settle, so it must stay a static account. */
  readonly redemptionRecord: Address;
  readonly hostConfig: Address;
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
}

/**
 * The ordered address set the settle Address Lookup Table holds — every settle account derivable at
 * `open_batch`, i.e. the full settle set except the fee payer (always static) and `redemption_record`
 * (seeded by a post-dispatch handle, so absent when the table is frozen). The demo seeder creates the
 * on-chain ALT from this exact ordered list and `settleBatch` compresses against the same list, so the
 * two agree by construction — the v0 message's table indices line up with the on-chain entries. The
 * `burnedHandle` here only feeds `redemptionRecord`, which is then dropped, so any 32 zero-bytes work;
 * callers that lack a handle (the seeder at open_batch) pass zeros.
 */
export async function deriveSettleLookupTableAddresses(
  roots: VaultDemoRoots,
  batch: BatchAddresses,
): Promise<Address[]> {
  const accounts = await deriveSettleAccounts(roots, batch, new Uint8Array(32) as Bytes32);
  return settleAccountsToLookupTableAddresses(accounts);
}

/** Flattens a settle account set into its ALT ordering (every field except `redemptionRecord`). */
export function settleAccountsToLookupTableAddresses(accounts: SolanaVaultSettleAccounts): Address[] {
  return Object.entries(accounts)
    .filter(([name]) => name !== 'redemptionRecord')
    .map(([, address]) => address);
}

export async function deriveSettleAccounts(
  roots: VaultDemoRoots,
  batch: BatchAddresses,
  burnedHandle: Bytes32,
): Promise<SolanaVaultSettleAccounts> {
  const [joinMintVaultAuthority] = await findMintVaultAuthorityPda({ mint: roots.joinConfidentialMint });
  const [payoutMintVaultAuthority] = await findMintVaultAuthorityPda({ mint: roots.payoutConfidentialMint });
  const [payoutComputeSigner] = await findComputeSignerPda({ mint: roots.payoutConfidentialMint });
  const [payoutTotalSupplyAuthority] = await findTotalSupplyAuthorityPda({ mint: roots.payoutConfidentialMint });
  const [vaultAuthority] = await findDemoVaultAuthorityPda({ vault: roots.vault });
  const [vaultTokenAccount] = await findVaultTokenAccountPda({ vault: roots.vault });
  return {
    batcher: roots.batcher,
    batch: batch.batch,
    joinConfidentialMint: roots.joinConfidentialMint,
    batchJoinTokenAccount: batch.batchJoinTokenAccount,
    joinUnderlyingMint: roots.joinUnderlyingMint,
    joinMintVaultUnderlying: await associatedTokenAddress(joinMintVaultAuthority, roots.joinUnderlyingMint),
    joinMintVaultAuthority,
    batchBurnedAmountValue: batch.batchBurnedAmountValue,
    redemptionRecord: await burnRedemptionAddress(roots.joinConfidentialMint, burnedHandle),
    hostConfig: roots.hostConfig,
    kmsContext: roots.kmsContext,
    vault: roots.vault,
    vaultAuthority,
    vaultTokenAccount,
    payoutConfidentialMint: roots.payoutConfidentialMint,
    payoutUnderlyingMint: roots.payoutUnderlyingMint,
    batchPayoutTokenAccount: batch.batchPayoutTokenAccount,
    payoutMintVaultUnderlying: await associatedTokenAddress(payoutMintVaultAuthority, roots.payoutUnderlyingMint),
    payoutMintVaultAuthority,
    payoutComputeSigner,
    payoutTotalSupplyAuthority,
    batchPayoutBalanceValue: batch.batchPayoutBalanceValue,
    // Total supply: acl domain = payout mint, app account = its total-supply authority, label =
    // `total_supply`.
    payoutTotalSupplyValue: await encryptedValueAddress(
      roots.payoutConfidentialMint,
      payoutTotalSupplyAuthority,
      TOTAL_SUPPLY_LABEL,
    ),
  };
}
