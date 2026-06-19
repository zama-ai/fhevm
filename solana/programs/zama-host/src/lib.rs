//! Anchor program for the Solana FHEVM host PoC.
//!
//! `zama-host` owns the protocol-facing parts of the PoC:
//! ACL records, handle derivation, FHE event emission, public-decrypt state,
//! test/mock gates, and the small set of account witnesses that a future
//! Gateway/KMS request must verify.
//!
//! The program intentionally keeps app semantics outside this crate. App
//! programs, such as `confidential-token`, decide which app accounts and
//! labels they authorize, then call this program by CPI to create or verify
//! host-owned ACL state.

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]
#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

/// Shared constants, seed bytes, role flags, and fixed protocol sizes.
pub mod constants;
/// EIP-712 v4 verification of EVM-signed KMS / coprocessor certificates.
pub mod eip712;
/// Program-specific errors returned by ZamaHost instructions.
pub mod errors;
/// Anchor events emitted by protocol and test-shim instructions.
pub mod events;
/// Instruction account contexts and handlers.
pub mod instructions;
/// Account layouts, PDA helpers, roles, and handle derivation helpers.
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

    #[cfg(feature = "poc")]
    pub fn test_emit_acl_allowed(
        ctx: Context<TestEmitProtocolEvent>,
        handle: [u8; 32],
        subject: Pubkey,
    ) -> Result<()> {
        instructions::test_emit_acl_allowed(ctx, handle, subject)
    }

    pub fn allow_acl_subjects<'info>(
        ctx: Context<'info, AllowAclSubjects<'info>>,
        handle: [u8; 32],
        subjects: Vec<AclSubjectEntry>,
    ) -> Result<()> {
        instructions::allow_acl_subjects(ctx, handle, subjects)
    }

    pub fn assert_acl_record(
        ctx: Context<AssertAclRecord>,
        nonce_key: [u8; 32],
        nonce_sequence: u64,
        acl_domain_key: Pubkey,
        app_account: Pubkey,
        encrypted_value_label: [u8; 32],
        handle: [u8; 32],
        subject: Pubkey,
    ) -> Result<()> {
        instructions::assert_acl_record(
            ctx,
            nonce_key,
            nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subject,
        )
    }

    pub fn allow_for_decryption(ctx: Context<AllowForDecryption>, handle: [u8; 32]) -> Result<()> {
        instructions::allow_for_decryption(ctx, handle)
    }

    pub fn commit_handle_material(
        ctx: Context<CommitHandleMaterial>,
        key_id: [u8; 32],
        ciphertext_digest: [u8; 32],
        sns_ciphertext_digest: [u8; 32],
        coprocessor_set_digest: [u8; 32],
    ) -> Result<()> {
        instructions::commit_handle_material(
            ctx,
            key_id,
            ciphertext_digest,
            sns_ciphertext_digest,
            coprocessor_set_digest,
        )
    }

    pub fn trivial_encrypt_and_bind(
        ctx: Context<TrivialEncryptAndBind>,
        plaintext: [u8; 32],
        fhe_type: u8,
        output_nonce_key: [u8; 32],
        output_nonce_sequence: u64,
        output_acl_domain_key: Pubkey,
        output_app_account: Pubkey,
        output_encrypted_value_label: [u8; 32],
        output_subjects: Vec<AclSubjectEntry>,
        output_public_decrypt: bool,
    ) -> Result<()> {
        instructions::trivial_encrypt_and_bind(
            ctx,
            plaintext,
            fhe_type,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects,
            output_public_decrypt,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn verify_coprocessor_input(
        ctx: Context<VerifyCoprocessorInput>,
        input_handle: [u8; 32],
        ct_handles: Vec<[u8; 32]>,
        handle_index: u8,
        user_address: [u8; 32],
        contract_address: [u8; 32],
        contract_chain_id: u64,
        extra_data: Vec<u8>,
        signatures: Vec<[u8; 65]>,
    ) -> Result<()> {
        instructions::verify_coprocessor_input(
            ctx,
            input_handle,
            ct_handles,
            handle_index,
            user_address,
            contract_address,
            contract_chain_id,
            extra_data,
            signatures,
        )
    }

    pub fn fhe_binary_op(
        ctx: Context<FheBinaryOp>,
        op: FheBinaryOpCode,
        lhs: [u8; 32],
        rhs: [u8; 32],
        scalar: bool,
        output_fhe_type: u8,
        result: [u8; 32],
    ) -> Result<()> {
        instructions::fhe_binary_op(ctx, op, lhs, rhs, scalar, output_fhe_type, result)
    }

    pub fn fhe_binary_op_and_bind_output(
        ctx: Context<FheBinaryOpAndBindOutput>,
        op: FheBinaryOpCode,
        lhs: [u8; 32],
        rhs: [u8; 32],
        scalar: bool,
        output_fhe_type: u8,
        result: [u8; 32],
        output_nonce_key: [u8; 32],
        output_nonce_sequence: u64,
        output_acl_domain_key: Pubkey,
        output_app_account: Pubkey,
        output_encrypted_value_label: [u8; 32],
        output_subjects: Vec<AclSubjectEntry>,
        output_public_decrypt: bool,
    ) -> Result<()> {
        instructions::fhe_binary_op_and_bind_output(
            ctx,
            op,
            lhs,
            rhs,
            scalar,
            output_fhe_type,
            result,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects,
            output_public_decrypt,
        )
    }

    pub fn fhe_ternary_op_and_bind_output(
        ctx: Context<FheTernaryOpAndBindOutput>,
        op: FheTernaryOpCode,
        control: [u8; 32],
        if_true: [u8; 32],
        if_false: [u8; 32],
        output_fhe_type: u8,
        result: [u8; 32],
        output_nonce_key: [u8; 32],
        output_nonce_sequence: u64,
        output_acl_domain_key: Pubkey,
        output_app_account: Pubkey,
        output_encrypted_value_label: [u8; 32],
        output_subjects: Vec<AclSubjectEntry>,
        output_public_decrypt: bool,
    ) -> Result<()> {
        instructions::fhe_ternary_op_and_bind_output(
            ctx,
            op,
            control,
            if_true,
            if_false,
            output_fhe_type,
            result,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects,
            output_public_decrypt,
        )
    }

    pub fn fhe_eval<'info>(ctx: Context<'info, FheEval<'info>>, args: FheEvalArgs) -> Result<()> {
        instructions::fhe_eval(ctx, args)
    }

    pub fn fhe_rand_and_bind(
        ctx: Context<FheRandAndBind>,
        fhe_type: u8,
        output_nonce_key: [u8; 32],
        output_nonce_sequence: u64,
        output_acl_domain_key: Pubkey,
        output_app_account: Pubkey,
        output_encrypted_value_label: [u8; 32],
        output_subjects: Vec<AclSubjectEntry>,
        output_public_decrypt: bool,
    ) -> Result<()> {
        instructions::fhe_rand_and_bind(
            ctx,
            fhe_type,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects,
            output_public_decrypt,
        )
    }

    pub fn fhe_rand_bounded_and_bind(
        ctx: Context<FheRandBoundedAndBind>,
        upper_bound: [u8; 32],
        fhe_type: u8,
        output_nonce_key: [u8; 32],
        output_nonce_sequence: u64,
        output_acl_domain_key: Pubkey,
        output_app_account: Pubkey,
        output_encrypted_value_label: [u8; 32],
        output_subjects: Vec<AclSubjectEntry>,
        output_public_decrypt: bool,
    ) -> Result<()> {
        instructions::fhe_rand_bounded_and_bind(
            ctx,
            upper_bound,
            fhe_type,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects,
            output_public_decrypt,
        )
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

    #[cfg(feature = "poc")]
    pub fn test_emit_input_verified(
        ctx: Context<TestEmitProtocolEvent>,
        input_handle: [u8; 32],
        result_handle: [u8; 32],
        user: Pubkey,
        acl_domain_key: Pubkey,
    ) -> Result<()> {
        instructions::test_emit_input_verified(
            ctx,
            input_handle,
            result_handle,
            user,
            acl_domain_key,
        )
    }
}
