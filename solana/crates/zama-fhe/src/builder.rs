//! `FheExecutionBuilder`: accumulates typed steps and lowers them to the wire execution.
//!
//! Public API surface: app programs. The op methods mirror the host's op set one-for-one on
//! purpose — an app author who can call `add` must be able to call `mul`, `shl`, or `is_in` — so
//! the surface is complete by design and is not trimmed to whatever the demo programs happen to
//! use. The host-side cost table and the operand validation are what keep it honest.
//!
//! Building on-chain: lowering interns into the builder's own tables and never copies them, so a
//! step costs a few hundred heap bytes. Anchor's default allocator is a 32 KB bump region that is
//! never freed, and the instruction pays out of it twice — once building, once serializing the
//! packet in `FheExecution::invoke` — so the budget belongs to the pair: 16 steps that each write a
//! persistent output, far past anything in this repo (the largest is five). 24 still fits with 610
//! bytes to spare, but nothing is left there for account resolution, and an execution near
//! `MAX_FHE_EXECUTION_STEPS` has to be built off-chain or by a program bringing its own allocator.
//! `heap_budget.rs` measures all of it, and [`MAX_ON_CHAIN_EXECUTION_STEPS`] enforces it: a program that
//! keeps adding steps past the budget is told so, instead of being aborted by the allocator with no
//! error of its own.

use crate::types::{binary_rhs_operand, BinaryRhs, FheBitwise, FheEq, FheNeg, FheNot, FheShift};
use crate::validate::handle_fhe_type;

use zama_host::{
    CoprocessorInputAttestation, FheBinaryOpCode, FheExecuteArgs, FheExecuteOperand,
    FheExecuteOutput, FheExecuteStep, FheTernaryOpCode, FheUnaryOpCode, MAX_FHE_EXECUTION_STEPS,
};

use crate::accounts::{ExecutionAccountMeta, ExecutionEncryptedValueAccountAuthority};
use crate::acl::{BoundedU64UpperBound, Output};
use crate::execution::FheExecution;
use crate::lower::{lower_operand, lower_output, StepTables};
use crate::operand::{BuilderBrand, Operand, OperandKind};
use crate::types::{Bool, Encrypted, FheIsIn, FheRandom, FheType, FheTyped, FheUint, Scalar, Uint};
use crate::validate::{
    max_reduction_operands, operand_fhe_type, scalar_is_zero_for_type, validate_binary_step,
    validate_encrypted_value_account_authority, validate_lowered_execution,
    validate_rand_steps_anchor_persistent_output, validate_supported_fhe_type,
    validate_supported_rand_type, validate_ternary_step, validate_uint_fhe_type,
    validate_unary_step,
};
use crate::{FheExecutionBuildError, Result};

/// Pubkey-oriented builder for `FheExecuteArgs`.
///
/// `'brand` is this builder's identity: [`FheExecution::build`] hands every invocation a fresh invariant
/// lifetime, every transient value it returns carries it, and the op methods only accept values of
/// their own brand — so mixing two builders' values does not compile. That replaces a runtime tag
/// which SBF could not make unique (writable statics are forbidden on-chain, so every builder in a
/// program shared one scope number and the check found nothing). It is also why there is no public
/// constructor and no `Clone`: both would hand out a second builder wearing the same brand.
#[derive(Debug)]
pub struct FheExecutionBuilder<'brand> {
    brand: BuilderBrand<'brand>,
    pub(crate) encrypted_value_account_authority: ExecutionEncryptedValueAccountAuthority,
    pub(crate) steps: Vec<FheExecuteStep>,
    pub(crate) produced_types: Vec<u8>,
    /// Persistent accounts this execution has already written. A later persistent-shaped reference
    /// to one of them is rejected with `PersistentOperandWrittenEarlier`: the app must feed the
    /// earlier step's transient value instead, which is the only spelling the host accepts.
    pub(crate) persistent_producers: Vec<anchor_lang::prelude::Pubkey>,
    pub(crate) remaining_accounts: Vec<ExecutionAccountMeta>,
    /// Interned 32-byte constant dictionary the lowered steps reference by `u8` index
    /// (operand handles, scalars, ACL domain keys, app accounts, labels, subjects). The
    /// entries are deliberately untyped so one entry can serve several roles; see
    /// `FheExecuteArgs::dictionary` in zama-host for why typing them would cost packet bytes.
    pub(crate) dictionary: Vec<[u8; 32]>,
    /// Coprocessor attestations backing `VerifiedInput` operands, referenced by index. Held here
    /// (rather than inline in the operand) so `Operand` stays `Copy`.
    pub(crate) verified_inputs: Vec<CoprocessorInputAttestation>,
}

