//! State and deterministic helper functions for the ZamaHost program.
//!
//! This module is intentionally reusable from app programs and tests. It
//! exposes the PDA seeds, role flags, account layouts, and handle formulas
//! needed to prepare CPI accounts and to verify host-owned ACL state off-chain.

use anchor_lang::prelude::*;
use solana_keccak_hasher::hashv as keccak_hashv;
use solana_sha256_hasher::hashv;
use solana_sysvar::get_sysvar;

use crate::constants::{
    COMPUTATION_DOMAIN_SEPARATOR, COMPUTED_HANDLE_MARKER, RANDOM_SEED_DOMAIN_SEPARATOR,
};
use crate::errors::ZamaHostError;

pub mod deny_subject_record;
pub mod encrypted_value;
pub mod host_chain_address;
pub mod host_config;
pub mod kms_context;
pub mod user_decryption_delegation;

pub use deny_subject_record::*;
pub use encrypted_value::*;
pub use host_chain_address::*;
pub use host_config::*;
pub use kms_context::*;
pub use user_decryption_delegation::*;

pub use crate::constants::*;

/// Initialization arguments for the singleton [`HostConfig`] account.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitializeHostConfigArgs {
    /// Host-chain id encoded into newly derived handles.
    pub chain_id: u64,
    /// Authority used by mock and signed encrypted-input bind paths.
    pub input_verifier_authority: Pubkey,
    /// EVM gateway chain id used in the coprocessor/KMS EIP-712 domain separators.
    pub gateway_chain_id: u64,
    /// EVM `InputVerification` contract address (EIP-712 verifying contract).
    pub input_verification_contract: [u8; 20],
    /// Authorized coprocessor EVM signer for input attestations (v0: single signer).
    pub coprocessor_signer: [u8; 20],
    /// EVM `Decryption` contract address (EIP-712 verifying contract for KMS certs).
    pub decryption_contract: [u8; 20],
    /// Authority allowed to commit ciphertext material readiness.
    pub material_authority: Pubkey,
    /// Authority allowed to call `test_emit_*` event shims.
    pub test_authority: Pubkey,
    /// Whether the mock encrypted-input bind path is enabled.
    pub mock_input_enabled: bool,
    /// Whether test event shims are enabled.
    pub test_shims_enabled: bool,
    /// Whether persistent grants must include a deny-list witness.
    pub grant_deny_list_enabled: bool,
}

/// Pubkey plus role flags stored inline or in an overflow permission PDA.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AclSubjectEntry {
    /// Subject identity. For app users this is usually the owner pubkey; for
    /// app programs this can be a program-controlled compute signer PDA.
    pub pubkey: Pubkey,
    /// Bitset of `ACL_ROLE_*` flags.
    pub role_flags: u8,
}

impl AclSubjectEntry {
    /// Builds an owner/user subject with use, grant, and public-decrypt roles.
    pub fn user(pubkey: Pubkey) -> Self {
        Self {
            pubkey,
            role_flags: ACL_ROLE_USER,
        }
    }

    /// Builds a compute subject without public-decrypt authority.
    pub fn compute(pubkey: Pubkey) -> Self {
        Self {
            pubkey,
            role_flags: ACL_ROLE_COMPUTE_SUBJECT,
        }
    }

    /// Builds a subject that may use the handle but may not grant or publish it.
    pub fn use_only(pubkey: Pubkey) -> Self {
        Self {
            pubkey,
            role_flags: ACL_ROLE_USE,
        }
    }
}

/// Binary FHE operators currently modeled by the PoC.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FheBinaryOpCode {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Greater-than-or-equal comparison.
    Ge,
}

/// Ternary FHE operators currently modeled by the PoC.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FheTernaryOpCode {
    /// Selects `if_true` when `control` is true, otherwise `if_false`.
    IfThenElse,
}

/// Arguments for composed instruction-local FHE evaluation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct FheEvalArgs {
    /// Caller-chosen domain separator for the instruction-local handles in this eval.
    ///
    /// Callers should use a fresh value for each logical eval session.
    pub context_id: [u8; 32],
    /// Ordered step list. Each `AllowedLocal` operand may only reference an output
    /// produced by an earlier index in this vector.
    pub steps: Vec<FheEvalStep>,
}

