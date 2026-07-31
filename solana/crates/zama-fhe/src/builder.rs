//! `BatchBuilder`: accumulates typed steps and lowers them to the wire batch.

use crate::types::{binary_rhs_operand, BinaryRhs, FheBitwise, FheEq, FheNeg, FheNot, FheShift};
use crate::validate::handle_fhe_type;

use zama_host::{
    CoprocessorInputAttestation, FheBinaryOpCode, FheExecuteArgs, FheExecuteOperand,
    FheExecuteStep, FheTernaryOpCode, FheUnaryOpCode, MAX_FHE_BATCH_OPS,
};

use crate::accounts::{BatchAccountMeta, BatchAppAuthority};
use crate::acl::{BoundedU64UpperBound, Output};
use crate::batch::Batch;
use crate::lower::{lower_operand, lower_output};
use crate::operand::{next_batch_builder_scope, BatchBuilderScope, Operand, OperandKind};
use crate::types::{Bool, Encrypted, FheIsIn, FheRandom, FheType, FheTyped, FheUint, Scalar, Uint};
use crate::validate::{
    max_reduction_operands, operand_fhe_type, scalar_is_zero_for_type, validate_app_authority,
    validate_binary_step, validate_lowered_batch, validate_rand_steps_anchor_persistent_output,
    validate_supported_fhe_type, validate_supported_rand_type, validate_ternary_step,
    validate_uint_fhe_type, validate_unary_step,
};
use crate::{BatchBuildError, Result};

/// Pubkey-oriented builder for `FheExecuteArgs`.
#[derive(Debug)]
pub struct BatchBuilder {
    pub(crate) scope: BatchBuilderScope,
    pub(crate) app_authority: BatchAppAuthority,
    pub(crate) steps: Vec<FheExecuteStep>,
    pub(crate) produced_types: Vec<u8>,
    /// Latest producer for every persistent account written by this batch. A later
    /// persistent-shaped reference to the same account is lowered canonically as
    /// `AllowedLocal`.
    pub(crate) persistent_producers: Vec<(anchor_lang::prelude::Pubkey, u16)>,
    pub(crate) remaining_accounts: Vec<BatchAccountMeta>,
    /// Interned 32-byte constant dictionary the lowered steps reference by `u8` index
    /// (operand handles, scalars, ACL domain keys, app accounts, labels, subjects).
    pub(crate) dictionary: Vec<[u8; 32]>,
    /// Coprocessor attestations backing `VerifiedInput` operands, referenced by index. Held here
    /// (rather than inline in the operand) so `Operand` stays `Copy`.
    pub(crate) verified_inputs: Vec<CoprocessorInputAttestation>,
}

impl Clone for BatchBuilder {
    fn clone(&self) -> Self {
        Self {
            scope: next_batch_builder_scope(),
            app_authority: self.app_authority,
            steps: self.steps.clone(),
            produced_types: self.produced_types.clone(),
            persistent_producers: self.persistent_producers.clone(),
            remaining_accounts: self.remaining_accounts.clone(),
            dictionary: self.dictionary.clone(),
            verified_inputs: self.verified_inputs.clone(),
        }
    }
}

impl BatchBuilder {
    pub fn new(app_authority: BatchAppAuthority) -> Self {
        Self {
            scope: next_batch_builder_scope(),
            app_authority,
            steps: Vec::new(),
            produced_types: Vec::new(),
            persistent_producers: Vec::new(),
            remaining_accounts: Vec::new(),
            dictionary: Vec::new(),
            verified_inputs: Vec::new(),
        }
    }

    /// Introduces a coprocessor-attested external input as a transient operand — the Solana analog
    /// of EVM `FHE.fromExternal`. The host re-verifies the attestation in-batch and requires the
    /// caller to be the attested contract (`compute_subject == contract_address`); derived outputs
    /// are then unconstrained, exactly like EVM `allowTransient(input, msg.sender)`. The returned
    /// value is an operand usable only in later steps of this builder.
    pub fn verified_input<T: FheTyped>(
        &mut self,
        attestation: CoprocessorInputAttestation,
    ) -> Result<Encrypted<T>> {
        if handle_fhe_type(attestation.input_handle) != T::FHE_TYPE.byte() {
            return Err(BatchBuildError::UnsupportedFheType);
        }
        let attestation_index =
            u16::try_from(self.verified_inputs.len()).map_err(|_| BatchBuildError::TooManyOps)?;
        let input_handle = attestation.input_handle;
        self.verified_inputs.push(attestation);
        Ok(Encrypted::from_operand(Operand::verified_input(
            input_handle,
            attestation_index,
        )))
    }

