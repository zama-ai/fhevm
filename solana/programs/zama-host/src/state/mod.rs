//! State and deterministic helper functions for the ZamaHost program.
//!
//! This module is intentionally reusable from app programs and tests. It
//! exposes the PDA seeds, role flags, account layouts, and handle formulas
//! needed to prepare CPI accounts and to verify host-owned ACL state off-chain.

use anchor_lang::prelude::*;
use solana_sha256_hasher::hashv;
use solana_sysvar::slot_hashes::PodSlotHashes;

use crate::constants::{
    COMPUTATION_DOMAIN_SEPARATOR, COMPUTED_HANDLE_MARKER, INPUT_PROOF_DOMAIN_SEPARATOR,
    RANDOM_SEED_DOMAIN_SEPARATOR,
};
use crate::errors::ZamaHostError;

pub mod acl_permission;
pub mod acl_record;
pub mod deny_subject_record;
pub mod handle_material_commitment;
pub mod host_config;
pub mod transient_session;
pub mod user_decryption_delegation;
pub mod verifier_set;

pub use acl_permission::*;
pub use acl_record::*;
pub use deny_subject_record::*;
pub use handle_material_commitment::*;
pub use host_config::*;
pub use transient_session::*;
pub use user_decryption_delegation::*;
pub use verifier_set::*;

pub use crate::constants::*;

/// Initialization arguments for the singleton [`HostConfig`] account.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitializeHostConfigArgs {
    /// Host-chain id encoded into newly derived handles.
    pub chain_id: u64,
    /// Active threshold verifier set used by encrypted-input bind paths.
    pub input_verifier_set: Pubkey,
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

/// Initialization arguments for a threshold [`VerifierSet`] account.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CreateVerifierSetArgs {
    /// Protocol purpose for this set.
    pub kind: u8,
    /// Scope that disambiguates sets of the same kind.
    pub scope: Pubkey,
    /// Monotonic version chosen by the admin for rotation.
    pub version: u64,
    /// Number of distinct signer signatures required.
    pub threshold: u8,
    /// Number of active entries in `signers`.
    pub signer_count: u8,
    /// Fixed signer list. Entries after `signer_count` must be default keys.
    pub signers: [Pubkey; MAX_VERIFIER_SET_SIGNERS],
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

/// Native Solana verifier payload for binding external encrypted inputs.
///
/// The proof is not trusted by itself. `verify_input_and_bind` requires an
/// Ed25519 verifier quorum from `HostConfig::input_verifier_set` over
/// [`input_proof_message_for_verifier_set`]. The selected handle from `handles` is the only
/// handle that may be written into the output ACL record.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SolanaInputProof {
    /// Ordered handles certified by the input verifier.
    pub handles: Vec<[u8; 32]>,
    /// Selected handle index for this bind operation.
    pub handle_index: u8,
    /// User associated with the encrypted input.
    pub user: Pubkey,
    /// App account to which this input authorization is scoped.
    pub app_account: Pubkey,
    /// App ACL domain to which this input authorization is scoped.
    pub acl_domain_key: Pubkey,
    /// Opaque verifier payload, such as transcript/proof metadata.
    pub extra_data: Vec<u8>,
}

/// ACL metadata covered by a native Solana encrypted-input verifier signature.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SolanaInputBindIntent {
    /// Nonce key for the output ACL record.
    pub output_nonce_key: [u8; 32],
    /// Nonce sequence for the output ACL record.
    pub output_nonce_sequence: u64,
    /// ACL domain key for the output ACL record.
    pub output_acl_domain_key: Pubkey,
    /// App account authorized to create the output ACL record.
    pub output_app_account: Pubkey,
    /// Encrypted value label for the output ACL record.
    pub output_encrypted_value_label: [u8; 32],
    /// Initial subjects on the output ACL record.
    pub output_subjects: Vec<AclSubjectEntry>,
    /// Initial public decrypt flag on the output ACL record.
    pub output_public_decrypt: bool,
}

