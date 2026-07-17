//! State and deterministic helper functions for the ZamaHost program.
//!
//! This module is intentionally reusable from app programs and tests. It
//! exposes the PDA seeds, account layouts, and handle formulas
//! needed to prepare CPI accounts and to verify host-owned ACL state off-chain.

use anchor_lang::prelude::*;
use solana_keccak_hasher::hashv as keccak_hashv;
use solana_sha256_hasher::hashv;
use solana_sysvar::get_sysvar;

use crate::constants::{COMPUTATION_DOMAIN_SEPARATOR, COMPUTED_HANDLE_MARKER};
use crate::errors::ZamaHostError;

pub mod deny_subject_record;
pub mod encrypted_value;
pub mod hcu_block_meter;
pub mod hcu_trusted_app_record;
pub mod host_config;
pub mod kms_context;
pub mod user_decryption_delegation;

pub use deny_subject_record::*;
pub use encrypted_value::*;
pub use hcu_block_meter::*;
pub use hcu_trusted_app_record::*;
pub use host_config::*;
pub use kms_context::*;
pub use user_decryption_delegation::*;

pub use crate::constants::*;

/// Initialization arguments for the singleton [`HostConfig`] account.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct InitializeHostConfigArgs {
    /// Host-chain id encoded into newly derived handles.
    pub chain_id: u64,
    /// EVM gateway chain id used in the coprocessor/KMS EIP-712 domain separators.
    pub gateway_chain_id: u64,
    /// EVM `InputVerification` contract address (EIP-712 verifying contract).
    pub input_verification_contract: [u8; 20],
    /// Registered coprocessor EVM signer set for input attestations (EVM `InputVerifier`
    /// parity). Must be non-empty, distinct, and free of the zero address; bounded by
    /// [`HostConfig::MAX_COPROCESSOR_SIGNERS`].
    pub coprocessor_signers: Vec<[u8; 20]>,
    /// Minimum distinct valid signatures (n-of-m) required over an input attestation;
    /// `1 <= coprocessor_threshold <= coprocessor_signers.len()`.
    pub coprocessor_threshold: u8,
    /// EVM `Decryption` contract address (EIP-712 verifying contract for KMS certs).
    pub decryption_contract: [u8; 20],
    /// Whether persistent grants must include a deny-list witness.
    pub grant_deny_list_enabled: bool,
}

/// Pubkey stored inline as an allowed subject.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AclSubjectEntry {
    /// Subject identity. For app users this is usually the owner pubkey; for
    /// app programs this can be a program-controlled compute signer PDA.
    pub pubkey: Pubkey,
}

impl AclSubjectEntry {
    /// Builds an owner/user subject.
    pub fn user(pubkey: Pubkey) -> Self {
        Self { pubkey }
    }

    /// Builds a compute subject.
    pub fn compute(pubkey: Pubkey) -> Self {
        Self { pubkey }
    }

    /// Builds an allowed subject.
    pub fn use_only(pubkey: Pubkey) -> Self {
        Self { pubkey }
    }
}

/// Binary FHE operators currently modeled by the PoC.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FheBinaryOpCode {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Multiplication.
    Mul,
    /// Division.
    Div,
    /// Remainder.
    Rem,
    /// Bitwise AND.
    And,
    /// Bitwise OR.
    Or,
    /// Bitwise XOR.
    Xor,
    /// Shift left.
    Shl,
    /// Shift right.
    Shr,
    /// Rotate left.
    Rotl,
    /// Rotate right.
    Rotr,
    /// Equality comparison.
    Eq,
    /// Inequality comparison.
    Ne,
    /// Greater-than-or-equal comparison.
    Ge,
    /// Greater-than comparison.
    Gt,
    /// Less-than-or-equal comparison.
    Le,
    /// Less-than comparison.
    Lt,
    /// Minimum.
    Min,
    /// Maximum.
    Max,
}

/// Ternary FHE operators currently modeled by the PoC.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FheTernaryOpCode {
    /// Selects `if_true` when `control` is true, otherwise `if_false`.
    IfThenElse,
}

/// Unary FHE operators.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FheUnaryOpCode {
    /// Arithmetic negation.
    Neg,
    /// Bitwise NOT.
    Not,
    /// Type cast.
    Cast,
}