/// One step inside a composed FHE eval.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheEvalStep {
    /// Binary operator step.
    Binary {
        /// Binary operator.
        op: FheBinaryOpCode,
        /// Left-hand encrypted operand.
        lhs: FheEvalOperand,
        /// Right-hand encrypted operand or scalar bytes.
        rhs: FheEvalOperand,
        /// FHE type byte embedded in the output handle.
        output_fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into durable ACL state.
        output: FheEvalOutput,
    },
    /// Ternary operator step.
    Ternary {
        /// Ternary operator.
        op: FheTernaryOpCode,
        /// Encrypted bool control operand.
        control: FheEvalOperand,
        /// Encrypted branch selected when control is true.
        if_true: FheEvalOperand,
        /// Encrypted branch selected when control is false.
        if_false: FheEvalOperand,
        /// FHE type byte embedded in the output handle.
        output_fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into durable ACL state.
        output: FheEvalOutput,
    },
    /// Trivial encryption step.
    TrivialEncrypt {
        /// Plaintext bytes encoded using the host scalar convention.
        plaintext: [u8; 32],
        /// FHE type byte embedded in the output handle.
        fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into durable ACL state.
        output: FheEvalOutput,
    },
    /// Random ciphertext step.
    Rand {
        /// FHE type byte embedded in the output handle.
        fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into durable ACL state.
        output: FheEvalOutput,
    },
}

/// A coprocessor input attestation carried inline by a [`FheEvalOperand::VerifiedInput`], re-verified
/// in-frame (no account, no PDA) — the instruction-local analog of EVM `allowTransient(input, contract)`.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct CoprocessorInputAttestation {
    /// The verified input handle used as the operand.
    pub input_handle: [u8; 32],
    /// All ciphertext handles covered by the proof.
    pub ct_handles: Vec<[u8; 32]>,
    /// Index of `input_handle` within `ct_handles`.
    pub handle_index: u8,
    /// Attested user identity (bytes32).
    pub user_address: [u8; 32],
    /// Attested contract identity — the input's ACL domain key (bytes32).
    pub contract_address: [u8; 32],
    /// Gateway-side contract chain id the attestation binds.
    pub contract_chain_id: u64,
    /// Opaque extra data covered by the attestation.
    pub extra_data: Vec<u8>,
    /// Coprocessor EIP-712 signatures (65-byte secp256k1).
    pub signatures: Vec<[u8; 65]>,
}

/// Operand source for a composed FHE eval operation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheEvalOperand {
    /// Input allowed through durable ACL state: a canonical `EncryptedValue`
    /// account in `remaining_accounts` whose current handle matches.
    AllowedDurable {
        /// Handle expected as the encrypted value's current handle.
        handle: [u8; 32],
        /// Index into `remaining_accounts` for the `EncryptedValue` account.
        encrypted_value_index: u16,
    },
    /// Instruction-local value produced by an earlier operation in this `fhe_eval`; allowed only
    /// inside the current evaluation scope and never stored.
    AllowedLocal {
        /// Producer operation index.
        producer_index: u16,
    },
    /// Plaintext scalar bytes. Scalar operands are only valid on the RHS.
    Scalar([u8; 32]),
    /// External encrypted input verified in-frame by re-running the coprocessor attestation.
    /// The "allow" is instruction-local (no ACL record, no session, no PDA): the input is usable
    /// only where it is consumed in the same `fhe_eval`. Valid as an encrypted operand, not a scalar.
    VerifiedInput {
        /// The inline attestation re-verified to authorize this operand.
        attestation: CoprocessorInputAttestation,
    },
}

/// Output policy for a composed FHE eval operation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheEvalOutput {
    /// Output stays allowed only inside the current `fhe_eval` scope; no durable ACL record.
    AllowedLocal,
    /// Output is bound into durable ACL state: the `EncryptedValue` lineage PDA
    /// is created when absent, or superseded (`update_encrypted_value`
    /// semantics) when it exists.
    AllowedDurable {
        /// Index into `remaining_accounts` for the output `EncryptedValue` PDA.
        output_encrypted_value_index: u16,
        /// Optional index into `remaining_accounts` for the app account authority signer.
        ///
        /// `None` uses the fixed `app_account_authority` account in the eval
        /// context. `Some(index)` requires that remaining account to be a signer
        /// and to match `output_app_account`.
        output_app_account_authority_index: Option<u16>,
        /// ACL domain key for the output lineage.
        output_acl_domain_key: Pubkey,
        /// App account authorized to bind the output lineage.
        output_app_account: Pubkey,
        /// Encrypted value label for the output lineage.
        output_encrypted_value_label: [u8; 32],
        /// Subjects on the output lineage. On create these are the initial
        /// subjects; on supersede they must equal the stored subjects exactly.
        output_subjects: Vec<AclSubjectEntry>,
        /// Superseded handle: `None` on create, `Some(current_handle)` on update.
        /// Carried in instruction data so indexers can reconstruct the appended
        /// MMR leaves without reading the account; validated against the account.
        previous_handle: Option<[u8; 32]>,
        /// Superseded subject set, parallel to `previous_handle` (`None` on create,
        /// exact stored subjects on update). Same indexer-reconstruction purpose.
        previous_subjects: Option<Vec<Pubkey>>,
    },
}