/// Caller-supplied policy for adding a transient capability to a session.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransientCapabilityGrant {
    /// Subject allowed to use the handle through the session.
    pub subject: Pubkey,
    /// Top-level receiver program expected to consume the capability.
    pub receiver_program: Pubkey,
    /// App-level ACL domain to which durable output, if allowed, must remain bound.
    pub acl_domain_key: Pubkey,
    /// App account to which durable output, if allowed, must remain bound.
    pub app_account: Pubkey,
    /// Roles granted by the transient capability.
    pub role_flags: u8,
    /// Maximum number of successful uses before the capability is exhausted.
    ///
    /// The host currently requires this to be `1`.
    pub max_uses: u8,
    /// Whether a computation using this capability may bind a durable output ACL record.
    pub durable_output_allowed: bool,
    /// Whether a computation using this capability may make the durable output public-decryptable.
    pub public_decrypt_allowed: bool,
}

impl TransientCapabilityGrant {
    /// Serialized size of the account body.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 1 + 1 + 1 + 1;
}

/// One one-shot transient handle capability.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransientCapability {
    /// Handle controlled by the capability.
    pub handle: [u8; 32],
    /// Capability policy.
    pub grant: TransientCapabilityGrant,
    /// Number of times the capability has been consumed.
    pub used_count: u8,
}

impl TransientCapability {
    /// Serialized size of the account body.
    pub const SPACE: usize = 32 + TransientCapabilityGrant::SPACE + 1;
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
    /// Caller-chosen domain separator for transient handles in this eval.
    ///
    /// Callers should use a fresh value for each logical eval session.
    pub context_id: [u8; 32],
    /// Ordered step list. Each transient operand may only reference an output
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
    /// Externally verified input step.
    Input {
        /// Input handle selected from the proof.
        input_handle: [u8; 32],
        /// Solana input proof verified by the host.
        proof: SolanaInputProof,
        /// Index into `remaining_accounts` for the active input [`VerifierSet`].
        verifier_set_index: u16,
        /// Durable ACL output bound from the verified input.
        output: FheEvalOutput,
    },
}

/// Operand source for a composed FHE eval operation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheEvalOperand {
    /// Durable input authorized by a canonical ACL record in `remaining_accounts`.
    Durable {
        /// Handle expected in the ACL record.
        handle: [u8; 32],
        /// Index into `remaining_accounts` for the ACL record.
        acl_record_index: u16,
        /// Optional index into `remaining_accounts` for overflow subject permission.
        permission_index: Option<u16>,
    },
    /// Instruction-local output produced by an earlier operation.
    Transient {
        /// Producer operation index.
        producer_index: u16,
    },
    /// Sealed one-shot session capability in `remaining_accounts`.
    TransientSession {
        /// Handle expected in the session capability.
        handle: [u8; 32],
        /// Index into `remaining_accounts` for the transient session account.
        session_index: u16,
        /// Entry index inside the transient session.
        capability_index: u16,
    },
    /// Plaintext scalar bytes. Scalar operands are only valid on the RHS.
    Scalar([u8; 32]),
}