impl FheUnaryOpCode {
    /// Stable byte encoding used in handle derivation and events; mirrors the shared coprocessor
    /// `SupportedFheOperations` discriminants (FheNeg=20, FheNot=21, FheCast=23; 22 is reserved).
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Neg => 20,
            Self::Not => 21,
            Self::Cast => 23,
        }
    }
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
    /// Unary operator step.
    Unary {
        /// Unary operator.
        op: FheUnaryOpCode,
        /// Encrypted operand.
        operand: FheEvalOperand,
        /// FHE type byte embedded in the output handle.
        output_fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into durable ACL state.
        output: FheEvalOutput,
    },
    /// Bounded random ciphertext step.
    RandBounded {
        /// Exclusive upper bound encoded as a 256-bit big-endian integer.
        upper_bound: [u8; 32],
        /// FHE type byte embedded in the output handle.
        fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into durable ACL state.
        output: FheEvalOutput,
    },
    /// Sum step.
    Sum {
        /// Encrypted operands.
        operands: Vec<FheEvalOperand>,
        /// FHE type byte embedded in the output handle.
        fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into durable ACL state.
        output: FheEvalOutput,
    },
    /// Is-in membership test step.
    IsIn {
        /// Encrypted value to test.
        value: FheEvalOperand,
        /// Encrypted set operands.
        set: Vec<FheEvalOperand>,
        /// FHE type byte of the value and set elements.
        fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into durable ACL state.
        output: FheEvalOutput,
    },
    /// Multiply-then-divide step.
    MulDiv {
        /// Left-hand encrypted factor.
        factor1: FheEvalOperand,
        /// Right-hand factor, encrypted or scalar bytes.
        factor2: FheEvalOperand,
        /// Divisor encoded as a 256-bit big-endian integer.
        divisor: [u8; 32],
        /// FHE type byte embedded in the output handle.
        output_fhe_type: u8,
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
        // Boxed so the ~190-byte attestation is paid only by operands that carry one, not
        // inlined into every `FheEvalOperand` slot of every step (a Rust enum is as large as
        // its fattest variant, and plans live in `Vec<FheEvalStep>` on the 32KB SBF bump heap
        // on both sides of the CPI). `Box<T>` is borsh- and IDL-transparent: the wire format
        // is unchanged.
        attestation: Box<CoprocessorInputAttestation>,
    },
}

/// Output policy for a composed FHE eval operation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheEvalOutput {
    /// Output stays allowed only inside the current `fhe_eval` scope; no durable ACL record.
    AllowedLocal,
    /// Output is bound into durable ACL state: the `EncryptedValue` lineage PDA
    /// is created when absent, or superseded when it exists.
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
        /// subjects; on supersede they become the new audience, which may rotate
        /// away from the stored set (the outgoing audience is sealed into
        /// historical leaves first; added subjects pass the grant deny-list).
        output_subjects: Vec<AclSubjectEntry>,
        /// Superseded handle: `None` on create, `Some(current_handle)` on update.
        /// Carried in instruction data so indexers can reconstruct the appended
        /// MMR leaves without reading the account; validated against the account.
        previous_handle: Option<[u8; 32]>,
        /// Superseded subject set, parallel to `previous_handle` (`None` on create,
        /// exact stored subjects on update). Same indexer-reconstruction purpose.
        previous_subjects: Option<Vec<Pubkey>>,
        /// When true, the newly bound handle is born publicly decryptable: after
        /// writing it as `current_handle`, a public-decrypt leaf is appended for
        /// the new handle (byte-identical to `make_handle_public`). Carried in
        /// instruction data so indexers reconstruct that leaf without reading the
        /// account. This is the opt-in relaxation of the "created lineages cannot
        /// be born public" invariant (DD-036).
        make_public: bool,
    },
}

