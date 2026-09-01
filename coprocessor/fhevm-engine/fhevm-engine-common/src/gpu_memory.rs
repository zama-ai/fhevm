use crate::{
    tfhe_ops::*,
    types::{FhevmError, SupportedFheCiphertexts, SupportedFheOperations},
};
use lazy_static::lazy_static;
use prometheus::{register_int_counter_vec, IntCounterVec};
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        LazyLock,
    },
    time::{Duration, Instant},
};
use tfhe::{core_crypto::gpu::get_number_of_gpus, prelude::*, FheUint2, GpuIndex};
use tokio_util::sync::CancellationToken;
use tracing::{error, warn};

lazy_static! {
    static ref GPU_MEMORY_POOL: GpuMemoryPool = GpuMemoryPool::new(get_number_of_gpus() as usize);
}

const RESERVATION_RETRY_INTERVAL: Duration = Duration::from_millis(2);
const RESERVATION_PROGRESS_INTERVAL: Duration = Duration::from_secs(5);

static RESERVATION_TIMEOUT_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_gpu_memory_reservation_timeouts_total",
        "GPU memory reservation waits which exhausted their configured deadline",
        &["gpu_idx"]
    )
    .expect("GPU memory reservation timeout metric registration")
});

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum GpuMemoryReservationError {
    #[error("GPU memory reservation cancelled for device {gpu_idx} ({amount} bytes)")]
    Cancelled { gpu_idx: usize, amount: u64 },
    #[error("GPU memory reservation requested unknown device {gpu_idx}")]
    UnknownDevice { gpu_idx: usize },
    #[error("GPU memory reservation accounting overflow on device {gpu_idx}")]
    AccountingOverflow { gpu_idx: usize },
    #[error(
        "GPU memory reservation timed out for device {gpu_idx} after {waited_ms}ms ({amount} bytes)"
    )]
    TimedOut {
        gpu_idx: usize,
        amount: u64,
        waited_ms: u64,
    },
}

struct GpuMemoryPool {
    reserved: Vec<AtomicU64>,
}

impl GpuMemoryPool {
    fn new(device_count: usize) -> Self {
        Self {
            reserved: (0..device_count).map(|_| AtomicU64::new(0)).collect(),
        }
    }

    fn acquire_with(
        &self,
        amount: u64,
        gpu_idx: usize,
        cancellation: &CancellationToken,
        max_wait: Duration,
        can_allocate: impl FnMut(u64, usize) -> bool,
    ) -> Result<GpuMemoryReservation<'_>, GpuMemoryReservationError> {
        self.acquire_with_limits(
            amount,
            gpu_idx,
            cancellation,
            RESERVATION_RETRY_INTERVAL,
            max_wait,
            can_allocate,
        )
    }

    fn acquire_with_limits(
        &self,
        amount: u64,
        gpu_idx: usize,
        cancellation: &CancellationToken,
        retry_interval: Duration,
        max_wait: Duration,
        mut can_allocate: impl FnMut(u64, usize) -> bool,
    ) -> Result<GpuMemoryReservation<'_>, GpuMemoryReservationError> {
        let counter = self
            .reserved
            .get(gpu_idx)
            .ok_or(GpuMemoryReservationError::UnknownDevice { gpu_idx })?;
        let started_at = Instant::now();
        let mut next_progress_log = RESERVATION_PROGRESS_INTERVAL;
        loop {
            if cancellation.is_cancelled() {
                return Err(GpuMemoryReservationError::Cancelled { gpu_idx, amount });
            }

            let previous = counter
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |reserved| {
                    reserved.checked_add(amount)
                })
                .map_err(|_| GpuMemoryReservationError::AccountingOverflow { gpu_idx })?;
            let total = previous + amount;
            let reservation = GpuMemoryReservation {
                pool: self,
                amount,
                gpu_idx,
            };
            if can_allocate(total, gpu_idx) {
                // Close the race where cancellation arrived while CUDA memory
                // availability was being queried.
                if cancellation.is_cancelled() {
                    drop(reservation);
                    return Err(GpuMemoryReservationError::Cancelled { gpu_idx, amount });
                }
                return Ok(reservation);
            }
            drop(reservation);

            let elapsed = started_at.elapsed();
            if elapsed >= max_wait {
                let waited_ms = u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX);
                warn!(
                    target: "gpu_memory",
                    gpu_idx,
                    amount,
                    waited_ms,
                    "GPU memory reservation timed out"
                );
                return Err(GpuMemoryReservationError::TimedOut {
                    gpu_idx,
                    amount,
                    waited_ms,
                });
            }
            if elapsed >= next_progress_log {
                warn!(
                    target: "gpu_memory",
                    gpu_idx,
                    amount,
                    waited_ms = u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX),
                    "waiting for GPU memory reservation"
                );
                next_progress_log = elapsed.saturating_add(RESERVATION_PROGRESS_INTERVAL);
            }
            std::thread::sleep(retry_interval.min(max_wait.saturating_sub(elapsed)));
        }
    }

    #[cfg(test)]
    fn reserved(&self, gpu_idx: usize) -> u64 {
        self.reserved[gpu_idx].load(Ordering::Acquire)
    }
}