/// Output policy for a composed FHE eval operation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheEvalOutput {
    /// Output remains instruction-local and has no durable ACL record.
    Transient,
    /// Output is appended to an open transient session instead of durable ACL state.
    TransientSession {
        /// Index into `remaining_accounts` for the transient session account.
        session_index: u16,
        /// Capability policy for the produced result.
        capability: TransientCapabilityGrant,
    },
    /// Output is bound into a durable ACL record.
    Durable {
        /// Index into `remaining_accounts` for the output ACL record PDA.
        output_acl_record_index: u16,
        /// Optional index into `remaining_accounts` for the app account authority signer.
        ///
        /// `None` uses the fixed `app_account_authority` account in the eval
        /// context. `Some(index)` requires that remaining account to be a signer
        /// and to match `output_app_account`.
        output_app_account_authority_index: Option<u16>,
        /// Nonce key for the output ACL record.
        output_nonce_key: [u8; 32],
        /// Nonce sequence for the output ACL record.
        output_nonce_sequence: u64,
        /// ACL domain key for the output ACL record.
        output_acl_domain_key: Pubkey,
        /// App account authorized to create the output ACL record.
        output_app_account: Pubkey,
        /// Encrypted value label for the output ACL record.
        output_encrypted_value_label: [u8; 32],
        /// Initial subjects on the output ACL record.
        output_subjects: Vec<AclSubjectEntry>,
        /// Initial public decrypt flag on the output ACL record.
        output_public_decrypt: bool,
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

/// Returns true when the inline ACL subject arrays are exact and KMS-decodable.
pub fn acl_record_subject_slots_are_canonical(record: &AclRecord) -> bool {
    let subject_count = record.subject_count as usize;
    if subject_count > MAX_ACL_SUBJECTS {
        return false;
    }
    let mut index = 0usize;
    while index < subject_count {
        let subject = record.subjects[index];
        let role_flags = record.subject_roles[index];
        if subject == Pubkey::default() || !role_flags_are_known(role_flags) {
            return false;
        }
        let mut previous = 0usize;
        while previous < index {
            if record.subjects[previous] == subject {
                return false;
            }
            previous += 1;
        }
        index += 1;
    }
    while index < MAX_ACL_SUBJECTS {
        if record.subjects[index] != Pubkey::default() || record.subject_roles[index] != 0 {
            return false;
        }
        index += 1;
    }
    true
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

/// Canonical bytes signed by the native Solana input verifier authority.
pub fn input_proof_message(
    proof: &SolanaInputProof,
    bind_intent: &SolanaInputBindIntent,
    host_program_id: Pubkey,
    chain_id: u64,
) -> Vec<u8> {
    let mut message = Vec::with_capacity(
        INPUT_PROOF_DOMAIN_SEPARATOR.len()
            + 32
            + 8
            + 32
            + 32
            + 32
            + 32
            + 8
            + 32
            + 32
            + 32
            + 4
            + (bind_intent.output_subjects.len() * (32 + 1))
            + 1
            + 4
            + 1
            + (proof.handles.len() * 32)
            + 4
            + proof.extra_data.len(),
    );
    message.extend_from_slice(INPUT_PROOF_DOMAIN_SEPARATOR);
    message.extend_from_slice(host_program_id.as_ref());
    message.extend_from_slice(&chain_id.to_be_bytes());
    message.extend_from_slice(proof.user.as_ref());
    message.extend_from_slice(proof.app_account.as_ref());
    message.extend_from_slice(proof.acl_domain_key.as_ref());
    message.extend_from_slice(&bind_intent.output_nonce_key);
    message.extend_from_slice(&bind_intent.output_nonce_sequence.to_be_bytes());
    message.extend_from_slice(bind_intent.output_acl_domain_key.as_ref());
    message.extend_from_slice(bind_intent.output_app_account.as_ref());
    message.extend_from_slice(&bind_intent.output_encrypted_value_label);
    message.extend_from_slice(&(bind_intent.output_subjects.len() as u32).to_be_bytes());
    for subject in &bind_intent.output_subjects {
        message.extend_from_slice(subject.pubkey.as_ref());
        message.push(subject.role_flags);
    }
    message.push(u8::from(bind_intent.output_public_decrypt));
    message.extend_from_slice(&(proof.handles.len() as u32).to_be_bytes());
    message.push(proof.handle_index);
    for handle in &proof.handles {
        message.extend_from_slice(handle);
    }
    message.extend_from_slice(&(proof.extra_data.len() as u32).to_be_bytes());
    message.extend_from_slice(&proof.extra_data);
    message
}

/// Canonical bytes signed by a threshold verifier-set quorum.
pub fn input_proof_message_for_verifier_set(
    proof: &SolanaInputProof,
    bind_intent: &SolanaInputBindIntent,
    host_program_id: Pubkey,
    chain_id: u64,
    verifier_set: Pubkey,
    verifier_set_kind: u8,
    verifier_set_scope: Pubkey,
    verifier_set_version: u64,
) -> Vec<u8> {
    let legacy_message = input_proof_message(proof, bind_intent, host_program_id, chain_id);
    let mut message = Vec::with_capacity(legacy_message.len() + 32 + 1 + 32 + 8);
    message.extend_from_slice(&legacy_message);
    message.extend_from_slice(verifier_set.as_ref());
    message.push(verifier_set_kind);
    message.extend_from_slice(verifier_set_scope.as_ref());
    message.extend_from_slice(&verifier_set_version.to_be_bytes());
    message
}

/// Returns the canonical singleton host config address.
pub fn host_config_address() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[HOST_CONFIG_SEED], &crate::ID)
}

