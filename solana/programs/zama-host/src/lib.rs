//! Anchor program for the Solana FHEVM host PoC.
//!
//! `zama-host` owns the protocol-facing parts of the PoC: the append-only
//! `EncryptedValue` ACL/MMR model, handle derivation, FHE eval, public-decrypt
//! state, test/mock gates, and the small set of account witnesses that a future
//! Gateway/KMS request must verify.
//!
//! The program intentionally keeps app semantics outside this crate. App
//! programs, such as `confidential-token`, decide which app accounts and
//! labels they authorize, then call this program by CPI to create or verify
//! host-owned ACL state.

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]
#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

/// Shared constants, seed bytes, and fixed protocol sizes.
pub mod constants;
/// EIP-712 v4 verification of EVM-signed KMS / coprocessor certificates.
pub mod eip712;
/// Program-specific errors returned by ZamaHost instructions.
pub mod errors;
/// Anchor events emitted by protocol and test-shim instructions.
pub mod events;
/// Instruction account contexts and handlers.
pub mod instructions;
/// Account layouts, PDA helpers, and handle derivation helpers.
pub mod state;

use anchor_lang::prelude::*;

/// Re-export constants for generated clients and tests.
pub use constants::*;
/// Re-export error types for generated clients and tests.
pub use errors::*;
/// Re-export event types for generated clients, listeners, and tests.
pub use events::*;
/// Re-export account layouts and helper functions used by app programs.
pub use state::*;

use instructions::*;

declare_id!("6AtbvED1rfX68aCT1tYgU1aeu4kFksPDxZG9gtB1Fgtu");

/// Anchor entrypoint module generated into the ZamaHost IDL.
#[program]
pub mod zama_host {
    use super::*;

    pub fn initialize_host_config(
        ctx: Context<InitializeHostConfig>,
        args: InitializeHostConfigArgs,
    ) -> Result<()> {
        instructions::initialize_host_config(ctx, args)
    }

    /// Defines a new KMS context (mirror of `ProtocolConfig.defineNewKmsContext`).
    pub fn define_kms_context(
        ctx: Context<DefineKmsContext>,
        context_id: u64,
        signers: Vec<[u8; 20]>,
        thresholds: KmsThresholds,
    ) -> Result<()> {
        instructions::define_kms_context(ctx, context_id, signers, thresholds)
    }

    /// Destroys a non-current KMS context (mirror of `ProtocolConfig.destroyKmsContext`).
    pub fn destroy_kms_context(ctx: Context<DestroyKmsContext>, context_id: u64) -> Result<()> {
        instructions::destroy_kms_context(ctx, context_id)
    }

    pub fn set_host_pause(ctx: Context<HostAdmin>, paused: bool) -> Result<()> {
        instructions::set_host_pause(ctx, paused)
    }

    #[cfg(feature = "poc")]
    pub fn set_test_shims_enabled(ctx: Context<HostAdmin>, enabled: bool) -> Result<()> {
        instructions::set_test_shims_enabled(ctx, enabled)
    }

    #[cfg(feature = "poc")]
    pub fn set_mock_input_enabled(ctx: Context<HostAdmin>, enabled: bool) -> Result<()> {
        instructions::set_mock_input_enabled(ctx, enabled)
    }

    pub fn set_grant_deny_list_enabled(ctx: Context<HostAdmin>, enabled: bool) -> Result<()> {
        instructions::set_grant_deny_list_enabled(ctx, enabled)
    }

    pub fn set_max_hcu_per_tx(ctx: Context<HostAdmin>, value: u64) -> Result<()> {
        instructions::set_max_hcu_per_tx(ctx, value)
    }

    pub fn set_max_hcu_depth_per_tx(ctx: Context<HostAdmin>, value: u64) -> Result<()> {
        instructions::set_max_hcu_depth_per_tx(ctx, value)
    }

    pub fn set_deny_subject(
        ctx: Context<SetDenySubject>,
        subject: Pubkey,
        denied: bool,
    ) -> Result<()> {
        instructions::set_deny_subject(ctx, subject, denied)
    }

    pub fn delegate_for_user_decryption(
        ctx: Context<DelegateForUserDecryption>,
        delegate: Pubkey,
        app_account: Pubkey,
        expiration_slot: u64,
    ) -> Result<()> {
        instructions::delegate_for_user_decryption(ctx, delegate, app_account, expiration_slot)
    }

    pub fn revoke_delegation_for_user_decryption(
        ctx: Context<RevokeDelegationForUserDecryption>,
    ) -> Result<()> {
        instructions::revoke_delegation_for_user_decryption(ctx)
    }

    pub fn fhe_eval<'info>(ctx: Context<'info, FheEval<'info>>, args: FheEvalArgs) -> Result<()> {
        instructions::fhe_eval(ctx, args)
    }

    #[cfg(feature = "poc")]
    pub fn test_emit_trivial_encrypt(
        ctx: Context<TestEmitProtocolEvent>,
        subject: Pubkey,
        plaintext: [u8; 32],
        fhe_type: u8,
        result: [u8; 32],
    ) -> Result<()> {
        instructions::test_emit_trivial_encrypt(ctx, subject, plaintext, fhe_type, result)
    }

    #[cfg(feature = "poc")]
    pub fn test_emit_fhe_rand(
        ctx: Context<TestEmitProtocolEvent>,
        subject: Pubkey,
        seed: [u8; 16],
        fhe_type: u8,
        result: [u8; 32],
    ) -> Result<()> {
        instructions::test_emit_fhe_rand(ctx, subject, seed, fhe_type, result)
    }

    // ---- RFC-024 EncryptedValue ACL model ----

    pub fn create_encrypted_value(
        ctx: Context<CreateEncryptedValue>,
        acl_domain_key: Pubkey,
        app_account: Pubkey,
        encrypted_value_label: [u8; 32],
        handle: [u8; 32],
        subjects: Vec<EncryptedValueSubjectGrant>,
    ) -> Result<()> {
        instructions::create_encrypted_value(
            ctx,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subjects,
        )
    }

    pub fn allow_subjects(
        ctx: Context<AllowEncryptedValueSubjects>,
        subjects: Vec<EncryptedValueSubjectGrant>,
    ) -> Result<()> {
        instructions::allow_subjects(ctx, subjects)
    }

    pub fn update_encrypted_value(
        ctx: Context<UpdateEncryptedValue>,
        new_handle: [u8; 32],
        previous_handle: [u8; 32],
        previous_subjects: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::update_encrypted_value(ctx, new_handle, previous_handle, previous_subjects)
    }

    pub fn make_handle_public(ctx: Context<MakeEncryptedValueHandlePublic>) -> Result<()> {
        instructions::make_handle_public(ctx)
    }
}
