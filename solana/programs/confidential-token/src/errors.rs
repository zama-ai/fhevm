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
    /// Mint authority did not match signer.
    #[msg("Confidential mint authority does not match signer")]
    MintAuthorityMismatch,
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
    /// Current EncryptedValue account did not match token account state.
    #[msg("current encrypted value does not match token account state")]
    CurrentEncryptedValueMismatch,
    /// Transfer amount handle does not carry the expected confidential balance type.
    #[msg("transfer amount handle type is invalid")]
    AmountHandleTypeMismatch,
    /// Transfer amount ACL record is not scoped to the sender token account.
    #[msg("transfer amount ACL record is invalid")]
    AmountAclMismatch,
    /// The attested input's user does not match the transaction owner/authority.
    #[msg("attested input user does not match owner")]
    AttestationUserMismatch,
    /// The attested input's contract is not the mint compute-signer PDA.
    #[msg("attested input contract does not match compute signer")]
    AttestationContractMismatch,
    /// The signer spending an existing amount value is not in that value's subject set.
    /// Token-level spend gate mirroring EVM's `FHE.isAllowed(amount, msg.sender)`.
    #[msg("amount value spender is not in the amount's subject set")]
    AmountSpendSubjectMismatch,
    /// Total-supply authority PDA did not match the mint.
    #[msg("total supply authority does not match mint")]
    TotalSupplyAuthorityMismatch,
    /// The KMS EIP-712 public-decrypt certificate failed secp256k1 threshold verification.
    #[msg("KMS public-decrypt certificate is invalid")]
    InvalidKmsCertificate,
    /// The MMR public-decrypt proof for the pinned burned handle did not verify against
    /// the lineage's current peaks.
    #[msg("public-decrypt MMR proof is invalid for this lineage")]
    PublicDecryptProofInvalid,
    /// The host gateway verifier config (KMS signer / decryption contract) is unset.
    #[msg("gateway verifier config is not set")]
    GatewayVerifierConfigUnset,
    /// The provided KMS context is not the request-pinned context or has been destroyed.
    #[msg("KMS context is not valid for this request")]
    InvalidKmsContext,
    /// Account-backed request witness does not match the disclosure or redemption.
    #[msg("request witness does not match")]
    RequestWitnessMismatch,
    /// Account-backed request witness is expired or already consumed.
    #[msg("request witness is expired or already consumed")]
    RequestWitnessUnavailable,
    /// Material commitment witness did not match the disclosed handle.
    #[msg("material commitment witness does not match")]
    MaterialCommitmentMismatch,
    /// The disclosed handle has not been released for public decrypt.
    #[msg("handle is not released for public decrypt")]
    PublicDecryptNotReleased,
    /// Internal FHE eval plan construction failed before the host CPI.
    #[msg("FHE eval plan is invalid")]
    InvalidFheEvalPlan,
    /// The FHE eval candidate account list contains the same account twice.
    #[msg("FHE eval account list contains a duplicate account")]
    DuplicateFheEvalAccount,
    /// The FHE eval candidate account list contains an account the plan does not require.
    #[msg("FHE eval account list contains an unexpected account")]
    UnexpectedFheEvalAccount,
    /// The FHE eval plan requires a dynamic account that was not provided.
    #[msg("FHE eval plan is missing a required dynamic account")]
    MissingFheEvalAccount,
    /// The FHE eval plan requires a writable dynamic account but the provided account is readonly.
    #[msg("FHE eval dynamic account must be writable")]
    FheEvalAccountNotWritable,
    /// The FHE eval output authority list contains the same authority twice.
    #[msg("FHE eval output authority list contains a duplicate authority")]
    DuplicateFheOutputAuthority,
    /// The FHE eval output authority list contains an authority the plan does not require.
    #[msg("FHE eval output authority list contains an unexpected authority")]
    UnexpectedFheOutputAuthority,
    /// The FHE eval plan requires an output authority that was not provided.
    #[msg("FHE eval plan is missing a required output authority")]
    MissingFheOutputAuthority,
    /// The host public-decrypt verifier CPI did not return well-formed `(handle, cleartext)`
    /// data, was not produced by the ZamaHost program, or certified a cleartext that does not fit
    /// the token's euint64 width (nonzero high bytes in the 32-byte `uint256`).
    #[msg("public-decrypt verifier return data is invalid")]
    VerifierReturnDataInvalid,
    /// The handle proven public by the host verifier did not equal the caller-pinned handle.
    #[msg("disclosed handle does not match the pinned handle")]
    DisclosedHandleMismatch,
    /// The redeem signer is on the host deny-list, so it cannot cash out.
    #[msg("redemption subject is denied")]
    RedemptionSubjectDenied,
    /// The redeem deny-list record was missing, malformed, or not the canonical PDA for the signer,
    /// or one was supplied while the host grant deny-list is disabled.
    #[msg("redemption deny-list record is invalid")]
    RedemptionDenyRecordInvalid,
}