/// Returns the canonical verifier-set address for `(kind, scope, version)`.
pub fn verifier_set_address(kind: u8, scope: Pubkey, version: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            VERIFIER_SET_SEED,
            &[kind],
            scope.as_ref(),
            &version.to_le_bytes(),
        ],
        &crate::ID,
    )
}

/// Returns the canonical ACL record address for a nonce key and sequence.
pub fn acl_record_address(nonce_key: [u8; 32], nonce_sequence: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            ACL_RECORD_SEED,
            nonce_key.as_ref(),
            &nonce_sequence.to_le_bytes(),
        ],
        &crate::ID,
    )
}

/// Returns the canonical overflow permission address for a subject.
pub fn acl_permission_address(acl_record: Pubkey, subject: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[ACL_PERMISSION_SEED, acl_record.as_ref(), subject.as_ref()],
        &crate::ID,
    )
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

/// Returns the canonical transient session address for an authority and nonce.
pub fn transient_session_address(authority: Pubkey, session_nonce: [u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            TRANSIENT_SESSION_SEED,
            authority.as_ref(),
            session_nonce.as_ref(),
        ],
        &crate::ID,
    )
}

/// Returns the canonical material commitment address for an ACL record.
pub fn handle_material_address(acl_record: Pubkey) -> (Pubkey, u8) {
    let (host_config, _) = host_config_address();
    Pubkey::find_program_address(
        &[
            HANDLE_MATERIAL_SEED,
            host_config.as_ref(),
            acl_record.as_ref(),
        ],
        &crate::ID,
    )
}

/// Computes the commitment hash stored in [`HandleMaterialCommitment`].
pub fn handle_material_commitment_hash(
    material_commitment: Pubkey,
    acl_record: Pubkey,
    key_id: [u8; 32],
    ciphertext_digest: [u8; 32],
    sns_ciphertext_digest: [u8; 32],
    coprocessor_set_digest: [u8; 32],
) -> [u8; 32] {
    let (host_config, _) = host_config_address();
    hashv(&[
        b"zama-solana-material-commitment-v1",
        host_config.as_ref(),
        crate::ID.as_ref(),
        material_commitment.as_ref(),
        acl_record.as_ref(),
        &key_id,
        &ciphertext_digest,
        &sns_ciphertext_digest,
        &coprocessor_set_digest,
    ])
    .to_bytes()
}

