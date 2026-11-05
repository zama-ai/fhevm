//! Pure application event metadata, shared by ingestion and application-only fuzz targets.
use crate::contracts::TfheContract;
use crate::contracts::TfheContract::TfheContractEvents;
use alloy::primitives::Address;
use alloy::sol_types::SolEvent;
use alloy_primitives::FixedBytes;
use alloy_primitives::Log as PrimitiveLog;
use alloy_primitives::U256;
use fhevm_engine_common::handle_metadata::get_ct_type;

type Handle = FixedBytes<32>;
type ScalarByte = FixedBytes<1>;
type OperandBoundaryMask = [u8; 32];
const OPERAND_BOUNDARY_MASK_BYTES: usize = 32;

/// Semantic version reported by `FHEVMExecutor.getVersion()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExecutorVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl ExecutorVersion {
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parse only the canonical value returned by the executor contract.
    /// Unknown formats intentionally remain permissive during historical replay.
    pub fn parse(value: &str) -> Option<Self> {
        let version = value.strip_prefix("FHEVMExecutor v")?;
        let mut components = version.split('.');
        let major = parse_canonical_component(components.next()?)?;
        let minor = parse_canonical_component(components.next()?)?;
        let patch = parse_canonical_component(components.next()?)?;
        if components.next().is_some() {
            return None;
        }
        let parsed = Self {
            major,
            minor,
            patch,
        };
        Some(parsed)
    }
}