impl FheBinaryOpCode {
    /// Stable byte encoding used in handle derivation and events.
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Add => 0,
            Self::Sub => 1,
            Self::Ge => 14,
        }
    }
}

impl FheTernaryOpCode {
    /// Stable byte encoding used in handle derivation and events.
    pub fn as_u8(self) -> u8 {
        match self {
            Self::IfThenElse => 25,
        }
    }
}

/// Returns true when `role_flags` contains every flag in `role`.
pub fn subject_has_role(role_flags: u8, role: u8) -> bool {
    role_flags & role == role
}

/// Returns true when a role bitset is nonempty and uses only known role bits.
pub fn role_flags_are_known(role_flags: u8) -> bool {
    role_flags != 0 && role_flags & !ACL_ROLE_KNOWN == 0
}

/// Returns the chain id embedded in a handle.
pub fn handle_chain_id(handle: [u8; 32]) -> u64 {
    let mut chain_id = [0u8; 8];
    chain_id.copy_from_slice(&handle[22..30]);
    u64::from_be_bytes(chain_id)
}

/// Returns the FHE type id embedded in a handle.
pub fn handle_fhe_type(handle: [u8; 32]) -> u8 {
    handle[30]
}

/// Checks that a handle targets this host chain and uses supported metadata.
pub fn assert_handle_for_chain(handle: [u8; 32], chain_id: u64) -> Result<()> {
    require!(
        handle_chain_id(handle) == chain_id,
        ZamaHostError::InvalidInputHandleChain
    );
    require!(
        handle[31] == HANDLE_VERSION,
        ZamaHostError::InvalidInputHandleVersion
    );
    require!(
        is_supported_fhe_type(handle_fhe_type(handle)),
        ZamaHostError::InvalidInputHandleType
    );
    Ok(())
}

/// Checks that an external encrypted-input handle targets this host chain.
pub fn assert_input_handle_for_chain(handle: [u8; 32], chain_id: u64) -> Result<()> {
    assert_handle_for_chain(handle, chain_id)?;
    require!(
        handle[21] != COMPUTED_HANDLE_MARKER,
        ZamaHostError::InvalidInputHandle
    );
    Ok(())
}

/// Checks that an external encrypted-input handle is in the selected proof slot.
pub fn assert_input_handle_metadata(
    handle: [u8; 32],
    chain_id: u64,
    handle_index: u8,
) -> Result<()> {
    assert_input_handle_for_chain(handle, chain_id)?;
    require!(
        handle[21] == handle_index,
        ZamaHostError::InvalidInputHandleIndex
    );
    Ok(())
}

pub fn assert_supported_fhe_type(fhe_type: u8) -> Result<()> {
    require!(
        is_supported_fhe_type(fhe_type),
        ZamaHostError::UnsupportedFheType
    );
    Ok(())
}

/// Checks that a binary operation's declared result type matches the shipped operator.
pub fn assert_supported_binary_output_type(op: FheBinaryOpCode, fhe_type: u8) -> Result<()> {
    assert_supported_fhe_type(fhe_type)?;
    let valid = match op {
        FheBinaryOpCode::Add | FheBinaryOpCode::Sub => matches!(fhe_type, 2..=6),
        FheBinaryOpCode::Ge => fhe_type == 0,
    };
    require!(valid, ZamaHostError::UnsupportedFheType);
    Ok(())
}

