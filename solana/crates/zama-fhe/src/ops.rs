//! `FheExecutionBuilder`'s named op methods — the catalog side of the builder.
//!
//! Public API surface: app programs. The op methods mirror the host's op set one-for-one on
//! purpose — an app author who can call `add` must be able to call `mul`, `shl`, or `is_in` — so
//! the surface is complete by design and is not trimmed to whatever the demo programs happen to
//! use. The host-side cost table and the operand validation are what keep it honest.
//!
//! Every method validates its operands, then appends through
//! [`FheExecutionBuilder::commit_step`] in `builder.rs` — the admission machine that owns the
//! step cap, the trace check, and the per-step heap gate. Nothing in this file mutates the
//! builder's tables directly.

use zama_host::{
    CoprocessorInputAttestation, FheBinaryOpCode, FheExecuteOperand, FheExecuteStep,
    FheTernaryOpCode, FheUnaryOpCode,
};

use crate::acl::{BoundedU64UpperBound, Output};
use crate::builder::FheExecutionBuilder;
use crate::heap_tally::TalliedVec;
use crate::operand::{Operand, OperandKind};
use crate::types::{
    binary_rhs_operand, BinaryRhs, Bool, Encrypted, FheBitwise, FheEq, FheIsIn, FheNeg, FheNot,
    FheRandom, FheShift, FheType, FheTyped, FheUint, Scalar, Uint,
};
use crate::validate::{
    handle_fhe_type, max_reduction_operands, operand_fhe_type, scalar_is_zero_for_type,
    validate_binary_step, validate_supported_fhe_type, validate_supported_rand_type,
    validate_ternary_step, validate_uint_fhe_type, validate_unary_step,
};
use crate::{FheExecutionBuildError, Result};