/// One step's view of the builder — see [`FheExecutionBuilder::commit_step`].
struct StepLowering<'b> {
    steps_len: usize,
    encrypted_value_account_authority: ExecutionEncryptedValueAccountAuthority,
    tables: StepTables<'b>,
    verified_inputs: &'b [CoprocessorInputAttestation],
}

impl StepLowering<'_> {
    fn operand(&mut self, operand: Operand) -> Result<FheExecuteOperand> {
        lower_operand(
            &mut self.tables,
            self.steps_len,
            self.verified_inputs,
            operand,
        )
    }

    fn output(&mut self, output: Output) -> Result<FheExecuteOutput> {
        lower_output(
            &mut self.tables,
            self.encrypted_value_account_authority,
            output,
        )
    }
}

/// Steps a program can build *and* invoke inside Anchor's default 32 KB bump heap, measured in
/// `heap_budget.rs`. The host itself accepts up to `MAX_FHE_EXECUTION_STEPS`; this is the smaller limit
/// the runtime imposes on a program that has not raised its heap.
pub const MAX_ON_CHAIN_EXECUTION_STEPS: usize = 16;

/// The ceiling only means something while it is stricter than the host's.
const _: () = assert!(MAX_ON_CHAIN_EXECUTION_STEPS < MAX_FHE_EXECUTION_STEPS);

/// The step ceiling in force for this build of the crate.
///
/// Only a program running under SBF pays the 32 KB budget, so off-chain builders — clients, tests,
/// the e2e live client — keep the host's full `MAX_FHE_EXECUTION_STEPS`. A program that installs its own
/// allocator opts back out with the `raised-heap` feature.
///
/// The `cfg` itself is not exercised by a host test: proving the on-chain branch would need an SBF
/// fixture program built only to overflow it, which is the same reason `heap_budget.rs` counts bytes
/// on the host. What is tested is the limit this returns and the rejection it drives. The predicate
/// is the same one `zama_solana_acl::sha256` uses to select the on-chain hasher, and the cost
/// snapshots would show it immediately if that stopped resolving under SBF.
pub(crate) const fn step_limit() -> usize {
    if cfg!(all(target_os = "solana", not(feature = "raised-heap"))) {
        MAX_ON_CHAIN_EXECUTION_STEPS
    } else {
        MAX_FHE_EXECUTION_STEPS
    }
}

impl<'brand> FheExecutionBuilder<'brand> {
    /// The single mutation path for appending a step. Every op method validates first, then lowers
    /// through this: lowering interns into the builder's own tables and, when any part of the step
    /// fails, [`StepTables::rollback`] undoes what it wrote, so a failed step leaves the builder
    /// exactly as it was. The tables are never copied per step — an app program builds its execution on
    /// Anchor's default 32 KB bump heap, which is never freed, so a clone-and-swap rollback would
    /// make the heap cost of an execution grow with the square of its step count.
    ///
    /// Ordering dependency inside a step: `operand()` reads `persistent_producers`, which still
    /// holds the pre-step state only because every op lowers its operands before its output. An op
    /// that lowered its output first would silently change what `PersistentOperandWrittenEarlier`
    /// sees for its own operands — keep operands first.
    fn commit_step(
        &mut self,
        produced_type: u8,
        lower: impl FnOnce(&mut StepLowering<'_>) -> Result<FheExecuteStep>,
    ) -> Result<u8> {
        // Checked before the step interns anything: past the limit the allocator, not this crate,
        // would be what ends the instruction.
        if self.steps.len() >= step_limit() {
            return Err(if step_limit() == MAX_FHE_EXECUTION_STEPS {
                FheExecutionBuildError::TooManySteps
            } else {
                FheExecutionBuildError::TooManyStepsForDefaultHeap
            });
        }
        let op_index =
            u8::try_from(self.steps.len()).map_err(|_| FheExecutionBuildError::TooManySteps)?;
        let Self {
            encrypted_value_account_authority,
            steps,
            produced_types,
            persistent_producers,
            remaining_accounts,
            dictionary,
            verified_inputs,
            brand: _,
        } = self;
        let mut lowering = StepLowering {
            steps_len: steps.len(),
            encrypted_value_account_authority: *encrypted_value_account_authority,
            tables: StepTables::open(remaining_accounts, dictionary, persistent_producers),
            verified_inputs,
        };
        match lower(&mut lowering) {
            Ok(step) => {
                steps.push(step);
                produced_types.push(produced_type);
                Ok(op_index)
            }
            Err(error) => {
                lowering.tables.rollback();
                Err(error)
            }
        }
    }
}

impl<'brand> FheExecutionBuilder<'brand> {
    /// Crate-internal: a public constructor would let two builders share one brand, which is the
    /// mixing hazard the brand exists to remove. App code gets a builder from [`FheExecution::build`].
    pub(crate) fn new(
        encrypted_value_account_authority: ExecutionEncryptedValueAccountAuthority,
    ) -> Self {
        Self {
            brand: std::marker::PhantomData,
            encrypted_value_account_authority,
            steps: Vec::new(),
            produced_types: Vec::new(),
            persistent_producers: Vec::new(),
            remaining_accounts: Vec::new(),
            dictionary: Vec::new(),
            verified_inputs: Vec::new(),
        }
    }

