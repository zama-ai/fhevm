//! Shared constants and PDA seed bytes for the demo-vault program.

/// App event schema version. This vault is not ingested by the coprocessor
/// host-listener; the field exists so app/demo indexers can version their reads
/// the same way the other PoC programs do.
pub const APP_EVENT_VERSION: u8 = 1;

/// Virtual assets added to the denominator of every share-price computation.
///
/// Together with [`VIRTUAL_SHARES`] this is the ERC-4626 virtual-offset defense
/// carried over to Solana: an empty vault prices shares 1:1, and a first
/// depositor can no longer inflate the share price by donating directly into
/// the vault, because the `+1/+1` offset keeps the price finite and leaves a
/// later victim with a non-zero, non-trivial share balance.
pub const VIRTUAL_ASSETS: u128 = 1;

/// Virtual shares added to the numerator of every share-price computation. See
/// [`VIRTUAL_ASSETS`].
pub const VIRTUAL_SHARES: u128 = 1;

/// PDA seed for the vault authority that owns the underlying token account and
/// mints/holds authority over the share mint.
pub const VAULT_AUTHORITY_SEED: &[u8] = b"authority";

/// PDA seed for the share mint owned by the vault authority.
pub const SHARE_MINT_SEED: &[u8] = b"shares";

/// PDA seed for the vault's underlying token account.
pub const VAULT_TOKEN_ACCOUNT_SEED: &[u8] = b"underlying";
