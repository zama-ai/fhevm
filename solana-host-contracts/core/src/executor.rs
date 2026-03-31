use crate::acl::{AclSession, AclState};
use crate::error::{HostContractError, Result};
use crate::events::HostEvent;
use crate::hcu::{HcuLimitState, HcuOperationKey, TransactionMeter};
use crate::input_verifier::{InputProofVerifier, InputVerifierSession, InputVerifierState};
use crate::types::{ContextUserInputs, FheType, Handle, Operator, Pubkey};
use borsh::{BorshDeserialize, BorshSerialize};
use sha3::{Digest, Keccak256};

const COMPUTATION_DOMAIN_SEPARATOR: &[u8] = b"FHE_comp";
const SEED_DOMAIN_SEPARATOR: &[u8] = b"FHE_seed";
const RAW_CT_HASH_DOMAIN_SEPARATOR: &[u8] = b"ZK-w_rct";
const HANDLE_HASH_DOMAIN_SEPARATOR: &[u8] = b"ZK-w_hdl";

const BINARY_ARITHMETIC_TYPES: &[FheType] = &[
    FheType::Uint8,
    FheType::Uint16,
    FheType::Uint32,
    FheType::Uint64,
    FheType::Uint128,
];
const BINARY_BITWISE_TYPES: &[FheType] = &[
    FheType::Bool,
    FheType::Uint8,
    FheType::Uint16,
    FheType::Uint32,
    FheType::Uint64,
    FheType::Uint128,
    FheType::Uint256,
];
const BINARY_SHIFT_TYPES: &[FheType] = &[
    FheType::Uint8,
    FheType::Uint16,
    FheType::Uint32,
    FheType::Uint64,
    FheType::Uint128,
    FheType::Uint256,
];
const EQ_NE_TYPES: &[FheType] = &[
    FheType::Bool,
    FheType::Uint8,
    FheType::Uint16,
    FheType::Uint32,
    FheType::Uint64,
    FheType::Uint128,
    FheType::Uint160,
    FheType::Uint256,
];
const UNARY_NEG_TYPES: &[FheType] = &[
    FheType::Uint8,
    FheType::Uint16,
    FheType::Uint32,
    FheType::Uint64,
    FheType::Uint128,
    FheType::Uint256,
];
const UNARY_NOT_TYPES: &[FheType] = &[
    FheType::Bool,
    FheType::Uint8,
    FheType::Uint16,
    FheType::Uint32,
    FheType::Uint64,
    FheType::Uint128,
    FheType::Uint256,
];
const CAST_INPUT_TYPES: &[FheType] = &[
    FheType::Bool,
    FheType::Uint8,
    FheType::Uint16,
    FheType::Uint32,
    FheType::Uint64,
    FheType::Uint128,
    FheType::Uint256,
];
const CAST_OUTPUT_TYPES: &[FheType] = &[
    FheType::Uint8,
    FheType::Uint16,
    FheType::Uint32,
    FheType::Uint64,
    FheType::Uint128,
    FheType::Uint256,
];
const TERNARY_TYPES: &[FheType] = &[
    FheType::Bool,
    FheType::Uint8,
    FheType::Uint16,
    FheType::Uint32,
    FheType::Uint64,
    FheType::Uint128,
    FheType::Uint160,
    FheType::Uint256,
];
const RAND_TYPES: &[FheType] = &[
    FheType::Bool,
    FheType::Uint8,
    FheType::Uint16,
    FheType::Uint32,
    FheType::Uint64,
    FheType::Uint128,
    FheType::Uint256,
];
const RAND_BOUNDED_TYPES: &[FheType] = &[
    FheType::Uint8,
    FheType::Uint16,
    FheType::Uint32,
    FheType::Uint64,
    FheType::Uint128,
    FheType::Uint256,
];
const TRIVIAL_ENCRYPT_TYPES: &[FheType] = &[
    FheType::Bool,
    FheType::Uint8,
    FheType::Uint16,
    FheType::Uint32,
    FheType::Uint64,
    FheType::Uint128,
    FheType::Uint160,
    FheType::Uint256,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum BinaryOperand {
    Handle(Handle),
    Scalar([u8; 32]),
}

impl BinaryOperand {
    fn as_bytes(&self) -> [u8; 32] {
        match self {
            Self::Handle(handle) => handle.into_bytes(),
            Self::Scalar(value) => *value,
        }
    }

    fn is_scalar(&self) -> bool {
        matches!(self, Self::Scalar(_))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutionMeta {
    pub chain_id: u64,
    pub slot: u64,
    pub timestamp: i64,
    pub recent_blockhash: [u8; 32],
    pub caller: Pubkey,
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ExecutorState {
    acl_program: Pubkey,
    counter_rand: u64,
}

impl ExecutorState {
    pub fn new(acl_program: Pubkey) -> Self {
        Self {
            acl_program,
            counter_rand: 0,
        }
    }

    pub fn counter_rand(&self) -> u64 {
        self.counter_rand
    }

    pub fn type_of(handle: Handle) -> Result<FheType> {
        handle.fhe_type()
    }

    pub fn compute_input_handles(
        ciphertext: &[u8],
        bit_widths: &[usize],
        acl_program: Pubkey,
        chain_id: u64,
    ) -> Result<Vec<Handle>> {
        let ciphertext_hash = keccak([RAW_CT_HASH_DOMAIN_SEPARATOR, ciphertext].concat());
        let chain_id_bytes = chain_id.to_be_bytes();

        bit_widths
            .iter()
            .enumerate()
            .map(|(index, bit_width)| {
                let handle_type = FheType::from_bit_width(*bit_width)?;
                let mut preimage = Vec::with_capacity(
                    HANDLE_HASH_DOMAIN_SEPARATOR.len() + 32 + 1 + acl_program.as_bytes().len() + 8,
                );
                preimage.extend_from_slice(HANDLE_HASH_DOMAIN_SEPARATOR);
                preimage.extend_from_slice(&ciphertext_hash);
                preimage.push(index as u8);
                preimage.extend_from_slice(acl_program.as_bytes());
                preimage.extend_from_slice(&chain_id_bytes);
                let prehandle = keccak(preimage);
                Ok(Handle::append_metadata(
                    prehandle,
                    chain_id,
                    handle_type,
                    index as u8,
                ))
            })
            .collect()
    }

    pub fn unary_op(
        &mut self,
        op: Operator,
        ct: Handle,
        meta: ExecutionMeta,
        acl: &AclState,
        session: &mut AclSession,
        hcu: Option<(&mut HcuLimitState, &mut TransactionMeter)>,
    ) -> Result<(Handle, HostEvent)> {
        let result_type = validate_unary_operation(op, ct.fhe_type()?)?;
        self.unary_like(op, ct, result_type, meta, acl, session, hcu)
    }

    pub fn binary_op(
        &mut self,
        op: Operator,
        lhs: Handle,
        rhs: BinaryOperand,
        meta: ExecutionMeta,
        acl: &AclState,
        session: &mut AclSession,
        requested_result_type: FheType,
        hcu: Option<(&mut HcuLimitState, &mut TransactionMeter)>,
    ) -> Result<(Handle, HostEvent)> {
        let caller = meta.caller;
        if !acl.is_allowed(lhs, caller, session) {
            return Err(HostContractError::ACLNotAllowed);
        }

        let lhs_type = lhs.fhe_type()?;
        let scalar = rhs.is_scalar();
        validate_binary_lhs(op, lhs_type)?;

        match rhs {
            BinaryOperand::Handle(rhs_handle) => {
                if !acl.is_allowed(rhs_handle, caller, session) {
                    return Err(HostContractError::ACLNotAllowed);
                }
                if requires_scalar_rhs(op) {
                    return Err(HostContractError::IsNotScalar);
                }
                if lhs_type != rhs_handle.fhe_type()? {
                    return Err(HostContractError::IncompatibleTypes);
                }
            }
            BinaryOperand::Scalar(value) => {
                if matches!(op, Operator::FheDiv | Operator::FheRem)
                    && scalar_is_zero_for_type(value, lhs_type)?
                {
                    return Err(HostContractError::DivisionByZero);
                }
            }
        }

        if requires_scalar_rhs(op) && !scalar {
            return Err(HostContractError::IsNotScalar);
        }

        let expected_result_type = binary_result_type(op, lhs_type);
        if requested_result_type != expected_result_type {
            return Err(HostContractError::InvalidType);
        }

        let scalar_flag = if scalar { 1_u8 } else { 0_u8 };
        let prehandle = keccak(
            [
                COMPUTATION_DOMAIN_SEPARATOR,
                &[op as u8],
                lhs.as_bytes(),
                &rhs.as_bytes(),
                &[scalar_flag],
                self.acl_program.as_bytes(),
                &u256_word_from_u64(meta.chain_id),
                &meta.recent_blockhash,
                &u256_word_from_u64(meta.timestamp as u64),
            ]
            .concat(),
        );
        let result = Handle::append_metadata(prehandle, meta.chain_id, expected_result_type, 0xff);
        acl.allow_transient(self.acl_program, result, caller, session)?;
        if let Some((hcu_state, tx_meter)) = hcu {
            let input_handles: Vec<Handle> = match rhs {
                BinaryOperand::Handle(rhs_handle) => vec![lhs, rhs_handle],
                BinaryOperand::Scalar(_) => vec![lhs],
            };
            hcu_state.charge_for_operation(
                HcuOperationKey {
                    op,
                    result_type: hcu_pricing_type_for_binary(op, lhs_type),
                    scalar,
                },
                &input_handles,
                result,
                caller,
                meta.slot,
                tx_meter,
            )?;
        }
        Ok((
            result,
            HostEvent::Operation {
                caller,
                op,
                operands: vec![lhs.into_bytes(), rhs.as_bytes()],
                scalar_flag: Some(scalar_flag),
                result_type: expected_result_type,
                result,
            },
        ))
    }

    pub fn ternary_op(
        &mut self,
        op: Operator,
        control: Handle,
        if_true: Handle,
        if_false: Handle,
        meta: ExecutionMeta,
        acl: &AclState,
        session: &mut AclSession,
        hcu: Option<(&mut HcuLimitState, &mut TransactionMeter)>,
    ) -> Result<(Handle, HostEvent)> {
        if op != Operator::FheIfThenElse {
            return Err(HostContractError::UnsupportedOperator(op));
        }

        let caller = meta.caller;
        for handle in [control, if_true, if_false] {
            if !acl.is_allowed(handle, caller, session) {
                return Err(HostContractError::ACLNotAllowed);
            }
        }

        let control_type = control.fhe_type()?;
        if control_type != FheType::Bool {
            return Err(HostContractError::UnsupportedType(control_type));
        }

        let result_type = if_true.fhe_type()?;
        validate_type(result_type, TERNARY_TYPES)?;
        if result_type != if_false.fhe_type()? {
            return Err(HostContractError::IncompatibleTypes);
        }

        let prehandle = keccak(
            [
                COMPUTATION_DOMAIN_SEPARATOR,
                &[op as u8],
                control.as_bytes(),
                if_true.as_bytes(),
                if_false.as_bytes(),
                self.acl_program.as_bytes(),
                &u256_word_from_u64(meta.chain_id),
                &meta.recent_blockhash,
                &u256_word_from_u64(meta.timestamp as u64),
            ]
            .concat(),
        );
        let result = Handle::append_metadata(prehandle, meta.chain_id, result_type, 0xff);
        acl.allow_transient(self.acl_program, result, caller, session)?;
        if let Some((hcu_state, tx_meter)) = hcu {
            hcu_state.charge_for_operation(
                HcuOperationKey {
                    op,
                    result_type,
                    scalar: false,
                },
                &[control, if_true, if_false],
                result,
                caller,
                meta.slot,
                tx_meter,
            )?;
        }
        Ok((
            result,
            HostEvent::Operation {
                caller,
                op,
                operands: vec![
                    control.into_bytes(),
                    if_true.into_bytes(),
                    if_false.into_bytes(),
                ],
                scalar_flag: None,
                result_type,
                result,
            },
        ))
    }

    pub fn cast(
        &mut self,
        ct: Handle,
        to_type: FheType,
        meta: ExecutionMeta,
        acl: &AclState,
        session: &mut AclSession,
        hcu: Option<(&mut HcuLimitState, &mut TransactionMeter)>,
    ) -> Result<(Handle, HostEvent)> {
        let input_type = ct.fhe_type()?;
        validate_type(input_type, CAST_INPUT_TYPES)?;
        validate_type(to_type, CAST_OUTPUT_TYPES)?;
        if input_type == to_type {
            return Err(HostContractError::InvalidType);
        }
        self.unary_like(Operator::Cast, ct, to_type, meta, acl, session, hcu)
    }

    pub fn trivial_encrypt(
        &mut self,
        plaintext: [u8; 32],
        to_type: FheType,
        meta: ExecutionMeta,
        acl: &AclState,
        session: &mut AclSession,
        hcu: Option<(&mut HcuLimitState, &mut TransactionMeter)>,
    ) -> Result<(Handle, HostEvent)> {
        validate_type(to_type, TRIVIAL_ENCRYPT_TYPES)?;
        let prehandle = keccak(
            [
                COMPUTATION_DOMAIN_SEPARATOR,
                &[Operator::TrivialEncrypt as u8],
                &plaintext,
                &[to_type as u8],
                self.acl_program.as_bytes(),
                &u256_word_from_u64(meta.chain_id),
                &meta.recent_blockhash,
                &u256_word_from_u64(meta.timestamp as u64),
            ]
            .concat(),
        );
        let result = Handle::append_metadata(prehandle, meta.chain_id, to_type, 0xff);
        acl.allow_transient(self.acl_program, result, meta.caller, session)?;
        if let Some((hcu_state, tx_meter)) = hcu {
            hcu_state.charge_for_operation(
                HcuOperationKey {
                    op: Operator::TrivialEncrypt,
                    result_type: to_type,
                    scalar: false,
                },
                &[],
                result,
                meta.caller,
                meta.slot,
                tx_meter,
            )?;
        }
        Ok((
            result,
            HostEvent::Operation {
                caller: meta.caller,
                op: Operator::TrivialEncrypt,
                operands: vec![plaintext],
                scalar_flag: None,
                result_type: to_type,
                result,
            },
        ))
    }

    pub fn fhe_rand(
        &mut self,
        rand_type: FheType,
        meta: ExecutionMeta,
        acl: &AclState,
        session: &mut AclSession,
        hcu: Option<(&mut HcuLimitState, &mut TransactionMeter)>,
    ) -> Result<(Handle, HostEvent)> {
        validate_type(rand_type, RAND_TYPES)?;
        let seed = self.generate_seed(meta);
        let prehandle = keccak(
            [
                COMPUTATION_DOMAIN_SEPARATOR,
                &[Operator::FheRand as u8],
                &[rand_type as u8],
                &seed,
            ]
            .concat(),
        );
        let result = Handle::append_metadata(prehandle, meta.chain_id, rand_type, 0xff);
        acl.allow_transient(self.acl_program, result, meta.caller, session)?;
        if let Some((hcu_state, tx_meter)) = hcu {
            hcu_state.charge_for_operation(
                HcuOperationKey {
                    op: Operator::FheRand,
                    result_type: rand_type,
                    scalar: false,
                },
                &[],
                result,
                meta.caller,
                meta.slot,
                tx_meter,
            )?;
        }
        Ok((
            result,
            HostEvent::Operation {
                caller: meta.caller,
                op: Operator::FheRand,
                operands: vec![seed_to_32(seed)],
                scalar_flag: None,
                result_type: rand_type,
                result,
            },
        ))
    }

    pub fn fhe_rand_bounded(
        &mut self,
        upper_bound: [u8; 32],
        rand_type: FheType,
        meta: ExecutionMeta,
        acl: &AclState,
        session: &mut AclSession,
        hcu: Option<(&mut HcuLimitState, &mut TransactionMeter)>,
    ) -> Result<(Handle, HostEvent)> {
        validate_type(rand_type, RAND_BOUNDED_TYPES)?;
        if !is_power_of_two_word(&upper_bound) {
            return Err(HostContractError::NotPowerOfTwo);
        }
        if upper_bound_exceeds_max_type(&upper_bound, rand_type)? {
            return Err(HostContractError::UpperBoundAboveMaxTypeValue);
        }

        let seed = self.generate_seed(meta);
        let prehandle = keccak(
            [
                COMPUTATION_DOMAIN_SEPARATOR,
                &[Operator::FheRandBounded as u8],
                &upper_bound,
                &[rand_type as u8],
                &seed,
            ]
            .concat(),
        );
        let result = Handle::append_metadata(prehandle, meta.chain_id, rand_type, 0xff);
        acl.allow_transient(self.acl_program, result, meta.caller, session)?;
        if let Some((hcu_state, tx_meter)) = hcu {
            hcu_state.charge_for_operation(
                HcuOperationKey {
                    op: Operator::FheRandBounded,
                    result_type: rand_type,
                    scalar: false,
                },
                &[],
                result,
                meta.caller,
                meta.slot,
                tx_meter,
            )?;
        }
        Ok((
            result,
            HostEvent::Operation {
                caller: meta.caller,
                op: Operator::FheRandBounded,
                operands: vec![upper_bound, seed_to_32(seed)],
                scalar_flag: None,
                result_type: rand_type,
                result,
            },
        ))
    }

    pub fn verify_input<V: InputProofVerifier>(
        &mut self,
        verifier: &mut InputVerifierState,
        verifier_session: &mut InputVerifierSession,
        proof_verifier: &V,
        context: ContextUserInputs,
        input_handle: Handle,
        input_proof: &[u8],
        meta: ExecutionMeta,
        acl: &AclState,
        session: &mut AclSession,
    ) -> Result<(Handle, HostEvent)> {
        let result = verifier.verify_input(
            context,
            input_handle,
            input_proof,
            verifier_session,
            proof_verifier,
            meta.chain_id,
        )?;
        let result_type = result.fhe_type()?;
        acl.allow_transient(self.acl_program, result, meta.caller, session)?;
        Ok((
            result,
            HostEvent::VerifyInput {
                caller: meta.caller,
                input_handle,
                user_address: context.user_address,
                input_proof_len: input_proof.len() as u32,
                input_type: result_type,
                result,
            },
        ))
    }

    fn unary_like(
        &mut self,
        op: Operator,
        ct: Handle,
        result_type: FheType,
        meta: ExecutionMeta,
        acl: &AclState,
        session: &mut AclSession,
        hcu: Option<(&mut HcuLimitState, &mut TransactionMeter)>,
    ) -> Result<(Handle, HostEvent)> {
        let caller = meta.caller;
        if !acl.is_allowed(ct, caller, session) {
            return Err(HostContractError::ACLNotAllowed);
        }
        let prehandle = keccak(
            [
                COMPUTATION_DOMAIN_SEPARATOR,
                &[op as u8],
                ct.as_bytes(),
                &[result_type as u8],
                self.acl_program.as_bytes(),
                &u256_word_from_u64(meta.chain_id),
                &meta.recent_blockhash,
                &u256_word_from_u64(meta.timestamp as u64),
            ]
            .concat(),
        );
        let result = Handle::append_metadata(prehandle, meta.chain_id, result_type, 0xff);
        acl.allow_transient(self.acl_program, result, caller, session)?;
        if let Some((hcu_state, tx_meter)) = hcu {
            hcu_state.charge_for_operation(
                HcuOperationKey {
                    op,
                    result_type,
                    scalar: false,
                },
                &[ct],
                result,
                caller,
                meta.slot,
                tx_meter,
            )?;
        }
        Ok((
            result,
            HostEvent::Operation {
                caller,
                op,
                operands: vec![ct.into_bytes()],
                scalar_flag: None,
                result_type,
                result,
            },
        ))
    }

    fn generate_seed(&mut self, meta: ExecutionMeta) -> [u8; 16] {
        let hash = keccak(
            [
                SEED_DOMAIN_SEPARATOR,
                &u256_word_from_u64(self.counter_rand),
                self.acl_program.as_bytes(),
                &u256_word_from_u64(meta.chain_id),
                &meta.recent_blockhash,
                &u256_word_from_u64(meta.timestamp as u64),
            ]
            .concat(),
        );
        self.counter_rand = self.counter_rand.wrapping_add(1);
        let mut seed = [0_u8; 16];
        seed.copy_from_slice(&hash[..16]);
        seed
    }
}

fn validate_unary_operation(op: Operator, input_type: FheType) -> Result<FheType> {
    match op {
        Operator::FheNeg => {
            validate_type(input_type, UNARY_NEG_TYPES)?;
            Ok(input_type)
        }
        Operator::FheNot => {
            validate_type(input_type, UNARY_NOT_TYPES)?;
            Ok(input_type)
        }
        _ => Err(HostContractError::UnsupportedOperator(op)),
    }
}

fn validate_binary_lhs(op: Operator, lhs_type: FheType) -> Result<()> {
    match op {
        Operator::FheAdd
        | Operator::FheSub
        | Operator::FheMul
        | Operator::FheDiv
        | Operator::FheRem
        | Operator::FheGe
        | Operator::FheGt
        | Operator::FheLe
        | Operator::FheLt
        | Operator::FheMin
        | Operator::FheMax => validate_type(lhs_type, BINARY_ARITHMETIC_TYPES),
        Operator::FheBitAnd | Operator::FheBitOr | Operator::FheBitXor => {
            validate_type(lhs_type, BINARY_BITWISE_TYPES)
        }
        Operator::FheShl | Operator::FheShr | Operator::FheRotl | Operator::FheRotr => {
            validate_type(lhs_type, BINARY_SHIFT_TYPES)
        }
        Operator::FheEq | Operator::FheNe => validate_type(lhs_type, EQ_NE_TYPES),
        _ => Err(HostContractError::UnsupportedOperator(op)),
    }
}

fn binary_result_type(op: Operator, lhs_type: FheType) -> FheType {
    match op {
        Operator::FheEq
        | Operator::FheNe
        | Operator::FheGe
        | Operator::FheGt
        | Operator::FheLe
        | Operator::FheLt => FheType::Bool,
        _ => lhs_type,
    }
}

fn hcu_pricing_type_for_binary(op: Operator, lhs_type: FheType) -> FheType {
    match op {
        Operator::FheEq
        | Operator::FheNe
        | Operator::FheGe
        | Operator::FheGt
        | Operator::FheLe
        | Operator::FheLt => lhs_type,
        _ => binary_result_type(op, lhs_type),
    }
}

fn requires_scalar_rhs(op: Operator) -> bool {
    matches!(op, Operator::FheDiv | Operator::FheRem)
}

fn validate_type(candidate: FheType, allowed: &[FheType]) -> Result<()> {
    if allowed.contains(&candidate) {
        Ok(())
    } else {
        Err(HostContractError::UnsupportedType(candidate))
    }
}

fn scalar_is_zero_for_type(value: [u8; 32], fhe_type: FheType) -> Result<bool> {
    let bit_width = fhe_type
        .bit_width()
        .ok_or(HostContractError::UnsupportedType(fhe_type))?;
    let byte_width = usize::from(bit_width / 8);
    Ok(value[32 - byte_width..].iter().all(|byte| *byte == 0))
}

fn is_power_of_two_word(word: &[u8; 32]) -> bool {
    let mut set_bits = 0_u32;
    for byte in word {
        set_bits += byte.count_ones();
        if set_bits > 1 {
            return false;
        }
    }
    set_bits == 1
}

fn upper_bound_exceeds_max_type(upper_bound: &[u8; 32], rand_type: FheType) -> Result<bool> {
    let bit_width = rand_type
        .bit_width()
        .ok_or(HostContractError::UnsupportedType(rand_type))?;
    if bit_width >= 256 {
        return Ok(false);
    }
    Ok(*upper_bound > power_of_two_word(bit_width))
}

fn power_of_two_word(bit_width: u16) -> [u8; 32] {
    let mut word = [0_u8; 32];
    let byte_index = 31 - usize::from(bit_width / 8);
    let bit_in_byte = (bit_width % 8) as u8;
    word[byte_index] = 1_u8 << bit_in_byte;
    word
}

fn u256_word_from_u64(value: u64) -> [u8; 32] {
    let mut word = [0_u8; 32];
    word[24..].copy_from_slice(&value.to_be_bytes());
    word
}

fn seed_to_32(seed: [u8; 16]) -> [u8; 32] {
    let mut bytes = [0_u8; 32];
    bytes[..16].copy_from_slice(&seed);
    bytes
}

fn keccak(bytes: Vec<u8>) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(bytes);
    hasher.finalize().into()
}