impl<'id> FheExecutionBuilder<'id> {
    /// Introduces a coprocessor-attested external input as a transient operand — the Solana analog
    /// of EVM `FHE.fromExternal`. The host re-verifies the attestation in-execution and requires the
    /// caller to be the attested contract (`compute_subject == contract_address`); derived outputs
    /// are then unconstrained, exactly like EVM `allowTransient(input, msg.sender)`. The returned
    /// value is an operand usable only in later steps of this builder.
    pub fn verified_input<T: FheTyped>(
        &mut self,
        attestation: CoprocessorInputAttestation,
    ) -> Result<Encrypted<'id, T>> {
        if handle_fhe_type(attestation.input_handle) != T::FHE_TYPE.byte() {
            return Err(FheExecutionBuildError::UnsupportedFheType);
        }
        let attestation_index = u8::try_from(self.verified_inputs.len())
            .map_err(|_| FheExecutionBuildError::TooManySteps)?;
        let input_handle = attestation.input_handle;
        // The attestation moves in — its tables are the app's own bytes — but the registry
        // vector itself grows by doubling, and that growth is builder cost the tallied push pays.
        self.verified_inputs.push(attestation);
        Ok(Encrypted::from_operand(Operand::verified_input(
            input_handle,
            attestation_index,
        )))
    }

    pub fn add<T: FheUint>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::Add,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn sub<T: FheUint>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::Sub,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn ge<T: FheUint>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, Bool>> {
        self.binary_op(
            FheBinaryOpCode::Ge,
            lhs.into().operand(),
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
            return Err(FheExecutionBuildError::ScalarLhsOperand);
        }
        validate_binary_step(op, &lhs, &rhs, output_fhe_type, self.steps.len(), |index| {
            self.produced_types.get(index as usize).copied()
        })?;
        let op_index = self.commit_step(output_fhe_type, |lowering| {
            let lhs = lowering.operand(lhs)?;
            let rhs = lowering.operand(rhs)?;
            let output = lowering.output(output)?;
            Ok(FheExecuteStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
                output,
            })
        })?;
        Ok(Operand::transient(op_index))
    }

    pub fn if_then_else<T: FheTyped>(
        &mut self,
        control: impl Into<Encrypted<'id, Bool>>,
        if_true: impl Into<Encrypted<'id, T>>,
        if_false: impl Into<Encrypted<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        let control = control.into().operand();
        let if_true = if_true.into().operand();
        let if_false = if_false.into().operand();
        let output_fhe_type =
            self.encrypted_operand_type(&if_true, FheExecutionBuildError::ScalarEncryptedOperand)?;
        let output_fhe_type = output_fhe_type.byte();
        validate_ternary_step(
            &control,
            &if_true,
            &if_false,
            output_fhe_type,
            self.steps.len(),
            |index| self.produced_types.get(index as usize).copied(),
        )?;
        let step_index = self.commit_step(output_fhe_type, |lowering| {
            let control = lowering.operand(control)?;
            let if_true = lowering.operand(if_true)?;
            let if_false = lowering.operand(if_false)?;
            let output = lowering.output(output)?;
            Ok(FheExecuteStep::Ternary {
                op: FheTernaryOpCode::IfThenElse,
                control,
                if_true,
                if_false,
                output_fhe_type,
                output,
            })
        })?;
        Ok(Encrypted::from_operand(Operand::transient(step_index)))
    }

    pub fn trivial_encrypt<T: FheTyped>(
        &mut self,
        plaintext: Scalar<T>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
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
        validate_supported_fhe_type(fhe_type)?;
        let step_index = self.commit_step(fhe_type, |lowering| {
            let output = lowering.output(output)?;
            Ok(FheExecuteStep::TrivialEncrypt {
                plaintext,
                fhe_type,
                output,
            })
        })?;
        Ok(Operand::transient(step_index))
    }

    pub fn trivial_encrypt_u64(
        &mut self,
        plaintext: u64,
        output: Output,
    ) -> Result<Encrypted<'id, Uint<64>>> {
        self.trivial_encrypt(Scalar::<Uint<64>>::u64(plaintext), output)
    }

    pub fn rand<T: FheRandom>(&mut self, output: Output) -> Result<Encrypted<'id, T>> {
        self.rand_raw(T::FHE_TYPE, output)
            .map(Encrypted::from_operand)
    }

    pub(crate) fn rand_raw(&mut self, fhe_type: FheType, output: Output) -> Result<Operand> {
        let fhe_type = fhe_type.byte();
        validate_supported_rand_type(fhe_type)?;
        let step_index = self.commit_step(fhe_type, |lowering| {
            let output = lowering.output(output)?;
            Ok(FheExecuteStep::Rand { fhe_type, output })
        })?;
        Ok(Operand::transient(step_index))
    }

    pub fn rand_u64(&mut self, output: Output) -> Result<Encrypted<'id, Uint<64>>> {
        self.rand::<Uint<64>>(output)
    }

    pub fn rand_bounded_u64(
        &mut self,
        upper_bound: BoundedU64UpperBound,
        output: Output,
    ) -> Result<Encrypted<'id, Uint<64>>> {
        let fhe_type = FheType::UINT64.byte();
        let step_index = self.commit_step(fhe_type, |lowering| {
            let output = lowering.output(output)?;
            Ok(FheExecuteStep::RandBounded {
                upper_bound: upper_bound.bytes(),
                fhe_type,
                output,
            })
        })?;
        Ok(Encrypted::from_operand(Operand::transient(step_index)))
    }

    // --- Binary ops not yet exposed as named methods ---

    pub fn mul<T: FheUint>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::Mul,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn div<T: FheUint>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::Div,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn rem<T: FheUint>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::Rem,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn and<T: FheBitwise>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::And,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn or<T: FheBitwise>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::Or,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn xor<T: FheBitwise>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::Xor,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn shl<T: FheShift>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::Shl,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn shr<T: FheShift>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::Shr,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn rotl<T: FheShift>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::Rotl,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn rotr<T: FheShift>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::Rotr,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn eq<T: FheEq>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, Bool>> {
        self.binary_op(
            FheBinaryOpCode::Eq,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn ne<T: FheEq>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, Bool>> {
        self.binary_op(
            FheBinaryOpCode::Ne,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn gt<T: FheUint>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, Bool>> {
        self.binary_op(
            FheBinaryOpCode::Gt,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn le<T: FheUint>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, Bool>> {
        self.binary_op(
            FheBinaryOpCode::Le,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn lt<T: FheUint>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, Bool>> {
        self.binary_op(
            FheBinaryOpCode::Lt,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn min<T: FheUint>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::Min,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn max<T: FheUint>(
        &mut self,
        lhs: impl Into<Encrypted<'id, T>>,
        rhs: impl Into<BinaryRhs<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.binary_op(
            FheBinaryOpCode::Max,
            lhs.into().operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    // --- Unary ops ---

    pub fn neg<T: FheNeg>(
        &mut self,
        operand: impl Into<Encrypted<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.unary_op(
            FheUnaryOpCode::Neg,
            operand.into().operand(),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn not<T: FheNot>(
        &mut self,
        operand: impl Into<Encrypted<'id, T>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        self.unary_op(
            FheUnaryOpCode::Not,
            operand.into().operand(),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn cast<FROM: FheTyped, TO: FheTyped>(
        &mut self,
        operand: impl Into<Encrypted<'id, FROM>>,
        output: Output,
    ) -> Result<Encrypted<'id, TO>> {
        self.unary_op(
            FheUnaryOpCode::Cast,
            operand.into().operand(),
            TO::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn sum<T: FheUint>(
        &mut self,
        operands: impl IntoIterator<Item = impl Into<Encrypted<'id, T>>>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        // EVM `fheSum` and the coprocessor enforce no minimum: a zero/single-operand sum is valid.
        // The table is reserved from the iterator's hint (a 60-operand table grown by doubling
        // would strand nearly its own size on the never-freeing heap); the tallied table pays
        // for the reservation and for any growth past a lying hint, harvested below whether or
        // not a later check rejects the step — on a bump region the bytes are spent either way.
        let operands = operands.into_iter();
        let (reserved_operands, _) = operands.size_hint();
        let mut operand_ops: TalliedVec<Operand> = TalliedVec::with_capacity(reserved_operands);
        for operand in operands {
            operand_ops.push(operand.into().operand());
        }
        self.explicit_heap_bytes += operand_ops.requested_bytes();
        let operand_ops = operand_ops.into_inner();
        for op in &operand_ops {
            if matches!(op.0, OperandKind::Scalar(_)) {
                return Err(FheExecutionBuildError::ScalarEncryptedOperand);
            }
        }
        let fhe_type = T::FHE_TYPE.byte();
        validate_uint_fhe_type(fhe_type)?;
        if operand_ops.len() > max_reduction_operands(fhe_type) {
            return Err(FheExecutionBuildError::TooManyReductionOperands);
        }
        let step_index = self.commit_step(fhe_type, |lowering| {
            lowering.tally_bytes(operand_ops.len() * std::mem::size_of::<FheExecuteOperand>());
            let mut lowered: Vec<FheExecuteOperand> = Vec::with_capacity(operand_ops.len());
            for op in operand_ops {
                lowered.push(lowering.operand(op)?);
            }
            let output = lowering.output(output)?;
            Ok(FheExecuteStep::Sum {
                operands: lowered,
                fhe_type,
                output,
            })
        })?;
        Ok(Encrypted::from_operand(Operand::transient(step_index)))
    }

    pub fn is_in<T: FheIsIn>(
        &mut self,
        value: impl Into<Encrypted<'id, T>>,
        set: impl IntoIterator<Item = impl Into<Encrypted<'id, T>>>,
        output: Output,
    ) -> Result<Encrypted<'id, Bool>> {
        // EVM `fheIsIn` and the coprocessor enforce no minimum: an empty set is valid (false result).
        // Reserved from the iterator's hint and harvested like `sum`'s operand table above.
        let set = set.into_iter();
        let (reserved_set, _) = set.size_hint();
        let mut set_ops: TalliedVec<Operand> = TalliedVec::with_capacity(reserved_set);
        for operand in set {
            set_ops.push(operand.into().operand());
        }
        self.explicit_heap_bytes += set_ops.requested_bytes();
        let set_ops = set_ops.into_inner();
        let value_op = value.into().operand();
        if matches!(value_op.0, OperandKind::Scalar(_)) {
            return Err(FheExecutionBuildError::ScalarEncryptedOperand);
        }
        for op in &set_ops {
            if matches!(op.0, OperandKind::Scalar(_)) {
                return Err(FheExecutionBuildError::ScalarEncryptedOperand);
            }
        }
        let fhe_type = T::FHE_TYPE.byte();
        validate_supported_fhe_type(fhe_type)?;
        if set_ops.len() > max_reduction_operands(fhe_type) {
            return Err(FheExecutionBuildError::TooManyReductionOperands);
        }
        let bool_type = FheType::BOOL.byte();
        let step_index = self.commit_step(bool_type, |lowering| {
            let value = lowering.operand(value_op)?;
            lowering.tally_bytes(set_ops.len() * std::mem::size_of::<FheExecuteOperand>());
            let mut set_lowered: Vec<FheExecuteOperand> = Vec::with_capacity(set_ops.len());
            for op in set_ops {
                set_lowered.push(lowering.operand(op)?);
            }
            let output = lowering.output(output)?;
            Ok(FheExecuteStep::IsIn {
                value,
                set: set_lowered,
                fhe_type,
                output,
            })
        })?;
        Ok(Encrypted::from_operand(Operand::transient(step_index)))
    }

    pub fn mul_div<T: FheUint>(
        &mut self,
        factor1: impl Into<Encrypted<'id, T>>,
        factor2: impl Into<BinaryRhs<'id, T>>,
        divisor: Scalar<T>,
        output: Output,
    ) -> Result<Encrypted<'id, T>> {
        let lhs = factor1.into().operand();
        let rhs = binary_rhs_operand(factor2);
        if matches!(lhs.0, OperandKind::Scalar(_)) {
            return Err(FheExecutionBuildError::ScalarLhsOperand);
        }
        let fhe_type = T::FHE_TYPE.byte();
        validate_uint_fhe_type(fhe_type)?;
        // fheMulDiv factor1 caps at Uint64 (EVM + coprocessor); reject Uint128.
        if !matches!(fhe_type, 2..=5) {
            return Err(FheExecutionBuildError::UnsupportedFheType);
        }
        // Divisor must be non-zero once truncated to the operand type (EVM DivisionByZero parity).
        let divisor_bytes = divisor.bytes();
        if scalar_is_zero_for_type(divisor_bytes, fhe_type) {
            return Err(FheExecutionBuildError::MulDivDivisorZero);
        }
        let step_index = self.commit_step(fhe_type, |lowering| {
            let factor1 = lowering.operand(lhs)?;
            let factor2 = lowering.operand(rhs)?;
            let output = lowering.output(output)?;
            Ok(FheExecuteStep::MulDiv {
                factor1,
                factor2,
                divisor: divisor_bytes,
                output_fhe_type: fhe_type,
                output,
            })
        })?;
        Ok(Encrypted::from_operand(Operand::transient(step_index)))
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
            return Err(FheExecutionBuildError::ScalarEncryptedOperand);
        }
        validate_unary_step(op, &operand, output_fhe_type, self.steps.len(), |index| {
            self.produced_types.get(index as usize).copied()
        })?;
        let step_index = self.commit_step(output_fhe_type, |lowering| {
            let operand = lowering.operand(operand)?;
            let output = lowering.output(output)?;
            Ok(FheExecuteStep::Unary {
                op,
                operand,
                output_fhe_type,
                output,
            })
        })?;
        Ok(Operand::transient(step_index))
    }

    fn encrypted_operand_type(
        &self,
        operand: &Operand,
        scalar_error: FheExecutionBuildError,
    ) -> Result<FheType> {
        let fhe_type = operand_fhe_type(operand, self.steps.len(), &|index| {
            self.produced_types.get(index as usize).copied()
        })?
        .ok_or(scalar_error)?;
        FheType::from_host_byte(fhe_type)
    }
}