/// Checks binary operand metadata against the EVM executor's type discipline.
pub fn assert_binary_operand_types(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    output_fhe_type: u8,
) -> Result<()> {
    assert_supported_binary_output_type(op, output_fhe_type)?;
    let lhs_type = handle_fhe_type(lhs);
    require!(matches!(lhs_type, 2..=6), ZamaHostError::UnsupportedFheType);
    if matches!(op, FheBinaryOpCode::Add | FheBinaryOpCode::Sub) {
        require!(
            lhs_type == output_fhe_type,
            ZamaHostError::BinaryOperandTypeMismatch
        );
    }
    if !scalar {
        require!(
            handle_fhe_type(rhs) == lhs_type,
            ZamaHostError::BinaryOperandTypeMismatch
        );
    }
    Ok(())
}

pub fn assert_supported_rand_type(fhe_type: u8) -> Result<()> {
    require!(
        matches!(fhe_type, 0 | 2 | 3 | 4 | 5 | 6 | 8),
        ZamaHostError::UnsupportedFheType
    );
    Ok(())
}

pub fn assert_supported_bounded_rand_type(fhe_type: u8) -> Result<()> {
    require!(
        bounded_rand_type_bits(fhe_type).is_some(),
        ZamaHostError::UnsupportedFheType
    );
    Ok(())
}

pub fn assert_valid_bounded_rand_upper_bound(upper_bound: [u8; 32], fhe_type: u8) -> Result<()> {
    assert_supported_bounded_rand_type(fhe_type)?;
    let bit_index =
        power_of_two_bit_index(upper_bound).ok_or(ZamaHostError::InvalidRandomUpperBound)?;
    if let Some(max_bits) = bounded_rand_type_bits(fhe_type).flatten() {
        require!(
            bit_index <= max_bits,
            ZamaHostError::InvalidRandomUpperBound
        );
    }
    Ok(())
}

fn is_supported_fhe_type(fhe_type: u8) -> bool {
    matches!(fhe_type, 0 | 2 | 3 | 4 | 5 | 6 | 7 | 8)
}

fn bounded_rand_type_bits(fhe_type: u8) -> Option<Option<u16>> {
    match fhe_type {
        2 => Some(Some(8)),
        3 => Some(Some(16)),
        4 => Some(Some(32)),
        5 => Some(Some(64)),
        6 => Some(Some(128)),
        8 => Some(None),
        _ => None,
    }
}

fn power_of_two_bit_index(value: [u8; 32]) -> Option<u16> {
    let mut bit_index = None;
    for (byte_index, byte) in value.iter().enumerate() {
        if *byte == 0 {
            continue;
        }
        if byte.count_ones() != 1 || bit_index.is_some() {
            return None;
        }
        let bit_in_byte = 7 - byte.leading_zeros() as u16;
        bit_index = Some(((31 - byte_index) as u16 * 8) + bit_in_byte);
    }
    bit_index
}

/// Returns the canonical singleton host config address.
pub fn host_config_address() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[HOST_CONFIG_SEED], &crate::ID)
}

/// Returns the canonical KMS context address for a context id.
pub fn kms_context_address(context_id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[KMS_CONTEXT_SEED, &context_id.to_le_bytes()], &crate::ID)
}

/// Returns the canonical deny-list address for a subject.
pub fn deny_subject_address(subject: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[DENY_SUBJECT_SEED, subject.as_ref()], &crate::ID)
}

/// Returns the canonical user-decryption delegation address.
pub fn user_decryption_delegation_address(
    delegator: Pubkey,
    delegate: Pubkey,
    app_account: Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            DELEGATION_SEED,
            delegator.as_ref(),
            delegate.as_ref(),
            app_account.as_ref(),
        ],
        &crate::ID,
    )
}

/// Derives an unbound binary-op handle using the current slot context.
///
/// This helper uses [`SOLANA_POC_CHAIN_ID`]. CPI callers that have a
/// [`HostConfig`] should prefer [`computed_binary_handle_for_current_slot_with_chain_id`].
pub fn computed_binary_handle_for_current_slot(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
) -> Result<[u8; 32]> {
    computed_binary_handle_for_current_slot_with_chain_id(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        SOLANA_POC_CHAIN_ID,
    )
}

/// Derives an unbound binary-op handle using the current slot context and chain id.
pub fn computed_binary_handle_for_current_slot_with_chain_id(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
) -> Result<[u8; 32]> {
    computed_binary_handle_for_current_slot_with_chain_id_and_test_fallback(
        op, lhs, rhs, scalar, fhe_type, chain_id, false,
    )
}