/// Owns one device's reservation accounting. Dropping the guard releases the
/// reservation on success, ordinary errors, cancellation, and unwinding.
pub struct GpuMemoryReservation<'a> {
    pool: &'a GpuMemoryPool,
    amount: u64,
    gpu_idx: usize,
}

impl Drop for GpuMemoryReservation<'_> {
    fn drop(&mut self) {
        let previous = self.pool.reserved[self.gpu_idx].fetch_sub(self.amount, Ordering::AcqRel);
        if previous < self.amount {
            // Restore the counter before reporting the invariant violation;
            // wrapping it would poison all later capacity decisions.
            self.pool.reserved[self.gpu_idx].fetch_add(self.amount, Ordering::Release);
            error!(
                target: "gpu_memory",
                gpu_idx = self.gpu_idx,
                amount = self.amount,
                reserved = previous,
                "GPU memory reservation accounting underflow"
            );
        }
    }
}

pub fn reserve_memory_on_gpu(
    amount: u64,
    gpu_idx: usize,
    cancellation: &CancellationToken,
    max_wait: Duration,
) -> Result<GpuMemoryReservation<'static>, GpuMemoryReservationError> {
    GPU_MEMORY_POOL
        .acquire_with(amount, gpu_idx, cancellation, max_wait, |total, idx| {
            check_valid_cuda_malloc(total, GpuIndex::new(idx as u32))
        })
        .inspect_err(|error| {
            if matches!(error, GpuMemoryReservationError::TimedOut { .. }) {
                RESERVATION_TIMEOUT_COUNTER
                    .with_label_values(&[&gpu_idx.to_string()])
                    .inc();
            }
        })
}