    /// Introduces a coprocessor-attested external input as a transient operand — the Solana analog
    /// of EVM `FHE.fromExternal`. The host re-verifies the attestation in-execution and requires the
    /// caller to be the attested contract (`compute_subject == contract_address`); derived outputs
    /// are then unconstrained, exactly like EVM `allowTransient(input, msg.sender)`. The returned
    /// value is an operand usable only in later steps of this builder.
    pub fn verified_input<T: FheTyped>(
        &mut self,
        attestation: CoprocessorInputAttestation,
    ) -> Result<Encrypted<'brand, T>> {
        if handle_fhe_type(attestation.input_handle) != T::FHE_TYPE.byte() {
            return Err(FheExecutionBuildError::UnsupportedFheType);
        }
        let attestation_index = u8::try_from(self.verified_inputs.len())
            .map_err(|_| FheExecutionBuildError::TooManySteps)?;
        let input_handle = attestation.input_handle;
        self.verified_inputs.push(attestation);
        Ok(Encrypted::from_operand(Operand::verified_input(
            input_handle,
            attestation_index,
        )))
    }

    pub fn add<T: FheUint>(
        &mut self,
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, Bool>> {
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
        if self.steps.len() >= MAX_FHE_EXECUTION_STEPS {
            return Err(FheExecutionBuildError::TooManySteps);
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
        control: impl Into<Encrypted<'brand, Bool>>,
        if_true: impl Into<Encrypted<'brand, T>>,
        if_false: impl Into<Encrypted<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
        let control = control.into().operand();
        let if_true = if_true.into().operand();
        let if_false = if_false.into().operand();
        let output_fhe_type =
            self.encrypted_operand_type(&if_true, FheExecutionBuildError::ScalarEncryptedOperand)?;
        let output_fhe_type = output_fhe_type.byte();
        if self.steps.len() >= MAX_FHE_EXECUTION_STEPS {
            return Err(FheExecutionBuildError::TooManySteps);
        }
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
    ) -> Result<Encrypted<'brand, T>> {
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
        if self.steps.len() >= MAX_FHE_EXECUTION_STEPS {
            return Err(FheExecutionBuildError::TooManySteps);
        }
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
    ) -> Result<Encrypted<'brand, Uint<64>>> {
        self.trivial_encrypt(Scalar::<Uint<64>>::u64(plaintext), output)
    }