/// Derives an unbound binary-op handle, optionally using the local-test zero
/// fallback when explicitly allowed by host config.
pub fn computed_binary_handle_for_current_slot_with_chain_id_and_test_fallback(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    allow_test_zero: bool,
) -> Result<[u8; 32]> {
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash_with_test_fallback(clock.slot, allow_test_zero)?;
    Ok(computed_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
    ))
}

/// Derives a nonce-bound binary-op output handle using the current slot context.
///
/// This helper uses [`SOLANA_POC_CHAIN_ID`]. CPI callers that have a
/// [`HostConfig`] should prefer
/// [`computed_bound_binary_handle_for_current_slot_with_chain_id`].
pub fn computed_bound_binary_handle_for_current_slot(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    computed_bound_binary_handle_for_current_slot_with_chain_id(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        SOLANA_POC_CHAIN_ID,
        output_nonce_key,
        output_nonce_sequence,
    )
}

/// Derives a nonce-bound binary-op output handle using the current slot context and chain id.
pub fn computed_bound_binary_handle_for_current_slot_with_chain_id(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    computed_bound_binary_handle_for_current_slot_with_chain_id_and_test_fallback(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        output_nonce_key,
        output_nonce_sequence,
        false,
    )
}

/// Derives a nonce-bound binary-op output handle, optionally using the
/// local-test zero fallback when explicitly allowed by host config.
#[allow(clippy::too_many_arguments)]
pub fn computed_bound_binary_handle_for_current_slot_with_chain_id_and_test_fallback(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    allow_test_zero: bool,
) -> Result<[u8; 32]> {
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash_with_test_fallback(clock.slot, allow_test_zero)?;
    Ok(computed_bound_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    ))
}

/// Derives a nonce-bound ternary-op output handle using the current slot context and chain id.
pub fn computed_bound_ternary_handle_for_current_slot_with_chain_id(
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    computed_bound_ternary_handle_for_current_slot_with_chain_id_and_test_fallback(
        op,
        control,
        if_true,
        if_false,
        fhe_type,
        chain_id,
        output_nonce_key,
        output_nonce_sequence,
        false,
    )
}

/// Derives a nonce-bound ternary-op output handle, optionally using the
/// local-test zero fallback when explicitly allowed by host config.
#[allow(clippy::too_many_arguments)]
pub fn computed_bound_ternary_handle_for_current_slot_with_chain_id_and_test_fallback(
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    allow_test_zero: bool,
) -> Result<[u8; 32]> {
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash_with_test_fallback(clock.slot, allow_test_zero)?;
    Ok(computed_bound_ternary_handle(
        op,
        control,
        if_true,
        if_false,
        fhe_type,
        chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    ))
}

/// Derives an instruction-local eval handle using the current slot context.
///
/// This helper uses [`SOLANA_POC_CHAIN_ID`]. CPI callers that have a
/// [`HostConfig`] should prefer [`computed_eval_handle_for_current_slot_with_chain_id`].
pub fn computed_eval_handle_for_current_slot(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    context_id: [u8; 32],
    op_index: u16,
) -> Result<[u8; 32]> {
    computed_eval_handle_for_current_slot_with_chain_id(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        SOLANA_POC_CHAIN_ID,
        context_id,
        op_index,
    )
}

/// Derives an instruction-local eval handle using the current slot context and chain id.
pub fn computed_eval_handle_for_current_slot_with_chain_id(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    context_id: [u8; 32],
    op_index: u16,
) -> Result<[u8; 32]> {
    computed_eval_handle_for_current_slot_with_chain_id_and_test_fallback(
        op, lhs, rhs, scalar, fhe_type, chain_id, context_id, op_index, false,
    )
}

/// Derives an instruction-local eval handle, optionally using the local-test
/// zero fallback when explicitly allowed by host config.
#[allow(clippy::too_many_arguments)]
pub fn computed_eval_handle_for_current_slot_with_chain_id_and_test_fallback(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    context_id: [u8; 32],
    op_index: u16,
    allow_test_zero: bool,
) -> Result<[u8; 32]> {
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash_with_test_fallback(clock.slot, allow_test_zero)?;
    Ok(computed_eval_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
        context_id,
        op_index,
    ))
}