fn parse_canonical_component(value: &str) -> Option<u64> {
    if value.is_empty()
        || (value.len() > 1 && value.starts_with('0'))
        || !value.bytes().all(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    value.parse().ok()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutorVersionTransition {
    /// The new implementation is active after the `Upgraded` log at this index.
    pub after_log_index: u64,
    /// `None` means the implementation version could not be established.
    pub version: Option<ExecutorVersion>,
}

/// Executor version at a block's start and any upgrades observed in that block.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExecutorVersionSchedule {
    resolved: bool,
    initial_version: Option<ExecutorVersion>,
    transitions: Vec<ExecutorVersionTransition>,
}

impl ExecutorVersionSchedule {
    pub fn new(initial_version: Option<ExecutorVersion>) -> Self {
        Self {
            resolved: true,
            initial_version,
            transitions: Vec::new(),
        }
    }

    pub fn is_resolved(&self) -> bool {
        self.resolved
    }

    pub fn add_transition(
        &mut self,
        after_log_index: u64,
        version: Option<ExecutorVersion>,
    ) {
        self.transitions.push(ExecutorVersionTransition {
            after_log_index,
            version,
        });
        self.transitions
            .sort_by_key(|transition| transition.after_log_index);
    }

    pub fn version_for_log_index(
        &self,
        log_index: u64,
    ) -> Option<ExecutorVersion> {
        let mut version = self.initial_version;
        for transition in &self.transitions {
            if transition.after_log_index >= log_index {
                break;
            }
            version = transition.version;
        }
        version
    }

    pub fn ending_version(&self) -> Option<ExecutorVersion> {
        self.transitions
            .last()
            .map_or(self.initial_version, |transition| transition.version)
    }
}

/// Extract the configured executor proxy's implementation changes from a
/// block. A new implementation is effective after its `Upgraded` log index.
pub fn executor_upgrade_transitions_for_proxy<'a>(
    logs: impl IntoIterator<Item = (Option<u64>, &'a PrimitiveLog)>,
    tfhe_proxy: Address,
) -> Vec<(u64, Address)> {
    let mut upgrades = logs
        .into_iter()
        .filter_map(|(log_index, log)| {
            if log.address != tfhe_proxy {
                return None;
            }
            let log_index = log_index?;
            if log.data.topics().first()
                != Some(&TfheContract::Upgraded::SIGNATURE_HASH)
            {
                return None;
            }
            let upgraded = TfheContract::Upgraded::decode_log(log).ok()?;
            Some((log_index, upgraded.implementation))
        })
        .collect::<Vec<_>>();
    upgrades.sort_by_key(|(log_index, _)| *log_index);
    upgrades
}

/// Before executor d28bce3ae (October 2025), binary operations masked the
/// scalar byte's low bit but emitted the original byte. Replay can cross that
/// upgrade, so accept those historical bytes and interpret the low bit in
/// both dependency extraction and persistence. Div/rem still require a scalar
/// divisor; MulDiv has its own later bitmask encoding.
pub fn validate_scalar_mode(
    op: &TfheContractEvents,
) -> Result<(), &'static str> {
    use TfheContract as C;
    use TfheContractEvents as E;
    match op {
        E::FheDiv(C::FheDiv { scalarByte, .. })
        | E::FheRem(C::FheRem { scalarByte, .. }) => {
            if binary_rhs_is_scalar(scalarByte) {
                Ok(())
            } else {
                Err("division/remainder event requires a scalar divisor")
            }
        }
        E::FheMulDiv(C::FheMulDiv { scalarByte, .. }) => {
            if matches!(scalarByte.0[0], 1 | 3) {
                Ok(())
            } else {
                Err("MulDiv event requires scalarByte 0x01 or 0x03")
            }
        }
        E::FheShl(C::FheShl {
            scalarByte, lhs, ..
        })
        | E::FheShr(C::FheShr {
            scalarByte, lhs, ..
        })
        | E::FheRotl(C::FheRotl {
            scalarByte, lhs, ..
        })
        | E::FheRotr(C::FheRotr {
            scalarByte, lhs, ..
        }) => {
            // The original executor accepted a full uint256 scalar shift.
            // Later versions bounded it by the lhs type, then by uint8.
            // Without an activation height, either bound rejects valid replay.
            if binary_rhs_is_scalar(scalarByte)
                && !matches!(get_ct_type(lhs.as_slice()), Some(2..=6 | 8))
            {
                Err("scalar shift has unsupported operand type")
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

fn scalar_is_zero_for_type(
    value: U256,
    handle_type: &Handle,
) -> Result<bool, &'static str> {
    let width = match get_ct_type(handle_type.as_slice()) {
        Some(2) => 8,
        Some(3) => 16,
        Some(4) => 32,
        Some(5) => 64,
        Some(6) => 128,
        _ => {
            return Err("division/remainder has an unsupported operand FheType")
        }
    };
    let mask = (U256::from(1) << width) - U256::from(1);
    Ok((value & mask) == U256::ZERO)
}

/// Validate only scalar-value constraints shared by the executor versions
/// whose events the listener may replay. This checks event arguments and
/// handle metadata only; it never reads ciphertexts or calls a TFHE operation.
/// Binary scalar range checks, trivial-encryption range checks and MulDiv
/// scalar range checks were added after older executors could emit those
/// events. Since logs do not carry an executor activation version, enforcing
/// the newer checks here could reject valid history. Div/Rem rejected only a
/// full-word zero before executor v0.3. MulDiv typed-zero checks have been
/// present since v0.14 introduced it, and bounded-random constraints have been stable since
/// that operation was introduced. Old shift events can have full-width amounts.
pub fn validate_scalar_values(
    op: &TfheContractEvents,
) -> Result<(), &'static str> {
    use TfheContract as C;
    use TfheContractEvents as E;

    match op {
        E::FheDiv(C::FheDiv {
            rhs, scalarByte, ..
        })
        | E::FheRem(C::FheRem {
            rhs, scalarByte, ..
        }) => {
            if binary_rhs_is_scalar(scalarByte)
                && U256::from_be_bytes(rhs.0) == U256::ZERO
            {
                Err("division/remainder scalar divisor is zero")
            } else {
                Ok(())
            }
        }
        E::FheMulDiv(C::FheMulDiv {
            factor1, divisor, ..
        }) => {
            if !matches!(get_ct_type(factor1.as_slice()), Some(2..=5)) {
                return Err("MulDiv has an unsupported operand FheType");
            }
            if scalar_is_zero_for_type(U256::from_be_bytes(divisor.0), factor1)?
            {
                Err("MulDiv divisor is zero for its FheType")
            } else {
                Ok(())
            }
        }
        E::FheRandBounded(C::FheRandBounded {
            upperBound,
            randType,
            ..
        }) => {
            let max_bound = match *randType {
                2 => U256::from(1) << 8,
                3 => U256::from(1) << 16,
                4 => U256::from(1) << 32,
                5 => U256::from(1) << 64,
                6 => U256::from(1) << 128,
                8 => U256::MAX,
                _ => {
                    return Err(
                        "bounded-random target has an unsupported FheType",
                    )
                }
            };
            if *upperBound == U256::ZERO
                || (*upperBound & (*upperBound - U256::from(1))) != U256::ZERO
            {
                return Err(
                    "bounded-random upper bound is not a nonzero power of two",
                );
            }
            if *upperBound > max_bound {
                return Err(
                    "bounded-random upper bound exceeds its FheType range",
                );
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Apply scalar-range checks introduced by the currently active executor.
pub fn validate_scalar_values_for_current_executor(
    op: &TfheContractEvents,
) -> Result<(), &'static str> {
    validate_scalar_values_for_executor_version(
        op,
        ExecutorVersion::new(0, 7, 0),
    )
}

/// Apply the constraints that were active in the executor version that emitted
/// this event. Version thresholds follow the contract history: Div/Rem typed-zero
/// checks in v0.3, trivial-encrypt range checks in v0.5, binary and MulDiv scalar checks in v0.6, and the
/// uint8 shift cap in v0.7.
pub fn validate_scalar_values_for_executor_version(
    op: &TfheContractEvents,
    version: ExecutorVersion,
) -> Result<(), &'static str> {
    use TfheContract as C;
    use TfheContractEvents as E;

    const V0_3_0: ExecutorVersion = ExecutorVersion::new(0, 3, 0);
    const V0_5_0: ExecutorVersion = ExecutorVersion::new(0, 5, 0);
    const V0_6_0: ExecutorVersion = ExecutorVersion::new(0, 6, 0);
    const V0_7_0: ExecutorVersion = ExecutorVersion::new(0, 7, 0);

    fn fits(value: U256, ty: i16) -> bool {
        let width = match ty {
            0 => 1,
            2 => 8,
            3 => 16,
            4 => 32,
            5 => 64,
            6 => 128,
            7 => 160,
            8 => return true,
            _ => return false,
        };
        value < (U256::from(1) << width)
    }

    validate_scalar_values(op)?;

    // 2da166bece introduced typed-zero rejection with executor v0.3.
    // Earlier contracts emitted nonzero full-word divisors that truncate to
    // zero, and historical replay must continue to admit those events.
    if version >= V0_3_0 {
        if let E::FheDiv(C::FheDiv {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRem(C::FheRem {
            lhs,
            rhs,
            scalarByte,
            ..
        }) = op
        {
            if binary_rhs_is_scalar(scalarByte)
                && scalar_is_zero_for_type(U256::from_be_bytes(rhs.0), lhs)?
            {
                return Err(
                    "division/remainder scalar divisor is zero for its FheType",
                );
            }
        }
    }

    if version >= V0_5_0 {
        if let E::TrivialEncrypt(C::TrivialEncrypt { pt, toType, .. }) = op {
            if !fits(*pt, i16::from(*toType)) {
                return Err(
                    "trivial-encryption value exceeds its target FheType range",
                );
            }
        }
    }

    if version >= V0_6_0 {
        if let Some((lhs, rhs, scalar_byte)) = binary_event_operands(op) {
            if binary_rhs_is_scalar(scalar_byte)
                && !fits(
                    U256::from_be_bytes(rhs.0),
                    get_ct_type(lhs.as_slice()).unwrap_or(255),
                )
            {
                return Err("binary scalar exceeds its operand FheType range");
            }
        }

        if let E::FheMulDiv(C::FheMulDiv {
            factor1,
            factor2,
            divisor,
            scalarByte,
            ..
        }) = op
        {
            let ty = get_ct_type(factor1.as_slice()).unwrap_or(255);
            if !fits(U256::from_be_bytes(divisor.0), ty)
                || (fhe_mul_div_factor2_is_scalar(scalarByte)
                    && !fits(U256::from_be_bytes(factor2.0), ty))
            {
                return Err("MulDiv scalar exceeds its factor FheType range");
            }
        }
    }

    if version >= V0_7_0 {
        if let Some((rhs, scalar_byte)) = scalar_shift_operands(op) {
            if binary_rhs_is_scalar(scalar_byte)
                && U256::from_be_bytes(rhs.0) > U256::from(u8::MAX)
            {
                return Err("scalar shift exceeds the current executor range");
            }
        }
    }

    Ok(())
}

fn binary_event_operands(
    op: &TfheContractEvents,
) -> Option<(&Handle, &Handle, &ScalarByte)> {
    use TfheContract as C;
    use TfheContractEvents as E;

    match op {
        E::FheAdd(C::FheAdd {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheSub(C::FheSub {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMul(C::FheMul {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitAnd(C::FheBitAnd {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitOr(C::FheBitOr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitXor(C::FheBitXor {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheEq(C::FheEq {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheNe(C::FheNe {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheGe(C::FheGe {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheGt(C::FheGt {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheLe(C::FheLe {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheLt(C::FheLt {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMin(C::FheMin {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMax(C::FheMax {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheDiv(C::FheDiv {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRem(C::FheRem {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheShl(C::FheShl {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheShr(C::FheShr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRotl(C::FheRotl {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRotr(C::FheRotr {
            lhs,
            rhs,
            scalarByte,
            ..
        }) => Some((lhs, rhs, scalarByte)),
        _ => None,
    }
}

fn scalar_shift_operands(
    op: &TfheContractEvents,
) -> Option<(&Handle, &ScalarByte)> {
    use TfheContract as C;
    use TfheContractEvents as E;

    match op {
        E::FheShl(C::FheShl {
            rhs, scalarByte, ..
        })
        | E::FheShr(C::FheShr {
            rhs, scalarByte, ..
        })
        | E::FheRotl(C::FheRotl {
            rhs, scalarByte, ..
        })
        | E::FheRotr(C::FheRotr {
            rhs, scalarByte, ..
        }) => Some((rhs, scalarByte)),
        _ => None,
    }
}

/// Check the host operation's declared FheType metadata without inspecting any
/// ciphertext bytes. These relationships are committed by the executor when
/// it emits an event and must agree with the result handle before ingestion.
pub fn validate_type_metadata(
    op: &TfheContractEvents,
) -> Result<(), &'static str> {
    use TfheContract as C;
    use TfheContractEvents as E;

    const BOOL: u8 = 0;
    const U8: u8 = 2;
    const U16: u8 = 3;
    const U32: u8 = 4;
    const U64: u8 = 5;
    const U128: u8 = 6;
    const U160: u8 = 7;
    const U256: u8 = 8;
    const ARITHMETIC: &[u8] = &[U8, U16, U32, U64, U128];
    const NUMERIC: &[u8] = &[U8, U16, U32, U64, U128, U256];
    const BITWISE: &[u8] = &[BOOL, U8, U16, U32, U64, U128, U256];
    const HOST_VALUE: &[u8] = &[BOOL, U8, U16, U32, U64, U128, U160, U256];
    const MEMBERSHIP: &[u8] = &[U8, U16, U32, U64, U128, U160, U256];
    const RANDOM: &[u8] = &[BOOL, U8, U16, U32, U64, U128, U256];

    fn ty(handle: &Handle) -> u8 {
        handle.0[30]
    }
    fn require_allowed(
        handle: &Handle,
        allowed: &[u8],
    ) -> Result<u8, &'static str> {
        let value = ty(handle);
        if allowed.contains(&value) {
            Ok(value)
        } else {
            Err("event operand has an unsupported FheType")
        }
    }
    fn require_result_type(
        handle: &Handle,
        expected: u8,
    ) -> Result<(), &'static str> {
        if ty(handle) == expected {
            Ok(())
        } else {
            Err("event result type disagrees with operation metadata")
        }
    }
    fn binary(
        lhs: &Handle,
        rhs: &Handle,
        scalar_byte: &ScalarByte,
        result: &Handle,
        allowed: &[u8],
        bool_result: bool,
    ) -> Result<(), &'static str> {
        let lhs_type = require_allowed(lhs, allowed)?;
        if !binary_rhs_is_scalar(scalar_byte) && ty(rhs) != lhs_type {
            return Err("encrypted binary operands have different FheTypes");
        }
        require_result_type(result, if bool_result { BOOL } else { lhs_type })
    }

    match op {
        E::FheAdd(C::FheAdd {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheSub(C::FheSub {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheMul(C::FheMul {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheDiv(C::FheDiv {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheRem(C::FheRem {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheMin(C::FheMin {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheMax(C::FheMax {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        }) => binary(lhs, rhs, scalarByte, result, ARITHMETIC, false),
        E::FheBitAnd(C::FheBitAnd {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheBitOr(C::FheBitOr {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheBitXor(C::FheBitXor {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        }) => binary(lhs, rhs, scalarByte, result, BITWISE, false),
        E::FheShl(C::FheShl {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheShr(C::FheShr {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheRotl(C::FheRotl {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheRotr(C::FheRotr {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        }) => binary(lhs, rhs, scalarByte, result, NUMERIC, false),
        E::FheEq(C::FheEq {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheNe(C::FheNe {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        }) => binary(lhs, rhs, scalarByte, result, HOST_VALUE, true),
        E::FheGe(C::FheGe {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheGt(C::FheGt {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheLe(C::FheLe {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        })
        | E::FheLt(C::FheLt {
            lhs,
            rhs,
            scalarByte,
            result,
            ..
        }) => binary(lhs, rhs, scalarByte, result, ARITHMETIC, true),
        E::FheNeg(C::FheNeg { ct, result, .. }) => {
            let input_type = require_allowed(ct, NUMERIC)?;
            require_result_type(result, input_type)
        }
        E::FheNot(C::FheNot { ct, result, .. }) => {
            let input_type = require_allowed(ct, BITWISE)?;
            require_result_type(result, input_type)
        }
        E::Cast(C::Cast {
            ct, toType, result, ..
        }) => {
            let input_type =
                require_allowed(ct, &[BOOL, U8, U16, U32, U64, U128, U256])?;
            if !NUMERIC.contains(toType) {
                return Err("cast target has an unsupported FheType");
            }
            if input_type == *toType {
                return Err("cast input and target FheTypes are identical");
            }
            require_result_type(result, *toType)
        }
        E::FheIfThenElse(C::FheIfThenElse {
            control,
            ifTrue,
            ifFalse,
            result,
            ..
        }) => {
            require_allowed(control, &[BOOL])?;
            let branch_type = require_allowed(ifTrue, HOST_VALUE)?;
            if ty(ifFalse) != branch_type {
                return Err("conditional branches have different FheTypes");
            }
            require_result_type(result, branch_type)
        }
        E::TrivialEncrypt(C::TrivialEncrypt { toType, result, .. }) => {
            if !HOST_VALUE.contains(toType) {
                return Err(
                    "trivial-encryption target has an unsupported FheType",
                );
            }
            require_result_type(result, *toType)
        }
        E::FheRand(C::FheRand {
            randType, result, ..
        }) => {
            if !RANDOM.contains(randType) {
                return Err("random target has an unsupported FheType");
            }
            require_result_type(result, *randType)
        }
        E::FheRandBounded(C::FheRandBounded {
            randType, result, ..
        }) => {
            if !NUMERIC.contains(randType) {
                return Err("bounded-random target has an unsupported FheType");
            }
            require_result_type(result, *randType)
        }
        E::FheSum(C::FheSum { values, result, .. }) => {
            let result_type = require_allowed(result, ARITHMETIC)?;
            if values.iter().any(|value| ty(value) != result_type) {
                return Err("sum operands disagree with result FheType");
            }
            Ok(())
        }
        E::FheIsIn(C::FheIsIn {
            value,
            values,
            result,
            ..
        }) => {
            let value_type = require_allowed(value, MEMBERSHIP)?;
            if values.iter().any(|element| ty(element) != value_type) {
                return Err("membership operands have different FheTypes");
            }
            require_result_type(result, BOOL)
        }
        E::FheMulDiv(C::FheMulDiv {
            factor1,
            factor2,
            scalarByte,
            result,
            ..
        }) => {
            let input_type = require_allowed(factor1, &[U8, U16, U32, U64])?;
            if !fhe_mul_div_factor2_is_scalar(scalarByte)
                && ty(factor2) != input_type
            {
                return Err("MulDiv encrypted factors have different FheTypes");
            }
            require_result_type(result, input_type)
        }
        E::VerifyInput(_) | E::Initialized(_) | E::Upgraded(_) => Ok(()),
    }
}

#[cfg(test)]
mod scalar_boundary_tests {
    use super::*;
    use alloy_primitives::Address;

    fn shifts(rhs: Handle, flag: u8, ty: u8) -> [TfheContractEvents; 4] {
        use TfheContract as C;
        use TfheContractEvents as E;
        let caller = Address::ZERO;
        let mut lhs = Handle::ZERO;
        lhs.0[30] = ty;
        let result = Handle::ZERO;
        let scalar_byte = FixedBytes([flag]);
        [
            E::FheShl(C::FheShl {
                caller,
                lhs,
                rhs,
                scalarByte: scalar_byte,
                result,
            }),
            E::FheShr(C::FheShr {
                caller,
                lhs,
                rhs,
                scalarByte: scalar_byte,
                result,
            }),
            E::FheRotl(C::FheRotl {
                caller,
                lhs,
                rhs,
                scalarByte: scalar_byte,
                result,
            }),
            E::FheRotr(C::FheRotr {
                caller,
                lhs,
                rhs,
                scalarByte: scalar_byte,
                result,
            }),
        ]
    }

    #[test]
    fn shifts_and_rotates_preserve_pre_upgrade_scalar_history() {
        for ty in [2, 3, 4, 5, 6, 8] {
            for flag in [0, 1, 2, 3, 128, 255] {
                // Cover both modern uint8 bounds and the original unchecked
                // uint256 amount, without truncating or changing the operand.
                for rhs in [
                    Handle::ZERO,
                    Handle::with_last_byte(255),
                    Handle::from(
                        alloy_primitives::U256::from(256).to_be_bytes::<32>(),
                    ),
                    FixedBytes([255; 32]),
                ] {
                    for event in shifts(rhs, flag, ty) {
                        assert!(validate_scalar_mode(&event).is_ok());
                        assert_eq!(
                            tfhe_inputs_handle(&event).len(),
                            if flag & 1 != 0 { 1 } else { 2 }
                        );
                    }
                }
            }
        }
        for ty in [0, 1, 7, 9, 83, 255] {
            for event in shifts(Handle::ZERO, 1, ty) {
                assert!(validate_scalar_mode(&event).is_err());
            }
        }
    }
}

/// The executor rejects oversized collections before emitting these events.
/// Their function-only type arguments are not present in the event ABI, but
/// are retained in the output (`FheSum`) or first operand (`FheIsIn`) handle.
/// This checks the host event's application metadata, never TFHE values.
///
/// These limits mirror FHE_COLLECTION_{NARROW,WIDE}_MAX_SIZE in
/// host-contracts/contracts/FHEVMExecutor.sol. Before increasing a contract
/// limit, raise the matching listener limit and deploy compatible listeners
/// to every operator; otherwise newly valid events will block ingestion.
/// Keep accepting historical limits if the contract later lowers its caps.
pub fn validate_collection_shape(
    op: &TfheContractEvents,
) -> Result<(), &'static str> {
    use TfheContract as C;
    use TfheContractEvents as E;
    match op {
        E::FheSum(C::FheSum { values, result, .. }) => {
            let limit = match get_ct_type(result.as_slice()) {
                Some(2..=4) => 100,
                Some(5 | 6) => 60,
                _ => return Err("FheSum event has unsupported result type"),
            };
            if values.len() > limit {
                Err("FheSum event exceeds collection limit")
            } else {
                Ok(())
            }
        }
        E::FheIsIn(C::FheIsIn { value, values, .. }) => {
            let limit = match get_ct_type(value.as_slice()) {
                Some(2..=4) => 100,
                Some(5..=8) => 60,
                _ => return Err("FheIsIn event has unsupported value type"),
            };
            if values.len() > limit {
                Err("FheIsIn event exceeds collection limit")
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn tfhe_inputs_handle(op: &TfheContractEvents) -> Vec<Handle> {
    use TfheContract as C;
    use TfheContractEvents as E;
    match op {
        E::Cast(C::Cast { ct, .. })
        | E::FheNeg(C::FheNeg { ct, .. })
        | E::FheNot(C::FheNot { ct, .. }) => vec![*ct],

        E::FheAdd(C::FheAdd {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitAnd(C::FheBitAnd {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitOr(C::FheBitOr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitXor(C::FheBitXor {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheDiv(C::FheDiv {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMax(C::FheMax {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMin(C::FheMin {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMul(C::FheMul {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRem(C::FheRem {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRotl(C::FheRotl {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRotr(C::FheRotr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheShl(C::FheShl {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheShr(C::FheShr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheSub(C::FheSub {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheEq(C::FheEq {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheGe(C::FheGe {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheGt(C::FheGt {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheLe(C::FheLe {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheLt(C::FheLt {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheNe(C::FheNe {
            lhs,
            rhs,
            scalarByte,
            ..
        }) => {
            if !binary_rhs_is_scalar(scalarByte) {
                vec![*lhs, *rhs]
            } else {
                vec![*lhs]
            }
        }

        E::FheIfThenElse(C::FheIfThenElse {
            control,
            ifTrue,
            ifFalse,
            ..
        }) => {
            vec![*control, *ifTrue, *ifFalse]
        }

        E::FheRand(_) | E::FheRandBounded(_) | E::TrivialEncrypt(_) => vec![],

        E::FheSum(C::FheSum { values, .. }) => values.clone(),

        E::FheIsIn(C::FheIsIn { value, values, .. }) => {
            let mut handles = vec![*value];
            handles.extend(values.iter().copied());
            handles
        }

        E::FheMulDiv(C::FheMulDiv {
            factor1,
            factor2,
            scalarByte,
            ..
        }) => {
            if fhe_mul_div_factor2_is_scalar(scalarByte) {
                vec![*factor1]
            } else {
                vec![*factor1, *factor2]
            }
        }

        E::Initialized(_) | E::Upgraded(_) | E::VerifyInput(_) => vec![],
    }
}

/// Returns every encrypted executor operand together with its position in the
/// `dependencies` vector stored for the computation. The position is also the
/// bit position used by `FHEVMExecutor`'s `boundaryBits` preimage. Plaintext
/// scalar positions are intentionally absent, and therefore retain their
/// required zero bit.
///
/// Keep this in lock-step with both `insert_tfhe_event` and the executor's
/// `_oneOperandBoundaryBit` callers. In particular, `FheMulDiv` always has a
/// scalar divisor at position 2, and binary scalar operations have a scalar
/// right operand at position 1.
pub fn tfhe_encrypted_operand_positions(
    op: &TfheContractEvents,
) -> Vec<(usize, Handle)> {
    use TfheContract as C;
    use TfheContractEvents as E;

    match op {
        E::Cast(C::Cast { ct, .. })
        | E::FheNeg(C::FheNeg { ct, .. })
        | E::FheNot(C::FheNot { ct, .. }) => vec![(0, *ct)],

        E::FheAdd(C::FheAdd {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitAnd(C::FheBitAnd {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitOr(C::FheBitOr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitXor(C::FheBitXor {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheDiv(C::FheDiv {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMax(C::FheMax {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMin(C::FheMin {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMul(C::FheMul {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRem(C::FheRem {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRotl(C::FheRotl {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRotr(C::FheRotr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheShl(C::FheShl {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheShr(C::FheShr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheSub(C::FheSub {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheEq(C::FheEq {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheGe(C::FheGe {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheGt(C::FheGt {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheLe(C::FheLe {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheLt(C::FheLt {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheNe(C::FheNe {
            lhs,
            rhs,
            scalarByte,
            ..
        }) => {
            let mut operands = vec![(0, *lhs)];
            if !binary_rhs_is_scalar(scalarByte) {
                operands.push((1, *rhs));
            }
            operands
        }

        E::FheIfThenElse(C::FheIfThenElse {
            control,
            ifTrue,
            ifFalse,
            ..
        }) => vec![(0, *control), (1, *ifTrue), (2, *ifFalse)],

        E::FheRand(_) | E::FheRandBounded(_) | E::TrivialEncrypt(_) => {
            vec![]
        }

        E::FheSum(C::FheSum { values, .. }) => {
            values.iter().copied().enumerate().collect()
        }

        E::FheIsIn(C::FheIsIn { value, values, .. }) => {
            let mut operands =
                Vec::with_capacity(values.len().saturating_add(1));
            operands.push((0, *value));
            operands.extend(
                values
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(i, value)| (i.saturating_add(1), value)),
            );
            operands
        }

        E::FheMulDiv(C::FheMulDiv {
            factor1,
            factor2,
            scalarByte,
            ..
        }) => {
            let mut operands = vec![(0, *factor1)];
            if !fhe_mul_div_factor2_is_scalar(scalarByte) {
                operands.push((1, *factor2));
            }
            operands
        }

        E::Initialized(_) | E::Upgraded(_) | E::VerifyInput(_) => vec![],
    }
}

/// Reconstructs the exact `uint256 boundaryBits` value committed by the
/// executor for one operation. `was_minted` must represent successful prior
/// executor operations in this transaction, in log order.
///
/// The executor only has 256 bits of provenance. Current collection events
/// are bounded below that limit; reject a larger event rather than silently
/// inventing a sourcing rule that its handle does not commit to.
pub fn operand_boundary_mask_from_minted<F>(
    op: &TfheContractEvents,
    mut was_minted: F,
) -> Result<OperandBoundaryMask, String>
where
    F: FnMut(&Handle) -> bool,
{
    let mut mask = [0_u8; OPERAND_BOUNDARY_MASK_BYTES];
    for (position, handle) in tfhe_encrypted_operand_positions(op) {
        if position >= OPERAND_BOUNDARY_MASK_BYTES * 8 {
            return Err(format!(
                "operation has encrypted operand at position {position}, beyond executor boundaryBits"
            ));
        }
        if !was_minted(&handle) {
            // Match `uint256` ABI byte order: Solidity bit 0 is the least
            // significant bit of the final byte.
            let byte_index = OPERAND_BOUNDARY_MASK_BYTES - 1 - position / 8;
            mask[byte_index] |= 1 << (position % 8);
        }
    }
    Ok(mask)
}

/// Shared by dependency extraction and the persisted worker scalar flag.
/// Non-zero even bytes are encrypted operands in historical executor events.
pub(crate) fn binary_rhs_is_scalar(scalar_byte: &ScalarByte) -> bool {
    scalar_byte.0[0] & 1 != 0
}

/// `fheMulDiv` `scalarByte` bit 1 — factor2 is a plaintext scalar (bit 0 is the
/// always-scalar divisor).
pub(crate) fn fhe_mul_div_factor2_is_scalar(scalar_byte: &ScalarByte) -> bool {
    scalar_byte.0[0] & 0b10 != 0
}

#[cfg(test)]
mod scalar_mode_tests {
    use super::*;
    use alloy_primitives::Address;
    #[test]
    fn scalar_flags_preserve_historical_low_bit_semantics() {
        use TfheContract as C;
        use TfheContractEvents as E;
        for flag in 0..=u8::MAX {
            let caller = Address::ZERO;
            let lhs = Handle::ZERO;
            let rhs = Handle::ZERO;
            let result = Handle::ZERO;
            let scalar_byte = FixedBytes([flag]);
            let add = E::FheAdd(C::FheAdd {
                caller,
                lhs,
                rhs,
                result,
                scalarByte: scalar_byte,
            });
            let div = E::FheDiv(C::FheDiv {
                caller,
                lhs,
                rhs,
                result,
                scalarByte: scalar_byte,
            });
            let rem = E::FheRem(C::FheRem {
                caller,
                lhs,
                rhs,
                result,
                scalarByte: scalar_byte,
            });
            let mul_div = E::FheMulDiv(C::FheMulDiv {
                caller,
                factor1: lhs,
                factor2: rhs,
                divisor: rhs,
                result,
                scalarByte: scalar_byte,
            });
            assert!(validate_scalar_mode(&add).is_ok());
            assert_eq!(
                tfhe_inputs_handle(&add).len(),
                if flag & 1 != 0 { 1 } else { 2 }
            );
            let mask =
                operand_boundary_mask_from_minted(&add, |_| false).unwrap();
            assert_eq!(mask[31], if flag & 1 != 0 { 1 } else { 3 });
            assert_eq!(validate_scalar_mode(&div).is_ok(), flag & 1 != 0);
            assert_eq!(validate_scalar_mode(&rem).is_ok(), flag & 1 != 0);
            assert_eq!(
                validate_scalar_mode(&mul_div).is_ok(),
                flag == 1 || flag == 3
            );
        }
    }
}

#[cfg(test)]
mod collection_boundary_tests {
    use super::*;
    use alloy_primitives::Address;
    #[test]
    fn collection_limits_follow_event_type_metadata() {
        use TfheContract as C;
        use TfheContractEvents as E;
        for type_byte in [0, 2, 4, 5, 6, 7, 8, 9, 83, 255] {
            for len in [0, 60, 61, 100, 101] {
                let mut handle = Handle::ZERO;
                handle.0[30] = type_byte;
                let values = vec![Handle::ZERO; len];
                let sum = E::FheSum(C::FheSum {
                    caller: Address::ZERO,
                    values: values.clone(),
                    result: handle,
                });
                let is_in = E::FheIsIn(C::FheIsIn {
                    caller: Address::ZERO,
                    value: handle,
                    values,
                    result: Handle::ZERO,
                });
                let sum_valid = match type_byte {
                    2..=4 => len <= 100,
                    5 | 6 => len <= 60,
                    _ => false,
                };
                let is_in_valid = match type_byte {
                    2..=4 => len <= 100,
                    5..=8 => len <= 60,
                    _ => false,
                };
                assert_eq!(validate_collection_shape(&sum).is_ok(), sum_valid);
                assert_eq!(
                    validate_collection_shape(&is_in).is_ok(),
                    is_in_valid
                );
            }
        }
    }
}
