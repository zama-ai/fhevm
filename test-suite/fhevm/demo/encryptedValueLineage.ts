// EncryptedValue lineage derivation for the demo lane. The public vault surface exports the
// batcher-owned lineage (`pendingJoinLineage`) but not the confidential-token balance/transferred
// lineages, so the constants and hashing below are restated byte-identical to their owners —
// `zama_solana_acl::derive_value_key` (value-key prefix + `encrypted-value` seed) and
// `confidential_token::state` (the two 32-byte labels). The SDK does the same internally (see
// sdk/js-sdk/src/solana/actions/confidentialTransfer.ts).
//
// Drift protection: encryptedValueLineage.test.ts pins this copy against the Rust crate's golden
// value-key vector (the same vector sdk/js-sdk/src/solana/proof.test.ts pins the SDK copy to) and
// against the SDK's golden lineage PDA (derive.test.ts), so a divergence fails in the cheap
// `bun test demo` sweep instead of only in the live acceptance gate.

import { createHash } from "node:crypto";

import { getAddressEncoder, getProgramDerivedAddress, type Address } from "@solana/kit";

const utf8 = (value: string): Uint8Array => new TextEncoder().encode(value);

const VALUE_KEY_PREFIX = utf8("zama-encrypted-value-key-v1");
const ENCRYPTED_VALUE_SEED = utf8("encrypted-value");

/** `confidential_token::state` balance lineage label — fixed 32 bytes, underscore-padded. */
export const BALANCE_LABEL = utf8("balance_________________________");
/** `confidential_token::state` transferred-amount lineage label — fixed 32 bytes, underscore-padded. */
export const TRANSFERRED_AMOUNT_LABEL = utf8("transferred_amount______________");
/** `confidential_token::state` total-supply lineage label — fixed 32 bytes, underscore-padded. */
export const TOTAL_SUPPLY_LABEL = utf8("total_supply____________________");

/** Confidential-token total-supply authority PDA seed (generated `findTotalSupplyAuthorityPda`). */
const TOTAL_SUPPLY_AUTHORITY_SEED = utf8("total-supply");
/**
 * Anchor's event-CPI authority seed (`__event_authority`). Both the zama-host and confidential-token
 * programs derive their event authority from it (the SDK's internal `EVENT_AUTHORITY_SEED`).
 */
const EVENT_AUTHORITY_SEED = utf8("__event_authority");

/** The raw 32 bytes of a base58 Solana address. */
export const addressBytes = (value: Address): Uint8Array => new Uint8Array(getAddressEncoder().encode(value));

/** sha256(prefix || aclDomain(32) || appAccount(32) || label(32)) — mirrors `zama_solana_acl::derive_value_key`. */
export const deriveValueKey = (aclDomain: Uint8Array, appAccount: Uint8Array, label: Uint8Array): Uint8Array =>
  new Uint8Array(
    createHash("sha256").update(VALUE_KEY_PREFIX).update(aclDomain).update(appAccount).update(label).digest(),
  );

/** The canonical `EncryptedValue` PDA for one lineage: PDA(hostProgram, ["encrypted-value", valueKey]). */
export const encryptedValueAddress = async (
  hostProgram: Address,
  aclDomain: Address,
  appAccount: Address,
  label: Uint8Array,
): Promise<Address> => {
  const [pda] = await getProgramDerivedAddress({
    programAddress: hostProgram,
    seeds: [ENCRYPTED_VALUE_SEED, deriveValueKey(addressBytes(aclDomain), addressBytes(appAccount), label)],
  });
  return pda;
};

/** The mint's total-supply authority PDA: PDA(tokenProgram, ["total-supply", mint]). */
export const totalSupplyAuthorityAddress = async (tokenProgram: Address, mint: Address): Promise<Address> => {
  const [pda] = await getProgramDerivedAddress({
    programAddress: tokenProgram,
    seeds: [TOTAL_SUPPLY_AUTHORITY_SEED, addressBytes(mint)],
  });
  return pda;
};

/** A program's Anchor event-CPI authority PDA: PDA(program, ["__event_authority"]). */
export const eventAuthorityAddress = async (program: Address): Promise<Address> => {
  const [pda] = await getProgramDerivedAddress({ programAddress: program, seeds: [EVENT_AUTHORITY_SEED] });
  return pda;
};