/// Derives a nonce-bound eval output handle using the current slot context.
///
/// This helper uses [`SOLANA_POC_CHAIN_ID`]. CPI callers that have a
/// [`HostConfig`] should prefer
/// [`computed_bound_eval_handle_for_current_slot_with_chain_id`].
pub fn computed_bound_eval_handle_for_current_slot(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    context_id: [u8; 32],
    op_index: u16,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    computed_bound_eval_handle_for_current_slot_with_chain_id(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        SOLANA_POC_CHAIN_ID,
        context_id,
        op_index,
        output_nonce_key,
        output_nonce_sequence,
    )
}

/// Derives a nonce-bound eval output handle using the current slot context and chain id.
pub fn computed_bound_eval_handle_for_current_slot_with_chain_id(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    context_id: [u8; 32],
    op_index: u16,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    computed_bound_eval_handle_for_current_slot_with_chain_id_and_test_fallback(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        context_id,
        op_index,
        output_nonce_key,
        output_nonce_sequence,
        false,
    )
}

/// Derives a nonce-bound eval output handle, optionally using the local-test
/// zero fallback when explicitly allowed by host config.
#[allow(clippy::too_many_arguments)]
pub fn computed_bound_eval_handle_for_current_slot_with_chain_id_and_test_fallback(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    context_id: [u8; 32],
    op_index: u16,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    allow_test_zero: bool,
) -> Result<[u8; 32]> {
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash_with_test_fallback(clock.slot, allow_test_zero)?;
    Ok(computed_bound_eval_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
        context_id,
        op_index,
        output_nonce_key,
        output_nonce_sequence,
    ))
}

fn finish_computed_handle(result: &mut [u8; 32], chain_id_bytes: &[u8; 8], fhe_type: u8) {
    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
}

