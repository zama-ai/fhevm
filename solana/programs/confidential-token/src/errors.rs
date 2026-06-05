//! Program-specific errors returned by confidential-token instructions.

use anchor_lang::prelude::*;

/// Errors returned by the confidential token PoC.
#[error_code]
pub enum ConfidentialTokenError {
    /// Token owner did not match the required signer.
    #[msg("Token owner does not match signer")]
    OwnerMismatch,
    /// Token account mint did not match the supplied mint.
    #[msg("Token account mint does not match")]
    MintMismatch,
    /// Confidential mint account shape or self-domain metadata is invalid.
    #[msg("Confidential mint account is invalid")]
    MintAccountMismatch,
    /// Confidential mint profile fields are unusable.
    #[msg("Confidential mint config is invalid")]
    InvalidMintConfig,
    /// The instruction included undeclared trailing account metas.
    #[msg("instruction has unexpected remaining accounts")]
    UnexpectedRemainingAccounts,
    /// Token account was not the canonical owner/mint PDA.
    #[msg("Confidential token account is not canonical")]
    TokenAccountMismatch,
    /// Balance nonce sequence overflowed.
    #[msg("ACL nonce overflow")]
    AclNonceOverflow,
    /// Token account initialization cannot mint unbacked confidential supply.
    #[msg("nonzero initial confidential balances are unsupported")]
    NonZeroInitialBalanceUnsupported,
    /// Underlying SPL mint did not match the confidential mint metadata.
    #[msg("Underlying mint does not match confidential mint")]
    UnderlyingMintMismatch,
    /// Vault token account owner did not match the vault authority PDA.
    #[msg("Vault token account authority does not match vault authority PDA")]
    VaultAuthorityMismatch,
    /// Vault token account was not the mint's canonical associated token account.
    #[msg("Vault token account is not the canonical mint vault")]
    VaultAccountMismatch,
    /// Confidential mint ACL domain key was not the expected mint key.
    #[msg("Confidential mint ACL domain key is invalid")]
    AclDomainKeyMismatch,
    /// Compute signer PDA did not match the confidential mint metadata.
    #[msg("Compute signer does not match confidential mint")]
    ComputeSignerMismatch,
    /// Current ACL record account did not match token account state.
    #[msg("current ACL record does not match token account state")]
    CurrentAclRecordMismatch,
    /// Operator authorization row did not match the requested transfer.
    #[msg("operator record does not match")]
    OperatorRecordMismatch,
    /// Operator authorization is missing or expired.
    #[msg("operator authorization is expired")]
    OperatorExpired,
    /// Transfer amount handle does not carry the expected confidential balance type.
    #[msg("transfer amount handle type is invalid")]
    AmountHandleTypeMismatch,
    /// Transfer amount ACL record is not scoped to the sender token account.
    #[msg("transfer amount ACL record is invalid")]
    AmountAclMismatch,
    /// Total-supply authority PDA did not match the mint.
    #[msg("total supply authority does not match mint")]
    TotalSupplyAuthorityMismatch,
    /// Disclosure certificate was not signed by the mint KMS verifier authority.
    #[msg("disclosure proof signature is missing or invalid")]
    DisclosureProofSignatureMissing,
    /// The KMS EIP-712 public-decrypt certificate failed secp256k1 threshold verification.
    #[msg("KMS public-decrypt certificate is invalid")]
    InvalidKmsCertificate,
    /// The host gateway verifier config (KMS signer / decryption contract) is unset.
    #[msg("gateway verifier config is not set")]
    GatewayVerifierConfigUnset,
    /// The provided KMS context is not the active context or has been destroyed.
    #[msg("KMS context is not active")]
    InvalidKmsContext,
    /// Material commitment witness did not match the disclosed handle.
    #[msg("material commitment witness does not match")]
    MaterialCommitmentMismatch,
    /// The disclosed handle has not been released for public decrypt.
    #[msg("handle is not released for public decrypt")]
    PublicDecryptNotReleased,
    /// Transfer callback settlement accounts do not match the prior transfer.
    #[msg("transfer callback settlement accounts are invalid")]
    CallbackSettlementMismatch,
    /// Receiver hook did not return the expected callback witness.
    #[msg("receiver hook return data is invalid")]
    ReceiverHookMismatch,
    /// Receiver hook payload or account list exceeds program limits.
    #[msg("receiver hook input exceeds program limits")]
    ReceiverHookInputTooLarge,
}