impl SupportedFheCiphertexts {
    pub fn move_to_current_device(&mut self) {
        match self {
            SupportedFheCiphertexts::FheBool(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint4(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint8(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint16(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint32(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint64(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint128(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint160(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint256(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheBytes64(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheBytes128(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheBytes256(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::Scalar(_) => {}
        };
    }

    pub fn get_size_on_gpu(&self) -> u64 {
        match self {
            SupportedFheCiphertexts::FheBool(v) => {
                let v: FheUint2 = v.to_owned().cast_into();
                v.get_size_on_gpu()
            } // TODO fix when available
            SupportedFheCiphertexts::FheUint4(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint8(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint16(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint32(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint64(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint128(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint160(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint256(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheBytes64(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheBytes128(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheBytes256(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::Scalar(v) => v.len() as u64,
        }
    }
}

fn get_fhe_sum_size_on_gpu(
    _fhe_operation: i16,
    input_operands: &[SupportedFheCiphertexts],
) -> Result<u64, FhevmError> {
    if input_operands.is_empty() {
        return Ok(0);
    }
    let n = input_operands.len() as u64;
    // No dedicated get_sum_size_on_gpu API exists in tfhe-rs; using N * ciphertext_size
    // as an approximation.
    match &input_operands[0] {
        SupportedFheCiphertexts::FheUint8(v) => Ok(v.get_size_on_gpu().saturating_mul(n)),
        SupportedFheCiphertexts::FheUint16(v) => Ok(v.get_size_on_gpu().saturating_mul(n)),
        SupportedFheCiphertexts::FheUint32(v) => Ok(v.get_size_on_gpu().saturating_mul(n)),
        SupportedFheCiphertexts::FheUint64(v) => Ok(v.get_size_on_gpu().saturating_mul(n)),
        SupportedFheCiphertexts::FheUint128(v) => Ok(v.get_size_on_gpu().saturating_mul(n)),
        _ => Err(FhevmError::UnsupportedFheTypes {
            fhe_operation: format!("{:?}", _fhe_operation),
            input_types: input_operands.iter().map(|i| i.type_name()).collect(),
        }),
    }
}

fn get_fhe_is_in_size_on_gpu(
    _fhe_operation: i16,
    input_operands: &[SupportedFheCiphertexts],
) -> Result<u64, FhevmError> {
    if input_operands.is_empty() {
        return Ok(0);
    }
    let n = input_operands.len() as u64;
    // No dedicated API exists; using N * ciphertext_size as an approximation.
    match &input_operands[0] {
        SupportedFheCiphertexts::FheUint8(v) => Ok(v.get_size_on_gpu().saturating_mul(n)),
        SupportedFheCiphertexts::FheUint16(v) => Ok(v.get_size_on_gpu().saturating_mul(n)),
        SupportedFheCiphertexts::FheUint32(v) => Ok(v.get_size_on_gpu().saturating_mul(n)),
        SupportedFheCiphertexts::FheUint64(v) => Ok(v.get_size_on_gpu().saturating_mul(n)),
        SupportedFheCiphertexts::FheUint128(v) => Ok(v.get_size_on_gpu().saturating_mul(n)),
        SupportedFheCiphertexts::FheUint160(v) => Ok(v.get_size_on_gpu().saturating_mul(n)),
        SupportedFheCiphertexts::FheUint256(v) => Ok(v.get_size_on_gpu().saturating_mul(n)),
        _ => Err(FhevmError::UnsupportedFheTypes {
            fhe_operation: format!("{:?}", _fhe_operation),
            input_types: input_operands.iter().map(|i| i.type_name()).collect(),
        }),
    }
}

// PRECONDITION (panic-avoidance sweep, 2026-08-08): every match arm below
// asserts `input_operands.len()` against the operation's fixed arity and then
// indexes the slice directly. That precondition is established by
// `tfhe_ops::check_fhe_operand_types`, which validates arity/type/scalar
// placement for every `SupportedFheOperations` variant (including the
// variable-arity `Other` ops) and returns a typed `FhevmError` on mismatch.
// The one production caller (`tfhe-worker::component_worker`) always runs
// that check before a computation reaches `perform_fhe_operation` /
// `get_op_size_on_gpu`, so the asserts/indexing here enforce an
// already-validated invariant rather than gating untrusted input directly.
// Rewriting every arm to re-validate defensively would touch ~40 match arms
// in this function alone (mirrored in `tfhe_ops::perform_fhe_operation_impl`)
// without a signature change; left as a follow-up given the size/risk of that
// change in a crypto hot path. Any new caller of these `pub fn`s must run
// `check_fhe_operand_types` first or risk a panic on malformed operand counts.
pub fn get_op_size_on_gpu(
    fhe_operation_int: i16,
    input_operands: &[SupportedFheCiphertexts],
    // for deterministic randomness functions
) -> Result<u64, FhevmError> {
    let fhe_operation: SupportedFheOperations = fhe_operation_int.try_into()?;
    match fhe_operation {
        SupportedFheOperations::FheAdd => {
            assert_eq!(input_operands.len(), 2);

            // fhe add
            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_add_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_add_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_add_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_add_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_add_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_add_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_add_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_add_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }

        SupportedFheOperations::FheSub => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_sub_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_sub_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_sub_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_sub_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_sub_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_sub_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_sub_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_sub_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }

        SupportedFheOperations::FheMul => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_mul_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_mul_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_mul_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_mul_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_mul_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_mul_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_mul_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_mul_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheDiv => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_div_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_div_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_div_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_div_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_div_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_div_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_div_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_div_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheRem => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_rem_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_rem_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_rem_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_rem_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_rem_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_rem_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_rem_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_rem_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheBitAnd => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(a.get_bitand_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_bitand_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_bitand_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_bitand_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_bitand_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_bitand_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_bitand_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_bitand_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_bitand_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_bitand_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_bitand_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_bitand_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u4_bit(b) > 0))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheBitOr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(a.get_bitor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_bitor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_bitor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_bitor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_bitor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_bitor_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_bitor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_bitor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_bitor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_bitor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_bitor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_bitor_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_bitor_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheBitXor => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_bitxor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_bitxor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_bitxor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_bitxor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_bitxor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_bitxor_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_bitxor_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheShl => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_left_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_left_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_left_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_left_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_left_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_left_shift_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheShr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_right_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_right_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_right_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_right_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_right_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_right_shift_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheRotl => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_rotate_left_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_rotate_left_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_rotate_left_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_rotate_left_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_rotate_left_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_rotate_left_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheRotr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_rotate_right_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_rotate_right_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_rotate_right_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_rotate_right_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_rotate_right_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_rotate_right_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheMin => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_min_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_min_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_min_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_min_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_min_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_min_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_min_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_min_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_min_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_min_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_min_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheMax => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_max_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_max_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_max_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_max_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_max_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_max_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_max_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_max_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_max_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_max_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_max_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheEq => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(a.get_eq_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_eq_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_eq_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_eq_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_eq_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_eq_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_eq_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_eq_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_eq_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_eq_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_eq_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_eq_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_eq_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheNe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(a.get_ne_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_ne_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_ne_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_ne_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_ne_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_ne_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_ne_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_ne_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_ne_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_ne_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_ne_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_ne_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_ne_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheGe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_ge_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_ge_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_ge_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_ge_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_ge_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_ge_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_ge_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_ge_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_ge_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_ge_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_ge_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_ge_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheGt => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_gt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_gt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_gt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_gt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_gt_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_gt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_gt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_gt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_gt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_gt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_gt_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_gt_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheLe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_le_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_le_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_le_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_le_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_le_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_le_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_le_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_le_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_le_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_le_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_le_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_le_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheLt => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_lt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_lt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_lt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_lt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_lt_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_lt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_lt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_lt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_lt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_lt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_lt_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_lt_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheNot => {
            assert_eq!(input_operands.len(), 1);

            match &input_operands[0] {
                SupportedFheCiphertexts::FheBool(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint4(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint8(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint16(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint32(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint64(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint128(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint160(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint256(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheBytes64(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheBytes128(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheBytes256(a) => Ok(a.get_bitnot_size_on_gpu()),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheNeg => {
            assert_eq!(input_operands.len(), 1);

            match &input_operands[0] {
                SupportedFheCiphertexts::FheUint4(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint8(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint16(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint32(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint64(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint128(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint160(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint256(a) => Ok(a.get_neg_size_on_gpu()),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheIfThenElse => {
            assert_eq!(input_operands.len(), 3);

            let SupportedFheCiphertexts::FheBool(flag) = &input_operands[0] else {
                return Ok(0);
            };

            match (&input_operands[1], &input_operands[2]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    let b: FheUint2 = b.to_owned().cast_into();
                    Ok(flag.get_if_then_else_size_on_gpu(&a, &b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(flag.get_if_then_else_size_on_gpu(a, b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(flag.get_if_then_else_size_on_gpu(a, b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(flag.get_if_then_else_size_on_gpu(a, b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(flag.get_if_then_else_size_on_gpu(a, b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(flag.get_if_then_else_size_on_gpu(a, b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(flag.get_if_then_else_size_on_gpu(a, b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(flag.get_if_then_else_size_on_gpu(a, b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(flag.get_if_then_else_size_on_gpu(a, b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(flag.get_if_then_else_size_on_gpu(a, b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(flag.get_if_then_else_size_on_gpu(a, b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(flag.get_if_then_else_size_on_gpu(a, b)),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheTrivialEncrypt | SupportedFheOperations::FheCast => {
            match (&input_operands[0], &input_operands[1]) {
                (_, SupportedFheCiphertexts::Scalar(op)) => {
                    Ok(trivial_encrypt_be_bytes(to_be_u16_bit(op) as i16, &[1u8])?
                        .get_size_on_gpu())
                }
                (_, _) => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheRand => {
            let SupportedFheCiphertexts::Scalar(to_type) = &input_operands[1] else {
                return Ok(0);
            };
            let to_type = to_be_u16_bit(to_type) as i16;
            match to_type {
                0 => Ok(tfhe::FheUint2::get_generate_oblivious_pseudo_random_size_on_gpu()),
                1 => Ok(tfhe::FheUint4::get_generate_oblivious_pseudo_random_size_on_gpu()),
                2 => Ok(tfhe::FheUint8::get_generate_oblivious_pseudo_random_size_on_gpu()),
                3 => Ok(tfhe::FheUint16::get_generate_oblivious_pseudo_random_size_on_gpu()),
                4 => Ok(tfhe::FheUint32::get_generate_oblivious_pseudo_random_size_on_gpu()),
                5 => Ok(tfhe::FheUint64::get_generate_oblivious_pseudo_random_size_on_gpu()),
                6 => Ok(tfhe::FheUint128::get_generate_oblivious_pseudo_random_size_on_gpu()),
                7 => Ok(tfhe::FheUint160::get_generate_oblivious_pseudo_random_size_on_gpu()),
                8 => Ok(tfhe::FheUint256::get_generate_oblivious_pseudo_random_size_on_gpu()),
                9 => Ok(tfhe::FheUint512::get_generate_oblivious_pseudo_random_size_on_gpu()),
                10 => Ok(tfhe::FheUint1024::get_generate_oblivious_pseudo_random_size_on_gpu()),
                11 => Ok(tfhe::FheUint2048::get_generate_oblivious_pseudo_random_size_on_gpu()),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheRandBounded => {
            let SupportedFheCiphertexts::Scalar(to_type) = &input_operands[2] else {
                return Ok(0);
            };
            let to_type = to_be_u16_bit(to_type) as i16;
            match to_type {
                0 => Ok(tfhe::FheUint2::get_generate_oblivious_pseudo_random_bounded_size_on_gpu()),
                1 => Ok(tfhe::FheUint4::get_generate_oblivious_pseudo_random_bounded_size_on_gpu()),
                2 => Ok(tfhe::FheUint8::get_generate_oblivious_pseudo_random_bounded_size_on_gpu()),
                3 => {
                    Ok(tfhe::FheUint16::get_generate_oblivious_pseudo_random_bounded_size_on_gpu())
                }
                4 => {
                    Ok(tfhe::FheUint32::get_generate_oblivious_pseudo_random_bounded_size_on_gpu())
                }
                5 => {
                    Ok(tfhe::FheUint64::get_generate_oblivious_pseudo_random_bounded_size_on_gpu())
                }
                6 => Ok(
                    tfhe::FheUint128::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                ),
                7 => Ok(
                    tfhe::FheUint160::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                ),
                8 => Ok(
                    tfhe::FheUint256::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                ),
                9 => Ok(
                    tfhe::FheUint512::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                ),
                10 => Ok(
                    tfhe::FheUint1024::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                ),
                11 => Ok(
                    tfhe::FheUint2048::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                ),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheSum => {
            get_fhe_sum_size_on_gpu(fhe_operation_int, input_operands)
        }
        SupportedFheOperations::FheIsIn => {
            get_fhe_is_in_size_on_gpu(fhe_operation_int, input_operands)
        }
        SupportedFheOperations::FheMulDiv => {
            // MulDiv[T] runs at 2T bit-width internally.
            let widened_workspace = |mul_size: u64, div_size: u64| -> u64 {
                (mul_size + div_size) * 2u64.pow(2) // double size with quadratic space complexity
            };
            assert_eq!(input_operands.len(), 3);
            match (&input_operands[0], &input_operands[1], &input_operands[2]) {
                (
                    SupportedFheCiphertexts::FheUint8(a),
                    SupportedFheCiphertexts::FheUint8(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(widened_workspace(
                    a.get_mul_size_on_gpu(b),
                    a.get_div_size_on_gpu(to_be_u8_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint16(a),
                    SupportedFheCiphertexts::FheUint16(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(widened_workspace(
                    a.get_mul_size_on_gpu(b),
                    a.get_div_size_on_gpu(to_be_u16_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint32(a),
                    SupportedFheCiphertexts::FheUint32(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(widened_workspace(
                    a.get_mul_size_on_gpu(b),
                    a.get_div_size_on_gpu(to_be_u32_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint64(a),
                    SupportedFheCiphertexts::FheUint64(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(widened_workspace(
                    a.get_mul_size_on_gpu(b),
                    a.get_div_size_on_gpu(to_be_u64_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint8(a),
                    SupportedFheCiphertexts::Scalar(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(widened_workspace(
                    a.get_mul_size_on_gpu(to_be_u8_bit(b)),
                    a.get_div_size_on_gpu(to_be_u8_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint16(a),
                    SupportedFheCiphertexts::Scalar(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(widened_workspace(
                    a.get_mul_size_on_gpu(to_be_u16_bit(b)),
                    a.get_div_size_on_gpu(to_be_u16_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint32(a),
                    SupportedFheCiphertexts::Scalar(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(widened_workspace(
                    a.get_mul_size_on_gpu(to_be_u32_bit(b)),
                    a.get_div_size_on_gpu(to_be_u32_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint64(a),
                    SupportedFheCiphertexts::Scalar(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(widened_workspace(
                    a.get_mul_size_on_gpu(to_be_u64_bit(b)),
                    a.get_div_size_on_gpu(to_be_u64_bit(d)),
                )),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        _ => Err(FhevmError::UnknownFheOperation(fhe_operation_int.into())),
    }
}

#[cfg(test)]
mod reservation_tests {
    use super::*;
    use std::sync::{atomic::AtomicUsize, Arc, Barrier};

    #[test]
    fn reservation_releases_on_success_error_and_drop() {
        let pool = GpuMemoryPool::new(2);
        let cancellation = CancellationToken::new();
        {
            let _first = pool
                .acquire_with(11, 0, &cancellation, Duration::from_secs(1), |_, _| true)
                .expect("first reservation");
            let _second = pool
                .acquire_with(7, 1, &cancellation, Duration::from_secs(1), |_, _| true)
                .expect("second reservation");
            assert_eq!(pool.reserved(0), 11);
            assert_eq!(pool.reserved(1), 7);

            let operation: Result<(), &'static str> = (|| {
                let _temporary = pool
                    .acquire_with(5, 0, &cancellation, Duration::from_secs(1), |_, _| true)
                    .expect("temporary reservation");
                Err("operation failed")
            })();
            assert_eq!(operation, Err("operation failed"));
            assert_eq!(pool.reserved(0), 11);
        }
        assert_eq!(pool.reserved(0), 0);
        assert_eq!(pool.reserved(1), 0);
    }

    #[test]
    fn reservation_releases_during_panic_unwind() {
        let pool = GpuMemoryPool::new(1);
        let cancellation = CancellationToken::new();
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _reservation = pool
                .acquire_with(13, 0, &cancellation, Duration::from_secs(1), |_, _| true)
                .expect("reservation");
            panic!("simulated TFHE panic");
        }));
        assert!(panic.is_err());
        assert_eq!(pool.reserved(0), 0);
    }

    #[test]
    fn waiting_reservation_observes_cancellation_without_leaking() {
        let pool = GpuMemoryPool::new(1);
        let cancellation = CancellationToken::new();
        let cancel_from_probe = cancellation.clone();
        let mut probes = 0;
        let result = pool.acquire_with(17, 0, &cancellation, Duration::from_secs(1), |_, _| {
            probes += 1;
            cancel_from_probe.cancel();
            false
        });
        let error = match result {
            Ok(_) => panic!("cancelled reservation must fail"),
            Err(error) => error,
        };
        assert_eq!(probes, 1);
        assert_eq!(
            error,
            GpuMemoryReservationError::Cancelled {
                gpu_idx: 0,
                amount: 17
            }
        );
        assert_eq!(pool.reserved(0), 0);
    }

    #[test]
    fn cancellation_after_capacity_probe_releases_reservation() {
        let pool = GpuMemoryPool::new(1);
        let cancellation = CancellationToken::new();
        let cancel_from_probe = cancellation.clone();
        let result = pool.acquire_with(19, 0, &cancellation, Duration::from_secs(1), |_, _| {
            cancel_from_probe.cancel();
            true
        });
        let error = match result {
            Ok(_) => panic!("cancellation must win the capacity race"),
            Err(error) => error,
        };
        assert!(matches!(error, GpuMemoryReservationError::Cancelled { .. }));
        assert_eq!(pool.reserved(0), 0);
    }

    #[test]
    fn waiting_reservation_times_out_without_leaking() {
        let pool = GpuMemoryPool::new(1);
        let cancellation = CancellationToken::new();
        let result = pool.acquire_with_limits(
            23,
            0,
            &cancellation,
            Duration::ZERO,
            Duration::ZERO,
            |_, _| false,
        );
        let error = match result {
            Ok(_) => panic!("bounded reservation wait must time out"),
            Err(error) => error,
        };
        assert!(matches!(
            error,
            GpuMemoryReservationError::TimedOut {
                gpu_idx: 0,
                amount: 23,
                ..
            }
        ));
        assert_eq!(pool.reserved(0), 0);
    }

    #[test]
    fn simultaneous_oversubscription_never_leaks_or_overcommits_accounting() {
        let pool = Arc::new(GpuMemoryPool::new(1));
        let start = Arc::new(Barrier::new(2));
        let largest_probe = Arc::new(AtomicU64::new(0));
        let successful_guards = Arc::new(AtomicUsize::new(0));
        std::thread::scope(|scope| {
            for _ in 0..2 {
                let pool = pool.clone();
                let start = start.clone();
                let largest_probe = largest_probe.clone();
                let successful_guards = successful_guards.clone();
                scope.spawn(move || {
                    let cancellation = CancellationToken::new();
                    start.wait();
                    let reservation = pool
                        .acquire_with(60, 0, &cancellation, Duration::from_secs(1), |total, _| {
                            largest_probe.fetch_max(total, Ordering::AcqRel);
                            total <= 100
                        })
                        .expect("oversubscribed waiter eventually acquires");
                    let concurrent_guards = successful_guards.fetch_add(1, Ordering::AcqRel) + 1;
                    assert!(concurrent_guards <= 1);
                    std::thread::sleep(Duration::from_millis(10));
                    successful_guards.fetch_sub(1, Ordering::AcqRel);
                    drop(reservation);
                });
            }
        });
        assert!(largest_probe.load(Ordering::Acquire) >= 120);
        assert_eq!(successful_guards.load(Ordering::Acquire), 0);
        assert_eq!(pool.reserved(0), 0);
    }
}
