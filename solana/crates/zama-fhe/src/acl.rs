//! Application identity and execution output policy.
//! Public API surface: apps construct output policies and validated bounds for random operations.

use crate::types::FheType;
use crate::{FheExecutionBuildError, Result};
pub use zama_host::AppScope;

/// Validated power-of-two upper bound for host bounded-random `euint64` creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedU64UpperBound {
    value: [u8; 32],
}

impl BoundedU64UpperBound {
    pub fn power_of_two(value: u64) -> Result<Self> {
        if value == 0 || !value.is_power_of_two() {
            return Err(FheExecutionBuildError::InvalidRandomUpperBound);
        }
        let mut bytes = [0u8; 32];
        bytes[24..].copy_from_slice(&value.to_be_bytes());
        Self::from_be_bytes(bytes)
    }

    pub fn from_be_bytes(value: [u8; 32]) -> Result<Self> {
        zama_host::assert_valid_bounded_rand_upper_bound(value, FheType::UINT64.byte())
            .map_err(|_| FheExecutionBuildError::InvalidRandomUpperBound)?;
        Ok(Self { value })
    }

    pub fn bytes(self) -> [u8; 32] {
        self.value
    }
}

impl TryFrom<u64> for BoundedU64UpperBound {
    type Error = FheExecutionBuildError;

    fn try_from(value: u64) -> Result<Self> {
        Self::power_of_two(value)
    }
}

/// Output policy exposed by the builder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output(pub(crate) OutputKind);

#[derive(Debug, Clone, PartialEq, Eq)]
// Passed by value into lowering, which consumes it in place; boxing the persistent variant
// would put one more allocation on the program's never-freeing heap per output.
#[allow(clippy::large_enum_variant)]
pub(crate) enum OutputKind {
    Transient,
    State(crate::StateOutput),
}

impl Output {
    pub fn state(output: crate::StateOutput) -> Self {
        Self(OutputKind::State(output))
    }

    pub fn transient() -> Self {
        Self(OutputKind::Transient)
    }
}
