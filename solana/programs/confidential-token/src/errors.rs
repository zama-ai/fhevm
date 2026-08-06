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
    /// Retired (zero references). Kept so Anchor error ordinals stay stable.
    #[msg("Confidential mint config is invalid")]
    InvalidMintConfig,
    /// Retired (zero references). Kept so Anchor error ordinals stay stable.
    #[msg("Confidential mint authority does not match signer")]
    MintAuthorityMismatch,
    /// The instruction included undeclared trailing account metas.
    #[msg("instruction has unexpected remaining accounts")]
    UnexpectedRemainingAccounts,
    /// Token account was not the canonical owner/mint PDA.
    #[msg("Confidential token account is not canonical")]
    TokenAccountMismatch,
    /// Retired (zero references). Kept so Anchor error ordinals stay stable.
    #[msg("ACL nonce overflow")]
    AclNonceOverflow,
    /// Retired (zero references): token accounts now always initialize with a hardcoded
    /// zero balance, so the nonzero-rejection check no longer exists to trip. Kept so Anchor
    /// error ordinals stay stable.
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
    DomainMismatch,
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
    /// the encrypted value account's current peaks.
    #[msg("public-decrypt MMR proof is invalid for this encrypted value account")]
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
    /// Tombstoned: disclosure material-commitment witness was removed with the
    /// `DisclosureRequest` lifecycle (fhevm#3231). Kept so Anchor error ordinals stay stable.
    #[msg("material commitment witness does not match (retired)")]
    MaterialCommitmentMismatch,
    /// Tombstoned: public-decrypt release gate lived on the deleted disclosure request path.
    /// Kept so Anchor error ordinals stay stable.
    #[msg("handle is not released for public decrypt (retired)")]
    PublicDecryptNotReleased,
    /// Internal FHE execution construction failed before the host CPI.
    #[msg("FHE execution is invalid")]
    InvalidFheExecution,
    /// The fhe_execute candidate account list contains the same account twice.
    #[msg("fhe_execute account list contains a duplicate account")]
    DuplicateFheExecuteAccount,
    /// The fhe_execute candidate account list contains an account the execution does not require.
    #[msg("fhe_execute account list contains an unexpected account")]
    UnexpectedFheExecuteAccount,
    /// The FHE execution requires a dynamic account that was not provided.
    #[msg("FHE execution is missing a required dynamic account")]
    MissingFheExecuteAccount,
    /// The FHE execution requires a writable dynamic account but the provided account is readonly.
    #[msg("fhe_execute dynamic account must be writable")]
    FheExecuteAccountNotWritable,
    /// The fhe_execute output authority list contains the same authority twice.
    #[msg("fhe_execute output authority list contains a duplicate authority")]
    DuplicateFheOutputAuthority,
    /// The fhe_execute output authority list contains an authority the execution does not require.
    #[msg("fhe_execute output authority list contains an unexpected authority")]
    UnexpectedFheOutputAuthority,
    /// The FHE execution requires an output authority that was not provided.
    #[msg("FHE execution is missing a required output authority")]
    MissingFheOutputAuthority,
    /// The host public-decrypt verifier CPI did not return well-formed `(handle, cleartext)`
    /// data, or the return was not produced by the ZamaHost program.
    #[msg("public-decrypt verifier return data is invalid")]
    VerifierReturnDataInvalid,
    /// The handle proven public by the host verifier did not equal the caller-pinned handle.
    #[msg("disclosed handle does not match the pinned handle")]
    DisclosedHandleMismatch,
    /// The certified `uint256` cleartext does not fit the token's euint64 width (nonzero high bytes).
    #[msg("certified cleartext exceeds euint64 width")]
    CleartextExceedsEuint64,
    /// The supplied pending-burn account is not the canonical PDA for `(mint, token_account)`.
    #[msg("pending burn address does not match")]
    PendingBurnAddressMismatch,
    /// The pending-burn PDA is already initialized (or is not a fresh system-owned empty account).
    #[msg("pending burn is already initialized")]
    PendingBurnAlreadyInitialized,
    /// Redeem/cancel requires the burned handle to still be the burned-amount encrypted value
    /// account's `current_handle`.
    #[msg("pending burn handle is not the burned amount current handle")]
    PendingBurnHandleNotCurrent,
    /// Pending-burn account fields do not match the redeem/cancel accounts.
    #[msg("pending burn fields do not match")]
    PendingBurnMismatch,
    /// The supplied token program does not own the underlying mint or token account.
    #[msg("underlying token program does not match")]
    UnderlyingTokenProgramMismatch,
    /// Token-2022 extensions are unsupported unless explicitly allowed by the wrapper.
    #[msg("unsupported Token-2022 extension")]
    UnsupportedToken2022Extension,
    /// A frozen underlying token account cannot participate in wrap or redeem.
    #[msg("underlying token account is frozen")]
    UnderlyingTokenAccountFrozen,
    /// The disclosed encrypted value account does not match the declared token state field.
    #[msg("disclosed value does not match its declared token state field")]
    DisclosedValueBindingMismatch,
    /// An encrypted value account is not controlled by the supplied token account PDA.
    #[msg("encrypted value account authority does not match token account")]
    EncryptedValueAuthorityMismatch,
    /// A token-account-scoped encrypted value has the wrong domain or canonical address.
    #[msg("token encrypted value account is not canonical")]
    TokenEncryptedValueMismatch,
    /// The encrypted total-supply value is not the canonical mint-scoped account.
    #[msg("encrypted total supply value is not canonical")]
    TotalSupplyValueMismatch,
}
