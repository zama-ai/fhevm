//! Vault account layout and the on-the-fly share-price math.

use anchor_lang::prelude::*;

use crate::constants::{VIRTUAL_ASSETS, VIRTUAL_SHARES};
use crate::errors::DemoVaultError;

/// Public share-mint vault state.
///
/// Deliberately minimal: totals are never mirrored here. Total assets are read
/// live from the vault token account balance and total shares from the share
/// mint supply, so a permissionless `harvest` (a plain donation transfer) is
/// accounted for by construction — the donation just increases the token
/// account balance, and every share-price read picks it up.
#[account]
#[derive(InitSpace)]
pub struct Vault {
    /// Underlying token mint the vault accepts.
    pub underlying_mint: Pubkey,
    /// Share mint issued 1:1 in decimals with the underlying.
    pub share_mint: Pubkey,
    /// Token account holding the vault's underlying assets, owned by the authority PDA.
    pub vault_token_account: Pubkey,
    /// Bump for the vault authority PDA that owns assets and the share mint.
    pub authority_bump: u8,
}

impl Vault {
    /// Serialized size of the account body, excluding the Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 1;
}

/// Shares minted for a deposit of `assets`, rounded DOWN (protocol-favoring).
///
/// `shares = assets * (total_shares + VIRTUAL_SHARES) / (total_assets + VIRTUAL_ASSETS)`
///
/// `total_assets` / `total_shares` are the balances *before* this deposit. The
/// virtual offset makes the denominator always positive, so an empty vault
/// prices 1:1 and there is no division by zero.
pub fn assets_to_shares(assets: u64, total_assets: u64, total_shares: u64) -> Result<u64> {
    let numerator = (assets as u128)
        .checked_mul((total_shares as u128).saturating_add(VIRTUAL_SHARES))
        .ok_or(DemoVaultError::MathOverflow)?;
    let denominator = (total_assets as u128).saturating_add(VIRTUAL_ASSETS);
    let shares = numerator / denominator;
    u64::try_from(shares).map_err(|_| DemoVaultError::MathOverflow.into())
}

/// Underlying assets returned for a redeem of `shares`, rounded DOWN (protocol-favoring).
///
/// `assets = shares * (total_assets + VIRTUAL_ASSETS) / (total_shares + VIRTUAL_SHARES)`
///
/// `total_assets` / `total_shares` are the balances *before* this withdraw.
pub fn shares_to_assets(shares: u64, total_assets: u64, total_shares: u64) -> Result<u64> {
    let numerator = (shares as u128)
        .checked_mul((total_assets as u128).saturating_add(VIRTUAL_ASSETS))
        .ok_or(DemoVaultError::MathOverflow)?;
    let denominator = (total_shares as u128).saturating_add(VIRTUAL_SHARES);
    let assets = numerator / denominator;
    u64::try_from(assets).map_err(|_| DemoVaultError::MathOverflow.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vault_prices_one_to_one() {
        assert_eq!(assets_to_shares(1_000, 0, 0).unwrap(), 1_000);
    }

    #[test]
    fn deposit_and_withdraw_round_trip_without_yield() {
        let shares = assets_to_shares(1_000, 0, 0).unwrap();
        // Same price back out: shares -> assets returns the original deposit.
        let assets = shares_to_assets(shares, 1_000, shares).unwrap();
        assert_eq!(assets, 1_000);
    }

    #[test]
    fn second_depositor_after_yield_gets_fewer_shares() {
        // First depositor: 1_000 assets -> 1_000 shares (1:1).
        let first = assets_to_shares(1_000, 0, 0).unwrap();
        assert_eq!(first, 1_000);
        // Yield doubles the assets (harvest of 1_000) with supply unchanged.
        let second = assets_to_shares(1_000, 2_000, 1_000).unwrap();
        // Price is ~2 assets/share now, so the same deposit buys ~half the shares.
        assert!(second < first);
        assert_eq!(second, 500);
    }

    #[test]
    fn rounding_is_down_on_both_sides() {
        // shares = 5 * (1 + 1) / (2 + 1) = 10 / 3 = 3.33... -> floor 3.
        assert_eq!(assets_to_shares(5, 2, 1).unwrap(), 3);
        // assets = 1 * (4 + 1) / (1 + 1) = 5 / 2 = 2.5 -> floor 2.
        assert_eq!(shares_to_assets(1, 4, 1).unwrap(), 2);
    }

    #[test]
    fn inflation_attack_leaves_victim_with_shares_and_no_attacker_profit() {
        const DONATION: u64 = 100_000_000;
        const VICTIM_DEPOSIT: u64 = 100_000_000;

        // Attacker seeds 1 asset -> 1 share (empty vault, 1:1).
        let attacker_shares = assets_to_shares(1, 0, 0).unwrap();
        assert_eq!(attacker_shares, 1);
        // Attacker donates a large amount directly (harvest); supply stays 1.
        let assets_after_donation = 1 + DONATION;
        // Victim deposits; without the virtual offset it would round to zero shares and be wiped.
        let victim_shares =
            assets_to_shares(VICTIM_DEPOSIT, assets_after_donation, attacker_shares).unwrap();
        assert!(victim_shares > 0, "virtual offset must protect the victim");

        // Attacker redeems their single share against the post-victim state. The donation is
        // shared with the victim, so the attacker cannot recover the 1 + DONATION they spent —
        // the attack is uneconomical.
        let assets_after_victim = assets_after_donation + VICTIM_DEPOSIT;
        let shares_after_victim = attacker_shares + victim_shares;
        let attacker_payout =
            shares_to_assets(attacker_shares, assets_after_victim, shares_after_victim).unwrap();
        assert!(
            attacker_payout < 1 + DONATION,
            "attacker must exit at a loss; spent {} and recovered {attacker_payout}",
            1 + DONATION
        );
    }
}