    pub fn add<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Add,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn sub<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Sub,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn ge<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        self.binary_op(
            FheBinaryOpCode::Ge,
            lhs.operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub(crate) fn binary_op(
        &mut self,
        op: FheBinaryOpCode,
        lhs: Operand,
        rhs: Operand,
        output_fhe_type: FheType,
        output: Output,
    ) -> Result<Operand> {
        let output_fhe_type = output_fhe_type.byte();
        // The host requires the left operand to be an encrypted handle; only the
        // RHS may be a plaintext scalar. Catch this before the CPI.
        if matches!(lhs.0, OperandKind::Scalar(_)) {
            return Err(BatchBuildError::ScalarLhsOperand);
        }
        if self.steps.len() >= MAX_FHE_BATCH_OPS {
            return Err(BatchBuildError::TooManyOps);
        }
        validate_binary_step(
            op,
            &lhs,
            &rhs,
            output_fhe_type,
            self.steps.len(),
            self.scope,
            |index| self.produced_types.get(index as usize).copied(),
        )?;
        let op_index = u16::try_from(self.steps.len()).map_err(|_| BatchBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let mut dictionary = self.dictionary.clone();
        let lhs = lower_operand(
            &mut remaining_accounts,
            &mut dictionary,
            self.steps.len(),
            self.scope,
            &self.persistent_producers,
            &self.verified_inputs,
            lhs,
        )?;
        let rhs = lower_operand(
            &mut remaining_accounts,
            &mut dictionary,
            self.steps.len(),
            self.scope,
            &self.persistent_producers,
            &self.verified_inputs,
            rhs,
        )?;
        let output = lower_output(
            &mut remaining_accounts,
            &mut dictionary,
            self.app_authority,
            &mut self.persistent_producers,
            op_index,
            output,
        )?;
        self.remaining_accounts = remaining_accounts;
        self.dictionary = dictionary;
        self.steps.push(FheExecuteStep::Binary {
            op,
            lhs,
            rhs,
            output_fhe_type,
            output,
        });
        self.produced_types.push(output_fhe_type);
        Ok(Operand::transient(op_index, self.scope))
    }

    pub fn if_then_else<T: FheTyped>(
        &mut self,
        control: Encrypted<Bool>,
        if_true: Encrypted<T>,
        if_false: Encrypted<T>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        let control = control.operand();
        let if_true = if_true.operand();
        let if_false = if_false.operand();
        let output_fhe_type =
            self.encrypted_operand_type(&if_true, BatchBuildError::ScalarEncryptedOperand)?;
        let output_fhe_type = output_fhe_type.byte();
        if self.steps.len() >= MAX_FHE_BATCH_OPS {
            return Err(BatchBuildError::TooManyOps);
        }
        validate_ternary_step(
            &control,
            &if_true,
            &if_false,
            output_fhe_type,
            self.steps.len(),
            |index| self.produced_types.get(index as usize).copied(),
            self.scope,
        )?;
        let step_index =
            u16::try_from(self.steps.len()).map_err(|_| BatchBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let mut dictionary = self.dictionary.clone();
        let control = lower_operand(
            &mut remaining_accounts,
            &mut dictionary,
            self.steps.len(),
            self.scope,
            &self.persistent_producers,
            &self.verified_inputs,
            control,
        )?;
        let if_true = lower_operand(
            &mut remaining_accounts,
            &mut dictionary,
            self.steps.len(),
            self.scope,
            &self.persistent_producers,
            &self.verified_inputs,
            if_true,
        )?;
        let if_false = lower_operand(
            &mut remaining_accounts,
            &mut dictionary,
            self.steps.len(),
            self.scope,
            &self.persistent_producers,
            &self.verified_inputs,
            if_false,
        )?;
        let output = lower_output(
            &mut remaining_accounts,
            &mut dictionary,
            self.app_authority,
            &mut self.persistent_producers,
            step_index,
            output,
        )?;
        self.remaining_accounts = remaining_accounts;
        self.dictionary = dictionary;
        self.steps.push(FheExecuteStep::Ternary {
            op: FheTernaryOpCode::IfThenElse,
            control,
            if_true,
            if_false,
            output_fhe_type,
            output,
        });
        self.produced_types.push(output_fhe_type);
        Ok(Encrypted::from_operand(Operand::transient(
            step_index, self.scope,
        )))
    }

    pub fn trivial_encrypt<T: FheTyped>(
        &mut self,
        plaintext: Scalar<T>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.trivial_encrypt_raw(plaintext.bytes(), T::FHE_TYPE, output)
            .map(Encrypted::from_operand)
    }

    fn trivial_encrypt_raw(
        &mut self,
        plaintext: [u8; 32],
        fhe_type: FheType,
        output: Output,
    ) -> Result<Operand> {
        let fhe_type = fhe_type.byte();
        if self.steps.len() >= MAX_FHE_BATCH_OPS {
            return Err(BatchBuildError::TooManyOps);
        }
        validate_supported_fhe_type(fhe_type)?;
        let step_index =
            u16::try_from(self.steps.len()).map_err(|_| BatchBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let mut dictionary = self.dictionary.clone();
        let output = lower_output(
            &mut remaining_accounts,
            &mut dictionary,
            self.app_authority,
            &mut self.persistent_producers,
            step_index,
            output,
        )?;
        self.remaining_accounts = remaining_accounts;
        self.dictionary = dictionary;
        self.steps.push(FheExecuteStep::TrivialEncrypt {
            plaintext,
            fhe_type,
            output,
        });
        self.produced_types.push(fhe_type);
        Ok(Operand::transient(step_index, self.scope))
    }

    pub fn trivial_encrypt_u64(
        &mut self,
        plaintext: u64,
        output: Output,
    ) -> Result<Encrypted<Uint<64>>> {
        self.trivial_encrypt(Scalar::<Uint<64>>::u64(plaintext), output)
    }

    pub fn rand<T: FheRandom>(&mut self, output: Output) -> Result<Encrypted<T>> {
        self.rand_raw(T::FHE_TYPE, output)
            .map(Encrypted::from_operand)
    }

    pub(crate) fn rand_raw(&mut self, fhe_type: FheType, output: Output) -> Result<Operand> {
        let fhe_type = fhe_type.byte();
        if self.steps.len() >= MAX_FHE_BATCH_OPS {
            return Err(BatchBuildError::TooManyOps);
        }
        validate_supported_rand_type(fhe_type)?;
        let step_index =
            u16::try_from(self.steps.len()).map_err(|_| BatchBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let mut dictionary = self.dictionary.clone();
        let output = lower_output(
            &mut remaining_accounts,
            &mut dictionary,
            self.app_authority,
            &mut self.persistent_producers,
            step_index,
            output,
        )?;
        self.remaining_accounts = remaining_accounts;
        self.dictionary = dictionary;
        self.steps.push(FheExecuteStep::Rand { fhe_type, output });
        self.produced_types.push(fhe_type);
        Ok(Operand::transient(step_index, self.scope))
    }

    pub fn rand_u64(&mut self, output: Output) -> Result<Encrypted<Uint<64>>> {
        self.rand::<Uint<64>>(output)
    }

    pub fn rand_bounded_u64(
        &mut self,
        upper_bound: BoundedU64UpperBound,
        output: Output,
    ) -> Result<Encrypted<Uint<64>>> {
        let fhe_type = FheType::UINT64.byte();
        if self.steps.len() >= MAX_FHE_BATCH_OPS {
            return Err(BatchBuildError::TooManyOps);
        }
        let step_index =
            u16::try_from(self.steps.len()).map_err(|_| BatchBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let mut dictionary = self.dictionary.clone();
        let output = lower_output(
            &mut remaining_accounts,
            &mut dictionary,
            self.app_authority,
            &mut self.persistent_producers,
            step_index,
            output,
        )?;
        self.remaining_accounts = remaining_accounts;
        self.dictionary = dictionary;
        self.steps.push(FheExecuteStep::RandBounded {
            upper_bound: upper_bound.bytes(),
            fhe_type,
            output,
        });
        self.produced_types.push(fhe_type);
        Ok(Encrypted::from_operand(Operand::transient(
            step_index, self.scope,
        )))
    }

    // --- Binary ops not yet exposed as named methods ---

    pub fn mul<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Mul,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn div<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Div,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn rem<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Rem,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn and<T: FheBitwise>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::And,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn or<T: FheBitwise>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Or,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn xor<T: FheBitwise>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Xor,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn shl<T: FheShift>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Shl,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn shr<T: FheShift>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Shr,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn rotl<T: FheShift>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Rotl,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn rotr<T: FheShift>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Rotr,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn eq<T: FheEq>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        self.binary_op(
            FheBinaryOpCode::Eq,
            lhs.operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn ne<T: FheEq>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        self.binary_op(
            FheBinaryOpCode::Ne,
            lhs.operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn gt<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        self.binary_op(
            FheBinaryOpCode::Gt,
            lhs.operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn le<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        self.binary_op(
            FheBinaryOpCode::Le,
            lhs.operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn lt<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        self.binary_op(
            FheBinaryOpCode::Lt,
            lhs.operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn min<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Min,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn max<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Max,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    // --- Unary ops ---

    pub fn neg<T: FheNeg>(
        &mut self,
        operand: Encrypted<T>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.unary_op(FheUnaryOpCode::Neg, operand.operand(), T::FHE_TYPE, output)
            .map(Encrypted::from_operand)
    }

    pub fn not<T: FheNot>(
        &mut self,
        operand: Encrypted<T>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.unary_op(FheUnaryOpCode::Not, operand.operand(), T::FHE_TYPE, output)
            .map(Encrypted::from_operand)
    }

    pub fn cast<FROM: FheTyped, TO: FheTyped>(
        &mut self,
        operand: Encrypted<FROM>,
        output: Output,
    ) -> Result<Encrypted<TO>> {
        self.unary_op(
            FheUnaryOpCode::Cast,
            operand.operand(),
            TO::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn sum<T: FheUint>(
        &mut self,
        operands: impl IntoIterator<Item = Encrypted<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        // EVM `fheSum` and the coprocessor enforce no minimum: a zero/single-operand sum is valid.
        let operand_ops: Vec<Operand> = operands.into_iter().map(|e| e.operand()).collect();
        for op in &operand_ops {
            if matches!(op.0, OperandKind::Scalar(_)) {
                return Err(BatchBuildError::ScalarEncryptedOperand);
            }
        }
        if self.steps.len() >= MAX_FHE_BATCH_OPS {
            return Err(BatchBuildError::TooManyOps);
        }
        let fhe_type = T::FHE_TYPE.byte();
        validate_uint_fhe_type(fhe_type)?;
        if operand_ops.len() > max_reduction_operands(fhe_type) {
            return Err(BatchBuildError::TooManyReductionOperands);
        }
        let step_index =
            u16::try_from(self.steps.len()).map_err(|_| BatchBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let mut dictionary = self.dictionary.clone();
        let mut lowered: Vec<FheExecuteOperand> = Vec::with_capacity(operand_ops.len());
        for op in operand_ops {
            lowered.push(lower_operand(
                &mut remaining_accounts,
                &mut dictionary,
                self.steps.len(),
                self.scope,
                &self.persistent_producers,
                &self.verified_inputs,
                op,
            )?);
        }
        let output = lower_output(
            &mut remaining_accounts,
            &mut dictionary,
            self.app_authority,
            &mut self.persistent_producers,
            step_index,
            output,
        )?;
        self.remaining_accounts = remaining_accounts;
        self.dictionary = dictionary;
        self.steps.push(FheExecuteStep::Sum {
            operands: lowered,
            fhe_type,
            output,
        });
        self.produced_types.push(fhe_type);
        Ok(Encrypted::from_operand(Operand::transient(
            step_index, self.scope,
        )))
    }

    pub fn is_in<T: FheIsIn>(
        &mut self,
        value: Encrypted<T>,
        set: impl IntoIterator<Item = Encrypted<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        // EVM `fheIsIn` and the coprocessor enforce no minimum: an empty set is valid (false result).
        let set_ops: Vec<Operand> = set.into_iter().map(|e| e.operand()).collect();
        let value_op = value.operand();
        if matches!(value_op.0, OperandKind::Scalar(_)) {
            return Err(BatchBuildError::ScalarEncryptedOperand);
        }
        for op in &set_ops {
            if matches!(op.0, OperandKind::Scalar(_)) {
                return Err(BatchBuildError::ScalarEncryptedOperand);
            }
        }
        if self.steps.len() >= MAX_FHE_BATCH_OPS {
            return Err(BatchBuildError::TooManyOps);
        }
        let fhe_type = T::FHE_TYPE.byte();
        validate_supported_fhe_type(fhe_type)?;
        if set_ops.len() > max_reduction_operands(fhe_type) {
            return Err(BatchBuildError::TooManyReductionOperands);
        }
        let step_index =
            u16::try_from(self.steps.len()).map_err(|_| BatchBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let mut dictionary = self.dictionary.clone();
        let value_lowered = lower_operand(
            &mut remaining_accounts,
            &mut dictionary,
            self.steps.len(),
            self.scope,
            &self.persistent_producers,
            &self.verified_inputs,
            value_op,
        )?;
        let mut set_lowered: Vec<FheExecuteOperand> = Vec::with_capacity(set_ops.len());
        for op in set_ops {
            set_lowered.push(lower_operand(
                &mut remaining_accounts,
                &mut dictionary,
                self.steps.len(),
                self.scope,
                &self.persistent_producers,
                &self.verified_inputs,
                op,
            )?);
        }
        let output = lower_output(
            &mut remaining_accounts,
            &mut dictionary,
            self.app_authority,
            &mut self.persistent_producers,
            step_index,
            output,
        )?;
        self.remaining_accounts = remaining_accounts;
        self.dictionary = dictionary;
        let bool_type = FheType::BOOL.byte();
        self.steps.push(FheExecuteStep::IsIn {
            value: value_lowered,
            set: set_lowered,
            fhe_type,
            output,
        });
        self.produced_types.push(bool_type);
        Ok(Encrypted::from_operand(Operand::transient(
            step_index, self.scope,
        )))
    }

    pub fn mul_div<T: FheUint>(
        &mut self,
        factor1: Encrypted<T>,
        factor2: impl Into<BinaryRhs<T>>,
        divisor: Scalar<T>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        let lhs = factor1.operand();
        let rhs = binary_rhs_operand(factor2);
        if matches!(lhs.0, OperandKind::Scalar(_)) {
            return Err(BatchBuildError::ScalarLhsOperand);
        }
        if self.steps.len() >= MAX_FHE_BATCH_OPS {
            return Err(BatchBuildError::TooManyOps);
        }
        let fhe_type = T::FHE_TYPE.byte();
        validate_uint_fhe_type(fhe_type)?;
        // fheMulDiv factor1 caps at Uint64 (EVM + coprocessor); reject Uint128.
        if !matches!(fhe_type, 2..=5) {
            return Err(BatchBuildError::UnsupportedFheType);
        }
        // Divisor must be non-zero once truncated to the operand type (EVM DivisionByZero parity).
        let divisor_bytes = divisor.bytes();
        if scalar_is_zero_for_type(divisor_bytes, fhe_type) {
            return Err(BatchBuildError::MulDivDivisorZero);
        }
        let step_index =
            u16::try_from(self.steps.len()).map_err(|_| BatchBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let mut dictionary = self.dictionary.clone();
        let factor1 = lower_operand(
            &mut remaining_accounts,
            &mut dictionary,
            self.steps.len(),
            self.scope,
            &self.persistent_producers,
            &self.verified_inputs,
            lhs,
        )?;
        let factor2 = lower_operand(
            &mut remaining_accounts,
            &mut dictionary,
            self.steps.len(),
            self.scope,
            &self.persistent_producers,
            &self.verified_inputs,
            rhs,
        )?;
        let output = lower_output(
            &mut remaining_accounts,
            &mut dictionary,
            self.app_authority,
            &mut self.persistent_producers,
            step_index,
            output,
        )?;
        self.remaining_accounts = remaining_accounts;
        self.dictionary = dictionary;
        self.steps.push(FheExecuteStep::MulDiv {
            factor1,
            factor2,
            divisor: divisor_bytes,
            output_fhe_type: fhe_type,
            output,
        });
        self.produced_types.push(fhe_type);
        Ok(Encrypted::from_operand(Operand::transient(
            step_index, self.scope,
        )))
    }

    pub(crate) fn unary_op(
        &mut self,
        op: FheUnaryOpCode,
        operand: Operand,
        output_fhe_type: FheType,
        output: Output,
    ) -> Result<Operand> {
        let output_fhe_type = output_fhe_type.byte();
        if matches!(operand.0, OperandKind::Scalar(_)) {
            return Err(BatchBuildError::ScalarEncryptedOperand);
        }
        if self.steps.len() >= MAX_FHE_BATCH_OPS {
            return Err(BatchBuildError::TooManyOps);
        }
        validate_unary_step(
            op,
            &operand,
            output_fhe_type,
            self.steps.len(),
            self.scope,
            |index| self.produced_types.get(index as usize).copied(),
        )?;
        let step_index =
            u16::try_from(self.steps.len()).map_err(|_| BatchBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let mut dictionary = self.dictionary.clone();
        let operand = lower_operand(
            &mut remaining_accounts,
            &mut dictionary,
            self.steps.len(),
            self.scope,
            &self.persistent_producers,
            &self.verified_inputs,
            operand,
        )?;
        let output = lower_output(
            &mut remaining_accounts,
            &mut dictionary,
            self.app_authority,
            &mut self.persistent_producers,
            step_index,
            output,
        )?;
        self.remaining_accounts = remaining_accounts;
        self.dictionary = dictionary;
        self.steps.push(FheExecuteStep::Unary {
            op,
            operand,
            output_fhe_type,
            output,
        });
        self.produced_types.push(output_fhe_type);
        Ok(Operand::transient(step_index, self.scope))
    }

    fn encrypted_operand_type(
        &self,
        operand: &Operand,
        scalar_error: BatchBuildError,
    ) -> Result<FheType> {
        let fhe_type = operand_fhe_type(operand, self.steps.len(), self.scope, &|index| {
            self.produced_types.get(index as usize).copied()
        })?
        .ok_or(scalar_error)?;
        FheType::from_host_byte(fhe_type)
    }

    /// Validates the accumulated batch and lowers it to an [`Batch`].
    ///
    /// Mirrors the host preflight checks (non-empty steps,
    /// `steps.len() <= MAX_FHE_BATCH_OPS`, rand steps anchored by a persistent
    /// output) so a malformed batch fails locally instead of on-chain.
    ///
    /// Not mirrored (it depends on the deployed `hcu_block_cap_per_app`, unknown here): under a
    /// finite block cap the host rejects a persist-nothing batch — one binding no persistent input, no
    /// verified input, and no persistent output — with `FheExecuteUnanchoredUnderBlockCap`
    /// (fhevm-internal#1744). Give such a batch a persistent output (the bootstrap/mint path) or a
    /// verified input if it must run under a finite cap.
    pub fn finish(self) -> Result<Batch> {
        validate_app_authority(self.app_authority)?;
        if self.steps.is_empty() {
            return Err(BatchBuildError::EmptyOps);
        }
        if self.steps.len() > MAX_FHE_BATCH_OPS {
            return Err(BatchBuildError::TooManyOps);
        }
        validate_lowered_batch(&self.steps, &self.remaining_accounts, &self.dictionary)?;
        validate_rand_steps_anchor_persistent_output(&self.steps)?;
        let account_count = u8::try_from(self.remaining_accounts.len())
            .map_err(|_| BatchBuildError::TooManyRemainingAccounts)?;
        Ok(Batch {
            app_authority: self.app_authority,
            args: FheExecuteArgs {
                account_count,
                dictionary: self.dictionary,
                steps: self.steps,
            },
            remaining_accounts: self.remaining_accounts,
        })
    }
}