/// Derives the app-controlled nonce key for one encrypted field.
///
/// The nonce key intentionally contains app metadata, not the opaque handle.
/// This lets the app predeclare the ACL account address before the host program
/// computes or binds the final handle.
pub fn acl_nonce_key(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> [u8; 32] {
    hashv(&[
        b"zama-acl-nonce-key-v1",
        acl_domain_key.as_ref(),
        app_account.as_ref(),
        &encrypted_value_label,
    ])
    .to_bytes()
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
    let mut result = hashv(&[
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
        &hashv(&[
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
    let mut result = hashv(&[
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
        &hashv(&[
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
    let mut result = hashv(&[
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
    let mut result = hashv(&[
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
    let hash = hashv(&[
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
    let mut result = hashv(&[
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
    let mut result = hashv(&[
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
    let slot_hashes =
        PodSlotHashes::fetch().map_err(|_| error!(ZamaHostError::PreviousBankHashUnavailable))?;
    let entries = slot_hashes
        .as_slice()
        .map_err(|_| error!(ZamaHostError::PreviousBankHashUnavailable))?
        .iter()
        .map(|slot_hash| (slot_hash.slot, slot_hash.hash.to_bytes()));
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

/// Returns true when an ACL record grants the base use role to an inline subject.
pub fn record_allows(record: &AclRecord, subject: Pubkey) -> bool {
    record.inline_subject_has_role(subject, ACL_ROLE_USE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_prior_bank_hash_tolerates_skipped_slots() {
        let entries = vec![(8, [8; 32]), (6, [6; 32]), (3, [3; 32])];

        assert_eq!(
            latest_prior_bank_hash_from_entries(10, entries.clone()),
            Some([8; 32])
        );
        assert_eq!(
            latest_prior_bank_hash_from_entries(8, entries.clone()),
            Some([6; 32])
        );
        assert_eq!(latest_prior_bank_hash_from_entries(3, entries), None);
    }

    fn host_config_with(
        chain_id: u64,
        test_shims_enabled: bool,
        mock_input_enabled: bool,
    ) -> HostConfig {
        HostConfig {
            admin: Pubkey::default(),
            chain_id,
            input_verifier_set: Pubkey::default(),
            input_verifier_set_version: 0,
            material_authority: Pubkey::default(),
            test_authority: Pubkey::default(),
            paused: false,
            mock_input_enabled,
            test_shims_enabled,
            grant_deny_list_enabled: false,
            updated_slot: 0,
            bump: 0,
        }
    }

    #[cfg(feature = "poc")]
    #[test]
    fn zero_birth_entropy_requires_poc_feature_chain_and_test_shims() {
        // Local PoC chain with the shim flag: relaxation allowed.
        assert!(host_config_with(SOLANA_POC_CHAIN_ID, true, false).zero_birth_entropy_allowed());
        // Local PoC chain without the shim flag: fails closed (drives the
        // PreviousBankHashUnavailable negative test).
        assert!(!host_config_with(SOLANA_POC_CHAIN_ID, false, false).zero_birth_entropy_allowed());
        // A deployed chain can NEVER zero birth entropy, even with the shim flag
        // toggled on by an admin — this is the security boundary.
        assert!(!host_config_with(101, true, false).zero_birth_entropy_allowed());
        assert!(!host_config_with(1, true, false).zero_birth_entropy_allowed());
    }

    #[cfg(not(feature = "poc"))]
    #[test]
    fn poc_chain_relaxations_are_disabled_without_poc_feature() {
        assert!(!host_config_with(SOLANA_POC_CHAIN_ID, true, true).is_local_poc_chain());
        assert!(!host_config_with(SOLANA_POC_CHAIN_ID, true, false).zero_birth_entropy_allowed());
        assert!(!host_config_with(SOLANA_POC_CHAIN_ID, false, true).mock_input_allowed());
    }

    #[cfg(feature = "poc")]
    #[test]
    fn mock_input_requires_poc_chain() {
        assert!(host_config_with(SOLANA_POC_CHAIN_ID, false, true).mock_input_allowed());
        assert!(!host_config_with(SOLANA_POC_CHAIN_ID, false, false).mock_input_allowed());
        // A deployed chain can never run the mock input bind path, even if an
        // admin sets mock_input_enabled.
        assert!(!host_config_with(101, false, true).mock_input_allowed());
    }

    #[test]
    fn native_v0_bound_eval_handle_uses_slot_entropy_and_preserves_layout() {
        let lhs = [1; 32];
        let rhs = [2; 32];
        let context_id = [3; 32];
        let nonce_key = [4; 32];
        let first = computed_bound_eval_handle(
            FheBinaryOpCode::Add,
            lhs,
            rhs,
            false,
            1,
            42,
            [5; 32],
            100,
            context_id,
            7,
            nonce_key,
            9,
        );
        let different_entropy = computed_bound_eval_handle(
            FheBinaryOpCode::Add,
            lhs,
            rhs,
            false,
            1,
            42,
            [6; 32],
            101,
            context_id,
            7,
            nonce_key,
            9,
        );
        let different_nonce = computed_bound_eval_handle(
            FheBinaryOpCode::Add,
            lhs,
            rhs,
            false,
            1,
            42,
            [5; 32],
            100,
            context_id,
            7,
            nonce_key,
            10,
        );

        assert_ne!(first, different_entropy);
        assert_ne!(first, different_nonce);
        assert_eq!(first[21], COMPUTED_HANDLE_MARKER);
        assert_eq!(&first[22..30], &42_u64.to_be_bytes());
        assert_eq!(first[30], 1);
        assert_eq!(first[31], HANDLE_VERSION);
    }
}