impl FheBinaryOpCode {
    /// Stable byte encoding used in handle derivation and events.
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Add => 0,
            Self::Sub => 1,
            Self::Mul => 2,
            Self::Div => 3,
            Self::Rem => 4,
            Self::And => 5,
            Self::Or => 6,
            Self::Xor => 7,
            Self::Shl => 8,
            Self::Shr => 9,
            Self::Rotl => 10,
            Self::Rotr => 11,
            Self::Eq => 12,
            Self::Ne => 13,
            Self::Ge => 14,
            Self::Gt => 15,
            Self::Le => 16,
            Self::Lt => 17,
            Self::Min => 18,
            Self::Max => 19,
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
        FheBinaryOpCode::Add
        | FheBinaryOpCode::Sub
        | FheBinaryOpCode::Mul
        | FheBinaryOpCode::Div
        | FheBinaryOpCode::Rem
        | FheBinaryOpCode::Min
        | FheBinaryOpCode::Max => matches!(fhe_type, 2..=6),
        // Bitwise: EVM allows Bool + Uint8..Uint128 + Uint256.
        FheBinaryOpCode::And | FheBinaryOpCode::Or | FheBinaryOpCode::Xor => {
            matches!(fhe_type, 0 | 2..=6 | 8)
        }
        // Shifts/rotations: EVM allows Uint8..Uint128 + Uint256.
        FheBinaryOpCode::Shl
        | FheBinaryOpCode::Shr
        | FheBinaryOpCode::Rotl
        | FheBinaryOpCode::Rotr => matches!(fhe_type, 2..=6 | 8),
        FheBinaryOpCode::Eq
        | FheBinaryOpCode::Ne
        | FheBinaryOpCode::Ge
        | FheBinaryOpCode::Gt
        | FheBinaryOpCode::Le
        | FheBinaryOpCode::Lt => fhe_type == 0,
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
    match op {
        // Comparisons produce `ebool`, so the operand type is gated here: Eq/Ne accept Bool..Uint256 while ordered comparisons accept Uint8..Uint128, matching EVM's fheEq/fheGe supportedTypes.
        FheBinaryOpCode::Eq | FheBinaryOpCode::Ne => {
            require!(
                matches!(lhs_type, 0 | 2..=8),
                ZamaHostError::UnsupportedFheType
            );
        }
        FheBinaryOpCode::Ge | FheBinaryOpCode::Gt | FheBinaryOpCode::Le | FheBinaryOpCode::Lt => {
            require!(matches!(lhs_type, 2..=6), ZamaHostError::UnsupportedFheType);
        }
        // Div/Rem: divisor must be a plaintext scalar (EVM `IsNotScalar`), non-zero after truncation.
        FheBinaryOpCode::Div | FheBinaryOpCode::Rem => {
            require!(
                lhs_type == output_fhe_type,
                ZamaHostError::BinaryOperandTypeMismatch
            );
            require!(scalar, ZamaHostError::DivisorMustBeScalar);
            require!(
                !scalar_is_zero_for_type(rhs, lhs_type),
                ZamaHostError::DivisionByZero
            );
        }
        // Remaining ops: the operand type must equal the (op-gated) output type.
        FheBinaryOpCode::Add
        | FheBinaryOpCode::Sub
        | FheBinaryOpCode::Mul
        | FheBinaryOpCode::And
        | FheBinaryOpCode::Or
        | FheBinaryOpCode::Xor
        | FheBinaryOpCode::Shl
        | FheBinaryOpCode::Shr
        | FheBinaryOpCode::Rotl
        | FheBinaryOpCode::Rotr
        | FheBinaryOpCode::Min
        | FheBinaryOpCode::Max => {
            require!(
                lhs_type == output_fhe_type,
                ZamaHostError::BinaryOperandTypeMismatch
            );
        }
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

pub fn assert_supported_unary_output_type(op: FheUnaryOpCode, fhe_type: u8) -> Result<()> {
    assert_supported_fhe_type(fhe_type)?;
    let valid = match op {
        FheUnaryOpCode::Neg => matches!(fhe_type, 2..=6 | 8),
        FheUnaryOpCode::Not => matches!(fhe_type, 0 | 2..=6 | 8),
        // EVM `cast` output set: Uint8..Uint128 | Uint256 (no ebool, no eaddress/Uint160).
        FheUnaryOpCode::Cast => matches!(fhe_type, 2..=6 | 8),
    };
    require!(valid, ZamaHostError::UnsupportedFheType);
    Ok(())
}

pub fn assert_unary_operand_type(
    op: FheUnaryOpCode,
    operand: [u8; 32],
    output_fhe_type: u8,
) -> Result<()> {
    assert_supported_unary_output_type(op, output_fhe_type)?;
    let operand_type = handle_fhe_type(operand);
    require!(
        is_supported_fhe_type(operand_type),
        ZamaHostError::UnsupportedFheType
    );
    match op {
        FheUnaryOpCode::Neg => {
            require!(
                matches!(operand_type, 2..=6 | 8),
                ZamaHostError::UnsupportedFheType
            );
            require!(
                operand_type == output_fhe_type,
                ZamaHostError::BinaryOperandTypeMismatch
            );
        }
        FheUnaryOpCode::Not => {
            require!(
                matches!(operand_type, 0 | 2..=6 | 8),
                ZamaHostError::UnsupportedFheType
            );
            require!(
                operand_type == output_fhe_type,
                ZamaHostError::BinaryOperandTypeMismatch
            );
        }
        FheUnaryOpCode::Cast => {
            // EVM `cast` input set: Bool | Uint8..Uint128 | Uint256 (no eaddress/Uint160).
            require!(
                matches!(operand_type, 0 | 2..=6 | 8),
                ZamaHostError::UnsupportedFheType
            );
            // Cast reinterprets to a different type; a same-type cast is rejected (EVM InvalidType).
            require!(
                operand_type != output_fhe_type,
                ZamaHostError::UnsupportedFheType
            );
        }
    }
    Ok(())
}

/// Requires every operand's resolved handle type to equal the declared uint type (2..=6). Like EVM
/// `fheSum` and the coprocessor, only the maximum operand count is bounded — a zero/single-operand
/// sum is valid (EVM enforces no minimum).
pub fn assert_sum_operand_types(operand_handles: &[[u8; 32]], fhe_type: u8) -> Result<()> {
    require!(matches!(fhe_type, 2..=6), ZamaHostError::UnsupportedFheType);
    // Cap the operand count at the coprocessor's FheSum limit (transient operands use no accounts).
    require!(
        operand_handles.len() <= max_reduction_operands(fhe_type),
        ZamaHostError::InvalidFheEvalAccount
    );
    for handle in operand_handles {
        require!(
            handle_fhe_type(*handle) == fhe_type,
            ZamaHostError::BinaryOperandTypeMismatch
        );
    }
    Ok(())
}

/// Requires the value and every set member to share the declared uint type (Uint8..Uint256, 2..=8) —
/// matching EVM `fheIsIn` and the coprocessor's FheIsIn type gate; `ebool` is excluded. Like EVM,
/// only the maximum set size is bounded — an empty set is valid (membership is trivially false).
pub fn assert_is_in_operand_types(
    value_handle: [u8; 32],
    set_handles: &[[u8; 32]],
    fhe_type: u8,
) -> Result<()> {
    require!(matches!(fhe_type, 2..=8), ZamaHostError::UnsupportedFheType);
    // Cap the set size at the coprocessor's FheIsIn limit (its `set_size` bound excludes the value).
    require!(
        set_handles.len() <= max_reduction_operands(fhe_type),
        ZamaHostError::InvalidFheEvalAccount
    );
    require!(
        handle_fhe_type(value_handle) == fhe_type,
        ZamaHostError::BinaryOperandTypeMismatch
    );
    for handle in set_handles {
        require!(
            handle_fhe_type(*handle) == fhe_type,
            ZamaHostError::BinaryOperandTypeMismatch
        );
    }
    Ok(())
}

/// MulDiv: factor1 is an encrypted uint8..uint64 (EVM + coprocessor cap at Uint64); factor2 is
/// either an encrypted operand of the same type or a plaintext scalar; divisor is an always-scalar
/// plaintext that must be non-zero (EVM DivisionByZero parity).
pub fn assert_mul_div_operand_types(
    factor1: [u8; 32],
    factor2: [u8; 32],
    factor2_scalar: bool,
    divisor: [u8; 32],
    output_fhe_type: u8,
) -> Result<()> {
    require!(
        matches!(output_fhe_type, 2..=5),
        ZamaHostError::UnsupportedFheType
    );
    require!(
        handle_fhe_type(factor1) == output_fhe_type,
        ZamaHostError::BinaryOperandTypeMismatch
    );
    if !factor2_scalar {
        require!(
            handle_fhe_type(factor2) == output_fhe_type,
            ZamaHostError::BinaryOperandTypeMismatch
        );
    }
    // Divisor must be non-zero once truncated to the operand type (EVM parity).
    require!(
        !scalar_is_zero_for_type(divisor, output_fhe_type),
        ZamaHostError::MulDivDivisorZero
    );
    Ok(())
}

pub(crate) fn is_supported_fhe_type(fhe_type: u8) -> bool {
    matches!(fhe_type, 0 | 2 | 3 | 4 | 5 | 6 | 7 | 8)
}

/// Whether a big-endian scalar is zero once truncated to `fhe_type`'s width (EVM `_isScalarZeroForType`).
fn scalar_is_zero_for_type(scalar: [u8; 32], fhe_type: u8) -> bool {
    let width = match fhe_type {
        2 => 1,  // Uint8
        3 => 2,  // Uint16
        4 => 4,  // Uint32
        5 => 8,  // Uint64
        6 => 16, // Uint128
        _ => 32, // unsupported for division: fall back to the whole buffer (fail closed)
    };
    scalar[32 - width..].iter().all(|byte| *byte == 0)
}

/// Coprocessor FheSum/FheIsIn max operand count: 100 for narrow types (Uint8..Uint32), 60 for wider.
fn max_reduction_operands(fhe_type: u8) -> usize {
    match fhe_type {
        2..=4 => 100,
        _ => 60,
    }
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

/// Returns the canonical HCU trust-registry record address for a compute subject.
pub fn hcu_trusted_app_address(app: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[HCU_TRUSTED_APP_SEED, app.as_ref()], &crate::ID)
}

/// Returns the canonical per-subject HCU block meter address for a compute subject.
pub fn hcu_block_meter_address(app: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[HCU_BLOCK_METER_SEED, app.as_ref()], &crate::ID)
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
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
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

fn finish_computed_handle(result: &mut [u8; 32], chain_id_bytes: &[u8; 8], fhe_type: u8) {
    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
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

/// Derives an instruction-local eval sum handle from explicit slot entropy.
#[allow(clippy::too_many_arguments)]
pub fn computed_eval_sum_handle(
    operand_handles: &[[u8; 32]],
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
    let mut preimage: Vec<&[u8]> = vec![
        b"FHE_eval_sum",
        &context_id,
        &op_index_bytes,
        &fhe_type_bytes,
    ];
    for h in operand_handles {
        preimage.push(h.as_ref());
    }
    preimage.push(crate::ID.as_ref());
    preimage.push(&chain_id_bytes);
    preimage.push(&previous_bank_hash);
    preimage.push(&timestamp_bytes);
    let mut result = hashv(preimage.as_slice()).to_bytes();
    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives an instruction-local eval is-in handle from explicit slot entropy.
#[allow(clippy::too_many_arguments)]
pub fn computed_eval_is_in_handle(
    value_handle: [u8; 32],
    set_handles: &[[u8; 32]],
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
    let mut preimage: Vec<&[u8]> = vec![
        b"FHE_eval_is_in",
        &context_id,
        &op_index_bytes,
        &fhe_type_bytes,
        &value_handle,
    ];
    for h in set_handles {
        preimage.push(h.as_ref());
    }
    preimage.push(crate::ID.as_ref());
    preimage.push(&chain_id_bytes);
    preimage.push(&previous_bank_hash);
    preimage.push(&timestamp_bytes);
    let mut result = hashv(preimage.as_slice()).to_bytes();
    finish_computed_handle(&mut result, &chain_id_bytes, 0 /* ebool */);
    result
}

/// Derives an instruction-local eval mul-div handle from explicit slot entropy.
#[allow(clippy::too_many_arguments)]
pub fn computed_eval_mul_div_handle(
    factor1: [u8; 32],
    factor2: [u8; 32],
    divisor: [u8; 32],
    scalar: bool,
    output_fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
) -> [u8; 32] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let op_index_bytes = op_index.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let scalar_byte = [u8::from(scalar)];
    let mut result = hashv(&[
        b"FHE_eval_mul_div",
        &context_id,
        &op_index_bytes,
        &factor1,
        &factor2,
        &divisor,
        &scalar_byte,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();
    finish_computed_handle(&mut result, &chain_id_bytes, output_fhe_type);
    result
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

/// Derives an unbound unary-op handle from explicit slot entropy.
pub fn computed_unary_handle(
    op: FheUnaryOpCode,
    operand: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> [u8; 32] {
    let op_byte = [op.as_u8()];
    let type_byte = [fhe_type];
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    // Cast binds the target type into the prehandle (EVM `FHEVMExecutor.cast`); Neg/Not derive it from the operand.
    let mut parts: Vec<&[u8]> = vec![COMPUTATION_DOMAIN_SEPARATOR, &op_byte, &operand];
    if matches!(op, FheUnaryOpCode::Cast) {
        parts.push(&type_byte);
    }
    parts.extend_from_slice(&[
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ]);
    let mut result = keccak_hashv(&parts).to_bytes();
    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives an instruction-local eval unary handle from explicit slot entropy.
pub fn computed_eval_unary_handle(
    op: FheUnaryOpCode,
    operand: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
) -> [u8; 32] {
    let op_byte = [op.as_u8()];
    let type_byte = [fhe_type];
    let chain_id_bytes = chain_id.to_be_bytes();
    let op_index_bytes = op_index.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    // Cast binds its target type into the prehandle (see `computed_unary_handle`); Neg/Not take it from the operand.
    let mut parts: Vec<&[u8]> = vec![
        b"FHE_eval_unary",
        &context_id,
        &op_index_bytes,
        &op_byte,
        &operand,
    ];
    if matches!(op, FheUnaryOpCode::Cast) {
        parts.push(&type_byte);
    }
    parts.extend_from_slice(&[
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ]);
    let mut result = hashv(&parts).to_bytes();
    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives the seed emitted for an instruction-local eval bounded-random handle.
pub fn computed_eval_rand_bounded_seed(
    upper_bound: [u8; 32],
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
        b"FHE_eval_bounded_seed",
        &context_id,
        &op_index_bytes,
        &upper_bound,
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