/// Derives an unbound binary-op handle from explicit slot entropy.
pub fn computed_binary_handle(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> [u8; 32] {
    let op_byte = [op.as_u8()];
    let scalar_byte = [u8::from(scalar)];
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let mut result = keccak_hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &op_byte,
        &lhs,
        &rhs,
        &scalar_byte,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();

    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives a nonce-bound binary-op output handle from explicit slot entropy.
///
/// Binding the handle to `(output_nonce_key, output_nonce_sequence)` prevents
/// two durable output ACL records from intentionally storing the same computed
/// handle for the same operation.
pub fn computed_bound_binary_handle(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let base_result = computed_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        previous_bank_hash,
        unix_timestamp,
    );
    let mut result = base_result;
    result[..21].copy_from_slice(
        &keccak_hashv(&[
            b"FHE_bound_output",
            &base_result,
            &output_nonce_key,
            &sequence_bytes,
        ])
        .to_bytes()[..21],
    );
    result
}

/// Derives an unbound ternary-op handle from explicit slot entropy.
pub fn computed_ternary_handle(
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> [u8; 32] {
    let op_byte = [op.as_u8()];
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let mut result = keccak_hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &op_byte,
        &control,
        &if_true,
        &if_false,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();

    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives a nonce-bound ternary-op output handle from explicit slot entropy.
pub fn computed_bound_ternary_handle(
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let base_result = computed_ternary_handle(
        op,
        control,
        if_true,
        if_false,
        fhe_type,
        chain_id,
        previous_bank_hash,
        unix_timestamp,
    );
    let mut result = base_result;
    result[..21].copy_from_slice(
        &keccak_hashv(&[
            b"FHE_bound_ternary_output",
            &base_result,
            &output_nonce_key,
            &sequence_bytes,
        ])
        .to_bytes()[..21],
    );
    result
}

/// Derives an instruction-local eval operation handle from explicit slot entropy.
pub fn computed_eval_handle(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
) -> [u8; 32] {
    let op_byte = [op.as_u8()];
    let scalar_byte = [u8::from(scalar)];
    let chain_id_bytes = chain_id.to_be_bytes();
    let op_index_bytes = op_index.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let mut result = keccak_hashv(&[
        b"FHE_eval",
        &context_id,
        &op_index_bytes,
        &op_byte,
        &lhs,
        &rhs,
        &scalar_byte,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();

    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives an instruction-local ternary eval operation handle from explicit slot entropy.
#[allow(clippy::too_many_arguments)]
pub fn computed_eval_ternary_handle(
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
) -> [u8; 32] {
    let op_byte = [op.as_u8()];
    let chain_id_bytes = chain_id.to_be_bytes();
    let op_index_bytes = op_index.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let mut result = hashv(&[
        b"FHE_eval_ternary",
        &context_id,
        &op_index_bytes,
        &op_byte,
        &control,
        &if_true,
        &if_false,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();

    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives an instruction-local trivial-encrypt eval handle from explicit slot entropy.
pub fn computed_eval_trivial_handle(
    plaintext: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
) -> [u8; 32] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let op_index_bytes = op_index.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let mut result = hashv(&[
        b"FHE_eval_trivial",
        &context_id,
        &op_index_bytes,
        &plaintext,
        &fhe_type_bytes,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();

    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives the seed emitted for an instruction-local eval random handle.
pub fn computed_eval_rand_seed(
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
) -> [u8; 16] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let op_index_bytes = op_index.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let hash = hashv(&[
        b"FHE_eval_seed",
        &context_id,
        &op_index_bytes,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();
    let mut seed = [0; 16];
    seed.copy_from_slice(&hash[..16]);
    seed
}

/// Derives a nonce-bound durable output handle for composed eval from explicit slot entropy.
pub fn computed_bound_eval_handle(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let base_result = computed_eval_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        op_index,
    );
    let mut result = base_result;
    result[..21].copy_from_slice(
        &keccak_hashv(&[
            b"FHE_bound_eval_output",
            &base_result,
            &output_nonce_key,
            &sequence_bytes,
        ])
        .to_bytes()[..21],
    );
    result
}

/// Derives a nonce-bound durable ternary output handle for composed eval from explicit slot entropy.
#[allow(clippy::too_many_arguments)]
pub fn computed_bound_eval_ternary_handle(
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let base_result = computed_eval_ternary_handle(
        op,
        control,
        if_true,
        if_false,
        fhe_type,
        chain_id,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        op_index,
    );
    let mut result = base_result;
    result[..21].copy_from_slice(
        &hashv(&[
            b"FHE_bound_eval_output",
            &base_result,
            &output_nonce_key,
            &sequence_bytes,
        ])
        .to_bytes()[..21],
    );
    result
}

/// Derives a nonce-bound durable trivial-encrypt eval handle from explicit slot entropy.
pub fn computed_bound_eval_trivial_handle(
    plaintext: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let op_index_bytes = op_index.to_be_bytes();
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let mut result = hashv(&[
        b"FHE_bound_eval_trivial_output",
        &context_id,
        &op_index_bytes,
        &plaintext,
        &fhe_type_bytes,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
        &output_nonce_key,
        &sequence_bytes,
    ])
    .to_bytes();
    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives the seed emitted for a nonce-bound durable eval random handle from explicit slot entropy.
pub fn computed_bound_eval_rand_seed(
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 16] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let op_index_bytes = op_index.to_be_bytes();
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let hash = hashv(&[
        b"FHE_bound_eval_seed",
        &context_id,
        &op_index_bytes,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
        &output_nonce_key,
        &sequence_bytes,
    ])
    .to_bytes();
    let mut seed = [0; 16];
    seed.copy_from_slice(&hash[..16]);
    seed
}

/// Derives a trivial-encrypt handle from explicit slot entropy.
pub fn computed_trivial_handle(
    plaintext: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let mut result = keccak_hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &[2],
        &plaintext,
        &fhe_type_bytes,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
        &output_nonce_key,
        &sequence_bytes,
    ])
    .to_bytes();

    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives the random seed emitted for a durable random-handle birth.
pub fn computed_rand_seed(
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 16] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let hash = keccak_hashv(&[
        RANDOM_SEED_DOMAIN_SEPARATOR,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
        &output_nonce_key,
        &sequence_bytes,
    ])
    .to_bytes();
    let mut seed = [0; 16];
    seed.copy_from_slice(&hash[..16]);
    seed
}

/// Deterministically derives a random-ciphertext handle from the emitted seed.
pub fn computed_rand_handle(seed: [u8; 16], fhe_type: u8, chain_id: u64) -> [u8; 32] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let mut result = keccak_hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &[3],
        &fhe_type_bytes,
        &seed,
        crate::ID.as_ref(),
        &chain_id_bytes,
    ])
    .to_bytes();

    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(&chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
    result
}

/// Deterministically derives a bounded-random ciphertext handle from the emitted seed.
pub fn computed_rand_bounded_handle(
    upper_bound: [u8; 32],
    seed: [u8; 16],
    fhe_type: u8,
    chain_id: u64,
) -> [u8; 32] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let mut result = keccak_hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &[4],
        &upper_bound,
        &fhe_type_bytes,
        &seed,
        crate::ID.as_ref(),
        &chain_id_bytes,
    ])
    .to_bytes();

    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(&chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
    result
}

/// Returns the latest prior bank hash.
///
/// Handle derivation must fail closed when the runtime cannot provide the
/// prior bank hash. Solana can skip slots, so this uses the most recent
/// `SlotHashes` entry below `current_slot` rather than requiring
/// `current_slot - 1` to exist.
pub fn previous_bank_hash(current_slot: u64) -> Result<[u8; 32]> {
    if current_slot == 0 {
        return Err(error!(ZamaHostError::PreviousBankHashUnavailable));
    }
    // `PodSlotHashes::fetch()` (solana-sysvar 3.1.1) allocates an align-1 `Vec<u8>` and then
    // rejects it with an 8-byte alignment check; the SBF bump allocator does not 8-align
    // align-1 allocations, so fetch() fails on a real validator (LiteSVM/mollusk mock the
    // `sol_get_sysvar` syscall and never exercise this allocation). Read the sysvar into an
    // 8-aligned buffer ourselves via the same syscall, then scan the entries — which are laid
    // out as `[u64 count][ (u64 slot, [u8;32] hash) ...]` — for the most recent slot below
    // `current_slot`.
    //
    // `SlotHashes` is ordered newest-first and every entry is a prior slot (slot < the
    // executing `current_slot`), so the answer is in the first entries. Read only a small
    // window rather than the full 20_488-byte sysvar: on the SBF bump allocator (default 32KB
    // heap, never freed) a 20KB buffer per call means a second FHE op in the same instruction
    // — e.g. wrap_usdc's balance-add then total-supply-add — runs out of heap. A small window
    // keeps the read well within the default heap.
    const ENTRY_LEN: usize = 40; // u64 slot + 32-byte hash
    const MAX_SCAN_ENTRIES: usize = 16;

    // Read the 8-byte entry count first (8-aligned stack buffer).
    let mut count_word = [0u64; 1];
    let count_bytes =
        unsafe { core::slice::from_raw_parts_mut(count_word.as_mut_ptr() as *mut u8, 8) };
    get_sysvar(count_bytes, &solana_sysvar::slot_hashes::id(), 0, 8)
        .map_err(|_| error!(ZamaHostError::PreviousBankHashUnavailable))?;
    let count = count_word[0] as usize;
    if count == 0 {
        return Err(error!(ZamaHostError::PreviousBankHashUnavailable));
    }

    let scan = count.min(MAX_SCAN_ENTRIES);
    // 8-aligned heap buffer for the scanned entries; ENTRY_LEN (40) is a multiple of 8.
    let mut aligned = vec![0u64; (scan * ENTRY_LEN) / 8];
    let data: &mut [u8] = unsafe {
        core::slice::from_raw_parts_mut(aligned.as_mut_ptr() as *mut u8, scan * ENTRY_LEN)
    };
    get_sysvar(
        data,
        &solana_sysvar::slot_hashes::id(),
        8,
        (scan * ENTRY_LEN) as u64,
    )
    .map_err(|_| error!(ZamaHostError::PreviousBankHashUnavailable))?;

    let entries = (0..scan).filter_map(|index| {
        let offset = index * ENTRY_LEN;
        let slot = u64::from_le_bytes(data[offset..offset + 8].try_into().ok()?);
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&data[offset + 8..offset + ENTRY_LEN]);
        Some((slot, hash))
    });
    latest_prior_bank_hash_from_entries(current_slot, entries)
        .ok_or_else(|| error!(ZamaHostError::PreviousBankHashUnavailable))
}

/// Returns the previous slot hash, allowing a zero-hash fallback only for local
/// test configurations that explicitly enable test shims.
pub fn previous_bank_hash_with_test_fallback(
    current_slot: u64,
    allow_test_zero: bool,
) -> Result<[u8; 32]> {
    if allow_test_zero {
        return Ok([0; 32]);
    }
    previous_bank_hash(current_slot)
}

fn latest_prior_bank_hash_from_entries<I>(current_slot: u64, entries: I) -> Option<[u8; 32]>
where
    I: IntoIterator<Item = (u64, [u8; 32])>,
{
    entries
        .into_iter()
        .find_map(|(slot, hash)| (slot < current_slot).then_some(hash))
}

#[cfg(test)]
mod tests;