    pub fn rand<T: FheRandom>(&mut self, output: Output) -> Result<Encrypted<'brand, T>> {
        self.rand_raw(T::FHE_TYPE, output)
            .map(Encrypted::from_operand)
    }

    pub(crate) fn rand_raw(&mut self, fhe_type: FheType, output: Output) -> Result<Operand> {
        let fhe_type = fhe_type.byte();
        if self.steps.len() >= MAX_FHE_EXECUTION_STEPS {
            return Err(FheExecutionBuildError::TooManySteps);
        }
        validate_supported_rand_type(fhe_type)?;
        let step_index = self.commit_step(fhe_type, |lowering| {
            let output = lowering.output(output)?;
            Ok(FheExecuteStep::Rand { fhe_type, output })
        })?;
        Ok(Operand::transient(step_index))
    }

    pub fn rand_u64(&mut self, output: Output) -> Result<Encrypted<'brand, Uint<64>>> {
        self.rand::<Uint<64>>(output)
    }

    pub fn rand_bounded_u64(
        &mut self,
        upper_bound: BoundedU64UpperBound,
        output: Output,
    ) -> Result<Encrypted<'brand, Uint<64>>> {
        let fhe_type = FheType::UINT64.byte();
        if self.steps.len() >= MAX_FHE_EXECUTION_STEPS {
            return Err(FheExecutionBuildError::TooManySteps);
        }
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, Bool>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, Bool>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, Bool>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, Bool>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, Bool>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        lhs: impl Into<Encrypted<'brand, T>>,
        rhs: impl Into<BinaryRhs<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        operand: impl Into<Encrypted<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        operand: impl Into<Encrypted<'brand, T>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
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
        operand: impl Into<Encrypted<'brand, FROM>>,
        output: Output,
    ) -> Result<Encrypted<'brand, TO>> {
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
        operands: impl IntoIterator<Item = impl Into<Encrypted<'brand, T>>>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
        // EVM `fheSum` and the coprocessor enforce no minimum: a zero/single-operand sum is valid.
        let operand_ops: Vec<Operand> = operands.into_iter().map(|e| e.into().operand()).collect();
        for op in &operand_ops {
            if matches!(op.0, OperandKind::Scalar(_)) {
                return Err(FheExecutionBuildError::ScalarEncryptedOperand);
            }
        }
        if self.steps.len() >= MAX_FHE_EXECUTION_STEPS {
            return Err(FheExecutionBuildError::TooManySteps);
        }
        let fhe_type = T::FHE_TYPE.byte();
        validate_uint_fhe_type(fhe_type)?;
        if operand_ops.len() > max_reduction_operands(fhe_type) {
            return Err(FheExecutionBuildError::TooManyReductionOperands);
        }
        let step_index = self.commit_step(fhe_type, |lowering| {
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
        value: impl Into<Encrypted<'brand, T>>,
        set: impl IntoIterator<Item = impl Into<Encrypted<'brand, T>>>,
        output: Output,
    ) -> Result<Encrypted<'brand, Bool>> {
        // EVM `fheIsIn` and the coprocessor enforce no minimum: an empty set is valid (false result).
        let set_ops: Vec<Operand> = set.into_iter().map(|e| e.into().operand()).collect();
        let value_op = value.into().operand();
        if matches!(value_op.0, OperandKind::Scalar(_)) {
            return Err(FheExecutionBuildError::ScalarEncryptedOperand);
        }
        for op in &set_ops {
            if matches!(op.0, OperandKind::Scalar(_)) {
                return Err(FheExecutionBuildError::ScalarEncryptedOperand);
            }
        }
        if self.steps.len() >= MAX_FHE_EXECUTION_STEPS {
            return Err(FheExecutionBuildError::TooManySteps);
        }
        let fhe_type = T::FHE_TYPE.byte();
        validate_supported_fhe_type(fhe_type)?;
        if set_ops.len() > max_reduction_operands(fhe_type) {
            return Err(FheExecutionBuildError::TooManyReductionOperands);
        }
        let bool_type = FheType::BOOL.byte();
        let step_index = self.commit_step(bool_type, |lowering| {
            let value = lowering.operand(value_op)?;
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
        factor1: impl Into<Encrypted<'brand, T>>,
        factor2: impl Into<BinaryRhs<'brand, T>>,
        divisor: Scalar<T>,
        output: Output,
    ) -> Result<Encrypted<'brand, T>> {
        let lhs = factor1.into().operand();
        let rhs = binary_rhs_operand(factor2);
        if matches!(lhs.0, OperandKind::Scalar(_)) {
            return Err(FheExecutionBuildError::ScalarLhsOperand);
        }
        if self.steps.len() >= MAX_FHE_EXECUTION_STEPS {
            return Err(FheExecutionBuildError::TooManySteps);
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
        if self.steps.len() >= MAX_FHE_EXECUTION_STEPS {
            return Err(FheExecutionBuildError::TooManySteps);
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

    /// Validates the accumulated execution and lowers it to an [`FheExecution`].
    ///
    /// Mirrors the host preflight checks (non-empty steps,
    /// `steps.len() <= MAX_FHE_EXECUTION_STEPS`, rand steps anchored by a persistent
    /// output) so a malformed execution fails locally instead of on-chain.
    ///
    /// Not mirrored (it depends on the deployed `hcu_block_cap_per_app`, unknown here): under a
    /// finite block cap the host rejects a persist-nothing execution — one binding no persistent input, no
    /// verified input, and no persistent output — with `FheExecuteUnanchoredUnderBlockCap`
    /// (fhevm-internal#1744). Give such an execution a persistent output (the bootstrap/mint path) or a
    /// verified input if it must run under a finite cap.
    pub(crate) fn finish(self) -> Result<FheExecution> {
        validate_encrypted_value_account_authority(self.encrypted_value_account_authority)?;
        if self.steps.is_empty() {
            return Err(FheExecutionBuildError::EmptySteps);
        }
        if self.steps.len() > MAX_FHE_EXECUTION_STEPS {
            return Err(FheExecutionBuildError::TooManySteps);
        }
        validate_lowered_execution(&self.steps, &self.remaining_accounts, &self.dictionary)?;
        validate_rand_steps_anchor_persistent_output(&self.steps)?;
        let account_count = u8::try_from(self.remaining_accounts.len())
            .map_err(|_| FheExecutionBuildError::TooManyRemainingAccounts)?;
        Ok(FheExecution {
            encrypted_value_account_authority: self.encrypted_value_account_authority,
            args: FheExecuteArgs {
                account_count,
                dictionary: self.dictionary,
                steps: self.steps,
            },
            remaining_accounts: self.remaining_accounts,
        })
    }
}
